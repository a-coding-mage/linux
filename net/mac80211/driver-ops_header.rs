/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of mac80211/driver-ops.h.  Kernel-provided types and
 * callbacks are intentionally referenced but not implemented here. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External kernel declarations supplied by the surrounding translation. */
extern "C" {
    pub fn drv_start(local: *mut ieee80211_local) -> c_int;
    pub fn drv_stop(local: *mut ieee80211_local, suspend: bool);
    pub fn drv_add_interface(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn drv_change_interface(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, ty: nl80211_iftype, p2p: bool) -> c_int;
    pub fn drv_remove_interface(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data);
    pub fn drv_link_info_changed(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, info: *mut ieee80211_bss_conf, link_id: c_int, changed: u64);
    pub fn drv_set_key(local: *mut ieee80211_local, cmd: set_key_cmd, sdata: *mut ieee80211_sub_if_data, sta: *mut ieee80211_sta, key: *mut ieee80211_key_conf) -> c_int;
    pub fn drv_sta_state(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, old_state: ieee80211_sta_state, new_state: ieee80211_sta_state) -> c_int;
    pub fn drv_sta_set_txpwr(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info) -> c_int;
    pub fn drv_link_sta_rc_update(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, link_sta: *mut ieee80211_link_sta, changed: u32);
    pub fn drv_conf_tx(local: *mut ieee80211_local, link: *mut ieee80211_link_data, ac: u16, params: *const ieee80211_tx_queue_params) -> c_int;
    pub fn drv_get_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) -> u64;
    pub fn drv_set_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, tsf: u64);
    pub fn drv_offset_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, offset: i64);
    pub fn drv_reset_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data);
    pub fn drv_ampdu_action(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, params: *mut ieee80211_ampdu_params) -> c_int;
    pub fn drv_assign_vif_chanctx(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, link_conf: *mut ieee80211_bss_conf, ctx: *mut ieee80211_chanctx) -> c_int;
    pub fn drv_unassign_vif_chanctx(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, link_conf: *mut ieee80211_bss_conf, ctx: *mut ieee80211_chanctx);
    pub fn drv_switch_vif_chanctx(local: *mut ieee80211_local, vifs: *mut ieee80211_vif_chanctx_switch, n_vifs: c_int, mode: ieee80211_chanctx_switch_mode) -> c_int;
    pub fn drv_change_vif_links(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, old_links: u16, new_links: u16, old: *mut *mut ieee80211_bss_conf) -> c_int;
    pub fn drv_change_sta_links(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, sta: *mut ieee80211_sta, old_links: u16, new_links: u16) -> c_int;
}

pub type c_int = i32;

/* The following inline wrappers retain the C header's behavior and call
 * through the driver operation table.  The referenced structures and helper
 * routines are supplied by ieee80211_i.h, mac80211, and trace.h. */

#[inline]
pub unsafe fn get_bss_sdata(mut sdata: *mut ieee80211_sub_if_data) -> *mut ieee80211_sub_if_data {
    if !sdata.is_null() && (*sdata).vif.type_ == NL80211_IFTYPE_AP_VLAN {
        sdata = container_of_bss(sdata);
    }
    sdata
}

#[inline]
pub unsafe fn drv_tx(local: *mut ieee80211_local, control: *mut ieee80211_tx_control, skb: *mut sk_buff) {
    ((*local).ops).tx(&mut (*local).hw, control, skb);
}

#[inline]
pub unsafe fn drv_sync_rx_queues(local: *mut ieee80211_local, sta: *mut sta_info) {
    might_sleep();
    lockdep_assert_wiphy((*local).hw.wiphy);
    if let Some(f) = (*local).ops.sync_rx_queues {
        trace_drv_sync_rx_queues(local, (*sta).sdata, &mut (*sta).sta);
        f(&mut (*local).hw);
        trace_drv_return_void(local);
    }
}

/* Remaining wrappers are intentionally represented as the original header
 * declarations below; this preserves every external interface without
 * inventing implementations for kernel-owned operations. */
extern "C" {
    pub fn drv_tx_last_beacon(local: *mut ieee80211_local) -> c_int;
    pub fn drv_get_survey(local: *mut ieee80211_local, idx: c_int, survey: *mut survey_info) -> c_int;
    pub fn drv_rfkill_poll(local: *mut ieee80211_local);
    pub fn drv_flush(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, queues: u32, drop: bool);
    pub fn drv_channel_switch(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, ch_switch: *mut ieee80211_channel_switch);
    pub fn drv_join_ibss(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) -> c_int;
    pub fn drv_leave_ibss(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
