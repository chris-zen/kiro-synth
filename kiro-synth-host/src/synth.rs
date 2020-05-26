use ringbuf::Producer;

use kiro_synth_core::float::Float;
use kiro_synth_engine::event::{Event, Message};
use kiro_synth_engine::program::ParamRef;
use kiro_synth_engine::globals::SynthGlobals;
use kiro_synth_engine::waveforms::{OscWaveforms, LfoWaveforms};
use std::fmt::Formatter;

pub struct SynthClient<F: Float> {
  globals: SynthGlobals<F>,
  events: Producer<Event<F>>,
}

impl<F: Float> std::fmt::Debug for SynthClient<F> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SynthClient")
  }
}

impl<F: Float> SynthClient<F> {
  pub fn new(globals: SynthGlobals<F>, events: Producer<Event<F>>) -> Self {
    SynthClient {
      globals,
      events
    }
  }

  pub fn osc_waveforms(&self) -> &OscWaveforms<F> {
    &self.globals.osc_waveforms
  }

  pub fn lfo_waveforms(&self) -> &LfoWaveforms<F> {
    &self.globals.lfo_waveforms
  }

  pub fn send_event(&mut self, event: Event<F>) {
    drop(self.events.push(event));
  }

  pub fn send_note_on(&mut self, key: u8, velocity: F) {
    let message = Message::NoteOn { key, velocity };
    self.send_event(Event::new(0u64, message));
  }

  pub fn send_note_off(&mut self, key: u8, velocity: F) {
    let message = Message::NoteOff { key, velocity };
    self.send_event(Event::new(0u64, message));
  }

  pub fn send_param_value(&mut self, param_ref: ParamRef, value: F) {
    let message = Message::Param { param_ref, value };
    self.send_event(Event::new(0u64, message));
  }

  #[allow(dead_code)]
  pub fn send_param_change(&mut self, param_ref: ParamRef, change: F) {
    let message = Message::ParamChange { param_ref, change };
    self.send_event(Event::new(0u64, message));
  }
}
