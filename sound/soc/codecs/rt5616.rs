// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5616.rs  --  RT5616 ALSA SoC audio codec driver
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 * Author: Bard Liao <bardliao@realtek.com>
 *
 * Rust source-level translation of rt5616.c.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// Dependencies originally supplied by Linux/ALSA headers and local rt5616/rl6231 headers.

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub id: c_int }
#[repr(C)] pub struct i2c_client { pub dev: device }

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
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route_desc { _private: [u8; 0] }
pub type snd_soc_dapm_widget_item = c_void;
pub type snd_soc_dapm_route_item = c_void;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route_item,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_desc {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_desc,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

pub type snd_soc_bias_level = c_uint;

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn rl6231_get_clk_info(sysclk: c_int, rate: c_int) -> c_int;
    fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, pll_code: *mut rl6231_pll_code) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_uint);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a.len() as c_uint)
    };
}

// External macros/constants from Linux, ALSA SoC, rl6231.h, and rt5616.h.
macro_rules! ext_const { ($name:ident) => { const $name: c_uint = 0; }; }
ext_const!(RT5616_PRIV_INDEX); ext_const!(RT5616_PRIV_DATA); ext_const!(RT5616_RESET);
ext_const!(RT5616_EQ_CTRL1); ext_const!(RT5616_DRC_AGC_1); ext_const!(RT5616_IRQ_CTRL2);
ext_const!(RT5616_INT_IRQ_ST); ext_const!(RT5616_PGM_REG_ARR1); ext_const!(RT5616_PGM_REG_ARR3);
ext_const!(RT5616_VENDOR_ID); ext_const!(RT5616_DEVICE_ID); ext_const!(RT5616_VERSION_ID);
ext_const!(RT5616_HP_VOL); ext_const!(RT5616_LOUT_CTRL1); ext_const!(RT5616_LOUT_CTRL2);
ext_const!(RT5616_IN1_IN2); ext_const!(RT5616_INL1_INR1_VOL); ext_const!(RT5616_DAC1_DIG_VOL);
ext_const!(RT5616_ADC_DIG_VOL); ext_const!(RT5616_ADC_BST_VOL); ext_const!(RT5616_STO1_ADC_MIXER);
ext_const!(RT5616_AD_DA_MIXER); ext_const!(RT5616_STO_DAC_MIXER); ext_const!(RT5616_REC_L1_MIXER);
ext_const!(RT5616_REC_L2_MIXER); ext_const!(RT5616_REC_R1_MIXER); ext_const!(RT5616_REC_R2_MIXER);
ext_const!(RT5616_HPO_MIXER); ext_const!(RT5616_OUT_L1_MIXER); ext_const!(RT5616_OUT_L2_MIXER);
ext_const!(RT5616_OUT_L3_MIXER); ext_const!(RT5616_OUT_R1_MIXER); ext_const!(RT5616_OUT_R2_MIXER);
ext_const!(RT5616_OUT_R3_MIXER); ext_const!(RT5616_LOUT_MIXER); ext_const!(RT5616_PWR_DIG1);
ext_const!(RT5616_PWR_DIG2); ext_const!(RT5616_PWR_ANLG1); ext_const!(RT5616_PWR_ANLG2);
ext_const!(RT5616_PWR_MIXER); ext_const!(RT5616_PWR_VOL); ext_const!(RT5616_I2S1_SDP);
ext_const!(RT5616_ADDA_CLK1); ext_const!(RT5616_ADDA_CLK2); ext_const!(RT5616_GLB_CLK);
ext_const!(RT5616_PLL_CTRL1); ext_const!(RT5616_PLL_CTRL2); ext_const!(RT5616_HP_OVCD);
ext_const!(RT5616_DEPOP_M1); ext_const!(RT5616_DEPOP_M2); ext_const!(RT5616_DEPOP_M3);
ext_const!(RT5616_CHARGE_PUMP); ext_const!(RT5616_PV_DET_SPK_G); ext_const!(RT5616_MICBIAS);
ext_const!(RT5616_A_JD_CTL1); ext_const!(RT5616_A_JD_CTL2); ext_const!(RT5616_EQ_CTRL2);
ext_const!(RT5616_WIND_FILTER); ext_const!(RT5616_DRC_AGC_2); ext_const!(RT5616_DRC_AGC_3);
ext_const!(RT5616_SVOL_ZC); ext_const!(RT5616_JD_CTRL1); ext_const!(RT5616_JD_CTRL2);
ext_const!(RT5616_IRQ_CTRL1); ext_const!(RT5616_GPIO_CTRL1); ext_const!(RT5616_GPIO_CTRL2);
ext_const!(RT5616_GPIO_CTRL3); ext_const!(RT5616_PGM_REG_ARR2); ext_const!(RT5616_PGM_REG_ARR4);
ext_const!(RT5616_PGM_REG_ARR5); ext_const!(RT5616_SCB_FUNC); ext_const!(RT5616_SCB_CTRL);
ext_const!(RT5616_BASE_BACK); ext_const!(RT5616_MP3_PLUS1); ext_const!(RT5616_MP3_PLUS2);
ext_const!(RT5616_ADJ_HPF_CTRL1); ext_const!(RT5616_ADJ_HPF_CTRL2); ext_const!(RT5616_HP_CALIB_AMP_DET);
ext_const!(RT5616_HP_CALIB2); ext_const!(RT5616_SV_ZCD1); ext_const!(RT5616_SV_ZCD2);
ext_const!(RT5616_D_MISC); ext_const!(RT5616_DUMMY2); ext_const!(RT5616_DUMMY3);
ext_const!(RT5616_SCLK_SRC_MASK); ext_const!(RT5616_SCLK_SRC_PLL1); ext_const!(RT5616_L_MUTE_SFT);
ext_const!(RT5616_R_MUTE_SFT); ext_const!(RT5616_VOL_L_SFT); ext_const!(RT5616_VOL_R_SFT);
ext_const!(RT5616_L_VOL_SFT); ext_const!(RT5616_R_VOL_SFT); ext_const!(RT5616_BST_SFT1);
ext_const!(RT5616_BST_SFT2); ext_const!(RT5616_INL_VOL_SFT); ext_const!(RT5616_INR_VOL_SFT);
ext_const!(RT5616_ADC_L_BST_SFT); ext_const!(RT5616_ADC_R_BST_SFT); ext_const!(RT5616_M_STO1_ADC_L1_SFT);
ext_const!(RT5616_M_STO1_ADC_R1_SFT); ext_const!(RT5616_M_ADCMIX_L_SFT); ext_const!(RT5616_M_IF1_DAC_L_SFT);
ext_const!(RT5616_M_ADCMIX_R_SFT); ext_const!(RT5616_M_IF1_DAC_R_SFT); ext_const!(RT5616_M_DAC_L1_MIXL_SFT);
ext_const!(RT5616_M_DAC_R1_MIXL_SFT); ext_const!(RT5616_M_DAC_R1_MIXR_SFT); ext_const!(RT5616_M_DAC_L1_MIXR_SFT);
ext_const!(RT5616_M_IN1_L_RM_L_SFT); ext_const!(RT5616_M_BST2_RM_L_SFT); ext_const!(RT5616_M_BST1_RM_L_SFT);
ext_const!(RT5616_M_IN1_R_RM_R_SFT); ext_const!(RT5616_M_BST2_RM_R_SFT); ext_const!(RT5616_M_BST1_RM_R_SFT);
ext_const!(RT5616_M_BST1_OM_L_SFT); ext_const!(RT5616_M_BST2_OM_L_SFT); ext_const!(RT5616_M_IN1_L_OM_L_SFT);
ext_const!(RT5616_M_RM_L_OM_L_SFT); ext_const!(RT5616_M_DAC_L1_OM_L_SFT); ext_const!(RT5616_M_BST2_OM_R_SFT);
ext_const!(RT5616_M_BST1_OM_R_SFT); ext_const!(RT5616_M_IN1_R_OM_R_SFT); ext_const!(RT5616_M_RM_R_OM_R_SFT);
ext_const!(RT5616_M_DAC_R1_OM_R_SFT); ext_const!(RT5616_M_DAC1_HM_SFT); ext_const!(RT5616_M_HPVOL_HM_SFT);
ext_const!(RT5616_M_DAC_L1_LM_SFT); ext_const!(RT5616_M_DAC_R1_LM_SFT); ext_const!(RT5616_M_OV_L_LM_SFT);
ext_const!(RT5616_M_OV_R_LM_SFT); ext_const!(RT5616_L_MUTE); ext_const!(RT5616_R_MUTE);
ext_const!(RT5616_DEPOP_MASK); ext_const!(RT5616_DEPOP_MAN); ext_const!(RT5616_HP_CP_MASK);
ext_const!(RT5616_HP_SG_MASK); ext_const!(RT5616_HP_CB_MASK); ext_const!(RT5616_HP_CP_PU);
ext_const!(RT5616_HP_SG_DIS); ext_const!(RT5616_HP_CB_PU); ext_const!(RT5616_HP_DCC_INT1);
ext_const!(RT5616_PWR_FV1); ext_const!(RT5616_PWR_FV2); ext_const!(RT5616_PWR_HV_L);
ext_const!(RT5616_PWR_HV_R); ext_const!(RT5616_PWR_HP_L); ext_const!(RT5616_PWR_HP_R);
ext_const!(RT5616_PWR_HA); ext_const!(RT5616_PM_HP_MASK); ext_const!(RT5616_PM_HP_HV);
ext_const!(RT5616_CHOP_DAC_ADC); ext_const!(RT5616_HP_CO_MASK); ext_const!(RT5616_HP_CO_EN);
ext_const!(RT5616_HP_L_SMT_MASK); ext_const!(RT5616_HP_R_SMT_MASK); ext_const!(RT5616_HP_L_SMT_DIS);
ext_const!(RT5616_HP_R_SMT_DIS); ext_const!(RT5616_SMT_TRIG_MASK); ext_const!(RT5616_HP_CD_PD_MASK);
ext_const!(RT5616_HP_CD_PD_EN); ext_const!(RT5616_HP_CO_DIS); ext_const!(RT5616_HP_CP_PD);
ext_const!(RT5616_HP_SG_EN); ext_const!(RT5616_HP_CB_PD); ext_const!(RT5616_SMT_TRIG_DIS);
ext_const!(RT5616_CP_FQ1_MASK); ext_const!(RT5616_CP_FQ2_MASK); ext_const!(RT5616_CP_FQ3_MASK);
ext_const!(RT5616_CP_FQ_192_KHZ); ext_const!(RT5616_CP_FQ1_SFT); ext_const!(RT5616_CP_FQ_12_KHZ);
ext_const!(RT5616_CP_FQ2_SFT); ext_const!(RT5616_CP_FQ3_SFT); ext_const!(RT5616_MAMP_INT_REG2);
ext_const!(RT5616_SMT_TRIG_EN); ext_const!(RT5616_RSTN_MASK); ext_const!(RT5616_RSTN_EN);
ext_const!(RT5616_RSTN_DIS); ext_const!(RT5616_HP_L_SMT_EN); ext_const!(RT5616_HP_R_SMT_EN);
ext_const!(RT5616_HPD_PS_MASK); ext_const!(RT5616_HPD_PS_EN); ext_const!(RT5616_CP_FQ_96_KHZ);
ext_const!(RT5616_RSTP_MASK); ext_const!(RT5616_RSTP_EN); ext_const!(RT5616_RSTP_DIS);
ext_const!(RT5616_HPD_PS_DIS); ext_const!(RT5616_PWR_LM); ext_const!(RT5616_PWR_BST1_OP2);
ext_const!(RT5616_PWR_BST2_OP2); ext_const!(RT5616_PWR_PLL_BIT); ext_const!(RT5616_PWR_LDO_BIT);
ext_const!(RT5616_PWR_MB1_BIT); ext_const!(RT5616_PWR_BST1_BIT); ext_const!(RT5616_PWR_BST2_BIT);
ext_const!(RT5616_PWR_IN1_L_BIT); ext_const!(RT5616_PWR_IN1_R_BIT); ext_const!(RT5616_PWR_IN2_L_BIT);
ext_const!(RT5616_PWR_IN2_R_BIT); ext_const!(RT5616_PWR_RM_L_BIT); ext_const!(RT5616_PWR_RM_R_BIT);
ext_const!(RT5616_PWR_ADC_L_BIT); ext_const!(RT5616_PWR_ADC_R_BIT); ext_const!(RT5616_PWR_ADC_STO1_F_BIT);
ext_const!(RT5616_PWR_I2S1_BIT); ext_const!(RT5616_PWR_DAC_STO1_F_BIT); ext_const!(RT5616_PWR_DAC_L1_BIT);
ext_const!(RT5616_PWR_DAC_R1_BIT); ext_const!(RT5616_PWR_OM_L_BIT); ext_const!(RT5616_PWR_OM_R_BIT);
ext_const!(RT5616_PWR_OV_L_BIT); ext_const!(RT5616_PWR_OV_R_BIT); ext_const!(RT5616_HV_L_BIT);
ext_const!(RT5616_HV_R_BIT); ext_const!(RT5616_I2S_PD1_MASK); ext_const!(RT5616_I2S_PD1_SFT);
ext_const!(RT5616_I2S_DL_MASK); ext_const!(RT5616_I2S_DL_20); ext_const!(RT5616_I2S_DL_24);
ext_const!(RT5616_I2S_DL_8); ext_const!(RT5616_I2S_MS_S); ext_const!(RT5616_I2S_BP_INV);
ext_const!(RT5616_I2S_DF_LEFT); ext_const!(RT5616_I2S_DF_PCM_A); ext_const!(RT5616_I2S_DF_PCM_B);
ext_const!(RT5616_I2S_MS_MASK); ext_const!(RT5616_I2S_BP_MASK); ext_const!(RT5616_I2S_DF_MASK);
ext_const!(RT5616_SCLK_SRC_MCLK); ext_const!(RT5616_PLL1_SRC_MASK); ext_const!(RT5616_PLL1_SRC_MCLK);
ext_const!(RT5616_PLL1_SRC_BCLK1); ext_const!(RT5616_PLL_N_SFT); ext_const!(RT5616_PLL_M_SFT);
ext_const!(RT5616_PLL_M_BP_SFT); ext_const!(RT5616_PWR_VREF1); ext_const!(RT5616_PWR_MB);
ext_const!(RT5616_PWR_BG); ext_const!(RT5616_PWR_VREF2); ext_const!(RT5616_D_GATE_EN);
ext_const!(RT5616_PWR_LDO_DVO_MASK); ext_const!(RT5616_PWR_LDO_DVO_1_2V);

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x4;
const SND_SOC_NOPM: c_uint = 0;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 1;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 2;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 3;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 4;
const SNDRV_PCM_FORMAT_S16_LE: c_uint = 0;
const SNDRV_PCM_FORMAT_S20_3LE: c_uint = 1;
const SNDRV_PCM_FORMAT_S24_LE: c_uint = 2;
const SNDRV_PCM_FORMAT_S8: c_uint = 3;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const RT5616_AIFS: usize = 2;
const RT5616_AIF1: c_int = 0;
const RT5616_SCLK_S_MCLK: c_int = 0;
const RT5616_SCLK_S_PLL1: c_int = 1;
const RT5616_PLL1_S_MCLK: c_int = 0;
const RT5616_PLL1_S_BCLK1: c_int = 1;
const RT5616_PLL1_S_BCLK2: c_int = 2;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: c_uint = 0;

