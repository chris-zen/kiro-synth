use thiserror::Error;

use ::cpal::{BuildStreamError, DefaultStreamConfigError, PlayStreamError};

mod config;
mod cpal;

pub use crate::config::AudioConfig;
pub use crate::cpal::AudioDriver;

type Result<T> = core::result::Result<T, AudioError>;

#[derive(Error, Debug)]
pub enum AudioError {
  #[error("No default output device")]
  NoDefaultOutputDevice,

  #[error("No default stream config")]
  NoDefaultStreamConfig(#[from] DefaultStreamConfigError),

  #[error("Error building stream")]
  BuildStream(#[from] BuildStreamError),

  #[error("Error playing stream")]
  PlayStream(#[from] PlayStreamError),
}

pub trait AudioHandler: Send {
  fn process(&mut self, data: &mut [f32], channels: usize);
}
