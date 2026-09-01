// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for Atmel I2S controller
 *
 * Copyright (C) 2015 Atmel Corporation
 *
 * Author: Cyrille Pitchen <cyrille.pitchen@atmel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_void = core::ffi::c_void;
type c_char = core::ffi::c_char;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;
type c_ulong = core::ffi::c_ulong;
type u64_t = u64;
type dma_addr_t = u64;
type irqreturn_t = c_int;
type bool_t = bool;

#[repr(C)]
pub struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
    stream: c_int,
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
pub struct snd_dmaengine_dai_dma_data {
    addr: dma_addr_t,
    maxburst: c_uint,
}

#[repr(C)]
pub struct resource {
    start: u64,
}

#[repr(C)]
pub struct platform_device {
    dev: device_with_of_node,
}

#[repr(C)]
pub struct device_with_of_node {
    of_node: *mut device_node,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    auto_selectable_formats: *const u64_t,
    num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64_t,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct platform_driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    driver: platform_driver_inner,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    let high = if h == 31 { !0u32 } else { (1u32 << (h + 1)) - 1 };
    let low = if l == 0 { 0u32 } else { (1u32 << l) - 1 };
    high & !low
}

const ATMEL_I2SC_MAX_TDM_CHANNELS: c_uint = 8;

/*
 * ---- I2S Controller Register map ----
 */
const ATMEL_I2SC_CR: c_uint = 0x0000; /* Control Register */
const ATMEL_I2SC_MR: c_uint = 0x0004; /* Mode Register */
const ATMEL_I2SC_SR: c_uint = 0x0008; /* Status Register */
const ATMEL_I2SC_SCR: c_uint = 0x000c; /* Status Clear Register */
const ATMEL_I2SC_SSR: c_uint = 0x0010; /* Status Set Register */
const ATMEL_I2SC_IER: c_uint = 0x0014; /* Interrupt Enable Register */
const ATMEL_I2SC_IDR: c_uint = 0x0018; /* Interrupt Disable Register */
const ATMEL_I2SC_IMR: c_uint = 0x001c; /* Interrupt Mask Register */
const ATMEL_I2SC_RHR: c_uint = 0x0020; /* Receiver Holding Register */
const ATMEL_I2SC_THR: c_uint = 0x0024; /* Transmitter Holding Register */
const ATMEL_I2SC_VERSION: c_uint = 0x0028; /* Version Register */

/*
 * ---- Control Register (Write-only) ----
 */
const ATMEL_I2SC_CR_RXEN: c_uint = BIT(0); /* Receiver Enable */
const ATMEL_I2SC_CR_RXDIS: c_uint = BIT(1); /* Receiver Disable */
const ATMEL_I2SC_CR_CKEN: c_uint = BIT(2); /* Clock Enable */
const ATMEL_I2SC_CR_CKDIS: c_uint = BIT(3); /* Clock Disable */
const ATMEL_I2SC_CR_TXEN: c_uint = BIT(4); /* Transmitter Enable */
const ATMEL_I2SC_CR_TXDIS: c_uint = BIT(5); /* Transmitter Disable */
const ATMEL_I2SC_CR_SWRST: c_uint = BIT(7); /* Software Reset */

/*
 * ---- Mode Register (Read/Write) ----
 */
const ATMEL_I2SC_MR_MODE_MASK: c_uint = GENMASK(0, 0);
const ATMEL_I2SC_MR_MODE_SLAVE: c_uint = 0 << 0;
const ATMEL_I2SC_MR_MODE_MASTER: c_uint = 1 << 0;

const ATMEL_I2SC_MR_DATALENGTH_MASK: c_uint = GENMASK(4, 2);
const ATMEL_I2SC_MR_DATALENGTH_32_BITS: c_uint = 0 << 2;
const ATMEL_I2SC_MR_DATALENGTH_24_BITS: c_uint = 1 << 2;
const ATMEL_I2SC_MR_DATALENGTH_20_BITS: c_uint = 2 << 2;
const ATMEL_I2SC_MR_DATALENGTH_18_BITS: c_uint = 3 << 2;
const ATMEL_I2SC_MR_DATALENGTH_16_BITS: c_uint = 4 << 2;
const ATMEL_I2SC_MR_DATALENGTH_16_BITS_COMPACT: c_uint = 5 << 2;
const ATMEL_I2SC_MR_DATALENGTH_8_BITS: c_uint = 6 << 2;
const ATMEL_I2SC_MR_DATALENGTH_8_BITS_COMPACT: c_uint = 7 << 2;

