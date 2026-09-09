/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and include directives have no Rust equivalent.

// In the kernel build, kernel_ulong_t is defined as unsigned long.  Rust's
// usize corresponds to the target-sized unsigned integer used here.
pub type kernel_ulong_t = usize;

/* i2c */

pub const I2C_NAME_SIZE: usize = 20;
pub const I2C_MODULE_PREFIX: &str = "i2c:";

#[repr(C)]
pub struct i2c_device_id {
    pub name: [std::ffi::c_char; I2C_NAME_SIZE],
    pub driver_data: kernel_ulong_t, // Data private to the driver
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
