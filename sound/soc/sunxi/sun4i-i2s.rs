// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Andrea Venturi
 * Andrea Venturi <be17068@iperbole.bo.it>
 *
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type bool_ = bool;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;
type c_ulong = core::ffi::c_ulong;
type c_char = core::ffi::c_char;
type c_void = core::ffi::c_void;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const DMA_SLAVE_BUSWIDTH_2_BYTES: u32 = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: u32 = 4;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0100;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0200;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0300;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x1000;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S20_LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const fn REG_FIELD(reg: c_uint, lsb: c_uint, msb: c_uint) -> reg_field {
    reg_field { reg, lsb, msb }
}

const SUN4I_I2S_CTRL_REG: c_uint = 0x00;
const SUN4I_I2S_CTRL_SDO_EN_MASK: c_uint = GENMASK(11, 8);
const fn SUN4I_I2S_CTRL_SDO_EN(sdo: c_uint) -> c_uint { BIT(8 + sdo) }
const SUN4I_I2S_CTRL_MODE_MASK: c_uint = BIT(5);
const SUN4I_I2S_CTRL_MODE_SLAVE: c_uint = 1 << 5;
const SUN4I_I2S_CTRL_MODE_MASTER: c_uint = 0 << 5;
const SUN4I_I2S_CTRL_TX_EN: c_uint = BIT(2);
const SUN4I_I2S_CTRL_RX_EN: c_uint = BIT(1);
const SUN4I_I2S_CTRL_GL_EN: c_uint = BIT(0);

const SUN4I_I2S_FMT0_REG: c_uint = 0x04;
const SUN4I_I2S_FMT0_LRCLK_POLARITY_MASK: c_uint = BIT(7);
const SUN4I_I2S_FMT0_LRCLK_POLARITY_INVERTED: c_uint = 1 << 7;
const SUN4I_I2S_FMT0_BCLK_POLARITY_MASK: c_uint = BIT(6);
const SUN4I_I2S_FMT0_BCLK_POLARITY_INVERTED: c_uint = 1 << 6;
const SUN4I_I2S_FMT0_SR_MASK: c_uint = GENMASK(5, 4);
const fn SUN4I_I2S_FMT0_SR(sr: c_uint) -> c_uint { sr << 4 }
const SUN4I_I2S_FMT0_WSS_MASK: c_uint = GENMASK(3, 2);
const fn SUN4I_I2S_FMT0_WSS(wss: c_uint) -> c_uint { wss << 2 }
const SUN4I_I2S_FMT0_FMT_MASK: c_uint = GENMASK(1, 0);
const SUN4I_I2S_FMT0_FMT_RIGHT_J: c_uint = 2 << 0;
const SUN4I_I2S_FMT0_FMT_LEFT_J: c_uint = 1 << 0;
const SUN4I_I2S_FMT0_FMT_I2S: c_uint = 0 << 0;

const SUN4I_I2S_FMT1_REG: c_uint = 0x08;
const SUN4I_I2S_FMT1_REG_SEXT_MASK: c_uint = BIT(8);
const fn SUN4I_I2S_FMT1_REG_SEXT(sext: c_uint) -> c_uint { sext << 8 }
const SUN4I_I2S_FIFO_TX_REG: c_uint = 0x0c;
const SUN4I_I2S_FIFO_RX_REG: c_uint = 0x10;
const SUN4I_I2S_FIFO_CTRL_REG: c_uint = 0x14;
const SUN4I_I2S_FIFO_CTRL_FLUSH_TX: c_uint = BIT(25);
const SUN4I_I2S_FIFO_CTRL_FLUSH_RX: c_uint = BIT(24);
const SUN4I_I2S_FIFO_CTRL_TX_MODE_MASK: c_uint = BIT(2);
const fn SUN4I_I2S_FIFO_CTRL_TX_MODE(mode: c_uint) -> c_uint { mode << 2 }
const SUN4I_I2S_FIFO_CTRL_RX_MODE_MASK: c_uint = GENMASK(1, 0);
const fn SUN4I_I2S_FIFO_CTRL_RX_MODE(mode: c_uint) -> c_uint { mode }
const SUN4I_I2S_FIFO_STA_REG: c_uint = 0x18;
const SUN4I_I2S_DMA_INT_CTRL_REG: c_uint = 0x1c;
const SUN4I_I2S_DMA_INT_CTRL_TX_DRQ_EN: c_uint = BIT(7);
const SUN4I_I2S_DMA_INT_CTRL_RX_DRQ_EN: c_uint = BIT(3);
const SUN4I_I2S_INT_STA_REG: c_uint = 0x20;
const SUN4I_I2S_CLK_DIV_REG: c_uint = 0x24;
const SUN4I_I2S_CLK_DIV_MCLK_EN: c_uint = BIT(7);
const SUN4I_I2S_CLK_DIV_BCLK_MASK: c_uint = GENMASK(6, 4);
const fn SUN4I_I2S_CLK_DIV_BCLK(bclk: c_int) -> c_uint { (bclk as c_uint) << 4 }
const SUN4I_I2S_CLK_DIV_MCLK_MASK: c_uint = GENMASK(3, 0);
const fn SUN4I_I2S_CLK_DIV_MCLK(mclk: c_int) -> c_uint { (mclk as c_uint) << 0 }
const SUN4I_I2S_TX_CNT_REG: c_uint = 0x28;
const SUN4I_I2S_RX_CNT_REG: c_uint = 0x2c;
const SUN4I_I2S_TX_CHAN_SEL_REG: c_uint = 0x30;
const SUN4I_I2S_CHAN_SEL_MASK: c_uint = GENMASK(2, 0);
const fn SUN4I_I2S_CHAN_SEL(num_chan: c_uint) -> c_uint { (num_chan - 1) << 0 }
const SUN4I_I2S_TX_CHAN_MAP_REG: c_uint = 0x34;
const fn SUN4I_I2S_TX_CHAN_MAP(chan: c_uint, sample: c_uint) -> c_uint { sample << (chan << 2) }
const SUN4I_I2S_RX_CHAN_SEL_REG: c_uint = 0x38;
const SUN4I_I2S_RX_CHAN_MAP_REG: c_uint = 0x3c;

/* Defines required for sun8i-h3 support */
const SUN8I_I2S_CTRL_BCLK_OUT: c_uint = BIT(18);
const SUN8I_I2S_CTRL_LRCK_OUT: c_uint = BIT(17);
const SUN8I_I2S_CTRL_MODE_MASK: c_uint = GENMASK(5, 4);
const SUN8I_I2S_CTRL_MODE_RIGHT: c_uint = 2 << 4;
const SUN8I_I2S_CTRL_MODE_LEFT: c_uint = 1 << 4;
const SUN8I_I2S_CTRL_MODE_PCM: c_uint = 0 << 4;
const SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK: c_uint = BIT(19);
const SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH: c_uint = 1 << 19;
const SUN8I_I2S_FMT0_LRCLK_POLARITY_START_LOW: c_uint = 0 << 19;
const SUN8I_I2S_FMT0_LRCK_PERIOD_MASK: c_uint = GENMASK(17, 8);
const fn SUN8I_I2S_FMT0_LRCK_PERIOD(period: c_uint) -> c_uint { (period - 1) << 8 }
const SUN8I_I2S_FMT0_BCLK_POLARITY_MASK: c_uint = BIT(7);
const SUN8I_I2S_FMT0_BCLK_POLARITY_INVERTED: c_uint = 1 << 7;
const SUN8I_I2S_FMT0_BCLK_POLARITY_NORMAL: c_uint = 0 << 7;
const SUN8I_I2S_FMT1_REG_SEXT_MASK: c_uint = GENMASK(5, 4);
const fn SUN8I_I2S_FMT1_REG_SEXT(sext: c_uint) -> c_uint { sext << 4 }
const SUN8I_I2S_INT_STA_REG: c_uint = 0x0c;
const SUN8I_I2S_FIFO_TX_REG: c_uint = 0x20;
const SUN8I_I2S_CHAN_CFG_REG: c_uint = 0x30;
const SUN8I_I2S_CHAN_CFG_RX_SLOT_NUM_MASK: c_uint = GENMASK(7, 4);
const fn SUN8I_I2S_CHAN_CFG_RX_SLOT_NUM(chan: c_uint) -> c_uint { (chan - 1) << 4 }
const SUN8I_I2S_CHAN_CFG_TX_SLOT_NUM_MASK: c_uint = GENMASK(3, 0);
const fn SUN8I_I2S_CHAN_CFG_TX_SLOT_NUM(chan: c_uint) -> c_uint { chan - 1 }
const SUN8I_I2S_TX_CHAN_MAP_REG: c_uint = 0x44;
const SUN8I_I2S_TX_CHAN_SEL_REG: c_uint = 0x34;
const SUN8I_I2S_TX_CHAN_OFFSET_MASK: c_uint = GENMASK(13, 12);
const fn SUN8I_I2S_TX_CHAN_OFFSET(offset: u8) -> c_uint { (offset as c_uint) << 12 }
const SUN8I_I2S_TX_CHAN_EN_MASK: c_uint = GENMASK(11, 4);
const fn SUN8I_I2S_TX_CHAN_EN(num_chan: c_uint) -> c_uint { ((1 << num_chan) - 1) << 4 }
const SUN8I_I2S_RX_CHAN_SEL_REG: c_uint = 0x54;
const SUN8I_I2S_RX_CHAN_MAP_REG: c_uint = 0x58;