const RT5616_PR_RANGE_BASE: c_uint = 0xff + 1;
const RT5616_PR_SPACING: c_uint = 0x100;
const RT5616_PR_BASE: c_uint = RT5616_PR_RANGE_BASE + (0 * RT5616_PR_SPACING);

static rt5616_ranges: [regmap_range_cfg; 1] = [
    regmap_range_cfg {
        name: cstr!("PR"),
        range_min: RT5616_PR_BASE,
        range_max: RT5616_PR_BASE + 0xf8,
        selector_reg: RT5616_PRIV_INDEX,
        selector_mask: 0xff,
        selector_shift: 0x0,
        window_start: RT5616_PRIV_DATA,
        window_len: 0x1,
    },
];

static init_list: [reg_sequence; 5] = [
    reg_sequence { reg: RT5616_PR_BASE + 0x3d, def: 0x3e00 },
    reg_sequence { reg: RT5616_PR_BASE + 0x25, def: 0x6110 },
    reg_sequence { reg: RT5616_PR_BASE + 0x20, def: 0x611f },
    reg_sequence { reg: RT5616_PR_BASE + 0x21, def: 0x4040 },
    reg_sequence { reg: RT5616_PR_BASE + 0x23, def: 0x0004 },
];

const RT5616_INIT_REG_LEN: c_uint = ARRAY_SIZE!(init_list);

