// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt1305.c  --  RT1305 ALSA SoC amplifier component driver
 *
 * Copyright 2018 Realtek Semiconductor Corp.
 * Author: Shuming Fan <shumingf@realtek.com>
 */

// Translated from C. Kernel, ALSA SoC, regmap, rl6231, and rt1305 header
// definitions are external dependencies supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

const RT1305_PR_RANGE_BASE: c_uint = 0xff + 1;
const RT1305_PR_SPACING: c_uint = 0x100;
const RT1305_PR_BASE: c_uint = RT1305_PR_RANGE_BASE + (0 * RT1305_PR_SPACING);

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
	pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
	pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_kcontrol {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol_new {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
	pub component: *mut snd_soc_component,
	pub id: c_int,
}
#[repr(C)]
pub struct i2c_client {
	pub dev: device,
}

#[repr(C)]
pub struct regmap_range_cfg {
	pub name: *const c_char,
	pub range_min: c_uint,
	pub range_max: c_uint,
	pub selector_reg: c_uint,
	pub selector_mask: c_uint,
	pub selector_shift: c_uint,
	pub window_start: c_uint,
	pub window_len: c_uint,
}

#[repr(C)]
pub struct reg_sequence {
	pub reg: c_uint,
	pub def: c_uint,
}

#[repr(C)]
struct rt1305_priv {
	component: *mut snd_soc_component,
	regmap: *mut regmap,
	sysclk: c_int,
	sysclk_src: c_int,
	lrck: c_int,
	bclk: c_int,
	master: c_int,
	pll_src: c_int,
	pll_in: c_int,
	pll_out: c_int,
}

#[repr(C)]
pub struct reg_default {
	pub reg: c_uint,
	pub def: c_uint,
}

#[repr(C)]
pub struct rl6231_pll_code {
	pub m_bp: c_int,
	pub m_code: c_int,
	pub n_code: c_int,
	pub k_code: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
	pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
	pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub stream_name: *const c_char,
	pub channels_min: c_uint,
	pub channels_max: c_uint,
	pub rates: c_uint,
	pub formats: c_ulonglong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub name: *const c_char,
	pub playback: snd_soc_pcm_stream,
	pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
	pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
	pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
	pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
	pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
	pub controls: *const snd_kcontrol_new,
	pub num_controls: c_uint,
	pub dapm_widgets: *const snd_soc_dapm_widget_desc,
	pub num_dapm_widgets: c_uint,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_uint,
	pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
	pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
	pub use_pmdown_time: c_uint,
	pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
	pub reg_bits: c_uint,
	pub val_bits: c_uint,
	pub max_register: c_uint,
	pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
	pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
	pub cache_type: c_uint,
	pub reg_defaults: *const reg_default,
	pub num_reg_defaults: c_uint,
	pub ranges: *const regmap_range_cfg,
	pub num_ranges: c_uint,
	pub use_single_read: bool,
	pub use_single_write: bool,
}

#[repr(C)]
pub struct of_device_id {
	pub compatible: *const c_char,
}
#[repr(C)]
pub struct acpi_device_id {
	pub id: *const c_char,
}
#[repr(C)]
pub struct i2c_device_id {
	pub name: *const c_char,
}
#[repr(C)]
pub struct device_driver {
	pub name: *const c_char,
	pub of_match_table: *const of_device_id,
	pub acpi_match_table: *const acpi_device_id,
}
#[repr(C)]
pub struct i2c_driver {
	pub driver: device_driver,
	pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
	pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
	pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
	static RT1305_PRIV_INDEX: c_uint;
	static RT1305_PRIV_DATA: c_uint;
	static RT1305_POWER_STATUS: c_uint;
	static RT1305_SPK_TEMP_PROTECTION_1: c_uint;
	static RT1305_SPK_TEMP_PROTECTION_2: c_uint;
	static RT1305_SPK_TEMP_PROTECTION_3: c_uint;
	static RT1305_DAC_SET_1: c_uint;
	static RT1305_ADC_SET_3: c_uint;
	static RT1305_ADC_SET_1: c_uint;
	static RT1305_RESET: c_uint;
	static RT1305_SPDIF_IN_SET_1: c_uint;
	static RT1305_SPDIF_IN_SET_2: c_uint;
	static RT1305_SPDIF_IN_SET_3: c_uint;
	static RT1305_POWER_CTRL_2: c_uint;
	static RT1305_CLOCK_DETECT: c_uint;
	static RT1305_BIQUAD_SET_1: c_uint;
	static RT1305_BIQUAD_SET_2: c_uint;
	static RT1305_EQ_SET_2: c_uint;
	static RT1305_SPK_TEMP_PROTECTION_0: c_uint;
	static RT1305_SPK_DC_DETECT_1: c_uint;
	static RT1305_SILENCE_DETECT: c_uint;
	static RT1305_VERSION_ID: c_uint;
	static RT1305_VENDOR_ID: c_uint;
	static RT1305_DEVICE_ID: c_uint;
	static RT1305_EFUSE_1: c_uint;
	static RT1305_EFUSE_3: c_uint;
	static RT1305_DC_CALIB_1: c_uint;
	static RT1305_DC_CALIB_3: c_uint;
	static RT1305_DAC_OFFSET_1: c_uint;
	static RT1305_DAC_OFFSET_2: c_uint;
	static RT1305_DAC_OFFSET_3: c_uint;
	static RT1305_DAC_OFFSET_4: c_uint;
	static RT1305_DAC_OFFSET_5: c_uint;
	static RT1305_DAC_OFFSET_6: c_uint;
	static RT1305_DAC_OFFSET_7: c_uint;
	static RT1305_DAC_OFFSET_8: c_uint;
	static RT1305_DAC_OFFSET_9: c_uint;
	static RT1305_DAC_OFFSET_10: c_uint;
	static RT1305_DAC_OFFSET_11: c_uint;
	static RT1305_TRIM_1: c_uint;
	static RT1305_TRIM_2: c_uint;
	static RT1305_CLK_1: c_uint;
	static RT1305_CAL_EFUSE_CLOCK: c_uint;
	static RT1305_PLL0_1: c_uint;
	static RT1305_PLL1_2: c_uint;
	static RT1305_MIXER_CTRL_1: c_uint;
	static RT1305_MIXER_CTRL_2: c_uint;
	static RT1305_DAC_SET_2: c_uint;
	static RT1305_ADC_SET_2: c_uint;
	static RT1305_PATH_SET: c_uint;
	static RT1305_SPDIF_OUT_SET_1: c_uint;
	static RT1305_SPDIF_OUT_SET_2: c_uint;
	static RT1305_SPDIF_OUT_SET_3: c_uint;
	static RT1305_I2S_SET_1: c_uint;
	static RT1305_I2S_SET_2: c_uint;
	static RT1305_PBTL_MONO_MODE_SRC: c_uint;
	static RT1305_MANUALLY_I2C_DEVICE: c_uint;
	static RT1305_POWER_CTRL_1: c_uint;
	static RT1305_POWER_CTRL_3: c_uint;
	static RT1305_POWER_CTRL_4: c_uint;
	static RT1305_POWER_CTRL_5: c_uint;
	static RT1305_ADJUSTED_HPF_1: c_uint;
	static RT1305_ADJUSTED_HPF_2: c_uint;
	static RT1305_EQ_SET_1: c_uint;
	static RT1305_SPK_DC_DETECT_2: c_uint;
	static RT1305_LOUDNESS: c_uint;
	static RT1305_THERMAL_FOLD_BACK_1: c_uint;
	static RT1305_THERMAL_FOLD_BACK_2: c_uint;
	static RT1305_SPK_EXCURSION_LIMITER_7: c_uint;
	static RT1305_EFUSE_2: c_uint;
	static RT1305_DC_CALIB_2: c_uint;
	static RT1305_DAC_OFFSET_14: c_uint;
	static RT1305_TUNE_INTERNAL_OSC: c_uint;
	static RT1305_BIQUAD1_H0_L_28_16: c_uint;
	static RT1305_BIQUAD3_A2_R_15_0: c_uint;
	static RT1305_FS_SYS_PRE_S_PLL1: c_int;
	static RT1305_SEL_PLL_SRC_2_RCCLK: c_uint;
	static SND_SOC_DAPM_POST_PMU: c_int;
	static SND_SOC_DAPM_PRE_PMD: c_int;
	static RT1305_POW_PDB_JD_MASK: c_uint;
	static RT1305_POW_PDB_JD: c_uint;
	static RT1305_DVOL_MUTE_L_EN_SFT: c_uint;
	static RT1305_DVOL_MUTE_R_EN_SFT: c_uint;
	static SND_SOC_NOPM: c_uint;
	static RT1305_POW_PLL0_EN_BIT: c_uint;
	static RT1305_POW_PLL1_EN_BIT: c_uint;
	static RT1305_POW_MBIAS_LV_BIT: c_uint;
	static RT1305_POW_BG_MBIAS_LV_BIT: c_uint;
	static RT1305_POW_LDO2_BIT: c_uint;
	static RT1305_POW_BG2_BIT: c_uint;
	static RT1305_POW_LDO2_IB2_BIT: c_uint;
	static RT1305_POW_VREF_BIT: c_uint;
	static RT1305_POW_VREF1_BIT: c_uint;
	static RT1305_POW_VREF2_BIT: c_uint;
	static RT1305_POW_DISC_VREF_BIT: c_uint;
	static RT1305_POW_FASTB_VREF_BIT: c_uint;
	static RT1305_POW_ULTRA_FAST_VREF_BIT: c_uint;
	static RT1305_POW_CKXEN_DAC_BIT: c_uint;
	static RT1305_POW_EN_CKGEN_DAC_BIT: c_uint;
	static RT1305_POW_CLAMP_BIT: c_uint;
	static RT1305_POW_BUFL_BIT: c_uint;
	static RT1305_POW_BUFR_BIT: c_uint;
	static RT1305_POW_EN_CKGEN_ADC_BIT: c_uint;
	static RT1305_POW_ADC3_L_BIT: c_uint;
	static RT1305_POW_ADC3_R_BIT: c_uint;
	static RT1305_POW_TRIOSC_BIT: c_uint;
	static RT1305_POR_AVDD1_BIT: c_uint;
	static RT1305_POR_AVDD2_BIT: c_uint;
	static RT1305_POW_VSENSE_RCH_BIT: c_uint;
	static RT1305_POW_VSENSE_LCH_BIT: c_uint;
	static RT1305_POW_ISENSE_RCH_BIT: c_uint;
	static RT1305_POW_ISENSE_LCH_BIT: c_uint;
	static RT1305_POW_POR_AVDD1_BIT: c_uint;
	static RT1305_POW_POR_AVDD2_BIT: c_uint;
	static RT1305_EN_VCM_6172_BIT: c_uint;
	static EINVAL: c_int;
	static RT1305_PLL1_S_BCLK: c_int;
	static SND_SOC_CLOCK_IN: c_int;
	static RT1305_I2S_DL_SEL_16B: c_uint;
	static RT1305_I2S_DL_SEL_20B: c_uint;
	static RT1305_I2S_DL_SEL_24B: c_uint;
	static RT1305_I2S_DL_SEL_8B: c_uint;
	static RT1305_AIF1: c_int;
	static RT1305_DIV_FS_SYS_MASK: c_uint;
	static RT1305_DIV_FS_SYS_SFT: c_uint;
	static RT1305_I2S_DL_SEL_MASK: c_uint;
	static RT1305_CLK_2: c_uint;
	static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
	static SND_SOC_DAIFMT_CBP_CFP: c_uint;
	static SND_SOC_DAIFMT_CBC_CFC: c_uint;
	static RT1305_SEL_I2S_OUT_MODE_M: c_uint;
	static RT1305_SEL_I2S_OUT_MODE_S: c_uint;
	static SND_SOC_DAIFMT_INV_MASK: c_uint;
	static SND_SOC_DAIFMT_NB_NF: c_uint;
	static SND_SOC_DAIFMT_IB_NF: c_uint;
	static RT1305_I2S_BCLK_INV: c_uint;
	static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
	static SND_SOC_DAIFMT_I2S: c_uint;
	static SND_SOC_DAIFMT_LEFT_J: c_uint;
	static SND_SOC_DAIFMT_DSP_A: c_uint;
	static SND_SOC_DAIFMT_DSP_B: c_uint;
	static RT1305_I2S_DF_SEL_LEFT: c_uint;
	static RT1305_I2S_DF_SEL_PCM_A: c_uint;
	static RT1305_I2S_DF_SEL_PCM_B: c_uint;
	static RT1305_SEL_I2S_OUT_MODE_MASK: c_uint;
	static RT1305_I2S_DF_SEL_MASK: c_uint;
	static RT1305_I2S_BCLK_MASK: c_uint;
	static RT1305_FS_SYS_PRE_S_MCLK: c_int;
	static RT1305_FS_SYS_PRE_S_RCCLK: c_int;
	static RT1305_SEL_FS_SYS_PRE_MCLK: c_uint;
	static RT1305_SEL_CLK_DET_SRC_MASK: c_uint;
	static RT1305_SEL_CLK_DET_SRC_MCLK: c_uint;
	static RT1305_SEL_FS_SYS_PRE_PLL: c_uint;
	static RT1305_SEL_FS_SYS_PRE_RCCLK: c_uint;
	static RT1305_SEL_FS_SYS_PRE_MASK: c_uint;
	static RT1305_PLL2_S_MCLK: c_int;
	static RT1305_PLL2_S_RCCLK: c_int;
	static RT1305_SEL_PLL_SRC_2_MASK: c_uint;
	static RT1305_SEL_PLL_SRC_1_MASK: c_uint;
	static RT1305_DIV_PLL_SRC_2_MASK: c_uint;
	static RT1305_SEL_PLL_SRC_2_MCLK: c_uint;
	static RT1305_SEL_PLL_SRC_1_PLL2: c_uint;
	static RT1305_SEL_PLL_SRC_1_BCLK: c_uint;
	static RT1305_PLL1_1: c_uint;
	static RT1305_PLL_1_M_SFT: c_uint;
	static RT1305_PLL_1_M_BYPASS_SFT: c_uint;
	static SNDRV_PCM_RATE_8000_192000: c_uint;
	static SNDRV_PCM_FMTBIT_S8: c_ulonglong;
	static SNDRV_PCM_FMTBIT_S20_3LE: c_ulonglong;
	static SNDRV_PCM_FMTBIT_S16_LE: c_ulonglong;
	static SNDRV_PCM_FMTBIT_S24_LE: c_ulonglong;
	static REGCACHE_MAPLE: c_uint;
	static RT1305_MAX_REG: c_uint;
	static R0_UPPER: c_ulonglong;
	static R0_LOWER: c_ulonglong;
	static RT1305_DEVICE_ID_NUM: c_uint;
	static ENOMEM: c_int;
	static ENODEV: c_int;
	static GFP_KERNEL: c_uint;

	fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
	fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
	fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
	fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
	fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
	fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
	fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
	fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
	fn usleep_range(min: c_uint, max: c_uint);
	fn msleep(msecs: c_uint);
	fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
	fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
	fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
	fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
	fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
	fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, pll_code: *mut rl6231_pll_code) -> c_int;
	fn regcache_cache_only(map: *mut regmap, enable: bool);
	fn regcache_mark_dirty(map: *mut regmap);
	fn regcache_sync(map: *mut regmap) -> c_int;
	fn regcache_cache_bypass(map: *mut regmap, enable: bool);
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
	fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
	fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
	fn IS_ERR(ptr: *const c_void) -> bool;
	fn PTR_ERR(ptr: *const c_void) -> c_int;
	fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn pr_info(fmt: *const c_char, ...);
	fn pr_debug(fmt: *const c_char, ...);
	fn pr_err(fmt: *const c_char, ...);
}

