// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip RK3308 internal audio codec driver
 *
 * Copyright (c) 2018, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.
 * Copyright (c) 2024, Vivax-Metrotech Ltd
 */

/* C dependencies translated as external Rust dependencies:
 * linux/clk.h, linux/device.h, linux/delay.h, linux/init.h, linux/io.h,
 * linux/mfd/syscon.h, linux/module.h, linux/of.h, linux/platform_device.h,
 * linux/regmap.h, linux/reset.h, linux/util_macros.h, sound/core.h,
 * sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h,
 * and "rk3308_codec.h".
 */

const ADC_LR_GROUP_MAX: i32 = 4;
const GRF_CHIP_ID: u32 = 0x800;

const ACODEC_VERSION_A: u8 = b'A';
const ACODEC_VERSION_B: u8 = ACODEC_VERSION_A + 1;
const ACODEC_VERSION_C: u8 = ACODEC_VERSION_B + 1;

#[repr(C)]
pub struct rk3308_codec_priv {
    dev: *const device,
    regmap: *mut regmap,
    grf: *mut regmap,
    reset: *mut reset_control,
    hclk: *mut clk,
    mclk_rx: *mut clk,
    mclk_tx: *mut clk,
    component: *mut snd_soc_component,
    codec_ver: u8,
}

static mut rk3308_codec_clocks: [clk_bulk_data; 3] = [
    clk_bulk_data { id: c_str!("hclk") },
    clk_bulk_data { id: c_str!("mclk_rx") },
    clk_bulk_data { id: c_str!("mclk_tx") },
];

static rk3308_codec_adc_alc_gain_tlv: _ = DECLARE_TLV_DB_SCALE!(-1800, 150, 0);
static rk3308_codec_dac_hpout_gain_tlv: _ = DECLARE_TLV_DB_SCALE!(-3900, 150, 0);
static rk3308_codec_dac_hpmix_gain_tlv: _ = DECLARE_TLV_DB_SCALE!(-600, 600, 0);

