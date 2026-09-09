/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright 2020 NXP
 * Lynx PCS helpers
 */

// Dependency types supplied by linux/mdio.h and linux/phylink.h.
#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phylink_pcs {
    _private: [u8; 0],
}

extern "C" {
    pub fn lynx_pcs_create_mdiodev(bus: *mut mii_bus, addr: ::core::ffi::c_int) -> *mut phylink_pcs;
    pub fn lynx_pcs_create_fwnode(node: *mut fwnode_handle) -> *mut phylink_pcs;

    pub fn lynx_pcs_destroy(pcs: *mut phylink_pcs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
