// SPDX-License-Identifier: GPL-2.0-only
/* HT handling; translated literally from ht.c. */

unsafe fn __check_htcap_disable(ht_capa: *mut ieee80211_ht_cap, ht_capa_mask: *mut ieee80211_ht_cap, ht_cap: *mut ieee80211_sta_ht_cap, flag: u16) {
    let le_flag = cpu_to_le16(flag);
    if (*ht_capa_mask).cap_info & le_flag != 0 && (*ht_capa).cap_info & le_flag == 0 { (*ht_cap).cap &= !flag; }
}

unsafe fn __check_htcap_enable(ht_capa: *mut ieee80211_ht_cap, ht_capa_mask: *mut ieee80211_ht_cap, ht_cap: *mut ieee80211_sta_ht_cap, flag: u16) {
    let le_flag = cpu_to_le16(flag);
    if (*ht_capa_mask).cap_info & le_flag != 0 && (*ht_capa).cap_info & le_flag != 0 { (*ht_cap).cap |= flag; }
}

pub unsafe fn ieee80211_apply_htcap_overrides(sdata: *mut ieee80211_sub_if_data, ht_cap: *mut ieee80211_sta_ht_cap) {
    if !(*ht_cap).ht_supported { return; }
    let (ht_capa, ht_capa_mask) = match (*sdata).vif.r#type {
        NL80211_IFTYPE_STATION => (&mut (*sdata).u.mgd.ht_capa as *mut _, &mut (*sdata).u.mgd.ht_capa_mask as *mut _),
        NL80211_IFTYPE_ADHOC => (&mut (*sdata).u.ibss.ht_capa as *mut _, &mut (*sdata).u.ibss.ht_capa_mask as *mut _),
        _ => { WARN_ON_ONCE(1); return; }
    };
    let scaps = (*ht_capa).mcs.rx_mask.as_ptr();
    let smask = (*ht_capa_mask).mcs.rx_mask.as_ptr();
    for i in 0..IEEE80211_HT_MCS_MASK_LEN {
        let m = *smask.add(i);
        (*ht_cap).mcs.rx_mask[i] &= !m;
        (*ht_cap).mcs.rx_mask[i] |= m & *scaps.add(i);
    }
    __check_htcap_disable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_SUP_WIDTH_20_40);
    __check_htcap_disable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_SGI_40);
    __check_htcap_disable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_SGI_20);
    __check_htcap_disable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_MAX_AMSDU);
    __check_htcap_disable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_LDPC_CODING);
    __check_htcap_enable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_40MHZ_INTOLERANT);
    __check_htcap_enable(ht_capa, ht_capa_mask, ht_cap, IEEE80211_HT_CAP_TX_STBC);
    if (*ht_capa_mask).cap_info & cpu_to_le16(IEEE80211_HT_CAP_RX_STBC) != 0 { (*ht_cap).cap |= le16_to_cpu((*ht_capa).cap_info) & IEEE80211_HT_CAP_RX_STBC; }
    if (*ht_capa_mask).ampdu_params_info & IEEE80211_HT_AMPDU_PARM_FACTOR != 0 { let n = (*ht_capa).ampdu_params_info & IEEE80211_HT_AMPDU_PARM_FACTOR; if n < (*ht_cap).ampdu_factor { (*ht_cap).ampdu_factor = n; } }
    if (*ht_capa_mask).ampdu_params_info & IEEE80211_HT_AMPDU_PARM_DENSITY != 0 { let n = ((*ht_capa).ampdu_params_info & IEEE80211_HT_AMPDU_PARM_DENSITY) >> IEEE80211_HT_AMPDU_PARM_DENSITY_SHIFT; if n > (*ht_cap).ampdu_density { (*ht_cap).ampdu_density = n; } }
}

