// SPDX-License-Identifier: GPL-2.0-only
//
// es8323.c -- es8323 ALSA SoC audio driver
//
// Copyright 2024 Rockchip Electronics Co. Ltd.
// Copyright 2024 Everest Semiconductor Co.,Ltd.
// Copyright 2024 Loongson Technology Co.,Ltd.
//
// Author: Mark Brown <broonie@kernel.org>
//         Jianqun Xu <jay.xu@rock-chips.com>
//         Nickey Yang <nickey.yang@rock-chips.com>
// Further cleanup and restructuring by:
//         Binbin Zhou <zhoubinbin@loongson.cn>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
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
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
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
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct es8323_priv {
    pub sysclk: c_uint,
    pub mclk: *mut clk,
    pub regmap: *mut regmap,
    pub sysclk_constraints: *mut snd_pcm_hw_constraint_list,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write_field(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_uint = 0;
const SNDRV_PCM_FORMAT_S20_3LE: c_uint = 1;
const SNDRV_PCM_FORMAT_S24_LE: c_uint = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_uint = 3;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;

// Register and bitfield constants are supplied by es8323.h in the original C source.

/* es8323 register cache */
static es8323_reg_defaults: [reg_default; 48] = [
    reg_default { reg: ES8323_CONTROL1, def: 0x06 },
    reg_default { reg: ES8323_CONTROL2, def: 0x1c },
    reg_default { reg: ES8323_CHIPPOWER, def: 0xc3 },
    reg_default { reg: ES8323_ADCPOWER, def: 0xfc },
    reg_default { reg: ES8323_DACPOWER, def: 0xc0 },
    reg_default { reg: ES8323_CHIPLOPOW1, def: 0x00 },
    reg_default { reg: ES8323_CHIPLOPOW2, def: 0x00 },
    reg_default { reg: ES8323_ANAVOLMANAG, def: 0x7c },
    reg_default { reg: ES8323_MASTERMODE, def: 0x80 },
    reg_default { reg: ES8323_ADCCONTROL1, def: 0x00 },
    reg_default { reg: ES8323_ADCCONTROL2, def: 0x00 },
    reg_default { reg: ES8323_ADCCONTROL3, def: 0x06 },
    reg_default { reg: ES8323_ADCCONTROL4, def: 0x00 },
    reg_default { reg: ES8323_ADCCONTROL5, def: 0x06 },
    reg_default { reg: ES8323_ADCCONTROL6, def: 0x30 },
    reg_default { reg: ES8323_ADCCONTROL7, def: 0x30 },
    reg_default { reg: ES8323_LADC_VOL, def: 0xc0 },
    reg_default { reg: ES8323_RADC_VOL, def: 0xc0 },
    reg_default { reg: ES8323_ADCCONTROL10, def: 0x38 },
    reg_default { reg: ES8323_ADCCONTROL11, def: 0xb0 },
    reg_default { reg: ES8323_ADCCONTROL12, def: 0x32 },
    reg_default { reg: ES8323_ADCCONTROL13, def: 0x06 },
    reg_default { reg: ES8323_ADCCONTROL14, def: 0x00 },
    reg_default { reg: ES8323_DACCONTROL1, def: 0x00 },
    reg_default { reg: ES8323_DACCONTROL2, def: 0x06 },
    reg_default { reg: ES8323_DACCONTROL3, def: 0x30 },
    reg_default { reg: ES8323_LDAC_VOL, def: 0xc0 },
    reg_default { reg: ES8323_RDAC_VOL, def: 0xc0 },
    reg_default { reg: ES8323_DACCONTROL6, def: 0x08 },
    reg_default { reg: ES8323_DACCONTROL7, def: 0x06 },
    reg_default { reg: ES8323_DACCONTROL8, def: 0x1f },
    reg_default { reg: ES8323_DACCONTROL9, def: 0xf7 },
    reg_default { reg: ES8323_DACCONTROL10, def: 0xfd },
    reg_default { reg: ES8323_DACCONTROL11, def: 0xff },
    reg_default { reg: ES8323_DACCONTROL12, def: 0x1f },
    reg_default { reg: ES8323_DACCONTROL13, def: 0xf7 },
    reg_default { reg: ES8323_DACCONTROL14, def: 0xfd },
    reg_default { reg: ES8323_DACCONTROL15, def: 0xff },
    reg_default { reg: ES8323_DACCONTROL16, def: 0x00 },
    reg_default { reg: ES8323_DACCONTROL17, def: 0x38 },
    reg_default { reg: ES8323_DACCONTROL18, def: 0x38 },
    reg_default { reg: ES8323_DACCONTROL19, def: 0x38 },
    reg_default { reg: ES8323_DACCONTROL20, def: 0x38 },
    reg_default { reg: ES8323_DACCONTROL21, def: 0x38 },
    reg_default { reg: ES8323_DACCONTROL22, def: 0x38 },
    reg_default { reg: ES8323_DACCONTROL23, def: 0x00 },
    reg_default { reg: ES8323_LOUT1_VOL, def: 0x00 },
    reg_default { reg: ES8323_ROUT1_VOL, def: 0x00 },
];

static es8323_stereo_3d_texts: [&[u8]; 8] =
    [b"No 3D  \0", b"Level 1\0", b"Level 2\0", b"Level 3\0", b"Level 4\0", b"Level 5\0", b"Level 6\0", b"Level 7\0"];
SOC_ENUM_SINGLE_DECL!(es8323_stereo_3d_enum, ES8323_DACCONTROL7, 2, es8323_stereo_3d_texts);

static es8323_alc_func_texts: [&[u8]; 4] = [b"Off\0", b"Right\0", b"Left\0", b"Stereo\0"];
SOC_ENUM_SINGLE_DECL!(es8323_alc_function_enum, ES8323_ADCCONTROL10, 6, es8323_alc_func_texts);

static es8323_ng_type_texts: [&[u8]; 2] = [b"Constant PGA Gain\0", b"Mute ADC Output\0"];
SOC_ENUM_SINGLE_DECL!(es8323_alc_ng_type_enum, ES8323_ADCCONTROL14, 1, es8323_ng_type_texts);

static es8323_deemph_texts: [&[u8]; 4] = [b"None\0", b"32Khz\0", b"44.1Khz\0", b"48Khz\0"];
SOC_ENUM_SINGLE_DECL!(es8323_playback_deemphasis_enum, ES8323_DACCONTROL6, 6, es8323_deemph_texts);

static es8323_adcpol_texts: [&[u8]; 4] = [b"Normal\0", b"L Invert\0", b"R Invert\0", b"L + R Invert\0"];
SOC_ENUM_SINGLE_DECL!(es8323_capture_polarity_enum, ES8323_ADCCONTROL6, 6, es8323_adcpol_texts);

DECLARE_TLV_DB_SCALE!(es8323_adc_tlv, -9600, 50, 1);
DECLARE_TLV_DB_SCALE!(es8323_dac_tlv, -9600, 50, 1);
DECLARE_TLV_DB_SCALE!(es8323_out_tlv, -4500, 150, 0);
DECLARE_TLV_DB_SCALE!(es8323_bypass_tlv, 0, 300, 0);
DECLARE_TLV_DB_SCALE!(es8323_bypass_tlv2, -15, 300, 0);

static es8323_snd_controls: [snd_kcontrol_new; 21] = [
    SOC_ENUM!(b"3D Mode\0", es8323_stereo_3d_enum),
    SOC_ENUM!(b"ALC Capture Function\0", es8323_alc_function_enum),
    SOC_ENUM!(b"ALC Capture NG Type\0", es8323_alc_ng_type_enum),
    SOC_ENUM!(b"Playback De-emphasis\0", es8323_playback_deemphasis_enum),
    SOC_ENUM!(b"Capture Polarity\0", es8323_capture_polarity_enum),
    SOC_SINGLE!(b"ALC Capture ZC Switch\0", ES8323_ADCCONTROL13, ES8323_ADCCONTROL13_ALCZC_OFF, 1, 0),
    SOC_SINGLE!(b"ALC Capture Decay Time\0", ES8323_ADCCONTROL12, ES8323_ADCCONTROL12_ALCDCY_OFF, 15, 0),
    SOC_SINGLE!(b"ALC Capture Attack Time\0", ES8323_ADCCONTROL12, ES8323_ADCCONTROL12_ALCATK_OFF, 15, 0),
    SOC_SINGLE!(b"ALC Capture NG Threshold\0", ES8323_ADCCONTROL14, ES8323_ADCCONTROL14_NGTH_OFF, 31, 0),
    SOC_SINGLE!(b"ALC Capture NG Switch\0", ES8323_ADCCONTROL14, ES8323_ADCCONTROL14_NGAT_OFF, 1, 0),
    SOC_SINGLE!(b"ZC Timeout Switch\0", ES8323_ADCCONTROL13, ES8323_ADCCONTROL13_TIMEOUT_OFF, 1, 0),
    SOC_SINGLE!(b"Capture Mute Switch\0", ES8323_ADCCONTROL7, ES8323_ADCCONTROL7_ADCMUTE_OFF, 1, 0),
    SOC_SINGLE_TLV!(b"Left Channel Capture Volume\0", ES8323_ADCCONTROL1, ES8323_ADCCONTROL1_MICAMPL_OFF, 8, 0, es8323_bypass_tlv),
    SOC_SINGLE_TLV!(b"Right Channel Capture Volume\0", ES8323_ADCCONTROL1, ES8323_ADCCONTROL1_MICAMPR_OFF, 8, 0, es8323_bypass_tlv),
    SOC_SINGLE_TLV!(b"Left Mixer Left Bypass Volume\0", ES8323_DACCONTROL17, ES8323_DACCONTROL17_LI2LOVOL_OFF, 7, 1, es8323_bypass_tlv2),
    SOC_SINGLE_TLV!(b"Right Mixer Right Bypass Volume\0", ES8323_DACCONTROL20, ES8323_DACCONTROL20_RI2ROVOL_OFF, 7, 1, es8323_bypass_tlv2),
    SOC_DOUBLE_R_TLV!(b"PCM Volume\0", ES8323_LDAC_VOL, ES8323_RDAC_VOL, 0, 192, 1, es8323_dac_tlv),
    SOC_DOUBLE_R_TLV!(b"Capture Digital Volume\0", ES8323_LADC_VOL, ES8323_RADC_VOL, 0, 192, 1, es8323_adc_tlv),
    SOC_DOUBLE_R_TLV!(b"Output 1 Playback Volume\0", ES8323_LOUT1_VOL, ES8323_ROUT1_VOL, 0, 33, 0, es8323_out_tlv),
    SOC_DOUBLE_R_TLV!(b"Output 2 Playback Volume\0", ES8323_LOUT2_VOL, ES8323_ROUT2_VOL, 0, 33, 0, es8323_out_tlv),
];

/* Left DAC Route */
static es8323_pga_sell: [&[u8]; 4] = [b"Line 1L\0", b"Line 2L\0", b"NC\0", b"DifferentialL\0"];
SOC_ENUM_SINGLE_DECL!(es8323_left_dac_enum, ES8323_ADCCONTROL2, 6, es8323_pga_sell);
static es8323_left_dac_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"Left DAC Route\0", es8323_left_dac_enum);

/* Right DAC Route */
static es8323_pga_selr: [&[u8]; 4] = [b"Line 1R\0", b"Line 2R\0", b"NC\0", b"DifferentialR\0"];
SOC_ENUM_SINGLE_DECL!(es8323_right_dac_enum, ES8323_ADCCONTROL2, 4, es8323_pga_selr);
static es8323_right_dac_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"Right DAC Route\0", es8323_right_dac_enum);

