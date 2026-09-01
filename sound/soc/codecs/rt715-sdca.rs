// SPDX-License-Identifier: GPL-2.0-only
//
// rt715-sdca.c -- rt715 ALSA SoC audio driver
//
// Copyright(c) 2020 Realtek Semiconductor Corp.
//
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_id {
    pub sdw_version: c_uint,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub id: sdw_id,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_data {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

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
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub shift: c_uint,
    pub rshift: c_uint,
    pub max: c_uint,
    pub platform_max: c_uint,
    pub invert: c_uint,
    pub autodisable: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub reg2: c_uint,
    pub shift_l: c_uchar,
    pub shift_r: c_uchar,
    pub items: c_uint,
    pub texts: *const *const c_char,
    pub values: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
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
pub struct sdw_stream_config {
    _data: [u8; 0],
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
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
    pub name: *const c_char,
    pub id: c_int,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub tlv: snd_kcontrol_tlv,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct rt715_sdca_priv {
    pub slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub hw_sdw_ver: c_uint,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub kctl_2ch_orig: [c_long; 2],
    pub kctl_4ch_orig: [c_long; 4],
    pub kctl_8ch_orig: [c_long; 8],
    pub kctl_switch_orig: [c_long; 4],
}

#[repr(C)]
pub struct rt715_sdca_kcontrol_private {
    pub reg_base: c_uint,
    pub count: c_uint,
    pub max: c_uint,
    pub shift: c_uint,
    pub invert: c_uint,
}

type c_long = i64;
type c_uchar = u8;

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt715_sdca_priv;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, mux: c_int, e: *mut soc_enum, update: *mut c_void) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config);
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, nports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device) -> c_int;
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
}

const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SND_SOC_NOPM: c_uint = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0x10;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x20;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 0;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S8: c_ulong = 1 << 3;

const RT715_SDCA_DB_STEP: c_uint = 75;
const RT715_VENDOR_HDA_CTL: c_uint = 0;
const RT715_VENDOR_REG: c_uint = 0;
const RT715_HDA_LEGACY_MUX_CTL1: c_uint = 0;
const RT715_AIF1: c_int = 1;
const RT715_AIF2: c_int = 2;
const RT715_SDCA_FU_ADC7_27_VOL: c_uint = 0;
const RT715_SDCA_FU_ADC8_9_VOL: c_uint = 0;
const RT715_SDCA_FU_ADC10_11_VOL: c_uint = 0;
const RT715_SDCA_FU_MUTE_CTRL: c_uint = 0;
const RT715_SDCA_FU_VOL_CTRL: c_uint = 0;
const RT715_SDCA_FU_DMIC_GAIN_EN: c_uint = 0;
const RT715_SDCA_FU_AMIC_GAIN_EN: c_uint = 0;
const RT715_SDCA_FU_DMIC_GAIN_CTRL: c_uint = 0;
const RT715_SDCA_CREQ_POW_EN: c_uint = 0;
const RT715_SDCA_REQ_POW_CTRL: c_uint = 0;
const RT715_SDW_INPUT_SEL: c_uint = 0;
const RT715_SDCA_CS_FREQ_IND_EN: c_uint = 0;
const RT715_SDCA_FREQ_IND_CTRL: c_uint = 0;
const RT715_PRODUCT_NUM: c_uint = 0;
const RT715_SDCA_CX_CLK_SEL_EN: c_uint = 0;
const RT715_SDCA_CX_CLK_SEL_CTRL: c_uint = 0;
const RT715_AD_FUNC_EN: c_uint = 0;
const RT715_REV_1: c_uint = 0;
const RT715_DFLL_VAD: c_uint = 0;
const RT715_SDCA_SMPU_TRIG_ST_EN: c_uint = 0;
const RT715_SDCA_SMPU_TRIG_EN_CTRL: c_uint = 0;
const RT715_INT_MASK: c_uint = 0;
const FUN_MIC_ARRAY: c_uint = 0;
const CH_00: c_uint = 0;
const CH_01: c_uint = 1;
const CH_02: c_uint = 2;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn SDW_SDCA_CTL(fun: c_uint, entity: c_uint, control: c_uint, channel: c_uint) -> c_uint {
    (fun << 16) | (entity << 8) | (control << 4) | channel
}

