// SPDX-License-Identifier: GPL-2.0
//
// sgtl5000.c  --  SGTL5000 ALSA SoC Audio driver
//
// Copyright 2010-2011 Freescale Semiconductor, Inc. All Rights Reserved.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type size_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct regulator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client_dev {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: i2c_client_dev,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
    pub consumer: *mut regulator,
}

// External declarations and constants are supplied by Linux/ASoC headers and
// "sgtl5000.h" in the final repository integration.
extern "C" {
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn udelay(usecs: c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn regulator_is_equal(a: *mut regulator, b: *mut regulator) -> bool;
    fn regulator_get_optional(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_put(regulator: *mut regulator);
    fn regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_free(num_consumers: c_int, consumers: *mut regulator_bulk_data);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const c_void, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
}

unsafe extern "C" {
    static SGTL5000_CHIP_DIG_POWER: c_uint;
    static SGTL5000_CHIP_I2S_CTRL: c_uint;
    static SGTL5000_CHIP_SSS_CTRL: c_uint;
    static SGTL5000_CHIP_ADCDAC_CTRL: c_uint;
    static SGTL5000_CHIP_DAC_VOL: c_uint;
    static SGTL5000_CHIP_PAD_STRENGTH: c_uint;
    static SGTL5000_CHIP_ANA_ADC_CTRL: c_uint;
    static SGTL5000_CHIP_ANA_HP_CTRL: c_uint;
    static SGTL5000_CHIP_ANA_CTRL: c_uint;
    static SGTL5000_CHIP_REF_CTRL: c_uint;
    static SGTL5000_CHIP_MIC_CTRL: c_uint;
    static SGTL5000_CHIP_LINE_OUT_CTRL: c_uint;
    static SGTL5000_CHIP_LINE_OUT_VOL: c_uint;
    static SGTL5000_CHIP_PLL_CTRL: c_uint;
    static SGTL5000_CHIP_CLK_TOP_CTRL: c_uint;
    static SGTL5000_CHIP_ANA_STATUS: c_uint;
    static SGTL5000_CHIP_ANA_TEST2: c_uint;
    static SGTL5000_CHIP_SHORT_CTRL: c_uint;
    static SGTL5000_DAP_CTRL: c_uint;
    static SGTL5000_DAP_PEQ: c_uint;
    static SGTL5000_DAP_BASS_ENHANCE: c_uint;
    static SGTL5000_DAP_BASS_ENHANCE_CTRL: c_uint;
    static SGTL5000_DAP_AUDIO_EQ: c_uint;
    static SGTL5000_DAP_SURROUND: c_uint;
    static SGTL5000_DAP_EQ_BASS_BAND0: c_uint;
    static SGTL5000_DAP_EQ_BASS_BAND1: c_uint;
    static SGTL5000_DAP_EQ_BASS_BAND2: c_uint;
    static SGTL5000_DAP_EQ_BASS_BAND3: c_uint;
    static SGTL5000_DAP_EQ_BASS_BAND4: c_uint;
    static SGTL5000_DAP_MAIN_CHAN: c_uint;
    static SGTL5000_DAP_MIX_CHAN: c_uint;
    static SGTL5000_DAP_AVC_CTRL: c_uint;
    static SGTL5000_DAP_AVC_THRESHOLD: c_uint;
    static SGTL5000_DAP_AVC_ATTACK: c_uint;
    static SGTL5000_DAP_AVC_DECAY: c_uint;
    static SGTL5000_CHIP_ANA_POWER: c_uint;
    static SGTL5000_CHIP_ID: c_uint;
}

const SGTL5000_DAP_REG_OFFSET: c_uint = 0x0100;
const SGTL5000_MAX_REG_OFFSET: c_uint = 0x013A;

/* Delay for the VAG ramp up */
const SGTL5000_VAG_POWERUP_DELAY: c_uint = 500; /* ms */
/* Delay for the VAG ramp down */
const SGTL5000_VAG_POWERDOWN_DELAY: c_uint = 500; /* ms */

const LDO_VOLTAGE: c_int = 1200000;
const LINREG_VDDD: c_int = (1600 - LDO_VOLTAGE / 1000) / 50;

const VDDA: usize = 0;
const VDDIO: usize = 1;
const VDDD: usize = 2;
const SGTL5000_SUPPLY_NUM: usize = 3;

const SGTL5000_MICBIAS_OFF: u32 = 0;
const SGTL5000_MICBIAS_2K: u32 = 2;
const SGTL5000_MICBIAS_4K: u32 = 4;
const SGTL5000_MICBIAS_8K: u32 = 8;

const I2S_LRCLK_STRENGTH_DISABLE: u32 = 0;
const I2S_LRCLK_STRENGTH_LOW: u32 = 1;
const I2S_LRCLK_STRENGTH_MEDIUM: u32 = 2;
const I2S_LRCLK_STRENGTH_HIGH: u32 = 3;

const I2S_SCLK_STRENGTH_DISABLE: u32 = 0;
const I2S_SCLK_STRENGTH_LOW: u32 = 1;
const I2S_SCLK_STRENGTH_MEDIUM: u32 = 2;
const I2S_SCLK_STRENGTH_HIGH: u32 = 3;

const HP_POWER_EVENT: usize = 0;
const DAC_POWER_EVENT: usize = 1;
const ADC_POWER_EVENT: usize = 2;
const LAST_POWER_EVENT: usize = ADC_POWER_EVENT;

#[repr(C)]
pub struct sgtl5000_priv {
    pub sysclk: c_int, /* sysclk rate */
    pub master: c_int, /* i2s master or not */
    pub fmt: c_int,    /* i2s data format */
    pub supplies: [regulator_bulk_data; SGTL5000_SUPPLY_NUM],
    pub num_supplies: c_int,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub revision: c_int,
    pub micbias_resistor: u8,
    pub micbias_voltage: u8,
    pub lrclk_strength: u8,
    pub sclk_strength: u8,
    pub mute_state: [u16; LAST_POWER_EVENT + 1],
}

/* default value of sgtl5000 registers */
static sgtl5000_reg_defaults: [reg_default; 35] = unsafe {
    [
        reg_default { reg: SGTL5000_CHIP_DIG_POWER, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_I2S_CTRL, def: 0x0010 },
        reg_default { reg: SGTL5000_CHIP_SSS_CTRL, def: 0x0010 },
        reg_default { reg: SGTL5000_CHIP_ADCDAC_CTRL, def: 0x020c },
        reg_default { reg: SGTL5000_CHIP_DAC_VOL, def: 0x3c3c },
        reg_default { reg: SGTL5000_CHIP_PAD_STRENGTH, def: 0x015f },
        reg_default { reg: SGTL5000_CHIP_ANA_ADC_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_ANA_HP_CTRL, def: 0x1818 },
        reg_default { reg: SGTL5000_CHIP_ANA_CTRL, def: 0x0111 },
        reg_default { reg: SGTL5000_CHIP_REF_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_MIC_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_LINE_OUT_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_LINE_OUT_VOL, def: 0x0404 },
        reg_default { reg: SGTL5000_CHIP_PLL_CTRL, def: 0x5000 },
        reg_default { reg: SGTL5000_CHIP_CLK_TOP_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_ANA_STATUS, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_ANA_TEST2, def: 0x0000 },
        reg_default { reg: SGTL5000_CHIP_SHORT_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_DAP_CTRL, def: 0x0000 },
        reg_default { reg: SGTL5000_DAP_PEQ, def: 0x0000 },
        reg_default { reg: SGTL5000_DAP_BASS_ENHANCE, def: 0x0040 },
        reg_default { reg: SGTL5000_DAP_BASS_ENHANCE_CTRL, def: 0x051f },
        reg_default { reg: SGTL5000_DAP_AUDIO_EQ, def: 0x0000 },
        reg_default { reg: SGTL5000_DAP_SURROUND, def: 0x0040 },
        reg_default { reg: SGTL5000_DAP_EQ_BASS_BAND0, def: 0x002f },
        reg_default { reg: SGTL5000_DAP_EQ_BASS_BAND1, def: 0x002f },
        reg_default { reg: SGTL5000_DAP_EQ_BASS_BAND2, def: 0x002f },
        reg_default { reg: SGTL5000_DAP_EQ_BASS_BAND3, def: 0x002f },
        reg_default { reg: SGTL5000_DAP_EQ_BASS_BAND4, def: 0x002f },
        reg_default { reg: SGTL5000_DAP_MAIN_CHAN, def: 0x8000 },
        reg_default { reg: SGTL5000_DAP_MIX_CHAN, def: 0x0000 },
        reg_default { reg: SGTL5000_DAP_AVC_CTRL, def: 0x5100 },
        reg_default { reg: SGTL5000_DAP_AVC_THRESHOLD, def: 0x1473 },
        reg_default { reg: SGTL5000_DAP_AVC_ATTACK, def: 0x0028 },
        reg_default { reg: SGTL5000_DAP_AVC_DECAY, def: 0x0050 },
    ]
};

/* AVC: Threshold dB -> register: pre-calculated values */
static avc_thr_db2reg: [u16; 97] = [
    0x5168, 0x488E, 0x40AA, 0x39A1, 0x335D, 0x2DC7, 0x28CC, 0x245D, 0x2068,
    0x1CE2, 0x19BE, 0x16F1, 0x1472, 0x1239, 0x103E, 0x0E7A, 0x0CE6, 0x0B7F,
    0x0A3F, 0x0922, 0x0824, 0x0741, 0x0677, 0x05C3, 0x0522, 0x0493, 0x0414,
    0x03A2, 0x033D, 0x02E3, 0x0293, 0x024B, 0x020B, 0x01D2, 0x019F, 0x0172,
    0x014A, 0x0126, 0x0106, 0x00E9, 0x00D0, 0x00B9, 0x00A5, 0x0093, 0x0083,
    0x0075, 0x0068, 0x005D, 0x0052, 0x0049, 0x0041, 0x003A, 0x0034, 0x002E,
    0x0029, 0x0025, 0x0021, 0x001D, 0x001A, 0x0017, 0x0014, 0x0012, 0x0010,
    0x000E, 0x000D, 0x000B, 0x000A, 0x0009, 0x0008, 0x0007, 0x0006, 0x0005,
    0x0005, 0x0004, 0x0004, 0x0003, 0x0003, 0x0002, 0x0002, 0x0002, 0x0002,
    0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0001, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
];

static supply_names: [*const c_char; SGTL5000_SUPPLY_NUM] = [
    b"VDDA\0".as_ptr() as *const c_char,
    b"VDDIO\0".as_ptr() as *const c_char,
    b"VDDD\0".as_ptr() as *const c_char,
];

#[inline]
unsafe fn hp_sel_input(component: *mut snd_soc_component) -> c_int {
    ((snd_soc_component_read(component, SGTL5000_CHIP_ANA_CTRL) & SGTL5000_HP_SEL_MASK)
        >> SGTL5000_HP_SEL_SHIFT) as c_int
}

#[inline]
unsafe fn mute_output(component: *mut snd_soc_component, mute_mask: u16) -> u16 {
    let mute_reg = snd_soc_component_read(component, SGTL5000_CHIP_ANA_CTRL) as u16;
    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_ANA_CTRL,
        mute_mask as c_uint,
        mute_mask as c_uint,
    );
    mute_reg
}

#[inline]
unsafe fn restore_output(component: *mut snd_soc_component, mute_mask: u16, mute_reg: u16) {
    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_ANA_CTRL,
        mute_mask as c_uint,
        mute_reg as c_uint,
    );
}