static rk3308_codec_dac_lineout_gain_tlv: _ = DECLARE_TLV_DB_RANGE!(
    0, 0, TLV_DB_SCALE_ITEM!(-600, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM!(-300, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM!(-150, 0, 0),
    3, 3, TLV_DB_SCALE_ITEM!(0, 0, 0),
);

static rk3308_codec_hpf_cutoff_text: [*const c_char; 3] = [
    c_str!("20 Hz"),
    c_str!("245 Hz"),
    c_str!("612 Hz"),
];

static rk3308_codec_hpf_cutoff_enum12: _ =
    SOC_ENUM_SINGLE_DECL!(RK3308_ADC_DIG_CON04(0), 0, rk3308_codec_hpf_cutoff_text);
static rk3308_codec_hpf_cutoff_enum34: _ =
    SOC_ENUM_SINGLE_DECL!(RK3308_ADC_DIG_CON04(1), 0, rk3308_codec_hpf_cutoff_text);
static rk3308_codec_hpf_cutoff_enum56: _ =
    SOC_ENUM_SINGLE_DECL!(RK3308_ADC_DIG_CON04(2), 0, rk3308_codec_hpf_cutoff_text);
static rk3308_codec_hpf_cutoff_enum78: _ =
    SOC_ENUM_SINGLE_DECL!(RK3308_ADC_DIG_CON04(3), 0, rk3308_codec_hpf_cutoff_text);

static rk3308_codec_controls: &[snd_kcontrol_new] = &[
    /* Despite the register names, these set the gain when AGC is OFF */
    SOC_SINGLE_RANGE_TLV!("MIC1 Capture Volume", RK3308_ADC_ANA_CON03(0), RK3308_ADC_CH1_ALC_GAIN_SFT, RK3308_ADC_CH1_ALC_GAIN_MIN, RK3308_ADC_CH1_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC2 Capture Volume", RK3308_ADC_ANA_CON04(0), RK3308_ADC_CH2_ALC_GAIN_SFT, RK3308_ADC_CH2_ALC_GAIN_MIN, RK3308_ADC_CH2_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC3 Capture Volume", RK3308_ADC_ANA_CON03(1), RK3308_ADC_CH1_ALC_GAIN_SFT, RK3308_ADC_CH1_ALC_GAIN_MIN, RK3308_ADC_CH1_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC4 Capture Volume", RK3308_ADC_ANA_CON04(1), RK3308_ADC_CH2_ALC_GAIN_SFT, RK3308_ADC_CH2_ALC_GAIN_MIN, RK3308_ADC_CH2_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC5 Capture Volume", RK3308_ADC_ANA_CON03(2), RK3308_ADC_CH1_ALC_GAIN_SFT, RK3308_ADC_CH1_ALC_GAIN_MIN, RK3308_ADC_CH1_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC6 Capture Volume", RK3308_ADC_ANA_CON04(2), RK3308_ADC_CH2_ALC_GAIN_SFT, RK3308_ADC_CH2_ALC_GAIN_MIN, RK3308_ADC_CH2_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC7 Capture Volume", RK3308_ADC_ANA_CON03(3), RK3308_ADC_CH1_ALC_GAIN_SFT, RK3308_ADC_CH1_ALC_GAIN_MIN, RK3308_ADC_CH1_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("MIC8 Capture Volume", RK3308_ADC_ANA_CON04(3), RK3308_ADC_CH2_ALC_GAIN_SFT, RK3308_ADC_CH2_ALC_GAIN_MIN, RK3308_ADC_CH2_ALC_GAIN_MAX, 0, rk3308_codec_adc_alc_gain_tlv),
    SOC_SINGLE!("MIC1 Capture Switch", RK3308_ADC_ANA_CON00(0), 3, 1, 0),
    SOC_SINGLE!("MIC2 Capture Switch", RK3308_ADC_ANA_CON00(0), 7, 1, 0),
    SOC_SINGLE!("MIC3 Capture Switch", RK3308_ADC_ANA_CON00(1), 3, 1, 0),
    SOC_SINGLE!("MIC4 Capture Switch", RK3308_ADC_ANA_CON00(1), 7, 1, 0),
    SOC_SINGLE!("MIC5 Capture Switch", RK3308_ADC_ANA_CON00(2), 3, 1, 0),
    SOC_SINGLE!("MIC6 Capture Switch", RK3308_ADC_ANA_CON00(2), 7, 1, 0),
    SOC_SINGLE!("MIC7 Capture Switch", RK3308_ADC_ANA_CON00(3), 3, 1, 0),
    SOC_SINGLE!("MIC8 Capture Switch", RK3308_ADC_ANA_CON00(3), 7, 1, 0),
    SOC_SINGLE!("MIC12 HPF Capture Switch", RK3308_ADC_DIG_CON04(0), 2, 1, 1),
    SOC_SINGLE!("MIC34 HPF Capture Switch", RK3308_ADC_DIG_CON04(1), 2, 1, 1),
    SOC_SINGLE!("MIC56 HPF Capture Switch", RK3308_ADC_DIG_CON04(2), 2, 1, 1),
    SOC_SINGLE!("MIC78 HPF Capture Switch", RK3308_ADC_DIG_CON04(3), 2, 1, 1),
    SOC_ENUM!("MIC12 HPF Cutoff", rk3308_codec_hpf_cutoff_enum12),
    SOC_ENUM!("MIC34 HPF Cutoff", rk3308_codec_hpf_cutoff_enum34),
    SOC_ENUM!("MIC56 HPF Cutoff", rk3308_codec_hpf_cutoff_enum56),
    SOC_ENUM!("MIC78 HPF Cutoff", rk3308_codec_hpf_cutoff_enum78),
    SOC_DOUBLE_TLV!("Line Out Playback Volume", RK3308_DAC_ANA_CON04, RK3308_DAC_L_LINEOUT_GAIN_SFT, RK3308_DAC_R_LINEOUT_GAIN_SFT, RK3308_DAC_x_LINEOUT_GAIN_MAX, 0, rk3308_codec_dac_lineout_gain_tlv),
    SOC_DOUBLE!("Line Out Playback Switch", RK3308_DAC_ANA_CON04, RK3308_DAC_L_LINEOUT_MUTE_SFT, RK3308_DAC_R_LINEOUT_MUTE_SFT, 1, 0),
    SOC_DOUBLE_R_TLV!("Headphone Playback Volume", RK3308_DAC_ANA_CON05, RK3308_DAC_ANA_CON06, RK3308_DAC_x_HPOUT_GAIN_SFT, RK3308_DAC_x_HPOUT_GAIN_MAX, 0, rk3308_codec_dac_hpout_gain_tlv),
    SOC_DOUBLE!("Headphone Playback Switch", RK3308_DAC_ANA_CON03, RK3308_DAC_L_HPOUT_MUTE_SFT, RK3308_DAC_R_HPOUT_MUTE_SFT, 1, 0),
    SOC_DOUBLE_RANGE_TLV!("DAC HPMIX Playback Volume", RK3308_DAC_ANA_CON12, RK3308_DAC_L_HPMIX_GAIN_SFT, RK3308_DAC_R_HPMIX_GAIN_SFT, 1, 2, 0, rk3308_codec_dac_hpmix_gain_tlv),
];

unsafe extern "C" fn rk3308_codec_pop_sound_set(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let rk3308 = snd_soc_component_get_drvdata(component) as *mut rk3308_codec_priv;
    let val: c_uint = if event == SND_SOC_DAPM_POST_PMU {
        RK3308_DAC_HPOUT_POP_SOUND_x_WORK
    } else {
        RK3308_DAC_HPOUT_POP_SOUND_x_INIT
    };
    let mask: c_uint = RK3308_DAC_HPOUT_POP_SOUND_x_MSK;

    regmap_update_bits(
        (*rk3308).regmap,
        RK3308_DAC_ANA_CON01,
        mask << (*w).shift,
        val << (*w).shift,
    );

    0
}

static rk3308_codec_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_INPUT!("MIC1"), SND_SOC_DAPM_INPUT!("MIC2"),
    SND_SOC_DAPM_INPUT!("MIC3"), SND_SOC_DAPM_INPUT!("MIC4"),
    SND_SOC_DAPM_INPUT!("MIC5"), SND_SOC_DAPM_INPUT!("MIC6"),
    SND_SOC_DAPM_INPUT!("MIC7"), SND_SOC_DAPM_INPUT!("MIC8"),
    SND_SOC_DAPM_SUPPLY!("ADC_CURRENT_EN12", RK3308_ADC_ANA_CON06(0), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC_CURRENT_EN34", RK3308_ADC_ANA_CON06(1), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC_CURRENT_EN56", RK3308_ADC_ANA_CON06(2), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC_CURRENT_EN78", RK3308_ADC_ANA_CON06(3), 0, 0, NULL, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC1_EN", RK3308_ADC_ANA_CON00(0), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC2_EN", RK3308_ADC_ANA_CON00(0), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC3_EN", RK3308_ADC_ANA_CON00(1), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC4_EN", RK3308_ADC_ANA_CON00(1), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC5_EN", RK3308_ADC_ANA_CON00(2), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC6_EN", RK3308_ADC_ANA_CON00(2), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC7_EN", RK3308_ADC_ANA_CON00(3), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC8_EN", RK3308_ADC_ANA_CON00(3), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC1_WORK", RK3308_ADC_ANA_CON00(0), 2, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC2_WORK", RK3308_ADC_ANA_CON00(0), 6, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC3_WORK", RK3308_ADC_ANA_CON00(1), 2, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC4_WORK", RK3308_ADC_ANA_CON00(1), 6, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC5_WORK", RK3308_ADC_ANA_CON00(2), 2, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC6_WORK", RK3308_ADC_ANA_CON00(2), 6, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC7_WORK", RK3308_ADC_ANA_CON00(3), 2, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_mic, "MIC8_WORK", RK3308_ADC_ANA_CON00(3), 6, 1, 1, 0),
    /*
     * In theory MIC1 and MIC2 can switch to LINE IN, but this is not
     * supported so all we can do is enabling the MIC input.
     */
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "CH1_IN_SEL", RK3308_ADC_ANA_CON07(0), 4, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "CH2_IN_SEL", RK3308_ADC_ANA_CON07(0), 6, 1, 1, 0),
    SND_SOC_DAPM_SUPPLY!("ADC1_BUF_REF_EN", RK3308_ADC_ANA_CON00(0), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC2_BUF_REF_EN", RK3308_ADC_ANA_CON00(0), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC3_BUF_REF_EN", RK3308_ADC_ANA_CON00(1), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC4_BUF_REF_EN", RK3308_ADC_ANA_CON00(1), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC5_BUF_REF_EN", RK3308_ADC_ANA_CON00(2), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC6_BUF_REF_EN", RK3308_ADC_ANA_CON00(2), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC7_BUF_REF_EN", RK3308_ADC_ANA_CON00(3), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC8_BUF_REF_EN", RK3308_ADC_ANA_CON00(3), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC_MCLK_GATE", RK3308_GLB_CON, 5, 1, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC1_CLK_EN", RK3308_ADC_ANA_CON05(0), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC2_CLK_EN", RK3308_ADC_ANA_CON05(0), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC3_CLK_EN", RK3308_ADC_ANA_CON05(1), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC4_CLK_EN", RK3308_ADC_ANA_CON05(1), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC5_CLK_EN", RK3308_ADC_ANA_CON05(2), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC6_CLK_EN", RK3308_ADC_ANA_CON05(2), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC7_CLK_EN", RK3308_ADC_ANA_CON05(3), 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("ADC8_CLK_EN", RK3308_ADC_ANA_CON05(3), 4, 0, NULL, 0),
    /* The "ALC" name from the TRM is misleading, these are needed even without ALC/AGC */
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC1_EN", RK3308_ADC_ANA_CON02(0), 0, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC2_EN", RK3308_ADC_ANA_CON02(0), 4, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC3_EN", RK3308_ADC_ANA_CON02(1), 0, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC4_EN", RK3308_ADC_ANA_CON02(1), 4, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC5_EN", RK3308_ADC_ANA_CON02(2), 0, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC6_EN", RK3308_ADC_ANA_CON02(2), 4, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC7_EN", RK3308_ADC_ANA_CON02(3), 0, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC8_EN", RK3308_ADC_ANA_CON02(3), 4, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC1_EN", RK3308_ADC_ANA_CON05(0), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC2_EN", RK3308_ADC_ANA_CON05(0), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC3_EN", RK3308_ADC_ANA_CON05(1), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC4_EN", RK3308_ADC_ANA_CON05(1), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC5_EN", RK3308_ADC_ANA_CON05(2), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC6_EN", RK3308_ADC_ANA_CON05(2), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC7_EN", RK3308_ADC_ANA_CON05(3), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ADC8_EN", RK3308_ADC_ANA_CON05(3), 5, 1, 1, 0),
    SND_SOC_DAPM_ADC!("ADC1_WORK", "Capture", RK3308_ADC_ANA_CON05(0), 2, 0),
    SND_SOC_DAPM_ADC!("ADC2_WORK", "Capture", RK3308_ADC_ANA_CON05(0), 6, 0),
    SND_SOC_DAPM_ADC!("ADC3_WORK", "Capture", RK3308_ADC_ANA_CON05(1), 2, 0),
    SND_SOC_DAPM_ADC!("ADC4_WORK", "Capture", RK3308_ADC_ANA_CON05(1), 6, 0),
    SND_SOC_DAPM_ADC!("ADC5_WORK", "Capture", RK3308_ADC_ANA_CON05(2), 2, 0),
    SND_SOC_DAPM_ADC!("ADC6_WORK", "Capture", RK3308_ADC_ANA_CON05(2), 6, 0),
    SND_SOC_DAPM_ADC!("ADC7_WORK", "Capture", RK3308_ADC_ANA_CON05(3), 2, 0),
    SND_SOC_DAPM_ADC!("ADC8_WORK", "Capture", RK3308_ADC_ANA_CON05(3), 6, 0),
    /* The "ALC" name from the TRM is misleading, these are needed even without ALC/AGC */
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC1_WORK", RK3308_ADC_ANA_CON02(0), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC2_WORK", RK3308_ADC_ANA_CON02(0), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC3_WORK", RK3308_ADC_ANA_CON02(1), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC4_WORK", RK3308_ADC_ANA_CON02(1), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC5_WORK", RK3308_ADC_ANA_CON02(2), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC6_WORK", RK3308_ADC_ANA_CON02(2), 5, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC7_WORK", RK3308_ADC_ANA_CON02(3), 1, 1, 1, 0),
    SND_SOC_DAPM_REG!(snd_soc_dapm_adc, "ALC8_WORK", RK3308_ADC_ANA_CON02(3), 5, 1, 1, 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS Current", RK3308_ADC_ANA_CON08(0), 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS1", RK3308_ADC_ANA_CON07(1), 3, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS2", RK3308_ADC_ANA_CON07(2), 3, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("DAC_L_HPMIX_EN", RK3308_DAC_ANA_CON13, 0, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("DAC_R_HPMIX_EN", RK3308_DAC_ANA_CON13, 4, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("DAC_L_HPMIX_WORK", RK3308_DAC_ANA_CON13, 1, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("DAC_R_HPMIX_WORK", RK3308_DAC_ANA_CON13, 5, 0, NULL, 0),
    /* HPMIX is not actually acting as a mixer as the only supported input is I2S */
    SND_SOC_DAPM_OUT_DRV!("DAC_L_HPMIX_SEL", RK3308_DAC_ANA_CON12, 2, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("DAC_R_HPMIX_SEL", RK3308_DAC_ANA_CON12, 6, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("DAC HPMIX Left", RK3308_DAC_ANA_CON13, 2, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("DAC HPMIX Right", RK3308_DAC_ANA_CON13, 6, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_MCLK_GATE", RK3308_GLB_CON, 4, 1, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_CURRENT_EN", RK3308_DAC_ANA_CON00, 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_L_REF_EN", RK3308_DAC_ANA_CON02, 0, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_R_REF_EN", RK3308_DAC_ANA_CON02, 4, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_L_CLK_EN", RK3308_DAC_ANA_CON02, 1, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_R_CLK_EN", RK3308_DAC_ANA_CON02, 5, 0, NULL, 0),
    SND_SOC_DAPM_DAC!("DAC_L_DAC_WORK", NULL, RK3308_DAC_ANA_CON02, 3, 0),
    SND_SOC_DAPM_DAC!("DAC_R_DAC_WORK", NULL, RK3308_DAC_ANA_CON02, 7, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_BUF_REF_L", RK3308_DAC_ANA_CON01, 2, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("DAC_BUF_REF_R", RK3308_DAC_ANA_CON01, 6, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV_E!("HPOUT_POP_SOUND_L", SND_SOC_NOPM, 0, 0, NULL, 0, rk3308_codec_pop_sound_set, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_OUT_DRV_E!("HPOUT_POP_SOUND_R", SND_SOC_NOPM, 4, 0, NULL, 0, rk3308_codec_pop_sound_set, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_OUT_DRV!("L_HPOUT_EN", RK3308_DAC_ANA_CON03, 1, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("R_HPOUT_EN", RK3308_DAC_ANA_CON03, 5, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("L_HPOUT_WORK", RK3308_DAC_ANA_CON03, 2, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("R_HPOUT_WORK", RK3308_DAC_ANA_CON03, 6, 0, NULL, 0),
    SND_SOC_DAPM_OUTPUT!("HPOUT_L"), SND_SOC_DAPM_OUTPUT!("HPOUT_R"),
    SND_SOC_DAPM_OUT_DRV!("L_LINEOUT_EN", RK3308_DAC_ANA_CON04, 0, 0, NULL, 0),
    SND_SOC_DAPM_OUT_DRV!("R_LINEOUT_EN", RK3308_DAC_ANA_CON04, 4, 0, NULL, 0),
    SND_SOC_DAPM_OUTPUT!("LINEOUT_L"), SND_SOC_DAPM_OUTPUT!("LINEOUT_R"),
];

static rk3308_codec_dapm_routes: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: c_str!("MICBIAS1"), control: NULL, source: c_str!("MICBIAS Current") },
    snd_soc_dapm_route { sink: c_str!("MICBIAS2"), control: NULL, source: c_str!("MICBIAS Current") },
    snd_soc_dapm_route { sink: c_str!("MIC1_EN"), control: NULL, source: c_str!("MIC1") },
    snd_soc_dapm_route { sink: c_str!("MIC2_EN"), control: NULL, source: c_str!("MIC2") },
    snd_soc_dapm_route { sink: c_str!("MIC3_EN"), control: NULL, source: c_str!("MIC3") },
    snd_soc_dapm_route { sink: c_str!("MIC4_EN"), control: NULL, source: c_str!("MIC4") },
    snd_soc_dapm_route { sink: c_str!("MIC5_EN"), control: NULL, source: c_str!("MIC5") },
    snd_soc_dapm_route { sink: c_str!("MIC6_EN"), control: NULL, source: c_str!("MIC6") },
    snd_soc_dapm_route { sink: c_str!("MIC7_EN"), control: NULL, source: c_str!("MIC7") },
    snd_soc_dapm_route { sink: c_str!("MIC8_EN"), control: NULL, source: c_str!("MIC8") },
    snd_soc_dapm_route { sink: c_str!("MIC1_WORK"), control: NULL, source: c_str!("MIC1_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC2_WORK"), control: NULL, source: c_str!("MIC2_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC3_WORK"), control: NULL, source: c_str!("MIC3_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC4_WORK"), control: NULL, source: c_str!("MIC4_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC5_WORK"), control: NULL, source: c_str!("MIC5_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC6_WORK"), control: NULL, source: c_str!("MIC6_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC7_WORK"), control: NULL, source: c_str!("MIC7_EN") },
    snd_soc_dapm_route { sink: c_str!("MIC8_WORK"), control: NULL, source: c_str!("MIC8_EN") },
    snd_soc_dapm_route { sink: c_str!("CH1_IN_SEL"), control: NULL, source: c_str!("MIC1_WORK") },
    snd_soc_dapm_route { sink: c_str!("CH2_IN_SEL"), control: NULL, source: c_str!("MIC2_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC1_EN"), control: NULL, source: c_str!("CH1_IN_SEL") },
    snd_soc_dapm_route { sink: c_str!("ALC2_EN"), control: NULL, source: c_str!("CH2_IN_SEL") },
    snd_soc_dapm_route { sink: c_str!("ALC3_EN"), control: NULL, source: c_str!("MIC3_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC4_EN"), control: NULL, source: c_str!("MIC4_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC5_EN"), control: NULL, source: c_str!("MIC5_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC6_EN"), control: NULL, source: c_str!("MIC6_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC7_EN"), control: NULL, source: c_str!("MIC7_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC8_EN"), control: NULL, source: c_str!("MIC8_WORK") },
    snd_soc_dapm_route { sink: c_str!("ADC1_EN"), control: NULL, source: c_str!("ALC1_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC2_EN"), control: NULL, source: c_str!("ALC2_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC3_EN"), control: NULL, source: c_str!("ALC3_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC4_EN"), control: NULL, source: c_str!("ALC4_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC5_EN"), control: NULL, source: c_str!("ALC5_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC6_EN"), control: NULL, source: c_str!("ALC6_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC7_EN"), control: NULL, source: c_str!("ALC7_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC8_EN"), control: NULL, source: c_str!("ALC8_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC1_WORK"), control: NULL, source: c_str!("ADC1_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC2_WORK"), control: NULL, source: c_str!("ADC2_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC3_WORK"), control: NULL, source: c_str!("ADC3_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC4_WORK"), control: NULL, source: c_str!("ADC4_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC5_WORK"), control: NULL, source: c_str!("ADC5_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC6_WORK"), control: NULL, source: c_str!("ADC6_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC7_WORK"), control: NULL, source: c_str!("ADC7_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC8_WORK"), control: NULL, source: c_str!("ADC8_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC1_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN12") },
    snd_soc_dapm_route { sink: c_str!("ADC2_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN12") },
    snd_soc_dapm_route { sink: c_str!("ADC3_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN34") },
    snd_soc_dapm_route { sink: c_str!("ADC4_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN34") },
    snd_soc_dapm_route { sink: c_str!("ADC5_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN56") },
    snd_soc_dapm_route { sink: c_str!("ADC6_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN56") },
    snd_soc_dapm_route { sink: c_str!("ADC7_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN78") },
    snd_soc_dapm_route { sink: c_str!("ADC8_BUF_REF_EN"), control: NULL, source: c_str!("ADC_CURRENT_EN78") },
    snd_soc_dapm_route { sink: c_str!("ADC1_WORK"), control: NULL, source: c_str!("ADC1_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC2_WORK"), control: NULL, source: c_str!("ADC2_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC3_WORK"), control: NULL, source: c_str!("ADC3_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC4_WORK"), control: NULL, source: c_str!("ADC4_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC5_WORK"), control: NULL, source: c_str!("ADC5_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC6_WORK"), control: NULL, source: c_str!("ADC6_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC7_WORK"), control: NULL, source: c_str!("ADC7_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC8_WORK"), control: NULL, source: c_str!("ADC8_BUF_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC1_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC2_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC3_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC4_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC5_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC6_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC7_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC8_CLK_EN"), control: NULL, source: c_str!("ADC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("ADC1_WORK"), control: NULL, source: c_str!("ADC1_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC2_WORK"), control: NULL, source: c_str!("ADC2_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC3_WORK"), control: NULL, source: c_str!("ADC3_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC4_WORK"), control: NULL, source: c_str!("ADC4_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC5_WORK"), control: NULL, source: c_str!("ADC5_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC6_WORK"), control: NULL, source: c_str!("ADC6_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC7_WORK"), control: NULL, source: c_str!("ADC7_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ADC8_WORK"), control: NULL, source: c_str!("ADC8_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("ALC1_WORK"), control: NULL, source: c_str!("ADC1_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC2_WORK"), control: NULL, source: c_str!("ADC2_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC3_WORK"), control: NULL, source: c_str!("ADC3_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC4_WORK"), control: NULL, source: c_str!("ADC4_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC5_WORK"), control: NULL, source: c_str!("ADC5_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC6_WORK"), control: NULL, source: c_str!("ADC6_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC7_WORK"), control: NULL, source: c_str!("ADC7_WORK") },
    snd_soc_dapm_route { sink: c_str!("ALC8_WORK"), control: NULL, source: c_str!("ADC8_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC1_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC2_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC3_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC4_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC5_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC6_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC7_WORK") },
    snd_soc_dapm_route { sink: c_str!("HiFi Capture"), control: NULL, source: c_str!("ALC8_WORK") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_HPMIX_EN"), control: NULL, source: c_str!("HiFi Playback") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_HPMIX_EN"), control: NULL, source: c_str!("HiFi Playback") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_HPMIX_WORK"), control: NULL, source: c_str!("DAC_L_HPMIX_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_HPMIX_WORK"), control: NULL, source: c_str!("DAC_R_HPMIX_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC HPMIX Left"), control: NULL, source: c_str!("DAC_L_HPMIX_WORK") },
    snd_soc_dapm_route { sink: c_str!("DAC HPMIX Right"), control: NULL, source: c_str!("DAC_R_HPMIX_WORK") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_DAC_WORK"), control: NULL, source: c_str!("DAC HPMIX Left") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_DAC_WORK"), control: NULL, source: c_str!("DAC HPMIX Right") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_REF_EN"), control: NULL, source: c_str!("DAC_CURRENT_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_REF_EN"), control: NULL, source: c_str!("DAC_CURRENT_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_CLK_EN"), control: NULL, source: c_str!("DAC_L_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_CLK_EN"), control: NULL, source: c_str!("DAC_R_REF_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_CLK_EN"), control: NULL, source: c_str!("DAC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_CLK_EN"), control: NULL, source: c_str!("DAC_MCLK_GATE") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_DAC_WORK"), control: NULL, source: c_str!("DAC_L_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_DAC_WORK"), control: NULL, source: c_str!("DAC_R_CLK_EN") },
    snd_soc_dapm_route { sink: c_str!("DAC_L_HPMIX_SEL"), control: NULL, source: c_str!("DAC_L_DAC_WORK") },
    snd_soc_dapm_route { sink: c_str!("DAC_R_HPMIX_SEL"), control: NULL, source: c_str!("DAC_R_DAC_WORK") },
    snd_soc_dapm_route { sink: c_str!("HPOUT_L"), control: NULL, source: c_str!("DAC_BUF_REF_L") },
    snd_soc_dapm_route { sink: c_str!("HPOUT_R"), control: NULL, source: c_str!("DAC_BUF_REF_R") },
    snd_soc_dapm_route { sink: c_str!("L_HPOUT_EN"), control: NULL, source: c_str!("DAC_L_HPMIX_SEL") },
    snd_soc_dapm_route { sink: c_str!("R_HPOUT_EN"), control: NULL, source: c_str!("DAC_R_HPMIX_SEL") },
    snd_soc_dapm_route { sink: c_str!("L_HPOUT_WORK"), control: NULL, source: c_str!("L_HPOUT_EN") },
    snd_soc_dapm_route { sink: c_str!("R_HPOUT_WORK"), control: NULL, source: c_str!("R_HPOUT_EN") },
    snd_soc_dapm_route { sink: c_str!("HPOUT_POP_SOUND_L"), control: NULL, source: c_str!("L_HPOUT_WORK") },
    snd_soc_dapm_route { sink: c_str!("HPOUT_POP_SOUND_R"), control: NULL, source: c_str!("R_HPOUT_WORK") },
    snd_soc_dapm_route { sink: c_str!("HPOUT_L"), control: NULL, source: c_str!("HPOUT_POP_SOUND_L") },
    snd_soc_dapm_route { sink: c_str!("HPOUT_R"), control: NULL, source: c_str!("HPOUT_POP_SOUND_R") },
    snd_soc_dapm_route { sink: c_str!("L_LINEOUT_EN"), control: NULL, source: c_str!("DAC_L_HPMIX_SEL") },
    snd_soc_dapm_route { sink: c_str!("R_LINEOUT_EN"), control: NULL, source: c_str!("DAC_R_HPMIX_SEL") },
    snd_soc_dapm_route { sink: c_str!("LINEOUT_L"), control: NULL, source: c_str!("L_LINEOUT_EN") },
    snd_soc_dapm_route { sink: c_str!("LINEOUT_R"), control: NULL, source: c_str!("R_LINEOUT_EN") },
];

unsafe extern "C" fn rk3308_codec_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let rk3308 = snd_soc_component_get_drvdata(component) as *mut rk3308_codec_priv;
    let inv_bits: c_uint = fmt & SND_SOC_DAIFMT_INV_MASK;
    let inv_bitclk: bool =
        (inv_bits & SND_SOC_DAIFMT_IB_IF) != 0 || (inv_bits & SND_SOC_DAIFMT_IB_NF) != 0;
    let inv_frmclk: bool =
        (inv_bits & SND_SOC_DAIFMT_IB_IF) != 0 || (inv_bits & SND_SOC_DAIFMT_NB_IF) != 0;
    let dac_master_bits: c_uint = if (*rk3308).codec_ver < ACODEC_VERSION_C {
        RK3308_DAC_IO_MODE_MASTER | RK3308_DAC_MODE_MASTER
    } else {
        RK3308BS_DAC_IO_MODE_MASTER | RK3308BS_DAC_MODE_MASTER
    };
    let mut adc_aif1: c_uint = 0;
    let mut adc_aif2: c_uint = 0;
    let mut dac_aif1: c_uint = 0;
    let mut dac_aif2: c_uint = 0;
    let mut is_master = false;
    let mut grp: c_int;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP => {
            adc_aif2 |= RK3308_ADC_IO_MODE_MASTER;
            adc_aif2 |= RK3308_ADC_MODE_MASTER;
            dac_aif2 |= dac_master_bits;
            is_master = true;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            adc_aif1 |= RK3308_ADC_I2S_MODE_PCM;
            dac_aif1 |= RK3308_DAC_I2S_MODE_PCM;
        }
        SND_SOC_DAIFMT_I2S => {
            adc_aif1 |= RK3308_ADC_I2S_MODE_I2S;
            dac_aif1 |= RK3308_DAC_I2S_MODE_I2S;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            adc_aif1 |= RK3308_ADC_I2S_MODE_RJ;
            dac_aif1 |= RK3308_DAC_I2S_MODE_RJ;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            adc_aif1 |= RK3308_ADC_I2S_MODE_LJ;
            dac_aif1 |= RK3308_DAC_I2S_MODE_LJ;
        }
        _ => return -EINVAL,
    }

    if inv_bitclk {
        adc_aif2 |= RK3308_ADC_I2S_BIT_CLK_POL_REVERSAL;
        dac_aif2 |= RK3308_DAC_I2S_BIT_CLK_POL_REVERSAL;
    }

    if inv_frmclk {
        adc_aif1 |= RK3308_ADC_I2S_LRC_POL_REVERSAL;
        dac_aif1 |= RK3308_DAC_I2S_LRC_POL_REVERSAL;
    }

    /*
     * Hold ADC Digital registers start at master mode
     *
     * There are 8 ADCs which use the same internal SCLK and LRCK for
     * master mode. We need to make sure that they are in effect at the
     * same time, otherwise they will cause abnormal clocks.
     */
    if is_master {
        regmap_clear_bits((*rk3308).regmap, RK3308_GLB_CON, RK3308_ADC_DIG_WORK);
    }

    grp = 0;
    while grp < ADC_LR_GROUP_MAX {
        regmap_update_bits(
            (*rk3308).regmap,
            RK3308_ADC_DIG_CON01(grp),
            RK3308_ADC_I2S_LRC_POL_REVERSAL | RK3308_ADC_I2S_MODE_MSK,
            adc_aif1,
        );
        regmap_update_bits(
            (*rk3308).regmap,
            RK3308_ADC_DIG_CON02(grp),
            RK3308_ADC_IO_MODE_MASTER
                | RK3308_ADC_MODE_MASTER
                | RK3308_ADC_I2S_BIT_CLK_POL_REVERSAL,
            adc_aif2,
        );
        grp += 1;
    }

    /* Hold ADC Digital registers end at master mode */
    if is_master {
        regmap_set_bits((*rk3308).regmap, RK3308_GLB_CON, RK3308_ADC_DIG_WORK);
    }

    regmap_update_bits(
        (*rk3308).regmap,
        RK3308_DAC_DIG_CON01,
        RK3308_DAC_I2S_LRC_POL_REVERSAL | RK3308_DAC_I2S_MODE_MSK,
        dac_aif1,
    );
    regmap_update_bits(
        (*rk3308).regmap,
        RK3308_DAC_DIG_CON02,
        dac_master_bits | RK3308_DAC_I2S_BIT_CLK_POL_REVERSAL,
        dac_aif2,
    );

    0
}

unsafe extern "C" fn rk3308_codec_dac_dig_config(
    rk3308: *mut rk3308_codec_priv,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut dac_aif1: c_uint = 0;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => dac_aif1 |= RK3308_DAC_I2S_VALID_LEN_16BITS,
        SNDRV_PCM_FORMAT_S20_3LE => dac_aif1 |= RK3308_DAC_I2S_VALID_LEN_20BITS,
        SNDRV_PCM_FORMAT_S24_LE => dac_aif1 |= RK3308_DAC_I2S_VALID_LEN_24BITS,
        SNDRV_PCM_FORMAT_S32_LE => dac_aif1 |= RK3308_DAC_I2S_VALID_LEN_32BITS,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*rk3308).regmap,
        RK3308_DAC_DIG_CON01,
        RK3308_DAC_I2S_VALID_LEN_MSK,
        dac_aif1,
    );
    regmap_set_bits((*rk3308).regmap, RK3308_DAC_DIG_CON02, RK3308_DAC_I2S_WORK);

    0
}

