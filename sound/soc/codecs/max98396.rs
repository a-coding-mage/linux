// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2022, Analog Devices Inc.
//
// Rust source-level translation of soc/codecs/max98396.c.
// C include dependencies translated as external Rust dependencies expected from
// the surrounding kernel/ALSA binding layer:
// linux/gpio/consumer.h, linux/i2c.h, linux/module.h,
// sound/pcm_params.h, linux/regulator/consumer.h, sound/soc.h, sound/tlv.h,
// and max98396.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;

extern "C" {
    static max98396_reg: [reg_default; 0];
    static max98397_reg: [reg_default; 0];
}

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct regulator { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct i2c_client { pub dev: device }
#[repr(C)]
pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)]
pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)]
pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)]
pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer { pub value: [i64; 4] }
#[repr(C)]
pub struct soc_enum { pub items: c_uint, pub shift_l: c_uint }
#[repr(C)]
pub struct soc_mixer_control { pub reg: c_uint }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)]
pub struct regulator_bulk_data { pub supply: *const c_char }
#[repr(C)]
pub struct max98396_priv {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub core_supplies: [regulator_bulk_data; MAX98396_NUM_CORE_SUPPLIES as usize],
    pub pvdd: *mut regulator,
    pub vbat: *mut regulator,
    pub device_id: c_uint,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
    pub spkfb_slot: c_uint,
    pub bypass_slot: c_uint,
    pub dmon_stuck_enable: bool,
    pub dmon_stuck_threshold: c_uint,
    pub dmon_mag_enable: bool,
    pub dmon_mag_threshold: c_uint,
    pub dmon_duration: c_uint,
    pub interleave_mode: bool,
    pub tdm_mode: bool,
    pub tdm_max_samplerate: c_uint,
}

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, val_len: usize) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut max98396_priv;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_component_test_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, value: c_uint) -> c_int;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, mux: c_uint, e: *mut soc_enum, update: *mut c_void) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut max98396_priv;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_disable(r: *mut regulator) -> c_int;
    fn regulator_enable(r: *mut regulator) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_int) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn GET_REG_ADDR_REV_ID(device_id: c_uint) -> c_uint;
}

extern "C" {
    static MAX98396_NUM_CORE_SUPPLIES: c_int;
    static CODEC_TYPE_MAX98396: c_uint;
    static CODEC_TYPE_MAX98397: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static EPROBE_DEFER: c_int;
}

const MAX98396_BSEL_32: c_int = 0x2;
const MAX98396_BSEL_48: c_int = 0x3;
const MAX98396_BSEL_64: c_int = 0x4;
const MAX98396_BSEL_96: c_int = 0x5;
const MAX98396_BSEL_128: c_int = 0x6;
const MAX98396_BSEL_192: c_int = 0x7;
const MAX98396_BSEL_256: c_int = 0x8;
const MAX98396_BSEL_384: c_int = 0x9;
const MAX98396_BSEL_512: c_int = 0xa;
const MAX98396_BSEL_320: c_int = 0xb;
const MAX98396_BSEL_250: c_int = 0xc;
const MAX98396_BSEL_125: c_int = 0xd;

#[repr(C)]
#[derive(Copy, Clone)]
struct max98396_pcm_config {
    in_: c_int,
    out: c_int,
    width: c_int,
    bsel: c_int,
    max_sr: c_int,
}

