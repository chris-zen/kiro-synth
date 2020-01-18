use core::ops::DerefMut;
use heapless::Vec;

use crate::float::Float;
use crate::key_freqs::KEY_FREQ;
use crate::program::{MaxSignals, MaxBlocks, Block, osc, Program, SignalRef, ParamRef};
use crate::synth::SynthWaveforms;
use crate::signal::{Signal, SignalBus};
use crate::voice::Voice;

#[derive(Debug)]
pub(crate) enum Processor<'a, F: Float> {
  Const(F, SignalRef),
  Param(ParamRef, SignalRef),
  Osc(osc::Processor<'a, F>),
  Out(SignalRef, SignalRef),
}

impl<'a, F: Float> Processor<'a, F> {
  pub fn new(sample_rate: F, waveforms: &'a SynthWaveforms<F>, block: &Block<F>) -> Self {
    match block.clone() {
      Block::Const { value, signal } => Processor::Const(value, signal),
      Block::Param { param, signal } => Processor::Param(param, signal),
      Block::Osc(osc_block) => Processor::Osc(osc::Processor::new(sample_rate, waveforms, osc_block)),
      Block::Out { left, right } => Processor::Out(left, right),
    }
  }

  pub fn process<'b>(&mut self, signals: &mut SignalBus<'b, F>, program: &Program<F>) {
    match self {
      Processor::Const(value, signal) => signals[*signal].set(*value),
      Processor::Param(param, signal) => program.get_param(*param).if_updated(|value| signals[*signal].set(value)),
      Processor::Osc(ref mut proc) => proc.process(signals, program),
      Processor::Out(ref left, ref right) => {
        let left_value = signals[left].get();
        signals[Program::<F>::output_left()].set(left_value);
        let right_value = signals[right].get();
        signals[Program::<F>::output_right()].set(right_value);
      }
    }
  }
}
