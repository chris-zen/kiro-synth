use std::any::type_name;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub struct Key<T: ?Sized>(u64, PhantomData<T>);

impl<T> Key<T> {
  pub fn new(value: u64) -> Self {
    Self(value, PhantomData)
  }
}

impl<T> Clone for Key<T> {
  fn clone(&self) -> Self {
    Key::new(self.0)
  }
}

impl<T> Copy for Key<T> {}

impl<T> PartialEq for Key<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T> Eq for Key<T> {}

impl<T> Hash for Key<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

impl<T> PartialOrd for Key<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<T> Ord for Key<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl<T> Debug for Key<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Key<{}>({})", type_name::<T>(), self.0))
  }
}

impl<T> Display for Key<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}[{}]", type_name::<T>(), self.0))
  }
}

unsafe impl<T> Send for Key<T> {}

#[derive(Debug)]
pub struct KeyGen<T> {
  next_value: u64,
  _phantom: PhantomData<T>,
}

impl<T> KeyGen<T> {
  pub fn new() -> Self {
    Self {
      next_value: 0,
      _phantom: PhantomData,
    }
  }

  pub fn next(&mut self) -> Key<T> {
    assert_ne!(self.next_value, u64::MAX);
    let key = self.next_value;
    self.next_value += 1;
    Key::new(key)
  }
}
