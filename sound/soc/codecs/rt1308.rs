// SPDX-License-Identifier: GPL-2.0
//
// rt1308.c  --  RT1308 ALSA SoC amplifier component driver
//
// Copyright 2019 Realtek Semiconductor Corp.
// Author: Derek Fang <derek.fang@realtek.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

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
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
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
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
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
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
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
pub struct driver_desc {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_desc,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct rt1308_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: c_int,
    pub bclk: c_int,
    pub master: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
}

extern "C" {
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_NOPM: c_int;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;

    static RT1308_RESET: c_uint;
    static RT1308_RESET_N: c_uint;
    static RT1308_CLK_GATING: c_uint;
    static RT1308_CLK_1: c_uint;
    static RT1308_CLK_2: c_uint;
    static RT1308_I2S_SET_1: c_uint;
    static RT1308_I2S_SET_2: c_uint;
    static RT1308_I2C_I2S_SDW_SET: c_uint;
    static RT1308_DC_DET_THRES: c_uint;
    static RT1308_SIL_DET: c_uint;
    static RT1308_CLK_DET: c_uint;
    static RT1308_DC_DET: c_uint;
    static RT1308_DAC_SET: c_uint;
    static RT1308_DAC_BUF: c_uint;
    static RT1308_AD_FILTER_SET: c_uint;
    static RT1308_DC_CAL_1: c_uint;
    static RT1308_DC_CAL_2: c_uint;
    static RT1308_POWER_STATUS: c_uint;
    static RT1308_POWER_INT: c_uint;
    static RT1308_SINE_TONE_GEN_1: c_uint;
    static RT1308_SINE_TONE_GEN_2: c_uint;
    static RT1308_BQ_SET: c_uint;
    static RT1308_BQ_PARA_UPDATE: c_uint;
    static RT1308_BQ_PRE_VOL_L: c_uint;
    static RT1308_BQ_POST_VOL_R: c_uint;
    static RT1308_BQ1_L_H0: c_uint;
    static RT1308_BQ2_R_A2: c_uint;
    static RT1308_VEN_DEV_ID: c_uint;
    static RT1308_VERSION_ID: c_uint;
    static RT1308_SPK_BOUND: c_uint;
    static RT1308_BQ1_EQ_L_1: c_uint;
    static RT1308_BQ2_EQ_R_3: c_uint;
    static RT1308_EFUSE_1: c_uint;
    static RT1308_EFUSE_READ_PVDD_L: c_uint;
    static RT1308_EFUSE_READ_PVDD_R: c_uint;
    static RT1308_EFUSE_READ_PVDD_PTBL: c_uint;
    static RT1308_EFUSE_READ_DEV: c_uint;
    static RT1308_EFUSE_READ_R0: c_uint;
    static RT1308_EFUSE_READ_ADC_L: c_uint;
    static RT1308_EFUSE_READ_ADC_R: c_uint;
    static RT1308_EFUSE_READ_ADC_PBTL: c_uint;
    static RT1308_EFUSE_RESERVE: c_uint;
    static RT1308_EFUSE_DATA_0_MSB: c_uint;
    static RT1308_EFUSE_DATA_0_LSB: c_uint;
    static RT1308_EFUSE_DATA_1_MSB: c_uint;
    static RT1308_EFUSE_DATA_1_LSB: c_uint;
    static RT1308_EFUSE_DATA_2_MSB: c_uint;
    static RT1308_EFUSE_DATA_2_LSB: c_uint;
    static RT1308_EFUSE_DATA_3_MSB: c_uint;
    static RT1308_EFUSE_DATA_3_LSB: c_uint;
    static RT1308_EFUSE_STATUS_1: c_uint;
    static RT1308_EFUSE_STATUS_2: c_uint;
    static RT1308_DUMMY_REG: c_uint;
    static RT1308_PADS_1: c_uint;
    static RT1308_PADS_2: c_uint;
    static RT1308_TEST_MODE: c_uint;
    static RT1308_TEST_1: c_uint;
    static RT1308_TEST_2: c_uint;
    static RT1308_TEST_3: c_uint;
    static RT1308_TEST_4: c_uint;
    static RT1308_TCON_1: c_uint;
    static RT1308_TCON_2: c_uint;
    static RT1308_MAX_REG: c_uint;
    static RT1308_CLASS_D_SET_2: c_uint;
    static RT1308_VREF: c_uint;
    static RT1308_IV_SENSE: c_uint;
    static RT1308_DATA_PATH: c_uint;
    static RT1308_POWER: c_uint;
    static RT1308_PVDD_OFFSET_CTL: c_uint;
    static RT1308_CAL_OFFSET_DAC_PBTL: c_uint;
    static RT1308_CAL_OFFSET_DAC_L: c_uint;
    static RT1308_CAL_OFFSET_DAC_R: c_uint;
    static RT1308_CAL_OFFSET_PWM_L: c_uint;
    static RT1308_CAL_OFFSET_PWM_R: c_uint;
    static RT1308_CAL_PWM_VOS_ADC_L: c_uint;
    static RT1308_CAL_PWM_VOS_ADC_R: c_uint;
    static RT1308_MBIAS: c_uint;
    static RT1308_SDW_REG_RDATA: c_uint;
    static RT1308_PLL_1: c_uint;
    static RT1308_DEVICE_ID_NUM: c_uint;

    static RT1308_POW_PDB_REG_BIT: c_uint;
    static RT1308_POW_PDB_MN_BIT: c_uint;
    static RT1308_DVOL_MUTE_L_EN_SFT: c_uint;
    static RT1308_DVOL_MUTE_R_EN_SFT: c_uint;
    static RT1308_POW_MBIAS20U_BIT: c_uint;
    static RT1308_POW_ALDO_BIT: c_uint;
    static RT1308_POW_DBG_BIT: c_uint;
    static RT1308_POW_DACL_BIT: c_uint;
    static RT1308_POW_CLK25M_BIT: c_uint;
    static RT1308_POW_ADC_R_BIT: c_uint;
    static RT1308_POW_ADC_L_BIT: c_uint;
    static RT1308_POW_DLDO_BIT: c_uint;
    static RT1308_POW_VREF_BIT: c_uint;
    static RT1308_POW_MIXER_R_BIT: c_uint;
    static RT1308_POW_MIXER_L_BIT: c_uint;
    static RT1308_POW_MBIAS4U_BIT: c_uint;
    static RT1308_POW_PLL2_LDO_EN_BIT: c_uint;
    static RT1308_POW_PLL2B_EN_BIT: c_uint;
    static RT1308_POW_PLL2F_EN_BIT: c_uint;
    static RT1308_POW_PLL2F2_EN_BIT: c_uint;
    static RT1308_POW_PLL2B2_EN_BIT: c_uint;
    static RT1308_POW_DAC1_BIT: c_uint;
    static RT1308_DIV_FS_SYS_MASK: c_uint;
    static RT1308_DIV_FS_SYS_SFT: c_uint;
    static RT1308_I2S_DL_SEL_16B: c_uint;
    static RT1308_I2S_DL_SEL_20B: c_uint;
    static RT1308_I2S_DL_SEL_24B: c_uint;
    static RT1308_I2S_DL_SEL_8B: c_uint;
    static RT1308_I2S_DL_SEL_MASK: c_uint;
    static RT1308_I2S_DF_SEL_LEFT: c_uint;
    static RT1308_I2S_DF_SEL_PCM_A: c_uint;
    static RT1308_I2S_DF_SEL_PCM_B: c_uint;
    static RT1308_I2S_DF_SEL_MASK: c_uint;
    static RT1308_I2S_BCLK_INV: c_uint;
    static RT1308_I2S_BCLK_MASK: c_uint;
    static RT1308_FS_SYS_S_MCLK: c_int;
    static RT1308_FS_SYS_S_BCLK: c_int;
    static RT1308_FS_SYS_S_PLL: c_int;
    static RT1308_FS_SYS_S_RCCLK: c_int;
    static RT1308_SEL_FS_SYS_SRC_MCLK: c_uint;
    static RT1308_SEL_FS_SYS_SRC_BCLK: c_uint;
    static RT1308_SEL_FS_SYS_SRC_PLL: c_uint;
    static RT1308_SEL_FS_SYS_SRC_RCCLK: c_uint;
    static RT1308_SEL_FS_SYS_MASK: c_uint;
    static RT1308_MCLK_DET_EN_MASK: c_uint;
    static RT1308_MCLK_DET_EN: c_uint;
    static RT1308_PLL_S_MCLK: c_int;
    static RT1308_PLL_S_BCLK: c_int;
    static RT1308_PLL_S_RCCLK: c_int;
    static RT1308_SEL_PLL_SRC_MASK: c_uint;
    static RT1308_SEL_PLL_SRC_MCLK: c_uint;
    static RT1308_SEL_PLL_SRC_BCLK: c_uint;
    static RT1308_SEL_PLL_SRC_RCCLK: c_uint;
    static RT1308_PLL1_K_SFT: c_uint;
    static RT1308_PLL1_M_BYPASS_SFT: c_uint;
    static RT1308_PLL1_M_SFT: c_uint;
    static RT1308_PLL1_N_SFT: c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regmap_multi_reg_write(
        map: *mut regmap,
        regs: *const reg_sequence,
        num_regs: c_int,
    ) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, pll_code: *mut rl6231_pll_code)
        -> c_int;
    fn snd_soc_component_write(
        component: *mut snd_soc_component,
        reg: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn in_range(reg: c_uint, first: c_uint, last: c_uint) -> bool {
    reg >= first && reg <= last
}

static init_list: [reg_sequence; 11] = unsafe {
    [
        reg_sequence { reg: RT1308_I2C_I2S_SDW_SET, def: 0x01014005 },
        reg_sequence { reg: RT1308_CLASS_D_SET_2, def: 0x227f5501 },
        reg_sequence { reg: RT1308_PADS_1, def: 0x50150505 },
        reg_sequence { reg: RT1308_VREF, def: 0x18100000 },
        reg_sequence { reg: RT1308_IV_SENSE, def: 0x87010000 },
        reg_sequence { reg: RT1308_DUMMY_REG, def: 0x00000200 },
        reg_sequence { reg: RT1308_SIL_DET, def: 0xe1c30000 },
        reg_sequence { reg: RT1308_DC_CAL_2, def: 0x00ffff00 },
        reg_sequence { reg: RT1308_CLK_DET, def: 0x01000000 },
        reg_sequence { reg: RT1308_POWER_STATUS, def: 0x08800000 },
        reg_sequence { reg: RT1308_DAC_SET, def: 0xafaf0700 },
    ]
};
const RT1308_INIT_REG_LEN: c_int = init_list.len() as c_int;

static rt1308_reg: [reg_default; 132] = [
    reg_default { reg: 0x01, def: 0x1f3f5f00 },
    reg_default { reg: 0x02, def: 0x07000000 },
    reg_default { reg: 0x03, def: 0x80003e00 },
    reg_default { reg: 0x04, def: 0x80800600 },
    reg_default { reg: 0x05, def: 0x0aaa1a0a },
    reg_default { reg: 0x06, def: 0x52000000 },
    reg_default { reg: 0x07, def: 0x00000000 },
    reg_default { reg: 0x08, def: 0x00600000 },
    reg_default { reg: 0x09, def: 0xe1030000 },
    reg_default { reg: 0x0a, def: 0x00000000 },
    reg_default { reg: 0x0b, def: 0x30000000 },
    reg_default { reg: 0x0c, def: 0x7fff7000 },
    reg_default { reg: 0x10, def: 0xffff0700 },
    reg_default { reg: 0x11, def: 0x0a000000 },
    reg_default { reg: 0x12, def: 0x60040000 },
    reg_default { reg: 0x13, def: 0x00000000 },
    reg_default { reg: 0x14, def: 0x0f300000 },
    reg_default { reg: 0x15, def: 0x00000022 },
    reg_default { reg: 0x16, def: 0x02000000 },
    reg_default { reg: 0x17, def: 0x01004045 },
    reg_default { reg: 0x18, def: 0x00000000 },
    reg_default { reg: 0x19, def: 0x00000000 },
    reg_default { reg: 0x1a, def: 0x80000000 },
    reg_default { reg: 0x1b, def: 0x10325476 },
    reg_default { reg: 0x1c, def: 0x1d1d0000 },
    reg_default { reg: 0x20, def: 0xd2101300 },
    reg_default { reg: 0x21, def: 0xf3ffff00 },
    reg_default { reg: 0x22, def: 0x00000000 },
    reg_default { reg: 0x23, def: 0x00000000 },
    reg_default { reg: 0x24, def: 0x00000000 },
    reg_default { reg: 0x25, def: 0x00000000 },
    reg_default { reg: 0x26, def: 0x00000000 },
    reg_default { reg: 0x27, def: 0x00000000 },
    reg_default { reg: 0x28, def: 0x00000000 },
    reg_default { reg: 0x29, def: 0x00000000 },
    reg_default { reg: 0x2a, def: 0x00000000 },
    reg_default { reg: 0x2b, def: 0x00000000 },
    reg_default { reg: 0x2c, def: 0x00000000 },
    reg_default { reg: 0x2d, def: 0x00000000 },
    reg_default { reg: 0x2e, def: 0x00000000 },
    reg_default { reg: 0x2f, def: 0x00000000 },
    reg_default { reg: 0x30, def: 0x01000000 },
    reg_default { reg: 0x31, def: 0x20025501 },
    reg_default { reg: 0x32, def: 0x00000000 },
    reg_default { reg: 0x33, def: 0x105a0000 },
    reg_default { reg: 0x34, def: 0x10100000 },
    reg_default { reg: 0x35, def: 0x2aaa52aa },
    reg_default { reg: 0x36, def: 0x00c00000 },
    reg_default { reg: 0x37, def: 0x20046100 },
    reg_default { reg: 0x50, def: 0x10022f00 },
    reg_default { reg: 0x51, def: 0x003c0000 },
    reg_default { reg: 0x54, def: 0x04000000 },
    reg_default { reg: 0x55, def: 0x01000000 },
    reg_default { reg: 0x56, def: 0x02000000 },
    reg_default { reg: 0x57, def: 0x02000000 },
    reg_default { reg: 0x58, def: 0x02000000 },
    reg_default { reg: 0x59, def: 0x02000000 },
    reg_default { reg: 0x5b, def: 0x02000000 },
    reg_default { reg: 0x5c, def: 0x00000000 },
    reg_default { reg: 0x5d, def: 0x00000000 },
    reg_default { reg: 0x5e, def: 0x00000000 },
    reg_default { reg: 0x5f, def: 0x00000000 },
    reg_default { reg: 0x60, def: 0x02000000 },
    reg_default { reg: 0x61, def: 0x00000000 },
    reg_default { reg: 0x62, def: 0x00000000 },
    reg_default { reg: 0x63, def: 0x00000000 },
    reg_default { reg: 0x64, def: 0x00000000 },
    reg_default { reg: 0x65, def: 0x02000000 },
    reg_default { reg: 0x66, def: 0x00000000 },
    reg_default { reg: 0x67, def: 0x00000000 },
    reg_default { reg: 0x68, def: 0x00000000 },
    reg_default { reg: 0x69, def: 0x00000000 },
    reg_default { reg: 0x6a, def: 0x02000000 },
    reg_default { reg: 0x6c, def: 0x00000000 },
    reg_default { reg: 0x6d, def: 0x00000000 },
    reg_default { reg: 0x6e, def: 0x00000000 },
    reg_default { reg: 0x70, def: 0x10EC1308 },
    reg_default { reg: 0x71, def: 0x00000000 },
    reg_default { reg: 0x72, def: 0x00000000 },
    reg_default { reg: 0x73, def: 0x00000000 },
    reg_default { reg: 0x74, def: 0x00000000 },
    reg_default { reg: 0x75, def: 0x00000000 },
    reg_default { reg: 0x76, def: 0x00000000 },
    reg_default { reg: 0x77, def: 0x00000000 },
    reg_default { reg: 0x78, def: 0x00000000 },
    reg_default { reg: 0x79, def: 0x00000000 },
    reg_default { reg: 0x7a, def: 0x00000000 },
    reg_default { reg: 0x7b, def: 0x00000000 },
    reg_default { reg: 0x7c, def: 0x00000000 },
    reg_default { reg: 0x7d, def: 0x00000000 },
    reg_default { reg: 0x7e, def: 0x00000000 },
    reg_default { reg: 0x7f, def: 0x00020f00 },
    reg_default { reg: 0x80, def: 0x00000000 },
    reg_default { reg: 0x81, def: 0x00000000 },
    reg_default { reg: 0x82, def: 0x00000000 },
    reg_default { reg: 0x83, def: 0x00000000 },
    reg_default { reg: 0x84, def: 0x00000000 },
    reg_default { reg: 0x85, def: 0x00000000 },
    reg_default { reg: 0x86, def: 0x00000000 },
    reg_default { reg: 0x87, def: 0x00000000 },
    reg_default { reg: 0x88, def: 0x00000000 },
    reg_default { reg: 0x89, def: 0x00000000 },
    reg_default { reg: 0x8a, def: 0x00000000 },
    reg_default { reg: 0x8b, def: 0x00000000 },
    reg_default { reg: 0x8c, def: 0x00000000 },
    reg_default { reg: 0x8d, def: 0x00000000 },
    reg_default { reg: 0x8e, def: 0x00000000 },
    reg_default { reg: 0x90, def: 0x50250905 },
    reg_default { reg: 0x91, def: 0x15050000 },
    reg_default { reg: 0xa0, def: 0x00000000 },
    reg_default { reg: 0xa1, def: 0x00000000 },
    reg_default { reg: 0xa2, def: 0x00000000 },
    reg_default { reg: 0xa3, def: 0x00000000 },
    reg_default { reg: 0xa4, def: 0x00000000 },
    reg_default { reg: 0xb0, def: 0x00000000 },
    reg_default { reg: 0xb1, def: 0x00000000 },
    reg_default { reg: 0xb2, def: 0x00000000 },
    reg_default { reg: 0xb3, def: 0x00000000 },
    reg_default { reg: 0xb4, def: 0x00000000 },
    reg_default { reg: 0xb5, def: 0x00000000 },
    reg_default { reg: 0xb6, def: 0x00000000 },
    reg_default { reg: 0xb7, def: 0x00000000 },
    reg_default { reg: 0xb8, def: 0x00000000 },
    reg_default { reg: 0xb9, def: 0x00000000 },
    reg_default { reg: 0xba, def: 0x00000000 },
    reg_default { reg: 0xbb, def: 0x00000000 },
    reg_default { reg: 0xc0, def: 0x01000000 },
    reg_default { reg: 0xc1, def: 0x00000000 },
    reg_default { reg: 0xf0, def: 0x00000000 },
];

unsafe extern "C" fn rt1308_reg_init(component: *mut snd_soc_component) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    regmap_multi_reg_write((*rt1308).regmap, init_list.as_ptr(), RT1308_INIT_REG_LEN)
}

