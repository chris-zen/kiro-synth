use std::collections::HashMap;
use std::iter::FromIterator;

use derivative::Derivative;

use druid::{Data, Lens};
use druid::im::Vector;
use druid::widget::ListIter;

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{SourceRef, Program, ParamRef};

use crate::synth::SynthClientMutex;
use crate::synth::program::kiro::KiroModule;
use crate::ui::model::Param;


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
  pub synth_client: SynthClientMutex<f32>,
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
  pub fn as_modulation(&self, index: usize, name: String, synth_client: SynthClientMutex<f32>) -> Modulation {
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

trait NamedReference {
  fn name(&self) -> String;
  fn reference(&self) -> Reference;
}

impl NamedReference for () {
  fn name(&self) -> String {
    unreachable!()
  }

  fn reference(&self) -> Reference {
    unreachable!()
  }
}

#[derive(Debug, Clone, Data)]
pub struct Source {
  pub name: String,
  #[data(same_fn = "PartialEq::eq")]
  pub reference: SourceRef,
}

impl NamedReference for Source {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn reference(&self) -> Reference {
    Reference::Source(self.reference)
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Modulations {
  pub view: View,

  pub modulations: Vector<InternalModulation>,

  #[data(same_fn = "PartialEq::eq")]
  pub config_source: Option<SourceRef>,

  #[data(ignore)]
  pub sources: Vector<Source>,

  #[data(ignore)]
  pub params: Vector<Param>,

  #[data(ignore)]
  pub synth_client: SynthClientMutex<f32>,
}

impl Modulations {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     _module: &KiroModule,
                                     synth_client: SynthClientMutex<f32>) -> Self {

    let mut modulations = Vector::<InternalModulation>::new();

    let mut params = Vector::<Param>::new();

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
      params.push_back(Param::new(program, param_ref, synth_client.clone()));
      for param_modulation in program.get_param_modulations(param_ref) {
        if let Some(source) = program.get_source(param_modulation.source_ref) {
          let modulation = InternalModulation {
            source_ref: param_modulation.source_ref,
            source_name: source.id.to_string(),
            param_ref,
            param_name: param.id.to_string(),
            origin: param.values.origin.to_f64().unwrap(),
            min: param.values.min.to_f64().unwrap(),
            max: param.values.max.to_f64().unwrap(),
            step: param.values.resolution.to_f64().unwrap(),
            amount: param_modulation.amount.to_f64().unwrap(),
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
      params,
      synth_client: synth_client.clone(),
    }
  }

  pub fn start_config(&mut self, source_ref: SourceRef) {
    println!("modulations start-config {:?}", source_ref);
    self.config_source = Some(source_ref);
  }

  pub fn stop_config(&mut self, source_ref: SourceRef) {
    println!("modulations stop-config {:?}", source_ref);
    self.config_source = self.config_source
        .filter(|v| *v != source_ref);
  }

  pub fn get_config_amounts_by_param(&self, source_ref: SourceRef) -> HashMap<usize, f64> {
    self.modulations.iter()
        .filter(|&modulation| modulation.source_ref == source_ref)
        .map(|modulation| (modulation.param_ref.into(), modulation.amount))
        .collect()
  }

  pub fn get_total_amounts_for_param(&self, param_ref: ParamRef) -> f64 {
    self.modulations.iter()
        .filter_map(|modulation| {
          Some(modulation.amount).filter(|_| modulation.param_ref == param_ref)
        })
        .fold(0.0, |acc, amount| acc + amount)
  }

  pub fn get_total_amounts_by_param(&self) -> HashMap<usize, f64> {
    self.modulations.iter()
        .map(|modulation| {
          let key: usize = modulation.param_ref.into();
          (key, modulation.amount)
        })
        .fold(HashMap::new(), |mut acc, (key, amount): (usize, f64)| {
          let total_amount = acc.entry(key).or_insert(0.0);
          *total_amount += amount;
          acc
        })
  }

  fn groups<NR>(&self,
                get_key: impl Fn(&InternalModulation) -> Reference,
                get_group_name: impl Fn(&InternalModulation) -> String,
                get_modulation_name: impl Fn(&InternalModulation) -> String,
                base_groups: &Vector<NR>) -> Vector<Group> where NR: NamedReference + Clone {

    let config_mode = |reference: Reference| {
      if reference.matches(self.config_source) {
        ConfigMode::Ongoing
      }
      else {
        self.config_source
            // .map(|_| ConfigMode::Disabled)
            .map(|_| ConfigMode::Ready)
            .unwrap_or(ConfigMode::Ready)
      }
    };

    let mut groups = HashMap::<usize, Group>::from_iter(
      base_groups.iter().map(|nr| {
        let key = nr.reference().into();
        (key, Group {
          index: key,
          name: nr.name(),
          reference: nr.reference(),
          modulations: Vector::new(),
          config_mode: config_mode(nr.reference()),
        })
      })
    );

    for (index, internal_modulation) in self.modulations.iter().enumerate() {
      let reference = get_key(&internal_modulation);
      let key = reference.into();

      let group = groups.entry(key).or_insert_with(|| {
        Group {
          index: key,
          reference,
          name: get_group_name(&internal_modulation),
          modulations: Vector::new(),
          config_mode: config_mode(reference),
        }
      });
      let modulation_name = get_modulation_name(&internal_modulation);
      let modulation = internal_modulation.as_modulation(index, modulation_name, self.synth_client.clone());
      group.modulations.push_back(modulation);
    }
    let mut result = Vector::from_iter(groups.into_iter().map(|(_, v)| v));
    result.sort_by(|g1, g2| g1.index.cmp(&g2.index));
    result
  }

  fn count<NR>(&self,
               get_key: impl Fn(&InternalModulation) -> Reference,
               base_groups: &Vector<NR>) -> usize where NR: NamedReference + Clone {

    let mut groups = HashMap::<usize, usize>::from_iter(
      base_groups.iter().map(|nr| (nr.reference().into(), 1))
    );

    for modulation in self.modulations.iter() {
      let key = get_key(&modulation).into();
      let m = groups.entry(key).or_insert(0usize);
      *m += 1usize;
    }

    groups.values().sum::<usize>()
  }

  pub fn get_source(&self, source_ref: SourceRef) -> Option<&Source> {
    self.sources.iter()
        .find(|&source| source.reference == source_ref)
  }

  pub fn get_param(&self, param_ref: ParamRef) -> Option<&Param> {
    self.params.iter()
        .find(|&param| param.param_ref == param_ref)
  }

  pub fn add_modulation(&mut self, modulation: InternalModulation) {
    // TODO check that it can be added according to the synth internal capacity
    self.synth_client.send_modulation_update(modulation.source_ref, modulation.param_ref, modulation.amount as f32).unwrap();
    self.modulations.push_back(modulation);
  }

  pub fn update_modulation(&mut self, source_ref: SourceRef, param_ref: ParamRef, config_amount: f64) {
    let same_source_and_param = |modulation: &&mut InternalModulation| {
      modulation.source_ref == source_ref && modulation.param_ref == param_ref
    };

    match self.modulations.iter_mut().find(same_source_and_param) {
      Some(modulation) => {
        modulation.amount = config_amount;
      }
      None => {
        let source_name = self.get_source(source_ref)
            .map(|source| source.name.clone())
            .unwrap();

        let param = self.get_param(param_ref).unwrap();

        let modulation = InternalModulation {
          source_ref,
          source_name,
          param_ref,
          param_name: param.name.to_string(),
          amount: config_amount,
          origin: param.origin,
          min: param.min,
          max: param.max,
          step: param.step,
        };

        self.add_modulation(modulation);
      }
    }
  }

  pub fn delete_modulation(&mut self, source_ref: SourceRef, param_ref: ParamRef) {
    self.modulations.iter()
        .position(|m| m.source_ref == source_ref && m.param_ref == param_ref)
        .iter()
        .for_each(|index| { self.modulations.remove(*index); });

    self.synth_client.send_modulation_delete(source_ref, param_ref).unwrap();
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
          &self.sources,
        )
      }
      View::GroupByParam => {
        self.groups(
          |m| Reference::Param(m.param_ref),
          |m| m.param_name.clone(),
          |m| m.source_name.clone(),
          &Vector::<()>::new(),
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
          &self.sources,
        )
      }
      View::GroupByParam => {
        self.groups(
          |m| Reference::Param(m.param_ref),
          |m| m.param_name.clone(),
          |m| m.source_name.clone(),
          &Vector::<()>::new(),
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
        &self.sources,
      ),
      View::GroupByParam => self.count(
        |m| Reference::Param(m.param_ref),
        &Vector::<()>::new(),
      ),
    }
  }
}