unsafe extern "C" fn rk3308_codec_adc_dig_config(
    rk3308: *mut rk3308_codec_priv,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut adc_aif1: c_uint = 0;
    /*
     * grp 0 = ADC1 and ADC2
     * grp 1 = ADC3 and ADC4
     * grp 2 = ADC5 and ADC6
     * grp 3 = ADC7 and ADC8
     */
    let used_adc_grps: u32;
    let mut grp: c_int;

    match params_channels(params) {
        1 => {
            adc_aif1 |= RK3308_ADC_I2S_MONO;
            used_adc_grps = 1;
        }
        2 | 4 | 6 | 8 => {
            used_adc_grps = (params_channels(params) / 2) as u32;
        }
        _ => {
            dev_err!(
                (*rk3308).dev,
                "Invalid channel number %d\n",
                params_channels(params)
            );
            return -EINVAL;
        }
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => adc_aif1 |= RK3308_ADC_I2S_VALID_LEN_16BITS,
        SNDRV_PCM_FORMAT_S20_3LE => adc_aif1 |= RK3308_ADC_I2S_VALID_LEN_20BITS,
        SNDRV_PCM_FORMAT_S24_LE => adc_aif1 |= RK3308_ADC_I2S_VALID_LEN_24BITS,
        SNDRV_PCM_FORMAT_S32_LE => adc_aif1 |= RK3308_ADC_I2S_VALID_LEN_32BITS,
        _ => return -EINVAL,
    }

    grp = 0;
    while (grp as u32) < used_adc_grps {
        regmap_update_bits(
            (*rk3308).regmap,
            RK3308_ADC_DIG_CON03(grp),
            RK3308_ADC_L_CH_BIST_MSK | RK3308_ADC_R_CH_BIST_MSK,
            RK3308_ADC_L_CH_NORMAL_LEFT | RK3308_ADC_R_CH_NORMAL_RIGHT,
        );
        regmap_update_bits(
            (*rk3308).regmap,
            RK3308_ADC_DIG_CON01(grp),
            RK3308_ADC_I2S_VALID_LEN_MSK | RK3308_ADC_I2S_MONO,
            adc_aif1,
        );
        regmap_set_bits(
            (*rk3308).regmap,
            RK3308_ADC_DIG_CON02(grp),
            RK3308_ADC_I2S_WORK,
        );
        grp += 1;
    }

    0
}