const ATMEL_I2SC_MR_FORMAT_MASK: c_uint = GENMASK(7, 6);
const ATMEL_I2SC_MR_FORMAT_I2S: c_uint = 0 << 6;
const ATMEL_I2SC_MR_FORMAT_LJ: c_uint = 1 << 6; /* Left Justified */
const ATMEL_I2SC_MR_FORMAT_TDM: c_uint = 2 << 6;
const ATMEL_I2SC_MR_FORMAT_TDMLJ: c_uint = 3 << 6;

/* Left audio samples duplicated to right audio channel */
const ATMEL_I2SC_MR_RXMONO: c_uint = BIT(8);

/* Receiver uses one DMA channel ... */
const ATMEL_I2SC_MR_RXDMA_MASK: c_uint = GENMASK(9, 9);
const ATMEL_I2SC_MR_RXDMA_SINGLE: c_uint = 0 << 9; /* for all audio channels */
const ATMEL_I2SC_MR_RXDMA_MULTIPLE: c_uint = 1 << 9; /* per audio channel */

/* I2SDO output of I2SC is internally connected to I2SDI input */
const ATMEL_I2SC_MR_RXLOOP: c_uint = BIT(10);

/* Left audio samples duplicated to right audio channel */
const ATMEL_I2SC_MR_TXMONO: c_uint = BIT(12);

/* Transmitter uses one DMA channel ... */
const ATMEL_I2SC_MR_TXDMA_MASK: c_uint = GENMASK(13, 13);
const ATMEL_I2SC_MR_TXDMA_SINGLE: c_uint = 0 << 13; /* for all audio channels */
const ATMEL_I2SC_MR_TXDME_MULTIPLE: c_uint = 1 << 13; /* per audio channel */

/* x sample transmitted when underrun */
const ATMEL_I2SC_MR_TXSAME_MASK: c_uint = GENMASK(14, 14);
const ATMEL_I2SC_MR_TXSAME_ZERO: c_uint = 0 << 14; /* Zero sample */
const ATMEL_I2SC_MR_TXSAME_PREVIOUS: c_uint = 1 << 14; /* Previous sample */

/* Audio Clock to I2SC Master Clock ratio */
const ATMEL_I2SC_MR_IMCKDIV_MASK: c_uint = GENMASK(21, 16);
fn ATMEL_I2SC_MR_IMCKDIV(div: c_int) -> c_uint {
    (((div as c_uint) << 16) & ATMEL_I2SC_MR_IMCKDIV_MASK) as c_uint
}

/* Master Clock to fs ratio */
const ATMEL_I2SC_MR_IMCKFS_MASK: c_uint = GENMASK(29, 24);
fn ATMEL_I2SC_MR_IMCKFS(fs: c_int) -> c_uint {
    (((fs as c_uint) << 24) & ATMEL_I2SC_MR_IMCKFS_MASK) as c_uint
}

/* Master Clock mode */
const ATMEL_I2SC_MR_IMCKMODE_MASK: c_uint = GENMASK(30, 30);
/* 0: No master clock generated (selected clock drives I2SCK pin) */
const ATMEL_I2SC_MR_IMCKMODE_I2SCK: c_uint = 0 << 30;
/* 1: master clock generated (internally generated clock drives I2SMCK pin) */
const ATMEL_I2SC_MR_IMCKMODE_I2SMCK: c_uint = 1 << 30;

/* Slot Width */
/* 0: slot is 32 bits wide for DATALENGTH = 18/20/24 bits. */
/* 1: slot is 24 bits wide for DATALENGTH = 18/20/24 bits. */
const ATMEL_I2SC_MR_IWS: c_uint = BIT(31);

