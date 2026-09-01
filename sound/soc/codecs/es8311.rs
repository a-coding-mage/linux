// SPDX-License-Identifier: GPL-2.0-only
/*
 * es8311.rs -- es8311 ALSA SoC audio driver
 *
 * Copyright (C) 2024 Matteo Martelli <matteomartelli3@gmail.com>
 *
 * Author: Matteo Martelli <matteomartelli3@gmail.com>
 */

// C dependencies: linux/array_size.h, sound/pcm.h, linux/clk.h, linux/i2c.h,
// linux/module.h, linux/regmap.h, sound/core.h, sound/pcm_params.h,
// sound/soc.h, sound/tlv.h, es8311.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const ES8311_NUM_RATES: usize = 10;
const ES8311_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const ES8311_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
struct es8311_priv {
    regmap: *mut regmap,
    mclk: *mut clk,
    mclk_freq: c_ulong,
    provider: bool,
    rates: [c_uint; ES8311_NUM_RATES],
    constraints: snd_pcm_hw_constraint_list,
}

static es8311_adc_vol_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-9550, 50, 0);
static es8311_pga_gain_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(0, 300, 0);
static es8311_adc_scale_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(0, 600, 0);

macro_rules! ES8311_DB_LRCK_STEPS {
    () => {
        c"0.25db/4LRCK".as_ptr(),
        c"0.25db/8LRCK".as_ptr(),
        c"0.25db/16LRCK".as_ptr(),
        c"0.25db/32LRCK".as_ptr(),
        c"0.25db/64LRCK".as_ptr(),
        c"0.25db/128LRCK".as_ptr(),
        c"0.25db/256LRCK".as_ptr(),
        c"0.25db/512LRCK".as_ptr(),
        c"0.25db/1024LRCK".as_ptr(),
        c"0.25db/2048LRCK".as_ptr(),
        c"0.25db/4096LRCK".as_ptr(),
        c"0.25db/8192LRCK".as_ptr(),
        c"0.25db/16384LRCK".as_ptr(),
        c"0.25db/32768LRCK".as_ptr(),
        c"0.25db/65536LRCK".as_ptr()
    };
}

static es8311_level_winsize_txt: [*const c_char; 16] = [
    c"0.25db/2LRCK".as_ptr(),
    ES8311_DB_LRCK_STEPS!(),
];

SOC_ENUM_SINGLE_DECL!(
    es8311_alc_winsize,
    ES8311_ADC4,
    ES8311_ADC4_ALC_WINSIZE_SHIFT,
    es8311_level_winsize_txt
);
static es8311_level_tlv: [c_uint; 40] = DECLARE_TLV_DB_RANGE!(
    0,
    1,
    TLV_DB_SCALE_ITEM!(-3010, 600, 0),
    2,
    3,
    TLV_DB_SCALE_ITEM!(-2060, 250, 0),
    4,
    5,
    TLV_DB_SCALE_ITEM!(-1610, 160, 0),
    6,
    7,
    TLV_DB_SCALE_ITEM!(-1320, 120, 0),
    8,
    9,
    TLV_DB_SCALE_ITEM!(-1100, 90, 0),
    10,
    11,
    TLV_DB_SCALE_ITEM!(-930, 80, 0),
    12,
    15,
    TLV_DB_SCALE_ITEM!(-780, 60, 0)
);

static es8311_ramprate_txt: [*const c_char; 16] = [
    c"Disabled".as_ptr(),
    ES8311_DB_LRCK_STEPS!(),
];
SOC_ENUM_SINGLE_DECL!(
    es8311_adc_ramprate,
    ES8311_ADC1,
    ES8311_ADC1_RAMPRATE_SHIFT,
    es8311_ramprate_txt
);

static es8311_automute_winsize_txt: [*const c_char; 16] = [
    c"2048 samples".as_ptr(),
    c"4096 samples".as_ptr(),
    c"6144 samples".as_ptr(),
    c"8192 samples".as_ptr(),
    c"10240 samples".as_ptr(),
    c"12288 samples".as_ptr(),
    c"14336 samples".as_ptr(),
    c"16384 samples".as_ptr(),
    c"18432 samples".as_ptr(),
    c"20480 samples".as_ptr(),
    c"22528 samples".as_ptr(),
    c"24576 samples".as_ptr(),
    c"26624 samples".as_ptr(),
    c"28672 samples".as_ptr(),
    c"30720 samples".as_ptr(),
    c"32768 samples".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(
    es8311_automute_winsize,
    ES8311_ADC6,
    ES8311_ADC6_AUTOMUTE_WS_SHIFT,
    es8311_automute_winsize_txt
);
static es8311_automute_ng_tlv: [c_uint; 12] = DECLARE_TLV_DB_RANGE!(
    0,
    7,
    TLV_DB_SCALE_ITEM!(-9600, 600, 0),
    8,
    15,
    TLV_DB_SCALE_ITEM!(-5100, 300, 0)
);
static es8311_automute_vol_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-2800, 400, 0);

