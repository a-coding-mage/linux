// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Common code for ADAU1X61 and ADAU1X81 codecs
 *
 * Copyright 2011-2014 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// Dependencies from the original C includes:
// linux/module.h, linux/init.h, linux/clk.h, linux/delay.h, linux/slab.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h,
// linux/i2c.h, linux/spi/spi.h, linux/regmap.h, linux/unaligned.h,
// sigmadsp.h, adau17x1.h, adau-utils.h.

pub const ADAU17X1_SAFELOAD_TARGET_ADDRESS: u32 = 0x0006;
pub const ADAU17X1_SAFELOAD_TRIGGER: u32 = 0x0007;
pub const ADAU17X1_SAFELOAD_DATA: u32 = 0x0001;
pub const ADAU17X1_SAFELOAD_DATA_SIZE: usize = 20;
pub const ADAU17X1_WORD_SIZE: usize = 4;

static adau17x1_capture_mixer_boost_text: [&str; 4] = [
    "Normal operation",
    "Boost Level 1",
    "Boost Level 2",
    "Boost Level 3",
];

static adau17x1_capture_boost_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ADAU17X1_REC_POWER_MGMT, 5, adau17x1_capture_mixer_boost_text);

static adau17x1_mic_bias_mode_text: [&str; 2] = [
    "Normal operation",
    "High performance",
];

static adau17x1_mic_bias_mode_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ADAU17X1_MICBIAS, 3, adau17x1_mic_bias_mode_text);

static adau17x1_digital_tlv: [u32; 4] = DECLARE_TLV_DB_MINMAX!(-9563, 0);

static adau17x1_controls: [snd_kcontrol_new; 6] = [
    SOC_DOUBLE_R_TLV!(
        "Digital Capture Volume",
        ADAU17X1_LEFT_INPUT_DIGITAL_VOL,
        ADAU17X1_RIGHT_INPUT_DIGITAL_VOL,
        0,
        0xff,
        1,
        adau17x1_digital_tlv
    ),
    SOC_DOUBLE_R_TLV!(
        "Digital Playback Volume",
        ADAU17X1_DAC_CONTROL1,
        ADAU17X1_DAC_CONTROL2,
        0,
        0xff,
        1,
        adau17x1_digital_tlv
    ),
    SOC_SINGLE!("ADC High Pass Filter Switch", ADAU17X1_ADC_CONTROL, 5, 1, 0),
    SOC_SINGLE!("Playback De-emphasis Switch", ADAU17X1_DAC_CONTROL0, 2, 1, 0),
    SOC_ENUM!("Capture Boost", adau17x1_capture_boost_enum),
    SOC_ENUM!("Mic Bias Mode", adau17x1_mic_bias_mode_enum),
];

unsafe extern "C" fn adau17x1_pll_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;

    if SND_SOC_DAPM_EVENT_ON(event) {
        (*adau).pll_regs[5] = 1;
    } else {
        (*adau).pll_regs[5] = 0;
        /*
         * Bypass the PLL when disabled, otherwise registers will become
         * inaccessible.
         */
        regmap_update_bits(
            (*adau).regmap,
            ADAU17X1_CLOCK_CONTROL,
            ADAU17X1_CLOCK_CONTROL_CORECLK_SRC_PLL,
            0,
        );
    }

    /* The PLL register is 6 bytes long and can only be written at once. */
    regmap_raw_write(
        (*adau).regmap,
        ADAU17X1_PLL_CONTROL,
        (*adau).pll_regs.as_mut_ptr() as *const c_void,
        (*adau).pll_regs.len(),
    );

    if SND_SOC_DAPM_EVENT_ON(event) {
        mdelay(5);
        regmap_update_bits(
            (*adau).regmap,
            ADAU17X1_CLOCK_CONTROL,
            ADAU17X1_CLOCK_CONTROL_CORECLK_SRC_PLL,
            ADAU17X1_CLOCK_CONTROL_CORECLK_SRC_PLL,
        );
    }

    0
}

unsafe extern "C" fn adau17x1_adc_fixup(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;

    /*
     * If we are capturing, toggle the ADOSR bit in Converter Control 0 to
     * avoid losing SNR (workaround from ADI). This must be done after
     * the ADC(s) have been enabled. According to the data sheet, it is
     * normally illegal to set this bit when the sampling rate is 96 kHz,
     * but according to ADI it is acceptable for this workaround.
     */
    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_CONVERTER0,
        ADAU17X1_CONVERTER0_ADOSR,
        ADAU17X1_CONVERTER0_ADOSR,
    );
    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_CONVERTER0,
        ADAU17X1_CONVERTER0_ADOSR,
        0,
    );

    0
}

static adau17x1_mono_stereo_text: [&str; 4] = [
    "Stereo",
    "Mono Left Channel (L+R)",
    "Mono Right Channel (L+R)",
    "Mono (L+R)",
];

static adau17x1_dac_mode_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ADAU17X1_DAC_CONTROL0, 6, adau17x1_mono_stereo_text);

static adau17x1_dac_mode_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("DAC Mono-Stereo-Mode", adau17x1_dac_mode_enum);

