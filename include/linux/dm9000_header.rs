/* SPDX-License-Identifier: GPL-2.0-only */
/* include/linux/dm9000.h
 *
 * Copyright (c) 2004 Simtec Electronics
 *   Ben Dooks <ben@simtec.co.uk>
 *
 * Header file for dm9000 platform data
 */

/* Dependency supplied by the surrounding Linux translation: ETH_ALEN from
 * linux/if_ether.h. */

/* IO control flags */

pub const DM9000_PLATF_8BITONLY: u32 = 0x0001;
pub const DM9000_PLATF_16BITONLY: u32 = 0x0002;
pub const DM9000_PLATF_32BITONLY: u32 = 0x0004;
pub const DM9000_PLATF_EXT_PHY: u32 = 0x0008;
pub const DM9000_PLATF_NO_EEPROM: u32 = 0x0010;
/* Use NSR to find LinkStatus */
pub const DM9000_PLATF_SIMPLE_PHY: u32 = 0x0020;

/* platform data for platform device structure's platform_data field */
#[repr(C)]
pub struct dm9000_plat_data {
    pub flags: ::core::ffi::c_uint,
    pub dev_addr: [u8; ETH_ALEN],

    /* allow replacement IO routines */

    pub inblk: Option<unsafe extern "C" fn(
        reg: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    )>,
    pub outblk: Option<unsafe extern "C" fn(
        reg: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    )>,
    pub dumpblk: Option<unsafe extern "C" fn(
        reg: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
