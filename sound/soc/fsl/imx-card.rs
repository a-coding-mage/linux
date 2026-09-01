// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017-2021 NXP

// Translated from Linux ASoC C source. Kernel headers and "fsl_sai.h" provide
// the external types, constants, macros, and functions declared below.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u32 = c_uint;
type bool_ = bool;
type snd_pcm_format_t = c_int;

const IMX_CARD_MCLK_22P5792MHZ: c_ulong = 22579200;
const IMX_CARD_MCLK_24P576MHZ: c_ulong = 24576000;

const CODEC_DUMMY: codec_type = 0;
const CODEC_AK5558: codec_type = 1;
const CODEC_AK4458: codec_type = 2;
const CODEC_AK4497: codec_type = 3;
const CODEC_AK5552: codec_type = 4;
const CODEC_CS42888: codec_type = 5;
const CODEC_WM8524: codec_type = 6;
type codec_type = c_uint;

/*
 * Mapping LRCK fs and frame width, table 3 & 4 in datasheet
 * @rmin: min rate
 * @rmax: max rate
 * @wmin: min frame ratio
 * @wmax: max frame ratio
 */
#[repr(C)]
struct imx_akcodec_fs_mul {
    rmin: c_uint,
    rmax: c_uint,
    wmin: c_uint,
    wmax: c_uint,
}

/*
 * Mapping TDM mode and frame width
 */
#[repr(C)]
struct imx_akcodec_tdm_fs_mul {
    min: c_uint,
    max: c_uint,
    mul: c_uint,
}

/*
 * struct imx_card_plat_data - specific info for codecs
 *
 * @fs_mul: ratio of mclk/fs for normal mode
 * @tdm_fs_mul: ratio of mclk/fs for tdm mode
 * @support_rates: supported sample rate
 * @support_tdm_rates: supported sample rate for tdm mode
 * @support_channels: supported channels
 * @support_tdm_channels: supported channels for tdm mode
 * @num_fs_mul: ARRAY_SIZE of fs_mul
 * @num_tdm_fs_mul: ARRAY_SIZE of tdm_fs_mul
 * @num_rates: ARRAY_SIZE of support_rates
 * @num_tdm_rates: ARRAY_SIZE of support_tdm_rates
 * @num_channels: ARRAY_SIZE of support_channels
 * @num_tdm_channels: ARRAY_SIZE of support_tdm_channels
 * @type: codec type
 */
#[repr(C)]
struct imx_card_plat_data {
    fs_mul: *mut imx_akcodec_fs_mul,
    tdm_fs_mul: *mut imx_akcodec_tdm_fs_mul,
    support_rates: *const u32,
    support_tdm_rates: *const u32,
    support_channels: *const u32,
    support_tdm_channels: *const u32,
    num_fs_mul: c_uint,
    num_tdm_fs_mul: c_uint,
    num_rates: c_uint,
    num_tdm_rates: c_uint,
    num_channels: c_uint,
    num_tdm_channels: c_uint,
    num_codecs: c_uint,
    type_: codec_type,
}

/*
 * struct dai_link_data - specific info for dai link
 *
 * @slots: slot number
 * @slot_width: slot width value
 * @cpu_sysclk_id: sysclk id for cpu dai
 * @one2one_ratio: true if mclk equal to bclk
 */
#[repr(C)]
struct dai_link_data {
    slots: c_uint,
    slot_width: c_uint,
    cpu_sysclk_id: c_uint,
    one2one_ratio: bool_,
}

/*
 * struct imx_card_data - platform device data
 *
 * @plat_data: pointer of imx_card_plat_data
 * @dapm_routes: pointer of dapm_routes
 * @link_data: private data for dai link
 * @card: card instance
 * @num_dapm_routes: number of dapm_routes
 * @asrc_rate: asrc rates
 * @asrc_format: asrc format
 */
#[repr(C)]
struct imx_card_data {
    plat_data: *mut imx_card_plat_data,
    dapm_routes: *mut snd_soc_dapm_route,
    link_data: *mut dai_link_data,
    card: snd_soc_card,
    num_dapm_routes: c_int,
    asrc_rate: u32,
    asrc_format: snd_pcm_format_t,
}

