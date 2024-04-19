mod voice_file;
mod worker;

use std::{process::Command, sync::OnceLock};

use log::{debug, error, info};
use teloxide::{net::Download, requests::Requester, types::Message, Bot};
use tokio::fs;

static CHAT_IDS: OnceLock<Vec<i64>> = OnceLock::new();

use crate::{
  voice_file::VoiceFile,
  worker::{spawn_worker_thread, SENDER},
};

// TODO: Have a way of obtaining the chat id (mention perms in docs as well)
#[tokio::main]
async fn main() {
  let _ = dotenvy::dotenv().expect(".env not found");

  // Run this once here to precompute the value.
  let _ = get_whitelisted_chat_ids_blocking();

  pretty_env_logger::init();

  info!("Starting bot...");

  let bot = Bot::from_env();

  let _worker_handle = spawn_worker_thread(bot.clone());
  info!("Started worker thread");

  teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    // If current chat id not contained in whitelisted chat ids list, return early
    if !get_whitelisted_chat_ids_blocking().contains(&msg.chat.id.0) {
      return Ok(());
    }

    if let Some(voice) = msg.voice() {
      debug!("Voice message detected");

      let file = bot.get_file(&voice.file.id).await?;

      let voice_file = VoiceFile::new(msg.chat.id.0, msg.id.0);

      let path = voice_file.path_no_extension().with_extension("opus");

      // The path should always have the voice folder for its parent.
      let parent = path.parent().expect("Path had no parent");

      if !parent.exists() {
        debug!("Voice dir not found, creating");
        fs::create_dir(parent).await?;
      }

      let mut dst = fs::File::create(&path).await?;
      bot.download_file(&file.path, &mut dst).await?;
      debug!("Voice file saved");

      voice_stuff(voice_file);
    } else {
      debug!("No voice message detected, skipping");
    }

    Ok(())
  })
  .await;
}

fn voice_stuff(voice_file: VoiceFile) {
  let path_opus = voice_file.path_no_extension().with_extension("opus");
  let path_wav = path_opus.with_extension("wav");

  let out = Command::new("ffmpeg")
    .args([
      "-i",
      &path_opus.to_string_lossy(),
      "-ar",
      "16000",
      &path_wav.to_string_lossy(),
      "-y",
    ])
    .output()
    .expect("failed to execute process");

  if !out.status.success() {
    error!(
      "{}",
      String::from_utf8(out.stderr).expect("stderr was not valid utf8")
    );
  }

  let _handle = tokio::spawn(async move {
    if let Err(_e) = SENDER.get().expect("Sender should be set").send(voice_file) {
      error!("Error sending {}", path_wav.to_string_lossy());
    }
  });
}

fn get_whitelisted_chat_ids_blocking() -> &'static Vec<i64> {
  if CHAT_IDS.get().is_none() {
    let chat_ids = std::env::var("CHAT_IDS")
      .expect("CHAT_IDS is not set")
      .split(',')
      .map(|el| el.parse::<i64>().expect("chat id is a valid i64"))
      .collect::<Vec<_>>();

    let _ = CHAT_IDS.set(chat_ids);

    // Safety: At this point we know that CHAT_IDS is set
    #[allow(clippy::unwrap_used)]
    CHAT_IDS.get().unwrap()
  } else {
    // loop until it is set
    loop {
      if let Some(chat_ids) = CHAT_IDS.get() {
        return chat_ids;
      }
    }
  }
}
