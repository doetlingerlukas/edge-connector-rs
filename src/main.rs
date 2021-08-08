use std::net::UdpSocket;
use std::str;
use sysinfo::{System, SystemExt};

use edge_connector::*;
use edge_connector::device::Device;

fn main() -> std::io::Result<()> {
  let sys = System::new_all();
  let device = Device::new(sys);

  let client = reqwest::blocking::Client::new();
  let socket = UdpSocket::bind(EDGE_CONNECTOR_UDP_SOCKET_BINDING)?;

  loop {
    let mut buf = [0; 30];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let buf = &mut buf[..amt];

    let msg = str::from_utf8(buf).unwrap();
    println!("{}", msg);

    if src.is_ipv6() {
      continue
    }
    let src_ip = src.ip().to_string();

    if msg.starts_with("apollo-available") && Device::bound_to_instance().is_none() {
      let res = client.post(format!("{}:5888", src_ip))
        .json(&serde_json::to_string(&device).unwrap())
        .send()
        .expect("no response received");

      match res.status() {
        reqwest::StatusCode::OK => {
          Device::set_instance_binding(Some(&src_ip));
          println!("Bound to instance {}", src_ip);
        },
        s => println!("Received response status: {:?}", s)
      }

    } else if msg.starts_with("apollo-release") {
      if let Some(_) = Device::bound_to_instance() {
        Device::set_instance_binding(None)
      }
    }
  }
}
