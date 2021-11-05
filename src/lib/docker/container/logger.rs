use futures::StreamExt;
use shiplift::{ LogsOptions, tty::TtyChunk };
use crate::lib::global::{ GLOBAL_CONTAINER_LOGGER_LOCK, container_logger_interfaces::ContainerLog };

use super::Container;

impl Container {
  pub async fn attach_logger(self) {
    let mut log_stream = self.docker.docker
      .containers()
      .get(self.id.clone())
      .logs(&LogsOptions::builder().stdout(true).stderr(true).build());

    while let Some(log_result) = log_stream.next().await {
      match log_result {
        Ok(log) => {
          match log {
            TtyChunk::StdOut(bytes) => {
              let log = std::str::from_utf8(&bytes).unwrap().to_string();
              let log_struct= ContainerLog {
                name: self.name.clone(),
                level: String::from("info"),
                data: log,
              };
              GLOBAL_CONTAINER_LOGGER_LOCK.push(log_struct);
            },
            TtyChunk::StdErr(bytes) => {
              let log = std::str::from_utf8(&bytes).unwrap().to_string();
              let log_struct= ContainerLog {
                name: self.name.clone(),
                level: String::from("error"),
                data: log,
              };
              GLOBAL_CONTAINER_LOGGER_LOCK.push(log_struct);
            },
            TtyChunk::StdIn(_) => unreachable!(),
          }
        }
        Err(e) => {
          error!("Error: {}", e);
        }
      }
    }
  }
}