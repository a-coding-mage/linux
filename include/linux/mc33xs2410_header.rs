/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Liebherr-Electronics and Drives GmbH
 */

// Dependency corresponding to <linux/spi/spi.h>.
// The concrete definition is supplied by the surrounding Linux bindings.
pub use crate::linux::spi::spi_device;

// Corresponds to MODULE_IMPORT_NS("PWM_MC33XS2410").

unsafe extern "C" {
    pub fn mc33xs2410_read_reg_ctrl(
        spi: *mut spi_device,
        reg: u8,
        val: *mut u16,
    ) -> core::ffi::c_int;

    pub fn mc33xs2410_read_reg_diag(
        spi: *mut spi_device,
        reg: u8,
        val: *mut u16,
    ) -> core::ffi::c_int;

    pub fn mc33xs2410_modify_reg(
        spi: *mut spi_device,
        reg: u8,
        mask: u8,
        val: u8,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
