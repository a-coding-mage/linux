// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u64 = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ETIMEDOUT: c_int = 110;
const EIO: c_int = 5;
const ENODEV: c_int = 19;

const GFP_KERNEL: c_uint = 0;
const BITS_PER_BYTE: c_uint = 8;
const UINT_MAX: c_uint = c_uint::MAX;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut c_void }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong, pub id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: *const c_char }

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_int; 128],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_uint,
    pub mask: c_uint,
    pub on_val: c_int,
    pub off_val: c_int,
    pub event_flags: c_uint,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub priv_: *mut c_void,
    pub dapm: *mut snd_soc_dapm_context,
    pub subseq: c_int,
    pub kcontrol_news: *const snd_kcontrol_new,
    pub num_kcontrols: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn() -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub access: c_uint,
    pub tlv: snd_kcontrol_new_tlv,
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *mut c_uint,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub items: c_uint,
    pub mask: c_uint,
    pub texts: *mut *const c_char,
    pub values: *mut c_uint,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub invert: c_uint,
    pub min: c_int,
    pub max: c_int,
    pub shift: c_uint,
    pub sign_bit: c_uint,
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
    pub id: c_int,
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *mut snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *mut snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct sdca_function_data {
    pub num_entities: c_int,
    pub entities: *mut sdca_entity,
    pub desc: *mut sdca_function_desc,
}

#[repr(C)]
pub struct sdca_function_desc {
    pub adr: c_uint,
}

#[repr(C)]
pub struct sdca_entity {
    pub type_: c_int,
    pub id: c_int,
    pub label: *const c_char,
    pub num_controls: c_int,
    pub controls: *mut sdca_control,
    pub num_sources: c_int,
    pub sources: *mut *mut sdca_entity,
    pub group: *mut sdca_entity,
    pub iot: sdca_iot,
    pub pde: sdca_pde,
    pub ge: sdca_ge,
    pub cs: sdca_cs,
}

#[repr(C)] pub struct sdca_iot { pub clock: *mut sdca_entity, pub is_dataport: bool_ }
#[repr(C)] pub struct sdca_pde { pub num_managed: c_int, pub managed: *mut *mut sdca_entity, pub max_delay: *const sdca_pde_delay, pub num_max_delay: c_int }
#[repr(C)] pub struct sdca_ge { pub num_modes: c_int, pub modes: *mut sdca_ge_mode, pub kctl: *mut snd_kcontrol_new }
#[repr(C)] pub struct sdca_cs { pub max_delay: c_uint }
#[repr(C)] pub struct sdca_ge_mode { pub val: c_int, pub num_controls: c_int, pub controls: *mut sdca_ge_control }
#[repr(C)] pub struct sdca_ge_control { pub id: c_int, pub sel: c_int, pub val: c_int }
#[repr(C)] pub struct sdca_control { pub sel: c_int, pub layers: c_uint, pub has_fixed: bool_, pub mode: c_int, pub type_: c_int, pub nbits: c_uint, pub label: *const c_char, pub cn_list: c_ulong, pub is_volatile: bool_ }
#[repr(C)] pub struct sdca_control_range { pub rows: c_int }
#[repr(C)] pub struct sdca_pde_delay { pub from_ps: c_int, pub to_ps: c_int, pub us: c_int }
#[repr(C)] pub struct sdca_cluster { pub num_channels: c_uint }

unsafe extern "C" {
    fn sdca_selector_find_control(dev: *mut device, entity: *mut sdca_entity, sel: c_int) -> *mut sdca_control;
    fn sdca_control_find_range(dev: *mut device, entity: *mut sdca_entity, control: *mut sdca_control, ncols: c_int, nrows: c_int) -> *mut sdca_control_range;
    fn sdca_selector_find_range(dev: *mut device, entity: *mut sdca_entity, sel: c_int, ncols: c_int, nrows: c_int) -> *mut sdca_control_range;
    fn sdca_range(range: *mut sdca_control_range, col: c_int, row: c_int) -> c_int;
    fn sdca_range_search(range: *mut sdca_control_range, search_col: c_int, search_val: c_int, return_col: c_int) -> c_int;
    fn sdca_find_terminal_name(type_: c_int) -> *const c_char;
    fn sdca_id_find_cluster(dev: *mut device, function: *mut sdca_function_data, id: c_int) -> *mut sdca_cluster;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, gfp: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_int;
    fn snd_soc_volsw_is_stereo(mc: *mut soc_mixer_control) -> bool_;
    fn sign_extend32(value: c_uint, index: c_int) -> c_int;
    fn fsleep(usecs: c_uint);
    fn snd_pcm_hw_constraint_list(runtime: *mut c_void, cond: c_uint, var: c_uint, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    static snd_soc_info_enum_double: Option<unsafe extern "C" fn() -> c_int>;
    static snd_soc_dapm_get_enum_double: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_dapm_put_enum_double: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_info_volsw: Option<unsafe extern "C" fn() -> c_int>;
    static snd_soc_dapm_get_volsw: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_dapm_put_volsw: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_get_volsw: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_put_volsw: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_dapm_info_pin_switch: Option<unsafe extern "C" fn() -> c_int>;
    static snd_soc_dapm_get_component_pin_switch: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
    static snd_soc_dapm_put_component_pin_switch: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>;
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char } }

fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 { c_uint::MAX << l } else { ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1) }
}
fn roundup_pow_of_two(mut v: c_uint) -> c_uint {
    if v <= 1 { return 1; }
    v -= 1; v |= v >> 1; v |= v >> 2; v |= v >> 4; v |= v >> 8; v |= v >> 16; v + 1
}
fn hweight8(v: u8) -> c_uint { v.count_ones() }
fn SDCA_CTL_TYPE(entity_type: c_int, sel: c_int) -> c_int { (entity_type << 16) | sel }
fn SDCA_CTL_TYPE_S(entity_type: c_int, sel: c_int) -> c_int { SDCA_CTL_TYPE(entity_type, sel) }
fn SDW_SDCA_CTL(adr: c_uint, entity: c_int, sel: c_int, cn: c_int) -> c_uint { (adr << 24) | ((entity as c_uint) << 16) | ((sel as c_uint) << 8) | cn as c_uint }
fn SDW_SDCA_CTL_CSEL(sel: c_uint) -> c_uint { sel << 8 }
fn SDW_SDCA_CTL_FUNC(reg: c_uint) -> c_int { ((reg >> 24) & 0xff) as c_int }
fn SDW_SDCA_CTL_ENT(reg: c_uint) -> c_int { ((reg >> 16) & 0xff) as c_int }

