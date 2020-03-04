use crate::float::Float;
use crate::program::{Block, Program, SignalRef, ParamRef, ParamBlock};
use crate::program::{dca, envgen, expr, filter, osc};
use crate::synth::SynthGlobals;
use crate::signal::SignalBus;

#[derive(Debug)]
pub(crate) enum Processor<F: Float> {
  Const(F, SignalRef),
  Param(ParamRef, SignalRef),
  DCA(dca::Processor<F>),
  EG(envgen::Processor<F>),
  Expr(expr::Processor<F>),
  Filter(filter::Processor<F>),
  Osc(osc::Processor<F>),
  Out(SignalRef, SignalRef),
}

impl<F: Float> Processor<F> {
  pub fn new(sample_rate: F, block: &Block<F>) -> Self {
    match block.clone() {
      Block::Const { value, signal } => Processor::Const(value, signal),
      Block::Param(ParamBlock { reference, signal }) => Processor::Param(reference, signal),
      Block::DCA(dca_block) => Processor::DCA(dca::Processor::new(sample_rate, dca_block)),
      Block::EG(eg_block) => Processor::EG(envgen::Processor::new(sample_rate, eg_block)),
      Block::Osc(osc_block) => Processor::Osc(osc::Processor::new(sample_rate, osc_block)),
      Block::Expr(expr_block) => Processor::Expr(expr::Processor::new(expr_block)),
      Block::Filter(filt_block) => Processor::Filter(filter::Processor::new(sample_rate, filt_block)),
      Block::Out { left, right } => Processor::Out(left, right),
    }
  }

  pub fn reset(&mut self) {
    match self {
      Processor::Const(_value, _signal) => {},
      Processor::Param(_param, _signal) => {},
      Processor::DCA(ref mut proc) => proc.reset(),
      Processor::EG(ref mut proc) => proc.reset(),
      Processor::Expr(ref mut proc) => proc.reset(),
      Processor::Filter(ref mut proc) => proc.reset(),
      Processor::Osc(ref mut proc) => proc.reset(),
      Processor::Out(ref _left, ref _right) => {},
    }
  }

  pub fn process<'b>(&mut self,
                     signals: &mut SignalBus<'b, F>,
                     program: &mut Program<F>,
                     synth_globals: &SynthGlobals<F>) {
    match self {
      Processor::Const(value, signal) => signals[*signal].set(*value),
      Processor::Param(param, signal) => {
        program.get_param_signal_mut(*param).if_updated(|value| signals[*signal].set(value))
      },
      Processor::DCA(ref mut proc) => proc.process(signals, program),
      Processor::EG(ref mut proc) => proc.process(signals, program),
      Processor::Expr(ref mut proc) => proc.process(signals, program),
      Processor::Filter(ref mut proc) => proc.process(signals, program),
      Processor::Osc(ref mut proc) => proc.process(signals, program, synth_globals),
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