unsafe fn vag_power_on(component: *mut snd_soc_component, source: u32) {
    if (snd_soc_component_read(component, SGTL5000_CHIP_ANA_POWER) & SGTL5000_VAG_POWERUP) != 0 {
        return;
    }

    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_ANA_POWER,
        SGTL5000_VAG_POWERUP,
        SGTL5000_VAG_POWERUP,
    );

    /* When VAG powering on to get local loop from Line-In, the sleep
     * is required to avoid loud pop.
     */
    if hp_sel_input(component) == SGTL5000_HP_SEL_LINE_IN as c_int && source == HP_POWER_EVENT as u32 {
        msleep(SGTL5000_VAG_POWERUP_DELAY);
    }
}

unsafe fn vag_power_consumers(component: *mut snd_soc_component, ana_pwr_reg: u16, source: u32) -> c_int {
    let mut consumers = 0;

    /* count dac/adc consumers unconditional */
    if (ana_pwr_reg as c_uint & SGTL5000_DAC_POWERUP) != 0 {
        consumers += 1;
    }
    if (ana_pwr_reg as c_uint & SGTL5000_ADC_POWERUP) != 0 {
        consumers += 1;
    }

    /*
     * If the event comes from HP and Line-In is selected,
     * current action is 'DAC to be powered down'.
     * As HP_POWERUP is not set when HP muxed to line-in,
     * we need to keep VAG power ON.
     */
    if source == HP_POWER_EVENT as u32 {
        if hp_sel_input(component) == SGTL5000_HP_SEL_LINE_IN as c_int {
            consumers += 1;
        }
    } else if (ana_pwr_reg as c_uint & SGTL5000_HP_POWERUP) != 0 {
        consumers += 1;
    }

    consumers
}

unsafe fn vag_power_off(component: *mut snd_soc_component, source: u32) {
    let ana_pwr = snd_soc_component_read(component, SGTL5000_CHIP_ANA_POWER) as u16;

    if (ana_pwr as c_uint & SGTL5000_VAG_POWERUP) == 0 {
        return;
    }

    /*
     * This function calls when any of VAG power consumers is disappearing.
     * Thus, if there is more than one consumer at the moment, as minimum
     * one consumer will definitely stay after the end of the current
     * event.
     * Don't clear VAG_POWERUP if 2 or more consumers of VAG present:
     * - LINE_IN (for HP events) / HP (for DAC/ADC events)
     * - DAC
     * - ADC
     * (the current consumer is disappearing right now)
     */
    if vag_power_consumers(component, ana_pwr, source) >= 2 {
        return;
    }

    snd_soc_component_update_bits(component, SGTL5000_CHIP_ANA_POWER, SGTL5000_VAG_POWERUP, 0);
    /* In power down case, we need wait 400-1000 ms
     * when VAG fully ramped down.
     * As longer we wait, as smaller pop we've got.
     */
    msleep(SGTL5000_VAG_POWERDOWN_DELAY);
}

unsafe extern "C" fn mic_bias_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;

    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            /* change mic bias resistor */
            snd_soc_component_update_bits(
                component,
                SGTL5000_CHIP_MIC_CTRL,
                SGTL5000_BIAS_R_MASK,
                ((*sgtl5000).micbias_resistor as c_uint) << SGTL5000_BIAS_R_SHIFT,
            );
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, SGTL5000_CHIP_MIC_CTRL, SGTL5000_BIAS_R_MASK, 0);
        }
        _ => {}
    }
    0
}

