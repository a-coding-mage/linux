/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/spi/mxs-spi.h
 *
 * Freescale i.MX233/i.MX28 SPI controller register definition
 *
 * Copyright 2008 Embedded Alley Solutions, Inc.
 * Copyright 2009-2011 Freescale Semiconductor, Inc.
 */

// Dependency supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn ssp_is_old(host: *const mxs_ssp) -> bool {
    (*host).devid == mxs_ssp_id::IMX23_SSP
}

/* SSP registers */
pub const HW_SSP_CTRL0: u32 = 0x000;
pub const BM_SSP_CTRL0_RUN: u32 = 1 << 29;
pub const BM_SSP_CTRL0_SDIO_IRQ_CHECK: u32 = 1 << 28;
pub const BM_SSP_CTRL0_LOCK_CS: u32 = 1 << 27;
pub const BM_SSP_CTRL0_IGNORE_CRC: u32 = 1 << 26;
pub const BM_SSP_CTRL0_READ: u32 = 1 << 25;
pub const BM_SSP_CTRL0_DATA_XFER: u32 = 1 << 24;
pub const BP_SSP_CTRL0_BUS_WIDTH: u32 = 22;
pub const BM_SSP_CTRL0_BUS_WIDTH: u32 = 0x3 << 22;
pub const BM_SSP_CTRL0_WAIT_FOR_IRQ: u32 = 1 << 21;
pub const BM_SSP_CTRL0_WAIT_FOR_CMD: u32 = 1 << 20;
pub const BM_SSP_CTRL0_LONG_RESP: u32 = 1 << 19;
pub const BM_SSP_CTRL0_GET_RESP: u32 = 1 << 17;
pub const BM_SSP_CTRL0_ENABLE: u32 = 1 << 16;
pub const BP_SSP_CTRL0_XFER_COUNT: u32 = 0;
pub const BM_SSP_CTRL0_XFER_COUNT: u32 = 0xffff;
pub const HW_SSP_CMD0: u32 = 0x010;
pub const BM_SSP_CMD0_DBL_DATA_RATE_EN: u32 = 1 << 25;
pub const BM_SSP_CMD0_SLOW_CLKING_EN: u32 = 1 << 22;
pub const BM_SSP_CMD0_CONT_CLKING_EN: u32 = 1 << 21;
pub const BM_SSP_CMD0_APPEND_8CYC: u32 = 1 << 20;
pub const BP_SSP_CMD0_BLOCK_SIZE: u32 = 16;
pub const BM_SSP_CMD0_BLOCK_SIZE: u32 = 0xf << 16;
pub const BP_SSP_CMD0_BLOCK_COUNT: u32 = 8;
pub const BM_SSP_CMD0_BLOCK_COUNT: u32 = 0xff << 8;
pub const BP_SSP_CMD0_CMD: u32 = 0;
pub const BM_SSP_CMD0_CMD: u32 = 0xff;
pub const HW_SSP_CMD1: u32 = 0x020;
pub const HW_SSP_XFER_SIZE: u32 = 0x030;
pub const HW_SSP_BLOCK_SIZE: u32 = 0x040;
pub const BP_SSP_BLOCK_SIZE_BLOCK_COUNT: u32 = 4;
pub const BM_SSP_BLOCK_SIZE_BLOCK_COUNT: u32 = 0xffffff << 4;
pub const BP_SSP_BLOCK_SIZE_BLOCK_SIZE: u32 = 0;
pub const BM_SSP_BLOCK_SIZE_BLOCK_SIZE: u32 = 0xf;

#[inline]
pub unsafe fn HW_SSP_TIMING(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x050 } else { 0x070 } }
pub const BP_SSP_TIMING_TIMEOUT: u32 = 16;
pub const BM_SSP_TIMING_TIMEOUT: u32 = 0xffff << 16;
pub const BP_SSP_TIMING_CLOCK_DIVIDE: u32 = 8;
pub const BM_SSP_TIMING_CLOCK_DIVIDE: u32 = 0xff << 8;
#[inline] pub const fn BF_SSP_TIMING_CLOCK_DIVIDE(v: u32) -> u32 { (v << 8) & BM_SSP_TIMING_CLOCK_DIVIDE }
pub const BP_SSP_TIMING_CLOCK_RATE: u32 = 0;
pub const BM_SSP_TIMING_CLOCK_RATE: u32 = 0xff;
#[inline] pub const fn BF_SSP_TIMING_CLOCK_RATE(v: u32) -> u32 { (v << 0) & BM_SSP_TIMING_CLOCK_RATE }

