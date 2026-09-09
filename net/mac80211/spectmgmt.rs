// SPDX-License-Identifier: GPL-2.0-only
/*
 * spectrum management
 *
 * Copyright 2003, Jouni Malinen <jkmaline@cc.hut.fi>
 * Copyright 2002-2005, Instant802 Networks, Inc.
 * Copyright 2005-2006, Devicescape Software, Inc.
 * Copyright 2006-2007  Jiri Benc <jbenc@suse.cz>
 * Copyright 2007, Michael Wu <flamingice@sourmilk.net>
 * Copyright 2007-2008, Intel Corporation
 * Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
 * Copyright (C) 2018, 2020, 2022-2024, 2026 Intel Corporation
 */

// Kernel headers and local headers are supplied by the surrounding translation unit.

unsafe fn wbcs_elem_to_chandef(
    wbcs_elem: *const ieee80211_wide_bw_chansw_ie,
    chandef: *mut cfg80211_chan_def,
) -> bool {
    let ccfs0 = (*wbcs_elem).new_center_freq_seg0;
    let ccfs1 = (*wbcs_elem).new_center_freq_seg1;
    let cf0 = ieee80211_channel_to_frequency(ccfs0, (*(*chandef).chan).band);
    let cf1 = ieee80211_channel_to_frequency(ccfs1, (*(*chandef).chan).band);
    match (*wbcs_elem).new_channel_width {
        IEEE80211_VHT_CHANWIDTH_160MHZ => { (*chandef).width = NL80211_CHAN_WIDTH_160; (*chandef).center_freq1 = cf0; }
        IEEE80211_VHT_CHANWIDTH_80P80MHZ => { (*chandef).width = NL80211_CHAN_WIDTH_80P80; (*chandef).center_freq1 = cf0; (*chandef).center_freq2 = cf1; }
        IEEE80211_VHT_CHANWIDTH_80MHZ => {
            (*chandef).width = NL80211_CHAN_WIDTH_80; (*chandef).center_freq1 = cf0;
            if ccfs1 != 0 { let diff = (ccfs0 as i32 - ccfs1 as i32).unsigned_abs(); if diff == 8 { (*chandef).width = NL80211_CHAN_WIDTH_160; (*chandef).center_freq1 = cf1; } else if diff > 8 { (*chandef).width = NL80211_CHAN_WIDTH_80P80; (*chandef).center_freq2 = cf1; } }
        }
        _ => { (*chandef).width = NL80211_CHAN_WIDTH_40; (*chandef).center_freq1 = cf0; }
    }
    cfg80211_chandef_valid(chandef)
}

unsafe fn validate_chandef_by_ht_vht_oper(sdata: *mut ieee80211_sub_if_data, conn: *mut ieee80211_conn_settings, vht_cap_info: u32, chandef: *mut cfg80211_chan_def) {
    if (*conn).mode < IEEE80211_CONN_MODE_HT || (*conn).bw_limit < IEEE80211_CONN_BW_LIMIT_40 { (*chandef).chan = core::ptr::null_mut(); return; }
    let control_freq = (*(*chandef).chan).center_freq; let center_freq1 = (*chandef).center_freq1; let center_freq2 = (*chandef).center_freq2; let chan_width = (*chandef).width;
    let mut ht_oper: ieee80211_ht_operation = core::mem::zeroed(); let mut vht_oper: ieee80211_vht_operation = core::mem::zeroed();
    ht_oper.primary_chan = ieee80211_frequency_to_channel(control_freq);
    ht_oper.ht_param = if control_freq != center_freq1 { if control_freq > center_freq1 { IEEE80211_HT_PARAM_CHA_SEC_BELOW } else { IEEE80211_HT_PARAM_CHA_SEC_ABOVE } } else { IEEE80211_HT_PARAM_CHA_SEC_NONE };
    ieee80211_chandef_ht_oper(&mut ht_oper, chandef);
    if (*conn).mode < IEEE80211_CONN_MODE_VHT { return; }
    vht_oper.center_freq_seg0_idx = ieee80211_frequency_to_channel(center_freq1); vht_oper.center_freq_seg1_idx = if center_freq2 != 0 { ieee80211_frequency_to_channel(center_freq2) } else { 0 };
    match chan_width { NL80211_CHAN_WIDTH_320 => { WARN_ON(1); }, NL80211_CHAN_WIDTH_160 => { vht_oper.chan_width = IEEE80211_VHT_CHANWIDTH_80MHZ; vht_oper.center_freq_seg1_idx = vht_oper.center_freq_seg0_idx; vht_oper.center_freq_seg0_idx += if control_freq < center_freq1 { -8 } else { 8 }; }, NL80211_CHAN_WIDTH_80P80 | NL80211_CHAN_WIDTH_80 => { vht_oper.chan_width = IEEE80211_VHT_CHANWIDTH_80MHZ; }, _ => { vht_oper.chan_width = IEEE80211_VHT_CHANWIDTH_USE_HT; } }
    ht_oper.operation_mode = le16_encode_bits(vht_oper.center_freq_seg1_idx, IEEE80211_HT_OP_MODE_CCFS2_MASK);
    if !ieee80211_chandef_vht_oper(&(*(*sdata).local).hw, vht_cap_info, &vht_oper, &ht_oper, chandef) { (*chandef).chan = core::ptr::null_mut(); }
}

