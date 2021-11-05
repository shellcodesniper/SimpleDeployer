
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use super::{ GLOBAL_CONTAINER_LOGGER_LOCK, WrapIt };
use super::ContainerLog;

impl GLOBAL_CONTAINER_LOGGER_LOCK {
  pub fn push(&self, val: ContainerLog) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut t = next_lock.write().unwrap();
    t.push(val);
  }

  pub fn pop(&self) -> Option<ContainerLog> {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut t = next_lock.write().unwrap();

    if let Some(_) = t.get(0) {
      Some(t.remove(0))
    } else {
      None
    }
  }

  pub fn flush(&self) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut t = next_lock.write().unwrap();
    while !t.is_empty() {
      t.pop();
    }
  }
}