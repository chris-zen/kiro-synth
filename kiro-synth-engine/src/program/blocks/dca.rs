use kiro_synth_core::dca::DCA;
use kiro_synth_core::float::Float;

use crate::program::{Program, SignalRef};
use crate::signal::SignalBus;

#[derive(Debug, Clone)]
pub struct Inputs {
  pub left: SignalRef,
  pub right: SignalRef,
  pub velocity: SignalRef,
  pub amplitude: SignalRef,
  pub amp_mod: SignalRef,
  pub eg_mod: SignalRef,
  pub pan: SignalRef,
  pub pan_mod: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Outputs {
  pub left: SignalRef,
  pub right: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Block {
  pub inputs: Inputs,
  pub outputs: Outputs,
}

#[derive(Debug)]
pub(crate) struct Processor<F: Float> {
  dca: DCA<F>,
  block: Block,
}

impl<F: Float> Processor<F> {
  pub fn new(_sample_rate: F, block: Block) -> Self {
    Processor {
      dca: DCA::new(),
      block,
    }
  }

  pub fn reset(&mut self) {}

  pub fn process<'a>(&mut self, signals: &mut SignalBus<'a, F>, _program: &Program<F>) {
    let Block { inputs, outputs } = self.block.clone();
    let Inputs {
      left,
      right,
      velocity,
      amplitude,
      amp_mod,
      eg_mod,
      pan,
      pan_mod,
    } = inputs;
    let Outputs {
      left: left_output,
      right: right_output,
    } = outputs;

    signals[velocity].if_updated(|value| self.dca.set_velocity(value));
    signals[amplitude].if_updated(|value| self.dca.set_amplitude_db(value));
    signals[amp_mod].if_updated(|value| self.dca.set_amp_mod_db(value));
    signals[eg_mod].if_updated(|value| self.dca.set_eg_mod(value));
    signals[pan].if_updated(|value| self.dca.set_pan(value));
    signals[pan_mod].if_updated(|value| self.dca.set_pan_mod(value));

    let left_in = signals[left].get();
    let right_in = signals[right].get();
    let (left_out, right_out) = self.dca.process(left_in, right_in);
    signals[left_output].set(left_out);
    signals[right_output].set(right_out);
  }
}
