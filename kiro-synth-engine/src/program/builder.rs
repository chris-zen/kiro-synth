use heapless::Vec;

use crate::float::Float;
use crate::program::blocks::expr::{self, ExprBuilder, OpRef};
use crate::program::modulations::Modulations;
use crate::program::references::{BlockRef, ParamRef, SignalRef, SignalRefs, SourceRef};
use crate::program::{
  Block, MaxBlocks, MaxParams, MaxSources, Param, ParamBlock, ParamValues, Program, Source,
  VoiceBlock,
};
use crate::signal::Signal;

pub struct ProgramBuilder<'a, F: Float> {
  signal_refs: SignalRefs,
  voice: VoiceBlock,
  sources: Vec<Source<'a>, MaxSources>,
  params: Vec<Param<'a, F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
  modulations: Modulations<F>,
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
      modulations: Modulations::default(),
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
    let source = Source { id: name, signal };

    self.sources.push(source).unwrap();

    SourceRef(self.sources.len() - 1)
  }

  pub fn param(&mut self, id: &'a str, values: ParamValues<F>) -> ParamBlock {
    let out_signal_ref = self.signal_refs.create();
    let mod_signal_ref = self.signal_refs.create();

    let value = Signal::new(values.initial_value);
    let param = Param {
      id,
      values,
      value,
      out_signal_ref,
      mod_signal_ref,
    };

    self.params.push(param).unwrap();

    ParamBlock {
      reference: ParamRef(self.params.len() - 1),
      out_signal_ref,
      mod_signal_ref,
    }
  }

  pub fn signal(&mut self) -> SignalRef {
    self.signal_refs.create()
  }

  pub fn modulation<P: Into<ParamRef>>(&mut self, param: P, source_ref: SourceRef, amount: F) {
    self
      .modulations
      .update(param.into(), source_ref, amount)
      .unwrap();
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
      modulations: self.modulations,
    }
  }
}
