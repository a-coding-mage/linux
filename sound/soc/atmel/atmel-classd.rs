// SPDX-License-Identifier: GPL-2.0-or-later
/* Atmel ALSA SoC Audio Class D Amplifier (CLASSD) driver
 *
 * Copyright (C) 2015 Atmel
 *
 * Author: Songjun Wu <songjun.wu@atmel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = c_uint;
type dma_addr_t = c_ulong;

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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_slave_config {
    pub direction: c_int,
    pub src_addr_width: c_int,
    pub dst_addr_width: c_int,
    pub src_addr: dma_addr_t,
    pub dst_addr: dma_addr_t,
    pub src_maxburst: c_uint,
    pub dst_maxburst: c_uint,
    pub device_fc: bool_,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub name: *const c_char,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub num_platforms: c_uint,
    pub name: *const c_char,
    pub stream_name: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut dma_slave_config,
        ) -> c_int,
    >,
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: usize,
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct atmel_classd_pdata {
    non_overlap_enable: bool_,
    non_overlap_time: c_int,
    pwm_type: c_int,
    card_name: *const c_char,
}

#[repr(C)]
struct atmel_classd {
    phy_base: dma_addr_t,
    regmap: *mut regmap,
    pclk: *mut clk,
    gclk: *mut clk,
    dev: *mut device,
    irq: c_int,
    pdata: *const atmel_classd_pdata,
}

unsafe extern "C" {
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: dev_pm_ops;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_int) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *const c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn str_enabled_disabled(v: bool_) -> *const c_char;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_int = 1;

const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 2;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 4;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 5;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 6;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 7;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const DMA_SLAVE_BUSWIDTH_2_BYTES: c_int = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_int = 4;
const DMA_MEM_TO_DEV: c_int = 1;

const CLASSD_THR: c_uint = 0x00;
const CLASSD_MR: c_uint = 0x04;
const CLASSD_INTPMR: c_uint = 0x08;
const CLASSD_INTPMR_MONO_MODE_SHIFT: c_uint = 0;
const CLASSD_INTPMR_EQCFG_SHIFT: c_uint = 0;
const CLASSD_INTPMR_ATTL_SHIFT: c_uint = 0;
const CLASSD_INTPMR_ATTR_SHIFT: c_uint = 0;
const CLASSD_INTPMR_DEEMP_SHIFT: c_uint = 0;
const CLASSD_INTPMR_MONO_SHIFT: c_uint = 0;
const CLASSD_INTPMR_SWAP_SHIFT: c_uint = 0;
const CLASSD_MR_PWMTYP_MASK: u32 = 0;
const CLASSD_MR_NON_OVERLAP_MASK: u32 = 0;
const CLASSD_MR_NOVR_VAL_MASK: u32 = 0;
const CLASSD_MR_LMUTE_MASK: u32 = 0;
const CLASSD_MR_RMUTE_MASK: u32 = 0;
const CLASSD_MR_LEN_MASK: u32 = 0;
const CLASSD_MR_REN_MASK: u32 = 0;
const CLASSD_INTPMR_DSP_CLK_FREQ_MASK: u32 = 0;
const CLASSD_INTPMR_FRAME_MASK: u32 = 0;
const CLASSD_MR_PWMTYP_DIFF: c_int = 1;
const CLASSD_MR_PWMTYP_SINGLE: c_int = 0;
const CLASSD_MR_NON_OVERLAP_EN: c_int = 1;
const CLASSD_MR_NOVR_VAL_5NS: c_int = 0;
const CLASSD_MR_NOVR_VAL_10NS: c_int = 1;
const CLASSD_MR_NOVR_VAL_15NS: c_int = 2;
const CLASSD_MR_NOVR_VAL_20NS: c_int = 3;
const CLASSD_MR_LEN_DIS: c_int = 0;
const CLASSD_MR_REN_DIS: c_int = 0;
const CLASSD_INTPMR_FRAME_8K: c_int = 0;
const CLASSD_INTPMR_FRAME_16K: c_int = 1;
const CLASSD_INTPMR_FRAME_32K: c_int = 2;
const CLASSD_INTPMR_FRAME_48K: c_int = 3;
const CLASSD_INTPMR_FRAME_96K: c_int = 4;
const CLASSD_INTPMR_FRAME_22K: c_int = 5;
const CLASSD_INTPMR_FRAME_44K: c_int = 6;
const CLASSD_INTPMR_FRAME_88K: c_int = 7;
const CLASSD_INTPMR_DSP_CLK_FREQ_12M288: c_int = 0;
const CLASSD_INTPMR_DSP_CLK_FREQ_11M2896: c_int = 1;
const CLASSD_INTPMR_EQCFG_T_CUT_12: c_uint = 0;
const CLASSD_INTPMR_EQCFG_T_CUT_6: c_uint = 1;
const CLASSD_INTPMR_EQCFG_M_CUT_8: c_uint = 2;
const CLASSD_INTPMR_EQCFG_M_CUT_3: c_uint = 3;
const CLASSD_INTPMR_EQCFG_B_CUT_12: c_uint = 4;
const CLASSD_INTPMR_EQCFG_B_CUT_6: c_uint = 5;
const CLASSD_INTPMR_EQCFG_FLAT: c_uint = 6;
const CLASSD_INTPMR_EQCFG_B_BOOST_6: c_uint = 7;
const CLASSD_INTPMR_EQCFG_B_BOOST_12: c_uint = 8;
const CLASSD_INTPMR_EQCFG_M_BOOST_3: c_uint = 9;
const CLASSD_INTPMR_EQCFG_M_BOOST_8: c_uint = 10;
const CLASSD_INTPMR_EQCFG_T_BOOST_6: c_uint = 11;
const CLASSD_INTPMR_EQCFG_T_BOOST_12: c_uint = 12;

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const fn FIELD_PREP(mask: u32, val: c_int) -> u32 {
    ((val as u32) << mask.trailing_zeros()) & mask
}

fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

// CONFIG_OF: device-tree match data and initialization.
static atmel_classd_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"atmel,sama5d2-classd\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel */
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, atmel_classd_of_match);

