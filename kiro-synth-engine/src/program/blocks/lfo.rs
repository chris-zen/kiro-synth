use kiro_synth_dsp::oscillators::lfo::Lfo;

use crate::float::Float;
use crate::globals::SynthGlobals;
use crate::program::{Program, SignalRef};
use crate::signal::SignalBus;

#[derive(Debug, Clone)]
pub struct Inputs {
  pub shape: SignalRef,
  pub rate: SignalRef,
  pub phase: SignalRef,
  pub depth: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Block {
  pub inputs: Inputs,
  pub output: SignalRef,
}

#[derive(Debug)]
pub(crate) struct Processor<F: Float> {
  lfo: Lfo<F>,
  block: Block,
}

impl<F: Float> Processor<F> {
  pub fn new(sample_rate: F, block: Block) -> Self {
    let lfo = Lfo::new(sample_rate);

    Processor { lfo, block }
  }

  pub fn reset(&mut self) {
    self.lfo.reset()
  }

  pub fn process<'a>(
    &mut self,
    signals: &mut SignalBus<'a, F>,
    _program: &Program<F>,
    synth_globals: &SynthGlobals<F>,
  ) {
    let Block { inputs, output } = self.block.clone();
    let Inputs {
      shape,
      rate,
      phase,
      depth,
    } = inputs;

    signals[shape].if_updated(|value| {
      self.lfo.set_waveform(
        synth_globals
          .lfo_waveforms
          .waveform(value.to_usize().unwrap())
          .clone(),
      )
    });
    signals[rate].if_updated(|value| self.lfo.set_rate(value));
    signals[phase].if_updated(|value| self.lfo.set_phase(value));
    signals[depth].if_updated(|value| self.lfo.set_depth(value));

    signals[output].set(self.lfo.generate());
  }
}
