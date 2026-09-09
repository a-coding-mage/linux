/****************************************************************************
 *
 *     Author: Xilinx, Inc.
 *
 *     This program is free software; you can redistribute it and/or modify it
 *     under the terms of the GNU General Public License as published by the
 *     Free Software Foundation; either version 2 of the License, or (at your
 *     option) any later version.
 *
 *     XILINX IS PROVIDING THIS DESIGN, CODE, OR INFORMATION "AS IS"
 *     AS A COURTESY TO YOU, SOLELY FOR USE IN DEVELOPING PROGRAMS AND
 *     SOLUTIONS FOR XILINX DEVICES. BY PROVIDING THIS DESIGN, CODE,
 *     OR INFORMATION AS ONE POSSIBLE IMPLEMENTATION OF THIS FEATURE,
 *     APPLICATION OR STANDARD, XILINX IS MAKING NO REPRESENTATION
 *     THAT THIS IMPLEMENTATION IS FREE FROM ANY CLAIMS OF INFRINGEMENT,
 *     AND YOU ARE RESPONSIBLE FOR OBTAINING ANY RIGHTS YOU MAY REQUIRE
 *     FOR YOUR IMPLEMENTATION. XILINX EXPRESSLY DISCLAIMS ANY
 *     WARRANTY WHATSOEVER WITH RESPECT TO THE ADEQUACY OF THE
 *     IMPLEMENTATION, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OR
 *     REPRESENTATIONS THAT THIS IMPLEMENTATION IS FREE FROM CLAIMS OF
 *     INFRINGEMENT, IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE.
 *
 *     (c) Copyright 2003-2007 Xilinx Inc.
 *     All rights reserved.
 *
 *     Licensed under the GNU General Public License, version 2 or later.
 *
 ****************************************************************************/

#[repr(C)]
pub struct hwicap_drvdata {
    pub write_buffer_in_use: u32, // Always in [0,3]
    pub write_buffer: [u8; 4],
    pub read_buffer_in_use: u32, // Always in [0,3]
    pub read_buffer: [u8; 4],
    pub mem_start: resource_size_t, // phys. address of the control registers
    pub mem_end: resource_size_t, // phys. address of the control registers
    pub mem_size: resource_size_t,
    pub base_address: *mut core::ffi::c_void, // virt. address of the control registers
    pub dev: *mut device,
    pub cdev: cdev, // Char device structure
    pub devt: dev_t,
    pub config: *const hwicap_driver_config,
    pub config_regs: *const config_registers,
    pub private_data: *mut core::ffi::c_void,
    pub is_open: bool,
    pub sem: mutex,
}

#[repr(C)]
pub struct hwicap_driver_config {
    // Read configuration data given by size into the data buffer.
    // Return 0 if successful.
    pub get_configuration: Option<unsafe extern "C" fn(*mut hwicap_drvdata, *mut u32, u32) -> i32>,
    // Write configuration data given by size from the data buffer.
    // Return 0 if successful.
    pub set_configuration: Option<unsafe extern "C" fn(*mut hwicap_drvdata, *mut u32, u32) -> i32>,
    // Get the status register.
    pub get_status: Option<unsafe extern "C" fn(*mut hwicap_drvdata) -> u32>,
    // Reset the hw
    pub reset: Option<unsafe extern "C" fn(*mut hwicap_drvdata)>,
}

// Number of times to poll the done register.
pub const XHI_MAX_RETRIES: u32 = 5000;

pub const XHI_PAD_FRAMES: u32 = 0x1;

// Mask for calculating configuration packet headers
pub const XHI_WORD_COUNT_MASK_TYPE_1: u32 = 0x7FF;
pub const XHI_WORD_COUNT_MASK_TYPE_2: u32 = 0x1FFFFF;
pub const XHI_TYPE_MASK: u32 = 0x7;
pub const XHI_REGISTER_MASK: u32 = 0xF;
pub const XHI_OP_MASK: u32 = 0x3;

pub const XHI_TYPE_SHIFT: u32 = 29;
pub const XHI_REGISTER_SHIFT: u32 = 13;
pub const XHI_OP_SHIFT: u32 = 27;

pub const XHI_TYPE_1: u32 = 1;
pub const XHI_TYPE_2: u32 = 2;
pub const XHI_OP_WRITE: u32 = 2;
pub const XHI_OP_READ: u32 = 1;