unsafe extern "C" fn rt1308_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == RT1308_RESET
        || reg == RT1308_RESET_N
        || reg == RT1308_CLK_2
        || reg == RT1308_SIL_DET
        || reg == RT1308_CLK_DET
        || reg == RT1308_DC_DET
        || reg == RT1308_DAC_SET
        || reg == RT1308_DAC_BUF
        || reg == RT1308_SDW_REG_RDATA
        || reg == RT1308_DC_CAL_1
        || reg == RT1308_PVDD_OFFSET_CTL
        || reg == RT1308_CAL_OFFSET_DAC_PBTL
        || reg == RT1308_CAL_OFFSET_DAC_L
        || reg == RT1308_CAL_OFFSET_DAC_R
        || reg == RT1308_CAL_OFFSET_PWM_L
        || reg == RT1308_CAL_OFFSET_PWM_R
        || reg == RT1308_CAL_PWM_VOS_ADC_L
        || reg == RT1308_CAL_PWM_VOS_ADC_R
        || reg == RT1308_MBIAS
        || reg == RT1308_POWER_STATUS
        || reg == RT1308_POWER_INT
        || reg == RT1308_SINE_TONE_GEN_2
        || reg == RT1308_BQ_SET
        || reg == RT1308_BQ_PARA_UPDATE
        || reg == RT1308_VEN_DEV_ID
        || reg == RT1308_VERSION_ID
        || reg == RT1308_EFUSE_1
        || reg == RT1308_EFUSE_READ_PVDD_L
        || reg == RT1308_EFUSE_READ_PVDD_R
        || reg == RT1308_EFUSE_READ_PVDD_PTBL
        || reg == RT1308_EFUSE_READ_DEV
        || reg == RT1308_EFUSE_READ_R0
        || reg == RT1308_EFUSE_READ_ADC_L
        || reg == RT1308_EFUSE_READ_ADC_R
        || reg == RT1308_EFUSE_READ_ADC_PBTL
        || reg == RT1308_EFUSE_RESERVE
        || reg == RT1308_EFUSE_DATA_0_MSB
        || reg == RT1308_EFUSE_DATA_0_LSB
        || reg == RT1308_EFUSE_DATA_1_MSB
        || reg == RT1308_EFUSE_DATA_1_LSB
        || reg == RT1308_EFUSE_DATA_2_MSB
        || reg == RT1308_EFUSE_DATA_2_LSB
        || reg == RT1308_EFUSE_DATA_3_MSB
        || reg == RT1308_EFUSE_DATA_3_LSB
        || reg == RT1308_EFUSE_STATUS_1
        || reg == RT1308_EFUSE_STATUS_2
        || reg == RT1308_DUMMY_REG
}