unsafe extern "C" fn rk3308_codec_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rk3308 = snd_soc_component_get_drvdata(component) as *mut rk3308_codec_priv;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        rk3308_codec_dac_dig_config(rk3308, params)
    } else {
        rk3308_codec_adc_dig_config(rk3308, params)
    }
}

static rk3308_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rk3308_codec_hw_params),
    set_fmt: Some(rk3308_codec_set_dai_fmt),
};

static mut rk3308_codec_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("rk3308-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("HiFi Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("HiFi Capture"),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &rk3308_codec_dai_ops,
};

unsafe extern "C" fn rk3308_codec_reset(component: *mut snd_soc_component) {
    let rk3308 = snd_soc_component_get_drvdata(component) as *mut rk3308_codec_priv;

    reset_control_assert((*rk3308).reset);
    usleep_range(10000, 11000); /* estimated value */
    reset_control_deassert((*rk3308).reset);

    regmap_write((*rk3308).regmap, RK3308_GLB_CON, 0x00);
    usleep_range(10000, 11000); /* estimated value */
    regmap_write(
        (*rk3308).regmap,
        RK3308_GLB_CON,
        RK3308_SYS_WORK | RK3308_DAC_DIG_WORK | RK3308_ADC_DIG_WORK,
    );
}

/*
 * Initialize register whose default after HW reset is problematic or which
 * are never modified.
 */
