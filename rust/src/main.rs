use std::process::Command;

use log::{debug, error, info};
use teloxide::{net::Download, requests::Requester, types::Message, Bot};
use tokio::fs;

// TODO: Have some sort of msg Q for resiliency
// TODO: Reply to telegram message with transcription (need to save msg_id to future web req)
#[tokio::main]
async fn main() {
  let _ = dotenvy::dotenv().expect(".env not found");
  pretty_env_logger::init();
  audio_stuff();

  info!("Starting throw dice bot...");

  let bot = Bot::from_env();

  teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    if let Some(audio) = msg.voice() {
      debug!("Voice message detected");

      // https://docs.rs/teloxide/latest/teloxide/net/trait.Download.html#tymethod.download_file_stream
      let file = bot.get_file(&audio.file.id).await?;
      let mut dst = fs::File::create("./test.opus").await?;
      bot.download_file(&file.path, &mut dst).await?;
      info!("Audio file saved");

      audio_stuff();
    } else {
      debug!("No voice message detected, skipping");
    }
    // bot.send_dice(msg.chat.id).await?;
    Ok(())
  })
  .await;
}

// TODO: Add a timeout
// TODO: Filename without extension as input (use in output, ulid?)
fn audio_stuff() {
  let out = Command::new("ffmpeg")
    .args(["-i", "test.opus", "test.wav", "-y"])
    .output()
    .expect("failed to execute process");

  if !out.status.success() {
    error!("{}", String::from_utf8(out.stderr).expect("valid utf8"));
  }
  println!("process finished with: {}", out.status);
}