/*
 * ---- Status Registers ----
 */
const ATMEL_I2SC_SR_RXEN: c_uint = BIT(0); /* Receiver Enabled */
const ATMEL_I2SC_SR_RXRDY: c_uint = BIT(1); /* Receive Ready */
const ATMEL_I2SC_SR_RXOR: c_uint = BIT(2); /* Receive Overrun */

const ATMEL_I2SC_SR_TXEN: c_uint = BIT(4); /* Transmitter Enabled */
const ATMEL_I2SC_SR_TXRDY: c_uint = BIT(5); /* Transmit Ready */
const ATMEL_I2SC_SR_TXUR: c_uint = BIT(6); /* Transmit Underrun */

/* Receive Overrun Channel */
const ATMEL_I2SC_SR_RXORCH_MASK: c_uint = GENMASK(15, 8);
fn ATMEL_I2SC_SR_RXORCH(ch: c_uint) -> c_uint {
    1 << (((ch) & 0x7) + 8)
}

/* Transmit Underrun Channel */
const ATMEL_I2SC_SR_TXURCH_MASK: c_uint = GENMASK(27, 20);
fn ATMEL_I2SC_SR_TXURCH(ch: c_uint) -> c_uint {
    1 << (((ch) & 0x7) + 20)
}

/*
 * ---- Interrupt Enable/Disable/Mask Registers ----
 */
const ATMEL_I2SC_INT_RXRDY: c_uint = ATMEL_I2SC_SR_RXRDY;
const ATMEL_I2SC_INT_RXOR: c_uint = ATMEL_I2SC_SR_RXOR;
const ATMEL_I2SC_INT_TXRDY: c_uint = ATMEL_I2SC_SR_TXRDY;
const ATMEL_I2SC_INT_TXUR: c_uint = ATMEL_I2SC_SR_TXUR;

static atmel_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: ATMEL_I2SC_VERSION,
};

#[repr(C)]
struct atmel_i2s_gck_param {
    fs: c_int,
    mck: c_ulong,
    imckdiv: c_int,
    imckfs: c_int,
}

const I2S_MCK_12M288: c_ulong = 12288000;
const I2S_MCK_11M2896: c_ulong = 11289600;
const I2S_MCK_6M144: c_ulong = 6144000;

/* mck = (32 * (imckfs+1) / (imckdiv+1)) * fs */
static gck_params: [atmel_i2s_gck_param; 13] = [
    /* mck = 6.144Mhz */
    atmel_i2s_gck_param { fs: 8000, mck: I2S_MCK_6M144, imckdiv: 1, imckfs: 47 }, /* mck =  768 fs */
    /* mck = 12.288MHz */
    atmel_i2s_gck_param { fs: 16000, mck: I2S_MCK_12M288, imckdiv: 1, imckfs: 47 }, /* mck =  768 fs */
    atmel_i2s_gck_param { fs: 24000, mck: I2S_MCK_12M288, imckdiv: 3, imckfs: 63 }, /* mck =  512 fs */
    atmel_i2s_gck_param { fs: 32000, mck: I2S_MCK_12M288, imckdiv: 3, imckfs: 47 }, /* mck =  384 fs */
    atmel_i2s_gck_param { fs: 48000, mck: I2S_MCK_12M288, imckdiv: 7, imckfs: 63 }, /* mck =  256 fs */
    atmel_i2s_gck_param { fs: 64000, mck: I2S_MCK_12M288, imckdiv: 7, imckfs: 47 }, /* mck =  192 fs */
    atmel_i2s_gck_param { fs: 96000, mck: I2S_MCK_12M288, imckdiv: 7, imckfs: 31 }, /* mck =  128 fs */
    atmel_i2s_gck_param { fs: 192000, mck: I2S_MCK_12M288, imckdiv: 7, imckfs: 15 }, /* mck =   64 fs */
    /* mck = 11.2896MHz */
    atmel_i2s_gck_param { fs: 11025, mck: I2S_MCK_11M2896, imckdiv: 1, imckfs: 63 }, /* mck = 1024 fs */
    atmel_i2s_gck_param { fs: 22050, mck: I2S_MCK_11M2896, imckdiv: 3, imckfs: 63 }, /* mck =  512 fs */
    atmel_i2s_gck_param { fs: 44100, mck: I2S_MCK_11M2896, imckdiv: 7, imckfs: 63 }, /* mck =  256 fs */
    atmel_i2s_gck_param { fs: 88200, mck: I2S_MCK_11M2896, imckdiv: 7, imckfs: 31 }, /* mck =  128 fs */
    atmel_i2s_gck_param { fs: 176400, mck: I2S_MCK_11M2896, imckdiv: 7, imckfs: 15 }, /* mck =   64 fs */
];

