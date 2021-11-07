use std::env;
use sysinfo::{System, SystemExt};
use serde::Serialize;

use crate::benchmark::BenchmarkResult;

const EDGE_CONNECTOR_BINDING_ENV_NAME: &'static str = "DEVICE_BOUND_TO";

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
  arch: String,
  name: String,
  num_cores: usize,
  ram_size: u64,
  key: String,
  pub benchmark_result: Option<BenchmarkResult>
}

impl Device {
  pub fn default() -> Device {
    let sys = System::new_all();

    Self {
      arch: env::consts::ARCH.to_string(),
      name: sys.host_name().unwrap_or(String::new()),
      num_cores: sys.processors().len(),
      ram_size: sys.total_memory() / 1000,
      key: crate::FAASD_AUTH_KEY.to_string(),
      benchmark_result: None
    }
  }

  pub fn get_instance_binding() -> Option<String> {
    match env::var(EDGE_CONNECTOR_BINDING_ENV_NAME) {
        Ok(instance) => Some(instance),
        Err(_) => None,
    }
  }

  pub fn set_instance_binding(instance: Option<&String>) {
    match instance {
      Some(i) => env::set_var(EDGE_CONNECTOR_BINDING_ENV_NAME, i),
      None => env::remove_var(EDGE_CONNECTOR_BINDING_ENV_NAME)
    }
  }
}
