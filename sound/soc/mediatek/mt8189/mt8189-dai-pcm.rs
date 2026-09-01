// SPDX-License-Identifier: GPL-2.0
/*
 *  MediaTek ALSA SoC Audio DAI I2S Control
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

/* Dependencies from the original C includes:
 * linux/regmap.h
 * sound/pcm_params.h
 * mt8189-afe-common.h
 * mt8189-interconnection.h
 * mt8189-afe-clk.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
	pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
	pub name: *const c_char,
	pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
	pub active: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
	pub sink: *const c_char,
	pub control: *const c_char,
	pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
	pub hw_params: Option<
		unsafe extern "C" fn(
			*mut snd_pcm_substream,
			*mut snd_pcm_hw_params,
			*mut snd_soc_dai,
		) -> c_int,
	>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub stream_name: *const c_char,
	pub channels_min: c_uint,
	pub channels_max: c_uint,
	pub rates: c_uint,
	pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub name: *const c_char,
	pub id: c_int,
	pub playback: snd_soc_pcm_stream,
	pub capture: snd_soc_pcm_stream,
	pub ops: *const snd_soc_dai_ops,
	pub symmetric_rate: c_uint,
	pub symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct list_head {
	pub next: *mut list_head,
	pub prev: *mut list_head,
}

#[repr(C)]
pub struct mtk_base_afe {
	pub dev: *mut device,
	pub regmap: *mut regmap,
	pub sub_dais: list_head,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
	pub list: list_head,
	pub dai_drivers: *mut snd_soc_dai_driver,
	pub num_dai_drivers: c_uint,
	pub dapm_widgets: *const snd_soc_dapm_widget,
	pub num_dapm_widgets: c_uint,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_uint,
}

unsafe extern "C" {
	fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
	fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
	fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
	fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn list_add(new: *mut list_head, head: *mut list_head);
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(u32)]
enum AUD_TX_LCH_RPT {
	AUD_TX_LCH_RPT_NO_REPEAT,
	AUD_TX_LCH_RPT_REPEAT,
}

#[repr(u32)]
enum AUD_VBT_16K_MODE {
	AUD_VBT_16K_MODE_DISABLE,
	AUD_VBT_16K_MODE_ENABLE,
}

#[repr(u32)]
enum AUD_EXT_MODEM {
	AUD_EXT_MODEM_SELECT_INTERNAL,
	AUD_EXT_MODEM_SELECT_EXTERNAL,
}

#[repr(u32)]
enum AUD_PCM_SYNC_TYPE {
	/* bck sync length = 1 */
	AUD_PCM_ONE_BCK_CYCLE_SYNC,
	/* bck sync length = PCM_INTF_CON1[9:13] */
	AUD_PCM_EXTENDED_BCK_CYCLE_SYNC,
}

#[repr(u32)]
enum AUD_BT_MODE {
	AUD_BT_MODE_DUAL_MIC_ON_TX,
	AUD_BT_MODE_SINGLE_MIC_ON_TX,
}

#[repr(u32)]
enum AUD_PCM_AFIFO_SRC {
	/* slave mode & external modem uses different crystal */
	AUD_PCM_AFIFO_ASRC,
	/* slave mode & external modem uses the same crystal */
	AUD_PCM_AFIFO_AFIFO,
}

#[repr(u32)]
enum AUD_PCM_CLOCK_SOURCE {
	AUD_PCM_CLOCK_MASTER_MODE,
	AUD_PCM_CLOCK_SLAVE_MODE,
}

#[repr(u32)]
enum AUD_PCM_WLEN {
	AUD_PCM_WLEN_PCM_32_BCK_CYCLES,
	AUD_PCM_WLEN_PCM_64_BCK_CYCLES,
}

#[repr(u32)]
enum AUD_PCM_MODE {
	AUD_PCM_MODE_PCM_MODE_8K,
	AUD_PCM_MODE_PCM_MODE_16K,
	AUD_PCM_MODE_PCM_MODE_32K,
	AUD_PCM_MODE_PCM_MODE_48K,
}

#[repr(u32)]
enum AUD_PCM_FMT {
	AUD_PCM_FMT_I2S,
	AUD_PCM_FMT_EIAJ,
	AUD_PCM_FMT_PCM_MODE_A,
	AUD_PCM_FMT_PCM_MODE_B,
}

#[repr(u32)]
enum AUD_BCLK_OUT_INV {
	AUD_BCLK_OUT_INV_NO_INVERSE,
	AUD_BCLK_OUT_INV_INVERSE,
}

