// SPDX-License-Identifier: GPL-2.0-only
/* Direct source-level translation of status.c. Kernel/mac80211 dependencies are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const STA_LOST_PKT_THRESHOLD: u32 = 50;
const STA_LOST_PKT_TIME: usize = HZ;
const STA_LOST_TDLS_PKT_TIME: usize = 10 * HZ;

pub unsafe fn ieee80211_tx_status_irqsafe(hw: *mut ieee80211_hw, mut skb: *mut sk_buff) {
    let local = hw_to_local(hw);
    let info = IEEE80211_SKB_CB(skb);
    (*skb).pkt_type = IEEE80211_TX_STATUS_MSG;
    skb_queue_tail(if (*info).flags & IEEE80211_TX_CTL_REQ_TX_STATUS != 0 { &mut (*local).skb_queue } else { &mut (*local).skb_queue_unreliable }, skb);
    let mut tmp = skb_queue_len(&(*local).skb_queue) + skb_queue_len(&(*local).skb_queue_unreliable);
    while tmp > IEEE80211_IRQSAFE_QUEUE_LIMIT {
        skb = skb_dequeue(&mut (*local).skb_queue_unreliable);
        if skb.is_null() { break; }
        ieee80211_free_txskb(hw, skb); tmp -= 1; I802_DEBUG_INC((*local).tx_status_drop);
    }
    tasklet_schedule(&mut (*local).tasklet);
}

unsafe fn ieee80211_handle_filtered_frame(local: *mut ieee80211_local, sta: *mut sta_info, skb: *mut sk_buff) {
    let info = IEEE80211_SKB_CB(skb); let hdr = (*skb).data as *mut ieee80211_hdr;
    if (*info).flags & (IEEE80211_TX_CTL_NO_PS_BUFFER | IEEE80211_TX_CTL_AMPDU | IEEE80211_TX_CTL_HW_80211_ENCAP) != 0 { ieee80211_free_txskb(&mut (*local).hw, skb); return; }
    memset(&mut (*info).control as *mut _, 0, size_of::<ieee80211_tx_control>());
    (*info).control.jiffies = jiffies; (*info).control.vif = &mut (*(*sta).sdata).vif;
    (*info).control.flags |= IEEE80211_TX_INTCFL_NEED_TXPROCESSING;
    (*info).flags |= IEEE80211_TX_INTFL_RETRANSMISSION; (*info).flags &= !IEEE80211_TX_TEMPORARY_FLAGS;
    (*sta).deflink.status_stats.filtered += 1;
    if (*hdr).frame_control & cpu_to_le16(IEEE80211_FCTL_MOREDATA) != 0 { (*hdr).frame_control &= !cpu_to_le16(IEEE80211_FCTL_MOREDATA); }
    let ac: i32;
    if ieee80211_is_data_qos((*hdr).frame_control) { let p = ieee80211_get_qos_ctl(hdr); let tid = (*p & IEEE80211_QOS_CTL_TID_MASK) as i32; if *p & IEEE80211_QOS_CTL_EOSP != 0 { *p &= !IEEE80211_QOS_CTL_EOSP; } ac = ieee80211_ac_from_tid(tid); } else { ac = IEEE80211_AC_BE; }
    set_sta_flag(sta, WLAN_STA_CLEAR_PS_FILT); ieee80211_clear_fast_xmit(sta);
    if test_sta_flag(sta, WLAN_STA_PS_STA) && skb_queue_len(&(*sta).tx_filtered[ac as usize]) < STA_MAX_TX_BUFFER { skb_queue_tail(&mut (*sta).tx_filtered[ac as usize], skb); sta_info_recalc_tim(sta); if !timer_pending(&(*local).sta_cleanup) { mod_timer(&mut (*local).sta_cleanup, round_jiffies(jiffies + STA_INFO_CLEANUP_INTERVAL)); } return; }
    if !test_sta_flag(sta, WLAN_STA_PS_STA) && (*info).flags & IEEE80211_TX_INTFL_RETRIED == 0 { (*info).flags |= IEEE80211_TX_INTFL_RETRIED; ieee80211_add_pending_skb(local, skb); return; }
    ps_dbg_ratelimited((*sta).sdata, "dropped TX filtered frame, queue_len=%d PS=%d @%lu\n", skb_queue_len(&(*sta).tx_filtered[ac as usize]), test_sta_flag(sta, WLAN_STA_PS_STA), jiffies); ieee80211_free_txskb(&mut (*local).hw, skb);
}

unsafe fn ieee80211_check_pending_bar(sta: *mut sta_info, addr: *mut u8, tid: u8) { let tx = rcu_dereference((*sta).ampdu_mlme.tid_tx[tid as usize]); if tx.is_null() || !(*tx).bar_pending { return; } (*tx).bar_pending = false; ieee80211_send_bar(&(*(*sta).sdata).vif, addr, tid, (*tx).failed_bar_ssn); }
unsafe fn ieee80211_frame_acked(sta: *mut sta_info, skb: *mut sk_buff) { let hdr = (*skb).data as *mut ieee80211_hdr; if ieee80211_is_data_qos((*hdr).frame_control) { let qc = ieee80211_get_qos_ctl(hdr); ieee80211_check_pending_bar(sta, (*hdr).addr1.as_mut_ptr(), *qc & 0xf); } }
unsafe fn ieee80211_set_bar_pending(sta: *mut sta_info, tid: u8, ssn: u16) { let tx = rcu_dereference((*sta).ampdu_mlme.tid_tx[tid as usize]); if !tx.is_null() { (*tx).failed_bar_ssn = ssn; (*tx).bar_pending = true; } }

unsafe fn ieee80211_tx_get_rates(hw: *mut ieee80211_hw, info: *mut ieee80211_tx_info, retry_count: *mut i32) -> i32 { let mut count = -1; let mut i = 0; while i < IEEE80211_TX_MAX_RATES { if (*info).flags & IEEE80211_TX_CTL_AMPDU != 0 && (*info).flags & IEEE80211_TX_STAT_AMPDU == 0 { (*info).status.rates[i].idx = -1; (*info).status.rates[i].count = 0; break; } else if (*info).status.rates[i].idx < 0 { break; } else if i >= (*hw).max_report_rates as usize { (*info).status.rates[i].idx = -1; (*info).status.rates[i].count = 0; break; } count += (*info).status.rates[i].count as i32; i += 1; } if count < 0 { count = 0; } *retry_count = count; i as i32 - 1 }

unsafe fn ieee80211_lost_packet(sta: *mut sta_info, info: *mut ieee80211_tx_info) { if ieee80211_hw_check(&(*sta).local.hw, REPORTS_LOW_ACK) || ((*info).flags & IEEE80211_TX_CTL_AMPDU != 0 && (*info).flags & IEEE80211_TX_STAT_AMPDU == 0) { return; } (*sta).deflink.status_stats.lost_packets += 1; let (time, thr) = if (*sta).sta.tdls { (STA_LOST_TDLS_PKT_TIME, STA_LOST_PKT_THRESHOLD) } else { (STA_LOST_PKT_TIME, STA_LOST_PKT_THRESHOLD) }; if (*sta).deflink.status_stats.lost_packets < thr || !time_after(jiffies, (*sta).deflink.status_stats.last_pkt_time + time) { return; } cfg80211_cqm_pktloss_notify((*sta).sdata).dev, (*sta).sta.addr, (*sta).deflink.status_stats.lost_packets, GFP_ATOMIC); (*sta).deflink.status_stats.lost_packets = 0; }

pub unsafe fn ieee80211_tx_status_skb(hw: *mut ieee80211_hw, skb: *mut sk_buff) { let hdr = (*skb).data as *mut ieee80211_hdr; let local = hw_to_local(hw); let mut status = ieee80211_tx_status { skb, info: IEEE80211_SKB_CB(skb), ..zeroed() }; rcu_read_lock(); let sta = sta_info_get_by_addrs(local, (*hdr).addr1.as_mut_ptr(), (*hdr).addr2.as_mut_ptr()); if !sta.is_null() { status.sta = &mut (*sta).sta; } ieee80211_tx_status_ext(hw, &mut status); rcu_read_unlock(); }

pub unsafe fn ieee80211_tx_status_ext(hw: *mut ieee80211_hw, status: *mut ieee80211_tx_status) { let local = hw_to_local(hw); let info = (*status).info; let skb = (*status).skb; let pubsta = (*status).sta; let mut retry = 0; let rates = ieee80211_tx_get_rates(hw, info, &mut retry); let acked = (*info).flags & IEEE80211_TX_STAT_ACK != 0; let noack = (*info).flags & IEEE80211_TX_STAT_NOACK_TRANSMITTED != 0; if !skb.is_null() && (*info).flags & IEEE80211_TX_CTL_HW_80211_ENCAP == 0 { __ieee80211_tx_status(hw, status, rates, retry); return; } if acked || noack { I802_DEBUG_INC((*local).dot11TransmittedFrameCount); if pubsta.is_null() { I802_DEBUG_INC((*local).dot11MulticastTransmittedFrameCount); } if retry > 0 { I802_DEBUG_INC((*local).dot11RetryCount); } if retry > 1 { I802_DEBUG_INC((*local).dot11MultipleRetryCount); } } else { I802_DEBUG_INC((*local).dot11FailedCount); } if !skb.is_null() { ieee80211_report_used_skb(local, skb, false, (*status).ack_hwtstamp); if (*status).free_list.is_null() { dev_kfree_skb(skb); } else { list_add_tail(&mut (*skb).list, (*status).free_list); } } }

pub unsafe fn ieee80211_tx_rate_update(hw: *mut ieee80211_hw, pubsta: *mut ieee80211_sta, info: *mut ieee80211_tx_info) { let local = hw_to_local(hw); let sta = container_of(pubsta, sta_info, sta); let mut status = ieee80211_tx_status { info, sta: pubsta, ..zeroed() }; rate_control_tx_status(local, &mut status); if ieee80211_hw_check(&(*local).hw, HAS_RATE_CONTROL) { (*sta).deflink.tx_stats.last_rate = (*info).status.rates[0]; } }
pub unsafe fn ieee80211_report_low_ack(pubsta: *mut ieee80211_sta, num_packets: u32) { let sta = container_of(pubsta, sta_info, sta); cfg80211_cqm_pktloss_notify((*sta).sdata).dev, (*sta).sta.addr, num_packets, GFP_ATOMIC); }
pub unsafe fn ieee80211_free_txskb(hw: *mut ieee80211_hw, skb: *mut sk_buff) { let local = hw_to_local(hw); ieee80211_report_used_skb(local, skb, true, ktime_set(0, 0)); dev_kfree_skb_any(skb); }
pub unsafe fn ieee80211_purge_tx_queue(hw: *mut ieee80211_hw, skbs: *mut sk_buff_head) { loop { let skb = __skb_dequeue(skbs); if skb.is_null() { break; } ieee80211_free_txskb(hw, skb); } }

// Remaining file-local helpers retain their exact kernel-facing interfaces and are declared externally.
extern "C" {
    fn __ieee80211_tx_status(hw: *mut ieee80211_hw, status: *mut ieee80211_tx_status, rates_idx: i32, retry_count: i32);
    fn ieee80211_report_used_skb(local: *mut ieee80211_local, skb: *mut sk_buff, dropped: bool, stamp: ktime_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
