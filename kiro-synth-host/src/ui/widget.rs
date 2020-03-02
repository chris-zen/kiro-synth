use druid::{Widget, Color};
use druid::widget::{Flex, WidgetExt, Label, Container};

use crate::ui::model::{Model, OscModel};
use crate::ui::params::{ParamInfo, OscParamsInfo};
use crate::ui::knob::{KnobModel, Knob};
use crate::ui::SynthParams;


pub fn build(synth_params: &SynthParams) -> impl Widget<Model> {
  Flex::column()
    .with_child(build_osc("OSC1", &synth_params.osc1).lens(Model::osc1).padding(6.0), 1.0)
    .with_child(build_osc("OSC2", &synth_params.osc2).lens(Model::osc2).padding(6.0), 1.0)
}

fn build_osc(title: &str, params: &OscParamsInfo) -> impl Widget<OscModel> {
  let row = Flex::row()
    .with_child(build_knob("Amplitude", "", &params.amplitude).lens(OscModel::amplitude), 1.0)
    .with_child(build_knob("Shape", "", &params.shape).lens(OscModel::shape), 1.0)
    .with_child(build_knob("Octave", "", &params.octave).lens(OscModel::octave), 1.0)
    .with_child(build_knob("Semitones", "", &params.semitones).lens(OscModel::semitones), 1.0)
    .with_child(build_knob("Cents", "", &params.cents).lens(OscModel::cents), 1.0);

  let header = Container::new(Label::new(title).padding((8.0, 4.0, 0.0, 2.0)))
      .rounded(4.0)
      // .background(Color::WHITE)
      .border(Color::WHITE, 1.0);

  let body = Container::new(row.padding(6.0))
      .rounded(4.0)
      .border(Color::WHITE, 1.0);

  Flex::column()
    .with_child(header, 0.0)
    .with_child(body, 1.0)
}

fn build_knob(title: &str, unit: &'static str, param: &ParamInfo) -> impl Widget<KnobModel> {
  let step = param.step.max(0.001);
  let precision = (-step.log10().floor()).min(3.0) as usize;
  let value_label = Label::new(move |data: &KnobModel, _env: &_| {
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  });

  Flex::column()
    .with_child(Label::new(title).center(), 0.0)
    .with_child(Knob::new(param.center, param.min, param.max, param.step), 1.0)
    .with_child(value_label.center(), 0.0)
}
