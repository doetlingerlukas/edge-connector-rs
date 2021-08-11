use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod device;

pub const EDGE_CONNECTOR_UDP_SOCKET_BINDING: &'static str = "127.0.0.1:5999";
pub const FAASD_AUTH_KEY_FILE: &'static str = "/var/lib/faasd/secrets/basic-auth-password";

pub fn get_faasd_auth_key() -> std::io::Result<String> {
  let mut key = String::default();

  let file = File::open(FAASD_AUTH_KEY_FILE)?;
  let mut reader = BufReader::new(file);
  reader.read_line(&mut key)?;

  Ok(key)
}