/* Refer to table 5 in the datasheet */
static max98396_pcm_configs: [max98396_pcm_config; 31] = [
    max98396_pcm_config { in_: 2, out: 4, width: 16, bsel: MAX98396_BSEL_32, max_sr: 192000 },
    max98396_pcm_config { in_: 2, out: 6, width: 24, bsel: MAX98396_BSEL_48, max_sr: 192000 },
    max98396_pcm_config { in_: 2, out: 8, width: 32, bsel: MAX98396_BSEL_64, max_sr: 192000 },
    max98396_pcm_config { in_: 3, out: 15, width: 32, bsel: MAX98396_BSEL_125, max_sr: 192000 },
    max98396_pcm_config { in_: 4, out: 8, width: 16, bsel: MAX98396_BSEL_64, max_sr: 192000 },
    max98396_pcm_config { in_: 4, out: 12, width: 24, bsel: MAX98396_BSEL_96, max_sr: 192000 },
    max98396_pcm_config { in_: 4, out: 16, width: 32, bsel: MAX98396_BSEL_128, max_sr: 192000 },
    max98396_pcm_config { in_: 5, out: 15, width: 24, bsel: MAX98396_BSEL_125, max_sr: 192000 },
    max98396_pcm_config { in_: 7, out: 15, width: 16, bsel: MAX98396_BSEL_125, max_sr: 192000 },
    max98396_pcm_config { in_: 2, out: 4, width: 16, bsel: MAX98396_BSEL_32, max_sr: 96000 },
    max98396_pcm_config { in_: 2, out: 6, width: 24, bsel: MAX98396_BSEL_48, max_sr: 96000 },
    max98396_pcm_config { in_: 2, out: 8, width: 32, bsel: MAX98396_BSEL_64, max_sr: 96000 },
    max98396_pcm_config { in_: 3, out: 15, width: 32, bsel: MAX98396_BSEL_125, max_sr: 96000 },
    max98396_pcm_config { in_: 4, out: 8, width: 16, bsel: MAX98396_BSEL_64, max_sr: 96000 },
    max98396_pcm_config { in_: 4, out: 12, width: 24, bsel: MAX98396_BSEL_96, max_sr: 96000 },
    max98396_pcm_config { in_: 4, out: 16, width: 32, bsel: MAX98396_BSEL_128, max_sr: 96000 },
    max98396_pcm_config { in_: 5, out: 15, width: 24, bsel: MAX98396_BSEL_125, max_sr: 96000 },
    max98396_pcm_config { in_: 7, out: 15, width: 16, bsel: MAX98396_BSEL_125, max_sr: 96000 },
    max98396_pcm_config { in_: 7, out: 31, width: 32, bsel: MAX98396_BSEL_250, max_sr: 96000 },
    max98396_pcm_config { in_: 8, out: 16, width: 16, bsel: MAX98396_BSEL_128, max_sr: 96000 },
    max98396_pcm_config { in_: 8, out: 24, width: 24, bsel: MAX98396_BSEL_192, max_sr: 96000 },
    max98396_pcm_config { in_: 8, out: 32, width: 32, bsel: MAX98396_BSEL_256, max_sr: 96000 },
    max98396_pcm_config { in_: 10, out: 31, width: 24, bsel: MAX98396_BSEL_250, max_sr: 96000 },
    max98396_pcm_config { in_: 15, out: 31, width: 16, bsel: MAX98396_BSEL_250, max_sr: 96000 },
    max98396_pcm_config { in_: 16, out: 32, width: 16, bsel: MAX98396_BSEL_256, max_sr: 96000 },
    max98396_pcm_config { in_: 7, out: 31, width: 32, bsel: MAX98396_BSEL_250, max_sr: 48000 },
    max98396_pcm_config { in_: 10, out: 31, width: 24, bsel: MAX98396_BSEL_250, max_sr: 48000 },
    max98396_pcm_config { in_: 10, out: 40, width: 32, bsel: MAX98396_BSEL_320, max_sr: 48000 },
    max98396_pcm_config { in_: 15, out: 31, width: 16, bsel: MAX98396_BSEL_250, max_sr: 48000 },
    max98396_pcm_config { in_: 16, out: 48, width: 24, bsel: MAX98396_BSEL_384, max_sr: 48000 },
    max98396_pcm_config { in_: 16, out: 64, width: 32, bsel: MAX98396_BSEL_512, max_sr: 48000 },
];

unsafe fn max98396_global_enable_onoff(regmap_: *mut regmap, onoff: bool) {
    regmap_write(regmap_, MAX98396_R210F_GLOBAL_EN, if onoff { 1 } else { 0 });
    usleep_range(11000, 12000);
}

fn max98396_pcm_config_index(in_slots: c_int, out_slots: c_int, width: c_int) -> c_int {
    let mut i = 0usize;
    while i < max98396_pcm_configs.len() {
        let c = max98396_pcm_configs[i];
        if in_slots == c.in_ && out_slots <= c.out && width == c.width {
            return i as c_int;
        }
        i += 1;
    }
    -1
}

