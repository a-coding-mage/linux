/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding platform register definitions:
// `io_p2v` performs the physical-to-virtual I/O address conversion.

/*
 * OS Timer & Match Registers
 */
pub const OST_PHYS: usize = 0x40A00000;
pub const OST_LEN: usize = 0x00000020;

macro_rules! OSMR0 { () => { io_p2v(0x40A00000) }; }
macro_rules! OSMR1 { () => { io_p2v(0x40A00004) }; }
macro_rules! OSMR2 { () => { io_p2v(0x40A00008) }; }
macro_rules! OSMR3 { () => { io_p2v(0x40A0000C) }; }
macro_rules! OSMR4 { () => { io_p2v(0x40A00080) }; }
macro_rules! OSCR  { () => { io_p2v(0x40A00010) }; }
macro_rules! OSCR4 { () => { io_p2v(0x40A00040) }; }
macro_rules! OMCR4 { () => { io_p2v(0x40A000C0) }; }
macro_rules! OSSR  { () => { io_p2v(0x40A00014) }; }
macro_rules! OWER  { () => { io_p2v(0x40A00018) }; }
macro_rules! OIER  { () => { io_p2v(0x40A0001C) }; }

pub const OSSR_M3: u32 = 1 << 3; // Match status channel 3
pub const OSSR_M2: u32 = 1 << 2; // Match status channel 2
pub const OSSR_M1: u32 = 1 << 1; // Match status channel 1
pub const OSSR_M0: u32 = 1 << 0; // Match status channel 0

pub const OWER_WME: u32 = 1 << 0; // Watchdog Match Enable

pub const OIER_E3: u32 = 1 << 3; // Interrupt enable channel 3
pub const OIER_E2: u32 = 1 << 2; // Interrupt enable channel 2
pub const OIER_E1: u32 = 1 << 1; // Interrupt enable channel 1
pub const OIER_E0: u32 = 1 << 0; // Interrupt enable channel 0

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