fn fls(x: c_uint) -> c_uint {
    if x == 0 { 0 } else { 32 - x.leading_zeros() }
}

fn set_mask_bits(ptr_: *mut c_uint, mask: c_uint, val: c_uint) {
    unsafe {
        *ptr_ = (*ptr_ & !mask) | (val & mask);
    }
}

unsafe fn rt715_sdca_index_write(
    rt715: *mut rt715_sdca_priv,
    nid: c_uint,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let regmap = (*rt715).mbq_regmap;
    let addr: c_uint = (nid << 20) | reg;
    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        dev_err(
            &mut (*(*rt715).slave).dev,
            b"%s: Failed to set private value: %08x <= %04x %d\n\0".as_ptr() as *const c_char,
            b"rt715_sdca_index_write\0".as_ptr(),
            addr,
            value,
            ret,
        );
    }
    ret
}

unsafe fn rt715_sdca_index_read(
    rt715: *mut rt715_sdca_priv,
    nid: c_uint,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let regmap = (*rt715).mbq_regmap;
    let addr: c_uint = (nid << 20) | reg;
    let ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        dev_err(
            &mut (*(*rt715).slave).dev,
            b"%s: Failed to get private value: %06x => %04x ret=%d\n\0".as_ptr() as *const c_char,
            b"rt715_sdca_index_read\0".as_ptr(),
            addr,
            *value,
            ret,
        );
    }
    ret
}

unsafe fn rt715_sdca_index_update_bits(
    rt715: *mut rt715_sdca_priv,
    nid: c_uint,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    let mut tmp: c_uint = 0;
    let ret = rt715_sdca_index_read(rt715, nid, reg, &mut tmp);
    if ret < 0 {
        return ret;
    }
    set_mask_bits(&mut tmp, mask, val);
    rt715_sdca_index_write(rt715, nid, reg, tmp)
}

fn rt715_sdca_vol_gain(mut u_ctrl_val: c_uint, vol_max: c_uint, vol_gain_sft: c_uint) -> c_uint {
    if u_ctrl_val > vol_max {
        u_ctrl_val = vol_max;
    }
    let val = u_ctrl_val;
    u_ctrl_val = ((u_ctrl_val.abs_diff(vol_gain_sft) * RT715_SDCA_DB_STEP) << 8) / 1000;
    if val <= vol_gain_sft {
        u_ctrl_val = !u_ctrl_val;
        u_ctrl_val = u_ctrl_val.wrapping_add(1);
    }
    u_ctrl_val &= 0xffff;
    u_ctrl_val
}

fn rt715_sdca_boost_gain(mut u_ctrl_val: c_uint, b_max: c_uint, b_gain_sft: c_uint) -> c_uint {
    if u_ctrl_val > b_max {
        u_ctrl_val = b_max;
    }
    (u_ctrl_val * 10) << b_gain_sft
}

fn rt715_sdca_get_gain(mut reg_val: c_uint, gain_sft: c_uint) -> c_uint {
    let mut neg_flag: c_uint = 0;
    if (reg_val & BIT(15)) != 0 {
        reg_val = !(reg_val.wrapping_sub(1)) & 0xffff;
        neg_flag = 1;
    }
    reg_val *= 1000;
    reg_val >>= 8;
    if neg_flag != 0 {
        reg_val = gain_sft - reg_val / RT715_SDCA_DB_STEP;
    } else {
        reg_val = gain_sft + reg_val / RT715_SDCA_DB_STEP;
    }
    reg_val
}

/* SDCA Volume/Boost control */
unsafe extern "C" fn rt715_sdca_set_amp_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt715 = snd_soc_component_get_drvdata(component);
    let mut k_changed: c_uint = 0;
    let mut i: usize = 0;
    while i < 2 {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_2ch_orig[i] {
            k_changed = 1;
            break;
        }
        i += 1;
    }
    i = 0;
    while i < 2 {
        (*rt715).kctl_2ch_orig[i] = (*ucontrol).value.integer.value[i];
        let gain_val = rt715_sdca_vol_gain((*ucontrol).value.integer.value[i] as c_uint, (*mc).max, (*mc).shift);
        let ret = regmap_write((*rt715).mbq_regmap, (*mc).reg + i as c_uint, gain_val);
        if ret != 0 {
            dev_err((*component).dev, b"%s: Failed to write 0x%x=0x%x\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_put\0".as_ptr(), (*mc).reg + i as c_uint, gain_val);
            return ret;
        }
        i += 1;
    }
    k_changed as c_int
}

