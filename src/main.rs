#[macro_use] extern crate log;
mod lib;
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

  let _ = docker::Docker::new();

  let registry = registry::Registry::new();
  
  registry_login(registry).await;

  Ok(())
}

async fn registry_login(registry: registry::Registry) {
  let _ = registry.login().await;
}