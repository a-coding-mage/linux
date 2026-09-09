// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015 Intel Deutschland GmbH
 * Copyright (C) 2022-2025 Intel Corporation
 */

pub unsafe fn drv_start(local: *mut ieee80211_local) -> i32 {
    let mut ret: i32;
    might_sleep();
    lockdep_assert_wiphy((*local).hw.wiphy);
    if WARN_ON((*local).started) { return -EALREADY; }
    trace_drv_start(local);
    (*local).started = true;
    // allow rx frames
    smp_mb();
    ret = ((*local).ops).start(&mut (*local).hw);
    trace_drv_return_int(local, ret);
    if ret != 0 { (*local).started = false; }
    ret
}

pub unsafe fn drv_stop(local: *mut ieee80211_local, suspend: bool) {
    might_sleep();
    lockdep_assert_wiphy((*local).hw.wiphy);
    if WARN_ON(!(*local).started) { return; }
    trace_drv_stop(local, suspend);
    ((*local).ops).stop(&mut (*local).hw, suspend);
    trace_drv_return_void(local);
    // sync away all work on the tasklet before clearing started
    tasklet_disable(&mut (*local).tasklet);
    tasklet_enable(&mut (*local).tasklet);
    barrier();
    (*local).started = false;
}

pub unsafe fn drv_add_interface(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) -> i32 {
    let ret: i32;
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if WARN_ON((*sdata).vif.type_ == NL80211_IFTYPE_AP_VLAN ||
        ((*sdata).vif.type_ == NL80211_IFTYPE_MONITOR &&
         !ieee80211_hw_check(&(*local).hw, WANT_MONITOR_VIF) &&
         !ieee80211_hw_check(&(*local).hw, NO_VIRTUAL_MONITOR) &&
         !((*sdata).u.mntr.flags & MONITOR_FLAG_ACTIVE) != 0)) { return -EINVAL; }
    trace_drv_add_interface(local, sdata);
    ret = ((*local).ops).add_interface(&mut (*local).hw, &mut (*sdata).vif);
    trace_drv_return_int(local, ret);
    if ret != 0 { return ret; }
    if (*sdata).flags & IEEE80211_SDATA_IN_DRIVER == 0 {
        (*sdata).flags |= IEEE80211_SDATA_IN_DRIVER;
        drv_vif_add_debugfs(local, sdata);
        // initially vif is not MLD
        ieee80211_link_debugfs_drv_add(&mut (*sdata).deflink);
    }
    0
}

pub unsafe fn drv_change_interface(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, type_: nl80211_iftype, p2p: bool) -> i32 {
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if !check_sdata_in_driver(sdata) { return -EIO; }
    trace_drv_change_interface(local, sdata, type_, p2p);
    let ret = ((*local).ops).change_interface(&mut (*local).hw, &mut (*sdata).vif, type_, p2p);
    trace_drv_return_int(local, ret); ret
}

pub unsafe fn drv_remove_interface(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) {
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if !check_sdata_in_driver(sdata) { return; }
    (*sdata).flags &= !IEEE80211_SDATA_IN_DRIVER;
    /* Remove driver debugfs entries. The virtual monitor interface doesn't
     * get a debugfs entry, so it's exempt here. */
    if sdata != rcu_access_pointer((*local).monitor_sdata) {
        ieee80211_debugfs_recreate_netdev(sdata, (*sdata).vif.valid_links);
    }
    trace_drv_remove_interface(local, sdata);
    ((*local).ops).remove_interface(&mut (*local).hw, &mut (*sdata).vif);
    trace_drv_return_void(local);
}

pub unsafe fn drv_sta_state(local: *mut ieee80211_local, mut sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info, old_state: ieee80211_sta_state, new_state: ieee80211_sta_state) -> i32 {
    let mut ret = 0;
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    sdata = get_bss_sdata(sdata);
    if !check_sdata_in_driver(sdata) { return -EIO; }
    trace_drv_sta_state(local, sdata, &mut (*sta).sta, old_state, new_state);
    if ((*local).ops).sta_state.is_some() {
        ret = ((*local).ops).sta_state.unwrap()(&mut (*local).hw, &mut (*sdata).vif, &mut (*sta).sta, old_state, new_state);
    } else if old_state == IEEE80211_STA_AUTH && new_state == IEEE80211_STA_ASSOC {
        ret = drv_sta_add(local, sdata, &mut (*sta).sta);
        if ret == 0 { (*sta).uploaded = true; if rcu_access_pointer((*sta).sta.rates).is_some() { drv_sta_rate_tbl_update(local, sdata, &mut (*sta).sta); } }
    } else if old_state == IEEE80211_STA_ASSOC && new_state == IEEE80211_STA_AUTH { drv_sta_remove(local, sdata, &mut (*sta).sta); }
    trace_drv_return_int(local, ret); ret
}

