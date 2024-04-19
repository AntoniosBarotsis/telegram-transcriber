use std::path::{Path, PathBuf};

use anyhow::anyhow;

static AUDIO_DIR: &str = "./audio";

#[derive(Debug, Clone, Copy)]
pub struct VoiceFile {
  pub chat_id: i64,
  pub msg_id: i32,
}

impl VoiceFile {
  pub const fn new(chat_id: i64, msg_id: i32) -> Self {
    Self { chat_id, msg_id }
  }

  pub fn path_no_extension(&self) -> PathBuf {
    Path::new(AUDIO_DIR).join(format!("{}_{}", self.chat_id, self.msg_id))
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
