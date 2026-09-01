// SPDX-License-Identifier: GPL-2.0-only
/*
 * SSM4567 amplifier audio driver
 *
 * Copyright 2014 Google Chromium project.
 *  Author: Anatol Pomozov <anatol@chromium.org>
 *
 * Based on code copyright/by:
 *   Copyright 2013 Analog Devices Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const fn BIT(nr: c_int) -> c_uint {
    1u32 << (nr as u32)
}

const SSM4567_REG_POWER_CTRL: c_uint = 0x00;
const SSM4567_REG_AMP_SNS_CTRL: c_uint = 0x01;
const SSM4567_REG_DAC_CTRL: c_uint = 0x02;
const SSM4567_REG_DAC_VOLUME: c_uint = 0x03;
const SSM4567_REG_SAI_CTRL_1: c_uint = 0x04;
const SSM4567_REG_SAI_CTRL_2: c_uint = 0x05;
const SSM4567_REG_SAI_PLACEMENT_1: c_uint = 0x06;
const SSM4567_REG_SAI_PLACEMENT_2: c_uint = 0x07;
const SSM4567_REG_SAI_PLACEMENT_3: c_uint = 0x08;
const SSM4567_REG_SAI_PLACEMENT_4: c_uint = 0x09;
const SSM4567_REG_SAI_PLACEMENT_5: c_uint = 0x0a;
const SSM4567_REG_SAI_PLACEMENT_6: c_uint = 0x0b;
const SSM4567_REG_BATTERY_V_OUT: c_uint = 0x0c;
const SSM4567_REG_LIMITER_CTRL_1: c_uint = 0x0d;
const SSM4567_REG_LIMITER_CTRL_2: c_uint = 0x0e;
const SSM4567_REG_LIMITER_CTRL_3: c_uint = 0x0f;
const SSM4567_REG_STATUS_1: c_uint = 0x10;
const SSM4567_REG_STATUS_2: c_uint = 0x11;
const SSM4567_REG_FAULT_CTRL: c_uint = 0x12;
const SSM4567_REG_PDM_CTRL: c_uint = 0x13;
const SSM4567_REG_MCLK_RATIO: c_uint = 0x14;
const SSM4567_REG_BOOST_CTRL_1: c_uint = 0x15;
const SSM4567_REG_BOOST_CTRL_2: c_uint = 0x16;
const SSM4567_REG_SOFT_RESET: c_uint = 0xff;

/* POWER_CTRL */
const SSM4567_POWER_APWDN_EN: c_uint = BIT(7);
const SSM4567_POWER_BSNS_PWDN: c_uint = BIT(6);
const SSM4567_POWER_VSNS_PWDN: c_uint = BIT(5);
const SSM4567_POWER_ISNS_PWDN: c_uint = BIT(4);
const SSM4567_POWER_BOOST_PWDN: c_uint = BIT(3);
const SSM4567_POWER_AMP_PWDN: c_uint = BIT(2);
const SSM4567_POWER_VBAT_ONLY: c_uint = BIT(1);
const SSM4567_POWER_SPWDN: c_uint = BIT(0);

/* DAC_CTRL */
const SSM4567_DAC_HV: c_uint = BIT(7);
const SSM4567_DAC_MUTE: c_uint = BIT(6);
const SSM4567_DAC_HPF: c_uint = BIT(5);
const SSM4567_DAC_LPM: c_uint = BIT(4);
const SSM4567_DAC_FS_MASK: c_uint = 0x7;
const SSM4567_DAC_FS_8000_12000: c_uint = 0x0;
const SSM4567_DAC_FS_16000_24000: c_uint = 0x1;
const SSM4567_DAC_FS_32000_48000: c_uint = 0x2;
const SSM4567_DAC_FS_64000_96000: c_uint = 0x3;
const SSM4567_DAC_FS_128000_192000: c_uint = 0x4;

/* SAI_CTRL_1 */
const SSM4567_SAI_CTRL_1_BCLK: c_uint = BIT(6);
const SSM4567_SAI_CTRL_1_TDM_BLCKS_MASK: c_uint = 0x3 << 4;
const SSM4567_SAI_CTRL_1_TDM_BLCKS_32: c_uint = 0x0 << 4;
const SSM4567_SAI_CTRL_1_TDM_BLCKS_48: c_uint = 0x1 << 4;
const SSM4567_SAI_CTRL_1_TDM_BLCKS_64: c_uint = 0x2 << 4;
const SSM4567_SAI_CTRL_1_FSYNC: c_uint = BIT(3);
const SSM4567_SAI_CTRL_1_LJ: c_uint = BIT(2);
const SSM4567_SAI_CTRL_1_TDM: c_uint = BIT(1);
const SSM4567_SAI_CTRL_1_PDM: c_uint = BIT(0);

