use core::f64::consts::{PI, FRAC_PI_2};
use core::f64::EPSILON;

use druid::kurbo::{BezPath, Arc};
use druid::{BoxConstraints, Color, Env, Event, LifeCycle, Widget, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, RenderContext, UpdateCtx, Data, Size, Point, Vec2, Selector, Command, Target};

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

pub const UPDATE_VALUE: Selector = Selector::new("knob.update-value");

#[derive(Debug, Clone, Data)]
pub struct KnobModel {
  pub value: f64,
  pub modulation: f64,
}

impl KnobModel {
  pub fn new(value: f64, modulation: f64) -> Self {
    KnobModel {
      value,
      modulation,
    }
  }
}

struct MouseMove {
  orig_pos: f64,
  orig_value: f64,
}

pub struct Knob<F> where F: Fn(&KnobModel) -> () {
  start: f64,
  min: f64,
  max: f64,
  step: f64,
  callback: F,
  // unit, ...

  sensitivity: f64,
  mouse_move: MouseMove,
}

impl<F> Knob<F> where F: Fn(&KnobModel) -> () {
  const START_ANGLE: f64 = 2.0 * PI * (20.0 / 360.0);
  const END_ANGLE: f64 = 2.0 * PI * (340.0 / 360.0);

  pub fn new(start: f64, min: f64, max: f64, step: f64, callback: F) -> Self {
    Knob {
      start,
      min,
      max,
      step,
      callback,

      sensitivity: 0.4,
      mouse_move: MouseMove { orig_pos: 0.0, orig_value: 0.0 },
    }
  }

  fn value_to_angle(&self, value: f64) -> f64 {
    let range = self.max - self.min;
    Self::START_ANGLE + (Self::END_ANGLE - Self::START_ANGLE) * (value - self.min) / range
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

    let mut arc_curve = BezPath::from_vec(arc.append_iter(1e-12).collect());

    if line_to_center {
      if sweep_angle.abs() <= EPSILON {
        let angle = start_angle + FRAC_PI_2;
        let p = Point::new(
          center.x + radius * angle.cos(),
          center.y + radius * angle.sin(),
        );
        arc_curve.move_to(p);
      }
      arc_curve.line_to(center);
    }

    paint_ctx.stroke(arc_curve, &color, width);
  }
}

impl<F> Widget<KnobModel> for Knob<F> where F: Fn(&KnobModel) -> () {
  fn event(&mut self,
           ctx: &mut EventCtx,
           event: &Event,
           data: &mut KnobModel,
           _env: &Env) {

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
      Event::MouseMoved(mouse) => {
        if ctx.is_active() {
          let inc = self.sensitivity * (self.mouse_move.orig_pos - mouse.pos.y);
          let value = (self.mouse_move.orig_value + self.step * inc).max(self.min).min(self.max);
          data.value = (value / self.step).round() * self.step;
          ctx.submit_command(Command::new(UPDATE_VALUE, data.value), Target::Global);
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
    _data: &KnobModel,
    _env: &Env,
  ) {
  }

  fn update(&mut self,
            _ctx: &mut UpdateCtx,
            _old_data: &KnobModel,
            data: &KnobModel,
            _env: &Env) {
    // println!("{} -> {}", _old_data.value, _data.value);
    (self.callback)(data);
  }

  fn layout(
    &mut self,
    _layout_ctx: &mut LayoutCtx,
    bc: &BoxConstraints,
    _data: &KnobModel,
    _env: &Env,
  ) -> Size {
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
           data: &KnobModel,
           env: &Env) {

    // paint_ctx.clear(env.get(theme::WINDOW_BACKGROUND_COLOR));

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

    let start_angle = self.value_to_angle(self.start);
    let end_angle = self.value_to_angle(data.value);
    let arc_fg_color = env.get(theme::KNOB_VALUE_FG);
    Self::paint_arc(paint_ctx,
                    center, radius,
                    start_angle, end_angle,
                    arc_fg_color, width, true);

    let modulated_value = (data.value + data.modulation).max(self.min).min(self.max);
    let start_angle = self.value_to_angle(data.value);
    let end_angle = self.value_to_angle(modulated_value);
    let arc_mod_color = env.get(theme::KNOB_MODULATION);
    Self::paint_arc(paint_ctx,
                    center, radius + 4.0,
                    start_angle, end_angle,
                    arc_mod_color, mod_width, false);
  }
}

// let stroke_color = env.get(theme::BORDER_DARK);
// let mut path = BezPath::new();
// path.move_to(Point::new(0.0, half_size.height));
// path.line_to(Point::new(size.width, half_size.height));
// paint_ctx.stroke(path, &stroke_color, 1.0);
// let mut path = BezPath::new();
// path.move_to(Point::new(half_size.width, 0.0));
// path.line_to(Point::new(half_size.width, size.height));
// paint_ctx.stroke(path, &stroke_color, 1.0);