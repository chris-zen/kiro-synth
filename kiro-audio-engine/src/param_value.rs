use std::fmt::{Debug, Formatter};
/// Adapted from vst::util::atomic_float
/// https://github.com/RustAudio/vst-rs/blob/master/src/util/atomic_float.rs
use std::sync::atomic::{AtomicU32, Ordering};

/// Parameter value using an atomic floating point variable with relaxed ordering.
///
/// Designed for the common case of sharing parameters between
/// multiple threads when no synchronization or change notification
/// is needed.
pub struct ParamValue(AtomicU32);

impl ParamValue {
  pub fn new(value: f32) -> Self {
    Self(AtomicU32::new(value.to_bits()))
  }

  pub fn get(&self) -> f32 {
    f32::from_bits(self.0.load(Ordering::Relaxed))
  }

  pub fn set(&self, value: f32) {
    self.0.store(value.to_bits(), Ordering::Relaxed)
  }
}

impl Clone for ParamValue {
  fn clone(&self) -> Self {
    ParamValue::new(self.get())
  }
}

impl Debug for ParamValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}", self.get()))
  }
}
