//#![no_std]

#[macro_use]
extern crate hash32_derive;

#[macro_use]
mod signal;
mod processor;
mod voice;
mod key_freqs;

pub mod program;
pub mod event;
pub mod waveforms;
pub mod globals;
pub mod synth;

pub use kiro_synth_core::float;
