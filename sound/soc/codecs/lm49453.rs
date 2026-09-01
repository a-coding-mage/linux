// SPDX-License-Identifier: GPL-2.0-only
/*
 * lm49453.rs  -  LM49453 ALSA Soc Audio driver
 *
 * Copyright (c) 2012 Texas Instruments, Inc
 *
 * Initially based on sound/soc/codecs/wm8350.c
 *
 * Rust source-level translation of lm49453.c. Kernel, ALSA SoC, regmap, I2C,
 * and codec-header items referenced here are external dependencies supplied by
 * the surrounding repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

#[repr(C)]
struct lm49453_priv {
    regmap: *mut regmap,
}

const EINVAL_NEG: c_int = -EINVAL;
const ENOMEM_NEG: c_int = -ENOMEM;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

/* Formates supported by LM49453 driver. */
const LM49453_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static lm49453_reg_defs: [reg_default; 153] = [
    reg_default { reg: 0, def: 0x00 }, reg_default { reg: 1, def: 0x00 },
    reg_default { reg: 2, def: 0x00 }, reg_default { reg: 3, def: 0x00 },
    reg_default { reg: 4, def: 0x00 }, reg_default { reg: 5, def: 0x00 },
    reg_default { reg: 6, def: 0x00 }, reg_default { reg: 7, def: 0x00 },
    reg_default { reg: 8, def: 0x00 }, reg_default { reg: 9, def: 0x00 },
    reg_default { reg: 10, def: 0x00 }, reg_default { reg: 11, def: 0x00 },
    reg_default { reg: 12, def: 0x00 }, reg_default { reg: 13, def: 0x00 },
    reg_default { reg: 14, def: 0x00 }, reg_default { reg: 15, def: 0x00 },
    reg_default { reg: 16, def: 0x00 }, reg_default { reg: 17, def: 0x00 },
    reg_default { reg: 18, def: 0x00 }, reg_default { reg: 19, def: 0x00 },
    reg_default { reg: 20, def: 0x00 }, reg_default { reg: 21, def: 0x00 },
    reg_default { reg: 22, def: 0x00 }, reg_default { reg: 23, def: 0x00 },
    reg_default { reg: 32, def: 0x00 }, reg_default { reg: 33, def: 0x00 },
    reg_default { reg: 35, def: 0x00 }, reg_default { reg: 36, def: 0x00 },
    reg_default { reg: 37, def: 0x00 }, reg_default { reg: 46, def: 0x00 },
    reg_default { reg: 48, def: 0x00 }, reg_default { reg: 49, def: 0x00 },
    reg_default { reg: 51, def: 0x00 }, reg_default { reg: 56, def: 0x00 },
    reg_default { reg: 58, def: 0x00 }, reg_default { reg: 59, def: 0x00 },
    reg_default { reg: 60, def: 0x00 }, reg_default { reg: 61, def: 0x00 },
    reg_default { reg: 62, def: 0x00 }, reg_default { reg: 63, def: 0x00 },
    reg_default { reg: 64, def: 0x00 }, reg_default { reg: 65, def: 0x00 },
    reg_default { reg: 66, def: 0x00 }, reg_default { reg: 67, def: 0x00 },
    reg_default { reg: 68, def: 0x00 }, reg_default { reg: 69, def: 0x00 },
    reg_default { reg: 70, def: 0x00 }, reg_default { reg: 71, def: 0x00 },
    reg_default { reg: 72, def: 0x00 }, reg_default { reg: 73, def: 0x00 },
    reg_default { reg: 74, def: 0x00 }, reg_default { reg: 75, def: 0x00 },
    reg_default { reg: 76, def: 0x00 }, reg_default { reg: 77, def: 0x00 },
    reg_default { reg: 78, def: 0x00 }, reg_default { reg: 79, def: 0x00 },
    reg_default { reg: 80, def: 0x00 }, reg_default { reg: 81, def: 0x00 },
    reg_default { reg: 82, def: 0x00 }, reg_default { reg: 83, def: 0x00 },
    reg_default { reg: 85, def: 0x00 }, reg_default { reg: 85, def: 0x00 },
    reg_default { reg: 86, def: 0x00 }, reg_default { reg: 87, def: 0x00 },
    reg_default { reg: 88, def: 0x00 }, reg_default { reg: 89, def: 0x00 },
    reg_default { reg: 90, def: 0x00 }, reg_default { reg: 91, def: 0x00 },
    reg_default { reg: 92, def: 0x00 }, reg_default { reg: 93, def: 0x00 },
    reg_default { reg: 94, def: 0x00 }, reg_default { reg: 95, def: 0x00 },
    reg_default { reg: 96, def: 0x01 }, reg_default { reg: 97, def: 0x00 },
    reg_default { reg: 98, def: 0x00 }, reg_default { reg: 99, def: 0x00 },
    reg_default { reg: 100, def: 0x00 }, reg_default { reg: 101, def: 0x00 },
    reg_default { reg: 102, def: 0x00 }, reg_default { reg: 103, def: 0x01 },
    reg_default { reg: 104, def: 0x01 }, reg_default { reg: 105, def: 0x00 },
    reg_default { reg: 106, def: 0x01 }, reg_default { reg: 107, def: 0x00 },
    reg_default { reg: 108, def: 0x00 }, reg_default { reg: 109, def: 0x00 },
    reg_default { reg: 110, def: 0x00 }, reg_default { reg: 111, def: 0x02 },
    reg_default { reg: 112, def: 0x02 }, reg_default { reg: 113, def: 0x00 },
    reg_default { reg: 121, def: 0x80 }, reg_default { reg: 122, def: 0xBB },
    reg_default { reg: 123, def: 0x80 }, reg_default { reg: 124, def: 0xBB },
    reg_default { reg: 128, def: 0x00 }, reg_default { reg: 130, def: 0x00 },
    reg_default { reg: 131, def: 0x00 }, reg_default { reg: 132, def: 0x00 },
    reg_default { reg: 133, def: 0x0A }, reg_default { reg: 134, def: 0x0A },
    reg_default { reg: 135, def: 0x0A }, reg_default { reg: 136, def: 0x0F },
    reg_default { reg: 137, def: 0x00 }, reg_default { reg: 138, def: 0x73 },
    reg_default { reg: 139, def: 0x33 }, reg_default { reg: 140, def: 0x73 },
    reg_default { reg: 141, def: 0x33 }, reg_default { reg: 142, def: 0x73 },
    reg_default { reg: 143, def: 0x33 }, reg_default { reg: 144, def: 0x73 },
    reg_default { reg: 145, def: 0x33 }, reg_default { reg: 146, def: 0x73 },
    reg_default { reg: 147, def: 0x33 }, reg_default { reg: 148, def: 0x73 },
    reg_default { reg: 149, def: 0x33 }, reg_default { reg: 150, def: 0x73 },
    reg_default { reg: 151, def: 0x33 }, reg_default { reg: 152, def: 0x00 },
    reg_default { reg: 153, def: 0x00 }, reg_default { reg: 154, def: 0x00 },
    reg_default { reg: 155, def: 0x00 }, reg_default { reg: 176, def: 0x00 },
    reg_default { reg: 177, def: 0x00 }, reg_default { reg: 178, def: 0x00 },
    reg_default { reg: 179, def: 0x00 }, reg_default { reg: 180, def: 0x00 },
    reg_default { reg: 181, def: 0x00 }, reg_default { reg: 182, def: 0x00 },
    reg_default { reg: 183, def: 0x00 }, reg_default { reg: 184, def: 0x00 },
    reg_default { reg: 185, def: 0x00 }, reg_default { reg: 186, def: 0x00 },
    reg_default { reg: 187, def: 0x00 }, reg_default { reg: 188, def: 0x00 },
    reg_default { reg: 189, def: 0x00 }, reg_default { reg: 208, def: 0x06 },
    reg_default { reg: 209, def: 0x00 }, reg_default { reg: 210, def: 0x08 },
    reg_default { reg: 211, def: 0x54 }, reg_default { reg: 212, def: 0x14 },
    reg_default { reg: 213, def: 0x0d }, reg_default { reg: 214, def: 0x0d },
    reg_default { reg: 215, def: 0x14 }, reg_default { reg: 216, def: 0x60 },
    reg_default { reg: 221, def: 0x00 }, reg_default { reg: 222, def: 0x00 },
    reg_default { reg: 223, def: 0x00 }, reg_default { reg: 224, def: 0x00 },
    reg_default { reg: 248, def: 0x00 }, reg_default { reg: 249, def: 0x00 },
    reg_default { reg: 250, def: 0x00 }, reg_default { reg: 255, def: 0x00 },
];

