mod param;
mod eg;
mod lfo;
mod oscillators;
mod filter;
mod dca;
pub mod modulations;

use druid::{Data, Lens};
use druid::im::{vector, Vector};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, SourceRef, ParamRef};

use crate::program::kiro::KiroModule;
use crate::synth::SynthClientMutex;

pub use param::{KnobDataFromParam, Param};
pub use eg::{EgFromSynth, EnvGen};
pub use lfo::{LfoFromSynth, Lfo};
pub use oscillators::{OscFromSynth, Osc};
pub use filter::{FilterFromSynth, Filter};
pub use dca::Dca;
pub use modulations::Modulations;
use crate::ui::model::modulations::InternalModulation;


pub struct ZeroIndex;

impl Lens<SynthModel, usize> for ZeroIndex {
  fn with<V, F: FnOnce(&usize) -> V>(&self, _data: &SynthModel, f: F) -> V {
    f(&0usize)
  }

  fn with_mut<V, F: FnOnce(&mut usize) -> V>(&self, _data: &mut SynthModel, f: F) -> V {
    f(&mut 0usize)
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct SynthModel {

  pub osc: Vector<Osc>,
  pub osc_index: usize,

  pub mod_index: usize,

  pub eg: Vector<EnvGen>,

  pub lfo: Vector<Lfo>,

  pub filter: Vector<Filter>,
  pub filter_index: usize,

  pub dca: Dca,

  pub modulations: Modulations,
}

impl SynthModel {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     module: &KiroModule,
                                     synth_client: SynthClientMutex<f32>) -> Self {

    let params = &module.params;

    SynthModel {
      osc: vector![
        Osc::new(program, &params.osc1, synth_client.clone()),
        Osc::new(program, &params.osc2, synth_client.clone()),
        Osc::new(program, &params.osc3, synth_client.clone()),
        Osc::new(program, &params.osc4, synth_client.clone()),
      ],
      osc_index: 0,

      mod_index: 0,

      eg: vector![
        EnvGen::new(program, &params.eg1, synth_client.clone()),
      ],

      lfo: vector![
        Lfo::new(program, &params.lfo1, synth_client.clone()),
        Lfo::new(program, &params.lfo2, synth_client.clone()),
      ],

      filter: vector![
        Filter::new(program, &params.filter1, synth_client.clone()),
      ],
      filter_index: 0,

      dca: Dca::new(program, &params.dca, synth_client.clone()),

      modulations: Modulations::new(program, module, synth_client.clone()),
    }.with_init_modulations_config()
  }
}

impl<'a> SynthModel {
  pub fn with_init_modulations_config(mut self) -> Self {
    let total_amounts = self.modulations.get_total_amounts_by_param();

    self.for_each_modulated_param(move |param| {
      param.modulation.total_amount = total_amounts
          .get(&param.param_ref.into())
          .cloned()
          .unwrap_or(0.0);
    });

    self
  }

  pub fn start_modulations_config(&mut self, source_ref: SourceRef) {
    self.modulations.start_config(source_ref);
    let config_amounts = self.modulations.get_config_amounts_by_param(source_ref);
    let total_amounts = self.modulations.get_total_amounts_by_param();
    // println!("{:#?}\n{:#?}", config_amounts, total_amounts);
    let config_source = self.modulations.config_source;
    self.for_each_modulated_param(move |param| {
      let key: usize = param.param_ref.into();
      param.modulation.config_source = config_source;
      param.modulation.config_amount = config_amounts.get(&key).cloned().unwrap_or(0.0);
      param.modulation.total_amount = total_amounts.get(&key).cloned().unwrap_or(0.0);
      // println!(">> {:?}", param);
    });
  }

  pub fn update_modulations_config(&mut self, source_ref: SourceRef, param_ref: ParamRef, config_amount: f64) {
    let same_source = self.modulations.config_source
        .filter(|source| *source == source_ref)
        .is_some();

    let total_amounts = self.modulations.get_total_amounts_by_param();

    self.for_each_modulated_param(move |param| {
      if same_source && param.param_ref == param_ref {
        param.modulation.config_amount = config_amount;
      }
      param.modulation.total_amount = total_amounts.get(&param.param_ref.into()).cloned().unwrap_or(0.0);
    });

    let same_source_and_param = |modulation: &&mut InternalModulation| {
      modulation.source_ref == source_ref && modulation.param_ref == param_ref
    };

    match self.modulations.modulations.iter_mut().find(same_source_and_param) {
      Some(modulation) => {
        modulation.amount = config_amount;
      }
      None => {
        let source_name = self.modulations.get_source(source_ref)
            .map(|source| source.name.clone())
            .unwrap();

        let param = self.modulations.get_param(param_ref).unwrap();

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

        self.modulations.add_modulation(modulation);
      }
    }
  }

  pub fn stop_modulations_config(&mut self, source_ref: SourceRef) {
    self.modulations.stop_config(source_ref);
    self.for_each_modulated_param(move |param| {
      param.modulation.config_source = None;
      param.modulation.config_amount = 0.0;
    });
  }

  fn for_each_modulated_param(&mut self, apply: impl Fn(&mut Param)) {
    for osc in self.osc.iter_mut() {
      osc.for_each_modulated_param(&apply);
    }
    for eg in self.eg.iter_mut() {
      eg.for_each_modulated_param(&apply);
    }
    for lfo in self.lfo.iter_mut() {
      lfo.for_each_modulated_param(&apply);
    }
    for filter in self.filter.iter_mut() {
      filter.for_each_modulated_param(&apply);
    }
    self.dca.for_each_modulated_param(&apply);
  }
}