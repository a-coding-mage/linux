// SPDX-License-Identifier: GPL-2.0
// Rust owner of mm/rmap.c at e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Original C remains the immutable reference. Layouts and native constants are
// supplied by the configured kernel headers; no synthetic C ABI is defined here.
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/rmap_native_generated.rs"
    ));
}
use b::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
type Pgoff = rust_rmap_pgoff_t;

// These are source-local arithmetic constants, not replicas of native ABI.
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
const TLB_FLUSH_BATCH_FLUSHED_SHIFT: c_int = 16;
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
const TLB_FLUSH_BATCH_PENDING_MASK: c_int = (1 << (TLB_FLUSH_BATCH_FLUSHED_SHIFT - 1)) - 1;
#[cfg(CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH)]
const TLB_FLUSH_BATCH_PENDING_LARGE: c_int = TLB_FLUSH_BATCH_PENDING_MASK / 2;

static mut ANON_VMA_CACHEP: *mut kmem_cache = null_mut();
static mut ANON_VMA_CHAIN_CACHEP: *mut kmem_cache = null_mut();

include!("rmap_anon.rs");
include!("rmap_walk.rs");
include!("rmap_account.rs");
include!("rmap_unmap.rs");
include!("rmap_migrate.rs");

// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
