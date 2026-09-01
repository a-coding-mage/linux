// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5677-spi.h  --  RT5677 ALSA SoC audio codec driver
 *
 * Copyright 2013 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// Header guard __RT5677_SPI_H__ omitted in Rust.

// C dependency types supplied by other headers:
// - u32
// - size_t
// - struct firmware
// - EINVAL

// Original C condition:
// #if IS_ENABLED(CONFIG_SND_SOC_RT5677_SPI)
#[cfg(CONFIG_SND_SOC_RT5677_SPI)]
extern "C" {
    pub fn rt5677_spi_read(addr: u32, rxbuf: *mut core::ffi::c_void, len: usize) -> core::ffi::c_int;
    pub fn rt5677_spi_write(addr: u32, txbuf: *const core::ffi::c_void, len: usize) -> core::ffi::c_int;
    pub fn rt5677_spi_write_firmware(addr: u32, fw: *const firmware) -> core::ffi::c_int;
    pub fn rt5677_spi_hotword_detected();
}

// Original C condition:
// #else
#[cfg(not(CONFIG_SND_SOC_RT5677_SPI))]
#[inline]
pub unsafe fn rt5677_spi_read(
    _addr: u32,
    _rxbuf: *mut core::ffi::c_void,
    _len: usize,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_RT5677_SPI))]
#[inline]
pub unsafe fn rt5677_spi_write(
    _addr: u32,
    _txbuf: *const core::ffi::c_void,
    _len: usize,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_RT5677_SPI))]
#[inline]
pub unsafe fn rt5677_spi_write_firmware(
    _addr: u32,
    _fw: *const firmware,
) -> core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_RT5677_SPI))]
#[inline]
pub unsafe fn rt5677_spi_hotword_detected() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
