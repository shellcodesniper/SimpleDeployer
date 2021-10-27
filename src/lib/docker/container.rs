pub mod modify;

#[allow(unused_imports)]
use modify::*;

#[derive(Clone)]
pub enum ContainerRole {
  Main,
  Rollback,
  None,
}

impl Default for ContainerRole {
  fn default() -> Self {
    ContainerRole::None
  }
}

#[derive(Default, Clone)]
pub struct Container {
    id: String,
    name: String,
    image: String,
    digest: String,
    role: ContainerRole,
    network_connected: bool,
}

impl Container {
  pub fn new(name: String, image: String, role: String) -> Container {
    let role = match role.as_str() {
      "main" => ContainerRole::Main,
      "rollback" => ContainerRole::Rollback,
      _ => ContainerRole::None,
    };

    Container {
      id: String::new(),
      name: name,
      image: image,
      digest: String::new(),
      role: role,
      network_connected: false,
    }
  }
}