unsafe extern "C" fn atmel_classd_dt_init(dev: *mut device) -> *mut atmel_classd_pdata {
    let np: *mut device_node = (*dev).of_node;
    let mut pdata: *mut atmel_classd_pdata;
    let mut pwm_type_s: *const c_char = ptr::null();
    let mut ret: c_int;

    if np.is_null() {
        dev_err(dev, b"device node not found\n\0".as_ptr() as *const c_char);
        return ERR_PTR(-EINVAL);
    }

    pdata = devm_kzalloc(dev, size_of::<atmel_classd_pdata>(), GFP_KERNEL) as *mut atmel_classd_pdata;
    if pdata.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    ret = of_property_read_string(np, b"atmel,pwm-type\0".as_ptr() as *const c_char, &mut pwm_type_s);
    if ret == 0 && strcmp(pwm_type_s, b"diff\0".as_ptr() as *const c_char) == 0 {
        (*pdata).pwm_type = CLASSD_MR_PWMTYP_DIFF;
    } else {
        (*pdata).pwm_type = CLASSD_MR_PWMTYP_SINGLE;
    }

    ret = of_property_read_u32(
        np,
        b"atmel,non-overlap-time\0".as_ptr() as *const c_char,
        &mut (*pdata).non_overlap_time,
    );
    if ret != 0 {
        (*pdata).non_overlap_enable = false;
    } else {
        (*pdata).non_overlap_enable = true;
    }

    ret = of_property_read_string(np, b"atmel,model\0".as_ptr() as *const c_char, &mut (*pdata).card_name);
    if ret != 0 {
        (*pdata).card_name = b"CLASSD\0".as_ptr() as *const c_char;
    }

    pdata
}

const ATMEL_CLASSD_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

