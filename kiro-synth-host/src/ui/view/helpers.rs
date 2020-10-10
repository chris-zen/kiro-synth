use druid::widget::{
  Container, CrossAxisAlignment, Flex, Label, ViewSwitcher, WidgetExt,
};
use druid::{
  Command, Data, Env, UpdateCtx, Widget,
};

use crate::ui::data::param::{KnobDataFromParam, Param};
use crate::ui::view::modulations::UPDATE_MODULATIONS_CONFIG;
use crate::ui::widgets::knob::{Knob, KnobData};
use crate::ui::widgets::tab::Tab;
use crate::ui::{GREY_65, GREY_74, GREY_83};


pub fn build_static_tabs<T, W, F>(tabs_data: Vec<T>, child_builder: F) -> impl Widget<T>
  where
      T: Data,
      W: Widget<T> + 'static,
      F: Fn(usize, &T) -> W,
{
  let mut tabs_row = Flex::row();
  tabs_row.add_spacer(2.0);
  for (index, tab_data) in tabs_data.iter().enumerate() {
    let moved_tab_data = tab_data.clone();
    let on_click = move |data: &mut T, _: &Env| *data = moved_tab_data.clone();
    let moved_tab_data = tab_data.clone();
    let is_selected = move |data: &T| data.same(&moved_tab_data);

    let child = child_builder(index, tab_data);

    let tab = Tab::new(child, on_click, is_selected)
        .border_width(2.0)
        .selected_border_color(GREY_83)
        .unselected_border_color(GREY_65)
        .hover_border_color(GREY_74)
        .selected_background(GREY_83)
        .unselected_background(GREY_65)
        .hover_background(GREY_74)
        .rounded(2.0);

    tabs_row.add_child(tab);
    tabs_row.add_spacer(4.0);
  }
  tabs_row
}

pub fn build_tabs(n: usize, title: impl Fn(usize) -> String + 'static) -> impl Widget<usize> {
  let tabs_data = (0..n).collect::<Vec<usize>>();
  build_static_tabs(tabs_data, move |_index: usize, data: &usize| {
    Label::<usize>::new(title(*data)).padding((6.0, 4.0, 4.0, 2.0))
  })
}

pub fn build_switcher<T, U, W>(
  tabs: W,
  child_picker: impl Fn(&T, &Env) -> U + 'static,
  child_builder: impl Fn(&U, &T, &Env) -> Box<dyn Widget<T>> + 'static,
) -> impl Widget<T>
  where
      T: Data,
      U: PartialEq + 'static,
      W: Widget<T> + 'static,
{
  let switcher = ViewSwitcher::new(child_picker, child_builder).padding(6.0);

  let body = Container::new(switcher).rounded(2.0).border(GREY_83, 2.0);

  Flex::column()
      .with_child(tabs)
      .with_child(body)
      .cross_axis_alignment(CrossAxisAlignment::Start)
}

pub fn build_knob_value(title: &'static str, unit: &'static str) -> impl Widget<Param> {
  let value_fn = move |data: &KnobData<Param>| {
    let step = data.step.max(0.001);
    let precision = (-step.log10().ceil()).max(0.0).min(3.0) as usize;
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  };

  build_knob(title, value_fn)
}

pub fn build_knob_enum(
  title: &'static str,
  value_fn: impl Fn(usize) -> String + 'static,
) -> impl Widget<Param> {
  build_knob(title, move |data: &KnobData<Param>| {
    value_fn(data.value as usize)
  })
}

pub fn build_knob(
  title: &'static str,
  value_fn: impl Fn(&KnobData<Param>) -> String + 'static,
) -> impl Widget<Param> {
  let callback = move |ctx: &mut UpdateCtx, data: &KnobData<Param>| {
    match data.context.modulation.config_source {
      Some(source_ref) => {
        let param_ref = data.context.param_ref;
        let config_amount = data.modulation.config_amount;
        // println!("parm: callback: {:?} {:?} {:?}", source_ref, param_ref, config_amount);
        let payload = (source_ref, param_ref, config_amount);
        let command = Command::new(UPDATE_MODULATIONS_CONFIG, payload);
        ctx.submit_command(command, None)
      }
      None => data
          .context
          .synth_client
          .send_param_value(data.context.param_ref, data.value as f32)
          .unwrap(),
    }
  };

  let knob = Knob::new(callback)
      .modulation_width(4.0)
      .padding(2.0)
      .center()
      .fix_size(48.0, 48.0);

  Flex::column()
      .with_child(Label::new(title).center().fix_width(48.0))
      .with_child(knob)
      .with_child(
        Label::new(move |data: &KnobData<Param>, _env: &Env| value_fn(data))
            .center()
            .fix_width(48.0),
      )
      .lens(KnobDataFromParam)
}
