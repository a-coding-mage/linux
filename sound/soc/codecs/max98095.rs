// SPDX-License-Identifier: GPL-2.0-only
/*
 * max98095.rs -- MAX98095 ALSA SoC Audio driver
 *
 * Copyright 2011 Maxim Integrated Products
 *
 * Source-level Rust translation of max98095.c. Linux/ALSA/regmap symbols,
 * register constants, and C macro constructors referenced here are external
 * dependencies supplied by the surrounding driver tree.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;

#[repr(C)]
pub enum max98095_type {
    MAX98095,
}

#[repr(C)]
pub struct max98095_cdata {
    pub rate: c_uint,
    pub fmt: c_uint,
    pub eq_sel: c_int,
    pub bq_sel: c_int,
}

#[repr(C)]
pub struct max98095_priv {
    pub regmap: *mut regmap,
    pub devtype: max98095_type,
    pub pdata: *mut max98095_pdata,
    pub mclk: *mut clk,
    pub sysclk: c_uint,
    pub dai: [max98095_cdata; 3],
    pub eq_texts: *mut *const c_char,
    pub bq_texts: *mut *const c_char,
    pub eq_enum: soc_enum,
    pub bq_enum: soc_enum,
    pub eq_textcnt: c_int,
    pub bq_textcnt: c_int,
    pub lin_state: u8,
    pub mic1pre: c_uint,
    pub mic2pre: c_uint,
    pub headphone_jack: *mut snd_soc_jack,
    pub mic_jack: *mut snd_soc_jack,
    pub lock: mutex,
}

// Original C includes:
// linux/cleanup.h, module.h, moduleparam.h, kernel.h, init.h, delay.h, pm.h,
// i2c.h, clk.h, mutex.h, slab.h, asm/div64.h, ALSA SoC/PCM/TLV headers,
// sound/max98095.h, sound/jack.h, and local max98095.h.

extern "C" {
    static mut max98095_reg_def: [reg_default; 138];
    static mut max98095_regmap: regmap_config;
}

pub static max98095_reg_def_init: &[reg_default] = &[
    reg_default { reg: 0x0f, def: 0x00 }, reg_default { reg: 0x10, def: 0x00 },
    reg_default { reg: 0x11, def: 0x00 }, reg_default { reg: 0x12, def: 0x00 },
    reg_default { reg: 0x13, def: 0x00 }, reg_default { reg: 0x14, def: 0x00 },
    reg_default { reg: 0x15, def: 0x00 }, reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x17, def: 0x00 }, reg_default { reg: 0x18, def: 0x00 },
    reg_default { reg: 0x19, def: 0x00 }, reg_default { reg: 0x1a, def: 0x00 },
    reg_default { reg: 0x1b, def: 0x00 }, reg_default { reg: 0x1c, def: 0x00 },
    reg_default { reg: 0x1d, def: 0x00 }, reg_default { reg: 0x1e, def: 0x00 },
    reg_default { reg: 0x1f, def: 0x00 }, reg_default { reg: 0x20, def: 0x00 },
    reg_default { reg: 0x21, def: 0x00 }, reg_default { reg: 0x22, def: 0x00 },
    reg_default { reg: 0x23, def: 0x00 }, reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x00 }, reg_default { reg: 0x26, def: 0x00 },
    reg_default { reg: 0x27, def: 0x00 }, reg_default { reg: 0x28, def: 0x00 },
    reg_default { reg: 0x29, def: 0x00 }, reg_default { reg: 0x2a, def: 0x00 },
    reg_default { reg: 0x2b, def: 0x00 }, reg_default { reg: 0x2c, def: 0x00 },
    reg_default { reg: 0x2d, def: 0x00 }, reg_default { reg: 0x2e, def: 0x00 },
    reg_default { reg: 0x2f, def: 0x00 }, reg_default { reg: 0x30, def: 0x00 },
    reg_default { reg: 0x31, def: 0x00 }, reg_default { reg: 0x32, def: 0x00 },
    reg_default { reg: 0x33, def: 0x00 }, reg_default { reg: 0x34, def: 0x00 },
    reg_default { reg: 0x35, def: 0x00 }, reg_default { reg: 0x36, def: 0x00 },
    reg_default { reg: 0x37, def: 0x00 }, reg_default { reg: 0x38, def: 0x00 },
    reg_default { reg: 0x39, def: 0x00 }, reg_default { reg: 0x3a, def: 0x00 },
    reg_default { reg: 0x3b, def: 0x00 }, reg_default { reg: 0x3c, def: 0x00 },
    reg_default { reg: 0x3d, def: 0x00 }, reg_default { reg: 0x3e, def: 0x00 },
    reg_default { reg: 0x3f, def: 0x00 }, reg_default { reg: 0x40, def: 0x00 },
    reg_default { reg: 0x41, def: 0x00 }, reg_default { reg: 0x42, def: 0x00 },
    reg_default { reg: 0x43, def: 0x00 }, reg_default { reg: 0x44, def: 0x00 },
    reg_default { reg: 0x45, def: 0x00 }, reg_default { reg: 0x46, def: 0x00 },
    reg_default { reg: 0x47, def: 0x00 }, reg_default { reg: 0x48, def: 0x00 },
    reg_default { reg: 0x49, def: 0x00 }, reg_default { reg: 0x4a, def: 0x00 },
    reg_default { reg: 0x4b, def: 0x00 }, reg_default { reg: 0x4c, def: 0x00 },
    reg_default { reg: 0x4d, def: 0x00 }, reg_default { reg: 0x4e, def: 0x00 },
    reg_default { reg: 0x4f, def: 0x00 }, reg_default { reg: 0x50, def: 0x00 },
    reg_default { reg: 0x51, def: 0x00 }, reg_default { reg: 0x52, def: 0x00 },
    reg_default { reg: 0x53, def: 0x00 }, reg_default { reg: 0x54, def: 0x00 },
    reg_default { reg: 0x55, def: 0x00 }, reg_default { reg: 0x56, def: 0x00 },
    reg_default { reg: 0x57, def: 0x00 }, reg_default { reg: 0x58, def: 0x00 },
    reg_default { reg: 0x59, def: 0x00 }, reg_default { reg: 0x5a, def: 0x00 },
    reg_default { reg: 0x5b, def: 0x00 }, reg_default { reg: 0x5c, def: 0x00 },
    reg_default { reg: 0x5d, def: 0x00 }, reg_default { reg: 0x5e, def: 0x00 },
    reg_default { reg: 0x5f, def: 0x00 }, reg_default { reg: 0x60, def: 0x00 },
    reg_default { reg: 0x61, def: 0x00 }, reg_default { reg: 0x62, def: 0x00 },
    reg_default { reg: 0x63, def: 0x00 }, reg_default { reg: 0x64, def: 0x00 },
    reg_default { reg: 0x65, def: 0x00 }, reg_default { reg: 0x66, def: 0x00 },
    reg_default { reg: 0x67, def: 0x00 }, reg_default { reg: 0x68, def: 0x00 },
    reg_default { reg: 0x69, def: 0x00 }, reg_default { reg: 0x6a, def: 0x00 },
    reg_default { reg: 0x6b, def: 0x00 }, reg_default { reg: 0x6c, def: 0x00 },
    reg_default { reg: 0x6d, def: 0x00 }, reg_default { reg: 0x6e, def: 0x00 },
    reg_default { reg: 0x6f, def: 0x00 }, reg_default { reg: 0x70, def: 0x00 },
    reg_default { reg: 0x71, def: 0x00 }, reg_default { reg: 0x72, def: 0x00 },
    reg_default { reg: 0x73, def: 0x00 }, reg_default { reg: 0x74, def: 0x00 },
    reg_default { reg: 0x75, def: 0x00 }, reg_default { reg: 0x76, def: 0x00 },
    reg_default { reg: 0x77, def: 0x00 }, reg_default { reg: 0x78, def: 0x00 },
    reg_default { reg: 0x79, def: 0x00 }, reg_default { reg: 0x7a, def: 0x00 },
    reg_default { reg: 0x7b, def: 0x00 }, reg_default { reg: 0x7c, def: 0x00 },
    reg_default { reg: 0x7d, def: 0x00 }, reg_default { reg: 0x7e, def: 0x00 },
    reg_default { reg: 0x7f, def: 0x00 }, reg_default { reg: 0x80, def: 0x00 },
    reg_default { reg: 0x81, def: 0x00 }, reg_default { reg: 0x82, def: 0x00 },
    reg_default { reg: 0x83, def: 0x00 }, reg_default { reg: 0x84, def: 0x00 },
    reg_default { reg: 0x85, def: 0x00 }, reg_default { reg: 0x86, def: 0x00 },
    reg_default { reg: 0x87, def: 0x00 }, reg_default { reg: 0x88, def: 0x00 },
    reg_default { reg: 0x89, def: 0x00 }, reg_default { reg: 0x8a, def: 0x00 },
    reg_default { reg: 0x8b, def: 0x00 }, reg_default { reg: 0x8c, def: 0x00 },
    reg_default { reg: 0x8d, def: 0x00 }, reg_default { reg: 0x8e, def: 0x00 },
    reg_default { reg: 0x8f, def: 0x00 }, reg_default { reg: 0x90, def: 0x00 },
    reg_default { reg: 0x91, def: 0x00 }, reg_default { reg: 0x92, def: 0x30 },
    reg_default { reg: 0x93, def: 0xF0 }, reg_default { reg: 0x94, def: 0x00 },
    reg_default { reg: 0x95, def: 0x00 }, reg_default { reg: 0x96, def: 0x3F },
    reg_default { reg: 0x97, def: 0x00 }, reg_default { reg: 0xff, def: 0x00 },
];

unsafe fn max98095_readable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        M98095_001_HOST_INT_STS..=M98095_097_PWR_SYS => true,
        M98095_0FF_REV_ID => true,
        _ => false,
    }
}

unsafe fn max98095_writeable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        M98095_00F_HOST_CFG..=M98095_097_PWR_SYS => true,
        _ => false,
    }
}

unsafe fn max98095_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        M98095_000_HOST_DATA..=M98095_00E_TEMP_SENSOR_STS => true,
        _ if reg >= M98095_REG_MAX_CACHED + 1 && reg <= M98095_0FF_REV_ID => true,
        _ => false,
    }
}

/*
 * Load equalizer DSP coefficient configurations registers
 */