// Address Block Types
pub const XHI_FAR_CLB_BLOCK: u32 = 0;
pub const XHI_FAR_BRAM_BLOCK: u32 = 1;
pub const XHI_FAR_BRAM_INT_BLOCK: u32 = 2;

#[repr(C)]
pub struct config_registers {
    pub CRC: u32,
    pub FAR: u32,
    pub FDRI: u32,
    pub FDRO: u32,
    pub CMD: u32,
    pub CTL: u32,
    pub MASK: u32,
    pub STAT: u32,
    pub LOUT: u32,
    pub COR: u32,
    pub MFWR: u32,
    pub FLR: u32,
    pub KEY: u32,
    pub CBC: u32,
    pub IDCODE: u32,
    pub AXSS: u32,
    pub C0R_1: u32,
    pub CSOB: u32,
    pub WBSTAR: u32,
    pub TIMER: u32,
    pub BOOTSTS: u32,
    pub CTL_1: u32,
}

// Configuration Commands
pub const XHI_CMD_NULL: u32 = 0;
pub const XHI_CMD_WCFG: u32 = 1;
pub const XHI_CMD_MFW: u32 = 2;
pub const XHI_CMD_DGHIGH: u32 = 3;
pub const XHI_CMD_RCFG: u32 = 4;
pub const XHI_CMD_START: u32 = 5;
pub const XHI_CMD_RCAP: u32 = 6;
pub const XHI_CMD_RCRC: u32 = 7;
pub const XHI_CMD_AGHIGH: u32 = 8;
pub const XHI_CMD_SWITCH: u32 = 9;
pub const XHI_CMD_GRESTORE: u32 = 10;
pub const XHI_CMD_SHUTDOWN: u32 = 11;
pub const XHI_CMD_GCAPTURE: u32 = 12;
pub const XHI_CMD_DESYNCH: u32 = 13;
pub const XHI_CMD_IPROG: u32 = 15; // Only in Virtex5
pub const XHI_CMD_CRCC: u32 = 16; // Only in Virtex5
pub const XHI_CMD_LTIMER: u32 = 17; // Only in Virtex5

// Packet constants
pub const XHI_SYNC_PACKET: u32 = 0xAA995566;
pub const XHI_DUMMY_PACKET: u32 = 0xFFFFFFFF;
pub const XHI_NOOP_PACKET: u32 = XHI_TYPE_1 << XHI_TYPE_SHIFT;
pub const XHI_TYPE_2_READ: u32 = (XHI_TYPE_2 << XHI_TYPE_SHIFT) | (XHI_OP_READ << XHI_OP_SHIFT);
pub const XHI_TYPE_2_WRITE: u32 = (XHI_TYPE_2 << XHI_TYPE_SHIFT) | (XHI_OP_WRITE << XHI_OP_SHIFT);
pub const XHI_TYPE2_CNT_MASK: u32 = 0x07FFFFFF;
pub const XHI_TYPE_1_PACKET_MAX_WORDS: u32 = 2047;
pub const XHI_TYPE_1_HEADER_BYTES: u32 = 4;
pub const XHI_TYPE_2_HEADER_BYTES: u32 = 8;

pub const XHI_DISABLED_AUTO_CRC: u32 = 0x0000DEFC;

// Meanings of the bits returned by get_status
pub const XHI_SR_CFGERR_N_MASK: u32 = 0x00000100; // Config Error Mask
pub const XHI_SR_DALIGN_MASK: u32 = 0x00000080; // Data Alignment Mask
pub const XHI_SR_RIP_MASK: u32 = 0x00000040; // Read back Mask
pub const XHI_SR_IN_ABORT_N_MASK: u32 = 0x00000020; // Select Map Abort Mask
pub const XHI_SR_DONE_MASK: u32 = 0x00000001; // Done bit Mask

/// hwicap_type_1_read - Generates a Type 1 read packet header.
/// `reg` is the address of the register to be read back.
pub const unsafe fn hwicap_type_1_read(reg: u32) -> u32 {
    (XHI_TYPE_1 << XHI_TYPE_SHIFT) |
        (reg << XHI_REGISTER_SHIFT) |
        (XHI_OP_READ << XHI_OP_SHIFT)
}

/// hwicap_type_1_write - Generates a Type 1 write packet header.
/// `reg` is the address of the register to be read back.
pub const unsafe fn hwicap_type_1_write(reg: u32) -> u32 {
    (XHI_TYPE_1 << XHI_TYPE_SHIFT) |
        (reg << XHI_REGISTER_SHIFT) |
        (XHI_OP_WRITE << XHI_OP_SHIFT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