unsafe fn vag_and_mute_control(
    component: *mut snd_soc_component,
    event: c_int,
    event_source: c_int,
) -> c_int {
    static mute_mask: [u16; 3] = [
        /* Mask for HP_POWER_EVENT. */
        SGTL5000_HP_MUTE as u16,
        /* Masks for DAC_POWER_EVENT/ADC_POWER_EVENT. */
        SGTL5000_OUTPUTS_MUTE as u16,
        SGTL5000_OUTPUTS_MUTE as u16,
    ];

    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;

    match event as c_uint {
        SND_SOC_DAPM_PRE_PMU => {
            (*sgtl5000).mute_state[event_source as usize] =
                mute_output(component, mute_mask[event_source as usize]);
        }
        SND_SOC_DAPM_POST_PMU => {
            vag_power_on(component, event_source as u32);
            restore_output(
                component,
                mute_mask[event_source as usize],
                (*sgtl5000).mute_state[event_source as usize],
            );
        }
        SND_SOC_DAPM_PRE_PMD => {
            (*sgtl5000).mute_state[event_source as usize] =
                mute_output(component, mute_mask[event_source as usize]);
            vag_power_off(component, event_source as u32);
        }
        SND_SOC_DAPM_POST_PMD => {
            restore_output(
                component,
                mute_mask[event_source as usize],
                (*sgtl5000).mute_state[event_source as usize],
            );
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn headphone_pga_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    vag_and_mute_control(component, event, HP_POWER_EVENT as c_int)
}

unsafe extern "C" fn adc_updown_depop(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    vag_and_mute_control(component, event, ADC_POWER_EVENT as c_int)
}

unsafe extern "C" fn dac_updown_depop(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    vag_and_mute_control(component, event, DAC_POWER_EVENT as c_int)
}

/* input sources and DAPM/control macro declarations from ASoC:
 * adc_mux_text, hp_mux_text, dac_mux_text, dap_mux_text, dapmix_mux_text;
 * SOC_ENUM_SINGLE_DECL(...), SOC_DAPM_ENUM(...);
 * sgtl5000_dapm_widgets[]; sgtl5000_dapm_routes[].
 * Their concrete Rust representation depends on external ASoC macro bindings.
 */

unsafe extern "C" fn dac_info_volsw(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (0xfc - 0x3c) as i64;
    0
}

unsafe extern "C" fn dac_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let reg = snd_soc_component_read(component, SGTL5000_CHIP_DAC_VOL) as c_int;
    let mut l: c_int;
    let mut r: c_int;

    /* get left channel volume */
    l = ((reg as c_uint & SGTL5000_DAC_VOL_LEFT_MASK) >> SGTL5000_DAC_VOL_LEFT_SHIFT) as c_int;

    /* get right channel volume */
    r = ((reg as c_uint & SGTL5000_DAC_VOL_RIGHT_MASK) >> SGTL5000_DAC_VOL_RIGHT_SHIFT) as c_int;

    /* make sure value fall in (0x3c,0xfc) */
    l = clamp(l, 0x3c, 0xfc);
    r = clamp(r, 0x3c, 0xfc);

    /* invert it and map to userspace value */
    l = 0xfc - l;
    r = 0xfc - r;

    (*ucontrol).value.integer.value[0] = l as i64;
    (*ucontrol).value.integer.value[1] = r as i64;

    0
}

unsafe extern "C" fn dac_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut l = (*ucontrol).value.integer.value[0] as c_int;
    let mut r = (*ucontrol).value.integer.value[1] as c_int;

    /* make sure userspace volume fall in (0, 0xfc-0x3c) */
    l = clamp(l, 0, 0xfc - 0x3c);
    r = clamp(r, 0, 0xfc - 0x3c);

    /* invert it, get the value can be set to register */
    l = 0xfc - l;
    r = 0xfc - r;

    /* shift to get the register value */
    let reg = ((l as c_uint) << SGTL5000_DAC_VOL_LEFT_SHIFT)
        | ((r as c_uint) << SGTL5000_DAC_VOL_RIGHT_SHIFT);

    snd_soc_component_write(component, SGTL5000_CHIP_DAC_VOL, reg);

    0
}

unsafe extern "C" fn avc_get_threshold(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut i: usize;
    let reg = snd_soc_component_read(component, SGTL5000_DAP_AVC_THRESHOLD) as u16;

    /* register value 0 => -96dB */
    if reg == 0 {
        (*ucontrol).value.integer.value[0] = 96;
        (*ucontrol).value.integer.value[1] = 96;
        return 0;
    }

    /* get dB from register value (rounded down) */
    i = 0;
    while avc_thr_db2reg[i] > reg {
        i += 1;
    }
    let db = i as c_int;

    (*ucontrol).value.integer.value[0] = db as i64;
    (*ucontrol).value.integer.value[1] = db as i64;

    0
}

unsafe extern "C" fn avc_put_threshold(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let db = (*ucontrol).value.integer.value[0] as c_int;
    if db < 0 || db > 96 {
        return -EINVAL;
    }
    let reg = avc_thr_db2reg[db as usize];
    snd_soc_component_write(component, SGTL5000_DAP_AVC_THRESHOLD, reg as c_uint);

    0
}

/* TLV declarations and sgtl5000_snd_controls[] are ASoC macro-generated:
 * DECLARE_TLV_DB_SCALE/RANGE/MINMAX and SOC_* control initializers.
 */

unsafe extern "C" fn sgtl5000_mute_stream(
    codec_dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let i2s_pwr: u16 = SGTL5000_I2S_IN_POWERUP as u16;

    /*
     * During 'digital mute' do not mute DAC
     * because LINE_IN would be muted aswell. We want to mute
     * only I2S block - this can be done by powering it off
     */
    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_DIG_POWER,
        i2s_pwr as c_uint,
        if mute != 0 { 0 } else { i2s_pwr as c_uint },
    );

    0
}

unsafe extern "C" fn sgtl5000_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;
    let mut i2sctl: u16 = 0;

    (*sgtl5000).master = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP => {
            i2sctl |= SGTL5000_I2S_MASTER as u16;
            (*sgtl5000).master = 1;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            i2sctl |= (SGTL5000_I2S_MODE_PCM << SGTL5000_I2S_MODE_SHIFT) as u16;
        }
        SND_SOC_DAIFMT_DSP_B => {
            i2sctl |= (SGTL5000_I2S_MODE_PCM << SGTL5000_I2S_MODE_SHIFT) as u16;
            i2sctl |= SGTL5000_I2S_LRALIGN as u16;
        }
        SND_SOC_DAIFMT_I2S => {
            i2sctl |= (SGTL5000_I2S_MODE_I2S_LJ << SGTL5000_I2S_MODE_SHIFT) as u16;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            i2sctl |= (SGTL5000_I2S_MODE_RJ << SGTL5000_I2S_MODE_SHIFT) as u16;
            i2sctl |= SGTL5000_I2S_LRPOL as u16;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            i2sctl |= (SGTL5000_I2S_MODE_I2S_LJ << SGTL5000_I2S_MODE_SHIFT) as u16;
            i2sctl |= SGTL5000_I2S_LRALIGN as u16;
        }
        _ => return -EINVAL,
    }

    (*sgtl5000).fmt = (fmt & SND_SOC_DAIFMT_FORMAT_MASK) as c_int;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => {
            i2sctl |= SGTL5000_I2S_SCLK_INV as u16;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, SGTL5000_CHIP_I2S_CTRL, i2sctl as c_uint);

    0
}