unsafe fn m98095_eq_band(component: *mut snd_soc_component, dai: c_uint, band: c_uint, coefs: *mut u16) {
    let mut eq_reg: c_uint;
    let mut i: c_uint;

    if WARN_ON((band > 4) as c_int) != 0 || WARN_ON((dai > 1) as c_int) != 0 {
        return;
    }

    /* Load the base register address */
    eq_reg = if dai != 0 { M98095_142_DAI2_EQ_BASE } else { M98095_110_DAI1_EQ_BASE };

    /* Add the band address offset, note adjustment for word address */
    eq_reg = eq_reg.wrapping_add(band.wrapping_mul(M98095_COEFS_PER_BAND << 1));

    /* Step through the registers and coefs */
    i = 0;
    while i < M98095_COEFS_PER_BAND {
        snd_soc_component_write(component, { let r = eq_reg; eq_reg = eq_reg.wrapping_add(1); r }, M98095_BYTE1(*coefs.add(i as usize) as c_uint));
        snd_soc_component_write(component, { let r = eq_reg; eq_reg = eq_reg.wrapping_add(1); r }, M98095_BYTE0(*coefs.add(i as usize) as c_uint));
        i = i.wrapping_add(1);
    }
}

/*
 * Load biquad filter coefficient configurations registers
 */
unsafe fn m98095_biquad_band(component: *mut snd_soc_component, dai: c_uint, band: c_uint, coefs: *mut u16) {
    let mut bq_reg: c_uint;
    let mut i: c_uint;

    if WARN_ON((band > 1) as c_int) != 0 || WARN_ON((dai > 1) as c_int) != 0 {
        return;
    }

    /* Load the base register address */
    bq_reg = if dai != 0 { M98095_17E_DAI2_BQ_BASE } else { M98095_174_DAI1_BQ_BASE };

    /* Add the band address offset, note adjustment for word address */
    bq_reg = bq_reg.wrapping_add(band.wrapping_mul(M98095_COEFS_PER_BAND << 1));

    /* Step through the registers and coefs */
    i = 0;
    while i < M98095_COEFS_PER_BAND {
        snd_soc_component_write(component, { let r = bq_reg; bq_reg = bq_reg.wrapping_add(1); r }, M98095_BYTE1(*coefs.add(i as usize) as c_uint));
        snd_soc_component_write(component, { let r = bq_reg; bq_reg = bq_reg.wrapping_add(1); r }, M98095_BYTE0(*coefs.add(i as usize) as c_uint));
        i = i.wrapping_add(1);
    }
}

static max98095_fltr_mode: [*const c_char; 2] = [c"Voice".as_ptr(), c"Music".as_ptr()];
SOC_ENUM_SINGLE_DECL!(max98095_dai1_filter_mode_enum, M98095_02E_DAI1_FILTERS, 7, max98095_fltr_mode);
SOC_ENUM_SINGLE_DECL!(max98095_dai2_filter_mode_enum, M98095_038_DAI2_FILTERS, 7, max98095_fltr_mode);

static max98095_extmic_text: [*const c_char; 3] = [c"None".as_ptr(), c"MIC1".as_ptr(), c"MIC2".as_ptr()];
SOC_ENUM_SINGLE_DECL!(max98095_extmic_enum, M98095_087_CFG_MIC, 0, max98095_extmic_text);
static max98095_extmic_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("External MIC Mux", max98095_extmic_enum);

static max98095_linein_text: [*const c_char; 2] = [c"INA".as_ptr(), c"INB".as_ptr()];
SOC_ENUM_SINGLE_DECL!(max98095_linein_enum, M98095_086_CFG_LINE, 6, max98095_linein_text);
static max98095_linein_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Linein Input Mux", max98095_linein_enum);

static max98095_line_mode_text: [*const c_char; 2] = [c"Stereo".as_ptr(), c"Differential".as_ptr()];
SOC_ENUM_SINGLE_DECL!(max98095_linein_mode_enum, M98095_086_CFG_LINE, 7, max98095_line_mode_text);
SOC_ENUM_SINGLE_DECL!(max98095_lineout_mode_enum, M98095_086_CFG_LINE, 4, max98095_line_mode_text);

static max98095_dai_fltr: [*const c_char; 6] = [
    c"Off".as_ptr(), c"Elliptical-HPF-16k".as_ptr(), c"Butterworth-HPF-16k".as_ptr(),
    c"Elliptical-HPF-8k".as_ptr(), c"Butterworth-HPF-8k".as_ptr(), c"Butterworth-HPF-Fs/240".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(max98095_dai1_dac_filter_enum, M98095_02E_DAI1_FILTERS, 0, max98095_dai_fltr);
SOC_ENUM_SINGLE_DECL!(max98095_dai2_dac_filter_enum, M98095_038_DAI2_FILTERS, 0, max98095_dai_fltr);
SOC_ENUM_SINGLE_DECL!(max98095_dai3_dac_filter_enum, M98095_042_DAI3_FILTERS, 0, max98095_dai_fltr);

unsafe fn max98095_mic1pre_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let sel = (*ucontrol).value.integer.value[0] as c_uint;
    (*max98095).mic1pre = sel;
    snd_soc_component_update_bits(component, M98095_05F_LVL_MIC1, M98095_MICPRE_MASK, (1 + sel) << M98095_MICPRE_SHIFT);
    0
}

unsafe fn max98095_mic1pre_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    (*ucontrol).value.integer.value[0] = (*max98095).mic1pre as _;
    0
}

unsafe fn max98095_mic2pre_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let sel = (*ucontrol).value.integer.value[0] as c_uint;
    (*max98095).mic2pre = sel;
    snd_soc_component_update_bits(component, M98095_060_LVL_MIC2, M98095_MICPRE_MASK, (1 + sel) << M98095_MICPRE_SHIFT);
    0
}

unsafe fn max98095_mic2pre_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    (*ucontrol).value.integer.value[0] = (*max98095).mic2pre as _;
    0
}

DECLARE_TLV_DB_RANGE!(max98095_micboost_tlv, 0, 1, TLV_DB_SCALE_ITEM!(0, 2000, 0), 2, 2, TLV_DB_SCALE_ITEM!(3000, 0, 0));
DECLARE_TLV_DB_SCALE!(max98095_mic_tlv, 0, 100, 0);
DECLARE_TLV_DB_SCALE!(max98095_adc_tlv, -1200, 100, 0);
DECLARE_TLV_DB_SCALE!(max98095_adcboost_tlv, 0, 600, 0);
DECLARE_TLV_DB_RANGE!(max98095_hp_tlv, 0, 6, TLV_DB_SCALE_ITEM!(-6700, 400, 0), 7, 14, TLV_DB_SCALE_ITEM!(-4000, 300, 0), 15, 21, TLV_DB_SCALE_ITEM!(-1700, 200, 0), 22, 27, TLV_DB_SCALE_ITEM!(-400, 100, 0), 28, 31, TLV_DB_SCALE_ITEM!(150, 50, 0));
DECLARE_TLV_DB_RANGE!(max98095_spk_tlv, 0, 10, TLV_DB_SCALE_ITEM!(-5900, 400, 0), 11, 18, TLV_DB_SCALE_ITEM!(-1700, 200, 0), 19, 27, TLV_DB_SCALE_ITEM!(-200, 100, 0), 28, 39, TLV_DB_SCALE_ITEM!(650, 50, 0));
DECLARE_TLV_DB_RANGE!(max98095_rcv_lout_tlv, 0, 6, TLV_DB_SCALE_ITEM!(-6200, 400, 0), 7, 14, TLV_DB_SCALE_ITEM!(-3500, 300, 0), 15, 21, TLV_DB_SCALE_ITEM!(-1200, 200, 0), 22, 27, TLV_DB_SCALE_ITEM!(100, 100, 0), 28, 31, TLV_DB_SCALE_ITEM!(650, 50, 0));
DECLARE_TLV_DB_RANGE!(max98095_lin_tlv, 0, 2, TLV_DB_SCALE_ITEM!(-600, 300, 0), 3, 3, TLV_DB_SCALE_ITEM!(300, 1100, 0), 4, 5, TLV_DB_SCALE_ITEM!(1400, 600, 0));

