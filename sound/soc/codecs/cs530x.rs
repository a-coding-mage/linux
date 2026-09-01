// SPDX-License-Identifier: GPL-2.0
//
// CS530x CODEC driver
//
// Copyright (C) 2024-2025 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.

// C dependencies removed from executable Rust:
// linux/delay.h, linux/i2c.h, linux/init.h, linux/module.h, linux/pm.h,
// linux/property.h, linux/slab.h, linux/spi/spi.h, sound/core.h,
// sound/initval.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h,
// and "cs530x.h".

static cs530x_supply_names: [*const core::ffi::c_char; CS530X_NUM_SUPPLIES] = [
    c"vdd-a".as_ptr(),
    c"vdd-io".as_ptr(),
];

static cs530x_reg_defaults: [reg_default; 32] = [
    reg_default { reg: CS530X_CLK_CFG_0, def: 0x30 },
    reg_default { reg: CS530X_CLK_CFG_1, def: 0x0001 },
    reg_default { reg: CS530X_CHIP_ENABLE, def: 0 },
    reg_default { reg: CS530X_ASP_CFG, def: 0 },
    reg_default { reg: CS530X_SIGNAL_PATH_CFG, def: 0 },
    reg_default { reg: CS530X_IN_ENABLES, def: 0 },
    reg_default { reg: CS530X_IN_RAMP_SUM, def: 0x0022 },
    reg_default { reg: CS530X_IN_FILTER, def: 0 },
    reg_default { reg: CS530X_IN_HIZ, def: 0 },
    reg_default { reg: CS530X_IN_INV, def: 0 },
    reg_default { reg: CS530X_IN_VOL_CTRL1_0, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL1_1, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL2_0, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL2_1, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL3_0, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL3_1, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL4_0, def: 0x8000 },
    reg_default { reg: CS530X_IN_VOL_CTRL4_1, def: 0x8000 },
    reg_default { reg: CS530X_OUT_ENABLES, def: 0 },
    reg_default { reg: CS530X_OUT_RAMP_SUM, def: 0x0022 },
    reg_default { reg: CS530X_OUT_FILTER, def: 0 },
    reg_default { reg: CS530X_OUT_INV, def: 0 },
    reg_default { reg: CS530X_OUT_VOL_CTRL1_0, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL1_1, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL2_0, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL2_1, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL3_0, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL3_1, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL4_0, def: 0x8000 },
    reg_default { reg: CS530X_OUT_VOL_CTRL4_1, def: 0x8000 },
    reg_default { reg: CS530X_PAD_FN, def: 0 },
    reg_default { reg: CS530X_PAD_LVL, def: 0 },
];

fn cs530x_read_and_write_regs(reg: core::ffi::c_uint) -> bool {
    match reg {
        CS530X_CLK_CFG_0 | CS530X_CLK_CFG_1 | CS530X_CHIP_ENABLE | CS530X_ASP_CFG
        | CS530X_SIGNAL_PATH_CFG | CS530X_IN_ENABLES | CS530X_IN_RAMP_SUM
        | CS530X_IN_FILTER | CS530X_IN_HIZ | CS530X_IN_INV | CS530X_IN_VOL_CTRL1_0
        | CS530X_IN_VOL_CTRL1_1 | CS530X_IN_VOL_CTRL2_0 | CS530X_IN_VOL_CTRL2_1
        | CS530X_IN_VOL_CTRL3_0 | CS530X_IN_VOL_CTRL3_1 | CS530X_IN_VOL_CTRL4_0
        | CS530X_IN_VOL_CTRL4_1 | CS530X_OUT_ENABLES | CS530X_OUT_RAMP_SUM
        | CS530X_OUT_DEEMPH | CS530X_OUT_FILTER | CS530X_OUT_INV
        | CS530X_OUT_VOL_CTRL1_0 | CS530X_OUT_VOL_CTRL1_1 | CS530X_OUT_VOL_CTRL2_0
        | CS530X_OUT_VOL_CTRL2_1 | CS530X_OUT_VOL_CTRL3_0 | CS530X_OUT_VOL_CTRL3_1
        | CS530X_OUT_VOL_CTRL4_0 | CS530X_OUT_VOL_CTRL4_1 | CS530X_PAD_FN
        | CS530X_PAD_LVL => true,
        _ => false,
    }
}

unsafe fn cs530x_readable_register(_dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        CS530X_DEVID | CS530X_REVID => true,
        _ => cs530x_read_and_write_regs(reg),
    }
}

unsafe fn cs530x_writeable_register(_dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        CS530X_SW_RESET | CS530X_IN_VOL_CTRL5 | CS530X_OUT_VOL_CTRL5 => true,
        _ => cs530x_read_and_write_regs(reg),
    }
}

unsafe fn cs530x_put_volsw_vu(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> core::ffi::c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dapm = snd_soc_component_to_dapm(component);
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;
    let ret: core::ffi::c_int;

    snd_soc_dapm_mutex_lock(dapm);

    ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret != 0 {
        snd_soc_dapm_mutex_unlock(dapm);
        return ret;
    }

    /* Write INOUT_VU bit for the volume change to take effect */
    regmap_write(regmap, CS530X_IN_VOL_CTRL5, CS530X_INOUT_VU);

    snd_soc_dapm_mutex_unlock(dapm);

    ret
}

static in_vol_tlv: [_; 1] = [DECLARE_TLV_DB_SCALE!(-1270, 50, 0)];

static cs530x_inout_filter_text: [*const core::ffi::c_char; 4] = [
    c"Min Phase Slow Roll-off".as_ptr(),
    c"Min Phase Fast Roll-off".as_ptr(),
    c"Linear Phase Slow Roll-off".as_ptr(),
    c"Linear Phase Fast Roll-off".as_ptr(),
];

static cs530x_in_filter_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_IN_FILTER, CS530X_INOUT_FILTER_SHIFT, cs530x_inout_filter_text);
static cs530x_out_filter_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_OUT_FILTER, CS530X_INOUT_FILTER_SHIFT, cs530x_inout_filter_text);

