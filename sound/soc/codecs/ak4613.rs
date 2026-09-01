// SPDX-License-Identifier: GPL-2.0
//
// ak4613.c  --  Asahi Kasei ALSA Soc Audio driver
//
// Copyright (C) 2015 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// Based on ak4642.c by Kuninori Morimoto
// Based on wm8731.c by Richard Purdie
// Based on ak4535.c by Richard Purdie
// Based on wm8753.c by Liam Girdwood

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = core::ffi::c_uchar;
type u64 = core::ffi::c_ulonglong;
type bool_ = bool;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct ak4613_interface {
    pub width: c_uint,
    pub fmt: c_uint,
    pub dif: u8,
}

#[repr(C)]
pub struct ak4613_priv {
    pub lock: mutex,
    pub constraint_rates: snd_pcm_hw_constraint_list,
    pub constraint_channels: snd_pcm_hw_constraint_list,
    pub dummy_write_work: work_struct,
    pub component: *mut snd_soc_component,
    pub rate: c_uint,
    pub sysclk: c_uint,
    pub fmt: c_uint,
    pub configs: c_uint,
    pub cnt: c_int,
    pub ctrl1: u8,
    pub oc: u8,
    pub ic: u8,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
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
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const PW_MGMT1: c_uint = 0x00;
const PW_MGMT2: c_uint = 0x01;
const PW_MGMT3: c_uint = 0x02;
const CTRL1: c_uint = 0x03;
const CTRL2: c_uint = 0x04;
const DEMP1: c_uint = 0x05;
const DEMP2: c_uint = 0x06;
const OFD: c_uint = 0x07;
const ZRD: c_uint = 0x08;
const ICTRL: c_uint = 0x09;
const OCTRL: c_uint = 0x0a;
const LOUT1: c_uint = 0x0b;
const ROUT1: c_uint = 0x0c;
const LOUT2: c_uint = 0x0d;
const ROUT2: c_uint = 0x0e;
const LOUT3: c_uint = 0x0f;
const ROUT3: c_uint = 0x10;
const LOUT4: c_uint = 0x11;
const ROUT4: c_uint = 0x12;
const LOUT5: c_uint = 0x13;
const ROUT5: c_uint = 0x14;
const LOUT6: c_uint = 0x15;
const ROUT6: c_uint = 0x16;

const RSTN: c_uint = BIT(0);
const PMDAC: c_uint = BIT(1);
const PMADC: c_uint = BIT(2);
const PMVR: c_uint = BIT(3);
const PMAD_ALL: c_uint = 0x7;
const PMDA_ALL: c_uint = 0x3f;
const DIF0: c_uint = BIT(3);
const DIF1: c_uint = BIT(4);
const DIF2: c_uint = BIT(5);
const TDM0: c_uint = BIT(6);
const TDM1: c_uint = BIT(7);
const NO_FMT: c_uint = 0xff;
const FMT_MASK: c_uint = 0xf8;
const DFS_MASK: c_uint = 3 << 2;
const DFS_NORMAL_SPEED: u8 = (0 << 2) as u8;
const DFS_DOUBLE_SPEED: u8 = (1 << 2) as u8;
const DFS_QUAD_SPEED: u8 = (2 << 2) as u8;
const ICTRL_MASK: c_uint = 0x3;
const OCTRL_MASK: c_uint = 0x3f;

const AK4613_CONFIG_SDTI_MASK: c_uint = 0xF << 4;
const fn AK4613_CONFIG_SDTI(x: c_uint) -> c_uint {
    (x & 0xF) << 4
}
unsafe fn AK4613_CONFIG_SDTI_set(priv_: *mut ak4613_priv, x: c_uint) {
    (*priv_).configs |= AK4613_CONFIG_SDTI(x);
}
unsafe fn AK4613_CONFIG_SDTI_get(priv_: *mut ak4613_priv) -> c_uint {
    (((*priv_).configs & AK4613_CONFIG_SDTI_MASK) >> 4) & 0xF
}

const AK4613_CONFIG_MODE_MASK: c_uint = 0xF;
const AK4613_CONFIG_MODE_STEREO: c_uint = 0x0;
const AK4613_CONFIG_MODE_TDM512: c_uint = 0x1;
const AK4613_CONFIG_MODE_TDM256: c_uint = 0x2;
const AK4613_CONFIG_MODE_TDM128: c_uint = 0x3;
unsafe fn AK4613_CONFIG_GET_MODE(priv_: *mut ak4613_priv) -> c_uint {
    (*priv_).configs & AK4613_CONFIG_MODE_MASK
}
unsafe fn AK4613_CONFIG_SET_MODE_TDM256(priv_: *mut ak4613_priv) {
    (*priv_).configs |= AK4613_CONFIG_MODE_TDM256;
}
unsafe fn AK4613_CTRL1_TO_MODE(priv_: *mut ak4613_priv) -> c_uint {
    ((*priv_).ctrl1 >> 6) as c_uint
}

const AK4613_CHANNEL_2: usize = 0;
const AK4613_CHANNEL_4: usize = 1;
const AK4613_CHANNEL_8: usize = 2;
const AK4613_CHANNEL_12: usize = 3;
const AK4613_CHANNEL_NONE: c_int = -1;
const MODE_MAX: usize = 4;
const SDTx_MAX: usize = 4;
const fn MASK(idx: usize) -> c_int {
    1 << idx
}

static out_tlv: [c_uint; 3] = [(-12750i32) as c_uint, 50, 1];

// SOC_DOUBLE_R_TLV initializers are supplied by the ASoC C macro environment.
static ak4613_snd_controls: [snd_kcontrol_new; 0] = [];

static ak4613_reg: [reg_default; 23] = [
    reg_default { reg: 0x0, def: 0x0f }, reg_default { reg: 0x1, def: 0x07 },
    reg_default { reg: 0x2, def: 0x3f }, reg_default { reg: 0x3, def: 0x20 },
    reg_default { reg: 0x4, def: 0x20 }, reg_default { reg: 0x5, def: 0x55 },
    reg_default { reg: 0x6, def: 0x05 }, reg_default { reg: 0x7, def: 0x07 },
    reg_default { reg: 0x8, def: 0x0f }, reg_default { reg: 0x9, def: 0x07 },
    reg_default { reg: 0xa, def: 0x3f }, reg_default { reg: 0xb, def: 0x00 },
    reg_default { reg: 0xc, def: 0x00 }, reg_default { reg: 0xd, def: 0x00 },
    reg_default { reg: 0xe, def: 0x00 }, reg_default { reg: 0xf, def: 0x00 },
    reg_default { reg: 0x10, def: 0x00 }, reg_default { reg: 0x11, def: 0x00 },
    reg_default { reg: 0x12, def: 0x00 }, reg_default { reg: 0x13, def: 0x00 },
    reg_default { reg: 0x14, def: 0x00 }, reg_default { reg: 0x15, def: 0x00 },
    reg_default { reg: 0x16, def: 0x00 },
];

static ak4613_iface: [ak4613_interface; 2] = [
    ak4613_interface { dif: 0x03, width: 24, fmt: SND_SOC_DAIFMT_LEFT_J },
    ak4613_interface { dif: 0x04, width: 24, fmt: SND_SOC_DAIFMT_I2S },
];

static ak4613_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0x16,
    reg_defaults: ak4613_reg.as_ptr(),
    num_reg_defaults: ak4613_reg.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
};