/* SAI_CTRL_2 */
const SSM4567_SAI_CTRL_2_AUTO_SLOT: c_uint = BIT(3);
const SSM4567_SAI_CTRL_2_TDM_SLOT_MASK: c_uint = 0x7;
const fn SSM4567_SAI_CTRL_2_TDM_SLOT(x: c_int) -> c_uint {
    x as c_uint
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_PDM: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ssm4567 {
    regmap: *mut regmap,
}

#[repr(C)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
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
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: usize,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    val_bits: c_uint,
    reg_bits: c_uint,
    max_register: c_uint,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    cache_type: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: usize,
}

#[repr(C)]
pub struct i2c_client {
    dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    name: [c_char; 20],
    driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    name: [c_char; 32],
    type_: [c_char; 32],
    compatible: [c_char; 128],
    data: *const c_void,
}

#[repr(C)]
pub struct acpi_device_id {
    id: [c_char; 16],
    driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

extern "C" {
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static ssm4567_reg_defaults: [reg_default; 24] = [
    reg_default { reg: SSM4567_REG_POWER_CTRL, def: 0x81 },
    reg_default { reg: SSM4567_REG_AMP_SNS_CTRL, def: 0x09 },
    reg_default { reg: SSM4567_REG_DAC_CTRL, def: 0x32 },
    reg_default { reg: SSM4567_REG_DAC_VOLUME, def: 0x40 },
    reg_default { reg: SSM4567_REG_SAI_CTRL_1, def: 0x00 },
    reg_default { reg: SSM4567_REG_SAI_CTRL_2, def: 0x08 },
    reg_default { reg: SSM4567_REG_SAI_PLACEMENT_1, def: 0x01 },
    reg_default { reg: SSM4567_REG_SAI_PLACEMENT_2, def: 0x20 },
    reg_default { reg: SSM4567_REG_SAI_PLACEMENT_3, def: 0x32 },
    reg_default { reg: SSM4567_REG_SAI_PLACEMENT_4, def: 0x07 },
    reg_default { reg: SSM4567_REG_SAI_PLACEMENT_5, def: 0x07 },
    reg_default { reg: SSM4567_REG_SAI_PLACEMENT_6, def: 0x07 },
    reg_default { reg: SSM4567_REG_BATTERY_V_OUT, def: 0x00 },
    reg_default { reg: SSM4567_REG_LIMITER_CTRL_1, def: 0xa4 },
    reg_default { reg: SSM4567_REG_LIMITER_CTRL_2, def: 0x73 },
    reg_default { reg: SSM4567_REG_LIMITER_CTRL_3, def: 0x00 },
    reg_default { reg: SSM4567_REG_STATUS_1, def: 0x00 },
    reg_default { reg: SSM4567_REG_STATUS_2, def: 0x00 },
    reg_default { reg: SSM4567_REG_FAULT_CTRL, def: 0x30 },
    reg_default { reg: SSM4567_REG_PDM_CTRL, def: 0x40 },
    reg_default { reg: SSM4567_REG_MCLK_RATIO, def: 0x11 },
    reg_default { reg: SSM4567_REG_BOOST_CTRL_1, def: 0x03 },
    reg_default { reg: SSM4567_REG_BOOST_CTRL_2, def: 0x00 },
    reg_default { reg: SSM4567_REG_SOFT_RESET, def: 0x00 },
];

unsafe extern "C" fn ssm4567_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SSM4567_REG_POWER_CTRL..=SSM4567_REG_BOOST_CTRL_2 => true,
        _ => false,
    }
}

unsafe extern "C" fn ssm4567_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SSM4567_REG_POWER_CTRL..=SSM4567_REG_SAI_PLACEMENT_6
        | SSM4567_REG_LIMITER_CTRL_1..=SSM4567_REG_LIMITER_CTRL_3
        | SSM4567_REG_FAULT_CTRL..=SSM4567_REG_BOOST_CTRL_2
        /* The datasheet states that soft reset register is read-only,
         * but logically it is write-only. */
        | SSM4567_REG_SOFT_RESET => true,
        _ => false,
    }
}

