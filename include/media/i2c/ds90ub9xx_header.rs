/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/types.h>
use core::ffi::c_ulong;

/// Opaque declaration corresponding to `struct i2c_atr`.
pub struct i2c_atr;

/// Platform data for FPD-Link Serializers.
///
/// `port`: Deserializer RX port for this Serializer
/// `atr`: I2C ATR
/// `bc_rate`: back-channel clock rate
#[repr(C)]
pub struct ds90ub9xx_platform_data {
    pub port: u32,
    pub atr: *mut i2c_atr,
    pub bc_rate: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