const SDCA_ACCESS_LAYER_USER: c_uint = 1 << 0;
const SDCA_ACCESS_LAYER_APPLICATION: c_uint = 1 << 1;
const SDCA_ACCESS_LAYER_DEVICE: c_uint = 1 << 2;
const SDCA_ACCESS_LAYER_CLASS: c_uint = 1 << 3;
const SDCA_ACCESS_MODE_RO: c_int = 1;
const SDCA_ENTITY_TYPE_IT: c_int = 1;
const SDCA_ENTITY_TYPE_OT: c_int = 2;
const SDCA_ENTITY_TYPE_PDE: c_int = 3;
const SDCA_ENTITY_TYPE_GE: c_int = 4;
const SDCA_ENTITY_TYPE_SU: c_int = 5;
const SDCA_ENTITY_TYPE_MU: c_int = 6;
const SDCA_ENTITY_TYPE_CS: c_int = 7;
const SDCA_ENTITY_TYPE_CX: c_int = 8;
const SDCA_ENTITY_TYPE_TG: c_int = 9;
const GE: c_int = SDCA_ENTITY_TYPE_GE;
const FU: c_int = 10;
const DETECTED_MODE: c_int = 1;
const MUTE: c_int = 2;
const SDCA_CTL_GE_DETECTED_MODE: c_int = DETECTED_MODE;
const SDCA_CTL_GE_SELECTED_MODE: c_int = 3;
const SDCA_CTL_SU_SELECTOR: c_int = 4;
const SDCA_CTL_PDE_ACTUAL_PS: c_int = 5;
const SDCA_CTL_PDE_REQUESTED_PS: c_int = 6;
const SDCA_CTL_MU_MIXER: c_int = 7;
const SDCA_CTL_CS_SAMPLERATEINDEX: c_int = 8;
const SDCA_CTL_IT_USAGE: c_int = 9;
const SDCA_CTL_OT_USAGE: c_int = 10;
const SDCA_CTL_IT_CLUSTERINDEX: c_int = 11;
const SDCA_CTL_IT_DATAPORT_SELECTOR: c_int = 12;
const SDCA_CTL_OT_DATAPORT_SELECTOR: c_int = 13;
const SDCA_CTL_DATATYPE_Q7P8DB: c_int = 1;
const SDCA_CTL_DATATYPE_ONEBIT: c_int = 2;
const SDCA_DETECTED_MODE_JACK_UNPLUGGED: c_uint = 0;
const SDCA_DETECTED_MODE_JACK_UNKNOWN: c_uint = 1;
const SDCA_DETECTED_MODE_DETECTION_IN_PROGRESS: c_int = 2;
const SDCA_SELECTED_MODE_NCOLS: c_int = 0;
const SDCA_SELECTED_MODE_TERM_TYPE: c_int = 1;
const SDCA_SELECTED_MODE_INDEX: c_int = 2;
const SDCA_REQUESTED_PS_NCOLS: c_int = 0;
const SDCA_REQUESTED_PS_STATE: c_int = 1;
const SDCA_PDE_PS0: c_uint = 0;
const SDCA_PDE_PS3: c_uint = 3;
const SDCA_VOLUME_LINEAR_NCOLS: c_int = 0;
const SDCA_VOLUME_LINEAR_MIN: c_int = 1;
const SDCA_VOLUME_LINEAR_MAX: c_int = 2;
const SDCA_VOLUME_LINEAR_STEP: c_int = 3;
const SDCA_SAMPLERATEINDEX_NCOLS: c_int = 0;
const SDCA_SAMPLERATEINDEX_RATE: c_int = 1;
const SDCA_SAMPLERATEINDEX_INDEX: c_int = 2;
const SDCA_USAGE_NCOLS: c_int = 0;
const SDCA_USAGE_SAMPLE_RATE: c_int = 1;
const SDCA_USAGE_SAMPLE_WIDTH: c_int = 2;
const SDCA_USAGE_NUMBER: c_int = 3;
const SDCA_CLUSTER_NCOLS: c_int = 0;
const SDCA_CLUSTER_CLUSTERID: c_int = 1;
const SDCA_CLUSTER_BYTEINDEX: c_int = 2;
const SDCA_DATAPORT_SELECTOR_NCOLS: c_int = 0;
const SDCA_DATAPORT_SELECTOR_NROWS: c_int = 0;
const SDCA_MAX_CHANNEL_COUNT: c_uint = 32;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 4;
const SNDRV_CTL_TLVT_DB_MINMAX: c_uint = 0x0004;
const SND_SOC_NOPM: c_uint = c_uint::MAX;
const SND_SOC_DAPM_POST_PMD: c_int = 1;
const SND_SOC_DAPM_POST_PMU: c_int = 2;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 0;
const snd_soc_dapm_aif_in: c_int = 1;
const snd_soc_dapm_aif_out: c_int = 2;
const snd_soc_dapm_mic: c_int = 3;
const snd_soc_dapm_spk: c_int = 4;
const snd_soc_dapm_supply: c_int = 5;
const snd_soc_dapm_mux_named_ctl: c_int = 6;
const snd_soc_dapm_mux: c_int = 7;
const snd_soc_dapm_mixer: c_int = 8;
const snd_soc_dapm_siggen: c_int = 9;
const snd_soc_dapm_pga: c_int = 10;

const SNDRV_PCM_RATE_8000_768000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_5512: c_uint = 1 << 1;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_11025: c_uint = 1 << 3;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 5;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 6;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 7;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 8;
const SNDRV_PCM_RATE_64000: c_uint = 1 << 9;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 10;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 11;
const SNDRV_PCM_RATE_176400: c_uint = 1 << 12;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 13;
const SNDRV_PCM_RATE_352800: c_uint = 1 << 14;
const SNDRV_PCM_RATE_384000: c_uint = 1 << 15;
const SNDRV_PCM_RATE_705600: c_uint = 1 << 16;
const SNDRV_PCM_RATE_768000: c_uint = 1 << 17;
const SNDRV_PCM_RATE_12000: c_uint = 1 << 18;
const SNDRV_PCM_RATE_24000: c_uint = 1 << 19;
const SNDRV_PCM_RATE_128000: c_uint = 1 << 20;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S20_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 4;

unsafe fn exported_control(entity: *mut sdca_entity, control: *mut sdca_control) -> bool_ {
    match SDCA_CTL_TYPE((*entity).type_, (*control).sel) {
        x if x == SDCA_CTL_TYPE_S(GE, DETECTED_MODE) => true,
        _ => ((*control).layers & (SDCA_ACCESS_LAYER_USER | SDCA_ACCESS_LAYER_APPLICATION)) != 0,
    }
}

unsafe fn readonly_control(control: *mut sdca_control) -> bool_ {
    (*control).has_fixed || (*control).mode == SDCA_ACCESS_MODE_RO
}

