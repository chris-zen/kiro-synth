use core::f64::consts::{PI, FRAC_PI_2};

use druid::kurbo::{Arc, Shape};
use druid::{Widget, BoxConstraints, Color, Env, Event, Data, Size, Point, Vec2, LifeCycle, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, RenderContext, UpdateCtx, KeyOrValue};
use std::marker::PhantomData;
use crate::ui::widgets::knob::theme::{KNOB_VALUE_FG_COLOR, KNOB_VALUE_BG_COLOR, KNOB_MODULATION_VALUE_FG_COLOR, KNOB_MODULATION_VALUE_BG_COLOR, KNOB_MODULATION_TOTAL_AMOUNT_COLOR, KNOB_MODULATION_CONFIG_AMOUNT_COLOR};

pub mod theme {
  use druid::{Key, Color, Env};
  pub use druid::theme::*;

  pub const KNOB_VALUE_FG_COLOR: Key<Color> = Key::new("knob.value-fg");
  pub const KNOB_VALUE_BG_COLOR: Key<Color> = Key::new("knob.value-bg");
  pub const KNOB_MODULATION_VALUE_FG_COLOR: Key<Color> = Key::new("knob.modulation-value-fg");
  pub const KNOB_MODULATION_VALUE_BG_COLOR: Key<Color> = Key::new("knob.modulation-value-bg");
  pub const KNOB_MODULATION_TOTAL_AMOUNT_COLOR: Key<Color> = Key::new("knob.modulation-total-amount-color");
  pub const KNOB_MODULATION_CONFIG_AMOUNT_COLOR: Key<Color> = Key::new("knob.modulation-config-amount-color");

  pub fn init(env: &mut Env) {
    env.set(KNOB_VALUE_FG_COLOR, Color::WHITE);
    env.set(KNOB_VALUE_BG_COLOR, Color::BLACK);
    env.set(KNOB_MODULATION_VALUE_FG_COLOR, Color::rgb(0.8, 0.3, 0.0));
    env.set(KNOB_MODULATION_VALUE_BG_COLOR, Color::rgb(0.2, 0.1, 0.1));
    env.set(KNOB_MODULATION_TOTAL_AMOUNT_COLOR, Color::rgb(1.0, 0.87, 0.14));
    env.set(KNOB_MODULATION_CONFIG_AMOUNT_COLOR, Color::rgb(1.0, 0.87, 0.14));
  }
}

const ARC_TOLERANCE: f64 = 0.1;


#[derive(Debug, Clone, Data)]
pub struct KnobModulationData {
  /// The value of the modulation applied to the parameter coming from the audio thread in real time
  pub value: f64,

  /// While in configuration mode it contains an identifier of the source
  pub config_source: Option<usize>,

  /// Amount of modulation from the selected source while in configuration mode
  pub config_amount: f64,

  /// Total amount of modulation applied to the parameter from all the connected sources
  pub total_amount: f64,
}

impl KnobModulationData {
  pub fn new(value: f64, config_source: Option<usize>, config_amount: f64, total_amount: f64) -> Self {
    KnobModulationData {
      value,
      config_source,
      config_amount,
      total_amount,
    }
  }
}

impl Default for KnobModulationData {
  fn default() -> Self {
    KnobModulationData {
      value: 0.0,
      config_source: None,
      config_amount: 0.0,
      total_amount: 0.0,
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
      modulation: KnobModulationData::default(),
      context,
    }
  }