unsafe extern "C" fn rt715_sdca_set_amp_gain_4ch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    let reg_base = (*p).reg_base;
    let mut k_changed: c_uint = 0;
    const gain_sft: c_uint = 0x2f;
    let mut i: usize = 0;
    while i < 4 {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_4ch_orig[i] {
            k_changed = 1;
            break;
        }
        i += 1;
    }
    i = 0;
    while i < 4 {
        (*rt715).kctl_4ch_orig[i] = (*ucontrol).value.integer.value[i];
        let gain_val = rt715_sdca_vol_gain((*ucontrol).value.integer.value[i] as c_uint, (*p).max, gain_sft);
        let ret = regmap_write((*rt715).mbq_regmap, reg_base + i as c_uint, gain_val);
        if ret != 0 {
            dev_err((*component).dev, b"%s: Failed to write 0x%x=0x%x\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_4ch_put\0".as_ptr(), reg_base + i as c_uint, gain_val);
            return ret;
        }
        i += 1;
    }
    k_changed as c_int
}

unsafe extern "C" fn rt715_sdca_set_amp_gain_8ch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    let reg_base = (*p).reg_base;
    let mut k_changed: c_uint = 0;
    const gain_sft: c_uint = 8;
    let mut i: usize = 0;
    while i < 8 {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_8ch_orig[i] {
            k_changed = 1;
            break;
        }
        i += 1;
    }
    i = 0;
    while i < 8 {
        (*rt715).kctl_8ch_orig[i] = (*ucontrol).value.integer.value[i];
        let gain_val = rt715_sdca_boost_gain((*ucontrol).value.integer.value[i] as c_uint, (*p).max, gain_sft);
        let reg = if i < 7 { reg_base + i as c_uint } else { (reg_base - 1) | BIT(15) };
        let ret = regmap_write((*rt715).mbq_regmap, reg, gain_val);
        if ret != 0 {
            dev_err((*component).dev, b"%s: Failed to write 0x%x=0x%x\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_8ch_put\0".as_ptr(), reg, gain_val);
            return ret;
        }
        i += 1;
    }
    k_changed as c_int
}

unsafe extern "C" fn rt715_sdca_set_amp_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt715 = snd_soc_component_get_drvdata(component);
    let mut i: usize = 0;
    while i < 2 {
        let mut val: c_uint = 0;
        let ret = regmap_read((*rt715).mbq_regmap, (*mc).reg + i as c_uint, &mut val);
        if ret < 0 {
            dev_err((*component).dev, b"%s: Failed to read 0x%x, ret=%d\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_get\0".as_ptr(), (*mc).reg + i as c_uint, ret);
            return ret;
        }
        (*ucontrol).value.integer.value[i] = rt715_sdca_get_gain(val, (*mc).shift) as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn rt715_sdca_set_amp_gain_4ch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    let reg_base = (*p).reg_base;
    const gain_sft: c_uint = 0x2f;
    let mut i: usize = 0;
    while i < 4 {
        let mut val: c_uint = 0;
        let ret = regmap_read((*rt715).mbq_regmap, reg_base + i as c_uint, &mut val);
        if ret < 0 {
            dev_err((*component).dev, b"%s: Failed to read 0x%x, ret=%d\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_4ch_get\0".as_ptr(), reg_base + i as c_uint, ret);
            return ret;
        }
        (*ucontrol).value.integer.value[i] = rt715_sdca_get_gain(val, gain_sft) as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn rt715_sdca_set_amp_gain_8ch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    let reg_base = (*p).reg_base;
    const gain_sft: c_uint = 8;
    let mut i: usize = 0;
    while i < 8 {
        let mut val_l: c_uint = 0;
        let ret = regmap_read((*rt715).mbq_regmap, reg_base + i as c_uint, &mut val_l);
        if ret < 0 {
            dev_err((*component).dev, b"%s: Failed to read 0x%x, ret=%d\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_8ch_get\0".as_ptr(), reg_base + i as c_uint, ret);
            return ret;
        }
        (*ucontrol).value.integer.value[i] = ((val_l >> gain_sft) / 10) as c_long;
        let reg = if i == 6 { (reg_base - 1) | BIT(15) } else { reg_base + 1 + i as c_uint };
        let mut val_r: c_uint = 0;
        let ret = regmap_read((*rt715).mbq_regmap, reg, &mut val_r);
        if ret < 0 {
            dev_err((*component).dev, b"%s: Failed to read 0x%x, ret=%d\n\0".as_ptr() as *const c_char, b"rt715_sdca_set_amp_gain_8ch_get\0".as_ptr(), reg, ret);
            return ret;
        }
        (*ucontrol).value.integer.value[i + 1] = ((val_r >> gain_sft) / 10) as c_long;
        i += 2;
    }
    0
}

static in_vol_tlv: [c_uint; 4] = [0, (-1725i32) as c_uint, 75, 0];
static mic_vol_tlv: [c_uint; 4] = [0, 0, 1000, 0];

unsafe extern "C" fn rt715_sdca_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    let reg_base = (*p).reg_base;
    let invert = (*p).invert;
    let mut i: usize = 0;
    while i < (*p).count as usize {
        let mut val = snd_soc_component_read(component, reg_base + i as c_uint);
        if val < 0 {
            return -EINVAL;
        }
        (*ucontrol).value.integer.value[i] = if invert != 0 { ((*p).max as c_int - val) as c_long } else { val as c_long };
        val = snd_soc_component_read(component, reg_base + 1 + i as c_uint);
        if val < 0 {
            return -EINVAL;
        }
        (*ucontrol).value.integer.value[i + 1] = if invert != 0 { ((*p).max as c_int - val) as c_long } else { val as c_long };
        i += 2;
    }
    0
}

