
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use super::{ GLOBAL_PARSED_CONFIG_LOCK, WrapIt };
use crate::lib::config::parser::{ ParsedConfig };

impl GLOBAL_PARSED_CONFIG_LOCK {
  pub fn get(&self) -> ParsedConfig {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x
  }

  pub fn set(&self, new: ParsedConfig) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    *x = new.clone().to_owned();
  }
}
