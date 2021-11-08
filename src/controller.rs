use super::lib::{ config, docker, registry, utils, logger, global};
use chrono::{DateTime, Duration, Utc };
#[allow(unused_imports)]
use chrono_tz::{ Tz, Asia::Seoul };
use cron_parser::parse;
pub mod container;


fn prepare_nginx_config() {
  use fs_extra::dir::{ copy, CopyOptions };
  let config = global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();

  let use_ssl = config.nginx.ssl;

  let mut options = CopyOptions::new();
  options.overwrite = true;
  options.copy_inside = true;
  if use_ssl {
    let _ = copy("/tmp/kuuwange/certs/", "/app/certs/", &options);
  }
}

pub async fn control_loop() {
  let container_main = global::GLOBAL_CONTAINER_MAIN_LOCK.get();
  let container_rollback= global::GLOBAL_CONTAINER_MAIN_LOCK.get();

  let config = global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();
  let update_interval = config.default.update_check_interval;
  let update_use_cron = config.default.update_check_use_cron;
  let update_cron_text = config.default.update_check_cron_text;

  let mut last_update_check_time: DateTime<Tz> = Utc::now().with_timezone(&Seoul);

  loop {
    if check_next_update_time(last_update_check_time, update_use_cron, update_interval, update_cron_text.clone()) {
      last_update_check_time = Utc::now().with_timezone(&Seoul);
      debug!("check update");
      check_update_and_update_container().await;
    }
    // let main_healthy = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap().is_healthy().await;
    // debug!("Main Healthy? : {}", main_healthy);
    std::thread::sleep(std::time::Duration::from_millis(10));
  }

}

fn check_next_update_time(last_update_check_time: DateTime<Tz>, update_use_cron: bool, update_interval: i64, update_cron_text: String) -> bool {
  let mut next: DateTime<Tz> = last_update_check_time + Duration::seconds(update_interval);
  if update_use_cron {
    if let Ok(cron_next) = parse(&update_cron_text, &last_update_check_time) {
      next = cron_next;
    }
  }
  if next < Utc::now().with_timezone(&Seoul) {
    return true;
  }
  return false;
}

async fn check_update_and_update_container() {

  let docker = global::GLOBAL_DOCKER_LOCK.get().clone();
  let config = global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();

  let is_development = config.clone().default.is_development;
  let image_base_url = config.clone().repository.registry_target_repo;

  let container_main = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap();
  let container_rollback= global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap();

  if container_main.clone().update_check().await {
    info!("Main Container Will Update!");
    container_rollback.clone().run().await;
    global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(Some(container_rollback.name.clone()), None).await;
    container_main.stop_self().await;
    let main_role = if is_development { String::from("dev") } else { String::from("main") };
    let container_main = docker::container::Container::new(docker.clone().to_owned(), image_base_url.clone(), main_role);
    global::GLOBAL_CONTAINER_MAIN_LOCK.set(Some(container_main.clone()));
    container_main.run().await;
  }
  if container_rollback.clone().update_check().await {
    info!("Rollback Container Will Update!");
    let container_main = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap();
    global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(Some(container_main.clone().name.clone()), None).await;
    container_rollback.stop_self().await;
    let container_rollback = docker::container::Container::new(docker.clone().to_owned(), image_base_url.clone(), String::from("rollback"));
    global::GLOBAL_CONTAINER_ROLLBACK_LOCK.set(Some(container_rollback));
  }

}

pub async fn entrypoint(main_docker: docker::Docker, registry: registry::Registry) {
  prepare_nginx_config();
  global::GLOBAL_DOCKER_LOCK.set(main_docker);
  global::GLOBAL_REGISTRY_LOCK.set(registry);
  info!("Enter Program EntryPoint");

  container::controller_check_outdated_and_pull_for_start().await;
  // download main, rollback, nginx -> create container

  container::controller_start_stage().await;
  // start main, nginx

  control_loop().await;
}