/* Left Line Mux */
static es8323_lin_sell: [&[u8]; 4] = [b"Line 1L\0", b"Line 2L\0", b"NC\0", b"MicL\0"];
SOC_ENUM_SINGLE_DECL!(es8323_llin_enum, ES8323_DACCONTROL16, 3, es8323_lin_sell);
static es8323_left_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"LLIN Mux\0", es8323_llin_enum);

/* Right Line Mux */
static es8323_lin_selr: [&[u8]; 4] = [b"Line 1R\0", b"Line 2R\0", b"NC\0", b"MicR\0"];
SOC_ENUM_SINGLE_DECL!(es8323_rlin_enum, ES8323_DACCONTROL16, 0, es8323_lin_selr);
static es8323_right_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"RLIN Mux\0", es8323_rlin_enum);

/* Differential Mux */
static es8323_diffmux_sel: [&[u8]; 2] = [b"Line 1\0", b"Line 2\0"];
SOC_ENUM_SINGLE_DECL!(es8323_diffmux_enum, ES8323_ADCCONTROL3, 7, es8323_diffmux_sel);
static es8323_diffmux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"Route2\0", es8323_diffmux_enum);

/* Mono ADC Mux */
static es8323_mono_adc_mux: [&[u8]; 3] = [b"Stereo\0", b"Mono (Left)\0", b"Mono (Right)\0"];
SOC_ENUM_SINGLE_DECL!(es8323_mono_adc_mux_enum, ES8323_ADCCONTROL3, 3, es8323_mono_adc_mux);
static es8323_mono_adc_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"Mono Mux\0", es8323_mono_adc_mux_enum);