unsafe extern "C" fn rk3308_codec_initialize(rk3308: *mut rk3308_codec_priv) -> c_int {
    let mut grp: c_int;

    /*
     * Init ADC digital vol to 0 dB (reset value is 0xff, undocumented).
     * Range: -97dB ~ +32dB.
     */
    if (*rk3308).codec_ver == ACODEC_VERSION_C {
        grp = 0;
        while grp < ADC_LR_GROUP_MAX {
            regmap_write((*rk3308).regmap, RK3308_ADC_DIG_CON05(grp), RK3308_ADC_DIG_VOL_CON_x_0DB);
            regmap_write((*rk3308).regmap, RK3308_ADC_DIG_CON06(grp), RK3308_ADC_DIG_VOL_CON_x_0DB);
            grp += 1;
        }
    }

    /* set HPMIX default gains (reset value is 0, which is illegal) */
    regmap_update_bits(
        (*rk3308).regmap,
        RK3308_DAC_ANA_CON12,
        RK3308_DAC_L_HPMIX_GAIN_MSK | RK3308_DAC_R_HPMIX_GAIN_MSK,
        RK3308_DAC_L_HPMIX_GAIN_NDB_6 | RK3308_DAC_R_HPMIX_GAIN_NDB_6,
    );

    /* recover DAC digital gain to 0 dB (reset value is 0xff, undocumented) */
    if (*rk3308).codec_ver == ACODEC_VERSION_C {
        regmap_write((*rk3308).regmap, RK3308_DAC_DIG_CON04, RK3308BS_DAC_DIG_GAIN_0DB);
    }

    /*
     * Unconditionally enable zero-cross detection (needed for AGC,
     * harmless without AGC)
     */
    grp = 0;
    while grp < ADC_LR_GROUP_MAX {
        regmap_set_bits(
            (*rk3308).regmap,
            RK3308_ADC_ANA_CON02(grp),
            RK3308_ADC_CH1_ZEROCROSS_DET_EN | RK3308_ADC_CH2_ZEROCROSS_DET_EN,
        );
        grp += 1;
    }

    0
}

