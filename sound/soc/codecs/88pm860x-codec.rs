// SPDX-License-Identifier: GPL-2.0-only
/*
 * 88pm860x-codec.rs -- 88PM860x ALSA SoC Audio Driver
 *
 * Copyright 2010 Marvell International Ltd.
 * Author: Haojian Zhuang <haojian.zhuang@marvell.com>
 *
 * Source-level Rust translation of the isolated implementation source.
 * External Linux/ALSA definitions and macros are referenced, not reimplemented.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, unused_variables, unused_mut)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const MAX_NAME_LEN: usize = 20;
const REG_CACHE_SIZE: c_uint = 0x40;
const REG_CACHE_BASE: c_uint = 0xb0;
const REG_STATUS_1: c_uint = 0x01;
const MIC_STATUS: c_int = 1 << 7;
const HOOK_STATUS: c_int = 1 << 6;
const HEADSET_STATUS: c_int = 1 << 5;
const REG_MIC_DET: c_uint = 0x37;
const CONTINUOUS_POLLING: c_int = 3 << 1;
const EN_MIC_DET: c_int = 1 << 0;
const MICDET_MASK: c_int = 0x07;
const REG_HS_DET: c_uint = 0x38;
const EN_HS_DET: c_int = 1 << 0;
const REG_MISC2: c_uint = 0x42;
const AUDIO_PLL: c_int = 1 << 5;
const AUDIO_SECTION_RESET: c_int = 1 << 4;
const AUDIO_SECTION_ON: c_int = 1 << 3;
const PCM_INF2_BCLK: c_uint = 1 << 6;
const PCM_INF2_FS: c_uint = 1 << 5;
const PCM_INF2_MASTER: c_uint = 1 << 4;
const PCM_INF2_18WL: c_uint = 1 << 3;
const PCM_GENERAL_I2S: c_uint = 0;
const PCM_EXACT_I2S: c_uint = 1;
const PCM_LEFT_I2S: c_uint = 2;
const PCM_RIGHT_I2S: c_uint = 3;
const PCM_SHORT_FS: c_uint = 4;
const PCM_LONG_FS: c_uint = 5;
const PCM_MODE_MASK: c_uint = 7;
const I2S_EQU_BYP: c_int = 1 << 6;
const DAC_MUTE: c_uint = 1 << 7;
const MUTE_LEFT: c_uint = 1 << 6;
const MUTE_RIGHT: c_uint = 1 << 2;
const REG_ADC_ANA_1: c_uint = 0xd0;
const MIC1BIAS_MASK: c_int = 0x60;
const REG_EAR2: c_uint = 0xda;
const RSYNC_CHANGE: c_uint = 1 << 2;
const REG_SUPPLIES2: c_uint = 0xdc;
const LDO15_READY: c_int = 1 << 4;
const LDO15_EN: c_uint = 1 << 3;
const CPUMP_READY: c_int = 1 << 2;
const CPUMP_EN: c_uint = 1 << 1;
const AUDIO_EN: c_uint = 1 << 0;
const SUPPLY_MASK: c_uint = LDO15_EN | CPUMP_EN | AUDIO_EN;
const ADC_MOD_RIGHT: c_int = 1 << 1;
const ADC_MOD_LEFT: c_int = 1 << 0;
const ADC_LEFT: c_int = 1 << 5;
const ADC_RIGHT: c_int = 1 << 4;
const DAC_LEFT: c_uint = 1 << 5;
const DAC_RIGHT: c_uint = 1 << 4;
const MODULATOR: c_uint = 1 << 3;
const REG_SHORTS: c_uint = 0xeb;
const CLR_SHORT_LO2: c_int = 1 << 7;
const SHORT_LO2: c_int = 1 << 6;
const CLR_SHORT_LO1: c_int = 1 << 5;
const SHORT_LO1: c_int = 1 << 4;
const CLR_SHORT_HS2: c_int = 1 << 3;
const SHORT_HS2: c_int = 1 << 2;
const CLR_SHORT_HS1: c_int = 1 << 1;
const SHORT_HS1: c_int = 1 << 0;

macro_rules! PM860X_DAPM_OUTPUT { ($wname:expr, $wevent:expr) => { SND_SOC_DAPM_PGA_E!($wname, SND_SOC_NOPM, 0, 0, ptr::null(), 0, $wevent, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD) }; }

#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct i2c_client { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct pm860x_chip { pub id: c_int, pub client: *mut i2c_client, pub companion: *mut i2c_client, pub regmap: *mut regmap, pub regmap_companion: *mut regmap, pub irq_base: c_int }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_int; 128] }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub rreg: c_uint, pub shift: c_uint, pub max: c_int }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct resource { pub start: c_int, pub name: *const c_char }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_def { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_enum { _private: [u8; 0] }
type irqreturn_t = c_int;
type snd_soc_bias_level = c_uint;

#[repr(C)] pub struct pm860x_det { hp_jack: *mut snd_soc_jack, mic_jack: *mut snd_soc_jack, hp_det: c_int, mic_det: c_int, hook_det: c_int, hs_shrt: c_int, lo_shrt: c_int }
#[repr(C)] pub struct pm860x_priv { sysclk: c_uint, pcmclk: c_uint, dir: c_uint, filter: c_uint, component: *mut snd_soc_component, i2c: *mut i2c_client, regmap: *mut regmap, chip: *mut pm860x_chip, det: pm860x_det, irq: [c_int; 4], name: [[u8; MAX_NAME_LEN]; 4] }

DECLARE_TLV_DB_SCALE!(dpga_tlv, -9450, 150, 1);
DECLARE_TLV_DB_SCALE!(adc_tlv, -900, 300, 0);
DECLARE_TLV_DB_RANGE!(mic_tlv, 0, 0, TLV_DB_SCALE_ITEM!(-2300, 0, 0), 1, 1, TLV_DB_SCALE_ITEM!(-1700, 0, 0), 2, 2, TLV_DB_SCALE_ITEM!(-1350, 0, 0), 3, 3, TLV_DB_SCALE_ITEM!(-1100, 0, 0), 4, 7, TLV_DB_SCALE_ITEM!(-900, 300, 0));
DECLARE_TLV_DB_RANGE!(aux_tlv, 0, 2, TLV_DB_SCALE_ITEM!(0, 0, 0), 3, 7, TLV_DB_SCALE_ITEM!(-600, 600, 0));
DECLARE_TLV_DB_RANGE!(out_tlv, 0, 3, TLV_DB_SCALE_ITEM!(-1600, 300, 1), 4, 4, TLV_DB_SCALE_ITEM!(-520, 0, 0), 5, 5, TLV_DB_SCALE_ITEM!(-330, 0, 0), 6, 7, TLV_DB_SCALE_ITEM!(-220, 220, 0));
DECLARE_TLV_DB_RANGE!(st_tlv, 0, 1, TLV_DB_SCALE_ITEM!(-12041, 602, 0), 2, 3, TLV_DB_SCALE_ITEM!(-11087, 250, 0), 4, 5, TLV_DB_SCALE_ITEM!(-10643, 158, 0), 6, 7, TLV_DB_SCALE_ITEM!(-10351, 116, 0), 8, 9, TLV_DB_SCALE_ITEM!(-10133, 92, 0), 10, 13, TLV_DB_SCALE_ITEM!(-9958, 70, 0), 14, 17, TLV_DB_SCALE_ITEM!(-9689, 53, 0), 18, 271, TLV_DB_SCALE_ITEM!(-9484, 37, 0));

/* Sidetone Gain = M * 2^(-5-N) */
#[repr(C)] #[derive(Copy, Clone)] struct st_gain { db: c_int, m: c_uint, n: c_uint }
/* {0, 0, 0, -6, 0, 6, 12, 18}dB */
static const DECLARE_TLV_DB_RANGE(aux_tlv,
	0, 2, TLV_DB_SCALE_ITEM(0, 0, 0),
	3, 7, TLV_DB_SCALE_ITEM(-600, 600, 0)
);

