use std::fs::remove_file;

use poise::serenity_prelude::{self as serenity, CreateActionRow, CreateAttachment};
use crate::apis::{youtube};
use crate::embeds::single_text_response;
use crate::{Context, Error, embeds};
use crate::osu;
use crate::discord_helper::{MessageState};
use crate::generate::thumbnail;

#[poise::command(slash_command, rename = "dev", subcommands("test_osu_client", "test_thumbnail", "regenerate_token", "test_upload"))]
pub async fn bundle(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

#[poise::command(slash_command)]
pub async fn test_osu_client(ctx: Context<'_>) -> Result<(), Error> {
    let score = osu::get_osu_instance().score(1724681877).await.expect("Score should exist");
    let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap exists");
    let embed = embeds::score_embed_from_score(&score, &map, None).await?;
    let button_id = format!("thumbnail:{}", score.id);
    let button = serenity::CreateButton::new(button_id)
    .label("Render Thumbnail")
    .emoji(crate::emojis::SATA_ANDAGI)
    .style(serenity::ButtonStyle::Primary);

    ctx.send(
        poise::CreateReply::default()
        .embed(embed)
        .components(vec![CreateActionRow::Buttons(vec![button])])
    ).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn test_thumbnail(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let score = osu::get_osu_instance().score(1611084369).await.expect("Score should exist");
    let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap exists");
    let image = thumbnail::generate_thumbnail_from_score(&score, &map, "Cool subtitle that i definitely just added").await;
    ctx.send(poise::CreateReply::default().attachment(CreateAttachment::bytes(image, "thumbnail.png"))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn test_upload(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    youtube::upload(&"videoForRegen/random.mp4".into(), "test".into(), "test".into(), vec![]).await?;
    single_text_response(&ctx, "video has been uploaded!", MessageState::SUCCESS, true).await;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn regenerate_token(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    remove_file("token.json").ok();
    youtube::upload(&"videoForRegen/random.mp4".into(), "test".into(), "test".into(), vec![]).await?;
    single_text_response(&ctx, "regenerated token!", MessageState::SUCCESS, true).await;
    Ok(())
}
