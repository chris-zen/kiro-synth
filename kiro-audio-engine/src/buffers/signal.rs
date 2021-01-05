use std::ops::{Index, IndexMut};

pub trait Signal<'a> {
  type Indexed: Index<usize, Output = f32>;
  type IndexedMut: IndexMut<usize, Output = f32>;
  type Iter: Iterator<Item = &'a f32>;
  type IterMut: Iterator<Item = &'a mut f32>;

  fn as_indexed(&'a self) -> Self::Indexed;
  fn as_indexed_mut(&'a mut self) -> Self::IndexedMut;

  fn iter(&'a self) -> Self::Iter;
  fn iter_mut(&'a mut self) -> Self::IterMut;

  fn as_slice(&self) -> &[f32];
  fn as_mut_slice(&mut self) -> &mut [f32];
}
