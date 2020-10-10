use std::fmt::Formatter;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use ringbuf::{Consumer, Producer};

use kiro_synth_dsp::float::Float;
use kiro_synth_engine::event::{Event, Message};
use kiro_synth_engine::globals::SynthGlobals;
use kiro_synth_engine::program::{ParamRef, SourceRef};
use kiro_synth_engine::waveforms::{LfoWaveforms, OscWaveforms};

use crate::synth::SynthFeedback;

pub struct SynthClient<F: Float> {
  globals: SynthGlobals<F>,
  events: Producer<Event<F>>,
  feedback: Consumer<SynthFeedback>,
}

impl<F: Float> SynthClient<F> {
  pub fn new(
    globals: SynthGlobals<F>,
    events: Producer<Event<F>>,
    feedback: Consumer<SynthFeedback>,
  ) -> Self {
    SynthClient {
      globals,
      events,
      feedback,
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
    let message = Message::ParamValue { param_ref, value };
    self.send_event(Event::new(0u64, message));
  }

  #[allow(dead_code)]
  pub fn send_param_change(&mut self, param_ref: ParamRef, change: F) {
    let message = Message::ParamChange { param_ref, change };
    self.send_event(Event::new(0u64, message));
  }

  pub fn send_modulation_update(&mut self, source_ref: SourceRef, param_ref: ParamRef, amount: F) {
    let message = Message::ModulationUpdate {
      source_ref,
      param_ref,
      amount,
    };
    self.send_event(Event::new(0u64, message));
  }

  pub fn send_modulation_delete(&mut self, source_ref: SourceRef, param_ref: ParamRef) {
    let message = Message::ModulationDelete {
      source_ref,
      param_ref,
    };
    self.send_event(Event::new(0u64, message));
  }
}

#[derive(Clone)]
pub struct SynthClientMutex<F: Float>(Arc<Mutex<SynthClient<F>>>);

impl<F: Float> SynthClientMutex<F> {
  pub fn new(mutex: Arc<Mutex<SynthClient<F>>>) -> Self {
    SynthClientMutex(mutex)
  }

  pub fn send_param_value(
    &self,
    param_ref: ParamRef,
    value: F,
  ) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self
      .0
      .lock()
      .map(|mut client| client.send_param_value(param_ref, value))
  }

  pub fn send_modulation_update(
    &self,
    source_ref: SourceRef,
    param_ref: ParamRef,
    amount: F,
  ) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self
      .0
      .lock()
      .map(|mut client| client.send_modulation_update(source_ref, param_ref, amount))
  }

  pub fn send_modulation_delete(
    &self,
    source_ref: SourceRef,
    param_ref: ParamRef,
  ) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self
      .0
      .lock()
      .map(|mut client| client.send_modulation_delete(source_ref, param_ref))
  }

  pub fn get_feedback(
    &mut self,
  ) -> Result<Option<SynthFeedback>, PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self.0.lock().map(|mut client| client.feedback.pop())
  }
}

impl<F: Float> std::fmt::Debug for SynthClientMutex<F> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SynthClient")
  }
}
