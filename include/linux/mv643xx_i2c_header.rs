/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

/* C header guard: _MV64XXX_I2C_H_ */

/* Dependency: linux/types.h */

pub const MV64XXX_I2C_CTLR_NAME: &str = "mv64xxx_i2c";

/* i2c Platform Device, Driver Data */
#[repr(C)]
pub struct mv64xxx_i2c_pdata {
    pub freq_m: u32,
    pub freq_n: u32,
    pub timeout: u32, /* In milliseconds */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
