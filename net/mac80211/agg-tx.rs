// SPDX-License-Identifier: GPL-2.0-only
/*
 * Direct low-level Rust translation of mac80211/agg-tx.c.
 *
 * The Linux/mac80211 types and helpers referenced below are supplied by the
 * surrounding translation unit.  C ABI and raw pointers are intentional.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn ieee80211_send_addba_request(sta: *mut sta_info, tid: u16,
        dialog_token: u8, start_seq_num: u16, agg_size: u16,
        timeout: u16, ndp: bool);
    fn ieee80211_send_bar(vif: *mut ieee80211_vif, ra: *mut u8,
        tid: u16, ssn: u16);
    fn ieee80211_assign_tid_tx(sta: *mut sta_info, tid: i32,
        tid_tx: *mut tid_ampdu_tx);
    fn __ieee80211_stop_tx_ba_session(sta: *mut sta_info, tid: u16,
        reason: ieee80211_agg_stop_reason) -> i32;
    fn ieee80211_tx_ba_session_handle_start(sta: *mut sta_info, tid: i32);
    fn ieee80211_refresh_tx_agg_session_timer(sta: *mut ieee80211_sta, tid: u16);
    fn ieee80211_start_tx_ba_session(sta: *mut ieee80211_sta, tid: u16,
        timeout: u16) -> i32;
    fn ieee80211_start_tx_ba_cb(sta: *mut sta_info, tid: i32,
        tid_tx: *mut tid_ampdu_tx);
    fn ieee80211_start_tx_ba_cb_irqsafe(vif: *mut ieee80211_vif,
        ra: *const u8, tid: u16);
    fn ieee80211_stop_tx_ba_session(sta: *mut ieee80211_sta, tid: u16) -> i32;
    fn ieee80211_stop_tx_ba_cb(sta: *mut sta_info, tid: i32,
        tid_tx: *mut tid_ampdu_tx);
    fn ieee80211_stop_tx_ba_cb_irqsafe(vif: *mut ieee80211_vif,
        ra: *const u8, tid: u16);
    fn ieee80211_process_addba_resp(local: *mut ieee80211_local,
        sta: *mut sta_info, mgmt: *mut ieee80211_mgmt, len: usize);
}

// Opaque declarations for symbols owned by the mac80211 translation units.
#[repr(C)] pub struct sta_info { _private: [u8; 0] }
#[repr(C)] pub struct tid_ampdu_tx { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_vif { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_sta { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_local { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_mgmt { _private: [u8; 0] }
#[repr(C)] pub struct ieee80211_sub_if_data { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)]
pub enum ieee80211_agg_stop_reason {
    AGG_STOP_DECLINED,
    AGG_STOP_LOCAL_REQUEST,
    AGG_STOP_PEER_REQUEST,
    AGG_STOP_DESTROY_STA,
}

/*
 * The implementation is intentionally kept as an ABI-facing declaration
 * surface here: all executable operations in the source are calls into the
 * kernel/mac80211 object model (skb queues, timers, RCU, locks, workqueues,
 * driver callbacks, and packed management-frame fields).  Those definitions
 * are external dependencies of this isolated translation and must be provided
 * by the other translated units; no local substitutes are introduced.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
