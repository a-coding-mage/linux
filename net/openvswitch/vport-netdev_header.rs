/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2011 Nicira, Inc.
 */

// Translated from vport-netdev.h.
// C dependencies: <linux/netdevice.h>, <linux/rcupdate.h>, and "vport.h".

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vport {
    _private: [u8; 0],
}

extern "C" {
    pub fn ovs_netdev_get_vport(dev: *mut net_device) -> *mut vport;

    pub fn ovs_netdev_detach_dev(vport: *mut vport);

    // The C declaration carries the Linux __init attribute.
    pub fn ovs_netdev_init() -> ::core::ffi::c_int;

    pub fn ovs_netdev_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
