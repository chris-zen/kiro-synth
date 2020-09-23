use druid::im::{vector, Vector};
use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{ParamRef, Program, SourceRef};

use crate::synth::program::kiro::KiroModule;
use crate::synth::{SynthAudioLevels, SynthClientMutex};

use crate::ui::model::{Dca, EnvGen, Filter, Lfo, Modulations, Osc, Param};

#[derive(Debug, Clone, Data)]
pub struct AudioLevel {
  pub peak: f64,
  pub level: f64,
}

impl Default for AudioLevel {
  fn default() -> Self {
    AudioLevel {
      peak: f64::NEG_INFINITY,
      level: f64::NEG_INFINITY,
    }
  }
}

impl AudioLevel {
  pub fn new(level: &SynthAudioLevels) -> Self {
    AudioLevel {
      peak: level.peak as f64,
      level: level.level as f64,
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Synth {
  pub active_voices: usize,

  pub left_level: AudioLevel,
  pub right_level: AudioLevel,

  pub osc: Vector<Osc>,
  pub osc_index: usize,

  pub mod_index: usize,

  pub eg: Vector<EnvGen>,

  pub lfo: Vector<Lfo>,

  pub filter: Vector<Filter>,
  pub filter_index: usize,

  pub dca: Dca,

  pub modulations: Modulations,

  #[data(ignore)]
  pub synth_client: SynthClientMutex<f32>,
}

impl Synth {
  pub fn new<'a, F: Float + 'static>(
    program: &Program<'a, F>,
    module: &KiroModule,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    let params = &module.params;

    Synth {
      active_voices: 0,

      left_level: AudioLevel::default(),
      right_level: AudioLevel::default(),

      osc: vector![
        Osc::new(program, &params.osc1, synth_client.clone()),
        Osc::new(program, &params.osc2, synth_client.clone()),
        Osc::new(program, &params.osc3, synth_client.clone()),
        Osc::new(program, &params.osc4, synth_client.clone()),
      ],
      osc_index: 0,

      mod_index: 0,

      eg: vector![EnvGen::new(program, &params.eg1, synth_client.clone()),],

      lfo: vector![
        Lfo::new(program, &params.lfo1, synth_client.clone()),
        Lfo::new(program, &params.lfo2, synth_client.clone()),
      ],

      filter: vector![Filter::new(program, &params.filter1, synth_client.clone()),],
      filter_index: 0,

      dca: Dca::new(program, &params.dca, synth_client.clone()),

      modulations: Modulations::new(program, module, synth_client.clone()),

      synth_client,
    }
    .with_init_modulations_config()
  }
}

impl<'a> Synth {
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

  pub fn update_modulations_config(
    &mut self,
    source_ref: SourceRef,
    param_ref: ParamRef,
    config_amount: f64,
  ) {
    self
      .modulations
      .update_modulation(source_ref, param_ref, config_amount);
    let total_amount = self.modulations.get_total_amounts_for_param(param_ref);
    let same_source = self
      .modulations
      .config_source
      .filter(|source| *source == source_ref)
      .is_some();
    self.for_each_modulated_param(move |param| {
      if param.param_ref == param_ref {
        if same_source {
          param.modulation.config_amount = config_amount;
        }
        param.modulation.total_amount = total_amount;
      }
    });
  }

  pub fn stop_modulations_config(&mut self, source_ref: SourceRef) {
    self.modulations.stop_config(source_ref);
    self.for_each_modulated_param(move |param| {
      param.modulation.config_source = None;
      param.modulation.config_amount = 0.0;
    });
  }

  pub fn delete_modulation(&mut self, source_ref: SourceRef, param_ref: ParamRef) {
    self.modulations.delete_modulation(source_ref, param_ref);
    let total_amount = self.modulations.get_total_amounts_for_param(param_ref);
    self.for_each_modulated_param(move |param| {
      if param.param_ref == param_ref {
        param.modulation.total_amount = total_amount;
      }
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

  pub fn update_feedback(&mut self) {
    if let Some(feedback) = self.synth_client.get_feedback().unwrap_or(None) {
      self.active_voices = feedback.num_active_voices;
      self.left_level = AudioLevel::new(&feedback.left_levels);
      self.right_level = AudioLevel::new(&feedback.right_levels);
      self.for_each_modulated_param(|param| {
        let param_index: usize = param.param_ref.into();
        let modulation = feedback.modulations[param_index];
        param.modulation.value = modulation as f64;
      });
    }
  }
}
