use std::collections::HashMap;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

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
        Ok(resp) => {
          if resp.status().is_success() {
            println!("Function {} deployed successfully.", &function);
          } else {
            println!("Error deploying function, status is {}.", resp.status());
          }
        },
        Err(_) => panic!()
    }
  }

  pub fn invoke(&self, service_name: String, input: String) {
    match self.client.post(format!("http://{}:{}/function/{}", self.host, self.port, &service_name))
      .body(input)
      .send() {
        Ok(resp) => {
          if resp.status().is_success() {
            println!("Function {} returned: {}", &service_name, resp.text().unwrap_or(String::new()));
          } else {
            println!("Error invoking function, status is {}.", resp.status());
          }
        },
        Err(_) => panic!()
      }
  }

  pub fn clear_functions(&self) {
    let functions = match self.client.get(format!("http://{}:{}/system/functions", self.host, self.port))
      .basic_auth("admin", Some(self.auth.clone()))
      .send() {
        Ok(resp) => {
          if resp.status().is_success() {
            let json: Vec<FunctionResponse> = resp.json().expect("Couldn't get response as JSON.");
            json
          } else {
            panic!("Error getting deployed functions.");
          }
        },
        Err(_) => panic!()
      };

    for f in functions.iter() {
      self.clear_function(f);
    }
  }

  pub fn clear_function(&self, f: &FunctionResponse) {
    match self.client.delete(format!("http://{}:{}/system/functions", self.host, self.port))
      .basic_auth("admin", Some(self.auth.clone()))
      .json(&f)
      .send() {
        Ok(resp) => {
          println!("Trying to delete function {} returned status {} ", &f.function_name, resp.status());
        },
        Err(_) => panic!()
      }
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponse {
  #[serde(alias = "name")]
  function_name: String
}

impl FunctionResponse {
  pub fn new(name: &str) -> Self {
    Self {
      function_name: name.to_string()
    }
  }
}
