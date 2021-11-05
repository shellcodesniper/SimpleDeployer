use super::lib::{ config, docker, registry, utils, logger, global};
pub mod container;


pub async fn entrypoint(main_docker: docker::Docker, registry: registry::Registry) {
  global::GLOBAL_DOCKER_LOCK.set(main_docker);
  global::GLOBAL_REGISTRY_LOCK.set(registry);
  info!("Enter Program EntryPoint");

  container::controller_check_outdated_and_pull_for_start().await;
  // download main, rollback, nginx -> create container

  container::controller_start_stage().await;
  // start main, nginx
}