macro_rules! cstr {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

static RT1305_RANGES: [regmap_range_cfg; 1] = unsafe {
	[regmap_range_cfg {
		name: cstr!("PR"),
		range_min: RT1305_PR_BASE,
		range_max: RT1305_PR_BASE + 0xff,
		selector_reg: RT1305_PRIV_INDEX,
		selector_mask: 0xff,
		selector_shift: 0x0,
		window_start: RT1305_PRIV_DATA,
		window_len: 0x1,
	}]
};

static INIT_LIST: [reg_sequence; 10] = unsafe {
	[
		reg_sequence { reg: RT1305_PR_BASE + 0xcf, def: 0x5548 },
		reg_sequence { reg: RT1305_PR_BASE + 0x5d, def: 0x0442 },
		reg_sequence { reg: RT1305_PR_BASE + 0xc1, def: 0x0320 },
		reg_sequence { reg: RT1305_POWER_STATUS, def: 0x0000 },
		reg_sequence { reg: RT1305_SPK_TEMP_PROTECTION_1, def: 0xd6de },
		reg_sequence { reg: RT1305_SPK_TEMP_PROTECTION_2, def: 0x0707 },
		reg_sequence { reg: RT1305_SPK_TEMP_PROTECTION_3, def: 0x4090 },
		reg_sequence { reg: RT1305_DAC_SET_1, def: 0xdfdf }, /* 4 ohm 2W  */
		reg_sequence { reg: RT1305_ADC_SET_3, def: 0x0219 },
		reg_sequence { reg: RT1305_ADC_SET_1, def: 0x170f }, /* 0.2 ohm RSense*/
	]
};
const RT1305_INIT_REG_LEN: c_int = INIT_LIST.len() as c_int;

static RT1305_REG: [reg_default; 157] = [
	reg_default { reg: 0x04, def: 0x0400 }, reg_default { reg: 0x05, def: 0x0880 },
	reg_default { reg: 0x06, def: 0x0000 }, reg_default { reg: 0x07, def: 0x3100 },
	reg_default { reg: 0x08, def: 0x8000 }, reg_default { reg: 0x09, def: 0x0000 },
	reg_default { reg: 0x0a, def: 0x087e }, reg_default { reg: 0x0b, def: 0x0020 },
	reg_default { reg: 0x0c, def: 0x0802 }, reg_default { reg: 0x0d, def: 0x0020 },
	reg_default { reg: 0x10, def: 0x1d1d }, reg_default { reg: 0x11, def: 0x1d1d },
	reg_default { reg: 0x12, def: 0xffff }, reg_default { reg: 0x14, def: 0x000c },
	reg_default { reg: 0x16, def: 0x1717 }, reg_default { reg: 0x17, def: 0x4000 },
	reg_default { reg: 0x18, def: 0x0019 }, reg_default { reg: 0x20, def: 0x0000 },
	reg_default { reg: 0x22, def: 0x0000 }, reg_default { reg: 0x24, def: 0x0000 },
	reg_default { reg: 0x26, def: 0x0000 }, reg_default { reg: 0x28, def: 0x0000 },
	reg_default { reg: 0x2a, def: 0x4000 }, reg_default { reg: 0x2b, def: 0x3000 },
	reg_default { reg: 0x2d, def: 0x6000 }, reg_default { reg: 0x2e, def: 0x0000 },
	reg_default { reg: 0x2f, def: 0x8000 }, reg_default { reg: 0x32, def: 0x0000 },
	reg_default { reg: 0x39, def: 0x0001 }, reg_default { reg: 0x3a, def: 0x0000 },
	reg_default { reg: 0x3b, def: 0x1020 }, reg_default { reg: 0x3c, def: 0x0000 },
	reg_default { reg: 0x3d, def: 0x0000 }, reg_default { reg: 0x3e, def: 0x4c00 },
	reg_default { reg: 0x3f, def: 0x3000 }, reg_default { reg: 0x40, def: 0x000c },
	reg_default { reg: 0x42, def: 0x0400 }, reg_default { reg: 0x46, def: 0xc22c },
	reg_default { reg: 0x47, def: 0x0000 }, reg_default { reg: 0x4b, def: 0x0000 },
	reg_default { reg: 0x4c, def: 0x0300 }, reg_default { reg: 0x4f, def: 0xf000 },
	reg_default { reg: 0x50, def: 0xc200 }, reg_default { reg: 0x51, def: 0x1f1f },
	reg_default { reg: 0x52, def: 0x01f0 }, reg_default { reg: 0x53, def: 0x407f },
	reg_default { reg: 0x54, def: 0xffff }, reg_default { reg: 0x58, def: 0x4005 },
	reg_default { reg: 0x5e, def: 0x0000 }, reg_default { reg: 0x5f, def: 0x0000 },
	reg_default { reg: 0x60, def: 0xee13 }, reg_default { reg: 0x62, def: 0x0000 },
	reg_default { reg: 0x63, def: 0x5f5f }, reg_default { reg: 0x64, def: 0x0040 },
	reg_default { reg: 0x65, def: 0x4000 }, reg_default { reg: 0x66, def: 0x4004 },
	reg_default { reg: 0x67, def: 0x0306 }, reg_default { reg: 0x68, def: 0x8c04 },
	reg_default { reg: 0x69, def: 0xe021 }, reg_default { reg: 0x6a, def: 0x0000 },
	reg_default { reg: 0x6c, def: 0xaaaa }, reg_default { reg: 0x70, def: 0x0333 },
	reg_default { reg: 0x71, def: 0x3330 }, reg_default { reg: 0x72, def: 0x3333 },
	reg_default { reg: 0x73, def: 0x3300 }, reg_default { reg: 0x74, def: 0x0000 },
	reg_default { reg: 0x75, def: 0x0000 }, reg_default { reg: 0x76, def: 0x0000 },
	reg_default { reg: 0x7a, def: 0x0003 }, reg_default { reg: 0x7c, def: 0x10ec },
	reg_default { reg: 0x7e, def: 0x6251 }, reg_default { reg: 0x80, def: 0x0800 },
	reg_default { reg: 0x81, def: 0x4000 }, reg_default { reg: 0x82, def: 0x0000 },
	reg_default { reg: 0x90, def: 0x7a01 }, reg_default { reg: 0x91, def: 0x8431 },
	reg_default { reg: 0x92, def: 0x0180 }, reg_default { reg: 0x93, def: 0x0000 },
	reg_default { reg: 0x94, def: 0x0000 }, reg_default { reg: 0x95, def: 0x0000 },
	reg_default { reg: 0x96, def: 0x0000 }, reg_default { reg: 0x97, def: 0x0000 },
	reg_default { reg: 0x98, def: 0x0000 }, reg_default { reg: 0x99, def: 0x0000 },
	reg_default { reg: 0x9a, def: 0x0000 }, reg_default { reg: 0x9b, def: 0x0000 },
	reg_default { reg: 0x9c, def: 0x0000 }, reg_default { reg: 0x9d, def: 0x0000 },
	reg_default { reg: 0x9e, def: 0x0000 }, reg_default { reg: 0x9f, def: 0x0000 },
	reg_default { reg: 0xa0, def: 0x0000 }, reg_default { reg: 0xb0, def: 0x8200 },
	reg_default { reg: 0xb1, def: 0x00ff }, reg_default { reg: 0xb2, def: 0x0008 },
	reg_default { reg: 0xc0, def: 0x0200 }, reg_default { reg: 0xc1, def: 0x0000 },
	reg_default { reg: 0xc2, def: 0x0000 }, reg_default { reg: 0xc3, def: 0x0000 },
	reg_default { reg: 0xc4, def: 0x0000 }, reg_default { reg: 0xc5, def: 0x0000 },
	reg_default { reg: 0xc6, def: 0x0000 }, reg_default { reg: 0xc7, def: 0x0000 },
	reg_default { reg: 0xc8, def: 0x0000 }, reg_default { reg: 0xc9, def: 0x0000 },
	reg_default { reg: 0xca, def: 0x0200 }, reg_default { reg: 0xcb, def: 0x0000 },
	reg_default { reg: 0xcc, def: 0x0000 }, reg_default { reg: 0xcd, def: 0x0000 },
	reg_default { reg: 0xce, def: 0x0000 }, reg_default { reg: 0xcf, def: 0x0000 },
	reg_default { reg: 0xd0, def: 0x0000 }, reg_default { reg: 0xd1, def: 0x0000 },
	reg_default { reg: 0xd2, def: 0x0000 }, reg_default { reg: 0xd3, def: 0x0000 },
	reg_default { reg: 0xd4, def: 0x0200 }, reg_default { reg: 0xd5, def: 0x0000 },
	reg_default { reg: 0xd6, def: 0x0000 }, reg_default { reg: 0xd7, def: 0x0000 },
	reg_default { reg: 0xd8, def: 0x0000 }, reg_default { reg: 0xd9, def: 0x0000 },
	reg_default { reg: 0xda, def: 0x0000 }, reg_default { reg: 0xdb, def: 0x0000 },
	reg_default { reg: 0xdc, def: 0x0000 }, reg_default { reg: 0xdd, def: 0x0000 },
	reg_default { reg: 0xde, def: 0x0200 }, reg_default { reg: 0xdf, def: 0x0000 },
	reg_default { reg: 0xe0, def: 0x0000 }, reg_default { reg: 0xe1, def: 0x0000 },
	reg_default { reg: 0xe2, def: 0x0000 }, reg_default { reg: 0xe3, def: 0x0000 },
	reg_default { reg: 0xe4, def: 0x0000 }, reg_default { reg: 0xe5, def: 0x0000 },
	reg_default { reg: 0xe6, def: 0x0000 }, reg_default { reg: 0xe7, def: 0x0000 },
	reg_default { reg: 0xe8, def: 0x0200 }, reg_default { reg: 0xe9, def: 0x0000 },
	reg_default { reg: 0xea, def: 0x0000 }, reg_default { reg: 0xeb, def: 0x0000 },
	reg_default { reg: 0xec, def: 0x0000 }, reg_default { reg: 0xed, def: 0x0000 },
	reg_default { reg: 0xee, def: 0x0000 }, reg_default { reg: 0xef, def: 0x0000 },
	reg_default { reg: 0xf0, def: 0x0000 }, reg_default { reg: 0xf1, def: 0x0000 },
	reg_default { reg: 0xf2, def: 0x0200 }, reg_default { reg: 0xf3, def: 0x0000 },
	reg_default { reg: 0xf4, def: 0x0000 }, reg_default { reg: 0xf5, def: 0x0000 },
	reg_default { reg: 0xf6, def: 0x0000 }, reg_default { reg: 0xf7, def: 0x0000 },
	reg_default { reg: 0xf8, def: 0x0000 }, reg_default { reg: 0xf9, def: 0x0000 },
	reg_default { reg: 0xfa, def: 0x0000 }, reg_default { reg: 0xfb, def: 0x0000 },
];

unsafe extern "C" fn rt1305_reg_init(component: *mut snd_soc_component) -> c_int {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	regmap_multi_reg_write((*rt1305).regmap, INIT_LIST.as_ptr(), RT1305_INIT_REG_LEN);
	0
}

unsafe extern "C" fn rt1305_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
	for range in RT1305_RANGES.iter() {
		if reg >= range.range_min && reg <= range.range_max {
			return true;
		}
	}
	reg == RT1305_RESET || reg == RT1305_SPDIF_IN_SET_1 || reg == RT1305_SPDIF_IN_SET_2 ||
	reg == RT1305_SPDIF_IN_SET_3 || reg == RT1305_POWER_CTRL_2 || reg == RT1305_CLOCK_DETECT ||
	reg == RT1305_BIQUAD_SET_1 || reg == RT1305_BIQUAD_SET_2 || reg == RT1305_EQ_SET_2 ||
	reg == RT1305_SPK_TEMP_PROTECTION_0 || reg == RT1305_SPK_TEMP_PROTECTION_2 ||
	reg == RT1305_SPK_DC_DETECT_1 || reg == RT1305_SILENCE_DETECT || reg == RT1305_VERSION_ID ||
	reg == RT1305_VENDOR_ID || reg == RT1305_DEVICE_ID || reg == RT1305_EFUSE_1 ||
	reg == RT1305_EFUSE_3 || reg == RT1305_DC_CALIB_1 || reg == RT1305_DC_CALIB_3 ||
	reg == RT1305_DAC_OFFSET_1 || reg == RT1305_DAC_OFFSET_2 || reg == RT1305_DAC_OFFSET_3 ||
	reg == RT1305_DAC_OFFSET_4 || reg == RT1305_DAC_OFFSET_5 || reg == RT1305_DAC_OFFSET_6 ||
	reg == RT1305_DAC_OFFSET_7 || reg == RT1305_DAC_OFFSET_8 || reg == RT1305_DAC_OFFSET_9 ||
	reg == RT1305_DAC_OFFSET_10 || reg == RT1305_DAC_OFFSET_11 || reg == RT1305_TRIM_1 ||
	reg == RT1305_TRIM_2
}

