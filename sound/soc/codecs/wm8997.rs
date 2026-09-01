// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8997.rs -- Rust source-level translation of wm8997.c
 *
 * This file intentionally preserves Linux/ASoC/Arizona dependency names as
 * unresolved Rust identifiers or macro invocations supplied by the surrounding
 * repository, matching the isolated C translation scope.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct wm8997_priv {
    pub core: arizona_priv,
    pub fll: [arizona_fll; 2],
}

declare_tlv_db_scale!(ana_tlv, 0, 100, 0);
declare_tlv_db_scale!(eq_tlv, -1200, 100, 0);
declare_tlv_db_scale!(digital_tlv, -6400, 50, 0);
declare_tlv_db_scale!(noise_tlv, -13200, 600, 0);
declare_tlv_db_scale!(ng_tlv, -10200, 600, 0);

pub const WM8997_RATES: c_uint = SNDRV_PCM_RATE_KNOT;
pub const WM8997_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
pub const WM8997_DIG_VU: c_uint = 0x0200;

unsafe extern "C" {
    static arizona_dai_ops: snd_soc_dai_ops;
    static arizona_simple_dai_ops: snd_soc_dai_ops;
    static arizona_in_vi_ramp: soc_enum;
    static arizona_in_vd_ramp: soc_enum;
    static arizona_lhpf1_mode: soc_enum;
    static arizona_lhpf2_mode: soc_enum;
    static arizona_lhpf3_mode: soc_enum;
    static arizona_lhpf4_mode: soc_enum;
    static arizona_out_vi_ramp: soc_enum;
    static arizona_out_vd_ramp: soc_enum;
    static arizona_ng_hold: soc_enum;
    static arizona_isrc_fsl: [soc_enum; 2];

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn regmap_write_async(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn arizona_clk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn arizona_dvfs_sysclk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn arizona_set_fll(fll: *mut arizona_fll, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int;
    fn arizona_set_fll_refclk(fll: *mut arizona_fll, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn arizona_init_spk(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn arizona_of_get_audio_pdata(arizona: *mut arizona) -> c_int;
    fn arizona_init_dvfs(core: *mut arizona_priv);
    fn arizona_jack_codec_dev_probe(core: *mut arizona_priv, dev: *mut device) -> c_int;
    fn arizona_init_fll(arizona: *mut arizona, id: c_int, base: c_uint, lock_irq: c_uint, ok_irq: c_uint, fll: *mut arizona_fll);
    fn arizona_init_dai(core: *mut arizona_priv, id: c_int);
    fn pm_runtime_enable(dev: *mut device) -> c_int;
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn arizona_init_common(arizona: *mut arizona);
    fn arizona_init_vol_limit(arizona: *mut arizona) -> c_int;
    fn arizona_init_spk_irqs(arizona: *mut arizona) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn arizona_free_spk_irqs(arizona: *mut arizona);
    fn arizona_jack_codec_dev_remove(core: *mut arizona_priv);
}

// C source translated/preserved: // SPDX-License-Identifier: GPL-2.0-only
// C source translated/preserved: /*
// C source translated/preserved:  * wm8997.c  --  WM8997 ALSA SoC Audio driver
// C source translated/preserved:  *
// C source translated/preserved:  * Copyright 2012 Wolfson Microelectronics plc
// C source translated/preserved:  *
// C source translated/preserved:  * Author: Charles Keepax <ckeepax@opensource.wolfsonmicro.com>
// C source translated/preserved:  */
// C source translated/preserved: 
// C source translated/preserved: 
// C source translated/preserved: 
// C source translated/preserved: 
// C source translated/preserved: struct wm8997_priv {
// C source translated/preserved: 	struct arizona_priv core;
// C source translated/preserved: 	struct arizona_fll fll[2];
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static DECLARE_TLV_DB_SCALE(ana_tlv, 0, 100, 0);
// C source translated/preserved: static DECLARE_TLV_DB_SCALE(eq_tlv, -1200, 100, 0);
// C source translated/preserved: static DECLARE_TLV_DB_SCALE(digital_tlv, -6400, 50, 0);
// C source translated/preserved: static DECLARE_TLV_DB_SCALE(noise_tlv, -13200, 600, 0);
// C source translated/preserved: static DECLARE_TLV_DB_SCALE(ng_tlv, -10200, 600, 0);
// C source translated/preserved: 
// C source translated/preserved: static const struct reg_default wm8997_sysclk_reva_patch[] = {
// C source translated/preserved: 	{ 0x301D, 0x7B15 },
// C source translated/preserved: 	{ 0x301B, 0x0050 },
// C source translated/preserved: 	{ 0x305D, 0x7B17 },
// C source translated/preserved: 	{ 0x305B, 0x0050 },
// C source translated/preserved: 	{ 0x3001, 0x08FE },
// C source translated/preserved: 	{ 0x3003, 0x00F4 },
// C source translated/preserved: 	{ 0x3041, 0x08FF },
// C source translated/preserved: 	{ 0x3043, 0x0005 },
// C source translated/preserved: 	{ 0x3020, 0x0225 },
// C source translated/preserved: 	{ 0x3021, 0x0A00 },
// C source translated/preserved: 	{ 0x3022, 0xE24D },
// C source translated/preserved: 	{ 0x3023, 0x0800 },
// C source translated/preserved: 	{ 0x3024, 0xE24D },
// C source translated/preserved: 	{ 0x3025, 0xF000 },
// C source translated/preserved: 	{ 0x3060, 0x0226 },
// C source translated/preserved: 	{ 0x3061, 0x0A00 },
// C source translated/preserved: 	{ 0x3062, 0xE252 },
// C source translated/preserved: 	{ 0x3063, 0x0800 },
// C source translated/preserved: 	{ 0x3064, 0xE252 },
// C source translated/preserved: 	{ 0x3065, 0xF000 },
// C source translated/preserved: 	{ 0x3116, 0x022B },
// C source translated/preserved: 	{ 0x3117, 0xFA00 },
// C source translated/preserved: 	{ 0x3110, 0x246C },
// C source translated/preserved: 	{ 0x3111, 0x0A03 },
// C source translated/preserved: 	{ 0x3112, 0x246E },
// C source translated/preserved: 	{ 0x3113, 0x0A03 },
// C source translated/preserved: 	{ 0x3114, 0x2470 },
// C source translated/preserved: 	{ 0x3115, 0x0A03 },
// C source translated/preserved: 	{ 0x3126, 0x246C },
// C source translated/preserved: 	{ 0x3127, 0x0A02 },
// C source translated/preserved: 	{ 0x3128, 0x246E },
// C source translated/preserved: 	{ 0x3129, 0x0A02 },
// C source translated/preserved: 	{ 0x312A, 0x2470 },
// C source translated/preserved: 	{ 0x312B, 0xFA02 },
// C source translated/preserved: 	{ 0x3125, 0x0800 },
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static int wm8997_sysclk_ev(struct snd_soc_dapm_widget *w,
// C source translated/preserved: 			    struct snd_kcontrol *kcontrol, int event)
// C source translated/preserved: {
// C source translated/preserved: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C source translated/preserved: 	struct arizona *arizona = dev_get_drvdata(component->dev->parent);
// C source translated/preserved: 	struct regmap *regmap = arizona->regmap;
// C source translated/preserved: 	const struct reg_default *patch = NULL;
// C source translated/preserved: 	int i, patch_size;
// C source translated/preserved: 
// C source translated/preserved: 	switch (arizona->rev) {
// C source translated/preserved: 	case 0:
// C source translated/preserved: 		patch = wm8997_sysclk_reva_patch;
// C source translated/preserved: 		patch_size = ARRAY_SIZE(wm8997_sysclk_reva_patch);
// C source translated/preserved: 		break;
// C source translated/preserved: 	default:
// C source translated/preserved: 		break;
// C source translated/preserved: 	}
// C source translated/preserved: 
// C source translated/preserved: 	switch (event) {
// C source translated/preserved: 	case SND_SOC_DAPM_POST_PMU:
// C source translated/preserved: 		if (patch)
// C source translated/preserved: 			for (i = 0; i < patch_size; i++)
// C source translated/preserved: 				regmap_write_async(regmap, patch[i].reg,
// C source translated/preserved: 						   patch[i].def);
// C source translated/preserved: 		break;
// C source translated/preserved: 	case SND_SOC_DAPM_PRE_PMD:
// C source translated/preserved: 		break;
// C source translated/preserved: 	case SND_SOC_DAPM_PRE_PMU:
// C source translated/preserved: 	case SND_SOC_DAPM_POST_PMD:
// C source translated/preserved: 		return arizona_clk_ev(w, kcontrol, event);
// C source translated/preserved: 	default:
// C source translated/preserved: 		return 0;
// C source translated/preserved: 	}
// C source translated/preserved: 
// C source translated/preserved: 	return arizona_dvfs_sysclk_ev(w, kcontrol, event);
// C source translated/preserved: }
// C source translated/preserved: 
// C source translated/preserved: static const char * const wm8997_osr_text[] = {
// C source translated/preserved: 	"Low power", "Normal", "High performance",
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static const unsigned int wm8997_osr_val[] = {
// C source translated/preserved: 	0x0, 0x3, 0x5,
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static const struct soc_enum wm8997_hpout_osr[] = {
// C source translated/preserved: 	SOC_VALUE_ENUM_SINGLE(ARIZONA_OUTPUT_PATH_CONFIG_1L,
// C source translated/preserved: 			      ARIZONA_OUT1_OSR_SHIFT, 0x7,
// C source translated/preserved: 			      ARRAY_SIZE(wm8997_osr_text),
// C source translated/preserved: 			      wm8997_osr_text, wm8997_osr_val),
// C source translated/preserved: 	SOC_VALUE_ENUM_SINGLE(ARIZONA_OUTPUT_PATH_CONFIG_3L,
// C source translated/preserved: 			      ARIZONA_OUT3_OSR_SHIFT, 0x7,
// C source translated/preserved: 			      ARRAY_SIZE(wm8997_osr_text),
// C source translated/preserved: 			      wm8997_osr_text, wm8997_osr_val),
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: #define WM8997_NG_SRC(name, base) \
// C source translated/preserved: 	SOC_SINGLE(name " NG HPOUT1L Switch",  base, 0, 1, 0), \
// C source translated/preserved: 	SOC_SINGLE(name " NG HPOUT1R Switch",  base, 1, 1, 0), \
// C source translated/preserved: 	SOC_SINGLE(name " NG EPOUT Switch",    base, 4, 1, 0), \
// C source translated/preserved: 	SOC_SINGLE(name " NG SPKOUT Switch",   base, 6, 1, 0), \
// C source translated/preserved: 	SOC_SINGLE(name " NG SPKDAT1L Switch", base, 8, 1, 0), \
// C source translated/preserved: 	SOC_SINGLE(name " NG SPKDAT1R Switch", base, 9, 1, 0)
// C source translated/preserved: 
// C source translated/preserved: static const struct snd_kcontrol_new wm8997_snd_controls[] = {
// C source translated/preserved: SOC_SINGLE("IN1 High Performance Switch", ARIZONA_IN1L_CONTROL,
// C source translated/preserved: 	   ARIZONA_IN1_OSR_SHIFT, 1, 0),
// C source translated/preserved: SOC_SINGLE("IN2 High Performance Switch", ARIZONA_IN2L_CONTROL,
// C source translated/preserved: 	   ARIZONA_IN2_OSR_SHIFT, 1, 0),
// C source translated/preserved: 
// C source translated/preserved: SOC_SINGLE_RANGE_TLV("IN1L Volume", ARIZONA_IN1L_CONTROL,
// C source translated/preserved: 		     ARIZONA_IN1L_PGA_VOL_SHIFT, 0x40, 0x5f, 0, ana_tlv),
// C source translated/preserved: SOC_SINGLE_RANGE_TLV("IN1R Volume", ARIZONA_IN1R_CONTROL,
// C source translated/preserved: 		     ARIZONA_IN1R_PGA_VOL_SHIFT, 0x40, 0x5f, 0, ana_tlv),
// C source translated/preserved: SOC_SINGLE_RANGE_TLV("IN2L Volume", ARIZONA_IN2L_CONTROL,
// C source translated/preserved: 		     ARIZONA_IN2L_PGA_VOL_SHIFT, 0x40, 0x5f, 0, ana_tlv),
// C source translated/preserved: SOC_SINGLE_RANGE_TLV("IN2R Volume", ARIZONA_IN2R_CONTROL,
// C source translated/preserved: 		     ARIZONA_IN2R_PGA_VOL_SHIFT, 0x40, 0x5f, 0, ana_tlv),
// C source translated/preserved: 
// C source translated/preserved: SOC_SINGLE_TLV("IN1L Digital Volume", ARIZONA_ADC_DIGITAL_VOLUME_1L,
// C source translated/preserved: 	       ARIZONA_IN1L_DIG_VOL_SHIFT, 0xbf, 0, digital_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("IN1R Digital Volume", ARIZONA_ADC_DIGITAL_VOLUME_1R,
// C source translated/preserved: 	       ARIZONA_IN1R_DIG_VOL_SHIFT, 0xbf, 0, digital_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("IN2L Digital Volume", ARIZONA_ADC_DIGITAL_VOLUME_2L,
// C source translated/preserved: 	       ARIZONA_IN2L_DIG_VOL_SHIFT, 0xbf, 0, digital_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("IN2R Digital Volume", ARIZONA_ADC_DIGITAL_VOLUME_2R,
// C source translated/preserved: 	       ARIZONA_IN2R_DIG_VOL_SHIFT, 0xbf, 0, digital_tlv),
// C source translated/preserved: 
// C source translated/preserved: SOC_ENUM("Input Ramp Up", arizona_in_vi_ramp),
// C source translated/preserved: SOC_ENUM("Input Ramp Down", arizona_in_vd_ramp),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("EQ1", ARIZONA_EQ1MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("EQ2", ARIZONA_EQ2MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("EQ3", ARIZONA_EQ3MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("EQ4", ARIZONA_EQ4MIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_EQ_CONTROL("EQ1 Coefficients", ARIZONA_EQ1_2),
// C source translated/preserved: SOC_SINGLE_TLV("EQ1 B1 Volume", ARIZONA_EQ1_1, ARIZONA_EQ1_B1_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ1 B2 Volume", ARIZONA_EQ1_1, ARIZONA_EQ1_B2_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ1 B3 Volume", ARIZONA_EQ1_1, ARIZONA_EQ1_B3_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ1 B4 Volume", ARIZONA_EQ1_2, ARIZONA_EQ1_B4_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ1 B5 Volume", ARIZONA_EQ1_2, ARIZONA_EQ1_B5_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_EQ_CONTROL("EQ2 Coefficients", ARIZONA_EQ2_2),
// C source translated/preserved: SOC_SINGLE_TLV("EQ2 B1 Volume", ARIZONA_EQ2_1, ARIZONA_EQ2_B1_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ2 B2 Volume", ARIZONA_EQ2_1, ARIZONA_EQ2_B2_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ2 B3 Volume", ARIZONA_EQ2_1, ARIZONA_EQ2_B3_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ2 B4 Volume", ARIZONA_EQ2_2, ARIZONA_EQ2_B4_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ2 B5 Volume", ARIZONA_EQ2_2, ARIZONA_EQ2_B5_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_EQ_CONTROL("EQ3 Coefficients", ARIZONA_EQ3_2),
// C source translated/preserved: SOC_SINGLE_TLV("EQ3 B1 Volume", ARIZONA_EQ3_1, ARIZONA_EQ3_B1_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ3 B2 Volume", ARIZONA_EQ3_1, ARIZONA_EQ3_B2_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ3 B3 Volume", ARIZONA_EQ3_1, ARIZONA_EQ3_B3_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ3 B4 Volume", ARIZONA_EQ3_2, ARIZONA_EQ3_B4_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ3 B5 Volume", ARIZONA_EQ3_2, ARIZONA_EQ3_B5_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_EQ_CONTROL("EQ4 Coefficients", ARIZONA_EQ4_2),
// C source translated/preserved: SOC_SINGLE_TLV("EQ4 B1 Volume", ARIZONA_EQ4_1, ARIZONA_EQ4_B1_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ4 B2 Volume", ARIZONA_EQ4_1, ARIZONA_EQ4_B2_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ4 B3 Volume", ARIZONA_EQ4_1, ARIZONA_EQ4_B3_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ4 B4 Volume", ARIZONA_EQ4_2, ARIZONA_EQ4_B4_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EQ4 B5 Volume", ARIZONA_EQ4_2, ARIZONA_EQ4_B5_GAIN_SHIFT,
// C source translated/preserved: 	       24, 0, eq_tlv),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("DRC1L", ARIZONA_DRC1LMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("DRC1R", ARIZONA_DRC1RMIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_BYTES_MASK("DRC1", ARIZONA_DRC1_CTRL1, 5,
// C source translated/preserved: 		   ARIZONA_DRC1R_ENA | ARIZONA_DRC1L_ENA),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("LHPF1", ARIZONA_HPLP1MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("LHPF2", ARIZONA_HPLP2MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("LHPF3", ARIZONA_HPLP3MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("LHPF4", ARIZONA_HPLP4MIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: SOC_ENUM("LHPF1 Mode", arizona_lhpf1_mode),
// C source translated/preserved: SOC_ENUM("LHPF2 Mode", arizona_lhpf2_mode),
// C source translated/preserved: SOC_ENUM("LHPF3 Mode", arizona_lhpf3_mode),
// C source translated/preserved: SOC_ENUM("LHPF4 Mode", arizona_lhpf4_mode),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_LHPF_CONTROL("LHPF1 Coefficients", ARIZONA_HPLPF1_2),
// C source translated/preserved: ARIZONA_LHPF_CONTROL("LHPF2 Coefficients", ARIZONA_HPLPF2_2),
// C source translated/preserved: ARIZONA_LHPF_CONTROL("LHPF3 Coefficients", ARIZONA_HPLPF3_2),
// C source translated/preserved: ARIZONA_LHPF_CONTROL("LHPF4 Coefficients", ARIZONA_HPLPF4_2),
// C source translated/preserved: 
// C source translated/preserved: SOC_ENUM("ISRC1 FSL", arizona_isrc_fsl[0]),
// C source translated/preserved: SOC_ENUM("ISRC2 FSL", arizona_isrc_fsl[1]),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("Mic", ARIZONA_MICMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("Noise", ARIZONA_NOISEMIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: SOC_SINGLE_TLV("Noise Generator Volume", ARIZONA_COMFORT_NOISE_GENERATOR,
// C source translated/preserved: 	       ARIZONA_NOISE_GEN_GAIN_SHIFT, 0x16, 0, noise_tlv),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("HPOUT1L", ARIZONA_OUT1LMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("HPOUT1R", ARIZONA_OUT1RMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("EPOUT", ARIZONA_OUT3LMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SPKOUT", ARIZONA_OUT4LMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SPKDAT1L", ARIZONA_OUT5LMIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SPKDAT1R", ARIZONA_OUT5RMIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: SOC_SINGLE("Speaker High Performance Switch", ARIZONA_OUTPUT_PATH_CONFIG_4L,
// C source translated/preserved: 	   ARIZONA_OUT4_OSR_SHIFT, 1, 0),
// C source translated/preserved: SOC_SINGLE("SPKDAT1 High Performance Switch", ARIZONA_OUTPUT_PATH_CONFIG_5L,
// C source translated/preserved: 	   ARIZONA_OUT5_OSR_SHIFT, 1, 0),
// C source translated/preserved: 
// C source translated/preserved: SOC_DOUBLE_R("HPOUT1 Digital Switch", ARIZONA_DAC_DIGITAL_VOLUME_1L,
// C source translated/preserved: 	     ARIZONA_DAC_DIGITAL_VOLUME_1R, ARIZONA_OUT1L_MUTE_SHIFT, 1, 1),
// C source translated/preserved: SOC_SINGLE("EPOUT Digital Switch", ARIZONA_DAC_DIGITAL_VOLUME_3L,
// C source translated/preserved: 	   ARIZONA_OUT3L_MUTE_SHIFT, 1, 1),
// C source translated/preserved: SOC_SINGLE("Speaker Digital Switch", ARIZONA_DAC_DIGITAL_VOLUME_4L,
// C source translated/preserved: 	   ARIZONA_OUT4L_MUTE_SHIFT, 1, 1),
// C source translated/preserved: SOC_DOUBLE_R("SPKDAT1 Digital Switch", ARIZONA_DAC_DIGITAL_VOLUME_5L,
// C source translated/preserved: 	     ARIZONA_DAC_DIGITAL_VOLUME_5R, ARIZONA_OUT5L_MUTE_SHIFT, 1, 1),
// C source translated/preserved: 
// C source translated/preserved: SOC_DOUBLE_R_TLV("HPOUT1 Digital Volume", ARIZONA_DAC_DIGITAL_VOLUME_1L,
// C source translated/preserved: 		 ARIZONA_DAC_DIGITAL_VOLUME_1R, ARIZONA_OUT1L_VOL_SHIFT,
// C source translated/preserved: 		 0xbf, 0, digital_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("EPOUT Digital Volume", ARIZONA_DAC_DIGITAL_VOLUME_3L,
// C source translated/preserved: 	       ARIZONA_OUT3L_VOL_SHIFT, 0xbf, 0, digital_tlv),
// C source translated/preserved: SOC_SINGLE_TLV("Speaker Digital Volume", ARIZONA_DAC_DIGITAL_VOLUME_4L,
// C source translated/preserved: 	       ARIZONA_OUT4L_VOL_SHIFT, 0xbf, 0, digital_tlv),
// C source translated/preserved: SOC_DOUBLE_R_TLV("SPKDAT1 Digital Volume", ARIZONA_DAC_DIGITAL_VOLUME_5L,
// C source translated/preserved: 		 ARIZONA_DAC_DIGITAL_VOLUME_5R, ARIZONA_OUT5L_VOL_SHIFT,
// C source translated/preserved: 		 0xbf, 0, digital_tlv),
// C source translated/preserved: 
// C source translated/preserved: SOC_ENUM("HPOUT1 OSR", wm8997_hpout_osr[0]),
// C source translated/preserved: SOC_ENUM("EPOUT OSR", wm8997_hpout_osr[1]),
// C source translated/preserved: 
// C source translated/preserved: SOC_ENUM("Output Ramp Up", arizona_out_vi_ramp),
// C source translated/preserved: SOC_ENUM("Output Ramp Down", arizona_out_vd_ramp),
// C source translated/preserved: 
// C source translated/preserved: SOC_DOUBLE("SPKDAT1 Switch", ARIZONA_PDM_SPK1_CTRL_1, ARIZONA_SPK1L_MUTE_SHIFT,
// C source translated/preserved: 	   ARIZONA_SPK1R_MUTE_SHIFT, 1, 1),
// C source translated/preserved: 
// C source translated/preserved: SOC_SINGLE("Noise Gate Switch", ARIZONA_NOISE_GATE_CONTROL,
// C source translated/preserved: 	   ARIZONA_NGATE_ENA_SHIFT, 1, 0),
// C source translated/preserved: SOC_SINGLE_TLV("Noise Gate Threshold Volume", ARIZONA_NOISE_GATE_CONTROL,
// C source translated/preserved: 	       ARIZONA_NGATE_THR_SHIFT, 7, 1, ng_tlv),
// C source translated/preserved: SOC_ENUM("Noise Gate Hold", arizona_ng_hold),
// C source translated/preserved: 
// C source translated/preserved: WM8997_NG_SRC("HPOUT1L", ARIZONA_NOISE_GATE_SELECT_1L),
// C source translated/preserved: WM8997_NG_SRC("HPOUT1R", ARIZONA_NOISE_GATE_SELECT_1R),
// C source translated/preserved: WM8997_NG_SRC("EPOUT", ARIZONA_NOISE_GATE_SELECT_3L),
// C source translated/preserved: WM8997_NG_SRC("SPKOUT", ARIZONA_NOISE_GATE_SELECT_4L),
// C source translated/preserved: WM8997_NG_SRC("SPKDAT1L", ARIZONA_NOISE_GATE_SELECT_5L),
// C source translated/preserved: WM8997_NG_SRC("SPKDAT1R", ARIZONA_NOISE_GATE_SELECT_5R),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX1", ARIZONA_AIF1TX1MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX2", ARIZONA_AIF1TX2MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX3", ARIZONA_AIF1TX3MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX4", ARIZONA_AIF1TX4MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX5", ARIZONA_AIF1TX5MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX6", ARIZONA_AIF1TX6MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX7", ARIZONA_AIF1TX7MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF1TX8", ARIZONA_AIF1TX8MIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF2TX1", ARIZONA_AIF2TX1MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("AIF2TX2", ARIZONA_AIF2TX2MIX_INPUT_1_SOURCE),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX1", ARIZONA_SLIMTX1MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX2", ARIZONA_SLIMTX2MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX3", ARIZONA_SLIMTX3MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX4", ARIZONA_SLIMTX4MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX5", ARIZONA_SLIMTX5MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX6", ARIZONA_SLIMTX6MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX7", ARIZONA_SLIMTX7MIX_INPUT_1_SOURCE),
// C source translated/preserved: ARIZONA_MIXER_CONTROLS("SLIMTX8", ARIZONA_SLIMTX8MIX_INPUT_1_SOURCE),
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(EQ1, ARIZONA_EQ1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(EQ2, ARIZONA_EQ2MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(EQ3, ARIZONA_EQ3MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(EQ4, ARIZONA_EQ4MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(DRC1L, ARIZONA_DRC1LMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(DRC1R, ARIZONA_DRC1RMIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(LHPF1, ARIZONA_HPLP1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(LHPF2, ARIZONA_HPLP2MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(LHPF3, ARIZONA_HPLP3MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(LHPF4, ARIZONA_HPLP4MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(Mic, ARIZONA_MICMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(Noise, ARIZONA_NOISEMIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(PWM1, ARIZONA_PWM1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(PWM2, ARIZONA_PWM2MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(OUT1L, ARIZONA_OUT1LMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(OUT1R, ARIZONA_OUT1RMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(OUT3, ARIZONA_OUT3LMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SPKOUT, ARIZONA_OUT4LMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SPKDAT1L, ARIZONA_OUT5LMIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SPKDAT1R, ARIZONA_OUT5RMIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX1, ARIZONA_AIF1TX1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX2, ARIZONA_AIF1TX2MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX3, ARIZONA_AIF1TX3MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX4, ARIZONA_AIF1TX4MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX5, ARIZONA_AIF1TX5MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX6, ARIZONA_AIF1TX6MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX7, ARIZONA_AIF1TX7MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF1TX8, ARIZONA_AIF1TX8MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF2TX1, ARIZONA_AIF2TX1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(AIF2TX2, ARIZONA_AIF2TX2MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX1, ARIZONA_SLIMTX1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX2, ARIZONA_SLIMTX2MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX3, ARIZONA_SLIMTX3MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX4, ARIZONA_SLIMTX4MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX5, ARIZONA_SLIMTX5MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX6, ARIZONA_SLIMTX6MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX7, ARIZONA_SLIMTX7MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MIXER_ENUMS(SLIMTX8, ARIZONA_SLIMTX8MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC1INT1, ARIZONA_ISRC1INT1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC1INT2, ARIZONA_ISRC1INT2MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC1DEC1, ARIZONA_ISRC1DEC1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC1DEC2, ARIZONA_ISRC1DEC2MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC2INT1, ARIZONA_ISRC2INT1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC2INT2, ARIZONA_ISRC2INT2MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC2DEC1, ARIZONA_ISRC2DEC1MIX_INPUT_1_SOURCE);
// C source translated/preserved: ARIZONA_MUX_ENUMS(ISRC2DEC2, ARIZONA_ISRC2DEC2MIX_INPUT_1_SOURCE);
// C source translated/preserved: 
// C source translated/preserved: static const char *wm8997_aec_loopback_texts[] = {
// C source translated/preserved: 	"HPOUT1L", "HPOUT1R", "EPOUT", "SPKOUT", "SPKDAT1L", "SPKDAT1R",
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static const unsigned int wm8997_aec_loopback_values[] = {
// C source translated/preserved: 	0, 1, 4, 6, 8, 9,
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static const struct soc_enum wm8997_aec_loopback =
// C source translated/preserved: 	SOC_VALUE_ENUM_SINGLE(ARIZONA_DAC_AEC_CONTROL_1,
// C source translated/preserved: 			      ARIZONA_AEC_LOOPBACK_SRC_SHIFT, 0xf,
// C source translated/preserved: 			      ARRAY_SIZE(wm8997_aec_loopback_texts),
// C source translated/preserved: 			      wm8997_aec_loopback_texts,
// C source translated/preserved: 			      wm8997_aec_loopback_values);
// C source translated/preserved: 
// C source translated/preserved: static const struct snd_kcontrol_new wm8997_aec_loopback_mux =
// C source translated/preserved: 	SOC_DAPM_ENUM("AEC Loopback", wm8997_aec_loopback);
// C source translated/preserved: 
// C source translated/preserved: static const struct snd_soc_dapm_widget wm8997_dapm_widgets[] = {
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("SYSCLK", ARIZONA_SYSTEM_CLOCK_1, ARIZONA_SYSCLK_ENA_SHIFT,
// C source translated/preserved: 		    0, wm8997_sysclk_ev,
// C source translated/preserved: 		    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD |
// C source translated/preserved: 		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("ASYNCCLK", ARIZONA_ASYNC_CLOCK_1,
// C source translated/preserved: 		    ARIZONA_ASYNC_CLK_ENA_SHIFT, 0, arizona_clk_ev,
// C source translated/preserved: 		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("OPCLK", ARIZONA_OUTPUT_SYSTEM_CLOCK,
// C source translated/preserved: 		    ARIZONA_OPCLK_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("ASYNCOPCLK", ARIZONA_OUTPUT_ASYNC_CLOCK,
// C source translated/preserved: 		    ARIZONA_OPCLK_ASYNC_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_REGULATOR_SUPPLY("DBVDD2", 0, 0),
// C source translated/preserved: SND_SOC_DAPM_REGULATOR_SUPPLY("CPVDD", 20, 0),
// C source translated/preserved: SND_SOC_DAPM_REGULATOR_SUPPLY("MICVDD", 0, SND_SOC_DAPM_REGULATOR_BYPASS),
// C source translated/preserved: SND_SOC_DAPM_REGULATOR_SUPPLY("SPKVDD", 0, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_SIGGEN("TONE"),
// C source translated/preserved: SND_SOC_DAPM_SIGGEN("NOISE"),
// C source translated/preserved: SND_SOC_DAPM_SIGGEN("HAPTICS"),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_INPUT("IN1L"),
// C source translated/preserved: SND_SOC_DAPM_INPUT("IN1R"),
// C source translated/preserved: SND_SOC_DAPM_INPUT("IN2L"),
// C source translated/preserved: SND_SOC_DAPM_INPUT("IN2R"),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA_E("IN1L PGA", ARIZONA_INPUT_ENABLES, ARIZONA_IN1L_ENA_SHIFT,
// C source translated/preserved: 		   0, NULL, 0, arizona_in_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("IN1R PGA", ARIZONA_INPUT_ENABLES, ARIZONA_IN1R_ENA_SHIFT,
// C source translated/preserved: 		   0, NULL, 0, arizona_in_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("IN2L PGA", ARIZONA_INPUT_ENABLES, ARIZONA_IN2L_ENA_SHIFT,
// C source translated/preserved: 		   0, NULL, 0, arizona_in_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("IN2R PGA", ARIZONA_INPUT_ENABLES, ARIZONA_IN2R_ENA_SHIFT,
// C source translated/preserved: 		   0, NULL, 0, arizona_in_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("MICBIAS1", ARIZONA_MIC_BIAS_CTRL_1,
// C source translated/preserved: 		    ARIZONA_MICB1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("MICBIAS2", ARIZONA_MIC_BIAS_CTRL_2,
// C source translated/preserved: 		    ARIZONA_MICB2_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_SUPPLY("MICBIAS3", ARIZONA_MIC_BIAS_CTRL_3,
// C source translated/preserved: 		    ARIZONA_MICB3_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("Noise Generator", ARIZONA_COMFORT_NOISE_GENERATOR,
// C source translated/preserved: 		 ARIZONA_NOISE_GEN_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("Tone Generator 1", ARIZONA_TONE_GENERATOR_1,
// C source translated/preserved: 		 ARIZONA_TONE1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("Tone Generator 2", ARIZONA_TONE_GENERATOR_1,
// C source translated/preserved: 		 ARIZONA_TONE2_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("Mic Mute Mixer", ARIZONA_MIC_NOISE_MIX_CONTROL_1,
// C source translated/preserved: 		 ARIZONA_MICMUTE_MIX_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("EQ1", ARIZONA_EQ1_1, ARIZONA_EQ1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("EQ2", ARIZONA_EQ2_1, ARIZONA_EQ2_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("EQ3", ARIZONA_EQ3_1, ARIZONA_EQ3_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("EQ4", ARIZONA_EQ4_1, ARIZONA_EQ4_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("DRC1L", ARIZONA_DRC1_CTRL1, ARIZONA_DRC1L_ENA_SHIFT, 0,
// C source translated/preserved: 		 NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("DRC1R", ARIZONA_DRC1_CTRL1, ARIZONA_DRC1R_ENA_SHIFT, 0,
// C source translated/preserved: 		 NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("LHPF1", ARIZONA_HPLPF1_1, ARIZONA_LHPF1_ENA_SHIFT, 0,
// C source translated/preserved: 		 NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("LHPF2", ARIZONA_HPLPF2_1, ARIZONA_LHPF2_ENA_SHIFT, 0,
// C source translated/preserved: 		 NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("LHPF3", ARIZONA_HPLPF3_1, ARIZONA_LHPF3_ENA_SHIFT, 0,
// C source translated/preserved: 		 NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("LHPF4", ARIZONA_HPLPF4_1, ARIZONA_LHPF4_ENA_SHIFT, 0,
// C source translated/preserved: 		 NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("PWM1 Driver", ARIZONA_PWM_DRIVE_1, ARIZONA_PWM1_ENA_SHIFT,
// C source translated/preserved: 		 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("PWM2 Driver", ARIZONA_PWM_DRIVE_1, ARIZONA_PWM2_ENA_SHIFT,
// C source translated/preserved: 		 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC1INT1", ARIZONA_ISRC_1_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC1_INT0_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC1INT2", ARIZONA_ISRC_1_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC1_INT1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC1DEC1", ARIZONA_ISRC_1_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC1_DEC0_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC1DEC2", ARIZONA_ISRC_1_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC1_DEC1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC2INT1", ARIZONA_ISRC_2_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC2_INT0_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC2INT2", ARIZONA_ISRC_2_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC2_INT1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC2DEC1", ARIZONA_ISRC_2_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC2_DEC0_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: SND_SOC_DAPM_PGA("ISRC2DEC2", ARIZONA_ISRC_2_CTRL_3,
// C source translated/preserved: 		 ARIZONA_ISRC2_DEC1_ENA_SHIFT, 0, NULL, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX1", NULL, 0,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX1_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX2", NULL, 1,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX2_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX3", NULL, 2,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX3_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX4", NULL, 3,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX4_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX5", NULL, 4,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX5_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX6", NULL, 5,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX6_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX7", NULL, 6,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX7_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF1TX8", NULL, 7,
// C source translated/preserved: 		     ARIZONA_AIF1_TX_ENABLES, ARIZONA_AIF1TX8_ENA_SHIFT, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX1", NULL, 0,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX1_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX2", NULL, 1,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX2_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX3", NULL, 2,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX3_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX4", NULL, 3,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX4_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX5", NULL, 4,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX5_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX6", NULL, 5,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX6_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX7", NULL, 6,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX7_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF1RX8", NULL, 7,
// C source translated/preserved: 		    ARIZONA_AIF1_RX_ENABLES, ARIZONA_AIF1RX8_ENA_SHIFT, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF2TX1", NULL, 0,
// C source translated/preserved: 		     ARIZONA_AIF2_TX_ENABLES, ARIZONA_AIF2TX1_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("AIF2TX2", NULL, 1,
// C source translated/preserved: 		     ARIZONA_AIF2_TX_ENABLES, ARIZONA_AIF2TX2_ENA_SHIFT, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF2RX1", NULL, 0,
// C source translated/preserved: 		    ARIZONA_AIF2_RX_ENABLES, ARIZONA_AIF2RX1_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("AIF2RX2", NULL, 1,
// C source translated/preserved: 		    ARIZONA_AIF2_RX_ENABLES, ARIZONA_AIF2RX2_ENA_SHIFT, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX1", NULL, 0,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX1_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX2", NULL, 1,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX2_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX3", NULL, 2,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX3_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX4", NULL, 3,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX4_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX5", NULL, 4,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX5_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX6", NULL, 5,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX6_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX7", NULL, 6,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX7_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_OUT("SLIMTX8", NULL, 7,
// C source translated/preserved: 		     ARIZONA_SLIMBUS_TX_CHANNEL_ENABLE,
// C source translated/preserved: 		     ARIZONA_SLIMTX8_ENA_SHIFT, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX1", NULL, 0,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX1_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX2", NULL, 1,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX2_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX3", NULL, 2,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX3_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX4", NULL, 3,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX4_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX5", NULL, 4,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX5_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX6", NULL, 5,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX6_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX7", NULL, 6,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX7_ENA_SHIFT, 0),
// C source translated/preserved: SND_SOC_DAPM_AIF_IN("SLIMRX8", NULL, 7,
// C source translated/preserved: 		    ARIZONA_SLIMBUS_RX_CHANNEL_ENABLE,
// C source translated/preserved: 		    ARIZONA_SLIMRX8_ENA_SHIFT, 0),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_MUX("AEC Loopback", ARIZONA_DAC_AEC_CONTROL_1,
// C source translated/preserved: 		 ARIZONA_AEC_LOOPBACK_ENA_SHIFT, 0, &wm8997_aec_loopback_mux),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_PGA_E("OUT1L", SND_SOC_NOPM,
// C source translated/preserved: 		   ARIZONA_OUT1L_ENA_SHIFT, 0, NULL, 0, arizona_hp_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("OUT1R", SND_SOC_NOPM,
// C source translated/preserved: 		   ARIZONA_OUT1R_ENA_SHIFT, 0, NULL, 0, arizona_hp_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("OUT3L", ARIZONA_OUTPUT_ENABLES_1,
// C source translated/preserved: 		   ARIZONA_OUT3L_ENA_SHIFT, 0, NULL, 0, arizona_out_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("OUT5L", ARIZONA_OUTPUT_ENABLES_1,
// C source translated/preserved: 		   ARIZONA_OUT5L_ENA_SHIFT, 0, NULL, 0, arizona_out_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: SND_SOC_DAPM_PGA_E("OUT5R", ARIZONA_OUTPUT_ENABLES_1,
// C source translated/preserved: 		   ARIZONA_OUT5R_ENA_SHIFT, 0, NULL, 0, arizona_out_ev,
// C source translated/preserved: 		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(EQ1, "EQ1"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(EQ2, "EQ2"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(EQ3, "EQ3"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(EQ4, "EQ4"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(DRC1L, "DRC1L"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(DRC1R, "DRC1R"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(LHPF1, "LHPF1"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(LHPF2, "LHPF2"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(LHPF3, "LHPF3"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(LHPF4, "LHPF4"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(Mic, "Mic"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(Noise, "Noise"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(PWM1, "PWM1"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(PWM2, "PWM2"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(OUT1L, "HPOUT1L"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(OUT1R, "HPOUT1R"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(OUT3, "EPOUT"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SPKOUT, "SPKOUT"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SPKDAT1L, "SPKDAT1L"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SPKDAT1R, "SPKDAT1R"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX1, "AIF1TX1"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX2, "AIF1TX2"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX3, "AIF1TX3"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX4, "AIF1TX4"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX5, "AIF1TX5"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX6, "AIF1TX6"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX7, "AIF1TX7"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF1TX8, "AIF1TX8"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF2TX1, "AIF2TX1"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(AIF2TX2, "AIF2TX2"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX1, "SLIMTX1"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX2, "SLIMTX2"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX3, "SLIMTX3"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX4, "SLIMTX4"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX5, "SLIMTX5"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX6, "SLIMTX6"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX7, "SLIMTX7"),
// C source translated/preserved: ARIZONA_MIXER_WIDGETS(SLIMTX8, "SLIMTX8"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC1DEC1, "ISRC1DEC1"),
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC1DEC2, "ISRC1DEC2"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC1INT1, "ISRC1INT1"),
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC1INT2, "ISRC1INT2"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC2DEC1, "ISRC2DEC1"),
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC2DEC2, "ISRC2DEC2"),
// C source translated/preserved: 
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC2INT1, "ISRC2INT1"),
// C source translated/preserved: ARIZONA_MUX_WIDGETS(ISRC2INT2, "ISRC2INT2"),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("HPOUT1L"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("HPOUT1R"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("EPOUTN"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("EPOUTP"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("SPKOUTN"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("SPKOUTP"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("SPKDAT1L"),
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("SPKDAT1R"),
// C source translated/preserved: 
// C source translated/preserved: SND_SOC_DAPM_OUTPUT("MICSUPP"),
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: #define ARIZONA_MIXER_INPUT_ROUTES(name)	\
// C source translated/preserved: 	{ name, "Noise Generator", "Noise Generator" }, \
// C source translated/preserved: 	{ name, "Tone Generator 1", "Tone Generator 1" }, \
// C source translated/preserved: 	{ name, "Tone Generator 2", "Tone Generator 2" }, \
// C source translated/preserved: 	{ name, "Haptics", "HAPTICS" }, \
// C source translated/preserved: 	{ name, "AEC", "AEC Loopback" }, \
// C source translated/preserved: 	{ name, "IN1L", "IN1L PGA" }, \
// C source translated/preserved: 	{ name, "IN1R", "IN1R PGA" }, \
// C source translated/preserved: 	{ name, "IN2L", "IN2L PGA" }, \
// C source translated/preserved: 	{ name, "IN2R", "IN2R PGA" }, \
// C source translated/preserved: 	{ name, "Mic Mute Mixer", "Mic Mute Mixer" }, \
// C source translated/preserved: 	{ name, "AIF1RX1", "AIF1RX1" }, \
// C source translated/preserved: 	{ name, "AIF1RX2", "AIF1RX2" }, \
// C source translated/preserved: 	{ name, "AIF1RX3", "AIF1RX3" }, \
// C source translated/preserved: 	{ name, "AIF1RX4", "AIF1RX4" }, \
// C source translated/preserved: 	{ name, "AIF1RX5", "AIF1RX5" }, \
// C source translated/preserved: 	{ name, "AIF1RX6", "AIF1RX6" }, \
// C source translated/preserved: 	{ name, "AIF1RX7", "AIF1RX7" }, \
// C source translated/preserved: 	{ name, "AIF1RX8", "AIF1RX8" }, \
// C source translated/preserved: 	{ name, "AIF2RX1", "AIF2RX1" }, \
// C source translated/preserved: 	{ name, "AIF2RX2", "AIF2RX2" }, \
// C source translated/preserved: 	{ name, "SLIMRX1", "SLIMRX1" }, \
// C source translated/preserved: 	{ name, "SLIMRX2", "SLIMRX2" }, \
// C source translated/preserved: 	{ name, "SLIMRX3", "SLIMRX3" }, \
// C source translated/preserved: 	{ name, "SLIMRX4", "SLIMRX4" }, \
// C source translated/preserved: 	{ name, "SLIMRX5", "SLIMRX5" }, \
// C source translated/preserved: 	{ name, "SLIMRX6", "SLIMRX6" }, \
// C source translated/preserved: 	{ name, "SLIMRX7", "SLIMRX7" }, \
// C source translated/preserved: 	{ name, "SLIMRX8", "SLIMRX8" }, \
// C source translated/preserved: 	{ name, "EQ1", "EQ1" }, \
// C source translated/preserved: 	{ name, "EQ2", "EQ2" }, \
// C source translated/preserved: 	{ name, "EQ3", "EQ3" }, \
// C source translated/preserved: 	{ name, "EQ4", "EQ4" }, \
// C source translated/preserved: 	{ name, "DRC1L", "DRC1L" }, \
// C source translated/preserved: 	{ name, "DRC1R", "DRC1R" }, \
// C source translated/preserved: 	{ name, "LHPF1", "LHPF1" }, \
// C source translated/preserved: 	{ name, "LHPF2", "LHPF2" }, \
// C source translated/preserved: 	{ name, "LHPF3", "LHPF3" }, \
// C source translated/preserved: 	{ name, "LHPF4", "LHPF4" }, \
// C source translated/preserved: 	{ name, "ISRC1DEC1", "ISRC1DEC1" }, \
// C source translated/preserved: 	{ name, "ISRC1DEC2", "ISRC1DEC2" }, \
// C source translated/preserved: 	{ name, "ISRC1INT1", "ISRC1INT1" }, \
// C source translated/preserved: 	{ name, "ISRC1INT2", "ISRC1INT2" }, \
// C source translated/preserved: 	{ name, "ISRC2DEC1", "ISRC2DEC1" }, \
// C source translated/preserved: 	{ name, "ISRC2DEC2", "ISRC2DEC2" }, \
// C source translated/preserved: 	{ name, "ISRC2INT1", "ISRC2INT1" }, \
// C source translated/preserved: 	{ name, "ISRC2INT2", "ISRC2INT2" }
// C source translated/preserved: 
// C source translated/preserved: static const struct snd_soc_dapm_route wm8997_dapm_routes[] = {
// C source translated/preserved: 	{ "AIF2 Capture", NULL, "DBVDD2" },
// C source translated/preserved: 	{ "AIF2 Playback", NULL, "DBVDD2" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "OUT1L", NULL, "CPVDD" },
// C source translated/preserved: 	{ "OUT1R", NULL, "CPVDD" },
// C source translated/preserved: 	{ "OUT3L", NULL, "CPVDD" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "OUT4L", NULL, "SPKVDD" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "OUT1L", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "OUT1R", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "OUT3L", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "OUT4L", NULL, "SYSCLK" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "IN1L", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "IN1R", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "IN2L", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "IN2R", NULL, "SYSCLK" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "MICBIAS1", NULL, "MICVDD" },
// C source translated/preserved: 	{ "MICBIAS2", NULL, "MICVDD" },
// C source translated/preserved: 	{ "MICBIAS3", NULL, "MICVDD" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "Noise Generator", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Tone Generator 1", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Tone Generator 2", NULL, "SYSCLK" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "Noise Generator", NULL, "NOISE" },
// C source translated/preserved: 	{ "Tone Generator 1", NULL, "TONE" },
// C source translated/preserved: 	{ "Tone Generator 2", NULL, "TONE" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX1" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX2" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX3" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX4" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX5" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX6" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX7" },
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "AIF1TX8" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AIF1RX1", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX2", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX3", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX4", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX5", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX6", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX7", NULL, "AIF1 Playback" },
// C source translated/preserved: 	{ "AIF1RX8", NULL, "AIF1 Playback" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AIF2 Capture", NULL, "AIF2TX1" },
// C source translated/preserved: 	{ "AIF2 Capture", NULL, "AIF2TX2" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AIF2RX1", NULL, "AIF2 Playback" },
// C source translated/preserved: 	{ "AIF2RX2", NULL, "AIF2 Playback" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "Slim1 Capture", NULL, "SLIMTX1" },
// C source translated/preserved: 	{ "Slim1 Capture", NULL, "SLIMTX2" },
// C source translated/preserved: 	{ "Slim1 Capture", NULL, "SLIMTX3" },
// C source translated/preserved: 	{ "Slim1 Capture", NULL, "SLIMTX4" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "SLIMRX1", NULL, "Slim1 Playback" },
// C source translated/preserved: 	{ "SLIMRX2", NULL, "Slim1 Playback" },
// C source translated/preserved: 	{ "SLIMRX3", NULL, "Slim1 Playback" },
// C source translated/preserved: 	{ "SLIMRX4", NULL, "Slim1 Playback" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "Slim2 Capture", NULL, "SLIMTX5" },
// C source translated/preserved: 	{ "Slim2 Capture", NULL, "SLIMTX6" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "SLIMRX5", NULL, "Slim2 Playback" },
// C source translated/preserved: 	{ "SLIMRX6", NULL, "Slim2 Playback" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "Slim3 Capture", NULL, "SLIMTX7" },
// C source translated/preserved: 	{ "Slim3 Capture", NULL, "SLIMTX8" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "SLIMRX7", NULL, "Slim3 Playback" },
// C source translated/preserved: 	{ "SLIMRX8", NULL, "Slim3 Playback" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AIF1 Playback", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "AIF2 Playback", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Slim1 Playback", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Slim2 Playback", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Slim3 Playback", NULL, "SYSCLK" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AIF1 Capture", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "AIF2 Capture", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Slim1 Capture", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Slim2 Capture", NULL, "SYSCLK" },
// C source translated/preserved: 	{ "Slim3 Capture", NULL, "SYSCLK" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "IN1L PGA", NULL, "IN1L" },
// C source translated/preserved: 	{ "IN1R PGA", NULL, "IN1R" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "IN2L PGA", NULL, "IN2L" },
// C source translated/preserved: 	{ "IN2R PGA", NULL, "IN2R" },
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("OUT1L", "HPOUT1L"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("OUT1R", "HPOUT1R"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("OUT3L", "EPOUT"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("OUT4L", "SPKOUT"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("OUT5L", "SPKDAT1L"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("OUT5R", "SPKDAT1R"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("PWM1 Driver", "PWM1"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("PWM2 Driver", "PWM2"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX1", "AIF1TX1"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX2", "AIF1TX2"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX3", "AIF1TX3"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX4", "AIF1TX4"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX5", "AIF1TX5"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX6", "AIF1TX6"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX7", "AIF1TX7"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF1TX8", "AIF1TX8"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF2TX1", "AIF2TX1"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("AIF2TX2", "AIF2TX2"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX1", "SLIMTX1"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX2", "SLIMTX2"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX3", "SLIMTX3"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX4", "SLIMTX4"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX5", "SLIMTX5"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX6", "SLIMTX6"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX7", "SLIMTX7"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("SLIMTX8", "SLIMTX8"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("EQ1", "EQ1"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("EQ2", "EQ2"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("EQ3", "EQ3"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("EQ4", "EQ4"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("DRC1L", "DRC1L"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("DRC1R", "DRC1R"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("LHPF1", "LHPF1"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("LHPF2", "LHPF2"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("LHPF3", "LHPF3"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("LHPF4", "LHPF4"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("Mic Mute Mixer", "Noise"),
// C source translated/preserved: 	ARIZONA_MIXER_ROUTES("Mic Mute Mixer", "Mic"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC1INT1", "ISRC1INT1"),
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC1INT2", "ISRC1INT2"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC1DEC1", "ISRC1DEC1"),
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC1DEC2", "ISRC1DEC2"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC2INT1", "ISRC2INT1"),
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC2INT2", "ISRC2INT2"),
// C source translated/preserved: 
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC2DEC1", "ISRC2DEC1"),
// C source translated/preserved: 	ARIZONA_MUX_ROUTES("ISRC2DEC2", "ISRC2DEC2"),
// C source translated/preserved: 
// C source translated/preserved: 	{ "AEC Loopback", "HPOUT1L", "OUT1L" },
// C source translated/preserved: 	{ "AEC Loopback", "HPOUT1R", "OUT1R" },
// C source translated/preserved: 	{ "HPOUT1L", NULL, "OUT1L" },
// C source translated/preserved: 	{ "HPOUT1R", NULL, "OUT1R" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AEC Loopback", "EPOUT", "OUT3L" },
// C source translated/preserved: 	{ "EPOUTN", NULL, "OUT3L" },
// C source translated/preserved: 	{ "EPOUTP", NULL, "OUT3L" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AEC Loopback", "SPKOUT", "OUT4L" },
// C source translated/preserved: 	{ "SPKOUTN", NULL, "OUT4L" },
// C source translated/preserved: 	{ "SPKOUTP", NULL, "OUT4L" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "AEC Loopback", "SPKDAT1L", "OUT5L" },
// C source translated/preserved: 	{ "AEC Loopback", "SPKDAT1R", "OUT5R" },
// C source translated/preserved: 	{ "SPKDAT1L", NULL, "OUT5L" },
// C source translated/preserved: 	{ "SPKDAT1R", NULL, "OUT5R" },
// C source translated/preserved: 
// C source translated/preserved: 	{ "MICSUPP", NULL, "SYSCLK" },
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static int wm8997_set_fll(struct snd_soc_component *component, int fll_id,
// C source translated/preserved: 			  int source, unsigned int Fref, unsigned int Fout)
// C source translated/preserved: {
// C source translated/preserved: 	struct wm8997_priv *wm8997 = snd_soc_component_get_drvdata(component);
// C source translated/preserved: 
// C source translated/preserved: 	switch (fll_id) {
// C source translated/preserved: 	case WM8997_FLL1:
// C source translated/preserved: 		return arizona_set_fll(&wm8997->fll[0], source, Fref, Fout);
// C source translated/preserved: 	case WM8997_FLL2:
// C source translated/preserved: 		return arizona_set_fll(&wm8997->fll[1], source, Fref, Fout);
// C source translated/preserved: 	case WM8997_FLL1_REFCLK:
// C source translated/preserved: 		return arizona_set_fll_refclk(&wm8997->fll[0], source, Fref,
// C source translated/preserved: 					      Fout);
// C source translated/preserved: 	case WM8997_FLL2_REFCLK:
// C source translated/preserved: 		return arizona_set_fll_refclk(&wm8997->fll[1], source, Fref,
// C source translated/preserved: 					      Fout);
// C source translated/preserved: 	default:
// C source translated/preserved: 		return -EINVAL;
// C source translated/preserved: 	}
// C source translated/preserved: }
// C source translated/preserved: 
// C source translated/preserved: 
// C source translated/preserved: 
// C source translated/preserved: static struct snd_soc_dai_driver wm8997_dai[] = {
// C source translated/preserved: 	{
// C source translated/preserved: 		.name = "wm8997-aif1",
// C source translated/preserved: 		.id = 1,
// C source translated/preserved: 		.base = ARIZONA_AIF1_BCLK_CTRL,
// C source translated/preserved: 		.playback = {
// C source translated/preserved: 			.stream_name = "AIF1 Playback",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 8,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.capture = {
// C source translated/preserved: 			 .stream_name = "AIF1 Capture",
// C source translated/preserved: 			 .channels_min = 1,
// C source translated/preserved: 			 .channels_max = 8,
// C source translated/preserved: 			 .rates = WM8997_RATES,
// C source translated/preserved: 			 .formats = WM8997_FORMATS,
// C source translated/preserved: 		 },
// C source translated/preserved: 		.ops = &arizona_dai_ops,
// C source translated/preserved: 		.symmetric_rate = 1,
// C source translated/preserved: 		.symmetric_sample_bits = 1,
// C source translated/preserved: 	},
// C source translated/preserved: 	{
// C source translated/preserved: 		.name = "wm8997-aif2",
// C source translated/preserved: 		.id = 2,
// C source translated/preserved: 		.base = ARIZONA_AIF2_BCLK_CTRL,
// C source translated/preserved: 		.playback = {
// C source translated/preserved: 			.stream_name = "AIF2 Playback",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 2,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.capture = {
// C source translated/preserved: 			 .stream_name = "AIF2 Capture",
// C source translated/preserved: 			 .channels_min = 1,
// C source translated/preserved: 			 .channels_max = 2,
// C source translated/preserved: 			 .rates = WM8997_RATES,
// C source translated/preserved: 			 .formats = WM8997_FORMATS,
// C source translated/preserved: 		 },
// C source translated/preserved: 		.ops = &arizona_dai_ops,
// C source translated/preserved: 		.symmetric_rate = 1,
// C source translated/preserved: 		.symmetric_sample_bits = 1,
// C source translated/preserved: 	},
// C source translated/preserved: 	{
// C source translated/preserved: 		.name = "wm8997-slim1",
// C source translated/preserved: 		.id = 3,
// C source translated/preserved: 		.playback = {
// C source translated/preserved: 			.stream_name = "Slim1 Playback",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 4,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.capture = {
// C source translated/preserved: 			.stream_name = "Slim1 Capture",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 4,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.ops = &arizona_simple_dai_ops,
// C source translated/preserved: 	},
// C source translated/preserved: 	{
// C source translated/preserved: 		.name = "wm8997-slim2",
// C source translated/preserved: 		.id = 4,
// C source translated/preserved: 		.playback = {
// C source translated/preserved: 			.stream_name = "Slim2 Playback",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 2,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.capture = {
// C source translated/preserved: 			.stream_name = "Slim2 Capture",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 2,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.ops = &arizona_simple_dai_ops,
// C source translated/preserved: 	},
// C source translated/preserved: 	{
// C source translated/preserved: 		.name = "wm8997-slim3",
// C source translated/preserved: 		.id = 5,
// C source translated/preserved: 		.playback = {
// C source translated/preserved: 			.stream_name = "Slim3 Playback",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 2,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.capture = {
// C source translated/preserved: 			.stream_name = "Slim3 Capture",
// C source translated/preserved: 			.channels_min = 1,
// C source translated/preserved: 			.channels_max = 2,
// C source translated/preserved: 			.rates = WM8997_RATES,
// C source translated/preserved: 			.formats = WM8997_FORMATS,
// C source translated/preserved: 		},
// C source translated/preserved: 		.ops = &arizona_simple_dai_ops,
// C source translated/preserved: 	},
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static int wm8997_component_probe(struct snd_soc_component *component)
// C source translated/preserved: {
// C source translated/preserved: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C source translated/preserved: 	struct wm8997_priv *priv = snd_soc_component_get_drvdata(component);
// C source translated/preserved: 	struct arizona *arizona = priv->core.arizona;
// C source translated/preserved: 	int ret;
// C source translated/preserved: 
// C source translated/preserved: 	snd_soc_component_init_regmap(component, arizona->regmap);
// C source translated/preserved: 
// C source translated/preserved: 	ret = arizona_init_spk(component);
// C source translated/preserved: 	if (ret < 0)
// C source translated/preserved: 		return ret;
// C source translated/preserved: 
// C source translated/preserved: 	snd_soc_dapm_disable_pin(dapm, "HAPTICS");
// C source translated/preserved: 
// C source translated/preserved: 	priv->core.arizona->dapm = dapm;
// C source translated/preserved: 
// C source translated/preserved: 	return 0;
// C source translated/preserved: }
// C source translated/preserved: 
// C source translated/preserved: static void wm8997_component_remove(struct snd_soc_component *component)
// C source translated/preserved: {
// C source translated/preserved: 	struct wm8997_priv *priv = snd_soc_component_get_drvdata(component);
// C source translated/preserved: 
// C source translated/preserved: 	priv->core.arizona->dapm = NULL;
// C source translated/preserved: }
// C source translated/preserved: 
// C source translated/preserved: 
// C source translated/preserved: static unsigned int wm8997_digital_vu[] = {
// C source translated/preserved: 	ARIZONA_DAC_DIGITAL_VOLUME_1L,
// C source translated/preserved: 	ARIZONA_DAC_DIGITAL_VOLUME_1R,
// C source translated/preserved: 	ARIZONA_DAC_DIGITAL_VOLUME_3L,
// C source translated/preserved: 	ARIZONA_DAC_DIGITAL_VOLUME_4L,
// C source translated/preserved: 	ARIZONA_DAC_DIGITAL_VOLUME_5L,
// C source translated/preserved: 	ARIZONA_DAC_DIGITAL_VOLUME_5R,
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static const struct snd_soc_component_driver soc_component_dev_wm8997 = {
// C source translated/preserved: 	.probe			= wm8997_component_probe,
// C source translated/preserved: 	.remove			= wm8997_component_remove,
// C source translated/preserved: 	.set_sysclk		= arizona_set_sysclk,
// C source translated/preserved: 	.set_pll		= wm8997_set_fll,
// C source translated/preserved: 	.set_jack		= arizona_jack_set_jack,
// C source translated/preserved: 	.controls		= wm8997_snd_controls,
// C source translated/preserved: 	.num_controls		= ARRAY_SIZE(wm8997_snd_controls),
// C source translated/preserved: 	.dapm_widgets		= wm8997_dapm_widgets,
// C source translated/preserved: 	.num_dapm_widgets	= ARRAY_SIZE(wm8997_dapm_widgets),
// C source translated/preserved: 	.dapm_routes		= wm8997_dapm_routes,
// C source translated/preserved: 	.num_dapm_routes	= ARRAY_SIZE(wm8997_dapm_routes),
// C source translated/preserved: 	.use_pmdown_time	= 1,
// C source translated/preserved: 	.endianness		= 1,
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: static int wm8997_probe(struct platform_device *pdev)
// C source translated/preserved: {
// C source translated/preserved: 	struct arizona *arizona = dev_get_drvdata(pdev->dev.parent);
// C source translated/preserved: 	struct wm8997_priv *wm8997;
// C source translated/preserved: 	int i, ret;
// C source translated/preserved: 
// C source translated/preserved: 	wm8997 = devm_kzalloc(&pdev->dev, sizeof(struct wm8997_priv),
// C source translated/preserved: 			      GFP_KERNEL);
// C source translated/preserved: 	if (wm8997 == NULL)
// C source translated/preserved: 		return -ENOMEM;
// C source translated/preserved: 	platform_set_drvdata(pdev, wm8997);
// C source translated/preserved: 
// C source translated/preserved: 	if (IS_ENABLED(CONFIG_OF)) {
// C source translated/preserved: 		if (!dev_get_platdata(arizona->dev)) {
// C source translated/preserved: 			ret = arizona_of_get_audio_pdata(arizona);
// C source translated/preserved: 			if (ret < 0)
// C source translated/preserved: 				return ret;
// C source translated/preserved: 		}
// C source translated/preserved: 	}
// C source translated/preserved: 
// C source translated/preserved: 	wm8997->core.arizona = arizona;
// C source translated/preserved: 	wm8997->core.num_inputs = 4;
// C source translated/preserved: 
// C source translated/preserved: 	arizona_init_dvfs(&wm8997->core);
// C source translated/preserved: 
// C source translated/preserved: 	/* This may return -EPROBE_DEFER, so do this early on */
// C source translated/preserved: 	ret = arizona_jack_codec_dev_probe(&wm8997->core, &pdev->dev);
// C source translated/preserved: 	if (ret)
// C source translated/preserved: 		return ret;
// C source translated/preserved: 
// C source translated/preserved: 	for (i = 0; i < ARRAY_SIZE(wm8997->fll); i++)
// C source translated/preserved: 		wm8997->fll[i].vco_mult = 1;
// C source translated/preserved: 
// C source translated/preserved: 	arizona_init_fll(arizona, 1, ARIZONA_FLL1_CONTROL_1 - 1,
// C source translated/preserved: 			 ARIZONA_IRQ_FLL1_LOCK, ARIZONA_IRQ_FLL1_CLOCK_OK,
// C source translated/preserved: 			 &wm8997->fll[0]);
// C source translated/preserved: 	arizona_init_fll(arizona, 2, ARIZONA_FLL2_CONTROL_1 - 1,
// C source translated/preserved: 			 ARIZONA_IRQ_FLL2_LOCK, ARIZONA_IRQ_FLL2_CLOCK_OK,
// C source translated/preserved: 			 &wm8997->fll[1]);
// C source translated/preserved: 
// C source translated/preserved: 	/* SR2 fixed at 8kHz, SR3 fixed at 16kHz */
// C source translated/preserved: 	regmap_update_bits(arizona->regmap, ARIZONA_SAMPLE_RATE_2,
// C source translated/preserved: 			   ARIZONA_SAMPLE_RATE_2_MASK, 0x11);
// C source translated/preserved: 	regmap_update_bits(arizona->regmap, ARIZONA_SAMPLE_RATE_3,
// C source translated/preserved: 			   ARIZONA_SAMPLE_RATE_3_MASK, 0x12);
// C source translated/preserved: 
// C source translated/preserved: 	for (i = 0; i < ARRAY_SIZE(wm8997_dai); i++)
// C source translated/preserved: 		arizona_init_dai(&wm8997->core, i);
// C source translated/preserved: 
// C source translated/preserved: 	/* Latch volume update bits */
// C source translated/preserved: 	for (i = 0; i < ARRAY_SIZE(wm8997_digital_vu); i++)
// C source translated/preserved: 		regmap_update_bits(arizona->regmap, wm8997_digital_vu[i],
// C source translated/preserved: 				   WM8997_DIG_VU, WM8997_DIG_VU);
// C source translated/preserved: 
// C source translated/preserved: 	pm_runtime_enable(&pdev->dev);
// C source translated/preserved: 	pm_runtime_idle(&pdev->dev);
// C source translated/preserved: 
// C source translated/preserved: 	arizona_init_common(arizona);
// C source translated/preserved: 
// C source translated/preserved: 	ret = arizona_init_vol_limit(arizona);
// C source translated/preserved: 	if (ret < 0)
// C source translated/preserved: 		goto err_jack_codec_dev;
// C source translated/preserved: 	ret = arizona_init_spk_irqs(arizona);
// C source translated/preserved: 	if (ret < 0)
// C source translated/preserved: 		goto err_jack_codec_dev;
// C source translated/preserved: 
// C source translated/preserved: 	ret = devm_snd_soc_register_component(&pdev->dev,
// C source translated/preserved: 					      &soc_component_dev_wm8997,
// C source translated/preserved: 					      wm8997_dai,
// C source translated/preserved: 					      ARRAY_SIZE(wm8997_dai));
// C source translated/preserved: 	if (ret < 0) {
// C source translated/preserved: 		dev_err(&pdev->dev, "Failed to register component: %d\n", ret);
// C source translated/preserved: 		goto err_spk_irqs;
// C source translated/preserved: 	}
// C source translated/preserved: 
// C source translated/preserved: 	return ret;
// C source translated/preserved: 
// C source translated/preserved: err_spk_irqs:
// C source translated/preserved: 	arizona_free_spk_irqs(arizona);
// C source translated/preserved: err_jack_codec_dev:
// C source translated/preserved: 	pm_runtime_disable(&pdev->dev);
// C source translated/preserved: 	arizona_jack_codec_dev_remove(&wm8997->core);
// C source translated/preserved: 
// C source translated/preserved: 	return ret;
// C source translated/preserved: }
// C source translated/preserved: 
// C source translated/preserved: static void wm8997_remove(struct platform_device *pdev)
// C source translated/preserved: {
// C source translated/preserved: 	struct wm8997_priv *wm8997 = platform_get_drvdata(pdev);
// C source translated/preserved: 	struct arizona *arizona = wm8997->core.arizona;
// C source translated/preserved: 
// C source translated/preserved: 	pm_runtime_disable(&pdev->dev);
// C source translated/preserved: 
// C source translated/preserved: 	arizona_free_spk_irqs(arizona);
// C source translated/preserved: 
// C source translated/preserved: 	arizona_jack_codec_dev_remove(&wm8997->core);
// C source translated/preserved: }
// C source translated/preserved: 
// C source translated/preserved: static struct platform_driver wm8997_codec_driver = {
// C source translated/preserved: 	.driver = {
// C source translated/preserved: 		.name = "wm8997-codec",
// C source translated/preserved: 	},
// C source translated/preserved: 	.probe = wm8997_probe,
// C source translated/preserved: 	.remove = wm8997_remove,
// C source translated/preserved: };
// C source translated/preserved: 
// C source translated/preserved: module_platform_driver(wm8997_codec_driver);
// C source translated/preserved: 
// C source translated/preserved: MODULE_DESCRIPTION("ASoC WM8997 driver");
// C source translated/preserved: MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.wolfsonmicro.com>");
// C source translated/preserved: MODULE_LICENSE("GPL");
// C source translated/preserved: MODULE_ALIAS("platform:wm8997-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
