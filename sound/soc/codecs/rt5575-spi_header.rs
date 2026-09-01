/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5575-spi.h  --  ALC5575 SPI driver
 *
 * Copyright(c) 2025 Realtek Semiconductor Corp.
 *
 */

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* Original C condition: IS_ENABLED(CONFIG_SND_SOC_RT5575_SPI). */
#[cfg(CONFIG_SND_SOC_RT5575_SPI)]
unsafe extern "C" {
    pub fn rt5575_spi_get_device(dev: *mut device) -> *mut spi_device;
    pub fn rt5575_spi_fw_load(spi: *mut spi_device) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_SND_SOC_RT5575_SPI))]
pub unsafe extern "C" fn rt5575_spi_get_device(_dev: *mut device) -> *mut spi_device {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_SND_SOC_RT5575_SPI))]
pub unsafe extern "C" fn rt5575_spi_fw_load(_spi: *mut spi_device) -> ::core::ffi::c_int {
    -EINVAL
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
