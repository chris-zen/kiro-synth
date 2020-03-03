use std::sync::{Arc, Mutex};

use ringbuf::Producer;

use druid::{Widget, Color, Data};
use druid::widget::{Flex, WidgetExt, Label, Container};

use kiro_synth_core::float::Float;
use kiro_synth_engine::event::{Event, Message};

use crate::ui::model::{Model, OscModel, EgModel};
use crate::ui::params::{ParamInfo, OscParamsInfo, EgParamsInfo};
use crate::ui::knob::{KnobModel, Knob};
use crate::ui::SynthParams;
use crate::synth::SynthClient;


pub fn build<F: Float + 'static>(params: &SynthParams,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Model> {

  Flex::column()
    .with_child(
      build_osc("OSC1", &params.osc1, synth_client.clone())
              .lens(Model::osc1)
              .padding(6.0),
      1.0
    )
    .with_child(
      build_osc("OSC2", &params.osc2, synth_client.clone())
              .lens(Model::osc2)
              .padding(6.0),
      1.0
    )
    .with_child(
      build_eg("EG1", &params.eg1, synth_client.clone())
              .lens(Model::eg1)
              .padding(6.0),
      1.0
    )
}

fn build_osc<F: Float + 'static>(title: &str,
                                 params: &OscParamsInfo,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<OscModel> {

  build_panel(title, Flex::row()
    .with_child(
      build_knob("Amplitude", "", &params.amplitude, synth_client.clone())
            .lens(OscModel::amplitude),
      1.0
    )
    .with_child(
      build_knob("Shape", "", &params.shape, synth_client.clone())
            .lens(OscModel::shape),
      1.0
    )
    .with_child(
      build_knob("Octaves", "", &params.octave, synth_client.clone())
            .lens(OscModel::octave),
      1.0
    )
    .with_child(
      build_knob("Semitones", "", &params.semitones, synth_client.clone())
            .lens(OscModel::semitones),
      1.0
    )
    .with_child(
      build_knob("Cents", "", &params.cents, synth_client.clone())
            .lens(OscModel::cents),
      1.0
    )
  )
}

fn build_eg<F: Float + 'static>(title: &str,
                                params: &EgParamsInfo,
                                synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<EgModel> {

  build_panel(title, Flex::row()
      .with_child(
        build_knob("Attack", " s", &params.attack, synth_client.clone())
            .lens(EgModel::attack),
        1.0
      )
      .with_child(
        build_knob("Decay", " s", &params.decay, synth_client.clone())
            .lens(EgModel::decay),
        1.0
      )
      .with_child(
        build_knob("Sustain", " s", &params.sustain, synth_client.clone())
            .lens(EgModel::sustain),
        1.0
      )
      .with_child(
        build_knob("Release", " s", &params.release, synth_client.clone())
            .lens(EgModel::release),
        1.0
      )
      .with_child(
        build_knob("Mode", "", &params.mode, synth_client.clone())
            .lens(EgModel::mode),
        1.0
      )
      .with_child(
        build_knob("Intensity", "", &params.dca_intensity, synth_client.clone())
            .lens(EgModel::dca_intensity),
        1.0
      )
  )
}

fn build_panel<T: Data>(title: &str, widget: impl Widget<T> + 'static) -> impl Widget<T> {
  let header = Container::new(Label::new(title).padding((8.0, 4.0, 0.0, 2.0)))
      .rounded(4.0)
      // .background(Color::WHITE)
      .border(Color::WHITE, 1.0);

  let body = Container::new(widget.padding(6.0))
      .rounded(4.0)
      .border(Color::WHITE, 1.0);

  Flex::column()
      .with_child(header, 0.0)
      .with_child(body, 1.0)
}

fn build_knob<F: Float + 'static>(title: &str,
                                  unit: &'static str,
                                  param: &ParamInfo,
                                  synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<KnobModel> {

  let step = param.step.max(0.001);
  let precision = (-step.log10().floor()).min(3.0) as usize;
  let value_label = Label::new(move |data: &KnobModel, _env: &_| {
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  });

  let param_ref = param.param_ref;
  let callback = move |data: &KnobModel| {
    synth_client.lock().unwrap().send_param_value(param_ref, F::val(data.value));
  };

  Flex::column()
    .with_child(Label::new(title).center(), 0.0)
    .with_child(
      Knob::new(param.center, param.min, param.max, param.step, callback)
              .fix_size(56.0, 56.0),
      0.0
    )
    .with_child(value_label.center(), 0.0)
}