static adau17x1_dapm_widgets: [snd_soc_dapm_widget; 11] = [
    SND_SOC_DAPM_SUPPLY_S!(
        "PLL",
        3,
        SND_SOC_NOPM,
        0,
        0,
        adau17x1_pll_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY!("AIFCLK", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS", ADAU17X1_MICBIAS, 0, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("Left Playback Enable", ADAU17X1_PLAY_POWER_MGMT, 0, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("Right Playback Enable", ADAU17X1_PLAY_POWER_MGMT, 1, 0, None, 0),
    SND_SOC_DAPM_MUX!("Left DAC Mode Mux", SND_SOC_NOPM, 0, 0, &adau17x1_dac_mode_mux),
    SND_SOC_DAPM_MUX!("Right DAC Mode Mux", SND_SOC_NOPM, 0, 0, &adau17x1_dac_mode_mux),
    SND_SOC_DAPM_ADC_E!("Left Decimator", None, ADAU17X1_ADC_CONTROL, 0, 0, adau17x1_adc_fixup, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_ADC!("Right Decimator", None, ADAU17X1_ADC_CONTROL, 1, 0),
    SND_SOC_DAPM_DAC!("Left DAC", None, ADAU17X1_DAC_CONTROL0, 0, 0),
    SND_SOC_DAPM_DAC!("Right DAC", None, ADAU17X1_DAC_CONTROL0, 1, 0),
];

static adau17x1_dapm_routes: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route { sink: "Left Decimator", control: None, source: "SYSCLK" },
    snd_soc_dapm_route { sink: "Right Decimator", control: None, source: "SYSCLK" },
    snd_soc_dapm_route { sink: "Left DAC", control: None, source: "SYSCLK" },
    snd_soc_dapm_route { sink: "Right DAC", control: None, source: "SYSCLK" },
    snd_soc_dapm_route { sink: "Capture", control: None, source: "SYSCLK" },
    snd_soc_dapm_route { sink: "Playback", control: None, source: "SYSCLK" },
    snd_soc_dapm_route { sink: "Left DAC", control: None, source: "Left DAC Mode Mux" },
    snd_soc_dapm_route { sink: "Right DAC", control: None, source: "Right DAC Mode Mux" },
    snd_soc_dapm_route { sink: "Capture", control: None, source: "AIFCLK" },
    snd_soc_dapm_route { sink: "Playback", control: None, source: "AIFCLK" },
];

static adau17x1_dapm_pll_route: snd_soc_dapm_route =
    snd_soc_dapm_route { sink: "SYSCLK", control: None, source: "PLL" };

/*
 * The MUX register for the Capture and Playback MUXs selects either DSP as
 * source/destination or one of the TDM slots. The TDM slot is selected via
 * snd_soc_dai_set_tdm_slot(), so we only expose whether to go to the DSP or
 * directly to the DAI interface with this control.
 */
unsafe extern "C" fn adau17x1_dsp_mux_enum_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mut update: snd_soc_dapm_update = core::mem::zeroed();
    let stream = (*e).shift_l;
    let mut val: u32;
    let change: u32;
    let reg: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    match (*ucontrol).value.enumerated.item[0] {
        0 => {
            val = 0;
            (*adau).dsp_bypass[stream as usize] = false;
        }
        _ => {
            val = ((*adau).tdm_slot[stream as usize] * 2) + 1;
            (*adau).dsp_bypass[stream as usize] = true;
        }
    }

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = ADAU17X1_SERIAL_INPUT_ROUTE as c_int;
    } else {
        reg = ADAU17X1_SERIAL_OUTPUT_ROUTE as c_int;
    }

    change = snd_soc_component_test_bits(component, reg as u32, 0xff, val);
    if change != 0 {
        update.kcontrol = kcontrol;
        update.mask = 0xff;
        update.reg = reg as u32;
        update.val = val;

        snd_soc_dapm_mux_update_power(
            dapm,
            kcontrol,
            (*ucontrol).value.enumerated.item[0],
            e,
            &mut update,
        );
    }

    change as c_int
}

unsafe extern "C" fn adau17x1_dsp_mux_enum_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let stream = (*e).shift_l;
    let reg: u32;
    let mut val: u32 = 0;
    let ret: c_int;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = ADAU17X1_SERIAL_INPUT_ROUTE;
    } else {
        reg = ADAU17X1_SERIAL_OUTPUT_ROUTE;
    }

    ret = regmap_read((*adau).regmap, reg, &mut val);
    if ret != 0 {
        return ret;
    }

    if val != 0 {
        val = 1;
    }
    (*ucontrol).value.enumerated.item[0] = val;

    0
}

// C macro DECLARE_ADAU17X1_DSP_MUX_CTRL translated at its two use sites below.
static adau17x1_dac_mux_text: [&str; 2] = ["DSP", "AIFIN"];
static adau17x1_capture_mux_text: [&str; 2] = ["DSP", "Decimator"];

static adau17x1_dac_mux: snd_kcontrol_new = SOC_ENUM_EXT!(
    "DAC Playback Mux",
    SOC_ENUM_SINGLE!(SND_SOC_NOPM, SNDRV_PCM_STREAM_PLAYBACK, adau17x1_dac_mux_text.len(), adau17x1_dac_mux_text),
    adau17x1_dsp_mux_enum_get,
    adau17x1_dsp_mux_enum_put
);

