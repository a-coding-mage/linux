// SPDX-License-Identifier: GPL-2.0-only
/* NAN mode implementation */

// C dependencies and build-time definitions are supplied by the surrounding kernel translation.

unsafe fn ieee80211_nan_init_channel(nan_channel: *mut ieee80211_nan_channel, cfg: *mut cfg80211_nan_channel) {
    memset(nan_channel as *mut _, 0, core::mem::size_of::<ieee80211_nan_channel>());
    (*nan_channel).chanreq.oper = (*cfg).chandef;
    memcpy((*nan_channel).channel_entry.as_mut_ptr() as *mut _, (*cfg).channel_entry.as_ptr() as *const _, core::mem::size_of_val(&(*nan_channel).channel_entry));
    (*nan_channel).needed_rx_chains = (*cfg).rx_nss;
}

unsafe fn ieee80211_nan_update_channel(local: *mut ieee80211_local, nan: *mut ieee80211_nan_channel, cfg: *mut cfg80211_nan_channel, deferred: bool) {
    let conf: *mut ieee80211_chanctx_conf;
    let reducing_nss: bool;
    if warn_on(!cfg80211_chandef_identical(&(*nan).chanreq.oper, &(*cfg).chandef)) || warn_on(memcmp((*nan).channel_entry.as_ptr() as *const _, (*cfg).channel_entry.as_ptr() as *const _, core::mem::size_of_val(&(*nan).channel_entry)) != 0) { return; }
    if (*nan).needed_rx_chains == (*cfg).rx_nss { return; }
    reducing_nss = (*nan).needed_rx_chains > (*cfg).rx_nss;
    (*nan).needed_rx_chains = (*cfg).rx_nss;
    conf = (*nan).chanctx_conf;
    if conf.is_null() || (deferred && reducing_nss) { return; }
    ieee80211_recalc_smps_chanctx(local, container_of(conf, ieee80211_chanctx, conf));
}

unsafe fn ieee80211_nan_use_chanctx(sdata: *mut ieee80211_sub_if_data, nan: *mut ieee80211_nan_channel, assign_on_failure: bool) -> i32 {
    if (*nan).chanreq.oper.chan.is_null() { return -EINVAL; }
    if ieee80211_check_combinations(sdata, &(*nan).chanreq.oper, IEEE80211_CHANCTX_SHARED, 0, -1) != 0 { return -EBUSY; }
    let mut reused = false;
    let ctx = ieee80211_find_or_create_chanctx(sdata, &mut (*nan).chanreq, IEEE80211_CHANCTX_SHARED, assign_on_failure, &mut reused);
    if is_err(ctx) { return ptr_err(ctx); }
    (*nan).chanctx_conf = &mut (*ctx).conf;
    if reused { warn_on(!(*ctx).will_be_used); (*ctx).will_be_used = false; }
    ieee80211_recalc_chanctx_min_def((*sdata).local, ctx);
    ieee80211_recalc_smps_chanctx((*sdata).local, ctx);
    0
}

unsafe fn ieee80211_nan_update_peer_channels(sdata: *mut ieee80211_sub_if_data, removed: *mut ieee80211_chanctx_conf) {
    let local = (*sdata).local;
    lockdep_assert_wiphy((*local).hw.wiphy);
    let mut sta = (*local).sta_list.next as *mut sta_info;
    while sta != (*local).sta_list as *mut _ {
        if (*sta).sdata == sdata {
            let sched = (*sta).sta.nan_sched;
            if !sched.is_null() {
                let mut write_idx = 0usize;
                let mut updated = false;
                for i in 0..(*sched).n_channels as usize {
                    if (*sched).channels[i].chanctx_conf == removed {
                        for m in 0..CFG80211_NAN_MAX_PEER_MAPS { let map = &mut (*sched).maps[m]; if map.map_id != CFG80211_NAN_INVALID_MAP_ID { for slot in 0..map.slots.len() { if map.slots[slot] == &mut (*sched).channels[i] { map.slots[slot] = core::ptr::null_mut(); } } } }
                    }
                }
                for i in 0..(*sched).n_channels as usize {
                    if (*sched).channels[i].chanctx_conf == removed { updated = true; continue; }
                    if write_idx != i { for m in 0..CFG80211_NAN_MAX_PEER_MAPS { let map = &mut (*sched).maps[m]; if map.map_id != CFG80211_NAN_INVALID_MAP_ID { for slot in 0..map.slots.len() { if map.slots[slot] == &mut (*sched).channels[i] { map.slots[slot] = &mut (*sched).channels[write_idx]; } } } } (*sched).channels[write_idx] = (*sched).channels[i]; }
                    write_idx += 1;
                }
                for i in write_idx..(*sched).n_channels as usize { memset(&mut (*sched).channels[i] as *mut _ as *mut _, 0, core::mem::size_of::<ieee80211_nan_channel>()); }
                (*sched).n_channels = write_idx as _;
                if updated { drv_nan_peer_sched_changed(local, sdata, sta); }
            }
        }
        sta = (*sta).list.next as *mut sta_info;
    }
}

