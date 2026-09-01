/*
 * SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2025 Emil Tsalapatis <etsal@meta.com>
 */

/* C header note: outside __BPF__, __arena is defined away. */

#[repr(C)]
pub struct scx_alloc_stats {
    pub chunk_allocs: u64,
    pub data_allocs: u64,
    pub alloc_ops: u64,
    pub free_ops: u64,
    pub active_allocs: u64,
    pub arena_pages_used: u64,
}

#[repr(C)]
pub struct sdt_pool {
    pub slab: *mut core::ffi::c_void,
    pub elem_size: u64,
    pub max_elems: u64,
    pub idx: u64,
}

/* C macros:
 * #ifndef div_round_up
 * #define div_round_up(a, b) (((a) + (b) - 1) / (b))
 * #endif
 *
 * #ifndef round_up
 * #define round_up(a, b) (div_round_up((a), (b)) * (b))
 * #endif
 */
pub const fn div_round_up(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

pub const fn round_up(a: u64, b: u64) -> u64 {
    div_round_up(a, b) * b
}

pub type sdt_desc_t = sdt_desc;

pub const SDT_TASK_ENTS_PER_PAGE_SHIFT: u64 = 9;
pub const SDT_TASK_LEVELS: u64 = 3;
pub const SDT_TASK_ENTS_PER_CHUNK: u64 = 1 << SDT_TASK_ENTS_PER_PAGE_SHIFT;
pub const SDT_TASK_CHUNK_BITMAP_U64S: u64 = div_round_up(SDT_TASK_ENTS_PER_CHUNK, 64);
pub const SDT_TASK_MIN_ELEM_PER_ALLOC: u64 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdt_id_fields {
    pub idx: i32,  /* index in the radix tree */
    pub genn: i32, /* ++'d on recycle so that it forms unique'ish 64bit ID */
}

#[repr(C)]
pub union sdt_id {
    pub val: i64,
    pub fields: sdt_id_fields,
}

/*
 * Each index page is described by the following descriptor which carries the
 * bitmap. This way the actual index can host power-of-two numbers of entries
 * which makes indexing cheaper.
 */
#[repr(C)]
pub struct sdt_desc {
    pub allocated: [u64; SDT_TASK_CHUNK_BITMAP_U64S as usize],
    pub nr_free: u64,
    pub chunk: *mut sdt_chunk,
}

/*
 * Leaf node containing per-task data.
 */
#[repr(C)]
pub struct sdt_data {
    pub tid: sdt_id,
    pub payload: [u64; 0],
}

/*
 * Intermediate node pointing to another intermediate node or leaf node.
 */
#[repr(C)]
pub union sdt_chunk {
    pub descs: [*mut sdt_desc_t; SDT_TASK_ENTS_PER_CHUNK as usize],
    pub data: [*mut sdt_data; SDT_TASK_ENTS_PER_CHUNK as usize],
}

#[repr(C)]
pub struct scx_allocator {
    pub pool: sdt_pool,
    pub root: *mut sdt_desc_t,
}

#[repr(C)]
pub struct scx_stats {
    pub seq: core::ffi::c_int,
    pub pid: pid_t,
    pub enqueue: u64,
    pub exit: u64,
    pub init: u64,
    pub select_busy_cpu: u64,
    pub select_idle_cpu: u64,
}

/* C: declarations are present only under #ifdef __BPF__. */
unsafe extern "C" {
    pub fn scx_task_data(p: *mut task_struct) -> *mut core::ffi::c_void;
    pub fn scx_task_init(data_size: u64) -> core::ffi::c_int;
    pub fn scx_task_alloc(p: *mut task_struct) -> *mut core::ffi::c_void;
    pub fn scx_task_free(p: *mut task_struct);
    pub fn scx_arena_subprog_init();

    pub fn scx_alloc_init(alloc: *mut scx_allocator, data_size: u64) -> core::ffi::c_int;
    pub fn scx_alloc_internal(alloc: *mut scx_allocator) -> u64;
    pub fn scx_alloc_free_idx(alloc: *mut scx_allocator, idx: u64) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
