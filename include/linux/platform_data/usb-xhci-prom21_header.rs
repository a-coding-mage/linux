/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AMD Promontory 21 xHCI auxiliary device platform data.
 *
 * Copyright (C) 2026 Jihong Min <hurryman2212@gmail.com>
 */

// Translated from the C header. The Linux `resource_size_t` type is supplied
// by the surrounding environment.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prom21_xhci_pdata {
    pub pdev: *mut pci_dev,
    pub regs: *mut core::ffi::c_void,
    pub rsrc_len: resource_size_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