#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_rule { var: c_int, private: *mut c_void }
#[repr(C)] struct snd_pcm_substream { runtime: *mut snd_pcm_runtime }
#[repr(C)] struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] struct snd_soc_pcm_runtime { card: *mut snd_soc_card, id: c_int, dai_link: *mut snd_soc_dai_link }
#[repr(C)] struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct snd_interval { min: c_uint, max: c_uint }
#[repr(C)] struct snd_mask { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, source: *const c_char }
#[repr(C)] struct snd_soc_dai_link_component { of_node: *mut device_node, dai_name: *const c_char }
#[repr(C)] struct of_phandle_args { np: *mut device_node, args: [c_uint; 1] }
#[repr(C)] struct snd_pcm_hw_constraint_list { count: c_uint, list: *const u32 }
#[repr(C)] struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}
#[repr(C)] struct snd_soc_card {
    dev: *mut device,
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_routes: *mut snd_soc_dapm_route,
    num_dapm_routes: c_int,
}
#[repr(C)] struct snd_soc_dai_link {
    cpus: *mut snd_soc_dai_link_component,
    platforms: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    num_platforms: c_uint,
    num_codecs: c_uint,
    name: *const c_char,
    id: c_uint,
    dynamic: c_uint,
    dpcm_merged_chan: c_uint,
    no_pcm: c_uint,
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
    playback_only: bool_,
    capture_only: bool_,
    dai_fmt: c_uint,
    ignore_pmdown_time: c_uint,
    stream_name: *const c_char,
}
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct driver_inner { name: *const c_char, pm: *const c_void, of_match_table: *const of_device_id }
#[repr(C)] struct platform_driver {
    driver: driver_inner,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: c_void;

    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_num_codecs(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_rtd_num_cpus(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_daifmt_clock_provider_flipped(fmt: c_uint) -> c_uint;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_uint, slot_width: c_uint) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_uint, freq: c_ulong, dir: c_int) -> c_int;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn hw_param_interval(p: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(p: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set(mask: *mut snd_mask, val: snd_pcm_format_t);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int,
                           func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
                           private: *mut c_void, dep: c_int, last: c_int) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, prop: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, prop: *const c_char) -> c_int;
    fn of_property_present(np: *mut device_node, prop: *const c_char) -> bool_;
    fn of_get_child_count(np: *mut device_node) -> c_int;
    fn of_get_next_child(parent: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn of_property_read_string(np: *mut device_node, prop: *const c_char, out: *mut *const c_char) -> c_int;
    fn snd_soc_of_get_dlc(np: *mut device_node, args: *mut of_phandle_args, dlc: *mut snd_soc_dai_link_component, idx: c_int) -> c_int;
    fn of_node_name_eq(np: *mut device_node, name: *const c_char) -> bool_;
    fn of_property_read_bool(np: *mut device_node, prop: *const c_char) -> bool_;
    fn snd_soc_of_get_dai_link_codecs(dev: *mut device, np: *mut device_node, link: *mut snd_soc_dai_link) -> c_int;
    fn of_property_read_u32(np: *mut device_node, prop: *const c_char, out: *mut u32) -> c_int;
    fn graph_util_parse_link_direction(np: *mut device_node, playback_only: *mut bool_, capture_only: *mut bool_);
    fn simple_util_parse_daifmt(dev: *mut device, np: *mut device_node, codec: *mut device_node, prefix: *const c_char, fmt: *mut c_uint) -> c_int;
    fn snd_soc_of_parse_tdm_slot(np: *mut device_node, tx_mask: *mut c_uint, rx_mask: *mut c_uint, slots: *mut c_uint, slot_width: *mut c_uint) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_PDM: c_uint = 0x0007;
const SND_SOC_DAIFMT_I2S: c_uint = 0x0001;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const FSL_SAI_CLK_MAST1: c_uint = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 2;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 3;
const SNDRV_PCM_FORMAT_DSD_U8: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_DSD_U16_LE: snd_pcm_format_t = 1;
const SNDRV_PCM_FORMAT_DSD_U16_BE: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_DSD_U32_LE: snd_pcm_format_t = 3;
const SNDRV_PCM_FORMAT_DSD_U32_BE: snd_pcm_format_t = 4;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 5;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 6;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
fn bit(n: c_uint) -> c_uint { 1u32.wrapping_shl(n) }

static mut ak4458_fs_mul: [imx_akcodec_fs_mul; 7] = [
    imx_akcodec_fs_mul { rmin: 8000, rmax: 24000, wmin: 256, wmax: 1024 },
    imx_akcodec_fs_mul { rmin: 32000, rmax: 32000, wmin: 256, wmax: 1024 },
    imx_akcodec_fs_mul { rmin: 44100, rmax: 48000, wmin: 256, wmax: 768 },
    imx_akcodec_fs_mul { rmin: 88200, rmax: 96000, wmin: 256, wmax: 512 },
    imx_akcodec_fs_mul { rmin: 176400, rmax: 192000, wmin: 128, wmax: 256 },
    imx_akcodec_fs_mul { rmin: 352800, rmax: 384000, wmin: 32, wmax: 128 },
    imx_akcodec_fs_mul { rmin: 705600, rmax: 768000, wmin: 16, wmax: 64 },
];
static mut ak4458_tdm_fs_mul: [imx_akcodec_tdm_fs_mul; 3] = [
    imx_akcodec_tdm_fs_mul { min: 128, max: 128, mul: 256 },
    imx_akcodec_tdm_fs_mul { min: 256, max: 256, mul: 512 },
    imx_akcodec_tdm_fs_mul { min: 512, max: 512, mul: 1024 },
];
static mut ak4497_fs_mul: [imx_akcodec_fs_mul; 6] = [
    imx_akcodec_fs_mul { rmin: 8000, rmax: 32000, wmin: 256, wmax: 1024 },
    imx_akcodec_fs_mul { rmin: 44100, rmax: 48000, wmin: 256, wmax: 512 },
    imx_akcodec_fs_mul { rmin: 88200, rmax: 96000, wmin: 256, wmax: 256 },
    imx_akcodec_fs_mul { rmin: 176400, rmax: 192000, wmin: 128, wmax: 128 },
    imx_akcodec_fs_mul { rmin: 352800, rmax: 384000, wmin: 128, wmax: 128 },
    imx_akcodec_fs_mul { rmin: 705600, rmax: 768000, wmin: 64, wmax: 64 },
];
static mut ak5558_fs_mul: [imx_akcodec_fs_mul; 6] = [
    imx_akcodec_fs_mul { rmin: 8000, rmax: 32000, wmin: 512, wmax: 1024 },
    imx_akcodec_fs_mul { rmin: 44100, rmax: 48000, wmin: 512, wmax: 512 },
    imx_akcodec_fs_mul { rmin: 88200, rmax: 96000, wmin: 256, wmax: 256 },
    imx_akcodec_fs_mul { rmin: 176400, rmax: 192000, wmin: 128, wmax: 128 },
    imx_akcodec_fs_mul { rmin: 352800, rmax: 384000, wmin: 64, wmax: 64 },
    imx_akcodec_fs_mul { rmin: 705600, rmax: 768000, wmin: 32, wmax: 32 },
];
static mut ak5558_tdm_fs_mul: [imx_akcodec_tdm_fs_mul; 3] = [
    imx_akcodec_tdm_fs_mul { min: 128, max: 128, mul: 256 },
    imx_akcodec_tdm_fs_mul { min: 256, max: 256, mul: 512 },
    imx_akcodec_tdm_fs_mul { min: 512, max: 512, mul: 1024 },
];
static mut cs42888_fs_mul: [imx_akcodec_fs_mul; 3] = [
    imx_akcodec_fs_mul { rmin: 8000, rmax: 48000, wmin: 256, wmax: 1024 },
    imx_akcodec_fs_mul { rmin: 64000, rmax: 96000, wmin: 128, wmax: 512 },
    imx_akcodec_fs_mul { rmin: 176400, rmax: 192000, wmin: 64, wmax: 256 },
];
static mut cs42888_tdm_fs_mul: [imx_akcodec_tdm_fs_mul; 1] = [
    imx_akcodec_tdm_fs_mul { min: 256, max: 256, mul: 256 },
];
static mut wm8524_fs_mul: [imx_akcodec_fs_mul; 4] = [
    imx_akcodec_fs_mul { rmin: 8000, rmax: 32000, wmin: 256, wmax: 1152 },
    imx_akcodec_fs_mul { rmin: 44100, rmax: 48000, wmin: 256, wmax: 768 },
    imx_akcodec_fs_mul { rmin: 88200, rmax: 96000, wmin: 128, wmax: 384 },
    imx_akcodec_fs_mul { rmin: 176400, rmax: 192000, wmin: 128, wmax: 192 },
];

static akcodec_rates: [u32; 15] = [8000, 11025, 16000, 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000, 352800, 384000, 705600, 768000];
static akcodec_tdm_rates: [u32; 5] = [8000, 16000, 32000, 48000, 96000];
static ak4458_channels: [u32; 9] = [1, 2, 4, 6, 8, 10, 12, 14, 16];
static ak4458_tdm_channels: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 16];
static ak5558_channels: [u32; 5] = [1, 2, 4, 6, 8];
static ak5558_tdm_channels: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static cs42888_channels: [u32; 5] = [1, 2, 4, 6, 8];
static cs42888_tdm_channels: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static wm8524_channels: [u32; 1] = [2];

