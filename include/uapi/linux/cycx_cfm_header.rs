/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * cycx_cfm.h    Cyclom 2X WAN Link Driver.
 *               Definitions for the Cyclom 2X Firmware Module (CFM).
 *
 * Author:        Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 *
 * Copyright:     (c) 1998-2003 Arnaldo Carvalho de Melo
 *
 * Based on sdlasfm.h by Gene Kozin <74604.152@compuserve.com>
 *
 *               This program is free software; you can redistribute it and/or
 *               modify it under the terms of the GNU General Public License
 *               as published by the Free Software Foundation; either version
 *               2 of the License, or (at your option) any later version.
 */

/* Defines */

pub const CFM_VERSION: u32 = 2;
pub const CFM_SIGNATURE: &str = "CFM - Cyclades CYCX Firmware Module";

/* min/max */
pub const CFM_IMAGE_SIZE: u32 = 0x20000; /* max size of CYCX code image file */
pub const CFM_DESCR_LEN: usize = 256; /* max length of description string */
pub const CFM_MAX_CYCX: usize = 1; /* max number of compatible adapters */
pub const CFM_LOAD_BUFSZ: u32 = 0x400; /* buffer size for reset code (buffer_load) */

/* Firmware Commands */
pub const GEN_POWER_ON: u32 = 0x1280;

pub const GEN_SET_SEG: u32 = 0x1401; /* boot segment setting. */
pub const GEN_BOOT_DAT: u32 = 0x1402; /* boot data. */
pub const GEN_START: u32 = 0x1403; /* board start. */
pub const GEN_DEFPAR: u32 = 0x1404; /* buffer length for boot. */

/* Adapter Types */
pub const CYCX_2X: u32 = 2;
/* for now only the 2X is supported, no plans to support 8X or 16X */
pub const CYCX_8X: u32 = 8;
pub const CYCX_16X: u32 = 16;

pub const CFID_X25_2X: u32 = 5200;

/**
 * struct cycx_fw_info - firmware module information.
 * @codeid - firmware ID
 * @version - firmware version number
 * @adapter - compatible adapter types
 * @memsize - minimum memory size
 * @reserved - reserved
 * @startoffs - entry point offset
 * @winoffs - dual-port memory window offset
 * @codeoffs - code load offset
 * @codesize - code size
 * @dataoffs - configuration data load offset
 * @datasize - configuration data size
 */
#[repr(C)]
pub struct cycx_fw_info {
    pub codeid: u16,
    pub version: u16,
    pub adapter: [u16; CFM_MAX_CYCX],
    pub memsize: std::ffi::c_ulong,
    pub reserved: [u16; 2],
    pub startoffs: u16,
    pub winoffs: u16,
    pub codeoffs: u16,
    pub codesize: std::ffi::c_ulong,
    pub dataoffs: u16,
    pub datasize: std::ffi::c_ulong,
}

/**
 * struct cycx_firmware - CYCX firmware file structure
 * @signature - CFM file signature
 * @version - file format version
 * @checksum - info + image
 * @reserved - reserved
 * @descr - description string
 * @info - firmware module info
 * @image - code image (variable size)
 */
#[repr(C)]
pub struct cycx_firmware {
    pub signature: [std::ffi::c_char; 80],
    pub version: u16,
    pub checksum: u16,
    pub reserved: [u16; 6],
    pub descr: [std::ffi::c_char; CFM_DESCR_LEN],
    pub info: cycx_fw_info,
    pub image: [u8; 0],
}

#[repr(C)]
pub struct cycx_fw_header {
    pub reset_size: std::ffi::c_ulong,
    pub data_size: std::ffi::c_ulong,
    pub code_size: std::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