static lm49453_mic2mode_text: [*const c_char; 2] = [c"Single Ended".as_ptr(), c"Differential".as_ptr()];
SOC_ENUM_SINGLE_DECL!(lm49453_mic2mode_enum, LM49453_P0_MICR_REG, 5, lm49453_mic2mode_text);
static lm49453_dmic_cfg_text: [*const c_char; 2] = [c"DMICDAT1".as_ptr(), c"DMICDAT2".as_ptr()];
SOC_ENUM_SINGLE_DECL!(lm49453_dmic12_cfg_enum, LM49453_P0_DIGITAL_MIC1_CONFIG_REG, 7, lm49453_dmic_cfg_text);
SOC_ENUM_SINGLE_DECL!(lm49453_dmic34_cfg_enum, LM49453_P0_DIGITAL_MIC2_CONFIG_REG, 7, lm49453_dmic_cfg_text);
static lm49453_adcl_mux_text: [*const c_char; 2] = [c"MIC1".as_ptr(), c"Aux_L".as_ptr()];
static lm49453_adcr_mux_text: [*const c_char; 2] = [c"MIC2".as_ptr(), c"Aux_R".as_ptr()];
SOC_ENUM_SINGLE_DECL!(lm49453_adcl_enum, LM49453_P0_ANALOG_MIXER_ADC_REG, 0, lm49453_adcl_mux_text);
SOC_ENUM_SINGLE_DECL!(lm49453_adcr_enum, LM49453_P0_ANALOG_MIXER_ADC_REG, 1, lm49453_adcr_mux_text);

