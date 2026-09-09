/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding platform-device bindings:
// use the corresponding Rust representation of `struct device` here.

/*
 * Version 2 of the I2C peripheral unit has a different register
 * layout and extra registers.  The ID register in the V2 peripheral
 * unit on the OMAP4430 reports the same ID as the V1 peripheral
 * unit on the OMAP3530, so we must inform the driver which IP
 * version we know it is running on from platform / cpu-specific
 * code using these constants in the hwmod class definition.
 */

pub const OMAP_I2C_IP_VERSION_1: u32 = 1;
pub const OMAP_I2C_IP_VERSION_2: u32 = 2;

/* struct omap_i2c_bus_platform_data .flags meanings */

pub const OMAP_I2C_FLAG_NO_FIFO: u32 = 1u32 << 0;
pub const OMAP_I2C_FLAG_SIMPLE_CLOCK: u32 = 1u32 << 1;
pub const OMAP_I2C_FLAG_16BIT_DATA_REG: u32 = 1u32 << 2;
pub const OMAP_I2C_FLAG_ALWAYS_ARMXOR_CLK: u32 = 1u32 << 5;
pub const OMAP_I2C_FLAG_FORCE_19200_INT_CLK: u32 = 1u32 << 6;
/* how the CPU address bus must be translated for I2C unit access */
pub const OMAP_I2C_FLAG_BUS_SHIFT_NONE: u32 = 0;
pub const OMAP_I2C_FLAG_BUS_SHIFT_1: u32 = 1u32 << 7;
pub const OMAP_I2C_FLAG_BUS_SHIFT_2: u32 = 1u32 << 8;
pub const OMAP_I2C_FLAG_BUS_SHIFT__SHIFT: u32 = 7;

#[repr(C)]
pub struct omap_i2c_bus_platform_data {
    pub clkrate: u32,
    pub rev: u32,
    pub flags: u32,
    pub set_mpu_wkup_lat:
        Option<unsafe extern "C" fn(dev: *mut device, set: core::ffi::c_long)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