static adau17x1_capture_mux: snd_kcontrol_new = SOC_ENUM_EXT!(
    "Capture Mux",
    SOC_ENUM_SINGLE!(SND_SOC_NOPM, SNDRV_PCM_STREAM_CAPTURE, adau17x1_capture_mux_text.len(), adau17x1_capture_mux_text),
    adau17x1_dsp_mux_enum_get,
    adau17x1_dsp_mux_enum_put
);

static adau17x1_dsp_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_PGA!("DSP", ADAU17X1_DSP_RUN, 0, 0, None, 0),
    SND_SOC_DAPM_SIGGEN!("DSP Siggen"),
    SND_SOC_DAPM_MUX!("DAC Playback Mux", SND_SOC_NOPM, 0, 0, &adau17x1_dac_mux),
    SND_SOC_DAPM_MUX!("Capture Mux", SND_SOC_NOPM, 0, 0, &adau17x1_capture_mux),
];

static adau17x1_dsp_dapm_routes: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route { sink: "DAC Playback Mux", control: Some("DSP"), source: "DSP" },
    snd_soc_dapm_route { sink: "DAC Playback Mux", control: Some("AIFIN"), source: "Playback" },
    snd_soc_dapm_route { sink: "Left DAC Mode Mux", control: Some("Stereo"), source: "DAC Playback Mux" },
    snd_soc_dapm_route { sink: "Left DAC Mode Mux", control: Some("Mono (L+R)"), source: "DAC Playback Mux" },
    snd_soc_dapm_route { sink: "Left DAC Mode Mux", control: Some("Mono Left Channel (L+R)"), source: "DAC Playback Mux" },
    snd_soc_dapm_route { sink: "Right DAC Mode Mux", control: Some("Stereo"), source: "DAC Playback Mux" },
    snd_soc_dapm_route { sink: "Right DAC Mode Mux", control: Some("Mono (L+R)"), source: "DAC Playback Mux" },
    snd_soc_dapm_route { sink: "Right DAC Mode Mux", control: Some("Mono Right Channel (L+R)"), source: "DAC Playback Mux" },
    snd_soc_dapm_route { sink: "Capture Mux", control: Some("DSP"), source: "DSP" },
    snd_soc_dapm_route { sink: "Capture Mux", control: Some("Decimator"), source: "Left Decimator" },
    snd_soc_dapm_route { sink: "Capture Mux", control: Some("Decimator"), source: "Right Decimator" },
    snd_soc_dapm_route { sink: "Capture", control: None, source: "Capture Mux" },
    snd_soc_dapm_route { sink: "DSP", control: None, source: "DSP Siggen" },
    snd_soc_dapm_route { sink: "DSP", control: None, source: "Left Decimator" },
    snd_soc_dapm_route { sink: "DSP", control: None, source: "Right Decimator" },
    snd_soc_dapm_route { sink: "DSP", control: None, source: "Playback" },
];

static adau17x1_no_dsp_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: "Left DAC Mode Mux", control: Some("Stereo"), source: "Playback" },
    snd_soc_dapm_route { sink: "Left DAC Mode Mux", control: Some("Mono (L+R)"), source: "Playback" },
    snd_soc_dapm_route { sink: "Left DAC Mode Mux", control: Some("Mono Left Channel (L+R)"), source: "Playback" },
    snd_soc_dapm_route { sink: "Right DAC Mode Mux", control: Some("Stereo"), source: "Playback" },
    snd_soc_dapm_route { sink: "Right DAC Mode Mux", control: Some("Mono (L+R)"), source: "Playback" },
    snd_soc_dapm_route { sink: "Right DAC Mode Mux", control: Some("Mono Right Channel (L+R)"), source: "Playback" },
    snd_soc_dapm_route { sink: "Capture", control: None, source: "Left Decimator" },
    snd_soc_dapm_route { sink: "Capture", control: None, source: "Right Decimator" },
];

unsafe fn adau17x1_has_dsp(adau: *mut adau) -> bool {
    match (*adau).type_ {
        ADAU1761 | ADAU1381 | ADAU1781 => true,
        _ => false,
    }
}

/* Chip has a DSP but we're pretending it doesn't. */
unsafe fn adau17x1_has_disused_dsp(adau: *mut adau) -> bool {
    match (*adau).type_ {
        ADAU1761_AS_1361 => true,
        _ => false,
    }
}

unsafe fn adau17x1_has_safeload(adau: *mut adau) -> bool {
    match (*adau).type_ {
        ADAU1761 | ADAU1781 => true,
        _ => false,
    }
}

unsafe extern "C" fn adau17x1_set_dai_pll(
    dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: u32,
    freq_out: u32,
) -> c_int {
    let component = (*dai).component;
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mut ret: c_int;

    if freq_in < 8000000 || freq_in > 27000000 {
        return -EINVAL;
    }

    ret = adau_calc_pll_cfg(freq_in, freq_out, (*adau).pll_regs.as_mut_ptr());
    if ret < 0 {
        return ret;
    }

    /* The PLL register is 6 bytes long and can only be written at once. */
    ret = regmap_raw_write(
        (*adau).regmap,
        ADAU17X1_PLL_CONTROL,
        (*adau).pll_regs.as_mut_ptr() as *const c_void,
        (*adau).pll_regs.len(),
    );
    if ret != 0 {
        return ret;
    }

    (*adau).pll_freq = freq_out;

    0
}

