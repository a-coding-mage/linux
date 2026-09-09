/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SB1250 Board Support Package
 * SMBUS Constants                         File: sb1250_smbus.h
 *
 * This module contains constants and macros useful for manipulating the
 * SB1250's SMbus devices.
 *
 * SB1250 specification level:  10/21/02
 * BCM1280 specification level: 11/24/03
 *
 * Copyright 2000,2001,2002,2003 Broadcom Corporation. All rights reserved.
 */

// Dependency supplied by the surrounding port: asm/sibyte/sb1250_defs.h.

/* SMBus Clock Frequency Register (Table 14-2) */
pub const S_SMB_FREQ_DIV: u32 = 0;
pub const M_SMB_FREQ_DIV: u32 = _SB_MAKEMASK!(13, S_SMB_FREQ_DIV);
macro_rules! V_SMB_FREQ_DIV { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_FREQ_DIV) }; }

pub const K_SMB_FREQ_400KHZ: u32 = 0x1F;
pub const K_SMB_FREQ_100KHZ: u32 = 0x7D;
pub const K_SMB_FREQ_10KHZ: u32 = 1250;

pub const S_SMB_CMD: u32 = 0;
pub const M_SMB_CMD: u32 = _SB_MAKEMASK!(8, S_SMB_CMD);
macro_rules! V_SMB_CMD { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_CMD) }; }

/* SMBus control register (Table 14-4) */
pub const M_SMB_ERR_INTR: u32 = _SB_MAKEMASK1!(0);
pub const M_SMB_FINISH_INTR: u32 = _SB_MAKEMASK1!(1);
pub const S_SMB_DATA_OUT: u32 = 4;
pub const M_SMB_DATA_OUT: u32 = _SB_MAKEMASK1!(S_SMB_DATA_OUT);
macro_rules! V_SMB_DATA_OUT { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_DATA_OUT) }; }
pub const M_SMB_DATA_DIR: u32 = _SB_MAKEMASK1!(5);
pub const M_SMB_DATA_DIR_OUTPUT: u32 = M_SMB_DATA_DIR;
pub const M_SMB_CLK_OUT: u32 = _SB_MAKEMASK1!(6);
pub const M_SMB_DIRECT_ENABLE: u32 = _SB_MAKEMASK1!(7);

/* SMBus status registers (Table 14-5) */
pub const M_SMB_BUSY: u32 = _SB_MAKEMASK1!(0);
pub const M_SMB_ERROR: u32 = _SB_MAKEMASK1!(1);
pub const M_SMB_ERROR_TYPE: u32 = _SB_MAKEMASK1!(2);

// Conditional in the C header: 1250 PASS3 || 112x PASS1 || 1480.
pub const S_SMB_SCL_IN: u32 = 5;
pub const M_SMB_SCL_IN: u32 = _SB_MAKEMASK1!(S_SMB_SCL_IN);
macro_rules! V_SMB_SCL_IN { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_SCL_IN) }; }
macro_rules! G_SMB_SCL_IN { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_SCL_IN, M_SMB_SCL_IN) }; }

pub const S_SMB_REF: u32 = 6;
pub const M_SMB_REF: u32 = _SB_MAKEMASK1!(S_SMB_REF);
macro_rules! V_SMB_REF { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_REF) }; }
macro_rules! G_SMB_REF { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_REF, M_SMB_REF) }; }
pub const S_SMB_DATA_IN: u32 = 7;
pub const M_SMB_DATA_IN: u32 = _SB_MAKEMASK1!(S_SMB_DATA_IN);
macro_rules! V_SMB_DATA_IN { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_DATA_IN) }; }
macro_rules! G_SMB_DATA_IN { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_DATA_IN, M_SMB_DATA_IN) }; }

/* SMBus Start/Command registers (Table 14-9) */
pub const S_SMB_ADDR: u32 = 0;
pub const M_SMB_ADDR: u32 = _SB_MAKEMASK!(7, S_SMB_ADDR);
macro_rules! V_SMB_ADDR { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_ADDR) }; }
macro_rules! G_SMB_ADDR { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_ADDR, M_SMB_ADDR) }; }
pub const M_SMB_QDATA: u32 = _SB_MAKEMASK1!(7);
pub const S_SMB_TT: u32 = 8;
pub const M_SMB_TT: u32 = _SB_MAKEMASK!(3, S_SMB_TT);
macro_rules! V_SMB_TT { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_TT) }; }
macro_rules! G_SMB_TT { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_TT, M_SMB_TT) }; }

pub const K_SMB_TT_WR1BYTE: u32 = 0;
pub const K_SMB_TT_WR2BYTE: u32 = 1;
pub const K_SMB_TT_WR3BYTE: u32 = 2;
pub const K_SMB_TT_CMD_RD1BYTE: u32 = 3;
pub const K_SMB_TT_CMD_RD2BYTE: u32 = 4;
pub const K_SMB_TT_RD1BYTE: u32 = 5;
pub const K_SMB_TT_QUICKCMD: u32 = 6;
pub const K_SMB_TT_EEPROMREAD: u32 = 7;
macro_rules! V_SMB_TT_WR1BYTE { () => { V_SMB_TT!(K_SMB_TT_WR1BYTE) }; }
macro_rules! V_SMB_TT_WR2BYTE { () => { V_SMB_TT!(K_SMB_TT_WR2BYTE) }; }
macro_rules! V_SMB_TT_WR3BYTE { () => { V_SMB_TT!(K_SMB_TT_WR3BYTE) }; }
macro_rules! V_SMB_TT_CMD_RD1BYTE { () => { V_SMB_TT!(K_SMB_TT_CMD_RD1BYTE) }; }
macro_rules! V_SMB_TT_CMD_RD2BYTE { () => { V_SMB_TT!(K_SMB_TT_CMD_RD2BYTE) }; }
macro_rules! V_SMB_TT_RD1BYTE { () => { V_SMB_TT!(K_SMB_TT_RD1BYTE) }; }
macro_rules! V_SMB_TT_QUICKCMD { () => { V_SMB_TT!(K_SMB_TT_QUICKCMD) }; }
macro_rules! V_SMB_TT_EEPROMREAD { () => { V_SMB_TT!(K_SMB_TT_EEPROMREAD) }; }
pub const M_SMB_PEC: u32 = _SB_MAKEMASK1!(15);

