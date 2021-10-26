use std::{fs::File, process::exit};
use std::io::prelude::*;
use std::any::Any;
use configparser::ini::Ini;

use super::parser_interfaces::{ Default, Nginx, Repository, Logging, S3 };

pub struct ParsedConfig {
  pub default: Default,
  pub nginx: Nginx,
  pub repository: Repository,
  pub logging: Logging,
  pub s3: S3,
}

impl ParsedConfig {

  pub fn new(cfg_path: Option<&str>) {
    let config_path = if let Some(x) = cfg_path { x } else { "deployConfig.cfg" };

    let mut file = File::open(config_path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut config = Ini::new();
    let config_parse_result= config.read(contents);
    if let Ok(_) = config_parse_result {
      println!("CONFIG_PARSE_INIT");
    } else {
      println!("PLEASE CHECK YOUR CONFIG FILE");
      exit(-1);
    };

    let default  = Default {
      container_prefix: config.get("Default", "container_prefix").unwrap_or(String::from("server")),
      burnup_waiting: config.getint("Default", "burnup_waiting").unwrap().unwrap_or(5),
      docker_socket: config.get("Default", "docker_socket").unwrap_or(String::from("unix://var/run/docker.sock")),

      health_check_interval: config.getint("Default", "health_check_interval").unwrap().unwrap_or(5),
      update_check_interval: config.getint("Default", "update_check_interval").unwrap().unwrap_or(10),
      update_check_use_cron: (config.get("Default", "update_check_use_cron").unwrap_or(String::from("no"))) == "yes",
      update_check_cron_text: config.get("Default", "update_check_cron_text").unwrap_or(String::from("*/2 * * * *")),
    };

    let nginx = Nginx {
      nginx: (config.get("Nginx", "nginx").unwrap_or(String::from("no")) == "yes"),
      http_redirect: (config.get("Nginx", "http_redirect").unwrap_or(String::from("no")) == "yes"),
      ssl: (config.get("Nginx", "ssl").unwrap_or(String::from("no")) == "yes"),
      ssl_fullchain: config.get("Nginx", "ssl_fullchain"),
      ssl_privkey: config.get("Nginx", "ssl_privkey"),
    };

    let repository = Repository {
      docker_hub_target_repo: config.get("Repository", "docker_hub_target_repo").unwrap(),
      docker_hub_login_info: (config.get("Default", "docker_hub_login_info").unwrap_or(String::from("no")) == "yes"),
      docker_hub_username: config.get("Repository", "docker_hub_username"),
      docker_hub_password: config.get("Repository", "docker_hub_password"),
    };

    let logging = Logging {
      logging: (config.get("Logging", "logging").unwrap_or(String::from("no")) == "yes"),

      controller_logname: config.get("Logging", "controller_logname").unwrap_or(String::from("KUUWANGE")),
      time_rotation: config.getint("Logging", "time_rotation").unwrap().unwrap_or(300),
      max_file_size_mb: config.getint("Logging", "max_file_size_mb").unwrap().unwrap_or(5),

      server_identity_prefix: config.get("Logging", "server_identity_prefix").unwrap_or(String::from("KUUWANGE_SERVER")),
      logging_s3: (config.get("Logging", "logging_s3").unwrap_or(String::from("no")) == "yes"),
    };

    let s3 = S3 {
      bucket: config.get("S3", "bucket"),
      access_key: config.get("S3", "access_key"),
      secret_key: config.get("S3", "secret_key"),
      region_name: config.get("S3", "region_name"),
      root_path: config.get("S3", "root_path"),
    };

    // * return parsedConfig Structure
    ParsedConfig {
      default,
      nginx,
      repository,
      logging,
      s3,
    };
  }
}