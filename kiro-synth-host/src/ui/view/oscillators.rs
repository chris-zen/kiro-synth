use std::sync::{Arc, Mutex};

use druid::{Widget, Env};
use druid::widget::{Flex, WidgetExt};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, Osc, OscFromSynth};
use crate::ui::view::{build_tabs, build_switcher, build_knob_value, build_knob_enum};


pub struct OscillatorsView;

impl OscillatorsView {

  pub fn new<F: Float + 'static>(synth_model: &SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

    let osc_len = synth_model.osc.lock().unwrap().len();
    let tabs = build_tabs(osc_len, |index| format!("OSC{}", index + 1))
        .lens(SynthModel::osc_index);

    build_switcher(tabs,
                   |data: &SynthModel, _env: &Env| data.osc_index,
                   move |_index: &usize, _data: &SynthModel, _env: &Env| {
                     Box::new(build_osc_view(synth_client.clone()).lens(OscFromSynth))
                   })
  }
}

fn build_osc_view<F: Float + 'static>(synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Osc> {

  let shape_client = synth_client.clone();
  let shape_fn = move |index: usize| shape_client.lock().unwrap().osc_waveforms().name(index).to_string();

  Flex::row()
      .with_child(
        build_knob_enum("Shape", shape_fn).lens(Osc::shape)
      )
      .with_child(
        build_knob_value("Octaves", "").lens(Osc::octaves)
      )
      .with_child(
        build_knob_value("Semitones", "").lens(Osc::semitones)
      )
      .with_child(
        build_knob_value("Cents", "").lens(Osc::cents)
      )
      .with_child(
        build_knob_value("Amplitude", "").lens(Osc::amplitude)
      )
      .with_flex_spacer(1.0)
}
