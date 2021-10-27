pub enum HealthyStatus {
  Ready,
  Starting,
  Healthy,
  Unhealthy,
}

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

pub struct ContainerStatus {
  pub healthy: HealthyStatus,
  container_id: String,
}