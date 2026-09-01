// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC WM9090 driver
 *
 * Copyright 2009-12 Wolfson Microelectronics
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

/* Dependencies supplied by Linux, ASoC, regmap, and wm9090 headers. */
#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
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
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct driver_name {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_name,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct wm9090_platform_data {
    pub lin1_diff: bool,
    pub lin2_diff: bool,
    pub agc_ena: bool,
    pub agc: [c_uint; 3],
}

pub type snd_soc_bias_level = c_uint;

unsafe extern "C" {
    static WM9090_SOFTWARE_RESET: c_uint;
    static WM9090_POWER_MANAGEMENT_1: c_uint;
    static WM9090_POWER_MANAGEMENT_2: c_uint;
    static WM9090_POWER_MANAGEMENT_3: c_uint;
    static WM9090_CLOCKING_1: c_uint;
    static WM9090_IN1_LINE_CONTROL: c_uint;
    static WM9090_IN2_LINE_CONTROL: c_uint;
    static WM9090_IN1_LINE_INPUT_A_VOLUME: c_uint;
    static WM9090_IN1_LINE_INPUT_B_VOLUME: c_uint;
    static WM9090_IN2_LINE_INPUT_A_VOLUME: c_uint;
    static WM9090_IN2_LINE_INPUT_B_VOLUME: c_uint;
    static WM9090_LEFT_OUTPUT_VOLUME: c_uint;
    static WM9090_RIGHT_OUTPUT_VOLUME: c_uint;
    static WM9090_SPKMIXL_ATTENUATION: c_uint;
    static WM9090_SPKOUT_MIXERS: c_uint;
    static WM9090_CLASSD3: c_uint;
    static WM9090_SPEAKER_VOLUME_LEFT: c_uint;
    static WM9090_OUTPUT_MIXER1: c_uint;
    static WM9090_OUTPUT_MIXER2: c_uint;
    static WM9090_OUTPUT_MIXER3: c_uint;
    static WM9090_OUTPUT_MIXER4: c_uint;
    static WM9090_SPEAKER_MIXER: c_uint;
    static WM9090_ANTIPOP2: c_uint;
    static WM9090_WRITE_SEQUENCER_0: c_uint;
    static WM9090_WRITE_SEQUENCER_1: c_uint;
    static WM9090_WRITE_SEQUENCER_2: c_uint;
    static WM9090_WRITE_SEQUENCER_3: c_uint;
    static WM9090_WRITE_SEQUENCER_4: c_uint;
    static WM9090_WRITE_SEQUENCER_5: c_uint;
    static WM9090_CHARGE_PUMP_1: c_uint;
    static WM9090_DC_SERVO_0: c_uint;
    static WM9090_DC_SERVO_1: c_uint;
    static WM9090_DC_SERVO_3: c_uint;
    static WM9090_DC_SERVO_READBACK_0: c_uint;
    static WM9090_DC_SERVO_READBACK_1: c_uint;
    static WM9090_DC_SERVO_READBACK_2: c_uint;
    static WM9090_ANALOGUE_HP_0: c_uint;
    static WM9090_AGC_CONTROL_0: c_uint;
    static WM9090_AGC_CONTROL_1: c_uint;
    static WM9090_AGC_CONTROL_2: c_uint;
    static WM9090_DCS_CAL_COMPLETE_MASK: c_uint;
    static WM9090_CP_ENA: c_uint;
    static WM9090_HPOUT1L_ENA: c_uint;
    static WM9090_HPOUT1R_ENA: c_uint;
    static WM9090_HPOUT1L_DLY: c_uint;
    static WM9090_HPOUT1R_DLY: c_uint;
    static WM9090_DCS_ENA_CHAN_0: c_uint;
    static WM9090_DCS_ENA_CHAN_1: c_uint;
    static WM9090_DCS_TRIG_STARTUP_1: c_uint;
    static WM9090_DCS_TRIG_STARTUP_0: c_uint;
    static WM9090_HPOUT1R_OUTP: c_uint;
    static WM9090_HPOUT1R_RMV_SHORT: c_uint;
    static WM9090_HPOUT1L_OUTP: c_uint;
    static WM9090_HPOUT1L_RMV_SHORT: c_uint;
    static WM9090_AGC_ENA: c_uint;
    static WM9090_VMID_ENA: c_uint;
    static WM9090_BIAS_ENA: c_uint;
    static WM9090_VMID_RES_MASK: c_uint;
    static WM9090_VMID_RES_SHIFT: c_uint;
    static WM9090_IN1_VU: c_uint;
    static WM9090_IN1A_ZC: c_uint;
    static WM9090_IN1B_ZC: c_uint;
    static WM9090_IN2_VU: c_uint;
    static WM9090_IN2A_ZC: c_uint;
    static WM9090_IN2B_ZC: c_uint;
    static WM9090_SPKOUT_VU: c_uint;
    static WM9090_SPKOUTL_ZC: c_uint;
    static WM9090_HPOUT1_VU: c_uint;
    static WM9090_HPOUT1L_ZC: c_uint;
    static WM9090_HPOUT1R_ZC: c_uint;
    static WM9090_TOCLK_ENA: c_uint;
    static WM9090_MAX_REGISTER: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static ENODEV: c_int;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn msleep(msecs: c_uint);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint)
        -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget_desc,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        control: *const snd_kcontrol_new,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;

    fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> c_uint;
    fn SOC_SINGLE_TLV(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        tlv: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DOUBLE_R_TLV(
        name: *const c_char,
        reg_left: c_uint,
        reg_right: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        tlv: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DOUBLE_R(
        name: *const c_char,
        reg_left: c_uint,
        reg_right: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DAPM_SINGLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_SUPPLY(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        event: *const c_void,
        flags: c_uint,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_PGA(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_MIXER(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_PGA_E(
        name: *const c_char,
        reg: c_int,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
        event: unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int,
        flags: c_int,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget_desc;
    fn module_i2c_driver(driver: *mut i2c_driver);
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static wm9090_reg_defaults: [reg_default; 36] = [
    reg_default { reg: 1, def: 0x0006 },     /* R1   - Power Management (1) */
    reg_default { reg: 2, def: 0x6000 },     /* R2   - Power Management (2) */
    reg_default { reg: 3, def: 0x0000 },     /* R3   - Power Management (3) */
    reg_default { reg: 6, def: 0x01C0 },     /* R6   - Clocking 1 */
    reg_default { reg: 22, def: 0x0003 },    /* R22  - IN1 Line Control */
    reg_default { reg: 23, def: 0x0003 },    /* R23  - IN2 Line Control */
    reg_default { reg: 24, def: 0x0083 },    /* R24  - IN1 Line Input A Volume */
    reg_default { reg: 25, def: 0x0083 },    /* R25  - IN1  Line Input B Volume */
    reg_default { reg: 26, def: 0x0083 },    /* R26  - IN2 Line Input A Volume */
    reg_default { reg: 27, def: 0x0083 },    /* R27  - IN2 Line Input B Volume */
    reg_default { reg: 28, def: 0x002D },    /* R28  - Left Output Volume */
    reg_default { reg: 29, def: 0x002D },    /* R29  - Right Output Volume */
    reg_default { reg: 34, def: 0x0100 },    /* R34  - SPKMIXL Attenuation */
    reg_default { reg: 35, def: 0x0010 },    /* R36  - SPKOUT Mixers */
    reg_default { reg: 37, def: 0x0140 },    /* R37  - ClassD3 */
    reg_default { reg: 38, def: 0x0039 },    /* R38  - Speaker Volume Left */
    reg_default { reg: 45, def: 0x0000 },    /* R45  - Output Mixer1 */
    reg_default { reg: 46, def: 0x0000 },    /* R46  - Output Mixer2 */
    reg_default { reg: 47, def: 0x0100 },    /* R47  - Output Mixer3 */
    reg_default { reg: 48, def: 0x0100 },    /* R48  - Output Mixer4 */
    reg_default { reg: 54, def: 0x0000 },    /* R54  - Speaker Mixer */
    reg_default { reg: 57, def: 0x000D },    /* R57  - AntiPOP2 */
    reg_default { reg: 70, def: 0x0000 },    /* R70  - Write Sequencer 0 */
    reg_default { reg: 71, def: 0x0000 },    /* R71  - Write Sequencer 1 */
    reg_default { reg: 72, def: 0x0000 },    /* R72  - Write Sequencer 2 */
    reg_default { reg: 73, def: 0x0000 },    /* R73  - Write Sequencer 3 */
    reg_default { reg: 74, def: 0x0000 },    /* R74  - Write Sequencer 4 */
    reg_default { reg: 75, def: 0x0000 },    /* R75  - Write Sequencer 5 */
    reg_default { reg: 76, def: 0x1F25 },    /* R76  - Charge Pump 1 */
    reg_default { reg: 85, def: 0x054A },    /* R85  - DC Servo 1 */
    reg_default { reg: 87, def: 0x0000 },    /* R87  - DC Servo 3 */
    reg_default { reg: 96, def: 0x0100 },    /* R96  - Analogue HP 0 */
    reg_default { reg: 98, def: 0x8640 },    /* R98  - AGC Control 0 */
    reg_default { reg: 99, def: 0xC000 },    /* R99  - AGC Control 1 */
    reg_default { reg: 100, def: 0x0200 },   /* R100 - AGC Control 2 */
];

/* This struct is used to save the context */
#[repr(C)]
struct wm9090_priv {
    pdata: wm9090_platform_data,
    regmap: *mut regmap,
}

unsafe extern "C" fn wm9090_volatile(_dev: *mut device, reg: c_uint) -> bool {
    if reg == WM9090_SOFTWARE_RESET
        || reg == WM9090_DC_SERVO_0
        || reg == WM9090_DC_SERVO_READBACK_0
        || reg == WM9090_DC_SERVO_READBACK_1
        || reg == WM9090_DC_SERVO_READBACK_2
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn wm9090_readable(_dev: *mut device, reg: c_uint) -> bool {
    if reg == WM9090_SOFTWARE_RESET
        || reg == WM9090_POWER_MANAGEMENT_1
        || reg == WM9090_POWER_MANAGEMENT_2
        || reg == WM9090_POWER_MANAGEMENT_3
        || reg == WM9090_CLOCKING_1
        || reg == WM9090_IN1_LINE_CONTROL
        || reg == WM9090_IN2_LINE_CONTROL
        || reg == WM9090_IN1_LINE_INPUT_A_VOLUME
        || reg == WM9090_IN1_LINE_INPUT_B_VOLUME
        || reg == WM9090_IN2_LINE_INPUT_A_VOLUME
        || reg == WM9090_IN2_LINE_INPUT_B_VOLUME
        || reg == WM9090_LEFT_OUTPUT_VOLUME
        || reg == WM9090_RIGHT_OUTPUT_VOLUME
        || reg == WM9090_SPKMIXL_ATTENUATION
        || reg == WM9090_SPKOUT_MIXERS
        || reg == WM9090_CLASSD3
        || reg == WM9090_SPEAKER_VOLUME_LEFT
        || reg == WM9090_OUTPUT_MIXER1
        || reg == WM9090_OUTPUT_MIXER2
        || reg == WM9090_OUTPUT_MIXER3
        || reg == WM9090_OUTPUT_MIXER4
        || reg == WM9090_SPEAKER_MIXER
        || reg == WM9090_ANTIPOP2
        || reg == WM9090_WRITE_SEQUENCER_0
        || reg == WM9090_WRITE_SEQUENCER_1
        || reg == WM9090_WRITE_SEQUENCER_2
        || reg == WM9090_WRITE_SEQUENCER_3
        || reg == WM9090_WRITE_SEQUENCER_4
        || reg == WM9090_WRITE_SEQUENCER_5
        || reg == WM9090_CHARGE_PUMP_1
        || reg == WM9090_DC_SERVO_0
        || reg == WM9090_DC_SERVO_1
        || reg == WM9090_DC_SERVO_3
        || reg == WM9090_DC_SERVO_READBACK_0
        || reg == WM9090_DC_SERVO_READBACK_1
        || reg == WM9090_DC_SERVO_READBACK_2
        || reg == WM9090_ANALOGUE_HP_0
        || reg == WM9090_AGC_CONTROL_0
        || reg == WM9090_AGC_CONTROL_1
        || reg == WM9090_AGC_CONTROL_2
    {
        true
    } else {
        false
    }
}

unsafe fn wait_for_dc_servo(component: *mut snd_soc_component) {
    let mut reg: c_uint;
    let mut count: c_int = 0;

    dev_dbg((*component).dev, c"Waiting for DC servo...\n".as_ptr());
    loop {
        count += 1;
        msleep(1);
        reg = snd_soc_component_read(component, WM9090_DC_SERVO_READBACK_0);
        dev_dbg((*component).dev, c"DC servo status: %x\n".as_ptr(), reg);
        if !((reg & WM9090_DCS_CAL_COMPLETE_MASK) != WM9090_DCS_CAL_COMPLETE_MASK
            && count < 1000)
        {
            break;
        }
    }

    if (reg & WM9090_DCS_CAL_COMPLETE_MASK) != WM9090_DCS_CAL_COMPLETE_MASK {
        dev_err((*component).dev, c"Timed out waiting for DC Servo\n".as_ptr());
    }
}

/* static const DECLARE_TLV_DB_RANGE(in_tlv, ...) */
static mut in_tlv: [c_uint; 9] = [
    0,
    0,
    0,
    1,
    3,
    0,
    4,
    6,
    0,
];
/* static const DECLARE_TLV_DB_RANGE(mix_tlv, ...) */
static mut mix_tlv: [c_uint; 6] = [0, 2, 0, 3, 3, 0];
/* static const DECLARE_TLV_DB_SCALE(out_tlv, -5700, 100, 0); */
static mut out_tlv: [c_uint; 3] = [(-5700i32) as c_uint, 100, 0];
/* static const DECLARE_TLV_DB_RANGE(spkboost_tlv, ...) */
static mut spkboost_tlv: [c_uint; 6] = [0, 6, 0, 7, 7, 0];

unsafe fn init_tlv_tables() {
    in_tlv[2] = TLV_DB_SCALE_ITEM(-600, 0, 0);
    in_tlv[5] = TLV_DB_SCALE_ITEM(-350, 350, 0);
    in_tlv[8] = TLV_DB_SCALE_ITEM(600, 600, 0);
    mix_tlv[2] = TLV_DB_SCALE_ITEM(-1200, 300, 0);
    mix_tlv[5] = TLV_DB_SCALE_ITEM(0, 0, 0);
    spkboost_tlv[2] = TLV_DB_SCALE_ITEM(0, 150, 0);
    spkboost_tlv[5] = TLV_DB_SCALE_ITEM(1200, 0, 0);
}

unsafe fn init_wm9090_controls() -> [snd_kcontrol_new; 25] {
    [
        SOC_SINGLE_TLV(c"IN1A Volume".as_ptr(), WM9090_IN1_LINE_INPUT_A_VOLUME, 0, 6, 0, in_tlv.as_ptr()),
        SOC_SINGLE(c"IN1A Switch".as_ptr(), WM9090_IN1_LINE_INPUT_A_VOLUME, 7, 1, 1),
        SOC_SINGLE(c"IN1A ZC Switch".as_ptr(), WM9090_IN1_LINE_INPUT_A_VOLUME, 6, 1, 0),
        SOC_SINGLE_TLV(c"IN2A Volume".as_ptr(), WM9090_IN2_LINE_INPUT_A_VOLUME, 0, 6, 0, in_tlv.as_ptr()),
        SOC_SINGLE(c"IN2A Switch".as_ptr(), WM9090_IN2_LINE_INPUT_A_VOLUME, 7, 1, 1),
        SOC_SINGLE(c"IN2A ZC Switch".as_ptr(), WM9090_IN2_LINE_INPUT_A_VOLUME, 6, 1, 0),
        SOC_SINGLE(c"MIXOUTL Switch".as_ptr(), WM9090_OUTPUT_MIXER3, 8, 1, 1),
        SOC_SINGLE_TLV(c"MIXOUTL IN1A Volume".as_ptr(), WM9090_OUTPUT_MIXER3, 6, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"MIXOUTL IN2A Volume".as_ptr(), WM9090_OUTPUT_MIXER3, 2, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE(c"MIXOUTR Switch".as_ptr(), WM9090_OUTPUT_MIXER4, 8, 1, 1),
        SOC_SINGLE_TLV(c"MIXOUTR IN1A Volume".as_ptr(), WM9090_OUTPUT_MIXER4, 6, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"MIXOUTR IN2A Volume".as_ptr(), WM9090_OUTPUT_MIXER4, 2, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE(c"SPKMIX Switch".as_ptr(), WM9090_SPKMIXL_ATTENUATION, 8, 1, 1),
        SOC_SINGLE_TLV(c"SPKMIX IN1A Volume".as_ptr(), WM9090_SPKMIXL_ATTENUATION, 6, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"SPKMIX IN2A Volume".as_ptr(), WM9090_SPKMIXL_ATTENUATION, 2, 3, 1, mix_tlv.as_ptr()),
        SOC_DOUBLE_R_TLV(c"Headphone Volume".as_ptr(), WM9090_LEFT_OUTPUT_VOLUME, WM9090_RIGHT_OUTPUT_VOLUME, 0, 63, 0, out_tlv.as_ptr()),
        SOC_DOUBLE_R(c"Headphone Switch".as_ptr(), WM9090_LEFT_OUTPUT_VOLUME, WM9090_RIGHT_OUTPUT_VOLUME, 6, 1, 1),
        SOC_DOUBLE_R(c"Headphone ZC Switch".as_ptr(), WM9090_LEFT_OUTPUT_VOLUME, WM9090_RIGHT_OUTPUT_VOLUME, 7, 1, 0),
        SOC_SINGLE_TLV(c"Speaker Volume".as_ptr(), WM9090_SPEAKER_VOLUME_LEFT, 0, 63, 0, out_tlv.as_ptr()),
        SOC_SINGLE(c"Speaker Switch".as_ptr(), WM9090_SPEAKER_VOLUME_LEFT, 6, 1, 1),
        SOC_SINGLE(c"Speaker ZC Switch".as_ptr(), WM9090_SPEAKER_VOLUME_LEFT, 7, 1, 0),
        SOC_SINGLE_TLV(c"Speaker Boost Volume".as_ptr(), WM9090_CLASSD3, 3, 7, 0, spkboost_tlv.as_ptr()),
        SOC_SINGLE(c"__unused0".as_ptr(), 0, 0, 0, 0),
        SOC_SINGLE(c"__unused1".as_ptr(), 0, 0, 0, 0),
        SOC_SINGLE(c"__unused2".as_ptr(), 0, 0, 0, 0),
    ]
}

unsafe fn init_wm9090_in1_se_controls() -> [snd_kcontrol_new; 6] {
    [
        SOC_SINGLE_TLV(c"IN1B Volume".as_ptr(), WM9090_IN1_LINE_INPUT_B_VOLUME, 0, 6, 0, in_tlv.as_ptr()),
        SOC_SINGLE(c"IN1B Switch".as_ptr(), WM9090_IN1_LINE_INPUT_B_VOLUME, 7, 1, 1),
        SOC_SINGLE(c"IN1B ZC Switch".as_ptr(), WM9090_IN1_LINE_INPUT_B_VOLUME, 6, 1, 0),
        SOC_SINGLE_TLV(c"SPKMIX IN1B Volume".as_ptr(), WM9090_SPKMIXL_ATTENUATION, 4, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"MIXOUTL IN1B Volume".as_ptr(), WM9090_OUTPUT_MIXER3, 4, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"MIXOUTR IN1B Volume".as_ptr(), WM9090_OUTPUT_MIXER4, 4, 3, 1, mix_tlv.as_ptr()),
    ]
}

unsafe fn init_wm9090_in2_se_controls() -> [snd_kcontrol_new; 6] {
    [
        SOC_SINGLE_TLV(c"IN2B Volume".as_ptr(), WM9090_IN2_LINE_INPUT_B_VOLUME, 0, 6, 0, in_tlv.as_ptr()),
        SOC_SINGLE(c"IN2B Switch".as_ptr(), WM9090_IN2_LINE_INPUT_B_VOLUME, 7, 1, 1),
        SOC_SINGLE(c"IN2B ZC Switch".as_ptr(), WM9090_IN2_LINE_INPUT_B_VOLUME, 6, 1, 0),
        SOC_SINGLE_TLV(c"SPKMIX IN2B Volume".as_ptr(), WM9090_SPKMIXL_ATTENUATION, 0, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"MIXOUTL IN2B Volume".as_ptr(), WM9090_OUTPUT_MIXER3, 0, 3, 1, mix_tlv.as_ptr()),
        SOC_SINGLE_TLV(c"MIXOUTR IN2B Volume".as_ptr(), WM9090_OUTPUT_MIXER4, 0, 3, 1, mix_tlv.as_ptr()),
    ]
}

unsafe extern "C" fn hp_ev(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut reg = snd_soc_component_read(component, WM9090_ANALOGUE_HP_0);

    if event == SND_SOC_DAPM_POST_PMU {
        snd_soc_component_update_bits(component, WM9090_CHARGE_PUMP_1, WM9090_CP_ENA, WM9090_CP_ENA);
        msleep(5);
        snd_soc_component_update_bits(
            component,
            WM9090_POWER_MANAGEMENT_1,
            WM9090_HPOUT1L_ENA | WM9090_HPOUT1R_ENA,
            WM9090_HPOUT1L_ENA | WM9090_HPOUT1R_ENA,
        );
        reg |= WM9090_HPOUT1L_DLY | WM9090_HPOUT1R_DLY;
        snd_soc_component_write(component, WM9090_ANALOGUE_HP_0, reg);

        /* Start the DC servo.  We don't currently use the
         * ability to save the state since we don't have full
         * control of the analogue paths and they can change
         * DC offsets; see the WM8904 driver for an example of
         * doing so.
         */
        snd_soc_component_write(
            component,
            WM9090_DC_SERVO_0,
            WM9090_DCS_ENA_CHAN_0
                | WM9090_DCS_ENA_CHAN_1
                | WM9090_DCS_TRIG_STARTUP_1
                | WM9090_DCS_TRIG_STARTUP_0,
        );
        wait_for_dc_servo(component);

        reg |= WM9090_HPOUT1R_OUTP
            | WM9090_HPOUT1R_RMV_SHORT
            | WM9090_HPOUT1L_OUTP
            | WM9090_HPOUT1L_RMV_SHORT;
        snd_soc_component_write(component, WM9090_ANALOGUE_HP_0, reg);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        reg &= !(WM9090_HPOUT1L_RMV_SHORT
            | WM9090_HPOUT1L_DLY
            | WM9090_HPOUT1L_OUTP
            | WM9090_HPOUT1R_RMV_SHORT
            | WM9090_HPOUT1R_DLY
            | WM9090_HPOUT1R_OUTP);
        snd_soc_component_write(component, WM9090_ANALOGUE_HP_0, reg);
        snd_soc_component_write(component, WM9090_DC_SERVO_0, 0);
        snd_soc_component_update_bits(
            component,
            WM9090_POWER_MANAGEMENT_1,
            WM9090_HPOUT1L_ENA | WM9090_HPOUT1R_ENA,
            0,
        );
        snd_soc_component_update_bits(component, WM9090_CHARGE_PUMP_1, WM9090_CP_ENA, 0);
    }

    0
}

unsafe fn init_spkmix() -> [snd_kcontrol_new; 4] {
    [
        SOC_DAPM_SINGLE(c"IN1A Switch".as_ptr(), WM9090_SPEAKER_MIXER, 6, 1, 0),
        SOC_DAPM_SINGLE(c"IN1B Switch".as_ptr(), WM9090_SPEAKER_MIXER, 4, 1, 0),
        SOC_DAPM_SINGLE(c"IN2A Switch".as_ptr(), WM9090_SPEAKER_MIXER, 2, 1, 0),
        SOC_DAPM_SINGLE(c"IN2B Switch".as_ptr(), WM9090_SPEAKER_MIXER, 0, 1, 0),
    ]
}

unsafe fn init_spkout() -> [snd_kcontrol_new; 1] {
    [SOC_DAPM_SINGLE(c"Mixer Switch".as_ptr(), WM9090_SPKOUT_MIXERS, 4, 1, 0)]
}

unsafe fn init_mixoutl() -> [snd_kcontrol_new; 4] {
    [
        SOC_DAPM_SINGLE(c"IN1A Switch".as_ptr(), WM9090_OUTPUT_MIXER1, 6, 1, 0),
        SOC_DAPM_SINGLE(c"IN1B Switch".as_ptr(), WM9090_OUTPUT_MIXER1, 4, 1, 0),
        SOC_DAPM_SINGLE(c"IN2A Switch".as_ptr(), WM9090_OUTPUT_MIXER1, 2, 1, 0),
        SOC_DAPM_SINGLE(c"IN2B Switch".as_ptr(), WM9090_OUTPUT_MIXER1, 0, 1, 0),
    ]
}

unsafe fn init_mixoutr() -> [snd_kcontrol_new; 4] {
    [
        SOC_DAPM_SINGLE(c"IN1A Switch".as_ptr(), WM9090_OUTPUT_MIXER2, 6, 1, 0),
        SOC_DAPM_SINGLE(c"IN1B Switch".as_ptr(), WM9090_OUTPUT_MIXER2, 4, 1, 0),
        SOC_DAPM_SINGLE(c"IN2A Switch".as_ptr(), WM9090_OUTPUT_MIXER2, 2, 1, 0),
        SOC_DAPM_SINGLE(c"IN2B Switch".as_ptr(), WM9090_OUTPUT_MIXER2, 0, 1, 0),
    ]
}

unsafe fn init_wm9090_dapm_widgets(
    spkmix: *const snd_kcontrol_new,
    spkout: *const snd_kcontrol_new,
    mixoutl: *const snd_kcontrol_new,
    mixoutr: *const snd_kcontrol_new,
) -> [snd_soc_dapm_widget_desc; 19] {
    [
        SND_SOC_DAPM_INPUT(c"IN1+".as_ptr()),
        SND_SOC_DAPM_INPUT(c"IN1-".as_ptr()),
        SND_SOC_DAPM_INPUT(c"IN2+".as_ptr()),
        SND_SOC_DAPM_INPUT(c"IN2-".as_ptr()),
        SND_SOC_DAPM_SUPPLY(c"OSC".as_ptr(), WM9090_POWER_MANAGEMENT_1, 3, 0, null(), 0),
        SND_SOC_DAPM_PGA(c"IN1A PGA".as_ptr(), WM9090_POWER_MANAGEMENT_2, 7, 0, null(), 0),
        SND_SOC_DAPM_PGA(c"IN1B PGA".as_ptr(), WM9090_POWER_MANAGEMENT_2, 6, 0, null(), 0),
        SND_SOC_DAPM_PGA(c"IN2A PGA".as_ptr(), WM9090_POWER_MANAGEMENT_2, 5, 0, null(), 0),
        SND_SOC_DAPM_PGA(c"IN2B PGA".as_ptr(), WM9090_POWER_MANAGEMENT_2, 4, 0, null(), 0),
        SND_SOC_DAPM_MIXER(c"SPKMIX".as_ptr(), WM9090_POWER_MANAGEMENT_3, 3, 0, spkmix, 4),
        SND_SOC_DAPM_MIXER(c"MIXOUTL".as_ptr(), WM9090_POWER_MANAGEMENT_3, 5, 0, mixoutl, 4),
        SND_SOC_DAPM_MIXER(c"MIXOUTR".as_ptr(), WM9090_POWER_MANAGEMENT_3, 4, 0, mixoutr, 4),
        SND_SOC_DAPM_PGA_E(
            c"HP PGA".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            null(),
            0,
            hp_ev,
            SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD,
        ),
        SND_SOC_DAPM_PGA(c"SPKPGA".as_ptr(), WM9090_POWER_MANAGEMENT_3, 8, 0, null(), 0),
        SND_SOC_DAPM_MIXER(c"SPKOUT".as_ptr(), WM9090_POWER_MANAGEMENT_1, 12, 0, spkout, 1),
        SND_SOC_DAPM_OUTPUT(c"HPR".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"HPL".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"Speaker".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"".as_ptr()),
    ]
}

static audio_map: [snd_soc_dapm_route; 19] = [
    snd_soc_dapm_route { sink: c"IN1A PGA".as_ptr(), control: null(), source: c"IN1+".as_ptr() },
    snd_soc_dapm_route { sink: c"IN2A PGA".as_ptr(), control: null(), source: c"IN2+".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKMIX".as_ptr(), control: c"IN1A Switch".as_ptr(), source: c"IN1A PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKMIX".as_ptr(), control: c"IN2A Switch".as_ptr(), source: c"IN2A PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTL".as_ptr(), control: c"IN1A Switch".as_ptr(), source: c"IN1A PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTL".as_ptr(), control: c"IN2A Switch".as_ptr(), source: c"IN2A PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTR".as_ptr(), control: c"IN1A Switch".as_ptr(), source: c"IN1A PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTR".as_ptr(), control: c"IN2A Switch".as_ptr(), source: c"IN2A PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"HP PGA".as_ptr(), control: null(), source: c"OSC".as_ptr() },
    snd_soc_dapm_route { sink: c"HP PGA".as_ptr(), control: null(), source: c"MIXOUTL".as_ptr() },
    snd_soc_dapm_route { sink: c"HP PGA".as_ptr(), control: null(), source: c"MIXOUTR".as_ptr() },
    snd_soc_dapm_route { sink: c"HPL".as_ptr(), control: null(), source: c"HP PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"HPR".as_ptr(), control: null(), source: c"HP PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKPGA".as_ptr(), control: null(), source: c"OSC".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKPGA".as_ptr(), control: null(), source: c"SPKMIX".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKOUT".as_ptr(), control: c"Mixer Switch".as_ptr(), source: c"SPKPGA".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker".as_ptr(), control: null(), source: c"SPKOUT".as_ptr() },
    snd_soc_dapm_route { sink: null(), control: null(), source: null() },
    snd_soc_dapm_route { sink: null(), control: null(), source: null() },
];

static audio_map_in1_se: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"IN1B PGA".as_ptr(), control: null(), source: c"IN1-".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKMIX".as_ptr(), control: c"IN1B Switch".as_ptr(), source: c"IN1B PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTL".as_ptr(), control: c"IN1B Switch".as_ptr(), source: c"IN1B PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTR".as_ptr(), control: c"IN1B Switch".as_ptr(), source: c"IN1B PGA".as_ptr() },
];

static audio_map_in1_diff: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: c"IN1A PGA".as_ptr(), control: null(), source: c"IN1-".as_ptr() },
];

static audio_map_in2_se: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"IN2B PGA".as_ptr(), control: null(), source: c"IN2-".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKMIX".as_ptr(), control: c"IN2B Switch".as_ptr(), source: c"IN2B PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTL".as_ptr(), control: c"IN2B Switch".as_ptr(), source: c"IN2B PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXOUTR".as_ptr(), control: c"IN2B Switch".as_ptr(), source: c"IN2B PGA".as_ptr() },
];

static audio_map_in2_diff: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: c"IN2A PGA".as_ptr(), control: null(), source: c"IN2-".as_ptr() },
];

unsafe extern "C" fn wm9090_add_controls(component: *mut snd_soc_component) -> c_int {
    let wm9090 = snd_soc_component_get_drvdata(component) as *mut wm9090_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut i: c_int;
    let controls = init_wm9090_controls();
    let in1_controls = init_wm9090_in1_se_controls();
    let in2_controls = init_wm9090_in2_se_controls();
    let spkmix_controls = init_spkmix();
    let spkout_controls = init_spkout();
    let mixoutl_controls = init_mixoutl();
    let mixoutr_controls = init_mixoutr();
    let widgets = init_wm9090_dapm_widgets(
        spkmix_controls.as_ptr(),
        spkout_controls.as_ptr(),
        mixoutl_controls.as_ptr(),
        mixoutr_controls.as_ptr(),
    );

    snd_soc_dapm_new_controls(dapm, widgets.as_ptr(), array_size(&widgets) as c_int);
    snd_soc_dapm_add_routes(dapm, audio_map.as_ptr(), array_size(&audio_map) as c_int);
    snd_soc_add_component_controls(component, controls.as_ptr(), 22);

    if (*wm9090).pdata.lin1_diff {
        snd_soc_dapm_add_routes(dapm, audio_map_in1_diff.as_ptr(), array_size(&audio_map_in1_diff) as c_int);
    } else {
        snd_soc_dapm_add_routes(dapm, audio_map_in1_se.as_ptr(), array_size(&audio_map_in1_se) as c_int);
        snd_soc_add_component_controls(component, in1_controls.as_ptr(), array_size(&in1_controls) as c_int);
    }

    if (*wm9090).pdata.lin2_diff {
        snd_soc_dapm_add_routes(dapm, audio_map_in2_diff.as_ptr(), array_size(&audio_map_in2_diff) as c_int);
    } else {
        snd_soc_dapm_add_routes(dapm, audio_map_in2_se.as_ptr(), array_size(&audio_map_in2_se) as c_int);
        snd_soc_add_component_controls(component, in2_controls.as_ptr(), array_size(&in2_controls) as c_int);
    }

    if (*wm9090).pdata.agc_ena {
        i = 0;
        while (i as usize) < (*wm9090).pdata.agc.len() {
            snd_soc_component_write(
                component,
                WM9090_AGC_CONTROL_0.wrapping_add(i as c_uint),
                (*wm9090).pdata.agc[i as usize],
            );
            i += 1;
        }
        snd_soc_component_update_bits(component, WM9090_POWER_MANAGEMENT_3, WM9090_AGC_ENA, WM9090_AGC_ENA);
    } else {
        snd_soc_component_update_bits(component, WM9090_POWER_MANAGEMENT_3, WM9090_AGC_ENA, 0);
    }

    0
}

/*
 * The machine driver should call this from their set_bias_level; if there
 * isn't one then this can just be set as the set_bias_level function.
 */
unsafe extern "C" fn wm9090_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm9090 = snd_soc_component_get_drvdata(component) as *mut wm9090_priv;
    let dapm = snd_soc_component_to_dapm(component);

    if level == SND_SOC_BIAS_ON {
    } else if level == SND_SOC_BIAS_PREPARE {
        snd_soc_component_update_bits(component, WM9090_ANTIPOP2, WM9090_VMID_ENA, WM9090_VMID_ENA);
        snd_soc_component_update_bits(
            component,
            WM9090_POWER_MANAGEMENT_1,
            WM9090_BIAS_ENA | WM9090_VMID_RES_MASK,
            WM9090_BIAS_ENA | (1u32 << WM9090_VMID_RES_SHIFT),
        );
        msleep(1); /* Probably an overestimate */
    } else if level == SND_SOC_BIAS_STANDBY {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            /* Restore the register cache */
            regcache_sync((*wm9090).regmap);
        }

        /* We keep VMID off during standby since the combination of
         * ground referenced outputs and class D speaker mean that
         * latency is not an issue.
         */
        snd_soc_component_update_bits(
            component,
            WM9090_POWER_MANAGEMENT_1,
            WM9090_BIAS_ENA | WM9090_VMID_RES_MASK,
            0,
        );
        snd_soc_component_update_bits(component, WM9090_ANTIPOP2, WM9090_VMID_ENA, 0);
    } else if level == SND_SOC_BIAS_OFF {
    }

    0
}

unsafe extern "C" fn wm9090_probe(component: *mut snd_soc_component) -> c_int {
    /* Configure some defaults; they will be written out when we
     * bring the bias up.
     */
    snd_soc_component_update_bits(
        component,
        WM9090_IN1_LINE_INPUT_A_VOLUME,
        WM9090_IN1_VU | WM9090_IN1A_ZC,
        WM9090_IN1_VU | WM9090_IN1A_ZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9090_IN1_LINE_INPUT_B_VOLUME,
        WM9090_IN1_VU | WM9090_IN1B_ZC,
        WM9090_IN1_VU | WM9090_IN1B_ZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9090_IN2_LINE_INPUT_A_VOLUME,
        WM9090_IN2_VU | WM9090_IN2A_ZC,
        WM9090_IN2_VU | WM9090_IN2A_ZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9090_IN2_LINE_INPUT_B_VOLUME,
        WM9090_IN2_VU | WM9090_IN2B_ZC,
        WM9090_IN2_VU | WM9090_IN2B_ZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9090_SPEAKER_VOLUME_LEFT,
        WM9090_SPKOUT_VU | WM9090_SPKOUTL_ZC,
        WM9090_SPKOUT_VU | WM9090_SPKOUTL_ZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9090_LEFT_OUTPUT_VOLUME,
        WM9090_HPOUT1_VU | WM9090_HPOUT1L_ZC,
        WM9090_HPOUT1_VU | WM9090_HPOUT1L_ZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9090_RIGHT_OUTPUT_VOLUME,
        WM9090_HPOUT1_VU | WM9090_HPOUT1R_ZC,
        WM9090_HPOUT1_VU | WM9090_HPOUT1R_ZC,
    );

    snd_soc_component_update_bits(component, WM9090_CLOCKING_1, WM9090_TOCLK_ENA, WM9090_TOCLK_ENA);
    wm9090_add_controls(component);

    0
}

static soc_component_dev_wm9090: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm9090_probe),
    set_bias_level: Some(wm9090_set_bias_level),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
};

static wm9090_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: unsafe { WM9090_MAX_REGISTER },
    volatile_reg: Some(wm9090_volatile),
    readable_reg: Some(wm9090_readable),
    cache_type: unsafe { REGCACHE_MAPLE },
    reg_defaults: wm9090_reg_defaults.as_ptr(),
    num_reg_defaults: wm9090_reg_defaults.len() as c_uint,
};