unsafe extern "C" fn rt1305_readable_register(_dev: *mut device, reg: c_uint) -> bool {
	for range in RT1305_RANGES.iter() {
		if reg >= range.range_min && reg <= range.range_max {
			return true;
		}
	}
	reg == RT1305_RESET ||
	(reg >= RT1305_CLK_1 && reg <= RT1305_CAL_EFUSE_CLOCK) ||
	(reg >= RT1305_PLL0_1 && reg <= RT1305_PLL1_2) ||
	reg == RT1305_MIXER_CTRL_1 || reg == RT1305_MIXER_CTRL_2 || reg == RT1305_DAC_SET_1 ||
	reg == RT1305_DAC_SET_2 || reg == RT1305_ADC_SET_1 || reg == RT1305_ADC_SET_2 ||
	reg == RT1305_ADC_SET_3 || reg == RT1305_PATH_SET || reg == RT1305_SPDIF_IN_SET_1 ||
	reg == RT1305_SPDIF_IN_SET_2 || reg == RT1305_SPDIF_IN_SET_3 || reg == RT1305_SPDIF_OUT_SET_1 ||
	reg == RT1305_SPDIF_OUT_SET_2 || reg == RT1305_SPDIF_OUT_SET_3 || reg == RT1305_I2S_SET_1 ||
	reg == RT1305_I2S_SET_2 || reg == RT1305_PBTL_MONO_MODE_SRC || reg == RT1305_MANUALLY_I2C_DEVICE ||
	reg == RT1305_POWER_STATUS || reg == RT1305_POWER_CTRL_1 || reg == RT1305_POWER_CTRL_2 ||
	reg == RT1305_POWER_CTRL_3 || reg == RT1305_POWER_CTRL_4 || reg == RT1305_POWER_CTRL_5 ||
	reg == RT1305_CLOCK_DETECT || reg == RT1305_BIQUAD_SET_1 || reg == RT1305_BIQUAD_SET_2 ||
	reg == RT1305_ADJUSTED_HPF_1 || reg == RT1305_ADJUSTED_HPF_2 || reg == RT1305_EQ_SET_1 ||
	reg == RT1305_EQ_SET_2 || reg == RT1305_SPK_TEMP_PROTECTION_0 || reg == RT1305_SPK_TEMP_PROTECTION_1 ||
	reg == RT1305_SPK_TEMP_PROTECTION_2 || reg == RT1305_SPK_TEMP_PROTECTION_3 || reg == RT1305_SPK_DC_DETECT_1 ||
	reg == RT1305_SPK_DC_DETECT_2 || reg == RT1305_LOUDNESS || reg == RT1305_THERMAL_FOLD_BACK_1 ||
	reg == RT1305_THERMAL_FOLD_BACK_2 || (reg >= RT1305_SILENCE_DETECT && reg <= RT1305_SPK_EXCURSION_LIMITER_7) ||
	reg == RT1305_VERSION_ID || reg == RT1305_VENDOR_ID || reg == RT1305_DEVICE_ID || reg == RT1305_EFUSE_1 ||
	reg == RT1305_EFUSE_2 || reg == RT1305_EFUSE_3 || reg == RT1305_DC_CALIB_1 || reg == RT1305_DC_CALIB_2 ||
	reg == RT1305_DC_CALIB_3 || (reg >= RT1305_DAC_OFFSET_1 && reg <= RT1305_DAC_OFFSET_14) ||
	reg == RT1305_TRIM_1 || reg == RT1305_TRIM_2 || reg == RT1305_TUNE_INTERNAL_OSC ||
	(reg >= RT1305_BIQUAD1_H0_L_28_16 && reg <= RT1305_BIQUAD3_A2_R_15_0)
}