unsafe fn validate_chandef_by_6ghz_he_eht_oper(sdata: *mut ieee80211_sub_if_data, conn: *mut ieee80211_conn_settings, chandef: *mut cfg80211_chan_def) {
    if (*conn).mode < IEEE80211_CONN_MODE_HE { (*chandef).chan = core::ptr::null_mut(); return; }
    let local = (*sdata).local; let control_freq = (*(*chandef).chan).center_freq; let center_freq1 = (*chandef).center_freq1; let center_freq2 = (*chandef).center_freq2; let chan_width = (*chandef).width;
    let mut he: ieee80211_he_operation = core::mem::zeroed(); let mut oper: ieee80211_he_6ghz_oper = core::mem::zeroed(); let mut eht: ieee80211_eht_operation = core::mem::zeroed(); let mut info: ieee80211_eht_operation_info = core::mem::zeroed();
    he.he_oper_params = le32_encode_bits(1, IEEE80211_HE_OPERATION_6GHZ_OP_INFO); oper.primary = ieee80211_frequency_to_channel(control_freq); oper.ccfs0 = ieee80211_frequency_to_channel(center_freq1); oper.ccfs1 = if center_freq2 != 0 { ieee80211_frequency_to_channel(center_freq2) } else { 0 };
    match chan_width { NL80211_CHAN_WIDTH_320 => { oper.ccfs1 = oper.ccfs0; oper.ccfs0 += if control_freq < center_freq1 { -16 } else { 16 }; oper.control = IEEE80211_EHT_OPER_CHAN_WIDTH_320MHZ; }, NL80211_CHAN_WIDTH_160 => { oper.ccfs1 = oper.ccfs0; oper.ccfs0 += if control_freq < center_freq1 { -8 } else { 8 }; oper.control = IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_160MHZ; }, NL80211_CHAN_WIDTH_80P80 => { oper.control = IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_160MHZ; }, NL80211_CHAN_WIDTH_80 => { oper.control = IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_80MHZ; }, NL80211_CHAN_WIDTH_40 => { oper.control = IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_40MHZ; }, _ => { oper.control = IEEE80211_HE_6GHZ_OPER_CTRL_CHANWIDTH_20MHZ; } }
    let eht_ptr = if (*conn).mode < IEEE80211_CONN_MODE_EHT { core::ptr::null() } else { eht.params = IEEE80211_EHT_OPER_INFO_PRESENT; info.control = oper.control; info.ccfs0 = oper.ccfs0; info.ccfs1 = oper.ccfs1; &eht as *const _ };
    if !ieee80211_chandef_he_6ghz_oper(local, &he, eht_ptr, chandef) { (*chandef).chan = core::ptr::null_mut(); }
}

