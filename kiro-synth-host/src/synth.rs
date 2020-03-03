use ringbuf::Producer;

use kiro_synth_core::float::Float;
use kiro_synth_engine::event::{Event, Message};
use kiro_synth_engine::program::ParamRef;

pub struct SynthClient<F: Float> {
  events: Producer<Event<F>>,
}

impl<F: Float> SynthClient<F> {
  pub fn new(events: Producer<Event<F>>) -> Self {
    SynthClient {
      events
    }
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

  pub fn send_param_change(&mut self, param_ref: ParamRef, change: F) {
    let message = Message::ParamChange { param_ref, change };
    self.send_event(Event::new(0u64, message));
  }
}
