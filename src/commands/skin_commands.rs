use poise::{CreateReply, serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor}};
use url::Url;

use crate::{Context, Error, discord_helper::MessageState, embeds::single_text_response, firebase};
use crate::discord_helper::user_has_replay_role;

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


#[poise::command(slash_command, rename = "skin", subcommands("set", "get"), required_permissions = "SEND_MESSAGES")]
pub async fn bundle(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

/// Save a url to your skin
#[poise::command(slash_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "link to your skin"] url: String,
) -> Result<(), Error> {
    if !is_url(&url) {
        single_text_response(&ctx, "Please enter a url", MessageState::WARN, false).await;
        return Ok(());
    }
    firebase::user::save_skin(&ctx.author().id.to_string(), &url).await;
    single_text_response(&ctx, "Skin has been saved", MessageState::SUCCESS, false).await;
    Ok(())
}

/// Get the url to a members skins
#[poise::command(slash_command, check = "has_replay_role")]
pub async fn get(
    ctx: Context<'_>,
    #[description = "Desired member"] member: Option<serenity::Member>,
) -> Result<(), Error> {
    let discord_user = match &member {
        Some(member) => member.user.clone(),
        None => ctx.author().clone(),
    };

    let username: String = match &member {
        Some(member) => {
            let user = member.clone();
            user.display_name().to_string()
        },
        None => {
            let author = ctx.author_member().await.expect("Member must exist").clone();
            author.display_name().to_string()
        },
    };

    let skin = firebase::user::get_user_skin(&discord_user.id.to_string()).await;
    match skin {
        Some(skin) => {
            ctx.send(CreateReply::default().embed(CreateEmbed::default().author(CreateEmbedAuthor::new(format!("Skins: {}", username))).description(&skin).url(&skin))).await?;
        },
        None => single_text_response(&ctx, "This user has not saved a skin", MessageState::INFO, false).await,
    };

    Ok(())
}