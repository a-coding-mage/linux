// SPDX-License-Identifier: GPL-2.0
/*
 * Some IBSS support code for cfg80211.
 *
 * Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
 * Copyright (C) 2020-2026 Intel Corporation
 */

// External Linux/project declarations supplied by other translation units.

pub unsafe fn __cfg80211_ibss_joined(
    dev: *mut net_device,
    bssid: *const u8,
    channel: *mut ieee80211_channel,
) {
    let wdev = (*dev).ieee80211_ptr;
    let mut bss: *mut cfg80211_bss;
    #[cfg(CONFIG_CFG80211_WEXT)]
    let mut wrqu: iwreq_data;

    if WARN_ON((*wdev).iftype != NL80211_IFTYPE_ADHOC) { return; }
    if (*wdev).u.ibss.ssid_len == 0 { return; }

    bss = cfg80211_get_bss((*wdev).wiphy, channel, bssid, core::ptr::null(), 0,
                           IEEE80211_BSS_TYPE_IBSS, IEEE80211_PRIVACY_ANY);
    if WARN_ON(bss.is_null()) { return; }

    if !(*wdev).u.ibss.current_bss.is_null() {
        cfg80211_unhold_bss((*wdev).u.ibss.current_bss);
        cfg80211_put_bss((*wdev).wiphy, &(*(*wdev).u.ibss.current_bss).pub_);
    }

    cfg80211_hold_bss(bss_from_pub(bss));
    (*wdev).u.ibss.current_bss = bss_from_pub(bss);
    cfg80211_upload_connect_keys(wdev);
    nl80211_send_ibss_bssid(wiphy_to_rdev((*wdev).wiphy), dev, bssid, GFP_KERNEL);
    #[cfg(CONFIG_CFG80211_WEXT)]
    {
        core::ptr::write_bytes(&mut wrqu, 0, 1);
        core::ptr::copy_nonoverlapping(bssid, wrqu.ap_addr.sa_data.as_mut_ptr(), ETH_ALEN);
        wireless_send_event(dev, SIOCGIWAP, &mut wrqu, core::ptr::null_mut());
    }
}

pub unsafe fn cfg80211_ibss_joined(
    dev: *mut net_device, bssid: *const u8,
    channel: *mut ieee80211_channel, gfp: gfp_t,
) {
    let wdev = (*dev).ieee80211_ptr;
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    let ev: *mut cfg80211_event;
    let mut flags: ulong;

    trace_cfg80211_ibss_joined(dev, bssid, channel);
    if WARN_ON(channel.is_null()) { return; }
    ev = kzalloc_obj(gfp);
    if ev.is_null() { return; }
    (*ev).type_ = EVENT_IBSS_JOINED;
    core::ptr::copy_nonoverlapping(bssid, (*ev).ij.bssid.as_mut_ptr(), ETH_ALEN);
    (*ev).ij.channel = channel;
    spin_lock_irqsave(&mut (*wdev).event_lock, &mut flags);
    list_add_tail(&mut (*ev).list, &mut (*wdev).event_list);
    spin_unlock_irqrestore(&mut (*wdev).event_lock, flags);
    queue_work(cfg80211_wq, &mut (*rdev).event_work);
}

