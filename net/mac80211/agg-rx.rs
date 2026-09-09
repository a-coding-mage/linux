// SPDX-License-Identifier: GPL-2.0-only
/*
 * HT handling
 *
 * RX A-MPDU aggregation
 *
 * Direct Rust translation of agg-rx.c. Kernel types, constants, and helpers
 * are supplied by the surrounding mac80211 implementation.
 */

unsafe fn ieee80211_free_tid_rx(h: *mut rcu_head) {
    let tid_rx = container_of!(h, tid_ampdu_rx, rcu_head);
    let mut i = 0;
    while i < (*tid_rx).buf_size {
        __skb_queue_purge(&mut (*tid_rx).reorder.add(i).read().buf);
        i += 1;
    }
    kfree(tid_rx as *mut core::ffi::c_void);
}

pub unsafe fn __ieee80211_stop_rx_ba_session(
    sta: *mut sta_info, tid: u16, initiator: u16, reason: u16, tx: bool,
) {
    let local = (*sta).local;
    let mut tid_rx: *mut tid_ampdu_rx;
    let mut params = ieee80211_ampdu_params {
        sta: &mut (*sta).sta,
        action: IEEE80211_AMPDU_RX_STOP,
        tid,
        amsdu: false,
        timeout: 0,
        ssn: 0,
    };

    lockdep_assert_wiphy((*sta).local.hw.wiphy);
    tid_rx = rcu_dereference_protected!((*sta).ampdu_mlme.tid_rx[tid as usize],
        lockdep_is_held!((*sta).local.hw.wiphy.mtx));
    if !test_bit(tid, (*sta).ampdu_mlme.agg_session_valid) { return; }

    RCU_INIT_POINTER!((*sta).ampdu_mlme.tid_rx[tid as usize], core::ptr::null_mut());
    __clear_bit(tid, (*sta).ampdu_mlme.agg_session_valid);
    ht_dbg!((*sta).sdata, "Rx BA session stop requested for %pM tid %u %s reason: %d\n",
        (*sta).sta.addr, tid,
        if initiator == WLAN_BACK_RECIPIENT { "recipient" } else { "initiator" },
        reason as i32);

    if drv_ampdu_action(local, (*sta).sdata, &mut params) != 0 {
        sdata_info!((*sta).sdata,
            "HW problem - can not stop rx aggregation for %pM tid %d\n", (*sta).sta.addr, tid);
    }
    if initiator == WLAN_BACK_RECIPIENT && tx {
        ieee80211_send_delba((*sta).sdata, (*sta).sta.addr, tid,
            WLAN_BACK_RECIPIENT, reason, ieee80211_s1g_use_ndp_ba((*sta).sdata, sta));
    }
    if tid_rx.is_null() { return; }
    timer_delete_sync(&mut (*tid_rx).session_timer);
    spin_lock_bh(&mut (*tid_rx).reorder_lock);
    (*tid_rx).removed = true;
    spin_unlock_bh(&mut (*tid_rx).reorder_lock);
    timer_delete_sync(&mut (*tid_rx).reorder_timer);
    call_rcu(&mut (*tid_rx).rcu_head, ieee80211_free_tid_rx);
}

pub unsafe fn ieee80211_stop_rx_ba_session(vif: *mut ieee80211_vif, ba_rx_bitmap: u16, addr: *const u8) {
    let sdata = vif_to_sdata(vif);
    rcu_read_lock();
    let sta = sta_info_get_bss(sdata, addr);
    if sta.is_null() { rcu_read_unlock(); return; }
    let mut i = 0;
    while i < IEEE80211_NUM_TIDS {
        if ba_rx_bitmap & BIT(i) != 0 { set_bit(i, (*sta).ampdu_mlme.tid_rx_stop_requested); }
        i += 1;
    }
    wiphy_work_queue((*sta).local.hw.wiphy, &mut (*sta).ampdu_mlme.work);
    rcu_read_unlock();
}

unsafe fn sta_rx_agg_session_timer_expired(t: *mut timer_list) {
    let tid_rx = timer_container_of!(t, tid_ampdu_rx, session_timer);
    let sta = (*tid_rx).sta;
    let tid = (*tid_rx).tid;
    let timeout = (*tid_rx).last_rx + TU_TO_JIFFIES((*tid_rx).timeout);
    if time_is_after_jiffies(timeout) { mod_timer(&mut (*tid_rx).session_timer, timeout); return; }
    ht_dbg!((*sta).sdata, "RX session timer expired on %pM tid %d\n", (*sta).sta.addr, tid);
    set_bit(tid, (*sta).ampdu_mlme.tid_rx_timer_expired);
    wiphy_work_queue((*sta).local.hw.wiphy, &mut (*sta).ampdu_mlme.work);
}