pub unsafe fn ieee80211_ht_cap_ie_to_sta_ht_cap(sdata: *mut ieee80211_sub_if_data, own_cap_ptr: *const ieee80211_sta_ht_cap, ht_cap_ie: *const ieee80211_ht_cap, link_sta: *mut link_sta_info) -> bool {
    let sta = (*link_sta).sta;
    let mut ht_cap: ieee80211_sta_ht_cap = core::mem::zeroed();
    if ht_cap_ie.is_null() || !(*own_cap_ptr).ht_supported { goto_apply!(); }
    if WARN_ON_ONCE((*sdata).vif.r#type == NL80211_IFTYPE_NAN_DATA) { return false; }
    ht_cap.ht_supported = true;
    let mut own_cap = *own_cap_ptr;
    if (*sdata).vif.r#type == NL80211_IFTYPE_STATION || (*sdata).vif.r#type == NL80211_IFTYPE_ADHOC { ieee80211_apply_htcap_overrides(sdata, &mut own_cap); }
    ht_cap.cap = le16_to_cpu((*ht_cap_ie).cap_info) & (own_cap.cap | !(IEEE80211_HT_CAP_LDPC_CODING | IEEE80211_HT_CAP_SUP_WIDTH_20_40 | IEEE80211_HT_CAP_GRN_FLD | IEEE80211_HT_CAP_SGI_20 | IEEE80211_HT_CAP_SGI_40 | IEEE80211_HT_CAP_DSSSCCK40));
    if own_cap.cap & IEEE80211_HT_CAP_TX_STBC == 0 { ht_cap.cap &= !IEEE80211_HT_CAP_RX_STBC; }
    if own_cap.cap & IEEE80211_HT_CAP_RX_STBC == 0 { ht_cap.cap &= !IEEE80211_HT_CAP_TX_STBC; }
    let ampdu_info = (*ht_cap_ie).ampdu_params_info;
    ht_cap.ampdu_factor = ampdu_info & IEEE80211_HT_AMPDU_PARM_FACTOR;
    ht_cap.ampdu_density = (ampdu_info & IEEE80211_HT_AMPDU_PARM_DENSITY) >> 2;
    let tx_mcs_set_cap = own_cap.mcs.tx_params;
    ht_cap.mcs.tx_params = (*ht_cap_ie).mcs.tx_params;
    if tx_mcs_set_cap & IEEE80211_HT_MCS_TX_DEFINED == 0 { goto_apply!(); }
    let max_tx_streams = if tx_mcs_set_cap & IEEE80211_HT_MCS_TX_RX_DIFF != 0 { ((tx_mcs_set_cap & IEEE80211_HT_MCS_TX_MAX_STREAMS_MASK) >> IEEE80211_HT_MCS_TX_MAX_STREAMS_SHIFT) + 1 } else { IEEE80211_HT_MCS_TX_MAX_STREAMS };
    for i in 0..max_tx_streams { ht_cap.mcs.rx_mask[i] = own_cap.mcs.rx_mask[i] & (*ht_cap_ie).mcs.rx_mask[i]; }
    if tx_mcs_set_cap & IEEE80211_HT_MCS_TX_UNEQUAL_MODULATION != 0 { for i in IEEE80211_HT_MCS_UNEQUAL_MODULATION_START_BYTE..IEEE80211_HT_MCS_MASK_LEN { ht_cap.mcs.rx_mask[i] = own_cap.mcs.rx_mask[i] & (*ht_cap_ie).mcs.rx_mask[i]; } }
    if own_cap.mcs.rx_mask[32 / 8] & (*ht_cap_ie).mcs.rx_mask[32 / 8] & 1 != 0 { ht_cap.mcs.rx_mask[32 / 8] |= 1; }
    ht_cap.mcs.rx_highest = (*ht_cap_ie).mcs.rx_highest;
    (*link_sta).pub_.agg.max_amsdu_len = if ht_cap.cap & IEEE80211_HT_CAP_MAX_AMSDU != 0 { IEEE80211_MAX_MPDU_LEN_HT_7935 } else { IEEE80211_MAX_MPDU_LEN_HT_3839 };
    ieee80211_sta_recalc_aggregates(&(*sta).sta);
goto_apply!();
}

macro_rules! goto_apply { () => {{
    let changed = memcmp(&(*link_sta).pub_.ht_cap as *const _ as *const u8, &ht_cap as *const _ as *const u8, core::mem::size_of::<ieee80211_sta_ht_cap>());
    memcpy(&mut (*link_sta).pub_.ht_cap as *mut _ as *mut u8, &ht_cap as *const _ as *const u8, core::mem::size_of::<ieee80211_sta_ht_cap>());
    if (*sta).sdata.vif.r#type == NL80211_IFTYPE_AP || (*sta).sdata.vif.r#type == NL80211_IFTYPE_AP_VLAN || (*sta).sdata.vif.r#type == NL80211_IFTYPE_NAN || (*sta).sdata.vif.r#type == NL80211_IFTYPE_NAN_DATA {
        let smps_mode = match (ht_cap.cap & IEEE80211_HT_CAP_SM_PS) >> IEEE80211_HT_CAP_SM_PS_SHIFT { WLAN_HT_CAP_SM_PS_INVALID | WLAN_HT_CAP_SM_PS_STATIC => IEEE80211_SMPS_STATIC, WLAN_HT_CAP_SM_PS_DYNAMIC => IEEE80211_SMPS_DYNAMIC, WLAN_HT_CAP_SM_PS_DISABLED => IEEE80211_SMPS_OFF, _ => IEEE80211_SMPS_OFF };
        let mut result = changed != 0; if smps_mode != (*link_sta).pub_.smps_mode { result = true; } (*link_sta).pub_.smps_mode = smps_mode; return result;
    } else { (*link_sta).pub_.smps_mode = IEEE80211_SMPS_OFF; }
    changed != 0
}} }

