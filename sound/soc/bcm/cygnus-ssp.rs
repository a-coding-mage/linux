// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014-2015 Broadcom Corporation
//
// Rust translation of soc/bcm/cygnus-ssp.c.
// Original C includes:
// <linux/clk.h>, <linux/delay.h>, <linux/init.h>, <linux/io.h>,
// <linux/module.h>, <linux/of.h>, <linux/slab.h>, <sound/core.h>,
// <sound/pcm.h>, <sound/pcm_params.h>, <sound/soc.h>, <sound/soc-dai.h>,
// and "cygnus-ssp.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u32 = u32;
type u64 = u64;
type bool_t = bool;

const DEFAULT_VCO: u32 = 1354750204;

const CAPTURE_FCI_ID_BASE: u32 = 0x180;
const CYGNUS_SSP_TRISTATE_MASK: u32 = 0x001fff;
const CYGNUS_PLLCLKSEL_MASK: u32 = 0xf;

/* Used with stream_on field to indicate which streams are active */
const PLAYBACK_STREAM_MASK: u32 = bit(0);
const CAPTURE_STREAM_MASK: u32 = bit(1);

const I2S_STREAM_CFG_MASK: u32 = 0xff003ff;
const I2S_CAP_STREAM_CFG_MASK: u32 = 0xf0;
const SPDIF_STREAM_CFG_MASK: u32 = 0x3ff;
const CH_GRP_STEREO: u32 = 0x1;

/* Begin register offset defines */
const AUD_MISC_SEROUT_OE_REG_BASE: usize = 0x01c;
const AUD_MISC_SEROUT_SPDIF_OE: u32 = 12;
const AUD_MISC_SEROUT_MCLK_OE: u32 = 3;
const AUD_MISC_SEROUT_LRCK_OE: u32 = 2;
const AUD_MISC_SEROUT_SCLK_OE: u32 = 1;
const AUD_MISC_SEROUT_SDAT_OE: u32 = 0;

/* AUD_FMM_BF_CTRL_xxx regs */
const BF_DST_CFG0_OFFSET: usize = 0x100;
const BF_DST_CFG1_OFFSET: usize = 0x104;
const BF_DST_CFG2_OFFSET: usize = 0x108;

const BF_DST_CTRL0_OFFSET: usize = 0x130;
const BF_DST_CTRL1_OFFSET: usize = 0x134;
const BF_DST_CTRL2_OFFSET: usize = 0x138;

const BF_SRC_CFG0_OFFSET: usize = 0x148;
const BF_SRC_CFG1_OFFSET: usize = 0x14c;
const BF_SRC_CFG2_OFFSET: usize = 0x150;
const BF_SRC_CFG3_OFFSET: usize = 0x154;

const BF_SRC_CTRL0_OFFSET: usize = 0x1c0;
const BF_SRC_CTRL1_OFFSET: usize = 0x1c4;
const BF_SRC_CTRL2_OFFSET: usize = 0x1c8;
const BF_SRC_CTRL3_OFFSET: usize = 0x1cc;

const BF_SRC_GRP0_OFFSET: usize = 0x1fc;
const BF_SRC_GRP1_OFFSET: usize = 0x200;
const BF_SRC_GRP2_OFFSET: usize = 0x204;
const BF_SRC_GRP3_OFFSET: usize = 0x208;

const BF_SRC_GRP_EN_OFFSET: usize = 0x320;
const BF_SRC_GRP_FLOWON_OFFSET: usize = 0x324;
const BF_SRC_GRP_SYNC_DIS_OFFSET: usize = 0x328;

/* AUD_FMM_IOP_OUT_I2S_xxx regs */
const OUT_I2S_0_STREAM_CFG_OFFSET: usize = 0xa00;
const OUT_I2S_0_CFG_OFFSET: usize = 0xa04;
const OUT_I2S_0_MCLK_CFG_OFFSET: usize = 0xa0c;

const OUT_I2S_1_STREAM_CFG_OFFSET: usize = 0xa40;
const OUT_I2S_1_CFG_OFFSET: usize = 0xa44;
const OUT_I2S_1_MCLK_CFG_OFFSET: usize = 0xa4c;

const OUT_I2S_2_STREAM_CFG_OFFSET: usize = 0xa80;
const OUT_I2S_2_CFG_OFFSET: usize = 0xa84;
const OUT_I2S_2_MCLK_CFG_OFFSET: usize = 0xa8c;

/* AUD_FMM_IOP_OUT_SPDIF_xxx regs */
const SPDIF_STREAM_CFG_OFFSET: usize = 0xac0;
const SPDIF_CTRL_OFFSET: usize = 0xac4;
const SPDIF_FORMAT_CFG_OFFSET: usize = 0xad8;
const SPDIF_MCLK_CFG_OFFSET: usize = 0xadc;

/* AUD_FMM_IOP_PLL_0_xxx regs */
const IOP_PLL_0_MACRO_OFFSET: usize = 0xb00;
const IOP_PLL_0_MDIV_Ch0_OFFSET: usize = 0xb14;
const IOP_PLL_0_MDIV_Ch1_OFFSET: usize = 0xb18;
const IOP_PLL_0_MDIV_Ch2_OFFSET: usize = 0xb1c;

const IOP_PLL_0_ACTIVE_MDIV_Ch0_OFFSET: usize = 0xb30;
const IOP_PLL_0_ACTIVE_MDIV_Ch1_OFFSET: usize = 0xb34;
const IOP_PLL_0_ACTIVE_MDIV_Ch2_OFFSET: usize = 0xb38;

/* AUD_FMM_IOP_xxx regs */
const IOP_PLL_0_CONTROL_OFFSET: usize = 0xb04;
const IOP_PLL_0_USER_NDIV_OFFSET: usize = 0xb08;
const IOP_PLL_0_ACTIVE_NDIV_OFFSET: usize = 0xb20;
const IOP_PLL_0_RESET_OFFSET: usize = 0xb5c;

/* AUD_FMM_IOP_IN_I2S_xxx regs */
const IN_I2S_0_STREAM_CFG_OFFSET: usize = 0x00;
const IN_I2S_0_CFG_OFFSET: usize = 0x04;
const IN_I2S_1_STREAM_CFG_OFFSET: usize = 0x40;
const IN_I2S_1_CFG_OFFSET: usize = 0x44;
const IN_I2S_2_STREAM_CFG_OFFSET: usize = 0x80;
const IN_I2S_2_CFG_OFFSET: usize = 0x84;

/* AUD_FMM_IOP_MISC_xxx regs */
const IOP_SW_INIT_LOGIC: usize = 0x1c0;

/* End register offset defines */

/* AUD_FMM_IOP_OUT_I2S_x_MCLK_CFG_0_REG */
const I2S_OUT_MCLKRATE_SHIFT: u32 = 16;

/* AUD_FMM_IOP_OUT_I2S_x_MCLK_CFG_REG */
const I2S_OUT_PLLCLKSEL_SHIFT: u32 = 0;

/* AUD_FMM_IOP_OUT_I2S_x_STREAM_CFG */
const I2S_OUT_STREAM_ENA: u32 = 31;
const I2S_OUT_STREAM_CFG_GROUP_ID: u32 = 20;
const I2S_OUT_STREAM_CFG_CHANNEL_GROUPING: u32 = 24;

/* AUD_FMM_IOP_IN_I2S_x_CAP */
const I2S_IN_STREAM_CFG_CAP_ENA: u32 = 31;
const I2S_IN_STREAM_CFG_0_GROUP_ID: u32 = 4;