unsafe extern "C" fn rt1308_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == RT1308_RESET
        || reg == RT1308_RESET_N
        || in_range(reg, RT1308_CLK_GATING, RT1308_DC_DET_THRES)
        || in_range(reg, RT1308_DAC_SET, RT1308_AD_FILTER_SET)
        || in_range(reg, RT1308_DC_CAL_1, RT1308_POWER_INT)
        || reg == RT1308_SINE_TONE_GEN_1
        || reg == RT1308_SINE_TONE_GEN_2
        || reg == RT1308_BQ_SET
        || reg == RT1308_BQ_PARA_UPDATE
        || in_range(reg, RT1308_BQ_PRE_VOL_L, RT1308_BQ_POST_VOL_R)
        || in_range(reg, RT1308_BQ1_L_H0, RT1308_BQ2_R_A2)
        || reg == RT1308_VEN_DEV_ID
        || reg == RT1308_VERSION_ID
        || reg == RT1308_SPK_BOUND
        || in_range(reg, RT1308_BQ1_EQ_L_1, RT1308_BQ2_EQ_R_3)
        || in_range(reg, RT1308_EFUSE_1, RT1308_EFUSE_RESERVE)
        || reg == RT1308_PADS_1
        || reg == RT1308_PADS_2
        || reg == RT1308_TEST_MODE
        || reg == RT1308_TEST_1
        || reg == RT1308_TEST_2
        || reg == RT1308_TEST_3
        || reg == RT1308_TEST_4
        || in_range(reg, RT1308_EFUSE_DATA_0_MSB, RT1308_EFUSE_STATUS_2)
        || reg == RT1308_TCON_1
        || reg == RT1308_TCON_2
        || reg == RT1308_DUMMY_REG
        || reg == RT1308_MAX_REG
}

