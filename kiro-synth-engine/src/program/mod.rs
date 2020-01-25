pub mod osc;
pub mod expr;

use heapless::Vec;
use heapless::consts;

use crate::float::Float;
use crate::signal::Signal;
use std::ops::{Deref, DerefMut};

pub type MaxSignals = consts::U256;
pub type MaxParams = consts::U512;
pub type MaxBlocks = consts::U64;

#[derive(Debug, Clone)]
pub struct ParamValues<F: Float> {
  pub initial_value: F,
  pub min: F,
  pub max: F,
  pub resolution: F,
}

impl<F: Float> ParamValues<F> {
  pub fn with_initial_value(self, initial_value: F) -> Self {
    Self {
      initial_value,
      .. self
    }
  }
}

#[derive(Debug, Clone)]
pub struct ParamDesc<'a> {
  pub id: &'a str,
  pub name: &'a str,
}

impl<'a> ParamDesc<'a> {
  pub const fn new(id: &'a str, name: &'a str) -> Self {
    ParamDesc {
      id,
      name,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Param<'a, F: Float> {
  pub id: &'a str,
  pub name: &'a str,
  pub values: ParamValues<F>,
  pub signal: Signal<F>,
}

#[derive(Debug, Clone)]
pub struct ParamBlock {
  pub reference: ParamRef,
  pub signal: SignalRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct ParamRef(pub usize);

impl From<ParamBlock> for ParamRef {
  fn from(block: ParamBlock) -> Self {
    block.reference
  }
}

#[derive(Debug, Clone, Copy)]
pub struct SignalRef(pub(crate) usize);

#[derive(Debug, Clone)]
pub enum Block<F: Float> {
  Const {
    value: F,
    signal: SignalRef,
  },

  Param(ParamBlock),

  Osc(osc::Block),

  Expr(expr::Block<F>),

  Out {
    left: SignalRef,
    right: SignalRef,
  },
}

#[derive(Debug, Clone, Copy)]
pub struct BlockRef(pub(crate) usize);

#[derive(Debug, Clone)]
pub struct Program<'a, F:Float> {
  signals_count: usize,
  params: Vec<Param<'a, F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
}

impl<'a, F: Float> Program<'a, F> {
  pub const NOTE_KEY_SIGNAL_REF: usize = 0;
  pub const NOTE_VELOCITY_SIGNAL_REF: usize = 1;
  pub const NOTE_PITCH_SIGNAL_REF: usize = 2;
  pub const OUTPUT_LEFT_SIGNAL_REF: usize = 3;
  pub const OUTPUT_RIGHT_SIGNAL_REF: usize = 4;
  pub const FIRST_FREE_SIGNAL_REF: usize = 5;

  pub fn get_signals_count(&self) -> usize {
    self.signals_count
  }

//  pub fn get_params_count(&self) -> usize {
//    self.params.len()
//  }

  pub fn get_params(&self) -> &[Param<'a, F>] {
    self.params.deref()
  }

  pub fn get_params_mut(&mut self) -> &mut [Param<'a, F>] {
    self.params.deref_mut()
  }

  pub fn get_param<R: Into<ParamRef>>(&self, param: R) -> Option<(ParamRef, &Param<'a, F>)> {
    let param_ref = param.into();
    self.params.get(param_ref.0).map(|param| (param_ref, param))
  }

  pub fn get_param_mut<R: Into<ParamRef>>(&mut self, param: R) -> Option<(ParamRef, &mut Param<'a, F>)> {
    let param_ref = param.into();
    self.params.get_mut(param_ref.0).map(|param| (param_ref, param))
  }

//  pub fn get_param_by_id(&self, id: &str) -> Option<(usize, &Param<'a, F>)> {
//    self.params.iter()
//        .position(|param| param.id == id)
//        .map(|param_index| (param_index, &self.params[param_index]))
//  }

//  pub fn get_param_ref(&self, id: &'a str) -> Option<ParamRef> {
//    self.params.iter()
//        .position(|param| param.id == id)
//        .map(|param_index| ParamRef(param_index))
//  }

//  pub fn get_param_value(&mut self, index: usize) -> F {
//    self.params.get(index)
//        .map_or(F::zero(), |param| param.signal.get())
//  }
//
//  pub fn set_param_value(&mut self, param_ref: ParamRef, value: F) {
//    if let Some(param) = self.params.get_mut(param_ref.0) {
//      param.signal.set(value)
//    }
//  }

  pub fn get_param_signal(&self, param: ParamRef) -> &Signal<F> {
    &self.params[param.0].signal
  }

  pub fn get_param_signal_mut(&mut self, param: ParamRef) -> &mut Signal<F> {
    &mut self.params[param.0].signal
  }

  pub fn reset_params(&mut self) {
    for param in self.params.iter_mut() {
      param.signal.set(param.values.initial_value);
      param.signal.reset();
    }
  }

  pub fn update_params(&mut self) {
    for param in self.params.iter_mut() {
      param.signal.update_state();
    }
  }

  pub fn get_blocks(&self) -> &[Block<F>] {
    &*self.blocks
  }
}

pub struct ProgramBuilder<'a, F: Float> {
  signals_count: usize,
  params: Vec<Param<'a, F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
}

impl<'a, F: Float> ProgramBuilder<'a, F> {