/* AUD_FMM_IOP_OUT_I2S_x_I2S_CFG_REG */
const I2S_OUT_CFGX_CLK_ENA: u32 = 0;
const I2S_OUT_CFGX_DATA_ENABLE: u32 = 1;
const I2S_OUT_CFGX_DATA_ALIGNMENT: u32 = 6;
const I2S_OUT_CFGX_BITS_PER_SLOT: u32 = 13;
const I2S_OUT_CFGX_VALID_SLOT: u32 = 14;
const I2S_OUT_CFGX_FSYNC_WIDTH: u32 = 18;
const I2S_OUT_CFGX_SCLKS_PER_1FS_DIV32: u32 = 26;
const I2S_OUT_CFGX_SLAVE_MODE: u32 = 30;
const I2S_OUT_CFGX_TDM_MODE: u32 = 31;

/* AUD_FMM_BF_CTRL_SOURCECH_CFGx_REG */
const BF_SRC_CFGX_SFIFO_ENA: u32 = 0;
const BF_SRC_CFGX_BUFFER_PAIR_ENABLE: u32 = 1;
const BF_SRC_CFGX_SAMPLE_CH_MODE: u32 = 2;
const BF_SRC_CFGX_SFIFO_SZ_DOUBLE: u32 = 5;
const BF_SRC_CFGX_NOT_PAUSE_WHEN_EMPTY: u32 = 10;
const BF_SRC_CFGX_BIT_RES: u32 = 20;
const BF_SRC_CFGX_PROCESS_SEQ_ID_VALID: u32 = 31;

/* AUD_FMM_BF_CTRL_DESTCH_CFGx_REG */
const BF_DST_CFGX_CAP_ENA: u32 = 0;
const BF_DST_CFGX_BUFFER_PAIR_ENABLE: u32 = 1;
const BF_DST_CFGX_DFIFO_SZ_DOUBLE: u32 = 2;
const BF_DST_CFGX_NOT_PAUSE_WHEN_FULL: u32 = 11;
const BF_DST_CFGX_FCI_ID: u32 = 12;
const BF_DST_CFGX_CAP_MODE: u32 = 24;
const BF_DST_CFGX_PROC_SEQ_ID_VALID: u32 = 31;

/* AUD_FMM_IOP_OUT_SPDIF_xxx */
const SPDIF_0_OUT_DITHER_ENA: u32 = 3;
const SPDIF_0_OUT_STREAM_ENA: u32 = 31;

/* AUD_FMM_IOP_PLL_0_USER */
const IOP_PLL_0_USER_NDIV_FRAC: u32 = 10;

/* AUD_FMM_IOP_PLL_0_ACTIVE */
const IOP_PLL_0_ACTIVE_NDIV_FRAC: u32 = 10;

const CYGNUS_RATE_MIN: u32 = 8000;
const CYGNUS_RATE_MAX: u32 = 384000;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: u32 = 0;
const PROP_LEN_MAX: usize = 32;
const CYGNUS_MAX_PORTS: usize = 4;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_RATE_KNOT: u32 = 1 << 31;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S32_LE;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x3000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x1000;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 1 << 2;

#[inline]
const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

#[repr(C)]
#[derive(Copy, Clone)]
struct pll_macro_entry {
    mclk: u32,
    pll_ch_num: u32,
}

/*
 * PLL has 3 output channels (1x, 2x, and 4x). Below are
 * the common MCLK frequencies used by audio driver
 */
static pll_predef_mclk: [pll_macro_entry; 18] = [
    pll_macro_entry { mclk: 4096000, pll_ch_num: 0 },
    pll_macro_entry { mclk: 8192000, pll_ch_num: 1 },
    pll_macro_entry { mclk: 16384000, pll_ch_num: 2 },
    pll_macro_entry { mclk: 5644800, pll_ch_num: 0 },
    pll_macro_entry { mclk: 11289600, pll_ch_num: 1 },
    pll_macro_entry { mclk: 22579200, pll_ch_num: 2 },
    pll_macro_entry { mclk: 6144000, pll_ch_num: 0 },
    pll_macro_entry { mclk: 12288000, pll_ch_num: 1 },
    pll_macro_entry { mclk: 24576000, pll_ch_num: 2 },
    pll_macro_entry { mclk: 12288000, pll_ch_num: 0 },
    pll_macro_entry { mclk: 24576000, pll_ch_num: 1 },
    pll_macro_entry { mclk: 49152000, pll_ch_num: 2 },
    pll_macro_entry { mclk: 22579200, pll_ch_num: 0 },
    pll_macro_entry { mclk: 45158400, pll_ch_num: 1 },
    pll_macro_entry { mclk: 90316800, pll_ch_num: 2 },
    pll_macro_entry { mclk: 24576000, pll_ch_num: 0 },
    pll_macro_entry { mclk: 49152000, pll_ch_num: 1 },
    pll_macro_entry { mclk: 98304000, pll_ch_num: 2 },
];

/* List of valid frame sizes for tdm mode */
static ssp_valid_tdm_framesize: [c_int; 5] = [32, 64, 128, 256, 512];

static cygnus_rates: [c_uint; 13] = [
    8000, 11025, 16000, 22050, 32000, 44100, 48000,
    88200, 96000, 176400, 192000, 352800, 384000,
];

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
}

static cygnus_rate_constraint: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: cygnus_rates.len() as c_uint,
    list: cygnus_rates.as_ptr(),
};

#[repr(C)]
#[derive(Copy, Clone)]
struct cygnus_ssp_regs {
    i2s_stream_cfg: usize,
    i2s_cap_stream_cfg: usize,
    i2s_cfg: usize,
    i2s_cap_cfg: usize,
    i2s_mclk_cfg: usize,
    bf_destch_ctrl: usize,
    bf_destch_cfg: usize,
    bf_sourcech_ctrl: usize,
    bf_sourcech_cfg: usize,
    bf_sourcech_grp: usize,
}