static max98095_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_R_TLV!("Headphone Volume", M98095_064_LVL_HP_L, M98095_065_LVL_HP_R, 0, 31, 0, max98095_hp_tlv),
    SOC_DOUBLE_R_TLV!("Speaker Volume", M98095_067_LVL_SPK_L, M98095_068_LVL_SPK_R, 0, 39, 0, max98095_spk_tlv),
    SOC_SINGLE_TLV!("Receiver Volume", M98095_066_LVL_RCV, 0, 31, 0, max98095_rcv_lout_tlv),
    SOC_DOUBLE_R_TLV!("Lineout Volume", M98095_062_LVL_LINEOUT1, M98095_063_LVL_LINEOUT2, 0, 31, 0, max98095_rcv_lout_tlv),
    SOC_DOUBLE_R!("Headphone Switch", M98095_064_LVL_HP_L, M98095_065_LVL_HP_R, 7, 1, 1),
    SOC_DOUBLE_R!("Speaker Switch", M98095_067_LVL_SPK_L, M98095_068_LVL_SPK_R, 7, 1, 1),
    SOC_SINGLE!("Receiver Switch", M98095_066_LVL_RCV, 7, 1, 1),
    SOC_DOUBLE_R!("Lineout Switch", M98095_062_LVL_LINEOUT1, M98095_063_LVL_LINEOUT2, 7, 1, 1),
    SOC_SINGLE_TLV!("MIC1 Volume", M98095_05F_LVL_MIC1, 0, 20, 1, max98095_mic_tlv),
    SOC_SINGLE_TLV!("MIC2 Volume", M98095_060_LVL_MIC2, 0, 20, 1, max98095_mic_tlv),
    SOC_SINGLE_EXT_TLV!("MIC1 Boost Volume", M98095_05F_LVL_MIC1, 5, 2, 0, max98095_mic1pre_get, max98095_mic1pre_set, max98095_micboost_tlv),
    SOC_SINGLE_EXT_TLV!("MIC2 Boost Volume", M98095_060_LVL_MIC2, 5, 2, 0, max98095_mic2pre_get, max98095_mic2pre_set, max98095_micboost_tlv),
    SOC_SINGLE_TLV!("Linein Volume", M98095_061_LVL_LINEIN, 0, 5, 1, max98095_lin_tlv),
    SOC_SINGLE_TLV!("ADCL Volume", M98095_05D_LVL_ADC_L, 0, 15, 1, max98095_adc_tlv),
    SOC_SINGLE_TLV!("ADCR Volume", M98095_05E_LVL_ADC_R, 0, 15, 1, max98095_adc_tlv),
    SOC_SINGLE_TLV!("ADCL Boost Volume", M98095_05D_LVL_ADC_L, 4, 3, 0, max98095_adcboost_tlv),
    SOC_SINGLE_TLV!("ADCR Boost Volume", M98095_05E_LVL_ADC_R, 4, 3, 0, max98095_adcboost_tlv),
    SOC_SINGLE!("EQ1 Switch", M98095_088_CFG_LEVEL, 0, 1, 0),
    SOC_SINGLE!("EQ2 Switch", M98095_088_CFG_LEVEL, 1, 1, 0),
    SOC_SINGLE!("Biquad1 Switch", M98095_088_CFG_LEVEL, 2, 1, 0),
    SOC_SINGLE!("Biquad2 Switch", M98095_088_CFG_LEVEL, 3, 1, 0),
    SOC_ENUM!("DAI1 Filter Mode", max98095_dai1_filter_mode_enum),
    SOC_ENUM!("DAI2 Filter Mode", max98095_dai2_filter_mode_enum),
    SOC_ENUM!("DAI1 DAC Filter", max98095_dai1_dac_filter_enum),
    SOC_ENUM!("DAI2 DAC Filter", max98095_dai2_dac_filter_enum),
    SOC_ENUM!("DAI3 DAC Filter", max98095_dai3_dac_filter_enum),
    SOC_ENUM!("Linein Mode", max98095_linein_mode_enum),
    SOC_ENUM!("Lineout Mode", max98095_lineout_mode_enum),
];

macro_rules! dapm_controls { ($name:ident, [$($item:expr),* $(,)?]) => { static $name: &[snd_kcontrol_new] = &[$($item),*]; } }
dapm_controls!(max98095_left_speaker_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_050_MIX_SPK_LEFT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_050_MIX_SPK_LEFT, 6, 1, 0),
    SOC_DAPM_SINGLE!("Mono DAC2 Switch", M98095_050_MIX_SPK_LEFT, 3, 1, 0),
    SOC_DAPM_SINGLE!("Mono DAC3 Switch", M98095_050_MIX_SPK_LEFT, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_050_MIX_SPK_LEFT, 4, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_050_MIX_SPK_LEFT, 5, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_050_MIX_SPK_LEFT, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_050_MIX_SPK_LEFT, 2, 1, 0),
]);
dapm_controls!(max98095_right_speaker_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_051_MIX_SPK_RIGHT, 6, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_051_MIX_SPK_RIGHT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Mono DAC2 Switch", M98095_051_MIX_SPK_RIGHT, 3, 1, 0),
    SOC_DAPM_SINGLE!("Mono DAC3 Switch", M98095_051_MIX_SPK_RIGHT, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_051_MIX_SPK_RIGHT, 5, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_051_MIX_SPK_RIGHT, 4, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_051_MIX_SPK_RIGHT, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_051_MIX_SPK_RIGHT, 2, 1, 0),
]);
dapm_controls!(max98095_left_hp_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_04C_MIX_HP_LEFT, 0, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_04C_MIX_HP_LEFT, 5, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_04C_MIX_HP_LEFT, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_04C_MIX_HP_LEFT, 4, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_04C_MIX_HP_LEFT, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_04C_MIX_HP_LEFT, 2, 1, 0),
]);
dapm_controls!(max98095_right_hp_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_04D_MIX_HP_RIGHT, 5, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_04D_MIX_HP_RIGHT, 0, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_04D_MIX_HP_RIGHT, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_04D_MIX_HP_RIGHT, 4, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_04D_MIX_HP_RIGHT, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_04D_MIX_HP_RIGHT, 2, 1, 0),
]);
dapm_controls!(max98095_mono_rcv_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_04F_MIX_RCV, 0, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_04F_MIX_RCV, 5, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_04F_MIX_RCV, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_04F_MIX_RCV, 4, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_04F_MIX_RCV, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_04F_MIX_RCV, 2, 1, 0),
]);
dapm_controls!(max98095_left_lineout_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_053_MIX_LINEOUT1, 5, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_053_MIX_LINEOUT1, 0, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_053_MIX_LINEOUT1, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_053_MIX_LINEOUT1, 4, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_053_MIX_LINEOUT1, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_053_MIX_LINEOUT1, 2, 1, 0),
]);
dapm_controls!(max98095_right_lineout_mixer_controls, [
    SOC_DAPM_SINGLE!("Left DAC1 Switch", M98095_054_MIX_LINEOUT2, 0, 1, 0),
    SOC_DAPM_SINGLE!("Right DAC1 Switch", M98095_054_MIX_LINEOUT2, 5, 1, 0),
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_054_MIX_LINEOUT2, 3, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_054_MIX_LINEOUT2, 4, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_054_MIX_LINEOUT2, 1, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_054_MIX_LINEOUT2, 2, 1, 0),
]);
dapm_controls!(max98095_left_ADC_mixer_controls, [
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_04A_MIX_ADC_LEFT, 7, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_04A_MIX_ADC_LEFT, 6, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_04A_MIX_ADC_LEFT, 3, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_04A_MIX_ADC_LEFT, 2, 1, 0),
]);
dapm_controls!(max98095_right_ADC_mixer_controls, [
    SOC_DAPM_SINGLE!("MIC1 Switch", M98095_04B_MIX_ADC_RIGHT, 7, 1, 0),
    SOC_DAPM_SINGLE!("MIC2 Switch", M98095_04B_MIX_ADC_RIGHT, 6, 1, 0),
    SOC_DAPM_SINGLE!("IN1 Switch", M98095_04B_MIX_ADC_RIGHT, 3, 1, 0),
    SOC_DAPM_SINGLE!("IN2 Switch", M98095_04B_MIX_ADC_RIGHT, 2, 1, 0),
]);