unsafe extern "C" fn ssm4567_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SSM4567_REG_BATTERY_V_OUT
        | SSM4567_REG_STATUS_1..=SSM4567_REG_STATUS_2
        | SSM4567_REG_SOFT_RESET => true,
        _ => false,
    }
}

// static const DECLARE_TLV_DB_MINMAX_MUTE(ssm4567_vol_tlv, -7125, 2400);
static ssm4567_vol_tlv: [c_uint; 0] = [];

// Static control/widget entries below preserve the source declarations; concrete
// macro-expanded layouts are supplied by the future ASoC dependency layer.
static ssm4567_snd_controls: [snd_kcontrol_new; 0] = [];
static ssm4567_amplifier_boost_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ssm4567_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static OUT: &[u8] = b"OUT\0";
static AMPLIFIER_BOOST: &[u8] = b"Amplifier Boost\0";
static SWITCH: &[u8] = b"Switch\0";
static DAC: &[u8] = b"DAC\0";
static CURRENT_SENSE: &[u8] = b"Current Sense\0";
static VOLTAGE_SENSE: &[u8] = b"Voltage Sense\0";
static VBAT_SENSE: &[u8] = b"VBAT Sense\0";
static SENSE: &[u8] = b"Sense\0";
static CAPTURE_SENSE: &[u8] = b"Capture Sense\0";

static ssm4567_routes: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: OUT.as_ptr() as *const c_char, control: ptr::null(), source: AMPLIFIER_BOOST.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: AMPLIFIER_BOOST.as_ptr() as *const c_char, control: SWITCH.as_ptr() as *const c_char, source: DAC.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: OUT.as_ptr() as *const c_char, control: ptr::null(), source: DAC.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: CURRENT_SENSE.as_ptr() as *const c_char, control: ptr::null(), source: SENSE.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: VOLTAGE_SENSE.as_ptr() as *const c_char, control: ptr::null(), source: SENSE.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: VBAT_SENSE.as_ptr() as *const c_char, control: ptr::null(), source: SENSE.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: CAPTURE_SENSE.as_ptr() as *const c_char, control: ptr::null(), source: CURRENT_SENSE.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: CAPTURE_SENSE.as_ptr() as *const c_char, control: ptr::null(), source: VOLTAGE_SENSE.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: CAPTURE_SENSE.as_ptr() as *const c_char, control: ptr::null(), source: VBAT_SENSE.as_ptr() as *const c_char },
];

unsafe extern "C" fn ssm4567_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ssm4567: *mut ssm4567 = snd_soc_component_get_drvdata(component) as *mut ssm4567;
    let rate: c_uint = params_rate(params);
    let dacfs: c_uint;

    if rate >= 8000 && rate <= 12000 {
        dacfs = SSM4567_DAC_FS_8000_12000;
    } else if rate >= 16000 && rate <= 24000 {
        dacfs = SSM4567_DAC_FS_16000_24000;
    } else if rate >= 32000 && rate <= 48000 {
        dacfs = SSM4567_DAC_FS_32000_48000;
    } else if rate >= 64000 && rate <= 96000 {
        dacfs = SSM4567_DAC_FS_64000_96000;
    } else if rate >= 128000 && rate <= 192000 {
        dacfs = SSM4567_DAC_FS_128000_192000;
    } else {
        return -EINVAL;
    }

    regmap_update_bits((*ssm4567).regmap, SSM4567_REG_DAC_CTRL, SSM4567_DAC_FS_MASK, dacfs)
}

unsafe extern "C" fn ssm4567_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let ssm4567: *mut ssm4567 = snd_soc_component_get_drvdata((*dai).component) as *mut ssm4567;
    let val: c_uint;

    val = if mute != 0 { SSM4567_DAC_MUTE } else { 0 };
    regmap_update_bits((*ssm4567).regmap, SSM4567_REG_DAC_CTRL, SSM4567_DAC_MUTE, val)
}

