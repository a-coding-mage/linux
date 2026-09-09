/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * heartbeat.h
 *
 * Function prototypes
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Dependency supplied by ocfs2_heartbeat.h.

pub const O2HB_REGION_TIMEOUT_MS: u32 = 2000;
pub const O2HB_MAX_REGION_NAME_LEN: usize = 32;

/* number of changes to be seen as live */
pub const O2HB_LIVE_THRESHOLD: u32 = 2;
/* number of equal samples to be seen as dead */
unsafe extern "C" {
    pub static mut o2hb_dead_threshold: c_uint;
}
pub const O2HB_DEFAULT_DEAD_THRESHOLD: u32 = 31;
/* Otherwise MAX_WRITE_TIMEOUT will be zero... */
pub const O2HB_MIN_DEAD_THRESHOLD: u32 = 2;

/* Equivalent to O2HB_MAX_WRITE_TIMEOUT_MS; its value depends on the external global. */
#[inline]
pub unsafe fn O2HB_MAX_WRITE_TIMEOUT_MS() -> c_uint {
    O2HB_REGION_TIMEOUT_MS * (o2hb_dead_threshold - 1)
}

pub const O2HB_CB_MAGIC: u32 = 0x51d1e4ec;

/* callback stuff */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum o2hb_callback_type {
    O2HB_NODE_DOWN_CB = 0,
    O2HB_NODE_UP_CB,
    O2HB_NUM_CB,
}

#[repr(C)]
pub struct o2nm_node {
    _private: [u8; 0],
}

pub type o2hb_cb_func = unsafe extern "C" fn(*mut o2nm_node, c_int, *mut c_void);

#[repr(C)]
pub struct o2hb_callback_func {
    pub hc_magic: u32,
    pub hc_item: list_head,
    pub hc_func: Option<o2hb_cb_func>,
    pub hc_data: *mut c_void,
    pub hc_priority: c_int,
    pub hc_type: o2hb_callback_type,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct config_group {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn o2hb_alloc_hb_set() -> *mut config_group;
    pub fn o2hb_free_hb_set(group: *mut config_group);

    pub fn o2hb_setup_callback(
        hc: *mut o2hb_callback_func,
        type_: o2hb_callback_type,
        func: Option<o2hb_cb_func>,
        data: *mut c_void,
        priority: c_int,
    );
    pub fn o2hb_register_callback(
        region_uuid: *const c_char,
        hc: *mut o2hb_callback_func,
    ) -> c_int;
    pub fn o2hb_unregister_callback(
        region_uuid: *const c_char,
        hc: *mut o2hb_callback_func,
    );
    pub fn o2hb_callback_read_lock();
    pub fn o2hb_callback_read_unlock();
    pub fn o2hb_synchronize_callbacks();
    pub fn o2hb_fill_node_map_locked(map: *mut c_ulong, bits: c_uint);
    pub fn o2hb_fill_node_map(map: *mut c_ulong, bits: c_uint);
    pub fn o2hb_exit();
    pub fn o2hb_init();
    pub fn o2hb_check_node_heartbeating_no_sem(node_num: u8) -> c_int;
    pub fn o2hb_check_node_heartbeating_from_callback(node_num: u8) -> c_int;
    pub fn o2hb_stop_all_regions();
    pub fn o2hb_get_all_regions(region_uuids: *mut c_char, numregions: u8) -> c_int;
    pub fn o2hb_global_heartbeat_active() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