/* Left Mixer */
static es8323_left_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(b"Left Playback Switch\0", ES8323_DACCONTROL17, 7, 1, 0),
    SOC_DAPM_SINGLE!(b"Left Bypass Switch\0", ES8323_DACCONTROL17, 6, 1, 0),
];

/* Right Mixer */
static es8323_right_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(b"Right Playback Switch\0", ES8323_DACCONTROL20, 7, 1, 0),
    SOC_DAPM_SINGLE!(b"Right Bypass Switch\0", ES8323_DACCONTROL20, 6, 1, 0),
];

static es8323_dapm_widgets: [snd_soc_dapm_widget; 33] = [
    SND_SOC_DAPM_INPUT!(b"LINPUT1\0"),
    SND_SOC_DAPM_INPUT!(b"LINPUT2\0"),
    SND_SOC_DAPM_INPUT!(b"RINPUT1\0"),
    SND_SOC_DAPM_INPUT!(b"RINPUT2\0"),
    SND_SOC_DAPM_SUPPLY!(b"Mic Bias\0", ES8323_ADCPOWER, ES8323_ADCPOWER_PDNMICB_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Mic Bias Gen\0", ES8323_ADCPOWER, ES8323_ADCPOWER_PDNADCBIS_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"DAC STM\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_DACSTM_RESET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"ADC STM\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_ADCSTM_RESET, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"DAC DIG\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_DACDIG_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"ADC DIG\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_ADCDIG_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"DAC DLL\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_DACDLL_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"ADC DLL\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_ADCDLL_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"ADC Vref\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_ADCVREF_OFF, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!(b"DAC Vref\0", ES8323_CHIPPOWER, ES8323_CHIPPOWER_DACVREF_OFF, 1, core::ptr::null_mut(), 0),
    /* Muxes */
    SND_SOC_DAPM_MUX!(b"Left PGA Mux\0", ES8323_ADCPOWER, ES8323_ADCPOWER_PDNAINL_OFF, 1, &es8323_left_dac_mux_controls),
    SND_SOC_DAPM_MUX!(b"Right PGA Mux\0", ES8323_ADCPOWER, ES8323_ADCPOWER_PDNAINR_OFF, 1, &es8323_right_dac_mux_controls),
    SND_SOC_DAPM_MUX!(b"Differential Mux\0", SND_SOC_NOPM, 0, 0, &es8323_diffmux_controls),
    SND_SOC_DAPM_MUX!(b"Left ADC Mux\0", SND_SOC_NOPM, 0, 0, &es8323_mono_adc_mux_controls),
    SND_SOC_DAPM_MUX!(b"Right ADC Mux\0", SND_SOC_NOPM, 0, 0, &es8323_mono_adc_mux_controls),
    SND_SOC_DAPM_MUX!(b"Left Line Mux\0", SND_SOC_NOPM, 0, 0, &es8323_left_line_controls),
    SND_SOC_DAPM_MUX!(b"Right Line Mux\0", SND_SOC_NOPM, 0, 0, &es8323_right_line_controls),
    SND_SOC_DAPM_ADC!(b"Right ADC\0", b"Right Capture\0", ES8323_ADCPOWER, ES8323_ADCPOWER_PDNADCR_OFF, 1),
    SND_SOC_DAPM_ADC!(b"Left ADC\0", b"Left Capture\0", ES8323_ADCPOWER, ES8323_ADCPOWER_PDNADCL_OFF, 1),
    SND_SOC_DAPM_DAC!(b"Right DAC\0", b"Right Playback\0", ES8323_DACPOWER, ES8323_DACPOWER_PDNDACR_OFF, 1),
    SND_SOC_DAPM_DAC!(b"Left DAC\0", b"Left Playback\0", ES8323_DACPOWER, ES8323_DACPOWER_PDNDACL_OFF, 1),
    SND_SOC_DAPM_MIXER!(b"Left Mixer\0", SND_SOC_NOPM, 0, 0, &es8323_left_mixer_controls[0], es8323_left_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!(b"Right Mixer\0", SND_SOC_NOPM, 0, 0, &es8323_right_mixer_controls[0], es8323_right_mixer_controls.len()),
    SND_SOC_DAPM_PGA!(b"Right Out 2\0", ES8323_DACPOWER, ES8323_DACPOWER_ROUT2_OFF, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(b"Left Out 2\0", ES8323_DACPOWER, ES8323_DACPOWER_LOUT2_OFF, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(b"Right Out 1\0", ES8323_DACPOWER, ES8323_DACPOWER_ROUT1_OFF, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(b"Left Out 1\0", ES8323_DACPOWER, ES8323_DACPOWER_LOUT1_OFF, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(b"LAMP\0", ES8323_ADCCONTROL1, ES8323_ADCCONTROL1_MICAMPL_OFF, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(b"RAMP\0", ES8323_ADCCONTROL1, ES8323_ADCCONTROL1_MICAMPR_OFF, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT!(b"LOUT1\0"),
    SND_SOC_DAPM_OUTPUT!(b"ROUT1\0"),
    SND_SOC_DAPM_OUTPUT!(b"LOUT2\0"),
    SND_SOC_DAPM_OUTPUT!(b"ROUT2\0"),
    SND_SOC_DAPM_OUTPUT!(b"VREF\0"),
];

static es8323_dapm_routes: [snd_soc_dapm_route; 49] = [
    /*12.22*/
    route(b"Left PGA Mux\0", b"Line 1L\0", b"LINPUT1\0"),
    route(b"Left PGA Mux\0", b"Line 2L\0", b"LINPUT2\0"),
    route(b"Left PGA Mux\0", b"DifferentialL\0", b"Differential Mux\0"),
    route(b"Right PGA Mux\0", b"Line 1R\0", b"RINPUT1\0"),
    route(b"Right PGA Mux\0", b"Line 2R\0", b"RINPUT2\0"),
    route(b"Right PGA Mux\0", b"DifferentialR\0", b"Differential Mux\0"),
    route(b"Differential Mux\0", b"Line 1\0", b"LINPUT1\0"),
    route(b"Differential Mux\0", b"Line 1\0", b"RINPUT1\0"),
    route(b"Differential Mux\0", b"Line 2\0", b"LINPUT2\0"),
    route(b"Differential Mux\0", b"Line 2\0", b"RINPUT2\0"),
    route(b"Left ADC Mux\0", b"Stereo\0", b"Left PGA Mux\0"),
    route(b"Left ADC Mux\0", b"Mono (Left)\0", b"Left PGA Mux\0"),
    route(b"Right ADC Mux\0", b"Stereo\0", b"Right PGA Mux\0"),
    route(b"Right ADC Mux\0", b"Mono (Right)\0", b"Right PGA Mux\0"),
    route_null(b"Left ADC\0", b"Left ADC Mux\0"),
    route_null(b"Right ADC\0", b"Right ADC Mux\0"),
    route_null(b"Mic Bias\0", b"Mic Bias Gen\0"),
    route_null(b"ADC DIG\0", b"ADC STM\0"),
    route_null(b"ADC DIG\0", b"ADC Vref\0"),
    route_null(b"ADC DIG\0", b"ADC DLL\0"),
    route_null(b"Left ADC\0", b"ADC DIG\0"),
    route_null(b"Right ADC\0", b"ADC DIG\0"),
    route_null(b"DAC DIG\0", b"DAC STM\0"),
    route_null(b"DAC DIG\0", b"DAC Vref\0"),
    route_null(b"DAC DIG\0", b"DAC DLL\0"),
    route_null(b"Left DAC\0", b"DAC DIG\0"),
    route_null(b"Right DAC\0", b"DAC DIG\0"),
    route(b"Left Line Mux\0", b"Line 1L\0", b"LINPUT1\0"),
    route(b"Left Line Mux\0", b"Line 2L\0", b"LINPUT2\0"),
    route(b"Left Line Mux\0", b"MicL\0", b"Left PGA Mux\0"),
    route(b"Right Line Mux\0", b"Line 1R\0", b"RINPUT1\0"),
    route(b"Right Line Mux\0", b"Line 2R\0", b"RINPUT2\0"),
    route(b"Right Line Mux\0", b"MicR\0", b"Right PGA Mux\0"),
    route(b"Left Mixer\0", b"Left Playback Switch\0", b"Left DAC\0"),
    route(b"Left Mixer\0", b"Left Bypass Switch\0", b"Left Line Mux\0"),
    route(b"Right Mixer\0", b"Right Playback Switch\0", b"Right DAC\0"),
    route(b"Right Mixer\0", b"Right Bypass Switch\0", b"Right Line Mux\0"),
    route_null(b"Left Out 1\0", b"Left Mixer\0"),
    route_null(b"LOUT1\0", b"Left Out 1\0"),
    route_null(b"Right Out 1\0", b"Right Mixer\0"),
    route_null(b"ROUT1\0", b"Right Out 1\0"),
    route_null(b"Left Out 2\0", b"Left Mixer\0"),
    route_null(b"LOUT2\0", b"Left Out 2\0"),
    route_null(b"Right Out 2\0", b"Right Mixer\0"),
    route_null(b"ROUT2\0", b"Right Out 2\0"),
];

const fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

const fn route(sink: &'static [u8], control: &'static [u8], source: &'static [u8]) -> snd_soc_dapm_route {
    snd_soc_dapm_route {
        sink: cstr(sink),
        control: cstr(control),
        source: cstr(source),
    }
}

const fn route_null(sink: &'static [u8], source: &'static [u8]) -> snd_soc_dapm_route {
    snd_soc_dapm_route {
        sink: cstr(sink),
        control: core::ptr::null(),
        source: cstr(source),
    }
}

#[repr(C)]
pub struct coeff_div {
    pub mclk: u32,
    pub rate: u32,
    pub fs: u16,
    pub sr_usb: u8,
}

impl coeff_div {
    const fn new(mclk: u32, rate: u32, fs: u16, sr: u8, usb: u8) -> Self {
        Self {
            mclk,
            rate,
            fs,
            sr_usb: (sr & 0x0f) | ((usb & 0x01) << 4),
        }
    }

    const fn sr(&self) -> u8 {
        self.sr_usb & 0x0f
    }

    const fn usb(&self) -> u8 {
        (self.sr_usb >> 4) & 0x01
    }
}

/* codec hifi mclk clock divider coefficients */
static es8323_coeff_div: [coeff_div; 30] = [
    /* 8k */
    coeff_div::new(12288000, 8000, 1536, 0xa, 0x0),
    coeff_div::new(11289600, 8000, 1408, 0x9, 0x0),
    coeff_div::new(18432000, 8000, 2304, 0xc, 0x0),
    coeff_div::new(16934400, 8000, 2112, 0xb, 0x0),
    coeff_div::new(12000000, 8000, 1500, 0xb, 0x1),
    /* 11.025k */
    coeff_div::new(11289600, 11025, 1024, 0x7, 0x0),
    coeff_div::new(16934400, 11025, 1536, 0xa, 0x0),
    coeff_div::new(12000000, 11025, 1088, 0x9, 0x1),
    /* 16k */
    coeff_div::new(12288000, 16000, 768, 0x6, 0x0),
    coeff_div::new(18432000, 16000, 1152, 0x8, 0x0),
    coeff_div::new(12000000, 16000, 750, 0x7, 0x1),
    /* 22.05k */
    coeff_div::new(11289600, 22050, 512, 0x4, 0x0),
    coeff_div::new(16934400, 22050, 768, 0x6, 0x0),
    coeff_div::new(12000000, 22050, 544, 0x6, 0x1),
    /* 32k */
    coeff_div::new(12288000, 32000, 384, 0x3, 0x0),
    coeff_div::new(18432000, 32000, 576, 0x5, 0x0),
    coeff_div::new(12000000, 32000, 375, 0x4, 0x1),
    /* 44.1k */
    coeff_div::new(11289600, 44100, 256, 0x2, 0x0),
    coeff_div::new(16934400, 44100, 384, 0x3, 0x0),
    coeff_div::new(12000000, 44100, 272, 0x3, 0x1),
    /* 48k */
    coeff_div::new(12288000, 48000, 256, 0x2, 0x0),
    coeff_div::new(18432000, 48000, 384, 0x3, 0x0),
    coeff_div::new(12000000, 48000, 250, 0x2, 0x1),
    /* 88.2k */
    coeff_div::new(11289600, 88200, 128, 0x0, 0x0),
    coeff_div::new(16934400, 88200, 192, 0x1, 0x0),
    coeff_div::new(12000000, 88200, 136, 0x1, 0x1),
    /* 96k */
    coeff_div::new(12288000, 96000, 128, 0x0, 0x0),
    coeff_div::new(18432000, 96000, 192, 0x1, 0x0),
    coeff_div::new(12000000, 96000, 125, 0x0, 0x1),
];

static mut rates_12288: [c_uint; 8] = [8000, 12000, 16000, 24000, 24000, 32000, 48000, 96000];

static mut constraints_12288: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 8,
    list: unsafe { rates_12288.as_mut_ptr() },
};

static mut rates_112896: [c_uint; 4] = [8000, 11025, 22050, 44100];

static mut constraints_112896: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 4,
    list: unsafe { rates_112896.as_mut_ptr() },
};

static mut rates_12: [c_uint; 12] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 48000, 88235, 96000];

static mut constraints_12: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 12,
    list: unsafe { rates_12.as_mut_ptr() },
};

#[inline]
unsafe fn get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;

    while i < es8323_coeff_div.len() {
        if es8323_coeff_div[i].rate == rate as u32 && es8323_coeff_div[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }

    -EINVAL
}

unsafe extern "C" fn es8323_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;

    match freq {
        11289600 | 18432000 | 22579200 | 36864000 => {
            (*es8323).sysclk_constraints = &raw mut constraints_112896;
        }
        12288000 | 16934400 | 24576000 | 33868800 => {
            (*es8323).sysclk_constraints = &raw mut constraints_12288;
        }
        12000000 | 24000000 => {
            (*es8323).sysclk_constraints = &raw mut constraints_12;
        }
        _ => return -EINVAL,
    }

    (*es8323).sysclk = freq;
    0
}

unsafe extern "C" fn es8323_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let format_mode: u8;
    let inv_mode: u8;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_BC_FP => {
            /* Master serial port mode */
            snd_soc_component_update_bits(
                component,
                ES8323_MASTERMODE,
                ES8323_MASTERMODE_MSC,
                ES8323_MASTERMODE_MSC,
            );
        }
        SND_SOC_DAIFMT_BC_FC => {
            /* Slave serial port mode */
            snd_soc_component_update_bits(component, ES8323_MASTERMODE, ES8323_MASTERMODE_MSC, 0);
        }
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => format_mode = ES8323_FMT_I2S as u8,
        SND_SOC_DAIFMT_LEFT_J => format_mode = ES8323_FMT_LEFT_J as u8,
        SND_SOC_DAIFMT_RIGHT_J => format_mode = ES8323_FMT_RIGHT_J as u8,
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => format_mode = ES8323_FMT_DSP as u8,
        _ => return -EINVAL,
    }

    snd_soc_component_write_field(component, ES8323_ADCCONTROL4, ES8323_ADCCONTROL4_ADCFORMAT, format_mode as c_uint);
    snd_soc_component_write_field(component, ES8323_DACCONTROL1, ES8323_DACCONTROL1_DACFORMAT, format_mode as c_uint);

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => inv_mode = 0,
        SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_NB_IF => inv_mode = 1,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, ES8323_MASTERMODE, ES8323_MASTERMODE_BCLKINV, inv_mode as c_uint);
    snd_soc_component_update_bits(component, ES8323_ADCCONTROL4, ES8323_ADCCONTROL4_ADCLRP, inv_mode as c_uint);
    snd_soc_component_update_bits(component, ES8323_DACCONTROL1, ES8323_DACCONTROL1_DACLRP, inv_mode as c_uint);

    0
}