static cs530x_4ch_sum_text: [*const core::ffi::c_char; 3] = [
    c"None".as_ptr(),
    c"Groups of 2".as_ptr(),
    c"Groups of 4".as_ptr(),
];

static cs530x_in_sum_ch4_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_IN_RAMP_SUM, CS530X_INOUT_SUM_MODE_SHIFT, cs530x_4ch_sum_text);
static cs530x_in_sum_4ch_controls: [snd_kcontrol_new; 1] =
    [SOC_ENUM!(c"IN Sum Select".as_ptr(), cs530x_in_sum_ch4_enum)];

static cs530x_out_sum_ch4_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_OUT_RAMP_SUM, CS530X_INOUT_SUM_MODE_SHIFT, cs530x_4ch_sum_text);
static cs530x_out_sum_4ch_controls: [snd_kcontrol_new; 1] =
    [SOC_ENUM!(c"OUT Sum Select".as_ptr(), cs530x_out_sum_ch4_enum)];

static cs530x_8ch_sum_text: [*const core::ffi::c_char; 4] = [
    c"None".as_ptr(),
    c"Groups of 2".as_ptr(),
    c"Groups of 4".as_ptr(),
    c"Groups of 8".as_ptr(),
];

static cs530x_in_sum_ch8_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_IN_RAMP_SUM, CS530X_INOUT_SUM_MODE_SHIFT, cs530x_8ch_sum_text);
static cs530x_in_sum_8ch_controls: [snd_kcontrol_new; 1] =
    [SOC_ENUM!(c"IN Sum Select".as_ptr(), cs530x_in_sum_ch8_enum)];

static cs530x_out_sum_ch8_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_OUT_RAMP_SUM, CS530X_INOUT_SUM_MODE_SHIFT, cs530x_8ch_sum_text);
static cs530x_out_sum_8ch_controls: [snd_kcontrol_new; 1] =
    [SOC_ENUM!(c"OUT Sum Select".as_ptr(), cs530x_out_sum_ch8_enum)];

static cs530x_vol_ramp_text: [*const core::ffi::c_char; 8] = [
    c"0ms/6dB".as_ptr(),
    c"0.5ms/6dB".as_ptr(),
    c"1ms/6dB".as_ptr(),
    c"2ms/6dB".as_ptr(),
    c"4ms/6dB".as_ptr(),
    c"8ms/6dB".as_ptr(),
    c"15ms/6dB".as_ptr(),
    c"30ms/6dB".as_ptr(),
];

static cs530x_ramp_inc_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_IN_RAMP_SUM, CS530X_RAMP_RATE_INC_SHIFT, cs530x_vol_ramp_text);
static cs530x_ramp_dec_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_IN_RAMP_SUM, CS530X_RAMP_RATE_DEC_SHIFT, cs530x_vol_ramp_text);