static atmel_classd_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: ATMEL_CLASSD_RATES,
    rate_min: 8000,
    rate_max: 96000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 256,
    period_bytes_max: 32 * 1024,
    periods_min: 2,
    periods_max: 256,
};

const ATMEL_CLASSD_PREALLOC_BUF_SIZE: usize = 64 * 1024;

/* cpu dai component */
unsafe extern "C" fn atmel_classd_cpu_dai_startup(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_classd;
    let mut err: c_int;

    regmap_write((*dd).regmap, CLASSD_THR, 0x0);

    err = clk_prepare_enable((*dd).pclk);
    if err != 0 {
        return err;
    }
    err = clk_prepare_enable((*dd).gclk);
    if err != 0 {
        clk_disable_unprepare((*dd).pclk);
        return err;
    }
    0
}

/* platform */
unsafe extern "C" fn atmel_classd_platform_configure_dma(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    slave_config: *mut dma_slave_config,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_classd;

    if params_physical_width(params) != 16 {
        dev_err((*dd).dev, b"only supports 16-bit audio data\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if params_channels(params) == 1 {
        (*slave_config).dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    } else {
        (*slave_config).dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    }

    (*slave_config).direction = DMA_MEM_TO_DEV;
    (*slave_config).dst_addr = (*dd).phy_base.wrapping_add(CLASSD_THR as dma_addr_t);
    (*slave_config).dst_maxburst = 1;
    (*slave_config).src_maxburst = 1;
    (*slave_config).device_fc = false;

    0
}

static atmel_classd_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(atmel_classd_platform_configure_dma),
    pcm_hardware: &atmel_classd_hw,
    prealloc_buffer_size: ATMEL_CLASSD_PREALLOC_BUF_SIZE,
};

