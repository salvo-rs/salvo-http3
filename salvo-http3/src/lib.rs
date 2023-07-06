//! HTTP/3 client and server
#![deny(missing_docs, clippy::self_named_module_files)]
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod client;
mod config;
pub mod error;
pub mod ext;
pub mod quic;
pub(crate) mod request;
pub mod server;

pub use error::Error;

mod buf;

#[allow(missing_docs)]
pub mod connection;
#[allow(missing_docs)]
pub mod frame;
#[allow(missing_docs)]
pub mod proto;
#[allow(missing_docs)]
pub mod stream;
#[allow(missing_docs)]
pub mod webtransport;

#[cfg(feature = "quinn")]
pub mod http3_quinn;

#[allow(dead_code)]
mod qpack;
#[cfg(test)]
mod tests;
#[cfg(test)]
extern crate self as h3;