unsafe fn max98095_mic_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    match event {
        SND_SOC_DAPM_POST_PMU => {
            if (*w).reg == M98095_05F_LVL_MIC1 {
                snd_soc_component_update_bits(component, (*w).reg, M98095_MICPRE_MASK, (1 + (*max98095).mic1pre) << M98095_MICPRE_SHIFT);
            } else {
                snd_soc_component_update_bits(component, (*w).reg, M98095_MICPRE_MASK, (1 + (*max98095).mic2pre) << M98095_MICPRE_SHIFT);
            }
        }
        SND_SOC_DAPM_POST_PMD => { snd_soc_component_update_bits(component, (*w).reg, M98095_MICPRE_MASK, 0); }
        _ => return -EINVAL,
    }
    0
}

/*
 * The line inputs are stereo inputs with the left and right
 * channels sharing a common PGA power control signal.
 */
unsafe fn max98095_line_pga(w: *mut snd_soc_dapm_widget, event: c_int, channel: u8) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let state = &mut (*max98095).lin_state as *mut u8;

    if WARN_ON(!((channel == 1) || (channel == 2)) as c_int) != 0 {
        return -EINVAL;
    }

    match event {
        SND_SOC_DAPM_POST_PMU => {
            *state |= channel;
            snd_soc_component_update_bits(component, (*w).reg, 1 << (*w).shift, 1 << (*w).shift);
        }
        SND_SOC_DAPM_POST_PMD => {
            *state &= !channel;
            if *state == 0 {
                snd_soc_component_update_bits(component, (*w).reg, 1 << (*w).shift, 0);
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn max98095_pga_in1_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    max98095_line_pga(w, event, 1)
}

unsafe fn max98095_pga_in2_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    max98095_line_pga(w, event, 2)
}

/*
 * The stereo line out mixer outputs to two stereo line outs.
 * The 2nd pair has a separate set of enables.
 */
unsafe fn max98095_lineout_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => snd_soc_component_update_bits(component, (*w).reg, 1 << ((*w).shift + 2), 1 << ((*w).shift + 2)),
        SND_SOC_DAPM_POST_PMD => snd_soc_component_update_bits(component, (*w).reg, 1 << ((*w).shift + 2), 0),
        _ => return -EINVAL,
    };
    0
}