#[repr(u32)]
enum AUD_PCM_EN {
	AUD_PCM_EN_DISABLE,
	AUD_PCM_EN_ENABLE,
}

#[repr(u32)]
enum AUD_PCM1_1X_EN_DOMAIN {
	HOPPING_26M,
	APLL,
	SLAVE = 6,
}

#[repr(u32)]
enum AUD_PCM1_1X_EN_SLAVE_MODE {
	PCM0_SLAVE_1X_EN,
	PCM1_SLAVE_1X_EN,
}

const PCM_8K: c_uint = 0;
const PCM_16K: c_uint = 4;
const PCM_32K: c_uint = 8;
const PCM_48K: c_uint = 10;

unsafe fn pcm_1x_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
	match rate {
		8000 => PCM_8K,
		16000 => PCM_16K,
		32000 => PCM_32K,
		48000 => PCM_48K,
		_ => {
			unsafe {
				dev_warn(
					dev,
					c"rate %u invalid, use %d!!!\n".as_ptr(),
					rate,
					PCM_48K,
				);
			}
			PCM_48K
		}
	}
}

unsafe fn pcm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
	match rate {
		8000 => MTK_AFE_PCM_RATE_8K,
		16000 => MTK_AFE_PCM_RATE_16K,
		32000 => MTK_AFE_PCM_RATE_32K,
		48000 => MTK_AFE_PCM_RATE_48K,
		_ => {
			unsafe {
				dev_warn(
					dev,
					c"rate %u invalid, use %d\n".as_ptr(),
					rate,
					MTK_AFE_PCM_RATE_48K,
				);
			}
			MTK_AFE_PCM_RATE_48K
		}
	}
}

/* dai component */
static mtk_pcm_0_playback_ch1_mix: [snd_kcontrol_new; 3] = [
	SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH1".as_ptr(), AFE_CONN096_0, I_ADDA_UL_CH1, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"DL2_CH1".as_ptr(), AFE_CONN096_1, I_DL2_CH1, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"DL_24CH_CH1".as_ptr(), AFE_CONN096_1, I_DL_24CH_CH1, 1, 0),
];

static mtk_pcm_0_playback_ch2_mix: [snd_kcontrol_new; 3] = [
	SOC_DAPM_SINGLE_AUTODISABLE!(c"ADDA_UL_CH2".as_ptr(), AFE_CONN097_0, I_ADDA_UL_CH2, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"DL2_CH2".as_ptr(), AFE_CONN097_1, I_DL2_CH2, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"DL_24CH_CH2".as_ptr(), AFE_CONN097_1, I_DL_24CH_CH2, 1, 0),
];

static mtk_pcm_0_playback_ch4_mix: [snd_kcontrol_new; 4] = [
	SOC_DAPM_SINGLE_AUTODISABLE!(c"I2SIN1_CH1".as_ptr(), AFE_CONN099_4, I_I2SIN1_CH1, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"I2SIN1_CH2".as_ptr(), AFE_CONN099_4, I_I2SIN1_CH2, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"DL0_CH1".as_ptr(), AFE_CONN099_1, I_DL0_CH1, 1, 0),
	SOC_DAPM_SINGLE_AUTODISABLE!(c"DL_24CH_CH1".as_ptr(), AFE_CONN099_1, I_DL_24CH_CH1, 1, 0),
];

static mtk_dai_pcm_widgets: [snd_soc_dapm_widget; 7] = [
	/* inter-connections */
	SND_SOC_DAPM_MIXER!(
		c"PCM_0_PB_CH1".as_ptr(),
		SND_SOC_NOPM,
		0,
		0,
		mtk_pcm_0_playback_ch1_mix.as_ptr(),
		mtk_pcm_0_playback_ch1_mix.len()
	),
	SND_SOC_DAPM_MIXER!(
		c"PCM_0_PB_CH2".as_ptr(),
		SND_SOC_NOPM,
		0,
		0,
		mtk_pcm_0_playback_ch2_mix.as_ptr(),
		mtk_pcm_0_playback_ch2_mix.len()
	),
	SND_SOC_DAPM_MIXER!(
		c"PCM_0_PB_CH4".as_ptr(),
		SND_SOC_NOPM,
		0,
		0,
		mtk_pcm_0_playback_ch4_mix.as_ptr(),
		mtk_pcm_0_playback_ch4_mix.len()
	),
	SND_SOC_DAPM_SUPPLY!(c"PCM_0_EN".as_ptr(), AFE_PCM0_INTF_CON0, PCM0_EN_SFT, 0, ptr::null_mut(), 0),
	SND_SOC_DAPM_SUPPLY!(c"PCM0_CG".as_ptr(), AUDIO_TOP_CON0, PDN_PCM0_SFT, 1, ptr::null_mut(), 0),
	SND_SOC_DAPM_INPUT!(c"AFE_PCM_INPUT".as_ptr()),
	SND_SOC_DAPM_OUTPUT!(c"AFE_PCM_OUTPUT".as_ptr()),
];

