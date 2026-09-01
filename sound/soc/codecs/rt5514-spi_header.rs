/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5514-spi.h  --  RT5514 driver
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

/**
 * RT5514_SPI_BUF_LEN is the buffer size of SPI master controller.
 */
pub const RT5514_SPI_BUF_LEN: usize = 240;

pub const RT5514_BUFFER_VOICE_BASE: u32 = 0x18000200;
pub const RT5514_BUFFER_VOICE_LIMIT: u32 = 0x18000204;
pub const RT5514_BUFFER_VOICE_WP: u32 = 0x1800020c;
pub const RT5514_IRQ_CTRL: u32 = 0x18002094;

pub const RT5514_IRQ_STATUS_BIT: u32 = 0x1 << 5;

/* SPI Command */
pub const RT5514_SPI_CMD_16_READ: u32 = 0;
pub const RT5514_SPI_CMD_16_WRITE: u32 = 1;
pub const RT5514_SPI_CMD_32_READ: u32 = 2;
pub const RT5514_SPI_CMD_32_WRITE: u32 = 3;
pub const RT5514_SPI_CMD_BURST_READ: u32 = 4;
pub const RT5514_SPI_CMD_BURST_WRITE: u32 = 5;

extern "C" {
    pub fn rt5514_spi_burst_read(addr: core::ffi::c_uint, rxbuf: *mut u8, len: usize) -> core::ffi::c_int;
    pub fn rt5514_spi_burst_write(addr: u32, txbuf: *const u8, len: usize) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