/* {-16, -13, -10, -7, -5.2, -3,3, -2.2, 0}dB, mute instead of -16dB */
static const DECLARE_TLV_DB_RANGE(out_tlv,
	0, 3, TLV_DB_SCALE_ITEM(-1600, 300, 1),
	4, 4, TLV_DB_SCALE_ITEM(-520, 0, 0),
	5, 5, TLV_DB_SCALE_ITEM(-330, 0, 0),
	6, 7, TLV_DB_SCALE_ITEM(-220, 220, 0)
);

static const DECLARE_TLV_DB_RANGE(st_tlv,
	0, 1, TLV_DB_SCALE_ITEM(-12041, 602, 0),
	2, 3, TLV_DB_SCALE_ITEM(-11087, 250, 0),
	4, 5, TLV_DB_SCALE_ITEM(-10643, 158, 0),
	6, 7, TLV_DB_SCALE_ITEM(-10351, 116, 0),
	8, 9, TLV_DB_SCALE_ITEM(-10133, 92, 0),
	10, 13, TLV_DB_SCALE_ITEM(-9958, 70, 0),
	14, 17, TLV_DB_SCALE_ITEM(-9689, 53, 0),
	18, 271, TLV_DB_SCALE_ITEM(-9484, 37, 0)
);

/* Sidetone Gain = M * 2^(-5-N) */
struct st_gain {
	unsigned int	db;
	unsigned int	m;
	unsigned int	n;
];

