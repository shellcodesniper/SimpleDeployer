pub mod container;
pub mod image;

use tokio;
// use futures::{Future, StreamExt};

#[allow(unused_imports)]
use image::*;

use super::global;
use super::config::parser::ParsedConfig;
use shiplift::{Docker as ShipDocker, PullOptions, RegistryAuth};

#[derive(Clone, Default)]
pub struct Docker {
  docker: ShipDocker,
  auth: Option<RegistryAuth>,
}

impl Docker {
  pub fn empty() -> Docker {
    Docker {
      ..Default::default()
    }
  }
  pub fn new() -> Docker {
    let config: ParsedConfig = global::GLOBAL_PARSED_CONFIG_LOCK.get();
    let docker = ShipDocker::unix(config.default.docker_socket.clone());

    let x = if config.repository.registry_login_info {
      if (!config.repository.registry_username.is_some()) || (!config.repository.registry_password.is_some()) {
        panic!("DOCKER LOGIN INFO was true but username or password not provided");
      }

      let username = config.repository.registry_username.unwrap().clone();
      let password = config.repository.registry_password.unwrap().clone();
      
      let auth: RegistryAuth = RegistryAuth::builder()
        .username(username)
        .password(password)
        .build();
      Docker {
        docker,
        auth: Some(auth),
      }
    } else {
      Docker {
        docker,
        auth: None,
      }
    };

    info!("Docker Authentication Complete");

    let x_test = x.clone();

    let test_thread = std::thread::spawn(move || {
      let connect_result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
          let test_result = x_test.test_connection().await;
          test_result
        });

      if connect_result {
        info!("Connection result : {}", connect_result);
      } else {
        error!("Connection result : {}", connect_result);
        error!("Please Check Repository Settings in config");
      };

    });
    let _ = test_thread.join();

    global::GLOBAL_DOCKER_LOCK.set(x.clone());
    x
  }
  pub async fn test_connection(self) -> bool {
    let self_clone = self.clone();
    let result = self.docker.ping().await.is_ok();
    result
  }
}