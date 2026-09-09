// SPDX-License-Identifier: GPL-2.0
/*
 * Handler for Realtek 4 byte DSA switch tags
 * Currently only supports protocol "A" found in RTL8366RB
 * Copyright (c) 2020 Linus Walleij <linus.walleij@linaro.org>
 *
 * This "proprietary tag" header looks like so:
 *
 * -------------------------------------------------
 * | MAC DA | MAC SA | 0x8899 | 2 bytes tag | Type |
 * -------------------------------------------------
 *
 * The 2 bytes tag form a 16 bit big endian word. The exact
 * meaning has been guessed from packet dumps from ingress
 * frames.
 */

// C dependencies supplied by the surrounding kernel/DSA implementation.

use core::ffi::c_void;

pub const RTL4_A_NAME: &[u8] = b"rtl4a\0";
pub const RTL4_A_HDR_LEN: usize = 4;
pub const RTL4_A_PROTOCOL_SHIFT: u32 = 12;
/*
 * 0x1 = Realtek Remote Control protocol (RRCP)
 * 0x2/0x3 seems to be used for loopback testing
 * 0x9 = RTL8306 DSA protocol
 * 0xa = RTL8366RB DSA protocol
 */
pub const RTL4_A_PROTOCOL_RTL8366RB: u16 = 0xa;

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct dsa_port {
    pub index: i32,
}
#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const u8,
    pub proto: i32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_headroom: usize,
}

extern "C" {
    fn dsa_user_to_port(dev: *mut net_device) -> *mut dsa_port;
    fn eth_skb_pad(skb: *mut sk_buff) -> i32;
    fn netdev_dbg(dev: *mut net_device, fmt: *const u8, ...);
    fn netdev_err(dev: *mut net_device, fmt: *const u8, ...);
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut u8;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u16;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> i32;
    fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut u8;
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_conduit_find_user(dev: *mut net_device, tree: i32, port: u8) -> *mut net_device;
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_strip_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
    fn htons(value: u16) -> u16;
    fn ntohs(value: u16) -> u16;
}

pub const ETH_P_REALTEK: u16 = 0x8899;
pub const DSA_TAG_PROTO_RTL4_A: i32 = 0;

unsafe extern "C" fn rtl4a_tag_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    let mut out: u16;

    /* Pad out to at least 60 bytes */
    if eth_skb_pad(skb) != 0 {
        return core::ptr::null_mut();
    }

    netdev_dbg(dev, b"add realtek tag to package to port %d\n\0".as_ptr(), (*dp).index);
    skb_push(skb, RTL4_A_HDR_LEN);

    dsa_alloc_etype_header(skb, RTL4_A_HDR_LEN);
    let tag = dsa_etype_header_pos_tx(skb);

    /* Set Ethertype */
    *(tag as *mut u16) = htons(ETH_P_REALTEK);

    out = RTL4_A_PROTOCOL_RTL8366RB << RTL4_A_PROTOCOL_SHIFT;
    /* The lower bits indicate the port number */
    out |= dsa_xmit_port_mask(skb, dev);

    *((tag.add(2)) as *mut u16) = htons(out);

    skb
}

unsafe extern "C" fn rtl4a_tag_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let mut protport: u16;

    if pskb_may_pull(skb, RTL4_A_HDR_LEN) == 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let tag = dsa_etype_header_pos_rx(skb);
    let mut p = tag as *mut u16;
    let etype = ntohs(*p);
    if etype != ETH_P_REALTEK {
        /* Not custom, just pass through */
        netdev_dbg(dev, b"non-realtek ethertype 0x%04x\n\0".as_ptr(), etype);
        return skb;
    }
    p = tag.add(2) as *mut u16;
    protport = ntohs(*p);
    /* The 4 upper bits are the protocol */
    let prot = ((protport >> RTL4_A_PROTOCOL_SHIFT) & 0x0f) as u8;
    if prot as u16 != RTL4_A_PROTOCOL_RTL8366RB {
        netdev_err(dev, b"unknown realtek protocol 0x%01x\n\0".as_ptr(), prot);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    let port = (protport & 0xff) as u8;

    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        netdev_dbg(dev, b"could not find user for port %d\n\0".as_ptr(), port);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Remove RTL4 tag and recalculate checksum */
    skb_pull_rcsum(skb, RTL4_A_HDR_LEN);

    dsa_strip_etype_header(skb, RTL4_A_HDR_LEN);

    dsa_default_offload_fwd_mark(skb);

    skb
}

pub static RTL4A_NETDEV_OPS: dsa_device_ops = dsa_device_ops {
    name: RTL4_A_NAME.as_ptr(),
    proto: DSA_TAG_PROTO_RTL4_A,
    xmit: Some(rtl4a_tag_xmit),
    rcv: Some(rtl4a_tag_rcv),
    needed_headroom: RTL4_A_HDR_LEN,
};

// module_dsa_tag_driver(rtl4a_netdev_ops);
// MODULE_DESCRIPTION("DSA tag driver for Realtek 4 byte protocol A tags");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_RTL4_A, RTL4_A_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
