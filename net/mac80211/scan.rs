// SPDX-License-Identifier: GPL-2.0-only
/* Scanning implementation.  Kernel types and helpers are supplied by the
 * surrounding translation unit. */

const IEEE80211_PROBE_DELAY: usize = HZ / 33;
const IEEE80211_CHANNEL_TIME: usize = HZ / 33;
const IEEE80211_PASSIVE_CHANNEL_TIME: usize = HZ / 9;

pub unsafe fn ieee80211_rx_bss_put(local: *mut ieee80211_local, bss: *mut ieee80211_bss) {
    if bss.is_null() { return; }
    cfg80211_put_bss((*(*local).hw.wiphy), container_of(bss, cfg80211_bss, priv_));
}

unsafe fn is_uapsd_supported(elems: *mut ieee802_11_elems) -> bool {
    let qos_info: u8;
    if !(*elems).wmm_info.is_null() && (*elems).wmm_info_len == 7 && *(*elems).wmm_info.add(5) == 1 {
        qos_info = *(*elems).wmm_info.add(6);
    } else if !(*elems).wmm_param.is_null() && (*elems).wmm_param_len == 24 && *(*elems).wmm_param.add(5) == 1 {
        qos_info = *(*elems).wmm_param.add(6);
    } else { return false; }
    (qos_info & IEEE80211_WMM_IE_AP_QOSINFO_UAPSD) != 0
}

#[repr(C)]
pub struct inform_bss_update_data { pub rx_status: *mut ieee80211_rx_status, pub beacon: bool }

pub unsafe fn ieee80211_inform_bss(wiphy: *mut wiphy, cbss: *mut cfg80211_bss,
    ies: *const cfg80211_bss_ies, data: *mut core::ffi::c_void) {
    let local = wiphy_priv(wiphy);
    let update = data as *mut inform_bss_update_data;
    if update.is_null() { return; }
    let bss = &mut *((*cbss).priv_ as *mut ieee80211_bss);
    let e = ieee802_11_parse_elems((*ies).data, (*ies).len,
        if (*update).beacon { IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_BEACON } else { IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_PROBE_RESP }, core::ptr::null_mut());
    if e.is_null() { return; }
    let rs = (*update).rx_status;
    if (*update).beacon { bss.device_ts_beacon = (*rs).device_timestamp; } else { bss.device_ts_presp = (*rs).device_timestamp; }
    if (*e).parse_error { if (*update).beacon { bss.corrupt_data |= IEEE80211_BSS_CORRUPT_BEACON; } else { bss.corrupt_data |= IEEE80211_BSS_CORRUPT_PROBE_RESP; } }
    else if (*update).beacon { bss.corrupt_data &= !IEEE80211_BSS_CORRUPT_BEACON; } else { bss.corrupt_data &= !IEEE80211_BSS_CORRUPT_PROBE_RESP; }
    if !(*e).erp_info.is_null() && (!(*e).parse_error || (bss.valid_data & IEEE80211_BSS_VALID_ERP) == 0) {
        bss.erp_value = *(*e).erp_info; bss.has_erp_value = true;
        if !(*e).parse_error { bss.valid_data |= IEEE80211_BSS_VALID_ERP; }
    }
    if !(*e).parse_error || (bss.valid_data & IEEE80211_BSS_VALID_RATES) == 0 {
        let mut n = 0usize;
        if !(*e).supp_rates.is_null() { let k = core::cmp::min(IEEE80211_MAX_SUPP_RATES, (*e).supp_rates_len); memcpy(bss.supp_rates, (*e).supp_rates, k); n += k; }
        if !(*e).ext_supp_rates.is_null() { let k = core::cmp::min(IEEE80211_MAX_SUPP_RATES - n, (*e).ext_supp_rates_len); memcpy(bss.supp_rates.add(n), (*e).ext_supp_rates, k); n += k; }
        if n != 0 { bss.supp_rates_len = n; if !(*e).parse_error { bss.valid_data |= IEEE80211_BSS_VALID_RATES; } }
    }
    if !(*e).parse_error || (bss.valid_data & IEEE80211_BSS_VALID_WMM) == 0 {
        bss.wmm_used = !(*e).wmm_param.is_null() || !(*e).wmm_info.is_null();
        bss.uapsd_supported = is_uapsd_supported(e);
        if !(*e).parse_error { bss.valid_data |= IEEE80211_BSS_VALID_WMM; }
    }
    if (*update).beacon && (*rs).encoding != RX_ENC_HT && (*rs).encoding != RX_ENC_VHT {
        let band = (*(*local).hw.wiphy).bands[(*rs).band as usize];
        bss.beacon_rate = &mut (*band).bitrates[(*rs).rate_idx as usize];
    }
    bss.vht_cap_info = if !(*e).vht_cap_elem.is_null() { le32_to_cpu((*(*e).vht_cap_elem).vht_cap_info) } else { 0 };
    kfree(e as *mut core::ffi::c_void);
}