static max98095_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_ADC!("ADCL", "HiFi Capture", M98095_090_PWR_EN_IN, 0, 0),
    SND_SOC_DAPM_ADC!("ADCR", "HiFi Capture", M98095_090_PWR_EN_IN, 1, 0),
    SND_SOC_DAPM_DAC!("DACL1", "HiFi Playback", M98095_091_PWR_EN_OUT, 0, 0),
    SND_SOC_DAPM_DAC!("DACR1", "HiFi Playback", M98095_091_PWR_EN_OUT, 1, 0),
    SND_SOC_DAPM_DAC!("DACM2", "Aux Playback", M98095_091_PWR_EN_OUT, 2, 0),
    SND_SOC_DAPM_DAC!("DACM3", "Voice Playback", M98095_091_PWR_EN_OUT, 2, 0),
    SND_SOC_DAPM_PGA!("HP Left Out", M98095_091_PWR_EN_OUT, 6, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("HP Right Out", M98095_091_PWR_EN_OUT, 7, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("SPK Left Out", M98095_091_PWR_EN_OUT, 4, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("SPK Right Out", M98095_091_PWR_EN_OUT, 5, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("RCV Mono Out", M98095_091_PWR_EN_OUT, 3, 0, NULL, 0),
    SND_SOC_DAPM_PGA_E!("LINE Left Out", M98095_092_PWR_EN_OUT, 0, 0, NULL, 0, max98095_lineout_event, SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_E!("LINE Right Out", M98095_092_PWR_EN_OUT, 1, 0, NULL, 0, max98095_lineout_event, SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX!("External MIC", SND_SOC_NOPM, 0, 0, &max98095_extmic_mux),
    SND_SOC_DAPM_MUX!("Linein Mux", SND_SOC_NOPM, 0, 0, &max98095_linein_mux),
    SND_SOC_DAPM_MIXER!("Left Headphone Mixer", SND_SOC_NOPM, 0, 0, &max98095_left_hp_mixer_controls[0], max98095_left_hp_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right Headphone Mixer", SND_SOC_NOPM, 0, 0, &max98095_right_hp_mixer_controls[0], max98095_right_hp_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Left Speaker Mixer", SND_SOC_NOPM, 0, 0, &max98095_left_speaker_mixer_controls[0], max98095_left_speaker_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right Speaker Mixer", SND_SOC_NOPM, 0, 0, &max98095_right_speaker_mixer_controls[0], max98095_right_speaker_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Receiver Mixer", SND_SOC_NOPM, 0, 0, &max98095_mono_rcv_mixer_controls[0], max98095_mono_rcv_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Left Lineout Mixer", SND_SOC_NOPM, 0, 0, &max98095_left_lineout_mixer_controls[0], max98095_left_lineout_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right Lineout Mixer", SND_SOC_NOPM, 0, 0, &max98095_right_lineout_mixer_controls[0], max98095_right_lineout_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Left ADC Mixer", SND_SOC_NOPM, 0, 0, &max98095_left_ADC_mixer_controls[0], max98095_left_ADC_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right ADC Mixer", SND_SOC_NOPM, 0, 0, &max98095_right_ADC_mixer_controls[0], max98095_right_ADC_mixer_controls.len()),
    SND_SOC_DAPM_PGA_E!("MIC1 Input", M98095_05F_LVL_MIC1, 5, 0, NULL, 0, max98095_mic_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("MIC2 Input", M98095_060_LVL_MIC2, 5, 0, NULL, 0, max98095_mic_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("IN1 Input", M98095_090_PWR_EN_IN, 7, 0, NULL, 0, max98095_pga_in1_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("IN2 Input", M98095_090_PWR_EN_IN, 7, 0, NULL, 0, max98095_pga_in2_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MICBIAS!("MICBIAS1", M98095_090_PWR_EN_IN, 2, 0),
    SND_SOC_DAPM_MICBIAS!("MICBIAS2", M98095_090_PWR_EN_IN, 3, 0),
    SND_SOC_DAPM_OUTPUT!("HPL"), SND_SOC_DAPM_OUTPUT!("HPR"), SND_SOC_DAPM_OUTPUT!("SPKL"),
    SND_SOC_DAPM_OUTPUT!("SPKR"), SND_SOC_DAPM_OUTPUT!("RCV"), SND_SOC_DAPM_OUTPUT!("OUT1"),
    SND_SOC_DAPM_OUTPUT!("OUT2"), SND_SOC_DAPM_OUTPUT!("OUT3"), SND_SOC_DAPM_OUTPUT!("OUT4"),
    SND_SOC_DAPM_INPUT!("MIC1"), SND_SOC_DAPM_INPUT!("MIC2"), SND_SOC_DAPM_INPUT!("INA1"),
    SND_SOC_DAPM_INPUT!("INA2"), SND_SOC_DAPM_INPUT!("INB1"), SND_SOC_DAPM_INPUT!("INB2"),
];

macro_rules! route { ($sink:literal, NULL, $src:literal) => { snd_soc_dapm_route { sink: c_str!($sink), control: core::ptr::null(), source: c_str!($src) } }; ($sink:literal, $ctl:literal, $src:literal) => { snd_soc_dapm_route { sink: c_str!($sink), control: c_str!($ctl), source: c_str!($src) } }; }
static max98095_audio_map: &[snd_soc_dapm_route] = &[
    route!("Left Headphone Mixer", "Left DAC1 Switch", "DACL1"), route!("Left Headphone Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Left Headphone Mixer", "MIC1 Switch", "MIC1 Input"), route!("Left Headphone Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Left Headphone Mixer", "IN1 Switch", "IN1 Input"), route!("Left Headphone Mixer", "IN2 Switch", "IN2 Input"),
    route!("Right Headphone Mixer", "Left DAC1 Switch", "DACL1"), route!("Right Headphone Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Right Headphone Mixer", "MIC1 Switch", "MIC1 Input"), route!("Right Headphone Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Right Headphone Mixer", "IN1 Switch", "IN1 Input"), route!("Right Headphone Mixer", "IN2 Switch", "IN2 Input"),
    route!("Left Speaker Mixer", "Left DAC1 Switch", "DACL1"), route!("Left Speaker Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Left Speaker Mixer", "Mono DAC2 Switch", "DACM2"), route!("Left Speaker Mixer", "Mono DAC3 Switch", "DACM3"),
    route!("Left Speaker Mixer", "MIC1 Switch", "MIC1 Input"), route!("Left Speaker Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Left Speaker Mixer", "IN1 Switch", "IN1 Input"), route!("Left Speaker Mixer", "IN2 Switch", "IN2 Input"),
    route!("Right Speaker Mixer", "Left DAC1 Switch", "DACL1"), route!("Right Speaker Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Right Speaker Mixer", "Mono DAC2 Switch", "DACM2"), route!("Right Speaker Mixer", "Mono DAC3 Switch", "DACM3"),
    route!("Right Speaker Mixer", "MIC1 Switch", "MIC1 Input"), route!("Right Speaker Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Right Speaker Mixer", "IN1 Switch", "IN1 Input"), route!("Right Speaker Mixer", "IN2 Switch", "IN2 Input"),
    route!("Receiver Mixer", "Left DAC1 Switch", "DACL1"), route!("Receiver Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Receiver Mixer", "MIC1 Switch", "MIC1 Input"), route!("Receiver Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Receiver Mixer", "IN1 Switch", "IN1 Input"), route!("Receiver Mixer", "IN2 Switch", "IN2 Input"),
    route!("Left Lineout Mixer", "Left DAC1 Switch", "DACL1"), route!("Left Lineout Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Left Lineout Mixer", "MIC1 Switch", "MIC1 Input"), route!("Left Lineout Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Left Lineout Mixer", "IN1 Switch", "IN1 Input"), route!("Left Lineout Mixer", "IN2 Switch", "IN2 Input"),
    route!("Right Lineout Mixer", "Left DAC1 Switch", "DACL1"), route!("Right Lineout Mixer", "Right DAC1 Switch", "DACR1"),
    route!("Right Lineout Mixer", "MIC1 Switch", "MIC1 Input"), route!("Right Lineout Mixer", "MIC2 Switch", "MIC2 Input"),
    route!("Right Lineout Mixer", "IN1 Switch", "IN1 Input"), route!("Right Lineout Mixer", "IN2 Switch", "IN2 Input"),
    route!("HP Left Out", NULL, "Left Headphone Mixer"), route!("HP Right Out", NULL, "Right Headphone Mixer"),
    route!("SPK Left Out", NULL, "Left Speaker Mixer"), route!("SPK Right Out", NULL, "Right Speaker Mixer"),
    route!("RCV Mono Out", NULL, "Receiver Mixer"), route!("LINE Left Out", NULL, "Left Lineout Mixer"),
    route!("LINE Right Out", NULL, "Right Lineout Mixer"), route!("HPL", NULL, "HP Left Out"), route!("HPR", NULL, "HP Right Out"),
    route!("SPKL", NULL, "SPK Left Out"), route!("SPKR", NULL, "SPK Right Out"), route!("RCV", NULL, "RCV Mono Out"),
    route!("OUT1", NULL, "LINE Left Out"), route!("OUT2", NULL, "LINE Right Out"), route!("OUT3", NULL, "LINE Left Out"),
    route!("OUT4", NULL, "LINE Right Out"), route!("Left ADC Mixer", "MIC1 Switch", "MIC1 Input"),
    route!("Left ADC Mixer", "MIC2 Switch", "MIC2 Input"), route!("Left ADC Mixer", "IN1 Switch", "IN1 Input"),
    route!("Left ADC Mixer", "IN2 Switch", "IN2 Input"), route!("Right ADC Mixer", "MIC1 Switch", "MIC1 Input"),
    route!("Right ADC Mixer", "MIC2 Switch", "MIC2 Input"), route!("Right ADC Mixer", "IN1 Switch", "IN1 Input"),
    route!("Right ADC Mixer", "IN2 Switch", "IN2 Input"), route!("ADCL", NULL, "Left ADC Mixer"),
    route!("ADCR", NULL, "Right ADC Mixer"), route!("IN1 Input", NULL, "INA1"), route!("IN2 Input", NULL, "INA2"),
    route!("MIC1 Input", NULL, "MIC1"), route!("MIC2 Input", NULL, "MIC2"),
];

#[repr(C)]
struct rate_table_entry { rate: u32, sr: u8 }
static rate_table: [rate_table_entry; 10] = [
    rate_table_entry { rate: 8000, sr: 0x01 }, rate_table_entry { rate: 11025, sr: 0x02 },
    rate_table_entry { rate: 16000, sr: 0x03 }, rate_table_entry { rate: 22050, sr: 0x04 },
    rate_table_entry { rate: 24000, sr: 0x05 }, rate_table_entry { rate: 32000, sr: 0x06 },
    rate_table_entry { rate: 44100, sr: 0x07 }, rate_table_entry { rate: 48000, sr: 0x08 },
    rate_table_entry { rate: 88200, sr: 0x09 }, rate_table_entry { rate: 96000, sr: 0x0A },
];

unsafe fn rate_value(rate: c_int, value: *mut u8) -> c_int {
    let mut i = 0usize;
    while i < rate_table.len() {
        if rate_table[i].rate >= rate as u32 {
            *value = rate_table[i].sr;
            return 0;
        }
        i += 1;
    }
    *value = rate_table[0].sr;
    -EINVAL
}

macro_rules! hw_params_fn {
    ($name:ident, $idx:expr, $fmtreg:expr, $clkmode:expr, $clkhi:expr, $clklo:expr, $filters:expr) => {
        unsafe fn $name(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
            let component = (*dai).component;
            let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
            let cdata = &mut (*max98095).dai[$idx] as *mut max98095_cdata;
            let mut ni: c_ulonglong;
            let rate = params_rate(params) as c_uint;
            let mut regval: u8 = 0;

            match params_width(params) {
                16 => { snd_soc_component_update_bits(component, $fmtreg, M98095_DAI_WS, 0); }
                24 => { snd_soc_component_update_bits(component, $fmtreg, M98095_DAI_WS, M98095_DAI_WS); }
                _ => return -EINVAL,
            }
            if rate_value(rate as c_int, &mut regval) != 0 { return -EINVAL; }
            snd_soc_component_update_bits(component, $clkmode, M98095_CLKMODE_MASK, regval as c_uint);
            (*cdata).rate = rate;
            /* Configure NI when operating as master */
            if (snd_soc_component_read(component, $fmtreg) & M98095_DAI_MAS as c_int) != 0 {
                if (*max98095).sysclk == 0 {
                    dev_err((*component).dev, c"Invalid system clock frequency\n".as_ptr());
                    return -EINVAL;
                }
                ni = 65536u64 * (if rate < 50000 { 96u64 } else { 48u64 }) * rate as u64;
                do_div(&mut ni, (*max98095).sysclk as c_ulonglong);
                snd_soc_component_write(component, $clkhi, ((ni >> 8) & 0x7F) as c_uint);
                snd_soc_component_write(component, $clklo, (ni & 0xFF) as c_uint);
            }
            /* Update sample rate mode */
            if rate < 50000 {
                snd_soc_component_update_bits(component, $filters, M98095_DAI_DHF, 0);
            } else {
                snd_soc_component_update_bits(component, $filters, M98095_DAI_DHF, M98095_DAI_DHF);
            }
            0
        }
    }
}
hw_params_fn!(max98095_dai1_hw_params, 0, M98095_02A_DAI1_FORMAT, M98095_027_DAI1_CLKMODE, M98095_028_DAI1_CLKCFG_HI, M98095_029_DAI1_CLKCFG_LO, M98095_02E_DAI1_FILTERS);
hw_params_fn!(max98095_dai2_hw_params, 1, M98095_034_DAI2_FORMAT, M98095_031_DAI2_CLKMODE, M98095_032_DAI2_CLKCFG_HI, M98095_033_DAI2_CLKCFG_LO, M98095_038_DAI2_FILTERS);
hw_params_fn!(max98095_dai3_hw_params, 2, M98095_03E_DAI3_FORMAT, M98095_03B_DAI3_CLKMODE, M98095_03C_DAI3_CLKCFG_HI, M98095_03D_DAI3_CLKCFG_LO, M98095_042_DAI3_FILTERS);

unsafe fn max98095_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, mut freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    if freq == (*max98095).sysclk { return 0; }
    if !IS_ERR((*max98095).mclk as *const c_void) {
        freq = clk_round_rate((*max98095).mclk, freq as _) as c_uint;
        clk_set_rate((*max98095).mclk, freq as _);
    }
    if freq >= 10000000 && freq < 20000000 {
        snd_soc_component_write(component, M98095_026_SYS_CLK, 0x10);
    } else if freq >= 20000000 && freq < 40000000 {
        snd_soc_component_write(component, M98095_026_SYS_CLK, 0x20);
    } else if freq >= 40000000 && freq < 60000000 {
        snd_soc_component_write(component, M98095_026_SYS_CLK, 0x30);
    } else {
        dev_err((*component).dev, c"Invalid master clock frequency\n".as_ptr());
        return -EINVAL;
    }
    dev_dbg((*dai).dev, c"Clock source is %d at %uHz\n".as_ptr(), clk_id, freq);
    (*max98095).sysclk = freq;
    0
}

macro_rules! set_fmt_fn {
    ($name:ident, $idx:expr, $fmtreg:expr, $clkhi:expr, $clklo:expr, $clock:expr) => {
        unsafe fn $name(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
            let component = (*codec_dai).component;
            let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
            let cdata = &mut (*max98095).dai[$idx] as *mut max98095_cdata;
            let mut regval: u8 = 0;
            if fmt != (*cdata).fmt {
                (*cdata).fmt = fmt;
                match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
                    SND_SOC_DAIFMT_CBC_CFC => {
                        /* Consumer mode PLL */
                        snd_soc_component_write(component, $clkhi, 0x80);
                        snd_soc_component_write(component, $clklo, 0x00);
                    }
                    SND_SOC_DAIFMT_CBP_CFP => { regval |= M98095_DAI_MAS as u8; }
                    _ => {
                        dev_err((*component).dev, c"Clock mode unsupported".as_ptr());
                        return -EINVAL;
                    }
                }
                match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
                    SND_SOC_DAIFMT_I2S => { regval |= M98095_DAI_DLY as u8; }
                    SND_SOC_DAIFMT_LEFT_J => {}
                    _ => return -EINVAL,
                }
                match fmt & SND_SOC_DAIFMT_INV_MASK {
                    SND_SOC_DAIFMT_NB_NF => {}
                    SND_SOC_DAIFMT_NB_IF => { regval |= M98095_DAI_WCI as u8; }
                    SND_SOC_DAIFMT_IB_NF => { regval |= M98095_DAI_BCI as u8; }
                    SND_SOC_DAIFMT_IB_IF => { regval |= (M98095_DAI_BCI | M98095_DAI_WCI) as u8; }
                    _ => return -EINVAL,
                }
                snd_soc_component_update_bits(component, $fmtreg, M98095_DAI_MAS | M98095_DAI_DLY | M98095_DAI_BCI | M98095_DAI_WCI, regval as c_uint);
                snd_soc_component_write(component, $clock, M98095_DAI_BSEL64);
            }
            0
        }
    }
}
set_fmt_fn!(max98095_dai1_set_fmt, 0, M98095_02A_DAI1_FORMAT, M98095_028_DAI1_CLKCFG_HI, M98095_029_DAI1_CLKCFG_LO, M98095_02B_DAI1_CLOCK);
set_fmt_fn!(max98095_dai2_set_fmt, 1, M98095_034_DAI2_FORMAT, M98095_032_DAI2_CLKCFG_HI, M98095_033_DAI2_CLKCFG_LO, M98095_035_DAI2_CLOCK);
set_fmt_fn!(max98095_dai3_set_fmt, 2, M98095_03E_DAI3_FORMAT, M98095_03C_DAI3_CLKCFG_HI, M98095_03D_DAI3_CLKCFG_LO, M98095_03F_DAI3_CLOCK);

unsafe fn max98095_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            if IS_ERR((*max98095).mclk as *const c_void) { return 0; }
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_ON {
                clk_disable_unprepare((*max98095).mclk);
            } else {
                ret = clk_prepare_enable((*max98095).mclk);
                if ret != 0 { return ret; }
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regcache_sync((*max98095).regmap);
                if ret != 0 {
                    dev_err((*component).dev, c"Failed to sync cache: %d\n".as_ptr(), ret);
                    return ret;
                }
            }
            snd_soc_component_update_bits(component, M98095_090_PWR_EN_IN, M98095_MBEN, M98095_MBEN);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, M98095_090_PWR_EN_IN, M98095_MBEN, 0);
            regcache_mark_dirty((*max98095).regmap);
        }
    }
    0
}

const MAX98095_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const MAX98095_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

static max98095_dai1_ops: snd_soc_dai_ops = snd_soc_dai_ops { set_sysclk: Some(max98095_dai_set_sysclk), set_fmt: Some(max98095_dai1_set_fmt), hw_params: Some(max98095_dai1_hw_params), ..unsafe { core::mem::zeroed() } };
static max98095_dai2_ops: snd_soc_dai_ops = snd_soc_dai_ops { set_sysclk: Some(max98095_dai_set_sysclk), set_fmt: Some(max98095_dai2_set_fmt), hw_params: Some(max98095_dai2_hw_params), ..unsafe { core::mem::zeroed() } };
static max98095_dai3_ops: snd_soc_dai_ops = snd_soc_dai_ops { set_sysclk: Some(max98095_dai_set_sysclk), set_fmt: Some(max98095_dai3_set_fmt), hw_params: Some(max98095_dai3_hw_params), ..unsafe { core::mem::zeroed() } };

static mut max98095_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver { name: c"HiFi".as_ptr(), playback: snd_soc_pcm_stream { stream_name: c"HiFi Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: MAX98095_RATES, formats: MAX98095_FORMATS, ..unsafe { core::mem::zeroed() } }, capture: snd_soc_pcm_stream { stream_name: c"HiFi Capture".as_ptr(), channels_min: 1, channels_max: 2, rates: MAX98095_RATES, formats: MAX98095_FORMATS, ..unsafe { core::mem::zeroed() } }, ops: &max98095_dai1_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"Aux".as_ptr(), playback: snd_soc_pcm_stream { stream_name: c"Aux Playback".as_ptr(), channels_min: 1, channels_max: 1, rates: MAX98095_RATES, formats: MAX98095_FORMATS, ..unsafe { core::mem::zeroed() } }, ops: &max98095_dai2_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"Voice".as_ptr(), playback: snd_soc_pcm_stream { stream_name: c"Voice Playback".as_ptr(), channels_min: 1, channels_max: 1, rates: MAX98095_RATES, formats: MAX98095_FORMATS, ..unsafe { core::mem::zeroed() } }, ops: &max98095_dai3_ops, ..unsafe { core::mem::zeroed() } },
];

unsafe fn max98095_get_eq_channel(name: *const c_char) -> c_int {
    if strcmp(name, c"EQ1 Mode".as_ptr()) == 0 { return 0; }
    if strcmp(name, c"EQ2 Mode".as_ptr()) == 0 { return 1; }
    -EINVAL
}

unsafe fn max98095_put_eq_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let pdata = (*max98095).pdata;
    let channel = max98095_get_eq_channel((*kcontrol).id.name.as_ptr());
    let sel = (*ucontrol).value.enumerated.item[0] as c_uint;
    let mut best = 0;
    let mut best_val = INT_MAX;
    if WARN_ON((channel > 1) as c_int) != 0 { return -EINVAL; }
    if pdata.is_null() || (*max98095).eq_textcnt == 0 { return 0; }
    if sel >= (*pdata).eq_cfgcnt { return -EINVAL; }
    let cdata = &mut (*max98095).dai[channel as usize] as *mut max98095_cdata;
    (*cdata).eq_sel = sel as c_int;
    let fs = (*cdata).rate as c_int;
    let mut i = 0;
    while i < (*pdata).eq_cfgcnt as c_int {
        let cfg = (*pdata).eq_cfg.add(i as usize);
        let diff = abs((*cfg).rate - fs);
        if strcmp((*cfg).name, *(*max98095).eq_texts.add(sel as usize)) == 0 && diff < best_val {
            best = i;
            best_val = diff;
        }
        i += 1;
    }
    dev_dbg((*component).dev, c"Selected %s/%dHz for %dHz sample rate\n".as_ptr(), (*(*pdata).eq_cfg.add(best as usize)).name, (*(*pdata).eq_cfg.add(best as usize)).rate, fs);
    let coef_set = (*pdata).eq_cfg.add(best as usize);
    let regmask = if channel == 0 { M98095_EQ1EN } else { M98095_EQ2EN };
    let regsave = snd_soc_component_read(component, M98095_088_CFG_LEVEL);
    snd_soc_component_update_bits(component, M98095_088_CFG_LEVEL, regmask, 0);
    mutex_lock(&mut (*max98095).lock);
    snd_soc_component_update_bits(component, M98095_00F_HOST_CFG, M98095_SEG, M98095_SEG);
    m98095_eq_band(component, channel as c_uint, 0, (*coef_set).band1.as_mut_ptr());
    m98095_eq_band(component, channel as c_uint, 1, (*coef_set).band2.as_mut_ptr());
    m98095_eq_band(component, channel as c_uint, 2, (*coef_set).band3.as_mut_ptr());
    m98095_eq_band(component, channel as c_uint, 3, (*coef_set).band4.as_mut_ptr());
    m98095_eq_band(component, channel as c_uint, 4, (*coef_set).band5.as_mut_ptr());
    snd_soc_component_update_bits(component, M98095_00F_HOST_CFG, M98095_SEG, 0);
    mutex_unlock(&mut (*max98095).lock);
    snd_soc_component_update_bits(component, M98095_088_CFG_LEVEL, regmask, regsave as c_uint);
    0
}

unsafe fn max98095_get_eq_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let channel = max98095_get_eq_channel((*kcontrol).id.name.as_ptr());
    let cdata = &mut (*max98095).dai[channel as usize] as *mut max98095_cdata;
    (*ucontrol).value.enumerated.item[0] = (*cdata).eq_sel as _;
    0
}

unsafe fn max98095_handle_eq_pdata(component: *mut snd_soc_component) {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let pdata = (*max98095).pdata;
    let cfg = (*pdata).eq_cfg;
    let cfgcnt = (*pdata).eq_cfgcnt;
    let mut controls = [
        SOC_ENUM_EXT!("EQ1 Mode", (*max98095).eq_enum, max98095_get_eq_enum, max98095_put_eq_enum),
        SOC_ENUM_EXT!("EQ2 Mode", (*max98095).eq_enum, max98095_get_eq_enum, max98095_put_eq_enum),
    ];
    (*max98095).eq_textcnt = 0;
    (*max98095).eq_texts = core::ptr::null_mut();
    let mut i = 0;
    while i < cfgcnt as c_int {
        let mut j = 0;
        while j < (*max98095).eq_textcnt {
            if strcmp((*cfg.add(i as usize)).name, *(*max98095).eq_texts.add(j as usize)) == 0 { break; }
            j += 1;
        }
        if j == (*max98095).eq_textcnt {
            let t = krealloc((*max98095).eq_texts as *mut c_void, core::mem::size_of::<*const c_char>() * ((*max98095).eq_textcnt as usize + 1), GFP_KERNEL) as *mut *const c_char;
            if !t.is_null() {
                *t.add((*max98095).eq_textcnt as usize) = (*cfg.add(i as usize)).name;
                (*max98095).eq_textcnt += 1;
                (*max98095).eq_texts = t;
            }
        }
        i += 1;
    }
    (*max98095).eq_enum.texts = (*max98095).eq_texts;
    (*max98095).eq_enum.items = (*max98095).eq_textcnt as c_uint;
    let ret = snd_soc_add_component_controls(component, controls.as_mut_ptr(), controls.len() as c_uint);
    if ret != 0 { dev_err((*component).dev, c"Failed to add EQ control: %d\n".as_ptr(), ret); }
}

static bq_mode_name: [*const c_char; 2] = [c"Biquad1 Mode".as_ptr(), c"Biquad2 Mode".as_ptr()];

unsafe fn max98095_get_bq_channel(component: *mut snd_soc_component, name: *const c_char) -> c_int {
    let ret = match_string(bq_mode_name.as_ptr(), bq_mode_name.len() as c_int, name);
    if ret < 0 { dev_err((*component).dev, c"Bad biquad channel name '%s'\n".as_ptr(), name); }
    ret
}

unsafe fn max98095_put_bq_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let pdata = (*max98095).pdata;
    let channel = max98095_get_bq_channel(component, (*kcontrol).id.name.as_ptr());
    let sel = (*ucontrol).value.enumerated.item[0] as c_uint;
    if channel < 0 { return channel; }
    if pdata.is_null() || (*max98095).bq_textcnt == 0 { return 0; }
    if sel >= (*pdata).bq_cfgcnt { return -EINVAL; }
    let cdata = &mut (*max98095).dai[channel as usize] as *mut max98095_cdata;
    (*cdata).bq_sel = sel as c_int;
    let fs = (*cdata).rate as c_int;
    let mut best = 0;
    let mut best_val = INT_MAX;
    let mut i = 0;
    while i < (*pdata).bq_cfgcnt as c_int {
        let cfg = (*pdata).bq_cfg.add(i as usize);
        let diff = abs((*cfg).rate - fs);
        if strcmp((*cfg).name, *(*max98095).bq_texts.add(sel as usize)) == 0 && diff < best_val {
            best = i;
            best_val = diff;
        }
        i += 1;
    }
    dev_dbg((*component).dev, c"Selected %s/%dHz for %dHz sample rate\n".as_ptr(), (*(*pdata).bq_cfg.add(best as usize)).name, (*(*pdata).bq_cfg.add(best as usize)).rate, fs);
    let coef_set = (*pdata).bq_cfg.add(best as usize);
    let regmask = if channel == 0 { M98095_BQ1EN } else { M98095_BQ2EN };
    let regsave = snd_soc_component_read(component, M98095_088_CFG_LEVEL);
    snd_soc_component_update_bits(component, M98095_088_CFG_LEVEL, regmask, 0);
    mutex_lock(&mut (*max98095).lock);
    snd_soc_component_update_bits(component, M98095_00F_HOST_CFG, M98095_SEG, M98095_SEG);
    m98095_biquad_band(component, channel as c_uint, 0, (*coef_set).band1.as_mut_ptr());
    m98095_biquad_band(component, channel as c_uint, 1, (*coef_set).band2.as_mut_ptr());
    snd_soc_component_update_bits(component, M98095_00F_HOST_CFG, M98095_SEG, 0);
    mutex_unlock(&mut (*max98095).lock);
    snd_soc_component_update_bits(component, M98095_088_CFG_LEVEL, regmask, regsave as c_uint);
    0
}

