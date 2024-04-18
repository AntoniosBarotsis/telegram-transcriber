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

    // TODO: Maybe it would be a good idea to  also look for messages that tag the bot and reply
    //       to a voice message (as a way of requesting transcriptions) but thats for later.
    if let Some(audio) = msg.voice() {
      debug!("Voice message detected");

      let file = bot.get_file(&audio.file.id).await?;

      let voice_file = VoiceFile::new(msg.chat.id.0, msg.id.0);
      let filename = voice_file.generate_opus_filename();

      let mut dst = fs::File::create(format!("./audio/{filename}")).await?;
      bot.download_file(&file.path, &mut dst).await?;
      debug!("Audio file saved");

      audio_stuff(&filename).await;
    } else {
      debug!("No voice message detected, skipping");
    }

    Ok(())
  })
  .await;
}

// TODO: Add a timeout?
async fn audio_stuff(filename: &str) {
  let path_old = format!("./audio/{filename}");
  let path_new = path_old.replace("opus", "wav");

  let out = Command::new("ffmpeg")
    .args(["-i", &path_old, "-ar", "16000", &path_new, "-y"])
    .output()
    .expect("failed to execute process");

  if !out.status.success() {
    error!(
      "{}",
      String::from_utf8(out.stderr).expect("stderr was not valid utf8")
    );
  }

  if let Err(e) = fs::remove_file(&path_old).await {
    error!(
      "Encountered error while trying to delete {}: {}",
      path_old, e
    );
  } else {
    debug!("Removed {}", path_old);

    // TODO: Move this to a separate function for readability
    // TODO: MAYBE instead of doing this I should instead use a single persistent bg thread to
    //       ensure I only make 1 request at a time (using channels) to not flood whisper
    let _handle = tokio::spawn(async move {
      if let Ok(voice_file) = VoiceFile::try_from(path_new.as_str()) {
        if let Err(e) = SENDER.get().expect("Sender should be set").send(voice_file) {
          error!("Error sending {}", e.0.generate_wav_filename());
        }
      } else {
        error!("Failed to parse {} as a voice file", path_new.as_str());
      }
    });
  }
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