/* Defines required for sun50i-h6 support */
const SUN50I_H6_I2S_TX_CHAN_SEL_OFFSET_MASK: c_uint = GENMASK(21, 20);
const fn SUN50I_H6_I2S_TX_CHAN_SEL_OFFSET(offset: u8) -> c_uint { (offset as c_uint) << 20 }
const SUN50I_H6_I2S_TX_CHAN_SEL_MASK: c_uint = GENMASK(19, 16);
const fn SUN50I_H6_I2S_TX_CHAN_SEL(chan: c_uint) -> c_uint { (chan - 1) << 16 }
const SUN50I_H6_I2S_TX_CHAN_EN_MASK: c_uint = GENMASK(15, 0);
const fn SUN50I_H6_I2S_TX_CHAN_EN(num_chan: c_uint) -> c_uint { (1 << num_chan) - 1 }
const fn SUN50I_H6_I2S_TX_CHAN_SEL_REG(pin: c_uint) -> c_uint { 0x34 + 4 * pin }
const fn SUN50I_H6_I2S_TX_CHAN_MAP0_REG(pin: c_uint) -> c_uint { 0x44 + 8 * pin }
const fn SUN50I_H6_I2S_TX_CHAN_MAP1_REG(pin: c_uint) -> c_uint { 0x48 + 8 * pin }
const SUN50I_H6_I2S_RX_CHAN_SEL_REG: c_uint = 0x64;
const SUN50I_H6_I2S_RX_CHAN_MAP0_REG: c_uint = 0x68;
const SUN50I_H6_I2S_RX_CHAN_MAP1_REG: c_uint = 0x6C;
const SUN50I_R329_I2S_RX_CHAN_MAP0_REG: c_uint = 0x68;
const SUN50I_R329_I2S_RX_CHAN_MAP1_REG: c_uint = 0x6c;
const SUN50I_R329_I2S_RX_CHAN_MAP2_REG: c_uint = 0x70;
const SUN50I_R329_I2S_RX_CHAN_MAP3_REG: c_uint = 0x74;

#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct reset_control { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] struct regmap_field { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct resource {
    start: c_ulong,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_field {
    reg: c_uint,
    lsb: c_uint,
    msb: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: c_ulong,
    addr_width: u32,
    maxburst: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    capture: snd_soc_pcm_stream,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: device_driver,
}

/**
 * struct sun4i_i2s_quirks - Differences between SoC variants.
 * @has_reset: SoC needs reset deasserted.
 * @pcm_formats: available PCM formats.
 * @reg_offset_txdata: offset of the tx fifo.
 * @sun4i_i2s_regmap: regmap config to use.
 * @field_clkdiv_mclk_en: regmap field to enable mclk output.
 * @field_fmt_wss: regmap field to set word select size.
 * @field_fmt_sr: regmap field to set sample resolution.
 * @num_din_pins: input pins
 * @num_dout_pins: output pins (currently set but unused)
 * @bclk_dividers: bit clock dividers array
 * @num_bclk_dividers: number of bit clock dividers
 * @mclk_dividers: mclk dividers array
 * @num_mclk_dividers: number of mclk dividers
 * @get_bclk_parent_rate: callback to get bclk parent rate
 * @get_sr: callback to get sample resolution
 * @get_wss: callback to get word select size
 * @set_chan_cfg: callback to set channel configuration
 * @set_fmt: callback to set format
 */
#[repr(C)]
struct sun4i_i2s_quirks {
    has_reset: bool_,
    pcm_formats: u64,
    reg_offset_txdata: c_uint, /* TX FIFO */
    sun4i_i2s_regmap: *const regmap_config,
    /* Register fields for i2s */
    field_clkdiv_mclk_en: reg_field,
    field_fmt_wss: reg_field,
    field_fmt_sr: reg_field,
    num_din_pins: c_uint,
    num_dout_pins: c_uint,
    bclk_dividers: *const sun4i_i2s_clk_div,
    num_bclk_dividers: c_uint,
    mclk_dividers: *const sun4i_i2s_clk_div,
    num_mclk_dividers: c_uint,
    get_bclk_parent_rate: Option<unsafe extern "C" fn(*const sun4i_i2s) -> c_ulong>,
    get_sr: Option<unsafe extern "C" fn(c_uint) -> c_int>,
    get_wss: Option<unsafe extern "C" fn(c_uint) -> c_int>,
    /*
     * In the set_chan_cfg() function pointer:
     * @slots: channels per frame + padding slots, regardless of format
     * @slot_width: bits per sample + padding bits, regardless of format
     */
    set_chan_cfg: Option<unsafe extern "C" fn(*const sun4i_i2s, c_uint, c_uint, c_uint) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*const sun4i_i2s, c_uint) -> c_int>,
}

