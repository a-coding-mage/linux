// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8737.rs  --  WM8737 ALSA SoC Audio driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Translated from C. Original includes:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h, linux/pm.h,
// linux/i2c.h, linux/regmap.h, linux/regulator/consumer.h, linux/spi/spi.h,
// linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-dapm.h, sound/initval.h, sound/tlv.h, and "wm8737.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const WM8737_NUM_SUPPLIES: usize = 4;

static wm8737_supply_names: [*const c_char; WM8737_NUM_SUPPLIES] = [
    b"DCVDD\0".as_ptr() as *const c_char,
    b"DBVDD\0".as_ptr() as *const c_char,
    b"AVDD\0".as_ptr() as *const c_char,
    b"MVDD\0".as_ptr() as *const c_char,
];

/* codec private data */
#[repr(C)]
struct wm8737_priv {
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; WM8737_NUM_SUPPLIES],
    mclk: c_uint,
}

static wm8737_reg_defaults: [reg_default; 15] = [
    reg_default { reg: 0, def: 0x00C3 },     /* R0  - Left PGA volume */
    reg_default { reg: 1, def: 0x00C3 },     /* R1  - Right PGA volume */
    reg_default { reg: 2, def: 0x0007 },     /* R2  - AUDIO path L */
    reg_default { reg: 3, def: 0x0007 },     /* R3  - AUDIO path R */
    reg_default { reg: 4, def: 0x0000 },     /* R4  - 3D Enhance */
    reg_default { reg: 5, def: 0x0000 },     /* R5  - ADC Control */
    reg_default { reg: 6, def: 0x0000 },     /* R6  - Power Management */
    reg_default { reg: 7, def: 0x000A },     /* R7  - Audio Format */
    reg_default { reg: 8, def: 0x0000 },     /* R8  - Clocking */
    reg_default { reg: 9, def: 0x000F },     /* R9  - MIC Preamp Control */
    reg_default { reg: 10, def: 0x0003 },    /* R10 - Misc Bias Control */
    reg_default { reg: 11, def: 0x0000 },    /* R11 - Noise Gate */
    reg_default { reg: 12, def: 0x007C },    /* R12 - ALC1 */
    reg_default { reg: 13, def: 0x0000 },    /* R13 - ALC2 */
    reg_default { reg: 14, def: 0x0032 },    /* R14 - ALC3 */
];

unsafe extern "C" fn wm8737_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8737_RESET => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8737_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, WM8737_RESET, 0)
}

// DECLARE_TLV_DB_RANGE(micboost_tlv, ...)
static micboost_tlv: [c_uint; 16] = [
    0, 0, TLV_DB_SCALE_ITEM(1300, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM(1800, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM(2800, 0, 0),
    3, 3, TLV_DB_SCALE_ITEM(3300, 0, 0),
    0,
];
// DECLARE_TLV_DB_SCALE(...)
static pga_tlv: [c_uint; 4] = TLV_DB_SCALE(-9750, 50, 1);
static adc_tlv: [c_uint; 4] = TLV_DB_SCALE(-600, 600, 0);
static ng_tlv: [c_uint; 4] = TLV_DB_SCALE(-7800, 600, 0);
static alc_max_tlv: [c_uint; 4] = TLV_DB_SCALE(-1200, 600, 0);
static alc_target_tlv: [c_uint; 4] = TLV_DB_SCALE(-1800, 100, 0);

static micbias_enum_text: [*const c_char; 4] = [
    b"25%\0".as_ptr() as *const c_char,
    b"50%\0".as_ptr() as *const c_char,
    b"75%\0".as_ptr() as *const c_char,
    b"100%\0".as_ptr() as *const c_char,
];

static micbias_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_MIC_PREAMP_CONTROL, 0, &micbias_enum_text);

static low_cutoff_text: [*const c_char; 2] = [
    b"Low\0".as_ptr() as *const c_char,
    b"High\0".as_ptr() as *const c_char,
];

static low_3d: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_3D_ENHANCE, 6, &low_cutoff_text);

static high_cutoff_text: [*const c_char; 2] = [
    b"High\0".as_ptr() as *const c_char,
    b"Low\0".as_ptr() as *const c_char,
];

static high_3d: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_3D_ENHANCE, 5, &high_cutoff_text);

static alc_fn_text: [*const c_char; 4] = [
    b"Disabled\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
    b"Left\0".as_ptr() as *const c_char,
    b"Stereo\0".as_ptr() as *const c_char,
];

