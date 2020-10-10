mod effects;
mod header;
pub mod modulations;
mod synth;
mod helpers;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use kiro_synth_dsp::float::Float;

use druid::widget::{
  Controller, CrossAxisAlignment, Flex, Label, ViewSwitcher, WidgetExt,
};
use druid::{
  Env, Event, EventCtx, LifeCycle, LifeCycleCtx, TimerToken, Widget,
};

use crate::synth::SynthClient;
use crate::ui::data::AppData;
use crate::ui::view::header::HeaderView;
use crate::ui::view::modulations::{ModulationController, ModulationsView};
use crate::ui::data::header::SelectedView;

pub use helpers::*;

// use druid::Selector;
//
// struct AnimFeedbackController;
//
// impl AnimFeedbackController {
//   const UPDATE_FEEDBACK: Selector<()> = Selector::new("synth.update-feedback");
//
//   pub fn new() -> Self {
//     AnimFeedbackController
//   }
// }
//
// impl<W: Widget<App>> Controller<App, W> for AnimFeedbackController {
//   fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut App, env: &Env) {
//     match event {
//       Event::Command(command) if command.is(Self::UPDATE_FEEDBACK) => {
//         data.update_feedback();
//         ctx.request_paint();
//       }
//       _ => child.event(ctx, event, data, env),
//     }
//   }
//
//   fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &App, env: &Env) {
//     match event {
//       LifeCycle::WidgetAdded => ctx.request_anim_frame(),
//       LifeCycle::AnimFrame(_) => {
//         ctx.submit_command(Self::UPDATE_FEEDBACK, None);
//         ctx.request_anim_frame();
//       },
//       _ => {}
//     }
//     child.lifecycle(ctx, event, data, env)
//   }
// }

pub struct TimerFeedbackController {
  timer_token: Option<TimerToken>,
}

impl TimerFeedbackController {
  const UPDATE_PERIOD: Duration = Duration::from_millis(1000 / 30);

  pub fn new() -> Self {
    TimerFeedbackController { timer_token: None }
  }
}

impl<W: Widget<AppData>> Controller<AppData, W> for TimerFeedbackController {
  fn event(
    &mut self,
    child: &mut W,
    ctx: &mut EventCtx,
    event: &Event,
    data: &mut AppData,
    env: &Env,
  ) {
    match event {
      Event::Timer(token) if Some(*token) == self.timer_token => {
        self.timer_token = Some(ctx.request_timer(Self::UPDATE_PERIOD));
        data.update_feedback();
        ctx.request_paint();
      }
      _ => child.event(ctx, event, data, env),
    }
  }

  fn lifecycle(
    &mut self,
    child: &mut W,
    ctx: &mut LifeCycleCtx,
    event: &LifeCycle,
    data: &AppData,
    env: &Env,
  ) {
    if let LifeCycle::WidgetAdded = event {
      self.timer_token = Some(ctx.request_timer(Self::UPDATE_PERIOD))
    }
    child.lifecycle(ctx, event, data, env);
  }
}

pub fn build<F: Float + 'static>(
  synth_client: Arc<Mutex<SynthClient<F>>>,
) -> impl Widget<AppData> + 'static {
  let header = HeaderView::build().lens(AppData::header);

  let body = ViewSwitcher::new(
    |data: &AppData, _env: &Env| data.header.selected_view,
    move |view: &SelectedView, data: &AppData, _env: &Env| {
      match view {
        SelectedView::Presets => build_presets_view(),
        SelectedView::Synth => build_synth_view(&data, synth_client.clone()),
        SelectedView::Effects => build_effects_view(),
      }
    });

  Flex::column()
    .with_child(header)
    .with_spacer(4.0)
    .with_flex_child(body, 1.0)
    .controller(TimerFeedbackController::new())
  // .debug_widget_id()
  // .debug_paint_layout()
}

fn build_presets_view() -> Box<dyn Widget<AppData>> {
  Flex::row()
      .with_child(Label::new("TODO: Presets"))
      .with_flex_spacer(1.0)
      .cross_axis_alignment(CrossAxisAlignment::Start)
      .boxed()
}

fn build_synth_view<F: Float + 'static>(data: &&AppData, synth_client: Arc<Mutex<SynthClient<F>>>) -> Box<dyn Widget<AppData>> {
  let synth = synth::build(&data.synth, synth_client)
      .lens(AppData::synth);

  let modulations = ModulationsView::build()
      .lens(AppData::modulations)
      .controller(ModulationController::new());

  Flex::row()
      .with_child(synth.fix_width(330.0))
      .with_flex_child(modulations, 1.0)
      .cross_axis_alignment(CrossAxisAlignment::Start)
      .boxed()
}

fn build_effects_view() -> Box<dyn Widget<AppData>> {
  let effects = Label::new("TODO: Effects");

  let modulations = ModulationsView::build()
      .lens(AppData::modulations)
      .controller(ModulationController::new());

  Flex::row()
      .with_child(effects.fix_width(330.0))
      .with_flex_child(modulations, 1.0)
      .cross_axis_alignment(CrossAxisAlignment::Start)
      .boxed()
}
