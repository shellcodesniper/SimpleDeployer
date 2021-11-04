
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use super::{ GLOBAL_CONTAINER_MAIN_LOCK, WrapIt };
use crate::lib::docker::container::Container;

impl GLOBAL_CONTAINER_MAIN_LOCK {
  pub fn get(&self) -> Option<Container> {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x
  }

  pub fn set(&self, new: Option<Container>) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    *x = new.clone().to_owned();
  }
}