static ak4613_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"asahi-kasei,ak4613".as_ptr(),
        data: &ak4613_regmap_cfg as *const _ as *const c_void,
    },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];

static ak4613_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"ak4613".as_ptr(),
        driver_data: &ak4613_regmap_cfg as *const _ as c_ulong,
    },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];

// SND_SOC_DAPM_* widget initializers are supplied by the ASoC C macro environment.
static ak4613_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static ak4613_intercon: [snd_soc_dapm_route; 28] = [
    route(c"LOUT1".as_ptr(), ptr::null(), c"DAC1".as_ptr()),
    route(c"LOUT2".as_ptr(), ptr::null(), c"DAC2".as_ptr()),
    route(c"LOUT3".as_ptr(), ptr::null(), c"DAC3".as_ptr()),
    route(c"LOUT4".as_ptr(), ptr::null(), c"DAC4".as_ptr()),
    route(c"LOUT5".as_ptr(), ptr::null(), c"DAC5".as_ptr()),
    route(c"LOUT6".as_ptr(), ptr::null(), c"DAC6".as_ptr()),
    route(c"ROUT1".as_ptr(), ptr::null(), c"DAC1".as_ptr()),
    route(c"ROUT2".as_ptr(), ptr::null(), c"DAC2".as_ptr()),
    route(c"ROUT3".as_ptr(), ptr::null(), c"DAC3".as_ptr()),
    route(c"ROUT4".as_ptr(), ptr::null(), c"DAC4".as_ptr()),
    route(c"ROUT5".as_ptr(), ptr::null(), c"DAC5".as_ptr()),
    route(c"ROUT6".as_ptr(), ptr::null(), c"DAC6".as_ptr()),
    route(c"DAC1".as_ptr(), ptr::null(), c"Playback".as_ptr()),
    route(c"DAC2".as_ptr(), ptr::null(), c"Playback".as_ptr()),
    route(c"DAC3".as_ptr(), ptr::null(), c"Playback".as_ptr()),
    route(c"DAC4".as_ptr(), ptr::null(), c"Playback".as_ptr()),
    route(c"DAC5".as_ptr(), ptr::null(), c"Playback".as_ptr()),
    route(c"DAC6".as_ptr(), ptr::null(), c"Playback".as_ptr()),
    route(c"Capture".as_ptr(), ptr::null(), c"ADC1".as_ptr()),
    route(c"Capture".as_ptr(), ptr::null(), c"ADC2".as_ptr()),
    route(c"ADC1".as_ptr(), ptr::null(), c"LIN1".as_ptr()),
    route(c"ADC2".as_ptr(), ptr::null(), c"LIN2".as_ptr()),
    route(c"ADC1".as_ptr(), ptr::null(), c"RIN1".as_ptr()),
    route(c"ADC2".as_ptr(), ptr::null(), c"RIN2".as_ptr()),
    route(ptr::null(), ptr::null(), ptr::null()),
    route(ptr::null(), ptr::null(), ptr::null()),
    route(ptr::null(), ptr::null(), ptr::null()),
    route(ptr::null(), ptr::null(), ptr::null()),
];

