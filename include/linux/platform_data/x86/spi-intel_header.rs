/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel PCH/PCU SPI flash driver.
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Mika Westerberg <mika.westerberg@linux.intel.com>
 */

use core::ffi::c_void;

#[repr(C)]
pub enum intel_spi_type {
    INTEL_SPI_BYT = 1,
    INTEL_SPI_LPT,
    INTEL_SPI_BXT,
    INTEL_SPI_CNL,
}

/**
 * struct intel_spi_boardinfo - Board specific data for Intel SPI driver
 * @type: Type which this controller is compatible with
 * @set_writeable: Try to make the chip writeable (optional)
 * @data: Data to be passed to @set_writeable can be %NULL
 */
#[repr(C)]
pub struct intel_spi_boardinfo {
    pub type_: intel_spi_type,
    pub set_writeable: Option<unsafe extern "C" fn(base: *mut c_void, data: *mut c_void) -> bool>,
    pub data: *mut c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