static es8311_dac_vol_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-9550, 50, 0);
SOC_ENUM_SINGLE_DECL!(
    es8311_drc_winsize,
    ES8311_DAC4,
    ES8311_DAC4_DRC_WINSIZE_SHIFT,
    es8311_level_winsize_txt
);
SOC_ENUM_SINGLE_DECL!(
    es8311_dac_ramprate,
    ES8311_DAC6,
    ES8311_DAC6_RAMPRATE_SHIFT,
    es8311_ramprate_txt
);

static es8311_out_mode_txt: [*const c_char; 2] = [c"Lineout".as_ptr(), c"Headphones".as_ptr()];
SOC_ENUM_SINGLE_DECL!(
    es8311_out_mode,
    ES8311_SYS9,
    ES8311_SYS9_HPSW_SHIFT,
    es8311_out_mode_txt
);

static es8311_snd_controls: [snd_kcontrol_new; 24] = [
    /* Capture path */
    SOC_SINGLE_TLV!(
        c"PGA Capture Volume".as_ptr(),
        ES8311_SYS10,
        ES8311_SYS10_PGAGAIN_SHIFT,
        ES8311_SYS10_PGAGAIN_MAX,
        0,
        es8311_pga_gain_tlv
    ),
    SOC_SINGLE!(
        c"ADC Polarity Invert Capture Switch".as_ptr(),
        ES8311_ADC2,
        ES8311_ADC2_INV_SHIFT,
        1,
        0
    ),
    SOC_SINGLE_TLV!(
        c"ADC Scale Capture Volume".as_ptr(),
        ES8311_ADC2,
        ES8311_ADC2_SCALE_SHIFT,
        ES8311_ADC2_SCALE_MAX,
        0,
        es8311_adc_scale_tlv
    ),
    SOC_SINGLE_TLV!(
        c"ADC Capture Volume".as_ptr(),
        ES8311_ADC3,
        ES8311_ADC3_VOLUME_SHIFT,
        ES8311_ADC3_VOLUME_MAX,
        0,
        es8311_adc_vol_tlv
    ),
    SOC_ENUM!(c"ADC Capture Ramp Rate".as_ptr(), es8311_adc_ramprate),
    SOC_SINGLE!(
        c"ADC Automute Capture Switch".as_ptr(),
        ES8311_ADC4,
        ES8311_ADC4_AUTOMUTE_EN_SHIFT,
        1,
        0
    ),
    SOC_ENUM!(
        c"ADC Automute Capture Winsize".as_ptr(),
        es8311_automute_winsize
    ),
    SOC_SINGLE_TLV!(
        c"ADC Automute Noise Gate Capture Volume".as_ptr(),
        ES8311_ADC6,
        ES8311_ADC6_AUTOMUTE_NG_SHIFT,
        ES8311_ADC6_AUTOMUTE_NG_MAX,
        0,
        es8311_automute_ng_tlv
    ),
    SOC_SINGLE_TLV!(
        c"ADC Automute Capture Volume".as_ptr(),
        ES8311_ADC7,
        ES8311_ADC7_AUTOMUTE_VOL_SHIFT,
        ES8311_ADC7_AUTOMUTE_VOL_MAX,
        0,
        es8311_automute_vol_tlv
    ),
    SOC_SINGLE!(
        c"ADC HPF Capture Switch".as_ptr(),
        ES8311_ADC8,
        ES8311_ADC8_HPF_SHIFT,
        1,
        0
    ),
    SOC_SINGLE!(
        c"ADC EQ Capture Switch".as_ptr(),
        ES8311_ADC8,
        ES8311_ADC8_EQBYPASS_SHIFT,
        1,
        1
    ),
    SOC_SINGLE!(
        c"ALC Capture Switch".as_ptr(),
        ES8311_ADC4,
        ES8311_ADC4_ALC_EN_SHIFT,
        1,
        0
    ),
    SOC_SINGLE_TLV!(
        c"ALC Capture Max Volume".as_ptr(),
        ES8311_ADC5,
        ES8311_ADC5_ALC_MAXLEVEL_SHIFT,
        ES8311_ADC5_ALC_MAXLEVEL_MAX,
        0,
        es8311_level_tlv
    ),
    SOC_SINGLE_TLV!(
        c"ALC Capture Min Volume".as_ptr(),
        ES8311_ADC5,
        ES8311_ADC5_ALC_MINLEVEL_SHIFT,
        ES8311_ADC5_ALC_MINLEVEL_MAX,
        0,
        es8311_level_tlv
    ),
    SOC_ENUM!(c"ALC Capture Winsize".as_ptr(), es8311_alc_winsize),
    /* Playback path */
    SOC_SINGLE_TLV!(
        c"DAC Playback Volume".as_ptr(),
        ES8311_DAC2,
        0,
        ES8311_DAC2_VOLUME_MAX,
        0,
        es8311_dac_vol_tlv
    ),
    SOC_SINGLE!(
        c"DRC Playback Switch".as_ptr(),
        ES8311_DAC4,
        ES8311_DAC4_DRC_EN_SHIFT,
        1,
        0
    ),
    SOC_SINGLE_TLV!(
        c"DRC Playback Max Volume".as_ptr(),
        ES8311_DAC5,
        ES8311_DAC5_DRC_MAXLEVEL_SHIFT,
        ES8311_DAC5_DRC_MAXLEVEL_MAX,
        0,
        es8311_level_tlv
    ),
    SOC_SINGLE_TLV!(
        c"DRC Playback Min Volume".as_ptr(),
        ES8311_DAC5,
        ES8311_DAC5_DRC_MINLEVEL_SHIFT,
        ES8311_DAC5_DRC_MINLEVEL_MAX,
        0,
        es8311_level_tlv
    ),
    SOC_ENUM!(c"DRC Playback Winsize".as_ptr(), es8311_drc_winsize),
    SOC_ENUM!(c"DAC Playback Ramp Rate".as_ptr(), es8311_dac_ramprate),
    SOC_SINGLE!(
        c"DAC EQ Playback Switch".as_ptr(),
        ES8311_DAC6,
        ES8311_DAC6_EQBYPASS_SHIFT,
        1,
        1
    ),
    SOC_ENUM!(c"Output Mode".as_ptr(), es8311_out_mode),
];

