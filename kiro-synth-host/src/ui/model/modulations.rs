use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use derivative::Derivative;

use druid::{Data, Lens};
use druid::im::{vector, Vector};
use druid::widget::ListIter;

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{SourceRef, Program, ParamRef};

use crate::program::kiro::KiroModule;
use crate::synth::SynthClient;


#[derive(Debug, Clone, Copy, PartialEq, Data)]
pub enum View {
  GroupBySource,
  GroupByParam,
}

#[derive(Debug, Clone, Copy, Data)]
pub enum Reference {
  Source(
    #[data(same_fn="PartialEq::eq")]
    SourceRef
  ),
  Param(
    #[data(same_fn="PartialEq::eq")]
    ParamRef
  ),
}

impl Reference {
  pub fn matches(&self, source: Option<SourceRef>) -> bool {
    match self {
      Reference::Source(source_ref) => source.filter(|s| *s == *source_ref).is_some(),
      Reference::Param(_) => false,
    }
  }
}

impl Into<usize> for Reference {
  fn into(self) -> usize {
    match self {
      Self::Source(reference) => reference.into(),
      Self::Param(reference) => reference.into(),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Data)]
pub enum ConfigMode {
  Ready,
  Ongoing,
  Disabled,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Group {
  index: usize,
  pub reference: Reference,
  pub name: String,
  pub modulations: Vector<Modulation>,
  pub config_mode: ConfigMode,
}

#[derive(Debug, Clone, Data, Lens, Derivative)]
#[derivative(PartialEq)]
pub struct Modulation {
  index: usize,

  pub name: String,

  #[data(same_fn="PartialEq::eq")]
  pub source_ref: SourceRef,
  #[data(same_fn = "PartialEq::eq")]
  pub param_ref: ParamRef,

  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,

  pub amount: f64,

  #[data(ignore)]
  #[derivative(PartialEq="ignore")]
  pub synth_client: Arc<Mutex<SynthClient<f32>>>,
}

impl Modulation {
  pub fn send_modulation_amount(&self, amount: f64) -> Result<(), ()> {
    self.synth_client.lock()
        .map_err(|_| ())
        .map(|mut client| client.send_modulation_amount(self.param_ref, self.source_ref, amount as f32))
  }
}

#[derive(Debug, Clone, Data)]
pub struct InternalModulation {
  #[data(same_fn="PartialEq::eq")]
  pub source_ref: SourceRef,
  pub source_name: String,

  #[data(same_fn = "PartialEq::eq")]
  pub param_ref: ParamRef,
  pub param_name: String,

  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub amount: f64,
}

impl InternalModulation {
  pub fn as_modulation(&self, index: usize, name: String, synth_client: Arc<Mutex<SynthClient<f32>>>) -> Modulation {
    Modulation {
      index,
      name,
      source_ref: self.source_ref,
      param_ref: self.param_ref,
      origin: self.origin,
      min: self.min,
      max: self.max,
      step: self.step,
      amount: self.amount,
      synth_client,
    }
  }
}

#[derive(Debug, Clone, Data)]
pub struct Source {
  pub name: String,
  #[data(same_fn = "PartialEq::eq")]
  pub reference: SourceRef,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Modulations {
  pub view: View,

  modulations: Vector<InternalModulation>,

  #[data(same_fn = "PartialEq::eq")]
  config_source: Option<SourceRef>,

  sources: Vector<Source>,

  #[data(ignore)]
  pub synth_client: Arc<Mutex<SynthClient<f32>>>,
}

impl Modulations {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     _module: &KiroModule,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {

    let mut modulations = Vector::<InternalModulation>::new();

    let mut params = Vector::<String>::new();

    let sources = program.get_sources().iter().enumerate()
        .map(|(index, source)| {
          Source {
            name: source.id.to_string(),
            reference: SourceRef::new(index),
          }
        })
        .collect();

    for (index, param) in program.get_params().iter().enumerate() {
      let param_ref = ParamRef::new(index); // TODO param_ref should come from the program.get_params() call
      params.push_back(param.id.to_string());
      for modulator in param.modulators.iter() {
        if let Some(source) = program.get_source(modulator.source) {
          let modulation = InternalModulation {
            source_ref: modulator.source,
            source_name: source.id.to_string(),
            param_ref,
            param_name: param.id.to_string(),
            origin: param.values.origin.to_f64().unwrap(),
            min: param.values.min.to_f64().unwrap(),
            max: param.values.max.to_f64().unwrap(),
            step: param.values.resolution.to_f64().unwrap(),
            amount: modulator.amount.to_f64().unwrap(),
          };
          modulations.push_back(modulation)
        }
      }
    }