unsafe fn max98095_get_bq_enum(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let channel = max98095_get_bq_channel(component, (*kcontrol).id.name.as_ptr());
    if channel < 0 { return channel; }
    let cdata = &mut (*max98095).dai[channel as usize] as *mut max98095_cdata;
    (*ucontrol).value.enumerated.item[0] = (*cdata).bq_sel as _;
    0
}

unsafe fn max98095_handle_bq_pdata(component: *mut snd_soc_component) {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let pdata = (*max98095).pdata;
    let cfg = (*pdata).bq_cfg;
    let cfgcnt = (*pdata).bq_cfgcnt;
    let mut controls = [
        SOC_ENUM_EXT!(bq_mode_name[0] as *mut c_char, (*max98095).bq_enum, max98095_get_bq_enum, max98095_put_bq_enum),
        SOC_ENUM_EXT!(bq_mode_name[1] as *mut c_char, (*max98095).bq_enum, max98095_get_bq_enum, max98095_put_bq_enum),
    ];
    BUILD_BUG_ON!(controls.len() != bq_mode_name.len());
    (*max98095).bq_textcnt = 0;
    (*max98095).bq_texts = core::ptr::null_mut();
    let mut i = 0;
    while i < cfgcnt as c_int {
        let mut j = 0;
        while j < (*max98095).bq_textcnt {
            if strcmp((*cfg.add(i as usize)).name, *(*max98095).bq_texts.add(j as usize)) == 0 { break; }
            j += 1;
        }
        if j == (*max98095).bq_textcnt {
            let t = krealloc((*max98095).bq_texts as *mut c_void, core::mem::size_of::<*const c_char>() * ((*max98095).bq_textcnt as usize + 1), GFP_KERNEL) as *mut *const c_char;
            if !t.is_null() {
                *t.add((*max98095).bq_textcnt as usize) = (*cfg.add(i as usize)).name;
                (*max98095).bq_textcnt += 1;
                (*max98095).bq_texts = t;
            }
        }
        i += 1;
    }
    (*max98095).bq_enum.texts = (*max98095).bq_texts;
    (*max98095).bq_enum.items = (*max98095).bq_textcnt as c_uint;
    let ret = snd_soc_add_component_controls(component, controls.as_mut_ptr(), controls.len() as c_uint);
    if ret != 0 { dev_err((*component).dev, c"Failed to add Biquad control: %d\n".as_ptr(), ret); }
}

