/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025 Tejun Heo <tj@kernel.org>
 */

use core::ffi::c_void;

#[repr(C)]
pub struct scx_sched {
    _private: [u8; 0],
}

extern "C" {
    pub fn scx_arena_pool_init(sch: *mut scx_sched) -> i32;
    pub fn scx_arena_pool_destroy(sch: *mut scx_sched);
    pub fn scx_arena_alloc(sch: *mut scx_sched, size: usize) -> *mut c_void;
    pub fn scx_arena_free(sch: *mut scx_sched, kern_va: *mut c_void, size: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