const fn route(sink: *const c_char, control: *const c_char, source: *const c_char) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink, control, source }
}

unsafe extern "C" fn ak4613_dai_shutdown(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4613_priv;
    let dev = (*component).dev;

    mutex_lock(&mut (*priv_).lock);
    (*priv_).cnt -= 1;
    if (*priv_).cnt < 0 {
        dev_err(dev, c"unexpected counter error\n".as_ptr());
        (*priv_).cnt = 0;
    }
    if (*priv_).cnt == 0 {
        (*priv_).ctrl1 = 0;
    }
    mutex_unlock(&mut (*priv_).lock);
}

unsafe fn ak4613_hw_constraints(priv_: *mut ak4613_priv, substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    static ak4613_rates: [c_uint; 8] = [32000, 44100, 48000, 64000, 88200, 96000, 176400, 192000];
    static ak4613_channels: [c_uint; 4] = [2, 4, 8, 12];
    static mask_list: [[c_int; SDTx_MAX]; MODE_MAX] = [
        [MASK(AK4613_CHANNEL_2), MASK(AK4613_CHANNEL_2), MASK(AK4613_CHANNEL_2), MASK(AK4613_CHANNEL_2)],
        [MASK(AK4613_CHANNEL_4), MASK(AK4613_CHANNEL_12), MASK(AK4613_CHANNEL_12), MASK(AK4613_CHANNEL_12)],
        [MASK(AK4613_CHANNEL_4), MASK(AK4613_CHANNEL_8), MASK(AK4613_CHANNEL_8) | MASK(AK4613_CHANNEL_12), MASK(AK4613_CHANNEL_8) | MASK(AK4613_CHANNEL_12)],
        [MASK(AK4613_CHANNEL_4), MASK(AK4613_CHANNEL_4), MASK(AK4613_CHANNEL_4) | MASK(AK4613_CHANNEL_8), MASK(AK4613_CHANNEL_4) | MASK(AK4613_CHANNEL_8) | MASK(AK4613_CHANNEL_12)],
    ];
    let mut constraint: *mut snd_pcm_hw_constraint_list;
    let mut mask: c_uint;
    let mode: c_uint;
    let mut fs: c_uint;
    let is_play = ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as usize;
    let sdti_num: c_int;

    constraint = &mut (*priv_).constraint_rates;
    (*constraint).list = ak4613_rates.as_ptr();
    (*constraint).mask = 0;
    (*constraint).count = 0;

    for i in 0..ak4613_rates.len() {
        fs = if ak4613_rates[i] <= 96000 { 256 } else { 128 };
        if (*priv_).sysclk >= ak4613_rates[i].wrapping_mul(fs) {
            (*constraint).count = (i + 1) as c_uint;
        }
    }

    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, constraint);

    sdti_num = AK4613_CONFIG_SDTI_get(priv_) as c_int;
    if WARN_ON((sdti_num as usize) >= SDTx_MAX) != 0 {
        return;
    }

    if (*priv_).cnt != 0 {
        mode = AK4613_CTRL1_TO_MODE(priv_);
        mask = 0;
    } else {
        mode = AK4613_CONFIG_GET_MODE(priv_);
        mask = mask_list[AK4613_CONFIG_MODE_STEREO as usize][is_play * sdti_num as usize] as c_uint;
    }

    if WARN_ON(mode as usize >= MODE_MAX) != 0 {
        return;
    }

    mask |= mask_list[mode as usize][is_play * sdti_num as usize] as c_uint;

    constraint = &mut (*priv_).constraint_channels;
    (*constraint).list = ak4613_channels.as_ptr();
    (*constraint).mask = mask;
    (*constraint).count = size_of::<[c_uint; 4]>() as c_uint;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, constraint);
}