unsafe fn max98095_handle_pdata(component: *mut snd_soc_component) {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let pdata = (*max98095).pdata;
    let mut regval: u8 = 0;
    if pdata.is_null() {
        dev_dbg((*component).dev, c"No platform data\n".as_ptr());
        return;
    }
    if (*pdata).digmic_left_mode { regval |= M98095_DIGMIC_L as u8; }
    if (*pdata).digmic_right_mode { regval |= M98095_DIGMIC_R as u8; }
    snd_soc_component_write(component, M98095_087_CFG_MIC, regval as c_uint);
    if (*pdata).eq_cfgcnt != 0 { max98095_handle_eq_pdata(component); }
    if (*pdata).bq_cfgcnt != 0 { max98095_handle_bq_pdata(component); }
}

unsafe fn max98095_report_jack(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let component = data as *mut snd_soc_component;
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let value = snd_soc_component_read(component, M98095_007_JACK_AUTO_STS) as c_uint;
    let mut hp_report = 0;
    let mut mic_report = 0;
    if (value & M98095_DDONE) == 0 { return IRQ_NONE; }
    if ((value & M98095_HP_IN) != 0 || (value & M98095_LO_IN) != 0) && !(*max98095).headphone_jack.is_null() { hp_report |= SND_JACK_HEADPHONE; }
    if (value & M98095_MIC_IN) != 0 && !(*max98095).mic_jack.is_null() { mic_report |= SND_JACK_MICROPHONE; }
    if (*max98095).headphone_jack == (*max98095).mic_jack {
        snd_soc_jack_report((*max98095).headphone_jack, hp_report | mic_report, SND_JACK_HEADSET);
    } else {
        if !(*max98095).headphone_jack.is_null() { snd_soc_jack_report((*max98095).headphone_jack, hp_report, SND_JACK_HEADPHONE); }
        if !(*max98095).mic_jack.is_null() { snd_soc_jack_report((*max98095).mic_jack, mic_report, SND_JACK_MICROPHONE); }
    }
    IRQ_HANDLED
}

