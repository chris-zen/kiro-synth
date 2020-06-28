use crate::float::Float;
use crate::funcs::signal_polarity::unipolar_to_bipolar;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub enum Shape {
  Unipolar,
  Bipolar,
}

#[derive(Debug, Clone)]
pub enum Direction {
  Up,
  Down,
}

#[derive(Debug, Clone)]
pub struct SawTrivial {
  shape: Shape,
  direction: Direction,
}

impl Default for SawTrivial {
  fn default() -> Self {
    SawTrivial {
      shape: Shape::Bipolar,
      direction: Direction::Up,
    }
  }
}

impl SawTrivial {
  pub fn new(shape: Shape, direction: Direction) -> Self {
    SawTrivial { shape, direction }
  }

  pub fn with_shape(self, shape: Shape) -> Self {
    SawTrivial { shape, ..self }
  }

  pub fn with_direction(self, direction: Direction) -> Self {
    SawTrivial { direction, ..self }
  }
}

impl<F: Float> Waveform<F> for SawTrivial {
  fn initial_modulo(&self) -> F {
    F::val(0.5)
  }

  fn generate(&mut self, modulo: F, _phase_inc: F) -> F {
    let out = match self.shape {
      Shape::Bipolar => unipolar_to_bipolar(modulo),
      Shape::Unipolar => modulo - F::one(),
    };

    match self.direction {
      Direction::Up => out,
      Direction::Down => out.neg(),
    }
  }
}
