use futures::StreamExt;
use shiplift::{tty::TtyChunk, ContainerOptions, ExecContainerOptions };
use std::str::from_utf8;

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
    let container_list  = self.docker.docker.containers().list(&Default::default()).await;
    let mut found: bool = false;
    if let Ok(containers) = container_list {
      for container in containers {
        debug!("container name {:?}", container.names);
        if container.names.concat().contains(&container_name) {
          if delete {
            let _ = self.docker.docker.containers().get(container.clone().id).kill(None).await;
            let _ = self.docker.docker.containers().get(container.clone().id).delete().await;
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
    let _ = self_ptr.clone().check_container_exist(server_name.clone().to_string(), true).await;
    // NOTE delete Container if Exist

    let image_selected_url = format!("{}:{}", self_ptr.clone().image, self_ptr.clone().role.tag());
    debug!("SELECTED IMAGE URL : {}", image_selected_url);

    let id = self.get_container_id(server_name.clone()).await;
    debug!("get container id result : {:?}", id);
    if let Some(id) = id {
      if let Err(e) = self_ptr.clone().docker.docker.containers().get(id.clone()).kill(None).await {
        error!("{:?}", e);
      }
      if let Err(e) = self_ptr.clone().docker.docker.containers().get(id.clone()).delete().await {
        error!("{:?}", e);
      }
    }

    let container_opts = ContainerOptions::builder(&image_selected_url)
      .name(&server_name)
      .expose(3000, "tcp", 3000)
      .auto_remove(true)
      .build();
      
    let create_result = self_ptr.clone().docker.docker.containers()
      .create(&container_opts)
      .await;
    
    
    debug!("Create Image! {} Container_Name: {}", image_selected_url, server_name);
    match create_result {
      Ok(info) => {
        info!("Created Image Id : {}", info.id.clone());
        warn!("{:?}", info.warnings);
        std::thread::sleep(std::time::Duration::from_secs(3));
        return Some(info.id.clone())
      },
      Err (e) => {
        error!("Just Create Failed!");
        error!("{}", e);
        return None ;
      }
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
}