unsafe extern "C" fn ssm4567_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    _slots: c_int,
    width: c_int,
) -> c_int {
    let ssm4567: *mut ssm4567 = snd_soc_dai_get_drvdata(dai) as *mut ssm4567;
    let blcks: c_uint;
    let slot: c_int;
    let mut ret: c_int;

    if tx_mask == 0 {
        return -EINVAL;
    }

    if rx_mask != 0 && rx_mask != tx_mask {
        return -EINVAL;
    }

    slot = tx_mask.trailing_zeros() as c_int;
    if tx_mask != BIT(slot) {
        return -EINVAL;
    }

    match width {
        32 => blcks = SSM4567_SAI_CTRL_1_TDM_BLCKS_32,
        48 => blcks = SSM4567_SAI_CTRL_1_TDM_BLCKS_48,
        64 => blcks = SSM4567_SAI_CTRL_1_TDM_BLCKS_64,
        _ => return -EINVAL,
    }

    ret = regmap_update_bits(
        (*ssm4567).regmap,
        SSM4567_REG_SAI_CTRL_2,
        SSM4567_SAI_CTRL_2_AUTO_SLOT | SSM4567_SAI_CTRL_2_TDM_SLOT_MASK,
        SSM4567_SAI_CTRL_2_TDM_SLOT(slot),
    );
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(
        (*ssm4567).regmap,
        SSM4567_REG_SAI_CTRL_1,
        SSM4567_SAI_CTRL_1_TDM_BLCKS_MASK,
        blcks,
    )
}

unsafe extern "C" fn ssm4567_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let ssm4567: *mut ssm4567 = snd_soc_dai_get_drvdata(dai) as *mut ssm4567;
    let mut ctrl1: c_uint = 0;
    let mut invert_fclk: bool;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            invert_fclk = false;
        }
        SND_SOC_DAIFMT_IB_NF => {
            ctrl1 |= SSM4567_SAI_CTRL_1_BCLK;
            invert_fclk = false;
        }
        SND_SOC_DAIFMT_NB_IF => {
            ctrl1 |= SSM4567_SAI_CTRL_1_FSYNC;
            invert_fclk = true;
        }
        SND_SOC_DAIFMT_IB_IF => {
            ctrl1 |= SSM4567_SAI_CTRL_1_BCLK;
            invert_fclk = true;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_LEFT_J => {
            ctrl1 |= SSM4567_SAI_CTRL_1_LJ;
            invert_fclk = !invert_fclk;
        }
        SND_SOC_DAIFMT_DSP_A => {
            ctrl1 |= SSM4567_SAI_CTRL_1_TDM;
        }
        SND_SOC_DAIFMT_DSP_B => {
            ctrl1 |= SSM4567_SAI_CTRL_1_TDM | SSM4567_SAI_CTRL_1_LJ;
        }
        SND_SOC_DAIFMT_PDM => {
            ctrl1 |= SSM4567_SAI_CTRL_1_PDM;
        }
        _ => return -EINVAL,
    }

    if invert_fclk {
        ctrl1 |= SSM4567_SAI_CTRL_1_FSYNC;
    }

    regmap_update_bits(
        (*ssm4567).regmap,
        SSM4567_REG_SAI_CTRL_1,
        SSM4567_SAI_CTRL_1_BCLK
            | SSM4567_SAI_CTRL_1_FSYNC
            | SSM4567_SAI_CTRL_1_LJ
            | SSM4567_SAI_CTRL_1_TDM
            | SSM4567_SAI_CTRL_1_PDM,
        ctrl1,
    )
}

unsafe extern "C" fn ssm4567_set_power(ssm4567: *mut ssm4567, enable: bool) -> c_int {
    let mut ret: c_int = 0;

    if !enable {
        ret = regmap_update_bits(
            (*ssm4567).regmap,
            SSM4567_REG_POWER_CTRL,
            SSM4567_POWER_SPWDN,
            SSM4567_POWER_SPWDN,
        );
        regcache_mark_dirty((*ssm4567).regmap);
    }

    regcache_cache_only((*ssm4567).regmap, !enable);

    if enable {
        ret = regmap_write((*ssm4567).regmap, SSM4567_REG_SOFT_RESET, 0x00);
        if ret != 0 {
            return ret;
        }

        ret = regmap_update_bits(
            (*ssm4567).regmap,
            SSM4567_REG_POWER_CTRL,
            SSM4567_POWER_SPWDN,
            0x00,
        );
        regcache_sync((*ssm4567).regmap);
    }

    ret
}

unsafe extern "C" fn ssm4567_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let ssm4567: *mut ssm4567 = snd_soc_component_get_drvdata(component) as *mut ssm4567;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) as c_int
                == snd_soc_bias_level::SND_SOC_BIAS_OFF as c_int
            {
                ret = ssm4567_set_power(ssm4567, true);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            ret = ssm4567_set_power(ssm4567, false);
        }
    }

    ret
}