unsafe extern "C" fn es8323_pcm_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;

    if (*es8323).sysclk != 0 {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            (*es8323).sysclk_constraints,
        );
    }

    0
}

unsafe extern "C" fn es8323_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;
    let wl_mode: u8;
    let fs: u8;
    let mut coeff: c_int;

    coeff = get_coeff((*es8323).sysclk as c_int, params_rate(params) as c_int);
    if coeff < 0 {
        coeff = get_coeff(((*es8323).sysclk / 2) as c_int, params_rate(params) as c_int);
        if coeff < 0 {
            dev_err(
                (*component).dev,
                b"Unable to configure sample rate %dHz with %dHz MCLK\n\0".as_ptr() as *const c_char,
                params_rate(params),
                (*es8323).sysclk,
            );
            return coeff;
        }

        snd_soc_component_update_bits(
            component,
            ES8323_MASTERMODE,
            ES8323_MASTERMODE_MCLKDIV2,
            ES8323_MASTERMODE_MCLKDIV2,
        );
    }

    fs = (FIELD_PREP!(ES8323_DACCONTROL2_DACFSMODE, es8323_coeff_div[coeff as usize].usb()) |
        FIELD_PREP!(ES8323_DACCONTROL2_DACFSRATIO, es8323_coeff_div[coeff as usize].sr())) as u8;

    snd_soc_component_write_field(component, ES8323_ADCCONTROL5, ES8323_ADCCONTROL5_ADCFS_MASK, fs as c_uint);

    snd_soc_component_write_field(component, ES8323_DACCONTROL2, ES8323_DACCONTROL2_DACFS_MASK, fs as c_uint);

    /* serial audio data word length */
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => wl_mode = ES8323_S16_LE as u8,
        SNDRV_PCM_FORMAT_S20_3LE => wl_mode = ES8323_S20_LE as u8,
        SNDRV_PCM_FORMAT_S24_LE => wl_mode = ES8323_S24_LE as u8,
        SNDRV_PCM_FORMAT_S32_LE => wl_mode = ES8323_S32_LE as u8,
        _ => return -EINVAL,
    }

    snd_soc_component_write_field(component, ES8323_ADCCONTROL4, ES8323_ADCCONTROL4_ADCWL, wl_mode as c_uint);

    snd_soc_component_write_field(component, ES8323_DACCONTROL1, ES8323_DACCONTROL1_DACWL, wl_mode as c_uint);

    0
}