unsafe extern "C" fn sgtl5000_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;

    match clk_id as c_uint {
        SGTL5000_SYSCLK => {
            (*sgtl5000).sysclk = freq as c_int;
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn sgtl5000_set_clock(component: *mut snd_soc_component, frame_rate: c_int) -> c_int {
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;
    let mut clk_ctl: c_int = 0;
    let sys_fs: c_int;

    match frame_rate {
        8000 | 16000 => sys_fs = 32000,
        11025 | 22050 => sys_fs = 44100,
        _ => sys_fs = frame_rate,
    }

    match sys_fs / frame_rate {
        4 => clk_ctl |= (SGTL5000_RATE_MODE_DIV_4 << SGTL5000_RATE_MODE_SHIFT) as c_int,
        2 => clk_ctl |= (SGTL5000_RATE_MODE_DIV_2 << SGTL5000_RATE_MODE_SHIFT) as c_int,
        1 => clk_ctl |= (SGTL5000_RATE_MODE_DIV_1 << SGTL5000_RATE_MODE_SHIFT) as c_int,
        _ => return -EINVAL,
    }

    match sys_fs {
        32000 => clk_ctl |= (SGTL5000_SYS_FS_32k << SGTL5000_SYS_FS_SHIFT) as c_int,
        44100 => clk_ctl |= (SGTL5000_SYS_FS_44_1k << SGTL5000_SYS_FS_SHIFT) as c_int,
        48000 => clk_ctl |= (SGTL5000_SYS_FS_48k << SGTL5000_SYS_FS_SHIFT) as c_int,
        96000 => clk_ctl |= (SGTL5000_SYS_FS_96k << SGTL5000_SYS_FS_SHIFT) as c_int,
        _ => {
            /* dev_err(component->dev, "frame rate %d not supported\n", frame_rate); */
            return -EINVAL;
        }
    }

    match (*sgtl5000).sysclk / frame_rate {
        256 => clk_ctl |= (SGTL5000_MCLK_FREQ_256FS << SGTL5000_MCLK_FREQ_SHIFT) as c_int,
        384 => clk_ctl |= (SGTL5000_MCLK_FREQ_384FS << SGTL5000_MCLK_FREQ_SHIFT) as c_int,
        512 => clk_ctl |= (SGTL5000_MCLK_FREQ_512FS << SGTL5000_MCLK_FREQ_SHIFT) as c_int,
        _ => {
            if (*sgtl5000).master != 0 {
                clk_ctl |= (SGTL5000_MCLK_FREQ_PLL << SGTL5000_MCLK_FREQ_SHIFT) as c_int;
            } else {
                return -EINVAL;
            }
        }
    }

    if (clk_ctl as c_uint & SGTL5000_MCLK_FREQ_MASK) == SGTL5000_MCLK_FREQ_PLL {
        let mut out: u64;
        let mut t: u64;
        let div2: c_int;
        let pll_ctl: c_int;
        let in_: c_uint;
        let int_div: c_uint;
        let frac_div: c_uint;

        if (*sgtl5000).sysclk > 17000000 {
            div2 = 1;
            in_ = ((*sgtl5000).sysclk / 2) as c_uint;
        } else {
            div2 = 0;
            in_ = (*sgtl5000).sysclk as c_uint;
        }
        if sys_fs == 44100 {
            out = 180633600;
        } else {
            out = 196608000;
        }
        t = out % in_ as u64;
        out /= in_ as u64;
        int_div = out as c_uint;
        t *= 2048;
        t %= in_ as u64;
        frac_div = t as c_uint;
        pll_ctl = ((int_div << SGTL5000_PLL_INT_DIV_SHIFT)
            | (frac_div << SGTL5000_PLL_FRAC_DIV_SHIFT)) as c_int;

        snd_soc_component_write(component, SGTL5000_CHIP_PLL_CTRL, pll_ctl as c_uint);
        if div2 != 0 {
            snd_soc_component_update_bits(
                component,
                SGTL5000_CHIP_CLK_TOP_CTRL,
                SGTL5000_INPUT_FREQ_DIV2,
                SGTL5000_INPUT_FREQ_DIV2,
            );
        } else {
            snd_soc_component_update_bits(component, SGTL5000_CHIP_CLK_TOP_CTRL, SGTL5000_INPUT_FREQ_DIV2, 0);
        }

        snd_soc_component_update_bits(
            component,
            SGTL5000_CHIP_ANA_POWER,
            SGTL5000_PLL_POWERUP | SGTL5000_VCOAMP_POWERUP,
            SGTL5000_PLL_POWERUP | SGTL5000_VCOAMP_POWERUP,
        );

        snd_soc_component_write(component, SGTL5000_CHIP_CLK_CTRL, clk_ctl as c_uint);
    } else {
        snd_soc_component_write(component, SGTL5000_CHIP_CLK_CTRL, clk_ctl as c_uint);
        snd_soc_component_update_bits(
            component,
            SGTL5000_CHIP_ANA_POWER,
            SGTL5000_PLL_POWERUP | SGTL5000_VCOAMP_POWERUP,
            0,
        );
    }

    0
}

unsafe extern "C" fn sgtl5000_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;
    let channels = params_channels(params);
    let mut i2s_ctl: c_int = 0;
    let stereo: c_uint;
    let ret: c_int;

    if (*sgtl5000).sysclk == 0 {
        return -EFAULT;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int {
        stereo = SGTL5000_DAC_STEREO;
    } else {
        stereo = SGTL5000_ADC_STEREO;
    }

    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_ANA_POWER,
        stereo,
        if channels == 1 { 0 } else { stereo },
    );

    ret = sgtl5000_set_clock(component, params_rate(params));
    if ret != 0 {
        return ret;
    }

    match params_width(params) {
        16 => {
            if (*sgtl5000).fmt == SND_SOC_DAIFMT_RIGHT_J as c_int {
                return -EINVAL;
            }
            i2s_ctl |= (SGTL5000_I2S_DLEN_16 << SGTL5000_I2S_DLEN_SHIFT) as c_int;
            i2s_ctl |= (SGTL5000_I2S_SCLKFREQ_32FS << SGTL5000_I2S_SCLKFREQ_SHIFT) as c_int;
        }
        20 => {
            i2s_ctl |= (SGTL5000_I2S_DLEN_20 << SGTL5000_I2S_DLEN_SHIFT) as c_int;
            i2s_ctl |= (SGTL5000_I2S_SCLKFREQ_64FS << SGTL5000_I2S_SCLKFREQ_SHIFT) as c_int;
        }
        24 => {
            i2s_ctl |= (SGTL5000_I2S_DLEN_24 << SGTL5000_I2S_DLEN_SHIFT) as c_int;
            i2s_ctl |= (SGTL5000_I2S_SCLKFREQ_64FS << SGTL5000_I2S_SCLKFREQ_SHIFT) as c_int;
        }
        32 => {
            if (*sgtl5000).fmt == SND_SOC_DAIFMT_RIGHT_J as c_int {
                return -EINVAL;
            }
            i2s_ctl |= (SGTL5000_I2S_DLEN_32 << SGTL5000_I2S_DLEN_SHIFT) as c_int;
            i2s_ctl |= (SGTL5000_I2S_SCLKFREQ_64FS << SGTL5000_I2S_SCLKFREQ_SHIFT) as c_int;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_I2S_CTRL,
        SGTL5000_I2S_DLEN_MASK | SGTL5000_I2S_SCLKFREQ_MASK,
        i2s_ctl as c_uint,
    );

    0
}

unsafe extern "C" fn sgtl5000_set_bias_level(
    component: *mut snd_soc_component,
    level: c_uint,
) -> c_int {
    let sgtl = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;
    let ret: c_int;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE | SND_SOC_BIAS_STANDBY => {
            regcache_cache_only((*sgtl).regmap, false);
            ret = regcache_sync((*sgtl).regmap);
            if ret != 0 {
                regcache_cache_only((*sgtl).regmap, true);
                return ret;
            }
            snd_soc_component_update_bits(
                component,
                SGTL5000_CHIP_ANA_POWER,
                SGTL5000_REFTOP_POWERUP,
                SGTL5000_REFTOP_POWERUP,
            );
        }
        SND_SOC_BIAS_OFF => {
            regcache_cache_only((*sgtl).regmap, true);
            snd_soc_component_update_bits(component, SGTL5000_CHIP_ANA_POWER, SGTL5000_REFTOP_POWERUP, 0);
        }
        _ => {}
    }

    0
}

const SGTL5000_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* sgtl5000_ops and sgtl5000_dai are translated from snd_soc_dai_ops and
 * snd_soc_dai_driver initializers; concrete struct layout is external.
 */

unsafe extern "C" fn sgtl5000_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r == SGTL5000_CHIP_ID || r == SGTL5000_CHIP_ADCDAC_CTRL || r == SGTL5000_CHIP_ANA_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn sgtl5000_readable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r == SGTL5000_CHIP_ID
            || r == SGTL5000_CHIP_DIG_POWER
            || r == SGTL5000_CHIP_CLK_CTRL
            || r == SGTL5000_CHIP_I2S_CTRL
            || r == SGTL5000_CHIP_SSS_CTRL
            || r == SGTL5000_CHIP_ADCDAC_CTRL
            || r == SGTL5000_CHIP_DAC_VOL
            || r == SGTL5000_CHIP_PAD_STRENGTH
            || r == SGTL5000_CHIP_ANA_ADC_CTRL
            || r == SGTL5000_CHIP_ANA_HP_CTRL
            || r == SGTL5000_CHIP_ANA_CTRL
            || r == SGTL5000_CHIP_LINREG_CTRL
            || r == SGTL5000_CHIP_REF_CTRL
            || r == SGTL5000_CHIP_MIC_CTRL
            || r == SGTL5000_CHIP_LINE_OUT_CTRL
            || r == SGTL5000_CHIP_LINE_OUT_VOL
            || r == SGTL5000_CHIP_ANA_POWER
            || r == SGTL5000_CHIP_PLL_CTRL
            || r == SGTL5000_CHIP_CLK_TOP_CTRL
            || r == SGTL5000_CHIP_ANA_STATUS
            || r == SGTL5000_CHIP_SHORT_CTRL
            || r == SGTL5000_CHIP_ANA_TEST2
            || r == SGTL5000_DAP_CTRL
            || r == SGTL5000_DAP_PEQ
            || r == SGTL5000_DAP_BASS_ENHANCE
            || r == SGTL5000_DAP_BASS_ENHANCE_CTRL
            || r == SGTL5000_DAP_AUDIO_EQ
            || r == SGTL5000_DAP_SURROUND
            || r == SGTL5000_DAP_FLT_COEF_ACCESS
            || r == SGTL5000_DAP_COEF_WR_B0_MSB
            || r == SGTL5000_DAP_COEF_WR_B0_LSB
            || r == SGTL5000_DAP_EQ_BASS_BAND0
            || r == SGTL5000_DAP_EQ_BASS_BAND1
            || r == SGTL5000_DAP_EQ_BASS_BAND2
            || r == SGTL5000_DAP_EQ_BASS_BAND3
            || r == SGTL5000_DAP_EQ_BASS_BAND4
            || r == SGTL5000_DAP_MAIN_CHAN
            || r == SGTL5000_DAP_MIX_CHAN
            || r == SGTL5000_DAP_AVC_CTRL
            || r == SGTL5000_DAP_AVC_THRESHOLD
            || r == SGTL5000_DAP_AVC_ATTACK
            || r == SGTL5000_DAP_AVC_DECAY
            || r == SGTL5000_DAP_COEF_WR_B1_MSB
            || r == SGTL5000_DAP_COEF_WR_B1_LSB
            || r == SGTL5000_DAP_COEF_WR_B2_MSB
            || r == SGTL5000_DAP_COEF_WR_B2_LSB
            || r == SGTL5000_DAP_COEF_WR_A1_MSB
            || r == SGTL5000_DAP_COEF_WR_A1_LSB
            || r == SGTL5000_DAP_COEF_WR_A2_MSB
            || r == SGTL5000_DAP_COEF_WR_A2_LSB => true,
        _ => false,
    }
}

