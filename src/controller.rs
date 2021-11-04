use super::lib::{ config, docker, registry, utils, logger, global};
pub mod container;


// let test_thread = std::thread::spawn(move || {
//   let connect_result = tokio::runtime::Builder::new_multi_thread()
//     .enable_all()
//     .build()
//     .unwrap()
//     .block_on(async {
//       let test_result = x_test.test_connection().await;
//       test_result
//     });

//   if connect_result {
//     info!("Connection result : {}", connect_result);
//   } else {
//     error!("Connection result : {}", connect_result);
//     error!("Please Check Repository Settings in config");
//   }

// });
pub async fn entrypoint(main_docker: docker::Docker, registry: registry::Registry) {
  global::GLOBAL_DOCKER_LOCK.set(main_docker);
  global::GLOBAL_REGISTRY_LOCK.set(registry);
  info!("Enter Program EntryPoint");

  container::controller_check_outdated().await;

}