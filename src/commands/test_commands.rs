use poise::serenity_prelude::{self as serenity, CreateActionRow, CreateAttachment};

use crate::{Context, Data, Error, embeds};
use crate::osu;
use crate::generate::thumbnail;

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Something went horribly wrong: {:?}", error);
}

#[poise::command(slash_command, rename = "test", subcommands("osu_client", "thumbnail"), on_error = "error_handler")]
pub async fn bundle(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

#[poise::command(slash_command, on_error = "error_handler")]
pub async fn osu_client(ctx: Context<'_>) -> Result<(), Error> {
    let score = osu::get_osu_instance().score(1724681877).await.expect("Score should exist");
    let embed = embeds::score_embed(&score).await?;
    let button_id = format!("thumbnail:{}", score.id);
    let button = serenity::CreateButton::new(button_id)
    .label("Render Thumbnail")
    .emoji(crate::emojis::SATA_ANDAGI)
    .style(serenity::ButtonStyle::Primary);

    ctx.send(
        poise::CreateReply::default()
        .embed(embed.footer(serenity::CreateEmbedFooter::new(format!("Requested by @{}", ctx.author().name))))
        .components(vec![CreateActionRow::Buttons(vec![button])])
    ).await?;
    Ok(())
}

#[poise::command(slash_command, on_error = "error_handler")]
pub async fn thumbnail(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let score = osu::get_osu_instance().score(1611084369).await.expect("Score should exist");
    let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap exists");
    let image = thumbnail::generate_thumbnail_from_score(score, map, "Cool subtitle that i definitely just added").await;
    ctx.send(poise::CreateReply::default().attachment(CreateAttachment::bytes(image, "thumbnail.png"))).await?;
    Ok(())
}