/* SPDX-License-Identifier: GPL-2.0-only */
/* include/net/ax88796.h
 *
 * Copyright 2005 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 */

// C dependency: <linux/types.h>

use core::ffi::c_int;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

pub const AXFLG_HAS_EEPROM: u32 = 1 << 0;
pub const AXFLG_MAC_FROMDEV: u32 = 1 << 1; // device already has MAC
pub const AXFLG_HAS_93CX6: u32 = 1 << 2; // use eeprom_93cx6 driver
pub const AXFLG_MAC_FROMPLATFORM: u32 = 1 << 3; // MAC given by platform data

#[repr(C)]
pub struct ax_plat_data {
    pub flags: u32,
    pub wordlength: u8, // 1 or 2
    pub dcr_val: u8,    // default value for DCR
    pub rcr_val: u8,    // default value for RCR
    pub gpoc_val: u8,   // default value for GPOC
    pub reg_offsets: *mut u32, // register offsets
    pub mac_addr: *mut u8, // MAC addr (only used when AXFLG_MAC_FROMPLATFORM is used)

    // uses default ax88796 buffer if set to NULL
    pub block_output: Option<unsafe extern "C" fn(
        dev: *mut net_device,
        count: c_int,
        buf: *const u8,
        star_page: c_int,
    )>,
    pub block_input: Option<unsafe extern "C" fn(
        dev: *mut net_device,
        count: c_int,
        skb: *mut sk_buff,
        ring_offset: c_int,
    )>,
    // returns nonzero if a pending interrupt request might be caused by
    // the ax88796. Handles all interrupts if set to NULL
    pub check_irq: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

// exported from ax88796.c for xsurf100.c
extern "C" {
    pub fn ax_NS8390_reinit(dev: *mut net_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