static vol_quot_table: [u8; 32] = [
    42, 45, 47, 50, 53, 56, 60, 63, 67, 71, 75, 79, 84, 89, 94, 100, 106, 112,
    119, 126, 133, 141, 150, 158, 168, 178, 188, 200, 211, 224, 237, 251,
];

unsafe fn sgtl5000_set_power_regs(component: *mut snd_soc_component) -> c_int {
    let mut vdda: c_int;
    let mut vddio: c_int;
    let mut vddd: c_int;
    let mut ana_pwr: u16;
    let mut lreg_ctrl: u16;
    let mut vag: c_int;
    let mut lo_vag: c_int;
    let vol_quot: c_int;
    let mut lo_vol: c_int;
    let mut i: size_t;
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;

    vdda = regulator_get_voltage((*sgtl5000).supplies[VDDA].consumer);
    vddio = regulator_get_voltage((*sgtl5000).supplies[VDDIO].consumer);
    vddd = if (*sgtl5000).num_supplies > VDDD as c_int {
        regulator_get_voltage((*sgtl5000).supplies[VDDD].consumer)
    } else {
        LDO_VOLTAGE
    };

    vdda /= 1000;
    vddio /= 1000;
    vddd /= 1000;

    if vdda <= 0 || vddio <= 0 || vddd < 0 {
        return -EINVAL;
    }

    if vdda > 3600 || vddio > 3600 || vddd > 1980 {
        return -EINVAL;
    }

    ana_pwr = snd_soc_component_read(component, SGTL5000_CHIP_ANA_POWER) as u16;
    ana_pwr |= (SGTL5000_DAC_STEREO | SGTL5000_ADC_STEREO | SGTL5000_REFTOP_POWERUP) as u16;
    lreg_ctrl = snd_soc_component_read(component, SGTL5000_CHIP_LINREG_CTRL) as u16;

    if vddio < 3100 && vdda < 3100 {
        snd_soc_component_update_bits(
            component,
            SGTL5000_CHIP_CLK_TOP_CTRL,
            SGTL5000_INT_OSC_EN,
            SGTL5000_INT_OSC_EN,
        );
        ana_pwr |= SGTL5000_VDDC_CHRGPMP_POWERUP as u16;
    } else {
        ana_pwr &= !(SGTL5000_VDDC_CHRGPMP_POWERUP as u16);
        if regulator_is_equal((*sgtl5000).supplies[VDDA].consumer, (*sgtl5000).supplies[VDDIO].consumer) {
            lreg_ctrl |= SGTL5000_VDDC_ASSN_OVRD as u16;
            lreg_ctrl |= (SGTL5000_VDDC_MAN_ASSN_VDDIO << SGTL5000_VDDC_MAN_ASSN_SHIFT) as u16;
        }
    }

    snd_soc_component_write(component, SGTL5000_CHIP_LINREG_CTRL, lreg_ctrl as c_uint);
    snd_soc_component_write(component, SGTL5000_CHIP_ANA_POWER, ana_pwr as c_uint);

    vag = vdda / 2;
    if vag <= SGTL5000_ANA_GND_BASE as c_int {
        vag = 0;
    } else if vag
        >= (SGTL5000_ANA_GND_BASE
            + SGTL5000_ANA_GND_STP * (SGTL5000_ANA_GND_MASK >> SGTL5000_ANA_GND_SHIFT)) as c_int
    {
        vag = (SGTL5000_ANA_GND_MASK >> SGTL5000_ANA_GND_SHIFT) as c_int;
    } else {
        vag = (vag - SGTL5000_ANA_GND_BASE as c_int) / SGTL5000_ANA_GND_STP as c_int;
    }

    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_REF_CTRL,
        SGTL5000_ANA_GND_MASK,
        (vag as c_uint) << SGTL5000_ANA_GND_SHIFT,
    );

    lo_vag = vddio / 2;
    if lo_vag <= SGTL5000_LINE_OUT_GND_BASE as c_int {
        lo_vag = 0;
    } else if lo_vag
        >= (SGTL5000_LINE_OUT_GND_BASE + SGTL5000_LINE_OUT_GND_STP * SGTL5000_LINE_OUT_GND_MAX) as c_int
    {
        lo_vag = SGTL5000_LINE_OUT_GND_MAX as c_int;
    } else {
        lo_vag =
            (lo_vag - SGTL5000_LINE_OUT_GND_BASE as c_int) / SGTL5000_LINE_OUT_GND_STP as c_int;
    }

    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_LINE_OUT_CTRL,
        SGTL5000_LINE_OUT_CURRENT_MASK | SGTL5000_LINE_OUT_GND_MASK,
        ((lo_vag as c_uint) << SGTL5000_LINE_OUT_GND_SHIFT)
            | (SGTL5000_LINE_OUT_CURRENT_360u << SGTL5000_LINE_OUT_CURRENT_SHIFT),
    );

    vol_quot = if lo_vag != 0 { (vag * 100) / lo_vag } else { 0 };
    lo_vol = 0;
    i = 0;
    while i < vol_quot_table.len() {
        if vol_quot >= vol_quot_table[i] as c_int {
            lo_vol = i as c_int;
        } else {
            break;
        }
        i += 1;
    }

    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_LINE_OUT_VOL,
        SGTL5000_LINE_OUT_VOL_RIGHT_MASK | SGTL5000_LINE_OUT_VOL_LEFT_MASK,
        ((lo_vol as c_uint) << SGTL5000_LINE_OUT_VOL_RIGHT_SHIFT)
            | ((lo_vol as c_uint) << SGTL5000_LINE_OUT_VOL_LEFT_SHIFT),
    );

    0
}

