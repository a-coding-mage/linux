// SPDX-License-Identifier: GPL-2.0
//
// Freescale Generic ASoC Sound Card driver with ASRC
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <nicoleotsuka@gmail.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DRIVER_NAME: *const c_char = b"fsl-asoc-card\0".as_ptr() as *const c_char;

const CS427x_SYSCLK_MCLK: u32 = 0;

const RX: usize = 0;
const TX: usize = 1;

/* Default DAI format without Master and Slave flag */
const DAI_FMT_BASE: u32 = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF;

type u8_t = u8;
type u32_t = u32;
type u64_t = u64;
type bool_t = bool;
type snd_pcm_format_t = c_int;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub private_data: *mut c_void,
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub driver: *mut snd_soc_component_driver,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub ops: *const snd_soc_ops,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub dai_fmt: c_uint,
    pub dynamic: c_uint,
    pub no_pcm: c_uint,
    pub dpcm_merged_chan: c_uint,
    pub ignore_pmdown_time: c_uint,
    pub playback_only: bool_t,
    pub capture_only: bool_t,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
    pub list: list_head,
}
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub driver_name: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub rtd_list: list_head,
}
#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
}
type c_uchar = u8;
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct simple_util_jack_gpio {
    pub desc: *mut c_void,
}
#[repr(C)]
pub struct simple_util_jack {
    pub jack: snd_soc_jack,
    pub gpio: simple_util_jack_gpio,
}
#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_inner,
}
#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const u32,
    pub mask: u32,
}

unsafe extern "C" {
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: c_void;
    static THIS_MODULE: *mut c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_mask64(runtime: *mut snd_pcm_runtime, var: c_int, mask: u64) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: snd_pcm_format_t);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_u32_index(np: *mut device_node, propname: *const c_char, index: c_uint, out_value: *mut u32) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool_t;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool_t;
    fn of_node_name_eq(np: *mut device_node, name: *const c_char) -> bool_t;
    fn of_device_is_compatible(np: *mut device_node, compat: *const c_char) -> bool_t;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn of_node_put(np: *mut device_node);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn snd_soc_daifmt_parse_clock_provider_as_phandle(np: *mut device_node, prefix: *const c_char, bit: *mut *mut device_node, frame: *mut *mut device_node);
    fn snd_soc_daifmt_parse_format(np: *mut device_node, prefix: *const c_char) -> c_uint;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn imx_audmux_v2_configure_port(port: c_int, ptcr: c_uint, pdcr: c_uint) -> c_int;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_put(clk: *mut clk);
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_jack_notifier_register(jack: *mut snd_soc_jack, nb: *mut notifier_block);
    fn snd_soc_jack_notifier_unregister(jack: *mut snd_soc_jack, nb: *mut notifier_block);
    fn snd_soc_jack_free_gpios(jack: *mut snd_soc_jack, count: c_int, gpio: *mut simple_util_jack_gpio);
    fn simple_util_init_jack(card: *mut snd_soc_card, jack: *mut simple_util_jack, is_hp: c_int, prefix: *const c_char, pin: *const c_char) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_uint, mask: c_uint, value: c_uint);
    fn module_platform_driver(driver: *mut platform_driver);
}
type c_long = i64;

extern "Rust" {
    static SND_SOC_DAIFMT_I2S: u32;
    static SND_SOC_DAIFMT_NB_NF: u32;
    static SND_SOC_DAIFMT_AC97: u32;
    static SND_SOC_DAIFMT_LEFT_J: u32;
    static SND_SOC_DAIFMT_CBP_CFP: u32;
    static SND_SOC_DAIFMT_CBP_CFC: u32;
    static SND_SOC_DAIFMT_CBC_CFP: u32;
    static SND_SOC_DAIFMT_CBC_CFC: u32;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_CLOCK_OUT: u32;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SND_JACK_HEADPHONE: c_ulong;
    static SND_JACK_MICROPHONE: c_ulong;
    static ENOTSUPP: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static SGTL5000_SYSCLK: u32;
    static WM8962_SYSCLK_MCLK: u32;
    static WM8962_SYSCLK_FLL: c_int;
    static WM8962_FLL: c_int;
    static WM8960_SYSCLK_AUTO: c_int;
    static WM8994_FLL_SRC_MCLK1: u32;
    static WM8994_SYSCLK_FLL1: c_int;
    static WM8994_FLL1: c_int;
    static NAU8822_CLK_MCLK: u32;
    static NAU8822_CLK_PLL: c_int;
    static WM8904_FLL_MCLK: u32;
    static WM8904_CLK_FLL: c_int;
    static ESAI_HCKT_EXTAL: u32;
    static ESAI_HCKR_EXTAL: u32;
    static FSL_SAI_CLK_MAST1: u32;
    static AC97_EXTENDED_STATUS: c_uint;
    static AC97_EA_SPSA_SLOT_MASK: c_uint;
    static AC97_EA_SPSA_3_4: c_uint;
}

unsafe fn BIT(nr: c_int) -> u8 {
    (1u8).wrapping_shl(nr as u32)
}
unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int {
    N as c_int
}
unsafe fn IMX_AUDMUX_V2_PTCR_RFSEL(v: c_int) -> c_uint { v as c_uint }
unsafe fn IMX_AUDMUX_V2_PTCR_RCSEL(v: c_int) -> c_uint { v as c_uint }
unsafe fn IMX_AUDMUX_V2_PTCR_TFSEL(v: c_int) -> c_uint { v as c_uint }
unsafe fn IMX_AUDMUX_V2_PTCR_TCSEL(v: c_int) -> c_uint { v as c_uint }
extern "Rust" {
    static IMX_AUDMUX_V2_PTCR_RFSDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_RCLKDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_TFSDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_TCLKDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_SYN: c_uint;
}
unsafe fn IMX_AUDMUX_V2_PDCR_RXDSEL(v: c_int) -> c_uint { v as c_uint }

static cs42888_rates_48k: [u32; 3] = [48000, 96000, 192000];
static cs42888_rates_44k: [u32; 3] = [44100, 88200, 176400];
static cs42888_channels: [u32; 5] = [1, 2, 4, 6, 8];

static cs42888_rate_48k_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: cs42888_rates_48k.as_ptr(),
    count: 3,
    mask: 0,
};
static cs42888_rate_44k_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: cs42888_rates_44k.as_ptr(),
    count: 3,
    mask: 0,
};
static cs42888_channel_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: cs42888_channels.as_ptr(),
    count: 5,
    mask: 0,
};

/**
 * struct codec_priv - CODEC private data
 * @mclk_freq: Clock rate of MCLK
 * @free_freq: Clock rate of MCLK for hw_free()
 * @mclk_id: MCLK (or main clock) id for set_sysclk()
 * @fll_id: FLL (or secordary clock) id for set_sysclk()
 * @pll_id: PLL id for set_pll()
 * @pll_ratio_s24: PLL output ratio for S24_LE format (PLL_freq = sample_rate × ratio)
 *                 Default is 384, but some codecs (e.g., WM8904) require lower values
 *                 to stay within PLL frequency limits
 */
#[repr(C)]
struct codec_priv {
    mclk_freq: c_ulong,
    free_freq: c_ulong,
    mclk_id: u32,
    fll_id: c_int,
    pll_id: c_int,
    pll_ratio_s24: c_int,
}

/**
 * struct cpu_priv - CPU private data
 * @sysclk_freq: SYSCLK rates for set_sysclk()
 * @sysclk_dir: SYSCLK directions for set_sysclk()
 * @sysclk_id: SYSCLK ids for set_sysclk()
 * @sysclk_ratio: SYSCLK ratio on sample rate
 * @slot_width: Slot width of each frame
 * @slot_num: Number of slots of each frame
 *
 * Note: [1] for tx and [0] for rx
 */