pub unsafe fn ieee80211_sta_tear_down_BA_sessions(sta: *mut sta_info, reason: ieee80211_agg_stop_reason) {
    lockdep_assert_wiphy((*sta).local.hw.wiphy);
    for i in 0..IEEE80211_NUM_TIDS { __ieee80211_stop_rx_ba_session(sta, i, WLAN_BACK_RECIPIENT, WLAN_REASON_QSTA_LEAVE_QBSS, reason != AGG_STOP_DESTROY_STA && reason != AGG_STOP_PEER_REQUEST); }
    for i in 0..IEEE80211_NUM_TIDS { __ieee80211_stop_tx_ba_session(sta, i, reason); }
    if reason == AGG_STOP_DESTROY_STA { wiphy_work_cancel((*sta).local.hw.wiphy, &mut (*sta).ampdu_mlme.work); for i in 0..IEEE80211_NUM_TIDS { let tid_tx = rcu_dereference_protected_tid_tx(sta, i); if !tid_tx.is_null() && test_and_clear_bit(HT_AGG_STATE_STOP_CB, &mut (*tid_tx).state) { ieee80211_stop_tx_ba_cb(sta, i, tid_tx); } } }
}

pub unsafe fn ieee80211_ba_session_work(wiphy: *mut wiphy, work: *mut wiphy_work) { let sta = container_of(work, sta_info, ampdu_mlme.work); lockdep_assert_wiphy((*sta).local.hw.wiphy); let blocked = test_sta_flag(sta, WLAN_STA_BLOCK_BA); for tid in 0..IEEE80211_NUM_TIDS { if test_and_clear_bit(tid, &mut (*sta).ampdu_mlme.tid_rx_timer_expired) { __ieee80211_stop_rx_ba_session(sta, tid, WLAN_BACK_RECIPIENT, WLAN_REASON_QSTA_TIMEOUT, true); } if test_and_clear_bit(tid, &mut (*sta).ampdu_mlme.tid_rx_stop_requested) { __ieee80211_stop_rx_ba_session(sta, tid, WLAN_BACK_RECIPIENT, WLAN_REASON_UNSPECIFIED, true); } if !blocked && test_and_clear_bit(tid, &mut (*sta).ampdu_mlme.tid_rx_manage_offl) { __ieee80211_start_rx_ba_session(sta, 0, 0, 0, 1, tid, IEEE80211_MAX_AMPDU_BUF_HT, false, true, false, 0); } if test_and_clear_bit(tid + IEEE80211_NUM_TIDS, &mut (*sta).ampdu_mlme.tid_rx_manage_offl) { __ieee80211_stop_rx_ba_session(sta, tid, WLAN_BACK_RECIPIENT, 0, false); } let tid_tx = rcu_dereference_protected_tid_tx(sta, tid); if tid_tx.is_null() { continue; } if !blocked && test_and_clear_bit(HT_AGG_STATE_START_CB, &mut (*tid_tx).state) { ieee80211_start_tx_ba_cb(sta, tid, tid_tx); } if test_and_clear_bit(HT_AGG_STATE_WANT_STOP, &mut (*tid_tx).state) { __ieee80211_stop_tx_ba_session(sta, tid, AGG_STOP_LOCAL_REQUEST); } if test_and_clear_bit(HT_AGG_STATE_STOP_CB, &mut (*tid_tx).state) { ieee80211_stop_tx_ba_cb(sta, tid, tid_tx); } } }

