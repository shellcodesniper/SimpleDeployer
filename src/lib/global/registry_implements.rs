use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use super::{ GLOBAL_REGISTRY_LOCK, WrapIt };
use crate::lib::registry::Registry;

impl GLOBAL_REGISTRY_LOCK {
  pub fn get(&self) -> Registry {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x
  }

  pub fn set(&self, new: Registry) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    *x = new.clone().to_owned();
  }
}
