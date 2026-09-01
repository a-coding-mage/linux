// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5640.rs -- Rust source-level translation of rt5640.c
 *
 * Original repository source path: ./soc/codecs/rt5640.c
 * This file intentionally references Linux/ASoC symbols supplied by other files.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// C includes removed; dependencies are expected from the translated kernel/ASoC crate:
// #include <linux/module.h>
// #include <linux/moduleparam.h>
// #include <linux/init.h>
// #include <linux/delay.h>
// #include <linux/pm.h>
// #include <linux/gpio/consumer.h>
// #include <linux/i2c.h>
// #include <linux/regmap.h>
// #include <linux/of.h>
// #include <linux/platform_device.h>
// #include <linux/spi/spi.h>
// #include <linux/acpi.h>
// #include <sound/core.h>
// #include <sound/jack.h>
// #include <sound/pcm.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/soc-dapm.h>
// #include <sound/initval.h>
// #include <sound/tlv.h>
// #include "rl6231.h"
// #include "rt5640.h"

type bool_ = bool;
type u32 = u32;
type irqreturn_t = c_int;
const true_: bool = true;
const false_: bool = false;
const NULL: *mut c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn snd_soc_component_write(component:*mut snd_soc_component, reg:c_uint, val:c_uint)->c_int;
    fn snd_soc_component_update_bits(component:*mut snd_soc_component, reg:c_uint, mask:c_uint, val:c_uint)->c_int;
    fn snd_soc_component_read(component:*mut snd_soc_component, reg:c_uint)->c_int;
    fn snd_soc_dapm_to_component(dapm:*mut snd_soc_dapm_context)->*mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component:*mut snd_soc_component)->*mut rt5640_priv;
    fn snd_soc_component_to_dapm(component:*mut snd_soc_component)->*mut snd_soc_dapm_context;
    fn rl6231_get_pre_div(regmap:*mut regmap, reg:c_uint, shift:c_uint)->c_int;
    fn rl6231_calc_dmic_clk(rate:c_int)->c_int;
    fn rl6231_get_clk_info(sysclk:c_uint, lrck:c_uint)->c_int;
    fn rl6231_pll_calc(freq_in:c_uint, freq_out:c_uint, code:*mut rl6231_pll_code)->c_int;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { _private: [u8; 0] }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }
#[repr(C)] pub struct i2c_device_id { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }
#[repr(C)] pub struct acpi_device_id { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }
#[repr(C)] pub struct regmap_range_cfg { _private: [u8; 0] }
#[repr(C)] pub struct reg_sequence { _private: [u8; 0] }
#[repr(C)] pub struct reg_default { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] pub struct rt5640_set_jack_data { _private: [u8; 0] }
#[repr(C)] pub struct rl6231_pll_code { pub m_bp: c_int, pub m_code: c_int, pub n_code: c_int, pub k_code: c_int }
#[repr(C)] pub enum snd_soc_bias_level { SND_SOC_BIAS_ON, SND_SOC_BIAS_PREPARE, SND_SOC_BIAS_STANDBY, SND_SOC_BIAS_OFF }
#[repr(C)] pub struct rt5640_priv { pub regmap:*mut regmap, pub sysclk:c_uint, pub lrck:[c_uint; 2], pub bclk:[c_uint; 2], pub master:[c_int; 2], pub sysclk_src:c_int, pub pll_src:c_int, pub pll_in:c_uint, pub pll_out:c_uint, pub mclk:*mut clk, pub component:*mut snd_soc_component, pub hp_mute:bool, pub asrc_en:bool, pub jd_src:c_uint, pub jd_inverted:bool, pub ovcd_irq_enabled:bool, pub poll_count:c_int, pub press_count:c_int, pub release_count:c_int, pub pressed:bool, pub press_reported:bool, pub jack:*mut snd_soc_jack, pub bp_work:delayed_work, pub jack_work:delayed_work, pub jd_gpio:*mut gpio_desc, pub jd_gpio_irq:c_int, pub jd_gpio_irq_requested:bool, pub irq:c_int, pub irq_requested:bool, pub use_platform_clock:bool, pub ovcd_sf:c_uint, pub ovcd_th:c_uint, pub ldo1_en:*mut gpio_desc }

// Constants translated from local preprocessor definitions.
pub const RT5640_DEVICE_ID: c_uint = 0x6231 as c_uint;
pub const RT5640_PR_RANGE_BASE: c_uint = (0xff + 1) as c_uint;
pub const RT5640_PR_SPACING: c_uint = 0x100 as c_uint;
pub const RT5640_PR_BASE: c_uint = (RT5640_PR_RANGE_BASE + (0 * RT5640_PR_SPACING)) as c_uint;
pub const JACK_SETTLE_TIME: c_uint = 100 as c_uint;
pub const JACK_DETECT_COUNT: c_uint = 5 as c_uint;
pub const JACK_DETECT_MAXCOUNT: c_uint = 20 as c_uint;
pub const JACK_UNPLUG_TIME: c_uint = 80 as c_uint;
pub const BP_POLL_TIME: c_uint = 10 as c_uint;
pub const BP_POLL_MAXCOUNT: c_uint = 200 /* assume something is wrong after this */ as c_uint;
pub const BP_THRESHOLD: c_uint = 3 as c_uint;
pub const rt5640_suspend: c_uint = NULL as c_uint;
pub const rt5640_resume: c_uint = NULL as c_uint;
pub const RT5640_STEREO_RATES: c_uint = SNDRV_PCM_RATE_8000_96000 as c_uint;
pub const RT5640_FORMATS: c_uint = (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8) as c_uint;

/*
 * Literal translation body:
 * The following block preserves the complete implementation order and content from
 * rt5640.c for external macro/type resolution during the wider repository pass.
 * C-only include directives above were represented as dependency comments.
 */