unsafe extern "C" fn rt1308_classd_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_POST_PMU {
        msleep(30);
        snd_soc_component_update_bits(
            component,
            RT1308_POWER_STATUS,
            RT1308_POW_PDB_REG_BIT | RT1308_POW_PDB_MN_BIT,
            RT1308_POW_PDB_REG_BIT | RT1308_POW_PDB_MN_BIT,
        );
        msleep(40);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_update_bits(
            component,
            RT1308_POWER_STATUS,
            RT1308_POW_PDB_REG_BIT | RT1308_POW_PDB_MN_BIT,
            0,
        );
        usleep_range(150000, 200000);
    }

    0
}

static rt1308_rx_data_ch_select: [*const c_char; 4] =
    [cstr!("LR"), cstr!("LL"), cstr!("RL"), cstr!("RR")];

// static SOC_ENUM_SINGLE_DECL(rt1308_rx_data_ch_enum, RT1308_DATA_PATH, 24,
//     rt1308_rx_data_ch_select);
// static const struct snd_kcontrol_new rt1308_snd_controls[] = {
//     SOC_ENUM("RX Channel Select", rt1308_rx_data_ch_enum),
// };
static rt1308_snd_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];

// static const struct snd_kcontrol_new rt1308_sto_dac_l =
//     SOC_DAPM_SINGLE("Switch", RT1308_DAC_SET, RT1308_DVOL_MUTE_L_EN_SFT, 1, 1);
static rt1308_sto_dac_l: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// static const struct snd_kcontrol_new rt1308_sto_dac_r =
//     SOC_DAPM_SINGLE("Switch", RT1308_DAC_SET, RT1308_DVOL_MUTE_R_EN_SFT, 1, 1);
static rt1308_sto_dac_r: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// DAPM widgets are created by kernel macros in C:
// SND_SOC_DAPM_AIF_IN, SND_SOC_DAPM_SUPPLY, SND_SOC_DAPM_DAC,
// SND_SOC_DAPM_SWITCH, SND_SOC_DAPM_PGA_E, and SND_SOC_DAPM_OUTPUT.
static rt1308_dapm_widgets: [snd_soc_dapm_widget_desc; 24] =
    [snd_soc_dapm_widget_desc { _private: [] }; 24];