#[repr(C)]
struct atmel_i2s_caps {
    mck_init: Option<unsafe extern "C" fn(*mut atmel_i2s_dev, *mut device_node) -> c_int>,
}

#[repr(C)]
struct atmel_i2s_dev {
    dev: *mut device,
    regmap: *mut regmap,
    pclk: *mut clk,
    gclk: *mut clk,
    playback: snd_dmaengine_dai_dma_data,
    capture: snd_dmaengine_dai_dma_data,
    fmt: c_uint,
    gck_param: *const atmel_i2s_gck_param,
    caps: *const atmel_i2s_caps,
    clk_use_no: c_int,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_match_node(matches: *const of_device_id, node: *mut device_node) -> *const of_device_id;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn of_property_match_string(np: *mut device_node, propname: *const c_char, string: *const c_char) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const INT_MAX: c_int = 2147483647;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: u64_t = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64_t = 1 << 2;
const SNDRV_PCM_FMTBIT_S18_3LE: u64_t = 1 << 6;
const SNDRV_PCM_FMTBIT_S20_3LE: u64_t = 1 << 7;
const SNDRV_PCM_FMTBIT_S24_3LE: u64_t = 1 << 8;
const SNDRV_PCM_FMTBIT_S24_LE: u64_t = 1 << 10;
const SNDRV_PCM_FMTBIT_S32_LE: u64_t = 1 << 14;
const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S18_3LE: c_int = 6;
const SNDRV_PCM_FORMAT_S20_3LE: c_int = 7;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 8;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 10;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 14;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x1000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x4000;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64_t = 1;
const SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX: c_uint = 1;

unsafe extern "C" fn atmel_i2s_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dev = dev_id as *mut atmel_i2s_dev;
    let mut sr: c_uint = 0;
    let mut imr: c_uint = 0;
    let pending: c_uint;
    let mut ch: c_uint;
    let mut mask: c_uint;
    let mut ret: irqreturn_t = IRQ_NONE;

    regmap_read((*dev).regmap, ATMEL_I2SC_SR, &mut sr);
    regmap_read((*dev).regmap, ATMEL_I2SC_IMR, &mut imr);
    pending = sr & imr;

    if pending == 0 {
        return IRQ_NONE;
    }

    if (pending & ATMEL_I2SC_INT_RXOR) != 0 {
        mask = ATMEL_I2SC_SR_RXOR;

        ch = 0;
        while ch < ATMEL_I2SC_MAX_TDM_CHANNELS {
            if (sr & ATMEL_I2SC_SR_RXORCH(ch)) != 0 {
                mask |= ATMEL_I2SC_SR_RXORCH(ch);
                dev_err((*dev).dev, c"RX overrun on channel %d\n".as_ptr(), ch);
            }
            ch += 1;
        }
        regmap_write((*dev).regmap, ATMEL_I2SC_SCR, mask);
        ret = IRQ_HANDLED;
    }

    if (pending & ATMEL_I2SC_INT_TXUR) != 0 {
        mask = ATMEL_I2SC_SR_TXUR;

        ch = 0;
        while ch < ATMEL_I2SC_MAX_TDM_CHANNELS {
            if (sr & ATMEL_I2SC_SR_TXURCH(ch)) != 0 {
                mask |= ATMEL_I2SC_SR_TXURCH(ch);
                dev_err((*dev).dev, c"TX underrun on channel %d\n".as_ptr(), ch);
            }
            ch += 1;
        }
        regmap_write((*dev).regmap, ATMEL_I2SC_SCR, mask);
        ret = IRQ_HANDLED;
    }

