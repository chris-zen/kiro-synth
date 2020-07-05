use std::ops::{Index, IndexMut};

use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::marker_traits::Unsigned;
use generic_array::typenum::consts;

use crate::float::Float;
use crate::program::{MaxParams, ParamRef, SourceRef};

pub type MaxModulations = consts::U1024;
type ModulationsPool<F> = Pool<Modulation<F>, MaxModulations>;

const NIL: usize = (1 << 16) - 1;

#[derive(Debug, Clone)]
pub enum Error {
  OutOfMemory,
}

#[derive(Debug, Clone)]
pub struct Modulation<F: Float> {
  pub source_ref: SourceRef,
  pub amount: F,
}

impl<F: Float> Modulation<F> {
  pub fn new(source_ref: SourceRef, amount: F) -> Self {
    Modulation { source_ref, amount }
  }
}

impl<F: Float> Default for Modulation<F> {
  fn default() -> Self {
    Modulation {
      source_ref: SourceRef::new(0),
      amount: F::zero(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Modulations<F: Float> {
  pool: ModulationsPool<F>,
  heads: GenericArray<usize, MaxParams>,
}

impl<F: Float> Default for Modulations<F> {
  fn default() -> Self {
    let pool = ModulationsPool::<F>::new();
    let mut heads: GenericArray<usize, MaxParams> = GenericArray::default();
    for i in 0..MaxParams::to_usize() {
      heads[i] = NIL;
    }

    Modulations { pool, heads }
  }
}

impl<'a, F: Float> Modulations<F> {
  fn find(&mut self, param_ref: ParamRef, source_ref: SourceRef) -> Option<(usize, usize)> {
    let param_index: usize = param_ref.into();
    let mut head = self.heads[param_index];
    let mut prev = NIL;
    let mut result = None;

    while head != NIL && result.is_none() {
      let node = self.pool.get(head);
      if node.data.source_ref == source_ref {
        result = Some((prev, head));
      }
      prev = head;
      head = node.next;
    }

    result
  }

  pub fn update(
    &mut self,
    param_ref: ParamRef,
    source_ref: SourceRef,
    amount: F,
  ) -> Result<(), Error> {
    match self.find(param_ref, source_ref) {
      Some((_prev, head)) => {
        let node = self.pool.get_mut(head);
        node.data.amount = amount;
      }
      None => {
        self.push(param_ref, source_ref, amount)?;
      }
    }
    Ok(())
  }

  pub fn push(
    &mut self,
    param_ref: ParamRef,
    source_ref: SourceRef,
    amount: F,
  ) -> Result<(), Error> {
    let (head, node) = self.pool.alloc()?;

    let param_index: usize = param_ref.into();
    let tail = self.heads[param_index];
    self.heads[param_index] = head;
    node.next = tail;

    node.data.source_ref = source_ref;
    node.data.amount = amount;

    Ok(())
  }

  pub fn delete(&mut self, param_ref: ParamRef, source_ref: SourceRef) -> Result<(), Error> {
    match self.find(param_ref, source_ref) {
      Some((prev, head)) => {
        let next = self.pool.get(head).next;
        match prev {
          NIL => {
            let param_index: usize = param_ref.into();
            self.heads[param_index] = next;
          }
          _ => {
            let prev_node = self.pool.get_mut(prev);
            prev_node.next = next;
          }
        }
        self.pool.free(head);
      }
      None => {
        println!(
          "Can not delete the modulation {:?} -> {:?}",
          source_ref, param_ref
        );
      }
    }
    Ok(())
  }

  // pub fn for_each_modulation<A>(&self, param_ref: ParamRef, mut process: A) where A: FnMut(&Modulation<F>) {
  //   let param_index: usize = param_ref.into();
  //   let mut head = self.heads[param_index];
  //
  //   while head != NIL {
  //     let node = self.pool.get(head);
  //     process(&node.data);
  //     head = node.next;
  //   }
  // }

  pub fn get_param_modulations(&self, param_ref: ParamRef) -> Iter<F> {
    let param_index: usize = param_ref.into();
    let head = self.heads[param_index];
    Iter {
      pool: &self.pool,
      next: head,
    }
  }
}

#[derive(Debug)]
pub struct Iter<'a, F: Float> {
  pool: &'a ModulationsPool<F>,
  next: usize,
}

impl<'a, F: Float> Iterator for Iter<'a, F> {
  type Item = &'a Modulation<F>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next {
      NIL => None,
      _ => {
        let node = &self.pool[self.next];
        self.next = node.next;
        Some(&node.data)
      }
    }
  }
}

#[derive(Debug, Clone)]
struct Node<T> {
  pub data: T,
  pub next: usize,
}

impl<T: Default> Default for Node<T> {
  fn default() -> Self {
    Node {
      data: T::default(),
      next: NIL,
    }
  }
}

#[derive(Debug, Clone)]
struct Pool<T, N: ArrayLength<Node<T>>> {
  elements: GenericArray<Node<T>, N>,
  free: usize,
}

impl<T: Default, N: ArrayLength<Node<T>>> Pool<T, N> {
  pub fn new() -> Self {
    let mut elements: GenericArray<Node<T>, N> = GenericArray::default();
    let capacity = elements.len();
    for i in 0..capacity - 1 {
      elements[i].next = i + 1;
    }
    elements[capacity - 1].next = NIL;

    Pool { elements, free: 0 }
  }

  pub fn alloc(&mut self) -> Result<(usize, &mut Node<T>), Error> {
    let index = self.free;
    if index == NIL {
      Err(Error::OutOfMemory)
    } else {
      let node = &mut self.elements[index];
      self.free = node.next;
      Ok((index, node))
    }
  }

  /// Precondition: No other element is referencing it
  pub fn free(&mut self, index: usize) {
    let node = &mut self.elements[index];
    node.next = self.free;
    self.free = index;
  }

  fn get(&self, index: usize) -> &Node<T> {
    &self.elements[index]
  }

  fn get_mut(&mut self, index: usize) -> &mut Node<T> {
    &mut self.elements[index]
  }
}

impl<T: Default, N: ArrayLength<Node<T>>> Index<usize> for Pool<T, N> {
  type Output = Node<T>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.elements[index]
  }
}

impl<T: Default, N: ArrayLength<Node<T>>> IndexMut<usize> for Pool<T, N> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.elements[index]
  }
}

#[cfg(test)]
mod tests {
  use crate::program::modulations::Modulation;
  use crate::program::SourceRef;

  #[test]
  fn modulation_new() {
    let source_ref = SourceRef::new(123);
    let m = Modulation::new(source_ref, 1.2);
    assert_eq!(m.source_ref, source_ref);
    assert_eq!(m.amount, 1.2);
  }

  #[test]
  fn modulation_default() {
    let m = Modulation::<f64>::default();
    assert_eq!(m.source_ref, SourceRef::new(0));
    assert_eq!(m.amount, 0.0);
  }
}