unsafe fn ieee80211_nan_remove_channel(sdata: *mut ieee80211_sub_if_data, nan: *mut ieee80211_nan_channel) {
    if warn_on(nan.is_null()) { return; }
    lockdep_assert_wiphy((*sdata).local.hw.wiphy);
    if (*nan).chanreq.oper.chan.is_null() { return; }
    let sched = &mut (*sdata).vif.cfg.nan_sched;
    for slot in 0..sched.schedule.len() { if sched.schedule[slot] == nan { sched.schedule[slot] = core::ptr::null_mut(); } }
    let conf = (*nan).chanctx_conf;
    if !conf.is_null() { ieee80211_nan_update_peer_channels(sdata, conf); }
    memset(nan as *mut _, 0, core::mem::size_of::<ieee80211_nan_channel>());
    drv_vif_cfg_changed((*sdata).local, sdata, BSS_CHANGED_NAN_LOCAL_SCHED);
    if conf.is_null() { return; }
    let ctx = container_of(conf, ieee80211_chanctx, conf);
    if ieee80211_chanctx_num_assigned((*sdata).local, ctx) > 0 { ieee80211_recalc_chanctx_chantype((*sdata).local, ctx); ieee80211_recalc_smps_chanctx((*sdata).local, ctx); ieee80211_recalc_chanctx_min_def((*sdata).local, ctx); }
    if ieee80211_chanctx_refcount((*sdata).local, ctx) == 0 { ieee80211_free_chanctx((*sdata).local, ctx, false); }
}

unsafe fn ieee80211_nan_update_all_ndi_carriers(local: *mut ieee80211_local) {
    lockdep_assert_wiphy((*local).hw.wiphy);
    let mut sdata = (*local).interfaces.next as *mut ieee80211_sub_if_data;
    while sdata != (*local).interfaces as *mut _ { if ieee80211_sdata_running(sdata) && (*sdata).vif.type_ == NL80211_IFTYPE_NAN_DATA { ieee80211_nan_update_ndi_carrier(sdata); } sdata = (*sdata).list.next as *mut _; }
}

unsafe fn ieee80211_nan_find_free_channel(cfg: *mut ieee80211_nan_sched_cfg) -> *mut ieee80211_nan_channel { for i in 0..(*cfg).channels.len() { if (*cfg).channels[i].chanreq.oper.chan.is_null() { return &mut (*cfg).channels[i]; } } core::ptr::null_mut() }

