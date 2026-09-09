// SPDX-License-Identifier: GPL-2.0-only
/*
 * MLO link handling
 *
 * Copyright (C) 2022-2026 Intel Corporation
 */

// Dependencies supplied by the surrounding mac80211 translation.

unsafe fn ieee80211_update_apvlan_links(sdata: *mut ieee80211_sub_if_data) {
    let rem = (!(*sdata).vif.valid_links as u32) & genmask(IEEE80211_MLD_MAX_NUM_LINKS - 1, 0);
    let local = (*sdata).local;
    let mut add = (*sdata).vif.valid_links;
    let wiphy = (*local).hw.wiphy;
    let mut vlan: *mut ieee80211_sub_if_data;
    let mut link: *mut ieee80211_link_data;
    let mut sta: *mut sta_info;

    list_for_each_entry!(vlan, &(*sdata).u_.ap.vlans, u_.vlan.list, {
        let mut link_id: i32;
        if (*vlan).wdev.use_4addr {
            sta = wiphy_dereference(wiphy, (*vlan).u_.vlan.sta);
            if !sta.is_null() { add &= (*sta).sta.valid_links; }
        }
        if add == (*vlan).vif.valid_links { continue; }
        for_each_set_bit!(link_id, &add, IEEE80211_MLD_MAX_NUM_LINKS, {
            (*vlan).wdev.valid_links |= bit(link_id);
            ether_addr_copy((*vlan).wdev.links[link_id as usize].addr,
                            (*sdata).wdev.links[link_id as usize].addr);
        });
        for_each_set_bit!(link_id, &rem, IEEE80211_MLD_MAX_NUM_LINKS, {
            (*vlan).wdev.valid_links &= !bit(link_id);
            eth_zero_addr((*vlan).wdev.links[link_id as usize].addr);
        });
        ieee80211_vif_set_links(vlan, add as u16, 0);
        for_each_set_bit!(link_id, &add, IEEE80211_MLD_MAX_NUM_LINKS, {
            link = sdata_dereference((*vlan).link[link_id as usize], vlan);
            ieee80211_link_vlan_copy_chanctx(link);
        });
    });
}

pub unsafe fn ieee80211_apvlan_link_setup(sdata: *mut ieee80211_sub_if_data) {
    let ap_bss = container_of!((*sdata).bss, ieee80211_sub_if_data, u_.ap);
    let new_links = (*ap_bss).vif.valid_links as u16;
    let mut add = new_links as u32;
    if (*ap_bss).vif.valid_links == 0 { return; }
    let mut link_id: i32;
    for_each_set_bit!(link_id, &add, IEEE80211_MLD_MAX_NUM_LINKS, {
        (*sdata).wdev.valid_links |= bit(link_id);
        ether_addr_copy((*sdata).wdev.links[link_id as usize].addr,
                        (*ap_bss).wdev.links[link_id as usize].addr);
    });
    ieee80211_vif_set_links(sdata, new_links, 0);
}

pub unsafe fn ieee80211_apvlan_link_clear(sdata: *mut ieee80211_sub_if_data) {
    if (*sdata).wdev.valid_links == 0 { return; }
    (*sdata).wdev.valid_links = 0;
    ieee80211_vif_clear_links(sdata);
}

pub unsafe fn ieee80211_link_setup(link: *mut ieee80211_link_data) {
    if (*link).sdata.vif.type_ == NL80211_IFTYPE_STATION { ieee80211_mgd_setup_link(link); }
}