static cs530x_in_1_to_2_controls: [snd_kcontrol_new; 7] = [
    SOC_SINGLE_EXT_TLV!(c"IN1 Volume".as_ptr(), CS530X_IN_VOL_CTRL1_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"IN2 Volume".as_ptr(), CS530X_IN_VOL_CTRL1_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_ENUM!(c"IN DEC Filter Select".as_ptr(), cs530x_in_filter_enum),
    SOC_ENUM!(c"Input Ramp Up".as_ptr(), cs530x_ramp_inc_enum),
    SOC_ENUM!(c"Input Ramp Down".as_ptr(), cs530x_ramp_dec_enum),
    SOC_SINGLE!(c"ADC1 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT1_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"ADC2 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT2_INV_SHIFT, 1, 0),
];

static cs530x_in_3_to_4_controls: [snd_kcontrol_new; 4] = [
    SOC_SINGLE_EXT_TLV!(c"IN3 Volume".as_ptr(), CS530X_IN_VOL_CTRL2_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"IN4 Volume".as_ptr(), CS530X_IN_VOL_CTRL2_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE!(c"ADC3 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT3_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"ADC4 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT4_INV_SHIFT, 1, 0),
];

static cs530x_in_5_to_8_controls: [snd_kcontrol_new; 8] = [
    SOC_SINGLE_EXT_TLV!(c"IN5 Volume".as_ptr(), CS530X_IN_VOL_CTRL3_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"IN6 Volume".as_ptr(), CS530X_IN_VOL_CTRL3_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"IN7 Volume".as_ptr(), CS530X_IN_VOL_CTRL4_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"IN8 Volume".as_ptr(), CS530X_IN_VOL_CTRL4_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE!(c"ADC5 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT5_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"ADC6 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT6_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"ADC7 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT7_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"ADC8 Invert Switch".as_ptr(), CS530X_IN_INV, CS530X_INOUT8_INV_SHIFT, 1, 0),
];

unsafe fn cs530x_adc_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            (*cs530x).adc_pairs_count += 1;
        }
        SND_SOC_DAPM_POST_PMU => {
            regmap_clear_bits(regmap, CS530X_IN_VOL_CTRL1_0 + ((*w).shift * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            regmap_clear_bits(regmap, CS530X_IN_VOL_CTRL1_0 + (((*w).shift + 1) * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            (*cs530x).adc_pairs_count -= 1;
            if (*cs530x).adc_pairs_count == 0 {
                usleep_range(1000, 1100);
                return regmap_write(regmap, CS530X_IN_VOL_CTRL5, CS530X_INOUT_VU);
            }
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_set_bits(regmap, CS530X_IN_VOL_CTRL1_0 + ((*w).shift * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            regmap_set_bits(regmap, CS530X_IN_VOL_CTRL1_0 + (((*w).shift + 1) * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            return regmap_write(regmap, CS530X_IN_VOL_CTRL5, CS530X_INOUT_VU);
        }
        _ => return -EINVAL,
    }

    0
}

static cs530x_ramp_out_inc_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_OUT_RAMP_SUM, CS530X_RAMP_RATE_INC_SHIFT, cs530x_vol_ramp_text);
static cs530x_ramp_out_dec_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(CS530X_OUT_RAMP_SUM, CS530X_RAMP_RATE_DEC_SHIFT, cs530x_vol_ramp_text);

static cs530x_out_1_to_2_controls: [snd_kcontrol_new; 7] = [
    SOC_SINGLE_EXT_TLV!(c"OUT1 Volume".as_ptr(), CS530X_OUT_VOL_CTRL1_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"OUT2 Volume".as_ptr(), CS530X_OUT_VOL_CTRL1_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_ENUM!(c"OUT DEC Filter Select".as_ptr(), cs530x_out_filter_enum),
    SOC_ENUM!(c"Output Ramp Up".as_ptr(), cs530x_ramp_out_inc_enum),
    SOC_ENUM!(c"Output Ramp Down".as_ptr(), cs530x_ramp_out_dec_enum),
    SOC_SINGLE!(c"DAC1 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT1_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"DAC2 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT2_INV_SHIFT, 1, 0),
];

static cs530x_out_3_to_4_controls: [snd_kcontrol_new; 4] = [
    SOC_SINGLE_EXT_TLV!(c"OUT3 Volume".as_ptr(), CS530X_OUT_VOL_CTRL2_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"OUT4 Volume".as_ptr(), CS530X_OUT_VOL_CTRL2_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE!(c"DAC3 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT3_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"DAC4 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT4_INV_SHIFT, 1, 0),
];

static cs530x_out_5_to_8_controls: [snd_kcontrol_new; 8] = [
    SOC_SINGLE_EXT_TLV!(c"OUT5 Volume".as_ptr(), CS530X_OUT_VOL_CTRL3_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"OUT6 Volume".as_ptr(), CS530X_OUT_VOL_CTRL3_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"OUT7 Volume".as_ptr(), CS530X_OUT_VOL_CTRL4_0, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE_EXT_TLV!(c"OUT8 Volume".as_ptr(), CS530X_OUT_VOL_CTRL4_1, 0, 255, 1, snd_soc_get_volsw, cs530x_put_volsw_vu, in_vol_tlv),
    SOC_SINGLE!(c"DAC5 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT5_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"DAC6 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT6_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"DAC7 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT7_INV_SHIFT, 1, 0),
    SOC_SINGLE!(c"DAC8 Invert Switch".as_ptr(), CS530X_OUT_INV, CS530X_INOUT8_INV_SHIFT, 1, 0),
];

unsafe fn cs530x_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            (*cs530x).dac_pairs_count += 1;
        }
        SND_SOC_DAPM_POST_PMU => {
            regmap_clear_bits(regmap, CS530X_OUT_VOL_CTRL1_0 + ((*w).shift * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            regmap_clear_bits(regmap, CS530X_OUT_VOL_CTRL1_0 + (((*w).shift + 1) * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            (*cs530x).dac_pairs_count -= 1;
            if (*cs530x).dac_pairs_count == 0 {
                usleep_range(1000, 1100);
                return regmap_write(regmap, CS530X_OUT_VOL_CTRL5, CS530X_INOUT_VU);
            }
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_set_bits(regmap, CS530X_OUT_VOL_CTRL1_0 + ((*w).shift * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            regmap_set_bits(regmap, CS530X_OUT_VOL_CTRL1_0 + (((*w).shift + 1) * 2) as core::ffi::c_uint, CS530X_INOUT_MUTE);
            return regmap_write(regmap, CS530X_OUT_VOL_CTRL5, CS530X_INOUT_VU);
        }
        _ => return -EINVAL,
    }

    0
}

static adc12_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static adc34_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static adc56_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static adc78_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static dac12_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static dac34_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static dac56_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static dac78_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static in_hpf_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);
static out_hpf_ctrl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT!(c"Switch".as_ptr(), 1);

/* General DAPM widgets for all devices */
static cs530x_gen_dapm_widgets: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_SUPPLY!(c"Global Enable".as_ptr(), CS530X_CHIP_ENABLE, 0, 0, core::ptr::null_mut(), 0),
];

/* ADC's Channels 1 and 2 plus generic ADC DAPM events */
static cs530x_adc_ch12_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_INPUT!(c"IN1".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"IN2".as_ptr()),
    SND_SOC_DAPM_ADC_E!(c"ADC1".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 0, 0, cs530x_adc_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_ADC!(c"ADC2".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 1, 0),
    SND_SOC_DAPM_SWITCH!(c"ADC12 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &adc12_ctrl),
    SND_SOC_DAPM_SWITCH!(c"IN HPF".as_ptr(), CS530X_IN_FILTER, CS530X_INOUT_HPF_EN_SHIFT, 0, &in_hpf_ctrl),
];

/* ADC's Channels 3 and 4 */
static cs530x_adc_ch34_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_INPUT!(c"IN3".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"IN4".as_ptr()),
    SND_SOC_DAPM_ADC_E!(c"ADC3".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 2, 0, cs530x_adc_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_ADC!(c"ADC4".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 3, 0),
    SND_SOC_DAPM_SWITCH!(c"ADC34 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &adc34_ctrl),
];

/* ADC's Channels 5 to 8 */
static cs530x_adc_ch58_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    SND_SOC_DAPM_INPUT!(c"IN5".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"IN6".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"IN7".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"IN8".as_ptr()),
    SND_SOC_DAPM_ADC_E!(c"ADC5".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 4, 0, cs530x_adc_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_ADC!(c"ADC6".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 5, 0),
    SND_SOC_DAPM_SWITCH!(c"ADC56 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &adc56_ctrl),
    SND_SOC_DAPM_ADC_E!(c"ADC7".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 6, 0, cs530x_adc_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_ADC!(c"ADC8".as_ptr(), core::ptr::null_mut(), CS530X_IN_ENABLES, 7, 0),
    SND_SOC_DAPM_SWITCH!(c"ADC78 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &adc78_ctrl),
];

static adc_ch1_2_routes: [snd_soc_dapm_route; 11] = [
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC12 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC12 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: core::ptr::null(), source: c"ADC12 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: core::ptr::null(), source: c"ADC12 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC1".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC2".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"IN HPF".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC1".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC2".as_ptr() },
];

static adc_ch3_4_routes: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route { sink: c"ADC3".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC4".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC34 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN3".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC34 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN4".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC3".as_ptr(), control: core::ptr::null(), source: c"ADC34 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC4".as_ptr(), control: core::ptr::null(), source: c"ADC34 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC3".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC4".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC3".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC4".as_ptr() },
];

static adc_ch5_8_routes: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: c"ADC5".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC6".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC7".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC8".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC56 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN5".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC56 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN6".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC5".as_ptr(), control: core::ptr::null(), source: c"ADC56 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC6".as_ptr(), control: core::ptr::null(), source: c"ADC56 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC5".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC6".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC5".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC6".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC78 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN7".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC78 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"IN8".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC7".as_ptr(), control: core::ptr::null(), source: c"ADC78 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC8".as_ptr(), control: core::ptr::null(), source: c"ADC78 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC7".as_ptr() },
    snd_soc_dapm_route { sink: c"IN HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"ADC8".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC7".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF Capture".as_ptr(), control: core::ptr::null(), source: c"ADC8".as_ptr() },
];

unsafe fn cs530x_add_12_adc_widgets(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_add_component_controls(component, cs530x_in_1_to_2_controls.as_ptr(), cs530x_in_1_to_2_controls.len() as core::ffi::c_uint);
    snd_soc_dapm_new_controls(dapm, cs530x_adc_ch12_dapm_widgets.as_ptr(), cs530x_adc_ch12_dapm_widgets.len() as core::ffi::c_int);
    snd_soc_dapm_add_routes(dapm, adc_ch1_2_routes.as_ptr(), adc_ch1_2_routes.len() as core::ffi::c_int);
}

unsafe fn cs530x_add_34_adc_widgets(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_add_component_controls(component, cs530x_in_3_to_4_controls.as_ptr(), cs530x_in_3_to_4_controls.len() as core::ffi::c_uint);
    snd_soc_dapm_new_controls(dapm, cs530x_adc_ch34_dapm_widgets.as_ptr(), cs530x_adc_ch34_dapm_widgets.len() as core::ffi::c_int);
    snd_soc_dapm_add_routes(dapm, adc_ch3_4_routes.as_ptr(), adc_ch3_4_routes.len() as core::ffi::c_int);
}

/* DAC's Channels 1 and 2 plus generic DAC DAPM events */
static cs530x_dac_ch12_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_OUTPUT!(c"OUT1".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"OUT2".as_ptr()),
    SND_SOC_DAPM_DAC_E!(c"DAC1".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 0, 0, cs530x_dac_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_DAC!(c"DAC2".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 1, 0),
    SND_SOC_DAPM_SWITCH!(c"DAC12 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &dac12_ctrl),
    SND_SOC_DAPM_SWITCH!(c"OUT HPF".as_ptr(), CS530X_OUT_FILTER, CS530X_INOUT_HPF_EN_SHIFT, 0, &out_hpf_ctrl),
];

/* DAC's Channels 3 and 4 */
static cs530x_dac_ch34_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_OUTPUT!(c"OUT3".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"OUT4".as_ptr()),
    SND_SOC_DAPM_DAC_E!(c"DAC3".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 2, 0, cs530x_dac_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_DAC!(c"DAC4".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 3, 0),
    SND_SOC_DAPM_SWITCH!(c"DAC34 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &dac34_ctrl),
];

/* DAC's Channels 5 to 8 */
static cs530x_dac_ch58_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    SND_SOC_DAPM_OUTPUT!(c"OUT5".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"OUT6".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"OUT7".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"OUT8".as_ptr()),
    SND_SOC_DAPM_DAC_E!(c"DAC5".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 4, 0, cs530x_dac_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_DAC!(c"DAC6".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 5, 0),
    SND_SOC_DAPM_SWITCH!(c"DAC56 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &dac56_ctrl),
    SND_SOC_DAPM_DAC_E!(c"DAC7".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 6, 0, cs530x_dac_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_DAC!(c"DAC8".as_ptr(), core::ptr::null_mut(), CS530X_OUT_ENABLES, 7, 0),
    SND_SOC_DAPM_SWITCH!(c"DAC78 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &dac78_ctrl),
];

static dac_ch1_2_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: c"DAC1".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC12 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC12 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT2".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC1".as_ptr(), control: core::ptr::null(), source: c"DAC12 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2".as_ptr(), control: core::ptr::null(), source: c"DAC12 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC1".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT1".as_ptr(), control: core::ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT2".as_ptr(), control: core::ptr::null(), source: c"DAC2".as_ptr() },
];

static dac_ch3_4_routes: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route { sink: c"DAC3".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC4".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC34 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT3".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC34 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT4".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC3".as_ptr(), control: core::ptr::null(), source: c"DAC34 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC4".as_ptr(), control: core::ptr::null(), source: c"DAC34 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC4".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC3".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC4".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT3".as_ptr(), control: core::ptr::null(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT4".as_ptr(), control: core::ptr::null(), source: c"DAC4".as_ptr() },
];

static dac_ch5_8_routes: [snd_soc_dapm_route; 24] = [
    snd_soc_dapm_route { sink: c"DAC5".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC6".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC56 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT5".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC56 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT6".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC5".as_ptr(), control: core::ptr::null(), source: c"DAC56 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC6".as_ptr(), control: core::ptr::null(), source: c"DAC56 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC5".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC6".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC5".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC6".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT5".as_ptr(), control: core::ptr::null(), source: c"DAC5".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT6".as_ptr(), control: core::ptr::null(), source: c"DAC6".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC7".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC8".as_ptr(), control: core::ptr::null(), source: c"Global Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC78 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT7".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC78 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"OUT8".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC7".as_ptr(), control: core::ptr::null(), source: c"DAC78 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC8".as_ptr(), control: core::ptr::null(), source: c"DAC78 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC7".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT HPF".as_ptr(), control: c"Switch".as_ptr(), source: c"DAC8".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC7".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC8".as_ptr(), control: core::ptr::null(), source: c"AIF Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT7".as_ptr(), control: core::ptr::null(), source: c"DAC7".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT8".as_ptr(), control: core::ptr::null(), source: c"DAC8".as_ptr() },
];

unsafe fn cs530x_add_12_dac_widgets(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_add_component_controls(component, cs530x_out_1_to_2_controls.as_ptr(), cs530x_out_1_to_2_controls.len() as core::ffi::c_uint);
    snd_soc_dapm_new_controls(dapm, cs530x_dac_ch12_dapm_widgets.as_ptr(), cs530x_dac_ch12_dapm_widgets.len() as core::ffi::c_int);
    snd_soc_dapm_add_routes(dapm, dac_ch1_2_routes.as_ptr(), dac_ch1_2_routes.len() as core::ffi::c_int);
}

unsafe fn cs530x_add_34_dac_widgets(component: *mut snd_soc_component) {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_add_component_controls(component, cs530x_out_3_to_4_controls.as_ptr(), cs530x_out_3_to_4_controls.len() as core::ffi::c_uint);
    snd_soc_dapm_new_controls(dapm, cs530x_dac_ch34_dapm_widgets.as_ptr(), cs530x_dac_ch34_dapm_widgets.len() as core::ffi::c_int);
    snd_soc_dapm_add_routes(dapm, dac_ch3_4_routes.as_ptr(), dac_ch3_4_routes.len() as core::ffi::c_int);
}

unsafe fn cs530x_set_bclk(component: *mut snd_soc_component, freq: core::ffi::c_int) -> core::ffi::c_int {
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;
    let bclk_val: core::ffi::c_uint;

    match freq {
        2822400 | 3072000 => bclk_val = CS530X_BCLK_2P822_3P072,
        5644800 | 6144000 => bclk_val = CS530X_BCLK_5P6448_6P144,
        11289600 | 12288000 => bclk_val = CS530X_BCLK_11P2896_12P288,
        22579200 | 24576000 => bclk_val = CS530X_BCLK_24P5792_24P576,
        _ => {
            dev_err((*component).dev, c"Invalid BCLK frequency %d\n".as_ptr(), freq);
            return -EINVAL;
        }
    }

    dev_dbg((*component).dev, c"BCLK frequency is %d\n".as_ptr(), freq);
    regmap_update_bits(regmap, CS530X_ASP_CFG, CS530X_ASP_BCLK_FREQ_MASK, bclk_val)
}

unsafe fn cs530x_set_pll_refclk(component: *mut snd_soc_component, freq: core::ffi::c_uint) -> core::ffi::c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*priv_).regmap;
    let refclk: core::ffi::c_uint;

    match freq {
        2822400 | 3072000 => refclk = CS530X_REFCLK_2P822_3P072,
        5644800 | 6144000 => refclk = CS530X_REFCLK_5P6448_6P144,
        11289600 | 12288000 => refclk = CS530X_REFCLK_11P2896_12P288,
        22579200 | 24576000 => refclk = CS530X_REFCLK_24P5792_24P576,
        _ => {
            dev_err((*component).dev, c"Invalid PLL refclk %d\n".as_ptr(), freq);
            return -EINVAL;
        }
    }

    regmap_update_bits(regmap, CS530X_CLK_CFG_0, CS530X_PLL_REFCLK_FREQ_MASK, refclk)
}

unsafe fn cs530x_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component = (*dai).component;
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;
    let mut ret: core::ffi::c_int = 0;
    let fs: core::ffi::c_int = params_rate(params);
    let bclk: core::ffi::c_int;
    let fs_val: core::ffi::c_uint;

    match fs {
        32000 => fs_val = CS530X_FS_32K,
        44100 | 48000 => fs_val = CS530X_FS_44P1K_48K,
        88200 | 96000 => fs_val = CS530X_FS_88P2K_96K,
        176400 | 192000 => fs_val = CS530X_FS_176P4K_192K,
        356800 | 384000 => fs_val = CS530X_FS_356P8K_384K,
        705600 | 768000 => fs_val = CS530X_FS_705P6K_768K,
        _ => {
            dev_err((*component).dev, c"Invalid sample rate %d\n".as_ptr(), fs);
            return -EINVAL;
        }
    }

    regmap_update_bits(regmap, CS530X_CLK_CFG_1, CS530X_SAMPLE_RATE_MASK, fs_val);

    if regmap_test_bits(regmap, CS530X_SIGNAL_PATH_CFG, CS530X_TDM_EN_MASK) {
        dev_dbg((*component).dev, c"Configuring for %d %d bit TDM slots\n".as_ptr(), (*cs530x).tdm_slots, (*cs530x).tdm_width);
        bclk = snd_soc_tdm_params_to_bclk(params, (*cs530x).tdm_width, (*cs530x).tdm_slots, 1);
    } else {
        bclk = snd_soc_params_to_bclk(params);
    }

    if !regmap_test_bits(regmap, CS530X_CLK_CFG_0, CS530X_PLL_REFCLK_SRC_MASK) {
        ret = cs530x_set_pll_refclk(component, bclk as core::ffi::c_uint);
        if ret != 0 {
            return ret;
        }
    }

    cs530x_set_bclk(component, bclk)
}

unsafe fn cs530x_set_fmt(dai: *mut snd_soc_dai, fmt: core::ffi::c_uint) -> core::ffi::c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*priv_).regmap;
    let asp_fmt: core::ffi::c_uint;
    let mut asp_cfg: core::ffi::c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP => asp_cfg = CS530X_ASP_PRIMARY,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => asp_fmt = CS530X_ASP_FMT_DSP_A,
        SND_SOC_DAIFMT_I2S => asp_fmt = CS530X_ASP_FMT_I2S,
        SND_SOC_DAIFMT_LEFT_J => asp_fmt = CS530X_ASP_FMT_LJ,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => asp_cfg |= CS530X_ASP_BCLK_INV,
        _ => return -EINVAL,
    }

    regmap_update_bits(regmap, CS530X_ASP_CFG, CS530X_ASP_PRIMARY | CS530X_ASP_BCLK_INV, asp_cfg);
    regmap_update_bits(regmap, CS530X_SIGNAL_PATH_CFG, CS530X_ASP_FMT_MASK, asp_fmt)
}

unsafe fn cs530x_check_mclk_freq(component: *mut snd_soc_component, freq: core::ffi::c_uint) -> bool {
    match freq {
        24576000 | 22579200 | 12288000 | 11289600 => true,
        _ => {
            dev_err((*component).dev, c"Invalid MCLK %d\n".as_ptr(), freq);
            false
        }
    }
}

unsafe fn cs530x_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: core::ffi::c_uint,
    _rx_mask: core::ffi::c_uint,
    slots: core::ffi::c_int,
    slot_width: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = (*dai).component;
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;
    let val: core::ffi::c_uint;

    match tx_mask {
        CS530X_0_1_TDM_SLOT_MASK | CS530X_0_3_TDM_SLOT_MASK | CS530X_0_7_TDM_SLOT_MASK => val = CS530X_0_7_TDM_SLOT_VAL,
        CS530X_2_3_TDM_SLOT_MASK => val = CS530X_2_3_TDM_SLOT_VAL,
        CS530X_4_5_TDM_SLOT_MASK | CS530X_4_7_TDM_SLOT_MASK => val = CS530X_4_7_TDM_SLOT_VAL,
        CS530X_6_7_TDM_SLOT_MASK => val = CS530X_6_7_TDM_SLOT_VAL,
        CS530X_8_9_TDM_SLOT_MASK | CS530X_8_11_TDM_SLOT_MASK | CS530X_8_15_TDM_SLOT_MASK => val = CS530X_8_15_TDM_SLOT_VAL,
        CS530X_10_11_TDM_SLOT_MASK => val = CS530X_10_11_TDM_SLOT_VAL,
        CS530X_12_13_TDM_SLOT_MASK | CS530X_12_15_TDM_SLOT_MASK => val = CS530X_12_15_TDM_SLOT_VAL,
        CS530X_14_15_TDM_SLOT_MASK => val = CS530X_14_15_TDM_SLOT_VAL,
        _ => {
            dev_err((*component).dev, c"Invalid TX slot(s) 0x%x\n".as_ptr(), tx_mask);
            return -EINVAL;
        }
    }

    (*cs530x).tdm_width = slot_width;
    (*cs530x).tdm_slots = slots;

    regmap_update_bits(regmap, CS530X_SIGNAL_PATH_CFG, CS530X_ASP_TDM_SLOT_MASK, val << CS530X_ASP_TDM_SLOT_SHIFT)
}

static cs530x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs530x_set_fmt),
    hw_params: Some(cs530x_hw_params),
    set_tdm_slot: Some(cs530x_set_tdm_slot),
};

static cs530x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"cs530x-dai".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"AIF Capture".as_ptr(),
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
        ..unsafe { core::mem::zeroed() }
    },
    playback: snd_soc_pcm_stream {
        stream_name: c"AIF Playback".as_ptr(),
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &cs530x_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn cs530x_set_pll(
    component: *mut snd_soc_component,
    _pll_id: core::ffi::c_int,
    source: core::ffi::c_int,
    freq_in: core::ffi::c_uint,
    _freq_out: core::ffi::c_uint,
) -> core::ffi::c_int {
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;
    let mut sysclk_src: core::ffi::c_uint = 0;
    let ret: core::ffi::c_int;

    regmap_read(regmap, CS530X_CLK_CFG_0, &mut sysclk_src);

    /* Check if the source is the PLL  */
    if (sysclk_src & CS530X_SYSCLK_SRC_MASK) == 0 {
        return 0;
    }

    match source {
        CS530X_PLL_SRC_MCLK => {
            if !cs530x_check_mclk_freq(component, freq_in) {
                return -EINVAL;
            }
            ret = cs530x_set_pll_refclk(component, freq_in);
            if ret != 0 {
                return ret;
            }
        }
        CS530X_PLL_SRC_BCLK => {}
        _ => {
            dev_err((*component).dev, c"Invalid PLL source %d\n".as_ptr(), source);
            return -EINVAL;
        }
    }

    regmap_update_bits(regmap, CS530X_CLK_CFG_0, CS530X_PLL_REFCLK_SRC_MASK, source as core::ffi::c_uint)
}

unsafe fn cs530x_component_probe(component: *mut snd_soc_component) -> core::ffi::c_int {
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut num_widgets: core::ffi::c_int;

    snd_soc_dapm_new_controls(dapm, cs530x_gen_dapm_widgets.as_ptr(), cs530x_gen_dapm_widgets.len() as core::ffi::c_int);

    match (*cs530x).devtype {
        CS4282 => {
            cs530x_add_12_adc_widgets(component);
            cs530x_add_12_dac_widgets(component);
        }
        CS4302 => {
            cs530x_add_12_dac_widgets(component);
        }
        CS4304 => {
            cs530x_add_12_dac_widgets(component);
            cs530x_add_34_dac_widgets(component);
            num_widgets = cs530x_out_sum_4ch_controls.len() as core::ffi::c_int;
            snd_soc_add_component_controls(component, cs530x_out_sum_4ch_controls.as_ptr(), num_widgets as core::ffi::c_uint);
        }
        CS4308 => {
            cs530x_add_12_dac_widgets(component);
            cs530x_add_34_dac_widgets(component);
            num_widgets = cs530x_out_5_to_8_controls.len() as core::ffi::c_int;
            snd_soc_add_component_controls(component, cs530x_out_5_to_8_controls.as_ptr(), num_widgets as core::ffi::c_uint);
            num_widgets = cs530x_out_sum_8ch_controls.len() as core::ffi::c_int;
            snd_soc_add_component_controls(component, cs530x_out_sum_8ch_controls.as_ptr(), num_widgets as core::ffi::c_uint);
            num_widgets = cs530x_dac_ch58_dapm_widgets.len() as core::ffi::c_int;
            snd_soc_dapm_new_controls(dapm, cs530x_dac_ch58_dapm_widgets.as_ptr(), num_widgets);
            snd_soc_dapm_add_routes(dapm, dac_ch5_8_routes.as_ptr(), dac_ch5_8_routes.len() as core::ffi::c_int);
        }
        CS5302 => {
            cs530x_add_12_adc_widgets(component);
        }
        CS5304 => {
            cs530x_add_12_adc_widgets(component);
            cs530x_add_34_adc_widgets(component);
            num_widgets = cs530x_in_sum_4ch_controls.len() as core::ffi::c_int;
            snd_soc_add_component_controls(component, cs530x_in_sum_4ch_controls.as_ptr(), num_widgets as core::ffi::c_uint);
        }
        CS5308 => {
            cs530x_add_12_adc_widgets(component);
            cs530x_add_34_adc_widgets(component);
            num_widgets = cs530x_in_5_to_8_controls.len() as core::ffi::c_int;
            snd_soc_add_component_controls(component, cs530x_in_5_to_8_controls.as_ptr(), num_widgets as core::ffi::c_uint);
            num_widgets = cs530x_in_sum_8ch_controls.len() as core::ffi::c_int;
            snd_soc_add_component_controls(component, cs530x_in_sum_8ch_controls.as_ptr(), num_widgets as core::ffi::c_uint);
            num_widgets = cs530x_adc_ch58_dapm_widgets.len() as core::ffi::c_int;
            snd_soc_dapm_new_controls(dapm, cs530x_adc_ch58_dapm_widgets.as_ptr(), num_widgets);
            snd_soc_dapm_add_routes(dapm, adc_ch5_8_routes.as_ptr(), adc_ch5_8_routes.len() as core::ffi::c_int);
        }
        _ => {
            dev_err((*component).dev, c"Invalid device type %d\n".as_ptr(), (*cs530x).devtype);
            return -EINVAL;
        }
    }

    0
}

unsafe fn cs530x_mclk_freq_is_valid(cs530x: *mut cs530x_priv, freq: core::ffi::c_uint) -> bool {
    /*
     * All these chips support 48 kHz- and 44.1 kHz-related sample rates,
     * but they differ in what MCLK frequency is required for achieving
     * the sample rate.
     */
    match (*cs530x).devtype {
        CS4282 | CS4302 | CS4304 | CS4308 => freq == 49152000 || freq == 45158400,
        CS5302 | CS5304 | CS5308 => freq == 24576000 || freq == 22579200,
        _ => false,
    }
}

unsafe fn cs530x_set_sysclk(
    component: *mut snd_soc_component,
    _clk_id: core::ffi::c_int,
    source: core::ffi::c_int,
    freq: core::ffi::c_uint,
    _dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let cs530x = snd_soc_component_get_drvdata(component) as *mut cs530x_priv;
    let regmap = (*cs530x).regmap;

    match source {
        CS530X_SYSCLK_SRC_MCLK => {
            if !cs530x_mclk_freq_is_valid(cs530x, freq) {
                dev_err((*component).dev, c"Invalid MCLK source rate %d\n".as_ptr(), freq);
                return -EINVAL;
            }
        }
        CS530X_SYSCLK_SRC_PLL => {}
        _ => {
            dev_err((*component).dev, c"Invalid sysclk source: %d\n".as_ptr(), source);
            return -EINVAL;
        }
    }

    regmap_update_bits(regmap, CS530X_CLK_CFG_0, CS530X_SYSCLK_SRC_MASK, (source as core::ffi::c_uint) << CS530X_SYSCLK_SRC_SHIFT)
}

static soc_component_dev_cs530x: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs530x_component_probe),
    set_sysclk: Some(cs530x_set_sysclk),
    set_pll: Some(cs530x_set_pll),
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

pub static cs530x_regmap_i2c: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 16,
    max_register: CS530X_MAX_REGISTER,
    readable_reg: Some(cs530x_readable_register),
    writeable_reg: Some(cs530x_writeable_register),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: cs530x_reg_defaults.as_ptr(),
    num_reg_defaults: cs530x_reg_defaults.len() as core::ffi::c_uint,
    ..unsafe { core::mem::zeroed() }
};
EXPORT_SYMBOL_NS_GPL!(cs530x_regmap_i2c, c"SND_SOC_CS530X".as_ptr());

pub static cs530x_regmap_spi: regmap_config = regmap_config {
    reg_bits: 16,
    pad_bits: 16,
    val_bits: 16,
    reg_stride: 2,
    reg_format_endian: REGMAP_ENDIAN_BIG,
    val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS530X_MAX_REGISTER,
    writeable_reg: Some(cs530x_writeable_register),
    readable_reg: Some(cs530x_readable_register),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: cs530x_reg_defaults.as_ptr(),
    num_reg_defaults: cs530x_reg_defaults.len() as core::ffi::c_uint,
    ..unsafe { core::mem::zeroed() }
};
EXPORT_SYMBOL_NS_GPL!(cs530x_regmap_spi, c"SND_SOC_CS530X".as_ptr());

unsafe fn cs530x_check_device_id(cs530x: *mut cs530x_priv) -> core::ffi::c_int {
    let dev = (*cs530x).dev;
    let mut dev_id: core::ffi::c_uint = 0;
    let mut rev: core::ffi::c_uint = 0;
    let mut ret: core::ffi::c_int;

    ret = regmap_read((*cs530x).regmap, CS530X_DEVID, &mut dev_id);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Can't read device ID\n".as_ptr());
    }

    ret = regmap_read((*cs530x).regmap, CS530X_REVID, &mut rev);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Can't read REV ID\n".as_ptr());
    }

    match dev_id {
        CS530X_2CH_CODEC_DEV_ID => {
            (*cs530x).num_dacs = 2;
            (*cs530x).num_adcs = 2;
        }
        CS530X_2CH_DAC_DEV_ID => (*cs530x).num_dacs = 2,
        CS530X_4CH_DAC_DEV_ID => (*cs530x).num_dacs = 4,
        CS530X_8CH_DAC_DEV_ID => (*cs530x).num_dacs = 8,
        CS530X_2CH_ADC_DEV_ID => (*cs530x).num_adcs = 2,
        CS530X_4CH_ADC_DEV_ID => (*cs530x).num_adcs = 4,
        CS530X_8CH_ADC_DEV_ID => (*cs530x).num_adcs = 8,
        _ => return dev_err_probe(dev, -EINVAL, c"Invalid device ID 0x%x\n".as_ptr(), dev_id),
    }

    if (*cs530x).devtype != dev_id {
        dev_err(dev, c"Read device ID 0x%x is not the expected devtype 0x%x\n".as_ptr(), dev_id, (*cs530x).devtype);
        return -EINVAL;
    }

    dev_dbg(dev, c"Device ID 0x%x Rev ID 0x%x (%d in %d out)\n".as_ptr(), dev_id, rev, (*cs530x).num_adcs, (*cs530x).num_dacs);

    0
}

unsafe fn cs530x_parse_device_properties(cs530x: *mut cs530x_priv) -> core::ffi::c_int {
    let regmap = (*cs530x).regmap;
    let dev = (*cs530x).dev;
    let mut val: core::ffi::c_uint = 0;

    match (*cs530x).num_adcs {
        8 => {
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin78".as_ptr()) {
                val = CS530X_IN78_HIZ;
            }
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin56".as_ptr()) {
                val |= CS530X_IN56_HIZ;
            }
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin34".as_ptr()) {
                val |= CS530X_IN34_HIZ;
            }
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin12".as_ptr()) {
                val |= CS530X_IN12_HIZ;
            }
            regmap_set_bits(regmap, CS530X_IN_HIZ, val)
        }
        4 => {
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin34".as_ptr()) {
                val |= CS530X_IN34_HIZ;
            }
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin12".as_ptr()) {
                val |= CS530X_IN12_HIZ;
            }
            regmap_set_bits(regmap, CS530X_IN_HIZ, val)
        }
        2 => {
            if device_property_read_bool(dev, c"cirrus,in-hiz-pin12".as_ptr()) {
                val |= CS530X_IN12_HIZ;
            }
            regmap_set_bits(regmap, CS530X_IN_HIZ, val)
        }
        0 => {
            /* No ADCs */
            0
        }
        _ => dev_err_probe(dev, -EINVAL, c"Invalid number of adcs %d\n".as_ptr(), (*cs530x).num_adcs),
    }
}

