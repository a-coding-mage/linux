// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012-2013, Marco Porsch <marco.porsch@s2005.tu-chemnitz.de>
 * Copyright 2012-2013, cozybit Inc.
 * Copyright (C) 2021 Intel Corporation
 * Copyright (C) 2023 Intel Corporation
 */

// Dependencies supplied by mesh.h and wme.h in the surrounding translation.

/* mesh PS management */

/// Create pre-addressed QoS Null frame for mesh powersave.
unsafe fn mps_qos_null_get(sta: *mut sta_info) -> *mut sk_buff {
    let sdata = (*sta).sdata;
    let local = (*sdata).local;
    let size = core::mem::size_of::<ieee80211_hdr>() as i32;
    let mut fc: __le16;
    let skb = dev_alloc_skb((*local).hw.extra_tx_headroom + size + 2);
    if skb.is_null() { return core::ptr::null_mut(); }
    skb_reserve(skb, (*local).hw.extra_tx_headroom);
    let nullfunc = skb_put(skb, size) as *mut ieee80211_hdr;
    fc = cpu_to_le16(IEEE80211_FTYPE_DATA | IEEE80211_STYPE_QOS_NULLFUNC);
    ieee80211_fill_mesh_addresses(nullfunc, &mut fc, (*sta).sta.addr, (*sdata).vif.addr);
    (*nullfunc).frame_control = fc;
    (*nullfunc).duration_id = 0;
    (*nullfunc).seq_ctrl = 0;
    memcpy((*nullfunc).addr1.as_mut_ptr() as *mut core::ffi::c_void,
           (*sta).sta.addr.as_ptr() as *const core::ffi::c_void, ETH_ALEN);
    skb_put_zero(skb, 2);
    ieee80211_mps_set_frame_flags(sdata, sta, nullfunc);
    skb
}

unsafe fn mps_qos_null_tx(sta: *mut sta_info) {
    let skb = mps_qos_null_get(sta);
    if skb.is_null() { return; }
    mps_dbg((*sta).sdata, "announcing peer-specific power mode to %pM\n", (*sta).sta.addr);
    if !test_sta_flag(sta, WLAN_STA_PS_STA) {
        let qc = ieee80211_get_qos_ctl((*skb).data as *mut core::ffi::c_void);
        *qc |= IEEE80211_QOS_CTL_EOSP;
    }
    ieee80211_tx_skb((*sta).sdata, skb);
}

pub unsafe fn ieee80211_mps_local_status_update(sdata: *mut ieee80211_sub_if_data) -> u64 {
    let ifmsh = &mut (*sdata).u.mesh;
    let mut peering = false;
    let mut light_sleep_cnt = 0;
    let mut deep_sleep_cnt = 0;
    let mut changed = 0;
    let nonpeer_pm;
    rcu_read_lock();
    list_for_each_entry_rcu!(sta, &(*(*sdata).local).sta_list, list, {
        if sdata != (*sta).sdata { continue; }
        match (*sta).mesh.plink_state {
            NL80211_PLINK_OPN_SNT | NL80211_PLINK_OPN_RCVD | NL80211_PLINK_CNF_RCVD => { peering = true; }
            NL80211_PLINK_ESTAB => {
                if (*sta).mesh.local_pm == NL80211_MESH_POWER_LIGHT_SLEEP { light_sleep_cnt += 1; }
                else if (*sta).mesh.local_pm == NL80211_MESH_POWER_DEEP_SLEEP { deep_sleep_cnt += 1; }
            }
            _ => {}
        }
    });
    rcu_read_unlock();
    if peering { mps_dbg(sdata, "setting non-peer PM to active for peering\n"); nonpeer_pm = NL80211_MESH_POWER_ACTIVE; }
    else if light_sleep_cnt != 0 || deep_sleep_cnt != 0 { mps_dbg(sdata, "setting non-peer PM to deep sleep\n"); nonpeer_pm = NL80211_MESH_POWER_DEEP_SLEEP; }
    else { mps_dbg(sdata, "setting non-peer PM to user value\n"); nonpeer_pm = ifmsh.mshcfg.power_mode; }
    if ifmsh.nonpeer_pm != nonpeer_pm || (ifmsh.ps_peers_light_sleep == 0) != (light_sleep_cnt == 0) || (ifmsh.ps_peers_deep_sleep == 0) != (deep_sleep_cnt == 0) { changed = BSS_CHANGED_BEACON; }
    ifmsh.nonpeer_pm = nonpeer_pm;
    ifmsh.ps_peers_light_sleep = light_sleep_cnt;
    ifmsh.ps_peers_deep_sleep = deep_sleep_cnt;
    changed
}

pub unsafe fn ieee80211_mps_set_sta_local_pm(sta: *mut sta_info, pm: nl80211_mesh_power_mode) -> u64 {
    let sdata = (*sta).sdata;
    if (*sta).mesh.local_pm == pm { return 0; }
    mps_dbg(sdata, "local STA operates in mode %d with %pM\n", pm, (*sta).sta.addr);
    (*sta).mesh.local_pm = pm;
    if (*sta).mesh.plink_state == NL80211_PLINK_ESTAB { mps_qos_null_tx(sta); }
    ieee80211_mps_local_status_update(sdata)
}

