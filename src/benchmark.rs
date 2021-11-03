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

    // TODO: replace testing functions
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
    self.faasd_host.clear_functions();

    let start = Instant::now();

    for (function, service) in self.functions.iter() {
      self.faasd_host.deploy(function.clone(), service.clone());
    }

    // TODO: make this more nice
    self.faasd_host.invoke(String::from("doetlingerlukas-addition"),
      String::from("{\"firstSummand\":35,\"secondSummand\":5,\"benchmark\":true}"));
    self.faasd_host.invoke(String::from("doetlingerlukas-monte-carlo-pi-function"),
      String::from("{\"samples\":100000,\"wait\":1}"));

    start.elapsed()
  }
}
