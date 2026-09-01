/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (ST) 2012 Rajeev Kumar (rajeevkumar.linux@gmail.com)
 */

use core::ffi::c_void;
use core::mem::ManuallyDrop;

/* Dependencies from the original C header:
 * linux/clk.h, linux/device.h, linux/types.h,
 * sound/dmaengine_pcm.h, sound/pcm.h, sound/designware_i2s.h
 */

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2s_dma_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2s_clk_config_data {
    _unused: [u8; 0],
}

pub type u32 = u32;

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

/* common register for all channel */
pub const IER: u32 = 0x000;
pub const IRER: u32 = 0x004;
pub const ITER: u32 = 0x008;
pub const CER: u32 = 0x00C;
pub const CCR: u32 = 0x010;
pub const RXFFR: u32 = 0x014;
pub const TXFFR: u32 = 0x018;

/* Enable register fields */
pub const IER_TDM_SLOTS_SHIFT: u32 = 8;
pub const IER_FRAME_OFF_SHIFT: u32 = 5;
pub const IER_FRAME_OFF: u32 = BIT(5);
pub const IER_INTF_TYPE: u32 = BIT(1);
pub const IER_IEN: u32 = BIT(0);

/* Interrupt status register fields */
pub const ISR_TXFO: u32 = BIT(5);
pub const ISR_TXFE: u32 = BIT(4);
pub const ISR_RXFO: u32 = BIT(1);
pub const ISR_RXDA: u32 = BIT(0);

/* I2STxRxRegisters for all channels */
pub const fn LRBR_LTHR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x020)
}

pub const fn RRBR_RTHR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x024)
}

pub const fn RER(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x028)
}

pub const fn TER(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x02C)
}

pub const fn RCR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x030)
}

pub const fn TCR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x034)
}

pub const fn ISR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x038)
}

pub const fn IMR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x03C)
}

pub const fn ROR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x040)
}

pub const fn TOR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x044)
}

pub const fn RFCR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x048)
}

pub const fn TFCR(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x04C)
}

pub const fn RFF(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x050)
}

pub const fn TFF(x: u32) -> u32 {
    0x40u32.wrapping_mul(x).wrapping_add(0x054)
}

pub const fn RSLOT_TSLOT(x: u32) -> u32 {
    0x4u32.wrapping_mul(x).wrapping_add(0x224)
}

/* Receive enable register fields */
pub const RER_RXSLOT_SHIFT: u32 = 8;
pub const RER_RXCHEN: u32 = BIT(0);

/* Transmit enable register fields */
pub const TER_TXSLOT_SHIFT: u32 = 8;
pub const TER_TXCHEN: u32 = BIT(0);

/* I2SCOMPRegisters */
pub const I2S_COMP_PARAM_2: u32 = 0x01F0;
pub const I2S_COMP_PARAM_1: u32 = 0x01F4;
pub const I2S_COMP_VERSION: u32 = 0x01F8;
pub const I2S_COMP_TYPE: u32 = 0x01FC;

pub const I2S_RRXDMA: u32 = 0x01C4;
pub const I2S_RTXDMA: u32 = 0x01CC;
pub const I2S_DMACR: u32 = 0x0200;
pub const I2S_DMAEN_RXBLOCK: u32 = 1 << 16;
pub const I2S_DMAEN_TXBLOCK: u32 = 1 << 17;

/*
 * Component parameter register fields - define the I2S block's
 * configuration.
 */
pub const fn COMP1_TX_WORDSIZE_3(r: u32) -> u32 {
    ((r & GENMASK(27, 25)) >> 25)
}

pub const fn COMP1_TX_WORDSIZE_2(r: u32) -> u32 {
    ((r & GENMASK(24, 22)) >> 22)
}

pub const fn COMP1_TX_WORDSIZE_1(r: u32) -> u32 {
    ((r & GENMASK(21, 19)) >> 19)
}

pub const fn COMP1_TX_WORDSIZE_0(r: u32) -> u32 {
    ((r & GENMASK(18, 16)) >> 16)
}

pub const fn COMP1_TX_CHANNELS(r: u32) -> u32 {
    ((r & GENMASK(10, 9)) >> 9)
}

pub const fn COMP1_RX_CHANNELS(r: u32) -> u32 {
    ((r & GENMASK(8, 7)) >> 7)
}

pub const fn COMP1_RX_ENABLED(r: u32) -> u32 {
    ((r & BIT(6)) >> 6)
}

