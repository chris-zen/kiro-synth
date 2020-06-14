use std::sync::{Arc, Mutex};
use std::marker::PhantomData;

use druid::{Widget, lens::{self, LensExt}, UnitPoint, Env, EventCtx, Command, Selector, Data, Event, LifeCycleCtx, LifeCycle};
use druid::widget::{List, Flex, Label, Scroll, Container, WidgetExt, CrossAxisAlignment, SizedBox, ViewSwitcher, Button, FillStrat, Click, Either, Controller};
use druid::theme::WINDOW_BACKGROUND_COLOR;
use druid::im::Vector;

use druid_icon::Icon;

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::SourceRef;

use crate::synth::SynthClient;
use crate::ui::{GREY_83, GREY_46};
use crate::ui::model::{SynthModel, Param};
use crate::ui::widgets::knob::{Knob, KnobData};
use crate::ui::model::modulations::{Group, Modulation, View, Modulations, Source, ConfigMode, Reference};
use crate::ui::view::{build_static_tabs, build_switcher};
use crate::ui::icons;


pub const MODULATION_BEGIN: Selector<SourceRef> = Selector::new("synth.modulation.begin");
pub const MODULATION_DONE: Selector<SourceRef> = Selector::new("synth.modulation.done");


pub struct ModulationController<T: Data> {
  _phantom: PhantomData<T>
}

impl<T: Data> ModulationController<T> {
  pub fn new() -> Self {
    ModulationController {
      _phantom: PhantomData
    }
  }
}

impl<W: Widget<Param>> Controller<Param, W> for ModulationController<Param> {
  fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut Param, env: &Env) {
    match event {
      Event::Command(command) => {
        data.value = data.value + 0.1;
        // println!("{:#?}", command);
      }
      _ => {}
    }

    child.event(ctx, event, data, env);
  }
}

impl<W: Widget<SynthModel>> Controller<SynthModel, W> for ModulationController<SynthModel> {
  fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut SynthModel, env: &Env) {
    match event {
      Event::Command(command) if command.is(MODULATION_BEGIN) => {
        if let Some(source_ref) = command.get::<SourceRef>(MODULATION_BEGIN) {
          println!("{:#?} {:?}", command, source_ref);
          data.modulations.begin(*source_ref);
        }
      }
      Event::Command(command) if command.is(MODULATION_DONE) => {
        if let Some(source_ref) = command.get::<SourceRef>(MODULATION_DONE) {
          println!("{:#?} {:?}", command, source_ref);
          data.modulations.done(*source_ref);
        }
      }
      _ => {}
    }

    child.event(ctx, event, data, env);
  }
}

pub struct ModulationsView;

impl ModulationsView {
  pub fn new<F: Float + 'static>(_synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

    let views = vec![
      View::GroupBySource,
      View::GroupByParam,
    ];
    let tabs = build_static_tabs(views, Self::build_tab)
        .lens(Modulations::view);

    let body = ViewSwitcher::new(
      |data: &Modulations, _: &Env| data.view,
      |view: &View, data: &Modulations, _: &Env| {
        match view {
          View::GroupBySource => Box::new(Self::build_modulations_list()),
          View::GroupByParam => Box::new(Self::build_modulations_list()),
        }
      }
    );

    let body = Container::new(body)
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

  fn build_tab(_index: usize, data: &View) -> impl Widget<View> {
    let icon = match data {
      View::GroupBySource => &icons::MODULATION_SOURCE,
      View::GroupByParam => &icons::MODULATION_PARAM,
    };

    Icon::new(icon)
        .fix_height(12.0)
        .center()
        .padding((6.0, 4.0, 4.0, 2.0))
  }

  fn build_modulations_list() -> impl Widget<Modulations> {
    let list = List::new(|| {
      Flex::column()
          .with_child(Self::build_group())
          .with_child(Self::build_modulation_knobs())
    });

    Scroll::new(list.padding((4.0, 0.0))).vertical()
  }

  fn build_group() -> impl Widget<Group> {

    let name = Label::new(|data: &Group, _env: &_| data.name.clone())
        .padding((0.0, 3.0))
        .expand_width()
        .height(20.0);

    let config_mode = ViewSwitcher::new(
      |data: &Group, _: &Env| data.config_mode,
      |config_mode: &ConfigMode, data: &Group, _: &Env| {
        println!("{:?} {:?}", data.name, data.config_mode);
        if let Reference::Source(source_ref) = data.reference {
          match config_mode {
            ConfigMode::Ready => {
              Icon::new(&icons::MODULATION_NEW)
                  .fill_strategy(FillStrat::ScaleDown)
                  .fix_height(8.0)
                  .on_click(move |ctx: &mut EventCtx, data: &mut Group, env| {
                    let command = Command::new(MODULATION_BEGIN, source_ref);
                    ctx.submit_command(command, None);
                  })
                  .boxed()
            }
            ConfigMode::Ongoing => {
              Icon::new(&icons::MODULATION_NEW)
                  .fill_strategy(FillStrat::ScaleDown)
                  .fix_height(8.0)
                  .on_click(move |ctx: &mut EventCtx, data: &mut Group, env| {
                    let command = Command::new(MODULATION_DONE, source_ref);
                    ctx.submit_command(command, None);
                  })
                  .boxed()
            }
            ConfigMode::Disabled => {
              SizedBox::empty().fix_height(8.0).boxed()
            }
          }
        }
        else {
          SizedBox::empty().fix_height(8.0).boxed()
        }
      },
    );

    Flex::row()
        .with_flex_child(name, 1.0)
        .with_child(config_mode)
  }

  fn build_modulation_knobs() -> impl Widget<Group> {
    List::new(|| Self::build_modulation_knob())
      .lens(lens::Id.map(
        |data: &Group| data.modulations.clone(),
        |data: &mut Group, list_data: Vector<Modulation>| data.modulations = list_data,
      ))
  }

  fn build_modulation_knob() -> impl Widget<Modulation> {
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

  fn build_add_modulation() -> impl Widget<Modulations> {
    let list = Scroll::new(List::new(|| {
        Label::new(|source: &Source, _: &Env| source.name.clone())
      }).expand_width()
    ).vertical().lens(Modulations::sources);

    let sources = Container::new(list)
        .rounded(2.0)
        .background(WINDOW_BACKGROUND_COLOR)
        .expand_height();

    let sources = Flex::column()
        .with_child(Label::new("Sources").padding(2.0))
        .with_flex_child(sources, 1.0);

    let sources = Container::new(sources)
        .background(GREY_83);

    let params = Container::new(SizedBox::empty().expand())
        .rounded(2.0)
        .background(WINDOW_BACKGROUND_COLOR);
        // .padding((0.0, 0.0, 0.0, 2.0));

    let params = Flex::column()
        .with_child(Label::new("Parameters").padding(2.0))
        .with_flex_child(params, 1.0);

    let params = Container::new(params)
        .background(GREY_83);

    let add_button = Button::new("Add")
        .expand_width()
        .padding(2.0);

    Flex::column()
        .with_child(add_button)
        .with_flex_child(sources, 1.0)
        .with_flex_child(params, 1.0)
  }
}
