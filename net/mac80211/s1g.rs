// SPDX-License-Identifier: GPL-2.0
/*
 * S1G handling
 * Copyright(c) 2020 Adapt-IP
 * Copyright (C) 2023, 2026 Intel Corporation
 */

pub unsafe fn ieee80211_s1g_sta_rate_init(sta: *mut sta_info) {
	/* avoid indicating legacy bitrates for S1G STAs */
	(*sta).deflink.tx_stats.last_rate.flags |= IEEE80211_TX_RC_S1G_MCS;
	(*sta).deflink.rx_stats.last_rate = STA_STATS_FIELD(TYPE, STA_STATS_RATE_TYPE_S1G);
}

pub unsafe fn ieee80211_s1g_is_twt_setup(skb: *mut sk_buff) -> bool {
	let mgmt = (*skb).data as *mut ieee80211_mgmt;

	if likely(!ieee80211_is_action((*mgmt).frame_control)) {
		return false;
	}

	if likely((*mgmt).u.action.category != WLAN_CATEGORY_S1G) {
		return false;
	}

	(*mgmt).u.action.action_code == WLAN_S1G_TWT_SETUP
}

unsafe fn ieee80211_s1g_send_twt_setup(
	sdata: *mut ieee80211_sub_if_data,
	da: *const u8,
	bssid: *const u8,
	twt: *mut ieee80211_twt_setup,
) {
	let len: i32 = IEEE80211_MIN_ACTION_SIZE(s1g) + 3 + (*twt).length as i32;
	let local = (*sdata).local;
	let mut mgmt: *mut ieee80211_mgmt;
	let skb: *mut sk_buff;

	skb = dev_alloc_skb((*local).hw.extra_tx_headroom + len as usize);
	if skb.is_null() {
		return;
	}

	skb_reserve(skb, (*local).hw.extra_tx_headroom);
	mgmt = skb_put_zero(skb, len as usize) as *mut ieee80211_mgmt;
	(*mgmt).frame_control = cpu_to_le16(IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_ACTION);
	memcpy((*mgmt).da.as_mut_ptr(), da, ETH_ALEN);
	memcpy((*mgmt).sa.as_mut_ptr(), (*sdata).vif.addr.as_ptr(), ETH_ALEN);
	memcpy((*mgmt).bssid.as_mut_ptr(), bssid, ETH_ALEN);

	(*mgmt).u.action.category = WLAN_CATEGORY_S1G;
	(*mgmt).u.action.action_code = WLAN_S1G_TWT_SETUP;
	memcpy((*mgmt).u.action.s1g.variable.as_mut_ptr(), twt as *const u8, 3 + (*twt).length as usize);

	(*IEEE80211_SKB_CB(skb)).flags |= IEEE80211_TX_INTFL_DONT_ENCRYPT |
		IEEE80211_TX_INTFL_MLME_CONN_TX | IEEE80211_TX_CTL_REQ_TX_STATUS;
	ieee80211_tx_skb(sdata, skb);
}

unsafe fn ieee80211_s1g_send_twt_teardown(
	sdata: *mut ieee80211_sub_if_data,
	da: *const u8,
	bssid: *const u8,
	flowid: u8,
) {
	let local = (*sdata).local;
	let mut mgmt: *mut ieee80211_mgmt;
	let skb: *mut sk_buff;
	let id: *mut u8;

	skb = dev_alloc_skb((*local).hw.extra_tx_headroom + IEEE80211_MIN_ACTION_SIZE(s1g) as usize + 1);
	if skb.is_null() {
		return;
	}

	skb_reserve(skb, (*local).hw.extra_tx_headroom);
	mgmt = skb_put_zero(skb, IEEE80211_MIN_ACTION_SIZE(s1g) as usize + 1) as *mut ieee80211_mgmt;
	(*mgmt).frame_control = cpu_to_le16(IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_ACTION);
	memcpy((*mgmt).da.as_mut_ptr(), da, ETH_ALEN);
	memcpy((*mgmt).sa.as_mut_ptr(), (*sdata).vif.addr.as_ptr(), ETH_ALEN);
	memcpy((*mgmt).bssid.as_mut_ptr(), bssid, ETH_ALEN);

	(*mgmt).u.action.category = WLAN_CATEGORY_S1G;
	(*mgmt).u.action.action_code = WLAN_S1G_TWT_TEARDOWN;
	id = (*mgmt).u.action.s1g.variable.as_mut_ptr();
	*id = flowid;

	(*IEEE80211_SKB_CB(skb)).flags |= IEEE80211_TX_INTFL_DONT_ENCRYPT |
		IEEE80211_TX_CTL_REQ_TX_STATUS;
	ieee80211_tx_skb(sdata, skb);
}