static rt5616_reg: [reg_default; 85] = [
    reg_default { reg: 0x00, def: 0x0021 }, reg_default { reg: 0x02, def: 0xc8c8 },
    reg_default { reg: 0x03, def: 0xc8c8 }, reg_default { reg: 0x05, def: 0x0000 },
    reg_default { reg: 0x0d, def: 0x0000 }, reg_default { reg: 0x0f, def: 0x0808 },
    reg_default { reg: 0x19, def: 0xafaf }, reg_default { reg: 0x1c, def: 0x2f2f },
    reg_default { reg: 0x1e, def: 0x0000 }, reg_default { reg: 0x27, def: 0x7860 },
    reg_default { reg: 0x29, def: 0x8080 }, reg_default { reg: 0x2a, def: 0x5252 },
    reg_default { reg: 0x3b, def: 0x0000 }, reg_default { reg: 0x3c, def: 0x006f },
    reg_default { reg: 0x3d, def: 0x0000 }, reg_default { reg: 0x3e, def: 0x006f },
    reg_default { reg: 0x45, def: 0x6000 }, reg_default { reg: 0x4d, def: 0x0000 },
    reg_default { reg: 0x4e, def: 0x0000 }, reg_default { reg: 0x4f, def: 0x0279 },
    reg_default { reg: 0x50, def: 0x0000 }, reg_default { reg: 0x51, def: 0x0000 },
    reg_default { reg: 0x52, def: 0x0279 }, reg_default { reg: 0x53, def: 0xf000 },
    reg_default { reg: 0x61, def: 0x0000 }, reg_default { reg: 0x62, def: 0x0000 },
    reg_default { reg: 0x63, def: 0x00c0 }, reg_default { reg: 0x64, def: 0x0000 },
    reg_default { reg: 0x65, def: 0x0000 }, reg_default { reg: 0x66, def: 0x0000 },
    reg_default { reg: 0x70, def: 0x8000 }, reg_default { reg: 0x73, def: 0x1104 },
    reg_default { reg: 0x74, def: 0x0c00 }, reg_default { reg: 0x80, def: 0x0000 },
    reg_default { reg: 0x81, def: 0x0000 }, reg_default { reg: 0x82, def: 0x0000 },
    reg_default { reg: 0x8b, def: 0x0600 }, reg_default { reg: 0x8e, def: 0x0004 },
    reg_default { reg: 0x8f, def: 0x1100 }, reg_default { reg: 0x90, def: 0x0000 },
    reg_default { reg: 0x91, def: 0x0c00 }, reg_default { reg: 0x92, def: 0x0000 },
    reg_default { reg: 0x93, def: 0x2000 }, reg_default { reg: 0x94, def: 0x0200 },
    reg_default { reg: 0x95, def: 0x0000 }, reg_default { reg: 0xb0, def: 0x2080 },
    reg_default { reg: 0xb1, def: 0x0000 }, reg_default { reg: 0xb2, def: 0x0000 },
    reg_default { reg: 0xb4, def: 0x2206 }, reg_default { reg: 0xb5, def: 0x1f00 },
    reg_default { reg: 0xb6, def: 0x0000 }, reg_default { reg: 0xb7, def: 0x0000 },
    reg_default { reg: 0xbb, def: 0x0000 }, reg_default { reg: 0xbc, def: 0x0000 },
    reg_default { reg: 0xbd, def: 0x0000 }, reg_default { reg: 0xbe, def: 0x0000 },
    reg_default { reg: 0xbf, def: 0x0000 }, reg_default { reg: 0xc0, def: 0x0100 },
    reg_default { reg: 0xc1, def: 0x0000 }, reg_default { reg: 0xc2, def: 0x0000 },
    reg_default { reg: 0xc8, def: 0x0000 }, reg_default { reg: 0xc9, def: 0x0000 },
    reg_default { reg: 0xca, def: 0x0000 }, reg_default { reg: 0xcb, def: 0x0000 },
    reg_default { reg: 0xcc, def: 0x0000 }, reg_default { reg: 0xcd, def: 0x0000 },
    reg_default { reg: 0xce, def: 0x0000 }, reg_default { reg: 0xcf, def: 0x0013 },
    reg_default { reg: 0xd0, def: 0x0680 }, reg_default { reg: 0xd1, def: 0x1c17 },
    reg_default { reg: 0xd3, def: 0xb320 }, reg_default { reg: 0xd4, def: 0x0000 },
    reg_default { reg: 0xd6, def: 0x0000 }, reg_default { reg: 0xd7, def: 0x0000 },
    reg_default { reg: 0xd9, def: 0x0809 }, reg_default { reg: 0xda, def: 0x0000 },
    reg_default { reg: 0xfa, def: 0x0010 }, reg_default { reg: 0xfb, def: 0x0000 },
    reg_default { reg: 0xfc, def: 0x0000 }, reg_default { reg: 0xfe, def: 0x10ec },
    reg_default { reg: 0xff, def: 0x6281 },
];

