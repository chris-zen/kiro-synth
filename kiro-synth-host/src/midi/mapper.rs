use heapless::consts;
use heapless::{FnvIndexMap, Vec};

use kiro_synth_dsp::float::Float;
use kiro_synth_engine::event::{Event, Message as SynthMessage};
use kiro_synth_engine::program::{Param, ParamRef};
use kiro_midi_core::types::{U14, U7};

type MaxMappings = consts::U128;
type MidiController = U7;
type MappingIndex = usize;
// type ParamIndex = usize;

pub enum Transform<F: Float> {
  // UnipolarU7,
  // UnipolarU14,
  // BipolarU7,
  // BipolarU14,
  // MinMaxU7(F, F, F),
  MinMaxU14(F, F, F),
  Relative64(F),
}

impl<F: Float> Transform<F> {
  fn param_value_from(&self, midi_value: usize) -> F {
    match self {
      // Transform::UnipolarU7 => F::val(midi_value & 0x7f) / F::val(127.0),
      // Transform::UnipolarU14 => F::val(midi_value & 0x3fff) / F::val(16383.0),
      // Transform::BipolarU7 => (F::val(midi_value & 0x7f) * F::val(2.0 / 127.0)) - F::one(),
      // Transform::BipolarU14 => (F::val(midi_value & 0x3fff) * F::val(2.0 / 16383.0)) - F::one(),
      // Transform::MinMaxU7(min, max, resolution) => {
      //   let midi_value = F::val(midi_value & 0x7f);
      //   let range = *max - *min;
      //   let value = midi_value * F::val(1.0 / 127.0) * range + *min;
      //   (value / *resolution).round() * *resolution
      // },
      Transform::MinMaxU14(min, max, resolution) => {
        let midi_value = F::val(midi_value & 0x3fff);
        let range = *max - *min;
        let value = midi_value * F::val(1.0 / 16383.0) * range + *min;
        (value / *resolution).round() * *resolution
      }
      Transform::Relative64(resolution) => {
        let midi_value = F::val(midi_value & 0x7f);
        (midi_value - F::val(64.0)) * *resolution
      }
    }
  }
}

pub struct PitchBendMapping<F: Float> {
  pub param_ref: ParamRef,
  transform: Transform<F>,
}

pub struct ControllerMapping<F: Float> {
  pub param_ref: ParamRef,
  pub controller: MidiController,
  transform: Transform<F>,
}

pub struct MidiMapper<F: Float> {
  pitch_bend_mapping: Option<PitchBendMapping<F>>,
  controller_mappings: Vec<ControllerMapping<F>, MaxMappings>,
  controller_to_param: FnvIndexMap<MidiController, MappingIndex, MaxMappings>,
  param_to_controller: FnvIndexMap<ParamRef, MappingIndex, MaxMappings>,
}

impl<F: Float> MidiMapper<F> {
  pub fn new() -> Self {
    MidiMapper {
      pitch_bend_mapping: None,
      controller_mappings: Vec::new(),
      controller_to_param: FnvIndexMap::new(),
      param_to_controller: FnvIndexMap::new(),
    }
  }

  pub fn pitch_bend<'a>(&mut self, param_info: Option<(ParamRef, &Param<'a, F>)>) {
    if let Some((param_ref, param)) = param_info {
      let transform =
        Transform::MinMaxU14(param.values.min, param.values.max, param.values.resolution);
      self.pitch_bend_mapping = Some(PitchBendMapping {
        param_ref,
        transform,
      })
    }
  }

  pub fn map_midi_pitch_bend(&self, midi_value: U14) -> Option<Event<F>> {
    self.pitch_bend_mapping.as_ref().map(|mapping| {
      let message = SynthMessage::ParamValue {
        param_ref: mapping.param_ref,
        value: mapping.transform.param_value_from(midi_value as usize),
      };
      Event::new(0u64, message)
    })
  }

  // pub fn controller<'a>(&mut self, midi_controller: MidiController, param_info: Option<(ParamRef, &Param<'a, F>)>) {
  //   if let Some((param_ref, param)) = param_info {
  //     let transform = Transform::MinMaxU7(param.values.min, param.values.max, param.values.resolution);
  //     self.add_controller_mapping(midi_controller, param_ref, transform)
  //   }
  // }

  pub fn rel_controller<'a>(
    &mut self,
    midi_controller: MidiController,
    param_info: Option<(ParamRef, &Param<'a, F>)>,
  ) {
    if let Some((param_ref, param)) = param_info {
      let transform = Transform::Relative64(param.values.resolution);
      self.add_controller_mapping(midi_controller, param_ref, transform)
    }
  }

  fn add_controller_mapping(
    &mut self,
    midi_controller: MidiController,
    param_ref: ParamRef,
    transform: Transform<F>,
  ) {
    let mapping_index = self.controller_mappings.len();
    let mapping = ControllerMapping {
      param_ref,
      controller: midi_controller,
      transform,
    };
    self.controller_mappings.push(mapping).ok();
    self
      .controller_to_param
      .insert(midi_controller, mapping_index)
      .ok();
    self
      .param_to_controller
      .insert(param_ref, mapping_index)
      .ok();
  }

  fn get_controller_mapping(&self, controller: MidiController) -> Option<&ControllerMapping<F>> {
    self
      .controller_to_param
      .get(&controller)
      .map(|mapping_index| &self.controller_mappings[*mapping_index])
  }

  pub fn map_midi_controller(
    &self,
    controller: MidiController,
    midi_value: U7,
  ) -> Option<Event<F>> {
    self.get_controller_mapping(controller).and_then(|mapping| {
      let is_relative = Self::is_relative_controller(mapping);
      let value = mapping.transform.param_value_from(midi_value as usize);
      let maybe_message = if is_relative {
        Some(SynthMessage::ParamChange {
          param_ref: mapping.param_ref,
          change: value,
        })
        .filter(|_| value != F::zero())
      } else {
        Some(SynthMessage::ParamValue {
          param_ref: mapping.param_ref,
          value,
        })
      };
      maybe_message.map(|message| Event::new(0u64, message))
    })
  }

  fn is_relative_controller(mapping: &ControllerMapping<F>) -> bool {
    match mapping.transform {
      Transform::Relative64(_) => true,
      _ => false,
    }
  }
}
