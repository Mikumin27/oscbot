use std::vec;

use poise::serenity_prelude::{self as serenity, CreateAttachment, CreateButton, CreateMessage};
use rosu_v2::prelude as rosu;
use crate::{Context, Error, defaults, discord_helper::MessageState, embeds, firebase, osu};

#[poise::command(slash_command, rename = "suggest", subcommands("score"), required_permissions = "SEND_MESSAGES")]
pub async fn bundle(_ctx: Context<'_>, _arg: String) -> Result<(), Error> { Ok(()) }

/// Either submit score id or score file
#[poise::command(slash_command)]
pub async fn score(
    ctx: Context<'_>,
    #[description = "score id"] scoreid: Option<u64>,
    #[description = "score file"] scorefile: Option<serenity::Attachment>,
    // #[description = "reason"] reason: Option<String>,
) -> Result<(), Error> {
    let suggestion: CreateMessage;
    ctx.defer().await?;
    if scoreid.is_some() {
        let unwrapped_score_id = scoreid.unwrap();
        if firebase::score::score_already_saved(&unwrapped_score_id.to_string()).await {
            embeds::single_text_response(&ctx, &format!("Score {} has already been requested", unwrapped_score_id), MessageState::WARN, false).await;
            return Ok(());
        }
        let score: rosu::Score = match osu::get_osu_instance().score(unwrapped_score_id).await {
            Ok(score) => score,
            Err(_) => {
                embeds::single_text_response(&ctx, &format!("Score with id {} does not exist", unwrapped_score_id), MessageState::ERROR, false).await;
                return Ok(());
            }
        };

        if !score.has_replay {
            embeds::single_text_response(&ctx, "Score has no replay to download. Please provide the replay file", MessageState::ERROR, false).await;
            return Ok(());
        }
        let mut buttons: Vec<CreateButton> = vec![];
        let render_replay_id = format!("thumbnail:{}", score.id);
        let render_replay_button = serenity::CreateButton::new(render_replay_id)
            .label("Render Thumbnail")
            .emoji(crate::emojis::SATA_ANDAGI)
            .style(serenity::ButtonStyle::Primary);

        buttons.push(render_replay_button);

        if score.mode == rosu::GameMode::Osu {
            let upload_score_id = format!("upload:{}", score.id);
            let upload_score_button = serenity::CreateButton::new(upload_score_id)
            .label("Upload to youtube")
            .emoji(crate::emojis::SATA_ANDAGI)
            .style(serenity::ButtonStyle::Primary);
            buttons.push(upload_score_button);
        }
        let map = osu::get_osu_instance().beatmap().map_id(score.map_id).await.expect("Beatmap exists");
        let embed = embeds::score_embed_from_score(&score, &map).await?;


        suggestion = serenity::CreateMessage::new()
            .embed(embed.footer(serenity::CreateEmbedFooter::new(format!("Requested by @{}", ctx.author().name))))
            .components(vec![serenity::CreateActionRow::Buttons(buttons)]);

            
        firebase::score::insert_score(&unwrapped_score_id.to_string()).await;

    }
    else if scorefile.is_some() {
        let bytes = scorefile.unwrap().download().await?;
        let replay = match osu_db::Replay::from_bytes(&bytes) {
            Ok(replay) => replay,
            Err(_) => {
                embeds::single_text_response(&ctx, "Replay could not be parsed", MessageState::ERROR, false).await;
                return Ok(());
            },
        };
        let default_checksum = "".to_string();
        let replay_checksum = replay.replay_hash.as_ref().unwrap_or(&default_checksum);
        if firebase::score::score_already_saved(replay_checksum).await {
            embeds::single_text_response(&ctx, "Score file has already been requested", MessageState::WARN, false).await;
            return Ok(());
        }
        let map: rosu::BeatmapExtended = match osu::get_beatmap_from_checksum(&replay.beatmap_hash).await {
            Some(map) => map,
            None => {
                embeds::single_text_response(&ctx, "Cannot find map related to the replay", MessageState::WARN, false).await;
                return Ok(());
            },
        };
        let embed = embeds::score_embed_from_replay_file(&replay, &map).await?;
        suggestion = serenity::CreateMessage::new()
            .embed(embed.footer(serenity::CreateEmbedFooter::new(format!("Requested by @{}", ctx.author().name))))
            .add_file(CreateAttachment::bytes(bytes, "replay.osr"));

        firebase::score::insert_score(replay_checksum).await;
    }
    else {
        embeds::single_text_response(&ctx, "Please define scoreid or scorefile", MessageState::WARN, false).await;
        return Ok(());
    }

    defaults::SUGGESTIONS_CHANNEL.send_message(ctx, suggestion).await?;
    embeds::single_text_response(&ctx, "Score has been requested!", MessageState::INFO, false).await;
    Ok(())
}