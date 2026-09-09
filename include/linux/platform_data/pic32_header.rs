/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// Dependency supplied by the surrounding Linux translation.
use core::ffi::{c_char, c_ulong};

/*
 * PIC32 register offsets for SET/CLR/INV where supported.
 */
macro_rules! PIC32_CLR {
    ($reg:expr) => (($reg) + 0x04)
}

macro_rules! PIC32_SET {
    ($reg:expr) => (($reg) + 0x08)
}

macro_rules! PIC32_INV {
    ($reg:expr) => (($reg) + 0x0C)
}

/*
 * PIC32 Base Register Offsets
 */
pub const PIC32_BASE_CONFIG: u32 = 0x1f800000;
pub const PIC32_BASE_OSC: u32 = 0x1f801200;
pub const PIC32_BASE_RESET: u32 = 0x1f801240;
pub const PIC32_BASE_PPS: u32 = 0x1f801400;
pub const PIC32_BASE_UART: u32 = 0x1f822000;
pub const PIC32_BASE_PORT: u32 = 0x1f860000;
pub const PIC32_BASE_DEVCFG2: u32 = 0x1fc4ff44;

// When CONFIG_MACH_PIC32 is enabled, the register unlock sequence is required
// for some register access. The build system should provide this condition.
#[cfg(CONFIG_MACH_PIC32)]
extern "C" {
    pub fn pic32_syskey_unlock_debug(fn_: *const c_char, ln: c_ulong);
}

#[cfg(CONFIG_MACH_PIC32)]
macro_rules! pic32_syskey_unlock {
    () => {
        unsafe {
            pic32_syskey_unlock_debug(
                concat!(module_path!(), "\0").as_ptr() as *const c_char,
                line!() as c_ulong,
            )
        }
    };
}

// COMPILE_TEST on all other architectures.
#[cfg(not(CONFIG_MACH_PIC32))]
macro_rules! pic32_syskey_unlock {
    () => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
