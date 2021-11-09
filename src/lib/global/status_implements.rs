
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use crate::lib::docker::container::ContainerRole;

use super::{ GLOBAL_SYSTEM_STATUS_LOCK, WrapIt };
use super::status_interfaces::{HealthyStatus, SystemStatus};

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
  pub fn set_update_start(&self) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.in_update = true;
  }

  pub fn set_update_finish(&self) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.in_update = false;
  }

  pub fn set_main(&self, health: HealthyStatus) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.main = health;
  }
  pub fn set_main_ip(&self, ip: Option<String>) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.main_ip = ip;
  }
  pub fn set_rollback(&self, health: HealthyStatus) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.rollback = health;
  }
  pub fn set_rollback_ip(&self, ip: Option<String>) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.rollback_ip = ip;
  }
  pub fn set_nginx(&self, health: HealthyStatus) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.nginx = health;
  }

  pub fn get_main(&self) -> HealthyStatus {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.main
  }
  pub fn get_main_ip(&self) -> Option<String> {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.main_ip
  }
  pub fn get_rollback(&self) -> HealthyStatus {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.rollback
  }
  pub fn get_rollback_ip(&self) -> Option<String> {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.rollback_ip
  }
  pub fn get_nginx(&self) -> HealthyStatus {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.nginx
  }

  pub fn set_nginx_target(&self, target: ContainerRole) {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let mut x = next_lock.write().unwrap();
    x.nginx_target_role = target.clone().to_owned();
  }

  pub fn get_nginx_target(&self) -> ContainerRole {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.nginx_target_role.clone()
  }

  pub fn is_updating(&self) -> bool {
    let next = (*self).clone();
    let next_lock = Arc::clone(&next);
    let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
    let x = wrapped_value.deref();
    let x = x.clone();
    x.in_update
  }
}