unsafe extern "C" fn es8323_mute_stream(dai: *mut snd_soc_dai, mute: c_int, _stream: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        ES8323_DACCONTROL3,
        ES8323_DACCONTROL3_DACMUTE,
        if mute != 0 { ES8323_DACCONTROL3_DACMUTE } else { 0 },
    )
}

static es8323_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(es8323_pcm_startup),
    hw_params: Some(es8323_pcm_hw_params),
    set_fmt: Some(es8323_set_dai_fmt),
    set_sysclk: Some(es8323_set_dai_sysclk),
    mute_stream: Some(es8323_mute_stream),
};

const ES8323_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static mut es8323_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ES8323 HiFi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: ES8323_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: ES8323_FORMATS,
    },
    ops: &es8323_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn es8323_probe(component: *mut snd_soc_component) -> c_int {
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;
    let ret: c_int;

    (*es8323).mclk = devm_clk_get_optional((*component).dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*es8323).mclk as *const c_void) {
        dev_err((*component).dev, b"unable to get mclk\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*es8323).mclk as *const c_void);
    }

    if (*es8323).mclk.is_null() {
        dev_warn((*component).dev, b"assuming static mclk\n\0".as_ptr() as *const c_char);
    }

    ret = clk_prepare_enable((*es8323).mclk);
    if ret != 0 {
        dev_err((*component).dev, b"unable to enable mclk\n\0".as_ptr() as *const c_char);
        return ret;
    }

    snd_soc_component_write(component, ES8323_CONTROL2, 0x60);
    snd_soc_component_write(component, ES8323_DACCONTROL21, 0x80);

    0
}

