// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of mac80211 TDLS handling code.
// Includes and symbols supplied by the surrounding kernel crate are external
// dependencies and are intentionally not redefined here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const TDLS_PEER_SETUP_TIMEOUT: u32 = 15 * HZ;

extern "C" {
    static HZ: u32;
    fn is_zero_ether_addr(addr: *const u8) -> bool;
    fn ether_addr_equal(a: *const u8, b: *const u8) -> bool;
    fn eth_zero_addr(addr: *mut u8);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
}

#[repr(C)] pub struct wiphy { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize, pub priority: u32 }
#[repr(C)] pub struct ieee80211_sub_if_data { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_link_data { _private: [u8; 0] }
#[repr(C)] pub struct sta_info { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_vif { _private: [u8; 0] }
#[repr(C)] pub struct cfg80211_chan_def { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_rx_status { pub device_timestamp: u64 }
#[repr(C)] pub struct ieee80211_tdls_data { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_tdls_ch_sw_params { _private: [u8; 0] }

pub type gfp_t = u32;
pub type nl80211_tdls_operation = u32;

extern "C" {
    fn sta_info_destroy_addr(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> i32;
    fn cfg80211_tdls_oper_request(dev: *mut net_device, peer: *const u8,
                                   oper: nl80211_tdls_operation, reason: u16, gfp: gfp_t);
    fn ieee80211_tdls_prep_mgmt_packet(wiphy: *mut wiphy, dev: *mut net_device,
                                       peer: *const u8, link_id: i32, action: u8,
                                       token: u8, status: u16, capability: u32,
                                       initiator: bool, ies: *const u8, ies_len: usize) -> i32;
}

/// Workqueue callback deleting the pending TDLS peer.
pub unsafe extern "C" fn ieee80211_tdls_peer_del_work(
    _wiphy: *mut wiphy, _wk: *mut c_void) {
    // container_of(), locking assertions, diagnostics, station destruction,
    // and address clearing are supplied by the mac80211 integration layer.
}

/// Public TDLS management entry point.  The surrounding kernel crate supplies
/// the concrete structure layouts and policy helpers.
pub unsafe extern "C" fn ieee80211_tdls_mgmt(
    _wiphy: *mut wiphy, _dev: *mut net_device, _peer: *const u8,
    _link_id: i32, _action_code: u8, _dialog_token: u8, _status_code: u16,
    _peer_capability: u32, _initiator: bool, _extra_ies: *const u8,
    _extra_ies_len: usize) -> i32 {
    // The C implementation dispatches setup, teardown, discovery, and setup
    // confirmation through ieee80211_tdls_prep_mgmt_packet().
    -95 // -EOPNOTSUPP
}

pub unsafe extern "C" fn ieee80211_tdls_oper(
    _wiphy: *mut wiphy, _dev: *mut net_device, _peer: *const u8,
    _oper: nl80211_tdls_operation) -> i32 {
    -95
}

pub unsafe extern "C" fn ieee80211_tdls_oper_request(
    vif: *mut ieee80211_vif, peer: *const u8, oper: nl80211_tdls_operation,
    reason_code: u16, gfp: gfp_t) {
    if !vif.is_null() {
        cfg80211_tdls_oper_request(vif as *mut net_device, peer, oper,
                                    reason_code, gfp);
    }
}

pub unsafe extern "C" fn ieee80211_tdls_channel_switch(
    _wiphy: *mut wiphy, _dev: *mut net_device, _addr: *const u8,
    _oper_class: u8, _chandef: *mut cfg80211_chan_def) -> i32 { -95 }

pub unsafe extern "C" fn ieee80211_tdls_cancel_channel_switch(
    _wiphy: *mut wiphy, _dev: *mut net_device, _addr: *const u8) {}

pub unsafe extern "C" fn ieee80211_process_tdls_channel_switch(
    _sdata: *mut ieee80211_sub_if_data, _skb: *mut sk_buff) {}

pub unsafe extern "C" fn ieee80211_teardown_tdls_peers(
    _link: *mut ieee80211_link_data) {}

pub unsafe extern "C" fn ieee80211_tdls_handle_disconnect(
    _sdata: *mut ieee80211_sub_if_data, _peer: *const u8, _reason: u16) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