pub const fn COMP1_TX_ENABLED(r: u32) -> u32 {
    ((r & BIT(5)) >> 5)
}

pub const fn COMP1_MODE_EN(r: u32) -> u32 {
    ((r & BIT(4)) >> 4)
}

pub const fn COMP1_FIFO_DEPTH_GLOBAL(r: u32) -> u32 {
    ((r & GENMASK(3, 2)) >> 2)
}

pub const fn COMP1_APB_DATA_WIDTH(r: u32) -> u32 {
    ((r & GENMASK(1, 0)) >> 0)
}

pub const fn COMP2_RX_WORDSIZE_3(r: u32) -> u32 {
    ((r & GENMASK(12, 10)) >> 10)
}

pub const fn COMP2_RX_WORDSIZE_2(r: u32) -> u32 {
    ((r & GENMASK(9, 7)) >> 7)
}

pub const fn COMP2_RX_WORDSIZE_1(r: u32) -> u32 {
    ((r & GENMASK(5, 3)) >> 3)
}

pub const fn COMP2_RX_WORDSIZE_0(r: u32) -> u32 {
    ((r & GENMASK(2, 0)) >> 0)
}

/* Number of entries in WORDSIZE and DATA_WIDTH parameter registers */
pub const COMP_MAX_WORDSIZE: u32 = 1 << 3;
pub const COMP_MAX_DATA_WIDTH: u32 = 1 << 2;

pub const MAX_CHANNEL_NUM: u32 = 8;
pub const MIN_CHANNEL_NUM: u32 = 2;

#[repr(C)]
pub union dw_i2s_snd_dma_data {
    pub pd: ManuallyDrop<i2s_dma_data>,
    pub dt: ManuallyDrop<snd_dmaengine_dai_dma_data>,
}

#[repr(C)]
pub struct dw_i2s_dev {
    pub i2s_base: *mut c_void,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub active: i32,
    pub capability: u32,
    pub quirks: u32,
    pub i2s_reg_comp1: u32,
    pub i2s_reg_comp2: u32,
    pub dev: *mut device,
    pub ccr: u32,
    pub xfer_resolution: u32,
    pub fifo_th: u32,
    pub l_reg: u32,
    pub r_reg: u32,
    pub is_jh7110: bool, /* Flag for StarFive JH7110 SoC */

    /* data related to DMA transfers b/w i2s and DMAC */
    pub play_dma_data: dw_i2s_snd_dma_data,
    pub capture_dma_data: dw_i2s_snd_dma_data,
    pub config: i2s_clk_config_data,
    pub i2s_clk_cfg: Option<unsafe extern "C" fn(config: *mut i2s_clk_config_data) -> i32>,

    /* data related to PIO transfers */
    pub use_pio: bool,

    /* data related to TDM mode */
    pub tdm_slots: u32,
    pub tdm_mask: u32,
    pub frame_offset: u32,

    pub tx_substream: *mut snd_pcm_substream, /* __rcu */
    pub rx_substream: *mut snd_pcm_substream, /* __rcu */
    pub tx_fn: Option<
        unsafe extern "C" fn(
            dev: *mut dw_i2s_dev,
            runtime: *mut snd_pcm_runtime,
            tx_ptr: u32,
            period_elapsed: *mut bool,
        ) -> u32,
    >,
    pub rx_fn: Option<
        unsafe extern "C" fn(
            dev: *mut dw_i2s_dev,
            runtime: *mut snd_pcm_runtime,
            rx_ptr: u32,
            period_elapsed: *mut bool,
        ) -> u32,
    >,
    pub tx_ptr: u32,
    pub rx_ptr: u32,
}

/* Original condition: #if IS_ENABLED(CONFIG_SND_DESIGNWARE_PCM) */
unsafe extern "C" {
    pub fn dw_pcm_push_tx(dev: *mut dw_i2s_dev);
    pub fn dw_pcm_pop_rx(dev: *mut dw_i2s_dev);
    pub fn dw_pcm_register(pdev: *mut platform_device) -> i32;
}

/* Original #else fallback:
 * static inline void dw_pcm_push_tx(struct dw_i2s_dev *dev) { }
 * static inline void dw_pcm_pop_rx(struct dw_i2s_dev *dev) { }
 * static inline int dw_pcm_register(struct platform_device *pdev)
 * {
 *     return -EINVAL;
 * }
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