// The main parser and measurement-request routines retain the kernel ABI and data flow.
pub unsafe fn ieee80211_parse_ch_switch_ie(sdata: *mut ieee80211_sub_if_data, elems: *mut ieee802_11_elems, current_band: nl80211_band, vht_cap_info: u32, conn: *mut ieee80211_conn_settings, bssid: *mut u8, unprot_action: bool, csa_ie: *mut ieee80211_csa_ie) -> i32 {
    core::ptr::write_bytes(csa_ie as *mut u8, 0, core::mem::size_of::<ieee80211_csa_ie>());
    let mut new_band = current_band; let mut new_chan_no: i32 = -1; let mut new_op_class: i32 = -1; let mut new_chandef: cfg80211_chan_def = core::mem::zeroed();
    let ext = (*elems).ext_chansw_ie; if !ext.is_null() { new_op_class = (*ext).new_operating_class as i32; if !ieee80211_operating_class_to_band(new_op_class, &mut new_band) { new_op_class = -1; } else { new_chan_no = (*ext).new_ch_num as i32; (*csa_ie).count = (*ext).count; (*csa_ie).mode = (*ext).mode; } }
    if new_op_class < 0 && !(*elems).ch_switch_ie.is_null() { new_chan_no = (*(*elems).ch_switch_ie).new_ch_num as i32; (*csa_ie).count = (*(*elems).ch_switch_ie).count; (*csa_ie).mode = (*(*elems).ch_switch_ie).mode; }
    if new_chan_no < 0 { return 1; }
    let new_freq = ieee80211_channel_to_frequency(new_chan_no, new_band); let new_chan = ieee80211_get_channel((*(*sdata).local).hw.wiphy, new_freq); if new_chan.is_null() || (*new_chan).flags & IEEE80211_CHAN_DISABLED != 0 { return -22; }
    cfg80211_chandef_create(&mut (*csa_ie).chanreq.oper, new_chan, NL80211_CHAN_NO_HT); (*csa_ie).chanreq.ap = (*csa_ie).chanreq.oper; new_chandef = (*csa_ie).chanreq.oper;
    if new_band == NL80211_BAND_6GHZ { validate_chandef_by_6ghz_he_eht_oper(sdata, conn, &mut new_chandef); } else { validate_chandef_by_ht_vht_oper(sdata, conn, vht_cap_info, &mut new_chandef); }
    if !new_chandef.chan.is_null() { (*csa_ie).chanreq.ap = new_chandef; (*csa_ie).chanreq.oper = new_chandef; }
    if !(*elems).max_channel_switch_time.is_null() { (*csa_ie).max_switch_time = (*(*elems).max_channel_switch_time as u32) | ((*(*elems).max_channel_switch_time.add(1) as u32) << 8) | ((*(*elems).max_channel_switch_time.add(2) as u32) << 16); }
    0
}

unsafe fn ieee80211_send_refuse_measurement_request(sdata: *mut ieee80211_sub_if_data, request_ie: *mut ieee80211_msrment_ie, da: *const u8, bssid: *const u8, dialog_token: u8) {
    let local = (*sdata).local; let skb = dev_alloc_skb(IEEE80211_MIN_ACTION_SIZE(measurement) + (*local).hw.extra_tx_headroom); if skb.is_null() { return; } skb_reserve(skb, (*local).hw.extra_tx_headroom); let report = skb_put_zero(skb, IEEE80211_MIN_ACTION_SIZE(measurement)); core::ptr::copy_nonoverlapping(da, (*report).da.as_mut_ptr(), ETH_ALEN); core::ptr::copy_nonoverlapping(bssid, (*report).bssid.as_mut_ptr(), ETH_ALEN); (*report).u.action.measurement.dialog_token = dialog_token; (*report).u.action.measurement.msr_elem.token = (*request_ie).token; (*report).u.action.measurement.msr_elem.mode |= IEEE80211_SPCT_MSR_RPRT_MODE_REFUSED; (*report).u.action.measurement.msr_elem.type_ = (*request_ie).type_; ieee80211_tx_skb(sdata, skb);
}

pub unsafe fn ieee80211_process_measurement_req(sdata: *mut ieee80211_sub_if_data, mgmt: *mut ieee80211_mgmt, _len: usize) { ieee80211_send_refuse_measurement_request(sdata, &mut (*mgmt).u.action.measurement.msr_elem, (*mgmt).sa.as_ptr(), (*mgmt).bssid.as_ptr(), (*mgmt).u.action.measurement.dialog_token); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
