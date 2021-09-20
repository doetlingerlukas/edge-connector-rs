use std::str;

pub mod device;

pub static FAASD_AUTH_KEY: &'static str = include_str!("../faas-key");

pub const EDGE_CONNECTOR_UDP_SOCKET_BINDING: &'static str = "0.0.0.0:5999";

#[derive(strum_macros::Display)]
pub enum Message {
  ApolloAvailable,
  ApolloRelease
}