static mtk_dai_pcm_routes: [snd_soc_dapm_route; 15] = [
	snd_soc_dapm_route { sink: c"PCM 0 Playback".as_ptr(), control: ptr::null(), source: c"PCM_0_PB_CH1".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Playback".as_ptr(), control: ptr::null(), source: c"PCM_0_PB_CH2".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Playback".as_ptr(), control: ptr::null(), source: c"PCM_0_PB_CH4".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Playback".as_ptr(), control: ptr::null(), source: c"PCM_0_EN".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Capture".as_ptr(), control: ptr::null(), source: c"PCM_0_EN".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Playback".as_ptr(), control: ptr::null(), source: c"PCM0_CG".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Capture".as_ptr(), control: ptr::null(), source: c"PCM0_CG".as_ptr() },
	snd_soc_dapm_route { sink: c"AFE_PCM_OUTPUT".as_ptr(), control: ptr::null(), source: c"PCM 0 Playback".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM 0 Capture".as_ptr(), control: ptr::null(), source: c"AFE_PCM_INPUT".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM_0_PB_CH1".as_ptr(), control: c"DL2_CH1".as_ptr(), source: c"DL2".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM_0_PB_CH2".as_ptr(), control: c"DL2_CH2".as_ptr(), source: c"DL2".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM_0_PB_CH4".as_ptr(), control: c"DL0_CH1".as_ptr(), source: c"DL0".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM_0_PB_CH1".as_ptr(), control: c"DL_24CH_CH1".as_ptr(), source: c"DL_24CH".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM_0_PB_CH2".as_ptr(), control: c"DL_24CH_CH2".as_ptr(), source: c"DL_24CH".as_ptr() },
	snd_soc_dapm_route { sink: c"PCM_0_PB_CH4".as_ptr(), control: c"DL_24CH_CH1".as_ptr(), source: c"DL_24CH".as_ptr() },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_pcm_hw_params(
	substream: *mut snd_pcm_substream,
	params: *mut snd_pcm_hw_params,
	dai: *mut snd_soc_dai,
) -> c_int {
	let afe: *mut mtk_base_afe = unsafe { snd_soc_dai_get_drvdata(dai) };
	let rate: c_uint = unsafe { params_rate(params) };
	let rate_reg: c_uint = unsafe { pcm_rate_transform((*afe).dev, rate) };
	let x_rate_reg: c_uint = unsafe { pcm_1x_rate_transform((*afe).dev, rate) };
	let mut pcm_con0: c_uint;
	let mut pcm_con1: c_uint;
	let mut playback_active: c_uint = 0;
	let mut capture_active: c_uint = 0;
	let playback_widget: *mut snd_soc_dapm_widget =
		unsafe { snd_soc_dai_get_widget(dai, SNDRV_PCM_STREAM_PLAYBACK) };
	let capture_widget: *mut snd_soc_dapm_widget =
		unsafe { snd_soc_dai_get_widget(dai, SNDRV_PCM_STREAM_CAPTURE) };

	if !playback_widget.is_null() {
		playback_active = unsafe { (*playback_widget).active };
	}
	if !capture_widget.is_null() {
		capture_active = unsafe { (*capture_widget).active };
	}
	unsafe {
		dev_dbg(
			(*afe).dev,
			c"id %d, stream %d, rate %d, rate_reg %d, active p %d, c %d\n".as_ptr(),
			(*dai).id,
			(*substream).stream,
			rate,
			rate_reg,
			playback_active,
			capture_active,
		);
	}

	if playback_active != 0 || capture_active != 0 {
		return 0;
	}
	match unsafe { (*dai).id } {
		MT8189_DAI_PCM_0 => {
			pcm_con0 = (AUD_BCLK_OUT_INV::AUD_BCLK_OUT_INV_NO_INVERSE as c_uint) << PCM0_BCLK_OUT_INV_SFT;
			pcm_con0 |= (AUD_TX_LCH_RPT::AUD_TX_LCH_RPT_NO_REPEAT as c_uint) << PCM0_TX_LCH_RPT_SFT;
			pcm_con0 |= (AUD_VBT_16K_MODE::AUD_VBT_16K_MODE_DISABLE as c_uint) << PCM0_VBT_16K_MODE_SFT;
			pcm_con0 |= 0 << PCM0_SYNC_LENGTH_SFT;
			pcm_con0 |= (AUD_PCM_SYNC_TYPE::AUD_PCM_ONE_BCK_CYCLE_SYNC as c_uint) << PCM0_SYNC_TYPE_SFT;
			pcm_con0 |= (AUD_PCM_AFIFO_SRC::AUD_PCM_AFIFO_AFIFO as c_uint) << PCM0_BYP_ASRC_SFT;
			pcm_con0 |= (AUD_PCM_CLOCK_SOURCE::AUD_PCM_CLOCK_MASTER_MODE as c_uint) << PCM0_SLAVE_SFT;
			pcm_con0 |= rate_reg << PCM0_MODE_SFT;
			pcm_con0 |= (AUD_PCM_FMT::AUD_PCM_FMT_I2S as c_uint) << PCM0_FMT_SFT;

			pcm_con1 = (AUD_EXT_MODEM::AUD_EXT_MODEM_SELECT_INTERNAL as c_uint) << PCM0_EXT_MODEM_SFT;
			pcm_con1 |= (AUD_BT_MODE::AUD_BT_MODE_DUAL_MIC_ON_TX as c_uint) << PCM0_BT_MODE_SFT;
			pcm_con1 |= (AUD_PCM1_1X_EN_DOMAIN::HOPPING_26M as c_uint) << PCM0_1X_EN_DOMAIN_SFT;
			pcm_con1 |= x_rate_reg << PCM0_1X_EN_MODE_SFT;

			unsafe {
				regmap_update_bits(
					(*afe).regmap,
					AFE_PCM0_INTF_CON0,
					!(PCM0_EN_MASK_SFT as c_uint),
					pcm_con0,
				);
				regmap_update_bits(
					(*afe).regmap,
					AFE_PCM0_INTF_CON1,
					AFE_PCM0_INTF_CON1_MASK_MON_MASK_SFT,
					pcm_con1,
				);
			}
		}
		_ => {
			unsafe {
				dev_err(
					(*afe).dev,
					c"%s(), id %d not support\n".as_ptr(),
					c"mtk_dai_pcm_hw_params".as_ptr(),
					(*dai).id,
				);
			}
			return -EINVAL;
		}
	}
	0
}

static mtk_dai_pcm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
	hw_params: Some(mtk_dai_pcm_hw_params),
};