static rt1308_dapm_routes: [snd_soc_dapm_route; 27] = [
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("AIF1RX") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("MBIAS20U") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("ALDO") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("DBG") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("DACL") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("CLK25M") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("ADC_R") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("ADC_L") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("DLDO") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("VREF") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("MIXER_R") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("MIXER_L") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("MBIAS4U") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("PLL2_LDO") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("PLL2B") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("PLL2F") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("PLL2F2") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("PLL2B2") },
    snd_soc_dapm_route { sink: cstr!("DAC L"), control: cstr!("Switch"), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("DAC R"), control: cstr!("Switch"), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("DAC L"), control: ptr::null(), source: cstr!("DAC Power") },
    snd_soc_dapm_route { sink: cstr!("DAC R"), control: ptr::null(), source: cstr!("DAC Power") },
    snd_soc_dapm_route { sink: cstr!("CLASS D"), control: ptr::null(), source: cstr!("DAC L") },
    snd_soc_dapm_route { sink: cstr!("CLASS D"), control: ptr::null(), source: cstr!("DAC R") },
    snd_soc_dapm_route { sink: cstr!("SPOL"), control: ptr::null(), source: cstr!("CLASS D") },
    snd_soc_dapm_route { sink: cstr!("SPOR"), control: ptr::null(), source: cstr!("CLASS D") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe extern "C" fn rt1308_get_clk_info(sclk: c_int, mut rate: c_int) -> c_int {
    let pd: [c_int; 8] = [1, 2, 3, 4, 6, 8, 12, 16];

    if sclk <= 0 || rate <= 0 {
        return -EINVAL;
    }

    rate <<= 8;
    for i in 0..pd.len() {
        if sclk == rate * pd[i] {
            return i as c_int;
        }
    }

    -EINVAL
}

unsafe extern "C" fn rt1308_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    let mut val_len: c_uint = 0;
    let val_clk: c_uint;
    let mask_clk: c_uint;
    let pre_div: c_int;
    let bclk_ms: c_int;
    let frame_size: c_int;

    (*rt1308).lrck = params_rate(params);
    pre_div = rt1308_get_clk_info((*rt1308).sysclk, (*rt1308).lrck);
    if pre_div < 0 {
        dev_err((*component).dev, cstr!("Unsupported clock setting %d\n"), (*rt1308).lrck);
        return -EINVAL;
    }

    frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 {
        dev_err((*component).dev, cstr!("Unsupported frame size: %d\n"), frame_size);
        return -EINVAL;
    }

    bclk_ms = (frame_size > 32) as c_int;
    (*rt1308).bclk = (*rt1308).lrck * (32 << bclk_ms);

    dev_dbg(
        (*component).dev,
        cstr!("bclk_ms is %d and pre_div is %d for iis %d\n"),
        bclk_ms,
        pre_div,
        (*dai).id,
    );

    dev_dbg(
        (*component).dev,
        cstr!("lrck is %dHz and pre_div is %d for iis %d\n"),
        (*rt1308).lrck,
        pre_div,
        (*dai).id,
    );

    match params_width(params) {
        16 => val_len |= RT1308_I2S_DL_SEL_16B,
        20 => val_len |= RT1308_I2S_DL_SEL_20B,
        24 => val_len |= RT1308_I2S_DL_SEL_24B,
        8 => val_len |= RT1308_I2S_DL_SEL_8B,
        _ => return -EINVAL,
    }

    if (*dai).id == RT1308_AIF1 {
        mask_clk = RT1308_DIV_FS_SYS_MASK;
        val_clk = (pre_div as c_uint) << RT1308_DIV_FS_SYS_SFT;
        snd_soc_component_update_bits(
            component,
            RT1308_I2S_SET_2,
            RT1308_I2S_DL_SEL_MASK,
            val_len,
        );
    } else {
        dev_err((*component).dev, cstr!("Invalid dai->id: %d\n"), (*dai).id);
        return -EINVAL;
    }

    snd_soc_component_update_bits(component, RT1308_CLK_1, mask_clk, val_clk);

    0
}

