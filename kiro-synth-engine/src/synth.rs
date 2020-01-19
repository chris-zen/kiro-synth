use heapless::Vec;
use heapless::consts;
use typenum::marker_traits::Unsigned;
use ringbuf::Consumer;
use core::ops::Index;

use kiro_synth_core::oscillators::osc_waveform::OscWaveform;
use kiro_synth_core::waveforms::saw;

use crate::float::Float;
use crate::program::Program;
use crate::voice::Voice;
use crate::event::{Message, Event};

type MaxWaveforms = consts::U8;
type MaxVoices = consts::U16;

#[derive(Debug)]
pub struct SynthWaveforms<F: Float>(Vec<OscWaveform<F>, MaxWaveforms>);

impl<F: Float> SynthWaveforms<F> {
  pub fn new() -> Self {
    let mut waveforms: Vec<OscWaveform<F>, MaxWaveforms> = heapless::Vec::new();
    for _ in 0..MaxWaveforms::to_usize() {
      drop(waveforms.push(OscWaveform::Saw(saw::Saw::new(
        saw::Mode::Bipolar,
        saw::Correction::EightPointBlepWithInterpolation,
        saw::Saw::default_saturation()
      ))));
    }
    SynthWaveforms(waveforms)
  }
}

impl<F: Float> Index<usize> for SynthWaveforms<F> {
  type Output = OscWaveform<F>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

pub struct Synth<'a, F: Float> {
  sample_rate: F,
  events: Consumer<Event<F>>,
  program: Program<F>,
  voices: Vec<Voice<'a, F>, MaxVoices>,
  active_voices: Vec<usize, MaxVoices>,
  free_voices: Vec<usize, MaxVoices>,
}

impl<'a, F: Float> Synth<'a, F> {
  pub fn new(sample_rate: F, events: Consumer<Event<F>>, waveforms: &'a SynthWaveforms<F>, program: Program<F>) -> Self {
    let mut voices: Vec<Voice<'a, F>, MaxVoices> = Vec::new();
    let mut free_voices: Vec<usize, MaxVoices> = Vec::new();
    for index in 0..MaxVoices::to_usize() {
      drop(voices.push(Voice::new(sample_rate, waveforms, &program)));
      drop(free_voices.push(MaxVoices::to_usize() - index));
    }

    Synth {
      sample_rate,
      events,
      program,
      voices,
      active_voices: Vec::new(),
      free_voices,
    }
  }

  pub fn prepare(&mut self) {
    while let Some(Event { timestamp: _, message }) = self.events.pop() {
      match message {
        Message::NoteOn { key, velocity } => self.note_on(key, velocity),
        Message::NoteOff { key, velocity } => self.note_off(key, velocity),
      }
    }
  }

  fn note_on(&mut self, key: u8, velocity: F) {
    if let Some(index) = self.allocate_voice(key, velocity) {
      drop(self.active_voices.push(index));
      self.voices[index].note_on(key, velocity);
    }
  }

  fn note_off(&mut self, key: u8, _velocity: F) {
    if let Some(index) = self.take_active_voice(key) {
      drop(self.free_voices.push(index));
      self.voices[index].note_off();
    }
  }

  fn allocate_voice(&mut self, key: u8, velocity: F) -> Option<usize> {
    self.free_voices.pop()
  }

  fn take_active_voice(&mut self, key: u8) -> Option<usize> {
    self.active_voices.iter()
      .position(|index| {
        let voice = &self.voices[*index];
        voice.get_key() == key
      })
      .map(|pos| {
        self.active_voices.swap_remove(pos)
      })
  }

  pub fn process(&mut self) -> (F, F) {
    let (mut left, mut right) = (F::zero(), F::zero());

    for index in self.active_voices.iter() {
      let voice = &mut self.voices[*index];
      voice.process(&mut self.program);
      let (voice_left, voice_right) = voice.output();
      left = left + voice_left;
      right = right + voice_right;
    }

    self.program.reset_param_updates();

    (left, right)
  }
}
