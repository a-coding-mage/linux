// SPDX-License-Identifier: GPL-2.0-only
/* Off-channel operation helpers.  Kernel/project dependencies are supplied externally. */

unsafe fn ieee80211_offchannel_ps_enable(sdata: *mut ieee80211_sub_if_data) {
    let local = (*sdata).local;
    let ifmgd = &mut (*sdata).u.mgd;
    let mut offchannel_ps_enabled = false;
    timer_delete_sync(&mut (*local).dynamic_ps_timer);
    timer_delete_sync(&mut ifmgd.bcn_mon_timer);
    timer_delete_sync(&mut ifmgd.conn_mon_timer);
    wiphy_work_cancel((*local).hw.wiphy, &mut (*local).dynamic_ps_enable_work);
    if (*local).hw.conf.flags & IEEE80211_CONF_PS != 0 {
        offchannel_ps_enabled = true;
        (*local).hw.conf.flags &= !IEEE80211_CONF_PS;
        ieee80211_hw_config(local, -1, IEEE80211_CONF_CHANGE_PS);
    }
    if !offchannel_ps_enabled || !ieee80211_hw_check(&(*local).hw, PS_NULLFUNC_STACK) {
        ieee80211_send_nullfunc(local, sdata, true);
    }
}

unsafe fn ieee80211_offchannel_ps_disable(sdata: *mut ieee80211_sub_if_data) {
    let local = (*sdata).local;
    if (*local).ps_sdata.is_null() {
        ieee80211_send_nullfunc(local, sdata, false);
    } else if (*local).hw.conf.dynamic_ps_timeout > 0 {
        ieee80211_send_nullfunc(local, sdata, false);
        mod_timer(&mut (*local).dynamic_ps_timer,
                  jiffies + msecs_to_jiffies((*local).hw.conf.dynamic_ps_timeout));
    }
    ieee80211_sta_reset_beacon_monitor(sdata);
    ieee80211_sta_reset_conn_monitor(sdata);
}

pub unsafe fn ieee80211_offchannel_stop_vifs(local: *mut ieee80211_local) {
    lockdep_assert_wiphy((*local).hw.wiphy);
    if WARN_ON(!(*local).emulate_chanctx) { return; }
    ieee80211_stop_queues_by_reason(&mut (*local).hw, IEEE80211_MAX_QUEUE_MAP,
                                    IEEE80211_QUEUE_STOP_REASON_OFFCHANNEL, false);
    ieee80211_flush_queues(local, core::ptr::null_mut(), false);
    let mut sdata = (*local).interfaces.next as *mut ieee80211_sub_if_data;
    while sdata != &mut (*local).interfaces as *mut _ as *mut ieee80211_sub_if_data {
        let next = (*sdata).list.next as *mut ieee80211_sub_if_data;
        if ieee80211_sdata_running(sdata) &&
           (*sdata).vif.type_ != NL80211_IFTYPE_P2P_DEVICE &&
           (*sdata).vif.type_ != NL80211_IFTYPE_NAN {
            if (*sdata).vif.type_ != NL80211_IFTYPE_MONITOR { set_bit(SDATA_STATE_OFFCHANNEL, &mut (*sdata).state); }
            if (*sdata).vif.bss_conf.enable_beacon {
                set_bit(SDATA_STATE_OFFCHANNEL_BEACON_STOPPED, &mut (*sdata).state);
                (*sdata).vif.bss_conf.enable_beacon = false;
                ieee80211_link_info_change_notify(sdata, &mut (*sdata).deflink, BSS_CHANGED_BEACON_ENABLED);
            }
            if (*sdata).vif.type_ == NL80211_IFTYPE_STATION && (*sdata).u.mgd.associated {
                ieee80211_offchannel_ps_enable(sdata);
            }
        }
        sdata = next;
    }
}

