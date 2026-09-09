// SPDX-License-Identifier: GPL-2.0-only
/*
 * HE handling
 *
 * Copyright(c) 2017 Intel Deutschland GmbH
 * Copyright(c) 2019-2026 Intel Corporation
 */

// Dependencies are supplied by the surrounding mac80211 translation unit.

unsafe fn ieee80211_update_from_he_6ghz_capa(
    he_6ghz_capa: *const ieee80211_he_6ghz_capa,
    link_sta: *mut link_sta_info,
) {
    let sta = (*link_sta).sta;
    let smps_mode;

    if (*(*sta).sdata).vif.r#type == NL80211_IFTYPE_AP ||
       (*(*sta).sdata).vif.r#type == NL80211_IFTYPE_AP_VLAN {
        smps_mode = match le16_get_bits((*he_6ghz_capa).capa, IEEE80211_HE_6GHZ_CAP_SM_PS) {
            WLAN_HT_CAP_SM_PS_INVALID | WLAN_HT_CAP_SM_PS_STATIC => IEEE80211_SMPS_STATIC,
            WLAN_HT_CAP_SM_PS_DYNAMIC => IEEE80211_SMPS_DYNAMIC,
            WLAN_HT_CAP_SM_PS_DISABLED => IEEE80211_SMPS_OFF,
            _ => unreachable!(),
        };
        (*(*link_sta).pub).smps_mode = smps_mode;
    } else {
        (*(*link_sta).pub).smps_mode = IEEE80211_SMPS_OFF;
    }

    (*(*link_sta).pub).agg.max_amsdu_len = match le16_get_bits((*he_6ghz_capa).capa,
                                                               IEEE80211_HE_6GHZ_CAP_MAX_MPDU_LEN) {
        IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_11454 => IEEE80211_MAX_MPDU_LEN_VHT_11454,
        IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_7991 => IEEE80211_MAX_MPDU_LEN_VHT_7991,
        IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_3895 | _ => IEEE80211_MAX_MPDU_LEN_VHT_3895,
    };
    ieee80211_sta_recalc_aggregates(&mut (*sta).sta);
    (*(*link_sta).pub).he_6ghz_capa = *he_6ghz_capa;
}

unsafe fn ieee80211_he_mcs_disable(he_mcs: *mut __le16) {
    for i in 0..8u32 {
        *he_mcs |= cpu_to_le16(IEEE80211_HE_MCS_NOT_SUPPORTED << (i * 2));
    }
}

unsafe fn ieee80211_he_mcs_intersection(
    he_own_rx: *mut __le16, he_peer_rx: *mut __le16,
    he_own_tx: *mut __le16, he_peer_tx: *mut __le16,
) {
    for i in 0..8u32 {
        let own_rx = (le16_to_cpu(*he_own_rx) >> (i * 2)) & IEEE80211_HE_MCS_NOT_SUPPORTED;
        let own_tx = (le16_to_cpu(*he_own_tx) >> (i * 2)) & IEEE80211_HE_MCS_NOT_SUPPORTED;
        let mut peer_rx = (le16_to_cpu(*he_peer_rx) >> (i * 2)) & IEEE80211_HE_MCS_NOT_SUPPORTED;
        let mut peer_tx = (le16_to_cpu(*he_peer_tx) >> (i * 2)) & IEEE80211_HE_MCS_NOT_SUPPORTED;

        if peer_tx != IEEE80211_HE_MCS_NOT_SUPPORTED {
            if own_rx == IEEE80211_HE_MCS_NOT_SUPPORTED { peer_tx = IEEE80211_HE_MCS_NOT_SUPPORTED; }
            else if own_rx < peer_tx { peer_tx = own_rx; }
        }
        if peer_rx != IEEE80211_HE_MCS_NOT_SUPPORTED {
            if own_tx == IEEE80211_HE_MCS_NOT_SUPPORTED { peer_rx = IEEE80211_HE_MCS_NOT_SUPPORTED; }
            else if own_tx < peer_rx { peer_rx = own_tx; }
        }
        *he_peer_rx &= !cpu_to_le16(IEEE80211_HE_MCS_NOT_SUPPORTED << (i * 2));
        *he_peer_rx |= cpu_to_le16(peer_rx << (i * 2));
        *he_peer_tx &= !cpu_to_le16(IEEE80211_HE_MCS_NOT_SUPPORTED << (i * 2));
        *he_peer_tx |= cpu_to_le16(peer_tx << (i * 2));
    }
}

