// SPDX-License-Identifier: GPL-2.0-only
// Process creation and task/mm lifetime. Authority: original kernel/fork.c.
// C contains only ABI/compiler/locking/subsystem primitives and metadata.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/fork_generated.rs"));
}
use b::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{
    addr_of, addr_of_mut, copy_nonoverlapping, null_mut, read_volatile, write_volatile,
};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
type rcu_head = callback_head;
include!("fork_namespace_layout.rs");
include!("fork_storage.rs");
include!("fork_header_algorithms.rs");
include!("fork_task.rs");
include!("fork_mm.rs");
include!("fork_process.rs");
include!("fork_clone.rs");
include!("fork_unshare.rs");