pub unsafe fn ieee80211_send_delba(sdata: *mut ieee80211_sub_if_data, da: *const u8, tid: u16, initiator: u16, reason_code: u16, use_ndp: bool) { let local = (*sdata).local; let skb = dev_alloc_skb(IEEE80211_MIN_ACTION_SIZE(delba) + (*local).hw.extra_tx_headroom); if skb.is_null() { return; } skb_reserve(skb, (*local).hw.extra_tx_headroom); let mgmt = ieee80211_mgmt_ba(skb, da, sdata); skb_put(skb, 2 + core::mem::size_of_val(&(*mgmt).u.action.delba)); (*mgmt).u.action.category = WLAN_CATEGORY_BACK; (*mgmt).u.action.action_code = if use_ndp { WLAN_ACTION_NDP_DELBA } else { WLAN_ACTION_DELBA }; let params = (initiator << 11) | (tid << 12); (*mgmt).u.action.delba.params = cpu_to_le16(params); (*mgmt).u.action.delba.reason_code = cpu_to_le16(reason_code); ieee80211_tx_skb(sdata, skb); }

pub unsafe fn ieee80211_process_delba(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, mgmt: *mut ieee80211_mgmt, _len: usize) { let params = le16_to_cpu((*mgmt).u.action.delba.params); let tid = (params & IEEE80211_DELBA_PARAM_TID_MASK) >> 12; let initiator = (params & IEEE80211_DELBA_PARAM_INITIATOR_MASK) >> 11; ht_dbg_ratelimited(sdata, "delba", (*mgmt).sa, if initiator != 0 { "initiator" } else { "recipient" }, tid, le16_to_cpu((*mgmt).u.action.delba.reason_code)); if initiator == WLAN_BACK_INITIATOR { __ieee80211_stop_rx_ba_session(sta, tid, WLAN_BACK_INITIATOR, 0, true); } else { __ieee80211_stop_tx_ba_session(sta, tid, AGG_STOP_PEER_REQUEST); } }

pub unsafe fn ieee80211_smps_mode_to_smps_mode(smps: ieee80211_smps_mode) -> nl80211_smps_mode { match smps { IEEE80211_SMPS_OFF => NL80211_SMPS_OFF, IEEE80211_SMPS_STATIC => NL80211_SMPS_STATIC, IEEE80211_SMPS_DYNAMIC => NL80211_SMPS_DYNAMIC, _ => NL80211_SMPS_OFF } }

