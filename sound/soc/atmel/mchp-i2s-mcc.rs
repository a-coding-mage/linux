// SPDX-License-Identifier: GPL-2.0
//
// Driver for Microchip I2S Multi-channel controller
//
// Copyright (C) 2018 Microchip Technology Inc. and its subsidiaries
//
// Author: Codrin Ciubotariu <codrin.ciubotariu@microchip.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type dma_addr_t = c_ulong;
type bool_t = bool;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct wait_queue_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub maxburst: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_sample_bits: c_uint,
    pub symmetric_channels: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn init_waitqueue_head(wq: *mut wait_queue_head);
    fn wake_up_interruptible(wq: *mut wait_queue_head);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn wait_event_interruptible_timeout(wq: wait_queue_head, condition: c_uint, timeout: c_ulong) -> c_long;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_property_read_u8(np: *mut device_node, propname: *const c_char, out_value: *mut u8) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn lcm(a: c_ulong, b: c_ulong) -> c_ulong;
    fn fls(x: c_int) -> c_int;
}

macro_rules! dev_dbg {
    ($($arg:tt)*) => {};
}
macro_rules! dev_err {
    ($($arg:tt)*) => {};
}
macro_rules! dev_warn {
    ($($arg:tt)*) => {};
}
macro_rules! dev_warn_once {
    ($($arg:tt)*) => {};
}
macro_rules! dev_err_once {
    ($($arg:tt)*) => {};
}
macro_rules! dev_info {
    ($($arg:tt)*) => {};
}

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;

const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x000f_0000;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0000_f000;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0x0000_1000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0000_2000;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0x0000_3000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0000_4000;
const SND_SOC_DAIFMT_CONT: c_uint = 0x0001_0000;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x0000_00ff;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_DSP_A: c_uint = 3;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 1 << 2;
const SND_SOC_POSSIBLE_DAIFMT_GATED: u64 = 1 << 3;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 1 << 4;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S8: c_uint = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_uint = 2;
const SNDRV_PCM_FORMAT_S18_3LE: c_uint = 3;
const SNDRV_PCM_FORMAT_S20_3LE: c_uint = 4;
const SNDRV_PCM_FORMAT_S24_3LE: c_uint = 5;
const SNDRV_PCM_FORMAT_S24_LE: c_uint = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_uint = 10;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << SNDRV_PCM_FORMAT_S8;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S18_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S18_3LE;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_3LE;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << SNDRV_PCM_FORMAT_S32_LE;

