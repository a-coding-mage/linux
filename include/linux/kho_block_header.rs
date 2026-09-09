/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2026, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/list.h, linux/types.h, and linux/kho/abi/block.h.

#[repr(C)]
pub struct kho_block {
    pub list: list_head,
    pub ser: *mut kho_block_header_ser,
}

#[repr(C)]
pub struct kho_block_set {
    pub blocks: list_head,
    pub nblocks: isize,
    pub head_pa: u64,
    pub entry_size: usize,
    pub count_per_block: u64,
    pub incoming: bool,
}

#[repr(C)]
pub struct kho_block_set_it {
    pub bs: *mut kho_block_set,
    pub block: *mut kho_block,
    pub i: u64,
}

// KHO_BLOCK_SET_INIT - Initialize a static kho_block_set.
#[macro_export]
macro_rules! KHO_BLOCK_SET_INIT {
    ($name:expr, $entry_size:expr) => {
        kho_block_set {
            blocks: LIST_HEAD_INIT!($name.blocks),
            nblocks: 0,
            head_pa: 0,
            entry_size: $entry_size,
            count_per_block: (KHO_BLOCK_SIZE - core::mem::size_of::<kho_block_header_ser>())
                / ($entry_size),
            incoming: false,
        }
    };
}

extern "C" {
    pub fn kho_block_set_init(bs: *mut kho_block_set, entry_size: usize);

    pub fn kho_block_set_grow(bs: *mut kho_block_set, count: u64) -> i32;
    pub fn kho_block_set_shrink(bs: *mut kho_block_set, count: u64);

    pub fn kho_block_set_restore(bs: *mut kho_block_set, head_pa: u64) -> i32;
    pub fn kho_block_set_destroy(bs: *mut kho_block_set);
    pub fn kho_block_set_clear(bs: *mut kho_block_set);

    pub fn kho_block_set_it_init(it: *mut kho_block_set_it, bs: *mut kho_block_set);
    pub fn kho_block_set_it_reserve_entry(it: *mut kho_block_set_it) -> *mut core::ffi::c_void;
    pub fn kho_block_set_it_read_entry(it: *mut kho_block_set_it) -> *mut core::ffi::c_void;
    pub fn kho_block_set_it_prev(it: *mut kho_block_set_it) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn kho_block_set_head_pa(bs: *mut kho_block_set) -> u64 {
    (*bs).head_pa
}

#[inline]
pub unsafe fn kho_block_set_is_empty(bs: *mut kho_block_set) -> bool {
    list_empty(&(*bs).blocks)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