// static const DECLARE_TLV_DB_SCALE(dac_vol_tlv, -9435, 37, 0);
static RT1305_RX_DATA_CH_SELECT: [*const c_char; 4] = [cstr!("LR"), cstr!("RL"), cstr!("Copy L"), cstr!("Copy R")];
// static SOC_ENUM_SINGLE_DECL(rt1305_rx_data_ch_enum, RT1305_I2S_SET_2, 2, rt1305_rx_data_ch_select);

unsafe extern "C" fn rt1305_reset(regmap: *mut regmap) {
	regmap_write(regmap, RT1305_RESET, 0);
}

// rt1305_snd_controls, rt1305_sto_dac_l, rt1305_sto_dac_r, rt1305_dapm_widgets,
// and rt1305_dapm_routes are macro-defined ALSA data in C. They are referenced
// here as external static data emitted by equivalent Rust/kernel bindings.
unsafe extern "C" {
	static rt1305_snd_controls: [snd_kcontrol_new; 2];
	static rt1305_sto_dac_l: snd_kcontrol_new;
	static rt1305_sto_dac_r: snd_kcontrol_new;
	static rt1305_dapm_widgets: [snd_soc_dapm_widget_desc; 44];
	static rt1305_dapm_routes: [snd_soc_dapm_route; 39];
}