#[repr(C)]
struct sun4i_i2s {
    bus_clk: *mut clk,
    mod_clk: *mut clk,
    regmap: *mut regmap,
    rst: *mut reset_control,
    format: c_uint,
    mclk_freq: c_uint,
    slots: c_uint,
    slot_width: c_uint,
    capture_dma_data: snd_dmaengine_dai_dma_data,
    playback_dma_data: snd_dmaengine_dai_dma_data,
    /* Register fields for i2s */
    field_clkdiv_mclk_en: *mut regmap_field,
    field_fmt_wss: *mut regmap_field,
    field_fmt_sr: *mut regmap_field,
    variant: *const sun4i_i2s_quirks,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sun4i_i2s_clk_div {
    div: u8,
    val: u8,
}

unsafe extern "C" {
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_field_write(field: *mut regmap_field, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_pcm_hw_constraint_mask64(runtime: *mut snd_pcm_runtime, var: c_int, mask: u64) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_;
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_;
    fn devm_regmap_field_alloc(dev: *mut device, map: *mut regmap, field: reg_field) -> *mut regmap_field;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

static sun4i_i2s_bclk_div: [sun4i_i2s_clk_div; 6] = [
    sun4i_i2s_clk_div { div: 2, val: 0 },
    sun4i_i2s_clk_div { div: 4, val: 1 },
    sun4i_i2s_clk_div { div: 6, val: 2 },
    sun4i_i2s_clk_div { div: 8, val: 3 },
    sun4i_i2s_clk_div { div: 12, val: 4 },
    sun4i_i2s_clk_div { div: 16, val: 5 },
    /* TODO - extend divide ratio supported by newer SoCs */
];

static sun4i_i2s_mclk_div: [sun4i_i2s_clk_div; 8] = [
    sun4i_i2s_clk_div { div: 1, val: 0 },
    sun4i_i2s_clk_div { div: 2, val: 1 },
    sun4i_i2s_clk_div { div: 4, val: 2 },
    sun4i_i2s_clk_div { div: 6, val: 3 },
    sun4i_i2s_clk_div { div: 8, val: 4 },
    sun4i_i2s_clk_div { div: 12, val: 5 },
    sun4i_i2s_clk_div { div: 16, val: 6 },
    sun4i_i2s_clk_div { div: 24, val: 7 },
    /* TODO - extend divide ratio supported by newer SoCs */
];

static sun8i_i2s_clk_div: [sun4i_i2s_clk_div; 15] = [
    sun4i_i2s_clk_div { div: 1, val: 1 },
    sun4i_i2s_clk_div { div: 2, val: 2 },
    sun4i_i2s_clk_div { div: 4, val: 3 },
    sun4i_i2s_clk_div { div: 6, val: 4 },
    sun4i_i2s_clk_div { div: 8, val: 5 },
    sun4i_i2s_clk_div { div: 12, val: 6 },
    sun4i_i2s_clk_div { div: 16, val: 7 },
    sun4i_i2s_clk_div { div: 24, val: 8 },
    sun4i_i2s_clk_div { div: 32, val: 9 },
    sun4i_i2s_clk_div { div: 48, val: 10 },
    sun4i_i2s_clk_div { div: 64, val: 11 },
    sun4i_i2s_clk_div { div: 96, val: 12 },
    sun4i_i2s_clk_div { div: 128, val: 13 },
    sun4i_i2s_clk_div { div: 176, val: 14 },
    sun4i_i2s_clk_div { div: 192, val: 15 },
];

unsafe extern "C" fn sun4i_i2s_get_bclk_parent_rate(i2s: *const sun4i_i2s) -> c_ulong {
    (*i2s).mclk_freq as c_ulong
}

unsafe extern "C" fn sun8i_i2s_get_bclk_parent_rate(i2s: *const sun4i_i2s) -> c_ulong {
    clk_get_rate((*i2s).mod_clk)
}

unsafe extern "C" fn sun4i_i2s_get_bclk_div(i2s: *mut sun4i_i2s, parent_rate: c_ulong, sampling_rate: c_uint, channels: c_uint, word_size: c_uint) -> c_int {
    let dividers = (*(*i2s).variant).bclk_dividers;
    let div = (parent_rate / sampling_rate as c_ulong / word_size as c_ulong / channels as c_ulong) as c_int;
    let mut i: c_uint = 0;
    while i < (*(*i2s).variant).num_bclk_dividers {
        let bdiv = dividers.add(i as usize);
        if (*bdiv).div as c_int == div {
            return (*bdiv).val as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn sun4i_i2s_get_mclk_div(i2s: *mut sun4i_i2s, parent_rate: c_ulong, mclk_rate: c_ulong) -> c_int {
    let dividers = (*(*i2s).variant).mclk_dividers;
    let div = (parent_rate / mclk_rate) as c_int;
    let mut i: c_uint = 0;
    while i < (*(*i2s).variant).num_mclk_dividers {
        let mdiv = dividers.add(i as usize);
        if (*mdiv).div as c_int == div {
            return (*mdiv).val as c_int;
        }
        i += 1;
    }
    -EINVAL
}

static mut sun4i_i2s_oversample_rates: [c_int; 6] = [128, 192, 256, 384, 512, 768];

unsafe extern "C" fn sun4i_i2s_oversample_is_valid(oversample: c_uint) -> bool_ {
    let mut i: usize = 0;
    while i < sun4i_i2s_oversample_rates.len() {
        if sun4i_i2s_oversample_rates[i] == oversample as c_int {
            return true;
        }
        i += 1;
    }
    false
}

unsafe extern "C" fn sun4i_i2s_set_clk_rate(dai: *mut snd_soc_dai, rate: c_uint, slots: c_uint, slot_width: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    let oversample_rate: c_uint;
    let clk_rate: c_uint;
    let bclk_parent_rate: c_uint;
    let bclk_div: c_int;
    let mclk_div: c_int;
    let mut ret: c_int;

    match rate {
        176400 | 88200 | 44100 | 22050 | 11025 => clk_rate = 22579200,
        192000 | 128000 | 96000 | 64000 | 48000 | 32000 | 24000 | 16000 | 12000 | 8000 => clk_rate = 24576000,
        _ => {
            dev_err((*dai).dev, b"Unsupported sample rate: %u\n\0".as_ptr() as *const c_char, rate);
            return -EINVAL;
        }
    }

    ret = clk_set_rate((*i2s).mod_clk, clk_rate);
    if ret != 0 {
        return ret;
    }

    oversample_rate = (*i2s).mclk_freq / rate;
    if !sun4i_i2s_oversample_is_valid(oversample_rate) {
        dev_err((*dai).dev, b"Unsupported oversample rate: %d\n\0".as_ptr() as *const c_char, oversample_rate);
        return -EINVAL;
    }

    bclk_parent_rate = ((*(*i2s).variant).get_bclk_parent_rate.unwrap())(i2s) as c_uint;
    bclk_div = sun4i_i2s_get_bclk_div(i2s, bclk_parent_rate as c_ulong, rate, slots, slot_width);
    if bclk_div < 0 {
        dev_err((*dai).dev, b"Unsupported BCLK divider: %d\n\0".as_ptr() as *const c_char, bclk_div);
        return -EINVAL;
    }

    mclk_div = sun4i_i2s_get_mclk_div(i2s, clk_rate as c_ulong, (*i2s).mclk_freq as c_ulong);
    if mclk_div < 0 {
        dev_err((*dai).dev, b"Unsupported MCLK divider: %d\n\0".as_ptr() as *const c_char, mclk_div);
        return -EINVAL;
    }

    regmap_write((*i2s).regmap, SUN4I_I2S_CLK_DIV_REG, SUN4I_I2S_CLK_DIV_BCLK(bclk_div) | SUN4I_I2S_CLK_DIV_MCLK(mclk_div));
    regmap_field_write((*i2s).field_clkdiv_mclk_en, 1);
    0
}

unsafe extern "C" fn sun4i_i2s_get_sr(width: c_uint) -> c_int {
    match width {
        16 => 0,
        20 => 1,
        24 => 2,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn sun4i_i2s_get_wss(width: c_uint) -> c_int {
    match width {
        16 => 0,
        20 => 1,
        24 => 2,
        32 => 3,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn sun8i_i2s_get_sr_wss(width: c_uint) -> c_int {
    match width {
        8 => 1,
        12 => 2,
        16 => 3,
        20 => 4,
        24 => 5,
        28 => 6,
        32 => 7,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn sun4i_i2s_set_chan_cfg(i2s: *const sun4i_i2s, channels: c_uint, _slots: c_uint, _slot_width: c_uint) -> c_int {
    /* Map the channels for playback and capture */
    regmap_write((*i2s).regmap, SUN4I_I2S_TX_CHAN_MAP_REG, 0x76543210);
    regmap_write((*i2s).regmap, SUN4I_I2S_RX_CHAN_MAP_REG, 0x00003210);

    /* Configure the channels */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_TX_CHAN_SEL_REG, SUN4I_I2S_CHAN_SEL_MASK, SUN4I_I2S_CHAN_SEL(channels));
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_RX_CHAN_SEL_REG, SUN4I_I2S_CHAN_SEL_MASK, SUN4I_I2S_CHAN_SEL(channels));
    0
}

unsafe extern "C" fn sun8i_i2s_set_chan_cfg(i2s: *const sun4i_i2s, channels: c_uint, slots: c_uint, slot_width: c_uint) -> c_int {
    let lrck_period: c_uint;
    /* Map the channels for playback and capture */
    regmap_write((*i2s).regmap, SUN8I_I2S_TX_CHAN_MAP_REG, 0x76543210);
    regmap_write((*i2s).regmap, SUN8I_I2S_RX_CHAN_MAP_REG, 0x76543210);
    /* Configure the channels */
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_TX_CHAN_SEL_REG, SUN4I_I2S_CHAN_SEL_MASK, SUN4I_I2S_CHAN_SEL(channels));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_RX_CHAN_SEL_REG, SUN4I_I2S_CHAN_SEL_MASK, SUN4I_I2S_CHAN_SEL(channels));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_CHAN_CFG_REG, SUN8I_I2S_CHAN_CFG_TX_SLOT_NUM_MASK, SUN8I_I2S_CHAN_CFG_TX_SLOT_NUM(channels));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_CHAN_CFG_REG, SUN8I_I2S_CHAN_CFG_RX_SLOT_NUM_MASK, SUN8I_I2S_CHAN_CFG_RX_SLOT_NUM(channels));
    match (*i2s).format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => lrck_period = slot_width * slots,
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_I2S => lrck_period = slot_width,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT0_REG, SUN8I_I2S_FMT0_LRCK_PERIOD_MASK, SUN8I_I2S_FMT0_LRCK_PERIOD(lrck_period));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_TX_CHAN_SEL_REG, SUN8I_I2S_TX_CHAN_EN_MASK, SUN8I_I2S_TX_CHAN_EN(channels));
    0
}

unsafe extern "C" fn sun50i_h6_i2s_set_chan_cfg(i2s: *const sun4i_i2s, channels: c_uint, slots: c_uint, slot_width: c_uint) -> c_int {
    let lrck_period: c_uint;
    /* Map the channels for playback and capture */
    regmap_write((*i2s).regmap, SUN50I_H6_I2S_TX_CHAN_MAP0_REG(0), 0xFEDCBA98);
    regmap_write((*i2s).regmap, SUN50I_H6_I2S_TX_CHAN_MAP1_REG(0), 0x76543210);
    if (*(*i2s).variant).num_din_pins > 1 {
        regmap_write((*i2s).regmap, SUN50I_R329_I2S_RX_CHAN_MAP0_REG, 0x0F0E0D0C);
        regmap_write((*i2s).regmap, SUN50I_R329_I2S_RX_CHAN_MAP1_REG, 0x0B0A0908);
        regmap_write((*i2s).regmap, SUN50I_R329_I2S_RX_CHAN_MAP2_REG, 0x07060504);
        regmap_write((*i2s).regmap, SUN50I_R329_I2S_RX_CHAN_MAP3_REG, 0x03020100);
    } else {
        regmap_write((*i2s).regmap, SUN50I_H6_I2S_RX_CHAN_MAP0_REG, 0xFEDCBA98);
        regmap_write((*i2s).regmap, SUN50I_H6_I2S_RX_CHAN_MAP1_REG, 0x76543210);
    }
    /* Configure the channels */
    regmap_update_bits((*i2s).regmap, SUN50I_H6_I2S_TX_CHAN_SEL_REG(0), SUN50I_H6_I2S_TX_CHAN_SEL_MASK, SUN50I_H6_I2S_TX_CHAN_SEL(channels));
    regmap_update_bits((*i2s).regmap, SUN50I_H6_I2S_RX_CHAN_SEL_REG, SUN50I_H6_I2S_TX_CHAN_SEL_MASK, SUN50I_H6_I2S_TX_CHAN_SEL(channels));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_CHAN_CFG_REG, SUN8I_I2S_CHAN_CFG_TX_SLOT_NUM_MASK, SUN8I_I2S_CHAN_CFG_TX_SLOT_NUM(channels));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_CHAN_CFG_REG, SUN8I_I2S_CHAN_CFG_RX_SLOT_NUM_MASK, SUN8I_I2S_CHAN_CFG_RX_SLOT_NUM(channels));
    match (*i2s).format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => lrck_period = slot_width * slots,
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_I2S => lrck_period = slot_width,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT0_REG, SUN8I_I2S_FMT0_LRCK_PERIOD_MASK, SUN8I_I2S_FMT0_LRCK_PERIOD(lrck_period));
    regmap_update_bits((*i2s).regmap, SUN50I_H6_I2S_TX_CHAN_SEL_REG(0), SUN50I_H6_I2S_TX_CHAN_EN_MASK, SUN50I_H6_I2S_TX_CHAN_EN(channels));
    0
}

unsafe extern "C" fn sun4i_i2s_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    let word_size = params_width(params);
    let mut slot_width = params_physical_width(params);
    let channels = params_channels(params);
    let mut slots = channels;
    let ret: c_int;
    let sr: c_int;
    let wss: c_int;
    let width: u32;

