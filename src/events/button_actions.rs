use poise::serenity_prelude::{ self as serenity, ComponentInteraction, CreateInteractionResponseMessage};
use crate::{Error, osu};
use crate::generate::thumbnail;

pub async fn handle_click(ctx: &serenity::Context, component: &ComponentInteraction) -> Result<(), Error> {
    let mut parts: std::str::Split<'_, char> = component.data.custom_id.split(':');

    let identifier = parts.next().unwrap();
    let data: Vec<&str> = parts.collect();


    let _ = match identifier {
        "thumbnail" => generate_thumbnail_from_button(ctx, component, &data.try_into().expect("Data must have 1 value")).await,
        _ => return Ok(())
    };
    Ok(())
}

pub async fn generate_thumbnail_from_button(ctx: &serenity::Context, component: &serenity::ComponentInteraction, data: &[&str; 1]) -> Result<(), Error> {
    component.create_response(ctx, serenity::CreateInteractionResponse::Defer(CreateInteractionResponseMessage::default().content("Thumbnail is being generated"))).await?;
    let score_id: u64 = data[0].parse().unwrap();
    let score = osu::get_osu_instance().score(score_id).await.expect("Score must exist");
    let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap must exist");
    let thumbnail = thumbnail::generate_thumbnail_from_score(score, map, &"").await;
    component.edit_response(
        ctx, 
        serenity::EditInteractionResponse::new().new_attachment(serenity::CreateAttachment::bytes(thumbnail, "thumbnail.png")
        )).await?;
    Ok(())
}