/* codec */
static mono_mode_text: [*const c_char; 4] = [
    b"mix\0".as_ptr() as *const c_char,
    b"sat\0".as_ptr() as *const c_char,
    b"left\0".as_ptr() as *const c_char,
    b"right\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(classd_mono_mode_enum, CLASSD_INTPMR,
//     CLASSD_INTPMR_MONO_MODE_SHIFT, mono_mode_text);
static classd_mono_mode_enum: soc_enum = soc_enum { _private: [] };

static eqcfg_text: [*const c_char; 13] = [
    b"Treble-12dB\0".as_ptr() as *const c_char,
    b"Treble-6dB\0".as_ptr() as *const c_char,
    b"Medium-8dB\0".as_ptr() as *const c_char,
    b"Medium-3dB\0".as_ptr() as *const c_char,
    b"Bass-12dB\0".as_ptr() as *const c_char,
    b"Bass-6dB\0".as_ptr() as *const c_char,
    b"0 dB\0".as_ptr() as *const c_char,
    b"Bass+6dB\0".as_ptr() as *const c_char,
    b"Bass+12dB\0".as_ptr() as *const c_char,
    b"Medium+3dB\0".as_ptr() as *const c_char,
    b"Medium+8dB\0".as_ptr() as *const c_char,
    b"Treble+6dB\0".as_ptr() as *const c_char,
    b"Treble+12dB\0".as_ptr() as *const c_char,
];

static eqcfg_value: [c_uint; 13] = [
    CLASSD_INTPMR_EQCFG_T_CUT_12,
    CLASSD_INTPMR_EQCFG_T_CUT_6,
    CLASSD_INTPMR_EQCFG_M_CUT_8,
    CLASSD_INTPMR_EQCFG_M_CUT_3,
    CLASSD_INTPMR_EQCFG_B_CUT_12,
    CLASSD_INTPMR_EQCFG_B_CUT_6,
    CLASSD_INTPMR_EQCFG_FLAT,
    CLASSD_INTPMR_EQCFG_B_BOOST_6,
    CLASSD_INTPMR_EQCFG_B_BOOST_12,
    CLASSD_INTPMR_EQCFG_M_BOOST_3,
    CLASSD_INTPMR_EQCFG_M_BOOST_8,
    CLASSD_INTPMR_EQCFG_T_BOOST_6,
    CLASSD_INTPMR_EQCFG_T_BOOST_12,
];

// static SOC_VALUE_ENUM_SINGLE_DECL(classd_eqcfg_enum, CLASSD_INTPMR,
//     CLASSD_INTPMR_EQCFG_SHIFT, 0xf, eqcfg_text, eqcfg_value);
static classd_eqcfg_enum: soc_enum = soc_enum { _private: [] };

// static const DECLARE_TLV_DB_SCALE(classd_digital_tlv, -7800, 100, 1);
static classd_digital_tlv: [c_uint; 4] = [0, (-7800_i32) as c_uint, 100, 1];

// The following controls preserve the C macro invocations as dependency intent:
// SOC_DOUBLE_TLV("Playback Volume", CLASSD_INTPMR, CLASSD_INTPMR_ATTL_SHIFT,
//     CLASSD_INTPMR_ATTR_SHIFT, 78, 1, classd_digital_tlv)
// SOC_SINGLE("Deemphasis Switch", CLASSD_INTPMR, CLASSD_INTPMR_DEEMP_SHIFT, 1, 0)
// SOC_SINGLE("Mono Switch", CLASSD_INTPMR, CLASSD_INTPMR_MONO_SHIFT, 1, 0)
// SOC_SINGLE("Swap Switch", CLASSD_INTPMR, CLASSD_INTPMR_SWAP_SHIFT, 1, 0)
// SOC_ENUM("Mono Mode", classd_mono_mode_enum)
// SOC_ENUM("EQ", classd_eqcfg_enum)
static atmel_classd_snd_controls: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static pwm_type: [*const c_char; 2] = [
    b"Single ended\0".as_ptr() as *const c_char,
    b"Differential\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn atmel_classd_component_probe(component: *mut snd_soc_component) -> c_int {
    let card: *mut snd_soc_card = snd_soc_component_get_drvdata(component) as *mut snd_soc_card;
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata(card) as *mut atmel_classd;
    let pdata: *const atmel_classd_pdata = (*dd).pdata;
    let mut mask: u32;
    let mut val: u32;

    mask = CLASSD_MR_PWMTYP_MASK;
    val = FIELD_PREP(CLASSD_MR_PWMTYP_MASK, (*pdata).pwm_type);

    mask |= CLASSD_MR_NON_OVERLAP_MASK;
    if (*pdata).non_overlap_enable {
        val |= FIELD_PREP(CLASSD_MR_NON_OVERLAP_MASK, CLASSD_MR_NON_OVERLAP_EN);

        mask |= CLASSD_MR_NOVR_VAL_MASK;
        match (*pdata).non_overlap_time {
            5 => {
                val |= FIELD_PREP(CLASSD_MR_NOVR_VAL_MASK, CLASSD_MR_NOVR_VAL_5NS);
            }
            10 => {
                val |= FIELD_PREP(CLASSD_MR_NOVR_VAL_MASK, CLASSD_MR_NOVR_VAL_10NS);
            }
            15 => {
                val |= FIELD_PREP(CLASSD_MR_NOVR_VAL_MASK, CLASSD_MR_NOVR_VAL_15NS);
            }
            20 => {
                val |= FIELD_PREP(CLASSD_MR_NOVR_VAL_MASK, CLASSD_MR_NOVR_VAL_20NS);
            }
            _ => {
                val |= FIELD_PREP(CLASSD_MR_NOVR_VAL_MASK, CLASSD_MR_NOVR_VAL_10NS);
                dev_warn(
                    (*component).dev,
                    b"non-overlapping value %d is invalid, the default value 10 is specified\n\0".as_ptr()
                        as *const c_char,
                    (*pdata).non_overlap_time,
                );
            }
        }
    }

    snd_soc_component_update_bits(component, CLASSD_MR, mask, val);

    dev_info(
        (*component).dev,
        b"PWM modulation type is %s, non-overlapping is %s\n\0".as_ptr() as *const c_char,
        pwm_type[(*pdata).pwm_type as usize],
        str_enabled_disabled((*pdata).non_overlap_enable),
    );

    0
}

unsafe extern "C" fn atmel_classd_component_resume(component: *mut snd_soc_component) -> c_int {
    let card: *mut snd_soc_card = snd_soc_component_get_drvdata(component) as *mut snd_soc_card;
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata(card) as *mut atmel_classd;

    regcache_sync((*dd).regmap)
}

unsafe extern "C" fn atmel_classd_cpu_dai_mute_stream(
    cpu_dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*cpu_dai).component;
    let mask: u32;
    let val: u32;

    mask = CLASSD_MR_LMUTE_MASK | CLASSD_MR_RMUTE_MASK;

    if mute != 0 {
        val = mask;
    } else {
        val = 0;
    }

    snd_soc_component_update_bits(component, CLASSD_MR, mask, val);

    0
}

