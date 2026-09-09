// SPDX-License-Identifier: ISC
/*
 * Copyright (C) 2017 Rafał Miłecki <rafal@milecki.pl>
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here without reimplementing them.

use core::ffi::c_void;
use core::ptr;

unsafe fn wiphy_freq_limits_valid_chan(
    _wiphy: *mut wiphy,
    freq_limits: *mut ieee80211_freq_range,
    n_freq_limits: u32,
    chan: *mut ieee80211_channel,
) -> bool {
    let bw: u32 = 20u32.wrapping_mul(1000);

    for i in 0..n_freq_limits {
        let limit = freq_limits.add(i as usize);

        if cfg80211_does_bw_fit_range(
            limit,
            ((*chan).center_freq as u32).wrapping_mul(1000),
            bw,
        ) {
            return true;
        }
    }

    false
}

unsafe fn wiphy_freq_limits_apply(
    wiphy: *mut wiphy,
    freq_limits: *mut ieee80211_freq_range,
    n_freq_limits: u32,
) {
    if n_freq_limits == 0 {
        // WARN_ON(!n_freq_limits)
        return;
    }

    for band in 0..NUM_NL80211_BANDS {
        let sband = (*wiphy).bands[band as usize];

        if sband.is_null() {
            continue;
        }

        for i in 0..(*sband).n_channels {
            let chan = (*sband).channels.add(i as usize);

            if (*chan).flags & IEEE80211_CHAN_DISABLED != 0 {
                continue;
            }

            if !wiphy_freq_limits_valid_chan(wiphy, freq_limits, n_freq_limits, chan) {
                pr_debug!(
                    "Disabling freq {} MHz as it's out of OF limits\n",
                    (*chan).center_freq
                );
                (*chan).flags |= IEEE80211_CHAN_DISABLED;
            }
        }
    }
}

pub unsafe fn wiphy_read_of_freq_limits(wiphy: *mut wiphy) {
    let dev = wiphy_dev(wiphy);
    let mut np: *mut device_node;
    let mut prop: *mut property;
    let mut freq_limits: *mut ieee80211_freq_range = ptr::null_mut();
    let mut n_freq_limits: u32;
    let mut p: *const u32;
    let mut len: i32 = 0;
    let mut err: i32 = 0;

    if dev.is_null() {
        return;
    }
    np = dev_of_node(dev);
    if np.is_null() {
        return;
    }

    prop = of_find_property(np, b"ieee80211-freq-limit\0".as_ptr() as *const i8, &mut len);
    if prop.is_null() {
        return;
    }

    if len == 0 || len % core::mem::size_of::<u32>() as i32 != 0
        || (len / core::mem::size_of::<u32>() as i32) % 2 != 0
    {
        dev_err!(dev, "ieee80211-freq-limit wrong format");
        return;
    }
    n_freq_limits = (len / core::mem::size_of::<u32>() as i32 / 2) as u32;

    freq_limits = kzalloc_objs::<ieee80211_freq_range>(n_freq_limits);
    if freq_limits.is_null() {
        err = -ENOMEM;
        goto_out_kfree(dev, freq_limits, err);
        return;
    }

    p = ptr::null();
    for i in 0..n_freq_limits {
        let limit = freq_limits.add(i as usize);

        p = of_prop_next_u32(prop, p, &mut (*limit).start_freq_khz);
        if p.is_null() {
            err = -EINVAL;
            goto_out_kfree(dev, freq_limits, err);
            return;
        }

        p = of_prop_next_u32(prop, p, &mut (*limit).end_freq_khz);
        if p.is_null() {
            err = -EINVAL;
            goto_out_kfree(dev, freq_limits, err);
            return;
        }

        if (*limit).start_freq_khz == 0
            || (*limit).end_freq_khz == 0
            || (*limit).start_freq_khz >= (*limit).end_freq_khz
        {
            err = -EINVAL;
            goto_out_kfree(dev, freq_limits, err);
            return;
        }
    }

    wiphy_freq_limits_apply(wiphy, freq_limits, n_freq_limits);
    goto_out_kfree(dev, freq_limits, err);
}

unsafe fn goto_out_kfree(
    dev: *mut device,
    freq_limits: *mut ieee80211_freq_range,
    err: i32,
) {
    kfree(freq_limits as *mut c_void);
    if err != 0 {
        dev_err!(dev, "Failed to get limits: {}\n", err);
    }
}

extern "C" {
    fn cfg80211_does_bw_fit_range(
        limit: *mut ieee80211_freq_range,
        center_freq_khz: u32,
        bw: u32,
    ) -> bool;
    fn wiphy_dev(wiphy: *mut wiphy) -> *mut device;
    fn dev_of_node(dev: *mut device) -> *mut device_node;
    fn of_find_property(np: *mut device_node, name: *const i8, len: *mut i32) -> *mut property;
    fn of_prop_next_u32(prop: *mut property, cur: *const u32, value: *mut u32) -> *const u32;
    fn kzalloc_objs<T>(count: u32) -> *mut T;
    fn kfree(ptr: *mut c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