unsafe extern "C" fn rt1305_is_rc_clk_from_pll(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> c_int {
	let component = snd_soc_dapm_to_component((*source).dapm);
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	let val = snd_soc_component_read(component, RT1305_CLK_1);

	if (*rt1305).sysclk_src == RT1305_FS_SYS_PRE_S_PLL1 && (val & RT1305_SEL_PLL_SRC_2_RCCLK) != 0 {
		1
	} else {
		0
	}
}

unsafe extern "C" fn rt1305_is_sys_clk_from_pll(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> c_int {
	let component = snd_soc_dapm_to_component((*source).dapm);
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	if (*rt1305).sysclk_src == RT1305_FS_SYS_PRE_S_PLL1 { 1 } else { 0 }
}

unsafe extern "C" fn rt1305_classd_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
	let component = snd_soc_dapm_to_component((*w).dapm);
	if event == SND_SOC_DAPM_POST_PMU {
		snd_soc_component_update_bits(component, RT1305_POWER_CTRL_1, RT1305_POW_PDB_JD_MASK, RT1305_POW_PDB_JD);
	} else if event == SND_SOC_DAPM_PRE_PMD {
		snd_soc_component_update_bits(component, RT1305_POWER_CTRL_1, RT1305_POW_PDB_JD_MASK, 0);
		usleep_range(150000, 200000);
	}
	0
}

fn rt1305_get_clk_info(sclk: c_int, mut rate: c_int) -> c_int {
	static PD: [c_int; 8] = [1, 2, 3, 4, 6, 8, 12, 16];
	if sclk <= 0 || rate <= 0 {
		return unsafe { -EINVAL };
	}
	rate <<= 8;
	for (i, pd) in PD.iter().enumerate() {
		if sclk == rate * *pd {
			return i as c_int;
		}
	}
	unsafe { -EINVAL }
}

unsafe extern "C" fn rt1305_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
	let _ = substream;
	let component = (*dai).component;
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	let mut val_len: c_uint = 0;
	let val_clk: c_uint;
	let mask_clk: c_uint;
	let mut pre_div: c_int;
	let bclk_ms: c_int;
	let frame_size: c_int;

	(*rt1305).lrck = params_rate(params);
	pre_div = rt1305_get_clk_info((*rt1305).sysclk, (*rt1305).lrck);
	if pre_div < 0 {
		dev_warn((*component).dev, cstr!("Force using PLL "));
		snd_soc_dai_set_pll(dai, 0, RT1305_PLL1_S_BCLK, ((*rt1305).lrck * 64) as c_uint, ((*rt1305).lrck * 256) as c_uint);
		snd_soc_dai_set_sysclk(dai, RT1305_FS_SYS_PRE_S_PLL1, ((*rt1305).lrck * 256) as c_uint, SND_SOC_CLOCK_IN);
		pre_div = 0;
	}
	frame_size = snd_soc_params_to_frame_size(params);
	if frame_size < 0 {
		dev_err((*component).dev, cstr!("Unsupported frame size: %d\n"), frame_size);
		return -EINVAL;
	}

	bclk_ms = if frame_size > 32 { 1 } else { 0 };
	(*rt1305).bclk = (*rt1305).lrck * (32 << bclk_ms);
	dev_dbg((*component).dev, cstr!("bclk_ms is %d and pre_div is %d for iis %d\n"), bclk_ms, pre_div, (*dai).id);
	dev_dbg((*component).dev, cstr!("lrck is %dHz and pre_div is %d for iis %d\n"), (*rt1305).lrck, pre_div, (*dai).id);

	match params_width(params) {
		16 => val_len |= RT1305_I2S_DL_SEL_16B,
		20 => val_len |= RT1305_I2S_DL_SEL_20B,
		24 => val_len |= RT1305_I2S_DL_SEL_24B,
		8 => val_len |= RT1305_I2S_DL_SEL_8B,
		_ => return -EINVAL,
	}

	if (*dai).id == RT1305_AIF1 {
		mask_clk = RT1305_DIV_FS_SYS_MASK;
		val_clk = (pre_div as c_uint) << RT1305_DIV_FS_SYS_SFT;
		snd_soc_component_update_bits(component, RT1305_I2S_SET_2, RT1305_I2S_DL_SEL_MASK, val_len);
	} else {
		dev_err((*component).dev, cstr!("Invalid dai->id: %d\n"), (*dai).id);
		return -EINVAL;
	}

	snd_soc_component_update_bits(component, RT1305_CLK_2, mask_clk, val_clk);
	0
}