unsafe extern "C" fn rk3308_codec_probe(component: *mut snd_soc_component) -> c_int {
    let rk3308 = snd_soc_component_get_drvdata(component) as *mut rk3308_codec_priv;

    (*rk3308).component = component;

    rk3308_codec_reset(component);
    rk3308_codec_initialize(rk3308);

    0
}

unsafe extern "C" fn rk3308_codec_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let rk3308 = snd_soc_component_get_drvdata(component) as *mut rk3308_codec_priv;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) != SND_SOC_BIAS_OFF {
                return 0;
            }

            /* Sequence from TRM Section 8.6.3 "Power Up" */
            regmap_set_bits(
                (*rk3308).regmap,
                RK3308_DAC_ANA_CON02,
                RK3308_DAC_L_DAC_EN | RK3308_DAC_R_DAC_EN,
            );
            regmap_update_bits(
                (*rk3308).regmap,
                RK3308_ADC_ANA_CON10(0),
                RK3308_ADC_CURRENT_CHARGE_MSK,
                1,
            );
            regmap_set_bits((*rk3308).regmap, RK3308_ADC_ANA_CON10(0), RK3308_ADC_REF_EN);
            regmap_update_bits(
                (*rk3308).regmap,
                RK3308_ADC_ANA_CON10(0),
                RK3308_ADC_CURRENT_CHARGE_MSK,
                0x7f,
            );
            msleep(20); /* estimated value */
        }
        SND_SOC_BIAS_OFF => {
            /* Sequence from TRM Section 8.6.4 "Power Down" */
            regmap_update_bits(
                (*rk3308).regmap,
                RK3308_ADC_ANA_CON10(0),
                RK3308_ADC_CURRENT_CHARGE_MSK,
                1,
            );
            regmap_clear_bits((*rk3308).regmap, RK3308_ADC_ANA_CON10(0), RK3308_ADC_REF_EN);
            regmap_clear_bits(
                (*rk3308).regmap,
                RK3308_DAC_ANA_CON02,
                RK3308_DAC_L_DAC_EN | RK3308_DAC_R_DAC_EN,
            );
            msleep(20); /* estimated value */
        }
    }
    0
}