const CLASSD_GCLK_RATE_11M2896_MPY_8: c_ulong = 112896 * 100 * 8;
const CLASSD_GCLK_RATE_12M288_MPY_8: c_ulong = 12288 * 1000 * 8;

#[repr(C)]
struct sample_rate_entry {
    rate: c_int,
    sample_rate: c_int,
    dsp_clk: c_int,
    gclk_rate: c_ulong,
}

static sample_rates: [sample_rate_entry; 8] = [
    sample_rate_entry {
        rate: 8000,
        sample_rate: CLASSD_INTPMR_FRAME_8K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_12M288,
        gclk_rate: CLASSD_GCLK_RATE_12M288_MPY_8,
    },
    sample_rate_entry {
        rate: 16000,
        sample_rate: CLASSD_INTPMR_FRAME_16K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_12M288,
        gclk_rate: CLASSD_GCLK_RATE_12M288_MPY_8,
    },
    sample_rate_entry {
        rate: 32000,
        sample_rate: CLASSD_INTPMR_FRAME_32K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_12M288,
        gclk_rate: CLASSD_GCLK_RATE_12M288_MPY_8,
    },
    sample_rate_entry {
        rate: 48000,
        sample_rate: CLASSD_INTPMR_FRAME_48K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_12M288,
        gclk_rate: CLASSD_GCLK_RATE_12M288_MPY_8,
    },
    sample_rate_entry {
        rate: 96000,
        sample_rate: CLASSD_INTPMR_FRAME_96K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_12M288,
        gclk_rate: CLASSD_GCLK_RATE_12M288_MPY_8,
    },
    sample_rate_entry {
        rate: 22050,
        sample_rate: CLASSD_INTPMR_FRAME_22K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_11M2896,
        gclk_rate: CLASSD_GCLK_RATE_11M2896_MPY_8,
    },
    sample_rate_entry {
        rate: 44100,
        sample_rate: CLASSD_INTPMR_FRAME_44K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_11M2896,
        gclk_rate: CLASSD_GCLK_RATE_11M2896_MPY_8,
    },
    sample_rate_entry {
        rate: 88200,
        sample_rate: CLASSD_INTPMR_FRAME_88K,
        dsp_clk: CLASSD_INTPMR_DSP_CLK_FREQ_11M2896,
        gclk_rate: CLASSD_GCLK_RATE_11M2896_MPY_8,
    },
];