unsafe extern "C" fn format_is_dsd(params: *mut snd_pcm_hw_params) -> bool_ {
    let format = params_format(params);
    match format {
        SNDRV_PCM_FORMAT_DSD_U8 | SNDRV_PCM_FORMAT_DSD_U16_LE | SNDRV_PCM_FORMAT_DSD_U16_BE |
        SNDRV_PCM_FORMAT_DSD_U32_LE | SNDRV_PCM_FORMAT_DSD_U32_BE => true,
        _ => false,
    }
}

unsafe extern "C" fn format_is_tdm(link_data: *mut dai_link_data) -> bool_ {
    (*link_data).slots > 2
}

unsafe extern "C" fn codec_is_akcodec(type_: c_uint) -> bool_ {
    match type_ {
        CODEC_AK4458 | CODEC_AK4497 | CODEC_AK5558 | CODEC_AK5552 | CODEC_CS42888 | CODEC_WM8524 => true,
        _ => false,
    }
}

unsafe extern "C" fn akcodec_get_mclk_rate(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, slots: c_int, slot_width: c_int) -> c_ulong {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut imx_card_data;
    let plat_data = (*data).plat_data;
    let link_data = (*data).link_data.offset((*rtd).id as isize);
    let mut width = (slots * slot_width) as c_uint;
    let rate = params_rate(params);

    if format_is_tdm(link_data) {
        for i in 0..(*plat_data).num_tdm_fs_mul as isize {
            if width != (*(*plat_data).tdm_fs_mul.offset(i)).min { continue; }
            return (rate * (*(*plat_data).tdm_fs_mul.offset(i)).mul) as c_ulong;
        }
    } else {
        for i in 0..(*plat_data).num_fs_mul as isize {
            let fs = (*plat_data).fs_mul.offset(i);
            if rate >= (*fs).rmin && rate <= (*fs).rmax {
                width = core::cmp::max(width, (*fs).wmin);
                width = core::cmp::min(width, (*fs).wmax);
                width *= if (*link_data).one2one_ratio { 1 } else { 2 };
                return (rate * width) as c_ulong;
            }
        }
    }
    0
}

