/*
 * Xtensa MX interrupt distributor
 *
 * Copyright (C) 2002 - 2013 Tensilica, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C header guard: __LINUX_IRQCHIP_XTENSA_MX_H

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn xtensa_mx_init_legacy(interrupt_parent: *mut device_node) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
