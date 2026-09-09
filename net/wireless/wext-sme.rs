// SPDX-License-Identifier: GPL-2.0
/*
 * cfg80211 wext compat for managed mode.
 *
 * Copyright 2009\tJohannes Berg <johannes@sipsolutions.net>
 * Copyright (C) 2009, 2020-2023 Intel Corporation
 */

// External Linux kernel declarations and constants are supplied by other files.

pub unsafe fn cfg80211_mgd_wext_connect(
    rdev: *mut cfg80211_registered_device,
    wdev: *mut wireless_dev,
) -> i32 {
    let mut ck: *mut cfg80211_cached_keys = core::ptr::null_mut();
    let mut prev_bssid: *const u8 = core::ptr::null();
    let err: i32;
    let mut i: i32;

    ASSERT_RTNL();
    lockdep_assert_wiphy((*wdev).wiphy);

    if !netif_running((*wdev).netdev) {
        return 0;
    }

    (*wdev).wext.connect.ie = (*wdev).wext.ie;
    (*wdev).wext.connect.ie_len = (*wdev).wext.ie_len;

    /* Use default background scan period */
    (*wdev).wext.connect.bg_scan_period = -1;

    if !(*wdev).wext.keys.is_null() {
        (*(*wdev).wext.keys).def = (*wdev).wext.default_key;
        if (*wdev).wext.default_key != -1 {
            (*wdev).wext.connect.privacy = true;
        }
    }

    if (*wdev).wext.connect.ssid_len == 0 {
        return 0;
    }

    if !(*wdev).wext.keys.is_null() && (*(*wdev).wext.keys).def != -1 {
        ck = kmemdup(
            (*wdev).wext.keys as *const core::ffi::c_void,
            core::mem::size_of::<cfg80211_cached_keys>(),
            GFP_KERNEL,
        ) as *mut cfg80211_cached_keys;
        if ck.is_null() {
            return -ENOMEM;
        }
        i = 0;
        while i < 4 {
            (*ck).params[i as usize].key = (*ck).data[i as usize].as_mut_ptr();
            i += 1;
        }
    }

    if (*wdev).wext.prev_bssid_valid {
        prev_bssid = (*wdev).wext.prev_bssid.as_ptr();
    }

    err = cfg80211_connect(
        rdev,
        (*wdev).netdev,
        &mut (*wdev).wext.connect,
        ck,
        prev_bssid,
    );
    if err != 0 {
        kfree_sensitive(ck as *mut core::ffi::c_void);
    }

    err
}

pub unsafe fn cfg80211_mgd_wext_siwfreq(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    wextfreq: *mut iw_freq,
    _extra: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    let mut chan: *mut ieee80211_channel = core::ptr::null_mut();
    let err: i32;
    let freq: i32;

    /* call only for station! */
    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_STATION) {
        return -EINVAL;
    }

    freq = cfg80211_wext_freq(wextfreq);
    if freq < 0 {
        return freq;
    }

    if freq != 0 {
        chan = ieee80211_get_channel((*wdev).wiphy, freq);
        if chan.is_null() {
            return -EINVAL;
        }
        if ((*chan).flags & IEEE80211_CHAN_DISABLED) != 0 {
            return -EINVAL;
        }
    }

    if !(*wdev).conn.is_null() {
        let mut event = true;

        if (*wdev).wext.connect.channel == chan {
            return 0;
        }

        /* if SSID set, we'll try right again, avoid event */
        if (*wdev).wext.connect.ssid_len != 0 {
            event = false;
        }
        err = cfg80211_disconnect(rdev, dev, WLAN_REASON_DEAUTH_LEAVING, event);
        if err != 0 {
            return err;
        }
    }

    (*wdev).wext.connect.channel = chan;
    cfg80211_mgd_wext_connect(rdev, wdev)
}

pub unsafe fn cfg80211_mgd_wext_giwfreq(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    freq: *mut iw_freq,
    _extra: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let mut chan: *mut ieee80211_channel = core::ptr::null_mut();

    /* call only for station! */
    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_STATION) {
        return -EINVAL;
    }

    if (*wdev).valid_links != 0 {
        return -EOPNOTSUPP;
    }

    if !(*wdev).links[0].client.current_bss.is_null() {
        chan = (*wdev).links[0].client.current_bss.as_ref().unwrap().pub_.channel;
    } else if !(*wdev).wext.connect.channel.is_null() {
        chan = (*wdev).wext.connect.channel;
    }

    if !chan.is_null() {
        (*freq).m = (*chan).center_freq;
        (*freq).e = 6;
        return 0;
    }

    /* no channel if not joining */
    -EINVAL
}

