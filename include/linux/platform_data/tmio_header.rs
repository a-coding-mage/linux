/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* TMIO MMC platform flags */

/*
 * Some controllers can support a 2-byte block size when the bus width is
 * configured in 4-bit mode.
 */
pub const TMIO_MMC_BLKSZ_2BYTES: u32 = 1u32 << 1;

/* Some controllers can support SDIO IRQ signalling */
pub const TMIO_MMC_SDIO_IRQ: u32 = 1u32 << 2;

/* Some features are only available or tested on R-Car Gen2 or later */
pub const TMIO_MMC_MIN_RCAR2: u32 = 1u32 << 3;

/*
 * Some controllers require waiting for the SD bus to become idle before
 * writing to some registers.
 */
pub const TMIO_MMC_HAS_IDLE_WAIT: u32 = 1u32 << 4;

/*
 * Use the busy timeout feature. Probably all TMIO versions support it. Yet,
 * we don't have documentation for old variants, so we enable only known good
 * variants with this flag. Can be removed once all variants are known good.
 */
pub const TMIO_MMC_USE_BUSY_TIMEOUT: u32 = 1u32 << 5;

/* Some controllers have CMD12 automatically issue/non-issue register */
pub const TMIO_MMC_HAVE_CMD12_CTRL: u32 = 1u32 << 7;

/* Controller has some SDIO status bits which must be 1 */
pub const TMIO_MMC_SDIO_STATUS_SETBITS: u32 = 1u32 << 8;

/* Some controllers have a 32-bit wide data port register */
pub const TMIO_MMC_32BIT_DATA_PORT: u32 = 1u32 << 9;

/* Some controllers allows to set SDx actual clock */
pub const TMIO_MMC_CLK_ACTUAL: u32 = 1u32 << 10;

/* Some controllers have a CBSY bit */
pub const TMIO_MMC_HAVE_CBSY: u32 = 1u32 << 11;

/* Some controllers have a 64-bit wide data port register */
pub const TMIO_MMC_64BIT_DATA_PORT: u32 = 1u32 << 12;

#[repr(C)]
pub struct tmio_mmc_data {
    pub chan_priv_tx: *mut c_void,
    pub chan_priv_rx: *mut c_void,
    pub hclk: u32,
    pub capabilities: u64,
    pub capabilities2: u64,
    pub flags: u64,
    pub ocr_mask: u32, /* available voltages */
    pub dma_rx_offset: dma_addr_t,
    pub max_blk_count: u32,
    pub max_segs: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