unsafe fn sta_rx_agg_reorder_timer_expired(t: *mut timer_list) {
    let tid_rx = timer_container_of!(t, tid_ampdu_rx, reorder_timer);
    rcu_read_lock();
    ieee80211_release_reorder_timeout((*tid_rx).sta, (*tid_rx).tid);
    rcu_read_unlock();
}

pub unsafe fn ieee80211_add_addbaext(skb: *mut sk_buff, req_addba_ext_data: u8, buf_size: u16) {
    let mut pos = skb_put_zero(skb, 2 + core::mem::size_of::<ieee80211_addba_ext_ie>()) as *mut u8;
    *pos = WLAN_EID_ADDBA_EXT; pos = pos.add(1);
    *pos = core::mem::size_of::<ieee80211_addba_ext_ie>() as u8; pos = pos.add(1);
    let ext = pos as *mut ieee80211_addba_ext_ie;
    (*ext).data = IEEE80211_ADDBA_EXT_NO_FRAG;
    if req_addba_ext_data != 0 { (*ext).data &= req_addba_ext_data; }
    (*ext).data |= u8_encode_bits(buf_size >> IEEE80211_ADDBA_EXT_BUF_SIZE_SHIFT,
                                  IEEE80211_ADDBA_EXT_BUF_SIZE_MASK);
}

pub unsafe fn ieee80211_retrieve_addba_ext_data(sta: *mut sta_info, elem_data: *const core::ffi::c_void,
                                                  elem_len: isize, buf_size: *mut u16) -> u8 {
    if !(*sta).sta.deflink.he_cap.has_he || elem_len <= 0 { return 0; }
    let elems = ieee802_11_parse_elems(elem_data, elem_len,
        IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_ACTION, core::ptr::null_mut());
    if elems.is_null() || (*elems).parse_error || (*elems).addba_ext_ie.is_null() {
        kfree(elems as *mut core::ffi::c_void); return 0;
    }
    let data = (*(*elems).addba_ext_ie).data;
    if !buf_size.is_null() && ((*sta).sta.valid_links || (*sta).sta.deflink.eht_cap.has_eht) {
        let n = u8_get_bits(data, IEEE80211_ADDBA_EXT_BUF_SIZE_MASK);
        *buf_size |= (n as u16) << IEEE80211_ADDBA_EXT_BUF_SIZE_SHIFT;
    }
    kfree(elems as *mut core::ffi::c_void); data
}

unsafe fn ieee80211_send_addba_resp(sta: *mut sta_info, da: *mut u8, tid: u16,
    dialog_token: u8, status: u16, policy: u16, buf_size: u16, timeout: u16,
    req_addba_ext_data: u8) {
    let sdata = (*sta).sdata;
    let local = (*sdata).local;
    let amsdu = ieee80211_hw_check!((*local).hw, SUPPORTS_AMSDU_IN_AMPDU);
    let use_ndp = ieee80211_s1g_use_ndp_ba(sdata, sta);
    let skb = dev_alloc_skb(core::mem::size_of::<ieee80211_mgmt>() + 2 +
        core::mem::size_of::<ieee80211_addba_ext_ie>() + (*local).hw.extra_tx_headroom);
    if skb.is_null() { return; }
    skb_reserve(skb, (*local).hw.extra_tx_headroom);
    let mgmt = ieee80211_mgmt_ba(skb, da, sdata);
    skb_put(skb, 2 + core::mem::size_of_val(&(*mgmt).u.action.addba_resp));
    (*mgmt).u.action.category = WLAN_CATEGORY_BACK;
    (*mgmt).u.action.action_code = if use_ndp { WLAN_ACTION_NDP_ADDBA_RESP } else { WLAN_ACTION_ADDBA_RESP };
    (*mgmt).u.action.addba_resp.dialog_token = dialog_token;
    let mut capab = u16_encode_bits(amsdu, IEEE80211_ADDBA_PARAM_AMSDU_MASK);
    capab |= u16_encode_bits(policy, IEEE80211_ADDBA_PARAM_POLICY_MASK);
    capab |= u16_encode_bits(tid, IEEE80211_ADDBA_PARAM_TID_MASK);
    capab |= u16_encode_bits(buf_size, IEEE80211_ADDBA_PARAM_BUF_SIZE_MASK);
    (*mgmt).u.action.addba_resp.capab = cpu_to_le16(capab);
    (*mgmt).u.action.addba_resp.timeout = cpu_to_le16(timeout);
    (*mgmt).u.action.addba_resp.status = cpu_to_le16(status);
    if (*sta).sta.valid_links || (*sta).sta.deflink.he_cap.has_he {
        ieee80211_add_addbaext(skb, req_addba_ext_data, buf_size);
    }
    ieee80211_tx_skb(sdata, skb);
}

