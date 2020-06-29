use num_traits::Float;

pub trait ParabolicSine: Float {
  const B: Self;
  const C: Self;
  const P: Self;

  /// http://devmaster.net/posts/9648/fast-and-accurate-sine-cosine
  /// self value is between -pi and +pi
  fn parabolic_sine(&self) -> Self;
}

impl ParabolicSine for f32 {
  const B: f32 = 4.0 / core::f32::consts::PI;
  const C: f32 = -4.0 / (core::f32::consts::PI * core::f32::consts::PI);
  const P: f32 = 0.225;

  fn parabolic_sine(&self) -> Self {
    let y = Self::B * *self + Self::C * *self * self.abs();
    Self::P * (y * y.abs() - y) + y
  }
}

impl ParabolicSine for f64 {
  const B: f64 = 4.0 / core::f64::consts::PI;
  const C: f64 = -4.0 / (core::f64::consts::PI * core::f64::consts::PI);
  const P: f64 = 0.225;

  fn parabolic_sine(&self) -> Self {
    let y = Self::B * *self + Self::C * *self * self.abs();
    Self::P * (y * y.abs() - y) + y
  }
}