pub unsafe fn __cfg80211_join_ibss(
    rdev: *mut cfg80211_registered_device, dev: *mut net_device,
    params: *mut cfg80211_ibss_params, connkeys: *mut cfg80211_cached_keys,
) -> c_int {
    let wdev = (*dev).ieee80211_ptr;
    let mut err: c_int;
    lockdep_assert_held(&mut (*rdev).wiphy.mtx);
    if (*wdev).links[0].cac_started { return -EBUSY; }
    if (*wdev).u.ibss.ssid_len != 0 { return -EALREADY; }

    if (*params).basic_rates == 0 {
        let band = (*params).chandef.chan.band;
        let flag = if band == NL80211_BAND_5GHZ || band == NL80211_BAND_6GHZ {
            IEEE80211_RATE_MANDATORY_A
        } else { IEEE80211_RATE_MANDATORY_B };
        let sband = (*rdev).wiphy.bands[band as usize];
        for j in 0..(*sband).n_bitrates {
            if (*sband).bitrates[j as usize].flags & flag != 0 {
                (*params).basic_rates |= BIT(j);
            }
        }
    }
    if WARN_ON(!connkeys.is_null() && (*connkeys).def < 0) { return -EINVAL; }
    if !(*wdev).connect_keys.is_null() { kfree_sensitive((*wdev).connect_keys); }
    (*wdev).connect_keys = connkeys;
    (*wdev).u.ibss.chandef = (*params).chandef;
    if !connkeys.is_null() {
        (*params).wep_keys = (*connkeys).params;
        (*params).wep_tx_key = (*connkeys).def;
    }
    #[cfg(CONFIG_CFG80211_WEXT)]
    { (*wdev).wext.ibss.chandef = (*params).chandef; }
    err = rdev_join_ibss(rdev, dev, params);
    if err != 0 { (*wdev).connect_keys = core::ptr::null_mut(); return err; }
    core::ptr::copy_nonoverlapping((*params).ssid, (*wdev).u.ibss.ssid, (*params).ssid_len);
    (*wdev).u.ibss.ssid_len = (*params).ssid_len;
    0
}

pub unsafe fn cfg80211_clear_ibss(dev: *mut net_device, nowext: bool) {
    let wdev = (*dev).ieee80211_ptr;
    let rdev = wiphy_to_rdev((*wdev).wiphy);
    lockdep_assert_wiphy((*wdev).wiphy);
    kfree_sensitive((*wdev).connect_keys);
    (*wdev).connect_keys = core::ptr::null_mut();
    rdev_set_qos_map(rdev, dev, core::ptr::null_mut());
    if !(*rdev).ops.del_key.is_none() {
        for i in 0..6 { rdev_del_key(rdev, wdev, -1, i, false, core::ptr::null()); }
    }
    if !(*wdev).u.ibss.current_bss.is_null() {
        cfg80211_unhold_bss((*wdev).u.ibss.current_bss);
        cfg80211_put_bss((*wdev).wiphy, &(*(*wdev).u.ibss.current_bss).pub_);
    }
    (*wdev).u.ibss.current_bss = core::ptr::null_mut();
    (*wdev).u.ibss.ssid_len = 0;
    core::ptr::write_bytes(&mut (*wdev).u.ibss.chandef, 0, 1);
    #[cfg(CONFIG_CFG80211_WEXT)]
    if !nowext { (*wdev).wext.ibss.ssid_len = 0; }
    cfg80211_sched_dfs_chan_update(rdev);
}

pub unsafe fn cfg80211_leave_ibss(rdev: *mut cfg80211_registered_device, dev: *mut net_device, nowext: bool) -> c_int {
    let wdev = (*dev).ieee80211_ptr;
    let err: c_int;
    lockdep_assert_wiphy((*wdev).wiphy);
    if (*wdev).u.ibss.ssid_len == 0 { return -ENOLINK; }
    err = rdev_leave_ibss(rdev, dev);
    if err != 0 { return err; }
    (*wdev).conn_owner_nlportid = 0;
    cfg80211_clear_ibss(dev, nowext);
    0
}