  pub fn new() -> Self {
    ProgramBuilder {
      signals_count: Program::<F>::FIRST_FREE_SIGNAL_REF,
      params: Vec::new(),
      blocks: Vec::new(),
    }
  }

  pub fn note_key() -> SignalRef {
    SignalRef(Program::<F>::NOTE_KEY_SIGNAL_REF)
  }

  pub fn note_velocity() -> SignalRef {
    SignalRef(Program::<F>::NOTE_VELOCITY_SIGNAL_REF)
  }

  pub fn note_pitch() -> SignalRef {
    SignalRef(Program::<F>::NOTE_PITCH_SIGNAL_REF)
  }

  pub fn output_left() -> SignalRef {
    SignalRef(Program::<F>::OUTPUT_LEFT_SIGNAL_REF)
  }

  pub fn output_right() -> SignalRef {
    SignalRef(Program::<F>::OUTPUT_RIGHT_SIGNAL_REF)
  }

  pub fn const_value(&mut self, value: F) -> SignalRef {
    let signal = SignalRef(self.signals_count);
    self.signals_count += 1;
    let block_ref = BlockRef(self.blocks.len());
    drop(self.blocks.push(Block::Const { value, signal }));
    signal
  }

  pub fn const_zero(&mut self) -> SignalRef {
    self.const_value(F::zero())
  }

  pub fn const_one(&mut self) -> SignalRef {
    self.const_value(F::one())
  }

  pub fn param(&mut self, id: &'a str, name: &'a str, values: ParamValues<F>) -> ParamBlock {
    let initial_value = values.initial_value;
    let param = Param {
      id,
      name,
      values,
      signal: Signal::new(initial_value)
    };

    let param_ref = ParamRef(self.params.len());
    drop(self.params.push(param));

    let signal_ref = SignalRef(self.signals_count);
    self.signals_count += 1;

    let param_block = ParamBlock {
      reference: param_ref,
      signal: signal_ref,
    };

    drop(self.blocks.push(Block::Param(param_block.clone())));

    param_block
  }

  pub fn signal(&mut self) -> SignalRef {
    let signal_ref = SignalRef(self.signals_count);
    self.signals_count += 1;
    signal_ref
  }

  pub fn block(&mut self, block: Block<F>) -> BlockRef {
    let block_ref = BlockRef(self.blocks.len());
    drop(self.blocks.push(block));
    block_ref
  }

  pub fn out(&mut self, left: SignalRef, right: SignalRef) -> BlockRef {
    let block_ref = BlockRef(self.blocks.len());
    drop(self.blocks.push(Block::Out { left: left.clone(), right: right.clone() }));
    block_ref
  }

  pub fn build(self) -> Program<'a, F> {
    Program {
      signals_count: self.signals_count,
      params: self.params,
      blocks: self.blocks,
    }
  }
}