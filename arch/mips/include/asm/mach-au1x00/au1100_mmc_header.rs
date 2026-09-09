/*
 * BRIEF MODULE DESCRIPTION
 *	Defines for using the MMC/SD controllers on the
 *      Alchemy Au1100 mips processor.
 *
 * Copyright (c) 2003 Embedded Edge, LLC.
 * Author: Embedded Edge, LLC.
 *         dan@embeddededge.com or tim@embeddededge.com
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */
/*
 * AU1100 MMC/SD definitions.
 *
 * From "AMD Alchemy Solutions Au1100 Processor Data Book - Preliminary"
 *    June, 2003
 */

// Dependency supplied by the Linux LED subsystem.
#[allow(non_camel_case_types)]
pub struct led_classdev;

#[repr(C)]
pub struct au1xmmc_platform_data {
    pub cd_setup: Option<unsafe extern "C" fn(mmc_host: *mut core::ffi::c_void, on: i32) -> i32>,
    pub card_inserted: Option<unsafe extern "C" fn(mmc_host: *mut core::ffi::c_void) -> i32>,
    pub card_readonly: Option<unsafe extern "C" fn(mmc_host: *mut core::ffi::c_void) -> i32>,
    pub set_power: Option<unsafe extern "C" fn(mmc_host: *mut core::ffi::c_void, state: i32)>,
    pub led: *mut led_classdev,
    pub mask_host_caps: core::ffi::c_ulong,
}

pub const SD0_BASE: u32 = 0xB0600000;
pub const SD1_BASE: u32 = 0xB0680000;

pub const SD_TXPORT: u32 = 0x0000;
pub const SD_RXPORT: u32 = 0x0004;
pub const SD_CONFIG: u32 = 0x0008;
pub const SD_ENABLE: u32 = 0x000C;
pub const SD_CONFIG2: u32 = 0x0010;
pub const SD_BLKSIZE: u32 = 0x0014;
pub const SD_STATUS: u32 = 0x0018;
pub const SD_DEBUG: u32 = 0x001C;
pub const SD_CMD: u32 = 0x0020;
pub const SD_CMDARG: u32 = 0x0024;
pub const SD_RESP3: u32 = 0x0028;
pub const SD_RESP2: u32 = 0x002C;
pub const SD_RESP1: u32 = 0x0030;
pub const SD_RESP0: u32 = 0x0034;
pub const SD_TIMEOUT: u32 = 0x0038;

pub const SD_TXPORT_TXD: u32 = 0x000000ff;
pub const SD_RXPORT_RXD: u32 = 0x000000ff;

pub const SD_CONFIG_DIV: u32 = 0x000001ff;
pub const SD_CONFIG_DE: u32 = 0x00000200;
pub const SD_CONFIG_NE: u32 = 0x00000400;
pub const SD_CONFIG_TU: u32 = 0x00000800;
pub const SD_CONFIG_TO: u32 = 0x00001000;
pub const SD_CONFIG_RU: u32 = 0x00002000;
pub const SD_CONFIG_RO: u32 = 0x00004000;
pub const SD_CONFIG_I: u32 = 0x00008000;
pub const SD_CONFIG_CR: u32 = 0x00010000;
pub const SD_CONFIG_RAT: u32 = 0x00020000;
pub const SD_CONFIG_DD: u32 = 0x00040000;
pub const SD_CONFIG_DT: u32 = 0x00080000;
pub const SD_CONFIG_SC: u32 = 0x00100000;
pub const SD_CONFIG_RC: u32 = 0x00200000;
pub const SD_CONFIG_WC: u32 = 0x00400000;
pub const SD_CONFIG_xxx: u32 = 0x00800000;
pub const SD_CONFIG_TH: u32 = 0x01000000;
pub const SD_CONFIG_TE: u32 = 0x02000000;
pub const SD_CONFIG_TA: u32 = 0x04000000;
pub const SD_CONFIG_RH: u32 = 0x08000000;
pub const SD_CONFIG_RA: u32 = 0x10000000;
pub const SD_CONFIG_RF: u32 = 0x20000000;
pub const SD_CONFIG_CD: u32 = 0x40000000;
pub const SD_CONFIG_SI: u32 = 0x80000000;

pub const SD_ENABLE_CE: u32 = 0x00000001;
pub const SD_ENABLE_R: u32 = 0x00000002;