unsafe fn ge_count_routes(entity: *mut sdca_entity) -> c_int {
    let mut count = 0;
    for i in 0..(*entity).ge.num_modes {
        let mode = (*entity).ge.modes.add(i as usize);
        for j in 0..(*mode).num_controls {
            let affected = (*mode).controls.add(j as usize);
            if (*affected).sel != SDCA_CTL_SU_SELECTOR || (*affected).val != 0 {
                count += 1;
            }
        }
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_count_component(dev: *mut device, function: *mut sdca_function_data, num_widgets: *mut c_int, num_routes: *mut c_int, num_controls: *mut c_int, num_dais: *mut c_int) -> c_int {
    *num_widgets = (*function).num_entities - 1;
    *num_routes = 0; *num_controls = 0; *num_dais = 0;
    for i in 0..((*function).num_entities - 1) {
        let entity = (*function).entities.add(i as usize);
        let mut skip_primary_routes = false;
        match (*entity).type_ {
            SDCA_ENTITY_TYPE_IT | SDCA_ENTITY_TYPE_OT => {
                *num_routes += (!(*entity).iot.clock.is_null()) as c_int;
                *num_routes += (*entity).iot.is_dataport as c_int;
                *num_controls += (!(*entity).iot.is_dataport) as c_int;
                *num_dais += (*entity).iot.is_dataport as c_int;
            }
            SDCA_ENTITY_TYPE_PDE => *num_routes += (*entity).pde.num_managed,
            SDCA_ENTITY_TYPE_GE => { *num_routes += ge_count_routes(entity); skip_primary_routes = true; }
            SDCA_ENTITY_TYPE_SU => {
                let control = sdca_selector_find_control(dev, entity, SDCA_CTL_SU_SELECTOR);
                if control.is_null() { return -EINVAL; }
                skip_primary_routes = (*control).layers == SDCA_ACCESS_LAYER_DEVICE;
            }
            _ => {}
        }
        if !(*entity).group.is_null() { *num_routes += 1; }
        if !skip_primary_routes { *num_routes += (*entity).num_sources; }
        for j in 0..(*entity).num_controls {
            if exported_control(entity, (*entity).controls.add(j as usize)) { *num_controls += 1; }
        }
    }
    0
}

unsafe extern "C" fn ge_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let dev = (*component).dev;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let mut reg = (*e).reg;
    if *item.add(0) >= (*e).items { return -EINVAL; }
    reg &= !SDW_SDCA_CTL_CSEL(0x3F);
    reg |= SDW_SDCA_CTL_CSEL(SDCA_CTL_GE_DETECTED_MODE as c_uint);
    let mut ret = pm_runtime_resume_and_get(dev);
    if ret < 0 {
        dev_err(dev, cstr!("failed to resume writing %s: %d\n"), (*kcontrol).id.name, ret);
        return ret;
    }
    ret = snd_soc_component_read(component, reg);
    pm_runtime_put(dev);
    if ret < 0 { return ret; } else if ret <= SDCA_DETECTED_MODE_DETECTION_IN_PROGRESS { return -EBUSY; }
    ret = snd_soc_enum_item_to_val(e, *item.add(0));
    if ret <= SDCA_DETECTED_MODE_DETECTION_IN_PROGRESS { return -EINVAL; }
    snd_soc_dapm_put_enum_double.unwrap()(kcontrol, ucontrol)
}

unsafe fn entity_early_parse_ge(dev: *mut device, function: *mut sdca_function_data, entity: *mut sdca_entity) -> c_int {
    let control = sdca_selector_find_control(dev, entity, SDCA_CTL_GE_SELECTED_MODE);
    if control.is_null() { return -EINVAL; }
    if (*control).layers != SDCA_ACCESS_LAYER_CLASS {
        dev_warn(dev, cstr!("%s: unexpected access layer: %x\n"), (*entity).label, (*control).layers);
    }
    let range = sdca_control_find_range(dev, entity, control, SDCA_SELECTED_MODE_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    let control_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s %s"), (*entity).label, (*control).label);
    if control_name.is_null() { return -ENOMEM; }
    let kctl = devm_kzalloc(dev, core::mem::size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if kctl.is_null() { return -ENOMEM; }
    let soc_enum = devm_kzalloc(dev, core::mem::size_of::<soc_enum>(), GFP_KERNEL) as *mut soc_enum;
    if soc_enum.is_null() { return -ENOMEM; }
    let texts = devm_kcalloc(dev, ((*range).rows + 3) as usize, core::mem::size_of::<*const c_char>(), GFP_KERNEL) as *mut *const c_char;
    if texts.is_null() { return -ENOMEM; }
    let values = devm_kcalloc(dev, ((*range).rows + 3) as usize, core::mem::size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
    if values.is_null() { return -ENOMEM; }
    *texts.add(0) = cstr!("Jack Unplugged"); *texts.add(1) = cstr!("Jack Unknown"); *texts.add(2) = cstr!("Detection in Progress");
    *values.add(0) = SDCA_DETECTED_MODE_JACK_UNPLUGGED; *values.add(1) = SDCA_DETECTED_MODE_JACK_UNKNOWN; *values.add(2) = SDCA_DETECTED_MODE_DETECTION_IN_PROGRESS as c_uint;
    for i in 0..(*range).rows {
        let type_ = sdca_range(range, SDCA_SELECTED_MODE_TERM_TYPE, i);
        *values.add((i + 3) as usize) = sdca_range(range, SDCA_SELECTED_MODE_INDEX, i) as c_uint;
        *texts.add((i + 3) as usize) = sdca_find_terminal_name(type_);
        if (*texts.add((i + 3) as usize)).is_null() {
            dev_err(dev, cstr!("%s: unrecognised terminal type: %#x\n"), (*entity).label, type_);
            return -EINVAL;
        }
    }
    (*soc_enum).reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, 0);
    (*soc_enum).items = ((*range).rows + 3) as c_uint;
    (*soc_enum).mask = roundup_pow_of_two((*soc_enum).items) - 1;
    (*soc_enum).texts = texts;
    (*soc_enum).values = values;
    (*kctl).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*kctl).name = control_name;
    (*kctl).info = snd_soc_info_enum_double;
    (*kctl).get = snd_soc_dapm_get_enum_double;
    (*kctl).put = Some(ge_put_enum_double);
    (*kctl).private_value = soc_enum as c_ulong;
    (*entity).ge.kctl = kctl;
    0
}

unsafe fn add_route(route: *mut *mut snd_soc_dapm_route, sink: *const c_char, control: *const c_char, source: *const c_char) {
    (**route).sink = sink; (**route).control = control; (**route).source = source; *route = (*route).add(1);
}

unsafe fn entity_parse_simple(_: *mut device, _: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route, id: c_int) -> c_int {
    (**widget).id = id; *widget = (*widget).add(1);
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, ptr::null(), (*(*entity).sources.add(i as usize)).label); }
    0
}

unsafe fn entity_parse_it(dev: *mut device, _: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    if (*entity).iot.is_dataport {
        let aif_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s %s"), (*entity).label, cstr!("Playback"));
        if aif_name.is_null() { return -ENOMEM; }
        (**widget).id = snd_soc_dapm_aif_in;
        add_route(route, (*entity).label, ptr::null(), aif_name);
    } else { (**widget).id = snd_soc_dapm_mic; }
    if !(*entity).iot.clock.is_null() { add_route(route, (*entity).label, ptr::null(), (*(*entity).iot.clock).label); }
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, ptr::null(), (*(*entity).sources.add(i as usize)).label); }
    *widget = (*widget).add(1); 0
}