unsafe extern "C" fn adau17x1_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: u32,
    _dir: c_int,
) -> c_int {
    let dapm = snd_soc_component_to_dapm((*dai).component);
    let adau = snd_soc_component_get_drvdata((*dai).component) as *mut adau;
    let is_pll: bool;
    let was_pll: bool;

    match clk_id {
        ADAU17X1_CLK_SRC_MCLK => is_pll = false,
        ADAU17X1_CLK_SRC_PLL_AUTO => {
            if (*adau).mclk.is_null() {
                return -EINVAL;
            }
            is_pll = true;
        }
        ADAU17X1_CLK_SRC_PLL => is_pll = true,
        _ => return -EINVAL,
    }

    match (*adau).clk_src {
        ADAU17X1_CLK_SRC_MCLK => was_pll = false,
        ADAU17X1_CLK_SRC_PLL | ADAU17X1_CLK_SRC_PLL_AUTO => was_pll = true,
        _ => return -EINVAL,
    }

    (*adau).sysclk = freq;

    if is_pll != was_pll {
        if is_pll {
            snd_soc_dapm_add_routes(dapm, &adau17x1_dapm_pll_route, 1);
        } else {
            snd_soc_dapm_del_routes(dapm, &adau17x1_dapm_pll_route, 1);
        }
    }

    (*adau).clk_src = clk_id;

    0
}

unsafe fn adau17x1_auto_pll(
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let adau = snd_soc_dai_get_drvdata(dai) as *mut adau;
    let pll_rate: u32;

    match params_rate(params) {
        48000 | 8000 | 12000 | 16000 | 24000 | 32000 | 96000 => {
            pll_rate = 48000 * 1024;
        }
        44100 | 7350 | 11025 | 14700 | 22050 | 29400 | 88200 => {
            pll_rate = 44100 * 1024;
        }
        _ => return -EINVAL,
    }

    adau17x1_set_dai_pll(
        dai,
        ADAU17X1_PLL,
        ADAU17X1_PLL_SRC_MCLK,
        clk_get_rate((*adau).mclk) as u32,
        pll_rate,
    )
}

unsafe extern "C" fn adau17x1_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mut val: u32;
    let div: u32;
    let dsp_div: u32;
    let freq: u32;
    let mut ret: c_int;

    match (*adau).clk_src {
        ADAU17X1_CLK_SRC_PLL_AUTO => {
            ret = adau17x1_auto_pll(dai, params);
            if ret != 0 {
                return ret;
            }
            freq = (*adau).pll_freq;
        }
        ADAU17X1_CLK_SRC_PLL => freq = (*adau).pll_freq,
        _ => freq = (*adau).sysclk,
    }

    if freq % params_rate(params) != 0 {
        return -EINVAL;
    }

    match freq / params_rate(params) {
        1024 => {
            /* fs */
            div = 0;
            dsp_div = 1;
        }
        6144 => {
            /* fs / 6 */
            div = 1;
            dsp_div = 6;
        }
        4096 => {
            /* fs / 4 */
            div = 2;
            dsp_div = 5;
        }
        3072 => {
            /* fs / 3 */
            div = 3;
            dsp_div = 4;
        }
        2048 => {
            /* fs / 2 */
            div = 4;
            dsp_div = 3;
        }
        1536 => {
            /* fs / 1.5 */
            div = 5;
            dsp_div = 2;
        }
        512 => {
            /* fs / 0.5 */
            div = 6;
            dsp_div = 0;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_CONVERTER0,
        ADAU17X1_CONVERTER0_CONVSR_MASK,
        div,
    );

    if adau17x1_has_dsp(adau) || adau17x1_has_disused_dsp(adau) {
        regmap_write((*adau).regmap, ADAU17X1_SERIAL_SAMPLING_RATE, div);
    }
    if adau17x1_has_dsp(adau) {
        regmap_write((*adau).regmap, ADAU17X1_DSP_SAMPLING_RATE, dsp_div);
    }

    if !(*adau).sigmadsp.is_null() {
        ret = adau17x1_setup_firmware(component, params_rate(params));
        if ret < 0 {
            return ret;
        }
    }

    if (*adau).dai_fmt != SND_SOC_DAIFMT_RIGHT_J {
        return 0;
    }

    match params_width(params) {
        16 => val = ADAU17X1_SERIAL_PORT1_DELAY16,
        24 => val = ADAU17X1_SERIAL_PORT1_DELAY8,
        32 => val = ADAU17X1_SERIAL_PORT1_DELAY0,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_SERIAL_PORT1,
        ADAU17X1_SERIAL_PORT1_DELAY_MASK,
        val,
    )
}

