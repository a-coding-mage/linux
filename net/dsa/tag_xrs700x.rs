// SPDX-License-Identifier: GPL-2.0+
/*
 * XRS700x tag format handling
 * Copyright (c) 2008-2009 Marvell Semiconductor
 * Copyright (c) 2020 NovaTech LLC
 */

// Dependencies supplied by the surrounding kernel/DSA sources.

const XRS700X_NAME: *const core::ffi::c_char = c"xrs700x".as_ptr();

unsafe extern "C" {
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u8;
    fn skb_tail_pointer(skb: *mut sk_buff) -> *mut u8;
    fn ffs(x: i32) -> i32;
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_conduit_find_user(
        dev: *mut net_device,
        tree_index: i32,
        port: i32,
    ) -> *mut net_device;
    fn pskb_trim_rcsum(skb: *mut sk_buff, len: usize) -> i32;
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
    pub len: usize,
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const core::ffi::c_char,
    pub proto: i32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_tailroom: usize,
}

const DSA_TAG_PROTO_XRS700X: i32 = 0;

unsafe extern "C" fn xrs700x_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let trailer: *mut u8 = unsafe { skb_put(skb, 1) };
    unsafe {
        *trailer = dsa_xmit_port_mask(skb, dev);
    }

    skb
}

unsafe extern "C" fn xrs700x_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let source_port: i32;
    let trailer: *mut u8;

    trailer = unsafe { skb_tail_pointer(skb).offset(-1) };

    source_port = unsafe { ffs(*trailer as i32) } - 1;

    if source_port < 0 {
        unsafe {
            kfree_skb(skb);
        }
        return core::ptr::null_mut();
    }

    unsafe {
        (*skb).dev = dsa_conduit_find_user(dev, 0, source_port);
    }
    if unsafe { (*skb).dev.is_null() } {
        unsafe {
            kfree_skb(skb);
        }
        return core::ptr::null_mut();
    }

    if unsafe { pskb_trim_rcsum(skb, (*skb).len - 1) } != 0 {
        unsafe {
            kfree_skb(skb);
        }
        return core::ptr::null_mut();
    }

    /* Frame is forwarded by hardware, don't forward in software. */
    unsafe {
        dsa_default_offload_fwd_mark(skb);
    }

    skb
}

static xrs700x_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: XRS700X_NAME,
    proto: DSA_TAG_PROTO_XRS700X,
    xmit: Some(xrs700x_xmit),
    rcv: Some(xrs700x_rcv),
    needed_tailroom: 1,
};

// MODULE_DESCRIPTION("DSA tag driver for XRS700x switches");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_XRS700X, XRS700X_NAME);
// module_dsa_tag_driver(xrs700x_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