static rk3308_codec_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rk3308_codec_probe),
    set_bias_level: Some(rk3308_codec_set_bias_level),
    controls: rk3308_codec_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(rk3308_codec_controls),
    dapm_widgets: rk3308_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(rk3308_codec_dapm_widgets),
    dapm_routes: rk3308_codec_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(rk3308_codec_dapm_routes),
};

static rk3308_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: RK3308_DAC_ANA_CON15,
};

unsafe extern "C" fn rk3308_codec_get_version(rk3308: *mut rk3308_codec_priv) -> c_int {
    let mut chip_id: c_uint = 0;
    let mut err: c_int;

    err = regmap_read((*rk3308).grf, GRF_CHIP_ID, &mut chip_id);
    if err != 0 {
        return err;
    }

    match chip_id {
        3306 => {
            (*rk3308).codec_ver = ACODEC_VERSION_A;
        }
        0x3308 => {
            (*rk3308).codec_ver = ACODEC_VERSION_B;
            return dev_err_probe(
                (*rk3308).dev,
                -EINVAL,
                c_str!("Chip version B not supported\n"),
            );
        }
        0x3308c => {
            (*rk3308).codec_ver = ACODEC_VERSION_C;
        }
        _ => {
            return dev_err_probe(
                (*rk3308).dev,
                -EINVAL,
                c_str!("Unknown chip_id: 0x%x\n"),
                chip_id,
            );
        }
    }

    dev_info!((*rk3308).dev, "Found codec version %c\n", (*rk3308).codec_ver);
    0
}