static ssm4567_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ssm4567_hw_params),
    mute_stream: Some(ssm4567_mute),
    set_fmt: Some(ssm4567_set_dai_fmt),
    set_tdm_slot: Some(ssm4567_set_tdm_slot),
    no_capture_mute: 1,
};

static SSM4567_HIFI: &[u8] = b"ssm4567-hifi\0";
static PLAYBACK: &[u8] = b"Playback\0";

static mut ssm4567_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: SSM4567_HIFI.as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: PLAYBACK.as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32,
    },
    capture: snd_soc_pcm_stream {
        stream_name: CAPTURE_SENSE.as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32,
    },
    ops: &ssm4567_dai_ops,
};

static ssm4567_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(ssm4567_set_bias_level),
    controls: ssm4567_snd_controls.as_ptr(),
    num_controls: ssm4567_snd_controls.len(),
    dapm_widgets: ssm4567_dapm_widgets.as_ptr(),
    num_dapm_widgets: ssm4567_dapm_widgets.len(),
    dapm_routes: ssm4567_routes.as_ptr(),
    num_dapm_routes: ssm4567_routes.len(),
    use_pmdown_time: 1,
    endianness: 1,
};

static ssm4567_regmap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 8,
    max_register: SSM4567_REG_SOFT_RESET,
    readable_reg: Some(ssm4567_readable_reg),
    writeable_reg: Some(ssm4567_writeable_reg),
    volatile_reg: Some(ssm4567_volatile_reg),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: ssm4567_reg_defaults.as_ptr(),
    num_reg_defaults: ssm4567_reg_defaults.len(),
};

unsafe extern "C" fn ssm4567_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ssm4567: *mut ssm4567;
    let mut ret: c_int;

    ssm4567 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<ssm4567>(),
        GFP_KERNEL,
    ) as *mut ssm4567;
    if ssm4567.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, ssm4567 as *mut c_void);

    (*ssm4567).regmap = devm_regmap_init_i2c(i2c, &ssm4567_regmap_config);
    if IS_ERR((*ssm4567).regmap as *const c_void) {
        return PTR_ERR((*ssm4567).regmap as *const c_void);
    }

    ret = regmap_write((*ssm4567).regmap, SSM4567_REG_SOFT_RESET, 0x00);
    if ret != 0 {
        return ret;
    }

    ret = ssm4567_set_power(ssm4567, false);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &ssm4567_component_driver,
        &mut ssm4567_dai,
        1,
    )
}

static SSM4567_NAME: [c_char; 20] = [
    b's' as c_char, b's' as c_char, b'm' as c_char, b'4' as c_char,
    b'5' as c_char, b'6' as c_char, b'7' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static ssm4567_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id { name: SSM4567_NAME, driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
// MODULE_DEVICE_TABLE(i2c, ssm4567_i2c_ids);

// #ifdef CONFIG_OF
static ADI_SSM4567_COMPATIBLE: [c_char; 128] = [
    b'a' as c_char, b'd' as c_char, b'i' as c_char, b',' as c_char,
    b's' as c_char, b's' as c_char, b'm' as c_char, b'4' as c_char,
    b'5' as c_char, b'6' as c_char, b'7' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static ssm4567_of_match: [of_device_id; 2] = [
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: ADI_SSM4567_COMPATIBLE,
        data: ptr::null(),
    },
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: [0; 128],
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ssm4567_of_match);
// #endif

// #ifdef CONFIG_ACPI
static INT343B_ID: [c_char; 16] = [
    b'I' as c_char, b'N' as c_char, b'T' as c_char, b'3' as c_char,
    b'4' as c_char, b'3' as c_char, b'B' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static ssm4567_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: INT343B_ID, driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
// MODULE_DEVICE_TABLE(acpi, ssm4567_acpi_match);
// #endif

static SSM4567_DRIVER_NAME: &[u8] = b"ssm4567\0";

static mut ssm4567_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: SSM4567_DRIVER_NAME.as_ptr() as *const c_char,
        of_match_table: ssm4567_of_match.as_ptr(),
        acpi_match_table: ssm4567_acpi_match.as_ptr(),
    },
    probe: Some(ssm4567_i2c_probe),
    id_table: ssm4567_i2c_ids.as_ptr(),
};
// module_i2c_driver(ssm4567_driver);

// MODULE_DESCRIPTION("ASoC SSM4567 driver");
// MODULE_AUTHOR("Anatol Pomozov <anatol@chromium.org>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