unsafe extern "C" fn atmel_classd_cpu_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_classd;
    let component: *mut snd_soc_component = (*cpu_dai).component;
    let fs: c_int;
    let mut i: usize;
    let mut best: usize;
    let mut best_val: c_int;
    let mut cur_val: c_int;
    let mut ret: c_int;
    let mask: u32;
    let val: u32;

    fs = params_rate(params);

    best = 0;
    best_val = (fs - sample_rates[0].rate).abs();
    i = 1;
    while i < ARRAY_SIZE(&sample_rates) {
        /* Closest match */
        cur_val = (fs - sample_rates[i].rate).abs();
        if cur_val < best_val {
            best = i;
            best_val = cur_val;
        }
        i += 1;
    }

    dev_dbg(
        (*component).dev,
        b"Selected SAMPLE_RATE of %dHz, GCLK_RATE of %ldHz\n\0".as_ptr() as *const c_char,
        sample_rates[best].rate,
        sample_rates[best].gclk_rate,
    );

    clk_disable_unprepare((*dd).gclk);

    ret = clk_set_rate((*dd).gclk, sample_rates[best].gclk_rate);
    if ret != 0 {
        return ret;
    }

    mask = CLASSD_INTPMR_DSP_CLK_FREQ_MASK | CLASSD_INTPMR_FRAME_MASK;
    val = FIELD_PREP(CLASSD_INTPMR_DSP_CLK_FREQ_MASK, sample_rates[best].dsp_clk)
        | FIELD_PREP(CLASSD_INTPMR_FRAME_MASK, sample_rates[best].sample_rate);

    snd_soc_component_update_bits(component, CLASSD_INTPMR, mask, val);

    clk_prepare_enable((*dd).gclk)
}

unsafe extern "C" fn atmel_classd_cpu_dai_shutdown(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_classd;

    clk_disable_unprepare((*dd).gclk);
}

unsafe extern "C" fn atmel_classd_cpu_dai_prepare(
    _substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*cpu_dai).component;

    snd_soc_component_update_bits(
        component,
        CLASSD_MR,
        CLASSD_MR_LEN_MASK | CLASSD_MR_REN_MASK,
        FIELD_PREP(CLASSD_MR_LEN_MASK, CLASSD_MR_LEN_DIS)
            | FIELD_PREP(CLASSD_MR_REN_MASK, CLASSD_MR_REN_DIS),
    );

    0
}

unsafe extern "C" fn atmel_classd_cpu_dai_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*cpu_dai).component;
    let mask: u32;
    let val: u32;

    mask = CLASSD_MR_LEN_MASK | CLASSD_MR_REN_MASK;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = mask;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val = FIELD_PREP(CLASSD_MR_LEN_MASK, CLASSD_MR_LEN_DIS)
                | FIELD_PREP(CLASSD_MR_REN_MASK, CLASSD_MR_REN_DIS);
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, CLASSD_MR, mask, val);

    0
}

static atmel_classd_cpu_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(atmel_classd_cpu_dai_startup),
    shutdown: Some(atmel_classd_cpu_dai_shutdown),
    mute_stream: Some(atmel_classd_cpu_dai_mute_stream),
    hw_params: Some(atmel_classd_cpu_dai_hw_params),
    prepare: Some(atmel_classd_cpu_dai_prepare),
    trigger: Some(atmel_classd_cpu_dai_trigger),
    no_capture_mute: 1,
};

static mut atmel_classd_cpu_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: ATMEL_CLASSD_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &atmel_classd_cpu_dai_ops,
};

static atmel_classd_cpu_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"atmel-classd\0".as_ptr() as *const c_char,
    probe: Some(atmel_classd_component_probe),
    resume: Some(atmel_classd_component_resume),
    controls: atmel_classd_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&atmel_classd_snd_controls) as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    legacy_dai_naming: 1,
};

/* ASoC sound card */
unsafe extern "C" fn atmel_classd_asoc_card_init(
    dev: *mut device,
    card: *mut snd_soc_card,
) -> c_int {
    let dai_link: *mut snd_soc_dai_link;
    let dd: *mut atmel_classd = snd_soc_card_get_drvdata(card) as *mut atmel_classd;
    let comp: *mut snd_soc_dai_link_component;

    dai_link = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    if dai_link.is_null() {
        return -ENOMEM;
    }

    comp = devm_kzalloc(dev, 2 * size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    if comp.is_null() {
        return -ENOMEM;
    }

    (*dai_link).cpus = comp.add(0);
    (*dai_link).codecs = &snd_soc_dummy_dlc as *const snd_soc_dai_link_component as *mut snd_soc_dai_link_component;
    (*dai_link).platforms = comp.add(1);

    (*dai_link).num_cpus = 1;
    (*dai_link).num_codecs = 1;
    (*dai_link).num_platforms = 1;

    (*dai_link).name = b"CLASSD\0".as_ptr() as *const c_char;
    (*dai_link).stream_name = b"CLASSD PCM\0".as_ptr() as *const c_char;
    (*(*dai_link).cpus).dai_name = dev_name(dev);
    (*(*dai_link).platforms).name = dev_name(dev);

    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).name = (*(*dd).pdata).card_name;
    (*card).dev = dev;

    0
}

