/* SPDX-License-Identifier: GPL-2.0 */

/* platform data for the MAX732x 8/16-bit I/O expander driver */

#[repr(C)]
pub struct max732x_platform_data {
    /* number of the first GPIO */
    pub gpio_base: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