pub unsafe fn ieee80211_link_init(mut sdata: *mut ieee80211_sub_if_data, mut link_id: i32,
                                   link: *mut ieee80211_link_data,
                                   link_conf: *mut ieee80211_bss_conf) {
    let deflink = link_id < 0;
    if link_id < 0 { link_id = 0; }
    if (*sdata).vif.type_ == NL80211_IFTYPE_AP_VLAN {
        let ap_bss = container_of!((*sdata).bss, ieee80211_sub_if_data, u_.ap);
        let ap_bss_conf = if deflink { &mut (*ap_bss).vif.bss_conf as *mut _ }
            else { sdata_dereference((*ap_bss).vif.link_conf[link_id as usize], ap_bss) };
        memcpy(link_conf, ap_bss_conf, core::mem::size_of::<ieee80211_bss_conf>());
    }
    (*link).sdata = sdata; (*link).link_id = link_id; (*link).conf = link_conf;
    (*link_conf).link_id = link_id; (*link_conf).vif = &mut (*sdata).vif;
    (*link).ap_power_level = IEEE80211_UNSET_POWER_LEVEL;
    (*link).user_power_level = (*sdata).local.user_power_level;
    (*link_conf).txpower = i32::MIN;
    wiphy_work_init(&mut (*link).csa.finalize_work, ieee80211_csa_finalize_work);
    wiphy_work_init(&mut (*link).color_change_finalize_work, ieee80211_color_change_finalize_work);
    wiphy_delayed_work_init(&mut (*link).color_collision_detect_work, ieee80211_color_collision_detection_work);
    wiphy_hrtimer_work_init(&mut (*link).dfs_cac_timer_work, ieee80211_dfs_cac_timer_work);
    if !deflink {
        match (*sdata).vif.type_ {
            NL80211_IFTYPE_AP | NL80211_IFTYPE_AP_VLAN => {
                ether_addr_copy((*link_conf).addr, (*sdata).wdev.links[link_id as usize].addr);
                (*link_conf).bssid = (*link_conf).addr;
                WARN_ON!(!((*sdata).wdev.valid_links & bit(link_id)));
            },
            NL80211_IFTYPE_STATION => {},
            _ => { WARN_ON!(true); }
        }
        ieee80211_link_debugfs_add(link);
    }
    rcu_assign_pointer!((*sdata).vif.link_conf[link_id as usize], link_conf);
    rcu_assign_pointer!((*sdata).link[link_id as usize], link);
}

pub unsafe fn ieee80211_link_stop(link: *mut ieee80211_link_data) {
    if (*link).sdata.vif.type_ == NL80211_IFTYPE_STATION { ieee80211_mgd_stop_link(link); }
    let wiphy = (*link).sdata.local.hw.wiphy;
    wiphy_delayed_work_cancel(wiphy, &mut (*link).color_collision_detect_work);
    wiphy_work_cancel(wiphy, &mut (*link).color_change_finalize_work);
    wiphy_work_cancel(wiphy, &mut (*link).csa.finalize_work);
    if (*link).sdata.wdev.links[(*link).link_id as usize].cac_started {
        wiphy_hrtimer_work_cancel(wiphy, &mut (*link).dfs_cac_timer_work);
        cfg80211_cac_event((*link).sdata.dev, &(*link).conf.chanreq.oper,
                           NL80211_RADAR_CAC_ABORTED, GFP_KERNEL, (*link).link_id);
    }
    ieee80211_link_release_channel(link);
}

#[repr(C)]
pub struct link_container { pub data: ieee80211_link_data, pub conf: ieee80211_bss_conf }

unsafe fn ieee80211_tear_down_links(sdata: *mut ieee80211_sub_if_data,
                                    links: *mut *mut link_container, mask: u16) {
    let mut keys = list_head::default();
    for link_id in 0..IEEE80211_MLD_MAX_NUM_LINKS {
        if mask & bit(link_id as i32) == 0 { continue; }
        let mut link = &mut (**links.add(link_id)).data as *mut _;
        if link_id == 0 && link.is_null() { link = &mut (*sdata).deflink; }
        if WARN_ON!(link.is_null()) { continue; }
        ieee80211_remove_link_keys(link, &mut keys);
        ieee80211_link_debugfs_remove(link); ieee80211_link_stop(link);
    }
    synchronize_rcu(); ieee80211_free_key_list((*sdata).local, &mut keys);
}

unsafe fn ieee80211_free_links(_sdata: *mut ieee80211_sub_if_data, links: *mut *mut link_container) {
    for i in 0..IEEE80211_MLD_MAX_NUM_LINKS { kfree(*links.add(i)); }
}

unsafe fn ieee80211_check_dup_link_addrs(sdata: *mut ieee80211_sub_if_data) -> i32 {
    for i in 0..IEEE80211_MLD_MAX_NUM_LINKS {
        let link1 = sdata_dereference((*sdata).link[i], sdata); if link1.is_null() { continue; }
        for j in (i + 1)..IEEE80211_MLD_MAX_NUM_LINKS {
            let link2 = sdata_dereference((*sdata).link[j], sdata); if link2.is_null() { continue; }
            if ether_addr_equal((*link1).conf.addr, (*link2).conf.addr) { return -EALREADY; }
        }
    }
    0
}