#[repr(C)]
struct cpu_priv {
    sysclk_freq: [c_ulong; 2],
    sysclk_dir: [u32; 2],
    sysclk_id: [u32; 2],
    sysclk_ratio: [u32; 2],
    slot_width: u32,
    slot_num: u32,
}

#[repr(C)]
struct fsl_asoc_card_pdata {
    sysclk_dir: [u32; 2],
    sysclk_ratio: [u32; 2],
    slot_width: u32,
    codec_dai_name: *const c_char,
    codec_mclk_id: u32,
    codec_fll_id: c_int,
    codec_pll_id: c_int,
    codec_pll_ratio_s24: c_int,
    has_pll: bool,
    playback_only: bool,
    capture_only: bool,
    dai_fmt: u32,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    exclude_format: u64,
    codec_init: Option<unsafe extern "C" fn(*mut fsl_asoc_card_priv) -> c_int>,
    probe_init: Option<unsafe extern "C" fn(*mut *mut device_node, *mut device_node, *mut *const c_char, *mut fsl_asoc_card_priv) -> c_int>,
}

#[repr(C)]
struct fsl_asoc_card_priv {
    dai_link: [snd_soc_dai_link; 3],
    hp_jack: simple_util_jack,
    mic_jack: simple_util_jack,
    pdev: *mut platform_device,
    pdata: *const fsl_asoc_card_pdata,
    codec_priv: [codec_priv; 2],
    cpu_priv: cpu_priv,
    card: snd_soc_card,
    constraint_rates: *const snd_pcm_hw_constraint_list,
    constraint_channels: *const snd_pcm_hw_constraint_list,
    streams: u8,
    sample_rate: u32,
    sample_format: snd_pcm_format_t,
    asrc_rate: u32,
    asrc_format: snd_pcm_format_t,
    dai_fmt: u32,
    exclude_format: u64,
    name: [c_char; 32],
}

/*
 * This dapm route map exists for DPCM link only.
 * The other routes shall go through Device Tree.
 *
 * Note: keep all ASRC routes in the second half
 *       to drop them easily for non-ASRC cases.
 */
static audio_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CPU-Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CPU-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CPU-Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASRC-Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASRC-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CPU-Capture\0".as_ptr() as *const c_char },
];

static audio_map_ac97: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"AC97 Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CPU AC97 Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CPU AC97 Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AC97 Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CPU AC97 Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASRC-Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASRC-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CPU AC97 Capture\0".as_ptr() as *const c_char },
];

static audio_map_tx: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CPU-Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CPU-Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASRC-Playback\0".as_ptr() as *const c_char },
];

static audio_map_rx: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"CPU-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASRC-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CPU-Capture\0".as_ptr() as *const c_char },
];

/* Add all possible widgets into here without being redundant */
static fsl_asoc_card_dapm_widgets: [snd_soc_dapm_widget; 7] = [
    snd_soc_dapm_widget { id: 0, name: b"Line Out Jack\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Line In Jack\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Headphone Jack\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Ext Spk\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"Mic Jack\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"AMIC\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
    snd_soc_dapm_widget { id: 0, name: b"DMIC\0".as_ptr() as *const c_char, reg: 0, shift: 0, mask: 0, on_val: 0, off_val: 0 },
];

unsafe extern "C" fn fsl_asoc_card_is_ac97(priv: *mut fsl_asoc_card_priv) -> bool {
    unsafe { (*priv).dai_fmt == SND_SOC_DAIFMT_AC97 }
}

unsafe extern "C" fn fsl_asoc_card_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    unsafe {
        let rtd = snd_soc_substream_to_rtd(substream);
        let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut fsl_asoc_card_priv;
        let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
        let txi = tx as usize;
        let cpu_priv = &mut (*priv_).cpu_priv as *mut cpu_priv;
        let dev = (*(*rtd).card).dev;
        let mut pll_out: c_uint;
        let sysclk_freq: c_uint;
        let mut ret: c_int;

        (*priv_).sample_rate = params_rate(params);
        (*priv_).sample_format = params_format(params);
        (*priv_).streams |= BIT((*substream).stream);

        if fsl_asoc_card_is_ac97(priv_) {
            return 0;
        }

        if (*cpu_priv).sysclk_freq[txi] == 0 && (*cpu_priv).sysclk_ratio[txi] != 0 {
            sysclk_freq = (*priv_).sample_rate.wrapping_mul((*cpu_priv).sysclk_ratio[txi]);
        } else {
            sysclk_freq = (*cpu_priv).sysclk_freq[txi] as c_uint;
        }

        /* Specific configurations of DAIs starts from here */
        ret = snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), (*cpu_priv).sysclk_id[txi] as c_int, sysclk_freq, (*cpu_priv).sysclk_dir[txi] as c_int);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err(dev, b"failed to set sysclk for cpu dai\n\0".as_ptr() as *const c_char);
            goto_fail(priv_, substream, ret)
        } else {
            if (*cpu_priv).slot_width != 0 {
                if (*cpu_priv).slot_num == 0 {
                    (*cpu_priv).slot_num = 2;
                }
                ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, (*cpu_priv).slot_num as c_int, (*cpu_priv).slot_width as c_int);
                if ret != 0 && ret != -ENOTSUPP {
                    dev_err(dev, b"failed to set TDM slot for cpu dai\n\0".as_ptr() as *const c_char);
                    return goto_fail(priv_, substream, ret);
                }
            }

            let mut codec_idx = 0;
            while codec_idx < (*(*rtd).dai_link).num_codecs as c_int {
                let codec_dai = snd_soc_rtd_to_codec(rtd, codec_idx);
                let codec_priv = &mut (*priv_).codec_priv[codec_idx as usize] as *mut codec_priv;
                if (*codec_priv).pll_id >= 0 && (*codec_priv).fll_id >= 0 {
                    if (*priv_).sample_format == SNDRV_PCM_FORMAT_S24_LE {
                        pll_out = (*priv_).sample_rate.wrapping_mul((*codec_priv).pll_ratio_s24 as u32);
                    } else {
                        pll_out = (*priv_).sample_rate.wrapping_mul(256);
                    }
                    ret = snd_soc_dai_set_pll(codec_dai, (*codec_priv).pll_id, (*codec_priv).mclk_id as c_int, (*codec_priv).mclk_freq as c_uint, pll_out);
                    if ret != 0 {
                        dev_err(dev, b"failed to start FLL: %d\n\0".as_ptr() as *const c_char, ret);
                        return goto_fail(priv_, substream, ret);
                    }
                    ret = snd_soc_dai_set_sysclk(codec_dai, (*codec_priv).fll_id, pll_out, SND_SOC_CLOCK_IN);
                    if ret != 0 && ret != -ENOTSUPP {
                        dev_err(dev, b"failed to set SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
                        return goto_fail(priv_, substream, ret);
                    }
                }
                codec_idx += 1;
            }
            0
        }
    }
}

unsafe fn goto_fail(priv_: *mut fsl_asoc_card_priv, substream: *mut snd_pcm_substream, ret: c_int) -> c_int {
    unsafe {
        (*priv_).streams &= !BIT((*substream).stream);
        ret
    }
}