pub unsafe fn _ieee80211_he_cap_ie_to_sta_he_cap(
    sdata: *mut ieee80211_sub_if_data,
    own_he_cap_ptr: *const ieee80211_sta_he_cap,
    he_cap_ie: *const u8, he_cap_len: u8,
    he_6ghz_capa: *const ieee80211_he_6ghz_capa,
    link_sta: *mut link_sta_info,
) {
    let he_cap = &mut (*(*link_sta).pub).he_cap;
    core::ptr::write_bytes(he_cap as *mut _, 0, 1);
    if he_cap_ie.is_null() || own_he_cap_ptr.is_null() || !(*own_he_cap_ptr).has_he { return; }
    if WARN_ON_ONCE((*sdata).vif.r#type == NL80211_IFTYPE_NAN_DATA) { return; }
    let own_he_cap = *own_he_cap_ptr;
    let elem = he_cap_ie as *const ieee80211_he_cap_elem;
    let mcs_nss_size = ieee80211_he_mcs_nss_size(elem);
    let he_ppe_size = ieee80211_he_ppe_size(he_cap_ie.add(core::mem::size_of_val(&he_cap.he_cap_elem) + mcs_nss_size), (*elem).phy_cap_info);
    let total = core::mem::size_of_val(&he_cap.he_cap_elem) + mcs_nss_size + he_ppe_size;
    if he_cap_len < total as u8 { return; }
    core::ptr::copy_nonoverlapping(he_cap_ie, &mut he_cap.he_cap_elem as *mut _ as *mut u8, core::mem::size_of_val(&he_cap.he_cap_elem));
    core::ptr::copy_nonoverlapping(he_cap_ie.add(core::mem::size_of_val(&he_cap.he_cap_elem)), &mut he_cap.he_mcs_nss_supp as *mut _ as *mut u8, mcs_nss_size);
    if he_cap.he_cap_elem.phy_cap_info[6] & IEEE80211_HE_PHY_CAP6_PPE_THRESHOLD_PRESENT != 0 {
        core::ptr::copy_nonoverlapping(he_cap_ie.add(core::mem::size_of_val(&he_cap.he_cap_elem) + mcs_nss_size), he_cap.ppe_thres.as_mut_ptr(), he_ppe_size);
    }
    he_cap.has_he = true;
    if !he_6ghz_capa.is_null() { ieee80211_update_from_he_6ghz_capa(he_6ghz_capa, link_sta); }
    ieee80211_he_mcs_intersection(&own_he_cap.he_mcs_nss_supp.rx_mcs_80 as *const _ as *mut _, &mut he_cap.he_mcs_nss_supp.rx_mcs_80, &own_he_cap.he_mcs_nss_supp.tx_mcs_80 as *const _ as *mut _, &mut he_cap.he_mcs_nss_supp.tx_mcs_80);
    let own_160 = own_he_cap.he_cap_elem.phy_cap_info[0] & IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_160MHZ_IN_5G;
    let peer_160 = he_cap.he_cap_elem.phy_cap_info[0] & IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_160MHZ_IN_5G;
    if peer_160 != 0 && own_160 != 0 { ieee80211_he_mcs_intersection(&own_he_cap.he_mcs_nss_supp.rx_mcs_160 as *const _ as *mut _, &mut he_cap.he_mcs_nss_supp.rx_mcs_160, &own_he_cap.he_mcs_nss_supp.tx_mcs_160 as *const _ as *mut _, &mut he_cap.he_mcs_nss_supp.tx_mcs_160); }
    else if peer_160 != 0 { ieee80211_he_mcs_disable(&mut he_cap.he_mcs_nss_supp.rx_mcs_160); ieee80211_he_mcs_disable(&mut he_cap.he_mcs_nss_supp.tx_mcs_160); he_cap.he_cap_elem.phy_cap_info[0] &= !IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_160MHZ_IN_5G; }
    let own_80p80 = own_he_cap.he_cap_elem.phy_cap_info[0] & IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_80PLUS80_MHZ_IN_5G;
    let peer_80p80 = he_cap.he_cap_elem.phy_cap_info[0] & IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_80PLUS80_MHZ_IN_5G;
    if peer_80p80 != 0 && own_80p80 != 0 { ieee80211_he_mcs_intersection(&own_he_cap.he_mcs_nss_supp.rx_mcs_80p80 as *const _ as *mut _, &mut he_cap.he_mcs_nss_supp.rx_mcs_80p80, &own_he_cap.he_mcs_nss_supp.tx_mcs_80p80 as *const _ as *mut _, &mut he_cap.he_mcs_nss_supp.tx_mcs_80p80); }
    else if peer_80p80 != 0 { ieee80211_he_mcs_disable(&mut he_cap.he_mcs_nss_supp.rx_mcs_80p80); ieee80211_he_mcs_disable(&mut he_cap.he_mcs_nss_supp.tx_mcs_80p80); he_cap.he_cap_elem.phy_cap_info[0] &= !IEEE80211_HE_PHY_CAP0_CHANNEL_WIDTH_SET_80PLUS80_MHZ_IN_5G; }
}

pub unsafe fn ieee80211_he_cap_ie_to_sta_he_cap(sdata: *mut ieee80211_sub_if_data, sband: *mut ieee80211_supported_band, he_cap_ie: *const u8, he_cap_len: u8, he_6ghz_capa: *const ieee80211_he_6ghz_capa, link_sta: *mut link_sta_info) {
    let own = ieee80211_get_he_iftype_cap_vif(sband, &(*sdata).vif);
    _ieee80211_he_cap_ie_to_sta_he_cap(sdata, own, he_cap_ie, he_cap_len, if (*sband).band == NL80211_BAND_6GHZ { he_6ghz_capa } else { core::ptr::null() }, link_sta);
}

pub unsafe fn ieee80211_he_op_ie_to_bss_conf(vif: *mut ieee80211_vif, he_op_ie: *const ieee80211_he_operation) {
    core::ptr::write_bytes(&mut (*vif).bss_conf.he_oper as *mut _, 0, 1);
    if he_op_ie.is_null() { return; }
    (*vif).bss_conf.he_oper.params = __le32_to_cpu((*he_op_ie).he_oper_params);
    (*vif).bss_conf.he_oper.nss_set = __le16_to_cpu((*he_op_ie).he_mcs_nss_set);
}

pub unsafe fn ieee80211_he_spr_ie_to_bss_conf(vif: *mut ieee80211_vif, elem: *const ieee80211_he_spr) {
    let pd = &mut (*vif).bss_conf.he_obss_pd;
    core::ptr::write_bytes(pd as *mut _, 0, 1);
    if elem.is_null() { return; }
    pd.sr_ctrl = (*elem).he_sr_control;
    let mut data = (*elem).optional;
    if pd.sr_ctrl & IEEE80211_HE_SPR_NON_SRG_OFFSET_PRESENT != 0 { pd.non_srg_max_offset = *data; data = data.add(1); }
    if pd.sr_ctrl & IEEE80211_HE_SPR_SRG_INFORMATION_PRESENT != 0 {
        pd.min_offset = *data; data = data.add(1); pd.max_offset = *data; data = data.add(1);
        core::ptr::copy_nonoverlapping(data, pd.bss_color_bitmap.as_mut_ptr(), 8); data = data.add(8);
        core::ptr::copy_nonoverlapping(data, pd.partial_bssid_bitmap.as_mut_ptr(), 8); pd.enable = true;
    }
}

pub unsafe fn ieee80211_prepare_rx_omi_bw(pub_link_sta: *mut ieee80211_link_sta, bw: ieee80211_sta_rx_bandwidth) -> bool {
    let sta = container_of((*pub_link_sta).sta, sta_info, sta);
    let local = (*(*sta).sdata).local;
    let link_sta = sdata_dereference((*sta).link[(*pub_link_sta).link_id], (*sta).sdata);
    let link = sdata_dereference((*(*sta).sdata).link[(*pub_link_sta).link_id], (*sta).sdata);
    if WARN_ON(link.is_null() || link_sta.is_null() || (*link_sta).pub != pub_link_sta) { return false; }
    let conf = sdata_dereference((*(*link).conf).chanctx_conf, (*sta).sdata);
    if WARN_ON(conf.is_null()) { return false; }
    trace_api_prepare_rx_omi_bw(local, (*sta).sdata, link_sta, bw);
    let chanctx = container_of(conf, ieee80211_chanctx, conf);
    let ret;
    if (*link_sta).rx_omi_bw_staging == bw { ret = false; }
    else if WARN_ON((*link_sta).rx_omi_bw_tx != (*link_sta).rx_omi_bw_staging || (*link_sta).rx_omi_bw_rx != (*link_sta).rx_omi_bw_staging) { ret = false; }
    else {
        if bw < (*link_sta).rx_omi_bw_staging { (*link_sta).rx_omi_bw_tx = bw; ieee80211_link_sta_update_rc_bw(link, link_sta); }
        else { (*link_sta).rx_omi_bw_rx = bw; ieee80211_recalc_chanctx_min_def(local, chanctx); }
        (*link_sta).rx_omi_bw_staging = bw; ret = true;
    }
    trace_api_return_bool(local, ret); ret
}

pub unsafe fn ieee80211_finalize_rx_omi_bw(pub_link_sta: *mut ieee80211_link_sta) {
    let sta = container_of((*pub_link_sta).sta, sta_info, sta);
    let local = (*(*sta).sdata).local;
    let link_sta = sdata_dereference((*sta).link[(*pub_link_sta).link_id], (*sta).sdata);
    let link = sdata_dereference((*(*sta).sdata).link[(*pub_link_sta).link_id], (*sta).sdata);
    if WARN_ON(link.is_null() || link_sta.is_null() || (*link_sta).pub != pub_link_sta) { return; }
    let conf = sdata_dereference((*(*link).conf).chanctx_conf, (*sta).sdata);
    if WARN_ON(conf.is_null()) { return; }
    trace_api_finalize_rx_omi_bw(local, (*sta).sdata, link_sta);
    let chanctx = container_of(conf, ieee80211_chanctx, conf);
    if (*link_sta).rx_omi_bw_tx != (*link_sta).rx_omi_bw_staging {
        WARN_ON((*link_sta).rx_omi_bw_tx > (*link_sta).rx_omi_bw_staging);
        (*link_sta).rx_omi_bw_tx = (*link_sta).rx_omi_bw_staging;
        ieee80211_link_sta_update_rc_bw(link, link_sta);
    }
    if (*link_sta).rx_omi_bw_rx != (*link_sta).rx_omi_bw_staging {
        WARN_ON((*link_sta).rx_omi_bw_rx < (*link_sta).rx_omi_bw_staging);
        (*link_sta).rx_omi_bw_rx = (*link_sta).rx_omi_bw_staging;
        ieee80211_recalc_chanctx_min_def(local, chanctx);
    }
    trace_api_return_void(local);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