unsafe fn sgtl5000_enable_regulators(client: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut external_vddd = 0;
    let vddd: *mut regulator;
    let sgtl5000 = i2c_get_clientdata(client) as *mut sgtl5000_priv;

    i = 0;
    while (i as usize) < (*sgtl5000).supplies.len() {
        (*sgtl5000).supplies[i as usize].supply = supply_names[i as usize];
        i += 1;
    }

    vddd = regulator_get_optional(&mut (*client).dev as *mut _ as *mut device, b"VDDD\0".as_ptr() as *const c_char);
    if IS_ERR(vddd as *mut c_void) {
        if PTR_ERR(vddd as *mut c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
    } else {
        external_vddd = 1;
        regulator_put(vddd);
    }

    (*sgtl5000).num_supplies = (*sgtl5000).supplies.len() as c_int - 1 + external_vddd;
    ret = regulator_bulk_get(
        &mut (*client).dev as *mut _ as *mut device,
        (*sgtl5000).num_supplies,
        (*sgtl5000).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        return ret;
    }

    ret = regulator_bulk_enable((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    if ret == 0 {
        usleep_range(10, 20);
    } else {
        regulator_bulk_free((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    }

    ret
}

unsafe extern "C" fn sgtl5000_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let reg: u16;
    let sgtl5000 = snd_soc_component_get_drvdata(component) as *mut sgtl5000_priv;
    let zcd_mask: c_uint = SGTL5000_HP_ZCD_EN | SGTL5000_ADC_ZCD_EN;

    ret = sgtl5000_set_power_regs(component);
    if ret != 0 {
        return ret;
    }

    snd_soc_component_update_bits(component, SGTL5000_CHIP_REF_CTRL, SGTL5000_SMALL_POP, SGTL5000_SMALL_POP);
    snd_soc_component_write(component, SGTL5000_CHIP_SHORT_CTRL, 0);
    snd_soc_component_write(component, SGTL5000_CHIP_DIG_POWER, SGTL5000_ADC_EN | SGTL5000_DAC_EN);
    snd_soc_component_write(
        component,
        SGTL5000_CHIP_ADCDAC_CTRL,
        SGTL5000_DAC_VOL_RAMP_EN | SGTL5000_DAC_MUTE_RIGHT | SGTL5000_DAC_MUTE_LEFT,
    );

    reg = (((*sgtl5000).lrclk_strength as c_uint) << SGTL5000_PAD_I2S_LRCLK_SHIFT
        | ((*sgtl5000).sclk_strength as c_uint) << SGTL5000_PAD_I2S_SCLK_SHIFT
        | 0x1f) as u16;
    snd_soc_component_write(component, SGTL5000_CHIP_PAD_STRENGTH, reg as c_uint);

    snd_soc_component_update_bits(component, SGTL5000_CHIP_ANA_CTRL, zcd_mask, zcd_mask);
    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_MIC_CTRL,
        SGTL5000_BIAS_R_MASK,
        ((*sgtl5000).micbias_resistor as c_uint) << SGTL5000_BIAS_R_SHIFT,
    );
    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_MIC_CTRL,
        SGTL5000_BIAS_VOLT_MASK,
        ((*sgtl5000).micbias_voltage as c_uint) << SGTL5000_BIAS_VOLT_SHIFT,
    );
    snd_soc_component_write(component, SGTL5000_DAP_AUDIO_EQ, SGTL5000_DAP_SEL_GEQ);
    snd_soc_component_update_bits(
        component,
        SGTL5000_CHIP_ADCDAC_CTRL,
        SGTL5000_DAC_MUTE_LEFT | SGTL5000_DAC_MUTE_RIGHT,
        0,
    );

    0
}

unsafe extern "C" fn sgtl5000_of_xlate_dai_id(
    _component: *mut snd_soc_component,
    _endpoint: *mut device_node,
) -> c_int {
    /* return dai id 0, whatever the endpoint index */
    0
}

/* sgtl5000_driver and sgtl5000_regmap are struct initializers whose concrete
 * Rust layouts are external to the isolated file.
 */

unsafe fn sgtl5000_fill_defaults(client: *mut i2c_client) {
    let sgtl5000 = i2c_get_clientdata(client) as *mut sgtl5000_priv;
    let mut i: c_int = 0;

    while (i as usize) < sgtl5000_reg_defaults.len() {
        let val = sgtl5000_reg_defaults[i as usize].def;
        let index = sgtl5000_reg_defaults[i as usize].reg;
        let ret = regmap_write((*sgtl5000).regmap, index, val);
        if ret != 0 {
            /* dev_err(&client->dev, "%s: error %d setting reg ...", __func__, ret, index, val); */
        }
        i += 1;
    }
}

unsafe extern "C" fn sgtl5000_i2c_probe(client: *mut i2c_client) -> c_int {
    let sgtl5000: *mut sgtl5000_priv;
    let mut ret: c_int;
    let mut reg: c_uint = 0;
    let rev: c_int;
    let np = (*client).dev.of_node;
    let mut value: u32 = 0;
    let mut ana_pwr: u16;

    sgtl5000 = devm_kzalloc(
        &mut (*client).dev as *mut _ as *mut device,
        core::mem::size_of::<sgtl5000_priv>(),
        GFP_KERNEL,
    ) as *mut sgtl5000_priv;
    if sgtl5000.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(client, sgtl5000 as *mut c_void);

    ret = sgtl5000_enable_regulators(client);
    if ret != 0 {
        return ret;
    }

    (*sgtl5000).regmap = devm_regmap_init_i2c(client, core::ptr::addr_of!(sgtl5000_regmap) as *const c_void);
    if IS_ERR((*sgtl5000).regmap as *mut c_void) {
        ret = PTR_ERR((*sgtl5000).regmap as *mut c_void);
        goto_disable_regs(client, sgtl5000, ret);
        return ret;
    }

    (*sgtl5000).mclk = devm_clk_get(&mut (*client).dev as *mut _ as *mut device, core::ptr::null());
    if IS_ERR((*sgtl5000).mclk as *mut c_void) {
        ret = PTR_ERR((*sgtl5000).mclk as *mut c_void);
        if ret == -ENOENT {
            ret = -EPROBE_DEFER;
        }
        goto_disable_regs(client, sgtl5000, ret);
        return ret;
    }

    ret = clk_prepare_enable((*sgtl5000).mclk);
    if ret != 0 {
        goto_disable_regs(client, sgtl5000, ret);
        return ret;
    }

    udelay(1);

    ret = regmap_read((*sgtl5000).regmap, SGTL5000_CHIP_ID, &mut reg);
    if ret != 0 {
        goto_disable_clk(client, sgtl5000, ret);
        return ret;
    }

    if ((reg & SGTL5000_PARTID_MASK) >> SGTL5000_PARTID_SHIFT) != SGTL5000_PARTID_PART_ID {
        ret = -ENODEV;
        goto_disable_clk(client, sgtl5000, ret);
        return ret;
    }

    rev = ((reg & SGTL5000_REVID_MASK) >> SGTL5000_REVID_SHIFT) as c_int;
    (*sgtl5000).revision = rev;

    ret = regmap_write((*sgtl5000).regmap, SGTL5000_CHIP_CLK_CTRL, SGTL5000_CHIP_CLK_CTRL_DEFAULT);
    if ret != 0 {}

    ret = regmap_write((*sgtl5000).regmap, SGTL5000_CHIP_ANA_CTRL, SGTL5000_CHIP_ANA_CTRL_DEFAULT);
    if ret != 0 {
        goto_disable_clk(client, sgtl5000, ret);
        return ret;
    }

    ret = regmap_read((*sgtl5000).regmap, SGTL5000_CHIP_ANA_POWER, &mut value);
    if ret != 0 {
        goto_disable_clk(client, sgtl5000, ret);
        return ret;
    }
    if (value & SGTL5000_VAG_POWERUP) != 0 {
        ret = regmap_update_bits((*sgtl5000).regmap, SGTL5000_CHIP_ANA_POWER, SGTL5000_VAG_POWERUP, 0);
        if ret != 0 {
            goto_disable_clk(client, sgtl5000, ret);
            return ret;
        }
        msleep(SGTL5000_VAG_POWERDOWN_DELAY);
    }

    ana_pwr = SGTL5000_ANA_POWER_DEFAULT as u16;
    if (*sgtl5000).num_supplies <= VDDD as c_int {
        ret = regmap_update_bits(
            (*sgtl5000).regmap,
            SGTL5000_CHIP_LINREG_CTRL,
            SGTL5000_LINREG_VDDD_MASK,
            LINREG_VDDD as c_uint,
        );
        if ret != 0 {}
        ana_pwr |= SGTL5000_LINEREG_D_POWERUP as u16;
    } else {
        ana_pwr &= !((SGTL5000_STARTUP_POWERUP | SGTL5000_LINREG_SIMPLE_POWERUP) as u16);
    }
    ret = regmap_write((*sgtl5000).regmap, SGTL5000_CHIP_ANA_POWER, ana_pwr as c_uint);
    if ret != 0 {}

    if !np.is_null() {
        if of_property_read_u32(np, b"micbias-resistor-k-ohms\0".as_ptr() as *const c_char, &mut value) == 0 {
            match value {
                SGTL5000_MICBIAS_OFF => (*sgtl5000).micbias_resistor = 0,
                SGTL5000_MICBIAS_2K => (*sgtl5000).micbias_resistor = 1,
                SGTL5000_MICBIAS_4K => (*sgtl5000).micbias_resistor = 2,
                SGTL5000_MICBIAS_8K => (*sgtl5000).micbias_resistor = 3,
                _ => (*sgtl5000).micbias_resistor = 2,
            }
        } else {
            (*sgtl5000).micbias_resistor = 2;
        }
        if of_property_read_u32(np, b"micbias-voltage-m-volts\0".as_ptr() as *const c_char, &mut value) == 0 {
            if value >= 1250 && value <= 3000 {
                (*sgtl5000).micbias_voltage = (value / 250 - 5) as u8;
            } else {
                (*sgtl5000).micbias_voltage = 0;
            }
        } else {
            (*sgtl5000).micbias_voltage = 0;
        }
    }

    (*sgtl5000).lrclk_strength = I2S_LRCLK_STRENGTH_LOW as u8;
    if of_property_read_u32(np, b"lrclk-strength\0".as_ptr() as *const c_char, &mut value) == 0 {
        if value > I2S_LRCLK_STRENGTH_HIGH {
            value = I2S_LRCLK_STRENGTH_LOW;
        }
        (*sgtl5000).lrclk_strength = value as u8;
    }

    (*sgtl5000).sclk_strength = I2S_SCLK_STRENGTH_LOW as u8;
    if of_property_read_u32(np, b"sclk-strength\0".as_ptr() as *const c_char, &mut value) == 0 {
        if value > I2S_SCLK_STRENGTH_HIGH {
            value = I2S_SCLK_STRENGTH_LOW;
        }
        (*sgtl5000).sclk_strength = value as u8;
    }

    sgtl5000_fill_defaults(client);

    ret = devm_snd_soc_register_component(
        &mut (*client).dev as *mut _ as *mut device,
        core::ptr::addr_of!(sgtl5000_driver) as *const c_void,
        core::ptr::addr_of_mut!(sgtl5000_dai) as *mut c_void,
        1,
    );
    if ret != 0 {
        goto_disable_clk(client, sgtl5000, ret);
        return ret;
    }

    0
}

unsafe fn goto_disable_clk(client: *mut i2c_client, sgtl5000: *mut sgtl5000_priv, ret: c_int) {
    let _ = ret;
    clk_disable_unprepare((*sgtl5000).mclk);
    regulator_bulk_disable((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    regulator_bulk_free((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    let _ = client;
}

unsafe fn goto_disable_regs(client: *mut i2c_client, sgtl5000: *mut sgtl5000_priv, ret: c_int) {
    let _ = ret;
    regulator_bulk_disable((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    regulator_bulk_free((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    let _ = client;
}

unsafe extern "C" fn sgtl5000_i2c_remove(client: *mut i2c_client) {
    let sgtl5000 = i2c_get_clientdata(client) as *mut sgtl5000_priv;

    regmap_write((*sgtl5000).regmap, SGTL5000_CHIP_CLK_CTRL, SGTL5000_CHIP_CLK_CTRL_DEFAULT);
    regmap_write((*sgtl5000).regmap, SGTL5000_CHIP_DIG_POWER, SGTL5000_DIG_POWER_DEFAULT);
    regmap_write((*sgtl5000).regmap, SGTL5000_CHIP_ANA_POWER, SGTL5000_ANA_POWER_DEFAULT);

    clk_disable_unprepare((*sgtl5000).mclk);
    regulator_bulk_disable((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
    regulator_bulk_free((*sgtl5000).num_supplies, (*sgtl5000).supplies.as_mut_ptr());
}

unsafe extern "C" fn sgtl5000_i2c_shutdown(client: *mut i2c_client) {
    sgtl5000_i2c_remove(client);
}

/* i2c_device_id sgtl5000_id[], MODULE_DEVICE_TABLE(i2c, ...),
 * of_device_id sgtl5000_dt_ids[], MODULE_DEVICE_TABLE(of, ...),
 * i2c_driver sgtl5000_i2c_driver, module_i2c_driver(...),
 * MODULE_DESCRIPTION/AUTHOR/LICENSE are preserved as module registration intent.
 */

#[inline]
fn clamp(v: c_int, lo: c_int, hi: c_int) -> c_int {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

#[inline]
unsafe fn IS_ERR(ptr: *mut c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn PTR_ERR(ptr: *mut c_void) -> c_int {
    ptr as isize as c_int
}

/* Placeholder extern constants/items for names supplied by included headers and
 * external macro expansions. These are declarations, not implementations.
 */
extern "C" {
    static SGTL5000_HP_MUTE: c_uint;
    static SGTL5000_LINE_OUT_MUTE: c_uint;
    static SGTL5000_OUTPUTS_MUTE: c_uint;
    static SGTL5000_HP_SEL_MASK: c_uint;
    static SGTL5000_HP_SEL_SHIFT: c_uint;
    static SGTL5000_HP_SEL_LINE_IN: c_uint;
    static SGTL5000_VAG_POWERUP: c_uint;
    static SGTL5000_DAC_POWERUP: c_uint;
    static SGTL5000_ADC_POWERUP: c_uint;
    static SGTL5000_HP_POWERUP: c_uint;
    static SGTL5000_BIAS_R_MASK: c_uint;
    static SGTL5000_BIAS_R_SHIFT: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_uint;
    static SND_SOC_DAPM_PRE_PMD: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_uint;
    static SND_SOC_DAPM_POST_PMD: c_uint;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SGTL5000_DAC_VOL_LEFT_MASK: c_uint;
    static SGTL5000_DAC_VOL_LEFT_SHIFT: c_uint;
    static SGTL5000_DAC_VOL_RIGHT_MASK: c_uint;
    static SGTL5000_DAC_VOL_RIGHT_SHIFT: c_uint;
    static EINVAL: c_int;
    static SGTL5000_I2S_IN_POWERUP: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SGTL5000_I2S_MASTER: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SGTL5000_I2S_MODE_PCM: c_uint;
    static SGTL5000_I2S_MODE_SHIFT: c_uint;
    static SGTL5000_I2S_LRALIGN: c_uint;
    static SGTL5000_I2S_MODE_I2S_LJ: c_uint;
    static SGTL5000_I2S_MODE_RJ: c_uint;
    static SGTL5000_I2S_LRPOL: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SGTL5000_I2S_SCLK_INV: c_uint;
    static SGTL5000_SYSCLK: c_uint;
    static SGTL5000_RATE_MODE_DIV_4: c_uint;
    static SGTL5000_RATE_MODE_DIV_2: c_uint;
    static SGTL5000_RATE_MODE_DIV_1: c_uint;
    static SGTL5000_RATE_MODE_SHIFT: c_uint;
    static SGTL5000_SYS_FS_32k: c_uint;
    static SGTL5000_SYS_FS_44_1k: c_uint;
    static SGTL5000_SYS_FS_48k: c_uint;
    static SGTL5000_SYS_FS_96k: c_uint;
    static SGTL5000_SYS_FS_SHIFT: c_uint;
    static SGTL5000_MCLK_FREQ_256FS: c_uint;
    static SGTL5000_MCLK_FREQ_384FS: c_uint;
    static SGTL5000_MCLK_FREQ_512FS: c_uint;
    static SGTL5000_MCLK_FREQ_PLL: c_uint;
    static SGTL5000_MCLK_FREQ_SHIFT: c_uint;
    static SGTL5000_MCLK_FREQ_MASK: c_uint;
    static SGTL5000_PLL_INT_DIV_SHIFT: c_uint;
    static SGTL5000_PLL_FRAC_DIV_SHIFT: c_uint;
    static SGTL5000_INPUT_FREQ_DIV2: c_uint;
    static SGTL5000_PLL_POWERUP: c_uint;
    static SGTL5000_VCOAMP_POWERUP: c_uint;
    static SGTL5000_CHIP_CLK_CTRL: c_uint;
    static EFAULT: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_uint;
    static SGTL5000_DAC_STEREO: c_uint;
    static SGTL5000_ADC_STEREO: c_uint;
    static SGTL5000_I2S_DLEN_16: c_uint;
    static SGTL5000_I2S_DLEN_20: c_uint;
    static SGTL5000_I2S_DLEN_24: c_uint;
    static SGTL5000_I2S_DLEN_32: c_uint;
    static SGTL5000_I2S_DLEN_SHIFT: c_uint;
    static SGTL5000_I2S_SCLKFREQ_32FS: c_uint;
    static SGTL5000_I2S_SCLKFREQ_64FS: c_uint;
    static SGTL5000_I2S_SCLKFREQ_SHIFT: c_uint;
    static SGTL5000_I2S_DLEN_MASK: c_uint;
    static SGTL5000_I2S_SCLKFREQ_MASK: c_uint;
    static SND_SOC_BIAS_ON: c_uint;
    static SND_SOC_BIAS_PREPARE: c_uint;
    static SND_SOC_BIAS_STANDBY: c_uint;
    static SND_SOC_BIAS_OFF: c_uint;
    static SGTL5000_REFTOP_POWERUP: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SGTL5000_CHIP_LINREG_CTRL: c_uint;
    static SGTL5000_DAP_FLT_COEF_ACCESS: c_uint;
    static SGTL5000_DAP_COEF_WR_B0_MSB: c_uint;
    static SGTL5000_DAP_COEF_WR_B0_LSB: c_uint;
    static SGTL5000_DAP_COEF_WR_B1_MSB: c_uint;
    static SGTL5000_DAP_COEF_WR_B1_LSB: c_uint;
    static SGTL5000_DAP_COEF_WR_B2_MSB: c_uint;
    static SGTL5000_DAP_COEF_WR_B2_LSB: c_uint;
    static SGTL5000_DAP_COEF_WR_A1_MSB: c_uint;
    static SGTL5000_DAP_COEF_WR_A1_LSB: c_uint;
    static SGTL5000_DAP_COEF_WR_A2_MSB: c_uint;
    static SGTL5000_DAP_COEF_WR_A2_LSB: c_uint;
    static SGTL5000_VDDC_CHRGPMP_POWERUP: c_uint;
    static SGTL5000_INT_OSC_EN: c_uint;
    static SGTL5000_VDDC_ASSN_OVRD: c_uint;
    static SGTL5000_VDDC_MAN_ASSN_VDDIO: c_uint;
    static SGTL5000_VDDC_MAN_ASSN_SHIFT: c_uint;
    static SGTL5000_ANA_GND_BASE: c_uint;
    static SGTL5000_ANA_GND_STP: c_uint;
    static SGTL5000_ANA_GND_MASK: c_uint;
    static SGTL5000_ANA_GND_SHIFT: c_uint;
    static SGTL5000_LINE_OUT_GND_BASE: c_uint;
    static SGTL5000_LINE_OUT_GND_STP: c_uint;
    static SGTL5000_LINE_OUT_GND_MAX: c_uint;
    static SGTL5000_LINE_OUT_CURRENT_MASK: c_uint;
    static SGTL5000_LINE_OUT_GND_MASK: c_uint;
    static SGTL5000_LINE_OUT_GND_SHIFT: c_uint;
    static SGTL5000_LINE_OUT_CURRENT_360u: c_uint;
    static SGTL5000_LINE_OUT_CURRENT_SHIFT: c_uint;
    static SGTL5000_LINE_OUT_VOL_RIGHT_MASK: c_uint;
    static SGTL5000_LINE_OUT_VOL_LEFT_MASK: c_uint;
    static SGTL5000_LINE_OUT_VOL_RIGHT_SHIFT: c_uint;
    static SGTL5000_LINE_OUT_VOL_LEFT_SHIFT: c_uint;
    static EPROBE_DEFER: c_int;
    static SGTL5000_HP_ZCD_EN: c_uint;
    static SGTL5000_ADC_ZCD_EN: c_uint;
    static SGTL5000_SMALL_POP: c_uint;
    static SGTL5000_ADC_EN: c_uint;
    static SGTL5000_DAC_EN: c_uint;
    static SGTL5000_DAC_VOL_RAMP_EN: c_uint;
    static SGTL5000_DAC_MUTE_RIGHT: c_uint;
    static SGTL5000_DAC_MUTE_LEFT: c_uint;
    static SGTL5000_PAD_I2S_LRCLK_SHIFT: c_uint;
    static SGTL5000_PAD_I2S_SCLK_SHIFT: c_uint;
    static SGTL5000_BIAS_VOLT_MASK: c_uint;
    static SGTL5000_BIAS_VOLT_SHIFT: c_uint;
    static SGTL5000_DAP_SEL_GEQ: c_uint;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static ENOENT: c_int;
    static ENODEV: c_int;
    static SGTL5000_PARTID_MASK: c_uint;
    static SGTL5000_PARTID_SHIFT: c_uint;
    static SGTL5000_PARTID_PART_ID: c_uint;
    static SGTL5000_REVID_MASK: c_uint;
    static SGTL5000_REVID_SHIFT: c_uint;
    static SGTL5000_CHIP_CLK_CTRL_DEFAULT: c_uint;
    static SGTL5000_CHIP_ANA_CTRL_DEFAULT: c_uint;
    static SGTL5000_ANA_POWER_DEFAULT: c_uint;
    static SGTL5000_LINREG_VDDD_MASK: c_uint;
    static SGTL5000_LINEREG_D_POWERUP: c_uint;
    static SGTL5000_STARTUP_POWERUP: c_uint;
    static SGTL5000_LINREG_SIMPLE_POWERUP: c_uint;
    static SGTL5000_DIG_POWER_DEFAULT: c_uint;
    static sgtl5000_regmap: c_void;
    static mut sgtl5000_dai: c_void;
    static sgtl5000_driver: c_void;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