unsafe fn max98396_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98396 = snd_soc_component_get_drvdata(component);

    match event {
        SND_SOC_DAPM_POST_PMU => max98396_global_enable_onoff((*max98396).regmap, true),
        SND_SOC_DAPM_PRE_PMD => {
            max98396_global_enable_onoff((*max98396).regmap, false);
            (*max98396).tdm_mode = false;
        }
        _ => return 0,
    }
    0
}

unsafe fn max98396_pcm_register_readable(reg: c_uint, is_98397: bool) -> bool {
    if is_98397 {
        (MAX98396_R2001_INT_RAW1..=MAX98396_R2004_INT_RAW4).contains(&reg)
            || (MAX98396_R2006_INT_STATE1..=MAX98396_R2009_INT_STATE4).contains(&reg)
            || (MAX98396_R200B_INT_FLAG1..=MAX98396_R200E_INT_FLAG4).contains(&reg)
            || (MAX98396_R2010_INT_EN1..=MAX98396_R2013_INT_EN4).contains(&reg)
            || (MAX98396_R2015_INT_FLAG_CLR1..=MAX98396_R2018_INT_FLAG_CLR4).contains(&reg)
            || (MAX98396_R201F_IRQ_CTRL..=MAX98396_R2024_THERM_FOLDBACK_SET).contains(&reg)
            || reg == MAX98396_R2027_THERM_FOLDBACK_EN
            || reg == MAX98396_R2030_NOISEGATE_MODE_CTRL
            || reg == MAX98396_R2033_NOISEGATE_MODE_EN
            || (MAX98396_R2038_CLK_MON_CTRL..=MAX98397_R203A_SPK_MON_THRESH).contains(&reg)
            || (MAX98396_R203F_ENABLE_CTRLS..=MAX98397_R2054_PCM_TX_HIZ_CTRL_8).contains(&reg)
            || (MAX98397_R2056_PCM_RX_SRC1..=MAX98396_R2058_PCM_BYPASS_SRC).contains(&reg)
            || (MAX98396_R205D_PCM_TX_SRC_EN..=MAX98397_R2060_PCM_TX_SUPPLY_SEL).contains(&reg)
            || (MAX98396_R2070_ICC_RX_EN_A..=MAX98396_R2072_ICC_TX_CTRL).contains(&reg)
            || reg == MAX98396_R207F_ICC_EN
            || (MAX98396_R2083_TONE_GEN_DC_CFG..=MAX98396_R2086_TONE_GEN_DC_LVL3).contains(&reg)
            || (MAX98396_R208F_TONE_GEN_EN..=MAX98396_R209F_BYPASS_PATH_CFG).contains(&reg)
            || (MAX98396_R20AF_AMP_EN..=MAX98397_R20C5_MEAS_ADC_OPTIMAL_MODE).contains(&reg)
            || reg == MAX98396_R20C7_ADC_CFG
            || (MAX98396_R20D0_DHT_CFG1..=MAX98396_R20D6_DHT_HYSTERESIS_CFG).contains(&reg)
            || reg == MAX98396_R20DF_DHT_EN
            || reg == MAX98396_R20E0_IV_SENSE_PATH_CFG
            || (MAX98396_R20E4_IV_SENSE_PATH_EN..=MAX98396_R2106_BPE_THRESH_HYSTERESIS).contains(&reg)
            || (MAX98396_R2108_BPE_SUPPLY_SRC..=MAX98396_R210B_BPE_LOW_LIMITER).contains(&reg)
            || (MAX98396_R210D_BPE_EN..=MAX98396_R210F_GLOBAL_EN).contains(&reg)
            || reg == MAX98397_R22FF_REVISION_ID
    } else {
        (MAX98396_R2001_INT_RAW1..=MAX98396_R2004_INT_RAW4).contains(&reg)
            || (MAX98396_R2006_INT_STATE1..=MAX98396_R2009_INT_STATE4).contains(&reg)
            || (MAX98396_R200B_INT_FLAG1..=MAX98396_R200E_INT_FLAG4).contains(&reg)
            || (MAX98396_R2010_INT_EN1..=MAX98396_R2013_INT_EN4).contains(&reg)
            || (MAX98396_R2015_INT_FLAG_CLR1..=MAX98396_R2018_INT_FLAG_CLR4).contains(&reg)
            || (MAX98396_R201F_IRQ_CTRL..=MAX98396_R2024_THERM_FOLDBACK_SET).contains(&reg)
            || reg == MAX98396_R2027_THERM_FOLDBACK_EN
            || reg == MAX98396_R2030_NOISEGATE_MODE_CTRL
            || reg == MAX98396_R2033_NOISEGATE_MODE_EN
            || (MAX98396_R2038_CLK_MON_CTRL..=MAX98396_R2039_DATA_MON_CTRL).contains(&reg)
            || (MAX98396_R203F_ENABLE_CTRLS..=MAX98396_R2053_PCM_TX_HIZ_CTRL_8).contains(&reg)
            || (MAX98396_R2055_PCM_RX_SRC1..=MAX98396_R2056_PCM_RX_SRC2).contains(&reg)
            || reg == MAX98396_R2058_PCM_BYPASS_SRC
            || (MAX98396_R205D_PCM_TX_SRC_EN..=MAX98396_R205F_PCM_TX_EN).contains(&reg)
            || (MAX98396_R2070_ICC_RX_EN_A..=MAX98396_R2072_ICC_TX_CTRL).contains(&reg)
            || reg == MAX98396_R207F_ICC_EN
            || (MAX98396_R2083_TONE_GEN_DC_CFG..=MAX98396_R2086_TONE_GEN_DC_LVL3).contains(&reg)
            || (MAX98396_R208F_TONE_GEN_EN..=MAX98396_R209A_SPK_EDGE_CTRL).contains(&reg)
            || (MAX98396_R209C_SPK_EDGE_CTRL1..=MAX98396_R20A0_AMP_SUPPLY_CTL).contains(&reg)
            || (MAX98396_R20AF_AMP_EN..=MAX98396_R20BF_ADC_LO_VBAT_READBACK_LSB).contains(&reg)
            || reg == MAX98396_R20C7_ADC_CFG
            || (MAX98396_R20D0_DHT_CFG1..=MAX98396_R20D6_DHT_HYSTERESIS_CFG).contains(&reg)
            || reg == MAX98396_R20DF_DHT_EN
            || reg == MAX98396_R20E0_IV_SENSE_PATH_CFG
            || (MAX98396_R20E4_IV_SENSE_PATH_EN..=MAX98396_R2106_BPE_THRESH_HYSTERESIS).contains(&reg)
            || (MAX98396_R2108_BPE_SUPPLY_SRC..=MAX98396_R210B_BPE_LOW_LIMITER).contains(&reg)
            || (MAX98396_R210D_BPE_EN..=MAX98396_R210F_GLOBAL_EN).contains(&reg)
            || reg == MAX98396_R21FF_REVISION_ID
    }
}

