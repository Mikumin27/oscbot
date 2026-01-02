use std::fs::remove_file;
use std::io::Cursor;
use std::process::Stdio;
use std::env;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use zip::ZipArchive;

use tokio::{fs::{File, create_dir}, io::AsyncWriteExt};

use crate::discord_helper::ContextForFunctions;
use crate::{Error, embeds};

pub async fn render(cff: &ContextForFunctions<'_>, title: &String, beatmap_hash: &String, replay_reference: &String, user_id: &u32) -> Result<String, Error> {
    let skin_path = &format!("{}/Skins/{}", env::var("OSC_BOT_DANSER_PATH").unwrap(), user_id);
    let replay_path = &format!("{}/Replays/{}/{}.osr", env::var("OSC_BOT_DANSER_PATH").unwrap(), beatmap_hash, replay_reference);
    let mut out = Command::new("danser-cli");
    out.args(["-replay", replay_path, "-record"]);
    if Path::new(skin_path).is_dir() {
        out.args(["-skin", &user_id.to_string()]);
    }
    let mut danser_terminal = out.stdout(Stdio::piped()).spawn()?;
    let stdout = danser_terminal.stdout.take().unwrap();

    let mut lines = BufReader::new(stdout).lines();

    while let Some(line) = lines.next_line().await? {
        match line {
            _ if line.contains("Progress") => {
                let first_version = line.split_once("Progress: ").map(|(_, rest)| rest).unwrap();
                let final_version = first_version.split_once(",").map(|(rest, _)| rest).unwrap();
                cff.edit(embeds::render_and_upload_embed(title, true, Some(final_version.to_string()), false)?
                ).await?;
                println!("{}", final_version);
            }
            _ if line.contains("Video is available at: ") => {
                _ = danser_terminal.wait();
                let rest = line.split_once("Video is available at: ").map(|(_, rest)| rest).unwrap();
                return Ok(rest.replace("\\", "/").trim().to_string())
            }
            _ => ()
        };
    }

    Err("Video could not be rendered".into())
}

pub async fn attach_replay(beatmap_hash: &String, replay_reference: &String, bytes: &Vec<u8>) -> Result<(), Error> {
    let replay_path = &format!("{}/Replays/{}", env::var("OSC_BOT_DANSER_PATH").unwrap(), beatmap_hash);
    if !Path::new(replay_path).is_dir() {
        create_dir(&replay_path).await?;
    }

    let mut file = File::create(format!("{}/{}.osr", replay_path, replay_reference)).await?;
    file.write_all(bytes).await?;
    file.flush().await?;
    Ok(())
}

pub async fn cleanup_files(beatmap_hash: &String, replay_reference: &String, video_path: &String) {
    let replay_path = &format!("{}/Replays/{}/{}.osr", env::var("OSC_BOT_DANSER_PATH").unwrap(), beatmap_hash, replay_reference);
    _ = remove_file(replay_path);
    _ = remove_file(video_path);
}

pub async fn _attach_skin_file(user_id: u32, bytes: &Vec<u8>) -> Result<(), Error> {
    let path = &format!("{}/Skins/{}", env::var("OSC_BOT_DANSER_PATH").unwrap(), user_id);
    
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    _ = zip.extract(path);
    Ok(())
}