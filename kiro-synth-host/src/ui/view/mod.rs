mod oscillators;
mod filters;
mod dca;
mod modulators;
mod modulations;

use std::sync::{Arc, Mutex};

use druid::{Widget, Data, Env, lens::{self, LensExt}};
use druid::widget::{Flex, WidgetExt, Label, Container, ViewSwitcher, CrossAxisAlignment};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, Param};
use crate::ui::widgets::knob::{KnobData, Knob};
use crate::ui::{GREY_83, GREY_65, GREY_74};
use crate::ui::widgets::tab::Tab;

use oscillators::OscillatorsView;
use filters::FiltersView;
use dca::DcaView;
use modulators::ModulatorsView;
use modulations::ModulationsView;


pub fn build<F: Float + 'static>(synth_model: &SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  let height = 114.0;
  let devices = Flex::column()
    .with_child(
      OscillatorsView::new(synth_model, synth_client.clone())
              .fix_height(height)
              .padding(4.0)
    )
    .with_child(
      Flex::row()
          .with_flex_child(
            FiltersView::new(synth_model)
                    .fix_height(height)
                    .padding(4.0),
            1.0
          )
          .with_child(
            DcaView::new(synth_model)
                    .fix_height(height)
                    .padding(4.0)
          )
          .must_fill_main_axis(true)
    )
    .with_child(
      ModulatorsView::new(synth_model, synth_client.clone())
              .fix_height(height * 2.0)
              .padding(4.0)
    )
    .cross_axis_alignment(CrossAxisAlignment::Start);

  let modulators = ModulationsView::new(synth_client.clone());

  Flex::row()
    .with_child(devices.fix_width(330.0))
    .with_flex_child(modulators, 1.0)
    // .debug_widget_id()
    // .debug_paint_layout()
}

pub fn build_tabs(n: usize, title: impl Fn(usize) -> String + 'static) -> impl Widget<usize> {
  let mut tabs = Flex::row();
  tabs.add_spacer(2.0);
  for tab_index in 0..n {
    let label = Label::<usize>::new((title)(tab_index))
        .padding((6.0, 4.0, 4.0, 2.0));

    let on_click = move |index: &mut usize, _env: &Env| *index = tab_index;
    let is_selected = move |index: &usize| *index == tab_index;
    let tab = Tab::new(label, on_click, is_selected)
        .border_width(2.0)
        .selected_border_color(GREY_83)
        .unselected_border_color(GREY_65)
        .hover_border_color(GREY_74)
        .selected_background(GREY_83)
        .unselected_background(GREY_65)
        .hover_background(GREY_74)
        .corner_radius(2.0);

    tabs.add_child(tab);
    tabs.add_spacer(4.0);
  }
  tabs
}

pub fn build_switcher<T, U, W>(tabs: W,
                               child_picker: impl Fn(&T, &Env) -> U + 'static,
                               child_builder: impl Fn(&U, &T, &Env) -> Box<dyn Widget<T>> + 'static) -> impl Widget<T>
  where T: Data, U: PartialEq + 'static, W: Widget<T> + 'static {

  let switcher = ViewSwitcher::new(child_picker, child_builder)
      .padding(6.0);

  let body = Container::new(switcher)
      .rounded(2.0)
      .border(GREY_83, 2.0);

  Flex::column()
      .with_child(tabs)
      .with_child(body)
      .cross_axis_alignment(CrossAxisAlignment::Start)
}

pub fn build_knob_value(title: &'static str,
                        unit: &'static str) -> impl Widget<Param> {

  let value_fn = move |data: &KnobData| {
    let step = data.step.max(0.001);
    let precision = (-step.log10().floor()).max(0.0).min(3.0) as usize;
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  };

  build_knob(title, value_fn)
}

pub fn build_knob_enum(title: &'static str,
                       value_fn: impl Fn(usize) -> String + 'static) -> impl Widget<Param> {

  build_knob(title, move |data: &KnobData| value_fn(data.value as usize))
}

pub fn build_knob(title: &'static str,
                  value_fn: impl Fn(&KnobData) -> String + 'static) -> impl Widget<Param> {

  let callback = move |param: &Param, data: &KnobData| {
    param.send_value(data.value).unwrap();
  };

  Flex::column()
    .with_child(Label::new(title).center().fix_width(48.0))
    .with_child(Knob::new(callback).center().fix_size(48.0, 48.0))
    .with_child(Label::new(move |data: &(Param, KnobData), _env: &Env| value_fn(&data.1))
        .center()
        .fix_width(48.0)
    )
    .lens(lens::Id.map(
      |param: &Param| (param.clone(), KnobData::new(param.origin, param.min, param.max, param.step, param.value, param.modulation)),
      |param: &mut Param, data: (Param, KnobData)| param.value = data.1.value
    ))
}
