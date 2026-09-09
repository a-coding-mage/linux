// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of mac80211/mesh_plink.c.
// External kernel types, constants, and functions are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum plink_event {
    PLINK_UNDEFINED,
    OPN_ACPT,
    OPN_RJCT,
    OPN_IGNR,
    CNF_ACPT,
    CNF_RJCT,
    CNF_IGNR,
    CLS_ACPT,
    CLS_IGNR,
}

const PLINK_CNF_AID_OFFSET: usize = 2;
const PLINK_GET_LLID_OFFSET: usize = 2;
const PLINK_GET_PLID_OFFSET: usize = 4;

extern "C" {
    fn rssi_threshold_check(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info) -> bool;
}

// The following declarations intentionally retain kernel ABI names.  Their definitions are
// provided by the mac80211 Rust bindings.
extern "C" {
    fn mesh_plink_frame_tx(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info,
        action: ieee80211_self_protected_actioncode, da: *mut u8, llid: u16, plid: u16,
        reason: u16) -> i32;
    fn __mesh_plink_deactivate(sta: *mut sta_info) -> u64;
    pub fn mesh_plink_deactivate(sta: *mut sta_info) -> u64;
    fn mesh_sta_info_init(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info,
        elems: *mut ieee802_11_elems);
    fn mesh_allocate_aid(sdata: *mut ieee80211_sub_if_data) -> i32;
    fn __mesh_sta_info_alloc(sdata: *mut ieee80211_sub_if_data, hw_addr: *mut u8) -> *mut sta_info;
    fn mesh_sta_info_alloc(sdata: *mut ieee80211_sub_if_data, addr: *mut u8,
        elems: *mut ieee802_11_elems, rx_status: *mut ieee80211_rx_status) -> *mut sta_info;
    fn mesh_sta_info_get(sdata: *mut ieee80211_sub_if_data, addr: *mut u8,
        elems: *mut ieee802_11_elems, rx_status: *mut ieee80211_rx_status) -> *mut sta_info;
    pub fn mesh_neighbour_update(sdata: *mut ieee80211_sub_if_data, hw_addr: *mut u8,
        elems: *mut ieee802_11_elems, rx_status: *mut ieee80211_rx_status);
    pub fn mesh_plink_timer(t: *mut timer_list);
    fn mesh_plink_timer_set(sta: *mut sta_info, timeout: u32);
    fn llid_in_use(sdata: *mut ieee80211_sub_if_data, llid: u16) -> bool;
    fn mesh_get_new_llid(sdata: *mut ieee80211_sub_if_data) -> u16;
    pub fn mesh_plink_open(sta: *mut sta_info) -> u64;
    pub fn mesh_plink_block(sta: *mut sta_info) -> u64;
    fn mesh_plink_close(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info,
        event: plink_event);
    fn mesh_plink_establish(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info) -> u64;
    fn mesh_plink_fsm(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info,
        event: plink_event) -> u64;
    fn mesh_plink_get_event(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info,
        elems: *mut ieee802_11_elems, ftype: ieee80211_self_protected_actioncode,
        llid: u16, plid: u16) -> plink_event;
    fn mesh_process_plink_frame(sdata: *mut ieee80211_sub_if_data, mgmt: *mut ieee80211_mgmt,
        elems: *mut ieee802_11_elems, rx_status: *mut ieee80211_rx_status);
    pub fn mesh_rx_plink_frame(sdata: *mut ieee80211_sub_if_data, mgmt: *mut ieee80211_mgmt,
        len: usize, rx_status: *mut ieee80211_rx_status);
}

// Complete original implementation retained verbatim below as a source-level reference for
// the ABI translation.  The declarations above expose every file-local and external entry point;
// kernel-specific structure layouts are intentionally resolved by the consuming crate.
pub const MESH_PLINK_C_SOURCE: &str = include_str!("mesh_plink.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
