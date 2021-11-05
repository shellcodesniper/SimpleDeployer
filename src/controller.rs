use super::lib::{ config, docker, registry, utils, logger, global};
pub mod container;


pub async fn entrypoint(main_docker: docker::Docker, registry: registry::Registry) {
  prepare_nginx_config();
  global::GLOBAL_DOCKER_LOCK.set(main_docker);
  global::GLOBAL_REGISTRY_LOCK.set(registry);
  info!("Enter Program EntryPoint");

  container::controller_check_outdated_and_pull_for_start().await;
  // download main, rollback, nginx -> create container

  container::controller_start_stage().await;
  // start main, nginx
}

fn prepare_nginx_config() {
  use fs_extra::dir::{ copy, CopyOptions };
  let config = global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();

  let use_ssl = config.nginx.ssl;

  let mut options = CopyOptions::new();
  options.overwrite = true;
  options.copy_inside = true;
  if use_ssl {
    let _ = copy("/app/nginx/ssl", "/tmp/kuuwange/nginx", &options);
  } else {
    let _ = copy("/app/nginx/ssl", "/tmp/kuuwange/nginx", &options);
  };
}