/* SMBus Data Register (Table 14-6) and SMBus Extra Register (Table 14-7) */
pub const S_SMB_LB: u32 = 0;
pub const M_SMB_LB: u32 = _SB_MAKEMASK!(8, S_SMB_LB);
macro_rules! V_SMB_LB { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_LB) }; }
pub const S_SMB_MB: u32 = 8;
pub const M_SMB_MB: u32 = _SB_MAKEMASK!(8, S_SMB_MB);
macro_rules! V_SMB_MB { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_MB) }; }

/* SMBus Packet Error Check register (Table 14-8) */
pub const S_SPEC_PEC: u32 = 0;
pub const M_SPEC_PEC: u32 = _SB_MAKEMASK!(8, S_SPEC_PEC);
macro_rules! V_SPEC_MB { ($x:expr) => { _SB_MAKEVALUE!($x, S_SPEC_PEC) }; }

// Conditional in the C header: 1250 PASS2 || 112x PASS1 || 1480.
pub const S_SMB_CMDH: u32 = 8;
pub const M_SMB_CMDH: u32 = _SB_MAKEMASK!(8, S_SMB_CMDH);
macro_rules! V_SMB_CMDH { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_CMDH) }; }
pub const M_SMB_EXTEND: u32 = _SB_MAKEMASK1!(14);
pub const S_SMB_DFMT: u32 = 8;
pub const M_SMB_DFMT: u32 = _SB_MAKEMASK!(3, S_SMB_DFMT);
macro_rules! V_SMB_DFMT { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_DFMT) }; }
macro_rules! G_SMB_DFMT { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_DFMT, M_SMB_DFMT) }; }
pub const K_SMB_DFMT_1BYTE: u32 = 0;
pub const K_SMB_DFMT_2BYTE: u32 = 1;
pub const K_SMB_DFMT_3BYTE: u32 = 2;
pub const K_SMB_DFMT_4BYTE: u32 = 3;
pub const K_SMB_DFMT_NODATA: u32 = 4;
pub const K_SMB_DFMT_CMD4BYTE: u32 = 5;
pub const K_SMB_DFMT_CMD5BYTE: u32 = 6;
pub const K_SMB_DFMT_RESERVED: u32 = 7;
macro_rules! V_SMB_DFMT_1BYTE { () => { V_SMB_DFMT!(K_SMB_DFMT_1BYTE) }; }
macro_rules! V_SMB_DFMT_2BYTE { () => { V_SMB_DFMT!(K_SMB_DFMT_2BYTE) }; }
macro_rules! V_SMB_DFMT_3BYTE { () => { V_SMB_DFMT!(K_SMB_DFMT_3BYTE) }; }
macro_rules! V_SMB_DFMT_4BYTE { () => { V_SMB_DFMT!(K_SMB_DFMT_4BYTE) }; }
macro_rules! V_SMB_DFMT_NODATA { () => { V_SMB_DFMT!(K_SMB_DFMT_NODATA) }; }
macro_rules! V_SMB_DFMT_CMD4BYTE { () => { V_SMB_DFMT!(K_SMB_DFMT_CMD4BYTE) }; }
macro_rules! V_SMB_DFMT_CMD5BYTE { () => { V_SMB_DFMT!(K_SMB_DFMT_CMD5BYTE) }; }
macro_rules! V_SMB_DFMT_RESERVED { () => { V_SMB_DFMT!(K_SMB_DFMT_RESERVED) }; }
pub const S_SMB_AFMT: u32 = 11;
pub const M_SMB_AFMT: u32 = _SB_MAKEMASK!(2, S_SMB_AFMT);
macro_rules! V_SMB_AFMT { ($x:expr) => { _SB_MAKEVALUE!($x, S_SMB_AFMT) }; }
macro_rules! G_SMB_AFMT { ($x:expr) => { _SB_GETVALUE!($x, S_SMB_AFMT, M_SMB_AFMT) }; }
pub const K_SMB_AFMT_NONE: u32 = 0;
pub const K_SMB_AFMT_ADDR: u32 = 1;
pub const K_SMB_AFMT_ADDR_CMD1BYTE: u32 = 2;
pub const K_SMB_AFMT_ADDR_CMD2BYTE: u32 = 3;
macro_rules! V_SMB_AFMT_NONE { () => { V_SMB_AFMT!(K_SMB_AFMT_NONE) }; }
macro_rules! V_SMB_AFMT_ADDR { () => { V_SMB_AFMT!(K_SMB_AFMT_ADDR) }; }
macro_rules! V_SMB_AFMT_ADDR_CMD1BYTE { () => { V_SMB_AFMT!(K_SMB_AFMT_ADDR_CMD1BYTE) }; }
macro_rules! V_SMB_AFMT_ADDR_CMD2BYTE { () => { V_SMB_AFMT!(K_SMB_AFMT_ADDR_CMD2BYTE) }; }
pub const M_SMB_DIR: u32 = _SB_MAKEMASK1!(13);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
