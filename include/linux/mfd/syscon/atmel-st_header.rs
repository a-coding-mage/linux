/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2005 Ivan Kokshaysky
 * Copyright (C) SAN People
 *
 * System Timer (ST) - System peripherals registers.
 * Based on AT91RM9200 datasheet revision E.
 */

// C header dependency: <linux/bitops.h>

pub const AT91_ST_CR: u32 = 0x00; // Control Register
pub const AT91_ST_WDRST: u32 = 1u32 << 0; // Watchdog Timer Restart

pub const AT91_ST_PIMR: u32 = 0x04; // Period Interval Mode Register
pub const AT91_ST_PIV: u32 = 0xffff; // Period Interval Value

pub const AT91_ST_WDMR: u32 = 0x08; // Watchdog Mode Register
pub const AT91_ST_WDV: u32 = 0xffff; // Watchdog Counter Value
pub const AT91_ST_RSTEN: u32 = 1u32 << 16; // Reset Enable
pub const AT91_ST_EXTEN: u32 = 1u32 << 17; // External Signal Assertion Enable

pub const AT91_ST_RTMR: u32 = 0x0c; // Real-time Mode Register
pub const AT91_ST_RTPRES: u32 = 0xffff; // Real-time Prescalar Value

pub const AT91_ST_SR: u32 = 0x10; // Status Register
pub const AT91_ST_PITS: u32 = 1u32 << 0; // Period Interval Timer Status
pub const AT91_ST_WDOVF: u32 = 1u32 << 1; // Watchdog Overflow
pub const AT91_ST_RTTINC: u32 = 1u32 << 2; // Real-time Timer Increment
pub const AT91_ST_ALMS: u32 = 1u32 << 3; // Alarm Status

pub const AT91_ST_IER: u32 = 0x14; // Interrupt Enable Register
pub const AT91_ST_IDR: u32 = 0x18; // Interrupt Disable Register
pub const AT91_ST_IMR: u32 = 0x1c; // Interrupt Mask Register

pub const AT91_ST_RTAR: u32 = 0x20; // Real-time Alarm Register
pub const AT91_ST_ALMV: u32 = 0xfffff; // Alarm Value

pub const AT91_ST_CRTR: u32 = 0x24; // Current Real-time Register
pub const AT91_ST_CRTV: u32 = 0xfffff; // Current Real-Time Value

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