unsafe extern "C" fn fsl_asoc_card_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let rtd = snd_soc_substream_to_rtd(substream);
        let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut fsl_asoc_card_priv;
        let dev = (*(*rtd).card).dev;
        (*priv_).streams &= !BIT((*substream).stream);
        let mut codec_idx = 0;
        while codec_idx < (*(*rtd).dai_link).num_codecs as c_int {
            let codec_dai = snd_soc_rtd_to_codec(rtd, codec_idx);
            let codec_priv = &mut (*priv_).codec_priv[codec_idx as usize] as *mut codec_priv;
            if (*priv_).streams == 0 && (*codec_priv).pll_id >= 0 && (*codec_priv).fll_id >= 0 {
                /* Force freq to be free_freq to avoid error message in codec */
                let mut ret = snd_soc_dai_set_sysclk(codec_dai, (*codec_priv).mclk_id as c_int, (*codec_priv).free_freq as c_uint, SND_SOC_CLOCK_IN);
                if ret != 0 {
                    dev_err(dev, b"failed to switch away from FLL: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                ret = snd_soc_dai_set_pll(codec_dai, (*codec_priv).pll_id, 0, 0, 0);
                if ret != 0 && ret != -ENOTSUPP {
                    dev_err(dev, b"failed to stop FLL: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
            }
            codec_idx += 1;
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_startup(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
        let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut fsl_asoc_card_priv;
        let runtime = (*substream).runtime;
        let mut ret: c_int;
        if (*priv_).exclude_format != 0 && (*(*rtd).dai_link).no_pcm == 0 {
            ret = snd_pcm_hw_constraint_mask64(runtime, SNDRV_PCM_HW_PARAM_FORMAT, !(*priv_).exclude_format);
            if ret != 0 { return ret; }
        }
        if !(*priv_).constraint_channels.is_null() {
            ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, (*priv_).constraint_channels);
            if ret != 0 { return ret; }
        }
        /*
         * Apply rate constraints only to frontend DAI links (no_pcm = 0).
         * Skip DPCM backend (no_pcm = 1) as rate is fixed by be_hw_params_fixup()
         * and ASRC frontend handles rate conversion.
         */
        if !(*priv_).constraint_rates.is_null() && (*(*rtd).dai_link).no_pcm == 0 {
            ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, (*priv_).constraint_rates);
            if ret != 0 { return ret; }
        }
        0
    }
}

static fsl_asoc_card_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(fsl_asoc_card_startup),
    hw_params: Some(fsl_asoc_card_hw_params),
    hw_free: Some(fsl_asoc_card_hw_free),
};

unsafe extern "C" fn be_hw_params_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int {
    unsafe {
        let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut fsl_asoc_card_priv;
        let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
        (*rate).min = (*priv_).asrc_rate;
        (*rate).max = (*rate).min;
        let mask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
        snd_mask_none(mask);
        snd_mask_set_format(mask, (*priv_).asrc_format);
        0
    }
}

static fsl_asoc_card_dai: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link { name: b"HiFi\0".as_ptr() as *const c_char, stream_name: b"HiFi\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, ops: &fsl_asoc_card_ops, be_hw_params_fixup: None, dai_fmt: 0, dynamic: 0, no_pcm: 0, dpcm_merged_chan: 0, ignore_pmdown_time: 0, playback_only: false, capture_only: false },
    snd_soc_dai_link { name: b"HiFi-ASRC-FE\0".as_ptr() as *const c_char, stream_name: b"HiFi-ASRC-FE\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, ops: ptr::null(), be_hw_params_fixup: None, dai_fmt: 0, dynamic: 1, no_pcm: 0, dpcm_merged_chan: 0, ignore_pmdown_time: 0, playback_only: false, capture_only: false },
    snd_soc_dai_link { name: b"HiFi-ASRC-BE\0".as_ptr() as *const c_char, stream_name: b"HiFi-ASRC-BE\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, ops: &fsl_asoc_card_ops, be_hw_params_fixup: Some(be_hw_params_fixup), dai_fmt: 0, dynamic: 0, no_pcm: 1, dpcm_merged_chan: 0, ignore_pmdown_time: 0, playback_only: false, capture_only: false },
];

unsafe extern "C" fn fsl_asoc_card_audmux_init(np: *mut device_node, priv_: *mut fsl_asoc_card_priv) -> c_int {
    unsafe {
        let dev = &mut (*(*priv_).pdev).dev as *mut device;
        let mut int_ptcr: u32 = 0;
        let mut ext_ptcr: u32 = 0;
        let mut int_port: c_int = 0;
        let mut ext_port: c_int = 0;
        let mut ret = of_property_read_u32(np, b"mux-int-port\0".as_ptr() as *const c_char, &mut int_port as *mut _ as *mut u32);
        if ret != 0 {
            dev_err(dev, b"mux-int-port missing or invalid\n\0".as_ptr() as *const c_char);
            return ret;
        }
        ret = of_property_read_u32(np, b"mux-ext-port\0".as_ptr() as *const c_char, &mut ext_port as *mut _ as *mut u32);
        if ret != 0 {
            dev_err(dev, b"mux-ext-port missing or invalid\n\0".as_ptr() as *const c_char);
            return ret;
        }
        /*
         * The port numbering in the hardware manual starts at 1, while
         * the AUDMUX API expects it starts at 0.
         */
        int_port -= 1;
        ext_port -= 1;

        match (*priv_).dai_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            x if x == SND_SOC_DAIFMT_CBP_CFP => {
                int_ptcr = IMX_AUDMUX_V2_PTCR_RFSEL(8 | ext_port) | IMX_AUDMUX_V2_PTCR_RCSEL(8 | ext_port) | IMX_AUDMUX_V2_PTCR_TFSEL(ext_port) | IMX_AUDMUX_V2_PTCR_TCSEL(ext_port) | IMX_AUDMUX_V2_PTCR_RFSDIR | IMX_AUDMUX_V2_PTCR_RCLKDIR | IMX_AUDMUX_V2_PTCR_TFSDIR | IMX_AUDMUX_V2_PTCR_TCLKDIR;
            }
            x if x == SND_SOC_DAIFMT_CBP_CFC => {
                int_ptcr = IMX_AUDMUX_V2_PTCR_RCSEL(8 | ext_port) | IMX_AUDMUX_V2_PTCR_TCSEL(ext_port) | IMX_AUDMUX_V2_PTCR_RCLKDIR | IMX_AUDMUX_V2_PTCR_TCLKDIR;
                ext_ptcr = IMX_AUDMUX_V2_PTCR_RFSEL(8 | int_port) | IMX_AUDMUX_V2_PTCR_TFSEL(int_port) | IMX_AUDMUX_V2_PTCR_RFSDIR | IMX_AUDMUX_V2_PTCR_TFSDIR;
            }
            x if x == SND_SOC_DAIFMT_CBC_CFP => {
                int_ptcr = IMX_AUDMUX_V2_PTCR_RFSEL(8 | ext_port) | IMX_AUDMUX_V2_PTCR_TFSEL(ext_port) | IMX_AUDMUX_V2_PTCR_RFSDIR | IMX_AUDMUX_V2_PTCR_TFSDIR;
                ext_ptcr = IMX_AUDMUX_V2_PTCR_RCSEL(8 | int_port) | IMX_AUDMUX_V2_PTCR_TCSEL(int_port) | IMX_AUDMUX_V2_PTCR_RCLKDIR | IMX_AUDMUX_V2_PTCR_TCLKDIR;
            }
            x if x == SND_SOC_DAIFMT_CBC_CFC => {
                ext_ptcr = IMX_AUDMUX_V2_PTCR_RFSEL(8 | int_port) | IMX_AUDMUX_V2_PTCR_RCSEL(8 | int_port) | IMX_AUDMUX_V2_PTCR_TFSEL(int_port) | IMX_AUDMUX_V2_PTCR_TCSEL(int_port) | IMX_AUDMUX_V2_PTCR_RFSDIR | IMX_AUDMUX_V2_PTCR_RCLKDIR | IMX_AUDMUX_V2_PTCR_TFSDIR | IMX_AUDMUX_V2_PTCR_TCLKDIR;
            }
            _ => {
                if !fsl_asoc_card_is_ac97(priv_) {
                    return -EINVAL;
                }
            }
        }

        if fsl_asoc_card_is_ac97(priv_) {
            int_ptcr = IMX_AUDMUX_V2_PTCR_SYN | IMX_AUDMUX_V2_PTCR_TCSEL(ext_port) | IMX_AUDMUX_V2_PTCR_TCLKDIR;
            ext_ptcr = IMX_AUDMUX_V2_PTCR_SYN | IMX_AUDMUX_V2_PTCR_TFSEL(int_port) | IMX_AUDMUX_V2_PTCR_TFSDIR;
        }

        /* Asynchronous mode can not be set along with RCLKDIR */
        if !fsl_asoc_card_is_ac97(priv_) {
            let pdcr = IMX_AUDMUX_V2_PDCR_RXDSEL(ext_port);
            ret = imx_audmux_v2_configure_port(int_port, 0, pdcr);
            if ret != 0 {
                dev_err(dev, b"audmux internal port setup failed\n\0".as_ptr() as *const c_char);
                return ret;
            }
        }
        ret = imx_audmux_v2_configure_port(int_port, int_ptcr, IMX_AUDMUX_V2_PDCR_RXDSEL(ext_port));
        if ret != 0 {
            dev_err(dev, b"audmux internal port setup failed\n\0".as_ptr() as *const c_char);
            return ret;
        }
        if !fsl_asoc_card_is_ac97(priv_) {
            let pdcr = IMX_AUDMUX_V2_PDCR_RXDSEL(int_port);
            ret = imx_audmux_v2_configure_port(ext_port, 0, pdcr);
            if ret != 0 {
                dev_err(dev, b"audmux external port setup failed\n\0".as_ptr() as *const c_char);
                return ret;
            }
        }
        ret = imx_audmux_v2_configure_port(ext_port, ext_ptcr, IMX_AUDMUX_V2_PDCR_RXDSEL(int_port));
        if ret != 0 {
            dev_err(dev, b"audmux external port setup failed\n\0".as_ptr() as *const c_char);
            return ret;
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_spdif_init(codec_np: *mut *mut device_node, cpu_np: *mut device_node, codec_dai_name: *mut *const c_char, priv_: *mut fsl_asoc_card_priv) -> c_int {
    unsafe {
        let dev = &mut (*(*priv_).pdev).dev as *mut device;
        let np = (*dev).of_node;
        if !of_node_name_eq(cpu_np, b"spdif\0".as_ptr() as *const c_char) {
            dev_err(dev, b"CPU phandle invalid, should be an SPDIF device\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        (*priv_).dai_link[0].playback_only = true;
        (*priv_).dai_link[0].capture_only = true;
        let mut i = 0;
        while i < 2 {
            if (*codec_np.add(i)).is_null() { break; }
            if of_device_is_compatible(*codec_np.add(i), b"linux,spdif-dit\0".as_ptr() as *const c_char) {
                (*priv_).dai_link[0].capture_only = false;
                *codec_dai_name.add(i) = b"dit-hifi\0".as_ptr() as *const c_char;
            } else if of_device_is_compatible(*codec_np.add(i), b"linux,spdif-dir\0".as_ptr() as *const c_char) {
                (*priv_).dai_link[0].playback_only = false;
                *codec_dai_name.add(i) = b"dir-hifi\0".as_ptr() as *const c_char;
            }
            i += 1;
        }
        // Old SPDIF DT binding
        if (*codec_np.add(0)).is_null() {
            *codec_dai_name.add(0) = snd_soc_dummy_dlc.dai_name;
            if of_property_read_bool(np, b"spdif-out\0".as_ptr() as *const c_char) { (*priv_).dai_link[0].capture_only = false; }
            if of_property_read_bool(np, b"spdif-in\0".as_ptr() as *const c_char) { (*priv_).dai_link[0].playback_only = false; }
        }
        if (*priv_).dai_link[0].playback_only && (*priv_).dai_link[0].capture_only {
            dev_err(dev, b"no enabled S/PDIF DAI link\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        if (*priv_).dai_link[0].playback_only {
            (*priv_).dai_link[1].playback_only = true;
            (*priv_).dai_link[2].playback_only = true;
            (*priv_).card.dapm_routes = audio_map_tx.as_ptr();
            (*priv_).card.num_dapm_routes = 2;
        } else if (*priv_).dai_link[0].capture_only {
            (*priv_).dai_link[1].capture_only = true;
            (*priv_).dai_link[2].capture_only = true;
            (*priv_).card.dapm_routes = audio_map_rx.as_ptr();
            (*priv_).card.num_dapm_routes = 2;
        }
        // No DAPM routes with old bindings and dummy codec
        if (*codec_np.add(0)).is_null() {
            (*priv_).card.dapm_routes = ptr::null();
            (*priv_).card.num_dapm_routes = 0;
        }
        if !(*codec_np.add(0)).is_null() && !(*codec_np.add(1)).is_null() {
            (*priv_).dai_link[0].num_codecs = 2;
            (*priv_).dai_link[2].num_codecs = 2;
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_cs42888_codec_init(priv_: *mut fsl_asoc_card_priv) -> c_int {
    unsafe {
        let mclk_freq = (*priv_).codec_priv[0].mclk_freq;
        if (*priv_).cpu_priv.sysclk_freq[TX] == 0 { (*priv_).cpu_priv.sysclk_freq[TX] = mclk_freq; }
        if (*priv_).cpu_priv.sysclk_freq[RX] == 0 { (*priv_).cpu_priv.sysclk_freq[RX] = mclk_freq; }
        (*priv_).constraint_channels = &cs42888_channel_constraints;
        if mclk_freq % 12288000 == 0 {
            (*priv_).constraint_rates = &cs42888_rate_48k_constraints;
        } else if mclk_freq % 11289600 == 0 {
            (*priv_).constraint_rates = &cs42888_rate_44k_constraints;
        } else {
            dev_warn(&mut (*(*priv_).pdev).dev, b"Unknown MCLK frequency %lu, no rate constraints\n\0".as_ptr() as *const c_char, mclk_freq);
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_wm8958_codec_init(priv_: *mut fsl_asoc_card_priv) -> c_int {
    unsafe {
        (*priv_).codec_priv[0].free_freq = (*priv_).codec_priv[0].mclk_freq;
        0
    }
}

macro_rules! pdata {
    ($codec:expr, $fmt:expr) => {
        fsl_asoc_card_pdata { sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_dai_name: $codec.as_ptr() as *const c_char, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, playback_only: false, capture_only: false, dai_fmt: $fmt, dapm_routes: audio_map.as_ptr(), num_dapm_routes: 4, exclude_format: 0, codec_init: None, probe_init: None }
    };
}

static fsl_asoc_cs42888_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"cs42888\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBC_CFC, sysclk_dir: [SND_SOC_CLOCK_OUT, SND_SOC_CLOCK_OUT], slot_width: 32, dapm_routes: audio_map.as_ptr(), num_dapm_routes: 4, codec_init: Some(fsl_asoc_card_cs42888_codec_init), sysclk_ratio: [0, 0], codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, playback_only: false, capture_only: false, exclude_format: 0, probe_init: None };
static fsl_asoc_cs427x_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_mclk_id: CS427x_SYSCLK_MCLK, ..pdata!(b"cs4271-hifi\0", DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP) };
static fsl_asoc_sgtl5000_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_mclk_id: SGTL5000_SYSCLK, ..pdata!(b"sgtl5000\0", DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP) };
static fsl_asoc_tlv320aic32x4_pdata: fsl_asoc_card_pdata = pdata!(b"tlv320aic32x4-hifi\0", DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP);
static fsl_asoc_tlv320aic31xx_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"tlv320dac31xx-hifi\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBC_CFC, sysclk_dir: [SND_SOC_CLOCK_OUT, SND_SOC_CLOCK_OUT], playback_only: true, dapm_routes: audio_map_tx.as_ptr(), num_dapm_routes: 2, sysclk_ratio: [0, 0], slot_width: 0, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_wm8962_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"wm8962\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP, codec_mclk_id: WM8962_SYSCLK_MCLK, has_pll: true, codec_fll_id: WM8962_SYSCLK_FLL, codec_pll_id: WM8962_FLL, exclude_format: SNDRV_PCM_FMTBIT_S20_3LE, dapm_routes: audio_map.as_ptr(), num_dapm_routes: 4, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_pll_ratio_s24: 0, playback_only: false, capture_only: false, codec_init: None, probe_init: None };
static fsl_asoc_wm8960_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"wm8960-hifi\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP, has_pll: true, codec_fll_id: WM8960_SYSCLK_AUTO, codec_pll_id: WM8960_SYSCLK_AUTO, exclude_format: SNDRV_PCM_FMTBIT_S20_3LE, dapm_routes: audio_map.as_ptr(), num_dapm_routes: 4, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_mclk_id: 0, codec_pll_ratio_s24: 0, playback_only: false, capture_only: false, codec_init: None, probe_init: None };
static fsl_asoc_ac97_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"ac97-hifi\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_AC97, dapm_routes: audio_map_ac97.as_ptr(), num_dapm_routes: 4, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, playback_only: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_mqs_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"fsl-mqs-dai\0".as_ptr() as *const c_char, dai_fmt: SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_NB_NF, playback_only: true, dapm_routes: audio_map_tx.as_ptr(), num_dapm_routes: 2, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_wm8524_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"wm8524-hifi\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBC_CFC, sysclk_dir: [0, SND_SOC_CLOCK_OUT], sysclk_ratio: [0, 256], slot_width: 32, playback_only: true, dapm_routes: audio_map_tx.as_ptr(), num_dapm_routes: 2, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_si476x_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"si476x-codec\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBC_CFC, dapm_routes: audio_map_rx.as_ptr(), num_dapm_routes: 2, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, playback_only: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_wm8958_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"wm8994-aif1\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP, codec_mclk_id: WM8994_FLL_SRC_MCLK1, has_pll: true, codec_fll_id: WM8994_SYSCLK_FLL1, codec_pll_id: WM8994_FLL1, codec_init: Some(fsl_asoc_card_wm8958_codec_init), sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_pll_ratio_s24: 0, playback_only: false, capture_only: false, dapm_routes: ptr::null(), num_dapm_routes: 0, exclude_format: 0, probe_init: None };
static fsl_asoc_nau8822_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"nau8822-hifi\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP, codec_mclk_id: NAU8822_CLK_MCLK, has_pll: true, codec_fll_id: NAU8822_CLK_PLL, codec_pll_id: NAU8822_CLK_PLL, dapm_routes: audio_map.as_ptr(), num_dapm_routes: 4, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_pll_ratio_s24: 0, playback_only: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_wm8904_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"wm8904-hifi\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE | SND_SOC_DAIFMT_CBP_CFP, codec_mclk_id: WM8904_FLL_MCLK, has_pll: true, codec_fll_id: WM8904_CLK_FLL, codec_pll_id: WM8904_FLL_MCLK as c_int, codec_pll_ratio_s24: 192, dapm_routes: audio_map.as_ptr(), num_dapm_routes: 4, sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, playback_only: false, capture_only: false, exclude_format: 0, codec_init: None, probe_init: None };
static fsl_asoc_spdif_pdata: fsl_asoc_card_pdata = fsl_asoc_card_pdata { codec_dai_name: b"spdif\0".as_ptr() as *const c_char, dai_fmt: DAI_FMT_BASE, probe_init: Some(fsl_asoc_card_spdif_init), sysclk_dir: [0, 0], sysclk_ratio: [0, 0], slot_width: 0, codec_mclk_id: 0, codec_fll_id: 0, codec_pll_id: 0, codec_pll_ratio_s24: 0, has_pll: false, playback_only: false, capture_only: false, dapm_routes: ptr::null(), num_dapm_routes: 0, exclude_format: 0, codec_init: None };

unsafe extern "C" fn hp_jack_event(_nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int {
    unsafe {
        let jack = data as *mut snd_soc_jack;
        let dapm = snd_soc_card_to_dapm((*jack).card);
        if event & SND_JACK_HEADPHONE != 0 {
            /* Disable speaker if headphone is plugged in */
            snd_soc_dapm_disable_pin(dapm, b"Ext Spk\0".as_ptr() as *const c_char)
        } else {
            snd_soc_dapm_enable_pin(dapm, b"Ext Spk\0".as_ptr() as *const c_char)
        }
    }
}
static mut hp_jack_nb: notifier_block = notifier_block { notifier_call: Some(hp_jack_event) };

unsafe extern "C" fn mic_jack_event(_nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int {
    unsafe {
        let jack = data as *mut snd_soc_jack;
        let dapm = snd_soc_card_to_dapm((*jack).card);
        if event & SND_JACK_MICROPHONE != 0 {
            /* Disable dmic if microphone is plugged in */
            snd_soc_dapm_disable_pin(dapm, b"DMIC\0".as_ptr() as *const c_char)
        } else {
            snd_soc_dapm_enable_pin(dapm, b"DMIC\0".as_ptr() as *const c_char)
        }
    }
}
static mut mic_jack_nb: notifier_block = notifier_block { notifier_call: Some(mic_jack_event) };

unsafe extern "C" fn fsl_asoc_card_init_cpu(card: *mut snd_soc_card, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    unsafe {
        let priv_ = snd_soc_card_get_drvdata(card) as *mut fsl_asoc_card_priv;
        let np = (*(*priv_).pdev).dev.of_node;
        let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
        let comp_drv_name = (*(*(*cpu_dai).component).driver).name;
        let dev = (*card).dev;
        let mut ret: c_int;
        if strcmp(comp_drv_name, b"fsl-ssi\0".as_ptr() as *const c_char) == 0 {
            /* Only SSI needs to configure AUDMUX */
            ret = fsl_asoc_card_audmux_init(np, priv_);
            if ret != 0 {
                dev_err(dev, b"failed to init audmux\n\0".as_ptr() as *const c_char);
                return ret;
            }
        } else if strcmp(comp_drv_name, b"fsl-esai\0".as_ptr() as *const c_char) == 0 {
            let esai_clk = clk_get((*cpu_dai).dev, b"extal\0".as_ptr() as *const c_char);
            if !IS_ERR(esai_clk as *const c_void) {
                (*priv_).cpu_priv.sysclk_freq[TX] = clk_get_rate(esai_clk);
                (*priv_).cpu_priv.sysclk_freq[RX] = clk_get_rate(esai_clk);
                clk_put(esai_clk);
            } else {
                dev_warn(dev, b"failed to get ESAI extal clock: %ld\n\0".as_ptr() as *const c_char, PTR_ERR(esai_clk as *const c_void));
            }
            (*priv_).cpu_priv.sysclk_id[TX] = ESAI_HCKT_EXTAL;
            (*priv_).cpu_priv.sysclk_id[RX] = ESAI_HCKR_EXTAL;
        } else if strcmp(comp_drv_name, b"fsl-sai\0".as_ptr() as *const c_char) == 0 {
            (*priv_).cpu_priv.sysclk_id[TX] = FSL_SAI_CLK_MAST1;
            (*priv_).cpu_priv.sysclk_id[RX] = FSL_SAI_CLK_MAST1;
            if (*(*priv_).pdata).exclude_format != 0 {
                (*priv_).exclude_format = (*(*priv_).pdata).exclude_format;
            }
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_init_codecs(card: *mut snd_soc_card, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    unsafe {
        let priv_ = snd_soc_card_get_drvdata(card) as *mut fsl_asoc_card_priv;
        let pdata = (*priv_).pdata;
        let dev = (*card).dev;
        let mut codec_idx = 0;
        while codec_idx < (*(*rtd).dai_link).num_codecs as c_int {
            let codec_dai = snd_soc_rtd_to_codec(rtd, codec_idx);
            let codec_clk = clk_get((*(*codec_dai).component).dev, ptr::null());
            let codec_priv = &mut (*priv_).codec_priv[codec_idx as usize] as *mut codec_priv;
            if !IS_ERR(codec_clk as *const c_void) {
                (*codec_priv).mclk_freq = clk_get_rate(codec_clk);
                clk_put(codec_clk);
            }
            codec_idx += 1;
        }
        if let Some(init) = (*pdata).codec_init {
            let ret = init(priv_);
            if ret != 0 { return ret; }
        }
        codec_idx = 0;
        while codec_idx < (*(*rtd).dai_link).num_codecs as c_int {
            let codec_dai = snd_soc_rtd_to_codec(rtd, codec_idx);
            let codec_priv = &mut (*priv_).codec_priv[codec_idx as usize] as *mut codec_priv;
            let ret = snd_soc_dai_set_sysclk(codec_dai, (*codec_priv).mclk_id as c_int, (*codec_priv).mclk_freq as c_uint, SND_SOC_CLOCK_IN);
            if ret != 0 && ret != -ENOTSUPP {
                dev_err(dev, b"failed to set sysclk in %s\n\0".as_ptr() as *const c_char, b"fsl_asoc_card_init_codecs\0".as_ptr() as *const c_char);
                return ret;
            }
            codec_idx += 1;
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_free_jack(card: *mut snd_soc_card) {
    unsafe {
        let priv_ = snd_soc_card_get_drvdata(card) as *mut fsl_asoc_card_priv;
        if !(*priv_).hp_jack.gpio.desc.is_null() {
            snd_soc_jack_notifier_unregister(&mut (*priv_).hp_jack.jack, &raw mut hp_jack_nb);
            snd_soc_jack_free_gpios(&mut (*priv_).hp_jack.jack, 1, &mut (*priv_).hp_jack.gpio);
            (*priv_).hp_jack.gpio.desc = ptr::null_mut();
        }
        if !(*priv_).mic_jack.gpio.desc.is_null() {
            snd_soc_jack_notifier_unregister(&mut (*priv_).mic_jack.jack, &raw mut mic_jack_nb);
            snd_soc_jack_free_gpios(&mut (*priv_).mic_jack.jack, 1, &mut (*priv_).mic_jack.gpio);
            (*priv_).mic_jack.gpio.desc = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn fsl_asoc_card_init_jack(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let priv_ = snd_soc_card_get_drvdata(card) as *mut fsl_asoc_card_priv;
        let np = (*(*priv_).pdev).dev.of_node;
        let mut ret: c_int;
        if of_property_present(np, b"hp-det-gpios\0".as_ptr() as *const c_char) || of_property_present(np, b"hp-det-gpio\0".as_ptr() as *const c_char) {
            ret = simple_util_init_jack(card, &mut (*priv_).hp_jack, 1, ptr::null(), b"Headphone Jack\0".as_ptr() as *const c_char);
            if ret != 0 { return ret; }
            snd_soc_jack_notifier_register(&mut (*priv_).hp_jack.jack, &raw mut hp_jack_nb);
        }
        if of_property_present(np, b"mic-det-gpios\0".as_ptr() as *const c_char) || of_property_present(np, b"mic-det-gpio\0".as_ptr() as *const c_char) {
            ret = simple_util_init_jack(card, &mut (*priv_).mic_jack, 0, ptr::null(), b"Mic Jack\0".as_ptr() as *const c_char);
            if ret != 0 { return ret; }
            snd_soc_jack_notifier_register(&mut (*priv_).mic_jack.jack, &raw mut mic_jack_nb);
        }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_late_probe(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let priv_ = snd_soc_card_get_drvdata(card) as *mut fsl_asoc_card_priv;
        /* Use the first rtd which carries the CPU+codec DAIs */
        let rtd = (*(*card).rtd_list.next).next as *mut snd_soc_pcm_runtime;
        let mut ret = fsl_asoc_card_init_jack(card);
        if ret != 0 { fsl_asoc_card_free_jack(card); return ret; }
        ret = fsl_asoc_card_init_cpu(card, rtd);
        if ret != 0 { fsl_asoc_card_free_jack(card); return ret; }
        if fsl_asoc_card_is_ac97(priv_) {
            /* IS_ENABLED(CONFIG_SND_AC97_CODEC): update AC97 slots when support is built in. */
            let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
            let ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;
            snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPSA_SLOT_MASK, AC97_EA_SPSA_3_4);
            return 0;
        }
        ret = fsl_asoc_card_init_codecs(card, rtd);
        if ret != 0 { fsl_asoc_card_free_jack(card); return ret; }
        0
    }
}

unsafe extern "C" fn fsl_asoc_card_card_remove(card: *mut snd_soc_card) -> c_int {
    unsafe { fsl_asoc_card_free_jack(card); }
    0
}

unsafe fn link_codec(link: *mut snd_soc_dai_link, idx: c_int) -> *mut snd_soc_dai_link_component {
    unsafe { (*link).codecs.add(idx as usize) }
}

unsafe extern "C" fn fsl_asoc_card_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut cpu_np: *mut device_node;
        let mut asrc_np: *mut device_node;
        let mut codec_np: [*mut device_node; 2] = [ptr::null_mut(), ptr::null_mut()];
        let np = (*pdev).dev.of_node;
        let mut asrc_pdev: *mut platform_device = ptr::null_mut();
        let mut bitclkprovider: *mut device_node = ptr::null_mut();
        let mut frameprovider: *mut device_node = ptr::null_mut();
        let priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<fsl_asoc_card_priv>(), GFP_KERNEL) as *mut fsl_asoc_card_priv;
        let pdata = of_device_get_match_data(&mut (*pdev).dev) as *const fsl_asoc_card_pdata;
        let mut codec_dai_name: [*const c_char; 2] = [ptr::null(), ptr::null()];
        let mut asrc_fmt: u32 = 0;
        let mut width: u32 = 0;
        let mut ret: c_int;

        if priv_.is_null() { return -ENOMEM; }
        (*priv_).pdev = pdev;
        if pdata.is_null() {
            dev_err(&mut (*pdev).dev, b"unknown Device Tree compatible\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        (*priv_).pdata = pdata;

        cpu_np = of_parse_phandle(np, b"audio-cpu\0".as_ptr() as *const c_char, 0);
        /* Give a chance to old DT bindings */
        if cpu_np.is_null() { cpu_np = of_parse_phandle(np, b"ssi-controller\0".as_ptr() as *const c_char, 0); }
        if cpu_np.is_null() { cpu_np = of_parse_phandle(np, b"spdif-controller\0".as_ptr() as *const c_char, 0); }
        if cpu_np.is_null() {
            dev_err(&mut (*pdev).dev, b"CPU phandle missing or invalid\n\0".as_ptr() as *const c_char);
            ret = -EINVAL;
            return ret;
        }

        codec_np[0] = of_parse_phandle(np, b"audio-codec\0".as_ptr() as *const c_char, 0);
        codec_np[1] = of_parse_phandle(np, b"audio-codec\0".as_ptr() as *const c_char, 1);
        asrc_np = of_parse_phandle(np, b"audio-asrc\0".as_ptr() as *const c_char, 0);
        if !asrc_np.is_null() { asrc_pdev = of_find_device_by_node(asrc_np); }

        (*priv_).sample_rate = 44100;
        (*priv_).sample_format = SNDRV_PCM_FORMAT_S16_LE;
        (*priv_).dai_fmt = DAI_FMT_BASE;
        ptr::copy_nonoverlapping(fsl_asoc_card_dai.as_ptr(), (*priv_).dai_link.as_mut_ptr(), 3);

        let dlc = devm_kcalloc(&mut (*pdev).dev, 10, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
        if dlc.is_null() { ret = -ENOMEM; goto_probe_fail(asrc_np, codec_np, cpu_np); return ret; }

        (*priv_).dai_link[0].cpus = dlc.add(0); (*priv_).dai_link[0].num_cpus = 1;
        (*priv_).dai_link[0].codecs = dlc.add(1); (*priv_).dai_link[0].num_codecs = 1;
        (*priv_).dai_link[0].platforms = dlc.add(3); (*priv_).dai_link[0].num_platforms = 1;
        (*priv_).dai_link[1].cpus = dlc.add(4); (*priv_).dai_link[1].num_cpus = 1;
        (*priv_).dai_link[1].codecs = dlc.add(5); (*priv_).dai_link[1].num_codecs = 0;
        (*priv_).dai_link[1].platforms = dlc.add(6); (*priv_).dai_link[1].num_platforms = 1;
        (*priv_).dai_link[2].cpus = dlc.add(7); (*priv_).dai_link[2].num_cpus = 1;
        (*priv_).dai_link[2].codecs = dlc.add(8); (*priv_).dai_link[2].num_codecs = 1;

        (*priv_).card.dapm_routes = audio_map.as_ptr();
        (*priv_).card.num_dapm_routes = 4;
        (*priv_).card.driver_name = DRIVER_NAME;
        for codec_idx in 0..2 {
            (*priv_).codec_priv[codec_idx].fll_id = -1;
            (*priv_).codec_priv[codec_idx].pll_id = -1;
            (*priv_).codec_priv[codec_idx].pll_ratio_s24 = 384;
        }

        (*priv_).cpu_priv.sysclk_dir[TX] = (*pdata).sysclk_dir[TX];
        (*priv_).cpu_priv.sysclk_dir[RX] = (*pdata).sysclk_dir[RX];
        (*priv_).cpu_priv.sysclk_ratio[TX] = (*pdata).sysclk_ratio[TX];
        (*priv_).cpu_priv.sysclk_ratio[RX] = (*pdata).sysclk_ratio[RX];
        (*priv_).cpu_priv.slot_width = (*pdata).slot_width;
        codec_dai_name[0] = (*pdata).codec_dai_name;
        (*priv_).codec_priv[0].mclk_id = (*pdata).codec_mclk_id;
        if (*pdata).has_pll {
            (*priv_).codec_priv[0].fll_id = (*pdata).codec_fll_id;
            (*priv_).codec_priv[0].pll_id = (*pdata).codec_pll_id;
        }
        if (*pdata).codec_pll_ratio_s24 != 0 { (*priv_).codec_priv[0].pll_ratio_s24 = (*pdata).codec_pll_ratio_s24; }
        if (*pdata).playback_only { (*priv_).dai_link[1].playback_only = true; (*priv_).dai_link[2].playback_only = true; }
        if (*pdata).capture_only { (*priv_).dai_link[1].capture_only = true; (*priv_).dai_link[2].capture_only = true; }
        (*priv_).dai_fmt = (*pdata).dai_fmt;
        (*priv_).card.dapm_routes = (*pdata).dapm_routes;
        (*priv_).card.num_dapm_routes = (*pdata).num_dapm_routes;

        if let Some(init) = (*pdata).probe_init {
            ret = init(codec_np.as_mut_ptr(), cpu_np, codec_dai_name.as_mut_ptr(), priv_);
            if ret != 0 { goto_probe_fail(asrc_np, codec_np, cpu_np); return ret; }
        }

        let mut codec_idx = 0;
        while codec_idx < (*priv_).dai_link[0].num_codecs as c_int {
            of_property_read_u32_index(np, b"mclk-id\0".as_ptr() as *const c_char, codec_idx as c_uint, &mut (*priv_).codec_priv[codec_idx as usize].mclk_id);
            codec_idx += 1;
        }

        snd_soc_daifmt_parse_clock_provider_as_phandle(np, ptr::null(), &mut bitclkprovider, &mut frameprovider);
        if !bitclkprovider.is_null() || !frameprovider.is_null() {
            let mut daifmt = snd_soc_daifmt_parse_format(np, ptr::null());
            let mut codec_bitclkprovider = false;
            let mut codec_frameprovider = false;
            codec_idx = 0;
            while codec_idx < (*priv_).dai_link[0].num_codecs as c_int {
                if !bitclkprovider.is_null() && codec_np[codec_idx as usize] == bitclkprovider { codec_bitclkprovider = true; }
                if !frameprovider.is_null() && codec_np[codec_idx as usize] == frameprovider { codec_frameprovider = true; }
                codec_idx += 1;
            }
            if codec_bitclkprovider {
                daifmt |= if codec_frameprovider { SND_SOC_DAIFMT_CBP_CFP } else { SND_SOC_DAIFMT_CBP_CFC };
            } else {
                daifmt |= if codec_frameprovider { SND_SOC_DAIFMT_CBC_CFP } else { SND_SOC_DAIFMT_CBC_CFC };
            }
            /* Override dai_fmt with value from DT */
            (*priv_).dai_fmt = daifmt;
        }
        if ((*priv_).dai_fmt & SND_SOC_DAIFMT_CBP_CFP) != 0 {
            (*priv_).cpu_priv.sysclk_dir[TX] = SND_SOC_CLOCK_IN as u32;
            (*priv_).cpu_priv.sysclk_dir[RX] = SND_SOC_CLOCK_IN as u32;
        }
        of_node_put(bitclkprovider);
        of_node_put(frameprovider);

        (*priv_).card.dev = &mut (*pdev).dev;
        (*priv_).card.owner = THIS_MODULE;
        ret = snd_soc_of_parse_card_name(&mut (*priv_).card, b"model\0".as_ptr() as *const c_char);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"Error parsing card name: %d\n\0".as_ptr() as *const c_char, ret);
            goto_probe_fail(asrc_np, codec_np, cpu_np);
            return ret;
        }
        (*priv_).card.dai_link = (*priv_).dai_link.as_mut_ptr();
        (*priv_).card.late_probe = Some(fsl_asoc_card_late_probe);
        (*priv_).card.remove = Some(fsl_asoc_card_card_remove);
        (*priv_).card.dapm_widgets = fsl_asoc_card_dapm_widgets.as_ptr();
        (*priv_).card.num_dapm_widgets = 7;

        if asrc_pdev.is_null() { (*priv_).card.num_dapm_routes /= 2; }
        if of_property_present(np, b"audio-routing\0".as_ptr() as *const c_char) {
            ret = snd_soc_of_parse_audio_routing(&mut (*priv_).card, b"audio-routing\0".as_ptr() as *const c_char);
            if ret != 0 {
                dev_err(&mut (*pdev).dev, b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char, ret);
                goto_probe_fail(asrc_np, codec_np, cpu_np);
                return ret;
            }
        }

        (*(*priv_).dai_link[0].cpus).of_node = cpu_np;
        codec_idx = 0;
        while codec_idx < (*priv_).dai_link[0].num_codecs as c_int {
            (*link_codec(&mut (*priv_).dai_link[0], codec_idx)).dai_name = codec_dai_name[codec_idx as usize];
            codec_idx += 1;
        }
        // Old SPDIF DT binding support
        if codec_dai_name[0] == snd_soc_dummy_dlc.dai_name {
            (*(*priv_).dai_link[0].codecs.add(0)).name = snd_soc_dummy_dlc.name;
        }
        if !fsl_asoc_card_is_ac97(priv_) {
            codec_idx = 0;
            while codec_idx < (*priv_).dai_link[0].num_codecs as c_int {
                (*link_codec(&mut (*priv_).dai_link[0], codec_idx)).of_node = codec_np[codec_idx as usize];
                codec_idx += 1;
            }
        } else {
            let mut idx: u32 = 0;
            ret = of_property_read_u32(cpu_np, b"cell-index\0".as_ptr() as *const c_char, &mut idx);
            if ret != 0 {
                dev_err(&mut (*pdev).dev, b"cannot get CPU index property\n\0".as_ptr() as *const c_char);
                goto_probe_fail(asrc_np, codec_np, cpu_np);
                return ret;
            }
            (*(*priv_).dai_link[0].codecs.add(0)).name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, b"ac97-codec.%u\0".as_ptr() as *const c_char, idx as c_uint);
            if (*(*priv_).dai_link[0].codecs.add(0)).name.is_null() {
                ret = -ENOMEM;
                goto_probe_fail(asrc_np, codec_np, cpu_np);
                return ret;
            }
        }
        (*(*priv_).dai_link[0].platforms).of_node = cpu_np;
        (*priv_).dai_link[0].dai_fmt = (*priv_).dai_fmt;
        (*priv_).card.num_links = 1;

        if !asrc_pdev.is_null() {
            (*priv_).dai_link[1].dpcm_merged_chan = 1;
            (*priv_).dai_link[1].ignore_pmdown_time = 1;
            (*(*priv_).dai_link[1].cpus).of_node = asrc_np;
            (*(*priv_).dai_link[1].platforms).of_node = asrc_np;
            codec_idx = 0;
            while codec_idx < (*priv_).dai_link[2].num_codecs as c_int {
                (*link_codec(&mut (*priv_).dai_link[2], codec_idx)).dai_name = (*(*priv_).dai_link[0].codecs.add(codec_idx as usize)).dai_name;
                (*link_codec(&mut (*priv_).dai_link[2], codec_idx)).of_node = (*(*priv_).dai_link[0].codecs.add(codec_idx as usize)).of_node;
                (*link_codec(&mut (*priv_).dai_link[2], codec_idx)).name = (*(*priv_).dai_link[0].codecs.add(codec_idx as usize)).name;
                codec_idx += 1;
            }
            (*(*priv_).dai_link[2].cpus).of_node = cpu_np;
            (*priv_).dai_link[2].dai_fmt = (*priv_).dai_fmt;
            (*priv_).dai_link[2].ignore_pmdown_time = 1;
            (*priv_).card.num_links = 3;
            ret = of_property_read_u32(asrc_np, b"fsl,asrc-rate\0".as_ptr() as *const c_char, &mut (*priv_).asrc_rate);
            if ret != 0 {
                dev_err(&mut (*pdev).dev, b"failed to get output rate\n\0".as_ptr() as *const c_char);
                ret = -EINVAL;
                goto_probe_fail(asrc_np, codec_np, cpu_np);
                return ret;
            }
            ret = of_property_read_u32(asrc_np, b"fsl,asrc-format\0".as_ptr() as *const c_char, &mut asrc_fmt);
            (*priv_).asrc_format = asrc_fmt as snd_pcm_format_t;
            if ret != 0 {
                ret = of_property_read_u32(asrc_np, b"fsl,asrc-width\0".as_ptr() as *const c_char, &mut width);
                if ret != 0 {
                    dev_err(&mut (*pdev).dev, b"failed to decide output format\n\0".as_ptr() as *const c_char);
                    goto_probe_fail(asrc_np, codec_np, cpu_np);
                    return ret;
                }
                if width == 24 { (*priv_).asrc_format = SNDRV_PCM_FORMAT_S24_LE; } else { (*priv_).asrc_format = SNDRV_PCM_FORMAT_S16_LE; }
            }
        }

        platform_set_drvdata(pdev, priv_ as *mut c_void);
        snd_soc_card_set_drvdata(&mut (*priv_).card, priv_ as *mut c_void);
        ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut (*priv_).card);
        if ret != 0 {
            dev_err_probe(&mut (*pdev).dev, ret, b"snd_soc_register_card failed\n\0".as_ptr() as *const c_char);
        }
        goto_probe_fail(asrc_np, codec_np, cpu_np);
        ret
    }
}

unsafe fn goto_probe_fail(asrc_np: *mut device_node, codec_np: [*mut device_node; 2], cpu_np: *mut device_node) {
    unsafe {
        of_node_put(asrc_np);
        of_node_put(codec_np[0]);
        of_node_put(codec_np[1]);
        of_node_put(cpu_np);
    }
}

static fsl_asoc_card_dt_ids: [of_device_id; 16] = [
    of_device_id { compatible: b"fsl,imx-audio-ac97\0".as_ptr() as *const c_char, data: &fsl_asoc_ac97_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-cs42888\0".as_ptr() as *const c_char, data: &fsl_asoc_cs42888_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-cs427x\0".as_ptr() as *const c_char, data: &fsl_asoc_cs427x_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-tlv320aic32x4\0".as_ptr() as *const c_char, data: &fsl_asoc_tlv320aic32x4_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-tlv320aic31xx\0".as_ptr() as *const c_char, data: &fsl_asoc_tlv320aic31xx_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-sgtl5000\0".as_ptr() as *const c_char, data: &fsl_asoc_sgtl5000_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-wm8962\0".as_ptr() as *const c_char, data: &fsl_asoc_wm8962_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-wm8960\0".as_ptr() as *const c_char, data: &fsl_asoc_wm8960_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-mqs\0".as_ptr() as *const c_char, data: &fsl_asoc_mqs_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-wm8524\0".as_ptr() as *const c_char, data: &fsl_asoc_wm8524_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-si476x\0".as_ptr() as *const c_char, data: &fsl_asoc_si476x_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-wm8958\0".as_ptr() as *const c_char, data: &fsl_asoc_wm8958_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-nau8822\0".as_ptr() as *const c_char, data: &fsl_asoc_nau8822_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-wm8904\0".as_ptr() as *const c_char, data: &fsl_asoc_wm8904_pdata as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx-audio-spdif\0".as_ptr() as *const c_char, data: &fsl_asoc_spdif_pdata as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, fsl_asoc_card_dt_ids); */

static mut fsl_asoc_card_driver: platform_driver = platform_driver {
    probe: Some(fsl_asoc_card_probe),
    driver: platform_driver_inner {
        name: DRIVER_NAME,
        pm: &snd_soc_pm_ops as *const _ as *const c_void,
        of_match_table: fsl_asoc_card_dt_ids.as_ptr(),
    },
};

unsafe fn register_fsl_asoc_card_driver() {
    unsafe { module_platform_driver(&raw mut fsl_asoc_card_driver); }
}

/* MODULE_DESCRIPTION("Freescale Generic ASoC Sound Card driver with ASRC"); */
/* MODULE_AUTHOR("Nicolin Chen <nicoleotsuka@gmail.com>"); */
/* MODULE_ALIAS("platform:" DRIVER_NAME); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