const fn BIT(n: c_uint) -> u32 {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> u32 {
    if h == 31 && l == 0 {
        !0u32
    } else {
        ((!0u32) << l) & ((!0u32) >> (31 - h))
    }
}

/*
 * ---- I2S Controller Register map ----
 */
const MCHP_I2SMCC_CR: u32 = 0x0000; /* Control Register */
const MCHP_I2SMCC_MRA: u32 = 0x0004; /* Mode Register A */
const MCHP_I2SMCC_MRB: u32 = 0x0008; /* Mode Register B */
const MCHP_I2SMCC_SR: u32 = 0x000C; /* Status Register */
const MCHP_I2SMCC_IERA: u32 = 0x0010; /* Interrupt Enable Register A */
const MCHP_I2SMCC_IDRA: u32 = 0x0014; /* Interrupt Disable Register A */
const MCHP_I2SMCC_IMRA: u32 = 0x0018; /* Interrupt Mask Register A */
const MCHP_I2SMCC_ISRA: u32 = 0X001C; /* Interrupt Status Register A */
const MCHP_I2SMCC_IERB: u32 = 0x0020; /* Interrupt Enable Register B */
const MCHP_I2SMCC_IDRB: u32 = 0x0024; /* Interrupt Disable Register B */
const MCHP_I2SMCC_IMRB: u32 = 0x0028; /* Interrupt Mask Register B */
const MCHP_I2SMCC_ISRB: u32 = 0X002C; /* Interrupt Status Register B */
const MCHP_I2SMCC_RHR: u32 = 0x0030; /* Receiver Holding Register */
const MCHP_I2SMCC_THR: u32 = 0x0034; /* Transmitter Holding Register */
const MCHP_I2SMCC_RHL0R: u32 = 0x0040; /* Receiver Holding Left 0 Register */
const MCHP_I2SMCC_RHR0R: u32 = 0x0044; /* Receiver Holding Right 0 Register */
const MCHP_I2SMCC_RHL1R: u32 = 0x0048; /* Receiver Holding Left 1 Register */
const MCHP_I2SMCC_RHR1R: u32 = 0x004C; /* Receiver Holding Right 1 Register */
const MCHP_I2SMCC_RHL2R: u32 = 0x0050; /* Receiver Holding Left 2 Register */
const MCHP_I2SMCC_RHR2R: u32 = 0x0054; /* Receiver Holding Right 2 Register */
const MCHP_I2SMCC_RHL3R: u32 = 0x0058; /* Receiver Holding Left 3 Register */
const MCHP_I2SMCC_RHR3R: u32 = 0x005C; /* Receiver Holding Right 3 Register */
const MCHP_I2SMCC_THL0R: u32 = 0x0060; /* Transmitter Holding Left 0 Register */
const MCHP_I2SMCC_THR0R: u32 = 0x0064; /* Transmitter Holding Right 0 Register */
const MCHP_I2SMCC_THL1R: u32 = 0x0068; /* Transmitter Holding Left 1 Register */
const MCHP_I2SMCC_THR1R: u32 = 0x006C; /* Transmitter Holding Right 1 Register */
const MCHP_I2SMCC_THL2R: u32 = 0x0070; /* Transmitter Holding Left 2 Register */
const MCHP_I2SMCC_THR2R: u32 = 0x0074; /* Transmitter Holding Right 2 Register */
const MCHP_I2SMCC_THL3R: u32 = 0x0078; /* Transmitter Holding Left 3 Register */
const MCHP_I2SMCC_THR3R: u32 = 0x007C; /* Transmitter Holding Right 3 Register */
const MCHP_I2SMCC_VERSION: u32 = 0x00FC; /* Version Register */

const MCHP_I2SMCC_CR_RXEN: u32 = BIT(0);
const MCHP_I2SMCC_CR_RXDIS: u32 = BIT(1);
const MCHP_I2SMCC_CR_CKEN: u32 = BIT(2);
const MCHP_I2SMCC_CR_CKDIS: u32 = BIT(3);
const MCHP_I2SMCC_CR_TXEN: u32 = BIT(4);
const MCHP_I2SMCC_CR_TXDIS: u32 = BIT(5);
const MCHP_I2SMCC_CR_SWRST: u32 = BIT(7);

const MCHP_I2SMCC_MRA_MODE_MASK: u32 = GENMASK(0, 0);
const MCHP_I2SMCC_MRA_MODE_SLAVE: u32 = 0 << 0;
const MCHP_I2SMCC_MRA_MODE_MASTER: u32 = 1 << 0;
const MCHP_I2SMCC_MRA_DATALENGTH_MASK: u32 = GENMASK(3, 1);
const MCHP_I2SMCC_MRA_DATALENGTH_32_BITS: u32 = 0 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_24_BITS: u32 = 1 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_20_BITS: u32 = 2 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_18_BITS: u32 = 3 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_16_BITS: u32 = 4 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_16_BITS_COMPACT: u32 = 5 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_8_BITS: u32 = 6 << 1;
const MCHP_I2SMCC_MRA_DATALENGTH_8_BITS_COMPACT: u32 = 7 << 1;
const MCHP_I2SMCC_MRA_WIRECFG_MASK: u32 = GENMASK(5, 4);
const fn MCHP_I2SMCC_MRA_WIRECFG_TDM(pin: u32) -> u32 {
    (pin << 4) & MCHP_I2SMCC_MRA_WIRECFG_MASK
}
const MCHP_I2SMCC_MRA_WIRECFG_I2S_1_TDM_0: u32 = 0 << 4;
const MCHP_I2SMCC_MRA_WIRECFG_I2S_2_TDM_1: u32 = 1 << 4;
const MCHP_I2SMCC_MRA_WIRECFG_I2S_4_TDM_2: u32 = 2 << 4;
const MCHP_I2SMCC_MRA_WIRECFG_TDM_3: u32 = 3 << 4;
const MCHP_I2SMCC_MRA_FORMAT_MASK: u32 = GENMASK(7, 6);
const MCHP_I2SMCC_MRA_FORMAT_I2S: u32 = 0 << 6;
const MCHP_I2SMCC_MRA_FORMAT_LJ: u32 = 1 << 6; /* Left Justified */
const MCHP_I2SMCC_MRA_FORMAT_TDM: u32 = 2 << 6;
const MCHP_I2SMCC_MRA_FORMAT_TDMLJ: u32 = 3 << 6;
/* Transmitter uses one DMA channel ... */
/* Left audio samples duplicated to right audio channel */
const MCHP_I2SMCC_MRA_RXMONO: u32 = BIT(8);
/* I2SDO output of I2SC is internally connected to I2SDI input */
const MCHP_I2SMCC_MRA_RXLOOP: u32 = BIT(9);
/* Receiver uses one DMA channel ... */
/* Left audio samples duplicated to right audio channel */
const MCHP_I2SMCC_MRA_TXMONO: u32 = BIT(10);
/* x sample transmitted when underrun */
const MCHP_I2SMCC_MRA_TXSAME_ZERO: u32 = 0 << 11; /* Zero sample */
const MCHP_I2SMCC_MRA_TXSAME_PREVIOUS: u32 = 1 << 11; /* Previous sample */
/* select between peripheral clock and generated clock */
const MCHP_I2SMCC_MRA_SRCCLK_PCLK: u32 = 0 << 12;
const MCHP_I2SMCC_MRA_SRCCLK_GCLK: u32 = 1 << 12;
/* Number of TDM Channels - 1 */
const MCHP_I2SMCC_MRA_NBCHAN_MASK: u32 = GENMASK(15, 13);
const fn MCHP_I2SMCC_MRA_NBCHAN(ch: u32) -> u32 {
    (((ch - 1) << 13) & MCHP_I2SMCC_MRA_NBCHAN_MASK)
}
/* Selected Clock to I2SMCC Master Clock ratio */
const MCHP_I2SMCC_MRA_IMCKDIV_MASK: u32 = GENMASK(21, 16);
const fn MCHP_I2SMCC_MRA_IMCKDIV(div: u32) -> u32 {
    (div << 16) & MCHP_I2SMCC_MRA_IMCKDIV_MASK
}
/* TDM Frame Synchronization */
const MCHP_I2SMCC_MRA_TDMFS_MASK: u32 = GENMASK(23, 22);
const MCHP_I2SMCC_MRA_TDMFS_SLOT: u32 = 0 << 22;
const MCHP_I2SMCC_MRA_TDMFS_HALF: u32 = 1 << 22;
const MCHP_I2SMCC_MRA_TDMFS_BIT: u32 = 2 << 22;
/* Selected Clock to I2SMC Serial Clock ratio */
const MCHP_I2SMCC_MRA_ISCKDIV_MASK: u32 = GENMASK(29, 24);
const fn MCHP_I2SMCC_MRA_ISCKDIV(div: u32) -> u32 {
    (div << 24) & MCHP_I2SMCC_MRA_ISCKDIV_MASK
}
/* Master Clock mode */
const MCHP_I2SMCC_MRA_IMCKMODE_MASK: u32 = GENMASK(30, 30);
/* 0: No master clock generated*/
const MCHP_I2SMCC_MRA_IMCKMODE_NONE: u32 = 0 << 30;
/* 1: master clock generated (internally generated clock drives I2SMCK pin) */
const MCHP_I2SMCC_MRA_IMCKMODE_GEN: u32 = 1 << 30;
/* Slot Width */
/* 0: slot is 32 bits wide for DATALENGTH = 18/20/24 bits. */
/* 1: slot is 24 bits wide for DATALENGTH = 18/20/24 bits. */
const MCHP_I2SMCC_MRA_IWS: u32 = BIT(31);

/* all enabled I2S left channels are filled first, then I2S right channels */
const MCHP_I2SMCC_MRB_CRAMODE_LEFT_FIRST: u32 = 0 << 0;
/*
 * an enabled I2S left channel is filled, then the corresponding right
 * channel, until all channels are filled
 */
const MCHP_I2SMCC_MRB_CRAMODE_REGULAR: u32 = 1 << 0;
const MCHP_I2SMCC_MRB_FIFOEN: u32 = BIT(4);
const MCHP_I2SMCC_MRB_DMACHUNK_MASK: u32 = GENMASK(9, 8);
unsafe fn MCHP_I2SMCC_MRB_DMACHUNK(no_words: c_int) -> u32 {
    (((fls(no_words) - 1) as u32) << 8) & MCHP_I2SMCC_MRB_DMACHUNK_MASK
}
const MCHP_I2SMCC_MRB_CLKSEL_MASK: u32 = GENMASK(16, 16);
const MCHP_I2SMCC_MRB_CLKSEL_EXT: u32 = 0 << 16;
const MCHP_I2SMCC_MRB_CLKSEL_INT: u32 = 1 << 16;
const MCHP_I2SMCC_SR_RXEN: u32 = BIT(0);
const MCHP_I2SMCC_SR_TXEN: u32 = BIT(4);
const fn MCHP_I2SMCC_INT_TXRDY_MASK(ch: u32) -> u32 {
    GENMASK(ch - 1, 0)
}
const fn MCHP_I2SMCC_INT_TXRDYCH(ch: u32) -> u32 {
    BIT(ch)
}
const fn MCHP_I2SMCC_INT_TXUNF_MASK(ch: u32) -> u32 {
    GENMASK(ch + 7, 8)
}
const fn MCHP_I2SMCC_INT_TXUNFCH(ch: u32) -> u32 {
    BIT(ch + 8)
}
const fn MCHP_I2SMCC_INT_RXRDY_MASK(ch: u32) -> u32 {
    GENMASK(ch + 15, 16)
}
const fn MCHP_I2SMCC_INT_RXRDYCH(ch: u32) -> u32 {
    BIT(ch + 16)
}
const fn MCHP_I2SMCC_INT_RXOVF_MASK(ch: u32) -> u32 {
    GENMASK(ch + 23, 24)
}
const fn MCHP_I2SMCC_INT_RXOVFCH(ch: u32) -> u32 {
    BIT(ch + 24)
}
const MCHP_I2SMCC_INT_WERR: u32 = BIT(0);
const MCHP_I2SMCC_INT_TXFFRDY: u32 = BIT(8);
const MCHP_I2SMCC_INT_TXFFEMP: u32 = BIT(9);
const MCHP_I2SMCC_INT_RXFFRDY: u32 = BIT(12);
const MCHP_I2SMCC_INT_RXFFFUL: u32 = BIT(13);
const MCHP_I2SMCC_VERSION_MASK: u32 = GENMASK(11, 0);
const MCHP_I2SMCC_MAX_CHANNELS: c_int = 8;
const MCHP_I2MCC_TDM_SLOT_WIDTH: c_int = 32;
const MCHP_I2SMCC_DMA_8_WORD_CHUNK: c_int = 8;
const MCHP_I2SMCC_DMA_4_WORD_CHUNK: c_int = 4;
const MCHP_I2SMCC_DMA_2_WORD_CHUNK: c_int = 2;
const MCHP_I2SMCC_DMA_1_WORD_CHUNK: c_int = 1;

fn DMA_BURST_ALIGNED(p: c_int, s: c_int, w: c_int) -> bool {
    p % (s * w) == 0
}

static mchp_i2s_mcc_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: MCHP_I2SMCC_VERSION,
};

