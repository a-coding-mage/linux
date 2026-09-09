/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  include/linux/mmc/sd.h
 *
 *  Copyright (C) 2005-2007 Pierre Ossman, All Rights Reserved.
 */

/* SD commands                           type  argument     response */
/* class 0 */
/* This is basically the same command as for MMC with some quirks. */
pub const SD_SEND_RELATIVE_ADDR: u32 = 3; // bcr                     R6
pub const SD_SEND_IF_COND: u32 = 8; // bcr  [11:0] See below   R7
pub const SD_SWITCH_VOLTAGE: u32 = 11; // ac                      R1

/* Class 2 */
pub const SD_ADDR_EXT: u32 = 22; // ac   [5:0]              R1

/* class 10 */
pub const SD_SWITCH: u32 = 6; // adtc [31:0] See below   R1

/* class 5 */
pub const SD_ERASE_WR_BLK_START: u32 = 32; // ac   [31:0] data addr   R1
pub const SD_ERASE_WR_BLK_END: u32 = 33; // ac   [31:0] data addr   R1

/* Application commands */
pub const SD_APP_SET_BUS_WIDTH: u32 = 6; // ac   [1:0] bus width    R1
pub const SD_APP_SD_STATUS: u32 = 13; // adtc                    R1
pub const SD_APP_SEND_NUM_WR_BLKS: u32 = 22; // adtc                    R1
pub const SD_APP_OP_COND: u32 = 41; // bcr  [31:0] OCR         R3
pub const SD_APP_SEND_SCR: u32 = 51; // adtc                    R1

/* class 11 */
pub const SD_READ_EXTR_SINGLE: u32 = 48; // adtc [31:0]             R1
pub const SD_WRITE_EXTR_SINGLE: u32 = 49; // adtc [31:0]             R1

/* OCR bit definitions */
pub const SD_OCR_S18R: u32 = 1u32 << 24; // 1.8V switching request
pub const SD_ROCR_S18A: u32 = SD_OCR_S18R; // 1.8V switching accepted by card
pub const SD_OCR_2T: u32 = 1u32 << 27; // HO2T/CO2T - SDUC support
pub const SD_OCR_XPC: u32 = 1u32 << 28; // SDXC power control
pub const SD_OCR_CCS: u32 = 1u32 << 30; // Card Capacity Status

/*
 * SD_SWITCH argument format:
 *
 *      [31] Check (0) or switch (1)
 *      [30:24] Reserved (0)
 *      [23:20] Function group 6
 *      [19:16] Function group 5
 *      [15:12] Function group 4
 *      [11:8] Function group 3
 *      [7:4] Function group 2
 *      [3:0] Function group 1
 */

/*
 * SD_SEND_IF_COND argument format:
 *
 *	[31:12] Reserved (0)
 *	[11:8] Host Voltage Supply Flags
 *	[7:0] Check Pattern (0xAA)
 */

/* SCR field definitions */

pub const SCR_SPEC_VER_0: u32 = 0; // Implements system specification 1.0 - 1.01
pub const SCR_SPEC_VER_1: u32 = 1; // Implements system specification 1.10
pub const SCR_SPEC_VER_2: u32 = 2; // Implements system specification 2.00-3.0X

/*
 * SD bus widths
 */
pub const SD_BUS_WIDTH_1: u32 = 0;
pub const SD_BUS_WIDTH_4: u32 = 2;

/*
 * SD_SWITCH mode
 */
pub const SD_SWITCH_CHECK: u32 = 0;
pub const SD_SWITCH_SET: u32 = 1;

/*
 * SD_SWITCH function groups
 */
pub const SD_SWITCH_GRP_ACCESS: u32 = 0;

/*
 * SD_SWITCH access modes
 */
pub const SD_SWITCH_ACCESS_DEF: u32 = 0;
pub const SD_SWITCH_ACCESS_HS: u32 = 1;

/*
 * Erase/discard
 */
pub const SD_ERASE_ARG: u32 = 0x00000000;
pub const SD_DISCARD_ARG: u32 = 0x00000001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