unsafe fn ieee80211_s1g_rx_twt_setup(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, skb: *mut sk_buff) {
	let mgmt = (*skb).data as *mut ieee80211_mgmt;
	let twt = (*mgmt).u.action.s1g.variable.as_mut_ptr() as *mut ieee80211_twt_setup;
	let twt_agrt = (*twt).params.as_mut_ptr() as *mut ieee80211_twt_params;

	if ((*twt).control & IEEE80211_TWT_CONTROL_NEG_TYPE_BROADCAST) == 0 &&
		(*twt).length < (core::mem::size_of::<u8>() + core::mem::size_of::<ieee80211_twt_params>()) as u8 {
		return;
	}

	(*twt_agrt).req_type &= cpu_to_le16(!IEEE80211_TWT_REQTYPE_REQUEST);
	/* broadcast TWT not supported yet */
	if (*twt).control & IEEE80211_TWT_CONTROL_NEG_TYPE_BROADCAST != 0 {
		(*twt_agrt).req_type &= !cpu_to_le16(IEEE80211_TWT_REQTYPE_SETUP_CMD);
		(*twt_agrt).req_type |= le16_encode_bits(TWT_SETUP_CMD_REJECT, IEEE80211_TWT_REQTYPE_SETUP_CMD);
		ieee80211_s1g_send_twt_setup(sdata, (*mgmt).sa.as_ptr(), (*sdata).vif.addr.as_ptr(), twt);
		return;
	}

	/* TWT Information not supported yet */
	(*twt).control |= IEEE80211_TWT_CONTROL_RX_DISABLED;
	drv_add_twt_setup((*sdata).local, sdata, &mut (*sta).sta, twt);
	ieee80211_s1g_send_twt_setup(sdata, (*mgmt).sa.as_ptr(), (*sdata).vif.addr.as_ptr(), twt);
}

unsafe fn ieee80211_s1g_rx_twt_teardown(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, skb: *mut sk_buff) {
	let mgmt = (*skb).data as *mut ieee80211_mgmt;
	drv_twt_teardown_request((*sdata).local, sdata, &mut (*sta).sta, (*mgmt).u.action.s1g.variable[0]);
}

unsafe fn ieee80211_s1g_tx_twt_setup_fail(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, skb: *mut sk_buff) {
	let mgmt = (*skb).data as *mut ieee80211_mgmt;
	let twt = (*mgmt).u.action.s1g.variable.as_mut_ptr() as *mut ieee80211_twt_setup;
	let twt_agrt = (*twt).params.as_mut_ptr() as *mut ieee80211_twt_params;
	let flowid = le16_get_bits((*twt_agrt).req_type, IEEE80211_TWT_REQTYPE_FLOWID);
	drv_twt_teardown_request((*sdata).local, sdata, &mut (*sta).sta, flowid);
	ieee80211_s1g_send_twt_teardown(sdata, (*mgmt).da.as_ptr(), (*sdata).vif.addr.as_ptr(), flowid);
}

pub unsafe fn ieee80211_s1g_rx_twt_action(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) {
	let mgmt = (*skb).data as *mut ieee80211_mgmt;
	let local = (*sdata).local;
	lockdep_assert_wiphy((*local).hw.wiphy);
	let sta = sta_info_get_bss(sdata, (*mgmt).sa.as_ptr());
	if sta.is_null() { return; }
	match (*mgmt).u.action.action_code {
		WLAN_S1G_TWT_SETUP => ieee80211_s1g_rx_twt_setup(sdata, sta, skb),
		WLAN_S1G_TWT_TEARDOWN => ieee80211_s1g_rx_twt_teardown(sdata, sta, skb),
		_ => (),
	}
}

pub unsafe fn ieee80211_s1g_status_twt_action(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) {
	let mgmt = (*skb).data as *mut ieee80211_mgmt;
	let local = (*sdata).local;
	lockdep_assert_wiphy((*local).hw.wiphy);
	let sta = sta_info_get_bss(sdata, (*mgmt).da.as_ptr());
	if sta.is_null() { return; }
	match (*mgmt).u.action.action_code {
		WLAN_S1G_TWT_SETUP => ieee80211_s1g_tx_twt_setup_fail(sdata, sta, skb),
		_ => (),
	}
}

pub unsafe fn ieee80211_s1g_cap_to_sta_s1g_cap(sdata: *mut ieee80211_sub_if_data, s1g_cap_ie: *const ieee80211_s1g_cap, link_sta: *mut link_sta_info) {
	let s1g_cap = &mut (*(*link_sta).pub_).s1g_cap;
	core::ptr::write_bytes(s1g_cap, 0, 1);
	memcpy(s1g_cap.cap.as_mut_ptr(), (*s1g_cap_ie).capab_info.as_ptr(), s1g_cap.cap.len());
	memcpy(s1g_cap.nss_mcs.as_mut_ptr(), (*s1g_cap_ie).supp_mcs_nss.as_ptr(), s1g_cap.nss_mcs.len());
	s1g_cap.s1g = true;
	if s1g_cap.cap[3] & S1G_CAP3_MAX_MPDU_LEN != 0 {
		(*(*link_sta).pub_).agg.max_amsdu_len = IEEE80211_MAX_MPDU_LEN_VHT_7991;
	} else {
		(*(*link_sta).pub_).agg.max_amsdu_len = IEEE80211_MAX_MPDU_LEN_VHT_3895;
	}
	ieee80211_sta_recalc_aggregates(&mut (*(*link_sta).sta).sta);
}

pub unsafe fn ieee80211_s1g_use_ndp_ba(sdata: *const ieee80211_sub_if_data, sta: *const sta_info) -> bool {
	(*sdata).vif.cfg.s1g && ieee80211_hw_check(&(*(*sdata).local).hw, SUPPORTS_NDP_BLOCKACK) &&
		(!sta.is_null() && (*sta).sta.deflink.s1g_cap.s1g)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
