use crate::lib::global;
use crate::lib::docker::container::Container;

pub async fn controller_check_outdated() {
  let config= global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();
  let docker = global::GLOBAL_DOCKER_LOCK.get();
  let registry = global::GLOBAL_REGISTRY_LOCK.get();
  // Global Vars to Use

  let config_ptr = config.clone().to_owned();
  let docker_ptr = docker.clone().to_owned();
  let registry_ptr = registry.clone().to_owned();
  // Pointers

  let is_development = config_ptr.clone().default.is_development;
  let image_base_url = config_ptr.clone().repository.registry_target_repo.clone();
  // Gloval Variable Destructed

  let main_image_tag = if is_development { String::from("latest") } else { String::from("stable") };

  let mut image_main_need_download = true;
  let registry_main_image_digest = registry_ptr.clone().get_digest_of_image(image_base_url.clone(), Some(main_image_tag.clone())).await;
  let local_main_image_digest = docker_ptr.clone().get_local_image_digest(image_base_url.clone(), Some(main_image_tag.clone())).await;
  if registry_main_image_digest.found{
    let digest_remote = registry_main_image_digest.digest;
    if let Some(digest_remote) = digest_remote {
      if local_main_image_digest.found {
        if let Some(digest_local) = local_main_image_digest.image_digest {
          if digest_remote != digest_local {
            info!("Main Image Have Update!\n");
            info!("Download Main Image (Dev Mode? {})", is_development);
          } else {
            info!("Local Main Image Already Updated");
            image_main_need_download = false;
          }
        } else {
          info!("Main Image Not Found on Local\n");
          info!("Download Main Image (Dev Mode? {})", is_development);
        }
      }
    }
  } else {
    error!("Unable To Check Registry [Main] Image");
    error!("please check your registry_target_repo setting");
    std::process::exit(-1);
  }
  if image_main_need_download {
    controller_download_stage(image_base_url.clone(), main_image_tag.clone()).await;
  }
  // NOTE download Latest Main Image

  let rollback_image_tag = String::from("rollback");

  let mut image_rollback_need_download = true;
  let registry_rollback_image_digest = registry_ptr.clone().get_digest_of_image(image_base_url.clone(), Some(rollback_image_tag.clone())).await;
  let local_rollback_image_digest = docker_ptr.clone().get_local_image_digest(image_base_url.clone(), Some(rollback_image_tag.clone())).await;
  if registry_rollback_image_digest.found{
    let digest_remote = registry_rollback_image_digest.digest;
    if let Some(digest_remote) = digest_remote {
      if local_rollback_image_digest.found {
        if let Some(digest_local) = local_rollback_image_digest.image_digest {
          if digest_remote != digest_local {
            info!("Rollback Image Have Update!\n");
            info!("Download Rollback Image (Dev Mode? {})", is_development);
          } else {
            info!("Local Rollback Image Already Updated");
            image_rollback_need_download = false;
          }
        } else {
          info!("Rollback Image Not Found on Local\n");
          info!("Download Rollback Image (Dev Mode? {})", is_development);
        }
      }
    }
  } else {
    error!("Unable To Check Registry [Rollback] Image");
    error!("please check your registry_target_repo setting");
    std::process::exit(-1);
  }
  if image_rollback_need_download {
    controller_download_stage(image_base_url.clone(), rollback_image_tag.clone()).await;
  }
  // NOTE download Latest Rollback Image

  info!("Nginx Image Have Update\n");
  info!("Download Nginx Image");
  controller_download_stage(String::from("nginx"), String::from("stable-alpine")).await;
  // NOTE download Stable-Alpine Nginx Image

  
  controller_create_stage(image_base_url.clone().to_owned(), is_development).await;

}

pub async fn controller_download_stage(image_base_url: String, image_tag: String) {
  let docker = global::GLOBAL_DOCKER_LOCK.get();

  docker.download_image(image_base_url, Some(image_tag)).await;
}
// NOTE downloader

pub async fn controller_create_stage(image_base_url: String, is_development: bool) {
  let docker = global::GLOBAL_DOCKER_LOCK.get();
  let config= global::GLOBAL_PARSED_CONFIG_LOCK.get().clone();

  let burn_up_time = config.default.burnup_waiting.unsigned_abs();
  let docker_ptr = docker.clone();
  
  let main_role = if is_development { String::from("dev") } else { String::from("main") };

  let container_main = Container::new(docker.clone().to_owned(), image_base_url.clone(), main_role);
  let container_rollback = Container::new(docker.clone().to_owned(), image_base_url.clone(), String::from("rollback"));
  let container_nginx = Container::new(docker.clone().to_owned(), String::from("nginx"), String::from("nginx"));
  // ? Create Container Object

  global::GLOBAL_CONTAINER_MAIN_LOCK.set(Some(container_main));

}
// NOTE create container & Register Global Variable

pub async fn controller_start_stage() {

}

pub async fn controller_nginx_reload_stage() {

}