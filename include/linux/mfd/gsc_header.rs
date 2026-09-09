/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2020 Gateworks Corporation
 */

// Dependency supplied by the surrounding kernel translation.
use core::ffi::c_void;

/* Device Addresses */
pub const GSC_MISC: u32 = 0x20;
pub const GSC_UPDATE: u32 = 0x21;
pub const GSC_GPIO: u32 = 0x23;
pub const GSC_HWMON: u32 = 0x29;
pub const GSC_EEPROM0: u32 = 0x50;
pub const GSC_EEPROM1: u32 = 0x51;
pub const GSC_EEPROM2: u32 = 0x52;
pub const GSC_EEPROM3: u32 = 0x53;
pub const GSC_RTC: u32 = 0x68;

/* Register offsets */
pub const GSC_CTRL_0: u32 = 0x00;
pub const GSC_CTRL_1: u32 = 0x01;
pub const GSC_TIME: u32 = 0x02;
pub const GSC_TIME_ADD: u32 = 0x06;
pub const GSC_IRQ_STATUS: u32 = 0x0A;
pub const GSC_IRQ_ENABLE: u32 = 0x0B;
pub const GSC_FW_CRC: u32 = 0x0C;
pub const GSC_FW_VER: u32 = 0x0E;
pub const GSC_WP: u32 = 0x0F;

/* Bit definitions */
pub const GSC_CTRL_0_PB_HARD_RESET: u32 = 0;
pub const GSC_CTRL_0_PB_CLEAR_SECURE_KEY: u32 = 1;
pub const GSC_CTRL_0_PB_SOFT_POWER_DOWN: u32 = 2;
pub const GSC_CTRL_0_PB_BOOT_ALTERNATE: u32 = 3;
pub const GSC_CTRL_0_PERFORM_CRC: u32 = 4;
pub const GSC_CTRL_0_TAMPER_DETECT: u32 = 5;
pub const GSC_CTRL_0_SWITCH_HOLD: u32 = 6;

pub const GSC_CTRL_1_SLEEP_ENABLE: u32 = 0;
pub const GSC_CTRL_1_SLEEP_ACTIVATE: u32 = 1;
pub const GSC_CTRL_1_SLEEP_ADD: u32 = 2;
pub const GSC_CTRL_1_SLEEP_NOWAKEPB: u32 = 3;
pub const GSC_CTRL_1_WDT_TIME: u32 = 4;
pub const GSC_CTRL_1_WDT_ENABLE: u32 = 5;
pub const GSC_CTRL_1_SWITCH_BOOT_ENABLE: u32 = 6;
pub const GSC_CTRL_1_SWITCH_BOOT_CLEAR: u32 = 7;

pub const GSC_IRQ_PB: u32 = 0;
pub const GSC_IRQ_KEY_ERASED: u32 = 1;
pub const GSC_IRQ_EEPROM_WP: u32 = 2;
pub const GSC_IRQ_RESV: u32 = 3;
pub const GSC_IRQ_GPIO: u32 = 4;
pub const GSC_IRQ_TAMPER: u32 = 5;
pub const GSC_IRQ_WDT_TIMEOUT: u32 = 6;
pub const GSC_IRQ_SWITCH_HOLD: u32 = 7;

extern "C" {
    pub fn gsc_read(context: *mut c_void, reg: u32, val: *mut u32) -> i32;
    pub fn gsc_write(context: *mut c_void, reg: u32, val: u32) -> i32;
}

#[repr(C)]
pub struct gsc_dev {
    pub dev: *mut c_void,
    pub i2c: *mut c_void,       /* 0x20: interrupt controller, WDT */
    pub i2c_hwmon: *mut c_void, /* 0x29: hwmon, fan controller */
    pub regmap: *mut c_void,
    pub fwver: u32,
    pub fwcrc: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