pub unsafe fn ieee80211_bss_info_update(local: *mut ieee80211_local, rs: *mut ieee80211_rx_status,
    mgmt: *mut ieee80211_mgmt, len: usize, channel: *mut ieee80211_channel) -> *mut ieee80211_bss {
    let beacon = ieee80211_is_beacon((*mgmt).frame_control) || ieee80211_is_s1g_beacon((*mgmt).frame_control);
    let mut update = inform_bss_update_data { rx_status: rs, beacon };
    let mut meta = cfg80211_inform_bss { boottime_ns: (*rs).boottime_ns, drv_data: &mut update as *mut _ as _, ..core::mem::zeroed() };
    if (*rs).flag & RX_FLAG_NO_SIGNAL_VAL != 0 { meta.signal = 0; }
    else if ieee80211_hw_check(&mut (*local).hw, SIGNAL_DBM) { meta.signal = (*rs).signal as i32 * 100; }
    else if ieee80211_hw_check(&mut (*local).hw, SIGNAL_UNSPEC) { meta.signal = ((*rs).signal as i32 * 100) / (*local).hw.max_signal; }
    meta.chan = channel;
    let cbss = cfg80211_inform_bss_frame_data((*local).hw.wiphy, &mut meta, mgmt, len, GFP_ATOMIC);
    if cbss.is_null() { return core::ptr::null_mut(); }
    if channel != (*cbss).channel { (*rs).flag |= RX_FLAG_NO_SIGNAL_VAL; }
    (*cbss).priv_ as *mut ieee80211_bss
}

pub unsafe fn ieee80211_scan_rx(local: *mut ieee80211_local, skb: *mut sk_buff) {
    let rs = IEEE80211_SKB_RXCB(skb); let mgmt = (*skb).data as *mut ieee80211_mgmt;
    if !ieee80211_is_probe_resp((*mgmt).frame_control) && !ieee80211_is_beacon((*mgmt).frame_control) && !ieee80211_is_s1g_beacon((*mgmt).frame_control) { return; }
    let channel = ieee80211_get_channel_khz((*local).hw.wiphy, ieee80211_rx_status_to_khz(rs));
    if channel.is_null() || (*channel).flags & IEEE80211_CHAN_DISABLED != 0 { return; }
    if (*local).open_count == (*local).monitors { return; }
    let bss = ieee80211_bss_info_update(local, rs, mgmt, (*skb).len, channel);
    if !bss.is_null() { ieee80211_rx_bss_put(local, bss); }
}

pub unsafe fn ieee80211_scan_completed(hw: *mut ieee80211_hw, info: *mut cfg80211_scan_info) {
    let local = hw_to_local(hw); trace_api_scan_completed(local, (*info).aborted);
    set_bit(SCAN_COMPLETED, &mut (*local).scanning);
    if (*info).aborted { set_bit(SCAN_ABORTED, &mut (*local).scanning); }
    memcpy(&mut (*local).scan_info as *mut _, info as *const _, core::mem::size_of::<cfg80211_scan_info>());
    wiphy_delayed_work_queue((*local).hw.wiphy, &mut (*local).scan_work, 0);
}

pub unsafe fn ieee80211_request_scan(sdata: *mut ieee80211_sub_if_data, req: *mut cfg80211_scan_request) -> i32 { __ieee80211_start_scan(sdata, req) }
pub unsafe fn ieee80211_sched_scan_results(hw: *mut ieee80211_hw) { let local = hw_to_local(hw); trace_api_sched_scan_results(local); cfg80211_sched_scan_results((*hw).wiphy, 0); }

// The remaining scan state-machine entry points retain their C ABI and are
// provided by the kernel translation environment.
extern "C" {
    fn __ieee80211_start_scan(sdata: *mut ieee80211_sub_if_data, req: *mut cfg80211_scan_request) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