unsafe extern "C" fn rt715_sdca_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    let mut val: [c_uint; 4] = [0; 4];
    let mut k_changed: c_uint = 0;
    let reg = (*p).reg_base;
    let shift = (*p).shift;
    let max = (*p).max;
    let mask = (1 << fls(max)) - 1;
    let invert = (*p).invert;
    let mut i: usize = 0;
    while i < 4 {
        if (*ucontrol).value.integer.value[i] != (*rt715).kctl_switch_orig[i] {
            k_changed = 1;
            break;
        }
        i += 1;
    }
    i = 0;
    while i < 2 {
        (*rt715).kctl_switch_orig[i * 2] = (*ucontrol).value.integer.value[i * 2];
        val[i * 2] = ((*ucontrol).value.integer.value[i * 2] as c_uint) & mask;
        if invert != 0 {
            val[i * 2] = max - val[i * 2];
        }
        let val_mask = mask << shift;
        val[i * 2] <<= shift;

        (*rt715).kctl_switch_orig[i * 2 + 1] = (*ucontrol).value.integer.value[i * 2 + 1];
        val[i * 2 + 1] = ((*ucontrol).value.integer.value[i * 2 + 1] as c_uint) & mask;
        if invert != 0 {
            val[i * 2 + 1] = max - val[i * 2 + 1];
        }
        val[i * 2 + 1] <<= shift;

        let mut err = snd_soc_component_update_bits(component, reg + (i * 2) as c_uint, val_mask, val[i * 2]);
        if err < 0 {
            return err;
        }
        err = snd_soc_component_update_bits(component, reg + 1 + (i * 2) as c_uint, val_mask, val[i * 2 + 1]);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    k_changed as c_int
}

unsafe extern "C" fn rt715_sdca_fu_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let p = (*kcontrol).private_value as *mut rt715_sdca_kcontrol_private;
    if (*p).max == 1 {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    } else {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    }
    (*uinfo).count = (*p).count;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*p).max as c_long;
    0
}

/* C macro initializers RT715_SDCA_PR_VALUE, RT715_SDCA_FU_CTRL,
 * RT715_SDCA_EXT_TLV, RT715_SDCA_BOOST_EXT_TLV, SOC_DOUBLE_R,
 * SOC_DOUBLE_R_EXT_TLV, and DAPM declarations depend on external ALSA macros.
 * The local controls and widgets are represented below with explicit empty
 * arrays where their exact kernel-side initializer layout is external.
 */