    if (*i2s).slots != 0 {
        slots = (*i2s).slots;
    }
    if (*i2s).slot_width != 0 {
        slot_width = (*i2s).slot_width;
    }
    ret = ((*(*i2s).variant).set_chan_cfg.unwrap())(i2s, channels, slots, slot_width);
    if ret < 0 {
        dev_err((*dai).dev, b"Invalid channel configuration\n\0".as_ptr() as *const c_char);
        return ret;
    }
    /* Set significant bits in our FIFOs */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FIFO_CTRL_REG, SUN4I_I2S_FIFO_CTRL_TX_MODE_MASK | SUN4I_I2S_FIFO_CTRL_RX_MODE_MASK, SUN4I_I2S_FIFO_CTRL_TX_MODE(1) | SUN4I_I2S_FIFO_CTRL_RX_MODE(1));
    match params_physical_width(params) {
        16 => width = DMA_SLAVE_BUSWIDTH_2_BYTES,
        32 => width = DMA_SLAVE_BUSWIDTH_4_BYTES,
        _ => {
            dev_err((*dai).dev, b"Unsupported physical sample width: %d\n\0".as_ptr() as *const c_char, params_physical_width(params));
            return -EINVAL;
        }
    }
    (*i2s).playback_dma_data.addr_width = width;
    sr = ((*(*i2s).variant).get_sr.unwrap())(word_size);
    if sr < 0 {
        return -EINVAL;
    }
    wss = ((*(*i2s).variant).get_wss.unwrap())(slot_width);
    if wss < 0 {
        return -EINVAL;
    }
    regmap_field_write((*i2s).field_fmt_wss, wss as c_uint);
    regmap_field_write((*i2s).field_fmt_sr, sr as c_uint);
    sun4i_i2s_set_clk_rate(dai, params_rate(params), slots, slot_width)
}

unsafe extern "C" fn sun4i_i2s_set_soc_fmt(i2s: *const sun4i_i2s, fmt: c_uint) -> c_int {
    let mut val: u32;
    /* DAI clock polarity */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => val = SUN4I_I2S_FMT0_BCLK_POLARITY_INVERTED | SUN4I_I2S_FMT0_LRCLK_POLARITY_INVERTED,
        SND_SOC_DAIFMT_IB_NF => val = SUN4I_I2S_FMT0_BCLK_POLARITY_INVERTED,
        SND_SOC_DAIFMT_NB_IF => val = SUN4I_I2S_FMT0_LRCLK_POLARITY_INVERTED,
        SND_SOC_DAIFMT_NB_NF => val = 0,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT0_REG, SUN4I_I2S_FMT0_LRCLK_POLARITY_MASK | SUN4I_I2S_FMT0_BCLK_POLARITY_MASK, val);
    /* DAI Mode */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => val = SUN4I_I2S_FMT0_FMT_I2S,
        SND_SOC_DAIFMT_LEFT_J => val = SUN4I_I2S_FMT0_FMT_LEFT_J,
        SND_SOC_DAIFMT_RIGHT_J => val = SUN4I_I2S_FMT0_FMT_RIGHT_J,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT0_REG, SUN4I_I2S_FMT0_FMT_MASK, val);
    /* DAI clock master masks */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => val = SUN4I_I2S_CTRL_MODE_MASTER,
        SND_SOC_DAIFMT_BC_FC => val = SUN4I_I2S_CTRL_MODE_SLAVE,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_MODE_MASK, val);
    0
}