unsafe extern "C" fn adau17x1_set_dai_fmt(
    dai: *mut snd_soc_dai,
    fmt: u32,
) -> c_int {
    let adau = snd_soc_component_get_drvdata((*dai).component) as *mut adau;
    let mut ctrl0: u32;
    let ctrl1: u32;
    let ctrl0_mask: u32;
    let mut lrclk_pol: c_int;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            ctrl0 = ADAU17X1_SERIAL_PORT0_MASTER;
            (*adau).master = true;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            ctrl0 = 0;
            (*adau).master = false;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            lrclk_pol = 0;
            ctrl1 = ADAU17X1_SERIAL_PORT1_DELAY1;
        }
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {
            lrclk_pol = 1;
            ctrl1 = ADAU17X1_SERIAL_PORT1_DELAY0;
        }
        SND_SOC_DAIFMT_DSP_A => {
            lrclk_pol = 1;
            ctrl0 |= ADAU17X1_SERIAL_PORT0_PULSE_MODE;
            ctrl1 = ADAU17X1_SERIAL_PORT1_DELAY1;
        }
        SND_SOC_DAIFMT_DSP_B => {
            lrclk_pol = 1;
            ctrl0 |= ADAU17X1_SERIAL_PORT0_PULSE_MODE;
            ctrl1 = ADAU17X1_SERIAL_PORT1_DELAY0;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => {
            ctrl0 |= ADAU17X1_SERIAL_PORT0_BCLK_POL;
        }
        SND_SOC_DAIFMT_NB_IF => {
            lrclk_pol = (lrclk_pol == 0) as c_int;
        }
        SND_SOC_DAIFMT_IB_IF => {
            ctrl0 |= ADAU17X1_SERIAL_PORT0_BCLK_POL;
            lrclk_pol = (lrclk_pol == 0) as c_int;
        }
        _ => return -EINVAL,
    }

    if lrclk_pol != 0 {
        ctrl0 |= ADAU17X1_SERIAL_PORT0_LRCLK_POL;
    }

    /* Set the mask to update all relevant bits in ADAU17X1_SERIAL_PORT0 */
    ctrl0_mask = ADAU17X1_SERIAL_PORT0_MASTER
        | ADAU17X1_SERIAL_PORT0_LRCLK_POL
        | ADAU17X1_SERIAL_PORT0_BCLK_POL
        | ADAU17X1_SERIAL_PORT0_PULSE_MODE;

    regmap_update_bits((*adau).regmap, ADAU17X1_SERIAL_PORT0, ctrl0_mask, ctrl0);
    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_SERIAL_PORT1,
        ADAU17X1_SERIAL_PORT1_DELAY_MASK,
        ctrl1,
    );

    (*adau).dai_fmt = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

    0
}

unsafe extern "C" fn adau17x1_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: u32,
    mut rx_mask: u32,
    mut slots: c_int,
    mut slot_width: c_int,
) -> c_int {
    let adau = snd_soc_component_get_drvdata((*dai).component) as *mut adau;
    let ser_ctrl0: u32;
    let ser_ctrl1: u32;
    let conv_ctrl0: u32;
    let conv_ctrl1: u32;

    /* I2S mode */
    if slots == 0 {
        slots = 2;
        rx_mask = 3;
        tx_mask = 3;
        slot_width = 32;
    }

    match slots {
        2 => ser_ctrl0 = ADAU17X1_SERIAL_PORT0_STEREO,
        4 => ser_ctrl0 = ADAU17X1_SERIAL_PORT0_TDM4,
        8 => {
            if (*adau).type_ == ADAU1361 {
                return -EINVAL;
            }
            ser_ctrl0 = ADAU17X1_SERIAL_PORT0_TDM8;
        }
        _ => return -EINVAL,
    }

    match slot_width * slots {
        32 => {
            if (*adau).type_ == ADAU1761 || (*adau).type_ == ADAU1761_AS_1361 {
                return -EINVAL;
            }
            ser_ctrl1 = ADAU17X1_SERIAL_PORT1_BCLK32;
        }
        64 => ser_ctrl1 = ADAU17X1_SERIAL_PORT1_BCLK64,
        48 => ser_ctrl1 = ADAU17X1_SERIAL_PORT1_BCLK48,
        128 => ser_ctrl1 = ADAU17X1_SERIAL_PORT1_BCLK128,
        256 => {
            if (*adau).type_ == ADAU1361 {
                return -EINVAL;
            }
            ser_ctrl1 = ADAU17X1_SERIAL_PORT1_BCLK256;
        }
        _ => return -EINVAL,
    }

    match rx_mask {
        0x03 => {
            conv_ctrl1 = ADAU17X1_CONVERTER1_ADC_PAIR(1);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_CAPTURE as usize] = 0;
        }
        0x0c => {
            conv_ctrl1 = ADAU17X1_CONVERTER1_ADC_PAIR(2);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_CAPTURE as usize] = 1;
        }
        0x30 => {
            conv_ctrl1 = ADAU17X1_CONVERTER1_ADC_PAIR(3);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_CAPTURE as usize] = 2;
        }
        0xc0 => {
            conv_ctrl1 = ADAU17X1_CONVERTER1_ADC_PAIR(4);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_CAPTURE as usize] = 3;
        }
        _ => return -EINVAL,
    }

    match tx_mask {
        0x03 => {
            conv_ctrl0 = ADAU17X1_CONVERTER0_DAC_PAIR(1);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_PLAYBACK as usize] = 0;
        }
        0x0c => {
            conv_ctrl0 = ADAU17X1_CONVERTER0_DAC_PAIR(2);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_PLAYBACK as usize] = 1;
        }
        0x30 => {
            conv_ctrl0 = ADAU17X1_CONVERTER0_DAC_PAIR(3);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_PLAYBACK as usize] = 2;
        }
        0xc0 => {
            conv_ctrl0 = ADAU17X1_CONVERTER0_DAC_PAIR(4);
            (*adau).tdm_slot[SNDRV_PCM_STREAM_PLAYBACK as usize] = 3;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_CONVERTER0,
        ADAU17X1_CONVERTER0_DAC_PAIR_MASK,
        conv_ctrl0,
    );
    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_CONVERTER1,
        ADAU17X1_CONVERTER1_ADC_PAIR_MASK,
        conv_ctrl1,
    );
    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_SERIAL_PORT0,
        ADAU17X1_SERIAL_PORT0_TDM_MASK,
        ser_ctrl0,
    );
    regmap_update_bits(
        (*adau).regmap,
        ADAU17X1_SERIAL_PORT1,
        ADAU17X1_SERIAL_PORT1_BCLK_MASK,
        ser_ctrl1,
    );

    if !adau17x1_has_dsp(adau) && !adau17x1_has_disused_dsp(adau) {
        return 0;
    }

    if (*adau).dsp_bypass[SNDRV_PCM_STREAM_PLAYBACK as usize] {
        regmap_write(
            (*adau).regmap,
            ADAU17X1_SERIAL_INPUT_ROUTE,
            ((*adau).tdm_slot[SNDRV_PCM_STREAM_PLAYBACK as usize] * 2) + 1,
        );
    }

    if (*adau).dsp_bypass[SNDRV_PCM_STREAM_CAPTURE as usize] {
        regmap_write(
            (*adau).regmap,
            ADAU17X1_SERIAL_OUTPUT_ROUTE,
            ((*adau).tdm_slot[SNDRV_PCM_STREAM_CAPTURE as usize] * 2) + 1,
        );
    }

    0
}