pub unsafe fn drv_sta_set_txpwr(local: *mut ieee80211_local, mut sdata: *mut ieee80211_sub_if_data, sta: *mut sta_info) -> i32 {
    let mut ret = -EOPNOTSUPP;
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    sdata = get_bss_sdata(sdata); if !check_sdata_in_driver(sdata) { return -EIO; }
    trace_drv_sta_set_txpwr(local, sdata, &mut (*sta).sta);
    if ((*local).ops).sta_set_txpwr.is_some() { ret = ((*local).ops).sta_set_txpwr.unwrap()(&mut (*local).hw, &mut (*sdata).vif, &mut (*sta).sta); }
    trace_drv_return_int(local, ret); ret
}

pub unsafe fn drv_link_sta_rc_update(local: *mut ieee80211_local, mut sdata: *mut ieee80211_sub_if_data, link_sta: *mut ieee80211_link_sta, changed: u32) {
    sdata = get_bss_sdata(sdata); if !check_sdata_in_driver(sdata) { return; }
    WARN_ON((changed & IEEE80211_RC_SUPP_RATES_CHANGED) != 0 && (*sdata).vif.type_ != NL80211_IFTYPE_ADHOC && (*sdata).vif.type_ != NL80211_IFTYPE_MESH_POINT);
    trace_drv_link_sta_rc_update(local, sdata, link_sta, changed);
    if ((*local).ops).link_sta_rc_update.is_some() { ((*local).ops).link_sta_rc_update.unwrap()(&mut (*local).hw, &mut (*sdata).vif, link_sta, changed); }
    trace_drv_return_void(local);
}

pub unsafe fn drv_conf_tx(local: *mut ieee80211_local, link: *mut ieee80211_link_data, ac: u16, params: *const ieee80211_tx_queue_params) -> i32 {
    let sdata = (*link).sdata; let mut ret = -EOPNOTSUPP;
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if !check_sdata_in_driver(sdata) { return -EIO; }
    if !ieee80211_vif_link_active(&(*sdata).vif, (*link).link_id) { return 0; }
    if (*params).cw_min == 0 || (*params).cw_min > (*params).cw_max { WARN_ONCE(((*local).ops).conf_tx.is_some(), "%s: invalid CW_min/CW_max: %d/%d\n", (*sdata).name, (*params).cw_min, (*params).cw_max); return -EINVAL; }
    trace_drv_conf_tx(local, sdata, (*link).link_id, ac, params);
    if ((*local).ops).conf_tx.is_some() { ret = ((*local).ops).conf_tx.unwrap()(&mut (*local).hw, &mut (*sdata).vif, (*link).link_id, ac, params); }
    trace_drv_return_int(local, ret); ret
}

pub unsafe fn drv_get_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) -> u64 {
    let mut ret = u64::MAX; might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if !check_sdata_in_driver(sdata) { return ret; }
    trace_drv_get_tsf(local, sdata); if ((*local).ops).get_tsf.is_some() { ret = ((*local).ops).get_tsf.unwrap()(&mut (*local).hw, &mut (*sdata).vif); }
    trace_drv_return_u64(local, ret); ret
}

pub unsafe fn drv_set_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, tsf: u64) { might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); if !check_sdata_in_driver(sdata) { return; } trace_drv_set_tsf(local, sdata, tsf); if ((*local).ops).set_tsf.is_some() { ((*local).ops).set_tsf.unwrap()(&mut (*local).hw, &mut (*sdata).vif, tsf); } trace_drv_return_void(local); }
pub unsafe fn drv_offset_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, offset: i64) { might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); if !check_sdata_in_driver(sdata) { return; } trace_drv_offset_tsf(local, sdata, offset); if ((*local).ops).offset_tsf.is_some() { ((*local).ops).offset_tsf.unwrap()(&mut (*local).hw, &mut (*sdata).vif, offset); } trace_drv_return_void(local); }
pub unsafe fn drv_reset_tsf(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data) { might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); if !check_sdata_in_driver(sdata) { return; } trace_drv_reset_tsf(local, sdata); if ((*local).ops).reset_tsf.is_some() { ((*local).ops).reset_tsf.unwrap()(&mut (*local).hw, &mut (*sdata).vif); } trace_drv_return_void(local); }

