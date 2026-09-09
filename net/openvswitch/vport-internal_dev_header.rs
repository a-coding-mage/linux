/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2011 Nicira, Inc.
 */

// Dependencies supplied by the translated datapath and vport headers.

use core::ffi::c_int;

// Opaque declarations corresponding to the C types used by this header.
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vport {
    _private: [u8; 0],
}

extern "C" {
    pub fn ovs_is_internal_dev(dev: *const net_device) -> c_int;
    pub fn ovs_internal_dev_get_vport(dev: *mut net_device) -> *mut vport;
    pub fn ovs_internal_dev_rtnl_link_register() -> c_int;
    pub fn ovs_internal_dev_rtnl_link_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