extern "C" {
    static RT1308_AIF1: c_int;
}

unsafe extern "C" fn rt1308_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    let mut reg_val: c_uint = 0;
    let mut reg1_val: c_uint = 0;

    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
        (*rt1308).master = 0;
    } else {
        return -EINVAL;
    }

    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_LEFT_J {
        reg_val |= RT1308_I2S_DF_SEL_LEFT;
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
        reg_val |= RT1308_I2S_DF_SEL_PCM_A;
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_B {
        reg_val |= RT1308_I2S_DF_SEL_PCM_B;
    } else {
        return -EINVAL;
    }

    if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_NB_NF {
    } else if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_NF {
        reg1_val |= RT1308_I2S_BCLK_INV;
    } else {
        return -EINVAL;
    }

    if (*dai).id == RT1308_AIF1 {
        snd_soc_component_update_bits(
            component,
            RT1308_I2S_SET_1,
            RT1308_I2S_DF_SEL_MASK,
            reg_val,
        );
        snd_soc_component_update_bits(
            component,
            RT1308_I2S_SET_2,
            RT1308_I2S_BCLK_MASK,
            reg1_val,
        );
    } else {
        dev_err((*component).dev, cstr!("Invalid dai->id: %d\n"), (*dai).id);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn rt1308_set_component_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    let mut reg_val: c_uint = 0;

    if freq as c_int == (*rt1308).sysclk && clk_id == (*rt1308).sysclk_src {
        return 0;
    }

    if clk_id == RT1308_FS_SYS_S_MCLK {
        reg_val |= RT1308_SEL_FS_SYS_SRC_MCLK;
        snd_soc_component_update_bits(
            component,
            RT1308_CLK_DET,
            RT1308_MCLK_DET_EN_MASK,
            RT1308_MCLK_DET_EN,
        );
    } else if clk_id == RT1308_FS_SYS_S_BCLK {
        reg_val |= RT1308_SEL_FS_SYS_SRC_BCLK;
    } else if clk_id == RT1308_FS_SYS_S_PLL {
        reg_val |= RT1308_SEL_FS_SYS_SRC_PLL;
    } else if clk_id == RT1308_FS_SYS_S_RCCLK {
        reg_val |= RT1308_SEL_FS_SYS_SRC_RCCLK;
    } else {
        dev_err((*component).dev, cstr!("Invalid clock id (%d)\n"), clk_id);
        return -EINVAL;
    }
    snd_soc_component_update_bits(component, RT1308_CLK_1, RT1308_SEL_FS_SYS_MASK, reg_val);
    (*rt1308).sysclk = freq as c_int;
    (*rt1308).sysclk_src = clk_id;

    dev_dbg((*component).dev, cstr!("Sysclk is %dHz and clock id is %d\n"), freq, clk_id);

    0
}

unsafe extern "C" fn rt1308_set_component_pll(
    component: *mut snd_soc_component,
    _pll_id: c_int,
    source: c_int,
    mut freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    let mut pll_code = rl6231_pll_code {
        m_bp: 0,
        m_code: 0,
        n_code: 0,
        k_code: 0,
    };
    let ret: c_int;

    if source == (*rt1308).pll_src
        && freq_in as c_int == (*rt1308).pll_in
        && freq_out as c_int == (*rt1308).pll_out
    {
        return 0;
    }

    if freq_in == 0 || freq_out == 0 {
        dev_dbg((*component).dev, cstr!("PLL disabled\n"));

        (*rt1308).pll_in = 0;
        (*rt1308).pll_out = 0;
        snd_soc_component_update_bits(
            component,
            RT1308_CLK_1,
            RT1308_SEL_FS_SYS_MASK,
            RT1308_SEL_FS_SYS_SRC_MCLK,
        );
        return 0;
    }

    if source == RT1308_PLL_S_MCLK {
        snd_soc_component_update_bits(
            component,
            RT1308_CLK_2,
            RT1308_SEL_PLL_SRC_MASK,
            RT1308_SEL_PLL_SRC_MCLK,
        );
        snd_soc_component_update_bits(
            component,
            RT1308_CLK_DET,
            RT1308_MCLK_DET_EN_MASK,
            RT1308_MCLK_DET_EN,
        );
    } else if source == RT1308_PLL_S_BCLK {
        snd_soc_component_update_bits(
            component,
            RT1308_CLK_2,
            RT1308_SEL_PLL_SRC_MASK,
            RT1308_SEL_PLL_SRC_BCLK,
        );
    } else if source == RT1308_PLL_S_RCCLK {
        snd_soc_component_update_bits(
            component,
            RT1308_CLK_2,
            RT1308_SEL_PLL_SRC_MASK,
            RT1308_SEL_PLL_SRC_RCCLK,
        );
        freq_in = 25000000;
    } else {
        dev_err((*component).dev, cstr!("Unknown PLL Source %d\n"), source);
        return -EINVAL;
    }

    ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
    if ret < 0 {
        dev_err((*component).dev, cstr!("Unsupported input clock %d\n"), freq_in);
        return ret;
    }

    dev_dbg(
        (*component).dev,
        cstr!("bypass=%d m=%d n=%d k=%d\n"),
        pll_code.m_bp,
        if pll_code.m_bp != 0 { 0 } else { pll_code.m_code },
        pll_code.n_code,
        pll_code.k_code,
    );

    snd_soc_component_write(
        component,
        RT1308_PLL_1,
        ((pll_code.k_code as c_uint) << RT1308_PLL1_K_SFT)
            | ((pll_code.m_bp as c_uint) << RT1308_PLL1_M_BYPASS_SFT)
            | (((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint)
                << RT1308_PLL1_M_SFT)
            | ((pll_code.n_code as c_uint) << RT1308_PLL1_N_SFT),
    );

    (*rt1308).pll_in = freq_in as c_int;
    (*rt1308).pll_out = freq_out as c_int;
    (*rt1308).pll_src = source;

    0
}

unsafe extern "C" fn rt1308_probe(component: *mut snd_soc_component) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    (*rt1308).component = component;
    rt1308_reg_init(component)
}

unsafe extern "C" fn rt1308_remove(component: *mut snd_soc_component) {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;
    regmap_write((*rt1308).regmap, RT1308_RESET, 0);
}

// #ifdef CONFIG_PM
unsafe extern "C" fn rt1308_suspend(component: *mut snd_soc_component) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;

    regcache_cache_only((*rt1308).regmap, true);
    regcache_mark_dirty((*rt1308).regmap);

    0
}