    Modulations {
      view: View::GroupBySource,
      modulations,
      config_source: None,
      sources,
      synth_client: synth_client.clone(),
    }
  }

  pub fn begin(&mut self, source_ref: SourceRef) {
    self.config_source = Some(source_ref);
  }

  pub fn done(&mut self, source_ref: SourceRef) {
    self.config_source = self.config_source
        .filter(|v| *v != source_ref);
  }

  fn groups(&self,
            get_key: impl Fn(&InternalModulation) -> Reference,
            get_group_name: impl Fn(&InternalModulation) -> String,
            get_modulation_name: impl Fn(&InternalModulation) -> String,
            allow_empty: bool) -> Vector<Group> {

    let mut v = HashMap::<usize, Group>::new();
    // let mut empty_sources = HashSet::<usize>::from_iter();
    // self.sources.iter().map(|s| s.reference.into())

    for (index, internal_modulation) in self.modulations.iter().enumerate() {
      let reference = get_key(&internal_modulation);
      let key = reference.into();
      let config_mode = if reference.matches(self.config_source) {
        ConfigMode::Ongoing
      }
      else {
        self.config_source
            .map(|_| ConfigMode::Disabled)
            .unwrap_or(ConfigMode::Ready)
      };

      let group = v.entry(key).or_insert_with(|| {
        Group {
          index: key,
          reference,
          name: get_group_name(&internal_modulation),
          modulations: Vector::new(),
          config_mode,
        }
      });
      let modulation_name = get_modulation_name(&internal_modulation);
      let modulation = internal_modulation.as_modulation(index, modulation_name, self.synth_client.clone());
      group.modulations.push_back(modulation);
    }
    let mut result = Vector::from_iter(v.into_iter().filter_map(|(_, group)| {
      Some(group).filter(|g| allow_empty || !g.modulations.is_empty())
    }));
    result.sort_by(|g1, g2| g1.index.cmp(&g2.index));
    result
  }

  fn count<K>(&self,
              get_key: impl Fn(&InternalModulation) -> K,
              allow_empty: bool) -> usize
    where K: Into<usize> {

    let mut v = HashMap::<usize, usize>::new();
    for modulation in self.modulations.iter() {
      let key = get_key(&modulation).into();
      let m = v.entry(key).or_insert(0usize);
      *m += 1usize;
    }
    v.values().filter_map(|count| {
      Some(1).filter(|c| allow_empty || *c > 0)
    }).sum::<usize>()
  }
}

impl ListIter<Group> for Modulations {
  fn for_each(&self, mut cb: impl FnMut(&Group, usize)) {
    let groups = match self.view {
      View::GroupBySource => {
        self.groups(
          |m| Reference::Source(m.source_ref),
          |m| m.source_name.clone(),
          |m| m.param_name.clone(),
          true,
        )
      }
      View::GroupByParam => {
        self.groups(
          |m| Reference::Param(m.param_ref),
          |m| m.param_name.clone(),
          |m| m.source_name.clone(),
          false,
        )
      }
    };
    groups.iter().enumerate().for_each(|(i, group)| cb(group, i));
  }

  fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Group, usize)) {
    let mut groups = match self.view {
      View::GroupBySource => {
        self.groups(
          |m| Reference::Source(m.source_ref),
          |m| m.source_name.clone(),
          |m| m.param_name.clone(),
          true,
        )
      }
      View::GroupByParam => {
        self.groups(
          |m| Reference::Param(m.param_ref),
          |m| m.param_name.clone(),
          |m| m.source_name.clone(),
          false,
        )
      }
    };
    groups.iter_mut().enumerate().for_each(|(i, group)| {
      cb(group, i);
      for modulation in group.modulations.iter() {
        self.modulations[modulation.index].amount = modulation.amount;
      }
    });
  }

  fn data_len(&self) -> usize {
    match self.view {
      View::GroupBySource => self.count(
        |m| Reference::Source(m.source_ref),
        true
      ),
      View::GroupByParam => self.count(
        |m| Reference::Param(m.param_ref),
        false
      ),
    }
  }
}
