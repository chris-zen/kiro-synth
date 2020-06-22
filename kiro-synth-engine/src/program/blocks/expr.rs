use heapless::Vec;
use heapless::consts;

use crate::float::Float;
use crate::program::{SignalRef, ParamRef, Program, ProgramBuilder};
use crate::signal::SignalBus;

type MaxOps = consts::U8;

#[derive(Debug, Clone)]
pub struct OpRef(usize);

#[derive(Debug, Clone)]
pub enum Op<F: Float> {
  Value(F),
  Param(ParamRef),
  Signal(SignalRef),
  Neg(OpRef),
  Add(OpRef, OpRef),
  AddValue(OpRef, F),
  AddParam(OpRef, ParamRef),
  AddSignal(OpRef, SignalRef),
  AddSignals(SignalRef, SignalRef),
  AddSignalValue(SignalRef, F),
  AddSignalParam(SignalRef, ParamRef),
  Mul(OpRef, OpRef),
  MulValue(OpRef, F),
  MulParam(OpRef, ParamRef),
  MulSignal(OpRef, SignalRef),
  MulSignals(SignalRef, SignalRef),
  MulSignalValue(SignalRef, F),
  MulSignalParam(SignalRef, ParamRef),
}

#[derive(Debug, Clone)]
pub struct Block<F: Float> {
  ops: Vec<Op<F>, MaxOps>,
  pub output: SignalRef,
}

pub struct ExprBuilder<F: Float> {
  ops: Vec<Op<F>, MaxOps>,
}

impl<F: Float> ExprBuilder<F> {
  pub fn new() -> Self {
    ExprBuilder {
      ops: Vec::new()
    }
  }

  pub fn value(&mut self, v: F) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::Value(v)));
    op_ref
  }

  pub fn param(&mut self, param_ref: ParamRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::Param(param_ref)));
    op_ref
  }

  pub fn signal(&mut self, signal_ref: SignalRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::Signal(signal_ref)));
    op_ref
  }

  pub fn neg(&mut self, input_ref: OpRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::Neg(input_ref)));
    op_ref
  }

  pub fn add(&mut self, x_ref: OpRef, y_ref: OpRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::Add(x_ref, y_ref)));
    op_ref
  }

  pub fn add_value(&mut self, x_ref: OpRef, value: F) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::AddValue(x_ref, value)));
    op_ref
  }

  pub fn add_param(&mut self, x_ref: OpRef, param: ParamRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::AddParam(x_ref, param)));
    op_ref
  }

  pub fn add_signal(&mut self, x_ref: OpRef, signal: SignalRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::AddSignal(x_ref, signal)));
    op_ref
  }

  pub fn add_signals(&mut self, signal1: SignalRef, signal2: SignalRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::AddSignals(signal1, signal2)));
    op_ref
  }

  pub fn add_signal_value(&mut self, signal: SignalRef, value: F) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::AddSignalValue(signal, value)));
    op_ref
  }

  pub fn add_signal_param(&mut self, signal: SignalRef, param: ParamRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::AddSignalParam(signal, param)));
    op_ref
  }

  pub fn mul(&mut self, x_ref: OpRef, y_ref: OpRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::Mul(x_ref, y_ref)));
    op_ref
  }

  pub fn mul_value(&mut self, x_ref: OpRef, value: F) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::MulValue(x_ref, value)));
    op_ref
  }

  pub fn mul_param(&mut self, x_ref: OpRef, param: ParamRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::MulParam(x_ref, param)));
    op_ref
  }

  pub fn mul_signal(&mut self, x_ref: OpRef, signal: SignalRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::MulSignal(x_ref, signal)));
    op_ref
  }

  pub fn mul_signals(&mut self, signal1: SignalRef, signal2: SignalRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::MulSignals(signal1, signal2)));
    op_ref
  }

  pub fn mul_signal_value(&mut self, signal: SignalRef, value: F) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::MulSignalValue(signal, value)));
    op_ref
  }

  pub fn mul_signal_param(&mut self, signal: SignalRef, param: ParamRef) -> OpRef {
    let op_ref = self.next_ref();
    drop(self.ops.push(Op::MulSignalParam(signal, param)));
    op_ref
  }

  pub fn build(self, program_builder: &mut ProgramBuilder<F>) -> Block<F> {
    Block {
      ops: self.ops,
      output: program_builder.signal(),
    }
  }

  fn next_ref(&self) -> OpRef {
    OpRef(self.ops.len())
  }
}