unsafe extern "C" fn wm9090_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm9090: *mut wm9090_priv;
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    wm9090 = devm_kzalloc(
        addr_of_mut!((*i2c).dev),
        size_of::<wm9090_priv>(),
        GFP_KERNEL,
    ) as *mut wm9090_priv;
    if wm9090.is_null() {
        return -ENOMEM;
    }

    (*wm9090).regmap = devm_regmap_init_i2c(i2c, &wm9090_regmap);
    if IS_ERR((*wm9090).regmap as *const c_void) {
        ret = PTR_ERR((*wm9090).regmap as *const c_void);
        dev_err(addr_of_mut!((*i2c).dev), c"Failed to allocate regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = regmap_read((*wm9090).regmap, WM9090_SOFTWARE_RESET, &mut reg);
    if ret < 0 {
        return ret;
    }

    if reg != 0x9093 {
        dev_err(addr_of_mut!((*i2c).dev), c"Device is not a WM9090, ID=%x\n".as_ptr(), reg);
        return -ENODEV;
    }

    ret = regmap_write((*wm9090).regmap, WM9090_SOFTWARE_RESET, 0);
    if ret < 0 {
        return ret;
    }

    if !(*i2c).dev.platform_data.is_null() {
        memcpy(
            addr_of_mut!((*wm9090).pdata) as *mut c_void,
            (*i2c).dev.platform_data as *const c_void,
            size_of::<wm9090_platform_data>(),
        );
    }

    i2c_set_clientdata(i2c, wm9090 as *mut c_void);

    ret = devm_snd_soc_register_component(
        addr_of_mut!((*i2c).dev),
        &soc_component_dev_wm9090,
        null(),
        0,
    );
    if ret != 0 {
        dev_err(addr_of_mut!((*i2c).dev), c"Failed to register CODEC: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static wm9090_id: [i2c_device_id; 3] = [
    i2c_device_id { name: *b"wm9090\0\0\0\0\0\0\0\0\0\0\0\0\0" },
    i2c_device_id { name: *b"wm9093\0\0\0\0\0\0\0\0\0\0\0\0\0" },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, wm9090_id); */

static mut wm9090_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_name {
        name: c"wm9090".as_ptr(),
    },
    probe: Some(wm9090_i2c_probe),
    id_table: wm9090_id.as_ptr(),
};

unsafe fn init_module_driver() {
    init_tlv_tables();
    module_i2c_driver(addr_of_mut!(wm9090_i2c_driver));
}

/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_DESCRIPTION("WM9090 ASoC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
