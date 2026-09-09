// SPDX-License-Identifier: GPL-2.0+
/*
 * net/dsa/tag_trailer.c - Trailer tag format handling
 * Copyright (c) 2008-2009 Marvell Semiconductor
 */

// Dependencies supplied by the surrounding kernel/DSA implementation.

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
    pub len: usize,
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u8;
    fn skb_linearize(skb: *mut sk_buff) -> i32;
    fn skb_tail_pointer(skb: *mut sk_buff) -> *mut u8;
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_conduit_find_user(
        dev: *mut net_device,
        tree: i32,
        port: i32,
    ) -> *mut net_device;
    fn pskb_trim_rcsum(skb: *mut sk_buff, len: usize) -> i32;
}

extern "C" {
    static DSA_TAG_PROTO_TRAILER: i32;
}

pub type DsaXmit = unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff;
pub type DsaRcv = unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff;

#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const u8,
    pub proto: i32,
    pub xmit: Option<DsaXmit>,
    pub rcv: Option<DsaRcv>,
    pub needed_tailroom: usize,
}

pub const TRAILER_NAME: &[u8] = b"trailer\0";

unsafe extern "C" fn trailer_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let trailer: *mut u8 = skb_put(skb, 4);

    *trailer.add(0) = 0x80;
    *trailer.add(1) = dsa_xmit_port_mask(skb, dev);
    *trailer.add(2) = 0x10;
    *trailer.add(3) = 0x00;

    skb
}

unsafe extern "C" fn trailer_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let trailer: *mut u8;
    let source_port: i32;

    if skb_linearize(skb) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    trailer = skb_tail_pointer(skb).sub(4);
    if *trailer.add(0) != 0x80
        || (*trailer.add(1) & 0xf8) != 0x00
        || (*trailer.add(2) & 0xef) != 0x00
        || *trailer.add(3) != 0x00
    {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    source_port = (*trailer.add(1) & 7) as i32;

    (*skb).dev = dsa_conduit_find_user(dev, 0, source_port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    if pskb_trim_rcsum(skb, (*skb).len - 4) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    skb
}

#[no_mangle]
pub static trailer_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: TRAILER_NAME.as_ptr(),
    proto: unsafe { DSA_TAG_PROTO_TRAILER },
    xmit: Some(trailer_xmit),
    rcv: Some(trailer_rcv),
    needed_tailroom: 4,
};

// MODULE_DESCRIPTION("DSA tag driver for switches using a trailer tag");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_TRAILER, TRAILER_NAME);
// module_dsa_tag_driver(trailer_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