unsafe extern "C" fn sun8i_i2s_set_soc_fmt(i2s: *const sun4i_i2s, fmt: c_uint) -> c_int {
    let mode: u32;
    let mut lrclk_pol: u32;
    let mut bclk_pol: u32;
    let val: u32;
    let offset: u8;
    /* DAI Mode */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_PCM; offset = 1; }
        SND_SOC_DAIFMT_DSP_B => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_PCM; offset = 0; }
        SND_SOC_DAIFMT_I2S => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_LOW; mode = SUN8I_I2S_CTRL_MODE_LEFT; offset = 1; }
        SND_SOC_DAIFMT_LEFT_J => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_LEFT; offset = 0; }
        SND_SOC_DAIFMT_RIGHT_J => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_RIGHT; offset = 0; }
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN8I_I2S_CTRL_MODE_MASK, mode);
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_TX_CHAN_SEL_REG, SUN8I_I2S_TX_CHAN_OFFSET_MASK, SUN8I_I2S_TX_CHAN_OFFSET(offset));
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_RX_CHAN_SEL_REG, SUN8I_I2S_TX_CHAN_OFFSET_MASK, SUN8I_I2S_TX_CHAN_OFFSET(offset));
    /* DAI clock polarity */
    bclk_pol = SUN8I_I2S_FMT0_BCLK_POLARITY_NORMAL;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => { lrclk_pol ^= SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK; bclk_pol = SUN8I_I2S_FMT0_BCLK_POLARITY_INVERTED; }
        SND_SOC_DAIFMT_IB_NF => bclk_pol = SUN8I_I2S_FMT0_BCLK_POLARITY_INVERTED,
        SND_SOC_DAIFMT_NB_IF => lrclk_pol ^= SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK,
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT0_REG, SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK | SUN8I_I2S_FMT0_BCLK_POLARITY_MASK, lrclk_pol | bclk_pol);
    /* DAI clock master masks */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => val = SUN8I_I2S_CTRL_BCLK_OUT | SUN8I_I2S_CTRL_LRCK_OUT,
        SND_SOC_DAIFMT_BC_FC => val = 0,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN8I_I2S_CTRL_BCLK_OUT | SUN8I_I2S_CTRL_LRCK_OUT, val);
    /* Set sign extension to pad out LSB with 0 */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT1_REG, SUN8I_I2S_FMT1_REG_SEXT_MASK, SUN8I_I2S_FMT1_REG_SEXT(0));
    0
}

unsafe extern "C" fn sun50i_h6_i2s_set_soc_fmt(i2s: *const sun4i_i2s, fmt: c_uint) -> c_int {
    let mode: u32;
    let mut lrclk_pol: u32;
    let mut bclk_pol: u32;
    let val: u32;
    let offset: u8;
    /* DAI Mode */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_PCM; offset = 1; }
        SND_SOC_DAIFMT_DSP_B => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_PCM; offset = 0; }
        SND_SOC_DAIFMT_I2S => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_LOW; mode = SUN8I_I2S_CTRL_MODE_LEFT; offset = 1; }
        SND_SOC_DAIFMT_LEFT_J => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_LEFT; offset = 0; }
        SND_SOC_DAIFMT_RIGHT_J => { lrclk_pol = SUN8I_I2S_FMT0_LRCLK_POLARITY_START_HIGH; mode = SUN8I_I2S_CTRL_MODE_RIGHT; offset = 0; }
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN8I_I2S_CTRL_MODE_MASK, mode);
    regmap_update_bits((*i2s).regmap, SUN8I_I2S_TX_CHAN_SEL_REG, SUN50I_H6_I2S_TX_CHAN_SEL_OFFSET_MASK, SUN50I_H6_I2S_TX_CHAN_SEL_OFFSET(offset));
    regmap_update_bits((*i2s).regmap, SUN50I_H6_I2S_RX_CHAN_SEL_REG, SUN50I_H6_I2S_TX_CHAN_SEL_OFFSET_MASK, SUN50I_H6_I2S_TX_CHAN_SEL_OFFSET(offset));
    /* DAI clock polarity */
    bclk_pol = SUN8I_I2S_FMT0_BCLK_POLARITY_NORMAL;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => { lrclk_pol ^= SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK; bclk_pol = SUN8I_I2S_FMT0_BCLK_POLARITY_INVERTED; }
        SND_SOC_DAIFMT_IB_NF => bclk_pol = SUN8I_I2S_FMT0_BCLK_POLARITY_INVERTED,
        SND_SOC_DAIFMT_NB_IF => lrclk_pol ^= SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK,
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT0_REG, SUN8I_I2S_FMT0_LRCLK_POLARITY_MASK | SUN8I_I2S_FMT0_BCLK_POLARITY_MASK, lrclk_pol | bclk_pol);
    /* DAI clock master masks */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => val = SUN8I_I2S_CTRL_BCLK_OUT | SUN8I_I2S_CTRL_LRCK_OUT,
        SND_SOC_DAIFMT_BC_FC => val = 0,
        _ => return -EINVAL,
    }
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN8I_I2S_CTRL_BCLK_OUT | SUN8I_I2S_CTRL_LRCK_OUT, val);
    /* Set sign extension to pad out LSB with 0 */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FMT1_REG, SUN8I_I2S_FMT1_REG_SEXT_MASK, SUN8I_I2S_FMT1_REG_SEXT(0));
    0
}

unsafe extern "C" fn sun4i_i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    let ret = ((*(*i2s).variant).set_fmt.unwrap())(i2s, fmt);
    if ret != 0 {
        dev_err((*dai).dev, b"Unsupported format configuration\n\0".as_ptr() as *const c_char);
        return ret;
    }
    (*i2s).format = fmt;
    0
}

unsafe extern "C" fn sun4i_i2s_start_capture(i2s: *mut sun4i_i2s) {
    /* Flush RX FIFO */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FIFO_CTRL_REG, SUN4I_I2S_FIFO_CTRL_FLUSH_RX, SUN4I_I2S_FIFO_CTRL_FLUSH_RX);
    /* Clear RX counter */
    regmap_write((*i2s).regmap, SUN4I_I2S_RX_CNT_REG, 0);
    /* Enable RX Block */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_RX_EN, SUN4I_I2S_CTRL_RX_EN);
    /* Enable RX DRQ */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_DMA_INT_CTRL_REG, SUN4I_I2S_DMA_INT_CTRL_RX_DRQ_EN, SUN4I_I2S_DMA_INT_CTRL_RX_DRQ_EN);
}

unsafe extern "C" fn sun4i_i2s_start_playback(i2s: *mut sun4i_i2s) {
    /* Flush TX FIFO */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_FIFO_CTRL_REG, SUN4I_I2S_FIFO_CTRL_FLUSH_TX, SUN4I_I2S_FIFO_CTRL_FLUSH_TX);
    /* Clear TX counter */
    regmap_write((*i2s).regmap, SUN4I_I2S_TX_CNT_REG, 0);
    /* Enable TX Block */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_TX_EN, SUN4I_I2S_CTRL_TX_EN);
    /* Enable TX DRQ */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_DMA_INT_CTRL_REG, SUN4I_I2S_DMA_INT_CTRL_TX_DRQ_EN, SUN4I_I2S_DMA_INT_CTRL_TX_DRQ_EN);
}

unsafe extern "C" fn sun4i_i2s_stop_capture(i2s: *mut sun4i_i2s) {
    /* Disable RX Block */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_RX_EN, 0);
    /* Disable RX DRQ */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_DMA_INT_CTRL_REG, SUN4I_I2S_DMA_INT_CTRL_RX_DRQ_EN, 0);
}

unsafe extern "C" fn sun4i_i2s_stop_playback(i2s: *mut sun4i_i2s) {
    /* Disable TX Block */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_TX_EN, 0);
    /* Disable TX DRQ */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_DMA_INT_CTRL_REG, SUN4I_I2S_DMA_INT_CTRL_TX_DRQ_EN, 0);
}

