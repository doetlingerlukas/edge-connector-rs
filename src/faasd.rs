use std::collections::HashMap;
use reqwest::blocking::Client;

use crate::FAASD_AUTH_KEY;

pub struct FaasdHost {
  host: String,
  port: u16,
  auth: String,
  client: Client
}

impl Default for FaasdHost {
  fn default() -> Self {
    Self {
      host: String::from("localhost"),
      port: 8080,
      auth: FAASD_AUTH_KEY.to_string(),
      client: Client::new()
    }
  }
}

impl FaasdHost {
  pub fn deploy(&self, function: String, service_name: String) {
    let mut map = HashMap::new();
    map.insert(String::from("service"), &service_name);
    map.insert(String::from("image"), &function);

    match self.client.post(format!("http://{}:{}/system/functions", self.host, self.port))
      .basic_auth("admin", Some(self.auth.clone()))
      .json(&map)
      .send() {
        Ok(res) => {
          if res.status().is_success() {
            println!("Function {} deployed successfully.", &function);
          } else {
            println!("Error deploying function, status is {}.", res.status());
          }
        },
        Err(_) => panic!(),
    }
  }
}