static mut st_table: [st_gain; 272] = {
	st_gain { db: -12041, m: 1, n: 15 }, st_gain { db: -11439, m: 1, n: 14 }, st_gain { db: -11087, m: 3, n: 15 }, st_gain { db: -10837, m: 1, n: 13 },
	st_gain { db: -10643, m: 5, n: 15 }, st_gain { db: -10485, m: 3, n: 14 }, st_gain { db: -10351, m: 7, n: 15 }, st_gain { db: -10235, m: 1, n: 12 },
	st_gain { db: -10133, m: 9, n: 15 }, st_gain { db: -10041, m: 5, n: 14 }, st_gain { db: -9958, m: 11, n: 15 }, st_gain { db: -9883, m: 3, n: 13 },
	st_gain { db: -9813, m: 13, n: 15 }, st_gain { db: -9749, m: 7, n: 14 }, st_gain { db: -9689, m: 15, n: 15 }, st_gain { db: -9633, m: 1, n: 11 },
	st_gain { db: -9580, m: 17, n: 15 }, st_gain { db: -9531, m: 9, n: 14 }, st_gain { db: -9484, m: 19, n: 15 }, st_gain { db: -9439, m: 5, n: 13 },
	st_gain { db: -9397, m: 21, n: 15 }, st_gain { db: -9356, m: 11, n: 14 }, st_gain { db: -9318, m: 23, n: 15 }, st_gain { db: -9281, m: 3, n: 12 },
	st_gain { db: -9245, m: 25, n: 15 }, st_gain { db: -9211, m: 13, n: 14 }, st_gain { db: -9178, m: 27, n: 15 }, st_gain { db: -9147, m: 7, n: 13 },
	st_gain { db: -9116, m: 29, n: 15 }, st_gain { db: -9087, m: 15, n: 14 }, st_gain { db: -9058, m: 31, n: 15 }, st_gain { db: -9031, m: 1, n: 10 },
	st_gain { db: -8978, m: 17, n: 14 }, st_gain { db: -8929, m: 9, n: 13 }, st_gain { db: -8882, m: 19, n: 14 }, st_gain { db: -8837, m: 5, n: 12 },
	st_gain { db: -8795, m: 21, n: 14 }, st_gain { db: -8754, m: 11, n: 13 }, st_gain { db: -8716, m: 23, n: 14 }, st_gain { db: -8679, m: 3, n: 11 },
	st_gain { db: -8643, m: 25, n: 14 }, st_gain { db: -8609, m: 13, n: 13 }, st_gain { db: -8576, m: 27, n: 14 }, st_gain { db: -8545, m: 7, n: 12 },
	st_gain { db: -8514, m: 29, n: 14 }, st_gain { db: -8485, m: 15, n: 13 }, st_gain { db: -8456, m: 31, n: 14 }, st_gain { db: -8429, m: 1, n: 9 },
	st_gain { db: -8376, m: 17, n: 13 }, st_gain { db: -8327, m: 9, n: 12 }, st_gain { db: -8280, m: 19, n: 13 }, st_gain { db: -8235, m: 5, n: 11 },
	st_gain { db: -8193, m: 21, n: 13 }, st_gain { db: -8152, m: 11, n: 12 }, st_gain { db: -8114, m: 23, n: 13 }, st_gain { db: -8077, m: 3, n: 10 },
	st_gain { db: -8041, m: 25, n: 13 }, st_gain { db: -8007, m: 13, n: 12 }, st_gain { db: -7974, m: 27, n: 13 }, st_gain { db: -7943, m: 7, n: 11 },
	st_gain { db: -7912, m: 29, n: 13 }, st_gain { db: -7883, m: 15, n: 12 }, st_gain { db: -7854, m: 31, n: 13 }, st_gain { db: -7827, m: 1, n: 8 },
	st_gain { db: -7774, m: 17, n: 12 }, st_gain { db: -7724, m: 9, n: 11 }, st_gain { db: -7678, m: 19, n: 12 }, st_gain { db: -7633, m: 5, n: 10 },
	st_gain { db: -7591, m: 21, n: 12 }, st_gain { db: -7550, m: 11, n: 11 }, st_gain { db: -7512, m: 23, n: 12 }, st_gain { db: -7475, m: 3, n: 9 },
	st_gain { db: -7439, m: 25, n: 12 }, st_gain { db: -7405, m: 13, n: 11 }, st_gain { db: -7372, m: 27, n: 12 }, st_gain { db: -7341, m: 7, n: 10 },
	st_gain { db: -7310, m: 29, n: 12 }, st_gain { db: -7281, m: 15, n: 11 }, st_gain { db: -7252, m: 31, n: 12 }, st_gain { db: -7225, m: 1, n: 7 },
	st_gain { db: -7172, m: 17, n: 11 }, st_gain { db: -7122, m: 9, n: 10 }, st_gain { db: -7075, m: 19, n: 11 }, st_gain { db: -7031, m: 5, n: 9 },
	st_gain { db: -6989, m: 21, n: 11 }, st_gain { db: -6948, m: 11, n: 10 }, st_gain { db: -6910, m: 23, n: 11 }, st_gain { db: -6873, m: 3, n: 8 },
	st_gain { db: -6837, m: 25, n: 11 }, st_gain { db: -6803, m: 13, n: 10 }, st_gain { db: -6770, m: 27, n: 11 }, st_gain { db: -6739, m: 7, n: 9 },
	st_gain { db: -6708, m: 29, n: 11 }, st_gain { db: -6679, m: 15, n: 10 }, st_gain { db: -6650, m: 31, n: 11 }, st_gain { db: -6623, m: 1, n: 6 },
	st_gain { db: -6570, m: 17, n: 10 }, st_gain { db: -6520, m: 9, n: 9 }, st_gain { db: -6473, m: 19, n: 10 }, st_gain { db: -6429, m: 5, n: 8 },
	st_gain { db: -6386, m: 21, n: 10 }, st_gain { db: -6346, m: 11, n: 9 }, st_gain { db: -6307, m: 23, n: 10 }, st_gain { db: -6270, m: 3, n: 7 },
	st_gain { db: -6235, m: 25, n: 10 }, st_gain { db: -6201, m: 13, n: 9 }, st_gain { db: -6168, m: 27, n: 10 }, st_gain { db: -6137, m: 7, n: 8 },
	st_gain { db: -6106, m: 29, n: 10 }, st_gain { db: -6077, m: 15, n: 9 }, st_gain { db: -6048, m: 31, n: 10 }, st_gain { db: -6021, m: 1, n: 5 },
	st_gain { db: -5968, m: 17, n: 9 }, st_gain { db: -5918, m: 9, n: 8 }, st_gain { db: -5871, m: 19, n: 9 }, st_gain { db: -5827, m: 5, n: 7 },
	st_gain { db: -5784, m: 21, n: 9 }, st_gain { db: -5744, m: 11, n: 8 }, st_gain { db: -5705, m: 23, n: 9 }, st_gain { db: -5668, m: 3, n: 6 },
	st_gain { db: -5633, m: 25, n: 9 }, st_gain { db: -5599, m: 13, n: 8 }, st_gain { db: -5566, m: 27, n: 9 }, st_gain { db: -5535, m: 7, n: 7 },
	st_gain { db: -5504, m: 29, n: 9 }, st_gain { db: -5475, m: 15, n: 8 }, st_gain { db: -5446, m: 31, n: 9 }, st_gain { db: -5419, m: 1, n: 4 },
	st_gain { db: -5366, m: 17, n: 8 }, st_gain { db: -5316, m: 9, n: 7 }, st_gain { db: -5269, m: 19, n: 8 }, st_gain { db: -5225, m: 5, n: 6 },
	st_gain { db: -5182, m: 21, n: 8 }, st_gain { db: -5142, m: 11, n: 7 }, st_gain { db: -5103, m: 23, n: 8 }, st_gain { db: -5066, m: 3, n: 5 },
	st_gain { db: -5031, m: 25, n: 8 }, st_gain { db: -4997, m: 13, n: 7 }, st_gain { db: -4964, m: 27, n: 8 }, st_gain { db: -4932, m: 7, n: 6 },
	st_gain { db: -4902, m: 29, n: 8 }, st_gain { db: -4873, m: 15, n: 7 }, st_gain { db: -4844, m: 31, n: 8 }, st_gain { db: -4816, m: 1, n: 3 },
	st_gain { db: -4764, m: 17, n: 7 }, st_gain { db: -4714, m: 9, n: 6 }, st_gain { db: -4667, m: 19, n: 7 }, st_gain { db: -4623, m: 5, n: 5 },
	st_gain { db: -4580, m: 21, n: 7 }, st_gain { db: -4540, m: 11, n: 6 }, st_gain { db: -4501, m: 23, n: 7 }, st_gain { db: -4464, m: 3, n: 4 },
	st_gain { db: -4429, m: 25, n: 7 }, st_gain { db: -4395, m: 13, n: 6 }, st_gain { db: -4362, m: 27, n: 7 }, st_gain { db: -4330, m: 7, n: 5 },
	st_gain { db: -4300, m: 29, n: 7 }, st_gain { db: -4270, m: 15, n: 6 }, st_gain { db: -4242, m: 31, n: 7 }, st_gain { db: -4214, m: 1, n: 2 },
	st_gain { db: -4162, m: 17, n: 6 }, st_gain { db: -4112, m: 9, n: 5 }, st_gain { db: -4065, m: 19, n: 6 }, st_gain { db: -4021, m: 5, n: 4 },
	st_gain { db: -3978, m: 21, n: 6 }, st_gain { db: -3938, m: 11, n: 5 }, st_gain { db: -3899, m: 23, n: 6 }, st_gain { db: -3862, m: 3, n: 3 },
	st_gain { db: -3827, m: 25, n: 6 }, st_gain { db: -3793, m: 13, n: 5 }, st_gain { db: -3760, m: 27, n: 6 }, st_gain { db: -3728, m: 7, n: 4 },
	st_gain { db: -3698, m: 29, n: 6 }, st_gain { db: -3668, m: 15, n: 5 }, st_gain { db: -3640, m: 31, n: 6 }, st_gain { db: -3612, m: 1, n: 1 },
	st_gain { db: -3560, m: 17, n: 5 }, st_gain { db: -3510, m: 9, n: 4 }, st_gain { db: -3463, m: 19, n: 5 }, st_gain { db: -3419, m: 5, n: 3 },
	st_gain { db: -3376, m: 21, n: 5 }, st_gain { db: -3336, m: 11, n: 4 }, st_gain { db: -3297, m: 23, n: 5 }, st_gain { db: -3260, m: 3, n: 2 },
	st_gain { db: -3225, m: 25, n: 5 }, st_gain { db: -3191, m: 13, n: 4 }, st_gain { db: -3158, m: 27, n: 5 }, st_gain { db: -3126, m: 7, n: 3 },
	st_gain { db: -3096, m: 29, n: 5 }, st_gain { db: -3066, m: 15, n: 4 }, st_gain { db: -3038, m: 31, n: 5 }, st_gain { db: -3010, m: 1, n: 0 },
	st_gain { db: -2958, m: 17, n: 4 }, st_gain { db: -2908, m: 9, n: 3 }, st_gain { db: -2861, m: 19, n: 4 }, st_gain { db: -2816, m: 5, n: 2 },
	st_gain { db: -2774, m: 21, n: 4 }, st_gain { db: -2734, m: 11, n: 3 }, st_gain { db: -2695, m: 23, n: 4 }, st_gain { db: -2658, m: 3, n: 1 },
	st_gain { db: -2623, m: 25, n: 4 }, st_gain { db: -2589, m: 13, n: 3 }, st_gain { db: -2556, m: 27, n: 4 }, st_gain { db: -2524, m: 7, n: 2 },
	st_gain { db: -2494, m: 29, n: 4 }, st_gain { db: -2464, m: 15, n: 3 }, st_gain { db: -2436, m: 31, n: 4 }, st_gain { db: -2408, m: 2, n: 0 },
	st_gain { db: -2356, m: 17, n: 3 }, st_gain { db: -2306, m: 9, n: 2 }, st_gain { db: -2259, m: 19, n: 3 }, st_gain { db: -2214, m: 5, n: 1 },
	st_gain { db: -2172, m: 21, n: 3 }, st_gain { db: -2132, m: 11, n: 2 }, st_gain { db: -2093, m: 23, n: 3 }, st_gain { db: -2056, m: 3, n: 0 },
	st_gain { db: -2021, m: 25, n: 3 }, st_gain { db: -1987, m: 13, n: 2 }, st_gain { db: -1954, m: 27, n: 3 }, st_gain { db: -1922, m: 7, n: 1 },
	st_gain { db: -1892, m: 29, n: 3 }, st_gain { db: -1862, m: 15, n: 2 }, st_gain { db: -1834, m: 31, n: 3 }, st_gain { db: -1806, m: 4, n: 0 },
	st_gain { db: -1754, m: 17, n: 2 }, st_gain { db: -1704, m: 9, n: 1 }, st_gain { db: -1657, m: 19, n: 2 }, st_gain { db: -1612, m: 5, n: 0 },
	st_gain { db: -1570, m: 21, n: 2 }, st_gain { db: -1530, m: 11, n: 1 }, st_gain { db: -1491, m: 23, n: 2 }, st_gain { db: -1454, m: 6, n: 0 },
	st_gain { db: -1419, m: 25, n: 2 }, st_gain { db: -1384, m: 13, n: 1 }, st_gain { db: -1352, m: 27, n: 2 }, st_gain { db: -1320, m: 7, n: 0 },
	st_gain { db: -1290, m: 29, n: 2 }, st_gain { db: -1260, m: 15, n: 1 }, st_gain { db: -1232, m: 31, n: 2 }, st_gain { db: -1204, m: 8, n: 0 },
	st_gain { db: -1151, m: 17, n: 1 }, st_gain { db: -1102, m: 9, n: 0 }, st_gain { db: -1055, m: 19, n: 1 }, st_gain { db: -1010, m: 10, n: 0 },
	st_gain { db: -968, m: 21, n: 1 }, st_gain { db: -928, m: 11, n: 0 }, st_gain { db: -889, m: 23, n: 1 }, st_gain { db: -852, m: 12, n: 0 },
	st_gain { db: -816, m: 25, n: 1 }, st_gain { db: -782, m: 13, n: 0 }, st_gain { db: -750, m: 27, n: 1 }, st_gain { db: -718, m: 14, n: 0 },
	st_gain { db: -688, m: 29, n: 1 }, st_gain { db: -658, m: 15, n: 0 }, st_gain { db: -630, m: 31, n: 1 }, st_gain { db: -602, m: 16, n: 0 },
	st_gain { db: -549, m: 17, n: 0 }, st_gain { db: -500, m: 18, n: 0 }, st_gain { db: -453, m: 19, n: 0 }, st_gain { db: -408, m: 20, n: 0 },
	st_gain { db: -366, m: 21, n: 0 }, st_gain { db: -325, m: 22, n: 0 }, st_gain { db: -287, m: 23, n: 0 }, st_gain { db: -250, m: 24, n: 0 },
	st_gain { db: -214, m: 25, n: 0 }, st_gain { db: -180, m: 26, n: 0 }, st_gain { db: -148, m: 27, n: 0 }, st_gain { db: -116, m: 28, n: 0 },
	st_gain { db: -86, m: 29, n: 0 }, st_gain { db: -56, m: 30, n: 0 }, st_gain { db: -28, m: 31, n: 0 }, st_gain { db: 0, m: 0, n: 0 },
];

