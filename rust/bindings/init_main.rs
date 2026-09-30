// SPDX-License-Identifier: GPL-2.0-only
//! Canonical built-in boot interfaces for the staged Rust init/main owner.
//!
//! Ordinary kernel bindings are generated with MODULE. This separate crate
//! retains actual built-in setup/constructor/initcall records from init_main.h.

#![no_std]
#![allow(
    clippy::all,
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unsafe_op_in_unsafe_fn
)]

include!(concat!(env!("OBJTREE"), "/rust/bindings/init_main_generated.rs"));

// These primitives already have canonical ABI helpers in the kernel. Reuse
// them; do not declare C macros or inline-only functions as external symbols.
pub use bindings::{BUG, IS_ERR};
