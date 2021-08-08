use std::env;
use sysinfo::{System, SystemExt};
use serde::Serialize;

pub const EDGE_CONNECTOR_BINDING_ENV_NAME: &'static str = "DEVICE_BOUND_TO";

#[derive(Debug, Serialize)]
pub struct Device {
  pub arch: String,
  pub num_cores: usize,
  pub ram_size: u64
}

impl Device {
  pub fn new(sys: System) -> Device {
    Self {
      arch: env::consts::ARCH.to_string(),
      num_cores: sys.processors().len(),
      ram_size: sys.total_memory() / 1000
    }
  }

  pub fn bound_to_instance() -> Option<String> {
    match env::var(EDGE_CONNECTOR_BINDING_ENV_NAME) {
        Ok(host) => Some(host),
        Err(_) => None,
    }
  }
}