/* SPDX-License-Identifier: GPL-2.0 */
/*
 * MMIO/IRQ and platform data for SH7760 I2C channels
 */

pub const SH7760_I2C_DEVNAME: &str = "sh7760-i2c";

pub const SH7760_I2C0_MMIO: u32 = 0xFE140000;
pub const SH7760_I2C0_MMIOEND: u32 = 0xFE14003B;

pub const SH7760_I2C1_MMIO: u32 = 0xFE150000;
pub const SH7760_I2C1_MMIOEND: u32 = 0xFE15003B;

#[repr(C)]
pub struct sh7760_i2c_platdata {
    pub speed_khz: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
