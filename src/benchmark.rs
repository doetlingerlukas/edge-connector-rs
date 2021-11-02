use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::faasd::FaasdHost;

pub struct BenchmarkRun<'b> {
  faasd_host: &'b FaasdHost,
  functions: HashMap<String, String>
}

impl<'b> BenchmarkRun<'b> {
  pub fn create(faasd_host: &'b FaasdHost) -> Self {
    let mut functions = HashMap::new();
    functions.insert(String::from("doetlingerlukas/addition"),
    String::from("doetlingerlukas-addition"));
    functions.insert(String::from("doetlingerlukas/monte-carlo-pi-function"),
    String::from("doetlingerlukas-monte-carlo-pi-function"));

    Self {
      faasd_host,
      functions
    }
  }

  pub fn run(&'b self) -> Duration {
    let start = Instant::now();

    for (function, service) in self.functions.iter() {
      self.faasd_host.deploy(function.clone(), service.clone());
    }

    start.elapsed()
  }
}
