use thiserror::Error;

#[cfg(target_os = "macos")]
mod coremidi;

#[cfg(target_os = "macos")]
pub use crate::midi::coremidi::{CoreMidiDriver as MidiDriver, CoreMidiError as MidiErrorSource};

use kiro_synth_midi::messages::Message;

#[derive(Error, Debug)]
pub enum MidiError {
  #[error("Error initialising the MIDI driver")]
  DriverInit(#[from] MidiErrorSource),
}

pub trait MidiHandler: Send {
  fn on_message(&mut self, timestamp: u64, message: Message);
  fn on_sysex(&mut self, timestamp: u64, data: &[u8]);
}
