use std::{sync::OnceLock, thread::JoinHandle};

use crate::voice_file::VoiceFile;

pub static SENDER: OnceLock<Sender> = OnceLock::new();

pub type Sender = std::sync::mpsc::Sender<VoiceFile>;
pub type Receiver = std::sync::mpsc::Receiver<VoiceFile>;

use log::{debug, error};
use reqwest::blocking::multipart;
use teloxide::{
  requests::Requester,
  types::{ChatId, MessageId},
  Bot,
};
use tokio::fs;

fn setup() -> Receiver {
  let (tx, rx) = std::sync::mpsc::channel();
  SENDER.set(tx).expect("Should only be called once");

  rx
}

#[allow(clippy::unwrap_used)]
pub fn spawn_worker_thread(bot: Bot) -> JoinHandle<()> {
  let whisper_url = std::env::var("WHISPER_URL").expect("WHISPER_URL is not set");

  let receiver = setup();

  let handle = std::thread::spawn(move || {
    let client = reqwest::blocking::Client::new();

    debug!("Listening for messages");

    loop {
      if let Ok(voice_file) = receiver.recv() {
        debug!("Message received");

        let path = voice_file.path_no_extension().with_extension("wav");

        let form = multipart::Form::new().file("file", &path).unwrap();

        // TODO: Let the user know of server errors instead of crashing
        debug!("Sending request");
        let res = client
          .post(whisper_url.clone())
          .multipart(form)
          .send()
          .unwrap()
          .text()
          .unwrap();

        let transcribed = &res[1..res.len() - 1].to_owned();

        let bot_clone = bot.clone();
        let future = async move {
          debug!("{:?}", res);

          let mut tmp = bot_clone.send_message(ChatId(voice_file.chat_id), transcribed);
          tmp.reply_to_message_id = Some(MessageId(voice_file.msg_id));
          let _ = tmp.await;

          // TODO: Make sure this is always run (ie don't exit early for errors unless fatal)
          if let Err(e) = fs::remove_file(&path).await {
            error!(
              "Encountered error while trying to delete {}: {}",
              path.to_string_lossy(),
              e
            );
          } else {
            debug!("Deleted {}", path.to_string_lossy());
          }
        };

        tokio::runtime::Builder::new_multi_thread()
          .enable_all()
          .build()
          .unwrap()
          .block_on(future);
      }
    }
  });

  handle
}