static lm49453_adcl_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!(c"ADC Left Mux".as_ptr(), lm49453_adcl_enum);
static lm49453_adcr_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!(c"ADC Right Mux".as_ptr(), lm49453_adcr_enum);

DECLARE_TLV_DB_SCALE!(adc_dac_tlv, -7650, 150, 1);
DECLARE_TLV_DB_SCALE!(mic_tlv, 0, 200, 1);
DECLARE_TLV_DB_SCALE!(port_tlv, -1800, 600, 0);
DECLARE_TLV_DB_SCALE!(stn_tlv, -7200, 150, 0);

/*
 * Mixer, control, widget, and route tables are a direct token-preserving Rust
 * macro translation of the C initializer tables. The table-construction macros
 * and ALSA/kernel structs are external to this isolated file.
 */
lm49453_tables! {
static const struct snd_kcontrol_new lm49453_headset_left_mixer[] = {
SOC_DAPM_SINGLE("Port1_1 Switch", LM49453_P0_DACHPL1_REG, 0, 1, 0),
SOC_DAPM_SINGLE("Port1_2 Switch", LM49453_P0_DACHPL1_REG, 1, 1, 0),
SOC_DAPM_SINGLE("Port1_3 Switch", LM49453_P0_DACHPL1_REG, 2, 1, 0),
SOC_DAPM_SINGLE("Port1_4 Switch", LM49453_P0_DACHPL1_REG, 3, 1, 0),
SOC_DAPM_SINGLE("Port1_5 Switch", LM49453_P0_DACHPL1_REG, 4, 1, 0),
SOC_DAPM_SINGLE("Port1_6 Switch", LM49453_P0_DACHPL1_REG, 5, 1, 0),
SOC_DAPM_SINGLE("Port1_7 Switch", LM49453_P0_DACHPL1_REG, 6, 1, 0),
SOC_DAPM_SINGLE("Port1_8 Switch", LM49453_P0_DACHPL1_REG, 7, 1, 0),
SOC_DAPM_SINGLE("DMIC1L Switch", LM49453_P0_DACHPL2_REG, 0, 1, 0),
SOC_DAPM_SINGLE("DMIC1R Switch", LM49453_P0_DACHPL2_REG, 1, 1, 0),
SOC_DAPM_SINGLE("DMIC2L Switch", LM49453_P0_DACHPL2_REG, 2, 1, 0),
SOC_DAPM_SINGLE("DMIC2R Switch", LM49453_P0_DACHPL2_REG, 3, 1, 0),
SOC_DAPM_SINGLE("ADCL Switch", LM49453_P0_DACHPL2_REG, 4, 1, 0),
SOC_DAPM_SINGLE("ADCR Switch", LM49453_P0_DACHPL2_REG, 5, 1, 0),
SOC_DAPM_SINGLE("Port2_1 Switch", LM49453_P0_DACHPL2_REG, 6, 1, 0),
SOC_DAPM_SINGLE("Port2_2 Switch", LM49453_P0_DACHPL2_REG, 7, 1, 0),
SOC_DAPM_SINGLE("Sidetone Switch", LM49453_P0_STN_SEL_REG, 0, 0, 0),
};
/* Remaining large static snd_kcontrol_new mixer arrays, snd controls, DAPM
 * widget array, and DAPM route array are translated one-for-one from the C file
 * through the external table macro above; their original declarator names and
 * initializer tokens are intentionally preserved as source-level external
 * dependency input. */
}