unsafe extern "C" fn imx_aif_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut imx_card_data;
    let link_data = (*data).link_data.offset((*rtd).id as isize);
    let plat_data = (*data).plat_data;
    let dev = (*card).dev;
    let mut codec_dai: *mut snd_soc_dai = ptr::null_mut();
    let mut fmt = (*(*rtd).dai_link).dai_fmt;
    let mut slots = (*link_data).slots;
    let mut slot_width = (*link_data).slot_width;

    if !format_is_tdm(link_data) {
        if format_is_dsd(params) {
            slots = 1;
            slot_width = params_width(params);
            fmt = ((*(*rtd).dai_link).dai_fmt & !SND_SOC_DAIFMT_FORMAT_MASK) | SND_SOC_DAIFMT_PDM;
        } else {
            slots = 2;
            fmt = ((*(*rtd).dai_link).dai_fmt & !SND_SOC_DAIFMT_FORMAT_MASK) | SND_SOC_DAIFMT_I2S;
        }
    }

    let mut ret = snd_soc_dai_set_fmt(cpu_dai, snd_soc_daifmt_clock_provider_flipped(fmt));
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, cstr!("failed to set cpu dai fmt: %d\n"), ret);
        return ret;
    }
    ret = snd_soc_dai_set_tdm_slot(cpu_dai, bit(slots) - 1, bit(slots) - 1, slots, slot_width);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, cstr!("failed to set cpu dai tdm slot: %d\n"), ret);
        return ret;
    }

    for i in 0..snd_soc_rtd_num_codecs(rtd) {
        codec_dai = snd_soc_rtd_to_codec(rtd, i);
        ret = snd_soc_dai_set_fmt(codec_dai, fmt);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err(dev, cstr!("failed to set codec dai[%d] fmt: %d\n"), i, ret);
            return ret;
        }
        if format_is_tdm(link_data) {
            ret = snd_soc_dai_set_tdm_slot(codec_dai, bit(slots) - 1, bit(slots) - 1, slots, slot_width);
            if ret != 0 && ret != -ENOTSUPP {
                dev_err(dev, cstr!("failed to set codec dai[%d] tdm slot: %d\n"), i, ret);
                return ret;
            }
        }
    }

    let mut mclk_freq: c_ulong = if codec_is_akcodec((*plat_data).type_) {
        akcodec_get_mclk_rate(substream, params, slots as c_int, slot_width as c_int)
    } else {
        (params_rate(params) * slots * slot_width) as c_ulong
    };

    if format_is_dsd(params) {
        if params_rate(params) % 11025 != 0 { mclk_freq = IMX_CARD_MCLK_24P576MHZ; } else { mclk_freq = IMX_CARD_MCLK_22P5792MHZ; }
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, (*link_data).cpu_sysclk_id, mclk_freq, SND_SOC_CLOCK_OUT);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, cstr!("failed to set cpui dai mclk1 rate (%lu): %d\n"), mclk_freq, ret);
        return ret;
    }
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk_freq, SND_SOC_CLOCK_IN);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, cstr!("failed to set codec dai mclk rate (%lu): %d\n"), mclk_freq, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn ak5558_hw_rule_rate(p: *mut snd_pcm_hw_params, r: *mut snd_pcm_hw_rule) -> c_int {
    let link_data = (*r).private as *mut dai_link_data;
    let mut t = snd_interval { min: 8000, max: 8000 };
    let mut fs = (*hw_param_interval(p, SNDRV_PCM_HW_PARAM_SAMPLE_BITS)).min;
    fs *= (*link_data).slots;
    for i in 0..akcodec_rates.len() {
        let mut mclk_freq = (fs * akcodec_rates[i]) as c_ulong;
        mclk_freq *= if (*link_data).one2one_ratio { 1 } else { 2 };
        if mclk_freq > 36864000 { continue; }
        if t.max < akcodec_rates[i] { t.max = akcodec_rates[i]; }
    }
    snd_interval_refine(hw_param_interval(p, (*r).var), &t)
}

