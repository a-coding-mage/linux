// SPDX-License-Identifier: GPL-2.0-or-later
//! Link the existing native decoder ABI into the original C host tests.

extern crate self as kernel;

#[allow(dead_code, missing_docs, non_camel_case_types, non_upper_case_globals)]
pub mod bindings {
    include!(env!("INSN_HOST_BINDINGS"));
}

#[path = "../lib/insn_rust.rs"]
mod implementation;
pub use implementation::*;
