use futures::StreamExt;
use shiplift::{ContainerListOptions, ContainerOptions, ExecContainerOptions, tty::TtyChunk};
use std::{collections::HashMap, str::from_utf8};
use rand::Rng;
use hyper::Client;
use hyper::{Body, Method, Request};

use crate::lib::global;

use super::Container;

fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("Stdout: {}", from_utf8(&bytes).unwrap()),
        TtyChunk::StdErr(bytes) => eprintln!("Stdout: {}", from_utf8(&bytes).unwrap()),
        TtyChunk::StdIn(_) => unreachable!(),
    }
}

impl Container {
  pub async fn check_container_exist(self, container_name: String, delete: bool) -> bool {
    let container_list  = self.docker.docker.containers().list(&ContainerListOptions::builder().all().build()).await;
    let mut found: bool = false;
    if let Ok(containers) = container_list {
      for container in containers {
        debug!("container name {:?}", container.names);
        if container.names.concat().contains(&container_name) {
          if delete {
            let _ = self.docker.docker.containers().get(container_name.clone()).kill(None).await;
            let _ = self.docker.docker.containers().get(container_name.clone()).delete().await;
            // ? Kill & Delete Existing Container
          }
          found = true;
          break;
        }
      }
    }
    found
  }

  pub async fn get_container_id(self, container_name: String) -> Option<String> {
    let container_list  = self.docker.docker.containers().list(&ContainerListOptions::builder().all().build()).await;
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
    let mut rng = rand::thread_rng();
    let self_ptr = self.clone();
    let server_name = self_ptr.clone().role.clone().name();
    let _ = self_ptr.clone().check_container_exist(server_name.clone().to_string(), true).await;
    // NOTE delete Container if Exist

    let image_selected_url = format!("{}:{}", self_ptr.clone().image, self_ptr.clone().role.tag());
    debug!("SELECTED IMAGE URL : {}", image_selected_url);

    let id = self.get_container_id(server_name.clone()).await;
    debug!("get container id result : {:?}", id);
    if let Some(id) = id {
      if let Err(e) = self_ptr.clone().docker.docker.containers().get(server_name.clone()).kill(None).await {
        error!("{:?}", e);
      }
      if let Err(e) = self_ptr.clone().docker.docker.containers().get(server_name.clone()).delete().await {
        error!("{:?}", e);
      }
    }
    

    let container_opts = if self_ptr.clone().role.name() == "nginx" {
      ContainerOptions::builder(&image_selected_url)
        .name(&server_name)
        .network_mode("overlay")
        .env(vec!["TARGET_CONTAINER=server_main", "TARGET_PORT=3000"])
        .links(vec!["server_main", "server_rollback"])
        .volumes(vec!["/tmp/kuuwange/certs/:/etc/certs/"])
        .volumes(vec!["/tmp/kuuwange/nginx/nginx.conf:/etc/nginx/nginx.conf", "/tmp/kuuwange/nginx/templates/:/etc/nginx/templates/", "/tmp/kuuwange/nginx/regenerate.sh:/etc/nginx/regenerate.sh"])
        .expose(80, "tcp", 80)
        .expose(443, "tcp", 443)
        // .auto_remove(true)
        .build()
    } else {
      ContainerOptions::builder(&image_selected_url)
        .name(&server_name)
        .network_mode("overlay")
        .volumes(vec!["/tmp/kuuwange/:/tmp/kuuwange/"])
        .expose(3000, "tcp", rng.gen_range(16300..17000))
        .auto_remove(true)
        .build()
    };
      
    let create_result = self_ptr.clone().docker.docker.containers()
      .create(&container_opts)
      .await;
    
    
    debug!("Create Image! {} Container_Name: {}", image_selected_url, server_name);
    match create_result {
      Ok(info) => {
        info!("Created Image Id : {}", info.id.clone());
        warn!("{:?}", info.warnings);
        return Some(info.id.clone())
      },
      Err (e) => {
        error!("Just Create Failed!");
        error!("{}", e);
        return None ;
      }
    }
  }

  pub async fn run(self) {
    let self_ptr = self.clone();
    let start_result = self_ptr.clone().docker.docker
        .containers()
        .get(self_ptr.id.clone())
        .start()
        .await;
      if let Err(e) = start_result {
        println!("{}", e);
      }
  }

  pub async fn execute_command(self, commands: Vec<&str>) {
    let self_ptr = self.clone();
    let container_id = self_ptr.clone().id.clone();
    let exec_opts = ExecContainerOptions::builder()
      .cmd(commands)
      .attach_stdout(false)
      .attach_stderr(false)
      .build();
    
    while let Some(exec_result) = self_ptr.docker.docker.containers().get(&container_id).exec(&exec_opts).next().await {
      match exec_result {
        Ok(chunk) => print_chunk(chunk),
        Err(e) => error!("Error On Execute Command\n{}", e),
      }
    };
  }

  pub async fn update_check(self) -> bool {
    let registry_ptr= global::GLOBAL_REGISTRY_LOCK.get().clone();
    let self_ptr = self.clone();
    let image_base_url = self_ptr.image.clone();
    let image_tag = self_ptr.role.tag().clone();


    let local_image_digest = self_ptr.docker.get_local_image_digest(image_base_url.clone(), Some(image_tag.clone())).await;
    let remote_image_digest = registry_ptr.get_digest_of_image(image_base_url, Some(image_tag)).await;

    let local_image_digest = if local_image_digest.found { local_image_digest.image_digest.unwrap_or(String::from("err")) } else { String::from("err")};
    if remote_image_digest.found && remote_image_digest.digest.is_some() {
      if local_image_digest != remote_image_digest.digest.unwrap() {
        return true;
      }
    }
    false
  }
  
  pub async fn is_healthy(self) -> bool {
    let client = Client::new();

    let request_url = format!("http://{}:{}/", self.role.name(), 3000);
    // NOTE request target url ( docker container )

    let req = Request::builder()
      .method(Method::GET)
      .uri(request_url)
      .header("content-type", "application/json")
      .body(Body::from("")).unwrap();
    let req = client.request(req).await;
    if let Ok(respbody) = req {
      if respbody.status() == 200 {
        return true;
      }
    }
    false
    
  }
}
