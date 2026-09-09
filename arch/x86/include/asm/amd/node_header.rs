/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AMD Node helper functions and common defines
 *
 * Copyright (c) 2024, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
 *
 * Note:
 * Items in this file may only be used in a single place.
 * However, it's prudent to keep all AMD Node functionality
 * in a unified place rather than spreading throughout the
 * kernel.
 */

// Dependency equivalent of: #include <linux/pci.h>

pub const MAX_AMD_NUM_NODES: u32 = 8;
pub const AMD_NODE0_PCI_SLOT: u32 = 0x18;

extern "C" {
    pub fn amd_node_get_func(node: u16, func: u8) -> *mut pci_dev;
}

pub type pci_dev = core::ffi::c_void;

#[inline]
pub unsafe fn amd_num_nodes() -> u16 {
    topology_amd_nodes_per_pkg() * topology_max_packages()
}

extern "C" {
    pub fn topology_amd_nodes_per_pkg() -> u16;
    pub fn topology_max_packages() -> u16;
}

// CONFIG_AMD_NODE is a build-time condition preserved below.
#[cfg(CONFIG_AMD_NODE)]
extern "C" {
    pub fn amd_smn_read(node: u16, address: u32, value: *mut u32) -> i32;
    pub fn amd_smn_write(node: u16, address: u32, value: u32) -> i32;

    /* Should only be used by the HSMP driver. */
    pub fn amd_smn_hsmp_rdwr(node: u16, address: u32, value: *mut u32, write: bool) -> i32;
}

#[cfg(not(CONFIG_AMD_NODE))]
#[inline]
pub unsafe fn amd_smn_read(_node: u16, _address: u32, _value: *mut u32) -> i32 {
    -ENODEV
}

#[cfg(not(CONFIG_AMD_NODE))]
#[inline]
pub unsafe fn amd_smn_write(_node: u16, _address: u32, _value: u32) -> i32 {
    -ENODEV
}

#[cfg(not(CONFIG_AMD_NODE))]
#[inline]
pub unsafe fn amd_smn_hsmp_rdwr(
    _node: u16,
    _address: u32,
    _value: *mut u32,
    _write: bool,
) -> i32 {
    -ENODEV
}

// helper for use with read_poll_timeout
#[inline]
pub unsafe fn smn_read_register(reg: u32) -> i32 {
    let mut data: i32 = 0;
    let rc = amd_smn_read(0, reg, &mut data as *mut i32 as *mut u32);
    if rc != 0 {
        return rc;
    }

    data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
