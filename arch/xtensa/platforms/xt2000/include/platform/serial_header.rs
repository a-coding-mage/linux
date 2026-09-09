/*
 * platform/serial.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 Tensilica Inc.
 */

// Dependencies supplied by the surrounding platform translation:
// asm/core.h and asm/io.h

/* National-Semi PC16552D DUART: */

pub const DUART16552_1_INTNUM: u32 = XCHAL_EXTINT4_NUM;
pub const DUART16552_2_INTNUM: u32 = XCHAL_EXTINT5_NUM;

pub const DUART16552_1_ADDR: usize = IOADDR(0x0d050020); // channel 1
pub const DUART16552_2_ADDR: usize = IOADDR(0x0d050000); // channel 2

pub const DUART16552_XTAL_FREQ: u32 = 18432000; // crystal frequency in Hz
pub const BASE_BAUD: u32 = DUART16552_XTAL_FREQ / 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
