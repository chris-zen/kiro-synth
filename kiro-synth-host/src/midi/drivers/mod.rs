use thiserror::Error;

use kiro_midi_core::messages::Message;

#[cfg(target_os = "macos")]
mod coremidi;

#[cfg(target_os = "macos")]
pub use crate::midi::drivers::coremidi::{
  CoreMidiDriver as MidiDriver, CoreMidiError as MidiErrorSource,
};

#[cfg(target_os = "linux")]
mod jackmidi;

#[cfg(target_os = "linux")]
pub use crate::midi::drivers::jackmidi::{
  JackMidiDriver as MidiDriver, JackMidiError as MidiErrorSource,
};

#[derive(Error, Debug)]
pub enum MidiError {
  #[error("Error initialising the MIDI driver")]
  DriverInit(#[from] MidiErrorSource),
}

pub trait MidiHandler: Send {
  fn on_message(&mut self, timestamp: u64, message: Message);
  fn on_sysex(&mut self, timestamp: u64, data: &[u8]);
}