    ret
}

const ATMEL_I2S_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

const ATMEL_I2S_FORMATS: u64_t = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

unsafe extern "C" fn atmel_i2s_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut atmel_i2s_dev;

    (*dev).fmt = fmt;
    0
}

unsafe extern "C" fn atmel_i2s_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut atmel_i2s_dev;
    let is_playback = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut rhr: c_uint = 0;
    let mut sr: c_uint = 0;

    if is_playback {
        regmap_read((*dev).regmap, ATMEL_I2SC_SR, &mut sr);
        if (sr & ATMEL_I2SC_SR_RXRDY) != 0 {
            /*
             * The RX Ready flag should not be set. However if here,
             * we flush (read) the Receive Holding Register to start
             * from a clean state.
             */
            dev_dbg((*dev).dev, c"RXRDY is set\n".as_ptr());
            regmap_read((*dev).regmap, ATMEL_I2SC_RHR, &mut rhr);
        }
    }

    0
}

unsafe fn atmel_i2s_get_gck_param(dev: *mut atmel_i2s_dev, fs: c_int) -> c_int {
    let mut i: usize;
    let mut best: c_int;

    if (*dev).gclk.is_null() {
        dev_err((*dev).dev, c"cannot generate the I2S Master Clock\n".as_ptr());
        return -EINVAL;
    }

    /*
     * Find the best possible settings to generate the I2S Master Clock
     * from the PLL Audio.
     */
    (*dev).gck_param = core::ptr::null();
    best = INT_MAX;
    i = 0;
    while i < gck_params.len() {
        let gck_param = &gck_params[i] as *const atmel_i2s_gck_param;
        let val = (fs - (*gck_param).fs).abs();

        if val < best {
            best = val;
            (*dev).gck_param = gck_param;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn atmel_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut atmel_i2s_dev;
    let is_playback = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut mr: c_uint = 0;
    let mut mr_mask: c_uint;
    let ret: c_int;

    mr_mask = ATMEL_I2SC_MR_FORMAT_MASK | ATMEL_I2SC_MR_MODE_MASK | ATMEL_I2SC_MR_DATALENGTH_MASK;
    if is_playback {
        mr_mask |= ATMEL_I2SC_MR_TXMONO;
    } else {
        mr_mask |= ATMEL_I2SC_MR_RXMONO;
    }

    match (*dev).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            mr |= ATMEL_I2SC_MR_FORMAT_I2S;
        }
        _ => {
            dev_err((*dev).dev, c"unsupported bus format\n".as_ptr());
            return -EINVAL;
        }
    }

    match (*dev).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            /* codec is slave, so cpu is master */
            mr |= ATMEL_I2SC_MR_MODE_MASTER;
            ret = atmel_i2s_get_gck_param(dev, params_rate(params));
            if ret != 0 {
                return ret;
            }
        }
        SND_SOC_DAIFMT_BC_FC => {
            /* codec is master, so cpu is slave */
            mr |= ATMEL_I2SC_MR_MODE_SLAVE;
            (*dev).gck_param = core::ptr::null();
        }
        _ => {
            dev_err((*dev).dev, c"unsupported master/slave mode\n".as_ptr());
            return -EINVAL;
        }
    }

    match params_channels(params) {
        1 => {
            if is_playback {
                mr |= ATMEL_I2SC_MR_TXMONO;
            } else {
                mr |= ATMEL_I2SC_MR_RXMONO;
            }
        }
        2 => {}
        _ => {
            dev_err((*dev).dev, c"unsupported number of audio channels\n".as_ptr());
            return -EINVAL;
        }
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_8_BITS;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_16_BITS;
        }
        SNDRV_PCM_FORMAT_S18_3LE => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_18_BITS | ATMEL_I2SC_MR_IWS;
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_20_BITS | ATMEL_I2SC_MR_IWS;
        }
        SNDRV_PCM_FORMAT_S24_3LE => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_24_BITS | ATMEL_I2SC_MR_IWS;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_24_BITS;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            mr |= ATMEL_I2SC_MR_DATALENGTH_32_BITS;
        }
        _ => {
            dev_err((*dev).dev, c"unsupported size/endianness for audio samples\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits((*dev).regmap, ATMEL_I2SC_MR, mr_mask, mr)
}

unsafe fn atmel_i2s_switch_mck_generator(dev: *mut atmel_i2s_dev, enabled: bool_t) -> c_int {
    let mut mr: c_uint;
    let mr_mask: c_uint;
    let gclk_rate: c_ulong;
    let mut ret: c_int;

    mr = 0;
    mr_mask = ATMEL_I2SC_MR_IMCKDIV_MASK | ATMEL_I2SC_MR_IMCKFS_MASK | ATMEL_I2SC_MR_IMCKMODE_MASK;

    if !enabled {
        /* Disable the I2S Master Clock generator. */
        ret = regmap_write((*dev).regmap, ATMEL_I2SC_CR, ATMEL_I2SC_CR_CKDIS);
        if ret != 0 {
            return ret;
        }

        /* Reset the I2S Master Clock generator settings. */
        ret = regmap_update_bits((*dev).regmap, ATMEL_I2SC_MR, mr_mask, mr);
        if ret != 0 {
            return ret;
        }

        /* Disable/unprepare the PMC generated clock. */
        clk_disable_unprepare((*dev).gclk);

        return 0;
    }

    if (*dev).gck_param.is_null() {
        return -EINVAL;
    }

    gclk_rate = (*(*dev).gck_param).mck * ((*(*dev).gck_param).imckdiv + 1) as c_ulong;

    ret = clk_set_rate((*dev).gclk, gclk_rate);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*dev).gclk);
    if ret != 0 {
        return ret;
    }

    /* Update the Mode Register to generate the I2S Master Clock. */
    mr |= ATMEL_I2SC_MR_IMCKDIV((*(*dev).gck_param).imckdiv);
    mr |= ATMEL_I2SC_MR_IMCKFS((*(*dev).gck_param).imckfs);
    mr |= ATMEL_I2SC_MR_IMCKMODE_I2SMCK;
    ret = regmap_update_bits((*dev).regmap, ATMEL_I2SC_MR, mr_mask, mr);
    if ret != 0 {
        return ret;
    }

    /* Finally enable the I2S Master Clock generator. */
    regmap_write((*dev).regmap, ATMEL_I2SC_CR, ATMEL_I2SC_CR_CKEN)
}