const fn INIT_SSP_REGS(num: usize) -> cygnus_ssp_regs {
    match num {
        0 => cygnus_ssp_regs {
            i2s_stream_cfg: OUT_I2S_0_STREAM_CFG_OFFSET,
            i2s_cap_stream_cfg: IN_I2S_0_STREAM_CFG_OFFSET,
            i2s_cfg: OUT_I2S_0_CFG_OFFSET,
            i2s_cap_cfg: IN_I2S_0_CFG_OFFSET,
            i2s_mclk_cfg: OUT_I2S_0_MCLK_CFG_OFFSET,
            bf_destch_ctrl: BF_DST_CTRL0_OFFSET,
            bf_destch_cfg: BF_DST_CFG0_OFFSET,
            bf_sourcech_ctrl: BF_SRC_CTRL0_OFFSET,
            bf_sourcech_cfg: BF_SRC_CFG0_OFFSET,
            bf_sourcech_grp: BF_SRC_GRP0_OFFSET,
        },
        1 => cygnus_ssp_regs {
            i2s_stream_cfg: OUT_I2S_1_STREAM_CFG_OFFSET,
            i2s_cap_stream_cfg: IN_I2S_1_STREAM_CFG_OFFSET,
            i2s_cfg: OUT_I2S_1_CFG_OFFSET,
            i2s_cap_cfg: IN_I2S_1_CFG_OFFSET,
            i2s_mclk_cfg: OUT_I2S_1_MCLK_CFG_OFFSET,
            bf_destch_ctrl: BF_DST_CTRL1_OFFSET,
            bf_destch_cfg: BF_DST_CFG1_OFFSET,
            bf_sourcech_ctrl: BF_SRC_CTRL1_OFFSET,
            bf_sourcech_cfg: BF_SRC_CFG1_OFFSET,
            bf_sourcech_grp: BF_SRC_GRP1_OFFSET,
        },
        _ => cygnus_ssp_regs {
            i2s_stream_cfg: OUT_I2S_2_STREAM_CFG_OFFSET,
            i2s_cap_stream_cfg: IN_I2S_2_STREAM_CFG_OFFSET,
            i2s_cfg: OUT_I2S_2_CFG_OFFSET,
            i2s_cap_cfg: IN_I2S_2_CFG_OFFSET,
            i2s_mclk_cfg: OUT_I2S_2_MCLK_CFG_OFFSET,
            bf_destch_ctrl: BF_DST_CTRL2_OFFSET,
            bf_destch_cfg: BF_DST_CFG2_OFFSET,
            bf_sourcech_ctrl: BF_SRC_CTRL2_OFFSET,
            bf_sourcech_cfg: BF_SRC_CFG2_OFFSET,
            bf_sourcech_grp: BF_SRC_GRP2_OFFSET,
        },
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cygnus_audio_port_type {
    PORT_TDM = 0,
    PORT_SPDIF = 1,
}
use cygnus_audio_port_type::*;

const CYGNUS_SSPMODE_UNKNOWN: c_int = 0;
const CYGNUS_SSPMODE_TDM: c_int = 1;
const CYGNUS_SSPMODE_I2S: c_int = 2;

#[repr(C)]
struct cygnus_clk_trace {
    play_en: bool_t,
    cap_en: bool_t,
    play_clk_en: bool_t,
    cap_clk_en: bool_t,
}

#[repr(C)]
struct cygnus_aio_port {
    cygaud: *mut cygnus_audio,
    regs: cygnus_ssp_regs,
    portnum: u32,
    port_type: cygnus_audio_port_type,
    fsync_width: c_int,
    mode: c_int,
    streams_on: u32,
    is_slave: c_int,
    bit_per_frame: c_uint,
    lrclk: c_uint,
    mclk: c_uint,
    clk_trace: cygnus_clk_trace,
    pll_clk_num: u32,
}

#[repr(C)]
struct cygnus_audio {
    audio: *mut u8,
    i2s_in: *mut u8,
    audio_clk: [*mut clk; 3],
    portinfo: [cygnus_aio_port; CYGNUS_MAX_PORTS],
    dev: *mut device,
    active_ports: c_int,
    irq_num: c_int,
}

#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, remove: Option<unsafe extern "C" fn(*mut platform_device)>, driver: device_driver }
#[repr(C)] struct device_driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw { rate_min: u32, rate_max: u32 }
#[repr(C)] struct snd_pcm_runtime { hw: snd_pcm_hw }
#[repr(C)] struct snd_pcm_substream { stream: c_int, runtime: *mut snd_pcm_runtime }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dai { id: c_int }

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: u32,
    formats: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    legacy_dai_naming: c_int,
}

extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut cygnus_audio;
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn of_property_read_u32(dn: *mut device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn of_get_child_count(dn: *mut device_node) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_platform_ioremap_resource_byname(pdev: *mut platform_device, name: *const c_char) -> *mut u8;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn cygnus_soc_platform_register(dev: *mut device, cygaud: *mut cygnus_audio) -> c_int;
    fn cygnus_soc_platform_unregister(dev: *mut device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn hweight32(w: u32) -> c_uint;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn cygnus_dai_get_portinfo(dai: *mut snd_soc_dai) -> *mut cygnus_aio_port {
    let cygaud = snd_soc_dai_get_drvdata(dai);
    (*cygaud).portinfo.as_mut_ptr().add((*dai).id as usize)
}

unsafe fn audio_ssp_init_portregs(aio: *mut cygnus_aio_port) -> c_int {
    let mut value: u32;
    let fci_id: u32;
    let mut status: c_int = 0;

    match (*aio).port_type {
        PORT_TDM => {
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_stream_cfg));
            value &= !I2S_STREAM_CFG_MASK;
            writel((*aio).portnum, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_grp));
            value |= (*aio).portnum << I2S_OUT_STREAM_CFG_GROUP_ID;
            value |= (*aio).portnum; /* FCI ID is the port num */
            value |= CH_GRP_STEREO << I2S_OUT_STREAM_CFG_CHANNEL_GROUPING;
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_stream_cfg));

            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value &= !bit(BF_SRC_CFGX_NOT_PAUSE_WHEN_EMPTY);
            value |= bit(BF_SRC_CFGX_SFIFO_SZ_DOUBLE);
            value |= bit(BF_SRC_CFGX_PROCESS_SEQ_ID_VALID);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));

            value = readl((*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_stream_cfg));
            value &= !I2S_CAP_STREAM_CFG_MASK;
            value |= (*aio).portnum << I2S_IN_STREAM_CFG_0_GROUP_ID;
            writel(value, (*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_stream_cfg));

            fci_id = CAPTURE_FCI_ID_BASE + (*aio).portnum;
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
            value |= bit(BF_DST_CFGX_DFIFO_SZ_DOUBLE);
            value &= !bit(BF_DST_CFGX_NOT_PAUSE_WHEN_FULL);
            value |= fci_id << BF_DST_CFGX_FCI_ID;
            value |= bit(BF_DST_CFGX_PROC_SEQ_ID_VALID);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));

            value = readl((*(*aio).cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
            value &= !bit(((*aio).portnum * 4) + AUD_MISC_SEROUT_SDAT_OE);
            writel(value, (*(*aio).cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
        }
        PORT_SPDIF => {
            writel((*aio).portnum, (*(*aio).cygaud).audio.add(BF_SRC_GRP3_OFFSET));
            value = readl((*(*aio).cygaud).audio.add(SPDIF_CTRL_OFFSET));
            value |= bit(SPDIF_0_OUT_DITHER_ENA);
            writel(value, (*(*aio).cygaud).audio.add(SPDIF_CTRL_OFFSET));
            value = readl((*(*aio).cygaud).audio.add(SPDIF_STREAM_CFG_OFFSET));
            value &= !SPDIF_STREAM_CFG_MASK;
            value |= (*aio).portnum; /* FCI ID is the port num */
            value |= bit(SPDIF_0_OUT_STREAM_ENA);
            writel(value, (*(*aio).cygaud).audio.add(SPDIF_STREAM_CFG_OFFSET));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value &= !bit(BF_SRC_CFGX_NOT_PAUSE_WHEN_EMPTY);
            value |= bit(BF_SRC_CFGX_SFIFO_SZ_DOUBLE);
            value |= bit(BF_SRC_CFGX_PROCESS_SEQ_ID_VALID);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value = readl((*(*aio).cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
            value &= !bit(AUD_MISC_SEROUT_SPDIF_OE);
            writel(value, (*(*aio).cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
        }
        _ => {
            dev_err((*(*aio).cygaud).dev, b"Port not supported\n\0".as_ptr() as *const c_char);
            status = -EINVAL;
        }
    }

    status
}

unsafe fn audio_ssp_in_enable(aio: *mut cygnus_aio_port) {
    let mut value: u32;
    value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
    value |= bit(BF_DST_CFGX_CAP_ENA);
    writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
    writel(0x1, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_ctrl));
    value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
    value |= bit(I2S_OUT_CFGX_CLK_ENA);
    value |= bit(I2S_OUT_CFGX_DATA_ENABLE);
    writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
    value = readl((*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_stream_cfg));
    value |= bit(I2S_IN_STREAM_CFG_CAP_ENA);
    writel(value, (*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_stream_cfg));
    (*aio).streams_on |= CAPTURE_STREAM_MASK;
}

unsafe fn audio_ssp_in_disable(aio: *mut cygnus_aio_port) {
    let mut value: u32;
    value = readl((*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_stream_cfg));
    value &= !bit(I2S_IN_STREAM_CFG_CAP_ENA);
    writel(value, (*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_stream_cfg));
    (*aio).streams_on &= !CAPTURE_STREAM_MASK;
    /* If both playback and capture are off */
    if (*aio).streams_on == 0 {
        value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
        value &= !bit(I2S_OUT_CFGX_CLK_ENA);
        value &= !bit(I2S_OUT_CFGX_DATA_ENABLE);
        writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
    }
    writel(0x0, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_ctrl));
    value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
    value &= !bit(BF_DST_CFGX_CAP_ENA);
    writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
}

unsafe fn audio_ssp_out_enable(aio: *mut cygnus_aio_port) -> c_int {
    let mut value: u32;
    let mut status: c_int = 0;
    match (*aio).port_type {
        PORT_TDM => {
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_stream_cfg));
            value |= bit(I2S_OUT_STREAM_ENA);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_stream_cfg));
            writel(1, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_ctrl));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
            value |= bit(I2S_OUT_CFGX_CLK_ENA);
            value |= bit(I2S_OUT_CFGX_DATA_ENABLE);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value |= bit(BF_SRC_CFGX_SFIFO_ENA);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            (*aio).streams_on |= PLAYBACK_STREAM_MASK;
        }
        PORT_SPDIF => {
            value = readl((*(*aio).cygaud).audio.add(SPDIF_FORMAT_CFG_OFFSET));
            value |= 0x3;
            writel(value, (*(*aio).cygaud).audio.add(SPDIF_FORMAT_CFG_OFFSET));
            writel(1, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_ctrl));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value |= bit(BF_SRC_CFGX_SFIFO_ENA);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
        }
        _ => {
            dev_err((*(*aio).cygaud).dev, b"Port not supported %d\n\0".as_ptr() as *const c_char, (*aio).portnum);
            status = -EINVAL;
        }
    }
    status
}

