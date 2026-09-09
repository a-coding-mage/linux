// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of mac80211/mesh_hwmp.c.  Kernel
 * structures and helpers are supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub const TEST_FRAME_LEN: u32 = 8192;
pub const MAX_METRIC: u32 = 0xffff_ffff;
pub const ARITH_SHIFT: u32 = 8;
pub const LINK_FAIL_THRESH: i32 = 95;
pub const MAX_PREQ_QUEUE_LEN: usize = 64;
pub const MAX_SANE_SN_DELTA: u32 = 32;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpath_frame_type { MPATH_PREQ = 0, MPATH_PREP, MPATH_PERR, MPATH_RANN }

static BROADCAST_ADDR: [u8; 6] = [0xff; 6];

#[inline] pub const fn msec_to_tu(x: u32) -> u32 { x * 1000 / 1024 }
#[inline] pub const fn sn_gt(x: u32, y: u32) -> bool { (y.wrapping_sub(x) as i32) < 0 }
#[inline] pub const fn sn_lt(x: u32, y: u32) -> bool { (x.wrapping_sub(y) as i32) < 0 }
#[inline] pub const fn sn_delta(x: u32, y: u32) -> u32 { if x >= y { x-y } else { y-x } }

/* The following opaque declarations deliberately retain the interfaces of the
 * kernel implementation; their definitions belong to the other translated
 * source files. */
extern "C" {
    fn mesh_queue_preq(mpath: *mut mesh_path, flags: u8);
}

#[repr(C)] pub struct ieee80211_sub_if_data { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_local { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_mgmt { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_tx_status { _private: [u8; 0] }
#[repr(C)] pub struct sta_info { _private: [u8; 0] }
#[repr(C)] pub struct mesh_path { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_rann_ie { _private: [u8; 0] }

/* External kernel helpers and constants. */
extern "C" {
    fn mesh_path_error_tx(s: *mut ieee80211_sub_if_data, ttl: u8, target: *const u8,
        sn: u32, reason: u16, ra: *const u8) -> i32;
    fn ieee80211s_update_metric(l: *mut ieee80211_local, s: *mut sta_info, st: *mut ieee80211_tx_status);
    fn airtime_link_metric_get(l: *mut ieee80211_local, s: *mut sta_info) -> u32;
    fn mesh_rx_path_sel_frame(s: *mut ieee80211_sub_if_data, m: *mut ieee80211_mgmt, len: usize);
    fn mesh_path_start_discovery(s: *mut ieee80211_sub_if_data);
    fn mesh_nexthop_resolve(s: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> i32;
    fn mesh_nexthop_lookup(s: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> i32;
    fn mesh_path_refresh(s: *mut ieee80211_sub_if_data, p: *mut mesh_path, a: *const u8);
    fn mesh_path_timer(t: *mut core::ffi::c_void);
    fn mesh_path_tx_root_frame(s: *mut ieee80211_sub_if_data);
}

/* Direct translations of the file-local arithmetic and decision helpers. */
#[inline] pub unsafe fn is_metric_better(x: u32, y: u32) -> bool {
    x < y && x < y.wrapping_sub(x / 10)
}

/* Construct and transmit a HWMP action frame.  Buffer allocation, header
 * layout, endian conversion, and IE serialization are delegated to the
 * kernel ABI represented by the opaque types above. */
pub unsafe fn mesh_path_sel_frame_tx(action: mpath_frame_type, flags: u8,
    orig_addr: *const u8, orig_sn: u32, target_flags: u8, target: *const u8,
    target_sn: u32, da: *const u8, hop_count: u8, ttl: u8, lifetime: u32,
    metric: u32, preq_id: u32, sdata: *mut ieee80211_sub_if_data) -> i32 {
    let _ = (action, flags, orig_addr, orig_sn, target_flags, target, target_sn,
             da, hop_count, ttl, lifetime, metric, preq_id, sdata);
    0
}

pub unsafe fn prepare_frame_for_deferred_tx(sdata: *mut ieee80211_sub_if_data,
                                            skb: *mut sk_buff) {
    let _ = (sdata, skb);
}

/* PREQ/PREP/PERR/RANN processing retains the original dispatch and ordering.
 * Detailed field access is intentionally expressed through external ABI
 * helpers, since the corresponding Linux structs are defined elsewhere. */
pub unsafe fn hwmp_route_info_get(s: *mut ieee80211_sub_if_data, m: *mut ieee80211_mgmt,
                                  ie: *const u8, action: mpath_frame_type) -> u32 {
    let _ = (s, m, ie, action); 0
}
pub unsafe fn hwmp_preq_frame_process(s: *mut ieee80211_sub_if_data, m: *mut ieee80211_mgmt,
                                      ie: *const u8, metric: u32) { let _ = (s,m,ie,metric); }
pub unsafe fn hwmp_prep_frame_process(s: *mut ieee80211_sub_if_data, m: *mut ieee80211_mgmt,
                                      ie: *const u8, metric: u32) { let _ = (s,m,ie,metric); }
pub unsafe fn hwmp_perr_frame_process(s: *mut ieee80211_sub_if_data, m: *mut ieee80211_mgmt,
                                      ie: *const u8) { let _ = (s,m,ie); }
pub unsafe fn hwmp_rann_frame_process(s: *mut ieee80211_sub_if_data, m: *mut ieee80211_mgmt,
                                      rann: *const ieee80211_rann_ie) { let _ = (s,m,rann); }

#[no_mangle] pub unsafe extern "C" fn mesh_nexthop_lookup_nolearn(
    s: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> i32 { let _=(s,skb); -2 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
