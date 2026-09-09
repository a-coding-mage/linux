/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Siemens SX1 board definitions
 *
 * Copyright: Vovan888 at gmail com
 */

// C header guard: __ASM_ARCH_SX1_I2C_CHIPS_H

pub const SOFIA_MAX_LIGHT_VAL: u8 = 0x2B;

pub const SOFIA_I2C_ADDR: u8 = 0x32;
/* Sofia reg 3 bits masks */
pub const SOFIA_POWER1_REG: u8 = 0x03;

pub const SOFIA_USB_POWER: u8 = 0x01;
pub const SOFIA_MMC_POWER: u8 = 0x04;
pub const SOFIA_BLUETOOTH_POWER: u8 = 0x08;
pub const SOFIA_MMILIGHT_POWER: u8 = 0x20;

pub const SOFIA_POWER2_REG: u8 = 0x04;
pub const SOFIA_BACKLIGHT_REG: u8 = 0x06;
pub const SOFIA_KEYLIGHT_REG: u8 = 0x07;
pub const SOFIA_DIMMING_REG: u8 = 0x09;

/* Function Prototypes for SX1 devices control on I2C bus */

unsafe extern "C" {
    pub fn sx1_setbacklight(backlight: u8) -> i32;
    pub fn sx1_getbacklight(backlight: *mut u8) -> i32;
    pub fn sx1_setkeylight(keylight: u8) -> i32;
    pub fn sx1_getkeylight(keylight: *mut u8) -> i32;

    pub fn sx1_setmmipower(onoff: u8) -> i32;
    pub fn sx1_setusbpower(onoff: u8) -> i32;
    pub fn sx1_i2c_read_byte(devaddr: u8, regoffset: u8, value: *mut u8) -> i32;
    pub fn sx1_i2c_write_byte(devaddr: u8, regoffset: u8, value: u8) -> i32;

    /* MMC prototypes */
    pub fn sx1_mmc_init();
    pub fn sx1_mmc_slot_cover_handler(arg: *mut core::ffi::c_void, state: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