unsafe extern "C" fn atmel_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut atmel_i2s_dev;
    let is_playback = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let is_master: bool_t;
    let mck_enabled: bool_t;
    let cr: c_uint;
    let mut mr: c_uint = 0;
    let mut err: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            cr = if is_playback { ATMEL_I2SC_CR_TXEN } else { ATMEL_I2SC_CR_RXEN };
            mck_enabled = true;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            cr = if is_playback { ATMEL_I2SC_CR_TXDIS } else { ATMEL_I2SC_CR_RXDIS };
            mck_enabled = false;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* Read the Mode Register to retrieve the master/slave state. */
    err = regmap_read((*dev).regmap, ATMEL_I2SC_MR, &mut mr);
    if err != 0 {
        return err;
    }
    is_master = (mr & ATMEL_I2SC_MR_MODE_MASK) == ATMEL_I2SC_MR_MODE_MASTER;

    /* If master starts, enable the audio clock. */
    if is_master && mck_enabled {
        if (*dev).clk_use_no == 0 {
            err = atmel_i2s_switch_mck_generator(dev, true);
            if err != 0 {
                return err;
            }
        }
        (*dev).clk_use_no += 1;
    }

    err = regmap_write((*dev).regmap, ATMEL_I2SC_CR, cr);
    if err != 0 {
        return err;
    }

    /* If master stops, disable the audio clock. */
    if is_master && !mck_enabled {
        if (*dev).clk_use_no == 1 {
            err = atmel_i2s_switch_mck_generator(dev, false);
            if err != 0 {
                return err;
            }
        }
        (*dev).clk_use_no -= 1;
    }

    err
}

