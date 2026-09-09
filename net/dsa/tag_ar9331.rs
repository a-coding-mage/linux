// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
 */

// Dependency intent from <linux/bitfield.h>, <linux/etherdevice.h>, and "tag.h".

const AR9331_NAME: &str = "ar9331";

const AR9331_HDR_LEN: usize = 2;
const AR9331_HDR_VERSION: u16 = 1;

const AR9331_HDR_VERSION_MASK: u16 = 0xc000;
const AR9331_HDR_PRIORITY_MASK: u16 = 0x3000;
const AR9331_HDR_TYPE_MASK: u16 = 0x0700;
const AR9331_HDR_BROADCAST: u16 = 1 << 7;
const AR9331_HDR_FROM_CPU: u16 = 1 << 6;
/* AR9331_HDR_RESERVED - not used or may be version field.
 * According to the AR8216 doc it should 0b10. On AR9331 it is 0b11 on RX path
 * and should be set to 0b11 to make it work.
 */
const AR9331_HDR_RESERVED_MASK: u16 = 0x0030;
const AR9331_HDR_PORT_NUM_MASK: u16 = 0x000f;

unsafe fn ar9331_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let dp: *mut dsa_port = dsa_user_to_port(dev);
    let phdr: *mut __le16;
    let mut hdr: u16;

    phdr = skb_push(skb, AR9331_HDR_LEN);

    hdr = ((AR9331_HDR_VERSION as u16) << 14) & AR9331_HDR_VERSION_MASK;
    hdr |= AR9331_HDR_FROM_CPU | (*dp).index as u16;
    /* 0b10 for AR8216 and 0b11 for AR9331 */
    hdr |= AR9331_HDR_RESERVED_MASK;

    *phdr = cpu_to_le16(hdr);

    skb
}

unsafe fn ar9331_tag_rcv(skb: *mut sk_buff, ndev: *mut net_device) -> *mut sk_buff {
    let mut ver: u8;
    let mut port: u8;
    let hdr: u16;

    if !pskb_may_pull(skb, AR9331_HDR_LEN) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    hdr = le16_to_cpu(*(skb_mac_header(skb) as *const __le16));

    ver = ((hdr & AR9331_HDR_VERSION_MASK) >> 14) as u8;
    if ver != AR9331_HDR_VERSION as u8 {
        netdev_warn_once(ndev, "%s:%i wrong header version 0x%2x\n", __func__, __LINE__, hdr);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    if hdr & AR9331_HDR_FROM_CPU != 0 {
        netdev_warn_once(ndev, "%s:%i packet should not be from cpu 0x%2x\n", __func__, __LINE__, hdr);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    skb_pull_rcsum(skb, AR9331_HDR_LEN);

    /* Get source port information */
    port = (hdr & AR9331_HDR_PORT_NUM_MASK) as u8;

    (*skb).dev = dsa_conduit_find_user(ndev, 0, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    skb
}

static ar9331_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: AR9331_NAME,
    proto: DSA_TAG_PROTO_AR9331,
    xmit: ar9331_tag_xmit,
    rcv: ar9331_tag_rcv,
    needed_headroom: AR9331_HDR_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for Atheros AR9331 SoC with built-in switch");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_AR9331, AR9331_NAME);
// module_dsa_tag_driver(ar9331_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
