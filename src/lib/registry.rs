pub mod request_interfaces;
pub mod image;

use serde_json;
use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::{Body, Method, Request};
use crate::lib::global::GLOBAL_PARSED_CONFIG_LOCK;

#[allow(unused_imports)]
use image::*;

#[derive(Default, Debug, Clone)]
pub struct Registry {
  registry_url: String,
  username: Option<String>,
  password: Option<String>,
  is_authenticated: bool,
  pub token: Option<String>,
}

impl Registry {
  pub fn new() -> Registry {
    let config = GLOBAL_PARSED_CONFIG_LOCK.get();
    Registry {
      registry_url: format!("https://{}", config.repository.registry_url.clone()),
      username: config.repository.registry_username.clone(),
      password: config.repository.registry_password.clone(),
      is_authenticated:false,
      token: None,
    }
  }

  pub async fn login(&mut self) {
    let url = format!("{}/v2/users/login", self.registry_url);

    debug!("LOGIN PROCESSSSSSS");
    // debug!("{}", url);

    let username = (self.username.clone().to_owned()).unwrap_or(String::from("anonymous"));
    let password= (self.password.clone().to_owned()).unwrap_or(String::from("anonymous"));

    let request_body = request_interfaces::RequestLogin {
      username,
      password,
    };

    let request_body_string = serde_json::to_string(&request_body).unwrap();
    
    let https = HttpsConnector::new();
    let client = Client::builder()
      .build::<_, hyper::Body>(https);
    
    let req = Request::builder()
      .method(Method::POST)
      .uri(url)
      .header("content-type", "application/json")
      .body(Body::from(request_body_string)).unwrap();
    let req = client.request(req).await.unwrap();
    
    let res_body =  hyper::body::to_bytes(req.into_body()).await.unwrap();
    let res_string = String::from_utf8(res_body.to_vec()).unwrap();
    let result: serde_json::Value = serde_json::from_str(&res_string).unwrap();
    if result.get("detail").is_some() {
      error!("Login To Registry Failed.");
      panic!("Please Check Your config");
    }
    let token = result.get("token").unwrap().as_str().unwrap().to_string();
    info!("Success LoggedIn to Registry");
    (*self).token = Some(token);
  }

}