use std::net::UdpSocket;
use std::str;

fn main() {
  let socket = UdpSocket::bind("127.0.0.1:5999").expect("failed to bind socket");

  loop {
    let mut buf = [0; 10];
    let (amt, ..) = socket.recv_from(&mut buf).expect("no data recieved");
    let buffer = &mut buf[..amt];

    let msg = str::from_utf8(buffer).unwrap();
    println!("{}", msg);
  }
}