#[repr(C)]
struct mchp_i2s_mcc_soc_data {
    data_pin_pair_num: c_uint,
    has_fifo: bool,
}

#[repr(C)]
struct mchp_i2s_mcc_dev {
    wq_txrdy: wait_queue_head,
    wq_rxrdy: wait_queue_head,
    dev: *mut device,
    regmap: *mut regmap,
    pclk: *mut clk,
    gclk: *mut clk,
    soc: *const mchp_i2s_mcc_soc_data,
    playback: snd_dmaengine_dai_dma_data,
    capture: snd_dmaengine_dai_dma_data,
    fmt: c_uint,
    sysclk: c_uint,
    frame_length: c_uint,
    tdm_slots: c_int,
    channels: c_int,
    tdm_data_pair: u8,
    gclk_use: c_uint,
    gclk_running: c_uint,
    tx_rdy: c_uint,
    rx_rdy: c_uint,
}

unsafe extern "C" fn mchp_i2s_mcc_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dev = dev_id as *mut mchp_i2s_mcc_dev;
    let mut sra: u32 = 0;
    let mut imra: u32 = 0;
    let mut srb: u32 = 0;
    let mut imrb: u32 = 0;
    let mut idra: u32 = 0;
    let mut idrb: u32 = 0;
    let mut ret: irqreturn_t = IRQ_NONE;

    unsafe {
        regmap_read((*dev).regmap, MCHP_I2SMCC_IMRA, &mut imra);
        regmap_read((*dev).regmap, MCHP_I2SMCC_ISRA, &mut sra);
        let pendinga = imra & sra;
        regmap_read((*dev).regmap, MCHP_I2SMCC_IMRB, &mut imrb);
        regmap_read((*dev).regmap, MCHP_I2SMCC_ISRB, &mut srb);
        let pendingb = imrb & srb;

        if pendinga == 0 && pendingb == 0 {
            return IRQ_NONE;
        }

        /*
         * Tx/Rx ready interrupts are enabled when stopping only, to assure
         * availability and to disable clocks if necessary
         */
        if (*(*dev).soc).has_fifo {
            idrb |= pendingb & (MCHP_I2SMCC_INT_TXFFRDY | MCHP_I2SMCC_INT_RXFFRDY);
        } else {
            idra |= pendinga
                & (MCHP_I2SMCC_INT_TXRDY_MASK((*dev).channels as u32)
                    | MCHP_I2SMCC_INT_RXRDY_MASK((*dev).channels as u32));
        }
        if idra != 0 || idrb != 0 {
            ret = IRQ_HANDLED;
        }

        if ((!(*(*dev).soc).has_fifo
            && (imra & MCHP_I2SMCC_INT_TXRDY_MASK((*dev).channels as u32)) != 0
            && (imra & MCHP_I2SMCC_INT_TXRDY_MASK((*dev).channels as u32))
                == (idra & MCHP_I2SMCC_INT_TXRDY_MASK((*dev).channels as u32)))
            || ((*(*dev).soc).has_fifo && (imrb & MCHP_I2SMCC_INT_TXFFRDY) != 0)
        {
            (*dev).tx_rdy = 1;
            wake_up_interruptible(&mut (*dev).wq_txrdy);
        }
        if ((!(*(*dev).soc).has_fifo
            && (imra & MCHP_I2SMCC_INT_RXRDY_MASK((*dev).channels as u32)) != 0
            && (imra & MCHP_I2SMCC_INT_RXRDY_MASK((*dev).channels as u32))
                == (idra & MCHP_I2SMCC_INT_RXRDY_MASK((*dev).channels as u32)))
            || ((*(*dev).soc).has_fifo && (imrb & MCHP_I2SMCC_INT_RXFFRDY) != 0)
        {
            (*dev).rx_rdy = 1;
            wake_up_interruptible(&mut (*dev).wq_rxrdy);
        }
        if (*(*dev).soc).has_fifo {
            regmap_write((*dev).regmap, MCHP_I2SMCC_IDRB, idrb);
        } else {
            regmap_write((*dev).regmap, MCHP_I2SMCC_IDRA, idra);
        }
    }

    ret
}

unsafe extern "C" fn mchp_i2s_mcc_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    dev_dbg!((*dev).dev, "%s() clk_id=%d freq=%u dir=%d\n", "mchp_i2s_mcc_set_sysclk", clk_id, freq, dir);
    /* We do not need SYSCLK */
    if dir == SND_SOC_CLOCK_IN {
        return 0;
    }
    unsafe {
        (*dev).sysclk = freq;
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    dev_dbg!((*dev).dev, "%s() ratio=%u\n", "mchp_i2s_mcc_set_bclk_ratio", ratio);
    unsafe {
        (*dev).frame_length = ratio;
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    dev_dbg!((*dev).dev, "%s() fmt=%#x\n", "mchp_i2s_mcc_set_dai_fmt", fmt);
    /* We don't support any kind of clock inversion */
    if (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF {
        return -EINVAL;
    }
    /* We can't generate only FSYNC */
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BC_FP {
        return -EINVAL;
    }
    /* We can only reconfigure the IP when it's stopped */
    if (fmt & SND_SOC_DAIFMT_CONT) != 0 {
        return -EINVAL;
    }
    unsafe {
        (*dev).fmt = fmt;
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    dev_dbg!((*dev).dev, "%s() tx_mask=0x%08x rx_mask=0x%08x slots=%d width=%d\n", "mchp_i2s_mcc_set_dai_tdm_slot", tx_mask, rx_mask, slots, slot_width);
    if slots < 0 || slots > MCHP_I2SMCC_MAX_CHANNELS || slot_width != MCHP_I2MCC_TDM_SLOT_WIDTH {
        return -EINVAL;
    }
    if slots != 0 {
        /* We do not support daisy chain */
        if rx_mask != GENMASK((slots - 1) as u32, 0) || rx_mask != tx_mask {
            return -EINVAL;
        }
    }
    unsafe {
        (*dev).tdm_slots = slots;
        (*dev).frame_length = (slots * MCHP_I2MCC_TDM_SLOT_WIDTH) as c_uint;
    }
    0
}

unsafe fn mchp_i2s_mcc_clk_get_rate_diff(
    clk: *mut clk,
    rate: c_ulong,
    best_clk: *mut *mut clk,
    best_rate: *mut c_ulong,
    best_diff_rate: *mut c_ulong,
) -> c_int {
    let round_rate = unsafe { clk_round_rate(clk, rate) };
    if round_rate < 0 {
        return round_rate as c_int;
    }
    let diff_rate = if rate > round_rate as c_ulong {
        rate - round_rate as c_ulong
    } else {
        round_rate as c_ulong - rate
    };
    unsafe {
        if diff_rate < *best_diff_rate {
            *best_clk = clk;
            *best_diff_rate = diff_rate;
            *best_rate = rate;
        }
    }
    0
}

unsafe fn mchp_i2s_mcc_config_divs(dev: *mut mchp_i2s_mcc_dev, bclk: c_uint, mra: *mut u32, best_rate: *mut c_ulong) -> c_int {
    let mut clk_rate: c_ulong;
    let mut best_diff_rate: c_ulong = !0;
    let mut best_clk: *mut clk = ptr::null_mut();
    let sysclk: c_uint;
    let mut ret: c_int;

    unsafe {
        /* For code simplification */
        if (*dev).sysclk == 0 {
            sysclk = bclk;
        } else {
            sysclk = (*dev).sysclk;
        }

        /*
         * MCLK is Selected CLK / (2 * IMCKDIV),
         * BCLK is Selected CLK / (2 * ISCKDIV);
         * if IMCKDIV or ISCKDIV are 0, MCLK or BCLK = Selected CLK
         */
        let mut lcm_rate = lcm(sysclk as c_ulong, bclk as c_ulong);
        if ((lcm_rate / sysclk as c_ulong) % 2 == 1 && lcm_rate / sysclk as c_ulong > 2)
            || ((lcm_rate / bclk as c_ulong) % 2 == 1 && lcm_rate / bclk as c_ulong > 2)
        {
            lcm_rate *= 2;
        }

        clk_rate = lcm_rate;
        while (clk_rate == sysclk as c_ulong || clk_rate / (sysclk as c_ulong * 2) <= GENMASK(5, 0) as c_ulong)
            && (clk_rate == bclk as c_ulong || clk_rate / (bclk as c_ulong * 2) <= GENMASK(5, 0) as c_ulong)
        {
            ret = mchp_i2s_mcc_clk_get_rate_diff((*dev).gclk, clk_rate, &mut best_clk, best_rate, &mut best_diff_rate);
            if ret != 0 {
                dev_err!((*dev).dev, "gclk error for rate %lu: %d", clk_rate, ret);
            } else if best_diff_rate == 0 {
                dev_dbg!((*dev).dev, "found perfect rate on gclk: %lu\n", clk_rate);
                break;
            }

            ret = mchp_i2s_mcc_clk_get_rate_diff((*dev).pclk, clk_rate, &mut best_clk, best_rate, &mut best_diff_rate);
            if ret != 0 {
                dev_err!((*dev).dev, "pclk error for rate %lu: %d", clk_rate, ret);
            } else if best_diff_rate == 0 {
                dev_dbg!((*dev).dev, "found perfect rate on pclk: %lu\n", clk_rate);
                break;
            }
            clk_rate = clk_rate.wrapping_add(lcm_rate);
        }

        /* check if clocks returned only errors */
        if best_clk.is_null() {
            dev_err!((*dev).dev, "unable to change rate to clocks\n");
            return -EINVAL;
        }

        dev_dbg!((*dev).dev, "source CLK is %s with rate %lu, diff %lu\n", if best_clk == (*dev).pclk { "pclk" } else { "gclk" }, *best_rate, best_diff_rate);

        /* Configure divisors */
        if (*dev).sysclk != 0 {
            *mra |= MCHP_I2SMCC_MRA_IMCKDIV((*best_rate / (2 * sysclk as c_ulong)) as u32);
        }
        *mra |= MCHP_I2SMCC_MRA_ISCKDIV((*best_rate / (2 * bclk as c_ulong)) as u32);
        if best_clk == (*dev).gclk {
            *mra |= MCHP_I2SMCC_MRA_SRCCLK_GCLK;
        } else {
            *mra |= MCHP_I2SMCC_MRA_SRCCLK_PCLK;
        }
    }
    0
}

unsafe fn mchp_i2s_mcc_is_running(dev: *mut mchp_i2s_mcc_dev) -> c_int {
    let mut sr: u32 = 0;
    unsafe {
        regmap_read((*dev).regmap, MCHP_I2SMCC_SR, &mut sr);
    }
    ((sr & (MCHP_I2SMCC_SR_TXEN | MCHP_I2SMCC_SR_RXEN)) != 0) as c_int
}

fn mchp_i2s_mcc_period_to_maxburst(period_size: c_int, sample_size: c_int) -> c_int {
    let p_size = period_size;
    let s_size = sample_size;
    if DMA_BURST_ALIGNED(p_size, s_size, MCHP_I2SMCC_DMA_8_WORD_CHUNK) {
        return MCHP_I2SMCC_DMA_8_WORD_CHUNK;
    }
    if DMA_BURST_ALIGNED(p_size, s_size, MCHP_I2SMCC_DMA_4_WORD_CHUNK) {
        return MCHP_I2SMCC_DMA_4_WORD_CHUNK;
    }
    if DMA_BURST_ALIGNED(p_size, s_size, MCHP_I2SMCC_DMA_2_WORD_CHUNK) {
        return MCHP_I2SMCC_DMA_2_WORD_CHUNK;
    }
    MCHP_I2SMCC_DMA_1_WORD_CHUNK
}

unsafe extern "C" fn mchp_i2s_mcc_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let mut rate: c_ulong = 0;
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    let sample_bytes = unsafe { params_physical_width(params) / 8 };
    let period_bytes = unsafe { params_period_size(params) * params_channels(params) as c_int * sample_bytes };
    let mut mra: u32 = 0;
    let mut mrb: u32 = 0;
    let mut channels = unsafe { params_channels(params) };
    let mut frame_length = unsafe { (*dev).frame_length };
    let mut set_divs = 0;
    let mut ret: c_int;
    let is_playback = unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK };

    dev_dbg!((*dev).dev, "%s() rate=%u format=%#x width=%u channels=%u period_bytes=%d\n", "mchp_i2s_mcc_hw_params", params_rate(params), params_format(params), params_width(params), params_channels(params), period_bytes);

    unsafe {
        match (*dev).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {
                if (*dev).tdm_slots != 0 {
                    dev_err!((*dev).dev, "I2S with TDM is not supported\n");
                    return -EINVAL;
                }
                mra |= MCHP_I2SMCC_MRA_FORMAT_I2S;
            }
            SND_SOC_DAIFMT_LEFT_J => {
                if (*dev).tdm_slots != 0 {
                    dev_err!((*dev).dev, "Left-Justified with TDM is not supported\n");
                    return -EINVAL;
                }
                mra |= MCHP_I2SMCC_MRA_FORMAT_LJ;
            }
            SND_SOC_DAIFMT_DSP_A => mra |= MCHP_I2SMCC_MRA_FORMAT_TDM,
            _ => {
                dev_err!((*dev).dev, "unsupported bus format\n");
                return -EINVAL;
            }
        }

        match (*dev).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BP_FP => {
                /* cpu is BCLK and LRC master */
                mra |= MCHP_I2SMCC_MRA_MODE_MASTER;
                if (*dev).sysclk != 0 {
                    mra |= MCHP_I2SMCC_MRA_IMCKMODE_GEN;
                }
                set_divs = 1;
            }
            SND_SOC_DAIFMT_BP_FC => {
                /* cpu is BCLK master */
                mrb |= MCHP_I2SMCC_MRB_CLKSEL_INT;
                set_divs = 1;
                /* fallthrough */
                mra |= MCHP_I2SMCC_MRA_MODE_SLAVE;
                if (*dev).sysclk != 0 {
                    dev_warn!((*dev).dev, "Unable to generate MCLK in Slave mode\n");
                }
            }
            SND_SOC_DAIFMT_BC_FC => {
                /* cpu is slave */
                mra |= MCHP_I2SMCC_MRA_MODE_SLAVE;
                if (*dev).sysclk != 0 {
                    dev_warn!((*dev).dev, "Unable to generate MCLK in Slave mode\n");
                }
            }
            _ => {
                dev_err!((*dev).dev, "unsupported master/slave mode\n");
                return -EINVAL;
            }
        }

        if ((*dev).fmt & (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J)) != 0 {
            /* for I2S and LEFT_J one pin is needed for every 2 channels */
            if channels > (*(*dev).soc).data_pin_pair_num * 2 {
                dev_err!((*dev).dev, "unsupported number of audio channels: %d\n", channels);
                return -EINVAL;
            }
            /* enable for interleaved format */
            mrb |= MCHP_I2SMCC_MRB_CRAMODE_REGULAR;
            match channels {
                1 => {
                    if is_playback {
                        mra |= MCHP_I2SMCC_MRA_TXMONO;
                    } else {
                        mra |= MCHP_I2SMCC_MRA_RXMONO;
                    }
                }
                2 => {}
                4 => mra |= MCHP_I2SMCC_MRA_WIRECFG_I2S_2_TDM_1,
                8 => mra |= MCHP_I2SMCC_MRA_WIRECFG_I2S_4_TDM_2,
                _ => {
                    dev_err!((*dev).dev, "unsupported number of audio channels\n");
                    return -EINVAL;
                }
            }
            if frame_length == 0 {
                frame_length = (2 * params_physical_width(params)) as c_uint;
            }
        } else if ((*dev).fmt & SND_SOC_DAIFMT_DSP_A) != 0 {
            mra |= MCHP_I2SMCC_MRA_WIRECFG_TDM((*dev).tdm_data_pair as u32);
            if (*dev).tdm_slots != 0 {
                if channels % 2 != 0 && channels * 2 <= (*dev).tdm_slots as c_uint {
                    /*
                     * Duplicate data for even-numbered channels
                     * to odd-numbered channels
                     */
                    if is_playback {
                        mra |= MCHP_I2SMCC_MRA_TXMONO;
                    } else {
                        mra |= MCHP_I2SMCC_MRA_RXMONO;
                    }
                }
                channels = (*dev).tdm_slots as c_uint;
            }
            mra |= MCHP_I2SMCC_MRA_NBCHAN(channels);
            if frame_length == 0 {
                frame_length = channels * MCHP_I2MCC_TDM_SLOT_WIDTH as c_uint;
            }
        }

        /*
         * We must have the same burst size configured
         * in the DMA transfer and in out IP
         */
        let maxburst = mchp_i2s_mcc_period_to_maxburst(period_bytes, sample_bytes);
        mrb |= MCHP_I2SMCC_MRB_DMACHUNK(maxburst);
        if is_playback {
            (*dev).playback.maxburst = maxburst;
        } else {
            (*dev).capture.maxburst = maxburst;
        }

        match params_format(params) {
            SNDRV_PCM_FORMAT_S8 => mra |= MCHP_I2SMCC_MRA_DATALENGTH_8_BITS,
            SNDRV_PCM_FORMAT_S16_LE => mra |= MCHP_I2SMCC_MRA_DATALENGTH_16_BITS,
            SNDRV_PCM_FORMAT_S18_3LE => mra |= MCHP_I2SMCC_MRA_DATALENGTH_18_BITS | MCHP_I2SMCC_MRA_IWS,
            SNDRV_PCM_FORMAT_S20_3LE => mra |= MCHP_I2SMCC_MRA_DATALENGTH_20_BITS | MCHP_I2SMCC_MRA_IWS,
            SNDRV_PCM_FORMAT_S24_3LE => mra |= MCHP_I2SMCC_MRA_DATALENGTH_24_BITS | MCHP_I2SMCC_MRA_IWS,
            SNDRV_PCM_FORMAT_S24_LE => mra |= MCHP_I2SMCC_MRA_DATALENGTH_24_BITS,
            SNDRV_PCM_FORMAT_S32_LE => mra |= MCHP_I2SMCC_MRA_DATALENGTH_32_BITS,
            _ => {
                dev_err!((*dev).dev, "unsupported size/endianness for audio samples\n");
                return -EINVAL;
            }
        }

        if set_divs != 0 {
            let bclk_rate = frame_length * params_rate(params);
            ret = mchp_i2s_mcc_config_divs(dev, bclk_rate, &mut mra, &mut rate);
            if ret != 0 {
                dev_err!((*dev).dev, "unable to configure the divisors: %d\n", ret);
                return ret;
            }
        }

        /* enable FIFO if available */
        if (*(*dev).soc).has_fifo {
            mrb |= MCHP_I2SMCC_MRB_FIFOEN;
        }

        /*
         * If we are already running, the wanted setup must be
         * the same with the one that's currently ongoing
         */
        if mchp_i2s_mcc_is_running(dev) != 0 {
            let mut mra_cur: u32 = 0;
            let mut mrb_cur: u32 = 0;
            regmap_read((*dev).regmap, MCHP_I2SMCC_MRA, &mut mra_cur);
            regmap_read((*dev).regmap, MCHP_I2SMCC_MRB, &mut mrb_cur);
            if mra != mra_cur || mrb != mrb_cur {
                return -EINVAL;
            }
            return 0;
        }

        if (mra & MCHP_I2SMCC_MRA_SRCCLK_GCLK) != 0 && (*dev).gclk_use == 0 {
            /* set the rate */
            ret = clk_set_rate((*dev).gclk, rate);
            if ret != 0 {
                dev_err!((*dev).dev, "unable to set rate %lu to GCLK: %d\n", rate, ret);
                return ret;
            }
            ret = clk_prepare((*dev).gclk);
            if ret < 0 {
                dev_err!((*dev).dev, "unable to prepare GCLK: %d\n", ret);
                return ret;
            }
            (*dev).gclk_use = 1;
        }

        /* Save the number of channels to know what interrupts to enable */
        (*dev).channels = channels as c_int;
        ret = regmap_write((*dev).regmap, MCHP_I2SMCC_MRA, mra);
        if ret < 0 {
            if (*dev).gclk_use != 0 {
                clk_unprepare((*dev).gclk);
                (*dev).gclk_use = 0;
            }
            return ret;
        }
        regmap_write((*dev).regmap, MCHP_I2SMCC_MRB, mrb)
    }
}