unsafe extern "C" fn es8323_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;
    let ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            ret = clk_prepare_enable((*es8323).mclk);
            if ret != 0 {
                return ret;
            }

            snd_soc_component_write(component, ES8323_CHIPLOPOW1, 0x00);
            snd_soc_component_write(component, ES8323_CHIPLOPOW2, 0x00);
            snd_soc_component_update_bits(component, ES8323_ADCPOWER, ES8323_ADCPOWER_PDNADCBIS, 0);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {}
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, ES8323_CHIPLOPOW1, 0xff);
            snd_soc_component_write(component, ES8323_CHIPLOPOW2, 0xff);
            clk_disable_unprepare((*es8323).mclk);
        }
    }

    0
}

unsafe extern "C" fn es8323_remove(component: *mut snd_soc_component) {
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;

    clk_disable_unprepare((*es8323).mclk);
    es8323_set_bias_level(component, snd_soc_bias_level::SND_SOC_BIAS_OFF);
}

unsafe extern "C" fn es8323_suspend(component: *mut snd_soc_component) -> c_int {
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;

    regcache_cache_only((*es8323).regmap, true);
    regcache_mark_dirty((*es8323).regmap);

    0
}

unsafe extern "C" fn es8323_resume(component: *mut snd_soc_component) -> c_int {
    let es8323: *mut es8323_priv = snd_soc_component_get_drvdata(component) as *mut es8323_priv;

    regcache_cache_only((*es8323).regmap, false);
    regcache_sync((*es8323).regmap);

    0
}

