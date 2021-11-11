use crate::lib::docker::container::ContainerRole;


 #[derive(Clone, Copy, Debug)]
pub enum HealthyStatus {
  Ready,
  Starting,
  Healthy,
  Unhealthy,
  Updating,
}

#[derive(Clone, Debug)]
pub struct SystemStatus {
  pub in_update: bool,
  pub main: HealthyStatus,
  pub main_ip: Option<String>,
  pub rollback: HealthyStatus,
  pub rollback_ip: Option<String>,
  pub nginx: HealthyStatus,
  pub nginx_target_role: ContainerRole,
  pub nginx_target_ip: String,
}

impl SystemStatus {
  pub fn new() -> SystemStatus {
    SystemStatus {
      in_update: false,
      main: HealthyStatus::Ready,
      main_ip: None,
      rollback: HealthyStatus::Ready,
      rollback_ip: None,
      nginx: HealthyStatus::Ready,
      nginx_target_role: ContainerRole::Main,
      nginx_target_ip: String::from(""),
    }
  }

}