unsafe extern "C" fn mchp_i2s_mcc_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    let is_playback = unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK };
    let err: c_long;
    unsafe {
        if is_playback {
            err = wait_event_interruptible_timeout((*dev).wq_txrdy, (*dev).tx_rdy, msecs_to_jiffies(500));
            if err == 0 {
                dev_warn_once!((*dev).dev, "Timeout waiting for Tx ready\n");
                if (*(*dev).soc).has_fifo {
                    regmap_write((*dev).regmap, MCHP_I2SMCC_IDRB, MCHP_I2SMCC_INT_TXFFRDY);
                } else {
                    regmap_write((*dev).regmap, MCHP_I2SMCC_IDRA, MCHP_I2SMCC_INT_TXRDY_MASK((*dev).channels as u32));
                }
                (*dev).tx_rdy = 1;
            }
        } else {
            err = wait_event_interruptible_timeout((*dev).wq_rxrdy, (*dev).rx_rdy, msecs_to_jiffies(500));
            if err == 0 {
                dev_warn_once!((*dev).dev, "Timeout waiting for Rx ready\n");
                if (*(*dev).soc).has_fifo {
                    regmap_write((*dev).regmap, MCHP_I2SMCC_IDRB, MCHP_I2SMCC_INT_RXFFRDY);
                } else {
                    regmap_write((*dev).regmap, MCHP_I2SMCC_IDRA, MCHP_I2SMCC_INT_RXRDY_MASK((*dev).channels as u32));
                }
                (*dev).rx_rdy = 1;
            }
        }

        if mchp_i2s_mcc_is_running(dev) == 0 {
            regmap_write((*dev).regmap, MCHP_I2SMCC_CR, MCHP_I2SMCC_CR_CKDIS);
            if (*dev).gclk_running != 0 {
                clk_disable((*dev).gclk);
                (*dev).gclk_running = 0;
            }
            if (*dev).gclk_use != 0 {
                clk_unprepare((*dev).gclk);
                (*dev).gclk_use = 0;
            }
        }
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    let is_playback = unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK };
    let mut cr: u32 = 0;
    let mut iera: u32 = 0;
    let mut ierb: u32 = 0;
    let mut sr: u32 = 0;
    let err: c_int;

    unsafe {
        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                if is_playback {
                    cr = MCHP_I2SMCC_CR_TXEN | MCHP_I2SMCC_CR_CKEN;
                } else {
                    cr = MCHP_I2SMCC_CR_RXEN | MCHP_I2SMCC_CR_CKEN;
                }
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                regmap_read((*dev).regmap, MCHP_I2SMCC_SR, &mut sr);
                if is_playback && (sr & MCHP_I2SMCC_SR_TXEN) != 0 {
                    cr = MCHP_I2SMCC_CR_TXDIS;
                    (*dev).tx_rdy = 0;
                    /*
                     * Enable Tx Ready interrupts on all channels
                     * to assure all data is sent
                     */
                    if (*(*dev).soc).has_fifo {
                        ierb = MCHP_I2SMCC_INT_TXFFRDY;
                    } else {
                        iera = MCHP_I2SMCC_INT_TXRDY_MASK((*dev).channels as u32);
                    }
                } else if !is_playback && (sr & MCHP_I2SMCC_SR_RXEN) != 0 {
                    cr = MCHP_I2SMCC_CR_RXDIS;
                    (*dev).rx_rdy = 0;
                    /*
                     * Enable Rx Ready interrupts on all channels
                     * to assure all data is received
                     */
                    if (*(*dev).soc).has_fifo {
                        ierb = MCHP_I2SMCC_INT_RXFFRDY;
                    } else {
                        iera = MCHP_I2SMCC_INT_RXRDY_MASK((*dev).channels as u32);
                    }
                }
            }
            _ => return -EINVAL,
        }

        if (cr & MCHP_I2SMCC_CR_CKEN) != 0 && (*dev).gclk_use != 0 && (*dev).gclk_running == 0 {
            err = clk_enable((*dev).gclk);
            if err != 0 {
                dev_err_once!((*dev).dev, "failed to enable GCLK: %d\n", err);
            } else {
                (*dev).gclk_running = 1;
            }
        }

        if (*(*dev).soc).has_fifo {
            regmap_write((*dev).regmap, MCHP_I2SMCC_IERB, ierb);
        } else {
            regmap_write((*dev).regmap, MCHP_I2SMCC_IERA, iera);
        }
        regmap_write((*dev).regmap, MCHP_I2SMCC_CR, cr);
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_startup(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    /* Software reset the IP if it's not running */
    unsafe {
        if mchp_i2s_mcc_is_running(dev) == 0 {
            return regmap_write((*dev).regmap, MCHP_I2SMCC_CR, MCHP_I2SMCC_CR_SWRST);
        }
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mchp_i2s_mcc_dev };
    unsafe {
        init_waitqueue_head(&mut (*dev).wq_txrdy);
        init_waitqueue_head(&mut (*dev).wq_rxrdy);
        (*dev).tx_rdy = 1;
        (*dev).rx_rdy = 1;
        snd_soc_dai_init_dma_data(dai, &mut (*dev).playback, &mut (*dev).capture);
    }
    0
}