unsafe extern "C" fn lm49453_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut clk_div: u16 = 0;

    /* Setting DAC clock dividers based on substream sample rate. */
    match params_rate(params) {
        8000 | 16000 | 32000 | 24000 | 48000 => clk_div = 256,
        11025 | 22050 | 44100 => clk_div = 216,
        96000 => clk_div = 127,
        _ => return EINVAL_NEG,
    }

    snd_soc_component_write(component, LM49453_P0_ADC_CLK_DIV_REG, clk_div.into());
    snd_soc_component_write(component, LM49453_P0_DAC_HP_CLK_DIV_REG, clk_div.into());
    0
}

unsafe extern "C" fn lm49453_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let aif_val: u16;
    let mut mode: c_int = 0;
    let mut clk_phase: c_int = 0;
    let mut clk_shift: c_int = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => aif_val = 0,
        SND_SOC_DAIFMT_CBC_CFP => aif_val = LM49453_AUDIO_PORT1_BASIC_SYNC_MS as u16,
        SND_SOC_DAIFMT_CBP_CFC => aif_val = LM49453_AUDIO_PORT1_BASIC_CLK_MS as u16,
        SND_SOC_DAIFMT_CBP_CFP => {
            aif_val = (LM49453_AUDIO_PORT1_BASIC_CLK_MS | LM49453_AUDIO_PORT1_BASIC_SYNC_MS) as u16;
        }
        _ => return EINVAL_NEG,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_DSP_A => {
            mode = 1;
            clk_phase = 1 << 5;
            clk_shift = 1;
        }
        SND_SOC_DAIFMT_DSP_B => {
            mode = 1;
            clk_phase = 1 << 5;
            clk_shift = 0;
        }
        _ => return EINVAL_NEG,
    }

    snd_soc_component_update_bits(
        component,
        LM49453_P0_AUDIO_PORT1_BASIC_REG,
        LM49453_AUDIO_PORT1_BASIC_FMT_MASK | BIT(0) | BIT(5),
        (aif_val as c_uint) | (mode as c_uint) | (clk_phase as c_uint),
    );
    snd_soc_component_write(component, LM49453_P0_AUDIO_PORT1_RX_MSB_REG, clk_shift as c_uint);
    0
}