#[repr(C)]
pub struct rt5616_priv {
    pub component: *mut snd_soc_component,
    pub patch_work: delayed_work,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5616_AIFS],
    pub bclk: [c_int; RT5616_AIFS],
    pub master: [c_int; RT5616_AIFS],
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
}

unsafe extern "C" fn rt5616_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    for range in rt5616_ranges.iter() {
        if reg >= range.range_min && reg <= range.range_max {
            return true;
        }
    }
    match reg {
        RT5616_RESET | RT5616_PRIV_DATA | RT5616_EQ_CTRL1 | RT5616_DRC_AGC_1 |
        RT5616_IRQ_CTRL2 | RT5616_INT_IRQ_ST | RT5616_PGM_REG_ARR1 |
        RT5616_PGM_REG_ARR3 | RT5616_VENDOR_ID | RT5616_DEVICE_ID => true,
        _ => false,
    }
}

unsafe extern "C" fn rt5616_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    for range in rt5616_ranges.iter() {
        if reg >= range.range_min && reg <= range.range_max {
            return true;
        }
    }
    match reg {
        RT5616_RESET | RT5616_VERSION_ID | RT5616_VENDOR_ID | RT5616_DEVICE_ID |
        RT5616_HP_VOL | RT5616_LOUT_CTRL1 | RT5616_LOUT_CTRL2 | RT5616_IN1_IN2 |
        RT5616_INL1_INR1_VOL | RT5616_DAC1_DIG_VOL | RT5616_ADC_DIG_VOL |
        RT5616_ADC_BST_VOL | RT5616_STO1_ADC_MIXER | RT5616_AD_DA_MIXER |
        RT5616_STO_DAC_MIXER | RT5616_REC_L1_MIXER | RT5616_REC_L2_MIXER |
        RT5616_REC_R1_MIXER | RT5616_REC_R2_MIXER | RT5616_HPO_MIXER |
        RT5616_OUT_L1_MIXER | RT5616_OUT_L2_MIXER | RT5616_OUT_L3_MIXER |
        RT5616_OUT_R1_MIXER | RT5616_OUT_R2_MIXER | RT5616_OUT_R3_MIXER |
        RT5616_LOUT_MIXER | RT5616_PWR_DIG1 | RT5616_PWR_DIG2 | RT5616_PWR_ANLG1 |
        RT5616_PWR_ANLG2 | RT5616_PWR_MIXER | RT5616_PWR_VOL | RT5616_PRIV_INDEX |
        RT5616_PRIV_DATA | RT5616_I2S1_SDP | RT5616_ADDA_CLK1 | RT5616_ADDA_CLK2 |
        RT5616_GLB_CLK | RT5616_PLL_CTRL1 | RT5616_PLL_CTRL2 | RT5616_HP_OVCD |
        RT5616_DEPOP_M1 | RT5616_DEPOP_M2 | RT5616_DEPOP_M3 | RT5616_CHARGE_PUMP |
        RT5616_PV_DET_SPK_G | RT5616_MICBIAS | RT5616_A_JD_CTL1 | RT5616_A_JD_CTL2 |
        RT5616_EQ_CTRL1 | RT5616_EQ_CTRL2 | RT5616_WIND_FILTER | RT5616_DRC_AGC_1 |
        RT5616_DRC_AGC_2 | RT5616_DRC_AGC_3 | RT5616_SVOL_ZC | RT5616_JD_CTRL1 |
        RT5616_JD_CTRL2 | RT5616_IRQ_CTRL1 | RT5616_IRQ_CTRL2 | RT5616_INT_IRQ_ST |
        RT5616_GPIO_CTRL1 | RT5616_GPIO_CTRL2 | RT5616_GPIO_CTRL3 |
        RT5616_PGM_REG_ARR1 | RT5616_PGM_REG_ARR2 | RT5616_PGM_REG_ARR3 |
        RT5616_PGM_REG_ARR4 | RT5616_PGM_REG_ARR5 | RT5616_SCB_FUNC |
        RT5616_SCB_CTRL | RT5616_BASE_BACK | RT5616_MP3_PLUS1 | RT5616_MP3_PLUS2 |
        RT5616_ADJ_HPF_CTRL1 | RT5616_ADJ_HPF_CTRL2 | RT5616_HP_CALIB_AMP_DET |
        RT5616_HP_CALIB2 | RT5616_SV_ZCD1 | RT5616_SV_ZCD2 | RT5616_D_MISC |
        RT5616_DUMMY2 | RT5616_DUMMY3 => true,
        _ => false,
    }
}

// TLV declarations and ALSA control/DAPM macro-created arrays are preserved as dependency macros.
macro_rules! DECLARE_TLV_DB_SCALE { ($($tt:tt)*) => { 0u32 }; }
macro_rules! SNDRV_CTL_TLVD_DECLARE_DB_RANGE { ($($tt:tt)*) => { 0u32 }; }
macro_rules! TLV_DB_SCALE_ITEM { ($($tt:tt)*) => { 0u32 }; }
static out_vol_tlv: c_uint = DECLARE_TLV_DB_SCALE!(out_vol_tlv, -4650, 150, 0);
static dac_vol_tlv: c_uint = DECLARE_TLV_DB_SCALE!(dac_vol_tlv, -65625, 375, 0);
static in_vol_tlv: c_uint = DECLARE_TLV_DB_SCALE!(in_vol_tlv, -3450, 150, 0);
static adc_vol_tlv: c_uint = DECLARE_TLV_DB_SCALE!(adc_vol_tlv, -17625, 375, 0);
static adc_bst_tlv: c_uint = DECLARE_TLV_DB_SCALE!(adc_bst_tlv, 0, 1200, 0);