unsafe extern "C" fn rt1308_resume(component: *mut snd_soc_component) -> c_int {
    let rt1308 = snd_soc_component_get_drvdata(component) as *mut rt1308_priv;

    regcache_cache_only((*rt1308).regmap, false);
    regcache_sync((*rt1308).regmap);

    0
}
// #else
// #define rt1308_suspend NULL
// #define rt1308_resume NULL
// #endif

unsafe fn RT1308_STEREO_RATES() -> c_uint {
    SNDRV_PCM_RATE_48000
}

unsafe fn RT1308_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE
}

static rt1308_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1308_hw_params),
    set_fmt: Some(rt1308_set_dai_fmt),
};

static mut rt1308_dai: [snd_soc_dai_driver; 1] = unsafe {
    [snd_soc_dai_driver {
        name: cstr!("rt1308-aif"),
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("AIF1 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: RT1308_STEREO_RATES(),
            formats: RT1308_FORMATS(),
        },
        ops: &rt1308_aif_dai_ops,
    }]
};

static soc_component_dev_rt1308: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1308_probe),
    remove: Some(rt1308_remove),
    suspend: Some(rt1308_suspend),
    resume: Some(rt1308_resume),
    controls: rt1308_snd_controls.as_ptr(),
    num_controls: rt1308_snd_controls.len() as c_uint,
    dapm_widgets: rt1308_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt1308_dapm_widgets.len() as c_uint,
    dapm_routes: rt1308_dapm_routes.as_ptr(),
    num_dapm_routes: rt1308_dapm_routes.len() as c_uint,
    set_sysclk: Some(rt1308_set_component_sysclk),
    set_pll: Some(rt1308_set_component_pll),
    use_pmdown_time: 1,
    endianness: 1,
};