unsafe fn max98095_jack_detect_enable(component: *mut snd_soc_component) -> c_int {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let mut detect_enable = M98095_JDEN;
    let mut slew = M98095_DEFAULT_SLEW_DELAY;
    if (*(*max98095).pdata).jack_detect_pin5en { detect_enable |= M98095_PIN5EN; }
    if (*(*max98095).pdata).jack_detect_delay != 0 { slew = (*(*max98095).pdata).jack_detect_delay; }
    let mut ret = snd_soc_component_write(component, M98095_08E_JACK_DC_SLEW, slew);
    if ret < 0 { dev_err((*component).dev, c"Failed to cfg auto detect %d\n".as_ptr(), ret); return ret; }
    ret = snd_soc_component_write(component, M98095_089_JACK_DET_AUTO, detect_enable);
    if ret < 0 { dev_err((*component).dev, c"Failed to cfg auto detect %d\n".as_ptr(), ret); return ret; }
    ret
}

unsafe fn max98095_jack_detect_disable(component: *mut snd_soc_component) -> c_int {
    let ret = snd_soc_component_write(component, M98095_089_JACK_DET_AUTO, 0x0);
    if ret < 0 { dev_err((*component).dev, c"Failed to cfg auto detect %d\n".as_ptr(), ret); return ret; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn max98095_jack_detect(component: *mut snd_soc_component, hp_jack: *mut snd_soc_jack, mic_jack: *mut snd_soc_jack) -> c_int {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let client = to_i2c_client((*component).dev);
    (*max98095).headphone_jack = hp_jack;
    (*max98095).mic_jack = mic_jack;
    if hp_jack.is_null() && mic_jack.is_null() { return -EINVAL; }
    max98095_jack_detect_enable(component);
    let ret = snd_soc_component_update_bits(component, M98095_013_JACK_INT_EN, M98095_IDDONE, M98095_IDDONE);
    if ret < 0 { dev_err((*component).dev, c"Failed to cfg jack irqs %d\n".as_ptr(), ret); return ret; }
    max98095_report_jack((*client).irq, component as *mut c_void);
    0
}
EXPORT_SYMBOL_GPL!(max98095_jack_detect);

// CONFIG_PM conditional in C: when disabled, suspend/resume are NULL.
unsafe fn max98095_suspend(component: *mut snd_soc_component) -> c_int {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let dapm = snd_soc_component_to_dapm(component);
    if !(*max98095).headphone_jack.is_null() || !(*max98095).mic_jack.is_null() { max98095_jack_detect_disable(component); }
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);
    0
}

unsafe fn max98095_resume(component: *mut snd_soc_component) -> c_int {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let client = to_i2c_client((*component).dev);
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);
    if !(*max98095).headphone_jack.is_null() || !(*max98095).mic_jack.is_null() {
        max98095_jack_detect_enable(component);
        max98095_report_jack((*client).irq, component as *mut c_void);
    }
    0
}

unsafe fn max98095_reset(component: *mut snd_soc_component) -> c_int {
    let mut ret = snd_soc_component_write(component, M98095_00F_HOST_CFG, 0);
    if ret < 0 { dev_err((*component).dev, c"Failed to reset DSP: %d\n".as_ptr(), ret); return ret; }
    ret = snd_soc_component_write(component, M98095_097_PWR_SYS, 0);
    if ret < 0 { dev_err((*component).dev, c"Failed to reset component: %d\n".as_ptr(), ret); return ret; }
    let mut i = M98095_010_HOST_INT_CFG;
    while i < M98095_REG_MAX_CACHED {
        ret = snd_soc_component_write(component, i, snd_soc_component_read(component, i) as c_uint);
        if ret < 0 { dev_err((*component).dev, c"Failed to reset: %d\n".as_ptr(), ret); return ret; }
        i += 1;
    }
    ret
}

unsafe fn max98095_probe(component: *mut snd_soc_component) -> c_int {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let mut ret = 0;
    (*max98095).mclk = devm_clk_get((*component).dev, c"mclk".as_ptr());
    if IS_ERR((*max98095).mclk as *const c_void) && PTR_ERR((*max98095).mclk as *const c_void) == -EPROBE_DEFER as isize { return -EPROBE_DEFER; }
    max98095_reset(component);
    let client = to_i2c_client((*component).dev);
    (*max98095).sysclk = !0u32;
    (*max98095).eq_textcnt = 0;
    (*max98095).bq_textcnt = 0;
    let mut idx = 0usize;
    while idx < 3 {
        (*max98095).dai[idx].rate = !0u32;
        (*max98095).dai[idx].fmt = !0u32;
        (*max98095).dai[idx].eq_sel = 0;
        (*max98095).dai[idx].bq_sel = 0;
        idx += 1;
    }
    (*max98095).lin_state = 0;
    (*max98095).mic1pre = 0;
    (*max98095).mic2pre = 0;
    if (*client).irq != 0 {
        ret = request_threaded_irq((*client).irq, None, Some(max98095_report_jack), IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING | IRQF_ONESHOT, c"max98095".as_ptr(), component as *mut c_void);
        if ret != 0 { dev_err((*component).dev, c"Failed to request IRQ: %d\n".as_ptr(), ret); return ret; }
    }
    ret = snd_soc_component_read(component, M98095_0FF_REV_ID);
    if ret < 0 {
        dev_err((*component).dev, c"Failure reading hardware revision: %d\n".as_ptr(), ret);
        if (*client).irq != 0 { free_irq((*client).irq, component as *mut c_void); }
        return ret;
    }
    dev_info((*component).dev, c"Hardware revision: %c\n".as_ptr(), ret - 0x40 + 'A' as c_int);
    snd_soc_component_write(component, M98095_097_PWR_SYS, M98095_PWRSV);
    snd_soc_component_write(component, M98095_048_MIX_DAC_LR, M98095_DAI1L_TO_DACL | M98095_DAI1R_TO_DACR);
    snd_soc_component_write(component, M98095_049_MIX_DAC_M, M98095_DAI2M_TO_DACM | M98095_DAI3M_TO_DACM);
    snd_soc_component_write(component, M98095_092_PWR_EN_OUT, M98095_SPK_SPREADSPECTRUM);
    snd_soc_component_write(component, M98095_045_CFG_DSP, M98095_DSPNORMAL);
    snd_soc_component_write(component, M98095_04E_CFG_HP, M98095_HPNORMAL);
    snd_soc_component_write(component, M98095_02C_DAI1_IOCFG, M98095_S1NORMAL | M98095_SDATA);
    snd_soc_component_write(component, M98095_036_DAI2_IOCFG, M98095_S2NORMAL | M98095_SDATA);
    snd_soc_component_write(component, M98095_040_DAI3_IOCFG, M98095_S3NORMAL | M98095_SDATA);
    max98095_handle_pdata(component);
    snd_soc_component_update_bits(component, M98095_097_PWR_SYS, M98095_SHDNRUN, M98095_SHDNRUN);
    0
}

unsafe fn max98095_remove(component: *mut snd_soc_component) {
    let max98095 = snd_soc_component_get_drvdata(component) as *mut max98095_priv;
    let client = to_i2c_client((*component).dev);
    if !(*max98095).headphone_jack.is_null() || !(*max98095).mic_jack.is_null() { max98095_jack_detect_disable(component); }
    if (*client).irq != 0 { free_irq((*client).irq, component as *mut c_void); }
}

static soc_component_dev_max98095: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98095_probe),
    remove: Some(max98095_remove),
    suspend: Some(max98095_suspend),
    resume: Some(max98095_resume),
    set_bias_level: Some(max98095_set_bias_level),
    controls: max98095_snd_controls.as_ptr(),
    num_controls: max98095_snd_controls.len() as c_uint,
    dapm_widgets: max98095_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98095_dapm_widgets.len() as c_uint,
    dapm_routes: max98095_audio_map.as_ptr(),
    num_dapm_routes: max98095_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static max98095_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"max98095\0_______________________", driver_data: max98095_type::MAX98095 as usize },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(i2c, max98095_i2c_id);

unsafe fn max98095_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let max98095 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<max98095_priv>(), GFP_KERNEL) as *mut max98095_priv;
    if max98095.is_null() { return -ENOMEM; }
    mutex_init(&mut (*max98095).lock);
    (*max98095).regmap = devm_regmap_init_i2c(i2c, &max98095_regmap);
    if IS_ERR((*max98095).regmap as *const c_void) {
        let ret = PTR_ERR((*max98095).regmap as *const c_void) as c_int;
        dev_err(&mut (*i2c).dev, c"Failed to allocate regmap: %d\n".as_ptr(), ret);
        return ret;
    }
    (*max98095).devtype = core::mem::transmute(i2c_get_match_data(i2c) as usize);
    i2c_set_clientdata(i2c, max98095 as *mut c_void);
    (*max98095).pdata = (*i2c).dev.platform_data as *mut max98095_pdata;
    devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_max98095, max98095_dai.as_mut_ptr(), max98095_dai.len() as c_int)
}

// CONFIG_OF conditional in C.
static max98095_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"maxim,max98095".as_ptr(), ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(of, max98095_of_match);

static mut max98095_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max98095".as_ptr(),
        of_match_table: of_match_ptr(max98095_of_match.as_ptr()),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(max98095_i2c_probe),
    id_table: max98095_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(max98095_i2c_driver);

MODULE_DESCRIPTION!("ALSA SoC MAX98095 driver");
MODULE_AUTHOR!("Peter Hsiang");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
