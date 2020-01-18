use ringbuf::Consumer;

use kiro_synth_core::waveforms::Waveform;
use kiro_synth_core::waveforms::saw::{self, Saw};
use kiro_synth_midi::messages::Message as MidiMessage;

use crate::audio::AudioHandler;

pub const KEY_FREQ: [f32; 128] = [
  8.176,      8.662,      9.177,      9.723,     10.301,     10.913,     11.562,     12.250,     12.978,     13.750,     14.568,     15.434,
  16.352,     17.324,     18.354,     19.445,     20.602,     21.827,     23.125,     24.500,     25.957,     27.500,     29.135,     30.868,
  32.703,     34.648,     36.708,     38.891,     41.203,     43.654,     46.249,     48.999,     51.913,     55.000,     58.270,     61.735,
  65.406,     69.296,     73.416,     77.782,     82.407,     87.307,     92.499,     97.999,    103.826,    110.000,    116.541,    123.471,
  130.813,    138.591,    146.832,    155.563,    164.814,    174.614,    184.997,    195.998,    207.652,    220.000,    233.082,    246.942,
  261.626,    277.183,    293.665,    311.127,    329.628,    349.228,    369.994,    391.995,    415.305,    440.000,    466.164,    493.883,
  523.251,    554.365,    587.330,    622.254,    659.255,    698.456,    739.989,    783.991,    830.609,    880.000,    932.328,    987.767,
  1046.502,   1108.731,   1174.659,   1244.508,   1318.510,   1396.913,   1479.978,   1567.982,   1661.219,   1760.000,   1864.655,   1975.533,
  2093.005,   2217.461,   2349.318,   2489.016,   2637.020,   2793.826,   2959.955,   3135.963,   3322.438,   3520.000,   3729.310,   3951.066,
  4186.009,   4434.922,   4698.636,   4978.032,   5274.041,   5587.652,   5919.911,   6271.927,   6644.875,   7040.000,   7458.620,   7902.133,
  8372.018,   8869.844,   9397.273,   9956.063,  10548.082,  11175.303,  11839.822,  12543.854];

pub struct Synth {
  sample_rate: f32,
  midi_consumer: Consumer<MidiMessage>,

  note_on: bool,
  note_freq: f32,
  note_velocity: f32,

  saw1: Saw<f32>,
  modulo1: f32,
  phase_inc1: f32,

  saw2: Saw<f32>,
  modulo2: f32,
  phase_inc2: f32,
  freq_scale2: f32,
}

impl Synth {
  pub fn new(sample_rate: f32, midi_consumer: Consumer<MidiMessage>) -> Self {
    let f0 = 440f32 * 0.25;

    let saw1 = Saw::new(
      saw::Mode::Bipolar,
      saw::Correction::EightPointBlepWithInterpolation,
      1.5,
    );

    //  let mut saw1 = save_waveform(f0, sample_rate, saw1)?;

    let saw2 = Saw::new(
      saw::Mode::Bipolar,
      saw::Correction::EightPointBlepWithInterpolation,
      1.5,
    );

    Synth {
      sample_rate,
      midi_consumer,
      note_on: false,
      note_freq: 0.0,
      note_velocity: 1.0,
      saw1,
      modulo1: 0.0,
      phase_inc1: 0.0,
      saw2,
      modulo2: 0.0,
      phase_inc2: 0.0,
      freq_scale2: 1.0,
    }
  }

  fn modulo_inc(modulo: f32, phase_inc: f32) -> f32 {
    let new_modulo = modulo + phase_inc;
    if new_modulo >= 1.0 {
      new_modulo - 1.0
    } else {
      new_modulo
    }
  }

  fn reset(&mut self) {
    self.modulo1 = 0f32;
    self.modulo2 = 0f32;
    self.phase_inc1 = self.note_freq / self.sample_rate;
    self.phase_inc2 = self.freq_scale2 * self.note_freq / self.sample_rate;
  }

  fn process_midi(&mut self) {
    while let Some(message) = self.midi_consumer.pop() {
      match message {
        MidiMessage::NoteOn { channel: _, key, velocity } => {
          self.note_on = true;
          self.note_freq = KEY_FREQ[key as usize];
          self.note_velocity = velocity as f32 / 127.0;
          self.reset();
        }
        MidiMessage::NoteOff { channel: _, key: _, velocity: _ } => {
          self.note_on = false;
        }
        MidiMessage::ControlChange { channel: _, controller, value } => {
          match controller {
            18 => {
              self.freq_scale2 = 2.0 * value as f32 / 127.0;
              self.phase_inc2 = self.freq_scale2 * self.note_freq / self.sample_rate;
            },
            _ => {}
          }
        }
        _ => {}
      }
    };
  }
}

impl AudioHandler for Synth {
  fn prepare(&mut self, len: usize) {
    self.process_midi();
  }

  fn next(&mut self) -> (f32, f32) {
    if self.note_on {
      let signal1 = self.saw1.generate(self.modulo1, self.phase_inc1);
      let signal2 = self.saw2.generate(self.modulo2, self.phase_inc2);
      self.modulo1 = Self::modulo_inc(self.modulo1, self.phase_inc1);
      self.modulo2 = Self::modulo_inc(self.modulo2, self.phase_inc2);

      let signal = self.note_velocity * 0.2 * (signal1 + signal2) / 2.0;
      (signal, signal)
    }
    else {
      (0.0, 0.0)
    }
  }
}
