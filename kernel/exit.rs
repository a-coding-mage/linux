// SPDX-License-Identifier: GPL-2.0-only
// Process exit, reparenting and wait. Authority: unchanged kernel/exit.c.
// All original translation-unit bodies and storage are owned by Rust.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
    dead_code, missing_docs, unsafe_op_in_unsafe_fn, clippy::all,
    unused_variables, unused_mut, unreachable_pub)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/exit_generated.rs"));
}
use b::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut, read_volatile, write_volatile};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
include!("exit_layout.rs");
include!("exit_header_algorithms.rs");
include!("exit_storage.rs");
include!("exit_lifetime.rs");
include!("exit_reparent.rs");
include!("exit_task.rs");
include!("exit_wait.rs");
