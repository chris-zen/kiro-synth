use crate::float::Float;
use crate::globals::SynthGlobals;
use crate::program::blocks::*;
use crate::program::{Block, ParamBlock, ParamRef, Program, SignalRef};
use crate::signal::SignalBus;

#[derive(Debug)]
pub(crate) enum Processor<F: Float> {
  Const(F, SignalRef),
  Param(ParamRef),
  DCA(dca::Processor<F>),
  EG(envgen::Processor<F>),
  Expr(expr::Processor<F>),
  Filter(filter::Processor<F>),
  Lfo(lfo::Processor<F>),
  Osc(osc::Processor<F>),
  Out(SignalRef, SignalRef),
}

impl<F: Float> Processor<F> {
  pub fn new(sample_rate: F, block: &Block<F>) -> Self {
    match block.clone() {
      Block::Const { value, signal } => Processor::Const(value, signal),
      Block::Param(ParamBlock {
        reference,
        out_signal_ref: _,
        mod_signal_ref: _,
      }) => Processor::Param(reference),
      Block::DCA(dca_block) => Processor::DCA(dca::Processor::new(sample_rate, dca_block)),
      Block::EG(eg_block) => Processor::EG(envgen::Processor::new(sample_rate, eg_block)),
      Block::Lfo(lfo_block) => Processor::Lfo(lfo::Processor::new(sample_rate, lfo_block)),
      Block::Osc(osc_block) => Processor::Osc(osc::Processor::new(sample_rate, osc_block)),
      Block::Expr(expr_block) => Processor::Expr(expr::Processor::new(expr_block)),
      Block::Filter(filt_block) => {
        Processor::Filter(filter::Processor::new(sample_rate, filt_block))
      }
      Block::Out { left, right } => Processor::Out(left, right),
    }
  }

  pub fn reset(&mut self) {
    match self {
      Processor::Const(_, _) => {}
      Processor::Param(_) => {}
      Processor::DCA(ref mut proc) => proc.reset(),
      Processor::EG(ref mut proc) => proc.reset(),
      Processor::Expr(ref mut proc) => proc.reset(),
      Processor::Filter(ref mut proc) => proc.reset(),
      Processor::Lfo(ref mut proc) => proc.reset(),
      Processor::Osc(ref mut proc) => proc.reset(),
      Processor::Out(ref _left, ref _right) => {}
    }
  }

  pub fn process<'b>(
    &mut self,
    signals: &mut SignalBus<'b, F>,
    program: &mut Program<F>,
    synth_globals: &SynthGlobals<F>,
  ) {
    match self {
      Processor::Const(value, signal) => signals[*signal].set(*value),
      Processor::Param(param_ref) => {
        if let Some((_, param)) = program.get_param(*param_ref) {
          let mut value = F::zero();
          for modulation in program.get_param_modulations(*param_ref) {
            if let Some(source) = program.get_source(modulation.source_ref) {
              let source_signal = signals[source.signal].get();
              value = value + source_signal * modulation.amount;
            }
          }
          signals[param.mod_signal_ref].set(value);
          value = value + param.value.get();
          value = value.max(param.values.min).min(param.values.max);
          signals[param.out_signal_ref].set(value);
        }
      }
      Processor::DCA(ref mut proc) => proc.process(signals, program),
      Processor::EG(ref mut proc) => proc.process(signals, program),
      Processor::Expr(ref mut proc) => proc.process(signals, program),
      Processor::Filter(ref mut proc) => proc.process(signals, program),
      Processor::Lfo(ref mut proc) => proc.process(signals, program, synth_globals),
      Processor::Osc(ref mut proc) => proc.process(signals, program, synth_globals),
      Processor::Out(ref left, ref right) => {
        let voice = program.voice();
        let left_value = signals[*left].consume();
        signals[voice.output_left].set(left_value);
        let right_value = signals[*right].consume();
        signals[voice.output_right].set(right_value);
      }
    }
  }
}