static mchp_i2s_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
        | SND_SOC_POSSIBLE_DAIFMT_DSP_A
        | SND_SOC_POSSIBLE_DAIFMT_GATED
        | SND_SOC_POSSIBLE_DAIFMT_NB_NF;

static mchp_i2s_mcc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(mchp_i2s_mcc_dai_probe),
    set_sysclk: Some(mchp_i2s_mcc_set_sysclk),
    set_bclk_ratio: Some(mchp_i2s_mcc_set_bclk_ratio),
    startup: Some(mchp_i2s_mcc_startup),
    trigger: Some(mchp_i2s_mcc_trigger),
    hw_params: Some(mchp_i2s_mcc_hw_params),
    hw_free: Some(mchp_i2s_mcc_hw_free),
    set_fmt: Some(mchp_i2s_mcc_set_dai_fmt),
    set_tdm_slot: Some(mchp_i2s_mcc_set_dai_tdm_slot),
    auto_selectable_formats: &mchp_i2s_selectable_formats,
    num_auto_selectable_formats: 1,
};

const MCHP_I2SMCC_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const MCHP_I2SMCC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static mut mchp_i2s_mcc_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: MCHP_I2SMCC_RATES,
        formats: MCHP_I2SMCC_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: MCHP_I2SMCC_RATES,
        formats: MCHP_I2SMCC_FORMATS,
    },
    ops: &mchp_i2s_mcc_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
    symmetric_channels: 1,
};

