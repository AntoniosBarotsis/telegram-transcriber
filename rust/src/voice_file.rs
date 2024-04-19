use std::{
  fs::remove_file,
  path::{Path, PathBuf},
};

use anyhow::anyhow;
use log::{debug, error};

static VOICE_DIR: &str = "./audio";

/// Represents a voice file on disk. Both the `wav` and `opus` files will automatically be deleted
/// when the `VoiceFile` instance is dropped.
#[derive(Debug, Clone)]
pub struct VoiceFile {
  pub chat_id: i64,
  pub msg_id: i32,
}

impl VoiceFile {
  pub const fn new(chat_id: i64, msg_id: i32) -> Self {
    Self { chat_id, msg_id }
  }

  pub fn path_no_extension(&self) -> PathBuf {
    Path::new(VOICE_DIR).join(format!("{}_{}", self.chat_id, self.msg_id))
  }
}

impl TryFrom<&PathBuf> for VoiceFile {
  type Error = anyhow::Error;

  fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
    let value = value
      .file_stem()
      .ok_or_else(|| anyhow!("weird path error"))?
      .to_string_lossy();

    let split = value.split('_').collect::<Vec<_>>();

    if split.len() != 2 {
      return Err(anyhow!("Invalid filename '{value}'"));
    }

    let chat_id = split[0];
    let msg_id = split[1];

    let chat_id = chat_id.parse::<i64>()?;
    let msg_id = msg_id.parse::<i32>()?;

    Ok(Self { chat_id, msg_id })
  }
}

impl Drop for VoiceFile {
  fn drop(&mut self) {
    let path = self.path_no_extension();

    let opus = path.with_extension("opus");
    let wav = path.with_extension("wav");

    let opus_res = remove_file(&opus).map(|()| debug!("Removed {}", opus.to_string_lossy()));
    let wav_res = remove_file(&wav).map(|()| debug!("Removed {}", wav.to_string_lossy()));

    if opus_res.is_err() && wav_res.is_err() {
      error!("Failed to remove file {}", path.to_string_lossy());
    }
  }
}