static int snd_soc_get_volsw_2r_st(struct snd_kcontrol *kcontrol,
				   struct snd_ctl_elem_value *ucontrol)
{
	struct soc_mixer_control *mc =
		(struct soc_mixer_control *)kcontrol->private_value;
	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
	unsigned int reg = mc->reg;
	unsigned int reg2 = mc->rreg;
	int val[2], val2[2], i;

	val[0] = snd_soc_component_read(component, reg) & 0x3f;
	val[1] = (snd_soc_component_read(component, PM860X_SIDETONE_SHIFT) >> 4) & 0xf;
	val2[0] = snd_soc_component_read(component, reg2) & 0x3f;
	val2[1] = (snd_soc_component_read(component, PM860X_SIDETONE_SHIFT)) & 0xf;

extern "C" {
    static PM860X_SIDETONE_SHIFT: c_uint; static PM860X_DAC_OFFSET: c_uint; static PM860X_EAR_CTRL_2: c_uint; static PM860X_DAC_EN_2: c_uint; static PM860X_PCM_IFACE_2: c_uint; static PM860X_PCM_RATE: c_uint; static PM860X_I2S_IFACE_2: c_uint; static PM860X_I2S_IFACE_4: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int; static SND_SOC_DAPM_PRE_PMD: c_int; static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint; static SND_SOC_DAIFMT_CBP_CFP: c_uint; static SND_SOC_DAIFMT_CBP_CFC: c_uint; static SND_SOC_DAIFMT_CBC_CFC: c_uint; static SND_SOC_DAIFMT_FORMAT_MASK: c_uint; static SND_SOC_DAIFMT_I2S: c_uint; static PM860X_CLK_DIR_OUT: c_uint; static PM860X_CLK_DIR_IN: c_uint; static SND_SOC_BIAS_STANDBY: snd_soc_bias_level; static SND_SOC_BIAS_OFF: snd_soc_bias_level; static SND_JACK_HEADPHONE: c_int; static SND_JACK_MICROPHONE: c_int; static IRQF_ONESHOT: c_uint; static IORESOURCE_IRQ: c_uint; static GFP_KERNEL: c_uint; static CHIP_PM8607: c_int;
    static mut pm860x_dai: [snd_soc_dai_driver; 2]; static soc_component_dev_pm860x: snd_soc_component_driver; static mut pm860x_codec_driver: platform_driver;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component; fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint; fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int; fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_int) -> c_int; fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component; fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, s: *const c_char) -> c_int; fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut pm860x_priv; fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context; fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level; fn pm860x_reg_write(i2c: *mut i2c_client, reg: c_uint, data: c_int) -> c_int; fn pm860x_reg_read(i2c: *mut i2c_client, reg: c_uint) -> c_int; fn pm860x_set_bits(i2c: *mut i2c_client, reg: c_uint, mask: c_int, data: c_int) -> c_int; fn udelay(usecs: c_uint); fn params_width(params: *mut snd_pcm_hw_params) -> c_uint; fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint; fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int); fn dev_name(dev: *mut device) -> *const c_char; fn trace_snd_soc_jack_irq(name: *const c_char); fn dev_dbg(dev: *mut device, fmt: *const c_char, ...); fn dev_err(dev: *mut device, fmt: *const c_char, ...); fn request_threaded_irq(irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_uint, name: *const u8, dev: *mut c_void) -> c_int; fn free_irq(irq: c_int, dev: *mut c_void); fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap); fn dev_get_drvdata(dev: *mut device) -> *mut c_void; fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void; fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void); fn platform_get_resource(pdev: *mut platform_device, ty: c_uint, num: c_uint) -> *mut resource; fn strscpy(dst: *mut u8, src: *const c_char, count: usize) -> isize; fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: usize) -> c_int;
}
const EINVAL: c_int = 22; const ENOMEM: c_int = 12; const IRQ_HANDLED: irqreturn_t = 1;
fn fls(x: c_int) -> c_int { if x == 0 { 0 } else { c_int::BITS as c_int - x.leading_zeros() as c_int } }