pub unsafe fn cfg80211_mgd_wext_siwessid(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    data: *mut iw_point,
    ssid: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    let mut len = (*data).length as usize;
    let err: i32;

    /* call only for station! */
    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_STATION) {
        return -EINVAL;
    }

    if (*data).flags == 0 {
        len = 0;
    }

    /* iwconfig uses nul termination in SSID.. */
    if len > 0 && *(ssid.add(len - 1) as *const u8) == 0 {
        len -= 1;
    }

    if !(*wdev).conn.is_null() {
        let mut event = true;

        if !(*wdev).wext.connect.ssid.is_null()
            && len != 0
            && len == (*wdev).wext.connect.ssid_len
            && memcmp((*wdev).wext.connect.ssid, ssid, len) == 0
        {
            return 0;
        }

        /* if SSID set now, we'll try to connect, avoid event */
        if len != 0 {
            event = false;
        }
        err = cfg80211_disconnect(rdev, dev, WLAN_REASON_DEAUTH_LEAVING, event);
        if err != 0 {
            return err;
        }
    }

    (*wdev).wext.prev_bssid_valid = false;
    (*wdev).wext.connect.ssid = (*wdev).wext.ssid.as_mut_ptr();
    memcpy((*wdev).wext.ssid.as_mut_ptr() as *mut core::ffi::c_void,
           ssid as *const core::ffi::c_void, len);
    (*wdev).wext.connect.ssid_len = len;

    (*wdev).wext.connect.crypto.control_port = false;
    (*wdev).wext.connect.crypto.control_port_ethertype = cpu_to_be16(ETH_P_PAE);

    cfg80211_mgd_wext_connect(rdev, wdev)
}

pub unsafe fn cfg80211_mgd_wext_giwessid(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    data: *mut iw_point,
    ssid: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let mut ret = 0;

    /* call only for station! */
    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_STATION) {
        return -EINVAL;
    }

    if (*wdev).valid_links != 0 {
        return -EINVAL;
    }

    (*data).flags = 0;

    if !(*wdev).links[0].client.current_bss.is_null() {
        let ssid_elem: *const element;

        rcu_read_lock();
        ssid_elem = ieee80211_bss_get_elem(
            &(*wdev).links[0].client.current_bss.as_ref().unwrap().pub_,
            WLAN_EID_SSID,
        );
        if !ssid_elem.is_null() {
            (*data).flags = 1;
            (*data).length = (*ssid_elem).datalen;
            if (*data).length > IW_ESSID_MAX_SIZE {
                ret = -EINVAL;
            } else {
                memcpy(ssid as *mut core::ffi::c_void, (*ssid_elem).data, (*data).length);
            }
        }
        rcu_read_unlock();
    } else if !(*wdev).wext.connect.ssid.is_null() && (*wdev).wext.connect.ssid_len != 0 {
        (*data).flags = 1;
        (*data).length = (*wdev).wext.connect.ssid_len;
        memcpy(ssid as *mut core::ffi::c_void, (*wdev).wext.connect.ssid, (*data).length);
    }

    ret
}

pub unsafe fn cfg80211_mgd_wext_siwap(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    ap_addr: *mut sockaddr,
    _extra: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    let mut bssid = (*ap_addr).sa_data.as_mut_ptr() as *mut u8;
    let err: i32;

    /* call only for station! */
    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_STATION) {
        return -EINVAL;
    }

    if (*ap_addr).sa_family != ARPHRD_ETHER {
        return -EINVAL;
    }

    /* automatic mode */
    if is_zero_ether_addr(bssid) || is_broadcast_ether_addr(bssid) {
        bssid = core::ptr::null_mut();
    }

    if !(*wdev).conn.is_null() {
        /* both automatic */
        if bssid.is_null() && (*wdev).wext.connect.bssid.is_null() {
            return 0;
        }

        /* fixed already - and no change */
        if !(*wdev).wext.connect.bssid.is_null()
            && !bssid.is_null()
            && ether_addr_equal(bssid, (*wdev).wext.connect.bssid)
        {
            return 0;
        }

        err = cfg80211_disconnect(rdev, dev, WLAN_REASON_DEAUTH_LEAVING, false);
        if err != 0 {
            return err;
        }
    }

    if !bssid.is_null() {
        memcpy((*wdev).wext.bssid.as_mut_ptr() as *mut core::ffi::c_void,
               bssid as *const core::ffi::c_void, ETH_ALEN);
        (*wdev).wext.connect.bssid = (*wdev).wext.bssid.as_mut_ptr();
    } else {
        (*wdev).wext.connect.bssid = core::ptr::null_mut();
    }

    cfg80211_mgd_wext_connect(rdev, wdev)
}