static mchp_i2s_mcc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"mchp-i2s-mcc\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

/* CONFIG_OF */
static mut mchp_i2s_mcc_sam9x60: mchp_i2s_mcc_soc_data = mchp_i2s_mcc_soc_data {
    data_pin_pair_num: 1,
    has_fifo: false,
};

static mut mchp_i2s_mcc_sama7g5: mchp_i2s_mcc_soc_data = mchp_i2s_mcc_soc_data {
    data_pin_pair_num: 4,
    has_fifo: true,
};

static mchp_i2s_mcc_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: b"microchip,sam9x60-i2smcc\0".as_ptr() as *const c_char,
        data: unsafe { &mchp_i2s_mcc_sam9x60 as *const _ as *const c_void },
    },
    of_device_id {
        compatible: b"microchip,sama7g5-i2smcc\0".as_ptr() as *const c_char,
        data: unsafe { &mchp_i2s_mcc_sama7g5 as *const _ as *const c_void },
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, mchp_i2s_mcc_dt_ids); */

unsafe fn mchp_i2s_mcc_soc_data_parse(pdev: *mut platform_device, dev: *mut mchp_i2s_mcc_dev) -> c_int {
    let mut err: c_int;
    unsafe {
        if (*dev).soc.is_null() {
            dev_err!(&mut (*pdev).dev, "failed to get soc data\n");
            return -ENODEV;
        }
        if (*(*dev).soc).data_pin_pair_num == 1 {
            return 0;
        }
        err = of_property_read_u8((*pdev).dev.of_node, b"microchip,tdm-data-pair\0".as_ptr() as *const c_char, &mut (*dev).tdm_data_pair);
        if err < 0 && err != -EINVAL {
            dev_err!(&mut (*pdev).dev, "bad property data for 'microchip,tdm-data-pair': %d", err);
            return err;
        }
        if err == -EINVAL {
            dev_info!(&mut (*pdev).dev, "'microchip,tdm-data-pair' not found; assuming DIN/DOUT 0 for TDM\n");
            (*dev).tdm_data_pair = 0;
        } else {
            if (*dev).tdm_data_pair as c_uint > (*(*dev).soc).data_pin_pair_num - 1 {
                dev_err!(&mut (*pdev).dev, "invalid value for 'microchip,tdm-data-pair': %d\n", (*dev).tdm_data_pair);
                return -EINVAL;
            }
            dev_dbg!(&mut (*pdev).dev, "TMD format on DIN/DOUT %d pins\n", (*dev).tdm_data_pair);
        }
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut mchp_i2s_mcc_dev;
    let mut mem: *mut resource = ptr::null_mut();
    let regmap: *mut regmap;
    let base: *mut c_void;
    let mut version: u32 = 0;
    let irq: c_int;
    let mut err: c_int;

    unsafe {
        dev = devm_kzalloc(&mut (*pdev).dev, size_of::<mchp_i2s_mcc_dev>(), GFP_KERNEL) as *mut mchp_i2s_mcc_dev;
        if dev.is_null() {
            return -ENOMEM;
        }
        base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
        if IS_ERR(base) {
            return PTR_ERR(base) as c_int;
        }
        regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &mchp_i2s_mcc_regmap_config);
        if IS_ERR(regmap as *const c_void) {
            return PTR_ERR(regmap as *const c_void) as c_int;
        }
        irq = platform_get_irq(pdev, 0);
        if irq < 0 {
            return irq;
        }
        err = devm_request_irq(&mut (*pdev).dev, irq, mchp_i2s_mcc_interrupt, 0, dev_name(&mut (*pdev).dev), dev as *mut c_void);
        if err != 0 {
            return err;
        }
        (*dev).pclk = devm_clk_get(&mut (*pdev).dev, b"pclk\0".as_ptr() as *const c_char);
        if IS_ERR((*dev).pclk as *const c_void) {
            err = PTR_ERR((*dev).pclk as *const c_void) as c_int;
            dev_err!(&mut (*pdev).dev, "failed to get the peripheral clock: %d\n", err);
            return err;
        }
        /* Get the optional generated clock */
        (*dev).gclk = devm_clk_get(&mut (*pdev).dev, b"gclk\0".as_ptr() as *const c_char);
        if IS_ERR((*dev).gclk as *const c_void) {
            if PTR_ERR((*dev).gclk as *const c_void) as c_int == -EPROBE_DEFER {
                return -EPROBE_DEFER;
            }
            dev_warn!(&mut (*pdev).dev, "generated clock not found: %d\n", err);
            (*dev).gclk = ptr::null_mut();
        }
        (*dev).soc = of_device_get_match_data(&mut (*pdev).dev) as *const mchp_i2s_mcc_soc_data;
        err = mchp_i2s_mcc_soc_data_parse(pdev, dev);
        if err < 0 {
            return err;
        }
        (*dev).dev = &mut (*pdev).dev;
        (*dev).regmap = regmap;
        platform_set_drvdata(pdev, dev as *mut c_void);
        err = clk_prepare_enable((*dev).pclk);
        if err != 0 {
            dev_err!(&mut (*pdev).dev, "failed to enable the peripheral clock: %d\n", err);
            return err;
        }
        err = devm_snd_soc_register_component(&mut (*pdev).dev, &mchp_i2s_mcc_component, &raw mut mchp_i2s_mcc_dai, 1);
        if err != 0 {
            dev_err!(&mut (*pdev).dev, "failed to register DAI: %d\n", err);
            clk_disable_unprepare((*dev).pclk);
            return err;
        }
        (*dev).playback.addr = (*mem).start + MCHP_I2SMCC_THR as dma_addr_t;
        (*dev).capture.addr = (*mem).start + MCHP_I2SMCC_RHR as dma_addr_t;
        err = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
        if err != 0 {
            dev_err!(&mut (*pdev).dev, "failed to register PCM: %d\n", err);
            clk_disable_unprepare((*dev).pclk);
            return err;
        }
        /* Get IP version. */
        regmap_read((*dev).regmap, MCHP_I2SMCC_VERSION, &mut version);
        dev_info!(&mut (*pdev).dev, "hw version: %#lx\n", version & MCHP_I2SMCC_VERSION_MASK);
    }
    0
}

unsafe extern "C" fn mchp_i2s_mcc_remove(pdev: *mut platform_device) {
    let dev = unsafe { platform_get_drvdata(pdev) as *mut mchp_i2s_mcc_dev };
    unsafe {
        clk_disable_unprepare((*dev).pclk);
    }
}

static mut mchp_i2s_mcc_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: b"mchp_i2s_mcc\0".as_ptr() as *const c_char,
        of_match_table: mchp_i2s_mcc_dt_ids.as_ptr(),
    },
    probe: Some(mchp_i2s_mcc_probe),
    remove: Some(mchp_i2s_mcc_remove),
};

/* module_platform_driver(mchp_i2s_mcc_driver); */
/* MODULE_DESCRIPTION("Microchip I2S Multi-Channel Controller driver"); */
/* MODULE_AUTHOR("Codrin Ciubotariu <codrin.ciubotariu@microchip.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