/* regmap configuration */
static atmel_classd_reg_defaults: [reg_default; 1] = [reg_default {
    reg: CLASSD_INTPMR,
    def: 0x00301212,
}];

const ATMEL_CLASSD_REG_MAX: c_uint = 0xE4;
static atmel_classd_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: ATMEL_CLASSD_REG_MAX,

    cache_type: REGCACHE_FLAT,
    reg_defaults: atmel_classd_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&atmel_classd_reg_defaults) as c_uint,
};

unsafe extern "C" fn atmel_classd_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut dd: *mut atmel_classd;
    let mut res: *mut resource = ptr::null_mut();
    let io_base: *mut c_void;
    let mut pdata: *const atmel_classd_pdata;
    let card: *mut snd_soc_card;
    let mut ret: c_int;

    pdata = dev_get_platdata(dev) as *const atmel_classd_pdata;
    if pdata.is_null() {
        pdata = atmel_classd_dt_init(dev);
        if IS_ERR(pdata) {
            return PTR_ERR(pdata);
        }
    }

    dd = devm_kzalloc(dev, size_of::<atmel_classd>(), GFP_KERNEL) as *mut atmel_classd;
    if dd.is_null() {
        return -ENOMEM;
    }

    (*dd).pdata = pdata;

    (*dd).irq = platform_get_irq(pdev, 0);
    if (*dd).irq < 0 {
        return (*dd).irq;
    }

    (*dd).pclk = devm_clk_get(dev, b"pclk\0".as_ptr() as *const c_char);
    if IS_ERR((*dd).pclk) {
        ret = PTR_ERR((*dd).pclk);
        dev_err(dev, b"failed to get peripheral clock: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*dd).gclk = devm_clk_get(dev, b"gclk\0".as_ptr() as *const c_char);
    if IS_ERR((*dd).gclk) {
        ret = PTR_ERR((*dd).gclk);
        dev_err(dev, b"failed to get GCK clock: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(io_base) {
        return PTR_ERR(io_base);
    }

    (*dd).phy_base = (*res).start;
    (*dd).dev = dev;

    (*dd).regmap = devm_regmap_init_mmio(dev, io_base, &atmel_classd_regmap_config);
    if IS_ERR((*dd).regmap) {
        ret = PTR_ERR((*dd).regmap);
        dev_err(dev, b"failed to init register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        dev,
        &atmel_classd_cpu_dai_component,
        &mut atmel_classd_cpu_dai,
        1,
    );
    if ret != 0 {
        dev_err(dev, b"could not register CPU DAI: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(dev, &atmel_classd_dmaengine_pcm_config, 0);
    if ret != 0 {
        dev_err(dev, b"could not register platform: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* register sound card */
    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    snd_soc_card_set_drvdata(card, dd as *mut c_void);

    ret = atmel_classd_asoc_card_init(dev, card);
    if ret != 0 {
        dev_err(dev, b"failed to init sound card\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        dev_err(dev, b"failed to register sound card: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

static mut atmel_classd_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"atmel-classd\0".as_ptr() as *const c_char,
        of_match_table: atmel_classd_of_match.as_ptr(),
        pm: &snd_soc_pm_ops,
    },
    probe: Some(atmel_classd_probe),
};

// module_platform_driver(atmel_classd_driver);
// MODULE_DESCRIPTION("Atmel ClassD driver under ALSA SoC architecture");
// MODULE_AUTHOR("Songjun Wu <songjun.wu@atmel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