pub unsafe fn drv_assign_vif_chanctx(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, link_conf: *mut ieee80211_bss_conf, ctx: *mut ieee80211_chanctx) -> i32 {
    let mut ret = 0; might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if (*sdata).vif.type_ == NL80211_IFTYPE_MONITOR && (*local).emulate_chanctx && !ieee80211_hw_check(&(*local).hw, WANT_MONITOR_VIF) { return 0; }
    if !check_sdata_in_driver(sdata) { return -EIO; } if !ieee80211_vif_link_active(&(*sdata).vif, (*link_conf).link_id) { return 0; }
    trace_drv_assign_vif_chanctx(local, sdata, link_conf, ctx); if ((*local).ops).assign_vif_chanctx.is_some() { WARN_ON_ONCE(!(*ctx).driver_present); ret = ((*local).ops).assign_vif_chanctx.unwrap()(&mut (*local).hw, &mut (*sdata).vif, link_conf, &mut (*ctx).conf); } trace_drv_return_int(local, ret); ret
}

pub unsafe fn drv_unassign_vif_chanctx(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, link_conf: *mut ieee80211_bss_conf, ctx: *mut ieee80211_chanctx) {
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); if (*sdata).vif.type_ == NL80211_IFTYPE_MONITOR && (*local).emulate_chanctx && !ieee80211_hw_check(&(*local).hw, WANT_MONITOR_VIF) { return; } if !check_sdata_in_driver(sdata) || !ieee80211_vif_link_active(&(*sdata).vif, (*link_conf).link_id) { return; }
    trace_drv_unassign_vif_chanctx(local, sdata, link_conf, ctx); if ((*local).ops).unassign_vif_chanctx.is_some() { WARN_ON_ONCE(!(*ctx).driver_present); ((*local).ops).unassign_vif_chanctx.unwrap()(&mut (*local).hw, &mut (*sdata).vif, link_conf, &mut (*ctx).conf); } trace_drv_return_void(local);
}

pub unsafe fn drv_switch_vif_chanctx(local: *mut ieee80211_local, vifs: *mut ieee80211_vif_chanctx_switch, n_vifs: i32, mode: ieee80211_chanctx_switch_mode) -> i32 {
    if ((*local).ops).switch_vif_chanctx.is_none() { return -EOPNOTSUPP; }
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    for i in 0..n_vifs { let v = vifs.add(i as usize); let new_ctx = container_of((*v).new_ctx, ieee80211_chanctx, conf); let old_ctx = container_of((*v).old_ctx, ieee80211_chanctx, conf); WARN_ON_ONCE(!(*old_ctx).driver_present); WARN_ON_ONCE((mode == CHANCTX_SWMODE_SWAP_CONTEXTS && (*new_ctx).driver_present) || (mode == CHANCTX_SWMODE_REASSIGN_VIF && !(*new_ctx).driver_present)); }
    trace_drv_switch_vif_chanctx(local, vifs, n_vifs, mode); let ret = ((*local).ops).switch_vif_chanctx.unwrap()(&mut (*local).hw, vifs, n_vifs, mode); trace_drv_return_int(local, ret);
    if ret == 0 && mode == CHANCTX_SWMODE_SWAP_CONTEXTS { for i in 0..n_vifs { let v = vifs.add(i as usize); let new_ctx = container_of((*v).new_ctx, ieee80211_chanctx, conf); let old_ctx = container_of((*v).old_ctx, ieee80211_chanctx, conf); (*new_ctx).driver_present = true; (*old_ctx).driver_present = false; } } ret
}

pub unsafe fn drv_ampdu_action(local: *mut ieee80211_local, mut sdata: *mut ieee80211_sub_if_data, params: *mut ieee80211_ampdu_params) -> i32 { let mut ret = -EOPNOTSUPP; might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); sdata = get_bss_sdata(sdata); if !check_sdata_in_driver(sdata) { return -EIO; } trace_drv_ampdu_action(local, sdata, params); if ((*local).ops).ampdu_action.is_some() { ret = ((*local).ops).ampdu_action.unwrap()(&mut (*local).hw, &mut (*sdata).vif, params); } trace_drv_return_int(local, ret); ret }

pub unsafe fn drv_link_info_changed(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, info: *mut ieee80211_bss_conf, link_id: i32, changed: u64) {
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy);
    if WARN_ON_ONCE((changed & (BSS_CHANGED_BEACON | BSS_CHANGED_BEACON_ENABLED)) != 0 && (*sdata).vif.type_ != NL80211_IFTYPE_AP && (*sdata).vif.type_ != NL80211_IFTYPE_ADHOC && (*sdata).vif.type_ != NL80211_IFTYPE_MESH_POINT && (*sdata).vif.type_ != NL80211_IFTYPE_OCB) { return; }
    if WARN_ON_ONCE((*sdata).vif.type_ == NL80211_IFTYPE_P2P_DEVICE || (*sdata).vif.type_ == NL80211_IFTYPE_NAN || ((*sdata).vif.type_ == NL80211_IFTYPE_MONITOR && (changed & !(BSS_CHANGED_TXPOWER | BSS_CHANGED_MU_GROUPS)) != 0)) { return; }
    if WARN_ON_ONCE((changed & BSS_CHANGED_MU_GROUPS) != 0 && !(*sdata).vif.bss_conf.mu_mimo_owner) { return; }
    if !check_sdata_in_driver(sdata) || !ieee80211_vif_link_active(&(*sdata).vif, link_id) { return; }
    trace_drv_link_info_changed(local, sdata, info, changed); if ((*local).ops).link_info_changed.is_some() { ((*local).ops).link_info_changed.unwrap()(&mut (*local).hw, &mut (*sdata).vif, info, changed); } else if ((*local).ops).bss_info_changed.is_some() { ((*local).ops).bss_info_changed.unwrap()(&mut (*local).hw, &mut (*sdata).vif, info, changed); } trace_drv_return_void(local);
}