static alc_fn: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_ALC1, 7, &alc_fn_text);

static alc_hold_text: [*const c_char; 16] = [
    b"0\0".as_ptr() as *const c_char,
    b"2.67ms\0".as_ptr() as *const c_char,
    b"5.33ms\0".as_ptr() as *const c_char,
    b"10.66ms\0".as_ptr() as *const c_char,
    b"21.32ms\0".as_ptr() as *const c_char,
    b"42.64ms\0".as_ptr() as *const c_char,
    b"85.28ms\0".as_ptr() as *const c_char,
    b"170.56ms\0".as_ptr() as *const c_char,
    b"341.12ms\0".as_ptr() as *const c_char,
    b"682.24ms\0".as_ptr() as *const c_char,
    b"1.364s\0".as_ptr() as *const c_char,
    b"2.728s\0".as_ptr() as *const c_char,
    b"5.458s\0".as_ptr() as *const c_char,
    b"10.916s\0".as_ptr() as *const c_char,
    b"21.832s\0".as_ptr() as *const c_char,
    b"43.691s\0".as_ptr() as *const c_char,
];

static alc_hold: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_ALC2, 0, &alc_hold_text);

static alc_atk_text: [*const c_char; 11] = [
    b"8.4ms\0".as_ptr() as *const c_char,
    b"16.8ms\0".as_ptr() as *const c_char,
    b"33.6ms\0".as_ptr() as *const c_char,
    b"67.2ms\0".as_ptr() as *const c_char,
    b"134.4ms\0".as_ptr() as *const c_char,
    b"268.8ms\0".as_ptr() as *const c_char,
    b"537.6ms\0".as_ptr() as *const c_char,
    b"1.075s\0".as_ptr() as *const c_char,
    b"2.15s\0".as_ptr() as *const c_char,
    b"4.3s\0".as_ptr() as *const c_char,
    b"8.6s\0".as_ptr() as *const c_char,
];

static alc_atk: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_ALC3, 0, &alc_atk_text);

static alc_dcy_text: [*const c_char; 11] = [
    b"33.6ms\0".as_ptr() as *const c_char,
    b"67.2ms\0".as_ptr() as *const c_char,
    b"134.4ms\0".as_ptr() as *const c_char,
    b"268.8ms\0".as_ptr() as *const c_char,
    b"537.6ms\0".as_ptr() as *const c_char,
    b"1.075s\0".as_ptr() as *const c_char,
    b"2.15s\0".as_ptr() as *const c_char,
    b"4.3s\0".as_ptr() as *const c_char,
    b"8.6s\0".as_ptr() as *const c_char,
    b"17.2s\0".as_ptr() as *const c_char,
    b"34.41s\0".as_ptr() as *const c_char,
];

static alc_dcy: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_ALC3, 4, &alc_dcy_text);

