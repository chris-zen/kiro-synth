use kiro_synth_core::oscillators::pitched_oscillator::PitchedOscillator;

use crate::float::Float;
use crate::program::{SignalRef, Program};
use crate::synth::{SynthWaveforms, SynthGlobals};
use crate::signal::SignalBus;
use kiro_synth_core::oscillators::osc_waveform::OscWaveform;


#[derive(Debug, Clone)]
pub struct Inputs {
  pub shape: SignalRef,
  pub amplitude: SignalRef,
  pub amp_mod: SignalRef,
  pub octave: SignalRef,
  pub semitones: SignalRef,
  pub cents: SignalRef,
  pub note_pitch: SignalRef,
  pub pitch_bend: SignalRef,
  pub freq_mod: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Block {
  pub inputs: Inputs,
  pub output: SignalRef,
}

#[derive(Debug)]
pub(crate) struct Processor<F: Float> {
  osc: PitchedOscillator<F>,
  block: Block,
}

impl<F: Float> Processor<F> {

  pub fn new(sample_rate: F, block: Block) -> Self {
    let waveform = OscWaveform::default();
    let osc = PitchedOscillator::new(sample_rate, waveform, F::zero());

    Processor {
      osc,
      block,
    }
  }

  pub fn reset(&mut self) {}

  pub fn process<'a>(&mut self,
                     signals: &mut SignalBus<'a, F>,
                     _program: &Program<F>,
                     synth_globals: &SynthGlobals<F>) {

    let Block { inputs, output } = self.block.clone();
    let Inputs { shape, amplitude, amp_mod,
                 octave, semitones, cents,
                 note_pitch, pitch_bend, freq_mod } = inputs;

    signals[shape].if_updated(|value| self.osc.set_waveform(synth_globals.waveforms[value.to_usize().unwrap()].clone()));
    signals[amplitude].if_updated(|value| self.osc.set_amplitude(value));
    signals[amp_mod].if_updated(|value| self.osc.set_amplitude_modulation(value));
    signals[octave].if_updated(|value| self.osc.set_octaves(value));
    signals[semitones].if_updated(|value| self.osc.set_semitones(value));
    signals[cents].if_updated(|value| self.osc.set_cents(value));
    signals[note_pitch].if_updated(|value| self.osc.set_pitch_frequency(value));
    signals[pitch_bend].if_updated(|value| self.osc.set_pitch_bend(value));
    signals[freq_mod].if_updated(|value| self.osc.set_frequency_modulation(value));

    signals[output].set(self.osc.generate());
  }
}
