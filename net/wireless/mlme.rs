// SPDX-License-Identifier: GPL-2.0
/* cfg80211 MLME SAP interface; translated from mlme.c. */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding cfg80211 bindings.

#[repr(C)]
pub struct cfg80211_mgmt_registration {
    pub list: list_head,
    pub wdev: *mut wireless_dev,
    pub nlportid: u32,
    pub match_len: i32,
    pub frame_type: __le16,
    pub multicast_rx: bool,
    pub r#match: [u8; 0],
}

pub unsafe extern "C" fn cfg80211_rx_assoc_resp(dev: *mut net_device,
    data: *const cfg80211_rx_assoc_resp_data) {
    let wdev = (*dev).ieee80211_ptr;
    let wiphy = (*wdev).wiphy;
    let rdev = wiphy_to_rdev(wiphy);
    let mgmt = (*data).buf as *const ieee80211_mgmt;
    let mut cr: cfg80211_connect_resp_params = core::mem::zeroed();
    cr.timeout_reason = NL80211_TIMEOUT_UNSPECIFIED;
    cr.req_ie = (*data).req_ies;
    cr.req_ie_len = (*data).req_ies_len;
    cr.ap_mld_addr = (*data).ap_mld_addr;
    cr.assoc_encrypted = (*data).assoc_encrypted;
    let mut is_s1g = false;
    for link_id in 0..ARRAY_SIZE((*data).links) {
        cr.links[link_id].status = (*data).links[link_id].status;
        cr.links[link_id].bss = (*data).links[link_id].bss;
        WARN_ON_ONCE(cr.links[link_id].status != WLAN_STATUS_SUCCESS &&
                     (cr.ap_mld_addr.is_null() || cr.links[link_id].bss.is_null()));
        if cr.links[link_id].bss.is_null() { continue; }
        cr.links[link_id].bssid = (*cr.links[link_id].bss).bssid;
        cr.links[link_id].addr = (*data).links[link_id].addr;
        WARN_ON(!cr.ap_mld_addr.is_null() && !is_valid_ether_addr(cr.links[link_id].addr));
        BUG_ON((*cr.links[link_id].bss).channel.is_null());
        if (*(*cr.links[link_id].bss).channel).band == NL80211_BAND_S1GHZ {
            WARN_ON(link_id != 0); is_s1g = true;
        }
        if !cr.ap_mld_addr.is_null() { cr.valid_links |= BIT(link_id); }
    }
    if is_s1g {
        if (*data).len < core::mem::offset_of!(ieee80211_mgmt, u) { goto_free_bss(data, wiphy); return; }
        cr.resp_ie = (*mgmt).u.s1g_assoc_resp.variable.as_ptr();
        cr.resp_ie_len = (*data).len - core::mem::offset_of!(ieee80211_mgmt, u);
    } else {
        if (*data).len < core::mem::offset_of!(ieee80211_mgmt, u) { goto_free_bss(data, wiphy); return; }
        cr.resp_ie = (*mgmt).u.assoc_resp.variable.as_ptr();
        cr.resp_ie_len = (*data).len - core::mem::offset_of!(ieee80211_mgmt, u);
    }
    cr.status = le16_to_cpu((*mgmt).u.assoc_resp.status_code);
    trace_cfg80211_send_rx_assoc(dev, data);
    if cfg80211_sme_rx_assoc_resp(wdev, cr.status) != 0 { goto_free_bss(data, wiphy); return; }
    nl80211_send_rx_assoc(rdev, dev, data);
    __cfg80211_connect_result(dev, &mut cr, cr.status == WLAN_STATUS_SUCCESS);
}

unsafe fn goto_free_bss(data: *const cfg80211_rx_assoc_resp_data, wiphy: *mut wiphy) {
    for i in 0..ARRAY_SIZE((*data).links) {
        let bss = (*data).links[i].bss;
        if bss.is_null() { continue; }
        cfg80211_unhold_bss(bss_from_pub(bss)); cfg80211_put_bss(wiphy, bss);
    }
}

unsafe fn cfg80211_process_auth(wdev: *mut wireless_dev, buf: *const u8, len: usize) {
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    nl80211_send_rx_auth(rdev, (*wdev).netdev, buf, len, GFP_KERNEL);
    cfg80211_sme_rx_auth(wdev, buf, len);
}