unsafe fn audio_ssp_out_disable(aio: *mut cygnus_aio_port) -> c_int {
    let mut value: u32;
    let mut status: c_int = 0;
    match (*aio).port_type {
        PORT_TDM => {
            (*aio).streams_on &= !PLAYBACK_STREAM_MASK;
            /* If both playback and capture are off */
            if (*aio).streams_on == 0 {
                value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
                value &= !bit(I2S_OUT_CFGX_CLK_ENA);
                value &= !bit(I2S_OUT_CFGX_DATA_ENABLE);
                writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
            }
            /* set group_sync_dis = 1 */
            value = readl((*(*aio).cygaud).audio.add(BF_SRC_GRP_SYNC_DIS_OFFSET));
            value |= bit((*aio).portnum);
            writel(value, (*(*aio).cygaud).audio.add(BF_SRC_GRP_SYNC_DIS_OFFSET));
            writel(0, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_ctrl));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value &= !bit(BF_SRC_CFGX_SFIFO_ENA);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            /* set group_sync_dis = 0 */
            value = readl((*(*aio).cygaud).audio.add(BF_SRC_GRP_SYNC_DIS_OFFSET));
            value &= !bit((*aio).portnum);
            writel(value, (*(*aio).cygaud).audio.add(BF_SRC_GRP_SYNC_DIS_OFFSET));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_stream_cfg));
            value &= !bit(I2S_OUT_STREAM_ENA);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_stream_cfg));
            /* IOP SW INIT on OUT_I2S_x */
            value = readl((*(*aio).cygaud).i2s_in.add(IOP_SW_INIT_LOGIC));
            value |= bit((*aio).portnum);
            writel(value, (*(*aio).cygaud).i2s_in.add(IOP_SW_INIT_LOGIC));
            value &= !bit((*aio).portnum);
            writel(value, (*(*aio).cygaud).i2s_in.add(IOP_SW_INIT_LOGIC));
        }
        PORT_SPDIF => {
            value = readl((*(*aio).cygaud).audio.add(SPDIF_FORMAT_CFG_OFFSET));
            value &= !0x3;
            writel(value, (*(*aio).cygaud).audio.add(SPDIF_FORMAT_CFG_OFFSET));
            writel(0, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_ctrl));
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
            value &= !bit(BF_SRC_CFGX_SFIFO_ENA);
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
        }
        _ => {
            dev_err((*(*aio).cygaud).dev, b"Port not supported %d\n\0".as_ptr() as *const c_char, (*aio).portnum);
            status = -EINVAL;
        }
    }
    status
}

unsafe fn pll_configure_mclk(cygaud: *mut cygnus_audio, mclk: u32, aio: *mut cygnus_aio_port) -> c_int {
    let mut found = false;
    let mut p_entry: *const pll_macro_entry = ptr::null();
    let ch_clk: *mut clk;
    let mut error: c_int;

    for i in 0..pll_predef_mclk.len() {
        p_entry = &pll_predef_mclk[i];
        if (*p_entry).mclk == mclk {
            found = true;
            break;
        }
    }
    if !found {
        dev_err((*cygaud).dev, b"%s No valid mclk freq (%u) found!\n\0".as_ptr() as *const c_char, b"pll_configure_mclk\0".as_ptr() as *const c_char, mclk);
        return -EINVAL;
    }

    ch_clk = (*cygaud).audio_clk[(*p_entry).pll_ch_num as usize];

    if (*aio).clk_trace.cap_en && !(*aio).clk_trace.cap_clk_en {
        error = clk_prepare_enable(ch_clk);
        if error != 0 {
            dev_err((*cygaud).dev, b"%s clk_prepare_enable failed %d\n\0".as_ptr() as *const c_char, b"pll_configure_mclk\0".as_ptr() as *const c_char, error);
            return error;
        }
        (*aio).clk_trace.cap_clk_en = true;
    }

    if (*aio).clk_trace.play_en && !(*aio).clk_trace.play_clk_en {
        error = clk_prepare_enable(ch_clk);
        if error != 0 {
            dev_err((*cygaud).dev, b"%s clk_prepare_enable failed %d\n\0".as_ptr() as *const c_char, b"pll_configure_mclk\0".as_ptr() as *const c_char, error);
            return error;
        }
        (*aio).clk_trace.play_clk_en = true;
    }

    error = clk_set_rate(ch_clk, mclk);
    if error != 0 {
        dev_err((*cygaud).dev, b"%s Set MCLK rate failed: %d\n\0".as_ptr() as *const c_char, b"pll_configure_mclk\0".as_ptr() as *const c_char, error);
        return error;
    }

    (*p_entry).pll_ch_num as c_int
}

