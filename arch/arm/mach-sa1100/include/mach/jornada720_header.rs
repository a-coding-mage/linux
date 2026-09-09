/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-sa1100/include/mach/jornada720.h
 *
 * SSP/MCU communication definitions for HP Jornada 710/720/728
 *
 * Copyright 2007,2008 Kristoffer Ericson <Kristoffer.Ericson@gmail.com>
 *  Copyright 2000 John Ankcorn <jca@lcs.mit.edu>
 */

/* HP Jornada 7xx microprocessor commands */
pub const GETBATTERYDATA: u8 = 0xc0;
pub const GETSCANKEYCODE: u8 = 0x90;
pub const GETTOUCHSAMPLES: u8 = 0xa0;
pub const GETCONTRAST: u8 = 0xD0;
pub const SETCONTRAST: u8 = 0xD1;
pub const GETBRIGHTNESS: u8 = 0xD2;
pub const SETBRIGHTNESS: u8 = 0xD3;
pub const CONTRASTOFF: u8 = 0xD8;
pub const BRIGHTNESSOFF: u8 = 0xD9;
pub const PWMOFF: u8 = 0xDF;
pub const TXDUMMY: u8 = 0x11;
pub const ERRORCODE: u8 = 0x00;

unsafe extern "C" {
    pub fn jornada_ssp_start();
    pub fn jornada_ssp_end();
    pub fn jornada_ssp_inout(byte: u8) -> i32;
    pub fn jornada_ssp_byte(byte: u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
