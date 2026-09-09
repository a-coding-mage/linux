/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * board-specific data for the libertas_spi driver.
 *
 * Copyright 2008 Analog Devices Inc.
 */

/* Forward declaration of the externally defined SPI device type. */
#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct libertas_spi_platform_data {
    /* There are two ways to read data from the WLAN module's SPI
     * interface. Setting 0 or 1 here controls which one is used.
     *
     * Usually you want to set use_dummy_writes = 1.
     * However, if that doesn't work or if you are using a slow SPI clock
     * speed, you may want to use 0 here. */
    pub use_dummy_writes: u16,

    /* Board specific setup/teardown */
    pub setup: Option<unsafe extern "C" fn(spi: *mut spi_device) -> core::ffi::c_int>,
    pub teardown: Option<unsafe extern "C" fn(spi: *mut spi_device) -> core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