static es8311_diff_src_txt: [*const c_char; 2] =
    [c"Disabled".as_ptr(), c"MIC1P-MIC1N".as_ptr()];
SOC_ENUM_SINGLE_DECL!(
    es8311_diff_src_enum,
    ES8311_SYS10,
    ES8311_SYS10_LINESEL_SHIFT,
    es8311_diff_src_txt
);
static es8311_diff_src_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"Differential Source".as_ptr(), es8311_diff_src_enum);

static es8311_dmic_src_txt: [*const c_char; 2] =
    [c"Disabled".as_ptr(), c"DMIC from MIC1P".as_ptr()];
SOC_ENUM_SINGLE_DECL!(
    es8311_dmic_src_enum,
    ES8311_SYS10,
    ES8311_SYS10_DMIC_ON_SHIFT,
    es8311_dmic_src_txt
);
static es8311_dmic_src_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"Digital Mic Source".as_ptr(), es8311_dmic_src_enum);

static es8311_aif1tx_src_txt: [*const c_char; 7] = [
    c"ADC + ADC".as_ptr(),
    c"ADC + 0".as_ptr(),
    c"0 + ADC".as_ptr(),
    c"0 + 0".as_ptr(),
    c"DACL + ADC".as_ptr(),
    c"ADC + DACR".as_ptr(),
    c"DACL + DACR".as_ptr(),
];
SOC_ENUM_SINGLE_DECL!(
    es8311_aif1tx_src_enum,
    ES8311_GPIO,
    ES8311_GPIO_ADCDAT_SEL_SHIFT,
    es8311_aif1tx_src_txt
);
static es8311_aif1tx_src_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"AIF1TX Source".as_ptr(), es8311_aif1tx_src_enum);

static es8311_dac_src_txt: [*const c_char; 2] = [c"Left".as_ptr(), c"Right".as_ptr()];
SOC_ENUM_SINGLE_DECL!(
    es8311_dac_src_enum,
    ES8311_SDP_IN,
    ES8311_SDP_IN_SEL_SHIFT,
    es8311_dac_src_txt
);
static es8311_dac_src_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"Mono DAC Source".as_ptr(), es8311_dac_src_enum);

