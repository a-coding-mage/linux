/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions for Device tree / OpenFirmware handling on X86
 *
 * based on arch/powerpc/include/asm/prom.h which is
 *         Copyright (C) 1996-2005 Paul Mackerras.
 */

//! Rust translation of the x86 OpenFirmware/device-tree declarations.
//!
//! The original header includes Linux and architecture headers; their symbols
//! are expected to be supplied by the surrounding translation.

use core::ffi::c_char;

// CONFIG_OF is a build-time C configuration condition.  The two cfg branches
// below preserve the corresponding declaration/no-op behavior.
#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub static mut of_ioapic: core::ffi::c_int;
    pub static mut initial_dtb: u64;
    pub fn add_dtb(data: u64);
    pub fn x86_of_pci_init();
    pub fn x86_flattree_get_config();
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub fn add_dtb(_data: u64) {}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub fn x86_of_pci_init() {}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub fn x86_flattree_get_config() {}

#[cfg(not(feature = "CONFIG_OF"))]
pub const of_ioapic: i32 = 0;

extern "C" {
    pub static mut cmd_line: [c_char; COMMAND_LINE_SIZE];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
