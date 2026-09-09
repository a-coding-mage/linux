/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Broadcom Cable Modem firmware format
 */

// Translated from <linux/types.h>; __u16 and __u32 correspond to fixed-width
// unsigned integer types.

#[repr(C)]
pub struct bcm_hcs {
    pub magic: u16,
    pub control: u16,
    pub rev_maj: u16,
    pub rev_min: u16,
    pub build_date: u32,
    pub filelen: u32,
    pub ldaddress: u32,
    pub filename: [core::ffi::c_char; 64],
    pub hcs: u16,
    pub her_znaet_chto: u16,
    pub crc: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