unsafe fn entity_parse_ot(dev: *mut device, _: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    if (*entity).iot.is_dataport {
        let aif_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s %s"), (*entity).label, cstr!("Capture"));
        if aif_name.is_null() { return -ENOMEM; }
        (**widget).id = snd_soc_dapm_aif_out;
        add_route(route, aif_name, ptr::null(), (*entity).label);
    } else { (**widget).id = snd_soc_dapm_spk; }
    if !(*entity).iot.clock.is_null() { add_route(route, (*entity).label, ptr::null(), (*(*entity).iot.clock).label); }
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, ptr::null(), (*(*entity).sources.add(i as usize)).label); }
    *widget = (*widget).add(1); 0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_pde_poll_actual_ps(regmap: *mut regmap, function_id: c_int, entity_id: c_int, from_ps: c_int, to_ps: c_int, pde_delays: *const sdca_pde_delay, num_delays: c_int) -> c_int {
    const POLLS: c_int = 100;
    const DEFAULT_POLL_US: c_int = 1000;
    let mut val: c_uint = 0;
    let mut poll_us = DEFAULT_POLL_US;
    if !pde_delays.is_null() && num_delays > 0 {
        for i in 0..num_delays {
            let delay = pde_delays.add(i as usize);
            if (*delay).from_ps == from_ps && (*delay).to_ps == to_ps { poll_us = (*delay).us / POLLS; break; }
        }
    }
    let reg = SDW_SDCA_CTL(function_id as c_uint, entity_id, SDCA_CTL_PDE_ACTUAL_PS, 0);
    for i in 0..POLLS {
        if i != 0 { fsleep(poll_us as c_uint); }
        let ret = regmap_read(regmap, reg, &mut val);
        if ret != 0 { return ret; } else if val == to_ps as c_uint { return 0; }
    }
    -ETIMEDOUT
}

unsafe extern "C" fn entity_pde_event(widget: *mut snd_soc_dapm_widget, _: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let entity = (*widget).priv_ as *mut sdca_entity;
    if component.is_null() { return -EIO; }
    let (from, to) = match event {
        SND_SOC_DAPM_POST_PMD => ((*widget).on_val, (*widget).off_val),
        SND_SOC_DAPM_POST_PMU => ((*widget).off_val, (*widget).on_val),
        _ => return 0,
    };
    let ret = sdca_asoc_pde_poll_actual_ps((*component).regmap, SDW_SDCA_CTL_FUNC((*widget).reg), SDW_SDCA_CTL_ENT((*widget).reg), from, to, (*entity).pde.max_delay, (*entity).pde.num_max_delay);
    if ret != 0 { dev_err((*component).dev, cstr!("%s: pde transition %x -> %x failed: %d\n"), (*entity).label, from, to, ret); }
    ret
}

unsafe fn entity_parse_pde(dev: *mut device, function: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    let target = (1u32 << SDCA_PDE_PS0) | (1u32 << SDCA_PDE_PS3);
    let control = sdca_selector_find_control(dev, entity, SDCA_CTL_PDE_REQUESTED_PS);
    if control.is_null() { return -EINVAL; }
    if (*control).layers != SDCA_ACCESS_LAYER_CLASS { dev_warn(dev, cstr!("%s: unexpected access layer: %x\n"), (*entity).label, (*control).layers); }
    let range = sdca_control_find_range(dev, entity, control, SDCA_REQUESTED_PS_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    let mut mask = 0u32;
    for i in 0..(*range).rows { mask |= 1u32 << sdca_range(range, SDCA_REQUESTED_PS_STATE, i); }
    if (mask & target) != target { dev_err(dev, cstr!("%s: power control missing states\n"), (*entity).label); return -EINVAL; }
    (**widget).id = snd_soc_dapm_supply;
    (**widget).reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, 0);
    (**widget).mask = GENMASK((*control).nbits - 1, 0);
    (**widget).on_val = SDCA_PDE_PS0 as c_int; (**widget).off_val = SDCA_PDE_PS3 as c_int;
    (**widget).event_flags = (SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD) as c_uint;
    (**widget).event = Some(entity_pde_event); (**widget).priv_ = entity as *mut c_void; *widget = (*widget).add(1);
    for i in 0..(*entity).pde.num_managed { add_route(route, (*(*entity).pde.managed.add(i as usize)).label, ptr::null(), (*entity).label); }
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, ptr::null(), (*(*entity).sources.add(i as usize)).label); }
    0
}

unsafe fn entity_parse_su_device(dev: *mut device, _: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    if (*entity).group.is_null() { dev_err(dev, cstr!("%s: device selector unit missing group\n"), (*entity).label); return -EINVAL; }
    let range = sdca_selector_find_range(dev, (*entity).group, SDCA_CTL_GE_SELECTED_MODE, SDCA_SELECTED_MODE_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    (**widget).id = snd_soc_dapm_mux_named_ctl; (**widget).kcontrol_news = (*(*entity).group).ge.kctl; (**widget).num_kcontrols = 1; *widget = (*widget).add(1);
    for i in 0..(*(*entity).group).ge.num_modes {
        let mode = (*(*entity).group).ge.modes.add(i as usize);
        for j in 0..(*mode).num_controls {
            let affected = (*mode).controls.add(j as usize);
            if (*affected).id != (*entity).id || (*affected).sel != SDCA_CTL_SU_SELECTOR || (*affected).val == 0 { continue; }
            if (*affected).val - 1 >= (*entity).num_sources { dev_err(dev, cstr!("%s: bad control value: %#x\n"), (*entity).label, (*affected).val); return -EINVAL; }
            let term = sdca_range_search(range, SDCA_SELECTED_MODE_INDEX, (*mode).val, SDCA_SELECTED_MODE_TERM_TYPE);
            if term == 0 { dev_err(dev, cstr!("%s: mode not found: %#x\n"), (*entity).label, (*mode).val); return -EINVAL; }
            add_route(route, (*entity).label, sdca_find_terminal_name(term), (*(*entity).sources.add(((*affected).val - 1) as usize)).label);
        }
    }
    0
}

unsafe fn entity_parse_su_class(dev: *mut device, function: *mut sdca_function_data, entity: *mut sdca_entity, control: *mut sdca_control, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    let kctl = devm_kzalloc(dev, core::mem::size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if kctl.is_null() { return -ENOMEM; }
    let soc_enum = devm_kzalloc(dev, core::mem::size_of::<soc_enum>(), GFP_KERNEL) as *mut soc_enum;
    if soc_enum.is_null() { return -ENOMEM; }
    let texts = devm_kcalloc(dev, ((*entity).num_sources + 1) as usize, core::mem::size_of::<*const c_char>(), GFP_KERNEL) as *mut *const c_char;
    if texts.is_null() { return -ENOMEM; }
    *texts.add(0) = cstr!("No Signal");
    for i in 0..(*entity).num_sources { *texts.add((i + 1) as usize) = (*(*entity).sources.add(i as usize)).label; }
    (*soc_enum).reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, 0);
    (*soc_enum).items = ((*entity).num_sources + 1) as c_uint; (*soc_enum).mask = roundup_pow_of_two((*soc_enum).items) - 1; (*soc_enum).texts = texts;
    (*kctl).iface = SNDRV_CTL_ELEM_IFACE_MIXER; (*kctl).name = cstr!("Route"); (*kctl).info = snd_soc_info_enum_double; (*kctl).get = snd_soc_dapm_get_enum_double; (*kctl).put = snd_soc_dapm_put_enum_double; (*kctl).private_value = soc_enum as c_ulong;
    (**widget).id = snd_soc_dapm_mux; (**widget).kcontrol_news = kctl; (**widget).num_kcontrols = 1; *widget = (*widget).add(1);
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, *texts.add((i + 1) as usize), (*(*entity).sources.add(i as usize)).label); }
    0
}

