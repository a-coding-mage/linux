// SPDX-License-Identifier: GPL-2.0
/*
 * Intel / Lantiq GSWIP V2.0 PMAC tag support
 *
 * Copyright (C) 2017 - 2018 Hauke Mehrtens <hauke@hauke-m.de>
 */

// Linux and DSA dependencies supplied by other translation units.

use core::ffi::c_void;

const GSWIP_NAME: &str = "gswip";
const GSWIP_TX_HEADER_LEN: usize = 4;

/* special tag in TX path header */
/* Byte 0 */
const GSWIP_TX_SLPID_SHIFT: u32 = 0; /* source port ID */
const GSWIP_TX_SLPID_CPU: u8 = 2;
const GSWIP_TX_SLPID_APP1: u8 = 3;
const GSWIP_TX_SLPID_APP2: u8 = 4;
const GSWIP_TX_SLPID_APP3: u8 = 5;
const GSWIP_TX_SLPID_APP4: u8 = 6;
const GSWIP_TX_SLPID_APP5: u8 = 7;

/* Byte 1 */
const GSWIP_TX_CRCGEN_DIS: u8 = 1 << 7;
const GSWIP_TX_DPID_SHIFT: u32 = 0; /* destination group ID */
const GSWIP_TX_DPID_ELAN: u8 = 0;
const GSWIP_TX_DPID_EWAN: u8 = 1;
const GSWIP_TX_DPID_CPU: u8 = 2;
const GSWIP_TX_DPID_APP1: u8 = 3;
const GSWIP_TX_DPID_APP2: u8 = 4;
const GSWIP_TX_DPID_APP3: u8 = 5;
const GSWIP_TX_DPID_APP4: u8 = 6;
const GSWIP_TX_DPID_APP5: u8 = 7;

/* Byte 2 */
const GSWIP_TX_PORT_MAP_EN: u8 = 1 << 7;
const GSWIP_TX_PORT_MAP_SEL: u8 = 1 << 6;
const GSWIP_TX_LRN_DIS: u8 = 1 << 5;
const GSWIP_TX_CLASS_EN: u8 = 1 << 4;
const GSWIP_TX_CLASS_SHIFT: u32 = 0;
const GSWIP_TX_CLASS_MASK: u8 = (1 << 4) - 1;

/* Byte 3 */
const GSWIP_TX_DPID_EN: u8 = 1 << 0;
const GSWIP_TX_PORT_MAP: u8 = ((1 << 7) - 1) & !1;

const GSWIP_RX_HEADER_LEN: usize = 8;

/* special tag in RX path header */
/* Byte 7 */
const GSWIP_RX_SPPID_SHIFT: u32 = 4;
const GSWIP_RX_SPPID_MASK: u8 = ((1 << 7) - 1) & !((1 << 4) - 1);

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    fn skb_push(skb: *mut sk_buff, len: usize);
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize);
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u8;
    fn dsa_conduit_find_user(dev: *mut net_device, tree: i32, port: i32) -> *mut net_device;
}

unsafe extern "C" fn gswip_tag_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let gswip_tag: *mut u8;

    skb_push(skb, GSWIP_TX_HEADER_LEN);

    gswip_tag = (*skb).data;
    *gswip_tag.add(0) = GSWIP_TX_SLPID_CPU;
    *gswip_tag.add(1) = GSWIP_TX_DPID_ELAN;
    *gswip_tag.add(2) = GSWIP_TX_PORT_MAP_EN | GSWIP_TX_PORT_MAP_SEL;
    *gswip_tag.add(3) = (dsa_xmit_port_mask(skb, dev) << 1) & GSWIP_TX_PORT_MAP;
    *gswip_tag.add(3) |= GSWIP_TX_DPID_EN;

    skb
}

unsafe extern "C" fn gswip_tag_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let port: i32;
    let gswip_tag: *mut u8;

    if !pskb_may_pull(skb, GSWIP_RX_HEADER_LEN) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    gswip_tag = (*skb).data.sub(14);

    /* Get source port information */
    port = ((*gswip_tag.add(7) & GSWIP_RX_SPPID_MASK) >> GSWIP_RX_SPPID_SHIFT) as i32;
    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* remove GSWIP tag */
    skb_pull_rcsum(skb, GSWIP_RX_HEADER_LEN);

    skb
}

#[repr(C)]
struct dsa_device_ops {
    name: *const u8,
    proto: i32,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    needed_headroom: usize,
}

static GSWIP_NETDEV_OPS: dsa_device_ops = dsa_device_ops {
    name: b"gswip\0".as_ptr(),
    proto: 0,
    xmit: Some(gswip_tag_xmit),
    rcv: Some(gswip_tag_rcv),
    needed_headroom: GSWIP_RX_HEADER_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for Lantiq / Intel GSWIP switches");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_GSWIP, GSWIP_NAME);
// module_dsa_tag_driver(gswip_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