unsafe extern "C" fn sun4i_i2s_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                sun4i_i2s_start_playback(i2s);
            } else {
                sun4i_i2s_start_capture(i2s);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                sun4i_i2s_stop_playback(i2s);
            } else {
                sun4i_i2s_stop_capture(i2s);
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn sun4i_i2s_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    if clk_id != 0 {
        return -EINVAL;
    }
    (*i2s).mclk_freq = freq;
    0
}

unsafe extern "C" fn sun4i_i2s_set_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    if slots > 8 {
        return -EINVAL;
    }
    (*i2s).slots = slots as c_uint;
    (*i2s).slot_width = slot_width as c_uint;
    0
}

unsafe extern "C" fn sun4i_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    snd_soc_dai_init_dma_data(dai, &mut (*i2s).playback_dma_data, &mut (*i2s).capture_dma_data);
    0
}

unsafe extern "C" fn sun4i_i2s_dai_startup(sub: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut sun4i_i2s;
    let runtime = (*sub).runtime;
    snd_pcm_hw_constraint_mask64(runtime, SNDRV_PCM_HW_PARAM_FORMAT, (*(*i2s).variant).pcm_formats)
}

static sun4i_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(sun4i_i2s_dai_probe),
    startup: Some(sun4i_i2s_dai_startup),
    hw_params: Some(sun4i_i2s_hw_params),
    set_fmt: Some(sun4i_i2s_set_fmt),
    set_sysclk: Some(sun4i_i2s_set_sysclk),
    set_tdm_slot: Some(sun4i_i2s_set_tdm_slot),
    trigger: Some(sun4i_i2s_trigger),
};

const SUN4I_FORMATS_ALL: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut sun4i_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SUN4I_FORMATS_ALL,
    },
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SUN4I_FORMATS_ALL,
    },
    ops: &sun4i_i2s_dai_ops,
    symmetric_rate: 1,
};

static sun4i_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"sun4i-dai\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn sun4i_i2s_rd_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SUN4I_I2S_FIFO_TX_REG => false,
        _ => true,
    }
}

unsafe extern "C" fn sun4i_i2s_wr_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SUN4I_I2S_FIFO_RX_REG | SUN4I_I2S_FIFO_STA_REG => false,
        _ => true,
    }
}

unsafe extern "C" fn sun4i_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SUN4I_I2S_FIFO_RX_REG | SUN4I_I2S_INT_STA_REG | SUN4I_I2S_RX_CNT_REG | SUN4I_I2S_TX_CNT_REG => true,
        _ => false,
    }
}

unsafe extern "C" fn sun8i_i2s_rd_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SUN8I_I2S_FIFO_TX_REG => false,
        _ => true,
    }
}

unsafe extern "C" fn sun8i_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SUN4I_I2S_FIFO_CTRL_REG | SUN4I_I2S_FIFO_RX_REG | SUN4I_I2S_FIFO_STA_REG | SUN4I_I2S_RX_CNT_REG | SUN4I_I2S_TX_CNT_REG | SUN8I_I2S_FIFO_TX_REG | SUN8I_I2S_INT_STA_REG => true,
        _ => false,
    }
}

static sun4i_i2s_reg_defaults: [reg_default; 10] = [
    reg_default { reg: SUN4I_I2S_CTRL_REG, def: 0x00000000 },
    reg_default { reg: SUN4I_I2S_FMT0_REG, def: 0x0000000c },
    reg_default { reg: SUN4I_I2S_FMT1_REG, def: 0x00004020 },
    reg_default { reg: SUN4I_I2S_FIFO_CTRL_REG, def: 0x000400f0 },
    reg_default { reg: SUN4I_I2S_DMA_INT_CTRL_REG, def: 0x00000000 },
    reg_default { reg: SUN4I_I2S_CLK_DIV_REG, def: 0x00000000 },
    reg_default { reg: SUN4I_I2S_TX_CHAN_SEL_REG, def: 0x00000001 },
    reg_default { reg: SUN4I_I2S_TX_CHAN_MAP_REG, def: 0x76543210 },
    reg_default { reg: SUN4I_I2S_RX_CHAN_SEL_REG, def: 0x00000001 },
    reg_default { reg: SUN4I_I2S_RX_CHAN_MAP_REG, def: 0x00003210 },
];

static sun8i_i2s_reg_defaults: [reg_default; 11] = [
    reg_default { reg: SUN4I_I2S_CTRL_REG, def: 0x00060000 },
    reg_default { reg: SUN4I_I2S_FMT0_REG, def: 0x00000033 },
    reg_default { reg: SUN4I_I2S_FMT1_REG, def: 0x00000030 },
    reg_default { reg: SUN4I_I2S_FIFO_CTRL_REG, def: 0x000400f0 },
    reg_default { reg: SUN4I_I2S_DMA_INT_CTRL_REG, def: 0x00000000 },
    reg_default { reg: SUN4I_I2S_CLK_DIV_REG, def: 0x00000000 },
    reg_default { reg: SUN8I_I2S_CHAN_CFG_REG, def: 0x00000000 },
    reg_default { reg: SUN8I_I2S_TX_CHAN_SEL_REG, def: 0x00000000 },
    reg_default { reg: SUN8I_I2S_TX_CHAN_MAP_REG, def: 0x00000000 },
    reg_default { reg: SUN8I_I2S_RX_CHAN_SEL_REG, def: 0x00000000 },
    reg_default { reg: SUN8I_I2S_RX_CHAN_MAP_REG, def: 0x00000000 },
];

static sun50i_h6_i2s_reg_defaults: [reg_default; 13] = [
    reg_default { reg: SUN4I_I2S_CTRL_REG, def: 0x00060000 },
    reg_default { reg: SUN4I_I2S_FMT0_REG, def: 0x00000033 },
    reg_default { reg: SUN4I_I2S_FMT1_REG, def: 0x00000030 },
    reg_default { reg: SUN4I_I2S_FIFO_CTRL_REG, def: 0x000400f0 },
    reg_default { reg: SUN4I_I2S_DMA_INT_CTRL_REG, def: 0x00000000 },
    reg_default { reg: SUN4I_I2S_CLK_DIV_REG, def: 0x00000000 },
    reg_default { reg: SUN8I_I2S_CHAN_CFG_REG, def: 0x00000000 },
    reg_default { reg: SUN50I_H6_I2S_TX_CHAN_SEL_REG(0), def: 0x00000000 },
    reg_default { reg: SUN50I_H6_I2S_TX_CHAN_MAP0_REG(0), def: 0x00000000 },
    reg_default { reg: SUN50I_H6_I2S_TX_CHAN_MAP1_REG(0), def: 0x00000000 },
    reg_default { reg: SUN50I_H6_I2S_RX_CHAN_SEL_REG, def: 0x00000000 },
    reg_default { reg: SUN50I_H6_I2S_RX_CHAN_MAP0_REG, def: 0x00000000 },
    reg_default { reg: SUN50I_H6_I2S_RX_CHAN_MAP1_REG, def: 0x00000000 },
];

static sun4i_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN4I_I2S_RX_CHAN_MAP_REG,
    cache_type: REGCACHE_FLAT, reg_defaults: sun4i_i2s_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE(&sun4i_i2s_reg_defaults),
    writeable_reg: Some(sun4i_i2s_wr_reg), readable_reg: Some(sun4i_i2s_rd_reg), volatile_reg: Some(sun4i_i2s_volatile_reg),
};

static sun8i_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN8I_I2S_RX_CHAN_MAP_REG,
    cache_type: REGCACHE_FLAT, reg_defaults: sun8i_i2s_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE(&sun8i_i2s_reg_defaults),
    writeable_reg: Some(sun4i_i2s_wr_reg), readable_reg: Some(sun8i_i2s_rd_reg), volatile_reg: Some(sun8i_i2s_volatile_reg),
};

static sun50i_h6_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN50I_R329_I2S_RX_CHAN_MAP3_REG,
    cache_type: REGCACHE_FLAT, reg_defaults: sun50i_h6_i2s_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE(&sun50i_h6_i2s_reg_defaults),
    writeable_reg: Some(sun4i_i2s_wr_reg), readable_reg: Some(sun8i_i2s_rd_reg), volatile_reg: Some(sun8i_i2s_volatile_reg),
};

