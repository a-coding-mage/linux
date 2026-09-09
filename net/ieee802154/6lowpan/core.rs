/* Copyright 2011, Siemens AG
 * written by Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

/* Based on patches from Jon Smirl <jonsmirl@gmail.com>
 * Copyright (c) 2011 Jon Smirl <jonsmirl@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation.
 */

/* The remaining license text and the original dependency context are retained
 * in the C source. Linux kernel includes are represented by external symbols
 * supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

static mut open_count: i32 = 0;

extern "C" {
    static lowpan_netdev_ops: net_device_ops;
    static lowpan_header_ops: header_ops;
}

// External declarations supplied by the Linux/6LoWPAN environment.
extern "C" {
    fn lowpan_header_create(dev: *mut net_device, skb: *mut sk_buff,
                            protocol: u16, daddr: *const c_void,
                            saddr: *const c_void, len: usize) -> i32;
    fn netdev_lockdep_set_classes(dev: *mut net_device);
    fn lowpan_rx_init();
    fn lowpan_rx_exit();
    fn lowpan_xmit(skb: *mut sk_buff, dev: *mut net_device) -> i32;
    fn lowpan_802154_neigh(priv_: *mut c_void) -> *mut lowpan_802154_neigh;
    fn lowpan_802154_dev(dev: *const net_device) -> *mut lowpan_802154_dev;
    fn lowpan_register_netdevice(dev: *mut net_device, typ: i32) -> i32;
    fn lowpan_unregister_netdevice(dev: *mut net_device);
    fn dev_get_by_index(net: *mut c_void, index: u32) -> *mut net_device;
    fn dev_put(dev: *mut net_device);
    fn dev_net(dev: *const net_device) -> *mut c_void;
    fn net_eq(a: *mut c_void, b: *mut c_void) -> bool;
    fn nla_len(attr: *const nlattr) -> usize;
    fn nla_get_u32(attr: *const nlattr) -> u32;
    fn __dev_addr_set(dev: *mut net_device, addr: *const u8, len: usize);
    fn lowpan_net_frag_init() -> i32;
    fn lowpan_net_frag_exit();
    fn rtnl_link_register(ops: *mut rtnl_link_ops) -> i32;
    fn rtnl_link_unregister(ops: *mut rtnl_link_ops);
    fn register_netdevice_notifier(nb: *mut notifier_block) -> i32;
    fn unregister_netdevice_notifier(nb: *mut notifier_block);
    fn netdev_notifier_info_to_dev(ptr: *mut c_void) -> *mut net_device;
    fn ASSERT_RTNL();
}

#[repr(C)] pub struct net_device { pub broadcast: [u8; 32], pub hard_header_len: u16, pub flags: u32, pub priv_flags: u32, pub netdev_ops: *const net_device_ops, pub header_ops: *const header_ops, pub needs_free_netdev: bool, pub netns_immutable: bool, pub needed_headroom: usize, pub needed_tailroom: usize, pub neigh_priv_len: usize, pub type_: u16, pub dev_addr: *mut u8, pub ieee802154_ptr: *mut wpan_dev }
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct neighbour { pub priv_: *mut c_void }
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct netlink_ext_ack;
#[repr(C)] pub struct rtnl_newlink_params { pub tb: *mut *mut nlattr, pub link_net: *mut c_void }
#[repr(C)] pub struct lowpan_802154_neigh { pub short_addr: u16 }
#[repr(C)] pub struct lowpan_802154_dev { pub wdev: *mut net_device }
#[repr(C)] pub struct wpan_dev { pub lowpan_dev: *mut net_device }
#[repr(C)] pub struct header_ops { pub create: Option<unsafe extern "C" fn(*mut net_device, *mut sk_buff, u16, *const c_void, *const c_void, usize) -> i32> }
#[repr(C)] pub struct net_device_ops { pub ndo_init: Option<unsafe extern "C" fn(*mut net_device) -> i32>, pub ndo_start_xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> i32>, pub ndo_open: Option<unsafe extern "C" fn(*mut net_device) -> i32>, pub ndo_stop: Option<unsafe extern "C" fn(*mut net_device) -> i32>, pub ndo_neigh_construct: Option<unsafe extern "C" fn(*mut net_device, *mut neighbour) -> i32>, pub ndo_get_iflink: Option<unsafe extern "C" fn(*const net_device) -> i32> }
#[repr(C)] pub struct rtnl_link_ops { pub kind: *const u8, pub priv_size: usize, pub setup: Option<unsafe extern "C" fn(*mut net_device)>, pub newlink: Option<unsafe extern "C" fn(*mut net_device, *mut rtnl_newlink_params, *mut netlink_ext_ack) -> i32>, pub dellink: Option<unsafe extern "C" fn(*mut net_device, *mut list_head)>, pub validate: Option<unsafe extern "C" fn(*mut *mut nlattr, *mut *mut nlattr, *mut netlink_ext_ack) -> i32> }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32> }

unsafe extern "C" fn lowpan_dev_init(ldev: *mut net_device) -> i32 { netdev_lockdep_set_classes(ldev); 0 }
unsafe extern "C" fn lowpan_open(_dev: *mut net_device) -> i32 { if open_count == 0 { lowpan_rx_init(); } open_count += 1; 0 }
unsafe extern "C" fn lowpan_stop(_dev: *mut net_device) -> i32 { open_count -= 1; if open_count == 0 { lowpan_rx_exit(); } 0 }
unsafe extern "C" fn lowpan_neigh_construct(_dev: *mut net_device, n: *mut neighbour) -> i32 { let neigh = lowpan_802154_neigh((*n).priv_); (*neigh).short_addr = 0xffffu16; 0 }
unsafe extern "C" fn lowpan_get_iflink(dev: *const net_device) -> i32 { (*(*lowpan_802154_dev(dev)).wdev).type_ as i32 }

unsafe extern "C" fn lowpan_setup(ldev: *mut net_device) { (*ldev).broadcast[..8].fill(0xff); (*ldev).hard_header_len = 40; (*ldev).flags = 0x2 | 0x1000; (*ldev).priv_flags |= 0x400; (*ldev).netdev_ops = &lowpan_netdev_ops; (*ldev).header_ops = &lowpan_header_ops; (*ldev).needs_free_netdev = true; (*ldev).netns_immutable = true; }

unsafe extern "C" fn lowpan_validate(tb: *mut *mut nlattr, _data: *mut *mut nlattr, _extack: *mut netlink_ext_ack) -> i32 { if !(*tb.add(1)).is_null() && nla_len(*tb.add(1)) != 8 { return -22; } 0 }

unsafe extern "C" fn lowpan_newlink(ldev: *mut net_device, params: *mut rtnl_newlink_params, _extack: *mut netlink_ext_ack) -> i32 {
    ASSERT_RTNL();
    if (*params).tb.is_null() || (*(*params).tb.add(3)).is_null() { return -22; }
    if !(*params).link_net.is_null() && !net_eq((*params).link_net, dev_net(ldev)) { return -22; }
    let wdev = dev_get_by_index(dev_net(ldev), nla_get_u32(*(*params).tb.add(3)));
    if wdev.is_null() { return -19; }
    if (*wdev).type_ != 0xC0 { dev_put(wdev); return -22; }
    (*lowpan_802154_dev(ldev)).wdev = wdev;
    (*ldev).needed_headroom = 0;
    (*ldev).needed_tailroom = (*wdev).needed_tailroom;
    (*ldev).neigh_priv_len = core::mem::size_of::<lowpan_802154_neigh>();
    let ret = lowpan_register_netdevice(ldev, 0);
    if ret < 0 { dev_put(wdev); return ret; }
    (*wdev).ieee802154_ptr.as_mut().unwrap().lowpan_dev = ldev;
    0
}

// The link-management callbacks and module entry points remain external-facing
// Rust functions; kernel constants and layout helpers are supplied elsewhere.
unsafe extern "C" fn lowpan_netlink_init() -> i32 { rtnl_link_register(&mut LOWPAN_LINK_OPS as *mut _) }
unsafe extern "C" fn lowpan_netlink_fini() { rtnl_link_unregister(&mut LOWPAN_LINK_OPS as *mut _); }

static mut LOWPAN_LINK_OPS: rtnl_link_ops = rtnl_link_ops { kind: b"lowpan\0".as_ptr(), priv_size: 0, setup: Some(lowpan_setup), newlink: Some(lowpan_newlink), dellink: Some(lowpan_dellink), validate: Some(lowpan_validate) };

unsafe extern "C" fn lowpan_device_event(_unused: *mut notifier_block, event: usize, ptr: *mut c_void) -> i32 { let ndev = netdev_notifier_info_to_dev(ptr); if (*ndev).type_ != 0xC0 { return 0; } let wpan_dev = (*ndev).ieee802154_ptr; if wpan_dev.is_null() { return 0; } if event == 6 { if !(*wpan_dev).lowpan_dev.is_null() { lowpan_dellink((*wpan_dev).lowpan_dev, core::ptr::null_mut()); } } else { return 0; } 1 }

static mut lowpan_dev_notifier: notifier_block = notifier_block { notifier_call: Some(lowpan_device_event) };

unsafe extern "C" fn lowpan_dellink(ldev: *mut net_device, _head: *mut list_head) { let wdev = (*lowpan_802154_dev(ldev)).wdev; (*(*wdev).ieee802154_ptr).lowpan_dev = core::ptr::null_mut(); lowpan_unregister_netdevice(ldev); dev_put(wdev); }

unsafe extern "C" fn lowpan_init_module() -> i32 { let mut err = lowpan_net_frag_init(); if err < 0 { return err; } err = lowpan_netlink_init(); if err < 0 { lowpan_net_frag_exit(); return err; } err = register_netdevice_notifier(&mut lowpan_dev_notifier); if err < 0 { lowpan_netlink_fini(); lowpan_net_frag_exit(); return err; } 0 }
unsafe extern "C" fn lowpan_cleanup_module() { lowpan_netlink_fini(); lowpan_net_frag_exit(); unregister_netdevice_notifier(&mut lowpan_dev_notifier); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
