// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio Misc Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// C dependencies:
// linux/delay.h, linux/dma-mapping.h, linux/io.h, linux/regmap.h, sound/soc.h,
// ../common/mtk-afe-fe-dai.h, ../common/mtk-afe-platform-driver.h,
// mt8186-afe-common.h

use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    static AFE_SINEGEN_CON0: c_uint;
    static AFE_SINEGEN_CON2: c_uint;
    static INNER_LOOP_BACK_MODE_MASK_SFT: c_uint;
    static INNER_LOOP_BACK_MODE_SFT: c_uint;
    static DAC_EN_MASK_SFT: c_uint;
    static DAC_EN_SFT: c_uint;
    static SINE_MODE_CH1_MASK_SFT: c_uint;
    static SINE_MODE_CH1_SFT: c_uint;
    static SINE_MODE_CH2_MASK_SFT: c_uint;
    static SINE_MODE_CH2_SFT: c_uint;
    static AMP_DIV_CH1_MASK: c_int;
    static AMP_DIV_CH1_MASK_SFT: c_uint;
    static AMP_DIV_CH1_SFT: c_uint;
    static AMP_DIV_CH2_MASK_SFT: c_uint;
    static AMP_DIV_CH2_SFT: c_uint;
    static MUTE_SW_CH1_MASK_SFT: c_uint;
    static MUTE_SW_CH1_MASK: c_uint;
    static MUTE_SW_CH2_MASK_SFT: c_uint;
    static MUTE_SW_CH2_MASK: c_uint;
    static FREQ_DIV_CH1_SFT: c_uint;
    static FREQ_DIV_CH1_MASK: c_uint;
    static FREQ_DIV_CH2_SFT: c_uint;
    static FREQ_DIV_CH2_MASK: c_uint;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub platform_priv: *mut mt8186_afe_private,
}