pub const SD_CONFIG2_EN: u32 = 0x00000001;
pub const SD_CONFIG2_FF: u32 = 0x00000002;
pub const SD_CONFIG2_xx1: u32 = 0x00000004;
pub const SD_CONFIG2_DF: u32 = 0x00000008;
pub const SD_CONFIG2_DC: u32 = 0x00000010;
pub const SD_CONFIG2_xx2: u32 = 0x000000e0;
pub const SD_CONFIG2_BB: u32 = 0x00000080;
pub const SD_CONFIG2_WB: u32 = 0x00000100;
pub const SD_CONFIG2_RW: u32 = 0x00000200;
pub const SD_CONFIG2_DP: u32 = 0x00000400;

pub const SD_BLKSIZE_BS: u32 = 0x000007ff;
pub const SD_BLKSIZE_BS_SHIFT: u32 = 0;
pub const SD_BLKSIZE_BC: u32 = 0x01ff0000;
pub const SD_BLKSIZE_BC_SHIFT: u32 = 16;

pub const SD_STATUS_DCRCW: u32 = 0x00000007;
pub const SD_STATUS_xx1: u32 = 0x00000008;
pub const SD_STATUS_CB: u32 = 0x00000010;
pub const SD_STATUS_DB: u32 = 0x00000020;
pub const SD_STATUS_CF: u32 = 0x00000040;
pub const SD_STATUS_D3: u32 = 0x00000080;
pub const SD_STATUS_xx2: u32 = 0x00000300;
pub const SD_STATUS_NE: u32 = 0x00000400;
pub const SD_STATUS_TU: u32 = 0x00000800;
pub const SD_STATUS_TO: u32 = 0x00001000;
pub const SD_STATUS_RU: u32 = 0x00002000;
pub const SD_STATUS_RO: u32 = 0x00004000;
pub const SD_STATUS_I: u32 = 0x00008000;
pub const SD_STATUS_CR: u32 = 0x00010000;
pub const SD_STATUS_RAT: u32 = 0x00020000;
pub const SD_STATUS_DD: u32 = 0x00040000;
pub const SD_STATUS_DT: u32 = 0x00080000;
pub const SD_STATUS_SC: u32 = 0x00100000;
pub const SD_STATUS_RC: u32 = 0x00200000;
pub const SD_STATUS_WC: u32 = 0x00400000;
pub const SD_STATUS_xx3: u32 = 0x00800000;
pub const SD_STATUS_TH: u32 = 0x01000000;
pub const SD_STATUS_TE: u32 = 0x02000000;
pub const SD_STATUS_TA: u32 = 0x04000000;
pub const SD_STATUS_RH: u32 = 0x08000000;
pub const SD_STATUS_RA: u32 = 0x10000000;
pub const SD_STATUS_RF: u32 = 0x20000000;
pub const SD_STATUS_CD: u32 = 0x40000000;
pub const SD_STATUS_SI: u32 = 0x80000000;

pub const SD_CMD_GO: u32 = 0x00000001;
pub const SD_CMD_RY: u32 = 0x00000002;
pub const SD_CMD_xx1: u32 = 0x0000000c;
pub const SD_CMD_CT_MASK: u32 = 0x000000f0;
pub const SD_CMD_CT_0: u32 = 0x00000000;
pub const SD_CMD_CT_1: u32 = 0x00000010;
pub const SD_CMD_CT_2: u32 = 0x00000020;
pub const SD_CMD_CT_3: u32 = 0x00000030;
pub const SD_CMD_CT_4: u32 = 0x00000040;
pub const SD_CMD_CT_5: u32 = 0x00000050;
pub const SD_CMD_CT_6: u32 = 0x00000060;
pub const SD_CMD_CT_7: u32 = 0x00000070;
pub const SD_CMD_CI: u32 = 0x0000ff00;
pub const SD_CMD_CI_SHIFT: u32 = 8;
pub const SD_CMD_RT_MASK: u32 = 0x00ff0000;
pub const SD_CMD_RT_0: u32 = 0x00000000;
pub const SD_CMD_RT_1: u32 = 0x00010000;
pub const SD_CMD_RT_2: u32 = 0x00020000;
pub const SD_CMD_RT_3: u32 = 0x00030000;
pub const SD_CMD_RT_4: u32 = 0x00040000;
pub const SD_CMD_RT_5: u32 = 0x00050000;
pub const SD_CMD_RT_6: u32 = 0x00060000;
pub const SD_CMD_RT_1B: u32 = 0x00810000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