static rt715_sdca_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn rt715_sdca_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let mut val: c_uint = 0;
    let mask_sft: c_uint;
    let name = (*ucontrol).id.name.as_ptr();
    if !strstr(name, b"ADC 22 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 12;
    } else if !strstr(name, b"ADC 23 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 8;
    } else if !strstr(name, b"ADC 24 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 4;
    } else if !strstr(name, b"ADC 25 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 0;
    } else {
        return -EINVAL;
    }

    rt715_sdca_index_read(rt715, RT715_VENDOR_HDA_CTL, RT715_HDA_LEGACY_MUX_CTL1, &mut val);
    val = (val >> mask_sft) & 0xf;

    /*
     * The first two indices of ADC Mux 24/25 are routed to the same
     * hardware source. ie, ADC Mux 24 0/1 will both connect to MIC2.
     * To have a unique set of inputs, we skip the index1 of the muxes.
     */
    if (!strstr(name, b"ADC 24 Mux\0".as_ptr() as *const c_char).is_null()
        || !strstr(name, b"ADC 25 Mux\0".as_ptr() as *const c_char).is_null()) && val > 0
    {
        val -= 1;
    }
    (*ucontrol).value.enumerated.item[0] = val;
    0
}

unsafe extern "C" fn rt715_sdca_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt715 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let mask_sft: c_uint;

    if *item >= (*e).items {
        return -EINVAL;
    }

    let name = (*ucontrol).id.name.as_ptr();
    if !strstr(name, b"ADC 22 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 12;
    } else if !strstr(name, b"ADC 23 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 8;
    } else if !strstr(name, b"ADC 24 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 4;
    } else if !strstr(name, b"ADC 25 Mux\0".as_ptr() as *const c_char).is_null() {
        mask_sft = 0;
    } else {
        return -EINVAL;
    }

    /* Verb ID = 0x701h, nid = e->reg */
    let val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    let mut val2: c_uint = 0;
    rt715_sdca_index_read(rt715, RT715_VENDOR_HDA_CTL, RT715_HDA_LEGACY_MUX_CTL1, &mut val2);
    val2 = (val2 >> mask_sft) & 0xf;
    let change = (val != val2) as c_uint;
    if change != 0 {
        rt715_sdca_index_update_bits(rt715, RT715_VENDOR_HDA_CTL, RT715_HDA_LEGACY_MUX_CTL1, 0xf << mask_sft, val << mask_sft);
    }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item as c_int, e, ptr::null_mut());
    change as c_int
}

static adc_22_23_mux_text: [&[u8]; 8] = [
    b"MIC1\0",
    b"MIC2\0",
    b"LINE1\0",
    b"LINE2\0",
    b"DMIC1\0",
    b"DMIC2\0",
    b"DMIC3\0",
    b"DMIC4\0",
];

/*
 * Due to mux design for nid 24 (MUX_IN3)/25 (MUX_IN4), connection index 0 and
 * 1 will be connected to the same dmic source, therefore we skip index 1 to
 * avoid misunderstanding on usage of dapm routing.
 */
static mut rt715_adc_24_25_values: [c_int; 5] = [0, 2, 3, 4, 5];

static adc_24_mux_text: [&[u8]; 5] = [b"MIC2\0", b"DMIC1\0", b"DMIC2\0", b"DMIC3\0", b"DMIC4\0"];
static adc_25_mux_text: [&[u8]; 5] = [b"MIC1\0", b"DMIC1\0", b"DMIC2\0", b"DMIC3\0", b"DMIC4\0"];

/* SOC_ENUM_SINGLE_DECL and SOC_VALUE_ENUM_SINGLE_DECL expansion depends on external ALSA declarations. */
static mut rt715_adc22_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, reg2: 0, shift_l: 0, shift_r: 0, items: 8, texts: ptr::null(), values: ptr::null(), mask: 0 };
static mut rt715_adc23_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, reg2: 0, shift_l: 0, shift_r: 0, items: 8, texts: ptr::null(), values: ptr::null(), mask: 0 };
static mut rt715_adc24_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, reg2: 0, shift_l: 0, shift_r: 0, items: 5, texts: ptr::null(), values: ptr::null(), mask: 0xf };
static mut rt715_adc25_enum: soc_enum = soc_enum { reg: SND_SOC_NOPM, reg2: 0, shift_l: 0, shift_r: 0, items: 5, texts: ptr::null(), values: ptr::null(), mask: 0xf };

