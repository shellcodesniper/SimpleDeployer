use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLogin {
  pub username: String,
  pub password: String,
}