unsafe extern "C" fn atmel_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut atmel_i2s_dev;

    snd_soc_dai_init_dma_data(dai, &mut (*dev).playback, &mut (*dev).capture);
    0
}

static atmel_i2s_selectable_formats: u64_t = SND_SOC_POSSIBLE_DAIFMT_I2S;

static atmel_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(atmel_i2s_dai_probe),
    prepare: Some(atmel_i2s_prepare),
    trigger: Some(atmel_i2s_trigger),
    hw_params: Some(atmel_i2s_hw_params),
    set_fmt: Some(atmel_i2s_set_dai_fmt),
    auto_selectable_formats: &atmel_i2s_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut atmel_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: ATMEL_I2S_RATES,
        formats: ATMEL_I2S_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: ATMEL_I2S_RATES,
        formats: ATMEL_I2S_FORMATS,
    },
    ops: &atmel_i2s_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

static atmel_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"atmel-i2s".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn atmel_i2s_sama5d2_mck_init(dev: *mut atmel_i2s_dev, _np: *mut device_node) -> c_int {
    let muxclk: *mut clk;
    let err: c_int;

    if (*dev).gclk.is_null() {
        return 0;
    }

    /* muxclk is optional, so we return error for probe defer only */
    muxclk = devm_clk_get((*dev).dev, c"muxclk".as_ptr());
    if IS_ERR(muxclk as *const c_void) {
        err = PTR_ERR(muxclk as *const c_void);
        if err == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
        dev_dbg((*dev).dev, c"failed to get the I2S clock control: %d\n".as_ptr(), err);
        return 0;
    }

    clk_set_parent(muxclk, (*dev).gclk)
}

static atmel_i2s_sama5d2_caps: atmel_i2s_caps = atmel_i2s_caps {
    mck_init: Some(atmel_i2s_sama5d2_mck_init),
};

static atmel_i2s_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"atmel,sama5d2-i2s".as_ptr(),
        data: &atmel_i2s_sama5d2_caps as *const _ as *const c_void,
    },
    of_device_id {
        /* sentinel */
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, atmel_i2s_dt_ids); */