unsafe extern "C" fn adau17x1_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let adau = snd_soc_component_get_drvdata((*dai).component) as *mut adau;

    if !(*adau).sigmadsp.is_null() {
        return sigmadsp_restrict_params((*adau).sigmadsp, substream);
    }

    0
}

pub static adau17x1_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(adau17x1_hw_params),
    set_sysclk: Some(adau17x1_set_dai_sysclk),
    set_fmt: Some(adau17x1_set_dai_fmt),
    set_pll: Some(adau17x1_set_dai_pll),
    set_tdm_slot: Some(adau17x1_set_dai_tdm_slot),
    startup: Some(adau17x1_startup),
};
// EXPORT_SYMBOL_GPL(adau17x1_dai_ops);

pub unsafe extern "C" fn adau17x1_set_micbias_voltage(
    component: *mut snd_soc_component,
    micbias: adau17x1_micbias_voltage,
) -> c_int {
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;

    match micbias {
        ADAU17X1_MICBIAS_0_90_AVDD | ADAU17X1_MICBIAS_0_65_AVDD => {}
        _ => return -EINVAL,
    }

    regmap_write((*adau).regmap, ADAU17X1_MICBIAS, (micbias as u32) << 2)
}
// EXPORT_SYMBOL_GPL(adau17x1_set_micbias_voltage);

pub unsafe extern "C" fn adau17x1_precious_register(
    _dev: *mut device,
    reg: u32,
) -> bool {
    /* SigmaDSP parameter memory */
    if reg < 0x400 {
        return true;
    }

    false
}
// EXPORT_SYMBOL_GPL(adau17x1_precious_register);

pub unsafe extern "C" fn adau17x1_readable_register(
    _dev: *mut device,
    reg: u32,
) -> bool {
    /* SigmaDSP parameter memory */
    if reg < 0x400 {
        return true;
    }

    match reg {
        ADAU17X1_CLOCK_CONTROL
        | ADAU17X1_PLL_CONTROL
        | ADAU17X1_REC_POWER_MGMT
        | ADAU17X1_MICBIAS
        | ADAU17X1_SERIAL_PORT0
        | ADAU17X1_SERIAL_PORT1
        | ADAU17X1_CONVERTER0
        | ADAU17X1_CONVERTER1
        | ADAU17X1_LEFT_INPUT_DIGITAL_VOL
        | ADAU17X1_RIGHT_INPUT_DIGITAL_VOL
        | ADAU17X1_ADC_CONTROL
        | ADAU17X1_PLAY_POWER_MGMT
        | ADAU17X1_DAC_CONTROL0
        | ADAU17X1_DAC_CONTROL1
        | ADAU17X1_DAC_CONTROL2
        | ADAU17X1_SERIAL_PORT_PAD
        | ADAU17X1_CONTROL_PORT_PAD0
        | ADAU17X1_CONTROL_PORT_PAD1
        | ADAU17X1_DSP_SAMPLING_RATE
        | ADAU17X1_SERIAL_INPUT_ROUTE
        | ADAU17X1_SERIAL_OUTPUT_ROUTE
        | ADAU17X1_DSP_ENABLE
        | ADAU17X1_DSP_RUN
        | ADAU17X1_SERIAL_SAMPLING_RATE => true,
        _ => false,
    }
}
// EXPORT_SYMBOL_GPL(adau17x1_readable_register);

