//#![no_std]

#[macro_use]
extern crate hash32_derive;

#[macro_use]
mod signal;
mod key_freqs;
mod processor;
mod voice;

pub mod event;
pub mod globals;
pub mod program;
pub mod synth;
pub mod waveforms;

pub use kiro_synth_dsp::float;