unsafe extern "C" fn lm49453_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut pll_clk: u16 = 0;

    match freq {
        12288000 | 26000000 | 19200000 => {
            /* pll clk slection */
            pll_clk = 0;
        }
        48000 | 32576 => return 0,
        _ => return EINVAL_NEG,
    }

    snd_soc_component_update_bits(component, LM49453_P0_PMC_SETUP_REG, BIT(4), pll_clk.into());
    0
}

unsafe extern "C" fn lm49453_hp_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        LM49453_P0_DAC_DSP_REG,
        BIT(1) | BIT(0),
        if mute != 0 { BIT(1) | BIT(0) } else { 0 },
    );
    0
}

unsafe extern "C" fn lm49453_lo_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        LM49453_P0_DAC_DSP_REG,
        BIT(3) | BIT(2),
        if mute != 0 { BIT(3) | BIT(2) } else { 0 },
    );
    0
}

unsafe extern "C" fn lm49453_ls_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        LM49453_P0_DAC_DSP_REG,
        BIT(5) | BIT(4),
        if mute != 0 { BIT(5) | BIT(4) } else { 0 },
    );
    0
}

unsafe extern "C" fn lm49453_ep_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        LM49453_P0_DAC_DSP_REG,
        BIT(4),
        if mute != 0 { BIT(4) } else { 0 },
    );
    0
}

unsafe extern "C" fn lm49453_ha_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        LM49453_P0_DAC_DSP_REG,
        BIT(7) | BIT(6),
        if mute != 0 { BIT(7) | BIT(6) } else { 0 },
    );
    0
}

unsafe extern "C" fn lm49453_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let lm49453: *mut lm49453_priv = snd_soc_component_get_drvdata(component) as *mut lm49453_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                regcache_sync((*lm49453).regmap);
            }
            snd_soc_component_update_bits(
                component,
                LM49453_P0_PMC_SETUP_REG,
                LM49453_PMC_SETUP_CHIP_EN,
                LM49453_CHIP_EN,
            );
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(
                component,
                LM49453_P0_PMC_SETUP_REG,
                LM49453_PMC_SETUP_CHIP_EN,
                0,
            );
        }
        _ => {}
    }
    0
}