pub unsafe extern "C" fn adau17x1_volatile_register(
    _dev: *mut device,
    reg: u32,
) -> bool {
    /* SigmaDSP parameter and program memory */
    if reg < 0x4000 {
        return true;
    }

    match reg {
        /* The PLL register is 6 bytes long */
        ADAU17X1_PLL_CONTROL
        | ADAU17X1_PLL_CONTROL_PLUS_1
        | ADAU17X1_PLL_CONTROL_PLUS_2
        | ADAU17X1_PLL_CONTROL_PLUS_3
        | ADAU17X1_PLL_CONTROL_PLUS_4
        | ADAU17X1_PLL_CONTROL_PLUS_5 => true,
        _ => false,
    }
}
// EXPORT_SYMBOL_GPL(adau17x1_volatile_register);

unsafe fn adau17x1_setup_firmware(
    component: *mut snd_soc_component,
    rate: u32,
) -> c_int {
    let mut ret: c_int;
    let mut dspsr: c_int = 0;
    let mut dsp_run: c_int = 0;
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let dapm = snd_soc_component_to_dapm(component);

    /*
     * Check if sample rate is the same as before. If it is there is no
     * point in performing the below steps as the call to
     * sigmadsp_setup(...) will return directly when it finds the sample
     * rate to be the same as before. By checking this we can prevent an
     * audiable popping noise which occours when toggling DSP_RUN.
     */
    if (*(*adau).sigmadsp).current_samplerate == rate {
        return 0;
    }

    snd_soc_dapm_mutex_lock(dapm);

    ret = regmap_read((*adau).regmap, ADAU17X1_DSP_SAMPLING_RATE, &mut dspsr as *mut c_int as *mut u32);
    if ret != 0 {
        goto_err!(ret, dapm);
        return ret;
    }

    ret = regmap_read((*adau).regmap, ADAU17X1_DSP_RUN, &mut dsp_run as *mut c_int as *mut u32);
    if ret != 0 {
        goto_err!(ret, dapm);
        return ret;
    }

    regmap_write((*adau).regmap, ADAU17X1_DSP_ENABLE, 1);
    regmap_write((*adau).regmap, ADAU17X1_DSP_SAMPLING_RATE, 0xf);
    regmap_write((*adau).regmap, ADAU17X1_DSP_RUN, 0);

    ret = sigmadsp_setup((*adau).sigmadsp, rate);
    if ret != 0 {
        regmap_write((*adau).regmap, ADAU17X1_DSP_ENABLE, 0);
        snd_soc_dapm_mutex_unlock(dapm);
        return ret;
    }
    regmap_write((*adau).regmap, ADAU17X1_DSP_SAMPLING_RATE, dspsr as u32);
    regmap_write((*adau).regmap, ADAU17X1_DSP_RUN, dsp_run as u32);

    snd_soc_dapm_mutex_unlock(dapm);

    ret
}

pub unsafe extern "C" fn adau17x1_add_widgets(
    component: *mut snd_soc_component,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mut ret: c_int;

    ret = snd_soc_add_component_controls(component, adau17x1_controls.as_ptr(), adau17x1_controls.len());
    if ret != 0 {
        return ret;
    }
    ret = snd_soc_dapm_new_controls(dapm, adau17x1_dapm_widgets.as_ptr(), adau17x1_dapm_widgets.len());
    if ret != 0 {
        return ret;
    }

    if adau17x1_has_dsp(adau) {
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau17x1_dsp_dapm_widgets.as_ptr(),
            adau17x1_dsp_dapm_widgets.len(),
        );
        if ret != 0 {
            return ret;
        }

        if (*adau).sigmadsp.is_null() {
            return 0;
        }

        ret = sigmadsp_attach((*adau).sigmadsp, component);
        if ret != 0 {
            dev_err((*component).dev, "Failed to attach firmware: %d\n", ret);
            return ret;
        }
    }

    0
}
// EXPORT_SYMBOL_GPL(adau17x1_add_widgets);

pub unsafe extern "C" fn adau17x1_add_routes(
    component: *mut snd_soc_component,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mut ret: c_int;

    ret = snd_soc_dapm_add_routes(dapm, adau17x1_dapm_routes.as_ptr(), adau17x1_dapm_routes.len());
    if ret != 0 {
        return ret;
    }

    if adau17x1_has_dsp(adau) {
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau17x1_dsp_dapm_routes.as_ptr(),
            adau17x1_dsp_dapm_routes.len(),
        );
    } else {
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau17x1_no_dsp_dapm_routes.as_ptr(),
            adau17x1_no_dsp_dapm_routes.len(),
        );
    }

    if (*adau).clk_src != ADAU17X1_CLK_SRC_MCLK {
        snd_soc_dapm_add_routes(dapm, &adau17x1_dapm_pll_route, 1);
    }

    ret
}
// EXPORT_SYMBOL_GPL(adau17x1_add_routes);

pub unsafe extern "C" fn adau17x1_resume(component: *mut snd_soc_component) -> c_int {
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;

    if let Some(switch_mode) = (*adau).switch_mode {
        switch_mode((*component).dev);
    }

    regcache_sync((*adau).regmap);

    0
}
// EXPORT_SYMBOL_GPL(adau17x1_resume);