unsafe extern "C" fn ak4613_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4613_priv;

    mutex_lock(&mut (*priv_).lock);
    ak4613_hw_constraints(priv_, substream);
    (*priv_).cnt += 1;
    mutex_unlock(&mut (*priv_).lock);

    0
}

unsafe extern "C" fn ak4613_dai_set_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4613_priv;
    (*priv_).sysclk = freq;
    0
}

unsafe extern "C" fn ak4613_dai_set_fmt(dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4613_priv;
    let mut fmt = format & SND_SOC_DAIFMT_FORMAT_MASK;

    match fmt {
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_I2S => (*priv_).fmt = fmt,
        _ => return -EINVAL,
    }

    fmt = format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    match fmt {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn ak4613_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4613_priv;
    let dev = (*component).dev;
    let width = params_width(params);
    let fmt = (*priv_).fmt;
    let rate: c_uint;
    let mut ret: c_int;
    let ctrl2: u8;

    rate = params_rate(params);
    match rate {
        32000 | 44100 | 48000 => ctrl2 = DFS_NORMAL_SPEED,
        64000 | 88200 | 96000 => ctrl2 = DFS_DOUBLE_SPEED,
        176400 | 192000 => ctrl2 = DFS_QUAD_SPEED,
        _ => return -EINVAL,
    }
    (*priv_).rate = rate;

    ret = -EINVAL;

    mutex_lock(&mut (*priv_).lock);
    if (*priv_).cnt > 1 {
        ret = 0;
    } else {
        let channel = params_channels(params);
        let tdm: u8 = if channel == 2 {
            AK4613_CONFIG_MODE_STEREO as u8
        } else {
            AK4613_CONFIG_GET_MODE(priv_) as u8
        };

        let mut i: isize = ak4613_iface.len() as isize - 1;
        while i >= 0 {
            let iface = ak4613_iface.as_ptr().offset(i);
            if (*iface).fmt == fmt && (*iface).width == width {
                (*priv_).ctrl1 = (tdm << 6) | ((*iface).dif << 3);
                ret = 0;
                break;
            }
            i -= 1;
        }
    }
    mutex_unlock(&mut (*priv_).lock);

    if ret < 0 {
        dev_warn(dev, c"unsupported data width/format combination\n".as_ptr());
        return ret;
    }

    snd_soc_component_update_bits(component, CTRL1, FMT_MASK, (*priv_).ctrl1 as c_uint);
    snd_soc_component_update_bits(component, CTRL2, DFS_MASK, ctrl2 as c_uint);
    snd_soc_component_update_bits(component, ICTRL, ICTRL_MASK, (*priv_).ic as c_uint);
    snd_soc_component_update_bits(component, OCTRL, OCTRL_MASK, (*priv_).oc as c_uint);

    ret
}

unsafe extern "C" fn ak4613_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let mut mgmt1: u8 = 0;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            mgmt1 |= RSTN as u8;
            mgmt1 |= (PMADC | PMDAC) as u8;
            mgmt1 |= PMVR as u8;
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            mgmt1 |= (PMADC | PMDAC) as u8;
            mgmt1 |= PMVR as u8;
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            mgmt1 |= PMVR as u8;
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {}
    }

    snd_soc_component_write(component, PW_MGMT1, mgmt1 as c_uint);
    0
}