pub unsafe fn cs530x_probe(cs530x: *mut cs530x_priv) -> core::ffi::c_int {
    let dev = (*cs530x).dev;
    let mut ret: core::ffi::c_int;
    let mut i: core::ffi::c_int;

    (*cs530x).dev_dai = devm_kmemdup(dev, &cs530x_dai as *const _ as *const core::ffi::c_void, core::mem::size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if (*cs530x).dev_dai.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as usize) < (*cs530x).supplies.len() {
        (*cs530x).supplies[i as usize].supply = cs530x_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*cs530x).supplies.len() as core::ffi::c_int, (*cs530x).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to request supplies".as_ptr());
    }

    ret = regulator_bulk_enable((*cs530x).supplies.len() as core::ffi::c_int, (*cs530x).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to enable supplies".as_ptr());
    }

    (*cs530x).reset_gpio = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*cs530x).reset_gpio) {
        ret = dev_err_probe(dev, PTR_ERR((*cs530x).reset_gpio) as core::ffi::c_int, c"Reset gpio not available\n".as_ptr());
        goto_err_regulator(cs530x, ret)
    } else {
        if !(*cs530x).reset_gpio.is_null() {
            usleep_range(2000, 2100);
            gpiod_set_value_cansleep((*cs530x).reset_gpio, 0);
        }

        usleep_range(5000, 5100);
        ret = cs530x_check_device_id(cs530x);
        if ret != 0 {
            return goto_err_reset(cs530x, ret);
        }

        if (*cs530x).reset_gpio.is_null() {
            ret = regmap_write((*cs530x).regmap, CS530X_SW_RESET, CS530X_SW_RST_VAL);
            if ret != 0 {
                dev_err_probe(dev, ret, c"Soft Reset Failed\n".as_ptr());
                return goto_err_reset(cs530x, ret);
            }
        }

        ret = cs530x_parse_device_properties(cs530x);
        if ret != 0 {
            return goto_err_reset(cs530x, ret);
        }

        if (*cs530x).num_adcs != 0 {
            (*(*cs530x).dev_dai).capture.channels_min = 2;
            (*(*cs530x).dev_dai).capture.channels_max = (*cs530x).num_adcs;
        }

        if (*cs530x).num_dacs != 0 {
            (*(*cs530x).dev_dai).playback.channels_min = 2;
            (*(*cs530x).dev_dai).playback.channels_max = (*cs530x).num_dacs;
        }

        ret = devm_snd_soc_register_component(dev, &soc_component_dev_cs530x, (*cs530x).dev_dai, 1);
        if ret != 0 {
            dev_err_probe(dev, ret, c"Can't register cs530x component\n".as_ptr());
            return goto_err_reset(cs530x, ret);
        }

        0
    }
}

unsafe fn goto_err_reset(cs530x: *mut cs530x_priv, ret: core::ffi::c_int) -> core::ffi::c_int {
    gpiod_set_value_cansleep((*cs530x).reset_gpio, 1);
    goto_err_regulator(cs530x, ret)
}

unsafe fn goto_err_regulator(cs530x: *mut cs530x_priv, ret: core::ffi::c_int) -> core::ffi::c_int {
    regulator_bulk_disable((*cs530x).supplies.len() as core::ffi::c_int, (*cs530x).supplies.as_mut_ptr());
    ret
}

EXPORT_SYMBOL_NS_GPL!(cs530x_probe, c"SND_SOC_CS530X".as_ptr());

MODULE_DESCRIPTION!(c"CS530X CODEC Driver".as_ptr());
MODULE_AUTHOR!(c"Paul Handrigan <paulha@opensource.cirrus.com>".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
