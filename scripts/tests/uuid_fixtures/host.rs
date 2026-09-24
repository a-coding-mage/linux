// SPDX-License-Identifier: GPL-2.0-only
//! Secondary ordinary ABI test crate, using the unmodified provider.
#![no_std]
#[path = "../lib/uuid.rs"]
mod implementation;
pub use implementation::*;
#[path = "panic.rs"]
mod panic;

#[path = "header.rs"]
pub mod header;