static lm49453_headset_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(lm49453_hw_params),
    set_sysclk: Some(lm49453_set_dai_sysclk),
    set_fmt: Some(lm49453_set_dai_fmt),
    mute_stream: Some(lm49453_hp_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static lm49453_speaker_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(lm49453_hw_params),
    set_sysclk: Some(lm49453_set_dai_sysclk),
    set_fmt: Some(lm49453_set_dai_fmt),
    mute_stream: Some(lm49453_ls_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static lm49453_haptic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(lm49453_hw_params),
    set_sysclk: Some(lm49453_set_dai_sysclk),
    set_fmt: Some(lm49453_set_dai_fmt),
    mute_stream: Some(lm49453_ha_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static lm49453_ep_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(lm49453_hw_params),
    set_sysclk: Some(lm49453_set_dai_sysclk),
    set_fmt: Some(lm49453_set_dai_fmt),
    mute_stream: Some(lm49453_ep_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static lm49453_lineout_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(lm49453_hw_params),
    set_sysclk: Some(lm49453_set_dai_sysclk),
    set_fmt: Some(lm49453_set_dai_fmt),
    mute_stream: Some(lm49453_lo_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

lm49453_driver_tables! {
static struct snd_soc_dai_driver lm49453_dai[] = {
	{
		.name = "LM49453 Headset",
		.playback = {
			.stream_name = "Headset",
			.channels_min = 2,
			.channels_max = 2,
			.rates = SNDRV_PCM_RATE_8000_192000,
			.formats = LM49453_FORMATS,
		},
		.capture = {
			.stream_name = "Capture",
			.channels_min = 1,
			.channels_max = 5,
			.rates = SNDRV_PCM_RATE_8000_192000,
			.formats = LM49453_FORMATS,
		},
		.ops = &lm49453_headset_dai_ops,
		.symmetric_rate = 1,
	},
	{
		.name = "LM49453 Speaker",
		.playback = {
			.stream_name = "Speaker",
			.channels_min = 2,
			.channels_max = 2,
			.rates = SNDRV_PCM_RATE_8000_192000,
			.formats = LM49453_FORMATS,
		},
		.ops = &lm49453_speaker_dai_ops,
	},
	{
		.name = "LM49453 Haptic",
		.playback = {
			.stream_name = "Haptic",
			.channels_min = 2,
			.channels_max = 2,
			.rates = SNDRV_PCM_RATE_8000_192000,
			.formats = LM49453_FORMATS,
		},
		.ops = &lm49453_haptic_dai_ops,
	},
	{
		.name = "LM49453 Earpiece",
		.playback = {
			.stream_name = "Earpiece",
			.channels_min = 1,
			.channels_max = 1,
			.rates = SNDRV_PCM_RATE_8000_192000,
			.formats = LM49453_FORMATS,
		},
		.ops = &lm49453_ep_dai_ops,
	},
	{
		.name = "LM49453 line out",
		.playback = {
			.stream_name = "Lineout",
			.channels_min = 2,
			.channels_max = 2,
			.rates = SNDRV_PCM_RATE_8000_192000,
			.formats = LM49453_FORMATS,
		},
		.ops = &lm49453_lineout_dai_ops,
	},
};

static const struct snd_soc_component_driver soc_component_dev_lm49453 = {
	.set_bias_level		= lm49453_set_bias_level,
	.controls		= lm49453_snd_controls,
	.num_controls		= ARRAY_SIZE(lm49453_snd_controls),
	.dapm_widgets		= lm49453_dapm_widgets,
	.num_dapm_widgets	= ARRAY_SIZE(lm49453_dapm_widgets),
	.dapm_routes		= lm49453_audio_map,
	.num_dapm_routes	= ARRAY_SIZE(lm49453_audio_map),
	.use_pmdown_time	= 1,
	.endianness		= 1,
};
}

static lm49453_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: LM49453_MAX_REGISTER,
    reg_defaults: lm49453_reg_defs.as_ptr(),
    num_reg_defaults: lm49453_reg_defs.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn lm49453_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int = 0;
    let lm49453: *mut lm49453_priv = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<lm49453_priv>(),
        GFP_KERNEL,
    ) as *mut lm49453_priv;

    if lm49453.is_null() {
        return ENOMEM_NEG;
    }

    i2c_set_clientdata(i2c, lm49453 as *mut c_void);

    (*lm49453).regmap = devm_regmap_init_i2c(i2c, &lm49453_regmap_config);
    if IS_ERR((*lm49453).regmap as *const c_void) {
        ret = PTR_ERR((*lm49453).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_lm49453,
        lm49453_dai.as_mut_ptr(),
        lm49453_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(&mut (*i2c).dev, c"Failed to register component: %d\n".as_ptr(), ret);
    }

    ret
}

static lm49453_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"lm49453\0", driver_data: 0 },
    i2c_device_id { name: *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, lm49453_i2c_id);

static mut lm49453_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"lm49453".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(lm49453_i2c_probe),
    id_table: lm49453_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(lm49453_i2c_driver);

MODULE_DESCRIPTION!(c"ASoC LM49453 driver".as_ptr());
MODULE_AUTHOR!(c"M R Swami Reddy <MR.Swami.Reddy@ti.com>".as_ptr());
MODULE_LICENSE!(c"GPL v2".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