pub unsafe fn ieee80211_offchannel_return(local: *mut ieee80211_local) {
    lockdep_assert_wiphy((*local).hw.wiphy);
    if WARN_ON(!(*local).emulate_chanctx) { return; }
    let mut sdata = (*local).interfaces.next as *mut ieee80211_sub_if_data;
    while sdata != &mut (*local).interfaces as *mut _ as *mut ieee80211_sub_if_data {
        let next = (*sdata).list.next as *mut ieee80211_sub_if_data;
        if (*sdata).vif.type_ != NL80211_IFTYPE_P2P_DEVICE {
            if (*sdata).vif.type_ != NL80211_IFTYPE_MONITOR { clear_bit(SDATA_STATE_OFFCHANNEL, &mut (*sdata).state); }
            if ieee80211_sdata_running(sdata) {
                if (*sdata).vif.type_ == NL80211_IFTYPE_STATION && (*sdata).u.mgd.associated { ieee80211_offchannel_ps_disable(sdata); }
                if test_and_clear_bit(SDATA_STATE_OFFCHANNEL_BEACON_STOPPED, &mut (*sdata).state) {
                    (*sdata).vif.bss_conf.enable_beacon = true;
                    ieee80211_link_info_change_notify(sdata, &mut (*sdata).deflink, BSS_CHANGED_BEACON_ENABLED);
                }
            }
        }
        sdata = next;
    }
    ieee80211_wake_queues_by_reason(&mut (*local).hw, IEEE80211_MAX_QUEUE_MAP,
                                    IEEE80211_QUEUE_STOP_REASON_OFFCHANNEL, false);
}

unsafe fn ieee80211_roc_notify_destroy(roc: *mut ieee80211_roc_work) {
    if !(*roc).frame.is_null() {
        cfg80211_mgmt_tx_status(&mut (*(*roc).sdata).wdev, (*roc).mgmt_tx_cookie,
            (*(*roc).frame).data, (*(*roc).frame).len, false, GFP_KERNEL);
        ieee80211_free_txskb(&mut (*(*(*roc).sdata).local).hw, (*roc).frame);
    }
    if (*roc).mgmt_tx_cookie == 0 { cfg80211_remain_on_channel_expired(&mut (*(*roc).sdata).wdev, (*roc).cookie, (*roc).chan, GFP_KERNEL); }
    else { cfg80211_tx_mgmt_expired(&mut (*(*roc).sdata).wdev, (*roc).mgmt_tx_cookie, (*roc).chan, GFP_KERNEL); }
    list_del(&mut (*roc).list); kfree(roc as *mut core::ffi::c_void);
}

unsafe fn ieee80211_end_finished_rocs(local: *mut ieee80211_local, now: c_ulong) -> c_ulong {
    let mut min = LONG_MAX as c_long;
    let mut roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    while roc != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work {
        let next = (*roc).list.next as *mut ieee80211_roc_work;
        if !(*roc).started { break; }
        let remaining = (*roc).start_time + msecs_to_jiffies((*roc).duration) - now;
        if (*roc).abort || (*roc).hw_begun || remaining <= 0 { ieee80211_roc_notify_destroy(roc); }
        else { min = core::cmp::min(min, remaining as c_long); }
        roc = next;
    }
    min as c_ulong
}

unsafe fn ieee80211_recalc_sw_work(local: *mut ieee80211_local, now: c_ulong) -> bool {
    let dur = ieee80211_end_finished_rocs(local, now);
    if dur as c_long == LONG_MAX { return false; }
    wiphy_delayed_work_queue((*local).hw.wiphy, &mut (*local).roc_work, dur); true
}

unsafe fn ieee80211_handle_roc_started(roc: *mut ieee80211_roc_work, start_time: c_ulong) {
    if WARN_ON((*roc).notified) { return; }
    (*roc).start_time = start_time; (*roc).started = true;
    if (*roc).mgmt_tx_cookie != 0 {
        if !WARN_ON((*roc).frame.is_null()) { ieee80211_tx_skb_tid_band((*roc).sdata, (*roc).frame, 7, (*(*roc).chan).band); (*roc).frame = core::ptr::null_mut(); }
    } else { cfg80211_ready_on_channel(&mut (*(*roc).sdata).wdev, (*roc).cookie, (*roc).chan, (*roc).req_duration, GFP_KERNEL); }
    (*roc).notified = true;
}

// Remaining callbacks and entry points retain the kernel ABI and are translated below.
pub unsafe fn ieee80211_ready_on_channel(hw: *mut ieee80211_hw) {
    let local = hw_to_local(hw); (*local).hw_roc_start_time = jiffies;
    trace_api_ready_on_channel(local); wiphy_work_queue((*hw).wiphy, &mut (*local).hw_roc_start);
}

pub unsafe fn ieee80211_remain_on_channel_expired(hw: *mut ieee80211_hw) {
    let local = hw_to_local(hw); trace_api_remain_on_channel_expired(local);
    wiphy_work_queue((*hw).wiphy, &mut (*local).hw_roc_done);
}

pub unsafe fn ieee80211_roc_setup(local: *mut ieee80211_local) {
    wiphy_work_init(&mut (*local).hw_roc_start, ieee80211_hw_roc_start);
    wiphy_work_init(&mut (*local).hw_roc_done, ieee80211_hw_roc_done);
    wiphy_delayed_work_init(&mut (*local).roc_work, ieee80211_roc_work);
    INIT_LIST_HEAD(&mut (*local).roc_list);
}