unsafe fn cygnus_ssp_set_clocks(aio: *mut cygnus_aio_port) -> c_int {
    let mut value: u32;
    let mask: u32 = 0xf;
    let mut sclk: u32;
    let mclk_rate: u32;
    let bit_rate: c_uint;
    let ratio: c_uint;

    bit_rate = (*aio).bit_per_frame.wrapping_mul((*aio).lrclk);
    if ((*aio).mclk % bit_rate) != 0 {
        return -EINVAL;
    }
    ratio = (*aio).mclk / bit_rate;
    match ratio {
        2 | 4 | 6 | 8 | 10 | 12 | 14 => mclk_rate = ratio / 2,
        _ => {
            dev_err((*(*aio).cygaud).dev, b"Invalid combination of MCLK and BCLK\n\0".as_ptr() as *const c_char);
            dev_err((*(*aio).cygaud).dev, b"lrclk = %u, bits/frame = %u, mclk = %u\n\0".as_ptr() as *const c_char, (*aio).lrclk, (*aio).bit_per_frame, (*aio).mclk);
            return -EINVAL;
        }
    }

    match (*aio).port_type {
        PORT_TDM => {
            sclk = (*aio).bit_per_frame;
            if sclk == 512 {
                sclk = 0;
            }
            /* sclks_per_1fs_div = sclk cycles/32 */
            sclk /= 32;
            value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
            value &= !(mask << I2S_OUT_CFGX_SCLKS_PER_1FS_DIV32);
            value |= sclk << I2S_OUT_CFGX_SCLKS_PER_1FS_DIV32;
            writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
            dev_dbg((*(*aio).cygaud).dev, b"SCLKS_PER_1FS_DIV32 = 0x%x\n\0".as_ptr() as *const c_char, value);
        }
        PORT_SPDIF => {}
        _ => {
            dev_err((*(*aio).cygaud).dev, b"Unknown port type\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_mclk_cfg));
    value &= !(0xf << I2S_OUT_MCLKRATE_SHIFT);
    value |= mclk_rate << I2S_OUT_MCLKRATE_SHIFT;
    writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_mclk_cfg));
    dev_dbg((*(*aio).cygaud).dev, b"mclk cfg reg = 0x%x\n\0".as_ptr() as *const c_char, value);
    dev_dbg((*(*aio).cygaud).dev, b"bits per frame = %u, mclk = %u Hz, lrclk = %u Hz\n\0".as_ptr() as *const c_char, (*aio).bit_per_frame, (*aio).mclk, (*aio).lrclk);
    0
}

unsafe extern "C" fn cygnus_ssp_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let aio = cygnus_dai_get_portinfo(dai);
    let rate: c_int;
    let bitres: c_int;
    let mut value: u32;
    let mask: u32 = 0x1f;
    let mut ret: c_int = 0;

    dev_dbg((*(*aio).cygaud).dev, b"%s port = %d\n\0".as_ptr() as *const c_char, b"cygnus_ssp_hw_params\0".as_ptr() as *const c_char, (*aio).portnum);
    dev_dbg((*(*aio).cygaud).dev, b"params_channels %d\n\0".as_ptr() as *const c_char, params_channels(params));
    dev_dbg((*(*aio).cygaud).dev, b"rate %d\n\0".as_ptr() as *const c_char, params_rate(params));
    dev_dbg((*(*aio).cygaud).dev, b"format %d\n\0".as_ptr() as *const c_char, params_format(params));

    rate = params_rate(params);
    match (*aio).mode {
        CYGNUS_SSPMODE_TDM => {
            if rate == 192000 && params_channels(params) > 4 {
                dev_err((*(*aio).cygaud).dev, b"Cannot run %d channels at %dHz\n\0".as_ptr() as *const c_char, params_channels(params), rate);
                return -EINVAL;
            }
        }
        CYGNUS_SSPMODE_I2S => (*aio).bit_per_frame = 64,
        _ => {
            dev_err((*(*aio).cygaud).dev, b"%s port running in unknown mode\n\0".as_ptr() as *const c_char, b"cygnus_ssp_hw_params\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
        value &= !bit(BF_SRC_CFGX_BUFFER_PAIR_ENABLE);
        value &= !bit(BF_SRC_CFGX_SAMPLE_CH_MODE);
        writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
        bitres = match params_format(params) {
            SNDRV_PCM_FORMAT_S16_LE => 16,
            SNDRV_PCM_FORMAT_S32_LE => 0, /* 32 bit mode is coded as 0 */
            _ => return -EINVAL,
        };
        value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
        value &= !(mask << BF_SRC_CFGX_BIT_RES);
        value |= (bitres as u32) << BF_SRC_CFGX_BIT_RES;
        writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_sourcech_cfg));
    } else {
        match params_format(params) {
            SNDRV_PCM_FORMAT_S16_LE => {
                value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
                value |= bit(BF_DST_CFGX_CAP_MODE);
                writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
            }
            SNDRV_PCM_FORMAT_S32_LE => {
                value = readl((*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
                value &= !bit(BF_DST_CFGX_CAP_MODE);
                writel(value, (*(*aio).cygaud).audio.add((*aio).regs.bf_destch_cfg));
            }
            _ => return -EINVAL,
        }
    }

    (*aio).lrclk = rate as c_uint;
    if (*aio).is_slave == 0 {
        ret = cygnus_ssp_set_clocks(aio);
    }
    ret
}

/*
 * This function sets the mclk frequency for pll clock
 */
unsafe extern "C" fn cygnus_ssp_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let sel: c_int;
    let mut value: u32;
    let aio = cygnus_dai_get_portinfo(dai);
    let cygaud = snd_soc_dai_get_drvdata(dai);

    dev_dbg((*aio).cygaud as *mut device, b"%s Enter port = %d\n\0".as_ptr() as *const c_char, b"cygnus_ssp_set_sysclk\0".as_ptr() as *const c_char, (*aio).portnum);
    sel = pll_configure_mclk(cygaud, freq, aio);
    if sel < 0 {
        dev_err((*(*aio).cygaud).dev, b"%s Setting mclk failed.\n\0".as_ptr() as *const c_char, b"cygnus_ssp_set_sysclk\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    (*aio).mclk = freq;
    dev_dbg((*(*aio).cygaud).dev, b"%s Setting MCLKSEL to %d\n\0".as_ptr() as *const c_char, b"cygnus_ssp_set_sysclk\0".as_ptr() as *const c_char, sel);
    value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_mclk_cfg));
    value &= !(0xf << I2S_OUT_PLLCLKSEL_SHIFT);
    value |= (sel as u32) << I2S_OUT_PLLCLKSEL_SHIFT;
    writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_mclk_cfg));
    0
}

unsafe extern "C" fn cygnus_ssp_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let aio = cygnus_dai_get_portinfo(dai);
    snd_soc_dai_set_dma_data(dai, substream, aio as *mut c_void);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*aio).clk_trace.play_en = true;
    } else {
        (*aio).clk_trace.cap_en = true;
    }
    (*(*substream).runtime).hw.rate_min = CYGNUS_RATE_MIN;
    (*(*substream).runtime).hw.rate_max = CYGNUS_RATE_MAX;
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &cygnus_rate_constraint);
    0
}

unsafe extern "C" fn cygnus_ssp_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let aio = cygnus_dai_get_portinfo(dai);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*aio).clk_trace.play_en = false;
    } else {
        (*aio).clk_trace.cap_en = false;
    }

    if (*aio).is_slave == 0 {
        let mut val: u32;
        val = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_mclk_cfg));
        val &= CYGNUS_PLLCLKSEL_MASK;
        if val as usize >= (*(*aio).cygaud).audio_clk.len() {
            dev_err((*(*aio).cygaud).dev, b"Clk index %u is out of bounds\n\0".as_ptr() as *const c_char, val);
            return;
        }
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*aio).clk_trace.play_clk_en {
                clk_disable_unprepare((*(*aio).cygaud).audio_clk[val as usize]);
                (*aio).clk_trace.play_clk_en = false;
            }
        } else if (*aio).clk_trace.cap_clk_en {
            clk_disable_unprepare((*(*aio).cygaud).audio_clk[val as usize]);
            (*aio).clk_trace.cap_clk_en = false;
        }
    }
}