pub unsafe fn __ieee80211_start_rx_ba_session(sta: *mut sta_info, dialog_token: u8,
    timeout: u16, start_seq_num: u16, ba_policy: u16, tid: u16, mut buf_size: u16,
    tx: bool, auto_seq: bool, req_ndp: bool, addba_ext_data: u8) {
    let local = (*sta).sdata.local;
    let mut params = ieee80211_ampdu_params { sta: &mut (*sta).sta,
        action: IEEE80211_AMPDU_RX_START, tid, amsdu: false, timeout, ssn: start_seq_num,
        buf_size: 0 };
    let mut status = WLAN_STATUS_REQUEST_DECLINED;
    let mut ret = -EOPNOTSUPP;
    if tid >= IEEE80211_FIRST_TSPEC_TSID { goto_end!(); }
    if tx && ieee80211_s1g_use_ndp_ba((*sta).sdata, sta) && !req_ndp {
        status = WLAN_STATUS_REJECTED_NDP_BLOCK_ACK_SUGGESTED; goto_end!();
    }
    if !(*sta).sta.valid_links && !(*sta).sta.deflink.ht_cap.ht_supported &&
       !(*sta).sta.deflink.he_cap.has_he && !(*sta).sta.deflink.s1g_cap.s1g { goto_end!(); }
    if test_sta_flag(sta, WLAN_STA_BLOCK_BA) { goto_end!(); }
    let max_buf_size = if (*sta).sta.valid_links || (*sta).sta.deflink.eht_cap.has_eht {
        IEEE80211_MAX_AMPDU_BUF_EHT
    } else if (*sta).sta.deflink.he_cap.has_he { IEEE80211_MAX_AMPDU_BUF_HE }
    else { IEEE80211_MAX_AMPDU_BUF_HT };
    if ((ba_policy != 1 && ((*sta).sta.valid_links ||
        !((*sta).sta.deflink.ht_cap.cap & IEEE80211_HT_CAP_DELAY_BA != 0) ||
        !((*sta).sta.deflink.s1g_cap.cap[3] & S1G_CAP3_HT_DELAYED_BA != 0))) || buf_size > max_buf_size) {
        status = WLAN_STATUS_INVALID_QOS_PARAM; goto_end!();
    }
    if buf_size == 0 { buf_size = max_buf_size; }
    if buf_size > (*sta).sta.max_rx_aggregation_subframes { buf_size = (*sta).sta.max_rx_aggregation_subframes; }
    params.buf_size = buf_size;
    if test_bit(tid, (*sta).ampdu_mlme.agg_session_valid) {
        if (*sta).ampdu_mlme.tid_rx_token[tid as usize] == dialog_token {
            rcu_read_lock(); let p = rcu_dereference!((*sta).ampdu_mlme.tid_rx[tid as usize]);
            status = if !p.is_null() && (*p).timeout == timeout { WLAN_STATUS_SUCCESS } else { WLAN_STATUS_REQUEST_DECLINED };
            rcu_read_unlock(); goto_end!();
        }
        __ieee80211_stop_rx_ba_session(sta, tid, WLAN_BACK_RECIPIENT, WLAN_STATUS_UNSPECIFIED_QOS, false);
    }
    if ieee80211_hw_check!((*local).hw, SUPPORTS_REORDERING_BUFFER) {
        ret = drv_ampdu_action(local, (*sta).sdata, &mut params);
        if ret == 0 { status = WLAN_STATUS_SUCCESS; }
        goto_end!();
    }
    let tid_agg_rx = kzalloc_flex!(tid_ampdu_rx, reorder, buf_size);
    if tid_agg_rx.is_null() { goto_end!(); }
    spin_lock_init(&mut (*tid_agg_rx).reorder_lock);
    timer_setup(&mut (*tid_agg_rx).session_timer, sta_rx_agg_session_timer_expired, TIMER_DEFERRABLE);
    timer_setup(&mut (*tid_agg_rx).reorder_timer, sta_rx_agg_reorder_timer_expired, 0);
    let mut i = 0; while i < buf_size { __skb_queue_head_init(&mut (*tid_agg_rx).reorder.add(i as usize).read().buf); i += 1; }
    ret = drv_ampdu_action(local, (*sta).sdata, &mut params);
    if ret != 0 { kfree(tid_agg_rx as *mut core::ffi::c_void); goto_end!(); }
    (*tid_agg_rx).ssn = start_seq_num; (*tid_agg_rx).head_seq_num = start_seq_num;
    (*tid_agg_rx).buf_size = buf_size; (*tid_agg_rx).timeout = timeout;
    (*tid_agg_rx).stored_mpdu_num = 0; (*tid_agg_rx).auto_seq = auto_seq; (*tid_agg_rx).started = false;
    (*tid_agg_rx).reorder_buf_filtered = 0; (*tid_agg_rx).tid = tid; (*tid_agg_rx).sta = sta;
    status = WLAN_STATUS_SUCCESS;
    rcu_assign_pointer!((*sta).ampdu_mlme.tid_rx[tid as usize], tid_agg_rx);
    if timeout != 0 { mod_timer(&mut (*tid_agg_rx).session_timer, TU_TO_EXP_TIME(timeout)); (*tid_agg_rx).last_rx = jiffies; }
    goto_end!();
    if status == WLAN_STATUS_SUCCESS { __set_bit(tid, (*sta).ampdu_mlme.agg_session_valid); __clear_bit(tid, (*sta).ampdu_mlme.unexpected_agg); (*sta).ampdu_mlme.tid_rx_token[tid as usize] = dialog_token; }
    if tx { ieee80211_send_addba_resp(sta, (*sta).sta.addr, tid, dialog_token, status, 1, buf_size, timeout, addba_ext_data); }
}

