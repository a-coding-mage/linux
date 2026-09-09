/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2021 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel/Rust environment:
// linux/bitops.h, linux/list.h, linux/slab.h, linux/sched.h,
// linux/rbtree.h, and linux/rbtree_augmented.h.

pub const GPU_BUDDY_RANGE_ALLOCATION: u64 = 1 << 0;
pub const GPU_BUDDY_TOPDOWN_ALLOCATION: u64 = 1 << 1;
pub const GPU_BUDDY_CONTIGUOUS_ALLOCATION: u64 = 1 << 2;
pub const GPU_BUDDY_CLEAR_ALLOCATION: u64 = 1 << 3;
pub const GPU_BUDDY_CLEARED: u64 = 1 << 4;
pub const GPU_BUDDY_TRIM_DISABLE: u64 = 1 << 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gpu_buddy_free_tree {
    GPU_BUDDY_CLEAR_TREE = 0,
    GPU_BUDDY_DIRTY_TREE,
    GPU_BUDDY_MAX_FREE_TREES,
}

// for_each_free_tree(tree): for tree = 0; tree < GPU_BUDDY_MAX_FREE_TREES; tree++

pub const GPU_BUDDY_HEADER_OFFSET: u64 = (!0u64) & !((1u64 << 12) - 1);
pub const GPU_BUDDY_HEADER_STATE: u64 = ((1u64 << 2) - 1) << 10;
pub const GPU_BUDDY_ALLOCATED: u64 = 1 << 10;
pub const GPU_BUDDY_FREE: u64 = 2 << 10;
pub const GPU_BUDDY_SPLIT: u64 = 3 << 10;
pub const GPU_BUDDY_HEADER_CLEAR: u64 = 1 << 9;
pub const GPU_BUDDY_HEADER_UNUSED: u64 = 0b111 << 6;
pub const GPU_BUDDY_HEADER_ORDER: u64 = (1 << 6) - 1;

// The surrounding environment supplies these C-layout kernel types.
#[repr(C)]
pub struct gpu_buddy_block {
    pub header: u64,
    pub left: *mut gpu_buddy_block,
    pub right: *mut gpu_buddy_block,
    pub parent: *mut gpu_buddy_block,
    pub private: *mut core::ffi::c_void,
    pub rb_or_link: gpu_buddy_block_rb_or_link,
    pub tmp_link: list_head,
    pub subtree_max_alignment: core::ffi::c_uint,
}

#[repr(C)]
pub union gpu_buddy_block_rb_or_link {
    pub rb: rb_node,
    pub link: list_head,
}

pub const GPU_BUDDY_MAX_ORDER: u32 = 63 - 12;

#[repr(C)]
pub struct gpu_buddy {
    pub free_trees: *mut *mut rb_root,
    pub roots: *mut *mut gpu_buddy_block,
    pub free_scoreboard: *mut u64,
    pub used_scoreboard: *mut u64,
    pub n_roots: core::ffi::c_uint,
    pub max_order: core::ffi::c_uint,
    pub chunk_size: u64,
    pub size: u64,
    pub avail: u64,
    pub clear_avail: u64,
    #[cfg(CONFIG_LOCKDEP)]
    pub lock_dep_map: *mut lockdep_map,
}

#[cfg(CONFIG_LOCKDEP)]
pub unsafe fn gpu_buddy_driver_set_lock(mm: *mut gpu_buddy, lock: *mut impl HasDepMap) {
    // C macro: warn if the lock dependency map is set more than once.
    let mm = &mut *mm;
    if !mm.lock_dep_map.is_null() {
        // WARN(__mm->lock_dep_map, "GPU BUDDY MM lock should be set only once.")
    } else {
        mm.lock_dep_map = (*lock).dep_map();
    }
}

#[cfg(not(CONFIG_LOCKDEP))]
pub unsafe fn gpu_buddy_driver_set_lock<T>(_mm: *mut gpu_buddy, _lock: *mut T) {}

#[cfg(CONFIG_LOCKDEP)]
pub unsafe fn gpu_buddy_driver_lock_held(mm: *mut gpu_buddy) {
    if !(*mm).lock_dep_map.is_null() {
        // lockdep_assert(lock_is_held_type((*mm).lock_dep_map, 0));
    }
}

#[cfg(not(CONFIG_LOCKDEP))]
pub unsafe fn gpu_buddy_driver_lock_held(_mm: *mut gpu_buddy) {}

pub unsafe fn gpu_buddy_block_offset(block: *const gpu_buddy_block) -> u64 {
    (*block).header & GPU_BUDDY_HEADER_OFFSET
}

pub unsafe fn gpu_buddy_block_order(block: *mut gpu_buddy_block) -> core::ffi::c_uint {
    ((*block).header & GPU_BUDDY_HEADER_ORDER) as core::ffi::c_uint
}

pub unsafe fn gpu_buddy_block_is_free(block: *mut gpu_buddy_block) -> bool {
    ((*block).header & GPU_BUDDY_HEADER_STATE) == GPU_BUDDY_FREE
}

pub unsafe fn gpu_buddy_block_is_clear(block: *mut gpu_buddy_block) -> bool {
    ((*block).header & GPU_BUDDY_HEADER_CLEAR) != 0
}

pub unsafe fn gpu_buddy_block_size(mm: *mut gpu_buddy,
                                   block: *mut gpu_buddy_block) -> u64 {
    (*mm).chunk_size << gpu_buddy_block_order(block)
}

unsafe extern "C" {
    pub fn gpu_buddy_init(mm: *mut gpu_buddy, size: u64, chunk_size: u64) -> core::ffi::c_int;
    pub fn gpu_buddy_fini(mm: *mut gpu_buddy);
    pub fn gpu_buddy_alloc_blocks(mm: *mut gpu_buddy, start: u64, end: u64, size: u64,
                                  min_page_size: u64, blocks: *mut list_head,
                                  flags: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn gpu_buddy_block_trim(mm: *mut gpu_buddy, start: *mut u64, new_size: u64,
                                blocks: *mut list_head) -> core::ffi::c_int;
    pub fn gpu_buddy_reset_clear(mm: *mut gpu_buddy, is_clear: bool);
    pub fn gpu_buddy_free_block(mm: *mut gpu_buddy, block: *mut gpu_buddy_block);
    pub fn gpu_buddy_allocated_addr_to_block(mm: *mut gpu_buddy, addr: u64)
        -> *mut gpu_buddy_block;
    pub fn gpu_buddy_free_list(mm: *mut gpu_buddy, objects: *mut list_head,
                               flags: core::ffi::c_uint);
    pub fn gpu_buddy_print(mm: *mut gpu_buddy);
    pub fn gpu_buddy_block_print(mm: *mut gpu_buddy, block: *mut gpu_buddy_block);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