// Direct translation of the large transactional schedule update; allocation, bitmap, and rollback helpers are external kernel primitives.
#[no_mangle]
pub unsafe extern "C" fn ieee80211_nan_set_local_sched(sdata: *mut ieee80211_sub_if_data, sched: *mut cfg80211_nan_local_sched) -> i32 {
    let cfg = &mut (*sdata).vif.cfg.nan_sched;
    if (*sched).n_channels > IEEE80211_NAN_MAX_CHANNELS { return -EOPNOTSUPP; }
    if (*sched).nan_avail_blob_len > IEEE80211_NAN_AVAIL_BLOB_MAX_LEN { return -EINVAL; }
    if warn_on(cfg.deferred && (*sched).n_channels != 0) { return -EBUSY; }
    bitmap_zero((*sdata).u.nan.removed_channels.as_mut_ptr(), IEEE80211_NAN_MAX_CHANNELS);
    for i in 0..cfg.channels.len() { let old = &mut cfg.channels[i]; if old.chanreq.oper.chan.is_null() { continue; } let mut needed = false; for j in 0..(*sched).n_channels as usize { if cfg80211_chandef_identical(&old.chanreq.oper, &(*sched).nan_channels[j].chandef) { needed = true; break; } } if !needed && !(*sched).deferred { ieee80211_nan_remove_channel(sdata, old); } }
    for i in 0..(*sched).n_channels as usize { let mut chan = ieee80211_nan_find_free_channel(cfg); if chan.is_null() { return -EINVAL; } ieee80211_nan_init_channel(chan, &mut (*sched).nan_channels[i]); if ieee80211_nan_use_chanctx(sdata, chan, false) != 0 { memset(chan as *mut _, 0, core::mem::size_of::<ieee80211_nan_channel>()); return -EINVAL; } }
    cfg.deferred = (*sched).deferred; drv_vif_cfg_changed((*sdata).local, sdata, BSS_CHANGED_NAN_LOCAL_SCHED); if !cfg.deferred { ieee80211_nan_update_all_ndi_carriers((*sdata).local); bitmap_zero((*sdata).u.nan.removed_channels.as_mut_ptr(), IEEE80211_NAN_MAX_CHANNELS); } 0
}

unsafe fn ieee80211_nan_update_ndi_carrier(ndi: *mut ieee80211_sub_if_data) {
    let local = (*ndi).local;
    lockdep_assert_wiphy((*local).hw.wiphy);
    if warn_on((*ndi).vif.type_ != NL80211_IFTYPE_NAN_DATA || (*ndi).dev.is_null()) || !ieee80211_sdata_running(ndi) { return; }
    let nmi = wiphy_dereference((*local).hw.wiphy, (*ndi).u.nan_data.nmi);
    if warn_on(nmi.is_null()) { return; }
    let mut sta = (*local).sta_list.next as *mut sta_info;
    while sta != (*local).sta_list as *mut _ {
        if (*sta).sdata == ndi && test_sta_flag(sta, WLAN_STA_AUTHORIZED) {
            let nmi_sta = wiphy_dereference((*local).hw.wiphy, (*sta).sta.nmi);
            if !warn_on(nmi_sta.is_null()) && !(*nmi_sta).nan_sched.is_null() && ieee80211_nan_has_common_slots(nmi, (*nmi_sta).nan_sched) { netif_carrier_on((*ndi).dev); return; }
        }
        sta = (*sta).list.next as *mut _;
    }
    netif_carrier_off((*ndi).dev);
}

unsafe fn ieee80211_nan_has_common_slots(sdata: *mut ieee80211_sub_if_data, peer: *mut ieee80211_nan_peer_sched) -> bool {
    for slot in 0..CFG80211_NAN_SCHED_NUM_TIME_SLOTS { let local = (*sdata).vif.cfg.nan_sched.schedule[slot]; if local.is_null() || (*local).chanctx_conf.is_null() { continue; } for m in 0..CFG80211_NAN_MAX_PEER_MAPS { let map = &(*peer).maps[m]; if map.map_id != CFG80211_NAN_INVALID_MAP_ID && !map.slots[slot].is_null() && (*local).chanctx_conf == (*map.slots[slot]).chanctx_conf { return true; } } }
    false
}

unsafe fn ieee80211_nan_update_peer_ndis_carrier(local: *mut ieee80211_local, nmi_sta: *mut sta_info) {
    let mut sta = (*local).sta_list.next as *mut sta_info;
    while sta != (*local).sta_list as *mut _ { if rcu_access_pointer((*sta).sta.nmi) == &mut (*nmi_sta).sta { ieee80211_nan_update_ndi_carrier((*sta).sdata); } sta = (*sta).list.next as *mut _; }
}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_nan_sched_update_done(vif: *mut ieee80211_vif) {
    let sdata = vif_to_sdata(vif); let cfg = &mut (*vif).cfg.nan_sched;
    lockdep_assert_wiphy((*sdata).local.hw.wiphy); if warn_on(!cfg.deferred) { return; }
    ieee80211_nan_update_all_ndi_carriers((*sdata).local); cfg.deferred = false;
    for i in 0..cfg.channels.len() { let ch = &mut cfg.channels[i]; let conf = ch.chanctx_conf; if ch.chanreq.oper.chan.is_null() { continue; } if test_bit(i, (*sdata).u.nan.removed_channels) { ieee80211_nan_remove_channel(sdata, ch); } else if !conf.is_null() { ieee80211_recalc_smps_chanctx((*sdata).local, container_of(conf, ieee80211_chanctx, conf)); } }
    bitmap_zero((*sdata).u.nan.removed_channels.as_mut_ptr(), IEEE80211_NAN_MAX_CHANNELS); cfg80211_nan_sched_update_done(ieee80211_vif_to_wdev(vif), true, GFP_KERNEL);
}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_nan_free_peer_sched(sched: *mut ieee80211_nan_peer_sched) { if !sched.is_null() { kfree((*sched).init_ulw); kfree(sched as *mut _); } }

