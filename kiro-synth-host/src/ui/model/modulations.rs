use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::iter::FromIterator;

use derivative::Derivative;

use druid::{Data, Lens};
use druid::im::Vector;
use druid::widget::ListIter;

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{SourceRef, Program, ParamRef};

use crate::program::kiro::KiroModule;
use crate::synth::SynthClient;


#[derive(Debug, Clone, PartialEq, Data)]
pub enum GroupBy {
  Param,
  Source,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Group {
  index: usize,
  pub name: String,
  pub modulations: Vector<Modulation>
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

#[derive(Debug, Clone, Data, Lens)]
pub struct Modulations {
  group_by: GroupBy,
  modulations: Vector<InternalModulation>,

  #[data(ignore)]
  pub synth_client: Arc<Mutex<SynthClient<f32>>>,
}

impl Modulations {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     _module: &KiroModule,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {

    let mut modulations = Vector::<InternalModulation>::new();

    for (index, param) in program.get_params().iter().enumerate() {
      let param_ref = ParamRef::new(index); // TODO param_ref should come from the program.get_params() call
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
      group_by: GroupBy::Source,
      modulations,
      synth_client: synth_client.clone(),
    }
  }

  fn groups<K>(&self,
               get_key: impl Fn(&InternalModulation) -> K,
               get_group_name: impl Fn(&InternalModulation) -> String,
               get_modulation_name: impl Fn(&InternalModulation) -> String) -> Vector<Group>
    where K: Into<usize> {

    let mut v = HashMap::<usize, Group>::new();
    for (index, internal_modulation) in self.modulations.iter().enumerate() {
      let key = get_key(&internal_modulation).into();
      let group = v.entry(key).or_insert_with(|| {
        Group {
          index: key,
          name: get_group_name(&internal_modulation),
          modulations: Vector::new(),
        }
      });
      let modulation_name = get_modulation_name(&internal_modulation);
      let modulation = internal_modulation.as_modulation(index, modulation_name, self.synth_client.clone());
      group.modulations.push_back(modulation);
    }
    let mut result = Vector::from_iter(v.into_iter().filter_map(|(_, group)| {
      if group.modulations.is_empty() { None } else { Some(group) }
    }));
    result.sort_by(|g1, g2| g1.index.cmp(&g2.index));
    result
  }

  fn count<K>(&self,
              get_key: impl Fn(&InternalModulation) -> K) -> usize
    where K: Into<usize> {

    let mut v = HashMap::<usize, usize>::new();
    for modulation in self.modulations.iter() {
      let key = get_key(&modulation).into();
      let m = v.entry(key).or_insert(0usize);
      *m += 1usize;
    }
    v.values().filter_map(|count| {
      if *count == 0 { None } else { Some(1) }
    }).sum::<usize>()
  }
}

impl ListIter<Group> for Modulations {
  fn for_each(&self, mut cb: impl FnMut(&Group, usize)) {
    let groups = match self.group_by {
      GroupBy::Param => {
        self.groups(
          |m| m.param_ref,
          |m| m.param_name.clone(),
          |m| m.source_name.clone(),
        )
      },
      GroupBy::Source => {
        self.groups(
          |m| m.source_ref,
          |m| m.source_name.clone(),
          |m| m.param_name.clone(),
        )
      },
    };
    groups.iter().enumerate().for_each(|(i, group)| cb(group, i));
  }

  fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Group, usize)) {
    let mut groups = match self.group_by {
      GroupBy::Param => {
        self.groups(
          |m| m.param_ref,
          |m| m.param_name.clone(),
          |m| m.source_name.clone(),
        )
      },
      GroupBy::Source => {
        self.groups(
          |m| m.source_ref,
          |m| m.source_name.clone(),
          |m| m.param_name.clone(),
        )
      },
    };
    groups.iter_mut().enumerate().for_each(|(i, group)| {
      cb(group, i);
      for modulation in group.modulations.iter() {
        self.modulations[modulation.index].amount = modulation.amount;
      }
    });
  }

  fn data_len(&self) -> usize {
    match self.group_by {
      GroupBy::Param => self.count(|m| m.param_ref),
      GroupBy::Source => self.count(|m| m.source_ref),
    }
  }
}