static es8311_dapm_widgets: [snd_soc_dapm_widget; 22] = [
    SND_SOC_DAPM_SUPPLY!(c"Bias".as_ptr(), ES8311_SYS3, ES8311_SYS3_PDN_IBIASGEN_SHIFT, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"Analog power".as_ptr(), ES8311_SYS3, ES8311_SYS3_PDN_ANA_SHIFT, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"Vref".as_ptr(), ES8311_SYS3, ES8311_SYS3_PDN_VREF_SHIFT, 1, ptr::null_mut(), 0),
    /* Capture path */
    SND_SOC_DAPM_INPUT!(c"DMIC".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"MIC1".as_ptr()),
    SND_SOC_DAPM_MUX!(c"Differential Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &es8311_diff_src_mux),
    SND_SOC_DAPM_SUPPLY!(c"ADC Bias Gen".as_ptr(), ES8311_SYS3, ES8311_SYS3_PDN_ADCBIASGEN_SHIFT, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"ADC Vref Gen".as_ptr(), ES8311_SYS3, ES8311_SYS3_PDN_ADCVREFGEN_SHIFT, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"ADC Clock".as_ptr(), ES8311_CLKMGR1, ES8311_CLKMGR1_CLKADC_ON_SHIFT, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"ADC Analog Clock".as_ptr(), ES8311_CLKMGR1, ES8311_CLKMGR1_ANACLKADC_ON_SHIFT, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"PGA".as_ptr(), ES8311_SYS4, ES8311_SYS4_PDN_PGA_SHIFT, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_ADC!(c"Mono ADC".as_ptr(), ptr::null(), ES8311_SYS4, ES8311_SYS4_PDN_MOD_SHIFT, 1),
    SND_SOC_DAPM_MUX!(c"Digital Mic Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &es8311_dmic_src_mux),
    SND_SOC_DAPM_MUX!(c"AIF1TX Source Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &es8311_aif1tx_src_mux),
    SND_SOC_DAPM_AIF_OUT!(c"AIF1TX".as_ptr(), c"AIF1 Capture".as_ptr(), 0, ES8311_SDP_OUT, ES8311_SDP_MUTE_SHIFT, 1),
    /* Playback path */
    SND_SOC_DAPM_AIF_IN!(c"AIF1RX".as_ptr(), c"AIF1 Playback".as_ptr(), 0, ES8311_SDP_IN, ES8311_SDP_MUTE_SHIFT, 1),
    SND_SOC_DAPM_MUX!(c"Mono DAC Source Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &es8311_dac_src_mux),
    SND_SOC_DAPM_DAC!(c"Mono DAC".as_ptr(), ptr::null(), ES8311_SYS8, ES8311_SYS8_PDN_DAC_SHIFT, 1),
    SND_SOC_DAPM_SUPPLY!(c"DAC Clock".as_ptr(), ES8311_CLKMGR1, ES8311_CLKMGR1_CLKDAC_ON_SHIFT, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"DAC Analog Clock".as_ptr(), ES8311_CLKMGR1, ES8311_CLKMGR1_ANACLKDAC_ON_SHIFT, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(c"DAC Vref Gen".as_ptr(), ES8311_SYS3, ES8311_SYS3_PDN_DACVREFGEN_SHIFT, 1, ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT!(c"OUT".as_ptr()),
];

static es8311_dapm_routes: [snd_soc_dapm_route; 30] = [
    /* Capture Path */
    snd_soc_dapm_route { sink: c"MIC1".as_ptr(), control: ptr::null(), source: c"Bias".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1".as_ptr(), control: ptr::null(), source: c"Analog power".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1".as_ptr(), control: ptr::null(), source: c"Vref".as_ptr() },
    snd_soc_dapm_route { sink: c"Differential Mux".as_ptr(), control: c"MIC1P-MIC1N".as_ptr(), source: c"MIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA".as_ptr(), control: ptr::null(), source: c"Differential Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono ADC".as_ptr(), control: ptr::null(), source: c"PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono ADC".as_ptr(), control: ptr::null(), source: c"ADC Bias Gen".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono ADC".as_ptr(), control: ptr::null(), source: c"ADC Vref Gen".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono ADC".as_ptr(), control: ptr::null(), source: c"ADC Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono ADC".as_ptr(), control: ptr::null(), source: c"ADC Analog Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital Mic Mux".as_ptr(), control: c"Disabled".as_ptr(), source: c"Mono ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital Mic Mux".as_ptr(), control: c"DMIC from MIC1P".as_ptr(), source: c"DMIC".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX Source Mux".as_ptr(), control: c"ADC + ADC".as_ptr(), source: c"Digital Mic Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX Source Mux".as_ptr(), control: c"ADC + 0".as_ptr(), source: c"Digital Mic Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX Source Mux".as_ptr(), control: c"0 + ADC".as_ptr(), source: c"Digital Mic Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX Source Mux".as_ptr(), control: c"DACL + ADC".as_ptr(), source: c"Digital Mic Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX Source Mux".as_ptr(), control: c"ADC + DACR".as_ptr(), source: c"Digital Mic Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: ptr::null(), source: c"AIF1TX Source Mux".as_ptr() },
    /* Playback Path */
    snd_soc_dapm_route { sink: c"Mono DAC Source Mux".as_ptr(), control: c"Left".as_ptr(), source: c"AIF1RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono DAC Source Mux".as_ptr(), control: c"Right".as_ptr(), source: c"AIF1RX".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono DAC".as_ptr(), control: ptr::null(), source: c"Mono DAC Source Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono DAC".as_ptr(), control: ptr::null(), source: c"DAC Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono DAC".as_ptr(), control: ptr::null(), source: c"DAC Analog Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"Mono DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"Bias".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"Analog power".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"Vref".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"DAC Vref Gen".as_ptr() },
];

/* Bit clock divider values:
 * from 1 to 20: the register takes the div value - 1
 * above 20: the register takes the corresponding idx of the div value
 *           in the following table + 20
 */
const ES8311_BCLK_DIV_IDX_OFFSET: c_uint = 20;
static es8311_bclk_divs: [c_uint; 12] = [22, 24, 25, 30, 32, 33, 34, 36, 44, 48, 66, 72];

#[repr(C)]
#[derive(Copy, Clone)]
struct es8311_mclk_coeff {
    rate: c_uint,
    mclk: c_uint,
    div: c_uint,
    mult: c_uint,
    div_adc_dac: c_uint,
}

const ES8311_MCLK_MAX_FREQ: c_uint = 49_200_000;

/* Coefficients for common master clock frequencies based on clock table from
 * documentation. Limited to have a ratio of adc (or dac) clock to lrclk equal
 * to 256. This to keep the default adc and dac oversampling and adc scale
 * settings. Internal mclk dividers and multipliers are dynamically adjusted to
 * support, respectively, multiples (up to x8) and factors (/2,4,8) of listed
 * mclks frequencies (see es8311_cmp_adj_mclk_coeff).
 * All rates are supported when mclk/rate ratio is 32, 64, 128, 256, 384 or 512
 * (upper limit due to max mclk freq of 49.2MHz).
 */
static es8311_mclk_coeffs: [es8311_mclk_coeff; 23] = [
    es8311_mclk_coeff { rate: 8000, mclk: 2048000, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 8000, mclk: 6144000, div: 3, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 8000, mclk: 18432000, div: 3, mult: 1, div_adc_dac: 3 },
    es8311_mclk_coeff { rate: 11025, mclk: 2822400, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 11025, mclk: 8467200, div: 3, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 16000, mclk: 4096000, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 16000, mclk: 12288000, div: 3, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 16000, mclk: 18432000, div: 3, mult: 2, div_adc_dac: 3 },
    es8311_mclk_coeff { rate: 22050, mclk: 5644800, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 22050, mclk: 16934400, div: 3, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 32000, mclk: 8192000, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 32000, mclk: 12288000, div: 3, mult: 2, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 32000, mclk: 18432000, div: 3, mult: 4, div_adc_dac: 3 },
    es8311_mclk_coeff { rate: 44100, mclk: 11289600, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 44100, mclk: 33868800, div: 3, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 48000, mclk: 12288000, div: 1, mult: 1, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 48000, mclk: 18432000, div: 3, mult: 2, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 64000, mclk: 8192000, div: 1, mult: 2, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 64000, mclk: 12288000, div: 3, mult: 4, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 88200, mclk: 11289600, div: 1, mult: 2, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 88200, mclk: 33868800, div: 3, mult: 2, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 96000, mclk: 12288000, div: 1, mult: 2, div_adc_dac: 1 },
    es8311_mclk_coeff { rate: 96000, mclk: 18432000, div: 3, mult: 4, div_adc_dac: 1 },
];

/* Compare coeff with provided mclk_freq and adjust it if needed.
 * If frequencies match, return 0 and the unaltered coeff copy into out_coeff.
 * If mclk_freq is a valid multiple or factor of coeff mclk freq, return 0 and
 * the adjusted coeff copy into out_coeff.
 * Return -EINVAL otherwise.
 */
unsafe fn es8311_cmp_adj_mclk_coeff(
    mclk_freq: c_uint,
    coeff: *const es8311_mclk_coeff,
    out_coeff: *mut es8311_mclk_coeff,
) -> c_int {
    if WARN_ON_ONCE!(coeff.is_null()) {
        return -EINVAL;
    }

    let mut div: c_uint = (*coeff).div;
    let mut mult: c_uint = (*coeff).mult;
    let mut match_: bool = false;

    if (*coeff).mclk == mclk_freq {
        match_ = true;
    } else if mclk_freq % (*coeff).mclk == 0 {
        div = mclk_freq / (*coeff).mclk;
        div *= (*coeff).div;
        if div <= 8 {
            match_ = true;
        }
    } else if (*coeff).mclk % mclk_freq == 0 {
        mult = (*coeff).mclk / mclk_freq;
        if mult == 2 || mult == 4 || mult == 8 {
            mult *= (*coeff).mult;
            if mult <= 8 {
                match_ = true;
            }
        }
    }
    if !match_ {
        return -EINVAL;
    }
    if !out_coeff.is_null() {
        *out_coeff = *coeff;
        (*out_coeff).div = div;
        (*out_coeff).mult = mult;
    }
    0
}

unsafe fn es8311_get_mclk_coeff(
    mclk_freq: c_uint,
    rate: c_uint,
    out_coeff: *mut es8311_mclk_coeff,
) -> c_int {
    for i in 0..es8311_mclk_coeffs.len() {
        let coeff = &es8311_mclk_coeffs[i] as *const es8311_mclk_coeff;

        if (*coeff).rate != rate {
            continue;
        }

        let ret = es8311_cmp_adj_mclk_coeff(mclk_freq, coeff, out_coeff);
        if ret == 0 {
            return 0;
        }
    }
    -EINVAL
}

unsafe fn es8311_set_sysclk_constraints(mclk_freq: c_uint, es8311: *mut es8311_priv) {
    let mut count: c_uint = 0;

    let mut i = 0usize;
    while i < es8311_mclk_coeffs.len() && (count as usize) < (*es8311).rates.len() {
        let coeff = &es8311_mclk_coeffs[i] as *const es8311_mclk_coeff;

        if count > 0 && (*coeff).rate == (*es8311).rates[count as usize - 1] {
            i += 1;
            continue;
        }

        let ret = es8311_cmp_adj_mclk_coeff(mclk_freq, coeff, ptr::null_mut());
        if ret == 0 {
            (*es8311).rates[count as usize] = (*coeff).rate;
            count += 1;
        }
        i += 1;
    }
    if count != 0 {
        (*es8311).constraints.list = (*es8311).rates.as_mut_ptr();
        (*es8311).constraints.count = count;
    }
}

unsafe extern "C" fn es8311_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let es8311: *mut es8311_priv = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        let mask: c_uint = ES8311_DAC1_DAC_DSMMUTE | ES8311_DAC1_DAC_DEMMUTE;
        let val: c_uint = if mute != 0 { mask } else { 0 };

        regmap_update_bits((*es8311).regmap, ES8311_DAC1, mask, val);
    }

    0
}

unsafe extern "C" fn es8311_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let es8311: *mut es8311_priv = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    if !(*es8311).constraints.list.is_null() {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &mut (*es8311).constraints,
        );
    }

    0
}

unsafe extern "C" fn es8311_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let es8311: *mut es8311_priv = snd_soc_component_get_drvdata(component) as *mut es8311_priv;
    let wl: c_uint;
    let par_width: c_int = params_width(params);

    match par_width {
        16 => wl = ES8311_SDP_WL_16,
        18 => wl = ES8311_SDP_WL_18,
        20 => wl = ES8311_SDP_WL_20,
        24 => wl = ES8311_SDP_WL_24,
        32 => wl = ES8311_SDP_WL_32,
        _ => return -EINVAL,
    }
    let width: c_uint = par_width as c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_component_update_bits(
            component,
            ES8311_SDP_IN,
            ES8311_SDP_WL_MASK,
            wl << ES8311_SDP_WL_SHIFT,
        );
    } else {
        snd_soc_component_update_bits(
            component,
            ES8311_SDP_OUT,
            ES8311_SDP_WL_MASK,
            wl << ES8311_SDP_WL_SHIFT,
        );
    }

    if (*es8311).mclk_freq > ES8311_MCLK_MAX_FREQ as c_ulong {
        dev_err!(
            (*component).dev,
            c"mclk frequency %lu too high\n".as_ptr(),
            (*es8311).mclk_freq
        );
        return -EINVAL;
    }

    let mut mclk_freq: c_uint = (*es8311).mclk_freq as c_uint;
    let rate: c_uint = params_rate(params);
    let mut clkmgr: c_uint = ES8311_CLKMGR1_MCLK_ON;

    if mclk_freq == 0 {
        if (*es8311).provider {
            dev_err!(
                (*component).dev,
                c"mclk not configured, cannot run as master\n".as_ptr()
            );
            return -EINVAL;
        }
        dev_dbg!(
            (*component).dev,
            c"mclk not configured, use bclk as internal mclk\n".as_ptr()
        );

        clkmgr = ES8311_CLKMGR1_MCLK_SEL;

        mclk_freq = rate * width * 2;
    }

    let mut coeff = es8311_mclk_coeff {
        rate: 0,
        mclk: 0,
        div: 0,
        mult: 0,
        div_adc_dac: 0,
    };
    let ret: c_int = es8311_get_mclk_coeff(mclk_freq, rate, &mut coeff);
    if ret != 0 {
        dev_err!(
            (*component).dev,
            c"unable to find mclk coefficient\n".as_ptr()
        );
        return ret;
    }

    let mut mask: c_uint =
        ES8311_CLKMGR1_MCLK_SEL | ES8311_CLKMGR1_MCLK_ON | ES8311_CLKMGR1_BCLK_ON;

    clkmgr |= ES8311_CLKMGR1_BCLK_ON;
    snd_soc_component_update_bits(component, ES8311_CLKMGR1, mask, clkmgr);

    if WARN_ON_ONCE!(
        coeff.div == 0 || coeff.div > 8 || coeff.div_adc_dac == 0 || coeff.div_adc_dac > 8
    ) {
        return -EINVAL;
    }

    let mult: c_uint;

    match coeff.mult {
        1 => mult = 0,
        2 => mult = 1,
        4 => mult = 2,
        8 => mult = 3,
        _ => {
            WARN_ON_ONCE!(true);
            return -EINVAL;
        }
    }

    mask = ES8311_CLKMGR2_DIV_PRE_MASK | ES8311_CLKMGR2_MULT_PRE_MASK;
    clkmgr = ((coeff.div - 1) << ES8311_CLKMGR2_DIV_PRE_SHIFT)
        | (mult << ES8311_CLKMGR2_MULT_PRE_SHIFT);
    snd_soc_component_update_bits(component, ES8311_CLKMGR2, mask, clkmgr);

    mask = ES8311_CLKMGR5_ADC_DIV_MASK | ES8311_CLKMGR5_DAC_DIV_MASK;
    clkmgr = ((coeff.div_adc_dac - 1) << ES8311_CLKMGR5_ADC_DIV_SHIFT)
        | ((coeff.div_adc_dac - 1) << ES8311_CLKMGR5_DAC_DIV_SHIFT);
    snd_soc_component_update_bits(component, ES8311_CLKMGR5, mask, clkmgr);

    if (*es8311).provider {
        let div_lrclk: c_uint = mclk_freq / rate;

        if WARN_ON_ONCE!(div_lrclk == 0 || div_lrclk > ES8311_CLKMGR_LRCLK_DIV_MAX + 1) {
            return -EINVAL;
        }

        mask = ES8311_CLKMGR7_LRCLK_DIV_H_MASK;
        clkmgr = (div_lrclk - 1) >> 8;
        snd_soc_component_update_bits(component, ES8311_CLKMGR7, mask, clkmgr);
        clkmgr = (div_lrclk - 1) & 0xFF;
        snd_soc_component_write(component, ES8311_CLKMGR8, clkmgr);

        if div_lrclk % (2 * width) != 0 {
            dev_err!(
                (*component).dev,
                c"unable to divide mclk %u to generate bclk\n".as_ptr(),
                mclk_freq
            );
            return -EINVAL;
        }

        let div_bclk: c_uint = div_lrclk / (2 * width);

        mask = ES8311_CLKMGR6_DIV_BCLK_MASK;
        if div_bclk <= ES8311_BCLK_DIV_IDX_OFFSET {
            clkmgr = div_bclk - 1;
        } else {
            let mut i: usize = 0;

            while i < es8311_bclk_divs.len() {
                if es8311_bclk_divs[i] == div_bclk {
                    break;
                }
                i += 1;
            }
            if i == es8311_bclk_divs.len() {
                dev_err!(
                    (*component).dev,
                    c"bclk divider %u not supported\n".as_ptr(),
                    div_bclk
                );
                return -EINVAL;
            }

            clkmgr = i as c_uint + ES8311_BCLK_DIV_IDX_OFFSET;
        }
        snd_soc_component_update_bits(component, ES8311_CLKMGR6, mask, clkmgr);
    }

    0
}

unsafe extern "C" fn es8311_set_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let es8311: *mut es8311_priv = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    if freq > ES8311_MCLK_MAX_FREQ {
        dev_err!(
            (*component).dev,
            c"invalid frequency %u: too high\n".as_ptr(),
            freq
        );
        return -EINVAL;
    }

    if (*es8311).mclk_freq == freq as c_ulong {
        return 0;
    }

    (*es8311).mclk_freq = freq as c_ulong;
    (*es8311).constraints.list = ptr::null_mut();
    (*es8311).constraints.count = 0;

    if freq == 0 {
        return 0;
    }

    let ret: c_int = clk_set_rate((*es8311).mclk, freq as c_ulong);
    if ret != 0 {
        dev_err!((*component).dev, c"unable to set mclk rate\n".as_ptr());
        return ret;
    }

    es8311_set_sysclk_constraints(freq, es8311);

    ret
}

unsafe extern "C" fn es8311_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let es8311: *mut es8311_priv = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            /* Master mode */
            (*es8311).provider = true;

            snd_soc_component_update_bits(
                component,
                ES8311_RESET,
                ES8311_RESET_MSC,
                ES8311_RESET_MSC,
            );
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            /* Slave mode */
            (*es8311).provider = false;
            snd_soc_component_update_bits(component, ES8311_RESET, ES8311_RESET_MSC, 0);
        }
        _ => return -EINVAL,
    }

    let mut sdp: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            sdp |= ES8311_SDP_FMT_I2S;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            sdp |= ES8311_SDP_FMT_LEFT_J;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            dev_err!(
                (*component).dev,
                c"right justified mode not supported\n".as_ptr()
            );
            return -EINVAL;
        }
        SND_SOC_DAIFMT_DSP_B => {
            sdp |= ES8311_SDP_LRP;
            sdp |= ES8311_SDP_FMT_DSP;
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => {}
                _ => {
                    dev_err!(
                        (*component).dev,
                        c"inverted fsync not supported in dsp mode\n".as_ptr()
                    );
                    return -EINVAL;
                }
            }
        }
        SND_SOC_DAIFMT_DSP_A => {
            sdp |= ES8311_SDP_FMT_DSP;
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => {}
                _ => {
                    dev_err!(
                        (*component).dev,
                        c"inverted fsync not supported in dsp mode\n".as_ptr()
                    );
                    return -EINVAL;
                }
            }
        }
        _ => return -EINVAL,
    }

    let mut clkmgr: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => {
            sdp |= ES8311_SDP_LRP;
        }
        SND_SOC_DAIFMT_IB_NF => {
            clkmgr |= ES8311_CLKMGR6_BCLK_INV;
        }
        SND_SOC_DAIFMT_IB_IF => {
            clkmgr |= ES8311_CLKMGR6_BCLK_INV;
            sdp |= ES8311_SDP_LRP;
        }
        _ => return -EINVAL,
    }

    let mut mask: c_uint = ES8311_CLKMGR6_BCLK_INV;

    snd_soc_component_update_bits(component, ES8311_CLKMGR6, mask, clkmgr);

    mask = ES8311_SDP_FMT_MASK | ES8311_SDP_LRP;
    snd_soc_component_update_bits(component, ES8311_SDP_IN, mask, sdp);
    snd_soc_component_update_bits(component, ES8311_SDP_OUT, mask, sdp);

    0
}

