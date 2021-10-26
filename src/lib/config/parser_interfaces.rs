pub struct Default {
  pub container_prefix: String,
  pub burnup_waiting: i64,
  pub docker_socket: String,

  pub health_check_interval: i64,

  pub update_check_interval: i64,
  pub update_check_use_cron: bool,
  pub update_check_cron_text: String,

}

pub struct Nginx {
  pub nginx: bool,
  pub http_redirect: bool,

  pub ssl: bool,
  pub ssl_fullchain: Option<String>,
  pub ssl_privkey: Option<String>,
}

pub struct Repository {
  pub docker_hub_target_repo: String,

  pub docker_hub_login_info: bool,
  pub docker_hub_username: Option<String>,
  pub docker_hub_password: Option<String>,
}

pub struct Logging {
  pub logging: bool,

  pub controller_logname: String,
  pub logging_path: String,
  pub logging_prefix: String,
  pub max_file_size_mb: i64,

  pub server_identity_prefix: String,
  pub logging_s3: bool,
}

pub struct S3 {
  pub bucket: Option<String>,
  pub access_key: Option<String>,
  pub secret_key: Option<String>,
  pub region_name: Option<String>,
  pub root_path: Option<String>,
}
