// SPDX-License-Identifier: GPL-2.0-only
/*
 * OCB mode implementation
 *
 * Copyright: (c) 2014 Czech Technical University in Prague
 *            (c) 2014 Volkswagen Group Research
 * Copyright (C) 2022 - 2024, 2026 Intel Corporation
 * Author:    Rostislav Lisovy <rostislav.lisovy@fel.cvut.cz>
 * Funded by: Volkswagen Group Research
 */

// Dependencies are supplied by the surrounding mac80211 translation unit.

const IEEE80211_OCB_HOUSEKEEPING_INTERVAL: u64 = 60 * HZ;
const IEEE80211_OCB_PEER_INACTIVITY_LIMIT: u64 = 240 * HZ;
const IEEE80211_OCB_MAX_STA_ENTRIES: i32 = 128;

/**
 * enum ocb_deferred_task_flags - mac80211 OCB deferred tasks
 * @OCB_WORK_HOUSEKEEPING: run the periodic OCB housekeeping tasks
 *
 * These flags are used in @wrkq_flags field of &struct ieee80211_if_ocb
 */
#[repr(C)]
enum OcbDeferredTaskFlags {
    OCB_WORK_HOUSEKEEPING,
}

pub unsafe fn ieee80211_ocb_rx_no_sta(
    sdata: *mut ieee80211_sub_if_data,
    bssid: *const u8,
    addr: *const u8,
    supp_rates: u32,
) {
    let ifocb = &mut (*sdata).u.ocb;
    let local = (*sdata).local;
    let mut chanctx_conf: *mut ieee80211_chanctx_conf;
    let mut sband: *mut ieee80211_supported_band;
    let mut sta: *mut sta_info;
    let band: i32;

    if !ifocb.joined {
        return;
    }

    /* XXX: Consider removing the least recently used entry and
     *      allow new one to be added.
     */
    if (*local).num_sta >= IEEE80211_OCB_MAX_STA_ENTRIES {
        net_info_ratelimited!("%s: No room for a new OCB STA entry %pM\n", (*sdata).name, addr);
        return;
    }

    ocb_dbg!(sdata, "Adding new OCB station %pM\n", addr);

    rcu_read_lock();
    chanctx_conf = rcu_dereference!((*sdata).vif.bss_conf.chanctx_conf);
    if WARN_ON_ONCE!(chanctx_conf.is_null()) {
        rcu_read_unlock();
        return;
    }
    band = (*(*chanctx_conf).def.chan).band;
    rcu_read_unlock();

    sta = sta_info_alloc(sdata, addr, GFP_ATOMIC);
    if sta.is_null() {
        return;
    }

    /* Add only mandatory rates for now */
    sband = (*(*local).hw.wiphy).bands[band as usize];
    (*sta).sta.deflink.supp_rates[band as usize] = ieee80211_mandatory_rates(sband);

    spin_lock(&mut ifocb.incomplete_lock);
    list_add(&mut (*sta).list, &mut ifocb.incomplete_stations);
    spin_unlock(&mut ifocb.incomplete_lock);
    wiphy_work_queue((*(*local).hw).wiphy, &mut (*sdata).work);
}

unsafe fn ieee80211_ocb_finish_sta(sta: *mut sta_info) -> *mut sta_info {
    let sdata = (*sta).sdata;
    let mut addr = [0u8; ETH_ALEN];

    memcpy(addr.as_mut_ptr() as *mut c_void, (*sta).sta.addr.as_ptr() as *const c_void, ETH_ALEN);

    ieee80211_sta_init_nss_bw_capa(
        &mut (*sta).deflink,
        &(*(*sdata).deflink.conf).chanreq.oper,
    );

    ocb_dbg!(sdata, "Adding new IBSS station %pM (dev=%s)\n", addr.as_ptr(), (*sdata).name);

    sta_info_move_state(sta, IEEE80211_STA_AUTH);
    sta_info_move_state(sta, IEEE80211_STA_ASSOC);
    sta_info_move_state(sta, IEEE80211_STA_AUTHORIZED);

    rate_control_rate_init(&mut (*sta).deflink);

    /* If it fails, maybe we raced another insertion? */
    if sta_info_insert_rcu(sta) != 0 {
        return sta_info_get(sdata, addr.as_ptr());
    }
    sta
}

unsafe fn ieee80211_ocb_housekeeping(sdata: *mut ieee80211_sub_if_data) {
    let ifocb = &mut (*sdata).u.ocb;

    ocb_dbg!(sdata, "Running ocb housekeeping\n");

    ieee80211_sta_expire(sdata, IEEE80211_OCB_PEER_INACTIVITY_LIMIT);

    mod_timer(
        &mut ifocb.housekeeping_timer,
        round_jiffies(jiffies + IEEE80211_OCB_HOUSEKEEPING_INTERVAL),
    );
}