/*
 * Bit    Update  Notes
 * 31     Yes     TDM Mode        (1 = TDM, 0 = i2s)
 * 30     Yes     Slave Mode	  (1 = Slave, 0 = Master)
 * 29:26  No      Sclks per frame
 * 25:18  Yes     FS Width
 * 17:14  No      Valid Slots
 * 13     No      Bits		  (1 = 16 bits, 0 = 32 bits)
 * 12:08  No     Bits per samp
 * 07     Yes     Justifcation    (1 = LSB, 0 = MSB)
 * 06     Yes     Alignment       (1 = Delay 1 clk, 0 = no delay
 * 05     Yes     SCLK polarity   (1 = Rising, 0 = Falling)
 * 04     Yes     LRCLK Polarity  (1 = High for left, 0 = Low for left)
 * 03:02  Yes     Reserved - write as zero
 * 01     No      Data Enable
 * 00     No      CLK Enable
 */
const I2S_OUT_CFG_REG_UPDATE_MASK: u32 = 0x3C03FF03;

/* Input cfg is same as output, but the FS width is not a valid field */
const I2S_IN_CFG_REG_UPDATE_MASK: u32 = I2S_OUT_CFG_REG_UPDATE_MASK | 0x03FC0000;

#[no_mangle]
pub unsafe extern "C" fn cygnus_ssp_set_custom_fsync_width(cpu_dai: *mut snd_soc_dai, len: c_int) -> c_int {
    let aio = cygnus_dai_get_portinfo(cpu_dai);
    if len > 0 && len < 256 {
        (*aio).fsync_width = len;
        0
    } else {
        -EINVAL
    }
}
/* EXPORT_SYMBOL_GPL(cygnus_ssp_set_custom_fsync_width); */

