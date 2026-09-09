/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * include file for HSR and PRP.
 */

// Dependencies supplied by the surrounding kernel/networking translation:
// linux/skbuff.h, linux/netdevice.h, linux/rtnetlink.h, and hsr_main.h.

extern "C" {
    pub fn hsr_add_port(
        hsr: *mut hsr_priv,
        dev: *mut net_device,
        pt: hsr_port_type,
        extack: *mut netlink_ext_ack,
    ) -> ::core::ffi::c_int;
    pub fn hsr_del_port(port: *mut hsr_port);
    pub fn hsr_port_exists(dev: *const net_device) -> bool;
    pub fn hsr_invalid_dan_ingress_frame(protocol: __be16) -> bool;
}

// These opaque declarations correspond to types supplied by hsr_main.h and
// the Linux networking headers.
#[repr(C)]
pub struct hsr_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hsr_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    pub rx_handler_data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

pub type hsr_port_type = ::core::ffi::c_int;
pub type __be16 = u16;

// ASSERT_RTNL() is a kernel assertion supplied by linux/rtnetlink.h.
#[inline]
pub unsafe fn hsr_port_get_rtnl(dev: *const net_device) -> *mut hsr_port {
    // ASSERT_RTNL();
    if hsr_port_exists(dev) {
        // rtnl_dereference(dev->rx_handler_data)
        (*dev).rx_handler_data as *mut hsr_port
    } else {
        ::core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn hsr_port_get_rcu(dev: *const net_device) -> *mut hsr_port {
    if hsr_port_exists(dev) {
        // rcu_dereference(dev->rx_handler_data)
        (*dev).rx_handler_data as *mut hsr_port
    } else {
        ::core::ptr::null_mut()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
