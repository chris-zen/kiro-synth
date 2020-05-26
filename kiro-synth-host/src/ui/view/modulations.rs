use std::sync::{Arc, Mutex};

use druid::{Widget, Insets, lens::{self, LensExt}};
use druid::widget::{List, Flex, Label, Scroll, Container, WidgetExt, CrossAxisAlignment};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, ParamModulation, Modulator, Param};
use crate::ui::GREY_83;
use crate::ui::widgets::knob::{Knob, KnobData};

pub struct ModulationsView;

impl ModulationsView {
  pub fn new<F: Float + 'static>(_synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

    let list = List::new(|| {
      Flex::column()
          .with_child(Self::param_modulation())
          .with_child(Self::modulators())
    });

    let scroll = Scroll::new(list).vertical();

    Container::new(scroll)
        .rounded(2.0)
        .border(GREY_83, 2.0)
        .padding(4.0)
        .lens(SynthModel::param_modulations)
  }

  fn param_modulation() -> impl Widget<ParamModulation> {
    Flex::row()
        .with_flex_child(
          Label::new(|item: &ParamModulation, _env: &_| item.name.clone())
              .expand_width()
              .height(24.0),
          1.0
        )
        .with_child(
          Label::new("+").fix_height(24.0)
        )
  }

  fn modulators() -> impl Widget<ParamModulation> {
    List::new(|| {
      Self::modulator()
    })//.lens(ParamModulator::modulators)
    .lens(lens::Id.map(
      |data: &ParamModulation| (data.param.clone(), data.modulators.clone()),
      |_data: &mut ParamModulation, _list_data: (Param, Arc<Vec<Modulator>>)| {},
    ))
  }

  fn modulator() -> impl Widget<(Param, Modulator)> {
    let name = Label::new(|item: &(Param, Modulator), _env: &_| item.1.name.clone())
        .fix_height(24.0);

    let value = Label::new(|item: &(Param, Modulator), _env: &_| format!("{:.3}", item.1.amount))
        .fix_height(24.0);

    let name_and_value = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(name)
        .with_child(value)
        .padding(Insets::new(8.0, 0.0, 0.0, 0.0))
        .expand_width();

    let callback = move |_param: &Param, _data: &KnobData| {
      // param.send_value(data.value).unwrap();
    };

    let knob = Knob::new(callback)
        .center()
        .fix_size(48.0, 48.0)
        .lens(lens::Id.map(
          |data: &(Param, Modulator)| {
            let (param, _) = data;
            (param.clone(), KnobData::new(param.origin, param.min, param.max, param.step, param.value, param.modulation))
          },
          |data: &mut (Param, Modulator), knob_data: (Param, KnobData)| {
            let (param, _) = data;
            param.value = knob_data.1.value
          }
        ));

    // let knob = Label::new(|item: &(Param, Modulator), _env: &_| "---".to_string());

    Flex::row()
        .with_flex_child(name_and_value, 1.0)
        .with_child(knob)
  }
}

