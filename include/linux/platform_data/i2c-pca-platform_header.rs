/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: I2C_PCA9564_PLATFORM_H

#[repr(C)]
pub struct i2c_pca9564_pf_platform_data {
    pub i2c_clock_speed: core::ffi::c_int, // values are defined in linux/i2c-algo-pca.h
    pub timeout: core::ffi::c_int,         // timeout in jiffies
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
