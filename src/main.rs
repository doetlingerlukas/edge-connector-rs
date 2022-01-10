use std::net::UdpSocket;
use std::str;

use reqwest::StatusCode;

use edge_connector::*;
use edge_connector::device::Device;
use edge_connector::faasd::FaasdHost;
use edge_connector::benchmark::BenchmarkRun;

fn main() -> std::io::Result<()> {
  let mut device = Device::default();

  let faasd_host = FaasdHost::default();

  println!("Running benchmark ...");

  let benchmark = BenchmarkRun::create(&faasd_host);
  device.benchmark_result = Some(benchmark.run());

  let client = reqwest::blocking::Client::new();
  let socket = UdpSocket::bind(EDGE_CONNECTOR_UDP_SOCKET_BINDING)?;

  println!("Edge-device ready for connection!");

  loop {
    let mut buf = [0; 30];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let buf = &mut buf[..amt];
    let msg = str::from_utf8(buf).unwrap();

    if src.is_ipv6() {
      continue
    }
    let src_ip = src.ip().to_string();

    if msg.starts_with(&Message::ApolloAvailable.to_string()) {
      if let Some(instance) = Device::get_instance_binding() {
        if !instance.eq(&src_ip) {
          continue
        }
      }

      if let Ok(resp) = client.post(format!("http://{}:5888/register/", src_ip))
        .json(&device)
        .send() {
          match resp.status() {
            StatusCode::OK => {
              Device::set_instance_binding(Some(&src_ip));
              println!("Bound to instance {}", src_ip);
            },
            s => println!("Registering to {} failed. Received response status: {:?}", src_ip, s)
          }
        } else {
          println!("Registering to {} failed.", src_ip);
          continue
        }

    } else if msg.starts_with(&Message::ApolloRelease.to_string()) {
      if let Some(instance) = Device::get_instance_binding() {
        println!("Removed binding from instance {}", instance);
        Device::set_instance_binding(None)
      }
    }
  }
}