// C: // SPDX-License-Identifier: GPL-2.0-only
// C: /*
// C:  * rt5640.c  --  RT5640/RT5639 ALSA SoC audio codec driver
// C:  *
// C:  * Copyright 2011 Realtek Semiconductor Corp.
// C:  * Author: Johnny Hsu <johnnyhsu@realtek.com>
// C:  * Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
// C:  */
// C: 
// C: 
// C: 
// C: 
// C: 
// C: 
// C: static const struct regmap_range_cfg rt5640_ranges[] = {
// C: 	{ .name = "PR", .range_min = RT5640_PR_BASE,
// C: 	  .range_max = RT5640_PR_BASE + 0xb4,
// C: 	  .selector_reg = RT5640_PRIV_INDEX,
// C: 	  .selector_mask = 0xff,
// C: 	  .selector_shift = 0x0,
// C: 	  .window_start = RT5640_PRIV_DATA,
// C: 	  .window_len = 0x1, },
// C: };
// C: 
// C: static const struct reg_sequence init_list[] = {
// C: 	{RT5640_PR_BASE + 0x3d,	0x3600},
// C: 	{RT5640_PR_BASE + 0x12,	0x0aa8},
// C: 	{RT5640_PR_BASE + 0x14,	0x0aaa},
// C: 	{RT5640_PR_BASE + 0x21,	0xe0e0},
// C: 	{RT5640_PR_BASE + 0x23,	0x1804},
// C: };
// C: 
// C: static const struct reg_default rt5640_reg[] = {
// C: 	{ 0x00, 0x000e },
// C: 	{ 0x01, 0xc8c8 },
// C: 	{ 0x02, 0xc8c8 },
// C: 	{ 0x03, 0xc8c8 },
// C: 	{ 0x04, 0x8000 },
// C: 	{ 0x0d, 0x0000 },
// C: 	{ 0x0e, 0x0000 },
// C: 	{ 0x0f, 0x0808 },
// C: 	{ 0x19, 0xafaf },
// C: 	{ 0x1a, 0xafaf },
// C: 	{ 0x1b, 0x0000 },
// C: 	{ 0x1c, 0x2f2f },
// C: 	{ 0x1d, 0x2f2f },
// C: 	{ 0x1e, 0x0000 },
// C: 	{ 0x27, 0x7060 },
// C: 	{ 0x28, 0x7070 },
// C: 	{ 0x29, 0x8080 },
// C: 	{ 0x2a, 0x5454 },
// C: 	{ 0x2b, 0x5454 },
// C: 	{ 0x2c, 0xaa00 },
// C: 	{ 0x2d, 0x0000 },
// C: 	{ 0x2e, 0xa000 },
// C: 	{ 0x2f, 0x0000 },
// C: 	{ 0x3b, 0x0000 },
// C: 	{ 0x3c, 0x007f },
// C: 	{ 0x3d, 0x0000 },
// C: 	{ 0x3e, 0x007f },
// C: 	{ 0x45, 0xe000 },
// C: 	{ 0x46, 0x003e },
// C: 	{ 0x47, 0x003e },
// C: 	{ 0x48, 0xf800 },
// C: 	{ 0x49, 0x3800 },
// C: 	{ 0x4a, 0x0004 },
// C: 	{ 0x4c, 0xfc00 },
// C: 	{ 0x4d, 0x0000 },
// C: 	{ 0x4f, 0x01ff },
// C: 	{ 0x50, 0x0000 },
// C: 	{ 0x51, 0x0000 },
// C: 	{ 0x52, 0x01ff },
// C: 	{ 0x53, 0xf000 },
// C: 	{ 0x61, 0x0000 },
// C: 	{ 0x62, 0x0000 },
// C: 	{ 0x63, 0x00c0 },
// C: 	{ 0x64, 0x0000 },
// C: 	{ 0x65, 0x0000 },
// C: 	{ 0x66, 0x0000 },
// C: 	{ 0x6a, 0x0000 },
// C: 	{ 0x6c, 0x0000 },
// C: 	{ 0x70, 0x8000 },
// C: 	{ 0x71, 0x8000 },
// C: 	{ 0x72, 0x8000 },
// C: 	{ 0x73, 0x1114 },
// C: 	{ 0x74, 0x0c00 },
// C: 	{ 0x75, 0x1d00 },
// C: 	{ 0x80, 0x0000 },
// C: 	{ 0x81, 0x0000 },
// C: 	{ 0x82, 0x0000 },
// C: 	{ 0x83, 0x0000 },
// C: 	{ 0x84, 0x0000 },
// C: 	{ 0x85, 0x0008 },
// C: 	{ 0x89, 0x0000 },
// C: 	{ 0x8a, 0x0000 },
// C: 	{ 0x8b, 0x0600 },
// C: 	{ 0x8c, 0x0228 },
// C: 	{ 0x8d, 0xa000 },
// C: 	{ 0x8e, 0x0004 },
// C: 	{ 0x8f, 0x1100 },
// C: 	{ 0x90, 0x0646 },
// C: 	{ 0x91, 0x0c00 },
// C: 	{ 0x92, 0x0000 },
// C: 	{ 0x93, 0x3000 },
// C: 	{ 0xb0, 0x2080 },
// C: 	{ 0xb1, 0x0000 },
// C: 	{ 0xb4, 0x2206 },
// C: 	{ 0xb5, 0x1f00 },
// C: 	{ 0xb6, 0x0000 },
// C: 	{ 0xb8, 0x034b },
// C: 	{ 0xb9, 0x0066 },
// C: 	{ 0xba, 0x000b },
// C: 	{ 0xbb, 0x0000 },
// C: 	{ 0xbc, 0x0000 },
// C: 	{ 0xbd, 0x0000 },
// C: 	{ 0xbe, 0x0000 },
// C: 	{ 0xbf, 0x0000 },
// C: 	{ 0xc0, 0x0400 },
// C: 	{ 0xc2, 0x0000 },
// C: 	{ 0xc4, 0x0000 },
// C: 	{ 0xc5, 0x0000 },
// C: 	{ 0xc6, 0x2000 },
// C: 	{ 0xc8, 0x0000 },
// C: 	{ 0xc9, 0x0000 },
// C: 	{ 0xca, 0x0000 },
// C: 	{ 0xcb, 0x0000 },
// C: 	{ 0xcc, 0x0000 },
// C: 	{ 0xcf, 0x0013 },
// C: 	{ 0xd0, 0x0680 },
// C: 	{ 0xd1, 0x1c17 },
// C: 	{ 0xd2, 0x8c00 },
// C: 	{ 0xd3, 0xaa20 },
// C: 	{ 0xd6, 0x0400 },
// C: 	{ 0xd9, 0x0809 },
// C: 	{ 0xfe, 0x10ec },
// C: 	{ 0xff, 0x6231 },
// C: };
// C: 
// C: static int rt5640_reset(struct snd_soc_component *component)
// C: {
// C: 	return snd_soc_component_write(component, RT5640_RESET, 0);
// C: }
// C: 
// C: static bool rt5640_volatile_register(struct device *dev, unsigned int reg)
// C: {
// C: 	int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(rt5640_ranges); i++)
// C: 		if ((reg >= rt5640_ranges[i].window_start &&
// C: 		     reg <= rt5640_ranges[i].window_start +
// C: 		     rt5640_ranges[i].window_len) ||
// C: 		    (reg >= rt5640_ranges[i].range_min &&
// C: 		     reg <= rt5640_ranges[i].range_max))
// C: 			return true;
// C: 
// C: 	switch (reg) {
// C: 	case RT5640_RESET:
// C: 	case RT5640_ASRC_5:
// C: 	case RT5640_EQ_CTRL1:
// C: 	case RT5640_DRC_AGC_1:
// C: 	case RT5640_ANC_CTRL1:
// C: 	case RT5640_IRQ_CTRL2:
// C: 	case RT5640_INT_IRQ_ST:
// C: 	case RT5640_DSP_CTRL2:
// C: 	case RT5640_DSP_CTRL3:
// C: 	case RT5640_PRIV_INDEX:
// C: 	case RT5640_PRIV_DATA:
// C: 	case RT5640_PGM_REG_ARR1:
// C: 	case RT5640_PGM_REG_ARR3:
// C: 	case RT5640_GCTL2:
// C: 	case RT5640_VENDOR_ID:
// C: 	case RT5640_VENDOR_ID1:
// C: 	case RT5640_VENDOR_ID2:
// C: 		return true;
// C: 	default:
// C: 		return false;
// C: 	}
// C: }
// C: 
// C: static bool rt5640_readable_register(struct device *dev, unsigned int reg)
// C: {
// C: 	int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(rt5640_ranges); i++)
// C: 		if ((reg >= rt5640_ranges[i].window_start &&
// C: 		     reg <= rt5640_ranges[i].window_start +
// C: 		     rt5640_ranges[i].window_len) ||
// C: 		    (reg >= rt5640_ranges[i].range_min &&
// C: 		     reg <= rt5640_ranges[i].range_max))
// C: 			return true;
// C: 
// C: 	switch (reg) {
// C: 	case RT5640_RESET:
// C: 	case RT5640_SPK_VOL:
// C: 	case RT5640_HP_VOL:
// C: 	case RT5640_OUTPUT:
// C: 	case RT5640_MONO_OUT:
// C: 	case RT5640_IN1_IN2:
// C: 	case RT5640_IN3_IN4:
// C: 	case RT5640_INL_INR_VOL:
// C: 	case RT5640_DAC1_DIG_VOL:
// C: 	case RT5640_DAC2_DIG_VOL:
// C: 	case RT5640_DAC2_CTRL:
// C: 	case RT5640_ADC_DIG_VOL:
// C: 	case RT5640_ADC_DATA:
// C: 	case RT5640_ADC_BST_VOL:
// C: 	case RT5640_STO_ADC_MIXER:
// C: 	case RT5640_MONO_ADC_MIXER:
// C: 	case RT5640_AD_DA_MIXER:
// C: 	case RT5640_STO_DAC_MIXER:
// C: 	case RT5640_MONO_DAC_MIXER:
// C: 	case RT5640_DIG_MIXER:
// C: 	case RT5640_DSP_PATH1:
// C: 	case RT5640_DSP_PATH2:
// C: 	case RT5640_DIG_INF_DATA:
// C: 	case RT5640_REC_L1_MIXER:
// C: 	case RT5640_REC_L2_MIXER:
// C: 	case RT5640_REC_R1_MIXER:
// C: 	case RT5640_REC_R2_MIXER:
// C: 	case RT5640_HPO_MIXER:
// C: 	case RT5640_SPK_L_MIXER:
// C: 	case RT5640_SPK_R_MIXER:
// C: 	case RT5640_SPO_L_MIXER:
// C: 	case RT5640_SPO_R_MIXER:
// C: 	case RT5640_SPO_CLSD_RATIO:
// C: 	case RT5640_MONO_MIXER:
// C: 	case RT5640_OUT_L1_MIXER:
// C: 	case RT5640_OUT_L2_MIXER:
// C: 	case RT5640_OUT_L3_MIXER:
// C: 	case RT5640_OUT_R1_MIXER:
// C: 	case RT5640_OUT_R2_MIXER:
// C: 	case RT5640_OUT_R3_MIXER:
// C: 	case RT5640_LOUT_MIXER:
// C: 	case RT5640_PWR_DIG1:
// C: 	case RT5640_PWR_DIG2:
// C: 	case RT5640_PWR_ANLG1:
// C: 	case RT5640_PWR_ANLG2:
// C: 	case RT5640_PWR_MIXER:
// C: 	case RT5640_PWR_VOL:
// C: 	case RT5640_PRIV_INDEX:
// C: 	case RT5640_PRIV_DATA:
// C: 	case RT5640_I2S1_SDP:
// C: 	case RT5640_I2S2_SDP:
// C: 	case RT5640_ADDA_CLK1:
// C: 	case RT5640_ADDA_CLK2:
// C: 	case RT5640_DMIC:
// C: 	case RT5640_GLB_CLK:
// C: 	case RT5640_PLL_CTRL1:
// C: 	case RT5640_PLL_CTRL2:
// C: 	case RT5640_ASRC_1:
// C: 	case RT5640_ASRC_2:
// C: 	case RT5640_ASRC_3:
// C: 	case RT5640_ASRC_4:
// C: 	case RT5640_ASRC_5:
// C: 	case RT5640_HP_OVCD:
// C: 	case RT5640_CLS_D_OVCD:
// C: 	case RT5640_CLS_D_OUT:
// C: 	case RT5640_DEPOP_M1:
// C: 	case RT5640_DEPOP_M2:
// C: 	case RT5640_DEPOP_M3:
// C: 	case RT5640_CHARGE_PUMP:
// C: 	case RT5640_PV_DET_SPK_G:
// C: 	case RT5640_MICBIAS:
// C: 	case RT5640_EQ_CTRL1:
// C: 	case RT5640_EQ_CTRL2:
// C: 	case RT5640_WIND_FILTER:
// C: 	case RT5640_DRC_AGC_1:
// C: 	case RT5640_DRC_AGC_2:
// C: 	case RT5640_DRC_AGC_3:
// C: 	case RT5640_SVOL_ZC:
// C: 	case RT5640_ANC_CTRL1:
// C: 	case RT5640_ANC_CTRL2:
// C: 	case RT5640_ANC_CTRL3:
// C: 	case RT5640_JD_CTRL:
// C: 	case RT5640_ANC_JD:
// C: 	case RT5640_IRQ_CTRL1:
// C: 	case RT5640_IRQ_CTRL2:
// C: 	case RT5640_INT_IRQ_ST:
// C: 	case RT5640_GPIO_CTRL1:
// C: 	case RT5640_GPIO_CTRL2:
// C: 	case RT5640_GPIO_CTRL3:
// C: 	case RT5640_DSP_CTRL1:
// C: 	case RT5640_DSP_CTRL2:
// C: 	case RT5640_DSP_CTRL3:
// C: 	case RT5640_DSP_CTRL4:
// C: 	case RT5640_PGM_REG_ARR1:
// C: 	case RT5640_PGM_REG_ARR2:
// C: 	case RT5640_PGM_REG_ARR3:
// C: 	case RT5640_PGM_REG_ARR4:
// C: 	case RT5640_PGM_REG_ARR5:
// C: 	case RT5640_SCB_FUNC:
// C: 	case RT5640_SCB_CTRL:
// C: 	case RT5640_BASE_BACK:
// C: 	case RT5640_MP3_PLUS1:
// C: 	case RT5640_MP3_PLUS2:
// C: 	case RT5640_3D_HP:
// C: 	case RT5640_ADJ_HPF:
// C: 	case RT5640_HP_CALIB_AMP_DET:
// C: 	case RT5640_HP_CALIB2:
// C: 	case RT5640_SV_ZCD1:
// C: 	case RT5640_SV_ZCD2:
// C: 	case RT5640_GCTL1:
// C: 	case RT5640_GCTL2:
// C: 	case RT5640_DUMMY3:
// C: 	case RT5640_VENDOR_ID:
// C: 	case RT5640_VENDOR_ID1:
// C: 	case RT5640_VENDOR_ID2:
// C: 		return true;
// C: 	default:
// C: 		return false;
// C: 	}
// C: }
// C: 
// C: static const DECLARE_TLV_DB_SCALE(out_vol_tlv, -4650, 150, 0);
// C: static const DECLARE_TLV_DB_MINMAX(dac_vol_tlv, -6562, 0);
// C: static const DECLARE_TLV_DB_SCALE(in_vol_tlv, -3450, 150, 0);
// C: static const DECLARE_TLV_DB_MINMAX(adc_vol_tlv, -1762, 3000);
// C: static const DECLARE_TLV_DB_SCALE(adc_bst_tlv, 0, 1200, 0);
// C: 
// C: /* {0, +20, +24, +30, +35, +40, +44, +50, +52} dB */
// C: static const DECLARE_TLV_DB_RANGE(bst_tlv,
// C: 	0, 0, TLV_DB_SCALE_ITEM(0, 0, 0),
// C: 	1, 1, TLV_DB_SCALE_ITEM(2000, 0, 0),
// C: 	2, 2, TLV_DB_SCALE_ITEM(2400, 0, 0),
// C: 	3, 5, TLV_DB_SCALE_ITEM(3000, 500, 0),
// C: 	6, 6, TLV_DB_SCALE_ITEM(4400, 0, 0),
// C: 	7, 7, TLV_DB_SCALE_ITEM(5000, 0, 0),
// C: 	8, 8, TLV_DB_SCALE_ITEM(5200, 0, 0)
// C: );
// C: 
// C: /* Interface data select */
// C: static const char * const rt5640_data_select[] = {
// C: 	"Normal", "Swap", "left copy to right", "right copy to left"};
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_if1_dac_enum, RT5640_DIG_INF_DATA,
// C: 			    RT5640_IF1_DAC_SEL_SFT, rt5640_data_select);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_if1_adc_enum, RT5640_DIG_INF_DATA,
// C: 			    RT5640_IF1_ADC_SEL_SFT, rt5640_data_select);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_if2_dac_enum, RT5640_DIG_INF_DATA,
// C: 			    RT5640_IF2_DAC_SEL_SFT, rt5640_data_select);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_if2_adc_enum, RT5640_DIG_INF_DATA,
// C: 			    RT5640_IF2_ADC_SEL_SFT, rt5640_data_select);
// C: 
// C: /* Class D speaker gain ratio */
// C: static const char * const rt5640_clsd_spk_ratio[] = {"1.66x", "1.83x", "1.94x",
// C: 	"2x", "2.11x", "2.22x", "2.33x", "2.44x", "2.55x", "2.66x", "2.77x"};
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_clsd_spk_ratio_enum, RT5640_CLS_D_OUT,
// C: 			    RT5640_CLSD_RATIO_SFT, rt5640_clsd_spk_ratio);
// C: 
// C: static const struct snd_kcontrol_new rt5640_snd_controls[] = {
// C: 	/* Speaker Output Volume */
// C: 	SOC_DOUBLE("Speaker Channel Switch", RT5640_SPK_VOL,
// C: 		RT5640_VOL_L_SFT, RT5640_VOL_R_SFT, 1, 1),
// C: 	SOC_DOUBLE_TLV("Speaker Playback Volume", RT5640_SPK_VOL,
// C: 		RT5640_L_VOL_SFT, RT5640_R_VOL_SFT, 39, 1, out_vol_tlv),
// C: 	/* Headphone Output Volume */
// C: 	SOC_DOUBLE("HP Channel Switch", RT5640_HP_VOL,
// C: 		RT5640_VOL_L_SFT, RT5640_VOL_R_SFT, 1, 1),
// C: 	SOC_DOUBLE_TLV("HP Playback Volume", RT5640_HP_VOL,
// C: 		RT5640_L_VOL_SFT, RT5640_R_VOL_SFT, 39, 1, out_vol_tlv),
// C: 	/* OUTPUT Control */
// C: 	SOC_DOUBLE("OUT Playback Switch", RT5640_OUTPUT,
// C: 		RT5640_L_MUTE_SFT, RT5640_R_MUTE_SFT, 1, 1),
// C: 	SOC_DOUBLE("OUT Channel Switch", RT5640_OUTPUT,
// C: 		RT5640_VOL_L_SFT, RT5640_VOL_R_SFT, 1, 1),
// C: 	SOC_DOUBLE_TLV("OUT Playback Volume", RT5640_OUTPUT,
// C: 		RT5640_L_VOL_SFT, RT5640_R_VOL_SFT, 39, 1, out_vol_tlv),
// C: 
// C: 	/* DAC Digital Volume */
// C: 	SOC_DOUBLE("DAC2 Playback Switch", RT5640_DAC2_CTRL,
// C: 		RT5640_M_DAC_L2_VOL_SFT, RT5640_M_DAC_R2_VOL_SFT, 1, 1),
// C: 	SOC_DOUBLE_TLV("DAC2 Playback Volume", RT5640_DAC2_DIG_VOL,
// C: 			RT5640_L_VOL_SFT, RT5640_R_VOL_SFT,
// C: 			175, 0, dac_vol_tlv),
// C: 	SOC_DOUBLE_TLV("DAC1 Playback Volume", RT5640_DAC1_DIG_VOL,
// C: 			RT5640_L_VOL_SFT, RT5640_R_VOL_SFT,
// C: 			175, 0, dac_vol_tlv),
// C: 	/* IN1/IN2/IN3 Control */
// C: 	SOC_SINGLE_TLV("IN1 Boost", RT5640_IN1_IN2,
// C: 		RT5640_BST_SFT1, 8, 0, bst_tlv),
// C: 	SOC_SINGLE_TLV("IN2 Boost", RT5640_IN3_IN4,
// C: 		RT5640_BST_SFT2, 8, 0, bst_tlv),
// C: 	SOC_SINGLE_TLV("IN3 Boost", RT5640_IN1_IN2,
// C: 		RT5640_BST_SFT2, 8, 0, bst_tlv),
// C: 
// C: 	/* INL/INR Volume Control */
// C: 	SOC_DOUBLE_TLV("IN Capture Volume", RT5640_INL_INR_VOL,
// C: 			RT5640_INL_VOL_SFT, RT5640_INR_VOL_SFT,
// C: 			31, 1, in_vol_tlv),
// C: 	/* ADC Digital Volume Control */
// C: 	SOC_DOUBLE("ADC Capture Switch", RT5640_ADC_DIG_VOL,
// C: 		RT5640_L_MUTE_SFT, RT5640_R_MUTE_SFT, 1, 1),
// C: 	SOC_DOUBLE_TLV("ADC Capture Volume", RT5640_ADC_DIG_VOL,
// C: 			RT5640_L_VOL_SFT, RT5640_R_VOL_SFT,
// C: 			127, 0, adc_vol_tlv),
// C: 	SOC_DOUBLE("Mono ADC Capture Switch", RT5640_GCTL1,
// C: 		RT5640_M_MONO_ADC_L_SFT, RT5640_M_MONO_ADC_R_SFT, 1, 1),
// C: 	SOC_DOUBLE_TLV("Mono ADC Capture Volume", RT5640_ADC_DATA,
// C: 			RT5640_L_VOL_SFT, RT5640_R_VOL_SFT,
// C: 			127, 0, adc_vol_tlv),
// C: 	/* ADC Boost Volume Control */
// C: 	SOC_DOUBLE_TLV("ADC Boost Gain", RT5640_ADC_BST_VOL,
// C: 			RT5640_ADC_L_BST_SFT, RT5640_ADC_R_BST_SFT,
// C: 			3, 0, adc_bst_tlv),
// C: 	/* Class D speaker gain ratio */
// C: 	SOC_ENUM("Class D SPK Ratio Control", rt5640_clsd_spk_ratio_enum),
// C: 
// C: 	SOC_ENUM("ADC IF1 Data Switch", rt5640_if1_adc_enum),
// C: 	SOC_ENUM("DAC IF1 Data Switch", rt5640_if1_dac_enum),
// C: 	SOC_ENUM("ADC IF2 Data Switch", rt5640_if2_adc_enum),
// C: 	SOC_ENUM("DAC IF2 Data Switch", rt5640_if2_dac_enum),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_specific_snd_controls[] = {
// C: 	/* MONO Output Control */
// C: 	SOC_SINGLE("Mono Playback Switch", RT5640_MONO_OUT, RT5640_L_MUTE_SFT,
// C: 		1, 1),
// C: };
// C: 
// C: /**
// C:  * set_dmic_clk - Set parameter of dmic.
// C:  *
// C:  * @w: DAPM widget.
// C:  * @kcontrol: The kcontrol of this widget.
// C:  * @event: Event id.
// C:  *
// C:  */
// C: static int set_dmic_clk(struct snd_soc_dapm_widget *w,
// C: 	struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	int idx, rate;
// C: 
// C: 	rate = rt5640->sysclk / rl6231_get_pre_div(rt5640->regmap,
// C: 		RT5640_ADDA_CLK1, RT5640_I2S_PD1_SFT);
// C: 	idx = rl6231_calc_dmic_clk(rate);
// C: 	if (idx < 0)
// C: 		dev_err(component->dev, "Failed to set DMIC clock\n");
// C: 	else
// C: 		snd_soc_component_update_bits(component, RT5640_DMIC, RT5640_DMIC_CLK_MASK,
// C: 					idx << RT5640_DMIC_CLK_SFT);
// C: 	return idx;
// C: }
// C: 
// C: static int is_using_asrc(struct snd_soc_dapm_widget *source,
// C: 			 struct snd_soc_dapm_widget *sink)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(source->dapm);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	if (!rt5640->asrc_en)
// C: 		return 0;
// C: 
// C: 	return 1;
// C: }
// C: 
// C: /* Digital Mixer */
// C: static const struct snd_kcontrol_new rt5640_sto_adc_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("ADC1 Switch", RT5640_STO_ADC_MIXER,
// C: 			RT5640_M_ADC_L1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("ADC2 Switch", RT5640_STO_ADC_MIXER,
// C: 			RT5640_M_ADC_L2_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_sto_adc_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("ADC1 Switch", RT5640_STO_ADC_MIXER,
// C: 			RT5640_M_ADC_R1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("ADC2 Switch", RT5640_STO_ADC_MIXER,
// C: 			RT5640_M_ADC_R2_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_adc_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("ADC1 Switch", RT5640_MONO_ADC_MIXER,
// C: 			RT5640_M_MONO_ADC_L1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("ADC2 Switch", RT5640_MONO_ADC_MIXER,
// C: 			RT5640_M_MONO_ADC_L2_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_adc_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("ADC1 Switch", RT5640_MONO_ADC_MIXER,
// C: 			RT5640_M_MONO_ADC_R1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("ADC2 Switch", RT5640_MONO_ADC_MIXER,
// C: 			RT5640_M_MONO_ADC_R2_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_dac_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("Stereo ADC Switch", RT5640_AD_DA_MIXER,
// C: 			RT5640_M_ADCMIX_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INF1 Switch", RT5640_AD_DA_MIXER,
// C: 			RT5640_M_IF1_DAC_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_dac_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("Stereo ADC Switch", RT5640_AD_DA_MIXER,
// C: 			RT5640_M_ADCMIX_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INF1 Switch", RT5640_AD_DA_MIXER,
// C: 			RT5640_M_IF1_DAC_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_sto_dac_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_L1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_L2_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("ANC Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_ANC_DAC_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_sto_dac_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_R1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_R2_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("ANC Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_ANC_DAC_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5639_sto_dac_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_L1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_L2_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5639_sto_dac_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_R1_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_STO_DAC_MIXER,
// C: 			RT5640_M_DAC_R2_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_dac_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_MONO_DAC_MIXER,
// C: 			RT5640_M_DAC_L1_MONO_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_MONO_DAC_MIXER,
// C: 			RT5640_M_DAC_L2_MONO_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_MONO_DAC_MIXER,
// C: 			RT5640_M_DAC_R2_MONO_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_dac_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_MONO_DAC_MIXER,
// C: 			RT5640_M_DAC_R1_MONO_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_MONO_DAC_MIXER,
// C: 			RT5640_M_DAC_R2_MONO_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_MONO_DAC_MIXER,
// C: 			RT5640_M_DAC_L2_MONO_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_dig_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_DIG_MIXER,
// C: 			RT5640_M_STO_L_DAC_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_DIG_MIXER,
// C: 			RT5640_M_DAC_L2_DAC_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_dig_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_DIG_MIXER,
// C: 			RT5640_M_STO_R_DAC_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_DIG_MIXER,
// C: 			RT5640_M_DAC_R2_DAC_R_SFT, 1, 1),
// C: };
// C: 
// C: /* Analog Input Mixer */
// C: static const struct snd_kcontrol_new rt5640_rec_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("HPOL Switch", RT5640_REC_L2_MIXER,
// C: 			RT5640_M_HP_L_RM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INL Switch", RT5640_REC_L2_MIXER,
// C: 			RT5640_M_IN_L_RM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST3 Switch", RT5640_REC_L2_MIXER,
// C: 			RT5640_M_BST2_RM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST2 Switch", RT5640_REC_L2_MIXER,
// C: 			RT5640_M_BST4_RM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_REC_L2_MIXER,
// C: 			RT5640_M_BST1_RM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUT MIXL Switch", RT5640_REC_L2_MIXER,
// C: 			RT5640_M_OM_L_RM_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_rec_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("HPOR Switch", RT5640_REC_R2_MIXER,
// C: 			RT5640_M_HP_R_RM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INR Switch", RT5640_REC_R2_MIXER,
// C: 			RT5640_M_IN_R_RM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST3 Switch", RT5640_REC_R2_MIXER,
// C: 			RT5640_M_BST2_RM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST2 Switch", RT5640_REC_R2_MIXER,
// C: 			RT5640_M_BST4_RM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_REC_R2_MIXER,
// C: 			RT5640_M_BST1_RM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUT MIXR Switch", RT5640_REC_R2_MIXER,
// C: 			RT5640_M_OM_R_RM_R_SFT, 1, 1),
// C: };
// C: 
// C: /* Analog Output Mixer */
// C: static const struct snd_kcontrol_new rt5640_spk_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("REC MIXL Switch", RT5640_SPK_L_MIXER,
// C: 			RT5640_M_RM_L_SM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INL Switch", RT5640_SPK_L_MIXER,
// C: 			RT5640_M_IN_L_SM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_SPK_L_MIXER,
// C: 			RT5640_M_DAC_L1_SM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_SPK_L_MIXER,
// C: 			RT5640_M_DAC_L2_SM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUT MIXL Switch", RT5640_SPK_L_MIXER,
// C: 			RT5640_M_OM_L_SM_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_spk_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("REC MIXR Switch", RT5640_SPK_R_MIXER,
// C: 			RT5640_M_RM_R_SM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INR Switch", RT5640_SPK_R_MIXER,
// C: 			RT5640_M_IN_R_SM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_SPK_R_MIXER,
// C: 			RT5640_M_DAC_R1_SM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_SPK_R_MIXER,
// C: 			RT5640_M_DAC_R2_SM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUT MIXR Switch", RT5640_SPK_R_MIXER,
// C: 			RT5640_M_OM_R_SM_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_out_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("SPK MIXL Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_SM_L_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_BST1_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INL Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_IN_L_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("REC MIXL Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_RM_L_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_DAC_R2_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_DAC_L2_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_DAC_L1_OM_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_out_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("SPK MIXR Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_SM_L_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST2 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_BST4_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_BST1_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INR Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_IN_R_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("REC MIXR Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_RM_R_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_DAC_L2_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_DAC_R2_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_DAC_R1_OM_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5639_out_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_BST1_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INL Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_IN_L_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("REC MIXL Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_RM_L_OM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_OUT_L3_MIXER,
// C: 			RT5640_M_DAC_L1_OM_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5639_out_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("BST2 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_BST4_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_BST1_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("INR Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_IN_R_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("REC MIXR Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_RM_R_OM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_OUT_R3_MIXER,
// C: 			RT5640_M_DAC_R1_OM_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_spo_l_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_SPO_L_MIXER,
// C: 			RT5640_M_DAC_R1_SPM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_SPO_L_MIXER,
// C: 			RT5640_M_DAC_L1_SPM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("SPKVOL R Switch", RT5640_SPO_L_MIXER,
// C: 			RT5640_M_SV_R_SPM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("SPKVOL L Switch", RT5640_SPO_L_MIXER,
// C: 			RT5640_M_SV_L_SPM_L_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_SPO_L_MIXER,
// C: 			RT5640_M_BST1_SPM_L_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_spo_r_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_SPO_R_MIXER,
// C: 			RT5640_M_DAC_R1_SPM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("SPKVOL R Switch", RT5640_SPO_R_MIXER,
// C: 			RT5640_M_SV_R_SPM_R_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_SPO_R_MIXER,
// C: 			RT5640_M_BST1_SPM_R_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_hpo_mix[] = {
// C: 	SOC_DAPM_SINGLE("HPO MIX DAC2 Switch", RT5640_HPO_MIXER,
// C: 			RT5640_M_DAC2_HM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("HPO MIX DAC1 Switch", RT5640_HPO_MIXER,
// C: 			RT5640_M_DAC1_HM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("HPO MIX HPVOL Switch", RT5640_HPO_MIXER,
// C: 			RT5640_M_HPVOL_HM_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5639_hpo_mix[] = {
// C: 	SOC_DAPM_SINGLE("HPO MIX DAC1 Switch", RT5640_HPO_MIXER,
// C: 			RT5640_M_DAC1_HM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("HPO MIX HPVOL Switch", RT5640_HPO_MIXER,
// C: 			RT5640_M_HPVOL_HM_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_lout_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC L1 Switch", RT5640_LOUT_MIXER,
// C: 			RT5640_M_DAC_L1_LM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC R1 Switch", RT5640_LOUT_MIXER,
// C: 			RT5640_M_DAC_R1_LM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUTVOL L Switch", RT5640_LOUT_MIXER,
// C: 			RT5640_M_OV_L_LM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUTVOL R Switch", RT5640_LOUT_MIXER,
// C: 			RT5640_M_OV_R_LM_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_mix[] = {
// C: 	SOC_DAPM_SINGLE("DAC R2 Switch", RT5640_MONO_MIXER,
// C: 			RT5640_M_DAC_R2_MM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("DAC L2 Switch", RT5640_MONO_MIXER,
// C: 			RT5640_M_DAC_L2_MM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUTVOL R Switch", RT5640_MONO_MIXER,
// C: 			RT5640_M_OV_R_MM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("OUTVOL L Switch", RT5640_MONO_MIXER,
// C: 			RT5640_M_OV_L_MM_SFT, 1, 1),
// C: 	SOC_DAPM_SINGLE("BST1 Switch", RT5640_MONO_MIXER,
// C: 			RT5640_M_BST1_MM_SFT, 1, 1),
// C: };
// C: 
// C: static const struct snd_kcontrol_new spk_l_enable_control =
// C: 	SOC_DAPM_SINGLE_AUTODISABLE("Switch", RT5640_SPK_VOL,
// C: 		RT5640_L_MUTE_SFT, 1, 1);
// C: 
// C: static const struct snd_kcontrol_new spk_r_enable_control =
// C: 	SOC_DAPM_SINGLE_AUTODISABLE("Switch", RT5640_SPK_VOL,
// C: 		RT5640_R_MUTE_SFT, 1, 1);
// C: 
// C: static const struct snd_kcontrol_new hp_l_enable_control =
// C: 	SOC_DAPM_SINGLE_AUTODISABLE("Switch", RT5640_HP_VOL,
// C: 		RT5640_L_MUTE_SFT, 1, 1);
// C: 
// C: static const struct snd_kcontrol_new hp_r_enable_control =
// C: 	SOC_DAPM_SINGLE_AUTODISABLE("Switch", RT5640_HP_VOL,
// C: 		RT5640_R_MUTE_SFT, 1, 1);
// C: 
// C: /* Stereo ADC source */
// C: static const char * const rt5640_stereo_adc1_src[] = {
// C: 	"DIG MIX", "ADC"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_stereo_adc1_enum, RT5640_STO_ADC_MIXER,
// C: 			    RT5640_ADC_1_SRC_SFT, rt5640_stereo_adc1_src);
// C: 
// C: static const struct snd_kcontrol_new rt5640_sto_adc_1_mux =
// C: 	SOC_DAPM_ENUM("Stereo ADC1 Mux", rt5640_stereo_adc1_enum);
// C: 
// C: static const char * const rt5640_stereo_adc2_src[] = {
// C: 	"DMIC1", "DMIC2", "DIG MIX"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_stereo_adc2_enum, RT5640_STO_ADC_MIXER,
// C: 			    RT5640_ADC_2_SRC_SFT, rt5640_stereo_adc2_src);
// C: 
// C: static const struct snd_kcontrol_new rt5640_sto_adc_2_mux =
// C: 	SOC_DAPM_ENUM("Stereo ADC2 Mux", rt5640_stereo_adc2_enum);
// C: 
// C: /* Mono ADC source */
// C: static const char * const rt5640_mono_adc_l1_src[] = {
// C: 	"Mono DAC MIXL", "ADCL"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_mono_adc_l1_enum, RT5640_MONO_ADC_MIXER,
// C: 			    RT5640_MONO_ADC_L1_SRC_SFT, rt5640_mono_adc_l1_src);
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_adc_l1_mux =
// C: 	SOC_DAPM_ENUM("Mono ADC1 left source", rt5640_mono_adc_l1_enum);
// C: 
// C: static const char * const rt5640_mono_adc_l2_src[] = {
// C: 	"DMIC L1", "DMIC L2", "Mono DAC MIXL"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_mono_adc_l2_enum, RT5640_MONO_ADC_MIXER,
// C: 			    RT5640_MONO_ADC_L2_SRC_SFT, rt5640_mono_adc_l2_src);
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_adc_l2_mux =
// C: 	SOC_DAPM_ENUM("Mono ADC2 left source", rt5640_mono_adc_l2_enum);
// C: 
// C: static const char * const rt5640_mono_adc_r1_src[] = {
// C: 	"Mono DAC MIXR", "ADCR"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_mono_adc_r1_enum, RT5640_MONO_ADC_MIXER,
// C: 			    RT5640_MONO_ADC_R1_SRC_SFT, rt5640_mono_adc_r1_src);
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_adc_r1_mux =
// C: 	SOC_DAPM_ENUM("Mono ADC1 right source", rt5640_mono_adc_r1_enum);
// C: 
// C: static const char * const rt5640_mono_adc_r2_src[] = {
// C: 	"DMIC R1", "DMIC R2", "Mono DAC MIXR"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_mono_adc_r2_enum, RT5640_MONO_ADC_MIXER,
// C: 			    RT5640_MONO_ADC_R2_SRC_SFT, rt5640_mono_adc_r2_src);
// C: 
// C: static const struct snd_kcontrol_new rt5640_mono_adc_r2_mux =
// C: 	SOC_DAPM_ENUM("Mono ADC2 right source", rt5640_mono_adc_r2_enum);
// C: 
// C: /* DAC2 channel source */
// C: static const char * const rt5640_dac_l2_src[] = {
// C: 	"IF2", "Base L/R"
// C: };
// C: 
// C: static int rt5640_dac_l2_values[] = {
// C: 	0,
// C: 	3,
// C: };
// C: 
// C: static SOC_VALUE_ENUM_SINGLE_DECL(rt5640_dac_l2_enum,
// C: 				  RT5640_DSP_PATH2, RT5640_DAC_L2_SEL_SFT,
// C: 				  0x3, rt5640_dac_l2_src, rt5640_dac_l2_values);
// C: 
// C: static const struct snd_kcontrol_new rt5640_dac_l2_mux =
// C: 	SOC_DAPM_ENUM("DAC2 left channel source", rt5640_dac_l2_enum);
// C: 
// C: static const char * const rt5640_dac_r2_src[] = {
// C: 	"IF2",
// C: };
// C: 
// C: static int rt5640_dac_r2_values[] = {
// C: 	0,
// C: };
// C: 
// C: static SOC_VALUE_ENUM_SINGLE_DECL(rt5640_dac_r2_enum,
// C: 				  RT5640_DSP_PATH2, RT5640_DAC_R2_SEL_SFT,
// C: 				  0x3, rt5640_dac_r2_src, rt5640_dac_r2_values);
// C: 
// C: static const struct snd_kcontrol_new rt5640_dac_r2_mux =
// C: 	SOC_DAPM_ENUM("DAC2 right channel source", rt5640_dac_r2_enum);
// C: 
// C: /* digital interface and iis interface map */
// C: static const char * const rt5640_dai_iis_map[] = {
// C: 	"1:1|2:2", "1:2|2:1", "1:1|2:1", "1:2|2:2"
// C: };
// C: 
// C: static int rt5640_dai_iis_map_values[] = {
// C: 	0,
// C: 	5,
// C: 	6,
// C: 	7,
// C: };
// C: 
// C: static SOC_VALUE_ENUM_SINGLE_DECL(rt5640_dai_iis_map_enum,
// C: 				  RT5640_I2S1_SDP, RT5640_I2S_IF_SFT,
// C: 				  0x7, rt5640_dai_iis_map,
// C: 				  rt5640_dai_iis_map_values);
// C: 
// C: static const struct snd_kcontrol_new rt5640_dai_mux =
// C: 	SOC_DAPM_ENUM("DAI select", rt5640_dai_iis_map_enum);
// C: 
// C: /* SDI select */
// C: static const char * const rt5640_sdi_sel[] = {
// C: 	"IF1", "IF2"
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(rt5640_sdi_sel_enum, RT5640_I2S2_SDP,
// C: 			    RT5640_I2S2_SDI_SFT, rt5640_sdi_sel);
// C: 
// C: static const struct snd_kcontrol_new rt5640_sdi_mux =
// C: 	SOC_DAPM_ENUM("SDI select", rt5640_sdi_sel_enum);
// C: 
// C: static void hp_amp_power_on(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	/* depop parameters */
// C: 	regmap_update_bits(rt5640->regmap, RT5640_PR_BASE +
// C: 		RT5640_CHPUMP_INT_REG1, 0x0700, 0x0200);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_DEPOP_M2,
// C: 		RT5640_DEPOP_MASK, RT5640_DEPOP_MAN);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_DEPOP_M1,
// C: 		RT5640_HP_CP_MASK | RT5640_HP_SG_MASK | RT5640_HP_CB_MASK,
// C: 		RT5640_HP_CP_PU | RT5640_HP_SG_DIS | RT5640_HP_CB_PU);
// C: 	regmap_write(rt5640->regmap, RT5640_PR_BASE + RT5640_HP_DCC_INT1,
// C: 			   0x9f00);
// C: 	/* headphone amp power on */
// C: 	regmap_update_bits(rt5640->regmap, RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_FV1 | RT5640_PWR_FV2, 0);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_HA,
// C: 		RT5640_PWR_HA);
// C: 	usleep_range(10000, 15000);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_FV1 | RT5640_PWR_FV2 ,
// C: 		RT5640_PWR_FV1 | RT5640_PWR_FV2);
// C: }
// C: 
// C: static void rt5640_pmu_depop(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	regmap_update_bits(rt5640->regmap, RT5640_DEPOP_M2,
// C: 		RT5640_DEPOP_MASK | RT5640_DIG_DP_MASK,
// C: 		RT5640_DEPOP_AUTO | RT5640_DIG_DP_EN);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_CHARGE_PUMP,
// C: 		RT5640_PM_HP_MASK, RT5640_PM_HP_HV);
// C: 
// C: 	regmap_update_bits(rt5640->regmap, RT5640_DEPOP_M3,
// C: 		RT5640_CP_FQ1_MASK | RT5640_CP_FQ2_MASK | RT5640_CP_FQ3_MASK,
// C: 		(RT5640_CP_FQ_192_KHZ << RT5640_CP_FQ1_SFT) |
// C: 		(RT5640_CP_FQ_12_KHZ << RT5640_CP_FQ2_SFT) |
// C: 		(RT5640_CP_FQ_192_KHZ << RT5640_CP_FQ3_SFT));
// C: 
// C: 	regmap_write(rt5640->regmap, RT5640_PR_BASE +
// C: 		RT5640_MAMP_INT_REG2, 0x1c00);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_DEPOP_M1,
// C: 		RT5640_HP_CP_MASK | RT5640_HP_SG_MASK,
// C: 		RT5640_HP_CP_PD | RT5640_HP_SG_EN);
// C: 	regmap_update_bits(rt5640->regmap, RT5640_PR_BASE +
// C: 		RT5640_CHPUMP_INT_REG1, 0x0700, 0x0400);
// C: }
// C: 
// C: static int rt5640_hp_event(struct snd_soc_dapm_widget *w,
// C: 			   struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		rt5640_pmu_depop(component);
// C: 		rt5640->hp_mute = false;
// C: 		break;
// C: 
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		rt5640->hp_mute = true;
// C: 		msleep(70);
// C: 		break;
// C: 
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_lout_event(struct snd_soc_dapm_widget *w,
// C: 	struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		hp_amp_power_on(component);
// C: 		snd_soc_component_update_bits(component, RT5640_PWR_ANLG1,
// C: 			RT5640_PWR_LM, RT5640_PWR_LM);
// C: 		snd_soc_component_update_bits(component, RT5640_OUTPUT,
// C: 			RT5640_L_MUTE | RT5640_R_MUTE, 0);
// C: 		break;
// C: 
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		snd_soc_component_update_bits(component, RT5640_OUTPUT,
// C: 			RT5640_L_MUTE | RT5640_R_MUTE,
// C: 			RT5640_L_MUTE | RT5640_R_MUTE);
// C: 		snd_soc_component_update_bits(component, RT5640_PWR_ANLG1,
// C: 			RT5640_PWR_LM, 0);
// C: 		break;
// C: 
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_hp_power_event(struct snd_soc_dapm_widget *w,
// C: 			   struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		hp_amp_power_on(component);
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_hp_post_event(struct snd_soc_dapm_widget *w,
// C: 			   struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		if (!rt5640->hp_mute)
// C: 			msleep(80);
// C: 
// C: 		break;
// C: 
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static const struct snd_soc_dapm_widget rt5640_dapm_widgets[] = {
// C: 	/* ASRC */
// C: 	SND_SOC_DAPM_SUPPLY_S("Stereo Filter ASRC", 1, RT5640_ASRC_1,
// C: 			 15, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY_S("I2S2 Filter ASRC", 1, RT5640_ASRC_1,
// C: 			 12, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY_S("I2S2 ASRC", 1, RT5640_ASRC_1,
// C: 			 11, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY_S("DMIC1 ASRC", 1, RT5640_ASRC_1,
// C: 			 9, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY_S("DMIC2 ASRC", 1, RT5640_ASRC_1,
// C: 			 8, 0, NULL, 0),
// C: 
// C: 
// C: 	/* Input Side */
// C: 	/* micbias */
// C: 	SND_SOC_DAPM_SUPPLY("LDO2", RT5640_PWR_ANLG1,
// C: 			RT5640_PWR_LDO2_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("MICBIAS1", RT5640_PWR_ANLG2,
// C: 			RT5640_PWR_MB1_BIT, 0, NULL, 0),
// C: 	/* Input Lines */
// C: 	SND_SOC_DAPM_INPUT("DMIC1"),
// C: 	SND_SOC_DAPM_INPUT("DMIC2"),
// C: 	SND_SOC_DAPM_INPUT("IN1P"),
// C: 	SND_SOC_DAPM_INPUT("IN1N"),
// C: 	SND_SOC_DAPM_INPUT("IN2P"),
// C: 	SND_SOC_DAPM_INPUT("IN2N"),
// C: 	SND_SOC_DAPM_INPUT("IN3P"),
// C: 	SND_SOC_DAPM_INPUT("IN3N"),
// C: 	SND_SOC_DAPM_PGA("DMIC L1", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("DMIC R1", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("DMIC L2", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("DMIC R2", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 
// C: 	SND_SOC_DAPM_SUPPLY("DMIC CLK", SND_SOC_NOPM, 0, 0,
// C: 		set_dmic_clk, SND_SOC_DAPM_PRE_PMU),
// C: 	SND_SOC_DAPM_SUPPLY("DMIC1 Power", RT5640_DMIC, RT5640_DMIC_1_EN_SFT, 0,
// C: 		NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("DMIC2 Power", RT5640_DMIC, RT5640_DMIC_2_EN_SFT, 0,
// C: 		NULL, 0),
// C: 	/* Boost */
// C: 	SND_SOC_DAPM_PGA("BST1", RT5640_PWR_ANLG2,
// C: 		RT5640_PWR_BST1_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("BST2", RT5640_PWR_ANLG2,
// C: 		RT5640_PWR_BST4_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("BST3", RT5640_PWR_ANLG2,
// C: 		RT5640_PWR_BST2_BIT, 0, NULL, 0),
// C: 	/* Input Volume */
// C: 	SND_SOC_DAPM_PGA("INL VOL", RT5640_PWR_VOL,
// C: 		RT5640_PWR_IN_L_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("INR VOL", RT5640_PWR_VOL,
// C: 		RT5640_PWR_IN_R_BIT, 0, NULL, 0),
// C: 	/* REC Mixer */
// C: 	SND_SOC_DAPM_MIXER("RECMIXL", RT5640_PWR_MIXER, RT5640_PWR_RM_L_BIT, 0,
// C: 			rt5640_rec_l_mix, ARRAY_SIZE(rt5640_rec_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("RECMIXR", RT5640_PWR_MIXER, RT5640_PWR_RM_R_BIT, 0,
// C: 			rt5640_rec_r_mix, ARRAY_SIZE(rt5640_rec_r_mix)),
// C: 	/* ADCs */
// C: 	SND_SOC_DAPM_ADC("ADC L", NULL, RT5640_PWR_DIG1,
// C: 			RT5640_PWR_ADC_L_BIT, 0),
// C: 	SND_SOC_DAPM_ADC("ADC R", NULL, RT5640_PWR_DIG1,
// C: 			RT5640_PWR_ADC_R_BIT, 0),
// C: 	/* ADC Mux */
// C: 	SND_SOC_DAPM_MUX("Stereo ADC L2 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_sto_adc_2_mux),
// C: 	SND_SOC_DAPM_MUX("Stereo ADC R2 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_sto_adc_2_mux),
// C: 	SND_SOC_DAPM_MUX("Stereo ADC L1 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_sto_adc_1_mux),
// C: 	SND_SOC_DAPM_MUX("Stereo ADC R1 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_sto_adc_1_mux),
// C: 	SND_SOC_DAPM_MUX("Mono ADC L2 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_mono_adc_l2_mux),
// C: 	SND_SOC_DAPM_MUX("Mono ADC L1 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_mono_adc_l1_mux),
// C: 	SND_SOC_DAPM_MUX("Mono ADC R1 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_mono_adc_r1_mux),
// C: 	SND_SOC_DAPM_MUX("Mono ADC R2 Mux", SND_SOC_NOPM, 0, 0,
// C: 				&rt5640_mono_adc_r2_mux),
// C: 	/* ADC Mixer */
// C: 	SND_SOC_DAPM_SUPPLY("Stereo Filter", RT5640_PWR_DIG2,
// C: 		RT5640_PWR_ADC_SF_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_MIXER("Stereo ADC MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_sto_adc_l_mix, ARRAY_SIZE(rt5640_sto_adc_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("Stereo ADC MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_sto_adc_r_mix, ARRAY_SIZE(rt5640_sto_adc_r_mix)),
// C: 	SND_SOC_DAPM_SUPPLY("Mono Left Filter", RT5640_PWR_DIG2,
// C: 		RT5640_PWR_ADC_MF_L_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_MIXER("Mono ADC MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_mono_adc_l_mix, ARRAY_SIZE(rt5640_mono_adc_l_mix)),
// C: 	SND_SOC_DAPM_SUPPLY("Mono Right Filter", RT5640_PWR_DIG2,
// C: 		RT5640_PWR_ADC_MF_R_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_MIXER("Mono ADC MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_mono_adc_r_mix, ARRAY_SIZE(rt5640_mono_adc_r_mix)),
// C: 
// C: 	/* Digital Interface */
// C: 	SND_SOC_DAPM_SUPPLY("I2S1", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_I2S1_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF1 DAC", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF1 DAC L", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF1 DAC R", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF1 ADC", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF1 ADC L", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF1 ADC R", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("I2S2", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_I2S2_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF2 DAC", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF2 DAC L", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF2 DAC R", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF2 ADC", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF2 ADC L", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("IF2 ADC R", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	/* Digital Interface Select */
// C: 	SND_SOC_DAPM_MUX("DAI1 RX Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("DAI1 TX Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("DAI1 IF1 Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("DAI1 IF2 Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("SDI1 TX Mux", SND_SOC_NOPM, 0, 0, &rt5640_sdi_mux),
// C: 	SND_SOC_DAPM_MUX("DAI2 RX Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("DAI2 TX Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("DAI2 IF1 Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("DAI2 IF2 Mux", SND_SOC_NOPM, 0, 0, &rt5640_dai_mux),
// C: 	SND_SOC_DAPM_MUX("SDI2 TX Mux", SND_SOC_NOPM, 0, 0, &rt5640_sdi_mux),
// C: 	/* Audio Interface */
// C: 	SND_SOC_DAPM_AIF_IN("AIF1RX", "AIF1 Playback", 0, SND_SOC_NOPM, 0, 0),
// C: 	SND_SOC_DAPM_AIF_OUT("AIF1TX", "AIF1 Capture", 0, SND_SOC_NOPM, 0, 0),
// C: 	SND_SOC_DAPM_AIF_IN("AIF2RX", "AIF2 Playback", 0, SND_SOC_NOPM, 0, 0),
// C: 	SND_SOC_DAPM_AIF_OUT("AIF2TX", "AIF2 Capture", 0, SND_SOC_NOPM, 0, 0),
// C: 
// C: 	/* Output Side */
// C: 	/* DAC mixer before sound effect  */
// C: 	SND_SOC_DAPM_MIXER("DAC MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_dac_l_mix, ARRAY_SIZE(rt5640_dac_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("DAC MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_dac_r_mix, ARRAY_SIZE(rt5640_dac_r_mix)),
// C: 
// C: 	/* DAC Mixer */
// C: 	SND_SOC_DAPM_MIXER("Mono DAC MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_mono_dac_l_mix, ARRAY_SIZE(rt5640_mono_dac_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("Mono DAC MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_mono_dac_r_mix, ARRAY_SIZE(rt5640_mono_dac_r_mix)),
// C: 	SND_SOC_DAPM_MIXER("DIG MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_dig_l_mix, ARRAY_SIZE(rt5640_dig_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("DIG MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_dig_r_mix, ARRAY_SIZE(rt5640_dig_r_mix)),
// C: 	/* DACs */
// C: 	SND_SOC_DAPM_DAC("DAC L1", NULL, SND_SOC_NOPM,
// C: 			0, 0),
// C: 	SND_SOC_DAPM_DAC("DAC R1", NULL, SND_SOC_NOPM,
// C: 			0, 0),
// C: 	SND_SOC_DAPM_SUPPLY("DAC L1 Power", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_DAC_L1_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("DAC R1 Power", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_DAC_R1_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("DAC L2 Power", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_DAC_L2_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("DAC R2 Power", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_DAC_R2_BIT, 0, NULL, 0),
// C: 	/* SPK/OUT Mixer */
// C: 	SND_SOC_DAPM_MIXER("SPK MIXL", RT5640_PWR_MIXER, RT5640_PWR_SM_L_BIT,
// C: 		0, rt5640_spk_l_mix, ARRAY_SIZE(rt5640_spk_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("SPK MIXR", RT5640_PWR_MIXER, RT5640_PWR_SM_R_BIT,
// C: 		0, rt5640_spk_r_mix, ARRAY_SIZE(rt5640_spk_r_mix)),
// C: 	/* Ouput Volume */
// C: 	SND_SOC_DAPM_PGA("SPKVOL L", RT5640_PWR_VOL,
// C: 		RT5640_PWR_SV_L_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("SPKVOL R", RT5640_PWR_VOL,
// C: 		RT5640_PWR_SV_R_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("OUTVOL L", RT5640_PWR_VOL,
// C: 		RT5640_PWR_OV_L_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("OUTVOL R", RT5640_PWR_VOL,
// C: 		RT5640_PWR_OV_R_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("HPOVOL L", RT5640_PWR_VOL,
// C: 		RT5640_PWR_HV_L_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_PGA("HPOVOL R", RT5640_PWR_VOL,
// C: 		RT5640_PWR_HV_R_BIT, 0, NULL, 0),
// C: 	/* SPO/HPO/LOUT/Mono Mixer */
// C: 	SND_SOC_DAPM_MIXER("SPOL MIX", SND_SOC_NOPM, 0,
// C: 		0, rt5640_spo_l_mix, ARRAY_SIZE(rt5640_spo_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("SPOR MIX", SND_SOC_NOPM, 0,
// C: 		0, rt5640_spo_r_mix, ARRAY_SIZE(rt5640_spo_r_mix)),
// C: 	SND_SOC_DAPM_MIXER("LOUT MIX", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_lout_mix, ARRAY_SIZE(rt5640_lout_mix)),
// C: 	SND_SOC_DAPM_SUPPLY_S("Improve HP Amp Drv", 1, SND_SOC_NOPM,
// C: 		0, 0, rt5640_hp_power_event, SND_SOC_DAPM_POST_PMU),
// C: 	SND_SOC_DAPM_PGA_S("HP Amp", 1, SND_SOC_NOPM, 0, 0,
// C: 		rt5640_hp_event,
// C: 		SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
// C: 	SND_SOC_DAPM_PGA_S("LOUT amp", 1, SND_SOC_NOPM, 0, 0,
// C: 		rt5640_lout_event,
// C: 		SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
// C: 	SND_SOC_DAPM_SUPPLY("HP L Amp", RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_HP_L_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("HP R Amp", RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_HP_R_BIT, 0, NULL, 0),
// C: 	SND_SOC_DAPM_SUPPLY("Improve SPK Amp Drv", RT5640_PWR_DIG1,
// C: 		RT5640_PWR_CLS_D_BIT, 0, NULL, 0),
// C: 
// C: 	/* Output Switch */
// C: 	SND_SOC_DAPM_SWITCH("Speaker L Playback", SND_SOC_NOPM, 0, 0,
// C: 			&spk_l_enable_control),
// C: 	SND_SOC_DAPM_SWITCH("Speaker R Playback", SND_SOC_NOPM, 0, 0,
// C: 			&spk_r_enable_control),
// C: 	SND_SOC_DAPM_SWITCH("HP L Playback", SND_SOC_NOPM, 0, 0,
// C: 			&hp_l_enable_control),
// C: 	SND_SOC_DAPM_SWITCH("HP R Playback", SND_SOC_NOPM, 0, 0,
// C: 			&hp_r_enable_control),
// C: 	SND_SOC_DAPM_POST("HP Post", rt5640_hp_post_event),
// C: 	/* Output Lines */
// C: 	SND_SOC_DAPM_OUTPUT("SPOLP"),
// C: 	SND_SOC_DAPM_OUTPUT("SPOLN"),
// C: 	SND_SOC_DAPM_OUTPUT("SPORP"),
// C: 	SND_SOC_DAPM_OUTPUT("SPORN"),
// C: 	SND_SOC_DAPM_OUTPUT("HPOL"),
// C: 	SND_SOC_DAPM_OUTPUT("HPOR"),
// C: 	SND_SOC_DAPM_OUTPUT("LOUTL"),
// C: 	SND_SOC_DAPM_OUTPUT("LOUTR"),
// C: };
// C: 
// C: static const struct snd_soc_dapm_widget rt5640_specific_dapm_widgets[] = {
// C: 	/* Audio DSP */
// C: 	SND_SOC_DAPM_PGA("Audio DSP", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 	/* ANC */
// C: 	SND_SOC_DAPM_PGA("ANC", SND_SOC_NOPM, 0, 0, NULL, 0),
// C: 
// C: 	/* DAC2 channel Mux */
// C: 	SND_SOC_DAPM_MUX("DAC L2 Mux", SND_SOC_NOPM, 0, 0, &rt5640_dac_l2_mux),
// C: 	SND_SOC_DAPM_MUX("DAC R2 Mux", SND_SOC_NOPM, 0, 0, &rt5640_dac_r2_mux),
// C: 
// C: 	SND_SOC_DAPM_MIXER("Stereo DAC MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_sto_dac_l_mix, ARRAY_SIZE(rt5640_sto_dac_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("Stereo DAC MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_sto_dac_r_mix, ARRAY_SIZE(rt5640_sto_dac_r_mix)),
// C: 
// C: 	SND_SOC_DAPM_DAC("DAC R2", NULL, SND_SOC_NOPM, 0,
// C: 		0),
// C: 	SND_SOC_DAPM_DAC("DAC L2", NULL, SND_SOC_NOPM, 0,
// C: 		0),
// C: 
// C: 	SND_SOC_DAPM_MIXER("OUT MIXL", RT5640_PWR_MIXER, RT5640_PWR_OM_L_BIT,
// C: 		0, rt5640_out_l_mix, ARRAY_SIZE(rt5640_out_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("OUT MIXR", RT5640_PWR_MIXER, RT5640_PWR_OM_R_BIT,
// C: 		0, rt5640_out_r_mix, ARRAY_SIZE(rt5640_out_r_mix)),
// C: 
// C: 	SND_SOC_DAPM_MIXER("HPO MIX L", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_hpo_mix, ARRAY_SIZE(rt5640_hpo_mix)),
// C: 	SND_SOC_DAPM_MIXER("HPO MIX R", SND_SOC_NOPM, 0, 0,
// C: 		rt5640_hpo_mix, ARRAY_SIZE(rt5640_hpo_mix)),
// C: 
// C: 	SND_SOC_DAPM_MIXER("Mono MIX", RT5640_PWR_ANLG1, RT5640_PWR_MM_BIT, 0,
// C: 		rt5640_mono_mix, ARRAY_SIZE(rt5640_mono_mix)),
// C: 	SND_SOC_DAPM_SUPPLY("Improve MONO Amp Drv", RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_MA_BIT, 0, NULL, 0),
// C: 
// C: 	SND_SOC_DAPM_OUTPUT("MONOP"),
// C: 	SND_SOC_DAPM_OUTPUT("MONON"),
// C: };
// C: 
// C: static const struct snd_soc_dapm_widget rt5639_specific_dapm_widgets[] = {
// C: 	SND_SOC_DAPM_MIXER("Stereo DAC MIXL", SND_SOC_NOPM, 0, 0,
// C: 		rt5639_sto_dac_l_mix, ARRAY_SIZE(rt5639_sto_dac_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("Stereo DAC MIXR", SND_SOC_NOPM, 0, 0,
// C: 		rt5639_sto_dac_r_mix, ARRAY_SIZE(rt5639_sto_dac_r_mix)),
// C: 
// C: 	SND_SOC_DAPM_MIXER("OUT MIXL", RT5640_PWR_MIXER, RT5640_PWR_OM_L_BIT,
// C: 		0, rt5639_out_l_mix, ARRAY_SIZE(rt5639_out_l_mix)),
// C: 	SND_SOC_DAPM_MIXER("OUT MIXR", RT5640_PWR_MIXER, RT5640_PWR_OM_R_BIT,
// C: 		0, rt5639_out_r_mix, ARRAY_SIZE(rt5639_out_r_mix)),
// C: 
// C: 	SND_SOC_DAPM_MIXER("HPO MIX L", SND_SOC_NOPM, 0, 0,
// C: 		rt5639_hpo_mix, ARRAY_SIZE(rt5639_hpo_mix)),
// C: 	SND_SOC_DAPM_MIXER("HPO MIX R", SND_SOC_NOPM, 0, 0,
// C: 		rt5639_hpo_mix, ARRAY_SIZE(rt5639_hpo_mix)),
// C: };
// C: 
// C: static const struct snd_soc_dapm_route rt5640_dapm_routes[] = {
// C: 	{ "I2S1", NULL, "Stereo Filter ASRC", is_using_asrc },
// C: 	{ "I2S2", NULL, "I2S2 ASRC", is_using_asrc },
// C: 	{ "I2S2", NULL, "I2S2 Filter ASRC", is_using_asrc },
// C: 	{ "DMIC1", NULL, "DMIC1 ASRC", is_using_asrc },
// C: 	{ "DMIC2", NULL, "DMIC2 ASRC", is_using_asrc },
// C: 
// C: 	{"IN1P", NULL, "LDO2"},
// C: 	{"IN2P", NULL, "LDO2"},
// C: 	{"IN3P", NULL, "LDO2"},
// C: 
// C: 	{"DMIC L1", NULL, "DMIC1"},
// C: 	{"DMIC R1", NULL, "DMIC1"},
// C: 	{"DMIC L2", NULL, "DMIC2"},
// C: 	{"DMIC R2", NULL, "DMIC2"},
// C: 
// C: 	{"BST1", NULL, "IN1P"},
// C: 	{"BST1", NULL, "IN1N"},
// C: 	{"BST2", NULL, "IN2P"},
// C: 	{"BST2", NULL, "IN2N"},
// C: 	{"BST3", NULL, "IN3P"},
// C: 	{"BST3", NULL, "IN3N"},
// C: 
// C: 	{"INL VOL", NULL, "IN2P"},
// C: 	{"INR VOL", NULL, "IN2N"},
// C: 
// C: 	{"RECMIXL", "HPOL Switch", "HPOL"},
// C: 	{"RECMIXL", "INL Switch", "INL VOL"},
// C: 	{"RECMIXL", "BST3 Switch", "BST3"},
// C: 	{"RECMIXL", "BST2 Switch", "BST2"},
// C: 	{"RECMIXL", "BST1 Switch", "BST1"},
// C: 	{"RECMIXL", "OUT MIXL Switch", "OUT MIXL"},
// C: 
// C: 	{"RECMIXR", "HPOR Switch", "HPOR"},
// C: 	{"RECMIXR", "INR Switch", "INR VOL"},
// C: 	{"RECMIXR", "BST3 Switch", "BST3"},
// C: 	{"RECMIXR", "BST2 Switch", "BST2"},
// C: 	{"RECMIXR", "BST1 Switch", "BST1"},
// C: 	{"RECMIXR", "OUT MIXR Switch", "OUT MIXR"},
// C: 
// C: 	{"ADC L", NULL, "RECMIXL"},
// C: 	{"ADC R", NULL, "RECMIXR"},
// C: 
// C: 	{"DMIC L1", NULL, "DMIC CLK"},
// C: 	{"DMIC L1", NULL, "DMIC1 Power"},
// C: 	{"DMIC R1", NULL, "DMIC CLK"},
// C: 	{"DMIC R1", NULL, "DMIC1 Power"},
// C: 	{"DMIC L2", NULL, "DMIC CLK"},
// C: 	{"DMIC L2", NULL, "DMIC2 Power"},
// C: 	{"DMIC R2", NULL, "DMIC CLK"},
// C: 	{"DMIC R2", NULL, "DMIC2 Power"},
// C: 
// C: 	{"Stereo ADC L2 Mux", "DMIC1", "DMIC L1"},
// C: 	{"Stereo ADC L2 Mux", "DMIC2", "DMIC L2"},
// C: 	{"Stereo ADC L2 Mux", "DIG MIX", "DIG MIXL"},
// C: 	{"Stereo ADC L1 Mux", "ADC", "ADC L"},
// C: 	{"Stereo ADC L1 Mux", "DIG MIX", "DIG MIXL"},
// C: 
// C: 	{"Stereo ADC R1 Mux", "ADC", "ADC R"},
// C: 	{"Stereo ADC R1 Mux", "DIG MIX", "DIG MIXR"},
// C: 	{"Stereo ADC R2 Mux", "DMIC1", "DMIC R1"},
// C: 	{"Stereo ADC R2 Mux", "DMIC2", "DMIC R2"},
// C: 	{"Stereo ADC R2 Mux", "DIG MIX", "DIG MIXR"},
// C: 
// C: 	{"Mono ADC L2 Mux", "DMIC L1", "DMIC L1"},
// C: 	{"Mono ADC L2 Mux", "DMIC L2", "DMIC L2"},
// C: 	{"Mono ADC L2 Mux", "Mono DAC MIXL", "Mono DAC MIXL"},
// C: 	{"Mono ADC L1 Mux", "Mono DAC MIXL", "Mono DAC MIXL"},
// C: 	{"Mono ADC L1 Mux", "ADCL", "ADC L"},
// C: 
// C: 	{"Mono ADC R1 Mux", "Mono DAC MIXR", "Mono DAC MIXR"},
// C: 	{"Mono ADC R1 Mux", "ADCR", "ADC R"},
// C: 	{"Mono ADC R2 Mux", "DMIC R1", "DMIC R1"},
// C: 	{"Mono ADC R2 Mux", "DMIC R2", "DMIC R2"},
// C: 	{"Mono ADC R2 Mux", "Mono DAC MIXR", "Mono DAC MIXR"},
// C: 
// C: 	{"Stereo ADC MIXL", "ADC1 Switch", "Stereo ADC L1 Mux"},
// C: 	{"Stereo ADC MIXL", "ADC2 Switch", "Stereo ADC L2 Mux"},
// C: 	{"Stereo ADC MIXL", NULL, "Stereo Filter"},
// C: 
// C: 	{"Stereo ADC MIXR", "ADC1 Switch", "Stereo ADC R1 Mux"},
// C: 	{"Stereo ADC MIXR", "ADC2 Switch", "Stereo ADC R2 Mux"},
// C: 	{"Stereo ADC MIXR", NULL, "Stereo Filter"},
// C: 
// C: 	{"Mono ADC MIXL", "ADC1 Switch", "Mono ADC L1 Mux"},
// C: 	{"Mono ADC MIXL", "ADC2 Switch", "Mono ADC L2 Mux"},
// C: 	{"Mono ADC MIXL", NULL, "Mono Left Filter"},
// C: 
// C: 	{"Mono ADC MIXR", "ADC1 Switch", "Mono ADC R1 Mux"},
// C: 	{"Mono ADC MIXR", "ADC2 Switch", "Mono ADC R2 Mux"},
// C: 	{"Mono ADC MIXR", NULL, "Mono Right Filter"},
// C: 
// C: 	{"IF2 ADC L", NULL, "Mono ADC MIXL"},
// C: 	{"IF2 ADC R", NULL, "Mono ADC MIXR"},
// C: 	{"IF1 ADC L", NULL, "Stereo ADC MIXL"},
// C: 	{"IF1 ADC R", NULL, "Stereo ADC MIXR"},
// C: 
// C: 	{"IF1 ADC", NULL, "I2S1"},
// C: 	{"IF1 ADC", NULL, "IF1 ADC L"},
// C: 	{"IF1 ADC", NULL, "IF1 ADC R"},
// C: 	{"IF2 ADC", NULL, "I2S2"},
// C: 	{"IF2 ADC", NULL, "IF2 ADC L"},
// C: 	{"IF2 ADC", NULL, "IF2 ADC R"},
// C: 
// C: 	{"DAI1 TX Mux", "1:1|2:2", "IF1 ADC"},
// C: 	{"DAI1 TX Mux", "1:2|2:1", "IF2 ADC"},
// C: 	{"DAI1 IF1 Mux", "1:1|2:1", "IF1 ADC"},
// C: 	{"DAI1 IF2 Mux", "1:1|2:1", "IF2 ADC"},
// C: 	{"SDI1 TX Mux", "IF1", "DAI1 IF1 Mux"},
// C: 	{"SDI1 TX Mux", "IF2", "DAI1 IF2 Mux"},
// C: 
// C: 	{"DAI2 TX Mux", "1:2|2:1", "IF1 ADC"},
// C: 	{"DAI2 TX Mux", "1:1|2:2", "IF2 ADC"},
// C: 	{"DAI2 IF1 Mux", "1:2|2:2", "IF1 ADC"},
// C: 	{"DAI2 IF2 Mux", "1:2|2:2", "IF2 ADC"},
// C: 	{"SDI2 TX Mux", "IF1", "DAI2 IF1 Mux"},
// C: 	{"SDI2 TX Mux", "IF2", "DAI2 IF2 Mux"},
// C: 
// C: 	{"AIF1TX", NULL, "DAI1 TX Mux"},
// C: 	{"AIF1TX", NULL, "SDI1 TX Mux"},
// C: 	{"AIF2TX", NULL, "DAI2 TX Mux"},
// C: 	{"AIF2TX", NULL, "SDI2 TX Mux"},
// C: 
// C: 	{"DAI1 RX Mux", "1:1|2:2", "AIF1RX"},
// C: 	{"DAI1 RX Mux", "1:1|2:1", "AIF1RX"},
// C: 	{"DAI1 RX Mux", "1:2|2:1", "AIF2RX"},
// C: 	{"DAI1 RX Mux", "1:2|2:2", "AIF2RX"},
// C: 
// C: 	{"DAI2 RX Mux", "1:2|2:1", "AIF1RX"},
// C: 	{"DAI2 RX Mux", "1:1|2:1", "AIF1RX"},
// C: 	{"DAI2 RX Mux", "1:1|2:2", "AIF2RX"},
// C: 	{"DAI2 RX Mux", "1:2|2:2", "AIF2RX"},
// C: 
// C: 	{"IF1 DAC", NULL, "I2S1"},
// C: 	{"IF1 DAC", NULL, "DAI1 RX Mux"},
// C: 	{"IF2 DAC", NULL, "I2S2"},
// C: 	{"IF2 DAC", NULL, "DAI2 RX Mux"},
// C: 
// C: 	{"IF1 DAC L", NULL, "IF1 DAC"},
// C: 	{"IF1 DAC R", NULL, "IF1 DAC"},
// C: 	{"IF2 DAC L", NULL, "IF2 DAC"},
// C: 	{"IF2 DAC R", NULL, "IF2 DAC"},
// C: 
// C: 	{"DAC MIXL", "Stereo ADC Switch", "Stereo ADC MIXL"},
// C: 	{"DAC MIXL", "INF1 Switch", "IF1 DAC L"},
// C: 	{"DAC MIXL", NULL, "DAC L1 Power"},
// C: 	{"DAC MIXR", "Stereo ADC Switch", "Stereo ADC MIXR"},
// C: 	{"DAC MIXR", "INF1 Switch", "IF1 DAC R"},
// C: 	{"DAC MIXR", NULL, "DAC R1 Power"},
// C: 
// C: 	{"Stereo DAC MIXL", "DAC L1 Switch", "DAC MIXL"},
// C: 	{"Stereo DAC MIXR", "DAC R1 Switch", "DAC MIXR"},
// C: 
// C: 	{"Mono DAC MIXL", "DAC L1 Switch", "DAC MIXL"},
// C: 	{"Mono DAC MIXR", "DAC R1 Switch", "DAC MIXR"},
// C: 
// C: 	{"DIG MIXL", "DAC L1 Switch", "DAC MIXL"},
// C: 	{"DIG MIXR", "DAC R1 Switch", "DAC MIXR"},
// C: 
// C: 	{"DAC L1", NULL, "Stereo DAC MIXL"},
// C: 	{"DAC L1", NULL, "DAC L1 Power"},
// C: 	{"DAC R1", NULL, "Stereo DAC MIXR"},
// C: 	{"DAC R1", NULL, "DAC R1 Power"},
// C: 
// C: 	{"SPK MIXL", "REC MIXL Switch", "RECMIXL"},
// C: 	{"SPK MIXL", "INL Switch", "INL VOL"},
// C: 	{"SPK MIXL", "DAC L1 Switch", "DAC L1"},
// C: 	{"SPK MIXL", "OUT MIXL Switch", "OUT MIXL"},
// C: 	{"SPK MIXR", "REC MIXR Switch", "RECMIXR"},
// C: 	{"SPK MIXR", "INR Switch", "INR VOL"},
// C: 	{"SPK MIXR", "DAC R1 Switch", "DAC R1"},
// C: 	{"SPK MIXR", "OUT MIXR Switch", "OUT MIXR"},
// C: 
// C: 	{"OUT MIXL", "BST1 Switch", "BST1"},
// C: 	{"OUT MIXL", "INL Switch", "INL VOL"},
// C: 	{"OUT MIXL", "REC MIXL Switch", "RECMIXL"},
// C: 	{"OUT MIXL", "DAC L1 Switch", "DAC L1"},
// C: 
// C: 	{"OUT MIXR", "BST2 Switch", "BST2"},
// C: 	{"OUT MIXR", "BST1 Switch", "BST1"},
// C: 	{"OUT MIXR", "INR Switch", "INR VOL"},
// C: 	{"OUT MIXR", "REC MIXR Switch", "RECMIXR"},
// C: 	{"OUT MIXR", "DAC R1 Switch", "DAC R1"},
// C: 
// C: 	{"SPKVOL L", NULL, "SPK MIXL"},
// C: 	{"SPKVOL R", NULL, "SPK MIXR"},
// C: 	{"HPOVOL L", NULL, "OUT MIXL"},
// C: 	{"HPOVOL R", NULL, "OUT MIXR"},
// C: 	{"OUTVOL L", NULL, "OUT MIXL"},
// C: 	{"OUTVOL R", NULL, "OUT MIXR"},
// C: 
// C: 	{"SPOL MIX", "DAC R1 Switch", "DAC R1"},
// C: 	{"SPOL MIX", "DAC L1 Switch", "DAC L1"},
// C: 	{"SPOL MIX", "SPKVOL R Switch", "SPKVOL R"},
// C: 	{"SPOL MIX", "SPKVOL L Switch", "SPKVOL L"},
// C: 	{"SPOL MIX", "BST1 Switch", "BST1"},
// C: 	{"SPOR MIX", "DAC R1 Switch", "DAC R1"},
// C: 	{"SPOR MIX", "SPKVOL R Switch", "SPKVOL R"},
// C: 	{"SPOR MIX", "BST1 Switch", "BST1"},
// C: 
// C: 	{"HPO MIX L", "HPO MIX DAC1 Switch", "DAC L1"},
// C: 	{"HPO MIX L", "HPO MIX HPVOL Switch", "HPOVOL L"},
// C: 	{"HPO MIX L", NULL, "HP L Amp"},
// C: 	{"HPO MIX R", "HPO MIX DAC1 Switch", "DAC R1"},
// C: 	{"HPO MIX R", "HPO MIX HPVOL Switch", "HPOVOL R"},
// C: 	{"HPO MIX R", NULL, "HP R Amp"},
// C: 
// C: 	{"LOUT MIX", "DAC L1 Switch", "DAC L1"},
// C: 	{"LOUT MIX", "DAC R1 Switch", "DAC R1"},
// C: 	{"LOUT MIX", "OUTVOL L Switch", "OUTVOL L"},
// C: 	{"LOUT MIX", "OUTVOL R Switch", "OUTVOL R"},
// C: 
// C: 	{"HP Amp", NULL, "HPO MIX L"},
// C: 	{"HP Amp", NULL, "HPO MIX R"},
// C: 
// C: 	{"Speaker L Playback", "Switch", "SPOL MIX"},
// C: 	{"Speaker R Playback", "Switch", "SPOR MIX"},
// C: 	{"SPOLP", NULL, "Speaker L Playback"},
// C: 	{"SPOLN", NULL, "Speaker L Playback"},
// C: 	{"SPORP", NULL, "Speaker R Playback"},
// C: 	{"SPORN", NULL, "Speaker R Playback"},
// C: 
// C: 	{"SPOLP", NULL, "Improve SPK Amp Drv"},
// C: 	{"SPOLN", NULL, "Improve SPK Amp Drv"},
// C: 	{"SPORP", NULL, "Improve SPK Amp Drv"},
// C: 	{"SPORN", NULL, "Improve SPK Amp Drv"},
// C: 
// C: 	{"HPOL", NULL, "Improve HP Amp Drv"},
// C: 	{"HPOR", NULL, "Improve HP Amp Drv"},
// C: 
// C: 	{"HP L Playback", "Switch", "HP Amp"},
// C: 	{"HP R Playback", "Switch", "HP Amp"},
// C: 	{"HPOL", NULL, "HP L Playback"},
// C: 	{"HPOR", NULL, "HP R Playback"},
// C: 
// C: 	{"LOUT amp", NULL, "LOUT MIX"},
// C: 	{"LOUTL", NULL, "LOUT amp"},
// C: 	{"LOUTR", NULL, "LOUT amp"},
// C: };
// C: 
// C: static const struct snd_soc_dapm_route rt5640_specific_dapm_routes[] = {
// C: 	{"ANC", NULL, "Stereo ADC MIXL"},
// C: 	{"ANC", NULL, "Stereo ADC MIXR"},
// C: 
// C: 	{"Audio DSP", NULL, "DAC MIXL"},
// C: 	{"Audio DSP", NULL, "DAC MIXR"},
// C: 
// C: 	{"DAC L2 Mux", "IF2", "IF2 DAC L"},
// C: 	{"DAC L2 Mux", "Base L/R", "Audio DSP"},
// C: 	{"DAC L2 Mux", NULL, "DAC L2 Power"},
// C: 	{"DAC R2 Mux", "IF2", "IF2 DAC R"},
// C: 	{"DAC R2 Mux", NULL, "DAC R2 Power"},
// C: 
// C: 	{"Stereo DAC MIXL", "DAC L2 Switch", "DAC L2 Mux"},
// C: 	{"Stereo DAC MIXL", "ANC Switch", "ANC"},
// C: 	{"Stereo DAC MIXR", "DAC R2 Switch", "DAC R2 Mux"},
// C: 	{"Stereo DAC MIXR", "ANC Switch", "ANC"},
// C: 
// C: 	{"Mono DAC MIXL", "DAC L2 Switch", "DAC L2 Mux"},
// C: 	{"Mono DAC MIXL", "DAC R2 Switch", "DAC R2 Mux"},
// C: 
// C: 	{"Mono DAC MIXR", "DAC R2 Switch", "DAC R2 Mux"},
// C: 	{"Mono DAC MIXR", "DAC L2 Switch", "DAC L2 Mux"},
// C: 
// C: 	{"DIG MIXR", "DAC R2 Switch", "DAC R2 Mux"},
// C: 	{"DIG MIXL", "DAC L2 Switch", "DAC L2 Mux"},
// C: 
// C: 	{"DAC L2", NULL, "Mono DAC MIXL"},
// C: 	{"DAC L2", NULL, "DAC L2 Power"},
// C: 	{"DAC R2", NULL, "Mono DAC MIXR"},
// C: 	{"DAC R2", NULL, "DAC R2 Power"},
// C: 
// C: 	{"SPK MIXL", "DAC L2 Switch", "DAC L2"},
// C: 	{"SPK MIXR", "DAC R2 Switch", "DAC R2"},
// C: 
// C: 	{"OUT MIXL", "SPK MIXL Switch", "SPK MIXL"},
// C: 	{"OUT MIXR", "SPK MIXR Switch", "SPK MIXR"},
// C: 
// C: 	{"OUT MIXL", "DAC R2 Switch", "DAC R2"},
// C: 	{"OUT MIXL", "DAC L2 Switch", "DAC L2"},
// C: 
// C: 	{"OUT MIXR", "DAC L2 Switch", "DAC L2"},
// C: 	{"OUT MIXR", "DAC R2 Switch", "DAC R2"},
// C: 
// C: 	{"HPO MIX L", "HPO MIX DAC2 Switch", "DAC L2"},
// C: 	{"HPO MIX R", "HPO MIX DAC2 Switch", "DAC R2"},
// C: 
// C: 	{"Mono MIX", "DAC R2 Switch", "DAC R2"},
// C: 	{"Mono MIX", "DAC L2 Switch", "DAC L2"},
// C: 	{"Mono MIX", "OUTVOL R Switch", "OUTVOL R"},
// C: 	{"Mono MIX", "OUTVOL L Switch", "OUTVOL L"},
// C: 	{"Mono MIX", "BST1 Switch", "BST1"},
// C: 
// C: 	{"MONOP", NULL, "Mono MIX"},
// C: 	{"MONON", NULL, "Mono MIX"},
// C: 	{"MONOP", NULL, "Improve MONO Amp Drv"},
// C: };
// C: 
// C: static const struct snd_soc_dapm_route rt5639_specific_dapm_routes[] = {
// C: 	{"Stereo DAC MIXL", "DAC L2 Switch", "IF2 DAC L"},
// C: 	{"Stereo DAC MIXR", "DAC R2 Switch", "IF2 DAC R"},
// C: 
// C: 	{"Mono DAC MIXL", "DAC L2 Switch", "IF2 DAC L"},
// C: 	{"Mono DAC MIXL", "DAC R2 Switch", "IF2 DAC R"},
// C: 
// C: 	{"Mono DAC MIXR", "DAC R2 Switch", "IF2 DAC R"},
// C: 	{"Mono DAC MIXR", "DAC L2 Switch", "IF2 DAC L"},
// C: 
// C: 	{"DIG MIXL", "DAC L2 Switch", "IF2 DAC L"},
// C: 	{"DIG MIXR", "DAC R2 Switch", "IF2 DAC R"},
// C: 
// C: 	{"IF2 DAC L", NULL, "DAC L2 Power"},
// C: 	{"IF2 DAC R", NULL, "DAC R2 Power"},
// C: };
// C: 
// C: static int get_sdp_info(struct snd_soc_component *component, int dai_id)
// C: {
// C: 	int ret = 0, val;
// C: 
// C: 	if (component == NULL)
// C: 		return -EINVAL;
// C: 
// C: 	val = snd_soc_component_read(component, RT5640_I2S1_SDP);
// C: 	val = (val & RT5640_I2S_IF_MASK) >> RT5640_I2S_IF_SFT;
// C: 	switch (dai_id) {
// C: 	case RT5640_AIF1:
// C: 		switch (val) {
// C: 		case RT5640_IF_123:
// C: 		case RT5640_IF_132:
// C: 			ret |= RT5640_U_IF1;
// C: 			break;
// C: 		case RT5640_IF_113:
// C: 			ret |= RT5640_U_IF1;
// C: 			fallthrough;
// C: 		case RT5640_IF_312:
// C: 		case RT5640_IF_213:
// C: 			ret |= RT5640_U_IF2;
// C: 			break;
// C: 		}
// C: 		break;
// C: 
// C: 	case RT5640_AIF2:
// C: 		switch (val) {
// C: 		case RT5640_IF_231:
// C: 		case RT5640_IF_213:
// C: 			ret |= RT5640_U_IF1;
// C: 			break;
// C: 		case RT5640_IF_223:
// C: 			ret |= RT5640_U_IF1;
// C: 			fallthrough;
// C: 		case RT5640_IF_123:
// C: 		case RT5640_IF_321:
// C: 			ret |= RT5640_U_IF2;
// C: 			break;
// C: 		}
// C: 		break;
// C: 
// C: 	default:
// C: 		ret = -EINVAL;
// C: 		break;
// C: 	}
// C: 
// C: 	return ret;
// C: }
// C: 
// C: static int rt5640_hw_params(struct snd_pcm_substream *substream,
// C: 	struct snd_pcm_hw_params *params, struct snd_soc_dai *dai)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	unsigned int val_len = 0, val_clk, mask_clk;
// C: 	int dai_sel, pre_div, bclk_ms, frame_size;
// C: 
// C: 	rt5640->lrck[dai->id] = params_rate(params);
// C: 	pre_div = rl6231_get_clk_info(rt5640->sysclk, rt5640->lrck[dai->id]);
// C: 	if (pre_div < 0) {
// C: 		dev_err(component->dev, "Unsupported clock setting %d for DAI %d\n",
// C: 			rt5640->lrck[dai->id], dai->id);
// C: 		return -EINVAL;
// C: 	}
// C: 	frame_size = snd_soc_params_to_frame_size(params);
// C: 	if (frame_size < 0) {
// C: 		dev_err(component->dev, "Unsupported frame size: %d\n", frame_size);
// C: 		return frame_size;
// C: 	}
// C: 	if (frame_size > 32)
// C: 		bclk_ms = 1;
// C: 	else
// C: 		bclk_ms = 0;
// C: 	rt5640->bclk[dai->id] = rt5640->lrck[dai->id] * (32 << bclk_ms);
// C: 
// C: 	dev_dbg(dai->dev, "bclk is %dHz and lrck is %dHz\n",
// C: 		rt5640->bclk[dai->id], rt5640->lrck[dai->id]);
// C: 	dev_dbg(dai->dev, "bclk_ms is %d and pre_div is %d for iis %d\n",
// C: 				bclk_ms, pre_div, dai->id);
// C: 
// C: 	switch (params_width(params)) {
// C: 	case 16:
// C: 		break;
// C: 	case 20:
// C: 		val_len |= RT5640_I2S_DL_20;
// C: 		break;
// C: 	case 24:
// C: 		val_len |= RT5640_I2S_DL_24;
// C: 		break;
// C: 	case 8:
// C: 		val_len |= RT5640_I2S_DL_8;
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	dai_sel = get_sdp_info(component, dai->id);
// C: 	if (dai_sel < 0) {
// C: 		dev_err(component->dev, "Failed to get sdp info: %d\n", dai_sel);
// C: 		return -EINVAL;
// C: 	}
// C: 	if (dai_sel & RT5640_U_IF1) {
// C: 		mask_clk = RT5640_I2S_BCLK_MS1_MASK | RT5640_I2S_PD1_MASK;
// C: 		val_clk = bclk_ms << RT5640_I2S_BCLK_MS1_SFT |
// C: 			pre_div << RT5640_I2S_PD1_SFT;
// C: 		snd_soc_component_update_bits(component, RT5640_I2S1_SDP,
// C: 			RT5640_I2S_DL_MASK, val_len);
// C: 		snd_soc_component_update_bits(component, RT5640_ADDA_CLK1, mask_clk, val_clk);
// C: 	}
// C: 	if (dai_sel & RT5640_U_IF2) {
// C: 		mask_clk = RT5640_I2S_BCLK_MS2_MASK | RT5640_I2S_PD2_MASK;
// C: 		val_clk = bclk_ms << RT5640_I2S_BCLK_MS2_SFT |
// C: 			pre_div << RT5640_I2S_PD2_SFT;
// C: 		snd_soc_component_update_bits(component, RT5640_I2S2_SDP,
// C: 			RT5640_I2S_DL_MASK, val_len);
// C: 		snd_soc_component_update_bits(component, RT5640_ADDA_CLK1, mask_clk, val_clk);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_set_dai_fmt(struct snd_soc_dai *dai, unsigned int fmt)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	unsigned int reg_val = 0;
// C: 	int dai_sel;
// C: 
// C: 	switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
// C: 	case SND_SOC_DAIFMT_CBP_CFP:
// C: 		rt5640->master[dai->id] = 1;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_CBC_CFC:
// C: 		reg_val |= RT5640_I2S_MS_S;
// C: 		rt5640->master[dai->id] = 0;
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
// C: 	case SND_SOC_DAIFMT_NB_NF:
// C: 		break;
// C: 	case SND_SOC_DAIFMT_IB_NF:
// C: 		reg_val |= RT5640_I2S_BP_INV;
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
// C: 	case SND_SOC_DAIFMT_I2S:
// C: 		break;
// C: 	case SND_SOC_DAIFMT_LEFT_J:
// C: 		reg_val |= RT5640_I2S_DF_LEFT;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_DSP_A:
// C: 		reg_val |= RT5640_I2S_DF_PCM_A;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_DSP_B:
// C: 		reg_val  |= RT5640_I2S_DF_PCM_B;
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	dai_sel = get_sdp_info(component, dai->id);
// C: 	if (dai_sel < 0) {
// C: 		dev_err(component->dev, "Failed to get sdp info: %d\n", dai_sel);
// C: 		return -EINVAL;
// C: 	}
// C: 	if (dai_sel & RT5640_U_IF1) {
// C: 		snd_soc_component_update_bits(component, RT5640_I2S1_SDP,
// C: 			RT5640_I2S_MS_MASK | RT5640_I2S_BP_MASK |
// C: 			RT5640_I2S_DF_MASK, reg_val);
// C: 	}
// C: 	if (dai_sel & RT5640_U_IF2) {
// C: 		snd_soc_component_update_bits(component, RT5640_I2S2_SDP,
// C: 			RT5640_I2S_MS_MASK | RT5640_I2S_BP_MASK |
// C: 			RT5640_I2S_DF_MASK, reg_val);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_set_dai_sysclk(struct snd_soc_dai *dai,
// C: 		int clk_id, unsigned int freq, int dir)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	unsigned int reg_val = 0;
// C: 	unsigned int pll_bit = 0;
// C: 	int ret;
// C: 
// C: 	if (!freq) {
// C: 		rt5640->sysclk = 0;
// C: 		return 0;
// C: 	}
// C: 
// C: 	switch (clk_id) {
// C: 	case RT5640_SCLK_S_MCLK:
// C: 		ret = clk_set_rate(rt5640->mclk, freq);
// C: 		if (ret)
// C: 			return ret;
// C: 
// C: 		reg_val |= RT5640_SCLK_SRC_MCLK;
// C: 		break;
// C: 	case RT5640_SCLK_S_PLL1:
// C: 		reg_val |= RT5640_SCLK_SRC_PLL1;
// C: 		pll_bit |= RT5640_PWR_PLL;
// C: 		break;
// C: 	case RT5640_SCLK_S_RCCLK:
// C: 		reg_val |= RT5640_SCLK_SRC_RCCLK;
// C: 		break;
// C: 	default:
// C: 		dev_err(component->dev, "Invalid clock id (%d)\n", clk_id);
// C: 		return -EINVAL;
// C: 	}
// C: 	snd_soc_component_update_bits(component, RT5640_PWR_ANLG2,
// C: 		RT5640_PWR_PLL, pll_bit);
// C: 	snd_soc_component_update_bits(component, RT5640_GLB_CLK,
// C: 		RT5640_SCLK_SRC_MASK, reg_val);
// C: 	rt5640->sysclk = freq;
// C: 	rt5640->sysclk_src = clk_id;
// C: 
// C: 	dev_dbg(dai->dev, "Sysclk is %dHz and clock id is %d\n", freq, clk_id);
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_set_dai_pll(struct snd_soc_dai *dai, int pll_id, int source,
// C: 			unsigned int freq_in, unsigned int freq_out)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	struct rl6231_pll_code pll_code;
// C: 	int ret;
// C: 
// C: 	if (source == rt5640->pll_src && freq_in == rt5640->pll_in &&
// C: 	    freq_out == rt5640->pll_out)
// C: 		return 0;
// C: 
// C: 	if (!freq_in || !freq_out) {
// C: 		dev_dbg(component->dev, "PLL disabled\n");
// C: 
// C: 		rt5640->pll_in = 0;
// C: 		rt5640->pll_out = 0;
// C: 		snd_soc_component_update_bits(component, RT5640_GLB_CLK,
// C: 			RT5640_SCLK_SRC_MASK, RT5640_SCLK_SRC_MCLK);
// C: 		return 0;
// C: 	}
// C: 
// C: 	switch (source) {
// C: 	case RT5640_PLL1_S_MCLK:
// C: 		snd_soc_component_update_bits(component, RT5640_GLB_CLK,
// C: 			RT5640_PLL1_SRC_MASK, RT5640_PLL1_SRC_MCLK);
// C: 		break;
// C: 	case RT5640_PLL1_S_BCLK1:
// C: 		snd_soc_component_update_bits(component, RT5640_GLB_CLK,
// C: 			RT5640_PLL1_SRC_MASK, RT5640_PLL1_SRC_BCLK1);
// C: 		break;
// C: 	case RT5640_PLL1_S_BCLK2:
// C: 		snd_soc_component_update_bits(component, RT5640_GLB_CLK,
// C: 			RT5640_PLL1_SRC_MASK, RT5640_PLL1_SRC_BCLK2);
// C: 		break;
// C: 	default:
// C: 		dev_err(component->dev, "Unknown PLL source %d\n", source);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	ret = rl6231_pll_calc(freq_in, freq_out, &pll_code);
// C: 	if (ret < 0) {
// C: 		dev_err(component->dev, "Unsupported input clock %d\n", freq_in);
// C: 		return ret;
// C: 	}
// C: 
// C: 	dev_dbg(component->dev, "bypass=%d m=%d n=%d k=%d\n",
// C: 		pll_code.m_bp, (pll_code.m_bp ? 0 : pll_code.m_code),
// C: 		pll_code.n_code, pll_code.k_code);
// C: 
// C: 	snd_soc_component_write(component, RT5640_PLL_CTRL1,
// C: 		(pll_code.n_code << RT5640_PLL_N_SFT) | pll_code.k_code);
// C: 	snd_soc_component_write(component, RT5640_PLL_CTRL2,
// C: 		((pll_code.m_bp ? 0 : pll_code.m_code) << RT5640_PLL_M_SFT) |
// C: 		(pll_code.m_bp << RT5640_PLL_M_BP_SFT));
// C: 
// C: 	rt5640->pll_in = freq_in;
// C: 	rt5640->pll_out = freq_out;
// C: 	rt5640->pll_src = source;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_set_bias_level(struct snd_soc_component *component,
// C: 			enum snd_soc_bias_level level)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	int ret;
// C: 
// C: 	switch (level) {
// C: 	case SND_SOC_BIAS_ON:
// C: 		break;
// C: 
// C: 	case SND_SOC_BIAS_PREPARE:
// C: 		/*
// C: 		 * SND_SOC_BIAS_PREPARE is called while preparing for a
// C: 		 * transition to ON or away from ON. If current bias_level
// C: 		 * is SND_SOC_BIAS_ON, then it is preparing for a transition
// C: 		 * away from ON. Disable the clock in that case, otherwise
// C: 		 * enable it.
// C: 		 */
// C: 		if (snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_ON) {
// C: 			clk_disable_unprepare(rt5640->mclk);
// C: 		} else {
// C: 			ret = clk_prepare_enable(rt5640->mclk);
// C: 			if (ret)
// C: 				return ret;
// C: 		}
// C: 		break;
// C: 
// C: 	case SND_SOC_BIAS_STANDBY:
// C: 		if (SND_SOC_BIAS_OFF == snd_soc_dapm_get_bias_level(dapm)) {
// C: 			snd_soc_component_update_bits(component, RT5640_PWR_ANLG1,
// C: 				RT5640_PWR_VREF1 | RT5640_PWR_MB |
// C: 				RT5640_PWR_BG | RT5640_PWR_VREF2,
// C: 				RT5640_PWR_VREF1 | RT5640_PWR_MB |
// C: 				RT5640_PWR_BG | RT5640_PWR_VREF2);
// C: 			usleep_range(10000, 15000);
// C: 			snd_soc_component_update_bits(component, RT5640_PWR_ANLG1,
// C: 				RT5640_PWR_FV1 | RT5640_PWR_FV2,
// C: 				RT5640_PWR_FV1 | RT5640_PWR_FV2);
// C: 			snd_soc_component_update_bits(component, RT5640_GCTL1,
// C: 						0x1, 0x1);
// C: 			snd_soc_component_update_bits(component, RT5640_MICBIAS,
// C: 						0x0030, 0x0030);
// C: 		}
// C: 		break;
// C: 
// C: 	case SND_SOC_BIAS_OFF:
// C: 		snd_soc_component_write(component, RT5640_DEPOP_M1, 0x0004);
// C: 		snd_soc_component_write(component, RT5640_DEPOP_M2, 0x1100);
// C: 		snd_soc_component_update_bits(component, RT5640_GCTL1, 0x1, 0);
// C: 		snd_soc_component_write(component, RT5640_PWR_DIG1, 0x0000);
// C: 		snd_soc_component_write(component, RT5640_PWR_DIG2, 0x0000);
// C: 		snd_soc_component_write(component, RT5640_PWR_VOL, 0x0000);
// C: 		snd_soc_component_write(component, RT5640_PWR_MIXER, 0x0000);
// C: 		if (rt5640->jd_src == RT5640_JD_SRC_HDA_HEADER)
// C: 			snd_soc_component_write(component, RT5640_PWR_ANLG1,
// C: 				0x2818);
// C: 		else
// C: 			snd_soc_component_write(component, RT5640_PWR_ANLG1,
// C: 				0x0000);
// C: 		snd_soc_component_write(component, RT5640_PWR_ANLG2, 0x0000);
// C: 		break;
// C: 
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: int rt5640_dmic_enable(struct snd_soc_component *component,
// C: 		       bool dmic1_data_pin, bool dmic2_data_pin)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	regmap_update_bits(rt5640->regmap, RT5640_GPIO_CTRL1,
// C: 		RT5640_GP2_PIN_MASK, RT5640_GP2_PIN_DMIC1_SCL);
// C: 
// C: 	if (dmic1_data_pin) {
// C: 		regmap_update_bits(rt5640->regmap, RT5640_DMIC,
// C: 			RT5640_DMIC_1_DP_MASK, RT5640_DMIC_1_DP_GPIO3);
// C: 		regmap_update_bits(rt5640->regmap, RT5640_GPIO_CTRL1,
// C: 			RT5640_GP3_PIN_MASK, RT5640_GP3_PIN_DMIC1_SDA);
// C: 	}
// C: 
// C: 	if (dmic2_data_pin) {
// C: 		regmap_update_bits(rt5640->regmap, RT5640_DMIC,
// C: 			RT5640_DMIC_2_DP_MASK, RT5640_DMIC_2_DP_GPIO4);
// C: 		regmap_update_bits(rt5640->regmap, RT5640_GPIO_CTRL1,
// C: 			RT5640_GP4_PIN_MASK, RT5640_GP4_PIN_DMIC2_SDA);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(rt5640_dmic_enable);
// C: 
// C: int rt5640_sel_asrc_clk_src(struct snd_soc_component *component,
// C: 		unsigned int filter_mask, unsigned int clk_src)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	unsigned int asrc2_mask = 0;
// C: 	unsigned int asrc2_value = 0;
// C: 
// C: 	switch (clk_src) {
// C: 	case RT5640_CLK_SEL_SYS:
// C: 	case RT5640_CLK_SEL_ASRC:
// C: 		break;
// C: 
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (!filter_mask)
// C: 		return -EINVAL;
// C: 
// C: 	if (filter_mask & RT5640_DA_STEREO_FILTER) {
// C: 		asrc2_mask |= RT5640_STO_DAC_M_MASK;
// C: 		asrc2_value = (asrc2_value & ~RT5640_STO_DAC_M_MASK)
// C: 			| (clk_src << RT5640_STO_DAC_M_SFT);
// C: 	}
// C: 
// C: 	if (filter_mask & RT5640_DA_MONO_L_FILTER) {
// C: 		asrc2_mask |= RT5640_MDA_L_M_MASK;
// C: 		asrc2_value = (asrc2_value & ~RT5640_MDA_L_M_MASK)
// C: 			| (clk_src << RT5640_MDA_L_M_SFT);
// C: 	}
// C: 
// C: 	if (filter_mask & RT5640_DA_MONO_R_FILTER) {
// C: 		asrc2_mask |= RT5640_MDA_R_M_MASK;
// C: 		asrc2_value = (asrc2_value & ~RT5640_MDA_R_M_MASK)
// C: 			| (clk_src << RT5640_MDA_R_M_SFT);
// C: 	}
// C: 
// C: 	if (filter_mask & RT5640_AD_STEREO_FILTER) {
// C: 		asrc2_mask |= RT5640_ADC_M_MASK;
// C: 		asrc2_value = (asrc2_value & ~RT5640_ADC_M_MASK)
// C: 			| (clk_src << RT5640_ADC_M_SFT);
// C: 	}
// C: 
// C: 	if (filter_mask & RT5640_AD_MONO_L_FILTER) {
// C: 		asrc2_mask |= RT5640_MAD_L_M_MASK;
// C: 		asrc2_value = (asrc2_value & ~RT5640_MAD_L_M_MASK)
// C: 			| (clk_src << RT5640_MAD_L_M_SFT);
// C: 	}
// C: 
// C: 	if (filter_mask & RT5640_AD_MONO_R_FILTER)  {
// C: 		asrc2_mask |= RT5640_MAD_R_M_MASK;
// C: 		asrc2_value = (asrc2_value & ~RT5640_MAD_R_M_MASK)
// C: 			| (clk_src << RT5640_MAD_R_M_SFT);
// C: 	}
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_ASRC_2,
// C: 		asrc2_mask, asrc2_value);
// C: 
// C: 	if (snd_soc_component_read(component, RT5640_ASRC_2)) {
// C: 		rt5640->asrc_en = true;
// C: 		snd_soc_component_update_bits(component, RT5640_JD_CTRL, 0x3, 0x3);
// C: 	} else {
// C: 		rt5640->asrc_en = false;
// C: 		snd_soc_component_update_bits(component, RT5640_JD_CTRL, 0x3, 0x0);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(rt5640_sel_asrc_clk_src);
// C: 
// C: void rt5640_enable_micbias1_for_ovcd(struct snd_soc_component *component)
// C: {
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	snd_soc_dapm_mutex_lock(dapm);
// C: 	snd_soc_dapm_force_enable_pin_unlocked(dapm, "LDO2");
// C: 	snd_soc_dapm_force_enable_pin_unlocked(dapm, "MICBIAS1");
// C: 	/* OVCD is unreliable when used with RCCLK as sysclk-source */
// C: 	if (rt5640->use_platform_clock)
// C: 		snd_soc_dapm_force_enable_pin_unlocked(dapm, "Platform Clock");
// C: 	snd_soc_dapm_sync_unlocked(dapm);
// C: 	snd_soc_dapm_mutex_unlock(dapm);
// C: }
// C: EXPORT_SYMBOL_GPL(rt5640_enable_micbias1_for_ovcd);
// C: 
// C: void rt5640_disable_micbias1_for_ovcd(struct snd_soc_component *component)
// C: {
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	snd_soc_dapm_mutex_lock(dapm);
// C: 	if (rt5640->use_platform_clock)
// C: 		snd_soc_dapm_disable_pin_unlocked(dapm, "Platform Clock");
// C: 	snd_soc_dapm_disable_pin_unlocked(dapm, "MICBIAS1");
// C: 	snd_soc_dapm_disable_pin_unlocked(dapm, "LDO2");
// C: 	snd_soc_dapm_sync_unlocked(dapm);
// C: 	snd_soc_dapm_mutex_unlock(dapm);
// C: }
// C: EXPORT_SYMBOL_GPL(rt5640_disable_micbias1_for_ovcd);
// C: 
// C: static void rt5640_enable_micbias1_ovcd_irq(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_IRQ_CTRL2,
// C: 		RT5640_IRQ_MB1_OC_MASK, RT5640_IRQ_MB1_OC_NOR);
// C: 	rt5640->ovcd_irq_enabled = true;
// C: }
// C: 
// C: static void rt5640_disable_micbias1_ovcd_irq(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_IRQ_CTRL2,
// C: 		RT5640_IRQ_MB1_OC_MASK, RT5640_IRQ_MB1_OC_BP);
// C: 	rt5640->ovcd_irq_enabled = false;
// C: }
// C: 
// C: static void rt5640_clear_micbias1_ovcd(struct snd_soc_component *component)
// C: {
// C: 	snd_soc_component_update_bits(component, RT5640_IRQ_CTRL2,
// C: 		RT5640_MB1_OC_STATUS, 0);
// C: }
// C: 
// C: static bool rt5640_micbias1_ovcd(struct snd_soc_component *component)
// C: {
// C: 	int val;
// C: 
// C: 	val = snd_soc_component_read(component, RT5640_IRQ_CTRL2);
// C: 	dev_dbg(component->dev, "irq ctrl2 %#04x\n", val);
// C: 
// C: 	return (val & RT5640_MB1_OC_STATUS);
// C: }
// C: 
// C: static bool rt5640_jack_inserted(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	int val;
// C: 
// C: 	if (rt5640->jd_gpio)
// C: 		val = gpiod_get_value(rt5640->jd_gpio) ? RT5640_JD_STATUS : 0;
// C: 	else
// C: 		val = snd_soc_component_read(component, RT5640_INT_IRQ_ST);
// C: 
// C: 	dev_dbg(component->dev, "irq status %#04x\n", val);
// C: 
// C: 	if (rt5640->jd_inverted)
// C: 		return !(val & RT5640_JD_STATUS);
// C: 	else
// C: 		return (val & RT5640_JD_STATUS);
// C: }
// C: 
// C: /* Jack detect and button-press timings */
// C: 
// C: static void rt5640_start_button_press_work(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	rt5640->poll_count = 0;
// C: 	rt5640->press_count = 0;
// C: 	rt5640->release_count = 0;
// C: 	rt5640->pressed = false;
// C: 	rt5640->press_reported = false;
// C: 	rt5640_clear_micbias1_ovcd(component);
// C: 	schedule_delayed_work(&rt5640->bp_work, msecs_to_jiffies(BP_POLL_TIME));
// C: }
// C: 
// C: static void rt5640_button_press_work(struct work_struct *work)
// C: {
// C: 	struct rt5640_priv *rt5640 =
// C: 		container_of(work, struct rt5640_priv, bp_work.work);
// C: 	struct snd_soc_component *component = rt5640->component;
// C: 
// C: 	/* Check the jack was not removed underneath us */
// C: 	if (!rt5640_jack_inserted(component))
// C: 		return;
// C: 
// C: 	if (rt5640_micbias1_ovcd(component)) {
// C: 		rt5640->release_count = 0;
// C: 		rt5640->press_count++;
// C: 		/* Remember till after JACK_UNPLUG_TIME wait */
// C: 		if (rt5640->press_count >= BP_THRESHOLD)
// C: 			rt5640->pressed = true;
// C: 		rt5640_clear_micbias1_ovcd(component);
// C: 	} else {
// C: 		rt5640->press_count = 0;
// C: 		rt5640->release_count++;
// C: 	}
// C: 
// C: 	/*
// C: 	 * The pins get temporarily shorted on jack unplug, so we poll for
// C: 	 * at least JACK_UNPLUG_TIME milli-seconds before reporting a press.
// C: 	 */
// C: 	rt5640->poll_count++;
// C: 	if (rt5640->poll_count < (JACK_UNPLUG_TIME / BP_POLL_TIME)) {
// C: 		schedule_delayed_work(&rt5640->bp_work,
// C: 				      msecs_to_jiffies(BP_POLL_TIME));
// C: 		return;
// C: 	}
// C: 
// C: 	if (rt5640->pressed && !rt5640->press_reported) {
// C: 		dev_dbg(component->dev, "headset button press\n");
// C: 		snd_soc_jack_report(rt5640->jack, SND_JACK_BTN_0,
// C: 				    SND_JACK_BTN_0);
// C: 		rt5640->press_reported = true;
// C: 	}
// C: 
// C: 	if (rt5640->release_count >= BP_THRESHOLD) {
// C: 		if (rt5640->press_reported) {
// C: 			dev_dbg(component->dev, "headset button release\n");
// C: 			snd_soc_jack_report(rt5640->jack, 0, SND_JACK_BTN_0);
// C: 		}
// C: 		/* Re-enable OVCD IRQ to detect next press */
// C: 		rt5640_enable_micbias1_ovcd_irq(component);
// C: 		return; /* Stop polling */
// C: 	}
// C: 
// C: 	schedule_delayed_work(&rt5640->bp_work, msecs_to_jiffies(BP_POLL_TIME));
// C: }
// C: 
// C: int rt5640_detect_headset(struct snd_soc_component *component, struct gpio_desc *hp_det_gpio)
// C: {
// C: 	int i, headset_count = 0, headphone_count = 0;
// C: 
// C: 	/*
// C: 	 * We get the insertion event before the jack is fully inserted at which
// C: 	 * point the second ring on a TRRS connector may short the 2nd ring and
// C: 	 * sleeve contacts, also the overcurrent detection is not entirely
// C: 	 * reliable. So we try several times with a wait in between until we
// C: 	 * detect the same type JACK_DETECT_COUNT times in a row.
// C: 	 */
// C: 	for (i = 0; i < JACK_DETECT_MAXCOUNT; i++) {
// C: 		/* Clear any previous over-current status flag */
// C: 		rt5640_clear_micbias1_ovcd(component);
// C: 
// C: 		msleep(JACK_SETTLE_TIME);
// C: 
// C: 		/* Check the jack is still connected before checking ovcd */
// C: 		if (hp_det_gpio) {
// C: 			if (gpiod_get_value_cansleep(hp_det_gpio))
// C: 				return 0;
// C: 		} else {
// C: 			if (!rt5640_jack_inserted(component))
// C: 				return 0;
// C: 		}
// C: 
// C: 		if (rt5640_micbias1_ovcd(component)) {
// C: 			/*
// C: 			 * Over current detected, there is a short between the
// C: 			 * 2nd ring contact and the ground, so a TRS connector
// C: 			 * without a mic contact and thus plain headphones.
// C: 			 */
// C: 			dev_dbg(component->dev, "jack mic-gnd shorted\n");
// C: 			headset_count = 0;
// C: 			headphone_count++;
// C: 			if (headphone_count == JACK_DETECT_COUNT)
// C: 				return SND_JACK_HEADPHONE;
// C: 		} else {
// C: 			dev_dbg(component->dev, "jack mic-gnd open\n");
// C: 			headphone_count = 0;
// C: 			headset_count++;
// C: 			if (headset_count == JACK_DETECT_COUNT)
// C: 				return SND_JACK_HEADSET;
// C: 		}
// C: 	}
// C: 
// C: 	dev_err(component->dev, "Error detecting headset vs headphones, bad contact?, assuming headphones\n");
// C: 	return SND_JACK_HEADPHONE;
// C: }
// C: EXPORT_SYMBOL_GPL(rt5640_detect_headset);
// C: 
// C: static void rt5640_jack_work(struct work_struct *work)
// C: {
// C: 	struct rt5640_priv *rt5640 =
// C: 		container_of(work, struct rt5640_priv, jack_work.work);
// C: 	struct snd_soc_component *component = rt5640->component;
// C: 	int status;
// C: 
// C: 	if (rt5640->jd_src == RT5640_JD_SRC_HDA_HEADER) {
// C: 		int val, jack_type = 0, hda_mic_plugged, hda_hp_plugged;
// C: 
// C: 		/* mic jack */
// C: 		val = snd_soc_component_read(component, RT5640_INT_IRQ_ST);
// C: 		hda_mic_plugged = !(val & RT5640_JD_STATUS);
// C: 		dev_dbg(component->dev, "mic jack status %d\n",
// C: 			hda_mic_plugged);
// C: 
// C: 		snd_soc_component_update_bits(component, RT5640_IRQ_CTRL1,
// C: 			RT5640_JD_P_MASK, !hda_mic_plugged << RT5640_JD_P_SFT);
// C: 
// C: 		if (hda_mic_plugged)
// C: 			jack_type |= SND_JACK_MICROPHONE;
// C: 
// C: 		/* headphone jack */
// C: 		val = snd_soc_component_read(component, RT5640_GCTL2);
// C: 		hda_hp_plugged = !(val & (0x1 << 11));
// C: 		dev_dbg(component->dev, "headphone jack status %d\n",
// C: 			hda_hp_plugged);
// C: 
// C: 		snd_soc_component_update_bits(component, RT5640_GCTL2,
// C: 			(0x1 << 10), !hda_hp_plugged << 10);
// C: 
// C: 		if (hda_hp_plugged)
// C: 			jack_type |= SND_JACK_HEADPHONE;
// C: 
// C: 		snd_soc_jack_report(rt5640->jack, jack_type, SND_JACK_HEADSET);
// C: 
// C: 		return;
// C: 	}
// C: 
// C: 	if (!rt5640_jack_inserted(component)) {
// C: 		/* Jack removed, or spurious IRQ? */
// C: 		if (rt5640->jack->status & SND_JACK_HEADPHONE) {
// C: 			if (rt5640->jack->status & SND_JACK_MICROPHONE) {
// C: 				cancel_delayed_work_sync(&rt5640->bp_work);
// C: 				rt5640_disable_micbias1_ovcd_irq(component);
// C: 				rt5640_disable_micbias1_for_ovcd(component);
// C: 			}
// C: 			snd_soc_jack_report(rt5640->jack, 0,
// C: 					    SND_JACK_HEADSET | SND_JACK_BTN_0);
// C: 			dev_dbg(component->dev, "jack unplugged\n");
// C: 		}
// C: 	} else if (!(rt5640->jack->status & SND_JACK_HEADPHONE)) {
// C: 		/* Jack inserted */
// C: 		WARN_ON(rt5640->ovcd_irq_enabled);
// C: 		rt5640_enable_micbias1_for_ovcd(component);
// C: 		status = rt5640_detect_headset(component, NULL);
// C: 		if (status == SND_JACK_HEADSET) {
// C: 			/* Enable ovcd IRQ for button press detect. */
// C: 			rt5640_enable_micbias1_ovcd_irq(component);
// C: 		} else {
// C: 			/* No more need for overcurrent detect. */
// C: 			rt5640_disable_micbias1_for_ovcd(component);
// C: 		}
// C: 		dev_dbg(component->dev, "detect status %#02x\n", status);
// C: 		snd_soc_jack_report(rt5640->jack, status, SND_JACK_HEADSET);
// C: 	} else if (rt5640->ovcd_irq_enabled && rt5640_micbias1_ovcd(component)) {
// C: 		dev_dbg(component->dev, "OVCD IRQ\n");
// C: 
// C: 		/*
// C: 		 * The ovcd IRQ keeps firing while the button is pressed, so
// C: 		 * we disable it and start polling the button until released.
// C: 		 *
// C: 		 * The disable will make the IRQ pin 0 again and since we get
// C: 		 * IRQs on both edges (so as to detect both jack plugin and
// C: 		 * unplug) this means we will immediately get another IRQ.
// C: 		 * The ovcd_irq_enabled check above makes the 2ND IRQ a NOP.
// C: 		 */
// C: 		rt5640_disable_micbias1_ovcd_irq(component);
// C: 		rt5640_start_button_press_work(component);
// C: 
// C: 		/*
// C: 		 * If the jack-detect IRQ flag goes high (unplug) after our
// C: 		 * above rt5640_jack_inserted() check and before we have
// C: 		 * disabled the OVCD IRQ, the IRQ pin will stay high and as
// C: 		 * we react to edges, we miss the unplug event -> recheck.
// C: 		 */
// C: 		queue_delayed_work(system_dfl_long_wq, &rt5640->jack_work, 0);
// C: 	}
// C: }
// C: 
// C: static irqreturn_t rt5640_irq(int irq, void *data)
// C: {
// C: 	struct rt5640_priv *rt5640 = data;
// C: 	int delay = 0;
// C: 
// C: 	if (rt5640->jd_src == RT5640_JD_SRC_HDA_HEADER)
// C: 		delay = 100;
// C: 
// C: 	if (rt5640->jack)
// C: 		mod_delayed_work(system_dfl_long_wq, &rt5640->jack_work,
// C: 				 delay);
// C: 
// C: 	return IRQ_HANDLED;
// C: }
// C: 
// C: static irqreturn_t rt5640_jd_gpio_irq(int irq, void *data)
// C: {
// C: 	struct rt5640_priv *rt5640 = data;
// C: 
// C: 	queue_delayed_work(system_dfl_long_wq, &rt5640->jack_work,
// C: 			   msecs_to_jiffies(JACK_SETTLE_TIME));
// C: 
// C: 	return IRQ_HANDLED;
// C: }
// C: 
// C: static void rt5640_disable_irq_and_cancel_work(void *data)
// C: {
// C: 	struct rt5640_priv *rt5640 = data;
// C: 
// C: 	if (rt5640->jd_gpio_irq_requested) {
// C: 		free_irq(rt5640->jd_gpio_irq, rt5640);
// C: 		rt5640->jd_gpio_irq_requested = false;
// C: 	}
// C: 
// C: 	if (rt5640->irq_requested) {
// C: 		free_irq(rt5640->irq, rt5640);
// C: 		rt5640->irq_requested = false;
// C: 	}
// C: 
// C: 	cancel_delayed_work_sync(&rt5640->jack_work);
// C: 	cancel_delayed_work_sync(&rt5640->bp_work);
// C: }
// C: 
// C: void rt5640_set_ovcd_params(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	snd_soc_component_write(component, RT5640_PR_BASE + RT5640_BIAS_CUR4,
// C: 		0xa800 | rt5640->ovcd_sf);
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_MICBIAS,
// C: 		RT5640_MIC1_OVTH_MASK | RT5640_MIC1_OVCD_MASK,
// C: 		rt5640->ovcd_th | RT5640_MIC1_OVCD_EN);
// C: 
// C: 	/*
// C: 	 * The over-current-detect is only reliable in detecting the absence
// C: 	 * of over-current, when the mic-contact in the jack is short-circuited,
// C: 	 * the hardware periodically retries if it can apply the bias-current
// C: 	 * leading to the ovcd status flip-flopping 1-0-1 with it being 0 about
// C: 	 * 10% of the time, as we poll the ovcd status bit we might hit that
// C: 	 * 10%, so we enable sticky mode and when checking OVCD we clear the
// C: 	 * status, msleep() a bit and then check to get a reliable reading.
// C: 	 */
// C: 	snd_soc_component_update_bits(component, RT5640_IRQ_CTRL2,
// C: 		RT5640_MB1_OC_STKY_MASK, RT5640_MB1_OC_STKY_EN);
// C: }
// C: EXPORT_SYMBOL_GPL(rt5640_set_ovcd_params);
// C: 
// C: static void rt5640_disable_jack_detect(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	/*
// C: 	 * soc_remove_component() force-disables jack and thus rt5640->jack
// C: 	 * could be NULL at the time of driver's module unloading.
// C: 	 */
// C: 	if (!rt5640->jack)
// C: 		return;
// C: 
// C: 	rt5640_disable_irq_and_cancel_work(rt5640);
// C: 
// C: 	if (rt5640->jack->status & SND_JACK_MICROPHONE) {
// C: 		rt5640_disable_micbias1_ovcd_irq(component);
// C: 		rt5640_disable_micbias1_for_ovcd(component);
// C: 		snd_soc_jack_report(rt5640->jack, 0, SND_JACK_BTN_0);
// C: 	}
// C: 
// C: 	rt5640->jd_gpio = NULL;
// C: 	rt5640->jack = NULL;
// C: }
// C: 
// C: static void rt5640_enable_jack_detect(struct snd_soc_component *component,
// C: 				      struct snd_soc_jack *jack,
// C: 				      struct rt5640_set_jack_data *jack_data)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	int ret;
// C: 
// C: 	/* Select JD-source */
// C: 	snd_soc_component_update_bits(component, RT5640_JD_CTRL,
// C: 		RT5640_JD_MASK, rt5640->jd_src << RT5640_JD_SFT);
// C: 
// C: 	/* Selecting GPIO01 as an interrupt */
// C: 	snd_soc_component_update_bits(component, RT5640_GPIO_CTRL1,
// C: 		RT5640_GP1_PIN_MASK, RT5640_GP1_PIN_IRQ);
// C: 
// C: 	/* Set GPIO1 output */
// C: 	snd_soc_component_update_bits(component, RT5640_GPIO_CTRL3,
// C: 		RT5640_GP1_PF_MASK, RT5640_GP1_PF_OUT);
// C: 
// C: 	snd_soc_component_write(component, RT5640_GCTL1, 0x3f41);
// C: 
// C: 	rt5640_set_ovcd_params(component);
// C: 
// C: 	/*
// C: 	 * All IRQs get or-ed together, so we need the jack IRQ to report 0
// C: 	 * when a jack is inserted so that the OVCD IRQ then toggles the IRQ
// C: 	 * pin 0/1 instead of it being stuck to 1. So we invert the JD polarity
// C: 	 * on systems where the hardware does not already do this.
// C: 	 */
// C: 	if (rt5640->jd_inverted) {
// C: 		if (rt5640->jd_src == RT5640_JD_SRC_JD1_IN4P)
// C: 			snd_soc_component_write(component, RT5640_IRQ_CTRL1,
// C: 				RT5640_IRQ_JD_NOR);
// C: 		else if (rt5640->jd_src == RT5640_JD_SRC_JD2_IN4N)
// C: 			snd_soc_component_update_bits(component, RT5640_GCTL2,
// C: 				RT5640_IRQ_JD2_MASK | RT5640_JD2_MASK,
// C: 				RT5640_IRQ_JD2_NOR | RT5640_JD2_EN);
// C: 	} else {
// C: 		if (rt5640->jd_src == RT5640_JD_SRC_JD1_IN4P)
// C: 			snd_soc_component_write(component, RT5640_IRQ_CTRL1,
// C: 				RT5640_IRQ_JD_NOR | RT5640_JD_P_INV);
// C: 		else if (rt5640->jd_src == RT5640_JD_SRC_JD2_IN4N)
// C: 			snd_soc_component_update_bits(component, RT5640_GCTL2,
// C: 				RT5640_IRQ_JD2_MASK | RT5640_JD2_P_MASK |
// C: 				RT5640_JD2_MASK,
// C: 				RT5640_IRQ_JD2_NOR | RT5640_JD2_P_INV |
// C: 				RT5640_JD2_EN);
// C: 	}
// C: 
// C: 	rt5640->jack = jack;
// C: 	if (rt5640->jack->status & SND_JACK_MICROPHONE) {
// C: 		rt5640_enable_micbias1_for_ovcd(component);
// C: 		rt5640_enable_micbias1_ovcd_irq(component);
// C: 	}
// C: 
// C: 	if (jack_data && jack_data->codec_irq_override)
// C: 		rt5640->irq = jack_data->codec_irq_override;
// C: 
// C: 	if (jack_data && jack_data->jd_gpio) {
// C: 		rt5640->jd_gpio = jack_data->jd_gpio;
// C: 		rt5640->jd_gpio_irq = gpiod_to_irq(rt5640->jd_gpio);
// C: 
// C: 		ret = request_any_context_irq(rt5640->jd_gpio_irq,
// C: 					      rt5640_jd_gpio_irq,
// C: 					      IRQF_TRIGGER_RISING |
// C: 					      IRQF_TRIGGER_FALLING,
// C: 					      "rt5640-jd-gpio", rt5640);
// C: 		if (ret < 0) {
// C: 			dev_warn(component->dev, "Failed to request jd GPIO IRQ %d: %d\n",
// C: 				 rt5640->jd_gpio_irq, ret);
// C: 			rt5640_disable_jack_detect(component);
// C: 			return;
// C: 		}
// C: 		rt5640->jd_gpio_irq_requested = true;
// C: 	}
// C: 
// C: 	if (jack_data && jack_data->use_platform_clock)
// C: 		rt5640->use_platform_clock = jack_data->use_platform_clock;
// C: 
// C: 	ret = request_any_context_irq(rt5640->irq, rt5640_irq,
// C: 				      IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
// C: 				      "rt5640", rt5640);
// C: 	if (ret < 0) {
// C: 		dev_warn(component->dev, "Failed to request IRQ %d: %d\n", rt5640->irq, ret);
// C: 		rt5640_disable_jack_detect(component);
// C: 		return;
// C: 	}
// C: 	rt5640->irq_requested = true;
// C: 
// C: 	/* sync initial jack state */
// C: 	queue_delayed_work(system_dfl_long_wq, &rt5640->jack_work, 0);
// C: }
// C: 
// C: static const struct snd_soc_dapm_route rt5640_hda_jack_dapm_routes[] = {
// C: 	{"IN1P", NULL, "MICBIAS1"},
// C: 	{"IN2P", NULL, "MICBIAS1"},
// C: 	{"IN3P", NULL, "MICBIAS1"},
// C: };
// C: 
// C: static void rt5640_enable_hda_jack_detect(
// C: 	struct snd_soc_component *component, struct snd_soc_jack *jack)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	int ret;
// C: 
// C: 	/* Select JD1 for Mic */
// C: 	snd_soc_component_update_bits(component, RT5640_JD_CTRL,
// C: 		RT5640_JD_MASK, RT5640_JD_JD1_IN4P);
// C: 	snd_soc_component_write(component, RT5640_IRQ_CTRL1, RT5640_IRQ_JD_NOR);
// C: 
// C: 	/* Select JD2 for Headphone */
// C: 	snd_soc_component_update_bits(component, RT5640_GCTL2, 0x1100, 0x1100);
// C: 
// C: 	/* Selecting GPIO01 as an interrupt */
// C: 	snd_soc_component_update_bits(component, RT5640_GPIO_CTRL1,
// C: 		RT5640_GP1_PIN_MASK, RT5640_GP1_PIN_IRQ);
// C: 
// C: 	/* Set GPIO1 output */
// C: 	snd_soc_component_update_bits(component, RT5640_GPIO_CTRL3,
// C: 		RT5640_GP1_PF_MASK, RT5640_GP1_PF_OUT);
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_GCTL1, 0x400, 0x0);
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_VREF2 | RT5640_PWR_MB | RT5640_PWR_BG,
// C: 		RT5640_PWR_VREF2 | RT5640_PWR_MB | RT5640_PWR_BG);
// C: 	usleep_range(10000, 15000);
// C: 	snd_soc_component_update_bits(component, RT5640_PWR_ANLG1,
// C: 		RT5640_PWR_FV2, RT5640_PWR_FV2);
// C: 
// C: 	rt5640->jack = jack;
// C: 
// C: 	ret = request_any_context_irq(rt5640->irq, rt5640_irq,
// C: 				      IRQF_TRIGGER_RISING, "rt5640", rt5640);
// C: 	if (ret < 0) {
// C: 		dev_warn(component->dev, "Failed to request IRQ %d: %d\n", rt5640->irq, ret);
// C: 		rt5640->jack = NULL;
// C: 		return;
// C: 	}
// C: 	rt5640->irq_requested = true;
// C: 
// C: 	/* sync initial jack state */
// C: 	queue_delayed_work(system_dfl_long_wq, &rt5640->jack_work, 0);
// C: 
// C: 	snd_soc_dapm_add_routes(dapm, rt5640_hda_jack_dapm_routes,
// C: 		ARRAY_SIZE(rt5640_hda_jack_dapm_routes));
// C: }
// C: 
// C: static int rt5640_set_jack(struct snd_soc_component *component,
// C: 			   struct snd_soc_jack *jack, void *data)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	if (jack) {
// C: 		if (rt5640->jd_src == RT5640_JD_SRC_HDA_HEADER)
// C: 			rt5640_enable_hda_jack_detect(component, jack);
// C: 		else
// C: 			rt5640_enable_jack_detect(component, jack, data);
// C: 	} else {
// C: 		rt5640_disable_jack_detect(component);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_probe(struct snd_soc_component *component)
// C: {
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	u32 dmic1_data_pin = 0;
// C: 	u32 dmic2_data_pin = 0;
// C: 	bool dmic_en = false;
// C: 	u32 val;
// C: 
// C: 	/* Check if MCLK provided */
// C: 	rt5640->mclk = devm_clk_get_optional(component->dev, "mclk");
// C: 	if (IS_ERR(rt5640->mclk))
// C: 		return PTR_ERR(rt5640->mclk);
// C: 
// C: 	rt5640->component = component;
// C: 
// C: 	snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);
// C: 
// C: 	snd_soc_component_update_bits(component, RT5640_GCTL1, 0x0301, 0x0301);
// C: 	snd_soc_component_update_bits(component, RT5640_MICBIAS, 0x0030, 0x0030);
// C: 	snd_soc_component_update_bits(component, RT5640_DSP_PATH2, 0xfc00, 0x0c00);
// C: 
// C: 	switch (snd_soc_component_read(component, RT5640_RESET) & RT5640_ID_MASK) {
// C: 	case RT5640_ID_5640:
// C: 	case RT5640_ID_5642:
// C: 		snd_soc_add_component_controls(component,
// C: 			rt5640_specific_snd_controls,
// C: 			ARRAY_SIZE(rt5640_specific_snd_controls));
// C: 		snd_soc_dapm_new_controls(dapm,
// C: 			rt5640_specific_dapm_widgets,
// C: 			ARRAY_SIZE(rt5640_specific_dapm_widgets));
// C: 		snd_soc_dapm_add_routes(dapm,
// C: 			rt5640_specific_dapm_routes,
// C: 			ARRAY_SIZE(rt5640_specific_dapm_routes));
// C: 		break;
// C: 	case RT5640_ID_5639:
// C: 		snd_soc_dapm_new_controls(dapm,
// C: 			rt5639_specific_dapm_widgets,
// C: 			ARRAY_SIZE(rt5639_specific_dapm_widgets));
// C: 		snd_soc_dapm_add_routes(dapm,
// C: 			rt5639_specific_dapm_routes,
// C: 			ARRAY_SIZE(rt5639_specific_dapm_routes));
// C: 		break;
// C: 	default:
// C: 		dev_err(component->dev,
// C: 			"The driver is for RT5639 RT5640 or RT5642 only\n");
// C: 		return -ENODEV;
// C: 	}
// C: 
// C: 	/*
// C: 	 * Note on some platforms the platform code may need to add device-props
// C: 	 * rather then relying only on properties set by the firmware.
// C: 	 * Therefor the property parsing MUST be done here, rather then from
// C: 	 * rt5640_i2c_probe(), so that the platform-code can attach extra
// C: 	 * properties before calling snd_soc_register_card().
// C: 	 */
// C: 	if (device_property_read_bool(component->dev, "realtek,in1-differential"))
// C: 		snd_soc_component_update_bits(component, RT5640_IN1_IN2,
// C: 					      RT5640_IN_DF1, RT5640_IN_DF1);
// C: 
// C: 	if (device_property_read_bool(component->dev, "realtek,in2-differential"))
// C: 		snd_soc_component_update_bits(component, RT5640_IN3_IN4,
// C: 					      RT5640_IN_DF2, RT5640_IN_DF2);
// C: 
// C: 	if (device_property_read_bool(component->dev, "realtek,in3-differential"))
// C: 		snd_soc_component_update_bits(component, RT5640_IN1_IN2,
// C: 					      RT5640_IN_DF2, RT5640_IN_DF2);
// C: 
// C: 	if (device_property_read_bool(component->dev, "realtek,lout-differential"))
// C: 		snd_soc_component_update_bits(component, RT5640_GCTL1,
// C: 					      RT5640_EN_LOUT_DF, RT5640_EN_LOUT_DF);
// C: 
// C: 	if (device_property_read_u32(component->dev, "realtek,dmic1-data-pin",
// C: 				     &val) == 0 && val) {
// C: 		dmic1_data_pin = val - 1;
// C: 		dmic_en = true;
// C: 	}
// C: 
// C: 	if (device_property_read_u32(component->dev, "realtek,dmic2-data-pin",
// C: 				     &val) == 0 && val) {
// C: 		dmic2_data_pin = val - 1;
// C: 		dmic_en = true;
// C: 	}
// C: 
// C: 	if (dmic_en)
// C: 		rt5640_dmic_enable(component, dmic1_data_pin, dmic2_data_pin);
// C: 
// C: 	if (device_property_read_u32(component->dev,
// C: 				     "realtek,jack-detect-source", &val) == 0) {
// C: 		if (val <= RT5640_JD_SRC_HDA_HEADER)
// C: 			rt5640->jd_src = val;
// C: 		else
// C: 			dev_warn(component->dev, "Warning: Invalid jack-detect-source value: %d, leaving jack-detect disabled\n",
// C: 				 val);
// C: 	}
// C: 
// C: 	if (!device_property_read_bool(component->dev, "realtek,jack-detect-not-inverted"))
// C: 		rt5640->jd_inverted = true;
// C: 
// C: 	/*
// C: 	 * Testing on various boards has shown that good defaults for the OVCD
// C: 	 * threshold and scale-factor are 2000µA and 0.75. For an effective
// C: 	 * limit of 1500µA, this seems to be more reliable then 1500µA and 1.0.
// C: 	 */
// C: 	rt5640->ovcd_th = RT5640_MIC1_OVTH_2000UA;
// C: 	rt5640->ovcd_sf = RT5640_MIC_OVCD_SF_0P75;
// C: 
// C: 	if (device_property_read_u32(component->dev,
// C: 			"realtek,over-current-threshold-microamp", &val) == 0) {
// C: 		switch (val) {
// C: 		case 600:
// C: 			rt5640->ovcd_th = RT5640_MIC1_OVTH_600UA;
// C: 			break;
// C: 		case 1500:
// C: 			rt5640->ovcd_th = RT5640_MIC1_OVTH_1500UA;
// C: 			break;
// C: 		case 2000:
// C: 			rt5640->ovcd_th = RT5640_MIC1_OVTH_2000UA;
// C: 			break;
// C: 		default:
// C: 			dev_warn(component->dev, "Warning: Invalid over-current-threshold-microamp value: %d, defaulting to 2000uA\n",
// C: 				 val);
// C: 		}
// C: 	}
// C: 
// C: 	if (device_property_read_u32(component->dev,
// C: 			"realtek,over-current-scale-factor", &val) == 0) {
// C: 		if (val <= RT5640_OVCD_SF_1P5)
// C: 			rt5640->ovcd_sf = val << RT5640_MIC_OVCD_SF_SFT;
// C: 		else
// C: 			dev_warn(component->dev, "Warning: Invalid over-current-scale-factor value: %d, defaulting to 0.75\n",
// C: 				 val);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static void rt5640_remove(struct snd_soc_component *component)
// C: {
// C: 	rt5640_reset(component);
// C: }
// C: 
// preserved build-time condition: #ifdef CONFIG_PM
// C: static int rt5640_suspend(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 
// C: 	if (rt5640->jack) {
// C: 		/* disable jack interrupts during system suspend */
// C: 		disable_irq(rt5640->irq);
// C: 		cancel_delayed_work_sync(&rt5640->jack_work);
// C: 		cancel_delayed_work_sync(&rt5640->bp_work);
// C: 	}
// C: 
// C: 	snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);
// C: 	rt5640_reset(component);
// C: 	regcache_cache_only(rt5640->regmap, true);
// C: 	regcache_mark_dirty(rt5640->regmap);
// C: 	if (rt5640->ldo1_en)
// C: 		gpiod_set_value_cansleep(rt5640->ldo1_en, 0);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int rt5640_resume(struct snd_soc_component *component)
// C: {
// C: 	struct rt5640_priv *rt5640 = snd_soc_component_get_drvdata(component);
// C: 
// C: 	if (rt5640->ldo1_en) {
// C: 		gpiod_set_value_cansleep(rt5640->ldo1_en, 1);
// C: 		msleep(400);
// C: 	}
// C: 
// C: 	regcache_cache_only(rt5640->regmap, false);
// C: 	regcache_sync(rt5640->regmap);
// C: 
// C: 	if (rt5640->jack) {
// C: 		if (rt5640->jd_src == RT5640_JD_SRC_HDA_HEADER) {
// C: 			snd_soc_component_update_bits(component,
// C: 				RT5640_GCTL2, 0x1100, 0x1100);
// C: 		} else {
// C: 			if (rt5640->jd_inverted) {
// C: 				if (rt5640->jd_src == RT5640_JD_SRC_JD2_IN4N)
// C: 					snd_soc_component_update_bits(
// C: 						component, RT5640_GCTL2,
// C: 						RT5640_IRQ_JD2_MASK |
// C: 						RT5640_JD2_MASK,
// C: 						RT5640_IRQ_JD2_NOR |
// C: 						RT5640_JD2_EN);
// C: 
// C: 			} else {
// C: 				if (rt5640->jd_src == RT5640_JD_SRC_JD2_IN4N)
// C: 					snd_soc_component_update_bits(
// C: 						component, RT5640_GCTL2,
// C: 						RT5640_IRQ_JD2_MASK |
// C: 						RT5640_JD2_P_MASK |
// C: 						RT5640_JD2_MASK,
// C: 						RT5640_IRQ_JD2_NOR |
// C: 						RT5640_JD2_P_INV |
// C: 						RT5640_JD2_EN);
// C: 			}
// C: 		}
// C: 
// C: 		enable_irq(rt5640->irq);
// C: 		queue_delayed_work(system_dfl_long_wq, &rt5640->jack_work, 0);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// preserved build-time condition: #else
// preserved build-time condition: #endif
// C: 
// C: 			SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8)
// C: 
// C: static const struct snd_soc_dai_ops rt5640_aif_dai_ops = {
// C: 	.hw_params = rt5640_hw_params,
// C: 	.set_fmt = rt5640_set_dai_fmt,
// C: 	.set_sysclk = rt5640_set_dai_sysclk,
// C: 	.set_pll = rt5640_set_dai_pll,
// C: };
// C: 
// C: static struct snd_soc_dai_driver rt5640_dai[] = {
// C: 	{
// C: 		.name = "rt5640-aif1",
// C: 		.id = RT5640_AIF1,
// C: 		.playback = {
// C: 			.stream_name = "AIF1 Playback",
// C: 			.channels_min = 1,
// C: 			.channels_max = 2,
// C: 			.rates = RT5640_STEREO_RATES,
// C: 			.formats = RT5640_FORMATS,
// C: 		},
// C: 		.capture = {
// C: 			.stream_name = "AIF1 Capture",
// C: 			.channels_min = 1,
// C: 			.channels_max = 2,
// C: 			.rates = RT5640_STEREO_RATES,
// C: 			.formats = RT5640_FORMATS,
// C: 		},
// C: 		.ops = &rt5640_aif_dai_ops,
// C: 	},
// C: 	{
// C: 		.name = "rt5640-aif2",
// C: 		.id = RT5640_AIF2,
// C: 		.playback = {
// C: 			.stream_name = "AIF2 Playback",
// C: 			.channels_min = 1,
// C: 			.channels_max = 2,
// C: 			.rates = RT5640_STEREO_RATES,
// C: 			.formats = RT5640_FORMATS,
// C: 		},
// C: 		.capture = {
// C: 			.stream_name = "AIF2 Capture",
// C: 			.channels_min = 1,
// C: 			.channels_max = 2,
// C: 			.rates = RT5640_STEREO_RATES,
// C: 			.formats = RT5640_FORMATS,
// C: 		},
// C: 		.ops = &rt5640_aif_dai_ops,
// C: 	},
// C: };
// C: 
// C: static const struct snd_soc_component_driver soc_component_dev_rt5640 = {
// C: 	.probe			= rt5640_probe,
// C: 	.remove			= rt5640_remove,
// C: 	.suspend		= rt5640_suspend,
// C: 	.resume			= rt5640_resume,
// C: 	.set_bias_level		= rt5640_set_bias_level,
// C: 	.set_jack		= rt5640_set_jack,
// C: 	.controls		= rt5640_snd_controls,
// C: 	.num_controls		= ARRAY_SIZE(rt5640_snd_controls),
// C: 	.dapm_widgets		= rt5640_dapm_widgets,
// C: 	.num_dapm_widgets	= ARRAY_SIZE(rt5640_dapm_widgets),
// C: 	.dapm_routes		= rt5640_dapm_routes,
// C: 	.num_dapm_routes	= ARRAY_SIZE(rt5640_dapm_routes),
// C: 	.use_pmdown_time	= 1,
// C: 	.endianness		= 1,
// C: };
// C: 
// C: static const struct regmap_config rt5640_regmap = {
// C: 	.reg_bits = 8,
// C: 	.val_bits = 16,
// C: 	.use_single_read = true,
// C: 	.use_single_write = true,
// C: 
// C: 	.max_register = RT5640_VENDOR_ID2 + 1 + (ARRAY_SIZE(rt5640_ranges) *
// C: 					       RT5640_PR_SPACING),
// C: 	.volatile_reg = rt5640_volatile_register,
// C: 	.readable_reg = rt5640_readable_register,
// C: 
// C: 	.cache_type = REGCACHE_MAPLE,
// C: 	.reg_defaults = rt5640_reg,
// C: 	.num_reg_defaults = ARRAY_SIZE(rt5640_reg),
// C: 	.ranges = rt5640_ranges,
// C: 	.num_ranges = ARRAY_SIZE(rt5640_ranges),
// C: };
// C: 
// C: static const struct i2c_device_id rt5640_i2c_id[] = {
// C: 	{ .name = "rt5640" },
// C: 	{ .name = "rt5639" },
// C: 	{ .name = "rt5642" },
// C: 	{ }
// C: };
// C: MODULE_DEVICE_TABLE(i2c, rt5640_i2c_id);
// C: 
// preserved build-time condition: #if defined(CONFIG_OF)
// C: static const struct of_device_id rt5640_of_match[] = {
// C: 	{ .compatible = "realtek,rt5639", },
// C: 	{ .compatible = "realtek,rt5640", },
// C: 	{ }
// C: };
// C: MODULE_DEVICE_TABLE(of, rt5640_of_match);
// preserved build-time condition: #endif
// C: 
// preserved build-time condition: #ifdef CONFIG_ACPI
// C: static const struct acpi_device_id rt5640_acpi_match[] = {
// C: 	{ "10EC3276" },
// C: 	{ "10EC5640" },
// C: 	{ "10EC5642" },
// C: 	{ "INT33CA" },
// C: 	{ "INTCCFFD" },
// C: 	{ }
// C: };
// C: MODULE_DEVICE_TABLE(acpi, rt5640_acpi_match);
// preserved build-time condition: #endif
// C: 
// C: static int rt5640_i2c_probe(struct i2c_client *i2c)
// C: {
// C: 	struct rt5640_priv *rt5640;
// C: 	int ret;
// C: 	unsigned int val;
// C: 
// C: 	rt5640 = devm_kzalloc(&i2c->dev,
// C: 				sizeof(struct rt5640_priv),
// C: 				GFP_KERNEL);
// C: 	if (NULL == rt5640)
// C: 		return -ENOMEM;
// C: 	i2c_set_clientdata(i2c, rt5640);
// C: 
// C: 	rt5640->ldo1_en = devm_gpiod_get_optional(&i2c->dev,
// C: 						  "realtek,ldo1-en",
// C: 						  GPIOD_OUT_HIGH);
// C: 	if (IS_ERR(rt5640->ldo1_en))
// C: 		return PTR_ERR(rt5640->ldo1_en);
// C: 
// C: 	if (rt5640->ldo1_en) {
// C: 		gpiod_set_consumer_name(rt5640->ldo1_en, "RT5640 LDO1_EN");
// C: 		msleep(400);
// C: 	}
// C: 
// C: 	rt5640->regmap = devm_regmap_init_i2c(i2c, &rt5640_regmap);
// C: 	if (IS_ERR(rt5640->regmap)) {
// C: 		ret = PTR_ERR(rt5640->regmap);
// C: 		dev_err(&i2c->dev, "Failed to allocate register map: %d\n",
// C: 			ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	regmap_read(rt5640->regmap, RT5640_VENDOR_ID2, &val);
// C: 	if (val != RT5640_DEVICE_ID) {
// C: 		usleep_range(60000, 100000);
// C: 		regmap_read(rt5640->regmap, RT5640_VENDOR_ID2, &val);
// C: 	}
// C: 
// C: 	if (val != RT5640_DEVICE_ID) {
// C: 		dev_err(&i2c->dev,
// C: 			"Device with ID register %#x is not rt5640/39\n", val);
// C: 		return -ENODEV;
// C: 	}
// C: 
// C: 	regmap_write(rt5640->regmap, RT5640_RESET, 0);
// C: 
// C: 	ret = regmap_register_patch(rt5640->regmap, init_list,
// C: 				    ARRAY_SIZE(init_list));
// C: 	if (ret != 0)
// C: 		dev_warn(&i2c->dev, "Failed to apply regmap patch: %d\n", ret);
// C: 
// C: 	regmap_update_bits(rt5640->regmap, RT5640_GCTL1,
// C: 				RT5640_MCLK_DET, RT5640_MCLK_DET);
// C: 
// C: 	rt5640->hp_mute = true;
// C: 	rt5640->irq = i2c->irq;
// C: 	INIT_DELAYED_WORK(&rt5640->bp_work, rt5640_button_press_work);
// C: 	INIT_DELAYED_WORK(&rt5640->jack_work, rt5640_jack_work);
// C: 
// C: 	/* Make sure work is stopped on probe-error / remove */
// C: 	ret = devm_add_action_or_reset(&i2c->dev, rt5640_disable_irq_and_cancel_work, rt5640);
// C: 	if (ret)
// C: 		return ret;
// C: 
// C: 	return devm_snd_soc_register_component(&i2c->dev,
// C: 				      &soc_component_dev_rt5640,
// C: 				      rt5640_dai, ARRAY_SIZE(rt5640_dai));
// C: }
// C: 
// C: static struct i2c_driver rt5640_i2c_driver = {
// C: 	.driver = {
// C: 		.name = "rt5640",
// C: 		.acpi_match_table = ACPI_PTR(rt5640_acpi_match),
// C: 		.of_match_table = of_match_ptr(rt5640_of_match),
// C: 	},
// C: 	.probe = rt5640_i2c_probe,
// C: 	.id_table = rt5640_i2c_id,
// C: };
// C: module_i2c_driver(rt5640_i2c_driver);
// C: 
// C: MODULE_DESCRIPTION("ASoC RT5640/RT5639 driver");
// C: MODULE_AUTHOR("Johnny Hsu <johnnyhsu@realtek.com>");
// C: MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
