/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * include file for HSR and PRP.
 */

// C dependencies:
// - linux/if_ether.h (ETH_ALEN)
// - linux/module.h (__init, __exit)
// - uapi/linux/hsr_netlink.h

#[repr(C)]
pub struct hsr_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hsr_port {
    _private: [u8; 0],
}

pub const ETH_ALEN: usize = 6;

extern "C" {
    // C declaration: int __init hsr_netlink_init(void);
    pub fn hsr_netlink_init() -> ::core::ffi::c_int;

    // C declaration: void __exit hsr_netlink_exit(void);
    pub fn hsr_netlink_exit();

    // C declaration: void hsr_nl_ringerror(struct hsr_priv *hsr,
    //                                      unsigned char addr[ETH_ALEN],
    //                                      struct hsr_port *port);
    pub fn hsr_nl_ringerror(
        hsr: *mut hsr_priv,
        addr: *mut u8,
        port: *mut hsr_port,
    );

    // C declaration: void hsr_nl_nodedown(struct hsr_priv *hsr,
    //                                     unsigned char addr[ETH_ALEN]);
    pub fn hsr_nl_nodedown(hsr: *mut hsr_priv, addr: *mut u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