static wm8737_snd_controls: [snd_kcontrol_new; 27] = [
    SOC_DOUBLE_R_TLV(b"Mic Boost Volume\0".as_ptr() as *const c_char, WM8737_AUDIO_PATH_L, WM8737_AUDIO_PATH_R, 6, 3, 0, &micboost_tlv),
    SOC_DOUBLE_R(b"Mic Boost Switch\0".as_ptr() as *const c_char, WM8737_AUDIO_PATH_L, WM8737_AUDIO_PATH_R, 4, 1, 0),
    SOC_DOUBLE(b"Mic ZC Switch\0".as_ptr() as *const c_char, WM8737_AUDIO_PATH_L, WM8737_AUDIO_PATH_R, 3, 1, 0),
    SOC_DOUBLE_R_TLV(b"Capture Volume\0".as_ptr() as *const c_char, WM8737_LEFT_PGA_VOLUME, WM8737_RIGHT_PGA_VOLUME, 0, 255, 0, &pga_tlv),
    SOC_DOUBLE(b"Capture ZC Switch\0".as_ptr() as *const c_char, WM8737_AUDIO_PATH_L, WM8737_AUDIO_PATH_R, 2, 1, 0),
    SOC_DOUBLE(b"INPUT1 DC Bias Switch\0".as_ptr() as *const c_char, WM8737_MISC_BIAS_CONTROL, 0, 1, 1, 0),
    SOC_ENUM(b"Mic PGA Bias\0".as_ptr() as *const c_char, &micbias_enum),
    SOC_SINGLE(b"ADC Low Power Switch\0".as_ptr() as *const c_char, WM8737_ADC_CONTROL, 2, 1, 0),
    SOC_SINGLE(b"High Pass Filter Switch\0".as_ptr() as *const c_char, WM8737_ADC_CONTROL, 0, 1, 1),
    SOC_DOUBLE(b"Polarity Invert Switch\0".as_ptr() as *const c_char, WM8737_ADC_CONTROL, 5, 6, 1, 0),
    SOC_SINGLE(b"3D Switch\0".as_ptr() as *const c_char, WM8737_3D_ENHANCE, 0, 1, 0),
    SOC_SINGLE(b"3D Depth\0".as_ptr() as *const c_char, WM8737_3D_ENHANCE, 1, 15, 0),
    SOC_ENUM(b"3D Low Cut-off\0".as_ptr() as *const c_char, &low_3d),
    SOC_ENUM(b"3D High Cut-off\0".as_ptr() as *const c_char, &high_3d),
    SOC_SINGLE_TLV(b"3D ADC Volume\0".as_ptr() as *const c_char, WM8737_3D_ENHANCE, 7, 1, 1, &adc_tlv),
    SOC_SINGLE(b"Noise Gate Switch\0".as_ptr() as *const c_char, WM8737_NOISE_GATE, 0, 1, 0),
    SOC_SINGLE_TLV(b"Noise Gate Threshold Volume\0".as_ptr() as *const c_char, WM8737_NOISE_GATE, 2, 7, 0, &ng_tlv),
    SOC_ENUM(b"ALC\0".as_ptr() as *const c_char, &alc_fn),
    SOC_SINGLE_TLV(b"ALC Max Gain Volume\0".as_ptr() as *const c_char, WM8737_ALC1, 4, 7, 0, &alc_max_tlv),
    SOC_SINGLE_TLV(b"ALC Target Volume\0".as_ptr() as *const c_char, WM8737_ALC1, 0, 15, 0, &alc_target_tlv),
    SOC_ENUM(b"ALC Hold Time\0".as_ptr() as *const c_char, &alc_hold),
    SOC_SINGLE(b"ALC ZC Switch\0".as_ptr() as *const c_char, WM8737_ALC2, 4, 1, 0),
    SOC_ENUM(b"ALC Attack Time\0".as_ptr() as *const c_char, &alc_atk),
    SOC_ENUM(b"ALC Decay Time\0".as_ptr() as *const c_char, &alc_dcy),
];

static linsel_text: [*const c_char; 4] = [
    b"LINPUT1\0".as_ptr() as *const c_char,
    b"LINPUT2\0".as_ptr() as *const c_char,
    b"LINPUT3\0".as_ptr() as *const c_char,
    b"LINPUT1 DC\0".as_ptr() as *const c_char,
];

static linsel_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_AUDIO_PATH_L, 7, &linsel_text);

static linsel_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM(b"LINSEL\0".as_ptr() as *const c_char, &linsel_enum);

static rinsel_text: [*const c_char; 4] = [
    b"RINPUT1\0".as_ptr() as *const c_char,
    b"RINPUT2\0".as_ptr() as *const c_char,
    b"RINPUT3\0".as_ptr() as *const c_char,
    b"RINPUT1 DC\0".as_ptr() as *const c_char,
];

static rinsel_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_AUDIO_PATH_R, 7, &rinsel_text);

static rinsel_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM(b"RINSEL\0".as_ptr() as *const c_char, &rinsel_enum);

static bypass_text: [*const c_char; 2] = [
    b"Direct\0".as_ptr() as *const c_char,
    b"Preamp\0".as_ptr() as *const c_char,
];

static lbypass_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_MIC_PREAMP_CONTROL, 2, &bypass_text);

static lbypass_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM(b"Left Bypass\0".as_ptr() as *const c_char, &lbypass_enum);

static rbypass_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL(WM8737_MIC_PREAMP_CONTROL, 3, &bypass_text);

static rbypass_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM(b"Left Bypass\0".as_ptr() as *const c_char, &rbypass_enum);