pub unsafe fn ieee80211_mps_set_frame_flags(sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, hdr: *mut ieee80211_hdr) {
    let pm;
    if WARN_ON(is_unicast_ether_addr((*hdr).addr1) && ieee80211_is_data_qos((*hdr).frame_control) && sta.is_null()) { return; }
    if is_unicast_ether_addr((*hdr).addr1) && ieee80211_is_data_qos((*hdr).frame_control) && (*sta).mesh.plink_state == NL80211_PLINK_ESTAB { pm = (*sta).mesh.local_pm; } else { pm = (*sdata).u.mesh.nonpeer_pm; }
    if pm == NL80211_MESH_POWER_ACTIVE { (*hdr).frame_control &= cpu_to_le16(!IEEE80211_FCTL_PM); } else { (*hdr).frame_control |= cpu_to_le16(IEEE80211_FCTL_PM); }
    if !ieee80211_is_data_qos((*hdr).frame_control) { return; }
    let qc = ieee80211_get_qos_ctl(hdr);
    if (is_unicast_ether_addr((*hdr).addr1) && pm == NL80211_MESH_POWER_DEEP_SLEEP) || (is_multicast_ether_addr((*hdr).addr1) && (*sdata).u.mesh.ps_peers_deep_sleep > 0) { *qc.add(1) |= IEEE80211_QOS_CTL_MESH_PS_LEVEL >> 8; } else { *qc.add(1) &= !(IEEE80211_QOS_CTL_MESH_PS_LEVEL >> 8); }
}

pub unsafe fn ieee80211_mps_sta_status_update(sta: *mut sta_info) {
    if (*sta).sta_state < IEEE80211_STA_ASSOC { return; }
    let pm = if (*sta).mesh.plink_state == NL80211_PLINK_ESTAB && (*sta).mesh.peer_pm != NL80211_MESH_POWER_UNKNOWN { (*sta).mesh.peer_pm } else { (*sta).mesh.nonpeer_pm };
    let do_buffer = pm != NL80211_MESH_POWER_ACTIVE;
    if (*sta).mesh.plink_state != NL80211_PLINK_ESTAB { clear_sta_flag(sta, WLAN_STA_MPSP_OWNER); clear_sta_flag(sta, WLAN_STA_MPSP_RECIPIENT); } else if !do_buffer { clear_sta_flag(sta, WLAN_STA_MPSP_OWNER); }
    if test_sta_flag(sta, WLAN_STA_PS_STA) == do_buffer { return; }
    if do_buffer { set_sta_flag(sta, WLAN_STA_PS_STA); atomic_inc(&mut (*sta).sdata.u.mesh.ps.num_sta_ps); mps_dbg((*sta).sdata, "start PS buffering frames towards %pM\n", (*sta).sta.addr); } else { ieee80211_sta_ps_deliver_wakeup(sta); }
}

// The remaining MPSP queue/release routines retain the C data-structure APIs.
unsafe fn mps_set_sta_peer_pm(sta: *mut sta_info, hdr: *mut ieee80211_hdr) { let qc = ieee80211_get_qos_ctl(hdr); let pm = if ieee80211_has_pm((*hdr).frame_control) { if *qc.add(1) & (IEEE80211_QOS_CTL_MESH_PS_LEVEL >> 8) != 0 { NL80211_MESH_POWER_DEEP_SLEEP } else { NL80211_MESH_POWER_LIGHT_SLEEP } } else { NL80211_MESH_POWER_ACTIVE }; if (*sta).mesh.peer_pm != pm { (*sta).mesh.peer_pm = pm; ieee80211_mps_sta_status_update(sta); } }
unsafe fn mps_set_sta_nonpeer_pm(sta: *mut sta_info, hdr: *mut ieee80211_hdr) { let pm = if ieee80211_has_pm((*hdr).frame_control) { NL80211_MESH_POWER_DEEP_SLEEP } else { NL80211_MESH_POWER_ACTIVE }; if (*sta).mesh.nonpeer_pm != pm { (*sta).mesh.nonpeer_pm = pm; ieee80211_mps_sta_status_update(sta); } }

pub unsafe fn ieee80211_mps_rx_h_sta_process(sta: *mut sta_info, hdr: *mut ieee80211_hdr) { if is_unicast_ether_addr((*hdr).addr1) && ieee80211_is_data_qos((*hdr).frame_control) { mps_set_sta_peer_pm(sta, hdr); ieee80211_mpsp_trigger_process(ieee80211_get_qos_ctl(hdr), sta, false, false); } else { mps_set_sta_nonpeer_pm(sta, hdr); } }