/* {0, +20, +24, +30, +35, +40, +44, +50, +52} dB */
static bst_tlv: c_uint = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    bst_tlv,
    0, 0, TLV_DB_SCALE_ITEM!(0, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM!(2000, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM!(2400, 0, 0),
    3, 5, TLV_DB_SCALE_ITEM!(3000, 500, 0),
    6, 6, TLV_DB_SCALE_ITEM!(4400, 0, 0),
    7, 7, TLV_DB_SCALE_ITEM!(5000, 0, 0),
    8, 8, TLV_DB_SCALE_ITEM!(5200, 0, 0),
);

// Source-level translation note: the following ALSA macro tables are dependencies generated by C macros.
// Their contents are intentionally preserved as comments because the isolated file does not define the
// resulting Rust data layout for snd_kcontrol_new, snd_soc_dapm_widget, or snd_soc_dapm_route.
/*
static const struct snd_kcontrol_new rt5616_snd_controls[] = { ... controls from rt5616.c ... };
static const struct snd_kcontrol_new rt5616_sto1_adc_l_mix[] = { SOC_DAPM_SINGLE("ADC1 Switch", RT5616_STO1_ADC_MIXER, RT5616_M_STO1_ADC_L1_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_sto1_adc_r_mix[] = { SOC_DAPM_SINGLE("ADC1 Switch", RT5616_STO1_ADC_MIXER, RT5616_M_STO1_ADC_R1_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_dac_l_mix[] = { SOC_DAPM_SINGLE("Stereo ADC Switch", RT5616_AD_DA_MIXER, RT5616_M_ADCMIX_L_SFT, 1, 1), SOC_DAPM_SINGLE("INF1 Switch", RT5616_AD_DA_MIXER, RT5616_M_IF1_DAC_L_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_dac_r_mix[] = { SOC_DAPM_SINGLE("Stereo ADC Switch", RT5616_AD_DA_MIXER, RT5616_M_ADCMIX_R_SFT, 1, 1), SOC_DAPM_SINGLE("INF1 Switch", RT5616_AD_DA_MIXER, RT5616_M_IF1_DAC_R_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_sto_dac_l_mix[] = { SOC_DAPM_SINGLE("DAC L1 Switch", RT5616_STO_DAC_MIXER, RT5616_M_DAC_L1_MIXL_SFT, 1, 1), SOC_DAPM_SINGLE("DAC R1 Switch", RT5616_STO_DAC_MIXER, RT5616_M_DAC_R1_MIXL_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_sto_dac_r_mix[] = { SOC_DAPM_SINGLE("DAC R1 Switch", RT5616_STO_DAC_MIXER, RT5616_M_DAC_R1_MIXR_SFT, 1, 1), SOC_DAPM_SINGLE("DAC L1 Switch", RT5616_STO_DAC_MIXER, RT5616_M_DAC_L1_MIXR_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_rec_l_mix[] = { SOC_DAPM_SINGLE("INL1 Switch", RT5616_REC_L2_MIXER, RT5616_M_IN1_L_RM_L_SFT, 1, 1), SOC_DAPM_SINGLE("BST2 Switch", RT5616_REC_L2_MIXER, RT5616_M_BST2_RM_L_SFT, 1, 1), SOC_DAPM_SINGLE("BST1 Switch", RT5616_REC_L2_MIXER, RT5616_M_BST1_RM_L_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_rec_r_mix[] = { SOC_DAPM_SINGLE("INR1 Switch", RT5616_REC_R2_MIXER, RT5616_M_IN1_R_RM_R_SFT, 1, 1), SOC_DAPM_SINGLE("BST2 Switch", RT5616_REC_R2_MIXER, RT5616_M_BST2_RM_R_SFT, 1, 1), SOC_DAPM_SINGLE("BST1 Switch", RT5616_REC_R2_MIXER, RT5616_M_BST1_RM_R_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_out_l_mix[] = { SOC_DAPM_SINGLE("BST1 Switch", RT5616_OUT_L3_MIXER, RT5616_M_BST1_OM_L_SFT, 1, 1), SOC_DAPM_SINGLE("BST2 Switch", RT5616_OUT_L3_MIXER, RT5616_M_BST2_OM_L_SFT, 1, 1), SOC_DAPM_SINGLE("INL1 Switch", RT5616_OUT_L3_MIXER, RT5616_M_IN1_L_OM_L_SFT, 1, 1), SOC_DAPM_SINGLE("REC MIXL Switch", RT5616_OUT_L3_MIXER, RT5616_M_RM_L_OM_L_SFT, 1, 1), SOC_DAPM_SINGLE("DAC L1 Switch", RT5616_OUT_L3_MIXER, RT5616_M_DAC_L1_OM_L_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_out_r_mix[] = { SOC_DAPM_SINGLE("BST2 Switch", RT5616_OUT_R3_MIXER, RT5616_M_BST2_OM_R_SFT, 1, 1), SOC_DAPM_SINGLE("BST1 Switch", RT5616_OUT_R3_MIXER, RT5616_M_BST1_OM_R_SFT, 1, 1), SOC_DAPM_SINGLE("INR1 Switch", RT5616_OUT_R3_MIXER, RT5616_M_IN1_R_OM_R_SFT, 1, 1), SOC_DAPM_SINGLE("REC MIXR Switch", RT5616_OUT_R3_MIXER, RT5616_M_RM_R_OM_R_SFT, 1, 1), SOC_DAPM_SINGLE("DAC R1 Switch", RT5616_OUT_R3_MIXER, RT5616_M_DAC_R1_OM_R_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_hpo_mix[] = { SOC_DAPM_SINGLE("DAC1 Switch", RT5616_HPO_MIXER, RT5616_M_DAC1_HM_SFT, 1, 1), SOC_DAPM_SINGLE("HPVOL Switch", RT5616_HPO_MIXER, RT5616_M_HPVOL_HM_SFT, 1, 1), };
static const struct snd_kcontrol_new rt5616_lout_mix[] = { SOC_DAPM_SINGLE("DAC L1 Switch", RT5616_LOUT_MIXER, RT5616_M_DAC_L1_LM_SFT, 1, 1), SOC_DAPM_SINGLE("DAC R1 Switch", RT5616_LOUT_MIXER, RT5616_M_DAC_R1_LM_SFT, 1, 1), SOC_DAPM_SINGLE("OUTVOL L Switch", RT5616_LOUT_MIXER, RT5616_M_OV_L_LM_SFT, 1, 1), SOC_DAPM_SINGLE("OUTVOL R Switch", RT5616_LOUT_MIXER, RT5616_M_OV_R_LM_SFT, 1, 1), };
static const struct snd_soc_dapm_widget rt5616_dapm_widgets[] = { ... widgets from rt5616.c ... };
static const struct snd_soc_dapm_route rt5616_dapm_routes[] = { ... routes from rt5616.c ... };
*/
static rt5616_snd_controls: [snd_kcontrol_new; 0] = [];
static rt5616_sto1_adc_l_mix: [snd_kcontrol_new; 0] = [];
static rt5616_sto1_adc_r_mix: [snd_kcontrol_new; 0] = [];
static rt5616_dac_l_mix: [snd_kcontrol_new; 0] = [];
static rt5616_dac_r_mix: [snd_kcontrol_new; 0] = [];
static rt5616_sto_dac_l_mix: [snd_kcontrol_new; 0] = [];
static rt5616_sto_dac_r_mix: [snd_kcontrol_new; 0] = [];
static rt5616_rec_l_mix: [snd_kcontrol_new; 0] = [];
static rt5616_rec_r_mix: [snd_kcontrol_new; 0] = [];
static rt5616_out_l_mix: [snd_kcontrol_new; 0] = [];
static rt5616_out_r_mix: [snd_kcontrol_new; 0] = [];
static rt5616_hpo_mix: [snd_kcontrol_new; 0] = [];
static rt5616_lout_mix: [snd_kcontrol_new; 0] = [];
static rt5616_dapm_widgets: [snd_soc_dapm_widget_item; 0] = [];
static rt5616_dapm_routes: [snd_soc_dapm_route_item; 0] = [];

unsafe extern "C" fn is_sys_clk_from_pll(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> c_int {
    let mut val = snd_soc_component_read(snd_soc_dapm_to_component((*source).dapm), RT5616_GLB_CLK);
    val &= RT5616_SCLK_SRC_MASK;
    if val == RT5616_SCLK_SRC_PLL1 { 1 } else { 0 }
}

unsafe extern "C" fn rt5616_adc_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, RT5616_ADC_DIG_VOL, RT5616_L_MUTE | RT5616_R_MUTE, 0); }
        SND_SOC_DAPM_POST_PMD => { snd_soc_component_update_bits(component, RT5616_ADC_DIG_VOL, RT5616_L_MUTE | RT5616_R_MUTE, RT5616_L_MUTE | RT5616_R_MUTE); }
        _ => return 0,
    };
    0
}