unsafe fn entity_parse_su(dev: *mut device, function: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    if (*entity).num_sources == 0 { dev_err(dev, cstr!("%s: selector with no inputs\n"), (*entity).label); return -EINVAL; }
    let control = sdca_selector_find_control(dev, entity, SDCA_CTL_SU_SELECTOR);
    if control.is_null() { return -EINVAL; }
    if (*control).layers == SDCA_ACCESS_LAYER_DEVICE { return entity_parse_su_device(dev, function, entity, widget, route); }
    if (*control).layers != SDCA_ACCESS_LAYER_CLASS { dev_warn(dev, cstr!("%s: unexpected access layer: %x\n"), (*entity).label, (*control).layers); }
    entity_parse_su_class(dev, function, entity, control, widget, route)
}

unsafe fn entity_parse_mu(dev: *mut device, _: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    if (*entity).num_sources == 0 { dev_err(dev, cstr!("%s: selector 1 or more inputs\n"), (*entity).label); return -EINVAL; }
    let control = sdca_selector_find_control(dev, entity, SDCA_CTL_MU_MIXER);
    if control.is_null() { return -EINVAL; }
    if (*control).layers != SDCA_ACCESS_LAYER_CLASS { dev_warn(dev, cstr!("%s: unexpected access layer: %x\n"), (*entity).label, (*control).layers); }
    let kctl = devm_kcalloc(dev, (*entity).num_sources as usize, core::mem::size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if kctl.is_null() { return -ENOMEM; }
    for i in 0..(*entity).num_sources {
        let control_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s %d"), (*control).label, i + 1);
        if control_name.is_null() { return -ENOMEM; }
        let mc = devm_kzalloc(dev, core::mem::size_of::<soc_mixer_control>(), GFP_KERNEL) as *mut soc_mixer_control;
        if mc.is_null() { return -ENOMEM; }
        (*mc).reg = SND_SOC_NOPM; (*mc).rreg = SND_SOC_NOPM; (*mc).invert = 1; (*mc).min = 0; (*mc).max = 1;
        (*kctl.add(i as usize)).name = control_name; (*kctl.add(i as usize)).private_value = mc as c_ulong; (*kctl.add(i as usize)).iface = SNDRV_CTL_ELEM_IFACE_MIXER; (*kctl.add(i as usize)).info = snd_soc_info_volsw; (*kctl.add(i as usize)).get = snd_soc_dapm_get_volsw; (*kctl.add(i as usize)).put = snd_soc_dapm_put_volsw;
    }
    (**widget).id = snd_soc_dapm_mixer; (**widget).kcontrol_news = kctl; (**widget).num_kcontrols = (*entity).num_sources; *widget = (*widget).add(1);
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, (*kctl.add(i as usize)).name, (*(*entity).sources.add(i as usize)).label); }
    0
}

unsafe extern "C" fn entity_cs_event(widget: *mut snd_soc_dapm_widget, _: *mut snd_kcontrol, _: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*widget).dapm);
    let entity = (*widget).priv_ as *mut sdca_entity;
    if component.is_null() { return -EIO; }
    if (*entity).cs.max_delay != 0 { fsleep((*entity).cs.max_delay); }
    0
}

unsafe fn entity_parse_cs(_: *mut device, _: *mut sdca_function_data, entity: *mut sdca_entity, widget: *mut *mut snd_soc_dapm_widget, route: *mut *mut snd_soc_dapm_route) -> c_int {
    (**widget).id = snd_soc_dapm_supply; (**widget).subseq = 1; (**widget).event_flags = SND_SOC_DAPM_POST_PMU as c_uint; (**widget).event = Some(entity_cs_event); (**widget).priv_ = entity as *mut c_void; *widget = (*widget).add(1);
    for i in 0..(*entity).num_sources { add_route(route, (*entity).label, ptr::null(), (*(*entity).sources.add(i as usize)).label); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_populate_dapm(dev: *mut device, function: *mut sdca_function_data, mut widget: *mut snd_soc_dapm_widget, mut route: *mut snd_soc_dapm_route) -> c_int {
    for i in 0..((*function).num_entities - 1) {
        let entity = (*function).entities.add(i as usize);
        if (*entity).type_ == SDCA_ENTITY_TYPE_GE {
            let ret = entity_early_parse_ge(dev, function, entity);
            if ret != 0 { return ret; }
        }
    }
    for i in 0..((*function).num_entities - 1) {
        let entity = (*function).entities.add(i as usize);
        (*widget).name = (*entity).label; (*widget).reg = SND_SOC_NOPM;
        let ret = match (*entity).type_ {
            SDCA_ENTITY_TYPE_IT => entity_parse_it(dev, function, entity, &mut widget, &mut route),
            SDCA_ENTITY_TYPE_OT => entity_parse_ot(dev, function, entity, &mut widget, &mut route),
            SDCA_ENTITY_TYPE_PDE => entity_parse_pde(dev, function, entity, &mut widget, &mut route),
            SDCA_ENTITY_TYPE_SU => entity_parse_su(dev, function, entity, &mut widget, &mut route),
            SDCA_ENTITY_TYPE_MU => entity_parse_mu(dev, function, entity, &mut widget, &mut route),
            SDCA_ENTITY_TYPE_CS => entity_parse_cs(dev, function, entity, &mut widget, &mut route),
            SDCA_ENTITY_TYPE_CX => { dev_warn(dev, cstr!("%s: clock selectors not fully supported yet\n"), (*entity).label); entity_parse_simple(dev, function, entity, &mut widget, &mut route, snd_soc_dapm_supply) }
            SDCA_ENTITY_TYPE_TG => entity_parse_simple(dev, function, entity, &mut widget, &mut route, snd_soc_dapm_siggen),
            SDCA_ENTITY_TYPE_GE => entity_parse_simple(dev, function, entity, &mut widget, &mut route, snd_soc_dapm_supply),
            _ => entity_parse_simple(dev, function, entity, &mut widget, &mut route, snd_soc_dapm_pga),
        };
        if ret != 0 { return ret; }
        if !(*entity).group.is_null() { add_route(&mut route, (*entity).label, ptr::null(), (*(*entity).group).label); }
    }
    0
}

unsafe fn q78_write(component: *mut snd_soc_component, mc: *mut soc_mixer_control, reg: c_uint, val: c_int) -> c_int {
    let mask = GENMASK((*mc).sign_bit, 0);
    if val < 0 || val > (*mc).max - (*mc).min { return -EINVAL; }
    let reg_val = ((val + (*mc).min) as c_uint).wrapping_mul((*mc).shift);
    snd_soc_component_update_bits(component, reg, mask, reg_val)
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_q78_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let ret = q78_write(component, mc, (*mc).reg, (*ucontrol).value.integer.value[0]);
    if ret < 0 { return ret; }
    if snd_soc_volsw_is_stereo(mc) {
        let err = q78_write(component, mc, (*mc).rreg, (*ucontrol).value.integer.value[1]);
        if err != 0 { return err; }
    }
    ret
}

unsafe fn q78_read(component: *mut snd_soc_component, mc: *mut soc_mixer_control, reg: c_uint) -> c_int {
    let reg_val = snd_soc_component_read(component, reg) as c_uint;
    let val = sign_extend32(reg_val, (*mc).sign_bit as c_int) / ((*mc).shift as c_int) - (*mc).min;
    val & GENMASK((*mc).sign_bit, 0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_q78_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = q78_read(component, mc, (*mc).reg);
    if snd_soc_volsw_is_stereo(mc) { (*ucontrol).value.integer.value[1] = q78_read(component, mc, (*mc).rreg); }
    0
}

unsafe fn control_limit_kctl(dev: *mut device, entity: *mut sdca_entity, control: *mut sdca_control, kctl: *mut snd_kcontrol_new) -> c_int {
    let mc = (*kctl).private_value as *mut soc_mixer_control;
    if (*control).type_ != SDCA_CTL_DATATYPE_Q7P8DB { return 0; }
    let range = sdca_control_find_range(dev, entity, control, SDCA_VOLUME_LINEAR_NCOLS, 1);
    if range.is_null() { return -EINVAL; }
    let step = sdca_range(range, SDCA_VOLUME_LINEAR_STEP, 0);
    let min = sign_extend32(sdca_range(range, SDCA_VOLUME_LINEAR_MIN, 0) as c_uint, ((*control).nbits - 1) as c_int);
    let max = sign_extend32(sdca_range(range, SDCA_VOLUME_LINEAR_MAX, 0) as c_uint, ((*control).nbits - 1) as c_int);
    let tlv = devm_kcalloc(dev, 4, core::mem::size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
    if tlv.is_null() { return -ENOMEM; }
    *tlv.add(0) = SNDRV_CTL_TLVT_DB_MINMAX; *tlv.add(1) = (2 * core::mem::size_of::<c_uint>()) as c_uint; *tlv.add(2) = ((min * 100) >> 8) as c_uint; *tlv.add(3) = ((max * 100) >> 8) as c_uint;
    (*mc).min = min / step; (*mc).max = max / step; (*mc).shift = step as c_uint; (*mc).sign_bit = 15;
    (*kctl).tlv.p = tlv; (*kctl).access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ; (*kctl).get = Some(sdca_asoc_q78_get_volsw); (*kctl).put = Some(sdca_asoc_q78_put_volsw);
    0
}

unsafe extern "C" fn volatile_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol); let dev = (*component).dev;
    let ret = pm_runtime_resume_and_get(dev);
    if ret < 0 { dev_err(dev, cstr!("failed to resume reading %s: %d\n"), (*kcontrol).id.name, ret); return ret; }
    let ret = snd_soc_get_volsw.unwrap()(kcontrol, ucontrol); pm_runtime_put(dev); ret
}

unsafe extern "C" fn volatile_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol); let dev = (*component).dev;
    let ret = pm_runtime_resume_and_get(dev);
    if ret < 0 { dev_err(dev, cstr!("failed to resume writing %s: %d\n"), (*kcontrol).id.name, ret); return ret; }
    let ret = snd_soc_put_volsw.unwrap()(kcontrol, ucontrol); pm_runtime_put(dev); ret
}