unsafe extern "C" fn es8311_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let es8311: *mut es8311_priv = snd_soc_component_get_drvdata(component) as *mut es8311_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = clk_prepare_enable((*es8311).mclk);
                if ret != 0 {
                    dev_err!((*component).dev, c"unable to prepare mclk\n".as_ptr());
                    return ret;
                }

                ret = snd_soc_component_update_bits(
                    component,
                    ES8311_SYS3,
                    ES8311_SYS3_PDN_VMIDSEL_MASK,
                    ES8311_SYS3_PDN_VMIDSEL_STARTUP_NORMAL_SPEED,
                );
                if ret < 0 {
                    clk_disable_unprepare((*es8311).mclk);
                    return ret;
                }
            }
        }
        SND_SOC_BIAS_OFF => {
            clk_disable_unprepare((*es8311).mclk);
            snd_soc_component_update_bits(
                component,
                ES8311_SYS3,
                ES8311_SYS3_PDN_VMIDSEL_MASK,
                ES8311_SYS3_PDN_VMIDSEL_POWER_DOWN,
            );
        }
        _ => {}
    }
    0
}

static es8311_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(es8311_startup),
    hw_params: Some(es8311_hw_params),
    mute_stream: Some(es8311_mute),
    set_sysclk: Some(es8311_set_sysclk),
    set_fmt: Some(es8311_set_dai_fmt),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut es8311_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"es8311".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"AIF1 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: ES8311_RATES,
        formats: ES8311_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"AIF1 Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: ES8311_RATES,
        formats: ES8311_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &es8311_dai_ops,
    symmetric_rate: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn es8311_reset(component: *mut snd_soc_component, reset: bool) {
    /* Reset procedure:
     * (1) power down state machine and reset codec blocks then,
     * (2) after a short delay, power up state machine and leave reset mode.
     * Specific delay is not documented, using the same as es8316.
     */
    let mask: c_uint = ES8311_RESET_CSM_ON | ES8311_RESET_RST_MASK;

    if reset {
        /* Enter reset mode */
        snd_soc_component_update_bits(component, ES8311_RESET, mask, ES8311_RESET_RST_MASK);
    } else {
        /* Leave reset mode */
        usleep_range(5000, 5500);
        snd_soc_component_update_bits(component, ES8311_RESET, mask, ES8311_RESET_CSM_ON);
    }
}