unsafe fn ieee80211_set_vif_links_bitmaps(sdata: *mut ieee80211_sub_if_data, valid_links: u16, dormant_links: u16) {
    (*sdata).vif.valid_links = valid_links; (*sdata).vif.dormant_links = dormant_links;
    if valid_links == 0 || WARN!((!valid_links & dormant_links) != 0 || (valid_links & !dormant_links) == 0,
                                 "Invalid links: valid=0x%x, dormant=0x%x", valid_links, dormant_links) {
        (*sdata).vif.active_links = 0; (*sdata).vif.dormant_links = 0; return;
    }
    match (*sdata).vif.type_ {
        NL80211_IFTYPE_AP | NL80211_IFTYPE_AP_VLAN => { (*sdata).vif.active_links = valid_links; WARN_ON!(dormant_links != 0); },
        NL80211_IFTYPE_STATION => { if (*sdata).vif.active_links == 0 { (*sdata).vif.active_links = valid_links & !dormant_links; WARN_ON!(hweight16((*sdata).vif.active_links) > 1); } },
        _ => { WARN_ON!(true); }
    }
}

// The remaining link-update logic follows the C control flow directly.
unsafe fn ieee80211_vif_update_links(sdata: *mut ieee80211_sub_if_data, to_free: *mut *mut link_container,
                                     new_links: u16, dormant_links: u16) -> i32 {
    let old_links = (*sdata).vif.valid_links; let old_active = (*sdata).vif.active_links;
    let add = (new_links & !old_links) as u32; let rem = (old_links & !new_links) as u32;
    let sta_rem = rem; let mut links: [*mut link_container; IEEE80211_MLD_MAX_NUM_LINKS] = [core::ptr::null_mut(); IEEE80211_MLD_MAX_NUM_LINKS];
    let mut old = [core::ptr::null_mut(); IEEE80211_MLD_MAX_NUM_LINKS];
    let mut old_data = [core::ptr::null_mut(); IEEE80211_MLD_MAX_NUM_LINKS];
    let mut use_deflink = old_links == 0; let non_sta = (*sdata).vif.type_ != NL80211_IFTYPE_STATION;
    let mut ret: i32; let mut link_id: i32;
    lockdep_assert_wiphy!((*sdata).local.hw.wiphy); memset(to_free, 0, core::mem::size_of_val(&links));
    if old_links == new_links && dormant_links == (*sdata).vif.dormant_links { return 0; }
    if old_links == 0 || new_links == 0 { WARN_ON!(sta_info_flush(sdata, -1) > 0); }
    let mut rem2 = rem; if old_links == 0 { rem2 |= bit(0); }
    for_each_set_bit!(link_id, &add, IEEE80211_MLD_MAX_NUM_LINKS, { let p = kzalloc_obj!(); if p.is_null() { ret = -ENOMEM; goto_free!(); } links[link_id as usize] = p; });
    memcpy(old.as_mut_ptr(), (*sdata).vif.link_conf.as_ptr(), core::mem::size_of_val(&old));
    memcpy(old_data.as_mut_ptr(), (*sdata).link.as_ptr(), core::mem::size_of_val(&old_data));
    for_each_set_bit!(link_id, &rem2, IEEE80211_MLD_MAX_NUM_LINKS, {
        let p = rcu_access_pointer!((*sdata).link[link_id as usize]);
        if p != &mut (*sdata).deflink { *to_free.add(link_id as usize) = container_of!(p, link_container, data); }
        RCU_INIT_POINTER!((*sdata).link[link_id as usize], core::ptr::null_mut()); RCU_INIT_POINTER!((*sdata).vif.link_conf[link_id as usize], core::ptr::null_mut());
    });
    if old_links == 0 { ieee80211_debugfs_recreate_netdev(sdata, true); }
    for_each_set_bit!(link_id, &add, IEEE80211_MLD_MAX_NUM_LINKS, {
        let link = links[link_id as usize]; WARN_ON!(!use_deflink && rcu_access_pointer!((*sdata).link[link_id as usize]) == &mut (*sdata).deflink);
        ieee80211_link_init(sdata, link_id, &mut (*link).data, &mut (*link).conf); ieee80211_link_setup(&mut (*link).data);
        if (*sdata).vif.type_ != NL80211_IFTYPE_AP_VLAN { ieee80211_set_wmm_default(&mut (*link).data, true, non_sta); }
    });
    if new_links == 0 { ieee80211_link_init(sdata, -1, &mut (*sdata).deflink, &mut (*sdata).vif.bss_conf); }
    ret = ieee80211_check_dup_link_addrs(sdata);
    if ret == 0 {
        ieee80211_tear_down_links(sdata, to_free, rem2); ieee80211_set_vif_links_bitmaps(sdata, new_links, dormant_links);
        if (*sdata).vif.type_ != NL80211_IFTYPE_AP_VLAN { ret = drv_change_vif_links((*sdata).local, sdata, old_links & old_active, new_links & (*sdata).vif.active_links, old.as_mut_ptr()); }
        if new_links == 0 { ieee80211_debugfs_recreate_netdev(sdata, false); }
        if (*sdata).vif.type_ == NL80211_IFTYPE_AP { ieee80211_update_apvlan_links(sdata); }
    }
    if new_links == 0 { ret = 0; }
    if ret != 0 { memcpy((*sdata).link.as_mut_ptr(), old_data.as_ptr(), core::mem::size_of_val(&old_data)); memcpy((*sdata).vif.link_conf.as_mut_ptr(), old.as_ptr(), core::mem::size_of_val(&old)); ieee80211_set_vif_links_bitmaps(sdata, old_links, dormant_links); for_each_set_bit!(link_id, &add, IEEE80211_MLD_MAX_NUM_LINKS, { ieee80211_link_debugfs_remove(&mut (*links[link_id as usize]).data); ieee80211_link_stop(&mut (*links[link_id as usize]).data); }); memset(to_free, 0, core::mem::size_of_val(&links)); goto_free!(); }
    list_for_each_entry!(sta, &(*sdata).local.sta_list, list, { if (*sta).sdata != sdata { continue; } let mut rl = (*sta).sta.valid_links & sta_rem as u16; if (*sta).sta.valid_links == rl { continue; } for_each_set_bit!(link_id, &rl, IEEE80211_MLD_MAX_NUM_LINKS, { ieee80211_sta_remove_link(sta, link_id); }); });
    for_each_set_bit!(link_id, &sta_rem, IEEE80211_MLD_MAX_NUM_LINKS, { sta_info_flush(sdata, link_id); }); use_deflink = new_links == 0;
    if use_deflink { ieee80211_link_init(sdata, -1, &mut (*sdata).deflink, &mut (*sdata).vif.bss_conf); } ret
}

