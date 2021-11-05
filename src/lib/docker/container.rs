pub mod modify;
pub mod logger;
use super::Docker;

#[allow(unused_imports)]
use modify::*;
#[allow(unused_imports)]
use logger::*;

#[derive(Clone)]
pub enum ContainerRole {
  Main,
  Dev,
  Rollback,
  Nginx,
  None,
}
impl ContainerRole {
  pub fn name(self) -> String {
    match self {
      ContainerRole::Main => "server_main".to_string(),
      ContainerRole::Dev => "server_main".to_string(),
      ContainerRole::Rollback => "server_rollback".to_string(),
      ContainerRole::Nginx => "nginx".to_string(),
      ContainerRole::None => "NONE".to_string(),
    }
  }
  pub fn tag(self) -> String {
    match self {
      ContainerRole::Main => "stable".to_string(),
      ContainerRole::Dev => "latest".to_string(),
      ContainerRole::Rollback => "rollback".to_string(),
      ContainerRole::Nginx => "stable-alpine".to_string(),
      ContainerRole::None => "latest".to_string(),
    }
  }
}

impl Default for ContainerRole {
  fn default() -> Self {
    ContainerRole::None
  }
}

#[derive(Default, Clone)]
pub struct Container {
  docker: Docker, 
  pub id: String,
  name: String,
  image: String,
  digest: String,
  role: ContainerRole,
  network_connected: bool,
}

impl Container {
  pub fn new(docker: Docker, image: String, role: String) -> Container {
    let role = match role.as_str().trim().to_lowercase().as_str() {
      "main" => ContainerRole::Main,
      "dev" => ContainerRole::Dev,
      "rollback" => ContainerRole::Rollback,
      "nginx" => ContainerRole::Nginx,
      _ => ContainerRole::None,
    };

    let this_container = Container {
      docker: docker.clone(),
      id: String::new(),
      name: role.clone().name(),
      image: image.clone(),
      digest: String::new(),
      role: role.clone(),
      network_connected: false,
    };

    debug!("CONTAINER CREATE! container_id : {} name : {} digest: {}", this_container.id, this_container.name, this_container.digest);

    let mut this_container_ptr = this_container.clone();

    let changed_container = std::thread::spawn(move || {
      let create_result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
          let create_result = this_container_ptr.clone().create_container().await;
          create_result
        });

      if let Some(x) = create_result {
        info!("Creation result: {}", x);
        this_container_ptr.id = x.clone().to_owned();
        debug!("PTR_ID : {}", this_container_ptr.id);
      } else {
        error!("Creation result : Failed..");
        error!("Please Check Config");
      };

      this_container_ptr
    }).join().unwrap();

    debug!("ID result : {}", changed_container.id.clone());

    changed_container
  }  
}
