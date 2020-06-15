use core::f64::consts::{PI, FRAC_PI_2};

use druid::kurbo::{Arc, Shape};
use druid::{Widget, BoxConstraints, Color, Env, Event, Data, Size, Point, Vec2, LifeCycle, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, RenderContext, UpdateCtx, KeyOrValue};
use std::marker::PhantomData;
use crate::ui::widgets::knob::theme::{KNOB_VALUE_FG, KNOB_VALUE_BG, KNOB_MODULATION_FG, KNOB_MODULATION_BG};

pub mod theme {
  use druid::{Key, Color, Env};
  pub use druid::theme::*;

  pub const KNOB_VALUE_FG: Key<Color> = Key::new("knob-value-fg");
  pub const KNOB_VALUE_BG: Key<Color> = Key::new("knob-value-bg");
  pub const KNOB_MODULATION_FG: Key<Color> = Key::new("knob-modulation-fg");
  pub const KNOB_MODULATION_BG: Key<Color> = Key::new("knob-modulation-bg");

  pub fn init(env: &mut Env) {
    env.set(KNOB_VALUE_FG, Color::WHITE);
    env.set(KNOB_VALUE_BG, Color::BLACK);
    env.set(KNOB_MODULATION_FG, Color::rgb(0.8, 0.3, 0.0));
    env.set(KNOB_MODULATION_BG, Color::rgb(0.1, 0.03, 0.0));
  }
}

const ARC_TOLERANCE: f64 = 0.1;


#[derive(Debug, Clone, Data)]
pub struct KnobModulationData {
  pub min: f64,
  pub max: f64,
  pub value: f64,
  pub weight: Option<f64>,
}

impl KnobModulationData {
  pub fn new(min: f64, max: f64, value: f64) -> Self {
    KnobModulationData {
      min,
      max,
      value,
      weight: None
    }
  }
}

#[derive(Debug, Clone, Data)]
pub struct KnobData<T> {
  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub value: f64,
  pub modulation: KnobModulationData,

  #[data(ignore)]
  pub context: T,
}

impl<T: Data> KnobData<T> {
  pub fn new(origin: f64, min: f64, max: f64, step: f64, value: f64, context: T) -> Self {
    KnobData {
      origin,
      min,
      max,
      step,
      value,
      modulation: KnobModulationData::new(min, max, 0.0),
      context,
    }
  }

  pub fn with_modulation_value(mut self, value: f64) -> Self {
    self.modulation.value = value;
    self
  }

  pub fn with_modulation_weight(mut self, weight: Option<f64>) -> Self {
    self.modulation.weight = weight;
    self
  }
}

struct MouseMove {
  orig_pos: f64,
  orig_value: f64,
}

pub struct Knob<Context, Callback>
  where
      Context: Data,
      Callback: Fn(&KnobData<Context>) -> () {

  callback: Callback,

  value_width: f64,
  value_fg_color: KeyOrValue<Color>,
  value_bg_color: KeyOrValue<Color>,

  modulation_width: f64,
  modulation_fg_color: KeyOrValue<Color>,
  modulation_bg_color: KeyOrValue<Color>,

  sensitivity: f64,
  mouse_move: MouseMove,

  _phantom: PhantomData<Context>,
}

impl<Context, Callback> Knob<Context, Callback>
  where
      Context: Data,
      Callback: Fn(&KnobData<Context>) -> () {
  
  const START_ANGLE: f64 = 2.0 * PI * (20.0 / 360.0);
  const END_ANGLE: f64 = 2.0 * PI * (340.0 / 360.0);

  pub fn new(callback: Callback) -> Self {
    Knob {
      callback,
      value_width: 2.0,
      value_fg_color: KeyOrValue::Key(KNOB_VALUE_FG),
      value_bg_color: KeyOrValue::Key(KNOB_VALUE_BG),
      modulation_width: 0.0,
      modulation_fg_color: KeyOrValue::Key(KNOB_MODULATION_FG),
      modulation_bg_color: KeyOrValue::Key(KNOB_MODULATION_BG),
      sensitivity: 0.6,
      mouse_move: MouseMove { orig_pos: 0.0, orig_value: 0.0 },
      _phantom: PhantomData,
    }
  }

  pub fn value_width(mut self, width: f64) -> Self {
    self.value_width = width;
    self
  }

  pub fn modulation_width(mut self, width: f64) -> Self {
    self.modulation_width = width;
    self
  }

  pub fn sensitivity(mut self, sensitivity: f64) -> Self {
    self.sensitivity = sensitivity;
    self
  }

  fn value_to_angle(&self, value: f64, min: f64, max: f64) -> f64 {
    let range = max - min;
    Self::START_ANGLE + (Self::END_ANGLE - Self::START_ANGLE) * (value - min) / range
  }

  fn paint_arc(paint_ctx: &mut PaintCtx,
               center: Point,
               radius: f64,
               start_angle: f64,
               end_angle: f64,
               color: Color,
               width: f64,
               line_to_center: bool) {

    let sweep_angle = end_angle - start_angle;

    let arc = Arc {
      center,
      radii: Vec2::new(radius, radius),
      start_angle,
      sweep_angle,
      x_rotation: FRAC_PI_2,
    };

    let mut arc_curve = arc.into_bez_path(ARC_TOLERANCE);

    if line_to_center {
      let angle_to = end_angle + FRAC_PI_2;
      let radius_to = radius * 0.30;
      let p_to = Point::new(
        center.x + radius_to * angle_to.cos(),
        center.y + radius_to * angle_to.sin(),
      );
      arc_curve.line_to(p_to);
    }

    paint_ctx.stroke(arc_curve, &color, width);
  }
}

