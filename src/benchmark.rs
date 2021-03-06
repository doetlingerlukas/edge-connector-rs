use std::collections::HashMap;
use std::time::Instant;

use serde::Serialize;

use crate::faasd::{FaasdHost, FunctionResponse};

pub struct BenchmarkRun<'b> {
  faasd_host: &'b FaasdHost,
  functions: HashMap<String, String>
}

#[derive(Debug, Default, Serialize)]
pub struct BenchmarkResult {
  deployment: u128,
  invocations: u128
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

  pub fn run(&'b self) -> BenchmarkResult {
    for (_, service) in self.functions.iter() {
      self.faasd_host.clear_function(&FunctionResponse::new(service));
    }

    let mut start = Instant::now();

    for (function, service) in self.functions.iter() {
      self.faasd_host.deploy(function.clone(), service.clone());
    }

    let deployment = start.elapsed();
    start = Instant::now();

    // TODO: make this more nice
    self.faasd_host.invoke(String::from("doetlingerlukas-addition"),
      String::from("{\"firstSummand\":35,\"secondSummand\":5,\"benchmark\":true}"));
    self.faasd_host.invoke(String::from("doetlingerlukas-monte-carlo-pi-function"),
      String::from("{\"samples\":100000,\"wait\":1}"));

    BenchmarkResult {
      deployment: deployment.as_nanos(),
      invocations: start.elapsed().as_nanos()
    }
  }
}