unsafe fn ieee80211_hw_roc_start(_wiphy: *mut wiphy, work: *mut wiphy_work) {
    let local = container_of!(work, ieee80211_local, hw_roc_start);
    let mut roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    while roc != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work {
        if !(*roc).started { break; }
        (*roc).hw_begun = true;
        ieee80211_handle_roc_started(roc, (*local).hw_roc_start_time);
        roc = (*roc).list.next as *mut ieee80211_roc_work;
    }
}

unsafe fn ieee80211_hw_roc_done(_wiphy: *mut wiphy, _work: *mut wiphy_work) {
    let local = container_of!(_work, ieee80211_local, hw_roc_done);
    ieee80211_end_finished_rocs(local, jiffies); ieee80211_start_next_roc(local);
}

unsafe fn ieee80211_roc_work(_wiphy: *mut wiphy, work: *mut wiphy_work) {
    let local = container_of!(work, ieee80211_local, roc_work.work);
    __ieee80211_roc_work(local);
}

unsafe fn __ieee80211_roc_work(local: *mut ieee80211_local) {
    if !(*local).ops.as_ref().unwrap().remain_on_channel.is_none() { return; }
    let roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    if roc == &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work { return; }
    if !(*roc).started { _ieee80211_start_next_roc(local); return; }
    let on_channel = (*roc).on_channel;
    if ieee80211_recalc_sw_work(local, jiffies) { return; }
    if !on_channel {
        ieee80211_flush_queues(local, core::ptr::null_mut(), false);
        (*local).tmp_channel = core::ptr::null_mut(); ieee80211_hw_conf_chan(local);
        ieee80211_offchannel_return(local);
    }
    ieee80211_recalc_idle(local); ieee80211_start_next_roc(local);
}

unsafe fn _ieee80211_start_next_roc(local: *mut ieee80211_local) {
    let roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    if roc == &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work || (*roc).started { return; }
    let mut min_dur = (*roc).duration;
    let mut tmp = (*roc).list.next as *mut ieee80211_roc_work;
    while tmp != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work {
        if (*tmp).sdata != (*roc).sdata || (*tmp).chan != (*roc).chan { break; }
        min_dur = core::cmp::min(min_dur, (*tmp).duration); (*tmp).started = true;
        tmp = (*tmp).list.next as *mut ieee80211_roc_work;
    }
    if !(*local).ops.as_ref().unwrap().remain_on_channel.is_none() {
        if drv_remain_on_channel(local, (*roc).sdata, (*roc).chan, (*roc).duration, (*roc).type_) != 0 {
            (*roc).started = true; (*roc).abort = true;
            wiphy_work_queue((*local).hw.wiphy, &mut (*local).hw_roc_done); return;
        }
        (*roc).started = true;
    } else {
        (*roc).on_channel = (*roc).chan == (*local).hw.conf.chandef.chan;
        ieee80211_recalc_idle(local);
        if !(*roc).on_channel { ieee80211_offchannel_stop_vifs(local); (*local).tmp_channel = (*roc).chan; ieee80211_hw_conf_chan(local); }
        wiphy_delayed_work_queue((*local).hw.wiphy, &mut (*local).roc_work, msecs_to_jiffies(min_dur));
        let mut p = roc;
        while p != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work && (*p).sdata == (*roc).sdata && (*p).chan == (*roc).chan {
            (*p).on_channel = (*roc).on_channel; ieee80211_handle_roc_started(p, jiffies); p = (*p).list.next as *mut _;
        }
    }
}

pub unsafe fn ieee80211_start_next_roc(local: *mut ieee80211_local) {
    if (*local).roc_list.next == &mut (*local).roc_list as *mut _ { ieee80211_run_deferred_scan(local); return; }
    if (*local).in_reconfig { return; }
    let roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    if (*roc).started { return; }
    if !(*local).ops.as_ref().unwrap().remain_on_channel.is_none() { _ieee80211_start_next_roc(local); }
    else { wiphy_delayed_work_queue((*local).hw.wiphy, &mut (*local).roc_work, round_jiffies_relative(HZ / 2)); }
}