unsafe fn ieee80211_nan_evacuate_channel(sdata: *mut ieee80211_sub_if_data, nan: *mut ieee80211_nan_channel) {
    lockdep_assert_wiphy((*sdata).local.hw.wiphy); if warn_on(nan.is_null() || (*nan).chanreq.oper.chan.is_null()) { return; }
    let conf = (*nan).chanctx_conf; if warn_on(conf.is_null()) { return; } (*nan).chanctx_conf = core::ptr::null_mut(); ieee80211_nan_update_peer_channels(sdata, conf); drv_vif_cfg_changed((*sdata).local, sdata, BSS_CHANGED_NAN_LOCAL_SCHED); cfg80211_nan_channel_evac(&mut (*sdata).wdev, &(*nan).chanreq.oper, GFP_KERNEL); ieee80211_nan_update_all_ndi_carriers((*sdata).local);
    let ctx = container_of(conf, ieee80211_chanctx, conf); if ieee80211_chanctx_num_assigned((*sdata).local, ctx) > 0 { ieee80211_recalc_chanctx_chantype((*sdata).local, ctx); ieee80211_recalc_smps_chanctx((*sdata).local, ctx); ieee80211_recalc_chanctx_min_def((*sdata).local, ctx); } if ieee80211_chanctx_refcount((*sdata).local, ctx) == 0 { ieee80211_free_chanctx((*sdata).local, ctx, false); }
}

unsafe fn ieee80211_nan_find_evac_chan(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, ctx: *mut ieee80211_chanctx) -> *mut ieee80211_nan_channel {
    lockdep_assert_wiphy((*local).hw.wiphy); if warn_on((*sdata).vif.type_ != NL80211_IFTYPE_NAN) { return core::ptr::null_mut(); } let cfg = &mut (*sdata).vif.cfg.nan_sched; let mut result = core::ptr::null_mut(); let mut min_slots = i32::MAX; let mut usable = 0;
    for i in 0..IEEE80211_NAN_MAX_CHANNELS { let ch = &mut cfg.channels[i]; if ch.chanreq.oper.chan.is_null() || ch.chanctx_conf.is_null() { continue; } usable += 1; let c = container_of(ch.chanctx_conf, ieee80211_chanctx, conf); if !ctx.is_null() { if c == ctx { result = ch; } continue; } if ieee80211_chanctx_refcount(local, c) > 1 { continue; } let mut count = 0; for slot in 0..CFG80211_NAN_SCHED_NUM_TIME_SLOTS { if cfg.schedule[slot] == ch { count += 1; } } if count < min_slots { min_slots = count; result = ch; } }
    if result.is_null() || usable < 2 { core::ptr::null_mut() } else { result }
}

#[no_mangle]
pub unsafe extern "C" fn ieee80211_nan_try_evacuate(hw: *mut ieee80211_hw, conf: *mut ieee80211_chanctx_conf) -> bool { let local = hw_to_local(hw); let sdata = ieee80211_find_nan_sdata(local); lockdep_assert_wiphy((*local).hw.wiphy); if sdata.is_null() { return false; } let ctx = if conf.is_null() { core::ptr::null_mut() } else { container_of(conf, ieee80211_chanctx, conf) }; let ch = ieee80211_nan_find_evac_chan(local, sdata, ctx); if ch.is_null() { return false; } ieee80211_nan_evacuate_channel(sdata, ch); true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