unsafe fn max98396_readable_register(_dev: *mut device, reg: c_uint) -> bool { max98396_pcm_register_readable(reg, false) }
unsafe fn max98397_readable_register(_dev: *mut device, reg: c_uint) -> bool { max98396_pcm_register_readable(reg, true) }

unsafe fn max98396_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == MAX98396_R2000_SW_RESET
        || (MAX98396_R2001_INT_RAW1..=MAX98396_R200E_INT_FLAG4).contains(&reg)
        || reg == MAX98396_R2041_PCM_MODE_CFG
        || (MAX98396_R20B6_ADC_PVDD_READBACK_MSB..=MAX98396_R20BF_ADC_LO_VBAT_READBACK_LSB).contains(&reg)
        || reg == MAX98396_R20E5_BPE_STATE
        || (MAX98396_R2109_BPE_LOW_STATE..=MAX98396_R210B_BPE_LOW_LIMITER).contains(&reg)
        || reg == MAX98396_R210F_GLOBAL_EN
        || reg == MAX98396_R21FF_REVISION_ID
}

unsafe fn max98397_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    (MAX98396_R2001_INT_RAW1..=MAX98396_R200E_INT_FLAG4).contains(&reg)
        || reg == MAX98396_R2041_PCM_MODE_CFG
        || (MAX98397_R20B7_ADC_PVDD_READBACK_MSB..=MAX98397_R20C4_ADC_LO_VDDH_READBACK_LSB).contains(&reg)
        || reg == MAX98396_R20E5_BPE_STATE
        || (MAX98396_R2109_BPE_LOW_STATE..=MAX98396_R210B_BPE_LOW_LIMITER).contains(&reg)
        || reg == MAX98396_R210F_GLOBAL_EN
        || reg == MAX98397_R22FF_REVISION_ID
}

