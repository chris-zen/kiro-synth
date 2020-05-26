use core::f64::consts::{PI, FRAC_PI_2};

use druid::kurbo::{Arc, Shape};
use druid::{Widget, BoxConstraints, Color, Env, Event, Data, Size, Point, Vec2,
            LifeCycle, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, RenderContext, UpdateCtx};
use std::marker::PhantomData;

pub mod theme {
  use druid::{Key, Color, Env};
  pub use druid::theme::*;

  pub const KNOB_VALUE_BG: Key<Color> = Key::new("knob-value-bg");
  pub const KNOB_VALUE_FG: Key<Color> = Key::new("knob-value-fg");
  pub const KNOB_MODULATION: Key<Color> = Key::new("knob-modulation");

  pub fn init(env: &mut Env) {
    env.set(KNOB_VALUE_BG, Color::BLACK);
    env.set(KNOB_VALUE_FG, Color::WHITE);
    env.set(KNOB_MODULATION, Color::rgb(0.8, 0.3, 0.0));
  }
}

const ARC_TOLERANCE: f64 = 0.1;

#[derive(Debug, Clone, Data)]
pub struct KnobData {
  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub value: f64,
  pub modulation: f64,
}

impl KnobData {
  pub fn new(origin: f64, min: f64, max: f64, step: f64, value: f64, modulation: f64) -> Self {
    KnobData {
      origin,
      min,
      max,
      step,
      value,
      modulation,
    }
  }
}

struct MouseMove {
  orig_pos: f64,
  orig_value: f64,
}

pub struct Knob<D, CF>
  where
      D: Data,
      CF: Fn(&D, &KnobData) -> () {

  callback: CF,
  sensitivity: f64,
  mouse_move: MouseMove,
  _phantom: PhantomData<D>,
}

impl<D, CF> Knob<D, CF>
  where
      D: Data,
      CF: Fn(&D, &KnobData) -> () {
  
  const START_ANGLE: f64 = 2.0 * PI * (20.0 / 360.0);
  const END_ANGLE: f64 = 2.0 * PI * (340.0 / 360.0);

  pub fn new(callback: CF) -> Self {
    Knob {
      callback,
      sensitivity: 0.6,
      mouse_move: MouseMove { orig_pos: 0.0, orig_value: 0.0 },
      _phantom: PhantomData,
    }
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

impl<D, CF> Widget<(D, KnobData)> for Knob<D, CF>
  where
      D: Data,
      CF: Fn(&D, &KnobData) -> () {
  
  fn event(&mut self,
           ctx: &mut EventCtx,
           event: &Event,
           data: &mut (D, KnobData),
           _env: &Env) {

    // println!("event {:?}: {:#?}", self.id(), event);

    let data = &mut data.1;
    
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
    _data: &(D, KnobData),
    _env: &Env,
  ) {
    // println!("lifecycle {:?}: {:#?}", self.id(), _event);
  }

  fn update(&mut self,
            _ctx: &mut UpdateCtx,
            _old_data: &(D, KnobData),
            data: &(D, KnobData),
            _env: &Env) {
    // println!("update {:?}: {} -> {}", self.id(), _old_data.1.value, data.1.value);
    (self.callback)(&data.0, &data.1);
  }

  fn layout(
    &mut self,
    _layout_ctx: &mut LayoutCtx,
    bc: &BoxConstraints,
    _data: &(D, KnobData),
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
           data: &(D, KnobData),
           env: &Env) {

    let data = &data.1;

    let size = paint_ctx.size();
    let half_size = size * 0.5;
    let center = Point::new(half_size.width, half_size.height);
    let radius = half_size.width.min(half_size.height) - 8.0;

    let width = 2.0;
    let mod_width = 4.0;

    let arc_bg_color = env.get(theme::KNOB_VALUE_BG);
    Self::paint_arc(paint_ctx,
                    center, radius,
                    Self::START_ANGLE, Self::END_ANGLE,
                    arc_bg_color, width, false);

    let start_angle = self.value_to_angle(data.origin, data.min, data.max);
    let end_angle = self.value_to_angle(data.value, data.min, data.max);
    let arc_fg_color = env.get(theme::KNOB_VALUE_FG);
    Self::paint_arc(paint_ctx,
                    center, radius,
                    start_angle, end_angle,
                    arc_fg_color, width, true);

    let modulated_value = (data.value + data.modulation).max(data.min).min(data.max);
    let start_angle = self.value_to_angle(data.value, data.min, data.max);
    let end_angle = self.value_to_angle(modulated_value, data.min, data.max);
    let arc_mod_color = env.get(theme::KNOB_MODULATION);
    Self::paint_arc(paint_ctx,
                    center, radius + mod_width,
                    start_angle, end_angle,
                    arc_mod_color, mod_width, false);
  }
}
