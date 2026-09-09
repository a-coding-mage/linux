/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * i2c-algo-bit.h: i2c driver algorithms for bit-shift adapters
 *
 *   Copyright (C) 1995-99 Simon G. Vogl
 * With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and even
 * Frodo Looijaard <frodol@dds.nl>
 */

/* Dependency supplied by linux/i2c.h in the source tree. */

/* --- Defines for bit-adapters --------------------------------------- */
/*
 * This struct contains the hw-dependent functions of bit-style adapters to
 * manipulate the line states, and to init any hw-specific features. This is
 * only used if you have more than one hw-type of adapter running.
 */
#[repr(C)]
pub struct i2c_algo_bit_data {
    pub data: *mut core::ffi::c_void, /* private data for lowlevel routines */
    pub setsda: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void, state: core::ffi::c_int)>,
    pub setscl: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void, state: core::ffi::c_int)>,
    pub getsda: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void) -> core::ffi::c_int>,
    pub getscl: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void) -> core::ffi::c_int>,
    pub pre_xfer: Option<unsafe extern "C" fn(adapter: *mut i2c_adapter) -> core::ffi::c_int>,
    pub post_xfer: Option<unsafe extern "C" fn(adapter: *mut i2c_adapter)>,

    /* local settings */
    pub udelay: core::ffi::c_int, /* half clock cycle time in us,
                                     minimum 2 us for fast-mode I2C,
                                     minimum 5 us for standard-mode I2C and SMBus,
                                     maximum 50 us for SMBus */
    pub timeout: core::ffi::c_int, /* in jiffies */
    pub can_do_atomic: bool, /* callbacks don't sleep, we can be atomic */
    pub skip_bit_test: bool, /* override bit_test module parameter */
}

extern "C" {
    pub fn i2c_bit_add_bus(adapter: *mut i2c_adapter) -> core::ffi::c_int;
    pub fn i2c_bit_add_numbered_bus(adapter: *mut i2c_adapter) -> core::ffi::c_int;
    pub static i2c_bit_algo: i2c_algorithm;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
