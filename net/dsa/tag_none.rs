// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/dsa/tag_none.c - Traffic handling for switches with no tag
 * Copyright (c) 2008-2009 Marvell Semiconductor
 * Copyright (c) 2013 Florian Fainelli <florian@openwrt.org>
 *
 * WARNING: do not use this for new switches. In case of no hardware
 * tagging support, look at tag_8021q.c instead.
 */

// Dependency declarations supplied by tag.h are referenced through the crate.

const NONE_NAME: &str = "none";

unsafe fn dsa_user_notag_xmit(
    skb: *mut crate::sk_buff,
    _dev: *mut crate::net_device,
) -> *mut crate::sk_buff {
    /* Just return the original SKB */
    skb
}

static NONE_OPS: crate::dsa_device_ops = crate::dsa_device_ops {
    name: NONE_NAME,
    proto: crate::DSA_TAG_PROTO_NONE,
    xmit: Some(dsa_user_notag_xmit),
};

// module_dsa_tag_driver(none_ops);
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_NONE, NONE_NAME);
// MODULE_DESCRIPTION("DSA no-op tag driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
