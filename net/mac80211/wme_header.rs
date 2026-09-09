/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2004, Instant802 Networks, Inc.
 * Copyright 2005, Devicescape Software, Inc.
 */

// Dependency intent from <linux/netdevice.h> and "ieee80211_i.h" is
// preserved here; the referenced types are supplied by other translation units.

extern "C" {
    pub fn ieee80211_select_queue_80211(
        sdata: *mut ieee80211_sub_if_data,
        skb: *mut sk_buff,
        hdr: *mut ieee80211_hdr,
    ) -> u16;

    pub fn ieee80211_select_queue(
        sdata: *mut ieee80211_sub_if_data,
        sta: *mut sta_info,
        skb: *mut sk_buff,
    ) -> u16;

    pub fn ieee80211_set_qos_hdr(
        sdata: *mut ieee80211_sub_if_data,
        skb: *mut sk_buff,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
