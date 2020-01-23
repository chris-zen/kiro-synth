use heapless::{Vec, FnvIndexMap};
use heapless::consts;

use kiro_synth_core::float::Float;
use kiro_synth_engine::event::{Event, Message as SynthMessage};
use kiro_synth_engine::program::Param;
use kiro_synth_midi::types::{U14, U7};

type MaxMappings = consts::U128;
type MidiController = U7;
type MappingIndex = usize;
type ParamIndex = usize;

pub enum Transform<F: Float> {
  UnipolarU7,
  UnipolarU14,
  BipolarU7,
  BipolarU14,
  MinMaxU7(F, F, F),
  MinMaxU14(F, F, F),
}

impl<F: Float> Transform<F> {
  fn param_value_from(&self, midi_value: usize) -> F {
    match self {
      Transform::UnipolarU7 => F::from(midi_value & 0x7f).unwrap() / F::from(127.0).unwrap(),
      Transform::UnipolarU14 => F::from(midi_value & 0x3fff).unwrap() / F::from(16383.0).unwrap(),
      Transform::BipolarU7 => (F::from(midi_value & 0x7f).unwrap() * F::from(2.0 / 127.0).unwrap()) - F::one(),
      Transform::BipolarU14 => (F::from(midi_value & 0x3fff).unwrap() * F::from(2.0 / 16383.0).unwrap()) - F::one(),
      Transform::MinMaxU7(min, max, resolution) => {
        let midi_value = F::from(midi_value & 0x7f).unwrap();
        let range = *max - *min;
        let value = midi_value * F::from(1.0 / 127.0).unwrap() * range + *min;
        (value / *resolution).round() * *resolution
      },
      Transform::MinMaxU14(min, max, resolution) => {
        let midi_value = F::from(midi_value & 0x3fff).unwrap();
        let range = *max - *min;
        let value = midi_value * F::from(1.0 / 16383.0).unwrap() * range + *min;
        (value / *resolution).round() * *resolution
      },
    }
  }
}

pub struct PitchBendMapping<F: Float> {
  pub param_index: ParamIndex,
  transform: Transform<F>,
}

pub struct ControllerMapping<F: Float> {
  pub param_index: ParamIndex,
  pub controller: MidiController,
  transform: Transform<F>,
}

pub struct MidiMapper<F: Float> {
  pitch_bend_mapping: Option<PitchBendMapping<F>>,
  controller_mappings: Vec<ControllerMapping<F>, MaxMappings>,
  controller_to_param: FnvIndexMap<MidiController, MappingIndex, MaxMappings>,
  param_to_controller: FnvIndexMap<ParamIndex, MappingIndex, MaxMappings>,
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

  pub fn pitch_bend<'a>(&mut self, param_info: Option<(usize, &Param<'a, F>)>) {
    if let Some((index, param)) = param_info {
      let transform = Transform::MinMaxU14(param.values.min, param.values.max, param.values.resolution);
      self.pitch_bend_mapping = Some(PitchBendMapping { param_index: index, transform })
    }
  }

  pub fn map_midi_pitch_bend(&self, midi_value: U14) -> Option<Event<F>> {
    self.pitch_bend_mapping.as_ref().map(|mapping| {
      let message = SynthMessage::Param {
        index: mapping.param_index,
        value: mapping.transform.param_value_from(midi_value as usize)
      };
      Event::new(0u64, message)
    })
  }

  pub fn controller<'a>(&mut self, midi_controller: MidiController, param_info: Option<(usize, &Param<'a, F>)>) {
    if let Some((index, param)) = param_info {
      let transform = Transform::MinMaxU7(param.values.min, param.values.max, param.values.resolution);
      self.add_controller_mapping(midi_controller, index, transform)
    }
  }

  fn add_controller_mapping(&mut self, midi_controller: MidiController, param_index: ParamIndex, transform: Transform<F>) {
    let mapping_index = self.controller_mappings.len();
    let mapping = ControllerMapping {
      param_index,
      controller: midi_controller,
      transform
    };
    drop(self.controller_mappings.push(mapping));
    drop(self.controller_to_param.insert(midi_controller, mapping_index));
    drop(self.param_to_controller.insert(param_index, mapping_index));
  }

  fn get_controller_mapping(&self, controller: MidiController) -> Option<&ControllerMapping<F>> {
    self.controller_to_param
        .get(&controller)
        .map(|mapping_index| &self.controller_mappings[*mapping_index])
  }

  pub fn map_midi_controller(&self, controller: MidiController, midi_value: U7) -> Option<Event<F>> {
    self.get_controller_mapping(controller).map(|mapping| {
      let message = SynthMessage::Param {
        index: mapping.param_index,
        value: mapping.transform.param_value_from(midi_value as usize)
      };
      Event::new(0u64, message)
    })
  }
}