unsafe extern "C" fn snd_soc_get_volsw_2r_st(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let mc = (*kcontrol).private_value as *mut soc_mixer_control; let component = snd_kcontrol_chip(kcontrol); let reg = (*mc).reg; let reg2 = (*mc).rreg; let val = [snd_soc_component_read(component, reg) & 0x3f, (snd_soc_component_read(component, PM860X_SIDETONE_SHIFT) >> 4) & 0xf]; let val2 = [snd_soc_component_read(component, reg2) & 0x3f, snd_soc_component_read(component, PM860X_SIDETONE_SHIFT) & 0xf]; let mut i = 0usize; while i < st_table.len() { if st_table[i].m == val[0] && st_table[i].n == val[1] { (*ucontrol).value.integer.value[0] = i as c_int; } if st_table[i].m == val2[0] && st_table[i].n == val2[1] { (*ucontrol).value.integer.value[1] = i as c_int; } i += 1; } 0 }
unsafe extern "C" fn snd_soc_put_volsw_2r_st(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let mc = (*kcontrol).private_value as *mut soc_mixer_control; let component = snd_kcontrol_chip(kcontrol); let reg = (*mc).reg; let reg2 = (*mc).rreg; let val = (*ucontrol).value.integer.value[0] as c_uint; let val2 = (*ucontrol).value.integer.value[1] as c_uint; if val as usize >= st_table.len() || val2 as usize >= st_table.len() { return -EINVAL; } let mut err = snd_soc_component_update_bits(component, reg, 0x3f, st_table[val as usize].m); if err < 0 { return err; } err = snd_soc_component_update_bits(component, PM860X_SIDETONE_SHIFT, 0xf0, st_table[val as usize].n << 4); if err < 0 { return err; } err = snd_soc_component_update_bits(component, reg2, 0x3f, st_table[val2 as usize].m); if err < 0 { return err; } snd_soc_component_update_bits(component, PM860X_SIDETONE_SHIFT, 0x0f, st_table[val2 as usize].n) }
unsafe extern "C" fn snd_soc_get_volsw_2r_out(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let mc = (*kcontrol).private_value as *mut soc_mixer_control; let component = snd_kcontrol_chip(kcontrol); let reg = (*mc).reg; let reg2 = (*mc).rreg; let shift = (*mc).shift; let max = (*mc).max; let mask = ((1u32 << fls(max)) - 1) as c_int; let val = (snd_soc_component_read(component, reg) >> shift) as c_int; let val2 = (snd_soc_component_read(component, reg2) >> shift) as c_int; (*ucontrol).value.integer.value[0] = (max - val) & mask; (*ucontrol).value.integer.value[1] = (max - val2) & mask; 0 }
unsafe extern "C" fn snd_soc_put_volsw_2r_out(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let mc = (*kcontrol).private_value as *mut soc_mixer_control; let component = snd_kcontrol_chip(kcontrol); let reg = (*mc).reg; let reg2 = (*mc).rreg; let shift = (*mc).shift; let max = (*mc).max; let mask = ((1u32 << fls(max)) - 1) as c_uint; let val_mask = mask << shift; let val = (((max - (*ucontrol).value.integer.value[0]) as c_uint) & mask) << shift; let val2 = (((max - (*ucontrol).value.integer.value[1]) as c_uint) & mask) << shift; let mut err = snd_soc_component_update_bits(component, reg, val_mask, val); if err < 0 { return err; } err = snd_soc_component_update_bits(component, reg2, val_mask, val2); err }
unsafe extern "C" fn pm860x_rsync_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let component = snd_soc_dapm_to_component((*w).dapm); snd_soc_component_update_bits(component, PM860X_DAC_OFFSET, DAC_MUTE, 0); snd_soc_component_update_bits(component, PM860X_EAR_CTRL_2, RSYNC_CHANGE, RSYNC_CHANGE); 0 }
unsafe extern "C" fn pm860x_dac_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let component = snd_soc_dapm_to_component((*w).dapm); let mut dac: c_uint = 0; if snd_soc_dapm_widget_name_cmp(w, b"Left DAC\0".as_ptr() as *const c_char) == 0 { dac = DAC_LEFT; } if snd_soc_dapm_widget_name_cmp(w, b"Right DAC\0".as_ptr() as *const c_char) == 0 { dac = DAC_RIGHT; } if event == SND_SOC_DAPM_PRE_PMU { if dac != 0 { dac |= MODULATOR; snd_soc_component_update_bits(component, PM860X_DAC_OFFSET, DAC_MUTE, DAC_MUTE); snd_soc_component_update_bits(component, PM860X_EAR_CTRL_2, RSYNC_CHANGE, RSYNC_CHANGE); snd_soc_component_update_bits(component, PM860X_DAC_EN_2, dac, dac); } } else if event == SND_SOC_DAPM_PRE_PMD { if dac != 0 { snd_soc_component_update_bits(component, PM860X_DAC_OFFSET, DAC_MUTE, DAC_MUTE); snd_soc_component_update_bits(component, PM860X_EAR_CTRL_2, RSYNC_CHANGE, RSYNC_CHANGE); let mut data = snd_soc_component_read(component, PM860X_DAC_EN_2) as c_int; data &= !(dac as c_int); if (data & ((DAC_LEFT | DAC_RIGHT) as c_int)) == 0 { data &= !(MODULATOR as c_int); } snd_soc_component_write(component, PM860X_DAC_EN_2, data); } } 0 }

