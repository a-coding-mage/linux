// SPDX-License-Identifier: GPL-2.0
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// Bodies reconciled with authoritative mm/slub.c at
// e1d84f501551943a11f4c5271e9f5c85d7e15168 during this source-only phase.
//! SLUB Rust owner; configured native headers retain all shared ABI authority.
#![cfg_attr(CONFIG_KMSAN, feature(no_sanitize))]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/slub_generated.rs"));
}
use bindings::*;
type rcu_head = callback_head;
use core::cmp::{max, min};
use core::mem::{align_of, offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char as CChar, c_long as Long, c_ulong as ULong, c_void as Void};
const _: () = {
    assert!(size_of::<slab>() == RSL_SIZEOF_SLAB as usize);
    assert!(align_of::<slab>() == RSL_ALIGNOF_SLAB as usize);
    assert!(size_of::<slab_sheaf>() == RSL_SIZEOF_SHEAF as usize);
    assert!(size_of::<track>() == RSL_SIZEOF_TRACK as usize);
};
include!("slub_foundation.rs");
include!("slub_debug.rs");
include!("slub_debug_tail.rs");
include!("slub_hooks.rs");
include!("slub_sheaves.rs");
include!("slub_slabs.rs");
include!("slub_allocation.rs");
