
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use super::{ GLOBAL_CONTAINER_NGINX_LOCK, WrapIt };
use crate::lib::docker::container::Container;

impl GLOBAL_CONTAINER_NGINX_LOCK {
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

  pub async fn regenerate(&self) {
    let nginx_container = self.get();
    if let Some(nginx) = nginx_container {
      let command = vec!["sh", "/etc/nginx/regenerate.sh"];
      nginx.execute_command(command).await;
    }
  }

  pub async fn change_target(&self, container_target: Option<String>, container_target_port: Option<String>) {
    let nginx_container= self.get();
    if let Some(nginx) = nginx_container {
      if let Some(target) = container_target {
        let command_arg = format!("TARGET_CONTAINER={}", target).clone().to_owned();
        let command = vec!["export", &command_arg];
        nginx.clone().execute_command(command).await;
      }

      if let Some(target) = container_target_port {
        let command_arg = format!("TARGET_PORT={}", target).clone().to_owned();
        let command = vec!["export", &command_arg];
        nginx.execute_command(command).await;
      }

    }
  }
}