#[repr(C)]
pub struct mt8186_afe_private {
    pub sgen_mode: c_int,
    pub sgen_rate: c_int,
    pub sgen_amplitude: c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

pub type c_long = isize;

const EINVAL: c_int = 22;

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// External kernel/ALSA initializer macros translated as required dependency hooks.
extern "C" {
    fn SOC_ENUM_SINGLE_EXT(items: c_uint, texts: *const *const c_char) -> soc_enum;
    fn SOC_ENUM_EXT(
        name: *const c_char,
        xenum: soc_enum,
        get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
        put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
}

static MT8186_SGEN_MODE_STR: [*const c_char; 64] = [
    c_str!("I0I1"), c_str!("I2"), c_str!("I3I4"), c_str!("I5I6"),
    c_str!("I7I8"), c_str!("I9I22"), c_str!("I10I11"), c_str!("I12I13"),
    c_str!("I14I21"), c_str!("I15I16"), c_str!("I17I18"), c_str!("I19I20"),
    c_str!("I23I24"), c_str!("I25I26"), c_str!("I27I28"), c_str!("I33"),
    c_str!("I34I35"), c_str!("I36I37"), c_str!("I38I39"), c_str!("I40I41"),
    c_str!("I42I43"), c_str!("I44I45"), c_str!("I46I47"), c_str!("I48I49"),
    c_str!("I56I57"), c_str!("I58I59"), c_str!("I60I61"), c_str!("I62I63"),
    c_str!("O0O1"), c_str!("O2"), c_str!("O3O4"), c_str!("O5O6"),
    c_str!("O7O8"), c_str!("O9O10"), c_str!("O11"), c_str!("O12"),
    c_str!("O13O14"), c_str!("O15O16"), c_str!("O17O18"), c_str!("O19O20"),
    c_str!("O21O22"), c_str!("O23O24"), c_str!("O25"), c_str!("O28O29"),
    c_str!("O34"), c_str!("O35"), c_str!("O32O33"), c_str!("O36O37"),
    c_str!("O38O39"), c_str!("O30O31"), c_str!("O40O41"), c_str!("O42O43"),
    c_str!("O44O45"), c_str!("O46O47"), c_str!("O48O49"), c_str!("O50O51"),
    c_str!("O58O59"), c_str!("O60O61"), c_str!("O62O63"), c_str!("O64O65"),
    c_str!("O66O67"), c_str!("O68O69"), c_str!("O26O27"), c_str!("OFF"),
];

static MT8186_SGEN_MODE_IDX: [c_int; 64] = [
    0, 2, 4, 6,
    8, 22, 10, 12,
    14, -1, 18, 20,
    24, 26, 28, 33,
    34, 36, 38, 40,
    42, 44, 46, 48,
    56, 58, 60, 62,
    128, 130, 132, 134,
    135, 138, 139, 140,
    142, 144, 166, 148,
    150, 152, 153, 156,
    162, 163, 160, 164,
    166, -1, 168, 170,
    172, 174, 176, 178,
    186, 188, 190, 192,
    194, 196, -1, -1,
];

static MT8186_SGEN_RATE_STR: [*const c_char; 13] = [
    c_str!("8K"), c_str!("11K"), c_str!("12K"), c_str!("16K"),
    c_str!("22K"), c_str!("24K"), c_str!("32K"), c_str!("44K"),
    c_str!("48K"), c_str!("88k"), c_str!("96k"), c_str!("176k"),
    c_str!("192k"),
];

static MT8186_SGEN_RATE_IDX: [c_int; 13] = [
    0, 1, 2, 4,
    5, 6, 8, 9,
    10, 11, 12, 13,
    14,
];

/* this order must match reg bit amp_div_ch1/2 */
static MT8186_SGEN_AMP_STR: [*const c_char; 8] = [
    c_str!("1/128"), c_str!("1/64"), c_str!("1/32"), c_str!("1/16"),
    c_str!("1/8"), c_str!("1/4"), c_str!("1/2"), c_str!("1"),
];

unsafe extern "C" fn mt8186_sgen_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv;

    (*ucontrol).value.integer.value[0] = (*afe_priv).sgen_mode as c_long;

    0
}

unsafe extern "C" fn mt8186_sgen_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv;
    let e: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let mode: c_int;
    let mode_idx: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    mode = (*ucontrol).value.integer.value[0] as c_int;
    mode_idx = MT8186_SGEN_MODE_IDX[mode as usize];

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), mode %d, mode_idx %d\n"),
        c_str!("mt8186_sgen_set"),
        mode,
        mode_idx,
    );

    if mode == (*afe_priv).sgen_mode {
        return 0;
    }

    if mode_idx >= 0 {
        regmap_update_bits(
            (*afe).regmap,
            AFE_SINEGEN_CON2,
            INNER_LOOP_BACK_MODE_MASK_SFT,
            (mode_idx as c_uint) << INNER_LOOP_BACK_MODE_SFT,
        );
        regmap_update_bits(
            (*afe).regmap,
            AFE_SINEGEN_CON0,
            DAC_EN_MASK_SFT,
            bit(DAC_EN_SFT),
        );
    } else {
        /* disable sgen */
        regmap_update_bits((*afe).regmap, AFE_SINEGEN_CON0, DAC_EN_MASK_SFT, 0);
        regmap_update_bits(
            (*afe).regmap,
            AFE_SINEGEN_CON2,
            INNER_LOOP_BACK_MODE_MASK_SFT,
            0x3f << INNER_LOOP_BACK_MODE_SFT,
        );
    }

    (*afe_priv).sgen_mode = mode;

    1
}

unsafe extern "C" fn mt8186_sgen_rate_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv;

    (*ucontrol).value.integer.value[0] = (*afe_priv).sgen_rate as c_long;

    0
}

unsafe extern "C" fn mt8186_sgen_rate_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv;
    let e: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let rate: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    rate = (*ucontrol).value.integer.value[0] as c_int;

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), rate %d\n"),
        c_str!("mt8186_sgen_rate_set"),
        rate,
    );

    if rate == (*afe_priv).sgen_rate {
        return 0;
    }

    regmap_update_bits(
        (*afe).regmap,
        AFE_SINEGEN_CON0,
        SINE_MODE_CH1_MASK_SFT,
        (MT8186_SGEN_RATE_IDX[rate as usize] as c_uint) << SINE_MODE_CH1_SFT,
    );

    regmap_update_bits(
        (*afe).regmap,
        AFE_SINEGEN_CON0,
        SINE_MODE_CH2_MASK_SFT,
        (MT8186_SGEN_RATE_IDX[rate as usize] as c_uint) << SINE_MODE_CH2_SFT,
    );

    (*afe_priv).sgen_rate = rate;

    1
}

