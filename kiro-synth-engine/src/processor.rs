use heapless::Vec;

use crate::float::Float;
use crate::key_freqs::KEY_FREQ;
use crate::program::{MaxSignals, MaxBlocks, Block, Program, SignalRef, ParamRef, ProgramBuilder, ParamBlock};
use crate::program::{dca, envgen, expr, osc};
use crate::synth::SynthWaveforms;
use crate::signal::{Signal, SignalBus};
use crate::voice::Voice;

#[derive(Debug)]
pub(crate) enum Processor<'a, F: Float> {
  Const(F, SignalRef),
  Param(ParamRef, SignalRef),
  DCA(dca::Processor<F>),
  EG(envgen::Processor<F>),
  Expr(expr::Processor<F>),
  Osc(osc::Processor<'a, F>),
  Out(SignalRef, SignalRef),
}

impl<'a, F: Float> Processor<'a, F> {
  pub fn new(sample_rate: F, waveforms: &'a SynthWaveforms<F>, block: &Block<F>) -> Self {
    match block.clone() {
      Block::Const { value, signal } => Processor::Const(value, signal),
      Block::Param(ParamBlock { reference, signal }) => Processor::Param(reference, signal),
      Block::DCA(dca_block) => Processor::DCA(dca::Processor::new(sample_rate, dca_block)),
      Block::EG(eg_block) => Processor::EG(envgen::Processor::new(sample_rate, eg_block)),
      Block::Osc(osc_block) => Processor::Osc(osc::Processor::new(sample_rate, waveforms, osc_block)),
      Block::Expr(expr_block) => Processor::Expr(expr::Processor::new(expr_block)),
      Block::Out { left, right } => Processor::Out(left, right),
    }
  }

  pub fn process<'b>(&mut self, signals: &mut SignalBus<'b, F>, program: &mut Program<F>) {
    match self {
      Processor::Const(value, signal) => {
        signals[*signal].set(*value)
      },
      Processor::Param(param, signal) => {
        program.get_param_signal_mut(*param).if_updated(|value| signals[*signal].set(value))
      },
      Processor::DCA(ref mut proc) => {
        proc.process(signals, program)
      },
      Processor::EG(ref mut proc) => {
        proc.process(signals, program)
      },
      Processor::Expr(ref mut proc) => {
        proc.process(signals, program)
      },
      Processor::Osc(ref mut proc) => {
        proc.process(signals, program)
      },
      Processor::Out(ref left, ref right) => {
        let voice = program.voice();
        let left_value = signals[left].consume();
        signals[voice.output_left].set(left_value);
        let right_value = signals[right].consume();
        signals[voice.output_right].set(right_value);
      }
    }
  }
}
