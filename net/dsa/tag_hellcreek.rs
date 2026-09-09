// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/*
 * net/dsa/tag_hellcreek.c - Hirschmann Hellcreek switch tag format handling
 *
 * Copyright (C) 2019,2020 Linutronix GmbH
 * Author Kurt Kanzenbach <kurt@linutronix.de>
 *
 * Based on tag_ksz.c.
 */

// C dependencies: <linux/skbuff.h>, <net/dsa.h>, and "tag.h".

pub const HELLCREEK_NAME: &[u8] = b"hellcreek\0";
pub const HELLCREEK_TAG_LEN: usize = 1;

#[repr(C)]
pub struct sk_buff {
    pub ip_summed: u32,
    pub dev: *mut net_device,
    pub len: usize,
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const u8,
    pub proto: u32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_tailroom: usize,
}

pub const CHECKSUM_PARTIAL: u32 = 3;
pub const DSA_TAG_PROTO_HELLCREEK: u32 = 0;

extern "C" {
    fn skb_checksum_help(skb: *mut sk_buff) -> i32;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u8;
    fn skb_tail_pointer(skb: *mut sk_buff) -> *mut u8;
    fn dsa_conduit_find_user(dev: *mut net_device, tree: i32, port: u32) -> *mut net_device;
    fn netdev_warn_once(dev: *mut net_device, fmt: *const u8, port: u32);
    fn pskb_trim_rcsum(skb: *mut sk_buff, len: usize) -> i32;
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
}

unsafe extern "C" fn hellcreek_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let tag: *mut u8;

    /* Calculate checksums (if required) before adding the trailer tag to
     * avoid including it in calculations. That would lead to wrong
     * checksums after the switch strips the tag.
     */
    if (*skb).ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Tag encoding */
    tag = skb_put(skb, HELLCREEK_TAG_LEN);
    *tag = dsa_xmit_port_mask(skb, dev);

    skb
}

unsafe extern "C" fn hellcreek_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    /* Tag decoding */
    let tag: *mut u8 = skb_tail_pointer(skb).sub(HELLCREEK_TAG_LEN);
    let port: u32 = (*tag & 0x03) as u32;

    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        netdev_warn_once(dev, b"Failed to get source port: %d\n\0".as_ptr(), port);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    if pskb_trim_rcsum(skb, (*skb).len - HELLCREEK_TAG_LEN) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    dsa_default_offload_fwd_mark(skb);

    skb
}

#[no_mangle]
pub static hellcreek_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: HELLCREEK_NAME.as_ptr(),
    proto: DSA_TAG_PROTO_HELLCREEK,
    xmit: Some(hellcreek_xmit),
    rcv: Some(hellcreek_rcv),
    needed_tailroom: HELLCREEK_TAG_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for Hirschmann Hellcreek TSN switches");
// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_HELLCREEK, HELLCREEK_NAME);
// module_dsa_tag_driver(hellcreek_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
