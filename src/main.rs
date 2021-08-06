use std::net::UdpSocket;
use std::str;
use std::collections::HashMap;
use sysinfo::{System, SystemExt};

use edge_connector::*;

fn main() -> std::io::Result<()> {
  let sys = System::new_all();
  let device = device::Device::new(sys);

  let client = reqwest::blocking::Client::new();
  let socket = UdpSocket::bind(EDGE_CONNECTOR_UDP_SOCKET_BINDING)?;

  loop {
    let mut buf = [0; 30];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let buf = &mut buf[..amt];

    let msg = str::from_utf8(buf).unwrap();
    println!("{}", msg);

    if src.is_ipv6() {
      continue;
    }

    if msg.starts_with("apollo-available") && device::Device::bound_to_instance().is_some() {
      let mut body = HashMap::new();
      body.insert("type", "faasd");
      body.insert("arch", &device.arch);

      let res = client.post(format!("{}:5888", src.ip().to_string()))
        .json(&body)
        .send()
        .expect("no response received");

      match res.status() {
        reqwest::StatusCode::OK => println!("Ok!"),
        s => println!("Received response status: {:?}", s)
      }

    } else if msg.starts_with("apollo-release") {

    }
  }
}