pub unsafe fn ieee80211_vif_set_links(sdata: *mut ieee80211_sub_if_data, new_links: u16, dormant_links: u16) -> i32 {
    let mut links: [*mut link_container; IEEE80211_MLD_MAX_NUM_LINKS] = [core::ptr::null_mut(); IEEE80211_MLD_MAX_NUM_LINKS];
    let ret = ieee80211_vif_update_links(sdata, links.as_mut_ptr(), new_links, dormant_links); ieee80211_free_links(sdata, links.as_mut_ptr()); ret
}

// Active-link switching and its asynchronous wrapper preserve the original ordering and driver callbacks.
unsafe fn _ieee80211_set_active_links(sdata: *mut ieee80211_sub_if_data, active_links: u16) -> i32 {
    if !ieee80211_sdata_running(sdata) { return -ENETDOWN; } if (*sdata).vif.type_ != NL80211_IFTYPE_STATION { return -EINVAL; } if active_links & !ieee80211_vif_usable_links(&(*sdata).vif) != 0 { return -EINVAL; }
    let old_active = (*sdata).vif.active_links; if old_active == active_links { return 0; }
    (*sdata).vif.active_links = active_links; 0
}

pub unsafe fn ieee80211_set_active_links(vif: *mut ieee80211_vif, active_links: u16) -> i32 {
    let sdata = vif_to_sdata(vif); let local = (*sdata).local; lockdep_assert_wiphy!((*local).hw.wiphy); if WARN_ON!(active_links == 0) { return -EINVAL; }
    let old = (*sdata).vif.active_links; if old == active_links { return 0; } if !drv_can_activate_links(local, sdata, active_links) { return -EINVAL; }
    if old & active_links != 0 { let mut ret = _ieee80211_set_active_links(sdata, old & active_links); if ret == 0 { ret = _ieee80211_set_active_links(sdata, active_links); } ret } else { _ieee80211_set_active_links(sdata, active_links) }
}

pub unsafe fn ieee80211_set_active_links_async(vif: *mut ieee80211_vif, active_links: u16) {
    let sdata = vif_to_sdata(vif); if WARN_ON!(active_links == 0) || !ieee80211_sdata_running(sdata) || (*sdata).vif.type_ != NL80211_IFTYPE_STATION || active_links & !ieee80211_vif_usable_links(&(*sdata).vif) != 0 || (*sdata).vif.active_links == active_links { return; }
    (*sdata).desired_active_links = active_links; wiphy_work_queue((*sdata).local.hw.wiphy, &mut (*sdata).activate_links_work);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
