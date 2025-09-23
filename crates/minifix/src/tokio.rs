//! First-class Tokio integration for MiniFix.
//!
//! This module provides Tokio-specific helpers and utilities that allow users
//! to easily integrate FIX sessions with popular async tooling in the Tokio
//! ecosystem. It includes ready-to-use components for building FIX clients
//! and servers with minimal boilerplate.
//!
//! # Examples
//!
//! ## Simple FIX Client
//!
//! ```no_run
//! use minifix::tokio::{TokioFixClient, TokioFixConfig};
//! use minifix::Dictionary;
//! use tokio::net::TcpStream;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = TokioFixConfig::builder()
//!         .heartbeat_interval(std::time::Duration::from_secs(30))
//!         .sender_comp_id("CLIENT")
//!         .target_comp_id("SERVER")
//!         .build();
//!
//!     let stream = TcpStream::connect("127.0.0.1:9878").await?;
//!     let mut client = TokioFixClient::new(config, Dictionary::fix44());
//!
//!     client.start(stream).await?;
//!     Ok(())
//! }
//! ```

#[cfg(feature = "utils-tokio")]
pub use self::client::*;
#[cfg(feature = "utils-tokio")]
pub use self::config::*;

#[cfg(feature = "utils-tokio")]
pub use crate::session::tokio_connection::{TokioAppChannels, TokioAppHandler, TokioFixConnection};
#[cfg(feature = "utils-tokio")]
pub use crate::session::tokio_event_loop::{TokioLlEvent, TokioLlEventLoop};
#[cfg(feature = "utils-tokio")]
pub use crate::tagvalue::tokio_decoder::{TokioDecoder, TokioEncoder, TokioRawDecoder};

#[cfg(feature = "utils-tokio")]
mod client;
#[cfg(feature = "utils-tokio")]
mod config;