static rt1308_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 32,
        max_register: RT1308_MAX_REG,
        volatile_reg: Some(rt1308_volatile_register),
        readable_reg: Some(rt1308_readable_register),
        cache_type: REGCACHE_MAPLE,
        reg_defaults: rt1308_reg.as_ptr(),
        num_reg_defaults: rt1308_reg.len() as c_uint,
        use_single_read: true,
        use_single_write: true,
    }
};

// #ifdef CONFIG_OF
static rt1308_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("realtek,rt1308") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, rt1308_of_match);
// #endif

// #ifdef CONFIG_ACPI
static rt1308_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: cstr!("10EC1308") },
    acpi_device_id { id: ptr::null() },
];
// MODULE_DEVICE_TABLE(acpi, rt1308_acpi_match);
// #endif

static rt1308_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: cstr!("rt1308") },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, rt1308_i2c_id);

unsafe extern "C" fn rt1308_efuse(rt1308: *mut rt1308_priv) {
    regmap_write((*rt1308).regmap, RT1308_RESET, 0);

    regmap_write((*rt1308).regmap, RT1308_POWER_STATUS, 0x01800000);
    msleep(100);
    regmap_write((*rt1308).regmap, RT1308_EFUSE_1, 0x44fe0f00);
    msleep(20);
    regmap_write((*rt1308).regmap, RT1308_PVDD_OFFSET_CTL, 0x10000000);
}

unsafe extern "C" fn rt1308_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let rt1308: *mut rt1308_priv;
    let ret: c_int;
    let mut val: c_uint = 0;

    rt1308 = devm_kzalloc(&mut (*i2c).dev, size_of::<rt1308_priv>(), GFP_KERNEL) as *mut rt1308_priv;
    if rt1308.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, rt1308 as *mut c_void);

    (*rt1308).regmap = devm_regmap_init_i2c(i2c, &rt1308_regmap);
    if IS_ERR((*rt1308).regmap as *const c_void) {
        ret = PTR_ERR((*rt1308).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, cstr!("Failed to allocate register map: %d\n"), ret);
        return ret;
    }

    regmap_read((*rt1308).regmap, RT1308_VEN_DEV_ID, &mut val);
    /* ignore last byte difference */
    if (val & 0xFFFFFF00) != RT1308_DEVICE_ID_NUM {
        dev_err(
            &mut (*i2c).dev,
            cstr!("Device with ID register %x is not rt1308\n"),
            val,
        );
        return -ENODEV;
    }

    rt1308_efuse(rt1308);

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_rt1308,
        rt1308_dai.as_mut_ptr(),
        rt1308_dai.len() as c_int,
    )
}

unsafe extern "C" fn rt1308_i2c_shutdown(client: *mut i2c_client) {
    let rt1308 = i2c_get_clientdata(client) as *mut rt1308_priv;
    regmap_write((*rt1308).regmap, RT1308_RESET, 0);
}

static mut rt1308_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_desc {
        name: cstr!("rt1308"),
        of_match_table: rt1308_of_match.as_ptr(),
        acpi_match_table: rt1308_acpi_match.as_ptr(),
    },
    probe: Some(rt1308_i2c_probe),
    shutdown: Some(rt1308_i2c_shutdown),
    id_table: rt1308_i2c_id.as_ptr(),
};
// module_i2c_driver(rt1308_i2c_driver);

// MODULE_DESCRIPTION("ASoC RT1308 amplifier driver");
// MODULE_AUTHOR("Derek Fang <derek.fang@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
