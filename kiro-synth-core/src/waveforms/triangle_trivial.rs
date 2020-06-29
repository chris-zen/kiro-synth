use crate::float::Float;
use crate::funcs::signal_polarity::unipolar_to_bipolar;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub enum Shape {
  Unipolar,
  Bipolar,
}

#[derive(Debug, Clone)]
pub struct TriangleTrivial {
  shape: Shape,
}

impl Default for TriangleTrivial {
  fn default() -> Self {
    TriangleTrivial {
      shape: Shape::Bipolar,
    }
  }
}

impl TriangleTrivial {
  pub fn new(shape: Shape) -> Self {
    TriangleTrivial { shape }
  }

  pub fn with_shape(self, shape: Shape) -> Self {
    TriangleTrivial { shape }
  }
}

impl<F: Float> Waveform<F> for TriangleTrivial {
  fn initial_modulo(&self) -> F {
    F::val(0.5)
  }

  fn generate(&mut self, modulo: F, _phase_inc: F) -> F {
    match self.shape {
      Shape::Bipolar => unipolar_to_bipolar(modulo).abs() * F::val(2.0) - F::one(),
      Shape::Unipolar => unipolar_to_bipolar(modulo).abs(),
    }
  }
}
