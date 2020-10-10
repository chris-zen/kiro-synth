pub mod header;
pub mod modulations;
pub mod param;
pub mod synth;

use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{ParamRef, Program, SourceRef};

use crate::synth::program::kiro::KiroModule;
use crate::synth::SynthClientMutex;

use header::Header;
use modulations::Modulations;
pub use param::Param;
use synth::Synth;

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
  pub header: Header,
  pub synth: Synth,
  pub modulations: Modulations,

  #[data(ignore)]
  pub synth_client: SynthClientMutex<f32>,
}

impl AppData {
  pub fn new<'a, F: Float + 'static>(
    program: &Program<'a, F>,
    module: &KiroModule,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    AppData {
      header: Header::new(synth_client.clone()),
      synth: Synth::new(program, module, synth_client.clone()),
      modulations: Modulations::new(program, module, synth_client.clone()),
      synth_client,
    }
  }

  pub fn with_init_modulations_config(mut self) -> Self {
    let total_amounts = self.modulations.get_total_amounts_by_param();

    self.synth.for_each_modulated_param(move |param| {
      param.modulation.total_amount = total_amounts
        .get(&param.param_ref.into())
        .cloned()
        .unwrap_or(0.0);
    });

    self
  }

  pub fn update_feedback(&mut self) {
    if let Some(feedback) = self.synth_client.get_feedback().unwrap_or(None) {
      self.header.update_feedback(&feedback);
      self.synth.update_feedback(&feedback);
    }
  }

  pub fn start_modulations_config(&mut self, source_ref: SourceRef) {
    self.modulations.start_config(source_ref);
    let config_amounts = self.modulations.get_config_amounts_by_param(source_ref);
    let total_amounts = self.modulations.get_total_amounts_by_param();
    // println!("{:#?}\n{:#?}", config_amounts, total_amounts);
    let config_source = self.modulations.config_source;
    self.synth.for_each_modulated_param(move |param| {
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

    // TODO optimize this block by providing a for_modulated_param(param_ref) at Synth
    self.synth.for_each_modulated_param(move |param| {
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
    self.synth.for_each_modulated_param(move |param| {
      param.modulation.config_source = None;
      param.modulation.config_amount = 0.0;
    });
  }

  pub fn delete_modulation(&mut self, source_ref: SourceRef, param_ref: ParamRef) {
    self.modulations.delete_modulation(source_ref, param_ref);
    let total_amount = self.modulations.get_total_amounts_for_param(param_ref);
    self.synth.for_each_modulated_param(move |param| {
      if param.param_ref == param_ref {
        param.modulation.total_amount = total_amount;
      }
    });
  }
}