static wm8737_dapm_widgets: [snd_soc_dapm_widget; 18] = [
    SND_SOC_DAPM_INPUT(b"LINPUT1\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"LINPUT2\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"LINPUT3\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"RINPUT1\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"RINPUT2\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"RINPUT3\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"LACIN\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT(b"RACIN\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_MUX(b"LINSEL\0".as_ptr() as *const c_char, SND_SOC_NOPM, 0, 0, &linsel_mux),
    SND_SOC_DAPM_MUX(b"RINSEL\0".as_ptr() as *const c_char, SND_SOC_NOPM, 0, 0, &rinsel_mux),
    SND_SOC_DAPM_MUX(b"Left Preamp Mux\0".as_ptr() as *const c_char, SND_SOC_NOPM, 0, 0, &lbypass_mux),
    SND_SOC_DAPM_MUX(b"Right Preamp Mux\0".as_ptr() as *const c_char, SND_SOC_NOPM, 0, 0, &rbypass_mux),
    SND_SOC_DAPM_PGA(b"PGAL\0".as_ptr() as *const c_char, WM8737_POWER_MANAGEMENT, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA(b"PGAR\0".as_ptr() as *const c_char, WM8737_POWER_MANAGEMENT, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_DAC(b"ADCL\0".as_ptr() as *const c_char, ptr::null(), WM8737_POWER_MANAGEMENT, 3, 0),
    SND_SOC_DAPM_DAC(b"ADCR\0".as_ptr() as *const c_char, ptr::null(), WM8737_POWER_MANAGEMENT, 2, 0),
    SND_SOC_DAPM_AIF_OUT(b"AIF\0".as_ptr() as *const c_char, b"Capture\0".as_ptr() as *const c_char, 0, WM8737_POWER_MANAGEMENT, 6, 0),
];

static intercon: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: b"LINSEL\0".as_ptr() as *const c_char, control: b"LINPUT1\0".as_ptr() as *const c_char, source: b"LINPUT1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINSEL\0".as_ptr() as *const c_char, control: b"LINPUT2\0".as_ptr() as *const c_char, source: b"LINPUT2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINSEL\0".as_ptr() as *const c_char, control: b"LINPUT3\0".as_ptr() as *const c_char, source: b"LINPUT3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINSEL\0".as_ptr() as *const c_char, control: b"LINPUT1 DC\0".as_ptr() as *const c_char, source: b"LINPUT1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RINSEL\0".as_ptr() as *const c_char, control: b"RINPUT1\0".as_ptr() as *const c_char, source: b"RINPUT1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RINSEL\0".as_ptr() as *const c_char, control: b"RINPUT2\0".as_ptr() as *const c_char, source: b"RINPUT2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RINSEL\0".as_ptr() as *const c_char, control: b"RINPUT3\0".as_ptr() as *const c_char, source: b"RINPUT3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RINSEL\0".as_ptr() as *const c_char, control: b"RINPUT1 DC\0".as_ptr() as *const c_char, source: b"RINPUT1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Preamp Mux\0".as_ptr() as *const c_char, control: b"Preamp\0".as_ptr() as *const c_char, source: b"LINSEL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Preamp Mux\0".as_ptr() as *const c_char, control: b"Direct\0".as_ptr() as *const c_char, source: b"LACIN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Preamp Mux\0".as_ptr() as *const c_char, control: b"Preamp\0".as_ptr() as *const c_char, source: b"RINSEL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Preamp Mux\0".as_ptr() as *const c_char, control: b"Direct\0".as_ptr() as *const c_char, source: b"RACIN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGAL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Left Preamp Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGAR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Right Preamp Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADCL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PGAL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADCR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PGAR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIF\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADCL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIF\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADCR\0".as_ptr() as *const c_char },
];

/* codec mclk clock divider coefficients */
#[repr(C)]
struct coeff_div_entry {
    mclk: u32,
    rate: u32,
    usb: u8,
    sr: u8,
}

static coeff_div: [coeff_div_entry; 35] = [
    coeff_div_entry { mclk: 12288000, rate: 8000, usb: 0, sr: 0x4 },
    coeff_div_entry { mclk: 12288000, rate: 12000, usb: 0, sr: 0x8 },
    coeff_div_entry { mclk: 12288000, rate: 16000, usb: 0, sr: 0xa },
    coeff_div_entry { mclk: 12288000, rate: 24000, usb: 0, sr: 0x1c },
    coeff_div_entry { mclk: 12288000, rate: 32000, usb: 0, sr: 0xc },
    coeff_div_entry { mclk: 12288000, rate: 48000, usb: 0, sr: 0 },
    coeff_div_entry { mclk: 12288000, rate: 96000, usb: 0, sr: 0xe },
    coeff_div_entry { mclk: 11289600, rate: 8000, usb: 0, sr: 0x14 },
    coeff_div_entry { mclk: 11289600, rate: 11025, usb: 0, sr: 0x18 },
    coeff_div_entry { mclk: 11289600, rate: 22050, usb: 0, sr: 0x1a },
    coeff_div_entry { mclk: 11289600, rate: 44100, usb: 0, sr: 0x10 },
    coeff_div_entry { mclk: 11289600, rate: 88200, usb: 0, sr: 0x1e },
    coeff_div_entry { mclk: 18432000, rate: 8000, usb: 0, sr: 0x5 },
    coeff_div_entry { mclk: 18432000, rate: 12000, usb: 0, sr: 0x9 },
    coeff_div_entry { mclk: 18432000, rate: 16000, usb: 0, sr: 0xb },
    coeff_div_entry { mclk: 18432000, rate: 24000, usb: 0, sr: 0x1b },
    coeff_div_entry { mclk: 18432000, rate: 32000, usb: 0, sr: 0xd },
    coeff_div_entry { mclk: 18432000, rate: 48000, usb: 0, sr: 0x1 },
    coeff_div_entry { mclk: 18432000, rate: 96000, usb: 0, sr: 0x1f },
    coeff_div_entry { mclk: 16934400, rate: 8000, usb: 0, sr: 0x15 },
    coeff_div_entry { mclk: 16934400, rate: 11025, usb: 0, sr: 0x19 },
    coeff_div_entry { mclk: 16934400, rate: 22050, usb: 0, sr: 0x1b },
    coeff_div_entry { mclk: 16934400, rate: 44100, usb: 0, sr: 0x11 },
    coeff_div_entry { mclk: 16934400, rate: 88200, usb: 0, sr: 0x1f },
    coeff_div_entry { mclk: 12000000, rate: 8000, usb: 1, sr: 0x4 },
    coeff_div_entry { mclk: 12000000, rate: 11025, usb: 1, sr: 0x19 },
    coeff_div_entry { mclk: 12000000, rate: 12000, usb: 1, sr: 0x8 },
    coeff_div_entry { mclk: 12000000, rate: 16000, usb: 1, sr: 0xa },
    coeff_div_entry { mclk: 12000000, rate: 22050, usb: 1, sr: 0x1b },
    coeff_div_entry { mclk: 12000000, rate: 24000, usb: 1, sr: 0x1c },
    coeff_div_entry { mclk: 12000000, rate: 32000, usb: 1, sr: 0xc },
    coeff_div_entry { mclk: 12000000, rate: 44100, usb: 1, sr: 0x11 },
    coeff_div_entry { mclk: 12000000, rate: 48000, usb: 1, sr: 0x0 },
    coeff_div_entry { mclk: 12000000, rate: 88200, usb: 1, sr: 0x1f },
    coeff_div_entry { mclk: 12000000, rate: 96000, usb: 1, sr: 0xe },
];

unsafe extern "C" fn wm8737_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8737: *mut wm8737_priv =
        snd_soc_component_get_drvdata(component) as *mut wm8737_priv;
    let mut i: usize;
    let mut clocking: u16 = 0;
    let mut af: u16 = 0;

    i = 0;
    while i < coeff_div.len() {
        if coeff_div[i].rate != params_rate(params) {
            i += 1;
            continue;
        }

        if coeff_div[i].mclk == (*wm8737).mclk {
            break;
        }

        if coeff_div[i].mclk == (*wm8737).mclk.wrapping_mul(2) {
            clocking |= WM8737_CLKDIV2 as u16;
            break;
        }
        i += 1;
    }

    if i == coeff_div.len() {
        dev_err(
            (*component).dev,
            b"%dHz MCLK can't support %dHz\n\0".as_ptr() as *const c_char,
            (*wm8737).mclk,
            params_rate(params),
        );
        return -EINVAL;
    }

    clocking |= (coeff_div[i].usb as u16) | ((coeff_div[i].sr as u16) << WM8737_SR_SHIFT);

    match params_width(params) {
        16 => {}
        20 => af |= 0x8,
        24 => af |= 0x10,
        32 => af |= 0x18,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, WM8737_AUDIO_FORMAT, WM8737_WL_MASK, af as c_uint);
    snd_soc_component_update_bits(
        component,
        WM8737_CLOCKING,
        WM8737_USB_MODE | WM8737_CLKDIV2 | WM8737_SR_MASK,
        clocking as c_uint,
    );

    0
}

