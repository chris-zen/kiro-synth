pub mod program;

use std::fmt::Formatter;
use std::sync::{Mutex, Arc, PoisonError, MutexGuard};

use ringbuf::{Producer, Consumer};

use kiro_synth_core::float::Float;
use kiro_synth_engine::event::{Event, Message};
use kiro_synth_engine::program::{ParamRef, SourceRef, MaxParams};
use kiro_synth_engine::globals::SynthGlobals;
use kiro_synth_engine::waveforms::{OscWaveforms, LfoWaveforms};
use kiro_synth_engine::synth::Synth;

use crate::audio::AudioHandler;
use generic_array::GenericArray;


#[derive(Debug, Clone)]
pub struct SynthFeedback {
  pub modulations: GenericArray<f32, MaxParams>
}

pub struct SynthAudioHandler<'a> {
  synth: Synth<'a, f32>,
  feedback: Producer<SynthFeedback>,
}

impl<'a> SynthAudioHandler<'a> {
  pub fn new(synth: Synth<'a, f32>, feedback: Producer<SynthFeedback>) -> Self {
    SynthAudioHandler {
      synth,
      feedback,
    }
  }
}

impl<'a> AudioHandler for SynthAudioHandler<'a> {
  fn prepare(&mut self, _len: usize) {
    self.synth.prepare();
  }

  fn next(&mut self) -> (f32, f32) {
    self.synth.process()
  }

  fn finalize(&mut self) {
    if let Some(index) = self.synth.get_active_voices().last() {
      if !self.feedback.is_full() {
        let mut modulations = GenericArray::default();

        let voice = &self.synth.get_voices()[*index];
        let signals = voice.get_signals();
        let program = self.synth.get_program();
        for (index, param) in program.get_params().iter().enumerate() {
          let signal_index: usize = param.mod_signal_ref.into();
          modulations[index] = signals[signal_index].get();
        }

        let feedback = SynthFeedback {
          modulations,
        };
        self.feedback.push(feedback).unwrap_or_default();
      }
    }
  }
}

pub struct SynthClient<F: Float> {
  globals: SynthGlobals<F>,
  events: Producer<Event<F>>,
  feedback: Consumer<SynthFeedback>,
}

impl<F: Float> SynthClient<F> {
  pub fn new(globals: SynthGlobals<F>,
             events: Producer<Event<F>>,
             feedback: Consumer<SynthFeedback>) -> Self {

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
    let message = Message::ModulationUpdate { source_ref, param_ref, amount };
    self.send_event(Event::new(0u64, message));
  }

  pub fn send_modulation_delete(&mut self, source_ref: SourceRef, param_ref: ParamRef) {
    let message = Message::ModulationDelete { source_ref, param_ref };
    self.send_event(Event::new(0u64, message));
  }
}

#[derive(Clone)]
pub struct SynthClientMutex<F: Float>(Arc<Mutex<SynthClient<F>>>);

impl<F: Float> SynthClientMutex<F> {
  pub fn new(mutex: Arc<Mutex<SynthClient<F>>>) -> Self {
    SynthClientMutex(mutex)
  }

  pub fn send_param_value(&self, param_ref: ParamRef, value: F) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self.0.lock()
        .map(|mut client| client.send_param_value(param_ref, value))
  }

  pub fn send_modulation_update(&self, source_ref: SourceRef, param_ref: ParamRef, amount: F) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self.0.lock()
        .map(|mut client| client.send_modulation_update(source_ref, param_ref, amount))
  }

  pub fn send_modulation_delete(&self, source_ref: SourceRef, param_ref: ParamRef) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self.0.lock()
        .map(|mut client| client.send_modulation_delete(source_ref, param_ref))
  }

  pub fn get_feedback(&mut self) -> Result<Option<SynthFeedback>, PoisonError<MutexGuard<'_, SynthClient<F>>>> {
    self.0.lock()
        .map(|mut client| client.feedback.pop())
  }
}

impl<F: Float> std::fmt::Debug for SynthClientMutex<F> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SynthClient")
  }
}
