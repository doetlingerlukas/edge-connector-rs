use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;

fn main() {
  let client = reqwest::blocking::Client::new();
  let socket = UdpSocket::bind("127.0.0.1:5999").expect("failed to bind socket");

  loop {
    let mut buf = [0; 30];
    let (amt, src) = socket.recv_from(&mut buf).expect("no data recieved");
    let buffer = &mut buf[..amt];

    let msg = str::from_utf8(buffer).unwrap();
    println!("{}", msg);

    if src.is_ipv6() {
      continue;
    }

    if msg.starts_with("apollo-available") {
      let mut body = HashMap::new();
      body.insert("type", "faasd");

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