unsafe extern "C" fn sun4i_i2s_runtime_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut sun4i_i2s;
    let mut ret: c_int;
    ret = clk_prepare_enable((*i2s).bus_clk);
    if ret != 0 {
        dev_err(dev, b"Failed to enable bus clock\n\0".as_ptr() as *const c_char);
        return ret;
    }
    regcache_cache_only((*i2s).regmap, false);
    regcache_mark_dirty((*i2s).regmap);
    ret = regcache_sync((*i2s).regmap);
    if ret != 0 {
        dev_err(dev, b"Failed to sync regmap cache\n\0".as_ptr() as *const c_char);
        clk_disable_unprepare((*i2s).bus_clk);
        return ret;
    }
    /* Enable the whole hardware block */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_GL_EN, SUN4I_I2S_CTRL_GL_EN);
    /* Enable the first output line */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_SDO_EN_MASK, SUN4I_I2S_CTRL_SDO_EN(0));
    ret = clk_prepare_enable((*i2s).mod_clk);
    if ret != 0 {
        dev_err(dev, b"Failed to enable module clock\n\0".as_ptr() as *const c_char);
        clk_disable_unprepare((*i2s).bus_clk);
        return ret;
    }
    0
}

unsafe extern "C" fn sun4i_i2s_runtime_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut sun4i_i2s;
    clk_disable_unprepare((*i2s).mod_clk);
    /* Disable our output lines */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_SDO_EN_MASK, 0);
    /* Disable the whole hardware block */
    regmap_update_bits((*i2s).regmap, SUN4I_I2S_CTRL_REG, SUN4I_I2S_CTRL_GL_EN, 0);
    regcache_cache_only((*i2s).regmap, true);
    clk_disable_unprepare((*i2s).bus_clk);
    0
}

const SUN4I_FORMATS_A10: u64 = SUN4I_FORMATS_ALL & !SNDRV_PCM_FMTBIT_S32_LE;
const SUN4I_FORMATS_H3: u64 = SUN4I_FORMATS_ALL;

const fn quirks(has_reset: bool_, pcm_formats: u64, reg_offset_txdata: c_uint, regmap_cfg: *const regmap_config, mclk: reg_field, wss: reg_field, sr: reg_field, din: c_uint, dout: c_uint, bdiv: *const sun4i_i2s_clk_div, nbdiv: c_uint, mdiv: *const sun4i_i2s_clk_div, nmdiv: c_uint, get_parent: unsafe extern "C" fn(*const sun4i_i2s) -> c_ulong, get_sr: unsafe extern "C" fn(c_uint) -> c_int, get_wss: unsafe extern "C" fn(c_uint) -> c_int, set_chan: unsafe extern "C" fn(*const sun4i_i2s, c_uint, c_uint, c_uint) -> c_int, set_fmt: unsafe extern "C" fn(*const sun4i_i2s, c_uint) -> c_int) -> sun4i_i2s_quirks {
    sun4i_i2s_quirks {
        has_reset, pcm_formats, reg_offset_txdata, sun4i_i2s_regmap: regmap_cfg,
        field_clkdiv_mclk_en: mclk, field_fmt_wss: wss, field_fmt_sr: sr,
        num_din_pins: din, num_dout_pins: dout,
        bclk_dividers: bdiv, num_bclk_dividers: nbdiv, mclk_dividers: mdiv, num_mclk_dividers: nmdiv,
        get_bclk_parent_rate: Some(get_parent), get_sr: Some(get_sr), get_wss: Some(get_wss),
        set_chan_cfg: Some(set_chan), set_fmt: Some(set_fmt),
    }
}

static sun4i_a10_i2s_quirks: sun4i_i2s_quirks = quirks(false, SUN4I_FORMATS_A10, SUN4I_I2S_FIFO_TX_REG, &sun4i_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 7, 7), REG_FIELD(SUN4I_I2S_FMT0_REG, 2, 3), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 5), 0, 0, sun4i_i2s_bclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_bclk_div), sun4i_i2s_mclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_mclk_div), sun4i_i2s_get_bclk_parent_rate, sun4i_i2s_get_sr, sun4i_i2s_get_wss, sun4i_i2s_set_chan_cfg, sun4i_i2s_set_soc_fmt);
static sun6i_a31_i2s_quirks: sun4i_i2s_quirks = quirks(true, SUN4I_FORMATS_A10, SUN4I_I2S_FIFO_TX_REG, &sun4i_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 7, 7), REG_FIELD(SUN4I_I2S_FMT0_REG, 2, 3), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 5), 0, 0, sun4i_i2s_bclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_bclk_div), sun4i_i2s_mclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_mclk_div), sun4i_i2s_get_bclk_parent_rate, sun4i_i2s_get_sr, sun4i_i2s_get_wss, sun4i_i2s_set_chan_cfg, sun4i_i2s_set_soc_fmt);

/*
 * This doesn't describe the TDM controller documented in the A83t
 * datasheet, but the three undocumented I2S controller that use the
 * older design.
 */
static sun8i_a83t_i2s_quirks: sun4i_i2s_quirks = quirks(true, SUN4I_FORMATS_A10, SUN8I_I2S_FIFO_TX_REG, &sun4i_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 7, 7), REG_FIELD(SUN4I_I2S_FMT0_REG, 2, 3), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 5), 0, 0, sun4i_i2s_bclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_bclk_div), sun4i_i2s_mclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_mclk_div), sun4i_i2s_get_bclk_parent_rate, sun4i_i2s_get_sr, sun4i_i2s_get_wss, sun4i_i2s_set_chan_cfg, sun4i_i2s_set_soc_fmt);
static sun8i_h3_i2s_quirks: sun4i_i2s_quirks = quirks(true, SUN4I_FORMATS_H3, SUN8I_I2S_FIFO_TX_REG, &sun8i_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 8, 8), REG_FIELD(SUN4I_I2S_FMT0_REG, 0, 2), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 6), 0, 0, sun8i_i2s_clk_div.as_ptr(), ARRAY_SIZE(&sun8i_i2s_clk_div), sun8i_i2s_clk_div.as_ptr(), ARRAY_SIZE(&sun8i_i2s_clk_div), sun8i_i2s_get_bclk_parent_rate, sun8i_i2s_get_sr_wss, sun8i_i2s_get_sr_wss, sun8i_i2s_set_chan_cfg, sun8i_i2s_set_soc_fmt);
static sun50i_a64_codec_i2s_quirks: sun4i_i2s_quirks = quirks(true, SUN4I_FORMATS_H3, SUN8I_I2S_FIFO_TX_REG, &sun4i_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 7, 7), REG_FIELD(SUN4I_I2S_FMT0_REG, 2, 3), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 5), 0, 0, sun4i_i2s_bclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_bclk_div), sun4i_i2s_mclk_div.as_ptr(), ARRAY_SIZE(&sun4i_i2s_mclk_div), sun4i_i2s_get_bclk_parent_rate, sun4i_i2s_get_sr, sun4i_i2s_get_wss, sun4i_i2s_set_chan_cfg, sun4i_i2s_set_soc_fmt);
static sun50i_h6_i2s_quirks: sun4i_i2s_quirks = quirks(true, SUN4I_FORMATS_H3, SUN8I_I2S_FIFO_TX_REG, &sun50i_h6_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 8, 8), REG_FIELD(SUN4I_I2S_FMT0_REG, 0, 2), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 6), 0, 0, sun8i_i2s_clk_div.as_ptr(), ARRAY_SIZE(&sun8i_i2s_clk_div), sun8i_i2s_clk_div.as_ptr(), ARRAY_SIZE(&sun8i_i2s_clk_div), sun8i_i2s_get_bclk_parent_rate, sun8i_i2s_get_sr_wss, sun8i_i2s_get_sr_wss, sun50i_h6_i2s_set_chan_cfg, sun50i_h6_i2s_set_soc_fmt);
static sun50i_r329_i2s_quirks: sun4i_i2s_quirks = quirks(true, SUN4I_FORMATS_H3, SUN8I_I2S_FIFO_TX_REG, &sun50i_h6_i2s_regmap_config, REG_FIELD(SUN4I_I2S_CLK_DIV_REG, 8, 8), REG_FIELD(SUN4I_I2S_FMT0_REG, 0, 2), REG_FIELD(SUN4I_I2S_FMT0_REG, 4, 6), 4, 4, sun8i_i2s_clk_div.as_ptr(), ARRAY_SIZE(&sun8i_i2s_clk_div), sun8i_i2s_clk_div.as_ptr(), ARRAY_SIZE(&sun8i_i2s_clk_div), sun8i_i2s_get_bclk_parent_rate, sun8i_i2s_get_sr_wss, sun8i_i2s_get_sr_wss, sun50i_h6_i2s_set_chan_cfg, sun50i_h6_i2s_set_soc_fmt);

