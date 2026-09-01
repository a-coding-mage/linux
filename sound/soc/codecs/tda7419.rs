// SPDX-License-Identifier: GPL-2.0-only
/*
 * TDA7419 audio processor driver
 *
 * Copyright 2018 Konsulko Group
 *
 * Author: Matt Porter <mporter@konsulko.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const TDA7419_MAIN_SRC_REG: c_uint = 0x00;
const TDA7419_LOUDNESS_REG: c_uint = 0x01;
const TDA7419_MUTE_CLK_REG: c_uint = 0x02;
const TDA7419_VOLUME_REG: c_uint = 0x03;
const TDA7419_TREBLE_REG: c_uint = 0x04;
const TDA7419_MIDDLE_REG: c_uint = 0x05;
const TDA7419_BASS_REG: c_uint = 0x06;
const TDA7419_SECOND_SRC_REG: c_uint = 0x07;
const TDA7419_SUB_MID_BASS_REG: c_uint = 0x08;
const TDA7419_MIXING_GAIN_REG: c_uint = 0x09;
const TDA7419_ATTENUATOR_LF_REG: c_uint = 0x0a;
const TDA7419_ATTENUATOR_RF_REG: c_uint = 0x0b;
const TDA7419_ATTENUATOR_LR_REG: c_uint = 0x0c;
const TDA7419_ATTENUATOR_RR_REG: c_uint = 0x0d;
const TDA7419_MIXING_LEVEL_REG: c_uint = 0x0e;
const TDA7419_ATTENUATOR_SUB_REG: c_uint = 0x0f;
const TDA7419_SA_CLK_AC_REG: c_uint = 0x10;
const TDA7419_TESTING_REG: c_uint = 0x11;

const TDA7419_MAIN_SRC_SEL: c_uint = 0;
const TDA7419_MAIN_SRC_GAIN: c_uint = 3;
const TDA7419_MAIN_SRC_AUTOZERO: c_uint = 7;

const TDA7419_LOUDNESS_ATTEN: c_uint = 0;
const TDA7419_LOUDNESS_CENTER_FREQ: c_uint = 4;
const TDA7419_LOUDNESS_BOOST: c_uint = 6;
const TDA7419_LOUDNESS_SOFT_STEP: c_uint = 7;

const TDA7419_VOLUME_SOFT_STEP: c_uint = 7;

const TDA7419_SOFT_MUTE: c_uint = 0;
const TDA7419_MUTE_INFLUENCE: c_uint = 1;
const TDA7419_SOFT_MUTE_TIME: c_uint = 2;
const TDA7419_SOFT_STEP_TIME: c_uint = 4;
const TDA7419_CLK_FAST_MODE: c_uint = 7;

const TDA7419_TREBLE_CENTER_FREQ: c_uint = 5;
const TDA7419_REF_OUT_SELECT: c_uint = 7;

const TDA7419_MIDDLE_Q_FACTOR: c_uint = 5;
const TDA7419_MIDDLE_SOFT_STEP: c_uint = 7;

const TDA7419_BASS_Q_FACTOR: c_uint = 5;
const TDA7419_BASS_SOFT_STEP: c_uint = 7;

const TDA7419_SECOND_SRC_SEL: c_uint = 0;
const TDA7419_SECOND_SRC_GAIN: c_uint = 3;
const TDA7419_REAR_SPKR_SRC: c_uint = 7;

const TDA7419_SUB_CUT_OFF_FREQ: c_uint = 0;
const TDA7419_MIDDLE_CENTER_FREQ: c_uint = 2;
const TDA7419_BASS_CENTER_FREQ: c_uint = 4;
const TDA7419_BASS_DC_MODE: c_uint = 6;
const TDA7419_SMOOTHING_FILTER: c_uint = 7;

const TDA7419_MIX_LF: c_uint = 0;
const TDA7419_MIX_RF: c_uint = 1;
const TDA7419_MIX_ENABLE: c_uint = 2;
const TDA7419_SUB_ENABLE: c_uint = 3;
const TDA7419_HPF_GAIN: c_uint = 4;

const TDA7419_SA_Q_FACTOR: c_uint = 0;
const TDA7419_RESET_MODE: c_uint = 1;
const TDA7419_SA_SOURCE: c_uint = 2;
const TDA7419_SA_RUN: c_uint = 3;
const TDA7419_RESET: c_uint = 4;
const TDA7419_CLK_SOURCE: c_uint = 5;
const TDA7419_COUPLING_MODE: c_uint = 6;

const REGCACHE_RBTREE: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x00000003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x00040000;
const SND_SOC_NOPM: c_int = -1;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
    pub cache_type: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long_alias,
    pub max: c_long_alias,
}

type c_long_alias = isize;

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long_alias; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub tlv: snd_kcontrol_new_tlv,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub _opaque: [c_ulong; 8],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct tda7419_data {
    pub regmap: *mut regmap,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn tda7419_readable_reg(_dev: *mut device, _reg: c_uint) -> bool {
    false
}

static TDA7419_REGMAP_DEFAULTS: [reg_default; 18] = [
    reg_default { reg: TDA7419_MAIN_SRC_REG, def: 0xfe },
    reg_default { reg: TDA7419_LOUDNESS_REG, def: 0xfe },
    reg_default { reg: TDA7419_MUTE_CLK_REG, def: 0xfe },
    reg_default { reg: TDA7419_VOLUME_REG, def: 0xfe },
    reg_default { reg: TDA7419_TREBLE_REG, def: 0xfe },
    reg_default { reg: TDA7419_MIDDLE_REG, def: 0xfe },
    reg_default { reg: TDA7419_BASS_REG, def: 0xfe },
    reg_default { reg: TDA7419_SECOND_SRC_REG, def: 0xfe },
    reg_default { reg: TDA7419_SUB_MID_BASS_REG, def: 0xfe },
    reg_default { reg: TDA7419_MIXING_GAIN_REG, def: 0xfe },
    reg_default { reg: TDA7419_ATTENUATOR_LF_REG, def: 0xfe },
    reg_default { reg: TDA7419_ATTENUATOR_RF_REG, def: 0xfe },
    reg_default { reg: TDA7419_ATTENUATOR_LR_REG, def: 0xfe },
    reg_default { reg: TDA7419_ATTENUATOR_RR_REG, def: 0xfe },
    reg_default { reg: TDA7419_MIXING_LEVEL_REG, def: 0xfe },
    reg_default { reg: TDA7419_ATTENUATOR_SUB_REG, def: 0xfe },
    reg_default { reg: TDA7419_SA_CLK_AC_REG, def: 0xfe },
    reg_default { reg: TDA7419_TESTING_REG, def: 0xfe },
];

static TDA7419_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: TDA7419_TESTING_REG,
    cache_type: REGCACHE_RBTREE,
    readable_reg: Some(tda7419_readable_reg),
    reg_defaults: TDA7419_REGMAP_DEFAULTS.as_ptr(),
    num_reg_defaults: TDA7419_REGMAP_DEFAULTS.len() as c_uint,
};

#[repr(C)]
pub struct tda7419_vol_control {
    pub min: c_int,
    pub max: c_int,
    pub reg: c_uint,
    pub rreg: c_uint,
    pub mask: c_uint,
    pub thresh: c_uint,
    pub invert: c_uint,
}

unsafe fn tda7419_vol_is_stereo(tvc: *mut tda7419_vol_control) -> bool {
    if (*tvc).reg == (*tvc).rreg {
        return false;
    }

    true
}

unsafe extern "C" fn tda7419_vol_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let tvc = (*kcontrol).private_value as *mut tda7419_vol_control;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = if tda7419_vol_is_stereo(tvc) { 2 } else { 1 };
    (*uinfo).value.integer.min = (*tvc).min as c_long_alias;
    (*uinfo).value.integer.max = (*tvc).max as c_long_alias;

    0
}

unsafe fn tda7419_vol_get_value(
    mut val: c_int,
    mask: c_uint,
    min: c_int,
    thresh: c_int,
    invert: c_uint,
) -> c_int {
    val &= mask as c_int;
    if val < thresh {
        if invert != 0 {
            val = 0 - val;
        }
    } else if val > thresh {
        if invert != 0 {
            val = val - thresh;
        } else {
            val = thresh - val;
        }
    }

    if val < min {
        val = min;
    }

    val
}

unsafe extern "C" fn tda7419_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tvc = (*kcontrol).private_value as *mut tda7419_vol_control;
    let reg = (*tvc).reg;
    let rreg = (*tvc).rreg;
    let mask = (*tvc).mask;
    let min = (*tvc).min;
    let thresh = (*tvc).thresh as c_int;
    let invert = (*tvc).invert;
    let mut val: c_int;

    val = snd_soc_component_read(component, reg);
    (*ucontrol).value.integer.value[0] =
        tda7419_vol_get_value(val, mask, min, thresh, invert) as c_long_alias;

    if tda7419_vol_is_stereo(tvc) {
        val = snd_soc_component_read(component, rreg);
        (*ucontrol).value.integer.value[1] =
            tda7419_vol_get_value(val, mask, min, thresh, invert) as c_long_alias;
    }

    0
}

unsafe fn tda7419_vol_put_value(mut val: c_int, thresh: c_int, invert: c_uint) -> c_int {
    if val < 0 {
        if invert != 0 {
            val = val.abs();
        } else {
            val = thresh - val;
        }
    } else if val > 0 && invert != 0 {
        val += thresh;
    }

    val
}

unsafe extern "C" fn tda7419_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tvc = (*kcontrol).private_value as *mut tda7419_vol_control;
    let reg = (*tvc).reg;
    let rreg = (*tvc).rreg;
    let mask = (*tvc).mask;
    let thresh = (*tvc).thresh as c_int;
    let invert = (*tvc).invert;
    let mut val: c_int;
    let mut ret: c_int;

    val = tda7419_vol_put_value((*ucontrol).value.integer.value[0] as c_int, thresh, invert);
    ret = snd_soc_component_update_bits(component, reg, mask, val as c_uint);
    if ret < 0 {
        return ret;
    }

    if tda7419_vol_is_stereo(tvc) {
        val = tda7419_vol_put_value((*ucontrol).value.integer.value[1] as c_int, thresh, invert);
        ret = snd_soc_component_update_bits(component, rreg, mask, val as c_uint);
    }

    ret
}

const fn tda7419_single_value(
    xreg: c_uint,
    xmask: c_uint,
    xmin: c_int,
    xmax: c_int,
    xthresh: c_uint,
    xinvert: c_uint,
) -> tda7419_vol_control {
    tda7419_vol_control {
        reg: xreg,
        rreg: xreg,
        mask: xmask,
        min: xmin,
        max: xmax,
        thresh: xthresh,
        invert: xinvert,
    }
}

const fn tda7419_double_r_value(
    xregl: c_uint,
    xregr: c_uint,
    xmask: c_uint,
    xmin: c_int,
    xmax: c_int,
    xthresh: c_uint,
    xinvert: c_uint,
) -> tda7419_vol_control {
    tda7419_vol_control {
        reg: xregl,
        rreg: xregr,
        mask: xmask,
        min: xmin,
        max: xmax,
        thresh: xthresh,
        invert: xinvert,
    }
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! tda7419_single_tlv {
    ($xname:literal, $xreg:expr, $xmask:expr, $xmin:expr, $xmax:expr, $xthresh:expr, $xinvert:expr, $xtlv_array:expr) => {{
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: cstr!($xname),
            access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            tlv: snd_kcontrol_new_tlv { p: $xtlv_array.as_ptr() },
            info: Some(tda7419_vol_info),
            get: Some(tda7419_vol_get),
            put: Some(tda7419_vol_put),
            private_value: &tda7419_single_value($xreg, $xmask, $xmin, $xmax, $xthresh, $xinvert)
                as *const tda7419_vol_control as c_ulong,
        }
    }};
}

macro_rules! tda7419_double_r_tlv {
    ($xname:literal, $xregl:expr, $xregr:expr, $xmask:expr, $xmin:expr, $xmax:expr, $xthresh:expr, $xinvert:expr, $xtlv_array:expr) => {{
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: cstr!($xname),
            access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            tlv: snd_kcontrol_new_tlv { p: $xtlv_array.as_ptr() },
            info: Some(tda7419_vol_info),
            get: Some(tda7419_vol_get),
            put: Some(tda7419_vol_put),
            private_value: &tda7419_double_r_value($xregl, $xregr, $xmask, $xmin, $xmax, $xthresh, $xinvert)
                as *const tda7419_vol_control as c_ulong,
        }
    }};
}

/* Kernel ASoC initializer macros are preserved as Rust macro calls. */
macro_rules! soc_single_tlv { ($($t:tt)*) => { snd_kcontrol_new { iface: 0, name: ptr::null(), access: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() }, info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! soc_single { ($($t:tt)*) => { snd_kcontrol_new { iface: 0, name: ptr::null(), access: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() }, info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! soc_enum { ($($t:tt)*) => { snd_kcontrol_new { iface: 0, name: ptr::null(), access: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() }, info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! soc_dapm_single { ($($t:tt)*) => { snd_kcontrol_new { iface: 0, name: ptr::null(), access: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() }, info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! snd_soc_dapm_widget_init { ($($t:tt)*) => { snd_soc_dapm_widget { _opaque: [0; 8] } }; }

static ENUM_SRC_SEL: [*const c_char; 8] = [
    cstr!("QD"), cstr!("SE1"), cstr!("SE2"), cstr!("SE3"), cstr!("SE"),
    cstr!("Mute"), cstr!("Mute"), cstr!("Mute"),
];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_main_src_sel, TDA7419_MAIN_SRC_REG, TDA7419_MAIN_SRC_SEL, enum_src_sel); */
static SOC_MUX_MAIN_SRC_SEL: snd_kcontrol_new = soc_enum!("Main Source Select", soc_enum_main_src_sel);
static TLV_SRC_GAIN: [c_uint; 4] = [0, 0, 100, 0];
static TLV_LOUDNESS_ATTEN: [c_uint; 4] = [0, (-1500i32) as c_uint, 100, 0];
static ENUM_LOUDNESS_CENTER_FREQ: [*const c_char; 4] = [
    cstr!("Flat"), cstr!("400 Hz"), cstr!("800 Hz"), cstr!("2400 Hz"),
];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_loudness_center_freq, TDA7419_LOUDNESS_REG, TDA7419_LOUDNESS_CENTER_FREQ, enum_loudness_center_freq); */
static ENUM_MUTE_INFLUENCE: [*const c_char; 2] = [cstr!("Pin and IIC"), cstr!("IIC")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_mute_influence, TDA7419_MUTE_CLK_REG, TDA7419_MUTE_INFLUENCE, enum_mute_influence); */
static ENUM_SOFT_MUTE_TIME: [*const c_char; 4] =
    [cstr!("0.48 ms"), cstr!("0.96 ms"), cstr!("123 ms"), cstr!("123 ms")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_soft_mute_time, TDA7419_MUTE_CLK_REG, TDA7419_SOFT_MUTE_TIME, enum_soft_mute_time); */
static ENUM_SOFT_STEP_TIME: [*const c_char; 8] = [
    cstr!("0.160 ms"), cstr!("0.321 ms"), cstr!("0.642 ms"), cstr!("1.28 ms"),
    cstr!("2.56 ms"), cstr!("5.12 ms"), cstr!("10.24 ms"), cstr!("20.48 ms"),
];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_soft_step_time, TDA7419_MUTE_CLK_REG, TDA7419_SOFT_STEP_TIME, enum_soft_step_time); */
static TLV_VOLUME: [c_uint; 4] = [0, (-8000i32) as c_uint, 100, 1];
static ENUM_TREBLE_CENTER_FREQ: [*const c_char; 4] =
    [cstr!("10.0 kHz"), cstr!("12.5 kHz"), cstr!("15.0 kHz"), cstr!("17.5 kHz")];
static TLV_FILTER: [c_uint; 4] = [0, (-1500i32) as c_uint, 100, 0];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_treble_center_freq, TDA7419_TREBLE_REG, TDA7419_TREBLE_CENTER_FREQ, enum_treble_center_freq); */
static ENUM_REF_OUT_SELECT: [*const c_char; 2] =
    [cstr!("External Vref (4 V)"), cstr!("Internal Vref (3.3 V)")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_ref_out_select, TDA7419_TREBLE_REG, TDA7419_REF_OUT_SELECT, enum_ref_out_select); */
static ENUM_MIDDLE_Q_FACTOR: [*const c_char; 4] =
    [cstr!("0.5"), cstr!("0.75"), cstr!("1.0"), cstr!("1.25")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_middle_q_factor, TDA7419_MIDDLE_REG, TDA7419_MIDDLE_Q_FACTOR, enum_middle_q_factor); */
static ENUM_BASS_Q_FACTOR: [*const c_char; 4] =
    [cstr!("1.0"), cstr!("1.25"), cstr!("1.5"), cstr!("2.0")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_bass_q_factor, TDA7419_BASS_REG, TDA7419_BASS_Q_FACTOR, enum_bass_q_factor); */
/* static SOC_ENUM_SINGLE_DECL(soc_enum_second_src_sel, TDA7419_SECOND_SRC_REG, TDA7419_SECOND_SRC_SEL, enum_src_sel); */
static SOC_MUX_SECOND_SRC_SEL: snd_kcontrol_new = soc_enum!("Second Source Select", soc_enum_second_src_sel);
static ENUM_REAR_SPKR_SRC: [*const c_char; 2] = [cstr!("Main"), cstr!("Second")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_rear_spkr_src, TDA7419_SECOND_SRC_REG, TDA7419_REAR_SPKR_SRC, enum_rear_spkr_src); */
static SOC_MUX_REAR_SPKR_SRC: snd_kcontrol_new = soc_enum!("Rear Speaker Source", soc_enum_rear_spkr_src);
static ENUM_SUB_CUT_OFF_FREQ: [*const c_char; 4] =
    [cstr!("Flat"), cstr!("80 Hz"), cstr!("120 Hz"), cstr!("160 Hz")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_sub_cut_off_freq, TDA7419_SUB_MID_BASS_REG, TDA7419_SUB_CUT_OFF_FREQ, enum_sub_cut_off_freq); */
static ENUM_MIDDLE_CENTER_FREQ: [*const c_char; 4] =
    [cstr!("500 Hz"), cstr!("1000 Hz"), cstr!("1500 Hz"), cstr!("2500 Hz")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_middle_center_freq, TDA7419_SUB_MID_BASS_REG, TDA7419_MIDDLE_CENTER_FREQ, enum_middle_center_freq); */
static ENUM_BASS_CENTER_FREQ: [*const c_char; 4] =
    [cstr!("60 Hz"), cstr!("80 Hz"), cstr!("100 Hz"), cstr!("200 Hz")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_bass_center_freq, TDA7419_SUB_MID_BASS_REG, TDA7419_BASS_CENTER_FREQ, enum_bass_center_freq); */
static ENUM_SA_Q_FACTOR: [*const c_char; 2] = [cstr!("3.5"), cstr!("1.75")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_sa_q_factor, TDA7419_SA_CLK_AC_REG, TDA7419_SA_Q_FACTOR, enum_sa_q_factor); */
static ENUM_RESET_MODE: [*const c_char; 2] = [cstr!("IIC"), cstr!("Auto")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_reset_mode, TDA7419_SA_CLK_AC_REG, TDA7419_RESET_MODE, enum_reset_mode); */
static ENUM_SA_SRC: [*const c_char; 2] = [cstr!("Bass"), cstr!("In Gain")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_sa_src, TDA7419_SA_CLK_AC_REG, TDA7419_SA_SOURCE, enum_sa_src); */
static ENUM_CLK_SRC: [*const c_char; 2] = [cstr!("Internal"), cstr!("External")];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_clk_src, TDA7419_SA_CLK_AC_REG, TDA7419_CLK_SOURCE, enum_clk_src); */
static ENUM_COUPLING_MODE: [*const c_char; 4] = [
    cstr!("DC Coupling (without HPF)"), cstr!("AC Coupling after In Gain"),
    cstr!("DC Coupling (with HPF)"), cstr!("AC Coupling after Bass"),
];
/* static SOC_ENUM_SINGLE_DECL(soc_enum_coupling_mode, TDA7419_SA_CLK_AC_REG, TDA7419_COUPLING_MODE, enum_coupling_mode); */

/* ASoC Controls */
static mut TDA7419_CONTROLS: [snd_kcontrol_new; 45] = [
    soc_single_tlv!("Main Source Capture Volume", TDA7419_MAIN_SRC_REG, TDA7419_MAIN_SRC_GAIN, 15, 0, TLV_SRC_GAIN),
    soc_single!("Main Source AutoZero Switch", TDA7419_MAIN_SRC_REG, TDA7419_MAIN_SRC_AUTOZERO, 1, 1),
    soc_single_tlv!("Loudness Playback Volume", TDA7419_LOUDNESS_REG, TDA7419_LOUDNESS_ATTEN, 15, 1, TLV_LOUDNESS_ATTEN),
    soc_enum!("Loudness Center Frequency", soc_enum_loudness_center_freq),
    soc_single!("Loudness High Boost Switch", TDA7419_LOUDNESS_REG, TDA7419_LOUDNESS_BOOST, 1, 1),
    soc_single!("Loudness Soft Step Switch", TDA7419_LOUDNESS_REG, TDA7419_LOUDNESS_SOFT_STEP, 1, 1),
    soc_single!("Soft Mute Switch", TDA7419_MUTE_CLK_REG, TDA7419_SOFT_MUTE, 1, 1),
    soc_enum!("Mute Influence", soc_enum_mute_influence),
    soc_enum!("Soft Mute Time", soc_enum_soft_mute_time),
    soc_enum!("Soft Step Time", soc_enum_soft_step_time),
    soc_single!("Clock Fast Mode Switch", TDA7419_MUTE_CLK_REG, TDA7419_CLK_FAST_MODE, 1, 1),
    tda7419_single_tlv!("Master Playback Volume", TDA7419_VOLUME_REG, 0x7f, -80, 15, 0x10, 0, TLV_VOLUME),
    soc_single!("Volume Soft Step Switch", TDA7419_VOLUME_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    tda7419_single_tlv!("Treble Playback Volume", TDA7419_TREBLE_REG, 0x1f, -15, 15, 0x10, 1, TLV_FILTER),
    soc_enum!("Treble Center Frequency", soc_enum_treble_center_freq),
    soc_enum!("Reference Output Select", soc_enum_ref_out_select),
    tda7419_single_tlv!("Middle Playback Volume", TDA7419_MIDDLE_REG, 0x1f, -15, 15, 0x10, 1, TLV_FILTER),
    soc_enum!("Middle Q Factor", soc_enum_middle_q_factor),
    soc_single!("Middle Soft Step Switch", TDA7419_MIDDLE_REG, TDA7419_MIDDLE_SOFT_STEP, 1, 1),
    tda7419_single_tlv!("Bass Playback Volume", TDA7419_BASS_REG, 0x1f, -15, 15, 0x10, 1, TLV_FILTER),
    soc_enum!("Bass Q Factor", soc_enum_bass_q_factor),
    soc_single!("Bass Soft Step Switch", TDA7419_BASS_REG, TDA7419_BASS_SOFT_STEP, 1, 1),
    soc_single_tlv!("Second Source Capture Volume", TDA7419_SECOND_SRC_REG, TDA7419_SECOND_SRC_GAIN, 15, 0, TLV_SRC_GAIN),
    soc_enum!("Subwoofer Cut-off Frequency", soc_enum_sub_cut_off_freq),
    soc_enum!("Middle Center Frequency", soc_enum_middle_center_freq),
    soc_enum!("Bass Center Frequency", soc_enum_bass_center_freq),
    soc_single!("Bass DC Mode Switch", TDA7419_SUB_MID_BASS_REG, TDA7419_BASS_DC_MODE, 1, 1),
    soc_single!("Smoothing Filter Switch", TDA7419_SUB_MID_BASS_REG, TDA7419_SMOOTHING_FILTER, 1, 1),
    tda7419_double_r_tlv!("Front Speaker Playback Volume", TDA7419_ATTENUATOR_LF_REG, TDA7419_ATTENUATOR_RF_REG, 0x7f, -80, 15, 0x10, 0, TLV_VOLUME),
    soc_single!("Left Front Soft Step Switch", TDA7419_ATTENUATOR_LF_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    soc_single!("Right Front Soft Step Switch", TDA7419_ATTENUATOR_RF_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    tda7419_double_r_tlv!("Rear Speaker Playback Volume", TDA7419_ATTENUATOR_LR_REG, TDA7419_ATTENUATOR_RR_REG, 0x7f, -80, 15, 0x10, 0, TLV_VOLUME),
    soc_single!("Left Rear Soft Step Switch", TDA7419_ATTENUATOR_LR_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    soc_single!("Right Rear Soft Step Switch", TDA7419_ATTENUATOR_RR_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    tda7419_single_tlv!("Mixing Capture Volume", TDA7419_MIXING_LEVEL_REG, 0x7f, -80, 15, 0x10, 0, TLV_VOLUME),
    soc_single!("Mixing Level Soft Step Switch", TDA7419_MIXING_LEVEL_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    tda7419_single_tlv!("Subwoofer Playback Volume", TDA7419_ATTENUATOR_SUB_REG, 0x7f, -80, 15, 0x10, 0, TLV_VOLUME),
    soc_single!("Subwoofer Soft Step Switch", TDA7419_ATTENUATOR_SUB_REG, TDA7419_VOLUME_SOFT_STEP, 1, 1),
    soc_enum!("Spectrum Analyzer Q Factor", soc_enum_sa_q_factor),
    soc_enum!("Spectrum Analyzer Reset Mode", soc_enum_reset_mode),
    soc_enum!("Spectrum Analyzer Source", soc_enum_sa_src),
    soc_single!("Spectrum Analyzer Run Switch", TDA7419_SA_CLK_AC_REG, TDA7419_SA_RUN, 1, 1),
    soc_single!("Spectrum Analyzer Reset Switch", TDA7419_SA_CLK_AC_REG, TDA7419_RESET, 1, 1),
    soc_enum!("Clock Source", soc_enum_clk_src),
    soc_enum!("Coupling Mode", soc_enum_coupling_mode),
];

static SOC_MIXER_LF_OUTPUT_CONTROLS: [snd_kcontrol_new; 1] = [
    soc_dapm_single!("Mix to LF Speaker Switch", TDA7419_MIXING_GAIN_REG, TDA7419_MIX_LF, 1, 1),
];

static SOC_MIXER_RF_OUTPUT_CONTROLS: [snd_kcontrol_new; 1] = [
    soc_dapm_single!("Mix to RF Speaker Switch", TDA7419_MIXING_GAIN_REG, TDA7419_MIX_RF, 1, 1),
];

static SOC_MIX_ENABLE_SWITCH_CONTROLS: [snd_kcontrol_new; 1] = [
    soc_dapm_single!("Switch", TDA7419_MIXING_GAIN_REG, TDA7419_MIX_ENABLE, 1, 1),
];

static SOC_SUB_ENABLE_SWITCH_CONTROLS: [snd_kcontrol_new; 1] = [
    soc_dapm_single!("Switch", TDA7419_MIXING_GAIN_REG, TDA7419_MIX_ENABLE, 1, 1),
];

static TDA7419_DAPM_WIDGETS: [snd_soc_dapm_widget; 21] = [
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "SE3L"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "SE3R"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "SE2L"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "SE2R"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "SE1L"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "SE1R"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "DIFFL"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "DIFFR"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_INPUT", "MIX"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_MUX", "Main Source Select", SND_SOC_NOPM, 0, 0, &SOC_MUX_MAIN_SRC_SEL),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_MUX", "Second Source Select", SND_SOC_NOPM, 0, 0, &SOC_MUX_SECOND_SRC_SEL),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_MUX", "Rear Speaker Source", SND_SOC_NOPM, 0, 0, &SOC_MUX_REAR_SPKR_SRC),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_SWITCH", "Mix Enable", SND_SOC_NOPM, 0, 0, &SOC_MIX_ENABLE_SWITCH_CONTROLS[0]),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_MIXER_NAMED_CTL", "LF Output Mixer", SND_SOC_NOPM, 0, 0, &SOC_MIXER_LF_OUTPUT_CONTROLS[0], SOC_MIXER_LF_OUTPUT_CONTROLS.len()),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_MIXER_NAMED_CTL", "RF Output Mixer", SND_SOC_NOPM, 0, 0, &SOC_MIXER_RF_OUTPUT_CONTROLS[0], SOC_MIXER_RF_OUTPUT_CONTROLS.len()),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_SWITCH", "Subwoofer Enable", SND_SOC_NOPM, 0, 0, &SOC_SUB_ENABLE_SWITCH_CONTROLS[0]),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_OUTPUT", "OUTLF"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_OUTPUT", "OUTRF"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_OUTPUT", "OUTLR"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_OUTPUT", "OUTRR"),
    snd_soc_dapm_widget_init!("SND_SOC_DAPM_OUTPUT", "OUTSW"),
];

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route { sink: cstr!($sink), control: ptr::null(), source: cstr!($source) }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route { sink: cstr!($sink), control: cstr!($control), source: cstr!($source) }
    };
}

static TDA7419_DAPM_ROUTES: [snd_soc_dapm_route; 33] = [
    route!("Main Source Select", "SE3", "SE3L"),
    route!("Main Source Select", "SE3", "SE3R"),
    route!("Main Source Select", "SE2", "SE2L"),
    route!("Main Source Select", "SE2", "SE2R"),
    route!("Main Source Select", "SE1", "SE1L"),
    route!("Main Source Select", "SE1", "SE1R"),
    route!("Main Source Select", "SE", "DIFFL"),
    route!("Main Source Select", "SE", "DIFFR"),
    route!("Main Source Select", "QD", "DIFFL"),
    route!("Main Source Select", "QD", "DIFFR"),
    route!("Second Source Select", "SE3", "SE3L"),
    route!("Second Source Select", "SE3", "SE3R"),
    route!("Second Source Select", "SE2", "SE2L"),
    route!("Second Source Select", "SE2", "SE2R"),
    route!("Second Source Select", "SE1", "SE1L"),
    route!("Second Source Select", "SE1", "SE1R"),
    route!("Second Source Select", "SE", "DIFFL"),
    route!("Second Source Select", "SE", "DIFFR"),
    route!("Second Source Select", "QD", "DIFFL"),
    route!("Second Source Select", "QD", "DIFFR"),
    route!("Rear Speaker Source", "Main", "Main Source Select"),
    route!("Rear Speaker Source", "Second", "Second Source Select"),
    route!("Subwoofer Enable", "Switch", "Main Source Select"),
    route!("Mix Enable", "Switch", "MIX"),
    route!("LF Output Mixer", NULL, "Main Source Select"),
    route!("LF Output Mixer", "Mix to LF Speaker Switch", "Mix Enable"),
    route!("RF Output Mixer", NULL, "Main Source Select"),
    route!("RF Output Mixer", "Mix to RF Speaker Switch", "Mix Enable"),
    route!("OUTLF", NULL, "LF Output Mixer"),
    route!("OUTRF", NULL, "RF Output Mixer"),
    route!("OUTLR", NULL, "Rear Speaker Source"),
    route!("OUTRR", NULL, "Rear Speaker Source"),
    route!("OUTSW", NULL, "Subwoofer Enable"),
];

static TDA7419_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    name: cstr!("tda7419"),
    controls: unsafe { TDA7419_CONTROLS.as_mut_ptr() },
    num_controls: unsafe { TDA7419_CONTROLS.len() as c_uint },
    dapm_widgets: TDA7419_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: TDA7419_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: TDA7419_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: TDA7419_DAPM_ROUTES.len() as c_uint,
};

unsafe extern "C" fn tda7419_probe(i2c: *mut i2c_client) -> c_int {
    let tda7419: *mut tda7419_data;
    let mut i: c_int;
    let mut ret: c_int;

    tda7419 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<tda7419_data>(),
        GFP_KERNEL,
    ) as *mut tda7419_data;
    if tda7419.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, tda7419 as *mut c_void);

    (*tda7419).regmap = devm_regmap_init_i2c(i2c, &TDA7419_REGMAP_CONFIG);
    if IS_ERR((*tda7419).regmap as *const c_void) {
        ret = PTR_ERR((*tda7419).regmap as *const c_void);
        dev_err(
            &mut (*i2c).dev,
            cstr!("error initializing regmap: %d\n"),
            ret,
        );
        return ret;
    }

    /*
     * Reset registers to power-on defaults. The part does not provide a
     * soft-reset function and the registers are not readable. This ensures
     * that the cache matches register contents even if the registers have
     * been previously initialized and not power cycled before probe.
     */
    i = 0;
    while i < TDA7419_REGMAP_DEFAULTS.len() as c_int {
        regmap_write(
            (*tda7419).regmap,
            TDA7419_REGMAP_DEFAULTS[i as usize].reg,
            TDA7419_REGMAP_DEFAULTS[i as usize].def,
        );
        i += 1;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &TDA7419_COMPONENT_DRIVER,
        ptr::null(),
        0,
    );
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            cstr!("error registering component: %d\n"),
            ret,
        );
    }

    ret
}

static TDA7419_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b't' as c_char, b'd' as c_char, b'a' as c_char, b'7' as c_char,
            b'4' as c_char, b'1' as c_char, b'9' as c_char, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, tda7419_i2c_id); */

static TDA7419_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr!("st,tda7419"),
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, tda7419_of_match); */

static mut TDA7419_DRIVER: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: cstr!("tda7419"),
        of_match_table: TDA7419_OF_MATCH.as_ptr(),
    },
    probe: Some(tda7419_probe),
    id_table: TDA7419_I2C_ID.as_ptr(),
};

/* module_i2c_driver(tda7419_driver); */

/* MODULE_AUTHOR("Matt Porter <mporter@konsulko.com>"); */
/* MODULE_DESCRIPTION("TDA7419 audio processor driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