pub unsafe fn ieee80211_ocb_work(sdata: *mut ieee80211_sub_if_data) {
    let ifocb = &mut (*sdata).u.ocb;
    let mut sta: *mut sta_info;

    lockdep_assert_wiphy!((*(*sdata).local).hw.wiphy);

    if ifocb.joined != true {
        return;
    }

    spin_lock_bh(&mut ifocb.incomplete_lock);
    while !list_empty(&ifocb.incomplete_stations) {
        sta = list_first_entry!(&mut ifocb.incomplete_stations, sta_info, list);
        list_del(&mut (*sta).list);
        spin_unlock_bh(&mut ifocb.incomplete_lock);

        ieee80211_ocb_finish_sta(sta);
        rcu_read_unlock();
        spin_lock_bh(&mut ifocb.incomplete_lock);
    }
    spin_unlock_bh(&mut ifocb.incomplete_lock);

    if test_and_clear_bit!(OCB_WORK_HOUSEKEEPING, &mut ifocb.wrkq_flags) {
        ieee80211_ocb_housekeeping(sdata);
    }
}

unsafe fn ieee80211_ocb_housekeeping_timer(t: *mut timer_list) {
    let sdata = timer_container_of!(sdata, t, u.ocb.housekeeping_timer);
    let local = (*sdata).local;
    let ifocb = &mut (*sdata).u.ocb;

    set_bit!(OCB_WORK_HOUSEKEEPING, &mut ifocb.wrkq_flags);

    wiphy_work_queue((*(*local).hw).wiphy, &mut (*sdata).work);
}

pub unsafe fn ieee80211_ocb_setup_sdata(sdata: *mut ieee80211_sub_if_data) {
    let ifocb = &mut (*sdata).u.ocb;

    timer_setup!(&mut ifocb.housekeeping_timer, ieee80211_ocb_housekeeping_timer, 0);
    INIT_LIST_HEAD!(&mut ifocb.incomplete_stations);
    spin_lock_init(&mut ifocb.incomplete_lock);
}

pub unsafe fn ieee80211_ocb_join(
    sdata: *mut ieee80211_sub_if_data,
    setup: *mut ocb_setup,
) -> i32 {
    let chanreq = ieee80211_chan_req { oper: (*setup).chandef };
    let local = (*sdata).local;
    let ifocb = &mut (*sdata).u.ocb;
    let changed: u64 = BSS_CHANGED_OCB | BSS_CHANGED_BSSID;
    let err: i32;

    lockdep_assert_wiphy!((*(*sdata).local).hw.wiphy);

    if ifocb.joined == true {
        return -EINVAL;
    }

    (*sdata).deflink.operating_11g_mode = true;
    (*sdata).deflink.smps_mode = IEEE80211_SMPS_OFF;
    (*sdata).deflink.needed_rx_chains = (*(*sdata).local).rx_chains;

    err = ieee80211_link_use_channel(&mut (*sdata).deflink, &chanreq, IEEE80211_CHANCTX_SHARED);
    if err != 0 {
        return err;
    }

    ieee80211_bss_info_change_notify(sdata, changed);

    ifocb.joined = true;

    set_bit!(OCB_WORK_HOUSEKEEPING, &mut ifocb.wrkq_flags);
    wiphy_work_queue((*(*local).hw).wiphy, &mut (*sdata).work);

    netif_carrier_on((*sdata).dev);
    0
}

pub unsafe fn ieee80211_ocb_leave(sdata: *mut ieee80211_sub_if_data) -> i32 {
    let ifocb = &mut (*sdata).u.ocb;
    let local = (*sdata).local;
    let mut sta: *mut sta_info;

    lockdep_assert_wiphy!((*(*sdata).local).hw.wiphy);

    ifocb.joined = false;
    sta_info_flush(sdata, -1);

    spin_lock_bh(&mut ifocb.incomplete_lock);
    while !list_empty(&ifocb.incomplete_stations) {
        sta = list_first_entry!(&mut ifocb.incomplete_stations, sta_info, list);
        list_del(&mut (*sta).list);
        spin_unlock_bh(&mut ifocb.incomplete_lock);

        sta_info_free(local, sta);
        spin_lock_bh(&mut ifocb.incomplete_lock);
    }
    spin_unlock_bh(&mut ifocb.incomplete_lock);

    netif_carrier_off((*sdata).dev);
    clear_bit!(SDATA_STATE_OFFCHANNEL, &mut (*sdata).state);
    ieee80211_bss_info_change_notify(sdata, BSS_CHANGED_OCB);

    ieee80211_link_release_channel(&mut (*sdata).deflink);

    skb_queue_purge(&mut (*sdata).skb_queue);

    timer_delete_sync(&mut (*sdata).u.ocb.housekeeping_timer);
    /* If the timer fired while we waited for it, it will have
     * requeued the work. Now the work will be running again
     * but will not rearm the timer again because it checks
     * whether we are connected to the network or not -- at this
     * point we shouldn't be anymore.
     */

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