#[derive(Debug)]
pub(crate) struct Processor<F: Float> {
  block: Block<F>
}

impl<F: Float> Processor<F> {
  pub fn new(block: Block<F>) -> Self {
    Processor {
      block
    }
  }

  pub fn reset(&mut self) {}

  pub fn process<'b>(&mut self, signals: &mut SignalBus<'b, F>, program: &Program<F>) {
    let mut stack = Vec::<F, MaxOps>::new();
    for op in self.block.ops.iter() {
      match op {
        Op::Value(value) => {
          drop(stack.push(*value))
        },
        Op::Param(param_ref) => {
          let param_value = program.get_param_signal(*param_ref).get();
          drop(stack.push(param_value))
        },
        Op::Signal(signal_ref) => {
          let signal_value = signals[*signal_ref].get();
          drop(stack.push(signal_value))
        },
        Op::Neg(_) => {
          let x = stack.pop().unwrap();
          drop(stack.push(-x));
        },
        Op::Add(_, _) => {
          let x = stack.pop().unwrap();
          let y = stack.pop().unwrap();
          drop(stack.push(x + y));
        },
        Op::AddValue(_, value) => {
          let x = stack.pop().unwrap();
          drop(stack.push(x + *value));
        },
        Op::AddParam(_, param_ref) => {
          let x = stack.pop().unwrap();
          let param_value = program.get_param_signal(*param_ref).get();
          drop(stack.push(x + param_value));
        },
        Op::AddSignal(_, signal_ref) => {
          let x = stack.pop().unwrap();
          let signal_value = signals[*signal_ref].get();
          drop(stack.push(x + signal_value));
        },
        Op::AddSignals(signal_ref1, signal_ref2) => {
          let signal_value1 = signals[*signal_ref1].get();
          let signal_value2 = signals[*signal_ref2].get();
          drop(stack.push(signal_value1 + signal_value2));
        },
        Op::AddSignalValue(signal_ref, value) => {
          let signal_value = signals[*signal_ref].get();
          drop(stack.push(signal_value + *value));
        },
        Op::AddSignalParam(signal_ref, param_ref) => {
          let signal_value = signals[*signal_ref].get();
          let param_value = program.get_param_signal(*param_ref).get();
          drop(stack.push(signal_value + param_value));
        },
        Op::Mul(_, _) => {
          let x = stack.pop().unwrap();
          let y = stack.pop().unwrap();
          drop(stack.push(x * y));
        },
        Op::MulValue(_, value) => {
          let x = stack.pop().unwrap();
          drop(stack.push(x * *value));
        },
        Op::MulParam(_, param_ref) => {
          let x = stack.pop().unwrap();
          let param_value = program.get_param_signal(*param_ref).get();
          drop(stack.push(x * param_value));
        },
        Op::MulSignal(_, signal_ref) => {
          let x = stack.pop().unwrap();
          let signal_value = signals[*signal_ref].get();
          drop(stack.push(x * signal_value));
        },
        Op::MulSignals(signal_ref1, signal_ref2) => {
          let signal_value1 = signals[*signal_ref1].get();
          let signal_value2 = signals[*signal_ref2].get();
          drop(stack.push(signal_value1 * signal_value2));
        },
        Op::MulSignalValue(signal_ref, value) => {
          let signal_value = signals[*signal_ref].get();
          drop(stack.push(signal_value * *value));
        },
        Op::MulSignalParam(signal_ref, param_ref) => {
          let signal_value = signals[*signal_ref].get();
          let param_value = program.get_param_signal(*param_ref).get();
          drop(stack.push(signal_value * param_value));
        },
      }
    }
    signals[self.block.output].set(stack.pop().unwrap());
  }
}