  pub fn with_modulation(mut self, modulation_data: KnobModulationData) -> Self {
    self.modulation = modulation_data;
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
      Callback: Fn(&mut UpdateCtx, &KnobData<Context>) -> () {

  callback: Callback,

  value_width: f64,
  value_fg_color: KeyOrValue<Color>,
  value_bg_color: KeyOrValue<Color>,

  modulation_width: f64,
  modulation_value_fg_color: KeyOrValue<Color>,
  modulation_value_bg_color: KeyOrValue<Color>,
  modulation_total_amount_color: KeyOrValue<Color>,
  modulation_config_amount_color: KeyOrValue<Color>,

  sensitivity: f64,
  mouse_move: MouseMove,

  _phantom: PhantomData<Context>,
}

impl<Context, Callback> Knob<Context, Callback>
  where
      Context: Data,
      Callback: Fn(&mut UpdateCtx, &KnobData<Context>) -> () {
  
  const START_ANGLE: f64 = 2.0 * PI * (20.0 / 360.0);
  const END_ANGLE: f64 = 2.0 * PI * (340.0 / 360.0);

  pub fn new(callback: Callback) -> Self {
    Knob {
      callback,
      value_width: 2.0,
      value_fg_color: KeyOrValue::Key(KNOB_VALUE_FG_COLOR),
      value_bg_color: KeyOrValue::Key(KNOB_VALUE_BG_COLOR),
      modulation_width: 0.0,
      modulation_value_fg_color: KeyOrValue::Key(KNOB_MODULATION_VALUE_FG_COLOR),
      modulation_value_bg_color: KeyOrValue::Key(KNOB_MODULATION_VALUE_BG_COLOR),
      modulation_total_amount_color: KeyOrValue::Key(KNOB_MODULATION_TOTAL_AMOUNT_COLOR),
      modulation_config_amount_color: KeyOrValue::Key(KNOB_MODULATION_CONFIG_AMOUNT_COLOR),
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

  fn paint_value_background(&mut self, ctx: &mut PaintCtx, env: &Env, center: Point, radius: f64) {
    let color = self.value_bg_color.resolve(env);
    Self::paint_arc(ctx,
                    center, radius,
                    Self::START_ANGLE, Self::END_ANGLE,
                    color, self.value_width, false);
  }

  fn paint_value(&mut self, ctx: &mut PaintCtx, data: &KnobData<Context>, env: &Env, center: Point, radius: f64) {
    let start_angle = self.value_to_angle(data.origin, data.min, data.max);
    let end_angle = self.value_to_angle(data.value, data.min, data.max);
    let color = self.value_fg_color.resolve(env);
    Self::paint_arc(ctx,
                    center, radius,
                    start_angle, end_angle,
                    color, self.value_width, true);
  }

  fn paint_modulation_background(&mut self, ctx: &mut PaintCtx, env: &Env, center: Point, radius: f64) {
    let color = self.modulation_value_bg_color.resolve(env);
    Self::paint_arc(ctx,
                    center, radius,
                    Self::START_ANGLE, Self::END_ANGLE,
                    color, self.modulation_width, false);
  }

  fn paint_modulation_value(&mut self, ctx: &mut PaintCtx, data: &KnobData<Context>, env: &Env, center: Point, radius: f64) {
    let value = (data.value + data.modulation.value).max(data.min).min(data.max);
    let start_angle = self.value_to_angle(data.value, data.min, data.max);
    let end_angle = self.value_to_angle(value, data.min, data.max);
    let color = self.modulation_value_fg_color.resolve(env);
    Self::paint_arc(ctx,
                    center, radius,
                    start_angle, end_angle,
                    color, self.modulation_width, false);
  }

  fn paint_modulation_total_amount(&mut self, ctx: &mut PaintCtx, data: &KnobData<Context>, env: &Env, center: Point, value_radius: f64) {
    let width = (self.modulation_width / 2.0).floor();
    let radius = value_radius + self.modulation_width + 0.9;
    let start_value = (data.value - data.modulation.total_amount).max(data.min).min(data.max);
    let start_angle = self.value_to_angle(start_value, data.min, data.max);
    let end_value = (data.value + data.modulation.total_amount).max(data.min).min(data.max);
    let end_angle = self.value_to_angle(end_value, data.min, data.max);
    let color = self.modulation_total_amount_color.resolve(env);
    Self::paint_arc(ctx,
                    center, radius,
                    start_angle, end_angle,
                    color, width, false);
  }

  fn paint_modulation_config_amount(&mut self, ctx: &mut PaintCtx, data: &KnobData<Context>, env: &Env, center: Point, value_radius: f64, config_amount: f64) {
    // let weight_width = self.modulation_width / 2.0;
    // let weight_radius = value_radius + weight_width + 0.9;
    let width = self.value_width;
    let radius = value_radius;
    let value = data.value + config_amount;
    let start_angle = self.value_to_angle(data.value, data.min, data.max);
    let end_angle = self.value_to_angle(value, data.min, data.max);
    let color = self.modulation_config_amount_color.resolve(env);
    Self::paint_arc(ctx,
                    center, radius,
                    start_angle, end_angle,
                    color, width, true);
  }
}

impl<Context, Callback> Widget<KnobData<Context>> for Knob<Context, Callback>
  where
      Context: Data,
      Callback: Fn(&mut UpdateCtx, &KnobData<Context>) -> () {
  
  fn event(&mut self,
           ctx: &mut EventCtx,
           event: &Event,
           data: &mut KnobData<Context>,
           _env: &Env) {

    // println!("event {:?}: {:#?}", self.id(), event);

    match event {
      Event::MouseDown(mouse) => {
        ctx.set_active(true);
        let value = match data.modulation.config_source {
          Some(_) => data.modulation.config_amount,
          None => data.value,
        };
        self.mouse_move = MouseMove {
          orig_pos: mouse.pos.y,
          orig_value: value,
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
          match data.modulation.config_source.as_mut() {
            Some(_) => data.modulation.config_amount = (value / data.step).round() * data.step,
            None => data.value = (value / data.step).round() * data.step,
          };
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
            ctx: &mut UpdateCtx,
            old_data: &KnobData<Context>,
            data: &KnobData<Context>,
            _env: &Env) {
    // println!("update {:?}: {:?} -> {:?} | {:?} -> {:?} | {:?} -> {:?}",
    //   self.id(), old_data.value, data.value,
    //   old_data.modulation.config_source, data.modulation.config_source,
    //   old_data.modulation.config_amount, data.modulation.config_amount
    // );
    if old_data.value != data.value ||
        (old_data.modulation.config_source == data.modulation.config_source &&
        old_data.modulation.config_amount != data.modulation.config_amount) {

      (self.callback)(ctx, &data);
    }
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
           ctx: &mut PaintCtx,
           data: &KnobData<Context>,
           env: &Env) {

    let size = ctx.size();
    let half_size = size * 0.5;
    let center = Point::new(half_size.width, half_size.height);
    let value_radius = half_size.width.min(half_size.height) - self.modulation_width - 2.0;

    self.paint_value_background(ctx, env, center, value_radius);
    self.paint_value(ctx, data, env, center, value_radius);

    if self.modulation_width > 0.0 {
      let modulation_radius = value_radius + self.modulation_width;
      self.paint_modulation_background(ctx, env, center, modulation_radius);
      self.paint_modulation_value(ctx, data, env, center, modulation_radius);
      self.paint_modulation_total_amount(ctx, data, env, center, value_radius);
    }

    if data.modulation.config_source.is_some() {
      self.paint_modulation_config_amount(ctx, data, env, center, value_radius, data.modulation.config_amount);
    }
  }
}
