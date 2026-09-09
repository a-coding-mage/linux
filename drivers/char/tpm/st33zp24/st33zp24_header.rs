/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * STMicroelectronics TPM Linux driver for TPM ST33ZP24
 * Copyright (C) 2009 - 2016  STMicroelectronics
 */

// The C header guard and include dependencies have no executable Rust equivalent.

pub const TPM_ST33_I2C: &str = "st33zp24-i2c";
pub const TPM_ST33_SPI: &str = "st33zp24-spi";

pub const TPM_WRITE_DIRECTION: u8 = 0x80;
pub const ST33ZP24_BUFSIZE: usize = 2048;

// Opaque types supplied by the surrounding kernel/device code.
pub enum tpm_chip {}
pub enum gpio_desc {}
pub enum wait_queue_head_t {}
pub enum device {}

#[repr(C)]
pub struct st33zp24_dev {
    pub chip: *mut tpm_chip,
    pub phy_id: *mut core::ffi::c_void,
    pub ops: *const st33zp24_phy_ops,
    pub locality: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub intrs: u32,
    pub io_lpcpd: *mut gpio_desc,
    pub read_queue: wait_queue_head_t,
}

#[repr(C)]
pub struct st33zp24_phy_ops {
    pub send: Option<unsafe extern "C" fn(
        phy_id: *mut core::ffi::c_void,
        tpm_register: u8,
        tpm_data: *mut u8,
        tpm_size: core::ffi::c_int,
    ) -> core::ffi::c_int>,
    pub recv: Option<unsafe extern "C" fn(
        phy_id: *mut core::ffi::c_void,
        tpm_register: u8,
        tpm_data: *mut u8,
        tpm_size: core::ffi::c_int,
    ) -> core::ffi::c_int>,
}

// Preserved from CONFIG_PM_SLEEP; enable these declarations when that build
// configuration is present in the surrounding crate.
#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" {
    pub fn st33zp24_pm_suspend(dev: *mut device) -> core::ffi::c_int;
    pub fn st33zp24_pm_resume(dev: *mut device) -> core::ffi::c_int;
}

unsafe extern "C" {
    pub fn st33zp24_probe(
        phy_id: *mut core::ffi::c_void,
        ops: *const st33zp24_phy_ops,
        dev: *mut device,
        irq: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn st33zp24_remove(chip: *mut tpm_chip);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