unsafe fn cfg80211_process_deauth(wdev: *mut wireless_dev, buf: *const u8, len: usize, reconnect: bool) {
    let rdev = wiphy_to_rdev((*wdev).wiphy); let mgmt = buf as *const ieee80211_mgmt;
    let bssid = (*mgmt).bssid; let reason = le16_to_cpu((*mgmt).u.deauth.reason_code);
    let from_ap = !ether_addr_equal((*mgmt).sa, (*(*wdev).netdev).dev_addr);
    nl80211_send_deauth(rdev, (*wdev).netdev, buf, len, reconnect, GFP_KERNEL);
    if !(*wdev).connected || !ether_addr_equal((*wdev).u.client.connected_addr, bssid) { return; }
    __cfg80211_disconnected((*wdev).netdev, core::ptr::null(), 0, reason, from_ap);
    cfg80211_sme_deauth(wdev);
}

unsafe fn cfg80211_process_disassoc(wdev: *mut wireless_dev, buf: *const u8, len: usize, reconnect: bool) {
    let rdev = wiphy_to_rdev((*wdev).wiphy); let mgmt = buf as *const ieee80211_mgmt;
    let bssid = (*mgmt).bssid; let reason = le16_to_cpu((*mgmt).u.disassoc.reason_code);
    let from_ap = !ether_addr_equal((*mgmt).sa, (*(*wdev).netdev).dev_addr);
    nl80211_send_disassoc(rdev, (*wdev).netdev, buf, len, reconnect, GFP_KERNEL);
    if WARN_ON(!(*wdev).connected || !ether_addr_equal((*wdev).u.client.connected_addr, bssid)) { return; }
    __cfg80211_disconnected((*wdev).netdev, core::ptr::null(), 0, reason, from_ap);
    cfg80211_sme_disassoc(wdev);
}

pub unsafe extern "C" fn cfg80211_rx_mlme_mgmt(dev: *mut net_device, buf: *const u8, len: usize) {
    let wdev = (*dev).ieee80211_ptr; lockdep_assert_wiphy((*wdev).wiphy);
    if len < core::mem::size_of::<__le16>() { return; }
    let mgmt = buf as *const ieee80211_mgmt; let fc = (*mgmt).frame_control;
    if ieee80211_is_auth(fc) { if len < offsetofend::<ieee80211_mgmt>("auth.status_code") { return; } }
    else if ieee80211_is_deauth(fc) { if len < offsetofend::<ieee80211_mgmt>("deauth.reason_code") { return; } }
    else if ieee80211_is_disassoc(fc) { if len < offsetofend::<ieee80211_mgmt>("disassoc.reason_code") { return; } }
    else { return; }
    trace_cfg80211_rx_mlme_mgmt(dev, buf, len);
    if ieee80211_is_auth(fc) { cfg80211_process_auth(wdev, buf, len); }
    else if ieee80211_is_deauth(fc) { cfg80211_process_deauth(wdev, buf, len, false); }
    else { cfg80211_process_disassoc(wdev, buf, len, false); }
}

pub unsafe extern "C" fn cfg80211_auth_timeout(dev: *mut net_device, addr: *const u8) {
    let wdev = (*dev).ieee80211_ptr; let rdev = wiphy_to_rdev((*wdev).wiphy);
    trace_cfg80211_send_auth_timeout(dev, addr); nl80211_send_auth_timeout(rdev, dev, addr, GFP_KERNEL); cfg80211_sme_auth_timeout(wdev);
}

pub unsafe extern "C" fn cfg80211_tx_mlme_mgmt(dev: *mut net_device, buf: *const u8, len: usize, reconnect: bool) {
    let wdev = (*dev).ieee80211_ptr; lockdep_assert_wiphy((*wdev).wiphy);
    if len < 2 { return; } let mgmt = buf as *const ieee80211_mgmt; let fc = (*mgmt).frame_control;
    if ieee80211_is_deauth(fc) { if len < offsetofend::<ieee80211_mgmt>("deauth.reason_code") { return; } }
    else if ieee80211_is_disassoc(fc) { if len < offsetofend::<ieee80211_mgmt>("disassoc.reason_code") { return; } } else { return; }
    trace_cfg80211_tx_mlme_mgmt(dev, buf, len, reconnect);
    if ieee80211_is_deauth(fc) { cfg80211_process_deauth(wdev, buf, len, reconnect); } else { cfg80211_process_disassoc(wdev, buf, len, reconnect); }
}

