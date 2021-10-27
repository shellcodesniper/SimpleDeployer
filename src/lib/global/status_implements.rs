
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use super::{ GLOBAL_SYSTEM_STATUS_LOCK, WrapIt };
use super::status_interfaces::{ SystemStatus, ContainerStatus };

impl GLOBAL_SYSTEM_STATUS_LOCK {
  pub fn get(&self) -> SystemStatus {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x
  }

  pub fn set(&self, new: SystemStatus) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    *x = new.clone().to_owned();
  }
}