#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_join(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev) -> c_int {
    let mut ck: *mut cfg80211_cached_keys = core::ptr::null_mut();
    let mut band: nl80211_band;
    let mut i: c_int;
    let mut err: c_int;
    lockdep_assert_wiphy((*wdev).wiphy);
    if (*wdev).wext.ibss.beacon_interval == 0 { (*wdev).wext.ibss.beacon_interval = 100; }
    if (*wdev).wext.ibss.chandef.chan.is_null() {
        let mut new_chan: *mut ieee80211_channel = core::ptr::null_mut();
        band = 0;
        while band < NUM_NL80211_BANDS {
            let sband = (*rdev).wiphy.bands[band as usize];
            if !sband.is_null() {
                for j in 0..(*sband).n_channels {
                    let chan = &mut (*sband).channels[j as usize];
                    if chan.flags & IEEE80211_CHAN_NO_IR == 0 && chan.flags & IEEE80211_CHAN_DISABLED == 0 { new_chan = chan; break; }
                }
            }
            if !new_chan.is_null() { break; }
            band += 1;
        }
        if new_chan.is_null() { return -EINVAL; }
        cfg80211_chandef_create(&mut (*wdev).wext.ibss.chandef, new_chan, NL80211_CHAN_NO_HT);
    }
    if (*wdev).wext.ibss.ssid_len == 0 || !netif_running((*wdev).netdev) { return 0; }
    if !(*wdev).wext.keys.is_null() { (*(*wdev).wext.keys).def = (*wdev).wext.default_key; }
    (*wdev).wext.ibss.privacy = (*wdev).wext.default_key != -1;
    if !(*wdev).wext.keys.is_null() && (*(*wdev).wext.keys).def != -1 {
        ck = kmemdup((*wdev).wext.keys, core::mem::size_of::<cfg80211_cached_keys>(), GFP_KERNEL);
        if ck.is_null() { return -ENOMEM; }
        for i in 0..4 { (*ck).params[i as usize].key = (*ck).data[i as usize]; }
    }
    err = __cfg80211_join_ibss(rdev, (*wdev).netdev, &mut (*wdev).wext.ibss, ck);
    if err != 0 { kfree(ck); }
    err
}