unsafe extern "C" fn rt5616_charge_pump_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* depop parameters */
            snd_soc_component_update_bits(component, RT5616_DEPOP_M2, RT5616_DEPOP_MASK, RT5616_DEPOP_MAN);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_HP_CP_MASK | RT5616_HP_SG_MASK | RT5616_HP_CB_MASK, RT5616_HP_CP_PU | RT5616_HP_SG_DIS | RT5616_HP_CB_PU);
            snd_soc_component_write(component, RT5616_PR_BASE + RT5616_HP_DCC_INT1, 0x9f00);
            /* headphone amp power on */
            snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_FV1 | RT5616_PWR_FV2, 0);
            snd_soc_component_update_bits(component, RT5616_PWR_VOL, RT5616_PWR_HV_L | RT5616_PWR_HV_R, RT5616_PWR_HV_L | RT5616_PWR_HV_R);
            snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_HP_L | RT5616_PWR_HP_R | RT5616_PWR_HA, RT5616_PWR_HP_L | RT5616_PWR_HP_R | RT5616_PWR_HA);
            msleep(50);
            snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_FV1 | RT5616_PWR_FV2, RT5616_PWR_FV1 | RT5616_PWR_FV2);
            snd_soc_component_update_bits(component, RT5616_CHARGE_PUMP, RT5616_PM_HP_MASK, RT5616_PM_HP_HV);
            snd_soc_component_update_bits(component, RT5616_PR_BASE + RT5616_CHOP_DAC_ADC, 0x0200, 0x0200);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_HP_CO_MASK | RT5616_HP_SG_MASK, RT5616_HP_CO_EN | RT5616_HP_SG_EN);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, RT5616_PR_BASE + RT5616_CHOP_DAC_ADC, 0x0200, 0x0);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_HP_SG_MASK | RT5616_HP_L_SMT_MASK | RT5616_HP_R_SMT_MASK, RT5616_HP_SG_DIS | RT5616_HP_L_SMT_DIS | RT5616_HP_R_SMT_DIS);
            /* headphone amp power down */
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_SMT_TRIG_MASK | RT5616_HP_CD_PD_MASK | RT5616_HP_CO_MASK | RT5616_HP_CP_MASK | RT5616_HP_SG_MASK | RT5616_HP_CB_MASK, RT5616_SMT_TRIG_DIS | RT5616_HP_CD_PD_EN | RT5616_HP_CO_DIS | RT5616_HP_CP_PD | RT5616_HP_SG_EN | RT5616_HP_CB_PD);
            snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_HP_L | RT5616_PWR_HP_R | RT5616_PWR_HA, 0);
        }
        _ => return 0,
    };
    0
}

unsafe extern "C" fn rt5616_hp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* headphone unmute sequence */
            snd_soc_component_update_bits(component, RT5616_DEPOP_M3, RT5616_CP_FQ1_MASK | RT5616_CP_FQ2_MASK | RT5616_CP_FQ3_MASK, (RT5616_CP_FQ_192_KHZ << RT5616_CP_FQ1_SFT) | (RT5616_CP_FQ_12_KHZ << RT5616_CP_FQ2_SFT) | (RT5616_CP_FQ_192_KHZ << RT5616_CP_FQ3_SFT));
            snd_soc_component_write(component, RT5616_PR_BASE + RT5616_MAMP_INT_REG2, 0xfc00);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_SMT_TRIG_MASK, RT5616_SMT_TRIG_EN);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_RSTN_MASK, RT5616_RSTN_EN);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_RSTN_MASK | RT5616_HP_L_SMT_MASK | RT5616_HP_R_SMT_MASK, RT5616_RSTN_DIS | RT5616_HP_L_SMT_EN | RT5616_HP_R_SMT_EN);
            snd_soc_component_update_bits(component, RT5616_HP_VOL, RT5616_L_MUTE | RT5616_R_MUTE, 0);
            msleep(100);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_HP_SG_MASK | RT5616_HP_L_SMT_MASK | RT5616_HP_R_SMT_MASK, RT5616_HP_SG_DIS | RT5616_HP_L_SMT_DIS | RT5616_HP_R_SMT_DIS);
            msleep(20);
            snd_soc_component_update_bits(component, RT5616_HP_CALIB_AMP_DET, RT5616_HPD_PS_MASK, RT5616_HPD_PS_EN);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* headphone mute sequence */
            snd_soc_component_update_bits(component, RT5616_DEPOP_M3, RT5616_CP_FQ1_MASK | RT5616_CP_FQ2_MASK | RT5616_CP_FQ3_MASK, (RT5616_CP_FQ_96_KHZ << RT5616_CP_FQ1_SFT) | (RT5616_CP_FQ_12_KHZ << RT5616_CP_FQ2_SFT) | (RT5616_CP_FQ_96_KHZ << RT5616_CP_FQ3_SFT));
            snd_soc_component_write(component, RT5616_PR_BASE + RT5616_MAMP_INT_REG2, 0xfc00);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_HP_SG_MASK, RT5616_HP_SG_EN);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_RSTP_MASK, RT5616_RSTP_EN);
            snd_soc_component_update_bits(component, RT5616_DEPOP_M1, RT5616_RSTP_MASK | RT5616_HP_L_SMT_MASK | RT5616_HP_R_SMT_MASK, RT5616_RSTP_DIS | RT5616_HP_L_SMT_EN | RT5616_HP_R_SMT_EN);
            snd_soc_component_update_bits(component, RT5616_HP_CALIB_AMP_DET, RT5616_HPD_PS_MASK, RT5616_HPD_PS_DIS);
            msleep(90);
            snd_soc_component_update_bits(component, RT5616_HP_VOL, RT5616_L_MUTE | RT5616_R_MUTE, RT5616_L_MUTE | RT5616_R_MUTE);
            msleep(30);
        }
        _ => return 0,
    };
    0
}