// MPSP trigger, frame delivery, and release logic.
unsafe fn mpsp_trigger_send(sta: *mut sta_info, rspi: bool, eosp: bool) { let sdata = (*sta).sdata; let skb = mps_qos_null_get(sta); if skb.is_null() { return; } let hdr = (*skb).data as *mut ieee80211_hdr; if !eosp { (*hdr).frame_control |= cpu_to_le16(IEEE80211_FCTL_MOREDATA); } let qc = ieee80211_get_qos_ctl(hdr); if rspi { *qc.add(1) |= IEEE80211_QOS_CTL_RSPI >> 8; } if eosp { *qc |= IEEE80211_QOS_CTL_EOSP; } let info = IEEE80211_SKB_CB(skb); (*info).flags |= IEEE80211_TX_CTL_NO_PS_BUFFER | IEEE80211_TX_CTL_REQ_TX_STATUS; ieee80211_tx_skb(sdata, skb); }
unsafe fn mpsp_qos_null_append(sta: *mut sta_info, frames: *mut sk_buff_head) { let skb = skb_peek_tail(frames); let hdr = (*skb).data as *mut ieee80211_hdr; if ieee80211_is_data_qos((*hdr).frame_control) { return; } let new_skb = mps_qos_null_get(sta); if new_skb.is_null() { return; } (*new_skb).priority = 1; skb_set_queue_mapping(new_skb, IEEE80211_AC_BK); ieee80211_set_qos_hdr((*sta).sdata, new_skb); let info = IEEE80211_SKB_CB(new_skb); (*info).control.vif = &mut (*(*sta).sdata).vif; (*info).control.flags |= IEEE80211_TX_INTCFL_NEED_TXPROCESSING; __skb_queue_tail(frames, new_skb); }
unsafe fn mps_frame_deliver(sta: *mut sta_info, mut n_frames: i32) { let local = (*(*sta).sdata).local; let mut frames: sk_buff_head = core::mem::zeroed(); skb_queue_head_init(&mut frames); for ac in 0..IEEE80211_NUM_ACS { while n_frames != 0 { let mut skb = skb_dequeue(&mut (*sta).tx_filtered[ac]); if skb.is_null() { skb = skb_dequeue(&mut (*sta).ps_tx_buf[ac]); if !skb.is_null() { (*local).total_ps_buffered -= 1; } } if skb.is_null() { break; } n_frames -= 1; __skb_queue_tail(&mut frames, skb); } } if skb_queue_empty(&frames) { mpsp_trigger_send(sta, false, true); return; } if test_sta_flag(sta, WLAN_STA_MPSP_OWNER) { mpsp_qos_null_append(sta, &mut frames); } let mut skb = (*frames).next; while !skb.is_null() && skb != &mut frames as *mut _ as *mut sk_buff { let info = IEEE80211_SKB_CB(skb); let hdr = (*skb).data as *mut ieee80211_hdr; (*info).flags |= IEEE80211_TX_CTL_NO_PS_BUFFER; (*hdr).frame_control |= cpu_to_le16(IEEE80211_FCTL_MOREDATA); skb = (*skb).next; } ieee80211_add_pending_skbs(local, &mut frames); sta_info_recalc_tim(sta); }
pub unsafe fn ieee80211_mpsp_trigger_process(qc: *mut u8, sta: *mut sta_info, tx: bool, acked: bool) { let rspi = *qc.add(1) & (IEEE80211_QOS_CTL_RSPI >> 8); let eosp = *qc & IEEE80211_QOS_CTL_EOSP; if tx { if rspi != 0 && acked { set_sta_flag(sta, WLAN_STA_MPSP_RECIPIENT); } if eosp != 0 { clear_sta_flag(sta, WLAN_STA_MPSP_OWNER); } else if acked && test_sta_flag(sta, WLAN_STA_PS_STA) && !test_and_set_sta_flag(sta, WLAN_STA_MPSP_OWNER) { mps_frame_deliver(sta, -1); } } else { if eosp != 0 { clear_sta_flag(sta, WLAN_STA_MPSP_RECIPIENT); } else if (*sta).mesh.local_pm != NL80211_MESH_POWER_ACTIVE { set_sta_flag(sta, WLAN_STA_MPSP_RECIPIENT); } if rspi != 0 && !test_and_set_sta_flag(sta, WLAN_STA_MPSP_OWNER) { mps_frame_deliver(sta, -1); } } }

pub unsafe fn ieee80211_mps_frame_release(sta: *mut sta_info, elems: *mut ieee802_11_elems) { let mut buffer_local = 0; let has_buffered = (*sta).mesh.plink_state == NL80211_PLINK_ESTAB && ieee80211_check_tim((*elems).tim, (*elems).tim_len, (*sta).mesh.aid, false); if test_sta_flag(sta, WLAN_STA_PS_STA) && ((*elems).awake_window.is_null() || get_unaligned_le16((*elems).awake_window) == 0) { return; } if !test_sta_flag(sta, WLAN_STA_MPSP_OWNER) { for ac in 0..IEEE80211_NUM_ACS { buffer_local += skb_queue_len(&(*sta).ps_tx_buf[ac]) + skb_queue_len(&(*sta).tx_filtered[ac]); } } if !has_buffered && buffer_local == 0 { return; } if (*sta).mesh.plink_state == NL80211_PLINK_ESTAB { mpsp_trigger_send(sta, has_buffered, buffer_local == 0); } else { mps_frame_deliver(sta, 1); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
