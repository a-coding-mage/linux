// SPDX-License-Identifier: GPL-2.0
/*
 * Parts of this file are
 * Copyright (C) 2022-2023 Intel Corporation
 */

// Translated from the Linux wireless and cfg80211 headers and local headers.

unsafe fn ___cfg80211_stop_ap(
    rdev: *mut cfg80211_registered_device,
    dev: *mut net_device,
    link_id: u32,
    notify: bool,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let mut err: i32;

    lockdep_assert_wiphy((*wdev).wiphy);

    if (*(*rdev).ops).stop_ap.is_none() {
        return -EOPNOTSUPP;
    }

    if (*wdev).iftype != NL80211_IFTYPE_AP && (*wdev).iftype != NL80211_IFTYPE_P2P_GO {
        return -EOPNOTSUPP;
    }

    if (*wdev).links[link_id as usize].ap.beacon_interval == 0 {
        return -ENOENT;
    }

    err = rdev_stop_ap(rdev, dev, link_id);
    if err == 0 {
        (*wdev).conn_owner_nlportid = 0;
        (*wdev).links[link_id as usize].ap.beacon_interval = 0;
        core::ptr::write_bytes(
            &mut (*wdev).links[link_id as usize].ap.chandef as *mut _,
            0,
            1,
        );
        (*wdev).u.ap.ssid_len = 0;
        rdev_set_qos_map(rdev, dev, core::ptr::null_mut());
        if notify {
            nl80211_send_ap_stopped(wdev, link_id);
        }

        /* Should we apply the grace period during beaconing interface
         * shutdown also?
         */
        cfg80211_sched_dfs_chan_update(rdev);
    }

    schedule_work(&mut cfg80211_disconnect_work);

    err
}

pub unsafe fn cfg80211_stop_ap(
    rdev: *mut cfg80211_registered_device,
    dev: *mut net_device,
    link_id: i32,
    notify: bool,
) -> i32 {
    let mut link: u32;
    let mut ret: i32 = 0;

    if link_id >= 0 {
        return ___cfg80211_stop_ap(rdev, dev, link_id as u32, notify);
    }

    // C macro equivalent: for_each_valid_link(dev->ieee80211_ptr, link)
    link = 0;
    while link < IEEE80211_MLD_MAX_NUM_LINKS {
        if (*(*dev).ieee80211_ptr).links[link as usize].valid {
            let ret1 = ___cfg80211_stop_ap(rdev, dev, link, notify);

            if ret1 != 0 {
                ret = ret1;
            }
            /* try the next one also if one errored */
        }
        link += 1;
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