pub unsafe fn ieee80211_send_smps_action(sdata: *mut ieee80211_sub_if_data, mut smps: ieee80211_smps_mode, da: *const u8, bssid: *const u8, link_id: i32) -> i32 { let local = (*sdata).local; let skb = dev_alloc_skb(IEEE80211_MIN_ACTION_SIZE(ht_smps) + (*local).hw.extra_tx_headroom); if skb.is_null() { return -ENOMEM; } skb_reserve(skb, (*local).hw.extra_tx_headroom); let action_frame = skb_put_zero(skb, IEEE80211_MIN_ACTION_SIZE(ht_smps)); memcpy((*action_frame).da.as_mut_ptr(), da, ETH_ALEN); memcpy((*action_frame).sa.as_mut_ptr(), (*sdata).dev.dev_addr.as_ptr(), ETH_ALEN); memcpy((*action_frame).bssid.as_mut_ptr(), bssid, ETH_ALEN); (*action_frame).frame_control = cpu_to_le16(IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_ACTION); (*action_frame).u.action.category = WLAN_CATEGORY_HT; (*action_frame).u.action.action_code = WLAN_HT_ACTION_SMPS; match smps { IEEE80211_SMPS_AUTOMATIC | IEEE80211_SMPS_NUM_MODES => { WARN_ON(1); smps = IEEE80211_SMPS_OFF; (*action_frame).u.action.ht_smps.smps_control = WLAN_HT_SMPS_CONTROL_DISABLED; }, IEEE80211_SMPS_OFF => (*action_frame).u.action.ht_smps.smps_control = WLAN_HT_SMPS_CONTROL_DISABLED, IEEE80211_SMPS_STATIC => (*action_frame).u.action.ht_smps.smps_control = WLAN_HT_SMPS_CONTROL_STATIC, IEEE80211_SMPS_DYNAMIC => (*action_frame).u.action.ht_smps.smps_control = WLAN_HT_SMPS_CONTROL_DYNAMIC, } let info = IEEE80211_SKB_CB(skb); (*info).flags |= IEEE80211_TX_CTL_REQ_TX_STATUS; (*info).status_data = IEEE80211_STATUS_TYPE_SMPS | u16_encode_bits(((if link_id < 0 { 0 } else { link_id }) as u16) << 2 | smps as u16, IEEE80211_STATUS_SUBDATA_MASK); ieee80211_tx_skb_tid(sdata, skb, 7, link_id); 0 }

pub unsafe fn ieee80211_request_smps(vif: *mut ieee80211_vif, link_id: usize, smps_mode: ieee80211_smps_mode) { let sdata = vif_to_sdata(vif); if WARN_ON_ONCE((*vif).r#type != NL80211_IFTYPE_STATION) { return; } rcu_read_lock(); let link = rcu_dereference((*sdata).link[link_id]); if WARN_ON(link.is_null()) { rcu_read_unlock(); return; } trace_api_request_smps((*sdata).local, sdata, link, smps_mode); if (*link).u.mgd.driver_smps_mode != smps_mode { (*link).u.mgd.driver_smps_mode = smps_mode; wiphy_work_queue((*sdata).local.hw.wiphy, &mut (*link).u.mgd.request_smps_work); } rcu_read_unlock(); }

pub unsafe fn ieee80211_ht_handle_chanwidth_notif(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, link_sta: *mut link_sta_info, chanwidth: u8, _band: nl80211_band) { lockdep_assert_wiphy((*local).hw.wiphy); let link = sdata_dereference((*sdata).link[(*link_sta).link_id], sdata); if WARN_ON(link.is_null()) { return; } (*link_sta).op_mode_bw = if chanwidth == IEEE80211_HT_CHANWIDTH_20MHZ { IEEE80211_STA_RX_BW_20 } else { IEEE80211_STA_RX_BW_MAX }; if !ieee80211_link_sta_update_rc_bw(link, link_sta) { return; } let mut sta_opmode = sta_opmode_info { changed: STA_OPMODE_MAX_BW_CHANGED, bw: ieee80211_sta_rx_bw_to_chan_width((*link_sta).pub_.bandwidth) }; cfg80211_sta_opmode_change_notify((*sdata).dev, (*sta).addr.as_ptr(), &mut sta_opmode, GFP_KERNEL); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