unsafe extern "C" fn wm8737_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let wm8737: *mut wm8737_priv =
        snd_soc_component_get_drvdata(component) as *mut wm8737_priv;
    let mut i: usize = 0;

    while i < coeff_div.len() {
        if freq == coeff_div[i].mclk || freq == coeff_div[i].mclk.wrapping_mul(2) {
            (*wm8737).mclk = freq;
            return 0;
        }
        i += 1;
    }

    dev_err(
        (*component).dev,
        b"MCLK rate %dHz not supported\n\0".as_ptr() as *const c_char,
        freq,
    );

    -EINVAL
}

unsafe extern "C" fn wm8737_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut af: u16 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => af |= WM8737_MS as u16,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => af |= 0x2,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => af |= 0x1,
        SND_SOC_DAIFMT_DSP_A => af |= 0x3,
        SND_SOC_DAIFMT_DSP_B => af |= 0x13,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => af |= WM8737_LRP as u16,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(
        component,
        WM8737_AUDIO_FORMAT,
        WM8737_FORMAT_MASK | WM8737_LRP | WM8737_MS,
        af as c_uint,
    );

    0
}

unsafe extern "C" fn wm8737_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8737: *mut wm8737_priv =
        snd_soc_component_get_drvdata(component) as *mut wm8737_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}

        SND_SOC_BIAS_PREPARE => {
            /* VMID at 2*75k */
            snd_soc_component_update_bits(
                component,
                WM8737_MISC_BIAS_CONTROL,
                WM8737_VMIDSEL_MASK,
                0,
            );
        }

        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable((*wm8737).supplies.len(), (*wm8737).supplies.as_mut_ptr());
                if ret != 0 {
                    dev_err(
                        (*component).dev,
                        b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char,
                        ret,
                    );
                    return ret;
                }

                regcache_sync((*wm8737).regmap);

                /* Fast VMID ramp at 2*2.5k */
                snd_soc_component_update_bits(
                    component,
                    WM8737_MISC_BIAS_CONTROL,
                    WM8737_VMIDSEL_MASK,
                    2 << WM8737_VMIDSEL_SHIFT,
                );

                /* Bring VMID up */
                snd_soc_component_update_bits(
                    component,
                    WM8737_POWER_MANAGEMENT,
                    WM8737_VMID_MASK | WM8737_VREF_MASK,
                    WM8737_VMID_MASK | WM8737_VREF_MASK,
                );

                msleep(500);
            }

            /* VMID at 2*300k */
            snd_soc_component_update_bits(
                component,
                WM8737_MISC_BIAS_CONTROL,
                WM8737_VMIDSEL_MASK,
                1 << WM8737_VMIDSEL_SHIFT,
            );
        }

        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(
                component,
                WM8737_POWER_MANAGEMENT,
                WM8737_VMID_MASK | WM8737_VREF_MASK,
                0,
            );

            regulator_bulk_disable((*wm8737).supplies.len(), (*wm8737).supplies.as_mut_ptr());
        }
    }

    0
}

