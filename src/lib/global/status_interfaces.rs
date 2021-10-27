
 #[derive(Clone, Copy, Debug)]
pub enum HealthyStatus {
  Ready,
  Starting,
  Healthy,
  Unhealthy,
}

#[derive(Clone, Debug)]
pub struct SystemStatus {
  pub healthy: HealthyStatus,
  pub container_list: Vec<ContainerStatus>,
}

impl SystemStatus {
  pub fn new() -> SystemStatus {
    SystemStatus {
      healthy: HealthyStatus::Ready,
      container_list: vec![],
    }
  }
}

#[derive(Clone, Debug)]
pub struct ContainerStatus {
  pub healthy: HealthyStatus,
  container_id: String,
}