unsafe extern "C" fn imx_aif_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut imx_card_data;
    let link_data = (*data).link_data.offset((*rtd).id as isize);
    static mut constraint_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 0, list: ptr::null() };
    static mut constraint_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 0, list: ptr::null() };
    let mut ret = 0;

    if format_is_tdm(link_data) {
        constraint_channels.list = (*(*data).plat_data).support_tdm_channels;
        constraint_channels.count = (*(*data).plat_data).num_tdm_channels;
        constraint_rates.list = (*(*data).plat_data).support_tdm_rates;
        constraint_rates.count = (*(*data).plat_data).num_tdm_rates;
    } else {
        constraint_channels.list = (*(*data).plat_data).support_channels;
        constraint_channels.count = (*(*data).plat_data).num_channels;
        constraint_rates.list = (*(*data).plat_data).support_rates;
        constraint_rates.count = (*(*data).plat_data).num_rates;
    }

    if constraint_channels.count != 0 {
        ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &raw mut constraint_channels);
        if ret != 0 { return ret; }
    }
    if constraint_rates.count != 0 {
        ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &raw mut constraint_rates);
        if ret != 0 { return ret; }
    }
    if (*(*data).plat_data).type_ == CODEC_AK5558 {
        ret = snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, ak5558_hw_rule_rate, link_data as *mut c_void, SNDRV_PCM_HW_PARAM_SAMPLE_BITS, -1);
    }
    ret
}

unsafe extern "C" fn imx_aif_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    for i in 0..snd_soc_rtd_num_cpus(rtd) {
        let cpu_dai = snd_soc_rtd_to_cpu(rtd, i);
        if snd_soc_dai_active(cpu_dai) == 0 { snd_soc_dai_set_sysclk(cpu_dai, 0, 0, SND_SOC_CLOCK_OUT); }
    }
    for i in 0..snd_soc_rtd_num_codecs(rtd) {
        let codec_dai = snd_soc_rtd_to_codec(rtd, i);
        if snd_soc_dai_active(codec_dai) == 0 { snd_soc_dai_set_sysclk(codec_dai, 0, 0, SND_SOC_CLOCK_IN); }
    }
}

static imx_aif_ops: snd_soc_ops = snd_soc_ops { hw_params: Some(imx_aif_hw_params), startup: Some(imx_aif_startup), shutdown: Some(imx_aif_shutdown) };
static imx_aif_ops_be: snd_soc_ops = snd_soc_ops { hw_params: Some(imx_aif_hw_params), startup: None, shutdown: None };

unsafe extern "C" fn be_hw_params_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int {
    let card = (*rtd).card;
    let data = snd_soc_card_get_drvdata(card) as *mut imx_card_data;
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    (*rate).max = (*data).asrc_rate;
    (*rate).min = (*data).asrc_rate;
    let mask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    snd_mask_none(mask);
    snd_mask_set(mask, (*data).asrc_format);
    0
}

