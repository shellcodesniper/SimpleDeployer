use shiplift::{ContainerOptions, LogsOptions};

use super::Container;

impl Container {
  pub async fn check_container_exist(self, container_name: String) -> bool {
    let container_list  = self.docker.docker.containers().list(&Default::default()).await;
    let mut found: bool = false;
    if let Ok(containers) = container_list {
      for container in containers {
        debug!("container name {:?}", container.names);
        if container.names.concat().contains(&container_name) {
          found = true;
          break;
        }
      }
    }
    found
  }

  pub async fn get_container_id(self, container_name: String) -> Option<String> {
    let container_list  = self.docker.docker.containers().list(&Default::default()).await;
    let mut found: Option<String> = None;
    if let Ok(containers) = container_list {
      for container in containers {
        println!("{:?}", container.names);
        if container.names.concat().contains(&container_name) {
          found = Some(container.id);
          break;
        }
      }
    }
    found
  }

  pub async fn create_container(self) -> Option<String> {
    let self_ptr = self.clone();
    let server_name = self_ptr.clone().role.clone().name();
    let container_exist = self_ptr.clone().check_container_exist(server_name.clone().to_string()).await;
    if container_exist {
      let container_id = self_ptr.clone().get_container_id(server_name.clone().to_string()).await;
      return container_id;
    }
    let image_selected_url = format!("{}:{}", self_ptr.clone().image, self_ptr.clone().role.tag());
    debug!("SELECTED IMAGE URL : {}", image_selected_url);
    let container_opts = ContainerOptions::builder(&image_selected_url)
      .name(&server_name)
      .expose(3000, "tcp", 3000)
      .auto_remove(true)
      .build();
      
    let create_result = self_ptr.clone().docker.docker.containers()
      .create(&container_opts)
      .await;
    
    
    if let Ok(info) = create_result {
      let start_result = self_ptr.clone().docker.docker
        .containers()
        .get(info.id.clone())
        .start()
        .await;
      if let Err(e) = start_result {
        println!("{}", e);
        return None;
      }
      Some(info.id.clone())
    } else {
      None 
    }
  }
}