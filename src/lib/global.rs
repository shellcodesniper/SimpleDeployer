#![allow(dead_code, unused_imports)]

pub mod status_interfaces;
pub mod container_logger_interfaces;

pub mod status_implements;
pub mod parser_implements;
pub mod docker_implements;
pub mod registry_implements;
pub mod container_logger_implements;
pub mod container_main_implements;
pub mod container_rollback_implements;
pub mod container_nginx_implements;

use lazy_static::lazy_static;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

use status_interfaces::{ SystemStatus, ContainerStatus };
use container_logger_interfaces:: { ContainerLog };

use super::config::parser::ParsedConfig;
use super::docker::Docker;
use super::registry::Registry;

use super::docker::container::Container;

use status_implements::*;
use parser_implements::*;
use docker_implements::*;
use registry_implements::*;
use container_main_implements::*;
use container_rollback_implements::*;
use container_nginx_implements::*;

enum WrapIt<'a, T>{
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
  pub static ref GLOBAL_PARSED_CONFIG_LOCK: Arc<RwLock<ParsedConfig>> = Arc::new(RwLock::new(ParsedConfig::empty()));
  pub static ref GLOBAL_DOCKER_LOCK: Arc<RwLock<Docker>> = Arc::new(RwLock::new(Docker::empty()));
  pub static ref GLOBAL_REGISTRY_LOCK: Arc<RwLock<Registry>> = Arc::new(RwLock::new(Registry::new()));
  pub static ref GLOBAL_CONTAINER_LOGGER_LOCK: Arc<RwLock<Vec<ContainerLog>>> = Arc::new(RwLock::new(Vec::new()));

  pub static ref GLOBAL_CONTAINER_MAIN_LOCK: Arc<RwLock<Option<Container>>> = Arc::new(RwLock::new(None));
  pub static ref GLOBAL_CONTAINER_ROLLBACK_LOCK: Arc<RwLock<Option<Container>>> = Arc::new(RwLock::new(None));
  pub static ref GLOBAL_CONTAINER_NGINX_LOCK: Arc<RwLock<Option<Container>>> = Arc::new(RwLock::new(None));
}