unsafe extern "C" fn ak4613_dummy_write(work: *mut work_struct) {
    let priv_ = (work as *mut u8).sub(offset_of!(ak4613_priv, dummy_write_work)) as *mut ak4613_priv;
    let component = (*priv_).component;

    udelay(5000000 / (*priv_).rate);

    let mgmt1 = snd_soc_component_read(component, PW_MGMT1);
    let mgmt3 = snd_soc_component_read(component, PW_MGMT3);

    snd_soc_component_write(component, PW_MGMT1, mgmt1);
    snd_soc_component_write(component, PW_MGMT3, mgmt3);
}

unsafe extern "C" fn ak4613_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4613_priv;

    if cmd != SNDRV_PCM_TRIGGER_START && cmd != SNDRV_PCM_TRIGGER_RESUME {
        return 0;
    }

    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
        return 0;
    }

    (*priv_).component = component;
    schedule_work(&mut (*priv_).dummy_write_work);

    0
}

static ak4613_dai_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_LEFT_J;

static ak4613_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ak4613_dai_startup),
    shutdown: Some(ak4613_dai_shutdown),
    set_sysclk: Some(ak4613_dai_set_sysclk),
    set_fmt: Some(ak4613_dai_set_fmt),
    trigger: Some(ak4613_dai_trigger),
    hw_params: Some(ak4613_dai_hw_params),
    auto_selectable_formats: &ak4613_dai_formats,
    num_auto_selectable_formats: 1,
};

const AK4613_PCM_RATE: c_uint = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_64000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;
const AK4613_PCM_FMTBIT: u64 = SNDRV_PCM_FMTBIT_S24_LE;

static mut ak4613_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak4613-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 12,
        rates: AK4613_PCM_RATE,
        formats: AK4613_PCM_FMTBIT,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 4,
        rates: AK4613_PCM_RATE,
        formats: AK4613_PCM_FMTBIT,
    },
    ops: &ak4613_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn ak4613_suspend(component: *mut snd_soc_component) -> c_int {
    let regmap = dev_get_regmap((*component).dev, ptr::null());
    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    0
}

unsafe extern "C" fn ak4613_resume(component: *mut snd_soc_component) -> c_int {
    let regmap = dev_get_regmap((*component).dev, ptr::null());
    regcache_cache_only(regmap, false);
    regcache_sync(regmap)
}

