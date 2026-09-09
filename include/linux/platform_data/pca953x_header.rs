/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent: Linux type and I2C declarations are supplied externally. */

/* platform data for the PCA9539 16-bit I/O expander driver */
#[repr(C)]
pub struct pca953x_platform_data {
	/* number of the first GPIO */
	pub gpio_base: ::core::ffi::c_uint,

	/* interrupt base */
	pub irq_base: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
