/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2019 NXP
 *
 */

/* Header guard _FSL_ASRC_COMMON_H omitted in Rust. */

/* directions */
pub const IN: u32 = 0;
pub const OUT: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum asrc_pair_index {
    ASRC_INVALID_PAIR = -1,
    ASRC_PAIR_A = 0,
    ASRC_PAIR_B = 1,
    ASRC_PAIR_C = 2,
    ASRC_PAIR_D = 3,
}

pub const PAIR_CTX_NUM: usize = 0x4;

/**
 * struct fsl_asrc_m2m_cap - capability data
 * @fmt_in: input sample format
 * @fmt_out: output sample format
 * @chan_min: minimum channel number
 * @chan_max: maximum channel number
 * @rate_in: minimum rate
 * @rate_out: maximum rete
 */
#[repr(C)]
pub struct fsl_asrc_m2m_cap {
    pub fmt_in: u64,
    pub fmt_out: u64,
    pub chan_min: i32,
    pub chan_max: i32,
    pub rate_in: *const u32,
    pub rate_in_count: i32,
    pub rate_out: *const u32,
    pub rate_out_count: i32,
}

/**
 * fsl_asrc_pair: ASRC Pair common data
 *
 * @asrc: pointer to its parent module
 * @error: error record
 * @index: pair index (ASRC_PAIR_A, ASRC_PAIR_B, ASRC_PAIR_C)
 * @channels: occupied channel number
 * @desc: input and output dma descriptors
 * @dma_chan: inputer and output DMA channels
 * @dma_data: private dma data
 * @pos: hardware pointer position
 * @req_dma_chan: flag to release dev_to_dev chan
 * @private: pair private area
 * @complete: dma task complete
 * @sample_format: format of m2m
 * @rate: rate of m2m
 * @buf_len: buffer length of m2m
 * @dma_buffer: buffer pointers
 * @first_convert: start of conversion
 * @ratio_mod_flag: flag for new ratio modifier
 * @ratio_mod: ratio modification
 */
#[repr(C)]
pub struct fsl_asrc_pair {
    pub asrc: *mut fsl_asrc,
    pub error: u32,

    pub index: asrc_pair_index,
    pub channels: u32,

    pub desc: [*mut dma_async_tx_descriptor; 2],
    pub dma_chan: [*mut dma_chan; 2],
    pub dma_data: imx_dma_data,
    pub pos: u32,
    pub req_dma_chan: bool,

    pub private: *mut core::ffi::c_void,

    /* used for m2m */
    pub complete: [completion; 2],
    pub sample_format: [snd_pcm_format_t; 2],
    pub rate: [u32; 2],
    pub buf_len: [u32; 2],
    pub dma_buffer: [snd_dma_buffer; 2],
    pub first_convert: u32,
    pub ratio_mod_flag: bool,
    pub ratio_mod: u32,
}

/**
 * fsl_asrc: ASRC common data
 *
 * @dma_params_rx: DMA parameters for receive channel
 * @dma_params_tx: DMA parameters for transmit channel
 * @pdev: platform device pointer
 * @regmap: regmap handler
 * @paddr: physical address to the base address of registers
 * @mem_clk: clock source to access register
 * @ipg_clk: clock source to drive peripheral
 * @spba_clk: SPBA clock (optional, depending on SoC design)
 * @card: compress sound card
 * @lock: spin lock for resource protection
 * @pair: pair pointers
 * @channel_avail: non-occupied channel numbers
 * @asrc_rate: default sample rate for ASoC Back-Ends
 * @asrc_format: default sample format for ASoC Back-Ends
 * @use_edma: edma is used
 * @start_before_dma: start asrc before dma
 * @get_dma_channel: function pointer
 * @request_pair: function pointer
 * @release_pair: function pointer
 * @get_fifo_addr: function pointer
 * @m2m_get_cap: function pointer
 * @m2m_prepare: function pointer
 * @m2m_start: function pointer
 * @m2m_unprepare: function pointer
 * @m2m_stop: function pointer
 * @m2m_output_ready: function pointer, check output fifo ready or not
 * @m2m_calc_out_len: function pointer
 * @m2m_get_maxburst: function pointer
 * @m2m_pair_suspend: function pointer
 * @m2m_pair_resume: function pointer
 * @m2m_set_ratio_mod: function pointer
 * @get_output_fifo_size: function pointer
 * @pair_priv_size: size of pair private struct.
 * @private: private data structure
 */
#[repr(C)]
pub struct fsl_asrc {
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub paddr: core::ffi::c_ulong,
    pub mem_clk: *mut clk,
    pub ipg_clk: *mut clk,
    pub spba_clk: *mut clk,
    pub card: *mut snd_card,
    pub lock: spinlock_t, /* spin lock for resource protection */

    pub pair: [*mut fsl_asrc_pair; PAIR_CTX_NUM],
    pub channel_avail: u32,

    pub asrc_rate: i32,
    pub asrc_format: snd_pcm_format_t,
    pub use_edma: bool,
    pub start_before_dma: bool,

    pub get_dma_channel:
        Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair, dir: bool) -> *mut dma_chan>,
    pub request_pair: Option<unsafe extern "C" fn(channels: i32, pair: *mut fsl_asrc_pair) -> i32>,
    pub release_pair: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair)>,
    pub get_fifo_addr: Option<unsafe extern "C" fn(dir: u8, index: asrc_pair_index) -> i32>,
    pub m2m_get_cap: Option<unsafe extern "C" fn(cap: *mut fsl_asrc_m2m_cap) -> i32>,

    pub m2m_prepare: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_start: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_unprepare: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_stop: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_output_ready: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> bool>,

    pub m2m_calc_out_len:
        Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair, input_buffer_length: i32) -> i32>,
    pub m2m_get_maxburst:
        Option<unsafe extern "C" fn(dir: u8, pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_pair_suspend: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_pair_resume: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> i32>,
    pub m2m_set_ratio_mod: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair, val: i32) -> i32>,

    pub get_output_fifo_size: Option<unsafe extern "C" fn(pair: *mut fsl_asrc_pair) -> u32>,
    pub pair_priv_size: usize,

    pub private: *mut core::ffi::c_void,
}

pub const DRV_NAME: &[u8; 13] = b"fsl-asrc-dai\0";

unsafe extern "C" {
    pub static mut fsl_asrc_component: snd_soc_component_driver;

    pub fn fsl_asrc_m2m_init(asrc: *mut fsl_asrc) -> i32;
    pub fn fsl_asrc_m2m_exit(asrc: *mut fsl_asrc);
    pub fn fsl_asrc_m2m_resume(asrc: *mut fsl_asrc) -> i32;
    pub fn fsl_asrc_m2m_suspend(asrc: *mut fsl_asrc) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
