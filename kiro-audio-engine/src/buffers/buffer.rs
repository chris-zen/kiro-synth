use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Buffer(Vec<f32>);

impl Buffer {
  pub fn new(len: usize) -> Self {
    let mut buffer = Vec::with_capacity(len);
    buffer.resize_with(len, || 0.0);
    Self(buffer)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn fill(&mut self, value: f32) {
    self.0.iter_mut().for_each(|v| *v = value);
  }

  pub fn fill_first(&mut self, len: usize, value: f32) {
    self.0.iter_mut().take(len).for_each(|v| *v = value);
  }

  pub fn iter(&self) -> core::slice::Iter<f32> {
    self.0.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<f32> {
    self.0.iter_mut()
  }

  pub fn as_slice(&self) -> &[f32] {
    self.0.as_slice()
  }

  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self.0.as_mut_slice()
  }
}

impl Index<usize> for &Buffer {
  type Output = f32;

  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl Index<usize> for &mut Buffer {
  type Output = f32;

  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl IndexMut<usize> for &mut Buffer {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

// impl<'a> Signal<'a> for Buffer {
//   type Indexed = &'a Buffer;
//   type IndexedMut = &'a mut Buffer;
//   type Iter = core::slice::Iter<'a, f32>;
//   type IterMut = core::slice::IterMut<'a, f32>;
//
//   fn as_indexed(&'a self) -> Self::Indexed {
//     self
//   }
//
//   fn as_indexed_mut(&'a mut self) -> Self::IndexedMut {
//     self
//   }
//
//   fn iter(&'a self) -> Self::Iter {
//     self.0.iter()
//   }
//
//   fn iter_mut(&'a mut self) -> Self::IterMut {
//     self.0.iter_mut()
//   }
//
//   fn as_slice(&self) -> &[f32] {
//     self.0.as_slice()
//   }
//
//   fn as_mut_slice(&mut self) -> &mut [f32] {
//     self.0.as_mut_slice()
//   }
// }