unsafe extern "C" fn es8311_suspend(component: *mut snd_soc_component) -> c_int {
    let es8311: *mut es8311_priv;

    es8311 = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    es8311_reset(component, true);

    regcache_cache_only((*es8311).regmap, true);
    regcache_mark_dirty((*es8311).regmap);

    0
}

unsafe extern "C" fn es8311_resume(component: *mut snd_soc_component) -> c_int {
    let es8311: *mut es8311_priv;
    let ret: c_int;

    es8311 = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    es8311_reset(component, false);

    regcache_cache_only((*es8311).regmap, false);
    ret = regcache_sync((*es8311).regmap);
    if ret != 0 {
        dev_err!((*component).dev, c"unable to sync regcache\n".as_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn es8311_component_probe(component: *mut snd_soc_component) -> c_int {
    let es8311: *mut es8311_priv;

    es8311 = snd_soc_component_get_drvdata(component) as *mut es8311_priv;

    (*es8311).mclk = devm_clk_get_optional((*component).dev, c"mclk".as_ptr());
    if IS_ERR((*es8311).mclk as *const c_void) {
        dev_err!((*component).dev, c"invalid mclk\n".as_ptr());
        return PTR_ERR((*es8311).mclk as *const c_void);
    }

    (*es8311).mclk_freq = clk_get_rate((*es8311).mclk);
    if (*es8311).mclk_freq > 0 && (*es8311).mclk_freq < ES8311_MCLK_MAX_FREQ as c_ulong {
        es8311_set_sysclk_constraints((*es8311).mclk_freq as c_uint, es8311);
    }

    es8311_reset(component, true);
    es8311_reset(component, false);

    /* Set minimal power up time */
    snd_soc_component_write(component, ES8311_SYS1, 0);
    snd_soc_component_write(component, ES8311_SYS2, 0);

    0
}

static es8311_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: ES8311_REG_MAX,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
    ..unsafe { core::mem::zeroed() }
};

static es8311_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es8311_component_probe),
    suspend: Some(es8311_suspend),
    resume: Some(es8311_resume),
    set_bias_level: Some(es8311_set_bias_level),
    controls: es8311_snd_controls.as_ptr(),
    num_controls: es8311_snd_controls.len() as c_uint,
    dapm_widgets: es8311_dapm_widgets.as_ptr(),
    num_dapm_widgets: es8311_dapm_widgets.len() as c_uint,
    dapm_routes: es8311_dapm_routes.as_ptr(),
    num_dapm_routes: es8311_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn es8311_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let es8311: *mut es8311_priv;

    let dev: *mut device = &mut (*i2c_client).dev;

    es8311 = devm_kzalloc(dev, core::mem::size_of::<es8311_priv>(), GFP_KERNEL) as *mut es8311_priv;
    if es8311.is_null() {
        return -ENOMEM;
    }

    (*es8311).regmap = devm_regmap_init_i2c(i2c_client, &es8311_regmap_config);
    if IS_ERR((*es8311).regmap as *const c_void) {
        return PTR_ERR((*es8311).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c_client, es8311 as *mut c_void);

    devm_snd_soc_register_component(dev, &es8311_component_driver, &mut es8311_dai, 1)
}

static es8311_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"es8311\0",
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(i2c, es8311_id);

static es8311_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"everest,es8311".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(of, es8311_of_match);

static mut es8311_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"es8311".as_ptr(),
        of_match_table: es8311_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(es8311_i2c_probe),
    id_table: es8311_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(es8311_i2c_driver);

MODULE_DESCRIPTION!(c"ASoC ES8311 driver".as_ptr());
MODULE_AUTHOR!(c"Matteo Martelli <matteomartelli3@gmail.com>".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