const WM8737_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;

const WM8737_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static wm8737_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8737_hw_params),
    set_sysclk: Some(wm8737_set_dai_sysclk),
    set_fmt: Some(wm8737_set_dai_fmt),
};

static mut wm8737_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8737\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,  /* Mono modes not yet supported */
        channels_max: 2,
        rates: WM8737_RATES,
        formats: WM8737_FORMATS,
    },
    ops: &wm8737_dai_ops,
};

unsafe extern "C" fn wm8737_probe(component: *mut snd_soc_component) -> c_int {
    let wm8737: *mut wm8737_priv =
        snd_soc_component_get_drvdata(component) as *mut wm8737_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    ret = regulator_bulk_enable((*wm8737).supplies.len(), (*wm8737).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(
            (*component).dev,
            b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = wm8737_reset(component);
    if ret < 0 {
        dev_err((*component).dev, b"Failed to issue reset\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable((*wm8737).supplies.len(), (*wm8737).supplies.as_mut_ptr());
        return ret;
    }

    snd_soc_component_update_bits(component, WM8737_LEFT_PGA_VOLUME, WM8737_LVU, WM8737_LVU);
    snd_soc_component_update_bits(component, WM8737_RIGHT_PGA_VOLUME, WM8737_RVU, WM8737_RVU);

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);

    /* Bias level configuration will have done an extra enable */
    regulator_bulk_disable((*wm8737).supplies.len(), (*wm8737).supplies.as_mut_ptr());

    0
}

static soc_component_dev_wm8737: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8737_probe),
    set_bias_level: Some(wm8737_set_bias_level),
    controls: wm8737_snd_controls.as_ptr(),
    num_controls: wm8737_snd_controls.len() as c_uint,
    dapm_widgets: wm8737_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8737_dapm_widgets.len() as c_uint,
    dapm_routes: intercon.as_ptr(),
    num_dapm_routes: intercon.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8737_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"wlf,wm8737\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];

// MODULE_DEVICE_TABLE(of, wm8737_of_match);

static wm8737_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8737_MAX_REGISTER,
    reg_defaults: wm8737_reg_defaults.as_ptr(),
    num_reg_defaults: wm8737_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(wm8737_volatile),
};

