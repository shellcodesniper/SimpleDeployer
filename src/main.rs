#[macro_use] extern crate log;
mod lib;
mod controller;
use std::process::exit;

use lib::{ config, docker, registry, utils, logger};

fn print_usage(args: Vec<String>) {
  println!("Usage: {} [config-file-path]", args[0]);
  println!("\tSample: {} ./bin/sampleConfig.cfg", args[0]);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    print_usage(args);
    exit(-1);
  }
  let config_file_path = String::from(&args[1]);
  let file_exist = utils::io::check_str_file_exist(config_file_path.clone());
  if !file_exist {
    println!("\n\n\nFILE NOT EXIST!!!\n\n");
    print_usage(args);
  exit(-2);
  }
  config::parser::ParsedConfig::new(config_file_path.clone());
  logger::log_init();
  info!("=== INITIALIZING DONE ===");

  let main_docker = docker::Docker::new();

  let mut registry = registry::Registry::new();
  
  registry = registry_login(registry).await;

  // NOTE 여기부터 도커, 레지스트리 준비가 완료된 시점임.
  // test_script(main_docker, registry).await;
  // TEST SCRIPT

  controller::entrypoint(main_docker, registry).await;
  Ok(())
}
async fn registry_login(mut registry: registry::Registry) -> registry::Registry {
  let _ = (registry).login().await;
  registry
}

#[allow(dead_code)]
async fn registry_get_digest_test(registry: registry::Registry) {
  let result = registry.clone().get_digest_of_image(String::from("nginx"), Some(String::from("stable-alpine"))).await;
  debug!("REsult Get");
  if result.found {
    debug!("FOUND : <{}:{}> {:?}", result.image_url, result.tag, result.digest);
  } else {
    debug!("NOT FOUND: <{}:{}> {:?}", result.image_url, result.tag, result.digest);
  }
  let result = registry.clone().get_digest_of_image(String::from("nginx"), Some(String::from("stable-alpine"))).await;
  if result.found {
    debug!("FOUND : <{}:{}> {}", result.image_url, result.tag, result.digest.unwrap());
  }
}

#[allow(dead_code)]
async fn test_script(docker: docker::Docker, registry: registry::Registry) {
  let config = lib::global::GLOBAL_PARSED_CONFIG_LOCK.get();
  let burn_up_time = config.default.burnup_waiting;
  debug!("test script start");
  registry_get_digest_test(registry.clone()).await;
  debug!("First Script Done");
  let r = docker.clone().get_local_image_digest(String::from("nginx"), None).await;
  debug!("{:?}", r);

  let image_downloaded = docker.clone().download_image(String::from("nginx"), Some(String::from("stable-alpine"))).await;
  debug!("Image Download ? {}", image_downloaded);
  let container = docker::container::Container::new(docker.clone(), String::from("nginx"), String::from("nginx"));
  debug!("CONTAINER : {}", container.id);

  debug!("burnup waiting time {}", burn_up_time.clone());

  std::thread::sleep(std::time::Duration::from_secs(burn_up_time.unsigned_abs()));

  debug!("TEST LOGGING");
  container.clone().attach_logger().await;
  // NOTE 한번 호출할때마다 stdout, stderr 데이터 받아옴
  debug!("EXEC COMMAND");
  let command = vec!["nginx", "-s", "reload"];
  container.execute_command(command).await;
}