static rt715_adc22_mux: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"ADC 22 Mux\0".as_ptr() as *const c_char, access: 0, tlv: snd_kcontrol_tlv { p: ptr::null() }, info: None, get: Some(rt715_sdca_mux_get), put: Some(rt715_sdca_mux_put), private_value: 0 };
static rt715_adc23_mux: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"ADC 23 Mux\0".as_ptr() as *const c_char, access: 0, tlv: snd_kcontrol_tlv { p: ptr::null() }, info: None, get: Some(rt715_sdca_mux_get), put: Some(rt715_sdca_mux_put), private_value: 0 };
static rt715_adc24_mux: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"ADC 24 Mux\0".as_ptr() as *const c_char, access: 0, tlv: snd_kcontrol_tlv { p: ptr::null() }, info: None, get: Some(rt715_sdca_mux_get), put: Some(rt715_sdca_mux_put), private_value: 0 };
static rt715_adc25_mux: snd_kcontrol_new = snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"ADC 25 Mux\0".as_ptr() as *const c_char, access: 0, tlv: snd_kcontrol_tlv { p: ptr::null() }, info: None, get: Some(rt715_sdca_mux_get), put: Some(rt715_sdca_mux_put), private_value: 0 };

unsafe extern "C" fn rt715_sdca_pde23_24_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt715 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt715).regmap, SDW_SDCA_CTL(FUN_MIC_ARRAY, RT715_SDCA_CREQ_POW_EN, RT715_SDCA_REQ_POW_CTRL, CH_00), 0x00);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt715).regmap, SDW_SDCA_CTL(FUN_MIC_ARRAY, RT715_SDCA_CREQ_POW_EN, RT715_SDCA_REQ_POW_CTRL, CH_00), 0x03);
        }
        _ => {}
    }
    0
}

static rt715_sdca_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static rt715_sdca_audio_map: [snd_soc_dapm_route; 44] = [
    snd_soc_dapm_route { sink: b"DP6TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 09\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP6TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 08\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP4TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 07\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP4TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 27\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP4TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 09\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP4TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 08\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINE1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINE2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MIC1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MIC2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC3\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC4\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23_24\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 09\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 22 Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 08\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 23 Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 07\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 24 Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 27\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC 25 Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"MIC1\0".as_ptr() as *const c_char, source: b"MIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"MIC2\0".as_ptr() as *const c_char, source: b"MIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"LINE1\0".as_ptr() as *const c_char, source: b"LINE1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"LINE2\0".as_ptr() as *const c_char, source: b"LINE2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"DMIC1\0".as_ptr() as *const c_char, source: b"DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"DMIC2\0".as_ptr() as *const c_char, source: b"DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"DMIC3\0".as_ptr() as *const c_char, source: b"DMIC3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 22 Mux\0".as_ptr() as *const c_char, control: b"DMIC4\0".as_ptr() as *const c_char, source: b"DMIC4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"MIC1\0".as_ptr() as *const c_char, source: b"MIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"MIC2\0".as_ptr() as *const c_char, source: b"MIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"LINE1\0".as_ptr() as *const c_char, source: b"LINE1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"LINE2\0".as_ptr() as *const c_char, source: b"LINE2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"DMIC1\0".as_ptr() as *const c_char, source: b"DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"DMIC2\0".as_ptr() as *const c_char, source: b"DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"DMIC3\0".as_ptr() as *const c_char, source: b"DMIC3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 23 Mux\0".as_ptr() as *const c_char, control: b"DMIC4\0".as_ptr() as *const c_char, source: b"DMIC4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 24 Mux\0".as_ptr() as *const c_char, control: b"MIC2\0".as_ptr() as *const c_char, source: b"MIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 24 Mux\0".as_ptr() as *const c_char, control: b"DMIC1\0".as_ptr() as *const c_char, source: b"DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 24 Mux\0".as_ptr() as *const c_char, control: b"DMIC2\0".as_ptr() as *const c_char, source: b"DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 24 Mux\0".as_ptr() as *const c_char, control: b"DMIC3\0".as_ptr() as *const c_char, source: b"DMIC3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 24 Mux\0".as_ptr() as *const c_char, control: b"DMIC4\0".as_ptr() as *const c_char, source: b"DMIC4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 25 Mux\0".as_ptr() as *const c_char, control: b"MIC1\0".as_ptr() as *const c_char, source: b"MIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 25 Mux\0".as_ptr() as *const c_char, control: b"DMIC1\0".as_ptr() as *const c_char, source: b"DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 25 Mux\0".as_ptr() as *const c_char, control: b"DMIC2\0".as_ptr() as *const c_char, source: b"DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 25 Mux\0".as_ptr() as *const c_char, control: b"DMIC3\0".as_ptr() as *const c_char, source: b"DMIC3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC 25 Mux\0".as_ptr() as *const c_char, control: b"DMIC4\0".as_ptr() as *const c_char, source: b"DMIC4\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn rt715_sdca_probe(component: *mut snd_soc_component) -> c_int {
    let rt715 = snd_soc_component_get_drvdata(component);
    if !(*rt715).first_hw_init {
        return 0;
    }
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }
    0
}

