/* SPDX-License-Identifier: GPL-2.0 */

// The C definition is enabled under __KERNEL__.  This Rust alias preserves
// the unsigned-long representation used by the header.
pub type kernel_ulong_t = core::ffi::c_ulong;

/* spi */

pub const SPI_NAME_SIZE: usize = 32;
pub const SPI_MODULE_PREFIX: &str = "spi:";

#[repr(C)]
pub struct spi_device_id {
    pub name: [core::ffi::c_char; SPI_NAME_SIZE],
    pub driver_data: kernel_ulong_t, /* Data private to the driver */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