static soc_component_dev_es8323: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es8323_probe),
    remove: Some(es8323_remove),
    suspend: Some(es8323_suspend),
    resume: Some(es8323_resume),
    set_bias_level: Some(es8323_set_bias_level),
    controls: es8323_snd_controls.as_ptr(),
    num_controls: es8323_snd_controls.len() as c_uint,
    dapm_widgets: es8323_dapm_widgets.as_ptr(),
    num_dapm_widgets: es8323_dapm_widgets.len() as c_uint,
    dapm_routes: es8323_dapm_routes.as_ptr(),
    num_dapm_routes: es8323_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static es8323_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    use_single_read: true,
    use_single_write: true,
    max_register: 0x53,
    reg_defaults: es8323_reg_defaults.as_ptr(),
    num_reg_defaults: es8323_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn es8323_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let es8323: *mut es8323_priv;
    let dev: *mut device = &raw mut (*i2c_client).dev;

    es8323 = devm_kzalloc(dev, core::mem::size_of::<es8323_priv>(), GFP_KERNEL) as *mut es8323_priv;
    if es8323.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c_client, es8323 as *mut c_void);

    (*es8323).regmap = devm_regmap_init_i2c(i2c_client, &es8323_regmap);
    if IS_ERR((*es8323).regmap as *const c_void) {
        return PTR_ERR((*es8323).regmap as *const c_void);
    }

    devm_snd_soc_register_component(dev, &soc_component_dev_es8323, &raw mut es8323_dai, 1)
}

static es8323_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'e' as c_char, b's' as c_char, b'8' as c_char, b'3' as c_char, b'2' as c_char, b'3' as c_char, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
MODULE_DEVICE_TABLE!(i2c, es8323_i2c_id);

static es8323_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'E' as c_char, b'S' as c_char, b'S' as c_char, b'X' as c_char, b'8' as c_char, b'3' as c_char,
            b'2' as c_char, b'3' as c_char, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(acpi, es8323_acpi_match);

static es8323_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"everest,es8323\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, es8323_of_match);

static mut es8323_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ES8323\0".as_ptr() as *const c_char,
        acpi_match_table: es8323_acpi_match.as_ptr(),
        of_match_table: es8323_of_match.as_ptr(),
    },
    probe: Some(es8323_i2c_probe),
    id_table: es8323_i2c_id.as_ptr(),
};
module_i2c_driver!(es8323_i2c_driver);

MODULE_DESCRIPTION!(b"Everest Semi ES8323 ALSA SoC Codec Driver\0");
MODULE_AUTHOR!(b"Mark Brown <broonie@kernel.org>\0");
MODULE_AUTHOR!(b"Binbin Zhou <zhoubinbin@loongson.cn>\0");
MODULE_LICENSE!(b"GPL\0");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