static soc_component_dev_ak4613: snd_soc_component_driver = snd_soc_component_driver {
    suspend: Some(ak4613_suspend),
    resume: Some(ak4613_resume),
    set_bias_level: Some(ak4613_set_bias_level),
    controls: ak4613_snd_controls.as_ptr(),
    num_controls: ak4613_snd_controls.len() as c_uint,
    dapm_widgets: ak4613_dapm_widgets.as_ptr(),
    num_dapm_widgets: ak4613_dapm_widgets.len() as c_uint,
    dapm_routes: ak4613_intercon.as_ptr(),
    num_dapm_routes: ak4613_intercon.len() as c_uint,
    idle_bias_on: 1,
    endianness: 1,
};

unsafe fn ak4613_parse_of(priv_: *mut ak4613_priv, dev: *mut device) {
    let np = (*dev).of_node;
    let mut prop: [c_char; 32] = [0; 32];
    let mut sdti_num: c_int;

    for i in 0..2 {
        snprintf(prop.as_mut_ptr(), prop.len(), c"asahi-kasei,in%d-single-end".as_ptr(), i + 1);
        if !of_property_read_bool(np, prop.as_ptr()) {
            (*priv_).ic |= (1 << i) as u8;
        }
    }

    for i in 0..6 {
        snprintf(prop.as_mut_ptr(), prop.len(), c"asahi-kasei,out%d-single-end".as_ptr(), i + 1);
        if !of_property_read_bool(np, prop.as_ptr()) {
            (*priv_).oc |= (1 << i) as u8;
        }
    }

    // If AK4613_ENABLE_TDM_TEST is defined in the original build, enable TDM256 test mode.
    #[cfg(AK4613_ENABLE_TDM_TEST)]
    AK4613_CONFIG_SET_MODE_TDM256(priv_);

    sdti_num = of_graph_get_endpoint_count(np);
    if sdti_num >= SDTx_MAX as c_int || sdti_num < 1 {
        sdti_num = 1;
    }

    AK4613_CONFIG_SDTI_set(priv_, sdti_num as c_uint);
}

unsafe extern "C" fn ak4613_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let regmap_cfg = i2c_get_match_data(i2c) as *const regmap_config;
    let regmap: *mut regmap;
    let priv_: *mut ak4613_priv;

    if regmap_cfg.is_null() {
        return -EINVAL;
    }

    priv_ = devm_kzalloc(dev, size_of::<ak4613_priv>(), GFP_KERNEL) as *mut ak4613_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    ak4613_parse_of(priv_, dev);

    (*priv_).ctrl1 = 0;
    (*priv_).cnt = 0;
    (*priv_).sysclk = 0;
    INIT_WORK(&mut (*priv_).dummy_write_work, Some(ak4613_dummy_write));

    mutex_init(&mut (*priv_).lock);

    i2c_set_clientdata(i2c, priv_ as *mut c_void);

    regmap = devm_regmap_init_i2c(i2c, regmap_cfg);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    devm_snd_soc_register_component(dev, &soc_component_dev_ak4613, &mut ak4613_dai, 1)
}

static mut ak4613_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"ak4613-codec".as_ptr(),
        of_match_table: ak4613_of_match.as_ptr(),
    },
    probe: Some(ak4613_i2c_probe),
    id_table: ak4613_i2c_id.as_ptr(),
};

// module_i2c_driver(ak4613_i2c_driver);
// MODULE_DESCRIPTION("Soc AK4613 driver");
// MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
// MODULE_LICENSE("GPL v2");

extern "C" {
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static REGCACHE_RBTREE: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_64000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn WARN_ON(condition: bool) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn udelay(usecs: c_uint);
    fn schedule_work(work: *mut work_struct) -> bool_;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool_;
    fn of_graph_get_endpoint_count(np: *mut device_node) -> c_int;
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn INIT_WORK(work: *mut work_struct, func: Option<unsafe extern "C" fn(*mut work_struct)>);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