unsafe fn populate_control(dev: *mut device, function: *mut sdca_function_data, entity: *mut sdca_entity, control: *mut sdca_control, kctl: *mut *mut snd_kcontrol_new) -> c_int {
    if !exported_control(entity, control) { return 0; }
    let control_suffix = if (*control).type_ == SDCA_CTL_DATATYPE_ONEBIT { cstr!(" Switch") } else { cstr!("") };
    let control_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s %s%s"), (*entity).label, (*control).label, control_suffix);
    if control_name.is_null() { return -ENOMEM; }
    let mc = devm_kzalloc(dev, core::mem::size_of::<soc_mixer_control>(), GFP_KERNEL) as *mut soc_mixer_control;
    if mc.is_null() { return -ENOMEM; }
    let mut index = 0;
    for cn in 0..(core::mem::size_of::<c_ulong>() * BITS_PER_BYTE as usize) {
        if ((*control).cn_list & (1usize << cn) as c_ulong) == 0 { continue; }
        match index {
            0 => { (*mc).reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, cn as c_int); (*mc).rreg = (*mc).reg; }
            1 => { (*mc).rreg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, (*control).sel, cn as c_int); }
            _ => { dev_err(dev, cstr!("%s: %s: only mono/stereo controls supported\n"), (*entity).label, (*control).label); return -EINVAL; }
        }
        index += 1;
    }
    (*mc).min = 0; (*mc).max = (((1u64 << (*control).nbits) - 1).min(c_int::MAX as u64)) as c_int;
    if SDCA_CTL_TYPE((*entity).type_, (*control).sel) == SDCA_CTL_TYPE_S(FU, MUTE) { (*mc).invert = 1; }
    (**kctl).name = control_name; (**kctl).private_value = mc as c_ulong; (**kctl).iface = SNDRV_CTL_ELEM_IFACE_MIXER; (**kctl).info = snd_soc_info_volsw;
    if (*control).is_volatile { (**kctl).get = Some(volatile_get_volsw); (**kctl).put = Some(volatile_put_volsw); } else { (**kctl).get = snd_soc_get_volsw; (**kctl).put = snd_soc_put_volsw; }
    (**kctl).access = if readonly_control(control) { SNDRV_CTL_ELEM_ACCESS_READ } else { SNDRV_CTL_ELEM_ACCESS_READWRITE };
    let ret = control_limit_kctl(dev, entity, control, *kctl);
    if ret != 0 { return ret; }
    *kctl = (*kctl).add(1); 0
}

