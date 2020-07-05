pub mod blocks;
pub mod builder;
pub mod modulations;
pub mod references;

use std::ops::{Deref, DerefMut};

use heapless::consts;
use heapless::Vec;

use crate::float::Float;
use crate::signal::Signal;

use blocks::*;
use modulations::Modulations;
pub use builder::ProgramBuilder;
pub use references::*;

pub type MaxSignals = consts::U256;
pub type MaxSources = consts::U32;
pub type MaxModulations = consts::U4;
pub type MaxParams = consts::U128;
pub type MaxBlocks = consts::U128;

#[derive(Debug, Clone)]
pub struct Source<'a> {
  pub id: &'a str,
  pub signal: SignalRef,
}

#[derive(Debug, Clone)]
pub struct ParamValues<F: Float> {
  pub initial_value: F,
  pub origin: F,
  pub min: F,
  pub max: F,
  pub resolution: F,
}

impl<F: Float> ParamValues<F> {
  pub fn with_initial_value(self, initial_value: F) -> Self {
    Self {
      initial_value,
      ..self
    }
  }
}

#[derive(Debug, Clone)]
pub struct Param<'a, F: Float> {
  pub id: &'a str,
  pub values: ParamValues<F>,
  pub value: Signal<F>,
  pub out_signal_ref: SignalRef,
  pub mod_signal_ref: SignalRef,
}

#[derive(Debug, Clone)]
pub struct ParamBlock {
  pub reference: ParamRef,
  pub out_signal_ref: SignalRef,
  pub mod_signal_ref: SignalRef,
}

#[derive(Debug, Clone)]
pub enum Block<F: Float> {
  Const { value: F, signal: SignalRef },

  Param(ParamBlock),

  DCA(dca::Block),

  EG(envgen::Block),

  Expr(expr::Block<F>),

  Filter(filter::Block),

  Lfo(lfo::Block),

  Osc(osc::Block),

  Out { left: SignalRef, right: SignalRef },
}

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

#[derive(Debug, Clone)]
pub struct Program<'a, F: Float> {
  signals_count: usize,
  voice: VoiceBlock,
  sources: Vec<Source<'a>, MaxSources>,
  params: Vec<Param<'a, F>, MaxParams>,
  blocks: Vec<Block<F>, MaxBlocks>,
  modulations: Modulations<F>,
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

  pub fn get_param_mut<R: Into<ParamRef>>(
    &mut self,
    param: R,
  ) -> Option<(ParamRef, &mut Param<'a, F>)> {
    let param_ref = param.into();
    self
      .params
      .get_mut(param_ref.0)
      .map(|param| (param_ref, param))
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

  pub fn get_param_signal(&self, param: ParamRef) -> &Signal<F> {
    &self.params[param.0].value
  }

  pub fn get_param_signal_mut(&mut self, param: ParamRef) -> &mut Signal<F> {
    &mut self.params[param.0].value
  }

  pub fn update_modulation(
    &mut self,
    param_ref: ParamRef,
    source_ref: SourceRef,
    amount: F,
  ) -> Result<(), modulations::Error> {
    self.modulations.update(param_ref, source_ref, amount)
  }

  pub fn delete_modulation(
    &mut self,
    param_ref: ParamRef,
    source_ref: SourceRef,
  ) -> Result<(), modulations::Error> {
    self.modulations.delete(param_ref, source_ref)
  }

  pub fn get_param_modulations(&self, param_ref: ParamRef) -> modulations::Iter<F> {
    self.modulations.get_param_modulations(param_ref)
  }

  // pub fn for_each_modulation<A>(&self, param_ref: ParamRef, process: A) where A: FnMut(&Modulation<F>) {
  //   self.modulations.for_each_modulation(param_ref, process)
  // }

  pub fn reset_params(&mut self) {
    for param in self.params.iter_mut() {
      param.value.set(param.values.initial_value);
      param.value.reset();
    }
  }

  pub fn update_params(&mut self) {
    for param in self.params.iter_mut() {
      param.value.update_state();
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