unsafe extern "C" fn rk3308_codec_set_micbias_level(
    rk3308: *mut rk3308_codec_priv,
) -> c_int {
    let np = (*(*rk3308).dev).of_node;
    let mut percent: u32 = 0;
    let mult: u32;
    let mut err: c_int;

    err = of_property_read_u32(np, c_str!("rockchip,micbias-avdd-percent"), &mut percent);
    if err == -EINVAL {
        return 0;
    }
    if err != 0 {
        return dev_err_probe(
            (*rk3308).dev,
            err,
            c_str!("Error reading 'rockchip,micbias-avdd-percent'\n"),
        );
    }

    /* Convert percent to register value, linerarly (50% -> 0, 5% step = +1) */
    mult = (percent - 50) / 5;

    /* Check range and that the percent was an exact value allowed */
    if mult > RK3308_ADC_LEVEL_RANGE_MICBIAS_MAX || mult * 5 + 50 != percent {
        return dev_err_probe(
            (*rk3308).dev,
            -EINVAL,
            c_str!("Invalid value %u for 'rockchip,micbias-avdd-percent'\n"),
            percent,
        );
    }

    regmap_update_bits(
        (*rk3308).regmap,
        RK3308_ADC_ANA_CON07(0),
        RK3308_ADC_LEVEL_RANGE_MICBIAS_MSK,
        mult << RK3308_ADC_LEVEL_RANGE_MICBIAS_SFT,
    );

    0
}

unsafe extern "C" fn rk3308_codec_platform_probe(pdev: *mut platform_device) -> c_int {
    let np = (*(*pdev).dev).of_node;
    let dev = &mut (*pdev).dev as *mut device;
    let rk3308: *mut rk3308_codec_priv;
    let base: *mut c_void;
    let mut err: c_int;

    rk3308 = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<rk3308_codec_priv>(), GFP_KERNEL)
        as *mut rk3308_codec_priv;
    if rk3308.is_null() {
        return -ENOMEM;
    }

    (*rk3308).dev = dev;

    (*rk3308).grf = syscon_regmap_lookup_by_phandle(np, c_str!("rockchip,grf"));
    if IS_ERR((*rk3308).grf) {
        return dev_err_probe(dev, PTR_ERR((*rk3308).grf), c_str!("Error getting GRF\n"));
    }

    (*rk3308).reset = devm_reset_control_get_optional_exclusive(dev, c_str!("codec"));
    if IS_ERR((*rk3308).reset) {
        return dev_err_probe(
            dev,
            PTR_ERR((*rk3308).reset),
            c_str!("Failed to get reset control\n"),
        );
    }

    err = devm_clk_bulk_get(dev, ARRAY_SIZE!(rk3308_codec_clocks), rk3308_codec_clocks.as_mut_ptr());
    if err != 0 {
        return dev_err_probe(dev, err, c_str!("Failed to get clocks\n"));
    }

    err = clk_bulk_prepare_enable(ARRAY_SIZE!(rk3308_codec_clocks), rk3308_codec_clocks.as_mut_ptr());
    if err != 0 {
        return dev_err_probe(dev, err, c_str!("Failed to enable clocks\n"));
    }

    err = rk3308_codec_get_version(rk3308);
    if err != 0 {
        return err;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*rk3308).regmap = devm_regmap_init_mmio(dev, base, &rk3308_codec_regmap_config);
    if IS_ERR((*rk3308).regmap) {
        return dev_err_probe(
            dev,
            PTR_ERR((*rk3308).regmap),
            c_str!("Failed to init regmap\n"),
        );
    }

    platform_set_drvdata(pdev, rk3308 as *mut c_void);

    err = rk3308_codec_set_micbias_level(rk3308);
    if err != 0 {
        return err;
    }

    err = devm_snd_soc_register_component(
        dev,
        &rk3308_codec_component_driver,
        &mut rk3308_codec_dai_driver,
        1,
    );
    if err != 0 {
        return dev_err_probe(dev, err, c_str!("Failed to register codec\n"));
    }

    0
}

static rk3308_codec_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("rockchip,rk3308-codec"),
    },
    of_device_id {},
];
MODULE_DEVICE_TABLE!(of, rk3308_codec_of_match);

static mut rk3308_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("rk3308-acodec"),
        of_match_table: rk3308_codec_of_match.as_ptr(),
    },
    probe: Some(rk3308_codec_platform_probe),
};
module_platform_driver!(rk3308_codec_driver);

MODULE_AUTHOR!("Xing Zheng <zhengxing@rock-chips.com>");
MODULE_AUTHOR!("Luca Ceresoli <luca.ceresoli@bootlin.com>");
MODULE_DESCRIPTION!("ASoC RK3308 Codec Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
