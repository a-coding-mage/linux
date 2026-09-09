/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022 Schneider Electric
 *
 * Clément Léger <clement.leger@bootlin.com>
 */

// Opaque C structs referenced by this header.
#[repr(C)]
pub struct phylink {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phylink_pcs {
    _private: [u8; 0],
}

extern "C" {
    pub fn miic_create(dev: *mut device, np: *mut device_node) -> *mut phylink_pcs;

    pub fn miic_destroy(pcs: *mut phylink_pcs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
