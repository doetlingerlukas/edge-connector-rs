use std::net::UdpSocket;
use std::str;

use edge_connector::*;
use edge_connector::device::Device;

fn main() -> std::io::Result<()> {
  let device = Device::default();

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

    if msg.starts_with(&Message::ApolloAvailable.to_string()) {
      if let Some(instance) = Device::get_instance_binding() {
        if !instance.eq(&src_ip) {
          continue
        }
      }

      let res = client.post(format!("http://{}:5888/register/", src_ip))
        .json(&device)
        .send()
        .expect("no response received");

      match res.status() {
        reqwest::StatusCode::OK => {
          Device::set_instance_binding(Some(&src_ip));
          println!("Bound to instance {}", src_ip);
        },
        s => println!("Received response status: {:?}", s)
      }

    } else if msg.starts_with(&Message::ApolloRelease.to_string()) {
      if let Some(_) = Device::get_instance_binding() {
        Device::set_instance_binding(None)
      }
    }
  }
}
