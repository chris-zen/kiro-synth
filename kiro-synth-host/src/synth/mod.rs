mod audio_handler;
mod client;
pub mod program;

pub use audio_handler::{SynthAudioHandler, SynthFeedback};
pub use client::{SynthClient, SynthClientMutex};