static max98396_op_mod_text: [&[u8]; 3] = [b"DG\0", b"PVDD\0", b"VBAT\0"];
static max98396_switch_text: [&[u8]; 3] = [b"Left\0", b"Right\0", b"LeftRight\0"];
static max98396_thermal_hyteresis_text: [&[u8]; 4] = [b"2C\0", b"5C\0", b"7C\0", b"10C\0"];
static max98396_foldback_slope_text: [&[u8]; 4] = [b"0.25\0", b"0.5\0", b"1.0\0", b"2.0\0"];
static max98396_foldback_reltime_text: [&[u8]; 4] = [b"3ms\0", b"10ms\0", b"100ms\0", b"300ms\0"];
static max98396_foldback_holdtime_text: [&[u8]; 4] = [b"0ms\0", b"20ms\0", b"40ms\0", b"80ms\0"];
static max98396_thermal_thresh_text: [&[u8]; 101] = [
    b"50C\0", b"51C\0", b"52C\0", b"53C\0", b"54C\0", b"55C\0", b"56C\0", b"57C\0",
    b"58C\0", b"59C\0", b"60C\0", b"61C\0", b"62C\0", b"63C\0", b"64C\0", b"65C\0",
    b"66C\0", b"67C\0", b"68C\0", b"69C\0", b"70C\0", b"71C\0", b"72C\0", b"73C\0",
    b"74C\0", b"75C\0", b"76C\0", b"77C\0", b"78C\0", b"79C\0", b"80C\0", b"81C\0",
    b"82C\0", b"83C\0", b"84C\0", b"85C\0", b"86C\0", b"87C\0", b"88C\0", b"89C\0",
    b"90C\0", b"91C\0", b"92C\0", b"93C\0", b"94C\0", b"95C\0", b"96C\0", b"97C\0",
    b"98C\0", b"99C\0", b"100C\0", b"101C\0", b"102C\0", b"103C\0", b"104C\0", b"105C\0",
    b"106C\0", b"107C\0", b"108C\0", b"109C\0", b"110C\0", b"111C\0", b"112C\0", b"113C\0",
    b"114C\0", b"115C\0", b"116C\0", b"117C\0", b"118C\0", b"119C\0", b"120C\0", b"121C\0",
    b"122C\0", b"123C\0", b"124C\0", b"125C\0", b"126C\0", b"127C\0", b"128C\0", b"129C\0",
    b"130C\0", b"131C\0", b"132C\0", b"133C\0", b"134C\0", b"135C\0", b"136C\0", b"137C\0",
    b"138C\0", b"139C\0", b"140C\0", b"141C\0", b"142C\0", b"143C\0", b"144C\0", b"145C\0",
    b"146C\0", b"147C\0", b"148C\0", b"149C\0", b"150C\0",
];

unsafe fn max98396_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let max98396 = snd_soc_component_get_drvdata(component);
    let reg = if (*max98396).device_id == CODEC_TYPE_MAX98396 {
        MAX98396_R2055_PCM_RX_SRC1
    } else {
        MAX98397_R2056_PCM_RX_SRC1
    };
    let mut val: c_int = 0;
    regmap_read((*max98396).regmap, reg, &mut val);
    (*ucontrol).value.enumerated.item[0] = val as c_uint;
    0
}

