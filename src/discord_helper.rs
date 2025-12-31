use poise::serenity_prelude::{self as serenity, CacheHttp};

use crate::{Data, Error, embeds::single_text_response};
use crate::defaults::{REPLAY_ROLE, SERVER};

#[derive(PartialEq)]
pub enum MessageState {
    SUCCESS,
    WARN,
    ERROR,
    INFO,
}

pub async fn handle_error(error: poise::FrameworkError<'_, Data, Error>) -> () {
    match error {
        poise::FrameworkError::CommandCheckFailed { .. } => return (),
        _ => ()
    };

    match error.ctx() {
        Some(ctx) => {
            single_text_response(&ctx, "Something went wrong. blame Mikumin.", MessageState::ERROR, false).await;
        }
        None => return ()
    };
    println!("{:?}", error)
}

pub async fn user_has_replay_role(ctx: impl CacheHttp, user: &serenity::User) -> Result<bool, Error> {
    let member = SERVER.member(ctx, user).await.unwrap();
    if !member.roles.contains(&REPLAY_ROLE) {
        return Ok(false);
    }
    Ok(true)
}