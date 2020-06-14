pub mod dca;
pub mod envgen;
pub mod expr;
pub mod filter;
pub mod lfo;
pub mod osc;

use heapless::Vec;
use heapless::consts;

use crate::float::Float;
use crate::signal::Signal;
use std::ops::{Deref, DerefMut};
use crate::program::expr::{ExprBuilder, OpRef};

pub type MaxSignals = consts::U256;
pub type MaxSources = consts::U32;
pub type MaxModulators = consts::U4;
pub type MaxParams = consts::U128;
pub type MaxBlocks = consts::U128;

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct SignalRef(pub(crate) usize);

#[derive(Debug, Clone)]
pub struct Source<'a> {
  pub id: &'a str,
  pub signal: SignalRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct SourceRef(pub(crate) usize);

impl SourceRef {
  pub fn new(reference: usize) -> Self {
    SourceRef(reference)
  }
}

impl Into<usize> for SourceRef {
  fn into(self) -> usize {
    self.0
  }
}

#[derive(Debug, Clone)]
pub struct Modulator<F: Float> {
  pub source: SourceRef,
  pub amount: F,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct ParamRef(pub(crate) usize);

impl ParamRef {
  pub fn new(reference: usize) -> Self {
    ParamRef(reference)
  }
}

impl From<ParamBlock> for ParamRef {
  fn from(block: ParamBlock) -> Self {
    block.reference
  }
}

impl From<&ParamBlock> for ParamRef {
  fn from(block: &ParamBlock) -> Self {
    block.reference
  }
}

impl Into<usize> for ParamRef {
  fn into(self) -> usize {
    self.0
  }
}

#[derive(Debug, Clone)]
pub struct ParamValues<F: Float> {
  pub initial_value: F,
  pub origin: F,
  pub min: F,
  pub max: F,
  pub resolution: F,
  // pub mod_min: F,
  // pub mod_max: F,
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
pub struct Param<'a, F: Float> {
  pub id: &'a str,
  pub values: ParamValues<F>,
  pub signal: Signal<F>,
  pub modulators: Vec<Modulator<F>, MaxModulators>,
}

#[derive(Debug, Clone)]
pub struct ParamBlock {
  pub reference: ParamRef,
  pub signal: SignalRef,
}

#[derive(Debug, Clone)]
pub enum Block<F: Float> {
  Const {
    value: F,
    signal: SignalRef,
  },

  Param(ParamBlock),

  DCA(dca::Block),

  EG(envgen::Block),

  Expr(expr::Block<F>),

  Filter(filter::Block),

  Lfo(lfo::Block),

  Osc(osc::Block),

  Out {
    left: SignalRef,
    right: SignalRef,
  },
}

#[derive(Debug, Clone, Copy)]
pub struct BlockRef(pub(crate) usize);

#[derive(Debug, Clone)]
pub struct VoiceBlock {
  pub key: SignalRef,
  pub velocity: SignalRef,
  pub note_pitch: SignalRef,
  pub gate: SignalRef,
  pub trigger: SignalRef,
  pub off: SignalRef,
  pub output_left: SignalRef,
  pub output_right: SignalRef,
}

struct SignalRefs(usize);

impl SignalRefs {
  pub fn new() -> Self {
    SignalRefs(0)
  }

  pub fn create(&mut self) -> SignalRef {
    let reference = SignalRef(self.0);
    self.0 += 1;
    reference
  }

  pub fn count(&self) -> usize {
    self.0
  }
}

#[derive(Debug, Clone)]
pub struct Program<'a, F:Float> {
  signals_count: usize,
  voice: VoiceBlock,
  sources: Vec<Source<'a>, MaxSources>,
  params: Vec<Param<'a, F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
}

impl<'a, F: Float> Program<'a, F> {

  pub fn get_signals_count(&self) -> usize {
    self.signals_count
  }

  pub fn voice(&self) -> &VoiceBlock {
    &self.voice
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

  pub fn get_sources(&self) -> &[Source<'a>] {
    self.sources.deref()
  }

  pub fn get_source(&self, source: SourceRef) -> Option<&Source<'a>> {
    self.sources.get(source.0)
  }

  pub fn get_blocks(&self) -> &[Block<F>] {
    &*self.blocks
  }
}

pub struct ProgramBuilder<'a, F: Float> {
  signal_refs: SignalRefs,
  voice: VoiceBlock,
  sources: Vec<Source<'a>, MaxSources>,
  params: Vec<Param<'a, F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
}

impl<'a, F: Float> ProgramBuilder<'a, F> {

  pub fn new() -> Self {
    let mut signal_refs = SignalRefs::new();

    let voice = VoiceBlock {
      key: signal_refs.create(),
      velocity: signal_refs.create(),
      note_pitch: signal_refs.create(),
      gate: signal_refs.create(),
      trigger: signal_refs.create(),
      off: signal_refs.create(),
      output_left: signal_refs.create(),
      output_right: signal_refs.create(),
    };

    ProgramBuilder {
      signal_refs,
      voice,
      sources: Vec::new(),
      params: Vec::new(),
      blocks: Vec::new(),
    }
  }

  pub fn voice(&self) -> &VoiceBlock {
    &self.voice
  }

  pub fn const_value(&mut self, value: F) -> SignalRef {
    let signal = self.signal_refs.create();
    self.blocks.push(Block::Const { value, signal }).unwrap();
    signal
  }

  pub fn const_zero(&mut self) -> SignalRef {
    self.const_value(F::zero())
  }

  pub fn const_one(&mut self) -> SignalRef {
    self.const_value(F::one())
  }

  pub fn source(&mut self, name: &'a str, signal: SignalRef) -> SourceRef {
    let source = Source {
      id: name,
      signal,
    };

    self.sources.push(source).unwrap();

    SourceRef(self.sources.len() - 1)
  }

  pub fn param(&mut self, id: &'a str, values: ParamValues<F>) -> ParamBlock {
    let initial_value = values.initial_value;
    let param = Param {
      id,
      values,
      signal: Signal::new(initial_value),
      modulators: Vec::new(),
    };

    self.params.push(param).unwrap();

    ParamBlock {
      reference: ParamRef(self.params.len() - 1),
      signal: self.signal_refs.create(),
    }
  }

  pub fn signal(&mut self) -> SignalRef {
    self.signal_refs.create()
  }

  pub fn modulation<P: Into<ParamRef>>(&mut self, param: P, source: SourceRef, amount: F) {
    let param_index = param.into().0;
    let modulator = Modulator {
      source,
      amount
    };
    self.params[param_index].modulators.push(modulator).unwrap();
  }

  pub fn expr<B: Fn(&mut ExprBuilder<F>) -> OpRef>(&mut self, build_expr: B) -> expr::Block<F> {
    let mut expr_builder = ExprBuilder::new();
    build_expr(&mut expr_builder);
    expr_builder.build(self)
  }

  pub fn block(&mut self, block: Block<F>) -> BlockRef {
    let block_ref = BlockRef(self.blocks.len());
    self.blocks.push(block).unwrap();
    block_ref
  }

  pub fn out(&mut self, left: SignalRef, right: SignalRef) -> BlockRef {
    let block_ref = BlockRef(self.blocks.len());
    self.blocks.push(Block::Out { left, right }).unwrap();
    block_ref
  }

  pub fn build(self) -> Program<'a, F> {
    Program {
      signals_count: self.signal_refs.count(),
      voice: self.voice,
      sources: self.sources,
      params: self.params,
      blocks: self.blocks,
    }
  }
}