/* dai driver */
const MTK_PCM_RATES: c_uint =
	SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;

const MTK_PCM_FORMATS: u64 =
	SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
	name: c"PCM 0".as_ptr(),
	id: MT8189_DAI_PCM_0,
	playback: snd_soc_pcm_stream {
		stream_name: c"PCM 0 Playback".as_ptr(),
		channels_min: 1,
		channels_max: 2,
		rates: MTK_PCM_RATES,
		formats: MTK_PCM_FORMATS,
	},
	capture: snd_soc_pcm_stream {
		stream_name: c"PCM 0 Capture".as_ptr(),
		channels_min: 1,
		channels_max: 2,
		rates: MTK_PCM_RATES,
		formats: MTK_PCM_FORMATS,
	},
	ops: &mtk_dai_pcm_ops,
	symmetric_rate: 1,
	symmetric_sample_bits: 1,
}];

#[no_mangle]
pub unsafe extern "C" fn mt8189_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int {
	let dai: *mut mtk_base_afe_dai;

	dai = unsafe { devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) }
		.cast::<mtk_base_afe_dai>();
	if dai.is_null() {
		return -ENOMEM;
	}

	unsafe {
		(*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
		(*dai).num_dai_drivers = mtk_dai_pcm_driver.len() as c_uint;
		(*dai).dapm_widgets = mtk_dai_pcm_widgets.as_ptr();
		(*dai).num_dapm_widgets = mtk_dai_pcm_widgets.len() as c_uint;
		(*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
		(*dai).num_dapm_routes = mtk_dai_pcm_routes.len() as c_uint;

		list_add(&mut (*dai).list, &mut (*afe).sub_dais);
	}

	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