unsafe extern "C" fn cygnus_ssp_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let aio = cygnus_dai_get_portinfo(cpu_dai);
    let ssp_curcfg: u32;
    let mut ssp_newcfg: u32;
    let ssp_outcfg: u32;
    let ssp_incfg: u32;
    let mut val: u32;
    let mut mask: u32;

    dev_dbg((*(*aio).cygaud).dev, b"%s Enter  fmt: %x\n\0".as_ptr() as *const c_char, b"cygnus_ssp_set_fmt\0".as_ptr() as *const c_char, fmt);
    if (*aio).port_type == PORT_SPDIF {
        return -EINVAL;
    }
    ssp_newcfg = 0;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {
            ssp_newcfg |= bit(I2S_OUT_CFGX_SLAVE_MODE);
            (*aio).is_slave = 1;
        }
        SND_SOC_DAIFMT_BP_FP => {
            ssp_newcfg &= !bit(I2S_OUT_CFGX_SLAVE_MODE);
            (*aio).is_slave = 0;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            ssp_newcfg |= bit(I2S_OUT_CFGX_DATA_ALIGNMENT);
            ssp_newcfg |= bit(I2S_OUT_CFGX_FSYNC_WIDTH);
            (*aio).mode = CYGNUS_SSPMODE_I2S;
        }
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            ssp_newcfg |= bit(I2S_OUT_CFGX_TDM_MODE);
            /* DSP_A = data after FS, DSP_B = data during FS */
            if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
                ssp_newcfg |= bit(I2S_OUT_CFGX_DATA_ALIGNMENT);
            }
            if (*aio).fsync_width > 0 && (*aio).fsync_width < 256 {
                ssp_newcfg |= ((*aio).fsync_width as u32) << I2S_OUT_CFGX_FSYNC_WIDTH;
            } else {
                ssp_newcfg |= bit(I2S_OUT_CFGX_FSYNC_WIDTH);
            }
            (*aio).mode = CYGNUS_SSPMODE_TDM;
        }
        _ => return -EINVAL,
    }

    /*
     * SSP out cfg.
     * Retain bits we do not want to update, then OR in new bits
     */
    let ssp_curcfg_out = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
    ssp_outcfg = (ssp_curcfg_out & I2S_OUT_CFG_REG_UPDATE_MASK) | ssp_newcfg;
    writel(ssp_outcfg, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));

    /*
     * SSP in cfg.
     * Retain bits we do not want to update, then OR in new bits
     */
    ssp_curcfg = readl((*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_cfg));
    ssp_incfg = (ssp_curcfg & I2S_IN_CFG_REG_UPDATE_MASK) | ssp_newcfg;
    writel(ssp_incfg, (*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_cfg));

    val = readl((*(*aio).cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
    /*
     * Configure the word clk and bit clk as output or tristate
     * Each port has 4 bits for controlling its pins.
     * Shift the mask based upon port number.
     */
    mask = bit(AUD_MISC_SEROUT_LRCK_OE) | bit(AUD_MISC_SEROUT_SCLK_OE) | bit(AUD_MISC_SEROUT_MCLK_OE);
    mask <<= (*aio).portnum * 4;
    if (*aio).is_slave != 0 {
        /* Set bit for tri-state */
        val |= mask;
    } else {
        /* Clear bit for drive */
        val &= !mask;
    }
    dev_dbg((*(*aio).cygaud).dev, b"%s  Set OE bits 0x%x\n\0".as_ptr() as *const c_char, b"cygnus_ssp_set_fmt\0".as_ptr() as *const c_char, val);
    writel(val, (*(*aio).cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
    0
}

unsafe extern "C" fn cygnus_ssp_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let aio = cygnus_dai_get_portinfo(dai);
    let cygaud = snd_soc_dai_get_drvdata(dai);
    dev_dbg((*(*aio).cygaud).dev, b"%s cmd %d at port = %d\n\0".as_ptr() as *const c_char, b"cygnus_ssp_trigger\0".as_ptr() as *const c_char, cmd, (*aio).portnum);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                audio_ssp_out_enable(aio);
            } else {
                audio_ssp_in_enable(aio);
            }
            (*cygaud).active_ports += 1;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                audio_ssp_out_disable(aio);
            } else {
                audio_ssp_in_disable(aio);
            }
            (*cygaud).active_ports -= 1;
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn cygnus_set_dai_tdm_slot(cpu_dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let aio = cygnus_dai_get_portinfo(cpu_dai);
    let mut value: u32;
    let bits_per_slot: c_int;     /* default to 32-bits per slot */
    let frame_bits: c_int;
    let mut active_slots: c_uint;
    let mut found = false;

    if tx_mask != rx_mask {
        dev_err((*(*aio).cygaud).dev, b"%s tx_mask must equal rx_mask\n\0".as_ptr() as *const c_char, b"cygnus_set_dai_tdm_slot\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    active_slots = hweight32(tx_mask);
    if active_slots > 16 {
        return -EINVAL;
    }
    /* Slot value must be even */
    if active_slots % 2 != 0 {
        return -EINVAL;
    }
    /* We encode 16 slots as 0 in the reg */
    if active_slots == 16 {
        active_slots = 0;
    }
    /* Slot Width is either 16 or 32 */
    bits_per_slot = match slot_width {
        16 => 1,
        32 => 0,
        _ => {
            dev_warn((*(*aio).cygaud).dev, b"%s Defaulting Slot Width to 32\n\0".as_ptr() as *const c_char, b"cygnus_set_dai_tdm_slot\0".as_ptr() as *const c_char);
            0
        }
    };
    frame_bits = slots * slot_width;
    for i in 0..ssp_valid_tdm_framesize.len() {
        if ssp_valid_tdm_framesize[i] == frame_bits {
            found = true;
            break;
        }
    }
    if !found {
        dev_err((*(*aio).cygaud).dev, b"%s In TDM mode, frame bits INVALID (%d)\n\0".as_ptr() as *const c_char, b"cygnus_set_dai_tdm_slot\0".as_ptr() as *const c_char, frame_bits);
        return -EINVAL;
    }
    (*aio).bit_per_frame = frame_bits as c_uint;
    dev_dbg((*(*aio).cygaud).dev, b"%s active_slots %u, bits per frame %d\n\0".as_ptr() as *const c_char, b"cygnus_set_dai_tdm_slot\0".as_ptr() as *const c_char, active_slots, frame_bits);

    /* Set capture side of ssp port */
    value = readl((*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_cfg));
    value &= !(0xf << I2S_OUT_CFGX_VALID_SLOT);
    value |= active_slots << I2S_OUT_CFGX_VALID_SLOT;
    value &= !bit(I2S_OUT_CFGX_BITS_PER_SLOT);
    value |= (bits_per_slot as u32) << I2S_OUT_CFGX_BITS_PER_SLOT;
    writel(value, (*(*aio).cygaud).i2s_in.add((*aio).regs.i2s_cap_cfg));

    /* Set playback side of ssp port */
    value = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
    value &= !(0xf << I2S_OUT_CFGX_VALID_SLOT);
    value |= active_slots << I2S_OUT_CFGX_VALID_SLOT;
    value &= !bit(I2S_OUT_CFGX_BITS_PER_SLOT);
    value |= (bits_per_slot as u32) << I2S_OUT_CFGX_BITS_PER_SLOT;
    writel(value, (*(*aio).cygaud).audio.add((*aio).regs.i2s_cfg));
    0
}

/* Original C condition: #ifdef CONFIG_PM_SLEEP */
unsafe fn __cygnus_ssp_suspend(cpu_dai: *mut snd_soc_dai) -> c_int {
    let aio = cygnus_dai_get_portinfo(cpu_dai);
    if snd_soc_dai_active(cpu_dai) == 0 {
        return 0;
    }
    if (*aio).is_slave == 0 {
        let mut val: u32;
        val = readl((*(*aio).cygaud).audio.add((*aio).regs.i2s_mclk_cfg));
        val &= CYGNUS_PLLCLKSEL_MASK;
        if val as usize >= (*(*aio).cygaud).audio_clk.len() {
            dev_err((*(*aio).cygaud).dev, b"Clk index %u is out of bounds\n\0".as_ptr() as *const c_char, val);
            return -EINVAL;
        }
        if (*aio).clk_trace.cap_clk_en {
            clk_disable_unprepare((*(*aio).cygaud).audio_clk[val as usize]);
        }
        if (*aio).clk_trace.play_clk_en {
            clk_disable_unprepare((*(*aio).cygaud).audio_clk[val as usize]);
        }
        (*aio).pll_clk_num = val;
    }
    0
}

unsafe extern "C" fn cygnus_ssp_suspend(component: *mut snd_soc_component) -> c_int {
    let _component = component;
    let mut ret: c_int = 0;
    /* C used for_each_component_dais(component, dai) ret |= __cygnus_ssp_suspend(dai); */
    ret
}

unsafe fn __cygnus_ssp_resume(cpu_dai: *mut snd_soc_dai) -> c_int {
    let aio = cygnus_dai_get_portinfo(cpu_dai);
    let mut error: c_int;
    if snd_soc_dai_active(cpu_dai) == 0 {
        return 0;
    }
    if (*aio).is_slave == 0 {
        if (*aio).clk_trace.cap_clk_en {
            error = clk_prepare_enable((*(*aio).cygaud).audio_clk[(*aio).pll_clk_num as usize]);
            if error != 0 {
                dev_err((*(*aio).cygaud).dev, b"%s clk_prepare_enable failed\n\0".as_ptr() as *const c_char, b"__cygnus_ssp_resume\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
        }
        if (*aio).clk_trace.play_clk_en {
            error = clk_prepare_enable((*(*aio).cygaud).audio_clk[(*aio).pll_clk_num as usize]);
            if error != 0 {
                if (*aio).clk_trace.cap_clk_en {
                    clk_disable_unprepare((*(*aio).cygaud).audio_clk[(*aio).pll_clk_num as usize]);
                }
                dev_err((*(*aio).cygaud).dev, b"%s clk_prepare_enable failed\n\0".as_ptr() as *const c_char, b"__cygnus_ssp_resume\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
        }
    }
    0
}

unsafe extern "C" fn cygnus_ssp_resume(component: *mut snd_soc_component) -> c_int {
    let _component = component;
    let mut ret: c_int = 0;
    /* C used for_each_component_dais(component, dai) ret |= __cygnus_ssp_resume(dai); */
    ret
}
/* Original C #else mapped cygnus_ssp_suspend and cygnus_ssp_resume to NULL when CONFIG_PM_SLEEP is unset. */

static cygnus_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S |
    SND_SOC_POSSIBLE_DAIFMT_DSP_A |
    SND_SOC_POSSIBLE_DAIFMT_DSP_B;

static cygnus_ssp_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cygnus_ssp_startup),
    shutdown: Some(cygnus_ssp_shutdown),
    trigger: Some(cygnus_ssp_trigger),
    hw_params: Some(cygnus_ssp_hw_params),
    set_fmt: Some(cygnus_ssp_set_fmt),
    set_sysclk: Some(cygnus_ssp_set_sysclk),
    set_tdm_slot: Some(cygnus_set_dai_tdm_slot),
    auto_selectable_formats: &cygnus_selectable_formats,
    num_auto_selectable_formats: 1,
};

static cygnus_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cygnus_ssp_startup),
    shutdown: Some(cygnus_ssp_shutdown),
    trigger: Some(cygnus_ssp_trigger),
    hw_params: Some(cygnus_ssp_hw_params),
    set_fmt: None,
    set_sysclk: Some(cygnus_ssp_set_sysclk),
    set_tdm_slot: None,
    auto_selectable_formats: ptr::null(),
    num_auto_selectable_formats: 0,
};

const fn INIT_CPU_DAI(name: *const c_char) -> snd_soc_dai_driver {
    snd_soc_dai_driver {
        name,
        playback: snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &cygnus_ssp_dai_ops,
    }
}

static cygnus_ssp_dai_info: [snd_soc_dai_driver; 3] = [
    INIT_CPU_DAI(b"cygnus-ssp0\0".as_ptr() as *const c_char),
    INIT_CPU_DAI(b"cygnus-ssp1\0".as_ptr() as *const c_char),
    INIT_CPU_DAI(b"cygnus-ssp2\0".as_ptr() as *const c_char),
];

static cygnus_spdif_dai_info: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"cygnus-spdif\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
    },
    ops: &cygnus_spdif_dai_ops,
};

static mut cygnus_ssp_dai: [snd_soc_dai_driver; CYGNUS_MAX_PORTS] = [
    INIT_CPU_DAI(ptr::null()),
    INIT_CPU_DAI(ptr::null()),
    INIT_CPU_DAI(ptr::null()),
    INIT_CPU_DAI(ptr::null()),
];

static cygnus_ssp_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"cygnus-audio\0".as_ptr() as *const c_char,
    suspend: Some(cygnus_ssp_suspend),
    resume: Some(cygnus_ssp_resume),
    legacy_dai_naming: 1,
};

/*
 * Return < 0 if error
 * Return 0 if disabled
 * Return 1 if enabled and node is parsed successfully
 */