unsafe extern "C" fn sun4i_i2s_init_regmap_fields(dev: *mut device, i2s: *mut sun4i_i2s) -> c_int {
    (*i2s).field_clkdiv_mclk_en = devm_regmap_field_alloc(dev, (*i2s).regmap, (*(*i2s).variant).field_clkdiv_mclk_en);
    if IS_ERR((*i2s).field_clkdiv_mclk_en as *const c_void) {
        return PTR_ERR((*i2s).field_clkdiv_mclk_en as *const c_void);
    }
    (*i2s).field_fmt_wss = devm_regmap_field_alloc(dev, (*i2s).regmap, (*(*i2s).variant).field_fmt_wss);
    if IS_ERR((*i2s).field_fmt_wss as *const c_void) {
        return PTR_ERR((*i2s).field_fmt_wss as *const c_void);
    }
    (*i2s).field_fmt_sr = devm_regmap_field_alloc(dev, (*i2s).regmap, (*(*i2s).variant).field_fmt_sr);
    if IS_ERR((*i2s).field_fmt_sr as *const c_void) {
        return PTR_ERR((*i2s).field_fmt_sr as *const c_void);
    }
    0
}

unsafe extern "C" fn sun4i_i2s_probe(pdev: *mut platform_device) -> c_int {
    let mut i2s: *mut sun4i_i2s;
    let mut res: *mut resource = core::ptr::null_mut();
    let regs: *mut c_void;
    let irq: c_int;
    let mut ret: c_int;

    i2s = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<sun4i_i2s>(), GFP_KERNEL) as *mut sun4i_i2s;
    if i2s.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, i2s as *mut c_void);
    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }
    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }
    (*i2s).variant = of_device_get_match_data(&mut (*pdev).dev) as *const sun4i_i2s_quirks;
    if (*i2s).variant.is_null() {
        dev_err(&mut (*pdev).dev, b"Failed to determine the quirks to use\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    (*i2s).bus_clk = devm_clk_get(&mut (*pdev).dev, b"apb\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).bus_clk as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*i2s).bus_clk as *const c_void), b"Can't get our bus clock\n\0".as_ptr() as *const c_char);
    }
    (*i2s).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, (*(*i2s).variant).sun4i_i2s_regmap);
    if IS_ERR((*i2s).regmap as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*i2s).regmap as *const c_void), b"Regmap initialisation failed\n\0".as_ptr() as *const c_char);
    }
    (*i2s).mod_clk = devm_clk_get(&mut (*pdev).dev, b"mod\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).mod_clk as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*i2s).mod_clk as *const c_void), b"Can't get our mod clock\n\0".as_ptr() as *const c_char);
    }
    if (*(*i2s).variant).has_reset {
        (*i2s).rst = devm_reset_control_get_exclusive(&mut (*pdev).dev, core::ptr::null());
        if IS_ERR((*i2s).rst as *const c_void) {
            return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*i2s).rst as *const c_void), b"Failed to get reset control\n\0".as_ptr() as *const c_char);
        }
    }
    if !IS_ERR((*i2s).rst as *const c_void) {
        ret = reset_control_deassert((*i2s).rst);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"Failed to deassert the reset control\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }
    (*i2s).playback_dma_data.addr = (*res).start + (*(*i2s).variant).reg_offset_txdata as c_ulong;
    (*i2s).playback_dma_data.maxburst = 8;
    (*i2s).capture_dma_data.addr = (*res).start + SUN4I_I2S_FIFO_RX_REG as c_ulong;
    (*i2s).capture_dma_data.maxburst = 8;
    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = sun4i_i2s_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            pm_runtime_disable(&mut (*pdev).dev);
            if !IS_ERR((*i2s).rst as *const c_void) {
                reset_control_assert((*i2s).rst);
            }
            return ret;
        }
    }
    ret = sun4i_i2s_init_regmap_fields(&mut (*pdev).dev, i2s);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"Could not initialise regmap fields\n\0".as_ptr() as *const c_char);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) { sun4i_i2s_runtime_suspend(&mut (*pdev).dev); }
        pm_runtime_disable(&mut (*pdev).dev);
        if !IS_ERR((*i2s).rst as *const c_void) { reset_control_assert((*i2s).rst); }
        return ret;
    }
    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, core::ptr::null(), 0);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"Could not register PCM\n\0".as_ptr() as *const c_char);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) { sun4i_i2s_runtime_suspend(&mut (*pdev).dev); }
        pm_runtime_disable(&mut (*pdev).dev);
        if !IS_ERR((*i2s).rst as *const c_void) { reset_control_assert((*i2s).rst); }
        return ret;
    }
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &sun4i_i2s_component, &mut sun4i_i2s_dai, 1);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"Could not register DAI\n\0".as_ptr() as *const c_char);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) { sun4i_i2s_runtime_suspend(&mut (*pdev).dev); }
        pm_runtime_disable(&mut (*pdev).dev);
        if !IS_ERR((*i2s).rst as *const c_void) { reset_control_assert((*i2s).rst); }
        return ret;
    }
    0
}

unsafe extern "C" fn sun4i_i2s_remove(pdev: *mut platform_device) {
    let i2s = dev_get_drvdata(&mut (*pdev).dev) as *mut sun4i_i2s;
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        sun4i_i2s_runtime_suspend(&mut (*pdev).dev);
    }
    if !IS_ERR((*i2s).rst as *const c_void) {
        reset_control_assert((*i2s).rst);
    }
}

static sun4i_i2s_match: [of_device_id; 8] = [
    of_device_id { compatible: b"allwinner,sun4i-a10-i2s\0".as_ptr() as *const c_char, data: &sun4i_a10_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun6i-a31-i2s\0".as_ptr() as *const c_char, data: &sun6i_a31_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun8i-a83t-i2s\0".as_ptr() as *const c_char, data: &sun8i_a83t_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun8i-h3-i2s\0".as_ptr() as *const c_char, data: &sun8i_h3_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun50i-a64-codec-i2s\0".as_ptr() as *const c_char, data: &sun50i_a64_codec_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun50i-h6-i2s\0".as_ptr() as *const c_char, data: &sun50i_h6_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun50i-r329-i2s\0".as_ptr() as *const c_char, data: &sun50i_r329_i2s_quirks as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, sun4i_i2s_match); */

static sun4i_i2s_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_resume: Some(sun4i_i2s_runtime_resume),
    runtime_suspend: Some(sun4i_i2s_runtime_suspend),
};

static mut sun4i_i2s_driver: platform_driver = platform_driver {
    probe: Some(sun4i_i2s_probe),
    remove: Some(sun4i_i2s_remove),
    driver: device_driver {
        name: b"sun4i-i2s\0".as_ptr() as *const c_char,
        of_match_table: sun4i_i2s_match.as_ptr(),
        pm: &sun4i_i2s_pm_ops,
    },
};
/* module_platform_driver(sun4i_i2s_driver); */

/* MODULE_AUTHOR("Andrea Venturi <be17068@iperbole.bo.it>"); */
/* MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>"); */
/* MODULE_DESCRIPTION("Allwinner A10 I2S driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
