/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common Block IO controller cgroup interface
 *
 * Based on ideas and code from CFQ, CFS and BFQ:
 * Copyright (C) 2003 Jens Axboe <axboe@kernel.dk>
 *
 * Copyright (C) 2008 Fabio Checconi <fabio@gandalf.sssup.it>
 *              Paolo Valente <paolo.valente@unimore.it>
 *
 * Copyright (C) 2009 Vivek Goyal <vgoyal@redhat.com>
 *                       Nauman Rafique <nauman@google.com>
 */

// The declarations below are supplied by the corresponding kernel headers.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct bio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gendisk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

pub const FC_APPID_LEN: usize = 129;

#[cfg(feature = "CONFIG_BLK_CGROUP")]
extern "C" {
    pub static blkcg_root_css: *mut cgroup_subsys_state;
    pub static mut blkcg_nr_congested: atomic_t;

    pub fn blkcg_schedule_throttle(disk: *mut gendisk, use_memdelay: bool);
    pub fn blkcg_maybe_throttle_current();
    pub fn __blk_cgroup_congested() -> bool;

    pub fn blkcg_pin_online(blkcg_css: *mut cgroup_subsys_state);
    pub fn blkcg_unpin_online(blkcg_css: *mut cgroup_subsys_state);
    pub fn blkcg_get_cgwb_list(css: *mut cgroup_subsys_state) -> *mut list_head;
    pub fn bio_blkcg_css(bio: *mut bio) -> *mut cgroup_subsys_state;
}

#[cfg(feature = "CONFIG_BLK_CGROUP")]
#[inline]
pub unsafe fn blk_cgroup_congested() -> bool {
    // Equivalent to likely(!atomic_read(&blkcg_nr_congested)); atomic access is
    // provided by the external kernel dependency.
    if atomic_read(&raw const blkcg_nr_congested) == 0 {
        return false;
    }
    __blk_cgroup_congested()
}

#[cfg(not(feature = "CONFIG_BLK_CGROUP"))]
#[inline]
pub unsafe fn blkcg_maybe_throttle_current() {}

#[cfg(not(feature = "CONFIG_BLK_CGROUP"))]
#[inline]
pub unsafe fn blk_cgroup_congested() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BLK_CGROUP"))]
#[inline]
pub unsafe fn bio_blkcg_css(_bio: *mut bio) -> *mut cgroup_subsys_state {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_BLK_CGROUP"))]
pub const blkcg_root_css: *mut cgroup_subsys_state = (-22isize) as *mut cgroup_subsys_state;

extern "C" {
    pub fn blkcg_set_fc_appid(
        app_id: *mut c_char,
        cgrp_id: u64,
        app_id_len: usize,
    ) -> i32;
    pub fn blkcg_get_fc_appid(bio: *mut bio) -> *mut c_char;
}

// External atomic operation supplied by linux/atomic.h.
extern "C" {
    fn atomic_read(v: *const atomic_t) -> i32;
}

// c_void is retained as the C header's opaque dependency vocabulary.
#[allow(dead_code)]
type _BlkCgroupOpaque = c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