unsafe fn max98396_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let max98396 = snd_soc_component_get_drvdata(component);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item0 = (*ucontrol).value.enumerated.item[0];
    if item0 >= (*e).items {
        return -EINVAL;
    }
    let val = snd_soc_enum_item_to_val(e, item0) << (*e).shift_l;
    let reg = if (*max98396).device_id == CODEC_TYPE_MAX98396 {
        MAX98396_R2055_PCM_RX_SRC1
    } else {
        MAX98397_R2056_PCM_RX_SRC1
    };
    let change = snd_soc_component_test_bits(component, reg, MAX98396_PCM_RX_MASK, val);
    if change != 0 {
        regmap_update_bits((*max98396).regmap, reg, MAX98396_PCM_RX_MASK, val);
    }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, item0, e, ptr::null_mut());
    change
}

unsafe fn max98396_adc_value_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let max98396 = snd_soc_component_get_drvdata(component);
    let mut val: [u8; 2] = [0; 2];
    let mut reg = (*mc).reg;

    /* ADC value is not available if the device is powered down */
    if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
        (*ucontrol).value.integer.value[0] = 0;
        return 0;
    }

    if (*max98396).device_id == CODEC_TYPE_MAX98397 {
        match (*mc).reg {
            MAX98396_R20B6_ADC_PVDD_READBACK_MSB => reg = MAX98397_R20B7_ADC_PVDD_READBACK_MSB,
            MAX98396_R20B8_ADC_VBAT_READBACK_MSB => reg = MAX98397_R20B9_ADC_VBAT_READBACK_MSB,
            MAX98396_R20BA_ADC_TEMP_READBACK_MSB => reg = MAX98397_R20BB_ADC_TEMP_READBACK_MSB,
            _ => {
                (*ucontrol).value.integer.value[0] = 0;
                return 0;
            }
        }
    }

    if regmap_raw_read((*max98396).regmap, reg, val.as_mut_ptr() as *mut c_void, 2) != 0 {
        (*ucontrol).value.integer.value[0] = 0;
        return 0;
    }

    /* ADC readback bits[8:0] rearrangement */
    (*ucontrol).value.integer.value[0] = (((val[0] as c_int) << 1) | ((val[1] as c_int) & 1)) as i64;
    0
}

unsafe fn max98396_reset(max98396: *mut max98396_priv, dev: *mut device) {
    let mut reg: c_int = 0;
    let mut ret = regmap_write((*max98396).regmap, MAX98396_R2000_SW_RESET, 1);
    /* dev_err(dev, "Reset command failed. (ret:%d)\n", ret) when ret != 0 */
    let mut count = 0;
    while count < 3 {
        usleep_range(5000, 6000);
        ret = regmap_read((*max98396).regmap, GET_REG_ADDR_REV_ID((*max98396).device_id), &mut reg);
        if ret == 0 {
            /* dev_info(dev, "Reset completed (retry:%d)\n", count) */
            let _ = dev;
            return;
        }
        count += 1;
    }
    /* dev_err(dev, "Reset failed. (ret:%d)\n", ret) */
    let _ = ret;
}

unsafe fn max98396_suspend(dev: *mut device) -> c_int {
    let max98396 = dev_get_drvdata(dev);
    regcache_cache_only((*max98396).regmap, true);
    regcache_mark_dirty((*max98396).regmap);
    regulator_bulk_disable(MAX98396_NUM_CORE_SUPPLIES, (*max98396).core_supplies.as_mut_ptr());
    if !(*max98396).pvdd.is_null() { regulator_disable((*max98396).pvdd); }
    if !(*max98396).vbat.is_null() { regulator_disable((*max98396).vbat); }
    0
}

