pub mod osc;

use heapless::Vec;
use heapless::consts;

use crate::float::Float;
use crate::signal::Signal;

pub type MaxSignals = consts::U256;
pub type MaxParams = consts::U512;
pub type MaxBlocks = consts::U64;

#[derive(Debug, Clone, Copy)]
pub struct ParamRef(pub(crate) usize);

#[derive(Debug, Clone, Copy)]
pub struct SignalRef(pub(crate) usize);

#[derive(Debug, Clone)]
pub enum Block<F: Float> {
  Const {
    value: F,
    signal: SignalRef,
  },

  Param {
    param: ParamRef,
    signal: SignalRef,
  },

  Osc(osc::Block),

  Out {
    left: SignalRef,
    right: SignalRef,
  },
}

#[derive(Debug, Clone, Copy)]
pub struct BlockRef(pub(crate) usize);

#[derive(Debug, Clone)]
pub struct Program<F:Float> {
  signals_count: usize,
  params: Vec<Signal<F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
}

impl<F: Float> Program<F> {
  pub const NOTE_KEY_SIGNAL_REF: usize = 0;
  pub const NOTE_VELOCITY_SIGNAL_REF: usize = 1;
  pub const NOTE_PITCH_SIGNAL_REF: usize = 2;
  pub const OUTPUT_LEFT_SIGNAL_REF: usize = 3;
  pub const OUTPUT_RIGHT_SIGNAL_REF: usize = 4;
  pub const FIRST_FREE_SIGNAL_REF: usize = 5;

  pub fn new() -> Self {
    Program {
      signals_count: Self::FIRST_FREE_SIGNAL_REF,
      params: Vec::new(),
      blocks: Vec::new(),
    }
  }

  pub fn note_key() -> SignalRef {
    SignalRef(Self::NOTE_KEY_SIGNAL_REF)
  }

  pub fn note_velocity() -> SignalRef {
    SignalRef(Self::NOTE_VELOCITY_SIGNAL_REF)
  }

  pub fn note_pitch() -> SignalRef {
    SignalRef(Self::NOTE_PITCH_SIGNAL_REF)
  }

  pub fn output_left() -> SignalRef {
    SignalRef(Self::OUTPUT_LEFT_SIGNAL_REF)
  }

  pub fn output_right() -> SignalRef {
    SignalRef(Self::OUTPUT_RIGHT_SIGNAL_REF)
  }

  pub fn const_value(&mut self, value: F) -> SignalRef {
    let signal_ref = SignalRef(self.signals_count);
    self.signals_count += 1;
    let block_ref = BlockRef(self.blocks.len());
    drop(self.blocks.push(Block::Const { value, signal: signal_ref }));
    signal_ref
  }

  pub fn const_zero(&mut self) -> SignalRef {
    self.const_value(F::zero())
  }

  pub fn const_one(&mut self) -> SignalRef {
    self.const_value(F::one())
  }

  pub fn param(&mut self) -> SignalRef {
    let signal_ref = SignalRef(self.signals_count);
    self.signals_count += 1;
    let param_ref = ParamRef(self.params.len());
    drop(self.params.push(Signal::default()));
    let block_ref = BlockRef(self.blocks.len());
    drop(self.blocks.push(Block::Param { param: param_ref, signal: signal_ref }));
    signal_ref
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

  pub fn get_signals_count(&self) -> usize {
    self.signals_count
  }

  pub fn get_param(&self, param: ParamRef) -> &Signal<F> {
    &self.params[param.0]
  }

  pub fn get_param_mut(&mut self, param: ParamRef) -> &mut Signal<F> {
    &mut self.params[param.0]
  }

  pub fn get_params_count(&self) -> usize {
    self.params.len()
  }

  pub fn reset_param_updates(&mut self) {
    for param in self.params.iter_mut() {
      param.reset();
    }
  }

  pub fn get_blocks(&self) -> &[Block<F>] {
    &*self.blocks
  }
}

pub struct ProgramBuilder<F: Float> {
  signals_count: usize,
  params: Vec<Signal<F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
}
