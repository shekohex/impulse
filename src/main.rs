#![deny(unsafe_code)]

use anyhow::{anyhow, Result};
use argh::FromArgs;
use std::{env, process};

mod constants;
mod onesignal;
use onesignal::{Contents, Headings, NotificationPayload};

/// Send push notification when your long build command finish
#[derive(Debug, FromArgs)]
struct Args {
    /// the command that we will execute.
    #[argh(positional)]
    cmd: String,
    /// override the UserIDs that we will send notification to.
    /// normally this will be stored in `IMPULSE_USER_IDS` env.
    /// UIDs is separated by `,`.
    #[argh(option)]
    uids: Option<String>,
    /// set the success message.
    /// default to: Build exit successfully
    #[argh(option, short = 's')]
    success_message: Option<String>,
    /// set the error message.
    /// default to: Build errored
    #[argh(option, short = 'e')]
    error_message: Option<String>,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let uids = env::var("IMPULSE_USER_IDS")
        .ok()
        .or(args.uids)
        .ok_or_else(|| anyhow!(constants::MISSING_USERID_ERR))?;
    let result = exec(&args.cmd)?;
    let msg = if result {
        args.success_message
            .unwrap_or_else(|| String::from("Build exit successfully"))
    } else {
        args.error_message
            .unwrap_or_else(|| String::from("Build errored"))
    };
    onesignal::send_notification(NotificationPayload {
        app_id: constants::ONE_SIGNAL_APP_ID,
        include_player_ids: uids.split(',').map(|v| v.to_owned()).collect(),
        headings: Headings {
            en: "Build Result".to_string(),
        },
        contents: Contents { en: msg },
        chrome_web_image: String::new(),
        web_url: String::new(),
    })?;
    Ok(())
}

pub fn exec(cmd: &str) -> Result<bool> {
    let mut cmd = cmd.split_whitespace();
    let mut child = process::Command::new(cmd.next().unwrap_or_default())
        .args(cmd)
        .spawn()?;
    Ok(child.wait()?.success())
}