unsafe extern "C" fn rt1305_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
	let component = (*dai).component;
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	let mut reg_val: c_uint = 0;
	let mut reg1_val: c_uint = 0;

	if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
		reg_val |= RT1305_SEL_I2S_OUT_MODE_M;
		(*rt1305).master = 1;
	} else if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
		reg_val |= RT1305_SEL_I2S_OUT_MODE_S;
		(*rt1305).master = 0;
	} else {
		return -EINVAL;
	}

	if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_NF {
	} else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_NF {
		reg1_val |= RT1305_I2S_BCLK_INV;
	} else {
		return -EINVAL;
	}

	if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
	} else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_LEFT_J {
		reg1_val |= RT1305_I2S_DF_SEL_LEFT;
	} else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
		reg1_val |= RT1305_I2S_DF_SEL_PCM_A;
	} else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_B {
		reg1_val |= RT1305_I2S_DF_SEL_PCM_B;
	} else {
		return -EINVAL;
	}

	if (*dai).id == RT1305_AIF1 {
		snd_soc_component_update_bits(component, RT1305_I2S_SET_1, RT1305_SEL_I2S_OUT_MODE_MASK, reg_val);
		snd_soc_component_update_bits(component, RT1305_I2S_SET_2, RT1305_I2S_DF_SEL_MASK | RT1305_I2S_BCLK_MASK, reg1_val);
	} else {
		dev_err((*component).dev, cstr!("Invalid dai->id: %d\n"), (*dai).id);
		return -EINVAL;
	}
	0
}

unsafe extern "C" fn rt1305_set_component_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	let mut reg_val: c_uint = 0;
	if freq as c_int == (*rt1305).sysclk && clk_id == (*rt1305).sysclk_src {
		return 0;
	}
	if clk_id == RT1305_FS_SYS_PRE_S_MCLK {
		reg_val |= RT1305_SEL_FS_SYS_PRE_MCLK;
		snd_soc_component_update_bits(component, RT1305_CLOCK_DETECT, RT1305_SEL_CLK_DET_SRC_MASK, RT1305_SEL_CLK_DET_SRC_MCLK);
	} else if clk_id == RT1305_FS_SYS_PRE_S_PLL1 {
		reg_val |= RT1305_SEL_FS_SYS_PRE_PLL;
	} else if clk_id == RT1305_FS_SYS_PRE_S_RCCLK {
		reg_val |= RT1305_SEL_FS_SYS_PRE_RCCLK;
	} else {
		dev_err((*component).dev, cstr!("Invalid clock id (%d)\n"), clk_id);
		return -EINVAL;
	}
	snd_soc_component_update_bits(component, RT1305_CLK_1, RT1305_SEL_FS_SYS_PRE_MASK, reg_val);
	(*rt1305).sysclk = freq as c_int;
	(*rt1305).sysclk_src = clk_id;
	dev_dbg((*component).dev, cstr!("Sysclk is %dHz and clock id is %d\n"), freq, clk_id);
	0
}

unsafe extern "C" fn rt1305_set_component_pll(component: *mut snd_soc_component, _pll_id: c_int, source: c_int, mut freq_in: c_uint, freq_out: c_uint) -> c_int {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	let mut pll_code = rl6231_pll_code { m_bp: 0, m_code: 0, n_code: 0, k_code: 0 };
	let ret: c_int;

	if source == (*rt1305).pll_src && freq_in as c_int == (*rt1305).pll_in && freq_out as c_int == (*rt1305).pll_out {
		return 0;
	}

	if freq_in == 0 || freq_out == 0 {
		dev_dbg((*component).dev, cstr!("PLL disabled\n"));
		(*rt1305).pll_in = 0;
		(*rt1305).pll_out = 0;
		snd_soc_component_update_bits(component, RT1305_CLK_1, RT1305_SEL_FS_SYS_PRE_MASK | RT1305_SEL_PLL_SRC_1_MASK, RT1305_SEL_FS_SYS_PRE_PLL | RT1305_SEL_PLL_SRC_1_BCLK);
		return 0;
	}

	if source == RT1305_PLL2_S_MCLK {
		snd_soc_component_update_bits(component, RT1305_CLK_1, RT1305_SEL_PLL_SRC_2_MASK | RT1305_SEL_PLL_SRC_1_MASK | RT1305_DIV_PLL_SRC_2_MASK, RT1305_SEL_PLL_SRC_2_MCLK | RT1305_SEL_PLL_SRC_1_PLL2);
		snd_soc_component_update_bits(component, RT1305_CLOCK_DETECT, RT1305_SEL_CLK_DET_SRC_MASK, RT1305_SEL_CLK_DET_SRC_MCLK);
	} else if source == RT1305_PLL1_S_BCLK {
		snd_soc_component_update_bits(component, RT1305_CLK_1, RT1305_SEL_PLL_SRC_1_MASK, RT1305_SEL_PLL_SRC_1_BCLK);
	} else if source == RT1305_PLL2_S_RCCLK {
		snd_soc_component_update_bits(component, RT1305_CLK_1, RT1305_SEL_PLL_SRC_2_MASK | RT1305_SEL_PLL_SRC_1_MASK | RT1305_DIV_PLL_SRC_2_MASK, RT1305_SEL_PLL_SRC_2_RCCLK | RT1305_SEL_PLL_SRC_1_PLL2);
		freq_in = 98304000;
	} else {
		dev_err((*component).dev, cstr!("Unknown PLL Source %d\n"), source);
		return -EINVAL;
	}

	ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
	if ret < 0 {
		dev_err((*component).dev, cstr!("Unsupported input clock %d\n"), freq_in);
		return ret;
	}

	dev_dbg((*component).dev, cstr!("bypass=%d m=%d n=%d k=%d\n"), pll_code.m_bp, if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }, pll_code.n_code, pll_code.k_code);
	snd_soc_component_write(component, RT1305_PLL1_1, (((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint) << RT1305_PLL_1_M_SFT) | ((pll_code.m_bp as c_uint) << RT1305_PLL_1_M_BYPASS_SFT) | (pll_code.n_code as c_uint));
	snd_soc_component_write(component, RT1305_PLL1_2, pll_code.k_code as c_uint);
	(*rt1305).pll_in = freq_in as c_int;
	(*rt1305).pll_out = freq_out as c_int;
	(*rt1305).pll_src = source;
	0
}

