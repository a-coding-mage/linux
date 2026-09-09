// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation boundary for mac80211 mesh.c.
// The implementation relies on the kernel/mac80211 ABI declarations supplied
// by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// C headers and kernel-provided definitions are intentionally external.
extern "C" {
    static mut mesh_allocated: c_int;
    static mut rm_cache: *mut c_void;
}

#[repr(C)]
pub struct ieee80211_mgmt { _opaque: [u8; 0] }
#[repr(C)]
pub struct ieee802_11_elems { _opaque: [u8; 0] }
#[repr(C)]
pub struct ieee80211_sub_if_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct ieee80211_if_mesh { _opaque: [u8; 0] }
#[repr(C)]
pub struct ieee80211s_hdr { _opaque: [u8; 0] }
#[repr(C)]
pub struct ieee80211_hdr { _opaque: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _opaque: [u8; 0] }
#[repr(C)]
pub struct sta_info { _opaque: [u8; 0] }
#[repr(C)]
pub struct ieee80211_rx_status { _opaque: [u8; 0] }
#[repr(C)]
pub struct cfg80211_csa_settings { _opaque: [u8; 0] }

extern "C" {
    pub fn mesh_action_is_path_sel(mgmt: *mut ieee80211_mgmt) -> bool;
    pub fn ieee80211s_init();
    pub fn ieee80211s_stop();
    pub fn mesh_matches_local(sdata: *mut ieee80211_sub_if_data, ie: *mut ieee802_11_elems) -> bool;
    pub fn mesh_peer_accepts_plinks(ie: *mut ieee802_11_elems) -> bool;
    pub fn mesh_accept_plinks_update(sdata: *mut ieee80211_sub_if_data) -> u64;
    pub fn mesh_sta_cleanup(sta: *mut sta_info);
    pub fn mesh_rmc_init(sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn mesh_rmc_free(sdata: *mut ieee80211_sub_if_data);
    pub fn mesh_rmc_check(sdata: *mut ieee80211_sub_if_data, sa: *const u8, mesh_hdr: *mut ieee80211s_hdr) -> c_int;
    pub fn mesh_add_meshconf_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_meshid_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_vendor_ies(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_rsn_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_ht_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_ht_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_vht_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_vht_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_he_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, ie_len: u8) -> c_int;
    pub fn mesh_add_he_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_he_6ghz_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn mesh_add_eht_cap_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, ie_len: u8) -> c_int;
    pub fn mesh_add_eht_oper_ie(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) -> c_int;
    pub fn ieee80211_mesh_root_setup(ifmsh: *mut ieee80211_if_mesh);
    pub fn ieee80211_mesh_xmit_fast(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff, ctrl_flags: u32) -> bool;
    pub fn ieee80211_fill_mesh_addresses(hdr: *mut ieee80211_hdr, fc: *mut u16, meshda: *const u8, meshsa: *const u8) -> c_int;
    pub fn ieee80211_new_mesh_header(sdata: *mut ieee80211_sub_if_data, meshhdr: *mut ieee80211s_hdr, addr4or5: *const c_char, addr6: *const c_char) -> u32;
    pub fn ieee80211_mesh_finish_csa(sdata: *mut ieee80211_sub_if_data, changed: *mut u64) -> c_int;
    pub fn ieee80211_mesh_csa_beacon(sdata: *mut ieee80211_sub_if_data, settings: *mut cfg80211_csa_settings, changed: *mut u64) -> c_int;
    pub fn ieee80211_mesh_rx_queued_mgmt(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff);
    pub fn ieee80211_start_mesh(sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn ieee80211_stop_mesh(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_mesh_work(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_mesh_init_sdata(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_mesh_teardown_sdata(sdata: *mut ieee80211_sub_if_data);
    pub fn ieee80211_mbss_info_change_notify(sdata: *mut ieee80211_sub_if_data, changed: u64);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