impl<Context, Callback> Widget<KnobData<Context>> for Knob<Context, Callback>
  where
      Context: Data,
      Callback: Fn(&KnobData<Context>) -> () {
  
  fn event(&mut self,
           ctx: &mut EventCtx,
           event: &Event,
           data: &mut KnobData<Context>,
           _env: &Env) {

    // println!("event {:?}: {:#?}", self.id(), event);

    match event {
      Event::MouseDown(mouse) => {
        ctx.set_active(true);
        self.mouse_move = MouseMove {
          orig_pos: mouse.pos.y,
          orig_value: data.value,
        };
        ctx.request_paint();
      }
      Event::MouseUp(_mouse) => {
        if ctx.is_active() {
          ctx.set_active(false);
          ctx.request_paint();
        }
      }
      Event::MouseMove(mouse) => {
        if ctx.is_active() {
          let height = ctx.size().height;
          let offset = self.mouse_move.orig_pos - mouse.pos.y;
          let value_inc = (data.max - data.min) * (self.sensitivity * offset / height);
          let value = (self.mouse_move.orig_value + value_inc).max(data.min).min(data.max);
          data.value = (value / data.step).round() * data.step;
          ctx.request_paint();
        }
      }
      _ => (),
    }
  }

  fn lifecycle(
    &mut self,
    _ctx: &mut LifeCycleCtx,
    _event: &LifeCycle,
    _data: &KnobData<Context>,
    _env: &Env,
  ) {
    // println!("lifecycle {:?}: {:#?}", self.id(), _event);
  }

  fn update(&mut self,
            _ctx: &mut UpdateCtx,
            _old_data: &KnobData<Context>,
            data: &KnobData<Context>,
            _env: &Env) {
    // println!("update {:?}: {} -> {}", self.id(), _old_data.1.value, data.1.value);
    (self.callback)(&data);
  }

  fn layout(
    &mut self,
    _layout_ctx: &mut LayoutCtx,
    bc: &BoxConstraints,
    _data: &KnobData<Context>,
    _env: &Env,
  ) -> Size {
    // println!("layout {:?}: {:#?}", self.id(), bc);

    // BoxConstraints are passed by the parent widget.
    // This method can return any Size within those constraints:
    // bc.constrain(my_size)
    //
    // To check if a dimension is infinite or not (e.g. scrolling):
    // bc.is_width_bounded() / bc.is_height_bounded()
    bc.max()
  }

  // The paint method gets called last, after an event flow.
  // It goes event -> update -> layout -> paint, and each method can influence the next.
  // Basically, anything that changes the appearance of a widget causes a paint.
  fn paint(&mut self,
           paint_ctx: &mut PaintCtx,
           data: &KnobData<Context>,
           env: &Env) {

    let size = paint_ctx.size();
    let half_size = size * 0.5;
    let center = Point::new(half_size.width, half_size.height);
    let value_radius = half_size.width.min(half_size.height) - self.modulation_width - 2.0;

    let value_width = 2.0;

    let value_bg_color = self.value_bg_color.resolve(env);
    Self::paint_arc(paint_ctx,
                    center, value_radius,
                    Self::START_ANGLE, Self::END_ANGLE,
                    value_bg_color, value_width, false);

    let start_angle = self.value_to_angle(data.origin, data.min, data.max);
    let end_angle = self.value_to_angle(data.value, data.min, data.max);
    let value_fg_color = self.value_fg_color.resolve(env);
    Self::paint_arc(paint_ctx,
                    center, value_radius,
                    start_angle, end_angle,
                    value_fg_color, value_width, true);

    if let Some(weight) = data.modulation.weight {
      // TODO paint weight arc
    }

    if self.modulation_width > 0.0 {
      let modulation_radius = value_radius + self.modulation_width;
      let modulation_bg_color = self.modulation_bg_color.resolve(env);
      Self::paint_arc(paint_ctx,
                      center, modulation_radius,
                      Self::START_ANGLE, Self::END_ANGLE,
                      modulation_bg_color, self.modulation_width, false);

      let modulated_value = (data.value + data.modulation.value).max(data.min).min(data.max);
      let start_angle = self.value_to_angle(data.value, data.min, data.max);
      // TODO this might need to be added to data.value
      let end_angle = self.value_to_angle(modulated_value, data.min, data.max);
      let modulation_fg_color = self.modulation_fg_color.resolve(env);
      Self::paint_arc(paint_ctx,
                      center, modulation_radius,
                      start_angle, end_angle,
                      modulation_fg_color, self.modulation_width, false);
    }
  }
}