pub unsafe fn drv_set_key(local: *mut ieee80211_local, cmd: set_key_cmd, mut sdata: *mut ieee80211_sub_if_data, sta: *mut ieee80211_sta, key: *mut ieee80211_key_conf) -> i32 {
    might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); sdata = get_bss_sdata(sdata); if !check_sdata_in_driver(sdata) { return -EIO; }
    if WARN_ON((*key).link_id >= 0 && (*sdata).vif.active_links != 0 && ((*sdata).vif.active_links & BIT((*key).link_id)) == 0) { return -ENOLINK; }
    if fips_enabled { return -EOPNOTSUPP; } trace_drv_set_key(local, cmd, sdata, sta, key); let ret = ((*local).ops).set_key(&mut (*local).hw, cmd, &mut (*sdata).vif, sta, key); trace_drv_return_int(local, ret); ret
}

pub unsafe fn drv_change_vif_links(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, old_links: u16, new_links: u16, old: *mut *mut ieee80211_bss_conf) -> i32 {
    let mut ret = -EOPNOTSUPP; might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); if !check_sdata_in_driver(sdata) { return -EIO; } if old_links == new_links { return 0; }
    let links_to_add = !old_links & new_links; let links_to_rem = old_links & !new_links;
    for link_id in 0..IEEE80211_MLD_MAX_NUM_LINKS { if (links_to_rem & (1 << link_id)) != 0 { let link = rcu_access_pointer((*sdata).link[link_id as usize]); ieee80211_link_debugfs_drv_remove(link); } }
    trace_drv_change_vif_links(local, sdata, old_links, new_links); if ((*local).ops).change_vif_links.is_some() { ret = ((*local).ops).change_vif_links.unwrap()(&mut (*local).hw, &mut (*sdata).vif, old_links, new_links, old); } trace_drv_return_int(local, ret); if ret != 0 { return ret; }
    if !(*local).in_reconfig && !(*local).resuming { for link_id in 0..IEEE80211_MLD_MAX_NUM_LINKS { if (links_to_add & (1 << link_id)) != 0 { let link = rcu_access_pointer((*sdata).link[link_id as usize]); ieee80211_link_debugfs_drv_add(link); } } } 0
}

pub unsafe fn drv_change_sta_links(local: *mut ieee80211_local, sdata: *mut ieee80211_sub_if_data, sta: *mut ieee80211_sta, mut old_links: u16, mut new_links: u16) -> i32 {
    let info = container_of(sta, sta_info, sta); let mut ret = -EOPNOTSUPP; might_sleep(); lockdep_assert_wiphy((*local).hw.wiphy); if !check_sdata_in_driver(sdata) { return -EIO; }
    old_links &= (*sdata).vif.active_links; new_links &= (*sdata).vif.active_links; if old_links == new_links { return 0; }
    let links_to_add = !old_links & new_links; let links_to_rem = old_links & !new_links;
    for link_id in 0..IEEE80211_MLD_MAX_NUM_LINKS { if (links_to_rem & (1 << link_id)) != 0 { let link_sta = rcu_dereference_protected((*info).link[link_id as usize], lockdep_is_held(&(*(*local).hw.wiphy).mtx)); ieee80211_link_sta_debugfs_drv_remove(link_sta); } }
    trace_drv_change_sta_links(local, sdata, sta, old_links, new_links); if ((*local).ops).change_sta_links.is_some() { ret = ((*local).ops).change_sta_links.unwrap()(&mut (*local).hw, &mut (*sdata).vif, sta, old_links, new_links); } trace_drv_return_int(local, ret); if ret != 0 { return ret; }
    // during reconfig don't add it to debugfs again
    if (*local).in_reconfig || (*local).resuming { return 0; }
    for link_id in 0..IEEE80211_MLD_MAX_NUM_LINKS { if (links_to_add & (1 << link_id)) != 0 { let link_sta = rcu_dereference_protected((*info).link[link_id as usize], lockdep_is_held(&(*(*local).hw.wiphy).mtx)); ieee80211_link_sta_debugfs_drv_add(link_sta); } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
