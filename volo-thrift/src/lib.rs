#![doc(
    html_logo_url = "https://github.com/cloudwego/volo/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod error;
mod message;
mod message_wrapper;
mod protocol;
pub mod tracing;
pub mod transport;

pub mod client;
pub use client::Client;
pub mod codec;
pub mod context;
pub mod server;
pub use anyhow::Error as AnyhowError;
pub use error::*;
pub use message::{EntryMessage, Message};
pub use message_wrapper::*;
