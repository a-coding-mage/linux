/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Intel KeemBay Platform driver
 *
 *  Copyright (C) 2020 Intel Corporation.
 *
 */

/* Register values with reference to KMB databook v1.1 */
/* common register for all channel */
pub const IER: u32 = 0x000;
pub const IRER: u32 = 0x004;
pub const ITER: u32 = 0x008;
pub const CER: u32 = 0x00C;
pub const CCR: u32 = 0x010;
pub const RXFFR: u32 = 0x014;
pub const TXFFR: u32 = 0x018;

const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

const fn genmask(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

const fn field_get(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask.trailing_zeros()
}

/* Interrupt status register fields */
pub const ISR_TXFO: u32 = bit(5);
pub const ISR_TXFE: u32 = bit(4);
pub const ISR_RXFO: u32 = bit(1);
pub const ISR_RXDA: u32 = bit(0);

/* I2S Tx Rx Registers for all channels */
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

/* I2S COMP Registers */
pub const I2S_COMP_PARAM_2: u32 = 0x01F0;
pub const I2S_COMP_PARAM_1: u32 = 0x01F4;
pub const I2S_COMP_VERSION: u32 = 0x01F8;
pub const I2S_COMP_TYPE: u32 = 0x01FC;

/* PSS_GEN_CTRL_I2S_GEN_CFG_0 Registers */
pub const I2S_GEN_CFG_0: u32 = 0x000;
pub const PSS_CPR_RST_EN: u32 = 0x010;
pub const PSS_CPR_RST_SET: u32 = 0x014;
pub const PSS_CPR_CLK_CLR: u32 = 0x000;
pub const PSS_CPR_AUX_RST_EN: u32 = 0x070;

pub const CLOCK_PROVIDER_MODE: u32 = bit(13);

/* Interrupt Flag */
pub const TX_INT_FLAG: u32 = genmask(5, 4);
pub const RX_INT_FLAG: u32 = genmask(1, 0);
/*
 * Component parameter register fields - define the I2S block's
 * configuration.
 */
pub const fn COMP1_TX_WORDSIZE_3(r: u32) -> u32 {
    field_get(genmask(27, 25), r)
}

pub const fn COMP1_TX_WORDSIZE_2(r: u32) -> u32 {
    field_get(genmask(24, 22), r)
}

pub const fn COMP1_TX_WORDSIZE_1(r: u32) -> u32 {
    field_get(genmask(21, 19), r)
}

pub const fn COMP1_TX_WORDSIZE_0(r: u32) -> u32 {
    field_get(genmask(18, 16), r)
}

pub const fn COMP1_RX_ENABLED(r: u32) -> u32 {
    field_get(bit(6), r)
}

pub const fn COMP1_TX_ENABLED(r: u32) -> u32 {
    field_get(bit(5), r)
}

pub const fn COMP1_MODE_EN(r: u32) -> u32 {
    field_get(bit(4), r)
}

pub const fn COMP1_APB_DATA_WIDTH(r: u32) -> u32 {
    field_get(genmask(1, 0), r)
}

pub const fn COMP2_RX_WORDSIZE_3(r: u32) -> u32 {
    field_get(genmask(12, 10), r)
}

pub const fn COMP2_RX_WORDSIZE_2(r: u32) -> u32 {
    field_get(genmask(9, 7), r)
}

pub const fn COMP2_RX_WORDSIZE_1(r: u32) -> u32 {
    field_get(genmask(5, 3), r)
}

pub const fn COMP2_RX_WORDSIZE_0(r: u32) -> u32 {
    field_get(genmask(2, 0), r)
}

/* Add 1 to the below registers to indicate the actual size */
pub const fn COMP1_TX_CHANNELS(r: u32) -> u32 {
    field_get(genmask(10, 9), r).wrapping_add(1)
}

pub const fn COMP1_RX_CHANNELS(r: u32) -> u32 {
    field_get(genmask(8, 7), r).wrapping_add(1)
}

pub const fn COMP1_FIFO_DEPTH(r: u32) -> u32 {
    field_get(genmask(3, 2), r).wrapping_add(1)
}

/* Number of entries in WORDSIZE and DATA_WIDTH parameter registers */
pub const COMP_MAX_WORDSIZE: u32 = 8; /* 3 bits register width */

pub const MAX_CHANNEL_NUM: u32 = 8;
pub const MIN_CHANNEL_NUM: u32 = 2;
pub const MAX_ISR: u32 = 4;

pub const TWO_CHANNEL_SUPPORT: u32 = 2; /* up to 2.0 */
pub const FOUR_CHANNEL_SUPPORT: u32 = 4; /* up to 3.1 */
pub const SIX_CHANNEL_SUPPORT: u32 = 6; /* up to 5.1 */
pub const EIGHT_CHANNEL_SUPPORT: u32 = 8; /* up to 7.1 */

pub const DWC_I2S_PLAY: u32 = bit(0);
pub const DWC_I2S_RECORD: u32 = bit(1);
pub const DW_I2S_CONSUMER: u32 = bit(2);
pub const DW_I2S_PROVIDER: u32 = bit(3);

pub const I2S_RXDMA: u32 = 0x01C0;
pub const I2S_RRXDMA: u32 = 0x01C4;
pub const I2S_TXDMA: u32 = 0x01C8;
pub const I2S_RTXDMA: u32 = 0x01CC;
pub const I2S_DMACR: u32 = 0x0200;
pub const I2S_DMAEN_RXBLOCK: u32 = 1 << 16;
pub const I2S_DMAEN_TXBLOCK: u32 = 1 << 17;

/*
 * struct i2s_clk_config_data - represent i2s clk configuration data
 * @chan_nr: number of channel
 * @data_width: number of bits per sample (8/16/24/32 bit)
 * @sample_rate: sampling frequency (8Khz, 16Khz, 48Khz)
 */
#[repr(C)]
pub struct i2s_clk_config_data {
    pub chan_nr: ::std::os::raw::c_int,
    pub data_width: u32,
    pub sample_rate: u32,
}

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct kmb_i2s_info {
    pub i2s_base: *mut ::std::os::raw::c_void,
    pub pss_base: *mut ::std::os::raw::c_void,
    pub clk_i2s: *mut clk,
    pub clk_apb: *mut clk,
    pub active: ::std::os::raw::c_int,
    pub capability: ::std::os::raw::c_uint,
    pub i2s_reg_comp1: ::std::os::raw::c_uint,
    pub i2s_reg_comp2: ::std::os::raw::c_uint,
    pub dev: *mut device,
    pub ccr: u32,
    pub xfer_resolution: u32,
    pub fifo_th: u32,
    pub clock_provider: bool,
    /* data related to DMA transfers b/w i2s and DMAC */
    pub play_dma_data: snd_dmaengine_dai_dma_data,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,

    pub config: i2s_clk_config_data,
    pub i2s_clk_cfg:
        Option<unsafe extern "C" fn(config: *mut i2s_clk_config_data) -> ::std::os::raw::c_int>,

    /* data related to PIO transfers */
    pub use_pio: bool,
    pub tx_substream: *mut snd_pcm_substream,
    pub rx_substream: *mut snd_pcm_substream,
    pub tx_ptr: ::std::os::raw::c_uint,
    pub rx_ptr: ::std::os::raw::c_uint,
    pub iec958_fmt: bool,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
