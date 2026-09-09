// SPDX-License-Identifier: GPL-2.0-only
/*
 * OCB mode implementation
 *
 * Copyright: (c) 2014 Czech Technical University in Prague
 *            (c) 2014 Volkswagen Group Research
 * Copyright (C) 2022-2023 Intel Corporation
 * Author:    Rostislav Lisovy <rostislav.lisovy@fel.cvut.cz>
 * Funded by: Volkswagen Group Research
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/ieee80211.h, net/cfg80211.h, nl80211.h, core.h, and rdev-ops.h.

unsafe extern "C" {
    fn rdev_join_ocb(
        rdev: *mut cfg80211_registered_device,
        dev: *mut net_device,
        setup: *mut ocb_setup,
    ) -> c_int;
    fn rdev_leave_ocb(
        rdev: *mut cfg80211_registered_device,
        dev: *mut net_device,
    ) -> c_int;
    fn lockdep_assert_wiphy(wiphy: *mut wiphy);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn WARN_ON(condition: bool) -> bool;
}

unsafe extern "C" {
    type cfg80211_registered_device;
    type net_device;
    type ocb_setup;
    type wiphy;
    type wireless_dev;
}

type c_int = i32;
type c_void = core::ffi::c_void;

const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const ENOTCONN: c_int = 107;
const NL80211_IFTYPE_OCB: i32 = 6;

pub unsafe fn cfg80211_join_ocb(
    rdev: *mut cfg80211_registered_device,
    dev: *mut net_device,
    setup: *mut ocb_setup,
) -> c_int {
    let wdev: *mut wireless_dev = (*dev).ieee80211_ptr;
    let mut err: c_int;

    lockdep_assert_wiphy((*wdev).wiphy);

    if (*dev).ieee80211_ptr.iftype != NL80211_IFTYPE_OCB {
        return -EOPNOTSUPP;
    }

    if (*rdev).ops.join_ocb.is_none() {
        return -EOPNOTSUPP;
    }

    if WARN_ON((*setup).chandef.chan.is_null()) {
        return -EINVAL;
    }

    err = rdev_join_ocb(rdev, dev, setup);
    if err == 0 {
        (*wdev).u.ocb.chandef = (*setup).chandef;
    }

    err
}

pub unsafe fn cfg80211_leave_ocb(
    rdev: *mut cfg80211_registered_device,
    dev: *mut net_device,
) -> c_int {
    let wdev: *mut wireless_dev = (*dev).ieee80211_ptr;
    let mut err: c_int;

    lockdep_assert_wiphy((*wdev).wiphy);

    if (*dev).ieee80211_ptr.iftype != NL80211_IFTYPE_OCB {
        return -EOPNOTSUPP;
    }

    if (*rdev).ops.leave_ocb.is_none() {
        return -EOPNOTSUPP;
    }

    if (*wdev).u.ocb.chandef.chan.is_null() {
        return -ENOTCONN;
    }

    err = rdev_leave_ocb(rdev, dev);
    if err == 0 {
        memset(
            core::ptr::addr_of_mut!((*wdev).u.ocb.chandef) as *mut c_void,
            0,
            core::mem::size_of_val(&(*wdev).u.ocb.chandef),
        );
    }

    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
