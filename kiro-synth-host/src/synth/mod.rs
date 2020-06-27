pub mod program;
mod audio_handler;
mod client;

pub use audio_handler::{SynthAudioHandler, SynthFeedback};
pub use client::{SynthClient, SynthClientMutex};