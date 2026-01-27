use poise::{CreateReply, serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor}};
use url::Url;

use crate::{Context, Error, db, discord_helper::MessageState, embeds::single_text_response, osu::{self, skin::DEFAULT}};
use crate::discord_helper::user_has_replay_role;
use crate::db::entities::skin;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

async fn has_replay_role(ctx: Context<'_>) -> Result<bool, Error> {
    if !user_has_replay_role(ctx, ctx.author()).await.unwrap() {
        single_text_response(&ctx, "No permission L", MessageState::INFO, true).await;
        return Ok(false);
    }
    Ok(true)
}

fn is_url(s: &str) -> bool {
    Url::parse(s).is_ok()
}


#[poise::command(slash_command, rename = "skin", subcommands("set", "get", "remove"), required_permissions = "SEND_MESSAGES")]
pub async fn bundle(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

/// Save a url to your skin
#[poise::command(slash_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "link to your skin"] url: String,
    #[description = "Name to reference your skin"] identifier: String,
    #[description = "default for when you upload the gamemode. If HDDT is not set, the DT skin will be used instead."] default: osu::skin::DEFAULT,
    #[description = "Desired member (uploaders only)"] member: Option<serenity::Member>,
) -> Result<(), Error> {
    ctx.defer().await?;

    if member.is_some() {
        if !user_has_replay_role(ctx.http(), ctx.author()).await? {
            single_text_response(&ctx, "No permission L", MessageState::INFO, true).await;
            return Ok(())
        }
    }

    if !is_url(&url) || !url.starts_with("https://git.sulej.net/") || !url.ends_with(".osk") {
        single_text_response(&ctx, "Please enter the download link to your skin in https://git.sulej.net/.\nHow to use: https://git.sulej.net/osc/skins/src/branch/main/how-to-use.md", MessageState::WARN, false).await;
        return Ok(());
    }

    let username = match &member {
        Some(member) => member.display_name().to_string(),
        None => {
            let member = ctx.author_member().await.unwrap();
            member.display_name().to_string()
        }
    };

    let user_id: i64 = match &member {
        Some(member) => member.user.id.into(),
        None => ctx.author().id.into()
    };

    let player = match osu::get_osu_instance().user(username).await {
        Ok(user) => user,
        Err(_) =>  {
            single_text_response(&ctx, "Your username is not related to your osu!account. Please inform the mods to rename you!", MessageState::WARN, false).await;
            return Ok(())
        }
    };

    let user = db::get_user_by_discord_id_or_create(user_id, player.user_id as i32).await?;

    let skin_upload_successful = osu::skin::download(&url).await?.is_some();

    if !skin_upload_successful {
        single_text_response(&ctx, "The skin file could not be downloaded", MessageState::WARN, false).await;
        return Ok(());
    }

    db::clean_up_default(user.clone(), default).await?;
    match skin::Entity::find().filter(skin::Column::User.eq(user.id)).filter(skin::Column::Identifier.eq(identifier.clone())).one(&db::get_db()).await? {
        Some(skin) => {
            let mut active_skin: skin::ActiveModel = skin.into();
            active_skin.url = Set(url);
            active_skin.default = Set(default.to_db());
            active_skin.update(&db::get_db()).await?;
        },
        None => {
            skin::ActiveModel {
                user: Set(user.id),
                identifier: Set(identifier),
                url: Set(url),
                default: Set(default.to_db()),
                ..Default::default()
            }.insert(&db::get_db()).await?;
        }
        };

    single_text_response(&ctx, "Skin has been saved", MessageState::SUCCESS, false).await;
    Ok(())
}

/// Get the url to a members skins
#[poise::command(slash_command, check = "has_replay_role")]
pub async fn get(
    ctx: Context<'_>,
    #[description = "Desired member"] member: Option<serenity::Member>,
    #[description = "leave empty for all skins"] identifier: Option<String>,
) -> Result<(), Error> {
    let username = match &member {
        Some(member) => member.display_name().to_string(),
        None => {
            let member = ctx.author_member().await.unwrap();
            member.display_name().to_string()
        }
    };

    let user_id: i64 = match &member {
        Some(member) => member.user.id.into(),
        None => ctx.author().id.into()
    };

    let player = match osu::get_osu_instance().user(&username).await {
        Ok(user) => user,
        Err(_) =>  {
            single_text_response(&ctx, "Your username is not related to your osu!account. Please inform the mods to rename you!", MessageState::SUCCESS, false).await;
            return Ok(())
        }
    };

    let user = db::get_user_by_discord_id_or_create(user_id, player.user_id as i32).await?;
    
    let skins = match identifier {
        Some(identifier) => {
            match db::get_skin_by_identifier(user, identifier).await? {
                Some(skin) => vec![skin],
                None => {
                    single_text_response(&ctx, "This user has not a skin with that identifier", MessageState::INFO, false).await;
                    return Ok(())
                }
            }
        },
        None => {
            db::get_all_skins_by_user(user).await?
        }
    };

    if skins.is_empty() {
        single_text_response(&ctx, "This user has not saved a skin", MessageState::INFO, false).await;
        return Ok(());
    }

    let mut embed = CreateEmbed::default().author(CreateEmbedAuthor::new(format!("Skins: {}", username)));
    for skin in skins {
        let default_text: String = if skin.default != DEFAULT::NODEFAULT.to_db() {format!("({})", DEFAULT::from_db(skin.default).to_string())} else {"".to_string()};
        embed = embed.field("", format!("[{} {}]({})", skin.identifier, default_text, skin.url), false);
    }

    ctx.send(CreateReply::default().embed(embed)).await.unwrap();
    Ok(())
}

/// Get the url to a members skins
#[poise::command(slash_command, check = "has_replay_role")]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "leave empty for all skins"] identifier: String,
    #[description = "Desired member (uploaders only)"] member: Option<serenity::Member>,
) -> Result<(), Error> {
    if member.is_some() {
        if !user_has_replay_role(ctx.http(), ctx.author()).await? {
            single_text_response(&ctx, "No permission L", MessageState::INFO, true).await;
            return Ok(())
        }
    }

    let username = match &member {
        Some(member) => member.display_name().to_string(),
        None => {
            let member = ctx.author_member().await.unwrap();
            member.display_name().to_string()
        }
    };

    let user_id: i64 = match &member {
        Some(member) => member.user.id.into(),
        None => ctx.author().id.into()
    };

    let player = match osu::get_osu_instance().user(&username).await {
        Ok(user) => user,
        Err(_) =>  {
            single_text_response(&ctx, "Your username is not related to your osu!account. Please inform the mods to rename you!", MessageState::SUCCESS, false).await;
            return Ok(())
        }
    };

    let user = db::get_user_by_discord_id_or_create(user_id, player.user_id as i32).await?;
    
    match db::get_skin_by_identifier(user, identifier.clone()).await? {
        Some(skin) => {
            skin.delete(&db::get_db()).await?;
            single_text_response(&ctx, &format!("Skin ```{}``` has been removed!", identifier), MessageState::SUCCESS, false).await;
        },
        None => {
            single_text_response(&ctx, &format!("Skin ```{}``` does not exist!", identifier), MessageState::INFO, false).await
        }
    }
    Ok(())
}