unsafe extern "C" fn atmel_i2s_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let match_id: *const of_device_id;
    let dev: *mut atmel_i2s_dev;
    let mut mem: *mut resource = core::ptr::null_mut();
    let regmap_ptr: *mut regmap;
    let base: *mut c_void;
    let irq: c_int;
    let mut err: c_int;
    let mut pcm_flags: c_uint = 0;
    let mut version: c_uint = 0;

    /* Get memory for driver data. */
    dev = devm_kzalloc(&mut (*pdev).dev as *mut _ as *mut device, core::mem::size_of::<atmel_i2s_dev>(), GFP_KERNEL) as *mut atmel_i2s_dev;
    if dev.is_null() {
        return -ENOMEM;
    }

    /* Get hardware capabilities. */
    match_id = of_match_node(atmel_i2s_dt_ids.as_ptr(), np);
    if !match_id.is_null() {
        (*dev).caps = (*match_id).data as *const atmel_i2s_caps;
    }

    /* Map I/O registers. */
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
    if IS_ERR(base as *const c_void) {
        return PTR_ERR(base as *const c_void);
    }

    regmap_ptr = devm_regmap_init_mmio(
        &mut (*pdev).dev as *mut _ as *mut device,
        base,
        &atmel_i2s_regmap_config,
    );
    if IS_ERR(regmap_ptr as *const c_void) {
        return PTR_ERR(regmap_ptr as *const c_void);
    }

    /* Request IRQ. */
    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    err = devm_request_irq(
        &mut (*pdev).dev as *mut _ as *mut device,
        irq,
        atmel_i2s_interrupt,
        0,
        dev_name(&mut (*pdev).dev as *mut _ as *mut device),
        dev as *mut c_void,
    );
    if err != 0 {
        return err;
    }

    /* Get the peripheral clock. */
    (*dev).pclk = devm_clk_get(&mut (*pdev).dev as *mut _ as *mut device, c"pclk".as_ptr());
    if IS_ERR((*dev).pclk as *const c_void) {
        err = PTR_ERR((*dev).pclk as *const c_void);
        dev_err(&mut (*pdev).dev as *mut _ as *mut device, c"failed to get the peripheral clock: %d\n".as_ptr(), err);
        return err;
    }

    /* Get audio clock to generate the I2S Master Clock (I2S_MCK) */
    (*dev).gclk = devm_clk_get(&mut (*pdev).dev as *mut _ as *mut device, c"gclk".as_ptr());
    if IS_ERR((*dev).gclk as *const c_void) {
        if PTR_ERR((*dev).gclk as *const c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
        /* Master Mode not supported */
        (*dev).gclk = core::ptr::null_mut();
    }
    (*dev).dev = &mut (*pdev).dev as *mut _ as *mut device;
    (*dev).regmap = regmap_ptr;
    platform_set_drvdata(pdev, dev as *mut c_void);

    /* Do hardware specific settings to initialize I2S_MCK generator */
    if !(*dev).caps.is_null() && (*(*dev).caps).mck_init.is_some() {
        err = ((*(*dev).caps).mck_init.unwrap())(dev, np);
        if err != 0 {
            return err;
        }
    }

    /* Enable the peripheral clock. */
    err = clk_prepare_enable((*dev).pclk);
    if err != 0 {
        return err;
    }

    /* Get IP version. */
    regmap_read((*dev).regmap, ATMEL_I2SC_VERSION, &mut version);
    dev_info(&mut (*pdev).dev as *mut _ as *mut device, c"hw version: %#x\n".as_ptr(), version);

    /* Enable error interrupts. */
    regmap_write((*dev).regmap, ATMEL_I2SC_IER, ATMEL_I2SC_INT_RXOR | ATMEL_I2SC_INT_TXUR);

    err = devm_snd_soc_register_component(
        &mut (*pdev).dev as *mut _ as *mut device,
        &atmel_i2s_component,
        &raw mut atmel_i2s_dai,
        1,
    );
    if err != 0 {
        dev_err(&mut (*pdev).dev as *mut _ as *mut device, c"failed to register DAI: %d\n".as_ptr(), err);
        clk_disable_unprepare((*dev).pclk);
        return err;
    }

    /* Prepare DMA config. */
    (*dev).playback.addr = (*mem).start as dma_addr_t + ATMEL_I2SC_THR as dma_addr_t;
    (*dev).playback.maxburst = 1;
    (*dev).capture.addr = (*mem).start as dma_addr_t + ATMEL_I2SC_RHR as dma_addr_t;
    (*dev).capture.maxburst = 1;

    if of_property_match_string(np, c"dma-names".as_ptr(), c"rx-tx".as_ptr()) == 0 {
        pcm_flags |= SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX;
    }
    err = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev as *mut _ as *mut device, core::ptr::null(), pcm_flags);
    if err != 0 {
        dev_err(&mut (*pdev).dev as *mut _ as *mut device, c"failed to register PCM: %d\n".as_ptr(), err);
        clk_disable_unprepare((*dev).pclk);
        return err;
    }

    0
}

unsafe extern "C" fn atmel_i2s_remove(pdev: *mut platform_device) {
    let dev = platform_get_drvdata(pdev) as *mut atmel_i2s_dev;

    clk_disable_unprepare((*dev).pclk);
}

static mut atmel_i2s_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"atmel_i2s".as_ptr(),
        of_match_table: atmel_i2s_dt_ids.as_ptr(),
    },
    probe: Some(atmel_i2s_probe),
    remove: Some(atmel_i2s_remove),
};

/* module_platform_driver(atmel_i2s_driver); */

/* MODULE_DESCRIPTION("Atmel I2S Controller driver"); */
/* MODULE_AUTHOR("Cyrille Pitchen <cyrille.pitchen@atmel.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