static soc_codec_dev_rt715_sdca: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt715_sdca_probe),
    controls: rt715_sdca_snd_controls.as_ptr(),
    num_controls: rt715_sdca_snd_controls.len() as c_uint,
    dapm_widgets: rt715_sdca_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt715_sdca_dapm_widgets.len() as c_uint,
    dapm_routes: rt715_sdca_audio_map.as_ptr(),
    num_dapm_routes: rt715_sdca_audio_map.len() as c_uint,
    endianness: 1,
};

unsafe extern "C" fn rt715_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt715_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt715_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt715 = snd_soc_component_get_drvdata(component);
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() {
        return -EINVAL;
    }
    if (*rt715).slave.is_null() {
        return -EINVAL;
    }
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);
    match (*dai).id {
        RT715_AIF1 => {
            port_config.num = 6;
            rt715_sdca_index_write(rt715, RT715_VENDOR_REG, RT715_SDW_INPUT_SEL, 0xa500);
        }
        RT715_AIF2 => {
            port_config.num = 4;
            rt715_sdca_index_write(rt715, RT715_VENDOR_REG, RT715_SDW_INPUT_SEL, 0xaf00);
        }
        _ => {
            dev_err((*component).dev, b"%s: Invalid DAI id %d\n\0".as_ptr() as *const c_char, b"rt715_sdca_pcm_hw_params\0".as_ptr(), (*dai).id);
            return -EINVAL;
        }
    }
    let retval = sdw_stream_add_slave((*rt715).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*component).dev, b"%s: Unable to configure port, retval:%d\n\0".as_ptr() as *const c_char, b"rt715_sdca_pcm_hw_params\0".as_ptr(), retval);
        return retval;
    }
    let val: c_uint = match params_rate(params) {
        8000 => 0x1,
        11025 => 0x2,
        12000 => 0x3,
        16000 => 0x4,
        22050 => 0x5,
        24000 => 0x6,
        32000 => 0x7,
        44100 => 0x8,
        48000 => 0x9,
        88200 => 0xa,
        96000 => 0xb,
        176400 => 0xc,
        192000 => 0xd,
        384000 => 0xe,
        768000 => 0xf,
        _ => {
            dev_err((*component).dev, b"%s: Unsupported sample rate %d\n\0".as_ptr() as *const c_char, b"rt715_sdca_pcm_hw_params\0".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    };
    regmap_write((*rt715).regmap, SDW_SDCA_CTL(FUN_MIC_ARRAY, RT715_SDCA_CS_FREQ_IND_EN, RT715_SDCA_FREQ_IND_CTRL, CH_00), val);
    0
}

unsafe extern "C" fn rt715_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt715 = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt715).slave.is_null() {
        return -EINVAL;
    }
    sdw_stream_remove_slave((*rt715).slave, sdw_stream);
    0
}

const RT715_STEREO_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const RT715_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

static rt715_sdca_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt715_sdca_pcm_hw_params),
    hw_free: Some(rt715_sdca_pcm_hw_free),
    set_stream: Some(rt715_sdca_set_sdw_stream),
    shutdown: Some(rt715_sdca_shutdown),
};

static mut rt715_sdca_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"rt715-sdca-aif1\0".as_ptr() as *const c_char,
        id: RT715_AIF1,
        capture: snd_soc_pcm_stream {
            stream_name: b"DP6 Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: RT715_STEREO_RATES,
            formats: RT715_FORMATS,
        },
        ops: &rt715_sdca_ops,
    },
    snd_soc_dai_driver {
        name: b"rt715-sdca-aif2\0".as_ptr() as *const c_char,
        id: RT715_AIF2,
        capture: snd_soc_pcm_stream {
            stream_name: b"DP4 Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: RT715_STEREO_RATES,
            formats: RT715_FORMATS,
        },
        ops: &rt715_sdca_ops,
    },
];

