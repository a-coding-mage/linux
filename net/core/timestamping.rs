// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PTP 1588 clock support - support for timestamping in PHY devices
 *
 * Copyright (C) 2010 OMICRON electronics GmbH
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn classify(skb: *const sk_buff) -> ::core::ffi::c_uint {
    if likely(!(*skb).dev.is_null()
        && !(*(*skb).dev).phydev.is_null()
        && !(*(*(*skb).dev).phydev).mii_ts.is_null())
    {
        ptp_classify_raw(skb)
    } else {
        PTP_CLASS_NONE
    }
}

pub unsafe fn skb_clone_tx_timestamp(skb: *mut sk_buff) {
    let mut hwprov: *mut hwtstamp_provider;
    let mut mii_ts: *mut mii_timestamper;
    let mut phydev: *mut phy_device;
    let mut clone: *mut sk_buff;
    let mut type_: ::core::ffi::c_uint;

    if (*skb).sk.is_null() || (*skb).dev.is_null() {
        return;
    }

    rcu_read_lock();
    hwprov = rcu_dereference((*(*skb).dev).hwprov);
    if !hwprov.is_null() {
        if (*hwprov).source != HWTSTAMP_SOURCE_PHYLIB || (*hwprov).phydev.is_null() {
            rcu_read_unlock();
            return;
        }

        phydev = (*hwprov).phydev;
    } else {
        phydev = (*(*skb).dev).phydev;
        if !phy_is_default_hwtstamp(phydev) {
            rcu_read_unlock();
            return;
        }
    }
    rcu_read_unlock();

    type_ = classify(skb);
    if type_ == PTP_CLASS_NONE {
        return;
    }

    mii_ts = (*phydev).mii_ts;
    if likely(!(*mii_ts).txtstamp.is_none()) {
        clone = skb_clone_sk(skb);
        if clone.is_null() {
            return;
        }
        ((*mii_ts).txtstamp.unwrap())(mii_ts, clone, type_);
    }
}

pub unsafe fn skb_defer_rx_timestamp(skb: *mut sk_buff) -> bool {
    let mut hwprov: *mut hwtstamp_provider;
    let mut mii_ts: *mut mii_timestamper;
    let mut phydev: *mut phy_device;
    let mut type_: ::core::ffi::c_uint;

    if (*skb).dev.is_null() {
        return false;
    }

    rcu_read_lock();
    hwprov = rcu_dereference((*(*skb).dev).hwprov);
    if !hwprov.is_null() {
        if (*hwprov).source != HWTSTAMP_SOURCE_PHYLIB || (*hwprov).phydev.is_null() {
            rcu_read_unlock();
            return false;
        }

        phydev = (*hwprov).phydev;
    } else {
        phydev = (*(*skb).dev).phydev;
        if !phy_is_default_hwtstamp(phydev) {
            rcu_read_unlock();
            return false;
        }
    }
    rcu_read_unlock();

    if skb_headroom(skb) < ETH_HLEN {
        return false;
    }

    __skb_push(skb, ETH_HLEN);

    type_ = ptp_classify_raw(skb);

    __skb_pull(skb, ETH_HLEN);

    if type_ == PTP_CLASS_NONE {
        return false;
    }

    mii_ts = (*phydev).mii_ts;
    if likely(!(*mii_ts).rxtstamp.is_none()) {
        return ((*mii_ts).rxtstamp.unwrap())(mii_ts, skb, type_);
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
