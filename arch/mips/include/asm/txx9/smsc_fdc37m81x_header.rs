/*
 * Interface for smsc fdc48m81x Super IO chip
 *
 * Author: MontaVista Software, Inc. source@mvista.com
 *
 * 2001-2003 (c) MontaVista Software, Inc. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 *
 * Copyright (C) 2004 MontaVista Software Inc.
 * Manish Lachwani, mlachwani@mvista.com
 */

/* Common Registers */
pub const SMSC_FDC37M81X_CONFIG_INDEX: u8 = 0x00;
pub const SMSC_FDC37M81X_CONFIG_DATA: u8 = 0x01;
pub const SMSC_FDC37M81X_CONF: u8 = 0x02;
pub const SMSC_FDC37M81X_INDEX: u8 = 0x03;
pub const SMSC_FDC37M81X_DNUM: u8 = 0x07;
pub const SMSC_FDC37M81X_DID: u8 = 0x20;
pub const SMSC_FDC37M81X_DREV: u8 = 0x21;
pub const SMSC_FDC37M81X_PCNT: u8 = 0x22;
pub const SMSC_FDC37M81X_PMGT: u8 = 0x23;
pub const SMSC_FDC37M81X_OSC: u8 = 0x24;
pub const SMSC_FDC37M81X_CONFPA0: u8 = 0x26;
pub const SMSC_FDC37M81X_CONFPA1: u8 = 0x27;
pub const SMSC_FDC37M81X_TEST4: u8 = 0x2B;
pub const SMSC_FDC37M81X_TEST5: u8 = 0x2C;
pub const SMSC_FDC37M81X_TEST1: u8 = 0x2D;
pub const SMSC_FDC37M81X_TEST2: u8 = 0x2E;
pub const SMSC_FDC37M81X_TEST3: u8 = 0x2F;

/* Logical device numbers */
pub const SMSC_FDC37M81X_FDD: u8 = 0x00;
pub const SMSC_FDC37M81X_PARALLEL: u8 = 0x03;
pub const SMSC_FDC37M81X_SERIAL1: u8 = 0x04;
pub const SMSC_FDC37M81X_SERIAL2: u8 = 0x05;
pub const SMSC_FDC37M81X_KBD: u8 = 0x07;
pub const SMSC_FDC37M81X_AUXIO: u8 = 0x08;
pub const SMSC_FDC37M81X_NONE: u8 = 0xff;

/* Logical device Config Registers */
pub const SMSC_FDC37M81X_ACTIVE: u8 = 0x30;
pub const SMSC_FDC37M81X_BASEADDR0: u8 = 0x60;
pub const SMSC_FDC37M81X_BASEADDR1: u8 = 0x61;
pub const SMSC_FDC37M81X_INT: u8 = 0x70;
pub const SMSC_FDC37M81X_INT2: u8 = 0x72;
pub const SMSC_FDC37M81X_LDCR_F0: u8 = 0xF0;

/* Chip Config Values */
pub const SMSC_FDC37M81X_CONFIG_ENTER: u8 = 0x55;
pub const SMSC_FDC37M81X_CONFIG_EXIT: u8 = 0xaa;
pub const SMSC_FDC37M81X_CHIP_ID: u8 = 0x4d;

unsafe extern "C" {
    pub fn smsc_fdc37m81x_init(port: core::ffi::c_ulong) -> core::ffi::c_ulong;

    pub fn smsc_fdc37m81x_config_beg();

    pub fn smsc_fdc37m81x_config_end();

    pub fn smsc_fdc37m81x_config_get(reg: u8) -> u8;
    pub fn smsc_fdc37m81x_config_set(reg: u8, val: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