/* Bus clock frequency */
const RT715_CLK_FREQ_9600000HZ: c_uint = 9600000;
const RT715_CLK_FREQ_12000000HZ: c_uint = 12000000;
const RT715_CLK_FREQ_6000000HZ: c_uint = 6000000;
const RT715_CLK_FREQ_4800000HZ: c_uint = 4800000;
const RT715_CLK_FREQ_2400000HZ: c_uint = 2400000;
const RT715_CLK_FREQ_12288000HZ: c_uint = 12288000;

#[no_mangle]
pub unsafe extern "C" fn rt715_sdca_init(dev: *mut device, mbq_regmap: *mut regmap, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt715 = devm_kzalloc(dev, core::mem::size_of::<rt715_sdca_priv>(), GFP_KERNEL) as *mut rt715_sdca_priv;
    if rt715.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(dev, rt715 as *mut c_void);
    (*rt715).slave = slave;
    (*rt715).regmap = regmap;
    (*rt715).mbq_regmap = mbq_regmap;
    (*rt715).hw_sdw_ver = (*slave).id.sdw_version;

    regcache_cache_only((*rt715).regmap, true);
    regcache_cache_only((*rt715).mbq_regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt715).hw_init = false;
    (*rt715).first_hw_init = false;

    let ret = devm_snd_soc_register_component(dev, &soc_codec_dev_rt715_sdca, rt715_sdca_dai.as_mut_ptr(), rt715_sdca_dai.len() as c_int);
    if ret < 0 {
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);

    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */

    dev_dbg(dev, b"%s\n\0".as_ptr() as *const c_char, b"rt715_sdca_init\0".as_ptr());
    ret
}

#[no_mangle]
pub unsafe extern "C" fn rt715_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt715 = dev_get_drvdata(dev) as *mut rt715_sdca_priv;
    let mut hw_ver: c_uint = 0;

    if (*rt715).hw_init {
        return 0;
    }

    regcache_cache_only((*rt715).regmap, false);
    regcache_cache_only((*rt715).mbq_regmap, false);

    /*
     * PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*rt715).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
        (*rt715).first_hw_init = true;
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    rt715_sdca_index_read(rt715, RT715_VENDOR_REG, RT715_PRODUCT_NUM, &mut hw_ver);
    hw_ver &= 0x000f;

    /* set clock selector = external */
    regmap_write((*rt715).regmap, SDW_SDCA_CTL(FUN_MIC_ARRAY, RT715_SDCA_CX_CLK_SEL_EN, RT715_SDCA_CX_CLK_SEL_CTRL, CH_00), 0x1);
    /* set GPIO_4/5/6 to be 3rd/4th DMIC usage */
    if hw_ver == 0x0 {
        rt715_sdca_index_update_bits(rt715, RT715_VENDOR_REG, RT715_AD_FUNC_EN, 0x54, 0x54);
    } else if hw_ver == 0x1 {
        rt715_sdca_index_update_bits(rt715, RT715_VENDOR_REG, RT715_AD_FUNC_EN, 0x55, 0x55);
        rt715_sdca_index_update_bits(rt715, RT715_VENDOR_REG, RT715_REV_1, 0x40, 0x40);
    }
    /* DFLL Calibration trigger */
    rt715_sdca_index_update_bits(rt715, RT715_VENDOR_REG, RT715_DFLL_VAD, 0x1, 0x1);
    /* trigger mode = VAD enable */
    regmap_write((*rt715).regmap, SDW_SDCA_CTL(FUN_MIC_ARRAY, RT715_SDCA_SMPU_TRIG_ST_EN, RT715_SDCA_SMPU_TRIG_EN_CTRL, CH_00), 0x2);
    /* SMPU-1 interrupt enable mask */
    regmap_update_bits((*rt715).regmap, RT715_INT_MASK, 0x1, 0x1);

    /* Mark Slave initialization complete */
    (*rt715).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    0
}

/* MODULE_DESCRIPTION("ASoC rt715 driver SDW SDCA"); */
/* MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