// #if IS_ENABLED(CONFIG_I2C)
unsafe extern "C" fn wm8737_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut wm8737: *mut wm8737_priv;
    let mut ret: c_int;
    let mut i: usize;

    wm8737 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm8737_priv>(),
        GFP_KERNEL,
    ) as *mut wm8737_priv;
    if wm8737.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*wm8737).supplies.len() {
        (*wm8737).supplies[i].supply = wm8737_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*i2c).dev,
        (*wm8737).supplies.len(),
        (*wm8737).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    (*wm8737).regmap = devm_regmap_init_i2c(i2c, &wm8737_regmap);
    if IS_ERR((*wm8737).regmap as *const c_void) {
        return PTR_ERR((*wm8737).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, wm8737 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8737,
        &mut wm8737_dai,
        1,
    );

    ret
}

static wm8737_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"wm8737\0" },
    i2c_device_id { name: [0; I2C_NAME_SIZE] },
];
// MODULE_DEVICE_TABLE(i2c, wm8737_i2c_id);

static mut wm8737_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"wm8737\0".as_ptr() as *const c_char,
        of_match_table: wm8737_of_match.as_ptr(),
    },
    probe: Some(wm8737_i2c_probe),
    id_table: wm8737_i2c_id.as_ptr(),
};
// #endif

// #if defined(CONFIG_SPI_MASTER)
unsafe extern "C" fn wm8737_spi_probe(spi: *mut spi_device) -> c_int {
    let mut wm8737: *mut wm8737_priv;
    let mut ret: c_int;
    let mut i: usize;

    wm8737 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<wm8737_priv>(),
        GFP_KERNEL,
    ) as *mut wm8737_priv;
    if wm8737.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*wm8737).supplies.len() {
        (*wm8737).supplies[i].supply = wm8737_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*spi).dev,
        (*wm8737).supplies.len(),
        (*wm8737).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            &mut (*spi).dev,
            b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    (*wm8737).regmap = devm_regmap_init_spi(spi, &wm8737_regmap);
    if IS_ERR((*wm8737).regmap as *const c_void) {
        return PTR_ERR((*wm8737).regmap as *const c_void);
    }

    spi_set_drvdata(spi, wm8737 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8737,
        &mut wm8737_dai,
        1,
    );

    ret
}

static mut wm8737_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"wm8737\0".as_ptr() as *const c_char,
        of_match_table: wm8737_of_match.as_ptr(),
    },
    probe: Some(wm8737_spi_probe),
};
// #endif /* CONFIG_SPI_MASTER */