unsafe extern "C" fn rt1305_probe(component: *mut snd_soc_component) -> c_int {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	(*rt1305).component = component;
	/* initial settings */
	rt1305_reg_init(component);
	0
}

unsafe extern "C" fn rt1305_remove(component: *mut snd_soc_component) {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	rt1305_reset((*rt1305).regmap);
}

// CONFIG_PM conditional in C: rt1305_suspend/rt1305_resume are NULL when CONFIG_PM is unset.
unsafe extern "C" fn rt1305_suspend(component: *mut snd_soc_component) -> c_int {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	regcache_cache_only((*rt1305).regmap, true);
	regcache_mark_dirty((*rt1305).regmap);
	0
}

unsafe extern "C" fn rt1305_resume(component: *mut snd_soc_component) -> c_int {
	let rt1305 = snd_soc_component_get_drvdata(component) as *mut rt1305_priv;
	regcache_cache_only((*rt1305).regmap, false);
	regcache_sync((*rt1305).regmap);
	0
}

static RT1305_AIF_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
	hw_params: Some(rt1305_hw_params),
	set_fmt: Some(rt1305_set_dai_fmt),
};

static mut RT1305_DAI: [snd_soc_dai_driver; 1] = unsafe {
	[snd_soc_dai_driver {
		name: cstr!("rt1305-aif"),
		playback: snd_soc_pcm_stream {
			stream_name: cstr!("AIF1 Playback"),
			channels_min: 1,
			channels_max: 2,
			rates: SNDRV_PCM_RATE_8000_192000,
			formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
		},
		ops: &RT1305_AIF_DAI_OPS,
	}]
};

static SOC_COMPONENT_DEV_RT1305: snd_soc_component_driver = snd_soc_component_driver {
	probe: Some(rt1305_probe),
	remove: Some(rt1305_remove),
	suspend: Some(rt1305_suspend),
	resume: Some(rt1305_resume),
	controls: unsafe { rt1305_snd_controls.as_ptr() },
	num_controls: 2,
	dapm_widgets: unsafe { rt1305_dapm_widgets.as_ptr() },
	num_dapm_widgets: 44,
	dapm_routes: unsafe { rt1305_dapm_routes.as_ptr() },
	num_dapm_routes: 39,
	set_sysclk: Some(rt1305_set_component_sysclk),
	set_pll: Some(rt1305_set_component_pll),
	use_pmdown_time: 1,
	endianness: 1,
};

static RT1305_REGMAP: regmap_config = unsafe {
	regmap_config {
		reg_bits: 8,
		val_bits: 16,
		max_register: RT1305_MAX_REG + 1 + (RT1305_RANGES.len() as c_uint * RT1305_PR_SPACING),
		volatile_reg: Some(rt1305_volatile_register),
		readable_reg: Some(rt1305_readable_register),
		cache_type: REGCACHE_MAPLE,
		reg_defaults: RT1305_REG.as_ptr(),
		num_reg_defaults: RT1305_REG.len() as c_uint,
		ranges: RT1305_RANGES.as_ptr(),
		num_ranges: RT1305_RANGES.len() as c_uint,
		use_single_read: true,
		use_single_write: true,
	}
};

// CONFIG_OF conditional device table.
static RT1305_OF_MATCH: [of_device_id; 3] = [
	of_device_id { compatible: cstr!("realtek,rt1305") },
	of_device_id { compatible: cstr!("realtek,rt1306") },
	of_device_id { compatible: ptr::null() },
];

// CONFIG_ACPI conditional device table.
static RT1305_ACPI_MATCH: [acpi_device_id; 3] = [
	acpi_device_id { id: cstr!("10EC1305") },
	acpi_device_id { id: cstr!("10EC1306") },
	acpi_device_id { id: ptr::null() },
];

static RT1305_I2C_ID: [i2c_device_id; 3] = [
	i2c_device_id { name: cstr!("rt1305") },
	i2c_device_id { name: cstr!("rt1306") },
	i2c_device_id { name: ptr::null() },
];

unsafe fn do_div(n: &mut c_ulonglong, base: c_uint) -> c_uint {
	let rem = (*n % base as c_ulonglong) as c_uint;
	*n /= base as c_ulonglong;
	rem
}