unsafe extern "C" fn adau17x1_safeload(
    sigmadsp: *mut sigmadsp,
    addr: u32,
    bytes: *const u8,
    len: usize,
) -> c_int {
    let mut buf: [u8; ADAU17X1_WORD_SIZE] = [0; ADAU17X1_WORD_SIZE];
    let mut data: [u8; ADAU17X1_SAFELOAD_DATA_SIZE] = [0; ADAU17X1_SAFELOAD_DATA_SIZE];
    let addr_offset: u32;
    let mut nbr_words: usize;
    let mut ret: c_int;

    /*
     * write data to safeload addresses. Check if len is not a multiple of
     * 4 bytes, if so we need to zero pad.
     */
    nbr_words = len / ADAU17X1_WORD_SIZE;
    if len - nbr_words * ADAU17X1_WORD_SIZE == 0 {
        ret = regmap_raw_write(
            (*sigmadsp).control_data,
            ADAU17X1_SAFELOAD_DATA,
            bytes as *const c_void,
            len,
        );
    } else {
        nbr_words += 1;
        core::ptr::write_bytes(data.as_mut_ptr(), 0, ADAU17X1_SAFELOAD_DATA_SIZE);
        core::ptr::copy_nonoverlapping(bytes, data.as_mut_ptr(), len);
        ret = regmap_raw_write(
            (*sigmadsp).control_data,
            ADAU17X1_SAFELOAD_DATA,
            data.as_ptr() as *const c_void,
            nbr_words * ADAU17X1_WORD_SIZE,
        );
    }

    if ret < 0 {
        return ret;
    }

    /* Write target address, target address is offset by 1 */
    addr_offset = addr.wrapping_sub(1);
    put_unaligned_be32(addr_offset, buf.as_mut_ptr());
    ret = regmap_raw_write(
        (*sigmadsp).control_data,
        ADAU17X1_SAFELOAD_TARGET_ADDRESS,
        buf.as_ptr() as *const c_void,
        ADAU17X1_WORD_SIZE,
    );
    if ret < 0 {
        return ret;
    }

    /* write nbr of words to trigger address */
    put_unaligned_be32(nbr_words as u32, buf.as_mut_ptr());
    ret = regmap_raw_write(
        (*sigmadsp).control_data,
        ADAU17X1_SAFELOAD_TRIGGER,
        buf.as_ptr() as *const c_void,
        ADAU17X1_WORD_SIZE,
    );
    if ret < 0 {
        return ret;
    }

    0
}

static adau17x1_sigmadsp_ops: sigmadsp_ops = sigmadsp_ops {
    safeload: Some(adau17x1_safeload),
};

pub unsafe extern "C" fn adau17x1_probe(
    dev: *mut device,
    regmap: *mut regmap,
    type_: adau17x1_type,
    switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    firmware_name: *const c_char,
) -> c_int {
    let adau: *mut adau;
    let mut ret: c_int;

    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    adau = devm_kzalloc(dev, core::mem::size_of::<adau>(), GFP_KERNEL) as *mut adau;
    if adau.is_null() {
        return -ENOMEM;
    }

    /* Clock is optional (for the driver) */
    (*adau).mclk = devm_clk_get_optional(dev, "mclk");
    if IS_ERR((*adau).mclk as *const c_void) {
        return PTR_ERR((*adau).mclk as *const c_void);
    }

    if !(*adau).mclk.is_null() {
        (*adau).clk_src = ADAU17X1_CLK_SRC_PLL_AUTO;

        /*
         * Any valid PLL output rate will work at this point, use one
         * that is likely to be chosen later as well. The register will
         * be written when the PLL is powered up for the first time.
         */
        ret = adau_calc_pll_cfg(
            clk_get_rate((*adau).mclk) as u32,
            48000 * 1024,
            (*adau).pll_regs.as_mut_ptr(),
        );
        if ret < 0 {
            return ret;
        }

        ret = clk_prepare_enable((*adau).mclk);
        if ret != 0 {
            return ret;
        }
    }

    (*adau).regmap = regmap;
    (*adau).switch_mode = switch_mode;
    (*adau).type_ = type_;

    dev_set_drvdata(dev, adau as *mut c_void);

    if !firmware_name.is_null() {
        if adau17x1_has_safeload(adau) {
            (*adau).sigmadsp = devm_sigmadsp_init_regmap(
                dev,
                regmap,
                &adau17x1_sigmadsp_ops,
                firmware_name,
            );
        } else {
            (*adau).sigmadsp = devm_sigmadsp_init_regmap(
                dev,
                regmap,
                core::ptr::null(),
                firmware_name,
            );
        }
        if IS_ERR((*adau).sigmadsp as *const c_void) {
            dev_warn(
                dev,
                "Could not find firmware file: %ld\n",
                PTR_ERR((*adau).sigmadsp as *const c_void),
            );
            (*adau).sigmadsp = core::ptr::null_mut();
        }
    }

    if let Some(switch_mode_fn) = switch_mode {
        switch_mode_fn(dev);
    }

    0
}
// EXPORT_SYMBOL_GPL(adau17x1_probe);

pub unsafe extern "C" fn adau17x1_remove(dev: *mut device) {
    let adau = dev_get_drvdata(dev) as *mut adau;

    clk_disable_unprepare((*adau).mclk);
}
// EXPORT_SYMBOL_GPL(adau17x1_remove);

// MODULE_DESCRIPTION("ASoC ADAU1X61/ADAU1X81 common code");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