unsafe fn populate_pin_switch(dev: *mut device, entity: *mut sdca_entity, kctl: *mut *mut snd_kcontrol_new) -> c_int {
    let control_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s Switch"), (*entity).label);
    if control_name.is_null() { return -ENOMEM; }
    (**kctl).name = control_name; (**kctl).private_value = (*entity).label as c_ulong; (**kctl).iface = SNDRV_CTL_ELEM_IFACE_MIXER; (**kctl).info = snd_soc_dapm_info_pin_switch; (**kctl).get = snd_soc_dapm_get_component_pin_switch; (**kctl).put = snd_soc_dapm_put_component_pin_switch;
    *kctl = (*kctl).add(1); 0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_populate_controls(dev: *mut device, function: *mut sdca_function_data, mut kctl: *mut snd_kcontrol_new) -> c_int {
    for i in 0..(*function).num_entities {
        let entity = (*function).entities.add(i as usize);
        match (*entity).type_ {
            SDCA_ENTITY_TYPE_IT | SDCA_ENTITY_TYPE_OT if !(*entity).iot.is_dataport => {
                let ret = populate_pin_switch(dev, entity, &mut kctl);
                if ret != 0 { return ret; }
            }
            _ => {}
        }
        for j in 0..(*entity).num_controls {
            let ret = populate_control(dev, function, entity, (*entity).controls.add(j as usize), &mut kctl);
            if ret != 0 { return ret; }
        }
    }
    0
}

fn rate_find_mask(rate: c_uint) -> c_uint {
    match rate {
        0 => SNDRV_PCM_RATE_8000_768000, 5512 => SNDRV_PCM_RATE_5512, 8000 => SNDRV_PCM_RATE_8000, 11025 => SNDRV_PCM_RATE_11025, 16000 => SNDRV_PCM_RATE_16000, 22050 => SNDRV_PCM_RATE_22050, 32000 => SNDRV_PCM_RATE_32000, 44100 => SNDRV_PCM_RATE_44100, 48000 => SNDRV_PCM_RATE_48000, 64000 => SNDRV_PCM_RATE_64000, 88200 => SNDRV_PCM_RATE_88200, 96000 => SNDRV_PCM_RATE_96000, 176400 => SNDRV_PCM_RATE_176400, 192000 => SNDRV_PCM_RATE_192000, 352800 => SNDRV_PCM_RATE_352800, 384000 => SNDRV_PCM_RATE_384000, 705600 => SNDRV_PCM_RATE_705600, 768000 => SNDRV_PCM_RATE_768000, 12000 => SNDRV_PCM_RATE_12000, 24000 => SNDRV_PCM_RATE_24000, 128000 => SNDRV_PCM_RATE_128000, _ => 0
    }
}

fn width_find_mask(bits: c_uint) -> u64 {
    match bits {
        0 => SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        8 => SNDRV_PCM_FMTBIT_S8, 16 => SNDRV_PCM_FMTBIT_S16_LE, 20 => SNDRV_PCM_FMTBIT_S20_LE, 24 => SNDRV_PCM_FMTBIT_S24_LE, 32 => SNDRV_PCM_FMTBIT_S32_LE, _ => 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_populate_rate_format(dev: *mut device, _function: *mut sdca_function_data, entity: *mut sdca_entity, stream: *mut snd_soc_pcm_stream) -> c_int {
    let sel = match (*entity).type_ {
        SDCA_ENTITY_TYPE_IT => SDCA_CTL_IT_USAGE,
        SDCA_ENTITY_TYPE_OT => SDCA_CTL_OT_USAGE,
        _ => { dev_err(dev, cstr!("%s: entity type has no usage control\n"), (*entity).label); return -EINVAL; }
    };
    let mut clock_rates = 0;
    if !(*entity).iot.clock.is_null() {
        let range = sdca_selector_find_range(dev, (*entity).iot.clock, SDCA_CTL_CS_SAMPLERATEINDEX, SDCA_SAMPLERATEINDEX_NCOLS, 0);
        if range.is_null() { return -EINVAL; }
        for i in 0..(*range).rows { clock_rates |= rate_find_mask(sdca_range(range, SDCA_SAMPLERATEINDEX_RATE, i) as c_uint); }
    } else { clock_rates = UINT_MAX; }
    let range = sdca_selector_find_range(dev, entity, sel, SDCA_USAGE_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    let mut rates = 0; let mut formats = 0u64;
    for i in 0..(*range).rows {
        let sample_rate = rate_find_mask(sdca_range(range, SDCA_USAGE_SAMPLE_RATE, i) as c_uint);
        if (sample_rate & clock_rates) != 0 {
            rates |= sample_rate;
            formats |= width_find_mask(sdca_range(range, SDCA_USAGE_SAMPLE_WIDTH, i) as c_uint);
        }
    }
    (*stream).formats = formats; (*stream).rates = rates; 0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_populate_dais(dev: *mut device, function: *mut sdca_function_data, dais: *mut snd_soc_dai_driver, ops: *const snd_soc_dai_ops) -> c_int {
    let mut j = 0;
    for i in 0..((*function).num_entities - 1) {
        let entity = (*function).entities.add(i as usize);
        let (stream, stream_suffix) = match (*entity).type_ {
            SDCA_ENTITY_TYPE_IT => (&mut (*dais.add(j as usize)).playback as *mut snd_soc_pcm_stream, cstr!("Playback")),
            SDCA_ENTITY_TYPE_OT => (&mut (*dais.add(j as usize)).capture as *mut snd_soc_pcm_stream, cstr!("Capture")),
            _ => continue,
        };
        if !(*entity).iot.is_dataport { continue; }
        (*stream).stream_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("%s %s"), (*entity).label, stream_suffix);
        if (*stream).stream_name.is_null() { return -ENOMEM; }
        (*stream).channels_min = 1; (*stream).channels_max = SDCA_MAX_CHANNEL_COUNT;
        let ret = sdca_asoc_populate_rate_format(dev, function, entity, stream);
        if ret != 0 { return ret; }
        (*dais.add(j as usize)).id = i; (*dais.add(j as usize)).name = (*entity).label; (*dais.add(j as usize)).ops = ops; j += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_populate_component(dev: *mut device, function: *mut sdca_function_data, component_drv: *mut snd_soc_component_driver, dai_drv: *mut *mut snd_soc_dai_driver, num_dai_drv: *mut c_int, ops: *const snd_soc_dai_ops) -> c_int {
    let mut num_widgets = 0; let mut num_routes = 0; let mut num_controls = 0; let mut num_dais = 0;
    let mut ret = sdca_asoc_count_component(dev, function, &mut num_widgets, &mut num_routes, &mut num_controls, &mut num_dais);
    if ret != 0 { return ret; }
    let widgets = devm_kcalloc(dev, num_widgets as usize, core::mem::size_of::<snd_soc_dapm_widget>(), GFP_KERNEL) as *mut snd_soc_dapm_widget;
    if widgets.is_null() { return -ENOMEM; }
    let routes = devm_kcalloc(dev, num_routes as usize, core::mem::size_of::<snd_soc_dapm_route>(), GFP_KERNEL) as *mut snd_soc_dapm_route;
    if routes.is_null() { return -ENOMEM; }
    let controls = devm_kcalloc(dev, num_controls as usize, core::mem::size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if controls.is_null() { return -ENOMEM; }
    let dais = devm_kcalloc(dev, num_dais as usize, core::mem::size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dais.is_null() { return -ENOMEM; }
    ret = sdca_asoc_populate_dapm(dev, function, widgets, routes); if ret != 0 { return ret; }
    ret = sdca_asoc_populate_controls(dev, function, controls); if ret != 0 { return ret; }
    ret = sdca_asoc_populate_dais(dev, function, dais, ops); if ret != 0 { return ret; }
    (*component_drv).dapm_widgets = widgets; (*component_drv).num_dapm_widgets = num_widgets; (*component_drv).dapm_routes = routes; (*component_drv).num_dapm_routes = num_routes; (*component_drv).controls = controls; (*component_drv).num_controls = num_controls;
    *dai_drv = dais; *num_dai_drv = num_dais; 0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_set_constraints(dev: *mut device, _regmap: *mut regmap, function: *mut sdca_function_data, substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    static channel_list: [c_uint; 32] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
    let entity = (*function).entities.add((*dai).id as usize);
    if (*entity).type_ != SDCA_ENTITY_TYPE_IT { return 0; }
    let control = sdca_selector_find_control(dev, entity, SDCA_CTL_IT_CLUSTERINDEX);
    if control.is_null() { return -EINVAL; }
    let range = sdca_control_find_range(dev, entity, control, SDCA_CLUSTER_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    let mut channel_mask = 0u32;
    for i in 0..(*range).rows {
        let clusterid = sdca_range(range, SDCA_CLUSTER_CLUSTERID, i);
        let cluster = sdca_id_find_cluster(dev, function, clusterid);
        if cluster.is_null() { return -ENODEV; }
        channel_mask |= 1u32 << ((*cluster).num_channels - 1);
    }
    dev_dbg(dev, cstr!("%s: set channel constraint mask: %#x\n"), (*entity).label, channel_mask);
    let constraint = kzalloc(core::mem::size_of::<snd_pcm_hw_constraint_list>(), GFP_KERNEL) as *mut snd_pcm_hw_constraint_list;
    if constraint.is_null() { return -ENOMEM; }
    (*constraint).count = channel_list.len() as c_uint; (*constraint).list = channel_list.as_ptr(); (*constraint).mask = channel_mask;
    let ret = snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, constraint);
    if ret != 0 { dev_err(dev, cstr!("%s: failed to add constraint: %d\n"), (*entity).label, ret); kfree(constraint as *mut c_void); return ret; }
    (*dai).priv_ = constraint as *mut c_void; 0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_free_constraints(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let constraint = (*dai).priv_ as *mut snd_pcm_hw_constraint_list;
    kfree(constraint as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_get_port(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data, dai: *mut snd_soc_dai) -> c_int {
    let entity = (*function).entities.add((*dai).id as usize);
    let mut sel = match (*entity).type_ { SDCA_ENTITY_TYPE_IT => SDCA_CTL_IT_DATAPORT_SELECTOR, SDCA_ENTITY_TYPE_OT => SDCA_CTL_OT_DATAPORT_SELECTOR, _ => -EINVAL };
    if sel < 0 || !(*entity).iot.is_dataport { dev_err(dev, cstr!("%s: port number only available for dataports\n"), (*entity).label); return -EINVAL; }
    let range = sdca_selector_find_range(dev, entity, sel, SDCA_DATAPORT_SELECTOR_NCOLS, SDCA_DATAPORT_SELECTOR_NROWS);
    if range.is_null() { return -EINVAL; }
    let reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, sel, 0);
    let mut val = 0u32;
    let ret = regmap_read(regmap, reg, &mut val);
    if ret != 0 { dev_err(dev, cstr!("%s: failed to read dataport selector: %d\n"), (*entity).label, ret); return ret; }
    for i in 0..(*range).rows {
        const port_mask: u8 = 0xF;
        sel = sdca_range(range, (val & port_mask as u32) as c_int, i);
        if sel != 0xFF { return sel; }
        val >>= hweight8(port_mask);
    }
    dev_err(dev, cstr!("%s: no dataport found\n"), (*entity).label); -ENODEV
}

unsafe fn set_cluster(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data, entity: *mut sdca_entity, channels: c_uint) -> c_int {
    let sel = SDCA_CTL_IT_CLUSTERINDEX;
    let range = sdca_selector_find_range(dev, entity, sel, SDCA_CLUSTER_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    for i in 0..(*range).rows {
        let cluster_id = sdca_range(range, SDCA_CLUSTER_CLUSTERID, i);
        let cluster = sdca_id_find_cluster(dev, function, cluster_id);
        if cluster.is_null() { return -ENODEV; }
        if (*cluster).num_channels == channels {
            let index = sdca_range(range, SDCA_CLUSTER_BYTEINDEX, i) as c_uint;
            let reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, sel, 0);
            let ret = regmap_update_bits(regmap, reg, 0xFF, index);
            if ret != 0 { dev_err(dev, cstr!("%s: failed to write cluster index: %d\n"), (*entity).label, ret); return ret; }
            dev_dbg(dev, cstr!("%s: set cluster to %d (%d channels)\n"), (*entity).label, index, channels);
            return 0;
        }
    }
    dev_err(dev, cstr!("%s: no cluster for %d channels\n"), (*entity).label, channels); -EINVAL
}

unsafe fn set_clock(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data, entity: *mut sdca_entity, target_rate: c_int) -> c_int {
    let sel = SDCA_CTL_CS_SAMPLERATEINDEX;
    let range = sdca_selector_find_range(dev, entity, sel, SDCA_SAMPLERATEINDEX_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    for i in 0..(*range).rows {
        let rate = sdca_range(range, SDCA_SAMPLERATEINDEX_RATE, i) as c_uint;
        if rate == target_rate as c_uint {
            let index = sdca_range(range, SDCA_SAMPLERATEINDEX_INDEX, i) as c_uint;
            let reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, sel, 0);
            let ret = regmap_update_bits(regmap, reg, 0xFF, index);
            if ret != 0 { dev_err(dev, cstr!("%s: failed to write clock rate: %d\n"), (*entity).label, ret); return ret; }
            dev_dbg(dev, cstr!("%s: set clock rate to %d (%dHz)\n"), (*entity).label, index, rate);
            return 0;
        }
    }
    dev_err(dev, cstr!("%s: no clock rate for %dHz\n"), (*entity).label, target_rate); -EINVAL
}

unsafe fn set_usage(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data, entity: *mut sdca_entity, sel: c_int, target_rate: c_int, target_width: c_int) -> c_int {
    let range = sdca_selector_find_range(dev, entity, sel, SDCA_USAGE_NCOLS, 0);
    if range.is_null() { return -EINVAL; }
    for i in 0..(*range).rows {
        let rate = sdca_range(range, SDCA_USAGE_SAMPLE_RATE, i);
        let width = sdca_range(range, SDCA_USAGE_SAMPLE_WIDTH, i);
        if (rate == 0 || rate == target_rate) && (width == 0 || width == target_width) {
            let usage = sdca_range(range, SDCA_USAGE_NUMBER, i) as c_uint;
            let reg = SDW_SDCA_CTL((*(*function).desc).adr, (*entity).id, sel, 0);
            let ret = regmap_update_bits(regmap, reg, 0xFF, usage);
            if ret != 0 { dev_err(dev, cstr!("%s: failed to write usage: %d\n"), (*entity).label, ret); return ret; }
            dev_dbg(dev, cstr!("%s: set usage to %#x (%dHz, %d bits)\n"), (*entity).label, usage, target_rate, target_width);
            return 0;
        }
    }
    dev_err(dev, cstr!("%s: no usage for %dHz, %dbits\n"), (*entity).label, target_rate, target_width); -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn sdca_asoc_hw_params(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data, _substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let entity = (*function).entities.add((*dai).id as usize);
    let channels = params_channels(params);
    let width = params_width(params);
    let rate = params_rate(params);
    let usage_sel;
    match (*entity).type_ {
        SDCA_ENTITY_TYPE_IT => {
            let ret = set_cluster(dev, regmap, function, entity, channels as c_uint);
            if ret != 0 { return ret; }
            usage_sel = SDCA_CTL_IT_USAGE;
        }
        SDCA_ENTITY_TYPE_OT => usage_sel = SDCA_CTL_OT_USAGE,
        _ => { dev_err(dev, cstr!("%s: hw_params on non-terminal entity\n"), (*entity).label); return -EINVAL; }
    }
    if !(*entity).iot.clock.is_null() {
        let ret = set_clock(dev, regmap, function, (*entity).iot.clock, rate);
        if ret != 0 { return ret; }
    }
    let ret = set_usage(dev, regmap, function, entity, usage_sel, rate, width);
    if ret != 0 { return ret; }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
