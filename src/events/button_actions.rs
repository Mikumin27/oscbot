use poise::serenity_prelude::{ self as serenity, ComponentInteraction, CreateEmbed, CreateInteractionResponseMessage};
use crate::discord_helper::{ContextForFunctions, MessageState, user_has_replay_role};
use crate::{Error, embeds, osu};
use crate::generate::{danser, thumbnail, upload};

pub async fn handle_click(ctx: &serenity::Context, component: &ComponentInteraction) -> Result<(), Error> {
    let mut parts: std::str::Split<'_, char> = component.data.custom_id.split(':');

    let identifier = parts.next().unwrap();
    let data: Vec<&str> = parts.collect();


    let _ = match identifier {
        "thumbnail" => generate_thumbnail_from_button(ctx, component, &data.try_into().expect("Data must have 1 value")).await,
        "upload" => upload_score_from_button(ctx, component, &data.try_into().expect("Data must have 1 value")).await,
        _ => return Ok(())
    };
    Ok(())
}

pub async fn generate_thumbnail_from_button(ctx: &serenity::Context, component: &serenity::ComponentInteraction, data: &[&str; 1]) -> Result<(), Error> {
    if !user_has_replay_role(ctx, &component.user).await.unwrap() {
        _ = component.create_response(ctx, 
            serenity::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default().embed(
                    CreateEmbed::default().description("No permission L").color(embeds::get_embed_color(&MessageState::INFO))
                ).ephemeral(true)
            )
        ).await?;
        return Ok(());
    }
    component.create_response(ctx, serenity::CreateInteractionResponse::Defer(CreateInteractionResponseMessage::default().content("Thumbnail is being generated"))).await?;
    let score_id: u64 = data[0].parse().unwrap();
    let score = osu::get_osu_instance().score(score_id).await.expect("Score must exist");
    let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap must exist");
    let thumbnail = thumbnail::generate_thumbnail_from_score(&score, &map, &"").await;
    component.edit_response(
        ctx, 
        serenity::EditInteractionResponse::new()
        .new_attachment(serenity::CreateAttachment::bytes(thumbnail, "thumbnail.png"))
    ).await?;
    Ok(())
}

pub async fn upload_score_from_button(ctx: &serenity::Context, component: &serenity::ComponentInteraction, data: &[&str; 1]) -> Result<(), Error> {
    if !user_has_replay_role(ctx, &component.user).await.unwrap() {
        _ = component.create_response(ctx, 
            serenity::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default().embed(
                    CreateEmbed::default().description("No permission L").color(embeds::get_embed_color(&MessageState::INFO))
                ).ephemeral(true)
            )
        ).await?;
        return Ok(());
    }
    let mut cff = ContextForFunctions {
        command_context: None,
        reply: None,
        event_context: Some(ctx),
        component: Some(component)
    };

    cff.send(embeds::render_and_upload_embed(&"...".to_string(), false, None, false)?).await?;
    let score_id: u64 = data[0].parse().unwrap();
    let score = osu::get_osu_instance().score(score_id).await.expect("Score must exist");
    let replay_bytes = osu::get_osu_instance().replay_raw(score_id).await.unwrap();
    let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap must exist");
    danser::attach_replay(&map.checksum.as_ref().unwrap(), &score_id.to_string(), &replay_bytes).await?;
    upload::render_and_upload_by_score(&cff, score, map, None).await?;
    Ok(())
}