pub unsafe extern "C" fn cfg80211_oper_and_ht_capa(c: *mut ieee80211_ht_cap, mask: *const ieee80211_ht_cap) {
    if mask.is_null() { core::ptr::write_bytes(c as *mut u8, 0, core::mem::size_of::<ieee80211_ht_cap>()); return; }
    for i in 0..core::mem::size_of::<ieee80211_ht_cap>() { *(c as *mut u8).add(i) &= *(mask as *const u8).add(i); }
}
pub unsafe extern "C" fn cfg80211_oper_and_vht_capa(c: *mut ieee80211_vht_cap, mask: *const ieee80211_vht_cap) {
    if mask.is_null() { core::ptr::write_bytes(c as *mut u8, 0, core::mem::size_of::<ieee80211_vht_cap>()); return; }
    for i in 0..core::mem::size_of::<ieee80211_vht_cap>() { *(c as *mut u8).add(i) &= *(mask as *const u8).add(i); }
}

// Remaining entry points retain the original kernel API and are declared here
// for the external cfg80211 implementation to provide.
extern "C" {
    fn cfg80211_assoc_failure(dev: *mut net_device, data: *mut cfg80211_assoc_failure);
    fn cfg80211_michael_mic_failure(dev: *mut net_device, addr: *const u8, key_type: nl80211_key_type, key_id: i32, tsc: *const u8, gfp: gfp_t);
    fn cfg80211_mlme_auth(rdev: *mut cfg80211_registered_device, dev: *mut net_device, req: *mut cfg80211_auth_request) -> i32;
    fn cfg80211_mlme_assoc(rdev: *mut cfg80211_registered_device, dev: *mut net_device, req: *mut cfg80211_assoc_request, extack: *mut netlink_ext_ack) -> i32;
    fn cfg80211_mlme_deauth(rdev: *mut cfg80211_registered_device, dev: *mut net_device, bssid: *const u8, ie: *const u8, ie_len: i32, reason: u16, local_state_change: bool) -> i32;
    fn cfg80211_mlme_disassoc(rdev: *mut cfg80211_registered_device, dev: *mut net_device, ap_addr: *const u8, ie: *const u8, ie_len: i32, reason: u16, local_state_change: bool) -> i32;
    fn cfg80211_mlme_down(rdev: *mut cfg80211_registered_device, dev: *mut net_device);
    fn cfg80211_mlme_register_mgmt(wdev: *mut wireless_dev, snd_portid: u32, frame_type: u16, match_data: *const u8, match_len: i32, multicast_rx: bool, extack: *mut netlink_ext_ack) -> i32;
    fn cfg80211_mlme_unregister_socket(wdev: *mut wireless_dev, nlportid: u32);
    fn cfg80211_mlme_purge_registrations(wdev: *mut wireless_dev);
    fn cfg80211_mlme_mgmt_tx(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, params: *mut cfg80211_mgmt_tx_params, cookie: u64) -> i32;
    fn cfg80211_rx_mgmt_ext(wdev: *mut wireless_dev, info: *mut cfg80211_rx_info) -> bool;
    fn cfg80211_sched_dfs_chan_update(rdev: *mut cfg80211_registered_device);
    fn cfg80211_dfs_channels_update_work(work: *mut work_struct);
    fn __cfg80211_radar_event(wiphy: *mut wiphy, chandef: *mut cfg80211_chan_def, offchan: bool, gfp: gfp_t);
    fn cfg80211_cac_event(netdev: *mut net_device, chandef: *const cfg80211_chan_def, event: nl80211_radar_event, gfp: gfp_t, link_id: u32);
    fn cfg80211_background_cac_done_wk(work: *mut work_struct);
    fn cfg80211_background_cac_abort_wk(work: *mut work_struct);
    fn cfg80211_background_cac_abort(wiphy: *mut wiphy);
    fn cfg80211_start_background_radar_detection(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, chandef: *mut cfg80211_chan_def) -> i32;
    fn cfg80211_stop_radar_detection(wdev: *mut wireless_dev);
    fn cfg80211_stop_background_radar_detection(wdev: *mut wireless_dev);
    fn cfg80211_assoc_ml_reconf(rdev: *mut cfg80211_registered_device, dev: *mut net_device, req: *mut cfg80211_ml_reconf_req) -> i32;
    fn cfg80211_mlo_reconf_add_done(dev: *mut net_device, data: *mut cfg80211_mlo_reconf_done_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
