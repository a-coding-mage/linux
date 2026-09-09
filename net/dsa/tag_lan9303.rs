// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Pengutronix, Juergen Borleis <jbe@pengutronix.de>
 */
// External kernel and DSA declarations are supplied by the surrounding build.

use core::ffi::c_char;

const LAN9303_NAME: *const c_char = b"lan9303\0".as_ptr() as *const c_char;
const LAN9303_TAG_LEN: usize = 4;
const LAN9303_TAG_TX_USE_ALR: u16 = 1 << 3;
const LAN9303_TAG_TX_STP_OVERRIDE: u16 = 1 << 4;
const LAN9303_TAG_RX_IGMP: u16 = 1 << 3;
const LAN9303_TAG_RX_STP: u16 = 1 << 4;
const LAN9303_TAG_RX_TRAPPED_TO_CPU: u16 = LAN9303_TAG_RX_IGMP | LAN9303_TAG_RX_STP;

#[repr(C)]
pub struct dsa_port {
    pub ds: *mut dsa_switch,
    pub index: u32,
}
#[repr(C)]
pub struct dsa_switch {
    pub priv_: *mut lan9303,
}
#[repr(C)]
pub struct lan9303 {
    pub is_bridged: bool,
}
#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
    pub dev: *mut net_device,
}
#[repr(C)]
pub struct net_device {
    pub dev: *mut device,
}
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const c_char,
    pub proto: u32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_headroom: usize,
}

extern "C" {
    fn dsa_user_to_port(dev: *mut net_device) -> *mut dsa_port;
    fn is_multicast_ether_addr(addr: *const u8) -> bool;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut u16;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_vlan_tag_present(skb: *mut sk_buff) -> bool;
    fn skb_vlan_tag_get(skb: *mut sk_buff) -> u16;
    fn __vlan_hwaccel_clear_tag(skb: *mut sk_buff);
    fn skb_push_rcsum(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn __skb_vlan_pop(skb: *mut sk_buff, tag: *mut u16) -> i32;
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_conduit_find_user(dev: *mut net_device, tree: u32, port: u32) -> *mut net_device;
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
}

unsafe fn lan9303_xmit_use_arl(dp: *mut dsa_port, dest_addr: *const u8) -> bool {
    let chip = (*(*dp).ds).priv_;
    (*chip).is_bridged && !is_multicast_ether_addr(dest_addr)
}

unsafe extern "C" fn lan9303_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    skb_push(skb, LAN9303_TAG_LEN);
    dsa_alloc_etype_header(skb, LAN9303_TAG_LEN);
    let lan9303_tag = dsa_etype_header_pos_tx(skb);
    let tag = if lan9303_xmit_use_arl(dp, (*skb).data) {
        LAN9303_TAG_TX_USE_ALR
    } else {
        (*dp).index as u16 | LAN9303_TAG_TX_STP_OVERRIDE
    };
    *lan9303_tag = u16::from_be(0x8100u16).to_be();
    *lan9303_tag.add(1) = tag.to_be();
    skb
}

unsafe extern "C" fn lan9303_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let mut lan9303_tag1: u16 = 0;
    if !pskb_may_pull(skb, LAN9303_TAG_LEN) {
        dev_warn_ratelimited((*dev).dev, b"Dropping packet, cannot pull\n\0".as_ptr() as *const c_char);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    if skb_vlan_tag_present(skb) {
        lan9303_tag1 = skb_vlan_tag_get(skb);
        __vlan_hwaccel_clear_tag(skb);
    } else {
        skb_push_rcsum(skb, 14);
        __skb_vlan_pop(skb, &mut lan9303_tag1);
        skb_pull_rcsum(skb, 14);
    }
    let source_port = (lan9303_tag1 & 0x3) as u32;
    (*skb).dev = dsa_conduit_find_user(dev, 0, source_port);
    if (*skb).dev.is_null() {
        dev_warn_ratelimited((*dev).dev, b"Dropping packet due to invalid source port\n\0".as_ptr() as *const c_char);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    if lan9303_tag1 & LAN9303_TAG_RX_TRAPPED_TO_CPU == 0 {
        dsa_default_offload_fwd_mark(skb);
    }
    skb
}

static lan9303_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: LAN9303_NAME,
    proto: 0,
    xmit: Some(lan9303_xmit),
    rcv: Some(lan9303_rcv),
    needed_headroom: LAN9303_TAG_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for SMSC/Microchip LAN9303 family of switches");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_LAN9303, LAN9303_NAME);
// module_dsa_tag_driver(lan9303_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
