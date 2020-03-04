use std::sync::{Arc, Mutex};

use druid::{Widget, Color, Data};
use druid::widget::{Flex, WidgetExt, Label, Container};

use kiro_synth_core::float::Float;

use crate::ui::model::{SynthData, Osc, EnvGen, ParamToKnobData, Param};
use crate::ui::knob::{KnobData, Knob};
use crate::synth::SynthClient;


pub fn build<F: Float + 'static>(data: &SynthData,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthData> {

  Flex::column()
    .with_child(
      build_osc("OSC1", &data.osc1, synth_client.clone())
              .lens(SynthData::osc1)
              .padding(6.0),
      1.0
    )
    .with_child(
      build_osc("OSC2", &data.osc2, synth_client.clone())
              .lens(SynthData::osc2)
              .padding(6.0),
      1.0
    )
    .with_child(
      build_eg("EG1", &data.eg1, synth_client.clone())
              .lens(SynthData::eg1)
              .padding(6.0),
      1.0
    )
}

fn build_osc<F: Float + 'static>(title: &str,
                                 data: &Osc,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Osc> {

  build_panel(title, Flex::row()
    .with_child(
      build_knob("Amplitude", "", &data.amplitude, synth_client.clone())
            .lens(Osc::amplitude),
      1.0
    )
    .with_child(
      build_knob("Shape", "", &data.shape, synth_client.clone())
            .lens(Osc::shape),
      1.0
    )
    .with_child(
      build_knob("Octaves", "", &data.octaves, synth_client.clone())
            .lens(Osc::octaves),
      1.0
    )
    .with_child(
      build_knob("Semitones", "", &data.semitones, synth_client.clone())
            .lens(Osc::semitones),
      1.0
    )
    .with_child(
      build_knob("Cents", "", &data.cents, synth_client.clone())
            .lens(Osc::cents),
      1.0
    )
  )
}

fn build_eg<F: Float + 'static>(title: &str,
                                data: &EnvGen,
                                synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<EnvGen> {

  build_panel(title, Flex::row()
      .with_child(
        build_knob("Attack", " s", &data.attack, synth_client.clone())
              .lens(EnvGen::attack),
        1.0
      )
      .with_child(
        build_knob("Decay", " s", &data.decay, synth_client.clone())
              .lens(EnvGen::decay),
        1.0
      )
      .with_child(
        build_knob("Sustain", " s", &data.sustain, synth_client.clone())
              .lens(EnvGen::sustain),
        1.0
      )
      .with_child(
        build_knob("Release", " s", &data.release, synth_client.clone())
              .lens(EnvGen::release),
        1.0
      )
      .with_child(
        build_knob("Mode", "", &data.mode, synth_client.clone())
              .lens(EnvGen::mode),
        1.0
      )
      .with_child(
        build_knob("Intensity", "", &data.dca_intensity, synth_client.clone())
              .lens(EnvGen::dca_intensity),
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
                                  param: &Param,
                                  synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  let step = param.step.max(0.001);
  let precision = (-step.log10().floor()).min(3.0) as usize;
  let value_label = Label::new(move |data: &KnobData, _env: &_| {
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  });

  let param_ref = param.param_ref;
  let callback = move |data: &KnobData| {
    synth_client.lock().unwrap().send_param_value(param_ref, F::val(data.value));
  };

  Flex::column()
    .with_child(Label::new(title).center(), 0.0)
    .with_child(
      Knob::new(param.origin, param.min, param.max, param.step, callback)
              .fix_size(56.0, 56.0),
      0.0
    )
    .with_child(value_label.center(), 0.0)
    .lens(ParamToKnobData)
}
