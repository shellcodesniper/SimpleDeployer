use crate::lib::docker::container::ContainerRole;

use super::lib::{ docker, registry, global};
use chrono::{DateTime, Duration, Utc };
#[allow(unused_imports)]
use chrono_tz::{ Tz, Asia::Seoul };
use cron_parser::parse;
pub mod container;
use super::lib::global::status_interfaces::HealthyStatus;


pub async fn control_loop() {
  let config = global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();
  let update_interval = config.default.update_check_interval;
  let update_use_cron = config.default.update_check_use_cron;
  let update_cron_text = config.default.update_check_cron_text;
  let health_check_interval = config.default.health_check_interval;

  let mut last_update_check_time: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
  let mut last_health_check_time: DateTime<Tz> = Utc::now().with_timezone(&Seoul);


  loop {
    if check_next_update_time(last_update_check_time, update_use_cron, update_interval, update_cron_text.clone()) {
      last_update_check_time = Utc::now().with_timezone(&Seoul);
      debug!("check update");
      check_update_and_update_container().await;
    }
    // let main_healthy = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap().is_healthy().await;
    // debug!("Main Healthy? : {}", main_healthy);

    if health_check_time(last_health_check_time, health_check_interval).await {
      last_health_check_time = Utc::now().with_timezone(&Seoul);
      debug!("health check");
      health_check_and_report().await;
    }
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

async fn health_check_time(last_health_check_time: DateTime<Tz>, health_check_interval: i64) -> bool {
 let next: DateTime<Tz> = last_health_check_time + Duration::seconds(health_check_interval);
 if next < Utc::now().with_timezone(&Seoul) {
   return true;
 }
 false
}

async fn health_check_and_report() {
  let container_main = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap();
  let container_rollback = global::GLOBAL_CONTAINER_ROLLBACK_LOCK.get().unwrap();


  let main_healthy = container_main.is_healthy().await;
  global::GLOBAL_SYSTEM_STATUS_LOCK.set_main(if main_healthy { HealthyStatus::Healthy } else { HealthyStatus::Unhealthy });
  let rollback_healthy = container_rollback.is_healthy().await;
  global::GLOBAL_SYSTEM_STATUS_LOCK.set_rollback(if rollback_healthy { HealthyStatus::Healthy } else { HealthyStatus::Unhealthy });

  if global::GLOBAL_SYSTEM_STATUS_LOCK.is_updating() {
    debug!("In Update Now, Be Patience");
  } else {
    if main_healthy && rollback_healthy {
      debug!("Main and Rollback is healthy");
      let main_container_role = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap().role;
      let main_container_ip = global::GLOBAL_SYSTEM_STATUS_LOCK.get_main_ip();

      let current_nginx_role= global::GLOBAL_SYSTEM_STATUS_LOCK.get_nginx_target();
      let current_nginx_target_ip = global::GLOBAL_SYSTEM_STATUS_LOCK.get_nginx_ip();
      if main_container_role.clone().name() == current_nginx_role.name() && main_container_ip.is_some() && main_container_ip.unwrap_or(String::from("")) == current_nginx_target_ip {
          debug!("Already Nginx Pointed to Main");
      } else {
        info!("Change Nginx Target To Main");
        global::GLOBAL_SYSTEM_STATUS_LOCK.set_nginx_target(main_container_role);
        global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(global::GLOBAL_SYSTEM_STATUS_LOCK.get_main_ip(), None).await;
      }
      debug!("Kill Rollback");
      global::GLOBAL_CONTAINER_ROLLBACK_LOCK.get().unwrap().stop_self().await;
    } else if main_healthy {
      debug!("Only Main Healthy");
      let main_container_role = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap().role;
      let main_container_ip = global::GLOBAL_SYSTEM_STATUS_LOCK.get_main_ip();

      let current_nginx_role= global::GLOBAL_SYSTEM_STATUS_LOCK.get_nginx_target();
      let current_nginx_target_ip = global::GLOBAL_SYSTEM_STATUS_LOCK.get_nginx_ip();
      if main_container_role.clone().name() == current_nginx_role.name() && main_container_ip.is_some() && main_container_ip.unwrap_or(String::from("")) == current_nginx_target_ip {
        info!("EveryThing Looks Normal");
      } else {
        info!("Change Nginx Target To Main");
        global::GLOBAL_SYSTEM_STATUS_LOCK.set_nginx_target(main_container_role);
        global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(global::GLOBAL_SYSTEM_STATUS_LOCK.get_main_ip(), None).await;
      }
    } else if rollback_healthy {
      warn!("Rollback is Healthy & Main is UnHealthy");
      let rollback_container_role = global::GLOBAL_CONTAINER_ROLLBACK_LOCK.get().unwrap().role;
      let rollback_container_ip = global::GLOBAL_SYSTEM_STATUS_LOCK.get_rollback_ip();

      let current_nginx_role= global::GLOBAL_SYSTEM_STATUS_LOCK.get_nginx_target();
      let current_nginx_target_ip = global::GLOBAL_SYSTEM_STATUS_LOCK.get_nginx_ip();
      if rollback_container_role.clone().name() == current_nginx_role.clone().name() && rollback_container_ip.is_some() && rollback_container_ip.unwrap_or(String::from("")) == current_nginx_target_ip {
          debug!("Already Nginx Pointed to Rollback");
      } else {
        info!("Change Nginx Target To Rollback");
        global::GLOBAL_SYSTEM_STATUS_LOCK.set_nginx_target(rollback_container_role.clone());
        global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(global::GLOBAL_SYSTEM_STATUS_LOCK.get_rollback_ip(), None).await;
      }
    } else {
      error!("Main and Rollback is not Healthy");
      global::GLOBAL_CONTAINER_MAIN_LOCK.get().clone().unwrap().run().await;
      global::GLOBAL_CONTAINER_ROLLBACK_LOCK.get().clone().unwrap().run().await;
      // global::GLOBAL_SYSTEM_STATUS_LOCK.set_nginx_target(ContainerRole::None);

      global::GLOBAL_SYSTEM_STATUS_LOCK.set_nginx_target(ContainerRole::Main);
      global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(global::GLOBAL_SYSTEM_STATUS_LOCK.get_main_ip(), None).await;
      error!("Just Tried to Wake up Main & Rollback Service, Hope to God");
    }
  }
}

async fn check_update_and_update_container() {
  info!("===UPDATE  CHECK===");

  let docker = global::GLOBAL_DOCKER_LOCK.get().clone();
  let config = global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();

  let is_development = config.clone().default.is_development;
  let image_base_url = config.clone().repository.registry_target_repo;
  let burn_up_time = config.default.burnup_waiting.clone().unsigned_abs();

  let container_main = global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap();
  let container_rollback= global::GLOBAL_CONTAINER_MAIN_LOCK.get().unwrap();

  global::GLOBAL_SYSTEM_STATUS_LOCK.set_update_start();
  if container_main.clone().update_check().await {
    info!("Main Container Will Update!");
    global::GLOBAL_CONTAINER_ROLLBACK_LOCK.get().clone().unwrap().run().await;
    std::thread::sleep(std::time::Duration::from_secs(burn_up_time));

    container_rollback.clone().run().await;
    global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(global::GLOBAL_SYSTEM_STATUS_LOCK.get_rollback_ip(), None).await;
    container_main.stop_self().await;

    global::GLOBAL_CONTAINER_MAIN_LOCK.set(None);


    let main_role = if is_development { String::from("dev") } else { String::from("main") };
    let main_tag = if is_development { String::from("latest") } else { String::from("stable") };
    container::controller_download_stage(image_base_url.clone(), main_tag.clone()).await;
    let container_main = docker::container::Container::new(docker.clone().to_owned(), image_base_url.clone(), main_role);
    global::GLOBAL_CONTAINER_MAIN_LOCK.set(Some(container_main.clone()));

    global::GLOBAL_CONTAINER_MAIN_LOCK.get().clone().unwrap().run().await;
    std::thread::sleep(std::time::Duration::from_secs(burn_up_time));
  }
  if container_rollback.clone().update_check().await {
    info!("Rollback Container Will Update!");
    global::GLOBAL_CONTAINER_NGINX_LOCK.change_target(global::GLOBAL_SYSTEM_STATUS_LOCK.get_main_ip(), None).await;
    container_rollback.stop_self().await;

    container::controller_download_stage(image_base_url.clone(), String::from("rollback")).await;
    let container_rollback = docker::container::Container::new(docker.clone().to_owned(), image_base_url.clone(), String::from("rollback"));
    global::GLOBAL_CONTAINER_ROLLBACK_LOCK.set(Some(container_rollback));
  }
  global::GLOBAL_SYSTEM_STATUS_LOCK.set_update_finish();

}

pub async fn entrypoint(main_docker: docker::Docker, registry: registry::Registry) {
  global::GLOBAL_DOCKER_LOCK.set(main_docker);
  global::GLOBAL_REGISTRY_LOCK.set(registry);
  info!("Enter Program EntryPoint");

  container::controller_check_outdated_and_pull_for_start().await;
  // download main, rollback, nginx -> create container

  container::controller_start_stage().await;
  // start main, nginx

  control_loop().await;
}
