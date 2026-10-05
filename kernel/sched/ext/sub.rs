// SPDX-License-Identifier: GPL-2.0
// Continuation of the existing sub.rs against 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native headers own every layout. Native leaves are an explicit, unqualified
// C runtime boundary; no old sub.c algorithm is used as an implementation.
#![no_std]

compile_error!("SOURCE ONLY HOLD: sched_ext sub-scheduler is not admitted");

use kernel::bindings::sched_ext_sub_native::*;
use kernel::ffi::{c_char, c_int, c_ulong, c_void};
#[cfg(CONFIG_EXT_SUB_SCHED)]
use core::ptr;

#[cfg(CONFIG_EXT_SUB_SCHED)]
include!("sched_ext_sub_tree.rs");
#[cfg(CONFIG_EXT_SUB_SCHED)]
include!("sched_ext_sub_rescue.rs");
#[cfg(CONFIG_EXT_SUB_SCHED)]
include!("sched_ext_sub_ecaps.rs");
#[cfg(CONFIG_EXT_SUB_SCHED)]
include!("sched_ext_sub_caps.rs");
#[cfg(CONFIG_EXT_SUB_SCHED)]
include!("sched_ext_sub_lifecycle.rs");
#[cfg(not(CONFIG_EXT_SUB_SCHED))]
include!("sched_ext_sub_disabled.rs");

/// Disabled-config fallback retained from the original Rust owner.
///
/// # Safety
/// No pointer is read or written in this configuration.
#[cfg(not(CONFIG_EXT_SUB_SCHED))]
pub unsafe fn scx_bpf_sub_grant(_: u64, _: u64, _: *const scx_cmask,
                                _: *mut scx_cmask, _: *const bpf_prog_aux) -> i32 { -95 }
/// Disabled-config fallback retained from the original Rust owner.
///
/// # Safety
/// No pointer is read or written in this configuration.
#[cfg(not(CONFIG_EXT_SUB_SCHED))]
pub unsafe fn scx_bpf_sub_revoke(_: u64, _: u64, _: *const scx_cmask, _: *const bpf_prog_aux) {}
/// Disabled-config fallback retained from the original Rust owner.
///
/// # Safety
/// No pointer is read or written in this configuration.
#[cfg(not(CONFIG_EXT_SUB_SCHED))]
pub unsafe fn scx_bpf_sub_caps(_: u64, _: u64, _: *mut scx_cmask, _: *const bpf_prog_aux) -> i32 { -95 }
/// Disabled-config fallback retained from the original Rust owner.
///
/// # Safety
/// No pointer is read or written in this configuration.
#[cfg(not(CONFIG_EXT_SUB_SCHED))]
pub unsafe fn scx_bpf_sub_kill_bstr(_: u64, _: *mut c_char, _: *mut u64, _: u32, _: *const bpf_prog_aux) -> i32 { -95 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
