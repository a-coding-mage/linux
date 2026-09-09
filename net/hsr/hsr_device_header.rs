/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * include file for HSR and PRP.
 */

/* Dependency intent: declarations from <linux/netdevice.h> and "hsr_main.h". */

use core::ffi::c_int;

/* Opaque declarations supplied by the included headers. */
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hsr_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

extern "C" {
    pub fn hsr_del_ports(hsr: *mut hsr_priv);
    pub fn hsr_dev_setup(dev: *mut net_device);
    pub fn hsr_dev_finalize(
        hsr_dev: *mut net_device,
        slave: *mut *mut net_device,
        interlink: *mut net_device,
        multicast_spec: u8,
        protocol_version: u8,
        extack: *mut netlink_ext_ack,
    ) -> c_int;
    pub fn hsr_check_carrier_and_operstate(hsr: *mut hsr_priv);
    pub fn hsr_get_max_mtu(hsr: *mut hsr_priv) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