unsafe extern "C" fn rt1305_calibrate(rt1305: *mut rt1305_priv) {
	let mut valmsb: c_uint = 0;
	let mut vallsb: c_uint = 0;
	let mut offsetl: c_uint;
	let mut offsetr: c_uint;
	let mut rh: c_uint = 0;
	let mut rl: c_uint = 0;
	let mut rhl: c_uint;
	let mut r0ohm: c_uint;
	let mut r0l: c_ulonglong;
	let mut r0r: c_ulonglong;

	regcache_cache_bypass((*rt1305).regmap, true);
	rt1305_reset((*rt1305).regmap);
	regmap_write((*rt1305).regmap, RT1305_ADC_SET_3, 0x0219);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xcf, 0x5548);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xc1, 0x0320);
	regmap_write((*rt1305).regmap, RT1305_CLOCK_DETECT, 0x1000);
	regmap_write((*rt1305).regmap, RT1305_CLK_1, 0x0600);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xffd0);
	regmap_write((*rt1305).regmap, RT1305_EFUSE_1, 0x0080);
	regmap_write((*rt1305).regmap, RT1305_EFUSE_1, 0x0880);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_1, 0x0dfe);

	/* Sin Gen */
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x5d, 0x0442);

	regmap_write((*rt1305).regmap, RT1305_CAL_EFUSE_CLOCK, 0xb000);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xc3, 0xd4a0);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xcc, 0x00cc);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xc1, 0x0320);
	regmap_write((*rt1305).regmap, RT1305_POWER_STATUS, 0x0000);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_2, 0xffff);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfc20);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x06, 0x00c0);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfca0);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfce0);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfcf0);

	/* EFUSE read */
	regmap_write((*rt1305).regmap, RT1305_EFUSE_1, 0x0080);
	regmap_write((*rt1305).regmap, RT1305_EFUSE_1, 0x0880);
	regmap_write((*rt1305).regmap, RT1305_EFUSE_1, 0x0880);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfce0);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfca0);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfc20);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x06, 0x0000);
	regmap_write((*rt1305).regmap, RT1305_EFUSE_1, 0x0000);

	regmap_read((*rt1305).regmap, RT1305_DAC_OFFSET_5, &mut valmsb);
	regmap_read((*rt1305).regmap, RT1305_DAC_OFFSET_6, &mut vallsb);
	offsetl = (valmsb << 16) | vallsb;
	regmap_read((*rt1305).regmap, RT1305_DAC_OFFSET_7, &mut valmsb);
	regmap_read((*rt1305).regmap, RT1305_DAC_OFFSET_8, &mut vallsb);
	offsetr = (valmsb << 16) | vallsb;
	pr_info(cstr!("DC offsetl=0x%x, offsetr=0x%x\n"), offsetl, offsetr);

	/* R0 calibration */
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x5d, 0x9542);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0xfcf0);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_2, 0xffff);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_1, 0x1dfe);
	regmap_write((*rt1305).regmap, RT1305_SILENCE_DETECT, 0x0e13);
	regmap_write((*rt1305).regmap, RT1305_CLK_1, 0x0650);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x50, 0x0064);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x51, 0x0770);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x52, 0xc30c);
	regmap_write((*rt1305).regmap, RT1305_SPK_TEMP_PROTECTION_1, 0x8200);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xd4, 0xfb00);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xd4, 0xff80);
	msleep(2000);
	regmap_read((*rt1305).regmap, RT1305_PR_BASE + 0x55, &mut rh);
	regmap_read((*rt1305).regmap, RT1305_PR_BASE + 0x56, &mut rl);
	rhl = (rh << 16) | rl;
	r0ohm = (rhl * 10) / 33554432;
	pr_debug(cstr!("Left_rhl = 0x%x rh=0x%x rl=0x%x\n"), rhl, rh, rl);
	pr_info(cstr!("Left channel %d.%dohm\n"), r0ohm / 10, r0ohm % 10);
	r0l = 562949953421312u64;
	if rhl != 0 {
		do_div(&mut r0l, rhl);
	}
	pr_debug(cstr!("Left_r0 = 0x%llx\n"), r0l);

	regmap_write((*rt1305).regmap, RT1305_SPK_TEMP_PROTECTION_1, 0x9200);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xd4, 0xfb00);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xd4, 0xff80);
	msleep(2000);
	regmap_read((*rt1305).regmap, RT1305_PR_BASE + 0x55, &mut rh);
	regmap_read((*rt1305).regmap, RT1305_PR_BASE + 0x56, &mut rl);
	rhl = (rh << 16) | rl;
	r0ohm = (rhl * 10) / 33554432;
	pr_debug(cstr!("Right_rhl = 0x%x rh=0x%x rl=0x%x\n"), rhl, rh, rl);
	pr_info(cstr!("Right channel %d.%dohm\n"), r0ohm / 10, r0ohm % 10);
	r0r = 562949953421312u64;
	if rhl != 0 {
		do_div(&mut r0r, rhl);
	}
	pr_debug(cstr!("Right_r0 = 0x%llx\n"), r0r);

	regmap_write((*rt1305).regmap, RT1305_SPK_TEMP_PROTECTION_1, 0xc2ec);
	if (r0l > R0_UPPER) && (r0l < R0_LOWER) && (r0r > R0_UPPER) && (r0r < R0_LOWER) {
		regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x4e, ((r0l >> 16) & 0xffff) as c_uint);
		regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x4f, (r0l & 0xffff) as c_uint);
		regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xfe, (((r0r >> 16) & 0xffff) as c_uint) | 0xf800);
		regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0xfd, (r0r & 0xffff) as c_uint);
	} else {
		pr_err(cstr!("R0 calibration failed\n"));
	}

	/* restore some registers */
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_1, 0x0dfe);
	usleep_range(200000, 400000);
	regmap_write((*rt1305).regmap, RT1305_PR_BASE + 0x5d, 0x0442);
	regmap_write((*rt1305).regmap, RT1305_CLOCK_DETECT, 0x3000);
	regmap_write((*rt1305).regmap, RT1305_CLK_1, 0x0400);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_1, 0x0000);
	regmap_write((*rt1305).regmap, RT1305_CAL_EFUSE_CLOCK, 0x8000);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_2, 0x1020);
	regmap_write((*rt1305).regmap, RT1305_POWER_CTRL_3, 0x0000);
	regcache_cache_bypass((*rt1305).regmap, false);
}

unsafe extern "C" fn rt1305_i2c_probe(i2c: *mut i2c_client) -> c_int {
	let rt1305: *mut rt1305_priv;
	let ret: c_int;
	let mut val: c_uint = 0;

	rt1305 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<rt1305_priv>(), GFP_KERNEL) as *mut rt1305_priv;
	if rt1305.is_null() {
		return -ENOMEM;
	}
	i2c_set_clientdata(i2c, rt1305 as *mut c_void);
	(*rt1305).regmap = devm_regmap_init_i2c(i2c, &RT1305_REGMAP);
	if IS_ERR((*rt1305).regmap as *const c_void) {
		ret = PTR_ERR((*rt1305).regmap as *const c_void);
		dev_err(&mut (*i2c).dev, cstr!("Failed to allocate register map: %d\n"), ret);
		return ret;
	}

	regmap_read((*rt1305).regmap, RT1305_DEVICE_ID, &mut val);
	if val != RT1305_DEVICE_ID_NUM {
		dev_err(&mut (*i2c).dev, cstr!("Device with ID register %x is not rt1305\n"), val);
		return -ENODEV;
	}

	rt1305_reset((*rt1305).regmap);
	rt1305_calibrate(rt1305);
	devm_snd_soc_register_component(&mut (*i2c).dev, &SOC_COMPONENT_DEV_RT1305, RT1305_DAI.as_mut_ptr(), RT1305_DAI.len() as c_int)
}

unsafe extern "C" fn rt1305_i2c_shutdown(client: *mut i2c_client) {
	let rt1305 = i2c_get_clientdata(client) as *mut rt1305_priv;
	rt1305_reset((*rt1305).regmap);
}

static mut RT1305_I2C_DRIVER: i2c_driver = i2c_driver {
	driver: device_driver {
		name: cstr!("rt1305"),
		of_match_table: RT1305_OF_MATCH.as_ptr(),
		acpi_match_table: RT1305_ACPI_MATCH.as_ptr(),
	},
	probe: Some(rt1305_i2c_probe),
	shutdown: Some(rt1305_i2c_shutdown),
	id_table: RT1305_I2C_ID.as_ptr(),
};

// module_i2c_driver(rt1305_i2c_driver);
// MODULE_DESCRIPTION("ASoC RT1305 amplifier driver");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
