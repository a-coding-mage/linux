// SPDX-License-Identifier: GPL-2.0
/*
 * Handler for Realtek 8 byte switch tags
 *
 * Copyright (C) 2021 Alvin Šipraga <alsi@bang-olufsen.dk>
 *
 * NOTE: Currently only supports protocol "4" found in the RTL8365MB, hence
 * named tag_rtl8_4.
 */

// C dependencies supplied by the surrounding kernel/DSA code are intentionally
// referenced here as external symbols.

use core::ffi::c_void;

type __be16 = u16;
type u8_ = u8;
type u16_ = u16;

#[repr(C)]
pub struct sk_buff {
    pub ip_summed: u32,
    pub dev: *mut net_device,
    pub len: usize,
}

#[repr(C)]
pub struct net_device {
    pub dev: device,
}

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const u8,
    pub proto: u32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_headroom: usize,
    pub needed_tailroom: usize,
}

#[repr(C)]
pub struct dsa_tag_driver;

extern "C" {
    fn htons(value: u16) -> __be16;
    fn ntohs(value: __be16) -> u16;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u16;
    fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut c_void;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_checksum_help(skb: *mut sk_buff) -> bool;
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut c_void;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize);
    fn dsa_strip_etype_header(skb: *mut sk_buff, len: usize);
    fn skb_linearize(skb: *mut sk_buff) -> i32;
    fn skb_tail_pointer(skb: *mut sk_buff) -> *mut u8;
    fn pskb_trim_rcsum(skb: *mut sk_buff, len: usize) -> i32;
    fn dsa_conduit_find_user(dev: *mut net_device, tree: i32, port: u8) -> *mut net_device;
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
    fn dev_warn_ratelimited(dev: *mut device, format: *const u8, ...);
}

const RTL8_4_NAME: &[u8] = b"rtl8_4\0";
const RTL8_4T_NAME: &[u8] = b"rtl8_4t\0";
const RTL8_4_TAG_LEN: usize = 8;
const RTL8_4_PROTOCOL: u16 = 0xff00;
const RTL8_4_PROTOCOL_RTL8365MB: u16 = 0x04;
const RTL8_4_REASON: u16 = 0x00ff;
const RTL8_4_REASON_TRAP: u8 = 80;
const RTL8_4_LEARN_DIS: u16 = 1 << 5;
const RTL8_4_KEEP: u16 = 1 << 7;
const RTL8_4_TX: u16 = 0x000f;
const RTL8_4_RX: u16 = 0x07ff;
const ETH_P_REALTEK: u16 = 0x8899;
const CHECKSUM_PARTIAL: u32 = 3;
const DSA_TAG_PROTO_RTL8_4: u32 = 0;
const DSA_TAG_PROTO_RTL8_4T: u32 = 1;

#[inline]
unsafe fn field_prep(mask: u16, value: u16) -> u16 {
    (value << mask.trailing_zeros()) & mask
}

#[inline]
unsafe fn field_get(mask: u16, value: u16) -> u16 {
    (value & mask) >> mask.trailing_zeros()
}

unsafe fn rtl8_4_write_tag(skb: *mut sk_buff, dev: *mut net_device, tag: *mut c_void) {
    let mut tag16: [__be16; RTL8_4_TAG_LEN / 2] = [0; RTL8_4_TAG_LEN / 2];
    tag16[0] = htons(ETH_P_REALTEK);
    tag16[1] = htons(field_prep(RTL8_4_PROTOCOL, RTL8_4_PROTOCOL_RTL8365MB));
    tag16[2] = htons(field_prep(RTL8_4_LEARN_DIS, 1) | field_prep(RTL8_4_KEEP, 1));
    tag16[3] = htons(field_prep(RTL8_4_RX, dsa_xmit_port_mask(skb, dev)));
    core::ptr::copy_nonoverlapping(tag16.as_ptr() as *const u8, tag as *mut u8, RTL8_4_TAG_LEN);
}

unsafe extern "C" fn rtl8_4_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    skb_push(skb, RTL8_4_TAG_LEN);
    dsa_alloc_etype_header(skb, RTL8_4_TAG_LEN);
    rtl8_4_write_tag(skb, dev, dsa_etype_header_pos_tx(skb));
    skb
}

unsafe extern "C" fn rtl8_4t_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if (*skb).ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    rtl8_4_write_tag(skb, dev, skb_put(skb, RTL8_4_TAG_LEN));
    skb
}

unsafe fn rtl8_4_read_tag(skb: *mut sk_buff, dev: *mut net_device, tag: *mut c_void) -> i32 {
    let mut tag16: [__be16; RTL8_4_TAG_LEN / 2] = [0; RTL8_4_TAG_LEN / 2];
    core::ptr::copy_nonoverlapping(tag as *const u8, tag16.as_mut_ptr() as *mut u8, RTL8_4_TAG_LEN);
    let etype = ntohs(tag16[0]);
    if etype != ETH_P_REALTEK {
        dev_warn_ratelimited(&mut (*dev).dev, b"non-realtek ethertype 0x%04x\0".as_ptr(), etype);
        return -71;
    }
    let proto = field_get(RTL8_4_PROTOCOL, ntohs(tag16[1])) as u8;
    if proto != RTL8_4_PROTOCOL_RTL8365MB as u8 {
        dev_warn_ratelimited(&mut (*dev).dev, b"unknown realtek protocol 0x%02x\0".as_ptr(), proto);
        return -71;
    }
    let reason = field_get(RTL8_4_REASON, ntohs(tag16[1])) as u8;
    let port = field_get(RTL8_4_TX, ntohs(tag16[3])) as u8;
    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        dev_warn_ratelimited(&mut (*dev).dev, b"could not find user for port %d\0".as_ptr(), port);
        return -2;
    }
    if reason != RTL8_4_REASON_TRAP { dsa_default_offload_fwd_mark(skb); }
    0
}

unsafe extern "C" fn rtl8_4_tag_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if !pskb_may_pull(skb, RTL8_4_TAG_LEN) { kfree_skb(skb); return core::ptr::null_mut(); }
    if rtl8_4_read_tag(skb, dev, dsa_etype_header_pos_rx(skb)) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    skb_pull_rcsum(skb, RTL8_4_TAG_LEN);
    dsa_strip_etype_header(skb, RTL8_4_TAG_LEN);
    skb
}

unsafe extern "C" fn rtl8_4t_tag_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if skb_linearize(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    let tag = skb_tail_pointer(skb).sub(RTL8_4_TAG_LEN) as *mut c_void;
    if rtl8_4_read_tag(skb, dev, tag) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    if pskb_trim_rcsum(skb, (*skb).len - RTL8_4_TAG_LEN) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    skb
}

#[no_mangle]
pub static rtl8_4_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: RTL8_4_NAME.as_ptr(), proto: DSA_TAG_PROTO_RTL8_4,
    xmit: Some(rtl8_4_tag_xmit), rcv: Some(rtl8_4_tag_rcv),
    needed_headroom: RTL8_4_TAG_LEN, needed_tailroom: 0,
};

#[no_mangle]
pub static rtl8_4t_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: RTL8_4T_NAME.as_ptr(), proto: DSA_TAG_PROTO_RTL8_4T,
    xmit: Some(rtl8_4t_tag_xmit), rcv: Some(rtl8_4t_tag_rcv),
    needed_headroom: 0, needed_tailroom: RTL8_4_TAG_LEN,
};

// DSA_TAG_DRIVER and MODULE_ALIAS_DSA_TAG_DRIVER registrations are supplied by
// the kernel build integration and have no direct file-local Rust equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
