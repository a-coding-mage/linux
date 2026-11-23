// SPDX-License-Identifier: GPL-2.0-or-later
//! Native kernel crate for the generic timer queue.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `timerqueue.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "timerqueue.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(timerqueue_add, timerqueue_add, "GPL", "");
ffi_export::export_symbol!(timerqueue_del, timerqueue_del, "GPL", "");
ffi_export::export_symbol!(timerqueue_iterate_next, timerqueue_iterate_next, "GPL", "");
ffi_export::export_symbol!(timerqueue_linked_add, timerqueue_linked_add, "GPL", "");
