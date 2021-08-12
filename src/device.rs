use std::env;
use sysinfo::{System, SystemExt};
use serde::Serialize;

const EDGE_CONNECTOR_BINDING_ENV_NAME: &'static str = "DEVICE_BOUND_TO";

#[derive(Debug, Default, Serialize)]
pub struct Device {
  pub arch: String,
  pub num_cores: usize,
  pub ram_size: u64
}

impl Device {
  pub fn default() -> Device {
    let sys = System::new_all();

    Self {
      arch: env::consts::ARCH.to_string(),
      num_cores: sys.processors().len(),
      ram_size: sys.total_memory() / 1000
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
