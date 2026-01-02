use std::env;

use futures_util::StreamExt;

use reqwest::Client;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{Error, embeds};
use crate::discord_helper::{ContextForFunctions, MessageState};
const URL: &str = "https://api.nerinyan.moe";

pub async fn download_mapset(cff: &ContextForFunctions<'_>, mapset_id: &u32) -> Result<(), Error> {
    let client = Client::new();
    let resp = client
        .get(format!("{}/d/{}", URL, mapset_id))
        .send()
        .await
        .unwrap()
        .error_for_status();

    let response = match resp {
        Ok(response) => response,
        Err(_) => {
            cff.edit(embeds::single_text_response_embed("nerinyan error: Mapset has not been found", MessageState::ERROR)).await?;
            return Ok(());
        }
    };

    let mut file = File::create(format!("{}/Songs/{}.osz", env::var("OSC_BOT_DANSER_PATH").expect("OSC_BOT_DANSER_PATH must exist"), mapset_id)).await?;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    Ok(())
}