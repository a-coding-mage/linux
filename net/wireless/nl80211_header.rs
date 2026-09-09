/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Portions of this file
 * Copyright (C) 2018, 2020-2026 Intel Corporation
 */

// Dependency intent: declarations from "core.h" are supplied by other files.

unsafe extern "C" {
    pub fn nl80211_init() -> ::core::ffi::c_int;
    pub fn nl80211_exit();

    pub fn nl80211hdr_put(
        skb: *mut sk_buff,
        portid: u32,
        seq: u32,
        flags: ::core::ffi::c_int,
        cmd: u8,
    ) -> *mut ::core::ffi::c_void;
    pub fn nl80211_put_sta_rate(
        msg: *mut sk_buff,
        info: *mut rate_info,
        attr: ::core::ffi::c_int,
    ) -> bool;

    pub fn nl80211_parse_chandef(
        rdev: *mut cfg80211_registered_device,
        extack: *mut netlink_ext_ack,
        attrs: *mut *mut nlattr,
        chandef: *mut cfg80211_chan_def,
        npca_permitted: bool,
    ) -> ::core::ffi::c_int;
    pub fn nl80211_parse_random_mac(
        attrs: *mut *mut nlattr,
        mac_addr: *mut u8,
        mac_addr_mask: *mut u8,
    ) -> ::core::ffi::c_int;

    pub fn nl80211_notify_wiphy(
        rdev: *mut cfg80211_registered_device,
        cmd: nl80211_commands,
    );
    pub fn nl80211_notify_iface(
        rdev: *mut cfg80211_registered_device,
        wdev: *mut wireless_dev,
        cmd: nl80211_commands,
    );
    pub fn nl80211_send_scan_start(
        rdev: *mut cfg80211_registered_device,
        wdev: *mut wireless_dev,
    );
    pub fn nl80211_build_scan_msg(
        rdev: *mut cfg80211_registered_device,
        wdev: *mut wireless_dev,
        aborted: bool,
    ) -> *mut sk_buff;
    pub fn nl80211_send_scan_msg(
        rdev: *mut cfg80211_registered_device,
        msg: *mut sk_buff,
    );
    pub fn nl80211_send_sched_scan(req: *mut cfg80211_sched_scan_request, cmd: u32);
    pub fn nl80211_common_reg_change_event(
        cmd_id: nl80211_commands,
        request: *mut regulatory_request,
    );

    pub fn nl80211_send_rx_auth(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        buf: *const u8,
        len: usize,
        gfp: gfp_t,
    );
    pub fn nl80211_send_rx_assoc(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        data: *const cfg80211_rx_assoc_resp_data,
    );
    pub fn nl80211_send_deauth(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        buf: *const u8,
        len: usize,
        reconnect: bool,
        gfp: gfp_t,
    );
    pub fn nl80211_send_disassoc(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        buf: *const u8,
        len: usize,
        reconnect: bool,
        gfp: gfp_t,
    );
    pub fn nl80211_send_auth_timeout(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        addr: *const u8,
        gfp: gfp_t,
    );
    pub fn nl80211_send_assoc_timeout(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        addr: *const u8,
        gfp: gfp_t,
    );
    pub fn nl80211_send_connect_result(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        params: *mut cfg80211_connect_resp_params,
        gfp: gfp_t,
    );
    pub fn nl80211_send_roamed(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        info: *mut cfg80211_roam_info,
        gfp: gfp_t,
    );
    /* For STA/GC, indicate port authorized with AP/GO bssid.
     * For GO/AP, use peer GC/STA mac_addr.
     */
    pub fn nl80211_send_port_authorized(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        peer_addr: *const u8,
        td_bitmap: *const u8,
        td_bitmap_len: u8,
    );
    pub fn nl80211_send_disconnected(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        reason: u16,
        ie: *const u8,
        ie_len: usize,
        from_ap: bool,
    );

    pub fn nl80211_michael_mic_failure(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        addr: *const u8,
        key_type: nl80211_key_type,
        key_id: ::core::ffi::c_int,
        tsc: *const u8,
        gfp: gfp_t,
    );
    pub fn nl80211_send_beacon_hint_event(
        wiphy: *mut wiphy,
        channel_before: *mut ieee80211_channel,
        channel_after: *mut ieee80211_channel,
    );
    pub fn nl80211_send_ibss_bssid(
        rdev: *mut cfg80211_registered_device,
        netdev: *mut net_device,
        bssid: *const u8,
        gfp: gfp_t,
    );
    pub fn nl80211_send_mgmt(
        rdev: *mut cfg80211_registered_device,
        wdev: *mut wireless_dev,
        nlpid: u32,
        info: *mut cfg80211_rx_info,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn nl80211_radar_notify(
        rdev: *mut cfg80211_registered_device,
        chandef: *const cfg80211_chan_def,
        event: nl80211_radar_event,
        netdev: *mut net_device,
        gfp: gfp_t,
    );
    pub fn nl80211_send_ap_stopped(wdev: *mut wireless_dev, link_id: ::core::ffi::c_uint);
    pub fn cfg80211_free_coalesce(coalesce: *mut cfg80211_coalesce);

    /* peer measurement */
    pub fn nl80211_pmsr_start(skb: *mut sk_buff, info: *mut genl_info) -> ::core::ffi::c_int;
    pub fn nl80211_mlo_reconf_add_done(
        dev: *mut net_device,
        data: *mut cfg80211_mlo_reconf_done_data,
    );
}

#[inline]
pub unsafe fn wdev_id(wdev: *mut wireless_dev) -> u64 {
    (*wdev).identifier as u64
        | ((wiphy_to_rdev((*wdev).wiphy).as_ref().unwrap().wiphy_idx as u64) << 32)
}

#[inline]
pub unsafe fn nl80211_send_reg_change_event(request: *mut regulatory_request) {
    nl80211_common_reg_change_event(NL80211_CMD_REG_CHANGE, request);
}

#[inline]
pub unsafe fn nl80211_send_wiphy_reg_change_event(request: *mut regulatory_request) {
    nl80211_common_reg_change_event(NL80211_CMD_WIPHY_REG_CHANGE, request);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
