//! Presenter Flow core: domain model, library operations, live presentation
//! state, slide layout and storage.
//!
//! This crate has no dependency on egui, eframe, winit or Windows APIs
//! (PLAN §3.2). Everything here is testable with plain `cargo test`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod domain;
pub mod error;
pub mod ids;
pub mod layout;
pub mod library;
pub mod presentation;
pub mod settings;
pub mod storage;

pub use error::CoreError;
