use std::sync::{Arc, Mutex};

use druid::{Widget, lens::{self, LensExt}, UnitPoint, Color};
use druid::widget::{List, Flex, Label, Scroll, Container, WidgetExt, CrossAxisAlignment, SizedBox};
use druid::im::Vector;

use druid_icon::Icon;

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::GREY_83;
use crate::ui::model::SynthModel;
use crate::ui::widgets::knob::{Knob, KnobData};
use crate::ui::model::modulations::{Group, Modulation, GroupBy, Modulations};
use crate::ui::view::{build_static_tabs, build_switcher};
use crate::ui::icons;


pub struct ModulationsView;

impl ModulationsView {
  pub fn new<F: Float + 'static>(_synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

    let list = List::new(|| {
      Flex::column()
          .with_child(Self::group())
          .with_child(Self::modulations())
    });

    let scroll = Scroll::new(list).vertical();

    let tabs = build_static_tabs(vec![GroupBy::Source, GroupBy::Param],
                                          |_index: usize, data: &GroupBy| {

      let icon = match data {
        GroupBy::Source => &icons::MODULATION_SOURCE,
        GroupBy::Param => &icons::MODULATION_PARAM,
      };
      Icon::<GroupBy>::new(icon)
          .color(Color::WHITE)
          .fix_height(12.0)
          .center()
          .padding((6.0, 4.0, 4.0, 2.0))

      // Label::new(match data {
      //   GroupBy::Source => "By Source".to_string(),
      //   GroupBy::Param => "By Param".to_string(),
      // }).padding((6.0, 4.0, 4.0, 2.0))
    }).lens(Modulations::group_by);

    let body = Container::new(scroll)
        .rounded(2.0)
        .border(GREY_83, 2.0)
        .padding((4.0, 0.0, 4.0, 4.0))
        .expand_height();

    Flex::column()
        .with_child(tabs.padding((4.0, 4.0, 0.0, 0.0)))
        .with_flex_child(body, 1.0)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .lens(SynthModel::modulations)
  }

  fn group() -> impl Widget<Group> {
    Flex::row()
        .with_flex_child(
          Label::new(|data: &Group, _env: &_| data.name.clone())
              .padding((0.0, 3.0))
              .expand_width()
              .height(20.0),
          1.0
        )
        .with_child(
          Label::new("+")
              .padding((0.0, 3.0))
              .fix_height(20.0)
        )
  }

  fn modulations() -> impl Widget<Group> {
    List::new(|| {
      Self::modulation()
    })
    .lens(lens::Id.map(
      |data: &Group| data.modulations.clone(),
      |data: &mut Group, list_data: Vector<Modulation>| data.modulations = list_data,
    ))
  }

  fn modulation() -> impl Widget<Modulation> {
    let name = Label::new(|data: &Modulation, _env: &_| data.name.clone())
        .align_vertical(UnitPoint::new(0.0, 0.5))
        .fix_height(19.0);

    // TODO formatting according to `step`
    let value = Label::new(|data: &Modulation, _env: &_| format!("{:.3}", data.amount))
        .align_vertical(UnitPoint::new(0.0, 0.5))
        .fix_height(19.0);

    let name_and_value = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(name)
        .with_child(value)
        .expand_width();

    let callback = move |data: &Modulation, knob_data: &KnobData| {
      data.send_modulation_amount(knob_data.value).unwrap();
    };

    let knob = Knob::new(callback)
        .center()
        .fix_size(38.0, 38.0)
        .lens(lens::Id.map(
          |data: &Modulation| {
            (
              data.clone(),
              KnobData::new(data.origin, data.min, data.max, data.step, data.amount, 0.0)
            )
          },
          |data: &mut Modulation, knob_data: (Modulation, KnobData)| {
            data.amount = knob_data.1.value
          }
        ));

    Flex::row()
        .with_child(knob)
        .with_flex_child(name_and_value, 1.0)
  }
}
