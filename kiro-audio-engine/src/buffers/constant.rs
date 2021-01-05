use crate::buffers::Signal;
use std::ops::{Index, IndexMut};

pub struct Constant {
  value: f32,
  len: usize,
}

impl Index<usize> for &Constant {
  type Output = f32;

  fn index(&self, _index: usize) -> &Self::Output {
    &self.value
  }
}

impl Index<usize> for &mut Constant {
  type Output = f32;

  fn index(&self, _index: usize) -> &Self::Output {
    &self.value
  }
}

impl IndexMut<usize> for &mut Constant {
  fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
    &mut self.value
  }
}

pub struct CtIter<'a> {
  value: &'a f32,
  count: usize,
}

impl<'a> Iterator for CtIter<'a> {
  type Item = &'a f32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count > 0 {
      self.count -= 1;
      Some(self.value)
    } else {
      None
    }
  }
}

impl<'a> Signal<'a> for Constant {
  type Indexed = &'a Constant;
  type IndexedMut = &'a mut Constant;
  type Iter = CtIter<'a>;
  type IterMut = std::iter::Empty<&'a mut f32>;

  fn as_indexed(&'a self) -> Self::Indexed {
    self
  }

  fn as_indexed_mut(&'a mut self) -> Self::IndexedMut {
    self
  }

  fn iter(&'a self) -> Self::Iter {
    CtIter {
      value: &self.value,
      count: self.len,
    }
  }

  fn iter_mut(&'a mut self) -> Self::IterMut {
    std::iter::empty()
  }

  fn as_slice(&self) -> &[f32] {
    unimplemented!()
  }

  fn as_mut_slice(&mut self) -> &mut [f32] {
    unimplemented!()
  }
}