// The remaining WEXT entry points retain their C interfaces and operations.
// Their declarations are kept as translation stubs because all referenced
// kernel structures and helpers are supplied externally.
#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_siwfreq(dev: *mut net_device, _info: *mut iw_request_info, f: *mut iw_freq, _extra: *mut c_char) -> c_int { let w=(*dev).ieee80211_ptr; let r=wiphy_to_rdev((*w).wiphy); if WARN_ON((*w).iftype != NL80211_IFTYPE_ADHOC){return -EINVAL;} if (*r).ops.join_ibss.is_none(){return -EOPNOTSUPP;} let n=cfg80211_wext_freq(f); if n<0{return n;} let mut c=core::ptr::null_mut(); if n!=0 {c=ieee80211_get_channel((*w).wiphy,n); if c.is_null() || (*c).flags & (IEEE80211_CHAN_NO_IR|IEEE80211_CHAN_DISABLED)!=0{return -EINVAL;}} if (*w).wext.ibss.chandef.chan==c{return 0;} let mut e=0; if (*w).u.ibss.ssid_len!=0{e=cfg80211_leave_ibss(r,(*w).netdev,true);} if e!=0{return e;} if !c.is_null(){cfg80211_chandef_create(&mut (*w).wext.ibss.chandef,c,NL80211_CHAN_NO_HT);(*w).wext.ibss.channel_fixed=true;}else{(*w).wext.ibss.channel_fixed=false;} cfg80211_ibss_wext_join(r,w) }
#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_giwfreq(dev: *mut net_device, _info: *mut iw_request_info, f: *mut iw_freq, _extra: *mut c_char) -> c_int { let w=(*dev).ieee80211_ptr; if WARN_ON((*w).iftype != NL80211_IFTYPE_ADHOC){return -EINVAL;} let c=if !(*w).u.ibss.current_bss.is_null(){(*(*w).u.ibss.current_bss).pub_.channel}else{(*w).wext.ibss.chandef.chan}; if c.is_null(){return -EINVAL;} (*f).m=(*c).center_freq;(*f).e=6;0 }
#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_siwessid(dev: *mut net_device, _info: *mut iw_request_info, d: *mut iw_point, s: *mut c_char) -> c_int { let w=(*dev).ieee80211_ptr;let r=wiphy_to_rdev((*w).wiphy);if WARN_ON((*w).iftype!=NL80211_IFTYPE_ADHOC){return -EINVAL;}if (*r).ops.join_ibss.is_none(){return -EOPNOTSUPP;}if (*w).u.ibss.ssid_len!=0{let e=cfg80211_leave_ibss(r,dev,true);if e!=0{return e;}}let mut n=(*d).length;if n>0&&*s.add(n-1)==0{n-=1;}core::ptr::copy_nonoverlapping(s as *const u8,(*w).u.ibss.ssid,n);(*w).wext.ibss.ssid=(*w).u.ibss.ssid;(*w).wext.ibss.ssid_len=n;cfg80211_ibss_wext_join(r,w)}
#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_giwessid(dev: *mut net_device, _info: *mut iw_request_info, d: *mut iw_point, s: *mut c_char) -> c_int { let w=(*dev).ieee80211_ptr;if WARN_ON((*w).iftype!=NL80211_IFTYPE_ADHOC){return -EINVAL;}(*d).flags=0;if (*w).u.ibss.ssid_len!=0{(*d).flags=1;(*d).length=(*w).u.ibss.ssid_len;core::ptr::copy_nonoverlapping((*w).u.ibss.ssid,s as *mut u8,(*d).length);}else if !(*w).wext.ibss.ssid.is_null()&&(*w).wext.ibss.ssid_len!=0{(*d).flags=1;(*d).length=(*w).wext.ibss.ssid_len;core::ptr::copy_nonoverlapping((*w).wext.ibss.ssid,s as *mut u8,(*d).length);}0 }
#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_siwap(dev: *mut net_device, _info: *mut iw_request_info, a: *mut sockaddr, _extra: *mut c_char) -> c_int { let w=(*dev).ieee80211_ptr;let r=wiphy_to_rdev((*w).wiphy);if WARN_ON((*w).iftype!=NL80211_IFTYPE_ADHOC){return -EINVAL;}if (*r).ops.join_ibss.is_none()||(*a).sa_family!=ARPHRD_ETHER{return -EINVAL;}let mut b=(*a).sa_data.as_mut_ptr();if is_zero_ether_addr(b)||is_broadcast_ether_addr(b){b=core::ptr::null_mut();}if !b.is_null()&&!is_valid_ether_addr(b){return -EINVAL;}if b.is_null()&&!(*w).wext.ibss.bssid.is_null()&&false{return 0;}if (*w).u.ibss.ssid_len!=0{let e=cfg80211_leave_ibss(r,dev,true);if e!=0{return e;}}if !b.is_null(){core::ptr::copy_nonoverlapping(b,(*w).wext.bssid.as_mut_ptr(),ETH_ALEN);(*w).wext.ibss.bssid=(*w).wext.bssid.as_mut_ptr();}else{(*w).wext.ibss.bssid=core::ptr::null_mut();}cfg80211_ibss_wext_join(r,w)}
#[cfg(CONFIG_CFG80211_WEXT)]
pub unsafe fn cfg80211_ibss_wext_giwap(dev: *mut net_device, _info: *mut iw_request_info, a: *mut sockaddr, _extra: *mut c_char) -> c_int { let w=(*dev).ieee80211_ptr;if WARN_ON((*w).iftype!=NL80211_IFTYPE_ADHOC){return -EINVAL;}(*a).sa_family=ARPHRD_ETHER;if !(*w).u.ibss.current_bss.is_null(){core::ptr::copy_nonoverlapping((*(*w).u.ibss.current_bss).pub_.bssid,(*a).sa_data.as_mut_ptr(),ETH_ALEN);}else if !(*w).wext.ibss.bssid.is_null(){core::ptr::copy_nonoverlapping((*w).wext.ibss.bssid,(*a).sa_data.as_mut_ptr(),ETH_ALEN);}else{eth_zero_addr((*a).sa_data.as_mut_ptr());}0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