unsafe fn parse_ssp_child_node(pdev: *mut platform_device, dn: *mut device_node, cygaud: *mut cygnus_audio, p_dai: *mut snd_soc_dai_driver) -> c_int {
    let aio: *mut cygnus_aio_port;
    let mut ssp_regs: [cygnus_ssp_regs; 3] = [INIT_SSP_REGS(0), INIT_SSP_REGS(1), INIT_SSP_REGS(2)];
    let mut rawval: u32 = 0;
    let mut portnum: c_int = -1;
    let port_type: cygnus_audio_port_type;

    if of_property_read_u32(dn, b"reg\0".as_ptr() as *const c_char, &mut rawval) != 0 {
        dev_err(&mut (*pdev).dev, b"Missing reg property\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    portnum = rawval as c_int;
    match rawval {
        0 => {
            ssp_regs[0] = INIT_SSP_REGS(0);
            port_type = PORT_TDM;
        }
        1 => {
            ssp_regs[1] = INIT_SSP_REGS(1);
            port_type = PORT_TDM;
        }
        2 => {
            ssp_regs[2] = INIT_SSP_REGS(2);
            port_type = PORT_TDM;
        }
        3 => {
            port_type = PORT_SPDIF;
        }
        _ => {
            dev_err(&mut (*pdev).dev, b"Bad value for reg %u\n\0".as_ptr() as *const c_char, rawval);
            return -EINVAL;
        }
    }

    aio = (*cygaud).portinfo.as_mut_ptr().add(portnum as usize);
    (*aio).cygaud = cygaud;
    (*aio).portnum = portnum as u32;
    (*aio).port_type = port_type;
    (*aio).fsync_width = -1;

    match port_type {
        PORT_TDM => {
            (*aio).regs = ssp_regs[portnum as usize];
            *p_dai = cygnus_ssp_dai_info[portnum as usize];
            (*aio).mode = CYGNUS_SSPMODE_UNKNOWN;
        }
        PORT_SPDIF => {
            (*aio).regs.bf_sourcech_cfg = BF_SRC_CFG3_OFFSET;
            (*aio).regs.bf_sourcech_ctrl = BF_SRC_CTRL3_OFFSET;
            (*aio).regs.i2s_mclk_cfg = SPDIF_MCLK_CFG_OFFSET;
            (*aio).regs.i2s_stream_cfg = SPDIF_STREAM_CFG_OFFSET;
            *p_dai = cygnus_spdif_dai_info;
            /* For the purposes of this code SPDIF can be I2S mode */
            (*aio).mode = CYGNUS_SSPMODE_I2S;
        }
        _ => {
            dev_err(&mut (*pdev).dev, b"Bad value for port_type %d\n\0".as_ptr() as *const c_char, port_type as c_int);
            return -EINVAL;
        }
    }

    dev_dbg(&mut (*pdev).dev, b"%s portnum = %d\n\0".as_ptr() as *const c_char, b"parse_ssp_child_node\0".as_ptr() as *const c_char, (*aio).portnum);
    (*aio).streams_on = 0;
    (*aio).cygaud.as_mut().unwrap().dev = &mut (*pdev).dev;
    (*aio).clk_trace.play_en = false;
    (*aio).clk_trace.cap_en = false;

    audio_ssp_init_portregs(aio);
    0
}

unsafe fn audio_clk_init(pdev: *mut platform_device, cygaud: *mut cygnus_audio) -> c_int {
    let mut clk_name: [c_char; PROP_LEN_MAX] = [0; PROP_LEN_MAX];
    for i in 0..(*cygaud).audio_clk.len() {
        snprintf(clk_name.as_mut_ptr(), PROP_LEN_MAX, b"ch%d_audio\0".as_ptr() as *const c_char, i as c_int);
        (*cygaud).audio_clk[i] = devm_clk_get(&mut (*pdev).dev, clk_name.as_ptr());
        if IS_ERR((*cygaud).audio_clk[i] as *const c_void) {
            return PTR_ERR((*cygaud).audio_clk[i] as *const c_void);
        }
    }
    0
}

unsafe extern "C" fn cygnus_ssp_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let cygaud: *mut cygnus_audio;
    let mut err: c_int;
    let node_count: c_int;
    let mut active_port_count: c_int;

    cygaud = devm_kzalloc(dev, core::mem::size_of::<cygnus_audio>(), GFP_KERNEL) as *mut cygnus_audio;
    if cygaud.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(dev, cygaud as *mut c_void);
    (*cygaud).audio = devm_platform_ioremap_resource_byname(pdev, b"aud\0".as_ptr() as *const c_char);
    if IS_ERR((*cygaud).audio as *const c_void) {
        return PTR_ERR((*cygaud).audio as *const c_void);
    }
    (*cygaud).i2s_in = devm_platform_ioremap_resource_byname(pdev, b"i2s_in\0".as_ptr() as *const c_char);
    if IS_ERR((*cygaud).i2s_in as *const c_void) {
        return PTR_ERR((*cygaud).i2s_in as *const c_void);
    }

    /* Tri-state all controlable pins until we know that we need them */
    writel(CYGNUS_SSP_TRISTATE_MASK, (*cygaud).audio.add(AUD_MISC_SEROUT_OE_REG_BASE));
    node_count = of_get_child_count((*dev).of_node);
    if node_count < 1 || node_count > CYGNUS_MAX_PORTS as c_int {
        dev_err(dev, b"child nodes is %d.  Must be between 1 and %d\n\0".as_ptr() as *const c_char, node_count, CYGNUS_MAX_PORTS as c_int);
        return -EINVAL;
    }
    active_port_count = 0;

    /* C used for_each_available_child_of_node_scoped(pdev->dev.of_node, child_node). */
    let child_node: *mut device_node = ptr::null_mut();
    while !child_node.is_null() {
        err = parse_ssp_child_node(pdev, child_node, cygaud, cygnus_ssp_dai.as_mut_ptr().add(active_port_count as usize));
        /* negative is err, 0 is active and good, 1 is disabled */
        if err < 0 {
            return err;
        }
        if err == 0 {
            dev_dbg(dev, b"Activating DAI: %s\n\0".as_ptr() as *const c_char, cygnus_ssp_dai[active_port_count as usize].name);
            active_port_count += 1;
        }
        break;
    }

    (*cygaud).dev = dev;
    (*cygaud).active_ports = 0;
    dev_dbg(dev, b"Registering %d DAIs\n\0".as_ptr() as *const c_char, active_port_count);
    err = devm_snd_soc_register_component(dev, &cygnus_ssp_component, cygnus_ssp_dai.as_mut_ptr(), active_port_count);
    if err != 0 {
        dev_err(dev, b"snd_soc_register_dai failed\n\0".as_ptr() as *const c_char);
        return err;
    }
    (*cygaud).irq_num = platform_get_irq(pdev, 0);
    if (*cygaud).irq_num <= 0 {
        return (*cygaud).irq_num;
    }
    err = audio_clk_init(pdev, cygaud);
    if err != 0 {
        dev_err(dev, b"audio clock initialization failed\n\0".as_ptr() as *const c_char);
        return err;
    }
    err = cygnus_soc_platform_register(dev, cygaud);
    if err != 0 {
        dev_err(dev, b"platform reg error %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }
    0
}

unsafe extern "C" fn cygnus_ssp_remove(pdev: *mut platform_device) {
    cygnus_soc_platform_unregister(&mut (*pdev).dev);
}

static cygnus_ssp_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"brcm,cygnus-audio\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, cygnus_ssp_of_match); */

static mut cygnus_ssp_driver: platform_driver = platform_driver {
    probe: Some(cygnus_ssp_probe),
    remove: Some(cygnus_ssp_remove),
    driver: device_driver {
        name: b"cygnus-ssp\0".as_ptr() as *const c_char,
        of_match_table: cygnus_ssp_of_match.as_ptr(),
    },
};

/* module_platform_driver(cygnus_ssp_driver); */

/* MODULE_ALIAS("platform:cygnus-ssp"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_AUTHOR("Broadcom"); */
/* MODULE_DESCRIPTION("Cygnus ASoC SSP Interface"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