#[inline]
pub unsafe fn HW_SSP_CTRL1(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x060 } else { 0x080 } }
pub const BM_SSP_CTRL1_SDIO_IRQ: u32 = 1 << 31;
pub const BM_SSP_CTRL1_SDIO_IRQ_EN: u32 = 1 << 30;
pub const BM_SSP_CTRL1_RESP_ERR_IRQ: u32 = 1 << 29;
pub const BM_SSP_CTRL1_RESP_ERR_IRQ_EN: u32 = 1 << 28;
pub const BM_SSP_CTRL1_RESP_TIMEOUT_IRQ: u32 = 1 << 27;
pub const BM_SSP_CTRL1_RESP_TIMEOUT_IRQ_EN: u32 = 1 << 26;
pub const BM_SSP_CTRL1_DATA_TIMEOUT_IRQ: u32 = 1 << 25;
pub const BM_SSP_CTRL1_DATA_TIMEOUT_IRQ_EN: u32 = 1 << 24;
pub const BM_SSP_CTRL1_DATA_CRC_IRQ: u32 = 1 << 23;
pub const BM_SSP_CTRL1_DATA_CRC_IRQ_EN: u32 = 1 << 22;
pub const BM_SSP_CTRL1_FIFO_UNDERRUN_IRQ: u32 = 1 << 21;
pub const BM_SSP_CTRL1_FIFO_UNDERRUN_IRQ_EN: u32 = 1 << 20;
pub const BM_SSP_CTRL1_RECV_TIMEOUT_IRQ: u32 = 1 << 17;
pub const BM_SSP_CTRL1_RECV_TIMEOUT_IRQ_EN: u32 = 1 << 16;
pub const BM_SSP_CTRL1_FIFO_OVERRUN_IRQ: u32 = 1 << 15;
pub const BM_SSP_CTRL1_FIFO_OVERRUN_IRQ_EN: u32 = 1 << 14;
pub const BM_SSP_CTRL1_DMA_ENABLE: u32 = 1 << 13;
pub const BM_SSP_CTRL1_PHASE: u32 = 1 << 10;
pub const BM_SSP_CTRL1_POLARITY: u32 = 1 << 9;
pub const BP_SSP_CTRL1_WORD_LENGTH: u32 = 4;
pub const BM_SSP_CTRL1_WORD_LENGTH: u32 = 0xf << 4;
#[inline] pub const fn BF_SSP_CTRL1_WORD_LENGTH(v: u32) -> u32 { (v << 4) & BM_SSP_CTRL1_WORD_LENGTH }
pub const BV_SSP_CTRL1_WORD_LENGTH__FOUR_BITS: u32 = 0x3;
pub const BV_SSP_CTRL1_WORD_LENGTH__EIGHT_BITS: u32 = 0x7;
pub const BV_SSP_CTRL1_WORD_LENGTH__SIXTEEN_BITS: u32 = 0xF;
pub const BP_SSP_CTRL1_SSP_MODE: u32 = 0;
pub const BM_SSP_CTRL1_SSP_MODE: u32 = 0xf;
#[inline] pub const fn BF_SSP_CTRL1_SSP_MODE(v: u32) -> u32 { (v << 0) & BM_SSP_CTRL1_SSP_MODE }
pub const BV_SSP_CTRL1_SSP_MODE__SPI: u32 = 0x0;
pub const BV_SSP_CTRL1_SSP_MODE__SSI: u32 = 0x1;
pub const BV_SSP_CTRL1_SSP_MODE__SD_MMC: u32 = 0x3;
pub const BV_SSP_CTRL1_SSP_MODE__MS: u32 = 0x4;

#[inline] pub unsafe fn HW_SSP_DATA(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x070 } else { 0x090 } }
#[inline] pub unsafe fn HW_SSP_SDRESP0(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x080 } else { 0x0a0 } }
#[inline] pub unsafe fn HW_SSP_SDRESP1(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x090 } else { 0x0b0 } }
#[inline] pub unsafe fn HW_SSP_SDRESP2(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x0a0 } else { 0x0c0 } }
#[inline] pub unsafe fn HW_SSP_SDRESP3(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x0b0 } else { 0x0d0 } }
#[inline] pub unsafe fn HW_SSP_STATUS(h: *const mxs_ssp) -> u32 { if ssp_is_old(h) { 0x0c0 } else { 0x100 } }
pub const BM_SSP_STATUS_CARD_DETECT: u32 = 1 << 28;
pub const BM_SSP_STATUS_SDIO_IRQ: u32 = 1 << 17;
pub const BM_SSP_STATUS_FIFO_EMPTY: u32 = 1 << 5;

#[macro_export]
macro_rules! BF_SSP {
    // Rust has no direct token-pasting equivalent; pass the corresponding
    // BP_* and BM_* constants explicitly at the call site.
    ($value:expr, $bp:expr, $bm:expr) => { (($value << $bp) & $bm) };
}

pub const SSP_PIO_NUM: usize = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxs_ssp_id { IMX23_SSP, IMX28_SSP }

#[repr(C)]
pub struct mxs_ssp {
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub clk_rate: u32,
    pub devid: mxs_ssp_id,
    pub dmach: *mut dma_chan,
    pub dma_dir: u32,
    pub slave_dirn: dma_transfer_direction,
    pub ssp_pio_words: [u32; SSP_PIO_NUM],
}

extern "C" {
    pub fn mxs_ssp_set_clk_rate(ssp: *mut mxs_ssp, rate: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
