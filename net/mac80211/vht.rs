// SPDX-License-Identifier: GPL-2.0-only
/*
 * VHT handling
 *
 * Portions of this file
 * Copyright(c) 2015 - 2016 Intel Deutschland GmbH
 * Copyright (C) 2018 - 2026 Intel Corporation
 */

unsafe fn __check_vhtcap_disable(
    sdata: *mut ieee80211_sub_if_data,
    vht_cap: *mut ieee80211_sta_vht_cap,
    flag: u32,
) {
    let le_flag = cpu_to_le32(flag);

    if ((*sdata).u_.mgd.vht_capa_mask.vht_cap_info & le_flag) != 0
        && ((*sdata).u_.mgd.vht_capa.vht_cap_info & le_flag) == 0
    {
        (*vht_cap).cap &= !flag;
    }
}

pub unsafe fn ieee80211_apply_vhtcap_overrides(
    sdata: *mut ieee80211_sub_if_data,
    vht_cap: *mut ieee80211_sta_vht_cap,
) {
    if !(*vht_cap).vht_supported || (*sdata).vif.type_ != NL80211_IFTYPE_STATION {
        return;
    }

    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_RXLDPC);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_SHORT_GI_80);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_SHORT_GI_160);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_TXSTBC);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_SU_BEAMFORMER_CAPABLE);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_SU_BEAMFORMEE_CAPABLE);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_RX_ANTENNA_PATTERN);
    __check_vhtcap_disable(sdata, vht_cap, IEEE80211_VHT_CAP_TX_ANTENNA_PATTERN);

    if ((*sdata).u_.mgd.vht_capa_mask.vht_cap_info
        & cpu_to_le32(IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK)) != 0
    {
        let mut n = le32_to_cpu((*sdata).u_.mgd.vht_capa.vht_cap_info)
            & IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK;
        n >>= IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_SHIFT;
        let mut cap = (*vht_cap).cap & IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK;
        cap >>= IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_SHIFT;
        if n < cap {
            (*vht_cap).cap &= !IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK;
            (*vht_cap).cap |= n << IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_SHIFT;
        }
    }

    let rxmcs_mask = le16_to_cpu((*sdata).u_.mgd.vht_capa_mask.supp_mcs.rx_mcs_map);
    let mut rxmcs_n = le16_to_cpu((*sdata).u_.mgd.vht_capa.supp_mcs.rx_mcs_map);
    rxmcs_n &= rxmcs_mask;
    let mut rxmcs_cap = le16_to_cpu((*vht_cap).vht_mcs.rx_mcs_map);
    let txmcs_mask = le16_to_cpu((*sdata).u_.mgd.vht_capa_mask.supp_mcs.tx_mcs_map);
    let mut txmcs_n = le16_to_cpu((*sdata).u_.mgd.vht_capa.supp_mcs.tx_mcs_map);
    txmcs_n &= txmcs_mask;
    let mut txmcs_cap = le16_to_cpu((*vht_cap).vht_mcs.tx_mcs_map);
    for i in 0..8 {
        let shift = 2 * i;
        let m = (rxmcs_mask >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let n = (rxmcs_n >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let c = (rxmcs_cap >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        if m != 0 && ((c != IEEE80211_VHT_MCS_NOT_SUPPORTED && n < c)
            || n == IEEE80211_VHT_MCS_NOT_SUPPORTED)
        {
            rxmcs_cap &= !(3 << shift);
            rxmcs_cap |= rxmcs_n & (3 << shift);
        }
        let m = (txmcs_mask >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let n = (txmcs_n >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let c = (txmcs_cap >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        if m != 0 && ((c != IEEE80211_VHT_MCS_NOT_SUPPORTED && n < c)
            || n == IEEE80211_VHT_MCS_NOT_SUPPORTED)
        {
            txmcs_cap &= !(3 << shift);
            txmcs_cap |= txmcs_n & (3 << shift);
        }
    }
    (*vht_cap).vht_mcs.rx_mcs_map = cpu_to_le16(rxmcs_cap);
    (*vht_cap).vht_mcs.tx_mcs_map = cpu_to_le16(txmcs_cap);
}

pub unsafe fn ieee80211_vht_cap_ie_to_sta_vht_cap(
    sdata: *mut ieee80211_sub_if_data,
    sband: *mut ieee80211_supported_band,
    own_vht_cap: *const ieee80211_sta_vht_cap,
    vht_cap_ie: *const ieee80211_vht_cap,
    vht_cap_ie2: *const ieee80211_vht_cap,
    link_sta: *mut link_sta_info,
) {
    let vht_cap = &mut (*(*link_sta).pub_).vht_cap;
    let mut own_cap: ieee80211_sta_vht_cap;
    let mut cap_info: u32;
    let mut i: u32;
    let mut mpdu_len: u32;

    memset(vht_cap as *mut _ as *mut u8, 0, core::mem::size_of::<ieee80211_sta_vht_cap>());
    if !(*(*link_sta).pub_).ht_cap.ht_supported || vht_cap_ie.is_null() || !(*own_vht_cap).vht_supported {
        return;
    }
    if WARN_ON_ONCE((*sdata).vif.type_ == NL80211_IFTYPE_NAN_DATA) { return; }
    if !sband.is_null() {
        let mut have_80mhz = false;
        for j in 0..(*sband).n_channels {
            let ch = (*sband).channels.add(j as usize);
            if ((*ch).flags & (IEEE80211_CHAN_DISABLED | IEEE80211_CHAN_NO_80MHZ)) != 0 { continue; }
            have_80mhz = true; break;
        }
        if !have_80mhz { return; }
    }
    vht_cap.vht_supported = true;
    own_cap = *own_vht_cap;
    if (*sdata).vif.type_ == NL80211_IFTYPE_STATION && !test_sta_flag((*link_sta).sta, WLAN_STA_TDLS_PEER) {
        ieee80211_apply_vhtcap_overrides(sdata, &mut own_cap);
    }
    cap_info = le32_to_cpu((*vht_cap_ie).vht_cap_info);
    vht_cap.cap = cap_info & (IEEE80211_VHT_CAP_RXLDPC | IEEE80211_VHT_CAP_VHT_TXOP_PS |
        IEEE80211_VHT_CAP_HTC_VHT | IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK |
        IEEE80211_VHT_CAP_VHT_LINK_ADAPTATION_VHT_UNSOL_MFB |
        IEEE80211_VHT_CAP_VHT_LINK_ADAPTATION_VHT_MRQ_MFB |
        IEEE80211_VHT_CAP_RX_ANTENNA_PATTERN | IEEE80211_VHT_CAP_TX_ANTENNA_PATTERN);
    vht_cap.cap |= core::cmp::min(cap_info & IEEE80211_VHT_CAP_MAX_MPDU_MASK,
        own_cap.cap & IEEE80211_VHT_CAP_MAX_MPDU_MASK);
    match own_cap.cap & IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_MASK {
        IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160MHZ => vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160MHZ,
        IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160_80PLUS80MHZ => vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_MASK,
        _ => {}
    }
    vht_cap.cap |= cap_info & own_cap.cap & (IEEE80211_VHT_CAP_SHORT_GI_80 | IEEE80211_VHT_CAP_SHORT_GI_160);
    if own_cap.cap & IEEE80211_VHT_CAP_SU_BEAMFORMEE_CAPABLE != 0 { vht_cap.cap |= cap_info & (IEEE80211_VHT_CAP_SU_BEAMFORMER_CAPABLE | IEEE80211_VHT_CAP_SOUNDING_DIMENSIONS_MASK); }
    if own_cap.cap & IEEE80211_VHT_CAP_SU_BEAMFORMER_CAPABLE != 0 { vht_cap.cap |= cap_info & (IEEE80211_VHT_CAP_SU_BEAMFORMEE_CAPABLE | IEEE80211_VHT_CAP_BEAMFORMEE_STS_MASK); }
    if own_cap.cap & IEEE80211_VHT_CAP_MU_BEAMFORMER_CAPABLE != 0 { vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_MU_BEAMFORMEE_CAPABLE; }
    if own_cap.cap & IEEE80211_VHT_CAP_MU_BEAMFORMEE_CAPABLE != 0 { vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_MU_BEAMFORMER_CAPABLE; }
    if own_cap.cap & IEEE80211_VHT_CAP_TXSTBC != 0 { vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_RXSTBC_MASK; }
    if own_cap.cap & IEEE80211_VHT_CAP_RXSTBC_MASK != 0 { vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_TXSTBC; }
    memcpy(&mut vht_cap.vht_mcs as *mut _ as *mut u8, &(*vht_cap_ie).supp_mcs as *const _ as *const u8, core::mem::size_of::<ieee80211_vht_mcs_info>());
    if ieee80211_hw_check(&(*(*sdata).local).hw, SUPPORTS_VHT_EXT_NSS_BW) { vht_cap.cap |= cap_info & IEEE80211_VHT_CAP_EXT_NSS_BW_MASK; }
    else { vht_cap.vht_mcs.tx_highest &= !cpu_to_le16(IEEE80211_VHT_EXT_NSS_BW_CAPABLE); }
    for j in 0..8 {
        i = j;
        let shift = i * 2;
        let own_rx = (le16_to_cpu(own_cap.vht_mcs.rx_mcs_map) >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let own_tx = (le16_to_cpu(own_cap.vht_mcs.tx_mcs_map) >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let mut peer_rx = (le16_to_cpu(vht_cap.vht_mcs.rx_mcs_map) >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        let mut peer_tx = (le16_to_cpu(vht_cap.vht_mcs.tx_mcs_map) >> shift) & IEEE80211_VHT_MCS_NOT_SUPPORTED;
        if peer_tx != IEEE80211_VHT_MCS_NOT_SUPPORTED { if own_rx == IEEE80211_VHT_MCS_NOT_SUPPORTED { peer_tx = IEEE80211_VHT_MCS_NOT_SUPPORTED; } else if own_rx < peer_tx { peer_tx = own_rx; } }
        if peer_rx != IEEE80211_VHT_MCS_NOT_SUPPORTED { if own_tx == IEEE80211_VHT_MCS_NOT_SUPPORTED { peer_rx = IEEE80211_VHT_MCS_NOT_SUPPORTED; } else if own_tx < peer_rx { peer_rx = own_tx; } }
        vht_cap.vht_mcs.rx_mcs_map &= !cpu_to_le16(IEEE80211_VHT_MCS_NOT_SUPPORTED << shift);
        vht_cap.vht_mcs.rx_mcs_map |= cpu_to_le16(peer_rx << shift);
        vht_cap.vht_mcs.tx_mcs_map &= !cpu_to_le16(IEEE80211_VHT_MCS_NOT_SUPPORTED << shift);
        vht_cap.vht_mcs.tx_mcs_map |= cpu_to_le16(peer_tx << shift);
    }
    if vht_cap.vht_mcs.rx_mcs_map == cpu_to_le16(0xFFFF) {
        vht_cap.vht_supported = false;
        sdata_info(sdata, "Ignoring VHT IE from %pM (link:%pM) due to invalid rx_mcs_map\n", (*link_sta).sta.addr, (*link_sta).addr);
        return;
    }
    mpdu_len = vht_cap.cap & IEEE80211_VHT_CAP_MAX_MPDU_MASK;
    if !vht_cap_ie2.is_null() { mpdu_len = core::cmp::min(mpdu_len, le32_get_bits((*vht_cap_ie2).vht_cap_info, IEEE80211_VHT_CAP_MAX_MPDU_MASK)); }
    (*(*link_sta).pub_).agg.max_amsdu_len = match mpdu_len {
        IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_11454 => IEEE80211_MAX_MPDU_LEN_VHT_11454,
        IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_7991 => IEEE80211_MAX_MPDU_LEN_VHT_7991,
        _ => IEEE80211_MAX_MPDU_LEN_VHT_3895,
    };
    ieee80211_sta_recalc_aggregates(&mut (*link_sta).sta.sta);
}

pub unsafe fn __ieee80211_vht_handle_opmode(sdata: *mut ieee80211_sub_if_data, link_sta: *mut link_sta_info, opmode: u8, band: nl80211_band) -> u32 {
    let mut changed = 0;
    let mut sta_opmode: sta_opmode_info = core::mem::zeroed();
    let link = sdata_dereference((*sdata).link[(*link_sta).link_id], sdata);
    if WARN_ON(link.is_null()) { return 0; }
    if opmode & IEEE80211_OPMODE_NOTIF_RX_NSS_TYPE_BF != 0 { return 0; }
    let nss = ((opmode & IEEE80211_OPMODE_NOTIF_RX_NSS_MASK) >> IEEE80211_OPMODE_NOTIF_RX_NSS_SHIFT) + 1;
    if (*link_sta).op_mode_nss != nss {
        if nss <= (*link_sta).capa_nss { (*link_sta).op_mode_nss = nss; if nss != (*(*link_sta).pub_).rx_nss { (*(*link_sta).pub_).rx_nss = nss; changed |= IEEE80211_RC_NSS_CHANGED; sta_opmode.rx_nss = nss; sta_opmode.changed |= STA_OPMODE_N_SS_CHANGED; } }
        else { sdata_dbg(sdata, "Ignore NSS change to invalid %d in VHT opmode notif from %pM", nss, (*(*link_sta).pub_).addr); }
    }
    match opmode & IEEE80211_OPMODE_NOTIF_CHANWIDTH_MASK {
        IEEE80211_OPMODE_NOTIF_CHANWIDTH_20MHZ => (*link_sta).op_mode_bw = IEEE80211_STA_RX_BW_20,
        IEEE80211_OPMODE_NOTIF_CHANWIDTH_40MHZ => (*link_sta).op_mode_bw = IEEE80211_STA_RX_BW_40,
        IEEE80211_OPMODE_NOTIF_CHANWIDTH_80MHZ => (*link_sta).op_mode_bw = if opmode & IEEE80211_OPMODE_NOTIF_BW_160_80P80 != 0 { IEEE80211_STA_RX_BW_160 } else { IEEE80211_STA_RX_BW_80 },
        IEEE80211_OPMODE_NOTIF_CHANWIDTH_160MHZ => (*link_sta).op_mode_bw = IEEE80211_STA_RX_BW_160,
        _ => {}
    }
    let new_bw = ieee80211_sta_current_bw(link_sta, &(*(*link).conf).chanreq.oper, IEEE80211_STA_BW_TX_TO_STA);
    if new_bw != (*(*link_sta).pub_).bandwidth { (*(*link_sta).pub_).bandwidth = new_bw; sta_opmode.bw = ieee80211_sta_rx_bw_to_chan_width(new_bw); changed |= IEEE80211_RC_BW_CHANGED; sta_opmode.changed |= STA_OPMODE_MAX_BW_CHANGED; }
    if sta_opmode.changed != 0 { cfg80211_sta_opmode_change_notify((*sdata).dev, (*link_sta).addr, &sta_opmode, GFP_KERNEL); }
    changed
}

pub unsafe fn ieee80211_process_mu_groups(sdata: *mut ieee80211_sub_if_data, link: *mut ieee80211_link_data, mgmt: *mut ieee80211_mgmt) {
    let link_conf = (*link).conf;
    if !(*link_conf).mu_mimo_owner { return; }
    if memcmp((*mgmt).u.action.vht_group_notif.position.as_ptr(), (*link_conf).mu_group.position.as_ptr(), WLAN_USER_POSITION_LEN) == 0 && memcmp((*mgmt).u.action.vht_group_notif.membership.as_ptr(), (*link_conf).mu_group.membership.as_ptr(), WLAN_MEMBERSHIP_LEN) == 0 { return; }
    memcpy((*link_conf).mu_group.membership.as_mut_ptr(), (*mgmt).u.action.vht_group_notif.membership.as_ptr(), WLAN_MEMBERSHIP_LEN);
    memcpy((*link_conf).mu_group.position.as_mut_ptr(), (*mgmt).u.action.vht_group_notif.position.as_ptr(), WLAN_USER_POSITION_LEN);
    ieee80211_link_info_change_notify(sdata, link, BSS_CHANGED_MU_GROUPS);
}

pub unsafe fn ieee80211_update_mu_groups(vif: *mut ieee80211_vif, link_id: u32, membership: *const u8, position: *const u8) {
    rcu_read_lock();
    let link_conf = rcu_dereference((*vif).link_conf[link_id as usize]);
    if !WARN_ON_ONCE(link_conf.is_null() || !(*link_conf).mu_mimo_owner) {
        memcpy((*link_conf).mu_group.membership.as_mut_ptr(), membership, WLAN_MEMBERSHIP_LEN);
        memcpy((*link_conf).mu_group.position.as_mut_ptr(), position, WLAN_USER_POSITION_LEN);
    }
    rcu_read_unlock();
}

pub unsafe fn ieee80211_vht_handle_opmode(sdata: *mut ieee80211_sub_if_data, link_sta: *mut link_sta_info, opmode: u8, band: nl80211_band) {
    let local = (*sdata).local;
    let sband = (*(*local).hw.wiphy).bands[band as usize];
    let changed = __ieee80211_vht_handle_opmode(sdata, link_sta, opmode, band);
    if changed > 0 { ieee80211_recalc_min_chandef(sdata, (*link_sta).link_id); rate_control_rate_update(local, sband, link_sta, changed); }
}

pub unsafe fn ieee80211_get_vht_mask_from_cap(vht_cap: __le16, vht_mask: *mut u16) {
    let cap = le16_to_cpu(vht_cap);
    for i in 0..NL80211_VHT_NSS_MAX {
        vht_mask.add(i as usize).write(match (cap >> (i * 2)) & IEEE80211_VHT_MCS_NOT_SUPPORTED {
            IEEE80211_VHT_MCS_SUPPORT_0_7 => 0x00FF,
            IEEE80211_VHT_MCS_SUPPORT_0_8 => 0x01FF,
            IEEE80211_VHT_MCS_SUPPORT_0_9 => 0x03FF,
            _ => 0,
        });
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
