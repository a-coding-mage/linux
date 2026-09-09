// SPDX-License-Identifier: GPL-2.0-only
/*
 * mac80211 - channel management
 * Copyright 2020-2026 Intel Corporation
 *
 * Direct low-level Rust translation boundary for chan.c.  Kernel types,
 * constants, list/RCU primitives, and driver operations are supplied by the
 * surrounding translation units.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    pub fn ieee80211_chanctx_num_assigned(local: *mut ieee80211_local,
                                          ctx: *mut ieee80211_chanctx) -> i32;
    pub fn ieee80211_chanctx_refcount(local: *mut ieee80211_local,
                                      ctx: *mut ieee80211_chanctx) -> i32;
    pub fn ieee80211_recalc_chanctx_min_def(local: *mut ieee80211_local,
                                             ctx: *mut ieee80211_chanctx);
    pub fn ieee80211_recalc_smps_chanctx(local: *mut ieee80211_local,
                                         ctx: *mut ieee80211_chanctx);
    pub fn ieee80211_link_release_channel(link: *mut ieee80211_link_data);
    pub fn ieee80211_iter_chan_contexts_atomic(
        hw: *mut ieee80211_hw,
        iter: Option<unsafe extern "C" fn(*mut ieee80211_hw,
                                           *mut ieee80211_chanctx_conf,
                                           *mut c_void),
        data: *mut c_void);
    pub fn ieee80211_iter_chan_contexts_mtx(
        hw: *mut ieee80211_hw,
        iter: Option<unsafe extern "C" fn(*mut ieee80211_hw,
                                           *mut ieee80211_chanctx_conf,
                                           *mut c_void),
        data: *mut c_void);
}

#[repr(C)]
pub struct ieee80211_chanctx_user_iter {
    pub chanreq: *mut ieee80211_chan_req,
    pub sdata: *mut ieee80211_sub_if_data,
    pub link: *mut ieee80211_link_data,
    pub nan_channel: *mut ieee80211_nan_channel,
    pub nan_channel_next_idx: i32,
    pub iftype: u32,
    pub reserved: bool,
    pub radar_required: bool,
    pub done: bool,
    pub per_link: chanctx_iter_pos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum chanctx_iter_pos { CHANCTX_ITER_POS_ASSIGNED, CHANCTX_ITER_POS_RESERVED,
    CHANCTX_ITER_POS_DONE }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ieee80211_chanctx_iter_type { CHANCTX_ITER_ALL, CHANCTX_ITER_RESERVED,
    CHANCTX_ITER_ASSIGNED }

// The remainder intentionally retains C kernel semantics at the FFI boundary:
// these declarations are implemented by the corresponding translated kernel
// units.  No ownership or synchronization semantics are changed here.
pub type ieee80211_local = c_void;
pub type ieee80211_chanctx = c_void;
pub type ieee80211_chan_req = c_void;
pub type ieee80211_sub_if_data = c_void;
pub type ieee80211_link_data = c_void;
pub type ieee80211_nan_channel = c_void;
pub type ieee80211_chanctx_conf = c_void;
pub type ieee80211_hw = c_void;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