pub unsafe fn ieee80211_process_addba_request(local: *mut ieee80211_local, sta: *mut sta_info, mgmt: *mut ieee80211_mgmt, len: usize) {
    let req_ndp = (*mgmt).u.action.action_code == WLAN_ACTION_NDP_ADDBA_REQ;
    let dialog_token = (*mgmt).u.action.addba_req.dialog_token;
    let timeout = le16_to_cpu((*mgmt).u.action.addba_req.timeout);
    let start_seq_num = le16_to_cpu((*mgmt).u.action.addba_req.start_seq_num) >> 4;
    let capab = le16_to_cpu((*mgmt).u.action.addba_req.capab);
    let ba_policy = (capab & IEEE80211_ADDBA_PARAM_POLICY_MASK) >> 1;
    let tid = (capab & IEEE80211_ADDBA_PARAM_TID_MASK) >> 2;
    let mut buf_size = (capab & IEEE80211_ADDBA_PARAM_BUF_SIZE_MASK) >> 6;
    let addba_ext_data = ieee80211_retrieve_addba_ext_data(sta, (*mgmt).u.action.addba_req.variable as *const _,
        (len - offset_of!((*mgmt), u.action.addba_req.variable)) as isize, &mut buf_size);
    __ieee80211_start_rx_ba_session(sta, dialog_token, timeout, start_seq_num, ba_policy, tid, buf_size, true, false, req_ndp, addba_ext_data);
}

pub unsafe fn ieee80211_manage_rx_ba_offl(vif: *mut ieee80211_vif, addr: *const u8, tid: u32) {
    let sdata = vif_to_sdata(vif); rcu_read_lock(); let sta = sta_info_get_bss(sdata, addr);
    if !sta.is_null() { set_bit(tid, (*sta).ampdu_mlme.tid_rx_manage_offl); wiphy_work_queue((*sta).local.hw.wiphy, &mut (*sta).ampdu_mlme.work); }
    rcu_read_unlock();
}

pub unsafe fn ieee80211_rx_ba_timer_expired(vif: *mut ieee80211_vif, addr: *const u8, tid: u32) {
    let sdata = vif_to_sdata(vif); rcu_read_lock(); let sta = sta_info_get_bss(sdata, addr);
    if !sta.is_null() { set_bit(tid, (*sta).ampdu_mlme.tid_rx_timer_expired); wiphy_work_queue((*sta).local.hw.wiphy, &mut (*sta).ampdu_mlme.work); }
    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
