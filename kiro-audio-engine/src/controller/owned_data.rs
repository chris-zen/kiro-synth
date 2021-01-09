use kiro_audio_graph::key_gen::KeyGen;
use kiro_audio_graph::Key;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Ref<T: ?Sized> {
  pub(crate) key: Key<T>,
  data: Arc<UnsafeCell<T>>,
}

impl<T> Ref<T> {
  pub fn new(key: Key<T>, data: Arc<UnsafeCell<T>>) -> Self {
    Self { key, data }
  }

  pub fn get_mut(&self) -> &mut T {
    unsafe { &mut *self.data.get() }
  }
}

impl<T> Clone for Ref<T> {
  fn clone(&self) -> Self {
    Ref {
      key: self.key,
      data: self.data.clone(),
    }
  }
}

impl<T> Debug for Ref<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Ref({:?})", self.key))
  }
}

impl<T> Deref for Ref<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.data.get() }
  }
}

impl<T> DerefMut for Ref<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.data.get() }
  }
}

// unsafe impl<T> Sync for Ref<T> {}

pub struct OwnedData<T> {
  key_gen: KeyGen<T>,
  data: HashMap<Key<T>, Arc<UnsafeCell<T>>>,
}

impl<T> OwnedData<T> {
  pub fn new() -> Self {
    Self {
      key_gen: KeyGen::new(),
      data: HashMap::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn keys<'a>(&'a self) -> impl Iterator<Item = Key<T>> + 'a {
    self.data.keys().cloned()
  }

  pub fn add(&mut self, value: T) -> Key<T> {
    let key = self.key_gen.next();
    self.data.insert(key, Arc::new(UnsafeCell::new(value)));
    key
  }

  pub fn get<DK>(&self, key: DK) -> Option<Ref<T>>
  where
    DK: Into<Key<T>>,
  {
    let key: Key<T> = key.into();
    self
      .data
      .get(&key)
      .map(|value| Ref::new(key, value.clone()))
  }
}