unsafe fn max98396_resume(dev: *mut device) -> c_int {
    let max98396 = dev_get_drvdata(dev);
    let mut ret = regulator_bulk_enable(MAX98396_NUM_CORE_SUPPLIES, (*max98396).core_supplies.as_mut_ptr());
    if ret < 0 { return ret; }
    if !(*max98396).pvdd.is_null() {
        ret = regulator_enable((*max98396).pvdd);
        if ret < 0 { goto_err_core_supplies(max98396); return ret; }
    }
    if !(*max98396).vbat.is_null() {
        ret = regulator_enable((*max98396).vbat);
        if ret < 0 {
            if !(*max98396).pvdd.is_null() { regulator_disable((*max98396).pvdd); }
            goto_err_core_supplies(max98396);
            return ret;
        }
    }
    regcache_cache_only((*max98396).regmap, false);
    max98396_reset(max98396, dev);
    ret = regcache_sync((*max98396).regmap);
    if ret < 0 {
        regcache_cache_only((*max98396).regmap, true);
        regcache_mark_dirty((*max98396).regmap);
        if !(*max98396).vbat.is_null() { regulator_disable((*max98396).vbat); }
        if !(*max98396).pvdd.is_null() { regulator_disable((*max98396).pvdd); }
        goto_err_core_supplies(max98396);
        return ret;
    }
    0
}

unsafe fn goto_err_core_supplies(max98396: *mut max98396_priv) {
    regulator_bulk_disable(MAX98396_NUM_CORE_SUPPLIES, (*max98396).core_supplies.as_mut_ptr());
}

unsafe fn max98396_read_device_property(dev: *mut device, max98396: *mut max98396_priv) {
    let mut value: c_int = 0;
    if device_property_read_u32(dev, b"adi,vmon-slot-no\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).v_slot = (value & 0xF) as c_uint;
    } else { (*max98396).v_slot = 0; }
    if device_property_read_u32(dev, b"adi,imon-slot-no\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).i_slot = (value & 0xF) as c_uint;
    } else { (*max98396).i_slot = 1; }
    if device_property_read_u32(dev, b"adi,spkfb-slot-no\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).spkfb_slot = (value & 0xF) as c_uint;
    } else { (*max98396).spkfb_slot = 2; }
    if device_property_read_u32(dev, b"adi,bypass-slot-no\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).bypass_slot = (value & 0xF) as c_uint;
    } else { (*max98396).bypass_slot = 0; }
    (*max98396).dmon_stuck_enable = device_property_read_bool(dev, b"adi,dmon-stuck-enable\0".as_ptr() as *const c_char);
    if device_property_read_u32(dev, b"adi,dmon-stuck-threshold-bits\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).dmon_stuck_threshold = value as c_uint;
    } else { (*max98396).dmon_stuck_threshold = 15; }
    (*max98396).dmon_mag_enable = device_property_read_bool(dev, b"adi,dmon-magnitude-enable\0".as_ptr() as *const c_char);
    if device_property_read_u32(dev, b"adi,dmon-magnitude-threshold-bits\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).dmon_mag_threshold = value as c_uint;
    } else { (*max98396).dmon_mag_threshold = 5; }
    if device_property_read_u32(dev, b"adi,dmon-duration-ms\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98396).dmon_duration = value as c_uint;
    } else { (*max98396).dmon_duration = 64; }
}

unsafe fn max98396_core_supplies_disable(priv_: *mut c_void) {
    let max98396 = priv_ as *mut max98396_priv;
    regulator_bulk_disable(MAX98396_NUM_CORE_SUPPLIES, (*max98396).core_supplies.as_mut_ptr());
}

unsafe fn max98396_supply_disable(r: *mut c_void) {
    regulator_disable(r as *mut regulator);
}

/*
 * The original C file also contains large static reg_default arrays, DAI ops,
 * DAPM widgets/routes, ALSA control tables, regmap_config objects, OF/ACPI/I2C
 * device-id tables, i2c probe registration, and MODULE_* metadata. Those items
 * are generated by Linux/ALSA C macros or depend on external C struct layouts
 * from headers intentionally outside this isolated file. Their source-level
 * intent is preserved here as external dependency items and Rust comments rather
 * than local stub implementations.
 *
 * Original conditional intent:
 * - CONFIG_OF gates max98396_of_match and MODULE_DEVICE_TABLE(of, ...)
 * - CONFIG_ACPI gates max98396_acpi_match and MODULE_DEVICE_TABLE(acpi, ...)
 * - module_i2c_driver(max98396_i2c_driver) registers the i2c driver
 * - MODULE_DESCRIPTION/AUTHOR/LICENSE declare module metadata
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