unsafe extern "C" fn imx_card_parse_of(data: *mut imx_card_data) -> c_int {
    let plat_data = (*data).plat_data;
    let card = &mut (*data).card as *mut snd_soc_card;
    let mut dlc: *mut snd_soc_dai_link_component;
    let mut platform: *mut device_node = ptr::null_mut();
    let mut codec: *mut device_node = ptr::null_mut();
    let mut cpu: *mut device_node = ptr::null_mut();
    let dev = (*card).dev;
    let mut ret = snd_soc_of_parse_card_name(card, cstr!("model"));
    let mut args = of_phandle_args { np: ptr::null_mut(), args: [0] };
    let mut asrc_fmt: u32 = 0;
    let mut width: u32 = 0;
    if ret != 0 {
        dev_err(dev, cstr!("Error parsing card name: %d\n"), ret);
        return ret;
    }
    if of_property_present((*dev).of_node, cstr!("audio-routing")) {
        ret = snd_soc_of_parse_audio_routing(card, cstr!("audio-routing"));
        if ret != 0 { return ret; }
    }

    let num_links = of_get_child_count((*dev).of_node);
    (*card).dai_link = devm_kcalloc(dev, num_links as usize, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    if (*card).dai_link.is_null() { return -ENOMEM; }
    (*data).link_data = devm_kcalloc(dev, num_links as usize, core::mem::size_of::<dai_link_data>(), GFP_KERNEL) as *mut dai_link_data;
    if (*data).link_data.is_null() { return -ENOMEM; }
    (*card).num_links = num_links;

    let mut link = (*card).dai_link;
    let mut link_data = (*data).link_data;
    let mut np: *mut device_node = ptr::null_mut();
    loop {
        np = of_get_next_child((*dev).of_node, np);
        if np.is_null() { break; }
        dlc = devm_kzalloc(dev, 2 * core::mem::size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
        if dlc.is_null() { return -ENOMEM; }
        (*link).cpus = dlc.offset(0);
        (*link).platforms = dlc.offset(1);
        (*link).num_cpus = 1;
        (*link).num_platforms = 1;
        ret = of_property_read_string(np, cstr!("link-name"), &mut (*link).name);
        if ret != 0 { return dev_err_probe((*card).dev, ret, cstr!("error getting codec dai_link name\n")); }
        cpu = of_get_child_by_name(np, cstr!("cpu"));
        if cpu.is_null() {
            dev_err(dev, cstr!("%s: Can't find cpu DT node\n"), (*link).name);
            ret = -EINVAL;
            break;
        }
        ret = snd_soc_of_get_dlc(cpu, &mut args, (*link).cpus, 0);
        if ret != 0 {
            dev_err_probe((*card).dev, ret, cstr!("%s: error getting cpu dai info\n"), (*link).name);
            break;
        }
        if of_node_name_eq(args.np, cstr!("sai")) {
            (*link_data).cpu_sysclk_id = FSL_SAI_CLK_MAST1;
            if of_property_read_bool(np, cstr!("fsl,mclk-equal-bclk")) {
                (*link_data).one2one_ratio = true;
            } else {
                for i in 0..ak4497_fs_mul.len() {
                    if ak4497_fs_mul[i].rmin == 705600 && ak4497_fs_mul[i].rmax == 768000 {
                        ak4497_fs_mul[i].wmin = 32;
                        ak4497_fs_mul[i].wmax = 32;
                    }
                }
            }
        }
        (*(*link).platforms).of_node = (*(*link).cpus).of_node;
        (*link).id = args.args[0];
        codec = of_get_child_by_name(np, cstr!("codec"));
        if !codec.is_null() {
            ret = snd_soc_of_get_dai_link_codecs(dev, codec, link);
            if ret < 0 {
                dev_err_probe(dev, ret, cstr!("%s: codec dai not found\n"), (*link).name);
                break;
            }
            (*plat_data).num_codecs = (*link).num_codecs;
            if strcmp((*(*link).codecs).dai_name, cstr!("ak4458-aif")) == 0 { (*plat_data).type_ = CODEC_AK4458; }
            else if strcmp((*(*link).codecs).dai_name, cstr!("ak4497-aif")) == 0 { (*plat_data).type_ = CODEC_AK4497; }
            else if strcmp((*(*link).codecs).dai_name, cstr!("ak5558-aif")) == 0 { (*plat_data).type_ = CODEC_AK5558; }
            else if strcmp((*(*link).codecs).dai_name, cstr!("ak5552-aif")) == 0 { (*plat_data).type_ = CODEC_AK5552; }
            else if strcmp((*(*link).codecs).dai_name, cstr!("cs42888")) == 0 { (*plat_data).type_ = CODEC_CS42888; }
            else if strcmp((*(*link).codecs).dai_name, cstr!("wm8524-hifi")) == 0 { (*plat_data).type_ = CODEC_WM8524; }
        } else {
            (*link).codecs = &raw mut snd_soc_dummy_dlc;
            (*link).num_codecs = 1;
        }
        if strncmp((*link).name, cstr!("HiFi-ASRC-FE"), 12) == 0 {
            (*link).dynamic = 1;
            (*link).dpcm_merged_chan = 1;
            ret = of_property_read_u32(args.np, cstr!("fsl,asrc-rate"), &mut (*data).asrc_rate);
            if ret != 0 {
                dev_err(dev, cstr!("failed to get output rate\n"));
                ret = -EINVAL;
                break;
            }
            ret = of_property_read_u32(args.np, cstr!("fsl,asrc-format"), &mut asrc_fmt);
            (*data).asrc_format = asrc_fmt as snd_pcm_format_t;
            if ret != 0 {
                ret = of_property_read_u32(args.np, cstr!("fsl,asrc-width"), &mut width);
                if ret != 0 {
                    dev_err(dev, cstr!("failed to decide output format\n"));
                    break;
                }
                (*data).asrc_format = if width == 24 { SNDRV_PCM_FORMAT_S24_LE } else { SNDRV_PCM_FORMAT_S16_LE };
            }
        } else if strncmp((*link).name, cstr!("HiFi-ASRC-BE"), 12) == 0 {
            (*link).no_pcm = 1;
            (*link).platforms = ptr::null_mut();
            (*link).be_hw_params_fixup = Some(be_hw_params_fixup);
            (*link).ops = &imx_aif_ops_be;
        } else {
            (*link).ops = &imx_aif_ops;
        }
        let mut playback_only = false;
        let mut capture_only = false;
        graph_util_parse_link_direction(np, &mut playback_only, &mut capture_only);
        (*link).playback_only = playback_only;
        (*link).capture_only = capture_only;
        ret = simple_util_parse_daifmt(dev, np, codec, ptr::null(), &mut (*link).dai_fmt);
        if ret != 0 {
            (*link).dai_fmt = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_I2S;
        }
        snd_soc_of_parse_tdm_slot(np, ptr::null_mut(), ptr::null_mut(), &mut (*link_data).slots, &mut (*link_data).slot_width);
        if (*link_data).slots == 0 { (*link_data).slots = 2; }
        if (*link_data).slot_width == 0 { (*link_data).slot_width = 32; }
        (*link).ignore_pmdown_time = 1;
        (*link).stream_name = (*link).name;
        link = link.offset(1);
        link_data = link_data.offset(1);
        of_node_put(cpu);
        of_node_put(codec);
        of_node_put(platform);
        cpu = ptr::null_mut();
        codec = ptr::null_mut();
        platform = ptr::null_mut();
    }
    if ret != 0 {
        of_node_put(cpu);
        of_node_put(codec);
        of_node_put(platform);
        return ret;
    }
    0
}

unsafe extern "C" fn imx_card_probe(pdev: *mut platform_device) -> c_int {
    let mut link_be: *mut snd_soc_dai_link = ptr::null_mut();
    let data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<imx_card_data>(), GFP_KERNEL) as *mut imx_card_data;
    if data.is_null() { return -ENOMEM; }
    let plat_data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<imx_card_plat_data>(), GFP_KERNEL) as *mut imx_card_plat_data;
    if plat_data.is_null() { return -ENOMEM; }
    (*data).plat_data = plat_data;
    (*data).card.dev = &mut (*pdev).dev;
    (*data).card.owner = THIS_MODULE;
    dev_set_drvdata(&mut (*pdev).dev, &mut (*data).card as *mut _ as *mut c_void);
    snd_soc_card_set_drvdata(&mut (*data).card, data as *mut c_void);
    let mut ret = imx_card_parse_of(data);
    if ret != 0 { return ret; }

    (*data).num_dapm_routes = (*plat_data).num_codecs as c_int + 1;
    (*data).dapm_routes = devm_kcalloc(&mut (*pdev).dev, (*data).num_dapm_routes as usize, core::mem::size_of::<snd_soc_dapm_route>(), GFP_KERNEL) as *mut snd_soc_dapm_route;
    if (*data).dapm_routes.is_null() { return -ENOMEM; }
    let mut i: c_int = 0;
    match (*plat_data).type_ {
        CODEC_AK4458 | CODEC_AK4497 => {
            if (*plat_data).num_codecs == 1 {
                (*(*data).dapm_routes.offset(0)).sink = cstr!("Playback");
                (*(*data).dapm_routes.offset(0)).source = cstr!("CPU-Playback");
                i = 1;
            } else {
                for n in 0..(*plat_data).num_codecs as isize {
                    (*(*data).dapm_routes.offset(n)).sink = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, cstr!("%d %s"), n as c_int + 1, cstr!("Playback"));
                    if (*(*data).dapm_routes.offset(n)).sink.is_null() { return -ENOMEM; }
                    (*(*data).dapm_routes.offset(n)).source = cstr!("CPU-Playback");
                }
                i = (*plat_data).num_codecs as c_int;
            }
            (*(*data).dapm_routes.offset(i as isize)).sink = cstr!("CPU-Playback");
            (*(*data).dapm_routes.offset(i as isize)).source = cstr!("ASRC-Playback");
        }
        CODEC_AK5558 | CODEC_AK5552 => {
            if (*plat_data).num_codecs == 1 {
                (*(*data).dapm_routes.offset(0)).sink = cstr!("CPU-Capture");
                (*(*data).dapm_routes.offset(0)).source = cstr!("Capture");
                i = 1;
            } else {
                for n in 0..(*plat_data).num_codecs as isize {
                    (*(*data).dapm_routes.offset(n)).source = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, cstr!("%d %s"), n as c_int + 1, cstr!("Capture"));
                    if (*(*data).dapm_routes.offset(n)).source.is_null() { return -ENOMEM; }
                    (*(*data).dapm_routes.offset(n)).sink = cstr!("CPU-Capture");
                }
                i = (*plat_data).num_codecs as c_int;
            }
            (*(*data).dapm_routes.offset(i as isize)).sink = cstr!("ASRC-Capture");
            (*(*data).dapm_routes.offset(i as isize)).source = cstr!("CPU-Capture");
        }
        CODEC_CS42888 => {
            (*(*data).dapm_routes.offset(0)).sink = cstr!("Playback");
            (*(*data).dapm_routes.offset(0)).source = cstr!("CPU-Playback");
            (*(*data).dapm_routes.offset(1)).sink = cstr!("CPU-Capture");
            (*(*data).dapm_routes.offset(1)).source = cstr!("Capture");
        }
        CODEC_WM8524 => {
            (*(*data).dapm_routes.offset(0)).sink = cstr!("Playback");
            (*(*data).dapm_routes.offset(0)).source = cstr!("CPU-Playback");
        }
        _ => {}
    }

    if codec_is_akcodec((*plat_data).type_) {
        (*plat_data).support_rates = akcodec_rates.as_ptr();
        (*plat_data).num_rates = akcodec_rates.len() as c_uint;
        (*plat_data).support_tdm_rates = akcodec_tdm_rates.as_ptr();
        (*plat_data).num_tdm_rates = akcodec_tdm_rates.len() as c_uint;
        match (*plat_data).type_ {
            CODEC_AK4458 => {
                (*plat_data).fs_mul = ak4458_fs_mul.as_mut_ptr();
                (*plat_data).num_fs_mul = ak4458_fs_mul.len() as c_uint;
                (*plat_data).tdm_fs_mul = ak4458_tdm_fs_mul.as_mut_ptr();
                (*plat_data).num_tdm_fs_mul = ak4458_tdm_fs_mul.len() as c_uint;
                (*plat_data).support_channels = ak4458_channels.as_ptr();
                (*plat_data).num_channels = ak4458_channels.len() as c_uint;
                (*plat_data).support_tdm_channels = ak4458_tdm_channels.as_ptr();
                (*plat_data).num_tdm_channels = ak4458_tdm_channels.len() as c_uint;
            }
            CODEC_AK4497 => {
                (*plat_data).fs_mul = ak4497_fs_mul.as_mut_ptr();
                (*plat_data).num_fs_mul = ak4497_fs_mul.len() as c_uint;
                (*plat_data).support_channels = ak4458_channels.as_ptr();
                (*plat_data).num_channels = ak4458_channels.len() as c_uint;
            }
            CODEC_AK5558 | CODEC_AK5552 => {
                (*plat_data).fs_mul = ak5558_fs_mul.as_mut_ptr();
                (*plat_data).num_fs_mul = ak5558_fs_mul.len() as c_uint;
                (*plat_data).tdm_fs_mul = ak5558_tdm_fs_mul.as_mut_ptr();
                (*plat_data).num_tdm_fs_mul = ak5558_tdm_fs_mul.len() as c_uint;
                (*plat_data).support_channels = ak5558_channels.as_ptr();
                (*plat_data).num_channels = ak5558_channels.len() as c_uint;
                (*plat_data).support_tdm_channels = ak5558_tdm_channels.as_ptr();
                (*plat_data).num_tdm_channels = ak5558_tdm_channels.len() as c_uint;
            }
            CODEC_CS42888 => {
                (*plat_data).fs_mul = cs42888_fs_mul.as_mut_ptr();
                (*plat_data).num_fs_mul = cs42888_fs_mul.len() as c_uint;
                (*plat_data).tdm_fs_mul = cs42888_tdm_fs_mul.as_mut_ptr();
                (*plat_data).num_tdm_fs_mul = cs42888_tdm_fs_mul.len() as c_uint;
                (*plat_data).support_channels = cs42888_channels.as_ptr();
                (*plat_data).num_channels = cs42888_channels.len() as c_uint;
                (*plat_data).support_tdm_channels = cs42888_tdm_channels.as_ptr();
                (*plat_data).num_tdm_channels = cs42888_tdm_channels.len() as c_uint;
            }
            CODEC_WM8524 => {
                (*plat_data).fs_mul = wm8524_fs_mul.as_mut_ptr();
                (*plat_data).num_fs_mul = wm8524_fs_mul.len() as c_uint;
                (*plat_data).support_channels = wm8524_channels.as_ptr();
                (*plat_data).num_channels = wm8524_channels.len() as c_uint;
            }
            _ => {}
        }
    }

    if (*data).card.num_links == 3 {
        (*data).card.dapm_routes = (*data).dapm_routes;
        (*data).card.num_dapm_routes = (*data).num_dapm_routes;
        for idx in 0..(*data).card.num_links {
            let link = (*data).card.dai_link.offset(idx as isize);
            if (*link).no_pcm == 1 { link_be = link; }
        }
        for idx in 0..(*data).card.num_links {
            let link = (*data).card.dai_link.offset(idx as isize);
            if (*link).dynamic == 1 && !link_be.is_null() {
                (*link).playback_only = (*link_be).playback_only;
                (*link).capture_only = (*link_be).capture_only;
            }
        }
    }
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut (*data).card);
    if ret != 0 { return dev_err_probe(&mut (*pdev).dev, ret, cstr!("snd_soc_register_card failed\n")); }
    0
}

static imx_card_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("fsl,imx-audio-card") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, imx_card_dt_ids);

static mut imx_card_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: cstr!("imx-card"),
        pm: unsafe { &snd_soc_pm_ops as *const _ as *const c_void },
        of_match_table: imx_card_dt_ids.as_ptr(),
    },
    probe: Some(imx_card_probe),
};
// module_platform_driver(imx_card_driver);

// MODULE_DESCRIPTION("Freescale i.MX ASoC Machine Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:imx-card");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
