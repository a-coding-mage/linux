// SPDX-License-Identifier: GPL-2.0-only
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Reconciled source authority: e1d84f501551943a11f4c5271e9f5c85d7e15168.
// SOURCE-PHASE ONLY: see sched_core_rust.mk and review/STATUS.md.
// All C ABI types/constants come from the configured native scheduler headers.
#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
use core::mem::{offset_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::bindings::sched_core_native::*;
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
include!("core_native_arch.rs");
include!("core_foundation.rs");
// Composed tail owns C5532..EOF; one root/native authority, no C fallback.
include!("core_tail.rs");