pub unsafe fn cfg80211_mgd_wext_giwap(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    ap_addr: *mut sockaddr,
    _extra: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;

    /* call only for station! */
    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_STATION) {
        return -EINVAL;
    }

    (*ap_addr).sa_family = ARPHRD_ETHER;

    if (*wdev).valid_links != 0 {
        return -EOPNOTSUPP;
    }

    if !(*wdev).links[0].client.current_bss.is_null() {
        memcpy((*ap_addr).sa_data.as_mut_ptr() as *mut core::ffi::c_void,
               (*wdev).links[0].client.current_bss.as_ref().unwrap().pub_.bssid,
               ETH_ALEN);
    } else {
        eth_zero_addr((*ap_addr).sa_data.as_mut_ptr() as *mut u8);
    }

    0
}

pub unsafe fn cfg80211_wext_siwgenie(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    wrqu: *mut iwreq_data,
    extra: *mut i8,
) -> i32 {
    let data = &mut (*wrqu).data;
    let wdev = (*dev).ieee80211_ptr;
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    let ie_len = data.length as usize;
    let mut ie = extra as *mut u8;

    if (*wdev).iftype != NL80211_IFTYPE_STATION {
        return -EOPNOTSUPP;
    }

    if ie_len == 0 {
        ie = core::ptr::null_mut();
    }

    let _wiphy_guard = guard(wiphy)((*wdev).wiphy);

    /* no change */
    if (*wdev).wext.ie_len == ie_len
        && memcmp((*wdev).wext.ie, ie, ie_len) == 0
    {
        return 0;
    }

    if ie_len != 0 {
        let mut elem: *const element = core::ptr::null();

        for_each_element(elem, extra, ie_len) {
            /* nothing */
        }

        if !for_each_element_completed(elem, extra, ie_len) {
            return -EINVAL;
        }

        ie = kmemdup(extra as *const core::ffi::c_void, ie_len, GFP_KERNEL) as *mut u8;
        if ie.is_null() {
            return -ENOMEM;
        }
    } else {
        ie = core::ptr::null_mut();
    }

    kfree((*wdev).wext.ie as *mut core::ffi::c_void);
    (*wdev).wext.ie = ie;
    (*wdev).wext.ie_len = ie_len;

    if !(*wdev).conn.is_null() {
        return cfg80211_disconnect(rdev, dev, WLAN_REASON_DEAUTH_LEAVING, false);
    }

    /* userspace better not think we'll reconnect */
    0
}

pub unsafe fn cfg80211_wext_siwmlme(
    dev: *mut net_device,
    _info: *mut iw_request_info,
    _wrqu: *mut iwreq_data,
    extra: *mut i8,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let mlme = extra as *mut iw_mlme;
    let rdev: *mut cfg80211_registered_device;

    if wdev.is_null() {
        return -EOPNOTSUPP;
    }

    rdev = wiphy_to_rdev((*wdev).wiphy);

    if (*wdev).iftype != NL80211_IFTYPE_STATION {
        return -EINVAL;
    }

    if (*mlme).addr.sa_family != ARPHRD_ETHER {
        return -EINVAL;
    }

    let _wiphy_guard = guard(wiphy)(&mut (*rdev).wiphy);

    match (*mlme).cmd {
        IW_MLME_DEAUTH | IW_MLME_DISASSOC => {
            cfg80211_disconnect(rdev, dev, (*mlme).reason_code, true)
        }
        _ => -EOPNOTSUPP,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