unsafe extern "C" fn mt8186_sgen_amplitude_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv;

    (*ucontrol).value.integer.value[0] = (*afe_priv).sgen_amplitude as c_long;
    0
}

unsafe extern "C" fn mt8186_sgen_amplitude_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv;
    let e: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let amplitude: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    amplitude = (*ucontrol).value.integer.value[0] as c_int;
    if amplitude > AMP_DIV_CH1_MASK {
        dev_err(
            (*afe).dev,
            c_str!("%s(), amplitude %d invalid\n"),
            c_str!("mt8186_sgen_amplitude_set"),
            amplitude,
        );
        return -EINVAL;
    }

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), amplitude %d\n"),
        c_str!("mt8186_sgen_amplitude_set"),
        amplitude,
    );

    if amplitude == (*afe_priv).sgen_amplitude {
        return 0;
    }

    regmap_update_bits(
        (*afe).regmap,
        AFE_SINEGEN_CON0,
        AMP_DIV_CH1_MASK_SFT,
        (amplitude as c_uint) << AMP_DIV_CH1_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_SINEGEN_CON0,
        AMP_DIV_CH2_MASK_SFT,
        (amplitude as c_uint) << AMP_DIV_CH2_SFT,
    );

    (*afe_priv).sgen_amplitude = amplitude;

    1
}

static MT8186_AFE_SGEN_ENUM: [soc_enum; 3] = unsafe {
    [
        SOC_ENUM_SINGLE_EXT(MT8186_SGEN_MODE_STR.len() as c_uint, MT8186_SGEN_MODE_STR.as_ptr()),
        SOC_ENUM_SINGLE_EXT(MT8186_SGEN_RATE_STR.len() as c_uint, MT8186_SGEN_RATE_STR.as_ptr()),
        SOC_ENUM_SINGLE_EXT(MT8186_SGEN_AMP_STR.len() as c_uint, MT8186_SGEN_AMP_STR.as_ptr()),
    ]
};

static MT8186_AFE_SGEN_CONTROLS: [snd_kcontrol_new; 7] = unsafe {
    [
        SOC_ENUM_EXT(
            c_str!("Audio_SineGen_Switch"),
            MT8186_AFE_SGEN_ENUM[0],
            mt8186_sgen_get,
            mt8186_sgen_set,
        ),
        SOC_ENUM_EXT(
            c_str!("Audio_SineGen_SampleRate"),
            MT8186_AFE_SGEN_ENUM[1],
            mt8186_sgen_rate_get,
            mt8186_sgen_rate_set,
        ),
        SOC_ENUM_EXT(
            c_str!("Audio_SineGen_Amplitude"),
            MT8186_AFE_SGEN_ENUM[2],
            mt8186_sgen_amplitude_get,
            mt8186_sgen_amplitude_set,
        ),
        SOC_SINGLE(
            c_str!("Audio_SineGen_Mute_Ch1"),
            AFE_SINEGEN_CON0,
            MUTE_SW_CH1_MASK_SFT,
            MUTE_SW_CH1_MASK,
            0,
        ),
        SOC_SINGLE(
            c_str!("Audio_SineGen_Mute_Ch2"),
            AFE_SINEGEN_CON0,
            MUTE_SW_CH2_MASK_SFT,
            MUTE_SW_CH2_MASK,
            0,
        ),
        SOC_SINGLE(
            c_str!("Audio_SineGen_Freq_Div_Ch1"),
            AFE_SINEGEN_CON0,
            FREQ_DIV_CH1_SFT,
            FREQ_DIV_CH1_MASK,
            0,
        ),
        SOC_SINGLE(
            c_str!("Audio_SineGen_Freq_Div_Ch2"),
            AFE_SINEGEN_CON0,
            FREQ_DIV_CH2_SFT,
            FREQ_DIV_CH2_MASK,
            0,
        ),
    ]
};

#[no_mangle]
pub unsafe extern "C" fn mt8186_add_misc_control(component: *mut snd_soc_component) -> c_int {
    snd_soc_add_component_controls(
        component,
        MT8186_AFE_SGEN_CONTROLS.as_ptr(),
        MT8186_AFE_SGEN_CONTROLS.len() as c_uint,
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
