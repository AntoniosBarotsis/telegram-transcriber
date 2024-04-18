use std::path::Path;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
pub struct VoiceFile {
  pub chat_id: i64,
  pub msg_id: i32,
}

impl VoiceFile {
  pub const fn new(chat_id: i64, msg_id: i32) -> Self {
    Self { chat_id, msg_id }
  }
  pub fn generate_opus_filename(&self) -> String {
    format!("{}_{}.opus", self.chat_id, self.msg_id)
  }

  pub fn generate_wav_filename(&self) -> String {
    format!("{}_{}.wav", self.chat_id, self.msg_id)
  }
}

impl TryFrom<&str> for VoiceFile {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let value = Path::new(value)
      .file_stem()
      .ok_or_else(|| anyhow!("weird path error"))?
      .to_str()
      .ok_or_else(|| anyhow!("weird path error"))?;

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
