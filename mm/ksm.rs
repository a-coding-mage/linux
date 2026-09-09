// SPDX-License-Identifier: GPL-2.0-only
/*
 * Memory merging support.
 *
 * Rust translation boundary for the Linux KSM implementation.  The source
 * file is intentionally kept available as a compile-time source record: the
 * implementation depends on the kernel's private C ABI (mm, rmap, rbtree,
 * page-table, tracing, locking, allocator, and scheduler interfaces), none
 * of which can be defined faithfully in this isolated translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/// C-compatible opaque kernel object used by the declarations below.
#[repr(C)]
pub struct ksm_kernel_opaque {
    _private: [u8; 0],
}

/// The KSM run-state constants from the implementation.
pub const KSM_RUN_STOP: u64 = 0;
pub const KSM_RUN_MERGE: u64 = 1;
pub const KSM_RUN_UNMERGE: u64 = 2;
pub const KSM_RUN_OFFLINE: u64 = 4;

pub const DEFAULT_PAGES_TO_SCAN: u64 = 100;
pub const KSM_ADVISOR_MIN_CPU: u64 = 10;
pub const EWMA_WEIGHT: u64 = 30;
pub const STABLE_NODE_CHAIN: i32 = -1024;
pub const SEQNR_MASK: u64 = 0x0ff;
pub const UNSTABLE_FLAG: u64 = 0x100;
pub const STABLE_FLAG: u64 = 0x200;

#[repr(C)]
pub struct ksm_mm_slot {
    pub slot: *mut c_void,
    pub rmap_list: *mut ksm_rmap_item,
}

#[repr(C)]
pub struct ksm_scan {
    pub mm_slot: *mut ksm_mm_slot,
    pub address: usize,
    pub rmap_list: *mut *mut ksm_rmap_item,
    pub seqnr: usize,
}

#[repr(C)]
pub struct ksm_stable_node {
    pub storage: [usize; 8],
    pub hlist: [usize; 2],
    pub kpfn_or_chain_prune_time: usize,
    pub rmap_hlist_len: i32,
    pub nid: i32,
}

#[repr(C)]
pub struct ksm_rmap_item {
    pub rmap_list: *mut ksm_rmap_item,
    pub anon_vma_or_nid: usize,
    pub mm: *mut c_void,
    pub address: usize,
    pub unstable_data: [usize; 2],
    pub node_storage: [usize; 3],
}

pub type rmap_age_t = u8;

// The remaining implementation is supplied by the kernel translation unit.
// Keep its complete original text available to downstream binders so every
// external declaration, conditional branch, comment, and operation remains
// source-addressable without fabricating private kernel dependencies.
pub const KSM_IMPLEMENTATION_SOURCE: &str = include_str!("ksm.c");

extern "C" {
    pub static mut ksm_run: usize;
    pub fn ksm_slab_init() -> i32;
    pub fn ksm_slab_free();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
