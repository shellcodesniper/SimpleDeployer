#![allow(dead_code)]

pub mod global_interfaces;

use lazy_static::lazy_static;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use global_interfaces::{ SystemStatus, ContainerStatus };

enum WrapIt<'a, T> {
  Read(RwLockReadGuard<'a, T>),
  Write(RwLockWriteGuard<'a, T>)
}

impl<'a, T> Deref for WrapIt<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match self {
      WrapIt::Read(r_g) => r_g.deref(),
      WrapIt::Write(w_g) => w_g.deref()
    }
  }
}


lazy_static! {
  pub static ref GLOBAL_SYSTEM_STATUS_LOCK: Arc<RwLock<SystemStatus>> = Arc::new(RwLock::new(SystemStatus::new()));
}

impl GLOBAL_SYSTEM_STATUS_LOCK {
  pub fn write_lock() -> RwLockWriteGuard<'static, SystemStatus> {
    GLOBAL_SYSTEM_STATUS_LOCK.write().unwrap()
  }
}