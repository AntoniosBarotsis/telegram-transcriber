use std::{process::Command, sync::OnceLock};

use log::{debug, error, info};
use teloxide::{net::Download, requests::Requester, types::Message, Bot};
use tokio::fs;

static CHAT_IDS: OnceLock<Vec<i64>> = OnceLock::new();
// TODO: Do something like this later down the line
//       for sending tasks to a worker thread that handles talk.
//       Task would need at least [msg.chat.id, msg.id, voice file name]
// static SENDER: OnceLock<Sender<Task>> = OnceLock::new();

#[tokio::main]
async fn main() {
  let _ = dotenvy::dotenv().expect(".env not found");

  // Run this once here to precompute the value.
  let _ = get_whitelisted_chat_ids_blocking();

  pretty_env_logger::init();

  info!("Starting throw dice bot...");

  let bot = Bot::from_env();

  teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    // If current chat id not contained in whitelisted chat ids list, return early
    if !get_whitelisted_chat_ids_blocking().contains(&msg.chat.id.0) {
      return Ok(());
    }

    if let Some(audio) = msg.voice() {
      debug!("Voice message detected");

      let file = bot.get_file(&audio.file.id).await?;
      // TODO: This needs to be random
      let mut dst = fs::File::create("./test.opus").await?;
      bot.download_file(&file.path, &mut dst).await?;
      info!("Audio file saved");

      audio_stuff();
    } else {
      debug!("No voice message detected, skipping");
    }

    // reply to original message
    let mut tmp = bot.send_dice(msg.chat.id);
    tmp.reply_to_message_id = Some(msg.id);
    let _ = tmp.await?;

    Ok(())
  })
  .await;
}

// TODO: Add a timeout
// TODO: Filename without extension as input (use in output, ulid?)
// TODO: Segment mp3 if >30s?
fn audio_stuff() {
  let out = Command::new("ffmpeg")
    .args(["-i", "test.opus", "test.wav", "-y"])
    .output()
    .expect("failed to execute process");

  if !out.status.success() {
    error!(
      "{}",
      String::from_utf8(out.stderr).expect("stderr was not valid utf8")
    );
  }
  println!("process finished with: {}", out.status);
}

fn get_whitelisted_chat_ids_blocking() -> &'static Vec<i64> {
  if CHAT_IDS.get().is_none() {
    let chat_ids = std::env::var("CHAT_IDS")
      .expect("CHAT_IDS is not defined")
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
