#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod model;

pub const API_VERSION: u8 = 1;
pub const SERVICE_ENDPOINT: &str = "https://classroom.googleapis.com";

pub struct Client {}