unsafe extern "C" fn wm8737_modinit() -> c_int {
    let mut ret: c_int;
    // #if IS_ENABLED(CONFIG_I2C)
    ret = i2c_add_driver(&mut wm8737_i2c_driver);
    if ret != 0 {
        printk(
            b"\x013Failed to register WM8737 I2C driver: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }
    // #endif
    // #if defined(CONFIG_SPI_MASTER)
    ret = spi_register_driver(&mut wm8737_spi_driver);
    if ret != 0 {
        printk(
            b"\x013Failed to register WM8737 SPI driver: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }
    // #endif
    0
}
// module_init(wm8737_modinit);

unsafe extern "C" fn wm8737_exit() {
    // #if defined(CONFIG_SPI_MASTER)
    spi_unregister_driver(&mut wm8737_spi_driver);
    // #endif
    // #if IS_ENABLED(CONFIG_I2C)
    i2c_del_driver(&mut wm8737_i2c_driver);
    // #endif
}
// module_exit(wm8737_exit);

// MODULE_DESCRIPTION("ASoC WM8737 driver");
// MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");

#[repr(C)]
struct regmap;
#[repr(C)]
struct device;
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
struct regulator_bulk_data {
    supply: *const c_char,
}
#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}
#[repr(C)]
struct snd_pcm_substream;
#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_soc_dapm_context;
#[repr(C)]
struct snd_kcontrol_new;
#[repr(C)]
struct soc_enum;
#[repr(C)]
struct snd_soc_dapm_widget;
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    suspend_bias_off: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    cache_type: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}
#[repr(C)]
struct i2c_client {
    dev: device,
}
const I2C_NAME_SIZE: usize = 20;
#[repr(C)]
struct i2c_device_id {
    name: [u8; I2C_NAME_SIZE],
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}
#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}
#[repr(C)]
struct spi_device {
    dev: device,
}
#[repr(C)]
struct spi_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

type snd_soc_bias_level = c_uint;

extern "C" {
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regulator_bulk_enable(num_consumers: usize, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: usize, consumers: *mut regulator_bulk_data) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: usize, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
}

extern "Rust" {
    fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> c_uint;
    fn TLV_DB_SCALE(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4];
    fn SOC_ENUM_SINGLE_DECL(reg: c_uint, shift: c_uint, texts: *const [*const c_char]) -> soc_enum;
    fn SOC_DOUBLE_R_TLV(name: *const c_char, reg_left: c_uint, reg_right: c_uint, shift: c_uint, max: c_uint, invert: c_uint, tlv: *const [c_uint]) -> snd_kcontrol_new;
    fn SOC_DOUBLE_R(name: *const c_char, reg_left: c_uint, reg_right: c_uint, shift: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SOC_DOUBLE(name: *const c_char, reg: c_uint, shift_left: c_uint, shift_right: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SOC_ENUM(name: *const c_char, xenum: *const soc_enum) -> snd_kcontrol_new;
    fn SOC_SINGLE(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint) -> snd_kcontrol_new;
    fn SOC_SINGLE_TLV(name: *const c_char, reg: c_uint, shift: c_uint, max: c_uint, invert: c_uint, tlv: *const [c_uint]) -> snd_kcontrol_new;
    fn SOC_DAPM_ENUM(name: *const c_char, xenum: *const soc_enum) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MUX(name: *const c_char, reg: c_int, shift: c_uint, invert: c_uint, kcontrol: *const snd_kcontrol_new) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_PGA(name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint, controls: *const snd_kcontrol_new, num_controls: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_DAC(name: *const c_char, stream_name: *const c_char, reg: c_uint, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_AIF_OUT(name: *const c_char, stream_name: *const c_char, slot: c_uint, reg: c_uint, shift: c_uint, invert: c_uint) -> snd_soc_dapm_widget;
}

extern "C" {
    static WM8737_RESET: c_uint;
    static WM8737_MIC_PREAMP_CONTROL: c_uint;
    static WM8737_3D_ENHANCE: c_uint;
    static WM8737_ALC1: c_uint;
    static WM8737_ALC2: c_uint;
    static WM8737_ALC3: c_uint;
    static WM8737_AUDIO_PATH_L: c_uint;
    static WM8737_AUDIO_PATH_R: c_uint;
    static WM8737_LEFT_PGA_VOLUME: c_uint;
    static WM8737_RIGHT_PGA_VOLUME: c_uint;
    static WM8737_MISC_BIAS_CONTROL: c_uint;
    static WM8737_ADC_CONTROL: c_uint;
    static WM8737_NOISE_GATE: c_uint;
    static WM8737_POWER_MANAGEMENT: c_uint;
    static WM8737_AUDIO_FORMAT: c_uint;
    static WM8737_CLOCKING: c_uint;
    static WM8737_MAX_REGISTER: c_uint;
    static WM8737_CLKDIV2: c_uint;
    static WM8737_SR_SHIFT: c_uint;
    static WM8737_WL_MASK: c_uint;
    static WM8737_USB_MODE: c_uint;
    static WM8737_SR_MASK: c_uint;
    static WM8737_MS: c_uint;
    static WM8737_LRP: c_uint;
    static WM8737_FORMAT_MASK: c_uint;
    static WM8737_VMIDSEL_MASK: c_uint;
    static WM8737_VMIDSEL_SHIFT: c_uint;
    static WM8737_VMID_MASK: c_uint;
    static WM8737_VREF_MASK: c_uint;
    static WM8737_LVU: c_uint;
    static WM8737_RVU: c_uint;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
