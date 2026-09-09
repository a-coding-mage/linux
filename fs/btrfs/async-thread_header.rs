/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 * Copyright (C) 2014 Fujitsu.  All rights reserved.
 */

// Translated from btrfs/async-thread.h. Linux workqueue and list types are
// supplied by the corresponding external Rust declarations.

use core::ffi::c_char;

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_workqueue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_work {
    pub func: btrfs_func_t,
    pub ordered_func: btrfs_ordered_func_t,

    /* Don't touch things below */
    pub normal_work: work_struct,
    pub ordered_list: list_head,
    pub wq: *mut btrfs_workqueue,
    pub flags: ::core::ffi::c_ulong,
}

pub type btrfs_func_t = Option<unsafe extern "C" fn(arg: *mut btrfs_work)>;
pub type btrfs_ordered_func_t =
    Option<unsafe extern "C" fn(arg: *mut btrfs_work, ordered: bool)>;

extern "C" {
    pub fn btrfs_alloc_workqueue(
        fs_info: *mut btrfs_fs_info,
        name: *const c_char,
        flags: u32,
        limit_active: i32,
        thresh: i32,
    ) -> *mut btrfs_workqueue;
    pub fn btrfs_alloc_ordered_workqueue(
        fs_info: *mut btrfs_fs_info,
        name: *const c_char,
        flags: u32,
    ) -> *mut btrfs_workqueue;
    pub fn btrfs_init_work(
        work: *mut btrfs_work,
        func: btrfs_func_t,
        ordered_func: btrfs_ordered_func_t,
    );
    pub fn btrfs_queue_work(wq: *mut btrfs_workqueue, work: *mut btrfs_work);
    pub fn btrfs_destroy_workqueue(wq: *mut btrfs_workqueue);
    pub fn btrfs_workqueue_set_max(wq: *mut btrfs_workqueue, max: i32);
    // __pure: declaration has no observable side effects.
    pub fn btrfs_work_owner(work: *const btrfs_work) -> *mut btrfs_fs_info;
    // __pure: declaration has no observable side effects.
    pub fn btrfs_workqueue_owner(wq: *const btrfs_workqueue) -> *mut btrfs_fs_info;
    pub fn btrfs_workqueue_normal_congested(wq: *const btrfs_workqueue) -> bool;
    pub fn btrfs_flush_workqueue(wq: *mut btrfs_workqueue);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
