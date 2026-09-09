// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 Schneider Electric
 *
 * Clément Léger <clement.leger@bootlin.com>
 */

// Translated from the Linux DSA tag driver. Kernel declarations referenced by
// this file are supplied by the surrounding Rust environment.

use core::ffi::c_void;

pub const A5PSW_NAME: &str = "a5psw";
pub const ETH_P_DSA_A5PSW: u16 = 0xE001;
pub const A5PSW_TAG_LEN: usize = 8;
pub const A5PSW_CTRL_DATA_FORCE_FORWARD: u16 = 1 << 0;
pub const A5PSW_CTRL_DATA_PORT: u16 = 0x000f;

#[repr(C)]
pub struct a5psw_tag {
    pub ctrl_tag: u16,
    pub ctrl_data: u16,
    pub ctrl_data2_hi: u16,
    pub ctrl_data2_lo: u16,
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
    fn eth_skb_pad(skb: *mut sk_buff) -> i32;
    fn skb_push(skb: *mut sk_buff, len: usize);
    fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut a5psw_tag;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u32;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut a5psw_tag;
    fn dsa_conduit_find_user(dev: *mut net_device, index: i32, port: i32) -> *mut net_device;
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize);
    fn dsa_strip_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
    fn kfree_skb(skb: *mut sk_buff);
    fn dev_warn_ratelimited(dev: *mut device, message: *const u8);
    fn htons(value: u16) -> u16;
    fn ntohs(value: u16) -> u16;
}

// DSA_TAG_PROTO_RZN1_A5PSW is provided by the DSA subsystem.
extern "C" {
    static DSA_TAG_PROTO_RZN1_A5PSW: i32;
}

unsafe extern "C" fn a5psw_tag_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let ptag: *mut a5psw_tag;
    let data2_val: u32;

    // BUILD_BUG_ON(sizeof(*ptag) != A5PSW_TAG_LEN);

    /*
     * The Ethernet switch we are interfaced with needs packets to be at
     * least 60 bytes otherwise they will be discarded when they enter the
     * switch port logic.
     */
    if eth_skb_pad(skb) != 0 {
        return core::ptr::null_mut();
    }

    /* provide 'A5PSW_TAG_LEN' bytes additional space */
    skb_push(skb, A5PSW_TAG_LEN);

    /* make room between MACs and Ether-Type to insert tag */
    dsa_alloc_etype_header(skb, A5PSW_TAG_LEN);

    ptag = dsa_etype_header_pos_tx(skb);

    data2_val = ((A5PSW_CTRL_DATA_PORT as u32)
        & dsa_xmit_port_mask(skb, dev));
    (*ptag).ctrl_tag = htons(ETH_P_DSA_A5PSW);
    (*ptag).ctrl_data = htons(A5PSW_CTRL_DATA_FORCE_FORWARD);
    (*ptag).ctrl_data2_lo = htons(data2_val as u16);
    (*ptag).ctrl_data2_hi = 0;

    skb
}

unsafe extern "C" fn a5psw_tag_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let tag: *mut a5psw_tag;
    let port: i32;

    if !pskb_may_pull(skb, A5PSW_TAG_LEN) {
        dev_warn_ratelimited(
            &mut (*dev).dev,
            b"Dropping packet, cannot pull\0".as_ptr(),
        );
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    tag = dsa_etype_header_pos_rx(skb);

    if (*tag).ctrl_tag != htons(ETH_P_DSA_A5PSW) {
        dev_warn_ratelimited(
            &mut (*dev).dev,
            b"Dropping packet due to invalid TAG marker\0".as_ptr(),
        );
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    port = ((ntohs((*tag).ctrl_data) & A5PSW_CTRL_DATA_PORT) as i32);

    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    skb_pull_rcsum(skb, A5PSW_TAG_LEN);
    dsa_strip_etype_header(skb, A5PSW_TAG_LEN);

    dsa_default_offload_fwd_mark(skb);

    skb
}

#[no_mangle]
pub static a5psw_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: b"a5psw\0".as_ptr(),
    proto: unsafe { DSA_TAG_PROTO_RZN1_A5PSW },
    xmit: Some(a5psw_tag_xmit),
    rcv: Some(a5psw_tag_rcv),
    needed_headroom: A5PSW_TAG_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for Renesas RZ/N1 A5PSW switch");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_A5PSW, A5PSW_NAME);
// module_dsa_tag_driver(a5psw_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
