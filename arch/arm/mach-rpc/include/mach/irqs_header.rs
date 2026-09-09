/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-rpc/include/mach/irqs.h
 *
 *  Copyright (C) 1996 Russell King
 */

pub const IRQ_PRINTER: i32 = 0;
pub const IRQ_BATLOW: i32 = 1;
pub const IRQ_FLOPPYINDEX: i32 = 2;
pub const IRQ_VSYNCPULSE: i32 = 3;
pub const IRQ_POWERON: i32 = 4;
pub const IRQ_TIMER0: i32 = 5;
pub const IRQ_TIMER1: i32 = 6;
pub const IRQ_IMMEDIATE: i32 = 7;
pub const IRQ_EXPCARDFIQ: i32 = 8;
pub const IRQ_HARDDISK: i32 = 9;
pub const IRQ_SERIALPORT: i32 = 10;
pub const IRQ_FLOPPYDISK: i32 = 12;
pub const IRQ_EXPANSIONCARD: i32 = 13;
pub const IRQ_KEYBOARDTX: i32 = 14;
pub const IRQ_KEYBOARDRX: i32 = 15;

pub const IRQ_DMA0: i32 = 16;
pub const IRQ_DMA1: i32 = 17;
pub const IRQ_DMA2: i32 = 18;
pub const IRQ_DMA3: i32 = 19;
pub const IRQ_DMAS0: i32 = 20;
pub const IRQ_DMAS1: i32 = 21;

pub const FIQ_FLOPPYDATA: i32 = 0;
pub const FIQ_ECONET: i32 = 2;
pub const FIQ_SERIALPORT: i32 = 4;
pub const FIQ_EXPANSIONCARD: i32 = 6;
pub const FIQ_FORCE: i32 = 7;

/*
 * This is the offset of the FIQ "IRQ" numbers
 */
pub const FIQ_START: i32 = 64;

pub const NR_IRQS: i32 = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
