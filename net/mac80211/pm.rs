// SPDX-License-Identifier: GPL-2.0
/*
 * Portions
 * Copyright (C) 2020-2021, 2023-2024 Intel Corporation
 */

// Dependencies supplied by the surrounding mac80211 translation.

unsafe fn ieee80211_sched_scan_cancel(local: *mut ieee80211_local) {
    if ieee80211_request_sched_scan_stop(local) {
        return;
    }
    cfg80211_sched_scan_stopped_locked((*(*local).hw.wiphy), 0);
}

pub unsafe fn __ieee80211_suspend(
    hw: *mut ieee80211_hw,
    wowlan: *mut cfg80211_wowlan,
) -> i32 {
    let local = hw_to_local(hw);
    let mut sdata: *mut ieee80211_sub_if_data;
    let mut sta: *mut sta_info;

    if (*local).open_count == 0 {
        goto_suspend!(local);
    }

    (*local).suspending = true;
    mb(); /* make suspending visible before any cancellation */

    ieee80211_scan_cancel(local);
    ieee80211_dfs_cac_cancel(local, core::ptr::null_mut());
    ieee80211_roc_purge(local, core::ptr::null_mut());
    ieee80211_del_virtual_monitor(local);

    if ieee80211_hw_check(hw, AMPDU_AGGREGATION) && !(wowlan.is_null() || (*wowlan).any == 0) {
        lockdep_assert_wiphy((*local).hw.wiphy);
        list_for_each_entry!(sta, (*local).sta_list, list, {
            set_sta_flag(sta, WLAN_STA_BLOCK_BA);
            ieee80211_sta_tear_down_BA_sessions(sta, AGG_STOP_LOCAL_REQUEST);
        });
    }

    /* keep sched_scan only in case of 'any' trigger */
    if wowlan.is_null() || (*wowlan).any == 0 {
        ieee80211_sched_scan_cancel(local);
    }

    ieee80211_stop_queues_by_reason(
        hw, IEEE80211_MAX_QUEUE_MAP, IEEE80211_QUEUE_STOP_REASON_SUSPEND, false,
    );
    /* flush out all packets */
    synchronize_net();
    ieee80211_flush_queues(local, core::ptr::null_mut(), true);

    (*local).quiescing = true;
    /* make quiescing visible to timers everywhere */
    mb();
    flush_workqueue((*local).workqueue);
    /* Don't try to run timers while suspended. */
    timer_delete_sync(&mut (*local).sta_cleanup);
    /*
     * Note that this particular timer doesn't need to be
     * restarted at resume.
     */
    wiphy_work_cancel((*local).hw.wiphy, &mut (*local).dynamic_ps_enable_work);
    timer_delete_sync(&mut (*local).dynamic_ps_timer);

    (*local).wowlan = wowlan;
    if !(*local).wowlan.is_null() {
        let err: i32;

        /* Drivers don't expect to suspend while some operations like
         * authenticating or associating are in progress. It doesn't
         * make sense anyway to accept that, since the authentication
         * or association would never finish since the driver can't do
         * that on its own.
         * Thus, clean up in-progress auth/assoc first.
         */
        list_for_each_entry!(sdata, (*local).interfaces, list, {
            if !ieee80211_sdata_running(sdata) || (*sdata).vif.r#type != NL80211_IFTYPE_STATION {
                continue;
            }
            ieee80211_mgd_quiesce(sdata);
            /* If suspended during TX in progress, and wowlan is enabled,
             * ensure the driver is returned to power-save mode. */
            if (*sdata).u.mgd.associated && (*sdata).u.mgd.powersave
                && ((*local).hw.conf.flags & IEEE80211_CONF_PS) == 0
            {
                (*local).hw.conf.flags |= IEEE80211_CONF_PS;
                ieee80211_hw_config(local, -1, IEEE80211_CONF_CHANGE_PS);
            }
        });

        err = drv_suspend(local, wowlan);
        if err < 0 {
            (*local).quiescing = false;
            (*local).wowlan = core::ptr::null_mut();
            if ieee80211_hw_check(hw, AMPDU_AGGREGATION) {
                lockdep_assert_wiphy((*local).hw.wiphy);
                list_for_each_entry!(sta, (*local).sta_list, list, {
                    clear_sta_flag(sta, WLAN_STA_BLOCK_BA);
                });
            }
            ieee80211_wake_queues_by_reason(
                hw, IEEE80211_MAX_QUEUE_MAP, IEEE80211_QUEUE_STOP_REASON_SUSPEND, false,
            );
            return err;
        } else if err > 0 {
            WARN_ON(err != 1);
            /* cfg80211 will call back into mac80211 to disconnect
             * all interfaces, allow that to proceed properly */
            ieee80211_wake_queues_by_reason(
                hw, IEEE80211_MAX_QUEUE_MAP, IEEE80211_QUEUE_STOP_REASON_SUSPEND, false,
            );
            return err;
        }
    }

    /* remove all interfaces that were created in the driver */
    list_for_each_entry!(sdata, (*local).interfaces, list, {
        if !ieee80211_sdata_running(sdata) {
            continue;
        }
        match (*sdata).vif.r#type {
            NL80211_IFTYPE_AP_VLAN | NL80211_IFTYPE_MONITOR => continue,
            NL80211_IFTYPE_STATION => ieee80211_mgd_quiesce(sdata),
            _ => (),
        }
        wiphy_delayed_work_flush((*local).hw.wiphy, &mut (*sdata).dec_tailroom_needed_wk);
        drv_remove_interface(local, sdata);
    });

    /* We disconnected on all interfaces before suspend, all channel
     * contexts should be released. */
    WARN_ON(!list_empty(&(*local).chanctx_list));
    /* stop hardware - this must stop RX */
    ieee80211_stop_device(local, true);

    goto_suspend!(local);
}

pub unsafe fn ieee80211_report_wowlan_wakeup(
    vif: *mut ieee80211_vif,
    wakeup: *mut cfg80211_wowlan_wakeup,
    gfp: gfp_t,
) {
    let sdata = vif_to_sdata(vif);
    cfg80211_report_wowlan_wakeup(&mut (*sdata).wdev, wakeup, gfp);
}

// EXPORT_SYMBOL(ieee80211_report_wowlan_wakeup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