/* The C source's SOC_ENUM/SOC_DAPM/SOC control arrays are direct external macro data declarations. They are intentionally represented by Rust macro invocations/declarations supplied by the future kernel binding layer. */

unsafe extern "C" fn pm860x_mute_stream(codec_dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int { let component = (*codec_dai).component; let mask = MUTE_LEFT | MUTE_RIGHT; let data = if mute != 0 { mask } else { 0 }; snd_soc_component_update_bits(component, PM860X_DAC_OFFSET, mask, data); snd_soc_component_update_bits(component, PM860X_EAR_CTRL_2, RSYNC_CHANGE, RSYNC_CHANGE); 0 }
unsafe extern "C" fn pm860x_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int { let component = (*dai).component; let mut inf: c_uint = 0; let mut mask: c_uint = 0; match params_width(params) { 16 => inf &= !PCM_INF2_18WL, 18 => inf |= PCM_INF2_18WL, _ => return -EINVAL } mask |= PCM_INF2_18WL; snd_soc_component_update_bits(component, PM860X_PCM_IFACE_2, mask, inf); inf = match params_rate(params) { 8000 => 0, 16000 => 3, 32000 => 6, 48000 => 8, _ => return -EINVAL }; snd_soc_component_update_bits(component, PM860X_PCM_RATE, 0x0f, inf); 0 }
unsafe extern "C" fn pm860x_pcm_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int { let component = (*codec_dai).component; let pm860x = snd_soc_component_get_drvdata(component); let mut inf: c_uint = 0; let mut mask: c_uint = PCM_INF2_BCLK | PCM_INF2_FS | PCM_INF2_MASTER; let mut ret = -EINVAL; if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFP || (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFC { if (*pm860x).dir == PM860X_CLK_DIR_OUT { inf |= PCM_INF2_MASTER; ret = 0; } } else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC { if (*pm860x).dir == PM860X_CLK_DIR_IN { inf &= !PCM_INF2_MASTER; ret = 0; } } if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S { inf |= PCM_EXACT_I2S; ret = 0; } mask |= PCM_MODE_MASK; if ret != 0 { return ret; } snd_soc_component_update_bits(component, PM860X_PCM_IFACE_2, mask, inf); 0 }
unsafe extern "C" fn pm860x_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int { let component = (*codec_dai).component; let pm860x = snd_soc_component_get_drvdata(component); if dir as c_uint == PM860X_CLK_DIR_OUT { (*pm860x).dir = PM860X_CLK_DIR_OUT; } else { return -EINVAL; } 0 }
unsafe extern "C" fn pm860x_i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int { let component = (*dai).component; let mut inf = match params_width(params) { 16 => 0, 18 => PCM_INF2_18WL, _ => return -EINVAL }; snd_soc_component_update_bits(component, PM860X_I2S_IFACE_2, PCM_INF2_18WL, inf); inf = match params_rate(params) { 8000 => 0, 11025 => 1, 16000 => 3, 22050 => 4, 32000 => 6, 44100 => 7, 48000 => 8, _ => return -EINVAL }; snd_soc_component_update_bits(component, PM860X_I2S_IFACE_4, 0xf, inf); 0 }
unsafe extern "C" fn pm860x_i2s_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int { let component = (*codec_dai).component; let pm860x = snd_soc_component_get_drvdata(component); let mut inf: c_uint = 0; let mut mask: c_uint = PCM_INF2_BCLK | PCM_INF2_FS | PCM_INF2_MASTER; if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFP { if (*pm860x).dir == PM860X_CLK_DIR_OUT { inf |= PCM_INF2_MASTER; } else { return -EINVAL; } } else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC { if (*pm860x).dir == PM860X_CLK_DIR_IN { inf &= !PCM_INF2_MASTER; } else { return -EINVAL; } } else { return -EINVAL; } if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S { inf |= PCM_EXACT_I2S; } else { return -EINVAL; } mask |= PCM_MODE_MASK; snd_soc_component_update_bits(component, PM860X_I2S_IFACE_2, mask, inf); 0 }
unsafe extern "C" fn pm860x_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int { let pm860x = snd_soc_component_get_drvdata(component); let dapm = snd_soc_component_to_dapm(component); if level == SND_SOC_BIAS_STANDBY { if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF { let mut data = AUDIO_PLL | AUDIO_SECTION_ON; pm860x_reg_write((*pm860x).i2c, REG_MISC2, data); udelay(300); data = AUDIO_PLL | AUDIO_SECTION_RESET | AUDIO_SECTION_ON; pm860x_reg_write((*pm860x).i2c, REG_MISC2, data); } } else if level == SND_SOC_BIAS_OFF { let data = AUDIO_PLL | AUDIO_SECTION_RESET | AUDIO_SECTION_ON; pm860x_set_bits((*pm860x).i2c, REG_MISC2, data, 0); } 0 }

unsafe extern "C" fn pm860x_component_handler(irq: c_int, data: *mut c_void) -> irqreturn_t { let pm860x = data as *mut pm860x_priv; let status = pm860x_reg_read((*pm860x).i2c, REG_STATUS_1); let shrt = pm860x_reg_read((*pm860x).i2c, REG_SHORTS); let mut report = 0; let mut mic_report = 0; let mask = (*pm860x).det.hs_shrt | (*pm860x).det.hook_det | (*pm860x).det.lo_shrt | (*pm860x).det.hp_det; if (status & (HEADSET_STATUS | MIC_STATUS | SHORT_HS1 | SHORT_HS2 | SHORT_LO1 | SHORT_LO2)) != 0 { trace_snd_soc_jack_irq(dev_name((*(*pm860x).component).dev)); } if ((*pm860x).det.hp_det & SND_JACK_HEADPHONE) != 0 && (status & HEADSET_STATUS) != 0 { report |= SND_JACK_HEADPHONE; } if ((*pm860x).det.mic_det & SND_JACK_MICROPHONE) != 0 && (status & MIC_STATUS) != 0 { mic_report |= SND_JACK_MICROPHONE; } if (*pm860x).det.hs_shrt != 0 && (shrt & (SHORT_HS1 | SHORT_HS2)) != 0 { report |= (*pm860x).det.hs_shrt; } if (*pm860x).det.hook_det != 0 && (status & HOOK_STATUS) != 0 { report |= (*pm860x).det.hook_det; } if (*pm860x).det.lo_shrt != 0 && (shrt & (SHORT_LO1 | SHORT_LO2)) != 0 { report |= (*pm860x).det.lo_shrt; } if report != 0 { snd_soc_jack_report((*pm860x).det.hp_jack, report, mask); } if mic_report != 0 { snd_soc_jack_report((*pm860x).det.mic_jack, SND_JACK_MICROPHONE, SND_JACK_MICROPHONE); } dev_dbg((*(*pm860x).component).dev, b"headphone report:0x%x, mask:%x\n\0".as_ptr() as *const c_char, report, mask); dev_dbg((*(*pm860x).component).dev, b"microphone report:0x%x\n\0".as_ptr() as *const c_char, mic_report); IRQ_HANDLED }
#[no_mangle] pub unsafe extern "C" fn pm860x_hs_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, det: c_int, hook: c_int, hs_shrt: c_int, lo_shrt: c_int) -> c_int { let pm860x = snd_soc_component_get_drvdata(component); (*pm860x).det.hp_jack = jack; (*pm860x).det.hp_det = det; (*pm860x).det.hook_det = hook; (*pm860x).det.hs_shrt = hs_shrt; (*pm860x).det.lo_shrt = lo_shrt; if (det & SND_JACK_HEADPHONE) != 0 { pm860x_set_bits((*pm860x).i2c, REG_HS_DET, EN_HS_DET, EN_HS_DET); } if hs_shrt != 0 { let data = CLR_SHORT_HS2 | CLR_SHORT_HS1; pm860x_set_bits((*pm860x).i2c, REG_SHORTS, data, data); } if lo_shrt != 0 { let data = CLR_SHORT_LO2 | CLR_SHORT_LO1; pm860x_set_bits((*pm860x).i2c, REG_SHORTS, data, data); } pm860x_component_handler(0, pm860x as *mut c_void); 0 }
EXPORT_SYMBOL_GPL!(pm860x_hs_jack_detect);
#[no_mangle] pub unsafe extern "C" fn pm860x_mic_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, det: c_int) -> c_int { let pm860x = snd_soc_component_get_drvdata(component); (*pm860x).det.mic_jack = jack; (*pm860x).det.mic_det = det; if (det & SND_JACK_MICROPHONE) != 0 { pm860x_set_bits((*pm860x).i2c, REG_MIC_DET, MICDET_MASK, MICDET_MASK); } pm860x_component_handler(0, pm860x as *mut c_void); 0 }
EXPORT_SYMBOL_GPL!(pm860x_mic_jack_detect);
unsafe extern "C" fn pm860x_probe(component: *mut snd_soc_component) -> c_int { let pm860x = snd_soc_component_get_drvdata(component); (*pm860x).component = component; snd_soc_component_init_regmap(component, (*pm860x).regmap); let mut i: c_int = 0; while i < 4 { let ret = request_threaded_irq((*pm860x).irq[i as usize], None, Some(pm860x_component_handler), IRQF_ONESHOT, (*pm860x).name[i as usize].as_ptr(), pm860x as *mut c_void); if ret < 0 { dev_err((*component).dev, b"Failed to request IRQ!\n\0".as_ptr() as *const c_char); while { i -= 1; i >= 0 } { free_irq((*pm860x).irq[i as usize], pm860x as *mut c_void); } return ret; } i += 1; } 0 }
unsafe extern "C" fn pm860x_remove(component: *mut snd_soc_component) { let pm860x = snd_soc_component_get_drvdata(component); let mut i: c_int = 3; while i >= 0 { free_irq((*pm860x).irq[i as usize], pm860x as *mut c_void); i -= 1; } }
unsafe extern "C" fn pm860x_codec_probe(pdev: *mut platform_device) -> c_int { let chip = dev_get_drvdata((*pdev).dev.parent) as *mut pm860x_chip; let pm860x = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pm860x_priv>(), GFP_KERNEL) as *mut pm860x_priv; if pm860x.is_null() { return -ENOMEM; } (*pm860x).chip = chip; if (*chip).id == CHIP_PM8607 { (*pm860x).i2c = (*chip).client; (*pm860x).regmap = (*chip).regmap; } else { (*pm860x).i2c = (*chip).companion; (*pm860x).regmap = (*chip).regmap_companion; } platform_set_drvdata(pdev, pm860x as *mut c_void); let mut i: c_uint = 0; while i < 4 { let res = platform_get_resource(pdev, IORESOURCE_IRQ, i); if res.is_null() { dev_err(&mut (*pdev).dev, b"Failed to get IRQ resources\n\0".as_ptr() as *const c_char); return -EINVAL; } (*pm860x).irq[i as usize] = (*res).start + (*chip).irq_base; strscpy((*pm860x).name[i as usize].as_mut_ptr(), (*res).name, MAX_NAME_LEN); i += 1; } let ret = devm_snd_soc_register_component(&mut (*pdev).dev, &soc_component_dev_pm860x, pm860x_dai.as_mut_ptr(), pm860x_dai.len()); if ret != 0 { dev_err(&mut (*pdev).dev, b"Failed to register component\n\0".as_ptr() as *const c_char); return -EINVAL; } ret }
module_platform_driver!(pm860x_codec_driver);
MODULE_DESCRIPTION!("ASoC 88PM860x driver"); MODULE_AUTHOR!("Haojian Zhuang <haojian.zhuang@marvell.com>"); MODULE_LICENSE!("GPL"); MODULE_ALIAS!("platform:88pm860x-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