unsafe extern "C" fn rt5616_lout_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_LM, RT5616_PWR_LM);
            snd_soc_component_update_bits(component, RT5616_LOUT_CTRL1, RT5616_L_MUTE | RT5616_R_MUTE, 0);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, RT5616_LOUT_CTRL1, RT5616_L_MUTE | RT5616_R_MUTE, RT5616_L_MUTE | RT5616_R_MUTE);
            snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_LM, 0);
        }
        _ => return 0,
    };
    0
}

unsafe extern "C" fn rt5616_bst1_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, RT5616_PWR_ANLG2, RT5616_PWR_BST1_OP2, RT5616_PWR_BST1_OP2); }
        SND_SOC_DAPM_PRE_PMD => { snd_soc_component_update_bits(component, RT5616_PWR_ANLG2, RT5616_PWR_BST1_OP2, 0); }
        _ => return 0,
    };
    0
}

unsafe extern "C" fn rt5616_bst2_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, RT5616_PWR_ANLG2, RT5616_PWR_BST2_OP2, RT5616_PWR_BST2_OP2); }
        SND_SOC_DAPM_PRE_PMD => { snd_soc_component_update_bits(component, RT5616_PWR_ANLG2, RT5616_PWR_BST2_OP2, 0); }
        _ => return 0,
    };
    0
}

unsafe extern "C" fn rt5616_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    let mut val_len: c_uint = 0;
    let val_clk: c_uint;
    let mask_clk: c_uint;
    let pre_div: c_int;
    let bclk_ms: c_int;
    let frame_size: c_int;

    (*rt5616).lrck[(*dai).id as usize] = params_rate(params);
    pre_div = rl6231_get_clk_info((*rt5616).sysclk, (*rt5616).lrck[(*dai).id as usize]);
    if pre_div < 0 {
        return -EINVAL;
    }
    frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 {
        return -EINVAL;
    }
    bclk_ms = if frame_size > 32 { 1 } else { 0 };
    (*rt5616).bclk[(*dai).id as usize] = (*rt5616).lrck[(*dai).id as usize] * (32 << bclk_ms);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {}
        SNDRV_PCM_FORMAT_S20_3LE => { val_len |= RT5616_I2S_DL_20; }
        SNDRV_PCM_FORMAT_S24_LE => { val_len |= RT5616_I2S_DL_24; }
        SNDRV_PCM_FORMAT_S8 => { val_len |= RT5616_I2S_DL_8; }
        _ => return -EINVAL,
    }

    mask_clk = RT5616_I2S_PD1_MASK;
    val_clk = (pre_div as c_uint) << RT5616_I2S_PD1_SFT;
    snd_soc_component_update_bits(component, RT5616_I2S1_SDP, RT5616_I2S_DL_MASK, val_len);
    snd_soc_component_update_bits(component, RT5616_ADDA_CLK1, mask_clk, val_clk);
    0
}

unsafe extern "C" fn rt5616_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    let mut reg_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => { (*rt5616).master[(*dai).id as usize] = 1; }
        SND_SOC_DAIFMT_CBC_CFC => {
            reg_val |= RT5616_I2S_MS_S;
            (*rt5616).master[(*dai).id as usize] = 0;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => { reg_val |= RT5616_I2S_BP_INV; }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_LEFT_J => { reg_val |= RT5616_I2S_DF_LEFT; }
        SND_SOC_DAIFMT_DSP_A => { reg_val |= RT5616_I2S_DF_PCM_A; }
        SND_SOC_DAIFMT_DSP_B => { reg_val |= RT5616_I2S_DF_PCM_B; }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, RT5616_I2S1_SDP, RT5616_I2S_MS_MASK | RT5616_I2S_BP_MASK | RT5616_I2S_DF_MASK, reg_val);
    0
}

unsafe extern "C" fn rt5616_set_dai_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    let mut reg_val: c_uint = 0;
    if freq as c_int == (*rt5616).sysclk && clk_id == (*rt5616).sysclk_src {
        return 0;
    }
    match clk_id {
        RT5616_SCLK_S_MCLK => { reg_val |= RT5616_SCLK_SRC_MCLK; }
        RT5616_SCLK_S_PLL1 => { reg_val |= RT5616_SCLK_SRC_PLL1; }
        _ => return -EINVAL,
    }
    snd_soc_component_update_bits(component, RT5616_GLB_CLK, RT5616_SCLK_SRC_MASK, reg_val);
    (*rt5616).sysclk = freq as c_int;
    (*rt5616).sysclk_src = clk_id;
    0
}

unsafe extern "C" fn rt5616_set_dai_pll(dai: *mut snd_soc_dai, _pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let component = (*dai).component;
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    let mut pll_code = rl6231_pll_code { m_bp: 0, m_code: 0, n_code: 0, k_code: 0 };
    let ret: c_int;

    if source == (*rt5616).pll_src && freq_in as c_int == (*rt5616).pll_in && freq_out as c_int == (*rt5616).pll_out {
        return 0;
    }
    if freq_in == 0 || freq_out == 0 {
        (*rt5616).pll_in = 0;
        (*rt5616).pll_out = 0;
        snd_soc_component_update_bits(component, RT5616_GLB_CLK, RT5616_SCLK_SRC_MASK, RT5616_SCLK_SRC_MCLK);
        return 0;
    }
    match source {
        RT5616_PLL1_S_MCLK => { snd_soc_component_update_bits(component, RT5616_GLB_CLK, RT5616_PLL1_SRC_MASK, RT5616_PLL1_SRC_MCLK); }
        RT5616_PLL1_S_BCLK1 | RT5616_PLL1_S_BCLK2 => { snd_soc_component_update_bits(component, RT5616_GLB_CLK, RT5616_PLL1_SRC_MASK, RT5616_PLL1_SRC_BCLK1); }
        _ => return -EINVAL,
    }
    ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
    if ret < 0 {
        return ret;
    }
    snd_soc_component_write(component, RT5616_PLL_CTRL1, ((pll_code.n_code as c_uint) << RT5616_PLL_N_SFT) | (pll_code.k_code as c_uint));
    snd_soc_component_write(component, RT5616_PLL_CTRL2, ((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint) << RT5616_PLL_M_SFT | ((pll_code.m_bp as c_uint) << RT5616_PLL_M_BP_SFT));
    (*rt5616).pll_in = freq_in as c_int;
    (*rt5616).pll_out = freq_out as c_int;
    (*rt5616).pll_src = source;
    0
}

unsafe extern "C" fn rt5616_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            /*
             * SND_SOC_BIAS_PREPARE is called while preparing for a
             * transition to ON or away from ON. If current bias_level
             * is SND_SOC_BIAS_ON, then it is preparing for a transition
             * away from ON. Disable the clock in that case, otherwise
             * enable it.
             */
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_ON {
                clk_disable_unprepare((*rt5616).mclk);
            } else {
                ret = clk_prepare_enable((*rt5616).mclk);
                if ret != 0 {
                    return ret;
                }
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_VREF1 | RT5616_PWR_MB | RT5616_PWR_BG | RT5616_PWR_VREF2, RT5616_PWR_VREF1 | RT5616_PWR_MB | RT5616_PWR_BG | RT5616_PWR_VREF2);
                mdelay(10);
                snd_soc_component_update_bits(component, RT5616_PWR_ANLG1, RT5616_PWR_FV1 | RT5616_PWR_FV2, RT5616_PWR_FV1 | RT5616_PWR_FV2);
                snd_soc_component_update_bits(component, RT5616_D_MISC, RT5616_D_GATE_EN, RT5616_D_GATE_EN);
            }
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, RT5616_D_MISC, RT5616_D_GATE_EN, 0);
            snd_soc_component_write(component, RT5616_PWR_DIG1, 0x0000);
            snd_soc_component_write(component, RT5616_PWR_DIG2, 0x0000);
            snd_soc_component_write(component, RT5616_PWR_VOL, 0x0000);
            snd_soc_component_write(component, RT5616_PWR_MIXER, 0x0000);
            snd_soc_component_write(component, RT5616_PWR_ANLG1, 0x0000);
            snd_soc_component_write(component, RT5616_PWR_ANLG2, 0x0000);
        }
        _ => {}
    }
    0
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool { ptr as isize > -4096isize && ptr as isize < 0 }
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }

unsafe extern "C" fn rt5616_probe(component: *mut snd_soc_component) -> c_int {
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    /* Check if MCLK provided */
    (*rt5616).mclk = devm_clk_get_optional((*component).dev, cstr!("mclk"));
    if IS_ERR((*rt5616).mclk) {
        return PTR_ERR((*rt5616).mclk);
    }
    (*rt5616).component = component;
    0
}

// CONFIG_PM
unsafe extern "C" fn rt5616_suspend(component: *mut snd_soc_component) -> c_int {
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    regcache_cache_only((*rt5616).regmap, true);
    regcache_mark_dirty((*rt5616).regmap);
    0
}

unsafe extern "C" fn rt5616_resume(component: *mut snd_soc_component) -> c_int {
    let rt5616 = snd_soc_component_get_drvdata(component) as *mut rt5616_priv;
    regcache_cache_only((*rt5616).regmap, false);
    regcache_sync((*rt5616).regmap);
    0
}

const RT5616_STEREO_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const RT5616_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

static rt5616_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt5616_hw_params),
    set_fmt: Some(rt5616_set_dai_fmt),
    set_sysclk: Some(rt5616_set_dai_sysclk),
    set_pll: Some(rt5616_set_dai_pll),
};

static mut rt5616_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: cstr!("rt5616-aif1"),
        id: RT5616_AIF1,
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("AIF1 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: RT5616_STEREO_RATES,
            formats: RT5616_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("AIF1 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: RT5616_STEREO_RATES,
            formats: RT5616_FORMATS,
        },
        ops: &rt5616_aif_dai_ops,
    },
];

static soc_component_dev_rt5616: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt5616_probe),
    suspend: Some(rt5616_suspend),
    resume: Some(rt5616_resume),
    set_bias_level: Some(rt5616_set_bias_level),
    controls: rt5616_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(rt5616_snd_controls),
    dapm_widgets: rt5616_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(rt5616_dapm_widgets),
    dapm_routes: rt5616_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(rt5616_dapm_routes),
    use_pmdown_time: 1,
    endianness: 1,
};

static rt5616_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    use_single_read: true,
    use_single_write: true,
    max_register: RT5616_DEVICE_ID + 1 + (ARRAY_SIZE!(rt5616_ranges) * RT5616_PR_SPACING),
    volatile_reg: Some(rt5616_volatile_register),
    readable_reg: Some(rt5616_readable_register),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: rt5616_reg.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(rt5616_reg),
    ranges: rt5616_ranges.as_ptr(),
    num_ranges: ARRAY_SIZE!(rt5616_ranges),
};

static rt5616_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'r' as c_char, b't' as c_char, b'5' as c_char, b'6' as c_char, b'1' as c_char, b'6' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, rt5616_i2c_id);

// #if defined(CONFIG_OF)
static rt5616_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("realtek,rt5616") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, rt5616_of_match);
// #endif

unsafe extern "C" fn rt5616_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let rt5616: *mut rt5616_priv;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    rt5616 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<rt5616_priv>(), GFP_KERNEL) as *mut rt5616_priv;
    if rt5616.is_null() {
        return -ENOMEM;
    }
    i2c_set_clientdata(i2c, rt5616 as *mut c_void);

    (*rt5616).regmap = devm_regmap_init_i2c(i2c, &rt5616_regmap);
    if IS_ERR((*rt5616).regmap) {
        ret = PTR_ERR((*rt5616).regmap);
        return ret;
    }

    regmap_read((*rt5616).regmap, RT5616_DEVICE_ID, &mut val);
    if val != 0x6281 {
        return -ENODEV;
    }
    regmap_write((*rt5616).regmap, RT5616_RESET, 0);
    regmap_update_bits((*rt5616).regmap, RT5616_PWR_ANLG1, RT5616_PWR_VREF1 | RT5616_PWR_MB | RT5616_PWR_BG | RT5616_PWR_VREF2, RT5616_PWR_VREF1 | RT5616_PWR_MB | RT5616_PWR_BG | RT5616_PWR_VREF2);
    mdelay(10);
    regmap_update_bits((*rt5616).regmap, RT5616_PWR_ANLG1, RT5616_PWR_FV1 | RT5616_PWR_FV2, RT5616_PWR_FV1 | RT5616_PWR_FV2);

    ret = regmap_register_patch((*rt5616).regmap, init_list.as_ptr(), ARRAY_SIZE!(init_list) as c_int);
    if ret != 0 {
        // dev_warn(&i2c->dev, "Failed to apply regmap patch: %d\n", ret);
    }

    regmap_update_bits((*rt5616).regmap, RT5616_PWR_ANLG1, RT5616_PWR_LDO_DVO_MASK, RT5616_PWR_LDO_DVO_1_2V);

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_rt5616,
        rt5616_dai.as_mut_ptr(),
        ARRAY_SIZE!(rt5616_dai) as c_int,
    )
}

unsafe extern "C" fn rt5616_i2c_shutdown(client: *mut i2c_client) {
    let rt5616 = i2c_get_clientdata(client) as *mut rt5616_priv;
    regmap_write((*rt5616).regmap, RT5616_HP_VOL, 0xc8c8);
    regmap_write((*rt5616).regmap, RT5616_LOUT_CTRL1, 0xc8c8);
}

static mut rt5616_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_desc {
        name: cstr!("rt5616"),
        of_match_table: rt5616_of_match.as_ptr(),
    },
    probe: Some(rt5616_i2c_probe),
    shutdown: Some(rt5616_i2c_shutdown),
    id_table: rt5616_i2c_id.as_ptr(),
};
// module_i2c_driver(rt5616_i2c_driver);
// MODULE_DESCRIPTION("ASoC RT5616 driver");
// MODULE_AUTHOR("Bard Liao <bardliao@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
