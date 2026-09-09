/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic HDLC support routines for Linux
 *
 * Copyright (C) 1999-2005 Krzysztof Halasa <khc@pm.waw.pl>
 */

// C dependencies supplied by the surrounding kernel translation:
// linux/skbuff.h, linux/netdevice.h, linux/hdlc/ioctl.h, uapi/linux/hdlc.h

#[repr(C)]
pub struct hdlc_proto {
    pub open: Option<unsafe extern "C" fn(dev: *mut net_device) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub start: Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub stop: Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub detach: Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub ioctl: Option<unsafe extern "C" fn(dev: *mut net_device, ifs: *mut if_settings) -> ::core::ffi::c_int>,
    pub type_trans: Option<unsafe extern "C" fn(skb: *mut sk_buff, dev: *mut net_device) -> __be16>,
    pub netif_rx: Option<unsafe extern "C" fn(skb: *mut sk_buff) -> ::core::ffi::c_int>,
    pub xmit: Option<unsafe extern "C" fn(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t>,
    pub module: *mut module,
    pub next: *mut hdlc_proto,
}

#[repr(C)]
pub struct hdlc_device {
    pub attach: Option<unsafe extern "C" fn(
        dev: *mut net_device,
        encoding: ::core::ffi::c_ushort,
        parity: ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int>,
    pub xmit: Option<unsafe extern "C" fn(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t>,
    pub proto: *const hdlc_proto,
    pub carrier: ::core::ffi::c_int,
    pub open: ::core::ffi::c_int,
    pub state_lock: spinlock_t,
    pub state: *mut ::core::ffi::c_void,
    pub priv_: *mut ::core::ffi::c_void,
}

extern "C" {
    pub fn hdlc_ioctl(dev: *mut net_device, ifs: *mut if_settings) -> ::core::ffi::c_int;
    pub fn register_netdev(dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn unregister_hdlc_device(dev: *mut net_device);
    pub fn register_hdlc_protocol(proto: *mut hdlc_proto);
    pub fn unregister_hdlc_protocol(proto: *mut hdlc_proto);
    pub fn alloc_hdlcdev(priv_: *mut ::core::ffi::c_void) -> *mut net_device;
    pub fn hdlc_open(dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn hdlc_close(dev: *mut net_device);
    pub fn hdlc_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
    pub fn attach_hdlc_protocol(
        dev: *mut net_device,
        proto: *mut hdlc_proto,
        size: usize,
    ) -> ::core::ffi::c_int;
    pub fn detach_hdlc_protocol(dev: *mut net_device) -> ::core::ffi::c_int;
    pub fn netdev_priv(dev: *mut net_device) -> *mut ::core::ffi::c_void;
    pub fn skb_reset_mac_header(skb: *mut sk_buff);
    pub fn printk(fmt: *const ::core::ffi::c_char, ...);
    pub fn htons(value: u16) -> __be16;
}

#[macro_export]
macro_rules! register_hdlc_device {
    ($dev:expr) => {
        unsafe { register_netdev($dev) }
    };
}

#[inline]
pub unsafe fn dev_to_hdlc(dev: *mut net_device) -> *mut hdlc_device {
    netdev_priv(dev) as *mut hdlc_device
}

#[inline]
pub unsafe fn debug_frame(skb: *const sk_buff) {
    let mut i: usize = 0;
    while i < (*skb).len as usize {
        if i == 100 {
            printk(b"...\n\0".as_ptr() as *const ::core::ffi::c_char);
            return;
        }
        printk(b" %02X\0".as_ptr() as *const ::core::ffi::c_char, (*skb).data.add(i).read());
        i += 1;
    }
    printk(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}

#[inline]
pub unsafe fn hdlc_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> __be16 {
    let hdlc: *mut hdlc_device = dev_to_hdlc(dev);
    (*skb).dev = dev;
    skb_reset_mac_header(skb);
    if let Some(type_trans) = (*(*hdlc).proto).type_trans {
        type_trans(skb, dev)
    } else {
        htons(ETH_P_HDLC)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