pub unsafe fn ieee80211_reconfig_roc(local: *mut ieee80211_local) {
    if (*local).ops.as_ref().unwrap().remain_on_channel.is_none() { return; }
    wiphy_work_flush((*local).hw.wiphy, &mut (*local).hw_roc_start);
    wiphy_work_flush((*local).hw.wiphy, &mut (*local).hw_roc_done);
    let mut roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    while roc != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work {
        let next = (*roc).list.next as *mut ieee80211_roc_work;
        if !(*roc).started { break; }
        if !(*roc).hw_begun { (*roc).started = false; } else { ieee80211_roc_notify_destroy(roc); }
        roc = next;
    }
    ieee80211_start_next_roc(local);
}

pub unsafe fn ieee80211_roc_purge(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) {
    let mut roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    while roc != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work {
        let next = (*roc).list.next as *mut ieee80211_roc_work;
        if sdata.is_null() || (*roc).sdata == sdata {
            if (*roc).started && !(*local).ops.as_ref().unwrap().remain_on_channel.is_none() { drv_cancel_remain_on_channel(local, (*roc).sdata); }
            else if (*roc).started { (*roc).abort = true; __ieee80211_roc_work(local); }
            else { ieee80211_roc_notify_destroy(roc); }
        }
        roc = next;
    }
}

unsafe fn ieee80211_start_roc_work(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, channel: *mut ieee80211_channel, duration: u32, cookie: *mut u64, txskb: *mut sk_buff, type_: ieee80211_roc_type) -> i32 {
    if (*channel).freq_offset != 0 { return -EOPNOTSUPP; }
    if !(*local).emulate_chanctx && (*local).ops.as_ref().unwrap().remain_on_channel.is_none() { return -EOPNOTSUPP; }
    let roc = kzalloc_obj::<ieee80211_roc_work>(); if roc.is_null() { return -ENOMEM; }
    (*roc).chan = channel; (*roc).duration = if duration == 0 { 10 } else { duration }; (*roc).req_duration = (*roc).duration;
    (*roc).frame = txskb; (*roc).type_ = type_; (*roc).sdata = sdata;
    if txskb.is_null() { (*roc).cookie = *cookie; } else { (*roc).mgmt_tx_cookie = *cookie; }
    list_add_tail(&mut (*roc).list, &mut (*local).roc_list);
    if (*local).ops.as_ref().unwrap().remain_on_channel.is_none() { wiphy_delayed_work_queue((*local).hw.wiphy, &mut (*local).roc_work, 0); }
    else { let ret = drv_remain_on_channel(local, sdata, channel, (*roc).duration, type_); if ret != 0 { kfree(roc as *mut _); return ret; } (*roc).started = true; }
    0
}

pub unsafe fn ieee80211_remain_on_channel(wiphy: *mut wiphy, wdev: *mut wireless_dev, chan: *mut ieee80211_channel, duration: u32, cookie: u64, _rx_addr: *const u8) -> i32 {
    let sdata = IEEE80211_WDEV_TO_SUB_IF(wdev); ieee80211_start_roc_work((*sdata).local, sdata, chan, duration, &cookie as *const _ as *mut _, core::ptr::null_mut(), IEEE80211_ROC_TYPE_NORMAL)
}

unsafe fn ieee80211_cancel_roc(local: *mut ieee80211_local, cookie: u64, mgmt_tx: bool) -> i32 {
    if cookie == 0 { return -ENOENT; }
    let mut roc = (*local).roc_list.next as *mut ieee80211_roc_work;
    while roc != &mut (*local).roc_list as *mut _ as *mut ieee80211_roc_work {
        if (!mgmt_tx && (*roc).cookie == cookie) || (mgmt_tx && (*roc).mgmt_tx_cookie == cookie) {
            if !(*roc).started { ieee80211_roc_notify_destroy(roc); return 0; }
            if !(*local).ops.as_ref().unwrap().remain_on_channel.is_none() { drv_cancel_remain_on_channel(local, (*roc).sdata); ieee80211_roc_notify_destroy(roc); ieee80211_start_next_roc(local); }
            else { (*roc).abort = true; wiphy_delayed_work_queue((*local).hw.wiphy, &mut (*local).roc_work, 0); }
            return 0;
        }
        roc = (*roc).list.next as *mut _;
    }
    -ENOENT
}

pub unsafe fn ieee80211_cancel_remain_on_channel(_wiphy: *mut wiphy, wdev: *mut wireless_dev, cookie: u64) -> i32 { ieee80211_cancel_roc((*IEEE80211_WDEV_TO_SUB_IF(wdev)).local, cookie, false) }
pub unsafe fn ieee80211_mgmt_tx_cancel_wait(wiphy: *mut wiphy, _wdev: *mut wireless_dev, cookie: u64) -> i32 { ieee80211_cancel_roc(wiphy_priv(wiphy), cookie, true) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
