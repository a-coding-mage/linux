// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8940.c  --  WM8940 ALSA Soc Audio driver
 *
 * Author: Jonathan Cameron <jic23@cam.ac.uk>
 *
 * Based on wm8510.c
 *    Copyright  2006 Wolfson Microelectronics PLC.
 *    Author:  Liam Girdwood <lrg@slimlogic.co.uk>
 *
 * Not currently handled:
 * Notch filter control
 * AUXMode (inverting vs mixer)
 * No means to obtain current gain if alc enabled.
 * No use made of gpio
 * Fast VMID discharge for power down
 * Soft Start
 * DLR and ALR Swaps not enabled
 * Digital Sidetone not supported
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

// Dependencies originally supplied by Linux, ALSA SoC, regmap, and "wm8940.h".

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
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
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub no_capture_mute: c_uint,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct wm8940_setup_data {
    pub vroi: u16,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct wm8940_priv {
    pub mclk: c_uint,
    pub fs: c_uint,

    pub regmap: *mut regmap,
}

extern "C" {
    static WM8940_SOFTRESET: c_uint;
    static WM8940_POWER1: c_uint;
    static WM8940_POWER2: c_uint;
    static WM8940_POWER3: c_uint;
    static WM8940_IFACE: c_uint;
    static WM8940_COMPANDINGCTL: c_uint;
    static WM8940_CLOCK: c_uint;
    static WM8940_ADDCNTRL: c_uint;
    static WM8940_GPIO: c_uint;
    static WM8940_CTLINT: c_uint;
    static WM8940_DAC: c_uint;
    static WM8940_DACVOL: c_uint;
    static WM8940_ADC: c_uint;
    static WM8940_ADCVOL: c_uint;
    static WM8940_NOTCH1: c_uint;
    static WM8940_NOTCH2: c_uint;
    static WM8940_NOTCH3: c_uint;
    static WM8940_NOTCH4: c_uint;
    static WM8940_NOTCH5: c_uint;
    static WM8940_NOTCH6: c_uint;
    static WM8940_NOTCH7: c_uint;
    static WM8940_NOTCH8: c_uint;
    static WM8940_DACLIM1: c_uint;
    static WM8940_DACLIM2: c_uint;
    static WM8940_ALC1: c_uint;
    static WM8940_ALC2: c_uint;
    static WM8940_ALC3: c_uint;
    static WM8940_NOISEGATE: c_uint;
    static WM8940_PLLN: c_uint;
    static WM8940_PLLK1: c_uint;
    static WM8940_PLLK2: c_uint;
    static WM8940_PLLK3: c_uint;
    static WM8940_ALC4: c_uint;
    static WM8940_INPUTCTL: c_uint;
    static WM8940_PGAGAIN: c_uint;
    static WM8940_ADCBOOST: c_uint;
    static WM8940_OUTPUTCTL: c_uint;
    static WM8940_SPKMIX: c_uint;
    static WM8940_SPKVOL: c_uint;
    static WM8940_MONOMIX: c_uint;
    static WM8940_CHIP_ID: u16;
    static WM8940_BCLKDIV: c_int;
    static WM8940_MCLKDIV: c_int;
    static WM8940_OPCLKDIV: c_int;
    static WM8940_MCLKDIV_1: c_int;
    static WM8940_MCLKDIV_1_5: c_int;
    static WM8940_MCLKDIV_2: c_int;
    static WM8940_MCLKDIV_3: c_int;
    static WM8940_MCLKDIV_4: c_int;
    static WM8940_MCLKDIV_6: c_int;
    static WM8940_MCLKDIV_8: c_int;
    static WM8940_MCLKDIV_12: c_int;

    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S32_LE: c_ulonglong;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;

    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> u16;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: u16) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const FIXED_PLL_SIZE: c_ulonglong = ((1u64 << 24) * 10) as c_ulonglong;

unsafe extern "C" fn wm8940_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == WM8940_SOFTRESET => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8940_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == WM8940_SOFTRESET
            || x == WM8940_POWER1
            || x == WM8940_POWER2
            || x == WM8940_POWER3
            || x == WM8940_IFACE
            || x == WM8940_COMPANDINGCTL
            || x == WM8940_CLOCK
            || x == WM8940_ADDCNTRL
            || x == WM8940_GPIO
            || x == WM8940_CTLINT
            || x == WM8940_DAC
            || x == WM8940_DACVOL
            || x == WM8940_ADC
            || x == WM8940_ADCVOL
            || x == WM8940_NOTCH1
            || x == WM8940_NOTCH2
            || x == WM8940_NOTCH3
            || x == WM8940_NOTCH4
            || x == WM8940_NOTCH5
            || x == WM8940_NOTCH6
            || x == WM8940_NOTCH7
            || x == WM8940_NOTCH8
            || x == WM8940_DACLIM1
            || x == WM8940_DACLIM2
            || x == WM8940_ALC1
            || x == WM8940_ALC2
            || x == WM8940_ALC3
            || x == WM8940_NOISEGATE
            || x == WM8940_PLLN
            || x == WM8940_PLLK1
            || x == WM8940_PLLK2
            || x == WM8940_PLLK3
            || x == WM8940_ALC4
            || x == WM8940_INPUTCTL
            || x == WM8940_PGAGAIN
            || x == WM8940_ADCBOOST
            || x == WM8940_OUTPUTCTL
            || x == WM8940_SPKMIX
            || x == WM8940_SPKVOL
            || x == WM8940_MONOMIX =>
        {
            true
        }
        _ => false,
    }
}

static wm8940_reg_defaults: [reg_default; 40] = [
    reg_default { reg: 0x1, def: 0x0000 }, /* Power 1 */
    reg_default { reg: 0x2, def: 0x0000 }, /* Power 2 */
    reg_default { reg: 0x3, def: 0x0000 }, /* Power 3 */
    reg_default { reg: 0x4, def: 0x0010 }, /* Interface Control */
    reg_default { reg: 0x5, def: 0x0000 }, /* Companding Control */
    reg_default { reg: 0x6, def: 0x0140 }, /* Clock Control */
    reg_default { reg: 0x7, def: 0x0000 }, /* Additional Controls */
    reg_default { reg: 0x8, def: 0x0000 }, /* GPIO Control */
    reg_default { reg: 0x9, def: 0x0002 }, /* Auto Increment Control */
    reg_default { reg: 0xa, def: 0x0000 }, /* DAC Control */
    reg_default { reg: 0xb, def: 0x00FF }, /* DAC Volume */
    reg_default { reg: 0xe, def: 0x0100 }, /* ADC Control */
    reg_default { reg: 0xf, def: 0x00FF }, /* ADC Volume */
    reg_default { reg: 0x10, def: 0x0000 }, /* Notch Filter 1 Control 1 */
    reg_default { reg: 0x11, def: 0x0000 }, /* Notch Filter 1 Control 2 */
    reg_default { reg: 0x12, def: 0x0000 }, /* Notch Filter 2 Control 1 */
    reg_default { reg: 0x13, def: 0x0000 }, /* Notch Filter 2 Control 2 */
    reg_default { reg: 0x14, def: 0x0000 }, /* Notch Filter 3 Control 1 */
    reg_default { reg: 0x15, def: 0x0000 }, /* Notch Filter 3 Control 2 */
    reg_default { reg: 0x16, def: 0x0000 }, /* Notch Filter 4 Control 1 */
    reg_default { reg: 0x17, def: 0x0000 }, /* Notch Filter 4 Control 2 */
    reg_default { reg: 0x18, def: 0x0032 }, /* DAC Limit Control 1 */
    reg_default { reg: 0x19, def: 0x0000 }, /* DAC Limit Control 2 */
    reg_default { reg: 0x20, def: 0x0038 }, /* ALC Control 1 */
    reg_default { reg: 0x21, def: 0x000B }, /* ALC Control 2 */
    reg_default { reg: 0x22, def: 0x0032 }, /* ALC Control 3 */
    reg_default { reg: 0x23, def: 0x0000 }, /* Noise Gate */
    reg_default { reg: 0x24, def: 0x0041 }, /* PLLN */
    reg_default { reg: 0x25, def: 0x000C }, /* PLLK1 */
    reg_default { reg: 0x26, def: 0x0093 }, /* PLLK2 */
    reg_default { reg: 0x27, def: 0x00E9 }, /* PLLK3 */
    reg_default { reg: 0x2a, def: 0x0030 }, /* ALC Control 4 */
    reg_default { reg: 0x2c, def: 0x0002 }, /* Input Control */
    reg_default { reg: 0x2d, def: 0x0050 }, /* PGA Gain */
    reg_default { reg: 0x2f, def: 0x0002 }, /* ADC Boost Control */
    reg_default { reg: 0x31, def: 0x0002 }, /* Output Control */
    reg_default { reg: 0x32, def: 0x0000 }, /* Speaker Mixer Control */
    reg_default { reg: 0x36, def: 0x0079 }, /* Speaker Volume */
    reg_default { reg: 0x38, def: 0x0000 }, /* Mono Mixer Control */
    reg_default { reg: 0x0, def: 0x0000 },
];

static wm8940_companding: [*const c_char; 4] = [c"Off".as_ptr(), c"NC".as_ptr(), c"u-law".as_ptr(), c"A-law".as_ptr()];
SOC_ENUM_SINGLE_DECL!(wm8940_adc_companding_enum, WM8940_COMPANDINGCTL, 1, wm8940_companding);
SOC_ENUM_SINGLE_DECL!(wm8940_dac_companding_enum, WM8940_COMPANDINGCTL, 3, wm8940_companding);

static wm8940_alc_mode_text: [*const c_char; 2] = [c"ALC".as_ptr(), c"Limiter".as_ptr()];
SOC_ENUM_SINGLE_DECL!(wm8940_alc_mode_enum, WM8940_ALC3, 8, wm8940_alc_mode_text);

static wm8940_mic_bias_level_text: [*const c_char; 2] = [c"0.9".as_ptr(), c"0.65".as_ptr()];
SOC_ENUM_SINGLE_DECL!(wm8940_mic_bias_level_enum, WM8940_INPUTCTL, 8, wm8940_mic_bias_level_text);

static wm8940_filter_mode_text: [*const c_char; 2] = [c"Audio".as_ptr(), c"Application".as_ptr()];
SOC_ENUM_SINGLE_DECL!(wm8940_filter_mode_enum, WM8940_ADC, 7, wm8940_filter_mode_text);

DECLARE_TLV_DB_SCALE!(wm8940_spk_vol_tlv, -5700, 100, 1);
DECLARE_TLV_DB_SCALE!(wm8940_att_tlv, -1000, 1000, 0);
DECLARE_TLV_DB_SCALE!(wm8940_pga_vol_tlv, -1200, 75, 0);
DECLARE_TLV_DB_SCALE!(wm8940_alc_min_tlv, -1200, 600, 0);
DECLARE_TLV_DB_SCALE!(wm8940_alc_max_tlv, 675, 600, 0);
DECLARE_TLV_DB_SCALE!(wm8940_alc_tar_tlv, -2250, 50, 0);
DECLARE_TLV_DB_SCALE!(wm8940_lim_boost_tlv, 0, 100, 0);
DECLARE_TLV_DB_SCALE!(wm8940_lim_thresh_tlv, -600, 100, 0);
DECLARE_TLV_DB_SCALE!(wm8940_adc_tlv, -12750, 50, 1);
DECLARE_TLV_DB_SCALE!(wm8940_capture_boost_vol_tlv, 0, 2000, 0);

static wm8940_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE!(c"Digital Loopback Switch".as_ptr(), WM8940_COMPANDINGCTL, 6, 1, 0),
    SOC_ENUM!(c"DAC Companding".as_ptr(), wm8940_dac_companding_enum),
    SOC_ENUM!(c"ADC Companding".as_ptr(), wm8940_adc_companding_enum),
    SOC_ENUM!(c"ALC Mode".as_ptr(), wm8940_alc_mode_enum),
    SOC_SINGLE!(c"ALC Switch".as_ptr(), WM8940_ALC1, 8, 1, 0),
    SOC_SINGLE_TLV!(c"ALC Capture Max Gain".as_ptr(), WM8940_ALC1, 3, 7, 1, wm8940_alc_max_tlv),
    SOC_SINGLE_TLV!(c"ALC Capture Min Gain".as_ptr(), WM8940_ALC1, 0, 7, 0, wm8940_alc_min_tlv),
    SOC_SINGLE_TLV!(c"ALC Capture Target".as_ptr(), WM8940_ALC2, 0, 14, 0, wm8940_alc_tar_tlv),
    SOC_SINGLE!(c"ALC Capture Hold".as_ptr(), WM8940_ALC2, 4, 10, 0),
    SOC_SINGLE!(c"ALC Capture Decay".as_ptr(), WM8940_ALC3, 4, 10, 0),
    SOC_SINGLE!(c"ALC Capture Attach".as_ptr(), WM8940_ALC3, 0, 10, 0),
    SOC_SINGLE!(c"ALC ZC Switch".as_ptr(), WM8940_ALC4, 1, 1, 0),
    SOC_SINGLE!(c"ALC Capture Noise Gate Switch".as_ptr(), WM8940_NOISEGATE, 3, 1, 0),
    SOC_SINGLE!(c"ALC Capture Noise Gate Threshold".as_ptr(), WM8940_NOISEGATE, 0, 7, 0),
    SOC_SINGLE!(c"DAC Playback Limiter Switch".as_ptr(), WM8940_DACLIM1, 8, 1, 0),
    SOC_SINGLE!(c"DAC Playback Limiter Attack".as_ptr(), WM8940_DACLIM1, 0, 9, 0),
    SOC_SINGLE!(c"DAC Playback Limiter Decay".as_ptr(), WM8940_DACLIM1, 4, 11, 0),
    SOC_SINGLE_TLV!(c"DAC Playback Limiter Threshold".as_ptr(), WM8940_DACLIM2, 4, 9, 1, wm8940_lim_thresh_tlv),
    SOC_SINGLE_TLV!(c"DAC Playback Limiter Boost".as_ptr(), WM8940_DACLIM2, 0, 12, 0, wm8940_lim_boost_tlv),
    SOC_SINGLE!(c"Capture PGA ZC Switch".as_ptr(), WM8940_PGAGAIN, 7, 1, 0),
    SOC_SINGLE_TLV!(c"Capture PGA Volume".as_ptr(), WM8940_PGAGAIN, 0, 63, 0, wm8940_pga_vol_tlv),
    SOC_SINGLE_TLV!(c"Digital Playback Volume".as_ptr(), WM8940_DACVOL, 0, 255, 0, wm8940_adc_tlv),
    SOC_SINGLE_TLV!(c"Digital Capture Volume".as_ptr(), WM8940_ADCVOL, 0, 255, 0, wm8940_adc_tlv),
    SOC_ENUM!(c"Mic Bias Level".as_ptr(), wm8940_mic_bias_level_enum),
    SOC_SINGLE_TLV!(c"Capture Boost Volume".as_ptr(), WM8940_ADCBOOST, 8, 1, 0, wm8940_capture_boost_vol_tlv),
    SOC_SINGLE_TLV!(c"Speaker Playback Volume".as_ptr(), WM8940_SPKVOL, 0, 63, 0, wm8940_spk_vol_tlv),
    SOC_SINGLE!(c"Speaker Playback Switch".as_ptr(), WM8940_SPKVOL, 6, 1, 1),
    SOC_SINGLE_TLV!(c"Speaker Mixer Line Bypass Volume".as_ptr(), WM8940_SPKVOL, 8, 1, 1, wm8940_att_tlv),
    SOC_SINGLE!(c"Speaker Playback ZC Switch".as_ptr(), WM8940_SPKVOL, 7, 1, 0),
    SOC_SINGLE!(c"Mono Out Switch".as_ptr(), WM8940_MONOMIX, 6, 1, 1),
    SOC_SINGLE_TLV!(c"Mono Mixer Line Bypass Volume".as_ptr(), WM8940_MONOMIX, 7, 1, 1, wm8940_att_tlv),
    SOC_SINGLE!(c"High Pass Filter Switch".as_ptr(), WM8940_ADC, 8, 1, 0),
    SOC_ENUM!(c"High Pass Filter Mode".as_ptr(), wm8940_filter_mode_enum),
    SOC_SINGLE!(c"High Pass Filter Cut Off".as_ptr(), WM8940_ADC, 4, 7, 0),
    SOC_SINGLE!(c"ADC Inversion Switch".as_ptr(), WM8940_ADC, 0, 1, 0),
    SOC_SINGLE!(c"DAC Inversion Switch".as_ptr(), WM8940_DAC, 0, 1, 0),
    SOC_SINGLE!(c"DAC Auto Mute Switch".as_ptr(), WM8940_DAC, 2, 1, 0),
    SOC_SINGLE!(c"ZC Timeout Clock Switch".as_ptr(), WM8940_ADDCNTRL, 0, 1, 0),
];

static wm8940_speaker_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!(c"Line Bypass Switch".as_ptr(), WM8940_SPKMIX, 1, 1, 0),
    SOC_DAPM_SINGLE!(c"Aux Playback Switch".as_ptr(), WM8940_SPKMIX, 5, 1, 0),
    SOC_DAPM_SINGLE!(c"PCM Playback Switch".as_ptr(), WM8940_SPKMIX, 0, 1, 0),
];

static wm8940_mono_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!(c"Line Bypass Switch".as_ptr(), WM8940_MONOMIX, 1, 1, 0),
    SOC_DAPM_SINGLE!(c"Aux Playback Switch".as_ptr(), WM8940_MONOMIX, 2, 1, 0),
    SOC_DAPM_SINGLE!(c"PCM Playback Switch".as_ptr(), WM8940_MONOMIX, 0, 1, 0),
];

DECLARE_TLV_DB_SCALE!(wm8940_boost_vol_tlv, -1500, 300, 1);
static wm8940_input_boost_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!(c"Mic PGA Switch".as_ptr(), WM8940_PGAGAIN, 6, 1, 1),
    SOC_DAPM_SINGLE_TLV!(c"Aux Volume".as_ptr(), WM8940_ADCBOOST, 0, 7, 0, wm8940_boost_vol_tlv),
    SOC_DAPM_SINGLE_TLV!(c"Mic Volume".as_ptr(), WM8940_ADCBOOST, 4, 7, 0, wm8940_boost_vol_tlv),
];

static wm8940_micpga_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!(c"AUX Switch".as_ptr(), WM8940_INPUTCTL, 2, 1, 0),
    SOC_DAPM_SINGLE!(c"MICP Switch".as_ptr(), WM8940_INPUTCTL, 0, 1, 0),
    SOC_DAPM_SINGLE!(c"MICN Switch".as_ptr(), WM8940_INPUTCTL, 1, 1, 0),
];

static wm8940_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_MIXER!(c"Speaker Mixer".as_ptr(), WM8940_POWER3, 2, 0, wm8940_speaker_mixer_controls.as_ptr(), wm8940_speaker_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Mono Mixer".as_ptr(), WM8940_POWER3, 3, 0, wm8940_mono_mixer_controls.as_ptr(), wm8940_mono_mixer_controls.len()),
    SND_SOC_DAPM_DAC!(c"DAC".as_ptr(), c"HiFi Playback".as_ptr(), WM8940_POWER3, 0, 0),
    SND_SOC_DAPM_PGA!(c"SpkN Out".as_ptr(), WM8940_POWER3, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"SpkP Out".as_ptr(), WM8940_POWER3, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Mono Out".as_ptr(), WM8940_POWER3, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!(c"MONOOUT".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"SPKOUTP".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"SPKOUTN".as_ptr()),
    SND_SOC_DAPM_PGA!(c"Aux Input".as_ptr(), WM8940_POWER1, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_ADC!(c"ADC".as_ptr(), c"HiFi Capture".as_ptr(), WM8940_POWER2, 0, 0),
    SND_SOC_DAPM_MIXER!(c"Mic PGA".as_ptr(), WM8940_POWER2, 2, 0, wm8940_micpga_controls.as_ptr(), wm8940_micpga_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Boost Mixer".as_ptr(), WM8940_POWER2, 4, 0, wm8940_input_boost_controls.as_ptr(), wm8940_input_boost_controls.len()),
    SND_SOC_DAPM_MICBIAS!(c"Mic Bias".as_ptr(), WM8940_POWER1, 4, 0),
    SND_SOC_DAPM_INPUT!(c"MICN".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"MICP".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AUX".as_ptr()),
];

static wm8940_dapm_routes: &[snd_soc_dapm_route] = &[
    /* Mono output mixer */
    snd_soc_dapm_route { sink: c"Mono Mixer".as_ptr(), control: c"PCM Playback Switch".as_ptr(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono Mixer".as_ptr(), control: c"Aux Playback Switch".as_ptr(), source: c"Aux Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono Mixer".as_ptr(), control: c"Line Bypass Switch".as_ptr(), source: c"Boost Mixer".as_ptr() },
    /* Speaker output mixer */
    snd_soc_dapm_route { sink: c"Speaker Mixer".as_ptr(), control: c"PCM Playback Switch".as_ptr(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker Mixer".as_ptr(), control: c"Aux Playback Switch".as_ptr(), source: c"Aux Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker Mixer".as_ptr(), control: c"Line Bypass Switch".as_ptr(), source: c"Boost Mixer".as_ptr() },
    /* Outputs */
    snd_soc_dapm_route { sink: c"Mono Out".as_ptr(), control: ptr::null(), source: c"Mono Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"MONOOUT".as_ptr(), control: ptr::null(), source: c"Mono Out".as_ptr() },
    snd_soc_dapm_route { sink: c"SpkN Out".as_ptr(), control: ptr::null(), source: c"Speaker Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"SpkP Out".as_ptr(), control: ptr::null(), source: c"Speaker Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKOUTN".as_ptr(), control: ptr::null(), source: c"SpkN Out".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKOUTP".as_ptr(), control: ptr::null(), source: c"SpkP Out".as_ptr() },
    /*  Microphone PGA */
    snd_soc_dapm_route { sink: c"Mic PGA".as_ptr(), control: c"MICN Switch".as_ptr(), source: c"MICN".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic PGA".as_ptr(), control: c"MICP Switch".as_ptr(), source: c"MICP".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic PGA".as_ptr(), control: c"AUX Switch".as_ptr(), source: c"AUX".as_ptr() },
    /* Boost Mixer */
    snd_soc_dapm_route { sink: c"Boost Mixer".as_ptr(), control: c"Mic PGA Switch".as_ptr(), source: c"Mic PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"Boost Mixer".as_ptr(), control: c"Mic Volume".as_ptr(), source: c"MICP".as_ptr() },
    snd_soc_dapm_route { sink: c"Boost Mixer".as_ptr(), control: c"Aux Volume".as_ptr(), source: c"Aux Input".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: ptr::null(), source: c"Boost Mixer".as_ptr() },
];

unsafe fn wm8940_reset(c: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(c, WM8940_SOFTRESET, 0)
}

unsafe extern "C" fn wm8940_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = snd_soc_component_read(component, WM8940_IFACE) & 0xFE67;
    let mut clk: u16 = snd_soc_component_read(component, WM8940_CLOCK) & 0x01fe;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            clk |= 1;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }
    snd_soc_component_write(component, WM8940_CLOCK, clk);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            iface |= 2 << 3;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            iface |= 1 << 3;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_DSP_A => {
            iface |= 3 << 3;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            iface |= (3 << 3) | (1 << 7);
        }
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_NB_IF => {
            iface |= 1 << 7;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            iface |= 1 << 8;
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            iface |= (1 << 8) | (1 << 7);
        }
        _ => {}
    }

    snd_soc_component_write(component, WM8940_IFACE, iface);

    0
}

unsafe extern "C" fn wm8940_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8940_priv;
    let mut iface: u16 = snd_soc_component_read(component, WM8940_IFACE) & 0xFD9F;
    let mut addcntrl: u16 = snd_soc_component_read(component, WM8940_ADDCNTRL) & 0xFFF1;
    let mut companding: u16 = snd_soc_component_read(component, WM8940_COMPANDINGCTL) & 0xFFDF;
    let mut ret: c_int;

    (*priv_).fs = params_rate(params);
    ret = wm8940_update_clocks(dai);
    if ret != 0 {
        return ret;
    }

    /* LoutR control */
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE && params_channels(params) == 2 {
        iface |= 1 << 9;
    }

    match params_rate(params) {
        8000 => addcntrl |= 0x5 << 1,
        11025 => addcntrl |= 0x4 << 1,
        16000 => addcntrl |= 0x3 << 1,
        22050 => addcntrl |= 0x2 << 1,
        32000 => addcntrl |= 0x1 << 1,
        44100 | 48000 => {}
        _ => {}
    }
    ret = snd_soc_component_write(component, WM8940_ADDCNTRL, addcntrl);
    if ret != 0 {
        return ret;
    }

    match params_width(params) {
        8 => companding = companding | (1 << 5),
        16 => {}
        20 => iface |= 1 << 5,
        24 => iface |= 2 << 5,
        32 => iface |= 3 << 5,
        _ => {}
    }
    ret = snd_soc_component_write(component, WM8940_COMPANDINGCTL, companding);
    if ret != 0 {
        return ret;
    }
    ret = snd_soc_component_write(component, WM8940_IFACE, iface);

    ret
}

unsafe extern "C" fn wm8940_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let mut mute_reg: u16 = snd_soc_component_read(component, WM8940_DAC) & 0xffbf;

    if mute != 0 {
        mute_reg |= 0x40;
    }

    snd_soc_component_write(component, WM8940_DAC, mute_reg)
}

unsafe extern "C" fn wm8940_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8940 = snd_soc_component_get_drvdata(component) as *mut wm8940_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut val: u16;
    let mut pwr_reg: u16 = snd_soc_component_read(component, WM8940_POWER1) & 0x01F0;
    let mut ret: c_int = 0;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            /* ensure bufioen and biasen */
            pwr_reg |= (1 << 2) | (1 << 3);
            /* Enable thermal shutdown */
            val = snd_soc_component_read(component, WM8940_OUTPUTCTL);
            ret = snd_soc_component_write(component, WM8940_OUTPUTCTL, val | 0x2);
            if ret == 0 {
                /* set vmid to 75k */
                ret = snd_soc_component_write(component, WM8940_POWER1, pwr_reg | 0x1);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            /* ensure bufioen and biasen */
            pwr_reg |= (1 << 2) | (1 << 3);
            ret = snd_soc_component_write(component, WM8940_POWER1, pwr_reg | 0x1);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) as c_int
                == snd_soc_bias_level::SND_SOC_BIAS_OFF as c_int
            {
                ret = regcache_sync((*wm8940).regmap);
                if ret < 0 {
                    dev_err(component.as_ref().unwrap().dev, c"Failed to sync cache: %d\n".as_ptr(), ret);
                    return ret;
                }
            }

            /* ensure bufioen and biasen */
            pwr_reg |= (1 << 2) | (1 << 3);
            /* set vmid to 300k for standby */
            ret = snd_soc_component_write(component, WM8940_POWER1, pwr_reg | 0x2);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            ret = snd_soc_component_write(component, WM8940_POWER1, pwr_reg);
        }
    }

    ret
}

#[repr(C)]
pub struct pll_ {
    pub pre_scale: c_uint,
    pub n: c_uint,
    pub k: c_uint,
}

static mut pll_div: pll_ = pll_ {
    pre_scale: 0,
    n: 0,
    k: 0,
};

/* The size in bits of the pll divide multiplied by 10
 * to allow rounding later */
unsafe fn pll_factors(target: c_uint, mut source: c_uint) {
    let mut Kpart: c_ulonglong;
    let mut K: c_uint;
    let mut Ndiv: c_uint;
    let Nmod: c_uint;
    /* The left shift ist to avoid accuracy loss when right shifting */
    Ndiv = target / source;

    if Ndiv > 12 {
        source <<= 1;
        /* Multiply by 2 */
        pll_div.pre_scale = 0;
        Ndiv = target / source;
    } else if Ndiv < 3 {
        source >>= 2;
        /* Divide by 4 */
        pll_div.pre_scale = 3;
        Ndiv = target / source;
    } else if Ndiv < 6 {
        source >>= 1;
        /* divide by 2 */
        pll_div.pre_scale = 2;
        Ndiv = target / source;
    } else {
        pll_div.pre_scale = 1;
    }

    if Ndiv < 6 || Ndiv > 12 {
        printk(c"WM8940 N value %d outwith recommended range!d\n".as_ptr(), Ndiv);
    }

    pll_div.n = Ndiv;
    Nmod = target % source;
    Kpart = FIXED_PLL_SIZE.wrapping_mul(Nmod as c_ulonglong);

    Kpart /= source as c_ulonglong;

    K = (Kpart & 0xFFFFFFFF) as c_uint;

    /* Check if we need to round */
    if (K % 10) >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    K /= 10;

    pll_div.k = K;
}

/* Untested at the moment */
unsafe extern "C" fn wm8940_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16;

    /* Turn off PLL */
    reg = snd_soc_component_read(component, WM8940_POWER1);
    snd_soc_component_write(component, WM8940_POWER1, reg & 0x01df);

    if freq_in == 0 || freq_out == 0 {
        /* Clock CODEC directly from MCLK */
        reg = snd_soc_component_read(component, WM8940_CLOCK);
        snd_soc_component_write(component, WM8940_CLOCK, reg & 0x00ff);
        /* Pll power down */
        snd_soc_component_write(component, WM8940_PLLN, 1 << 7);
        return 0;
    }

    /* Pll is followed by a frequency divide by 4 */
    pll_factors(freq_out.wrapping_mul(4), freq_in);
    if pll_div.k != 0 {
        snd_soc_component_write(
            component,
            WM8940_PLLN,
            ((pll_div.pre_scale << 4) | pll_div.n | (1 << 6)) as u16,
        );
    } else {
        /* No factional component */
        snd_soc_component_write(
            component,
            WM8940_PLLN,
            ((pll_div.pre_scale << 4) | pll_div.n) as u16,
        );
    }
    snd_soc_component_write(component, WM8940_PLLK1, (pll_div.k >> 18) as u16);
    snd_soc_component_write(component, WM8940_PLLK2, ((pll_div.k >> 9) & 0x1ff) as u16);
    snd_soc_component_write(component, WM8940_PLLK3, (pll_div.k & 0x1ff) as u16);
    /* Enable the PLL */
    reg = snd_soc_component_read(component, WM8940_POWER1);
    snd_soc_component_write(component, WM8940_POWER1, reg | 0x020);

    /* Run CODEC from PLL instead of MCLK */
    reg = snd_soc_component_read(component, WM8940_CLOCK);
    snd_soc_component_write(component, WM8940_CLOCK, reg | 0x100);

    0
}

unsafe extern "C" fn wm8940_set_dai_clkdiv(
    codec_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16;
    let mut ret: c_int = 0;

    match div_id {
        x if x == WM8940_BCLKDIV => {
            reg = snd_soc_component_read(component, WM8940_CLOCK) & 0xFFE3;
            ret = snd_soc_component_write(component, WM8940_CLOCK, reg | ((div << 2) as u16));
        }
        x if x == WM8940_MCLKDIV => {
            reg = snd_soc_component_read(component, WM8940_CLOCK) & 0xFF1F;
            ret = snd_soc_component_write(component, WM8940_CLOCK, reg | ((div << 5) as u16));
        }
        x if x == WM8940_OPCLKDIV => {
            reg = snd_soc_component_read(component, WM8940_GPIO) & 0xFFCF;
            ret = snd_soc_component_write(component, WM8940_GPIO, reg | ((div << 4) as u16));
        }
        _ => {}
    }
    ret
}

unsafe fn wm8940_get_mclkdiv(f_in: c_uint, f_out: c_uint, mclkdiv: *mut c_int) -> c_uint {
    let mut ratio: c_uint = 2 * f_in / f_out;

    if ratio <= 2 {
        *mclkdiv = WM8940_MCLKDIV_1;
        ratio = 2;
    } else if ratio == 3 {
        *mclkdiv = WM8940_MCLKDIV_1_5;
    } else if ratio == 4 {
        *mclkdiv = WM8940_MCLKDIV_2;
    } else if ratio <= 6 {
        *mclkdiv = WM8940_MCLKDIV_3;
        ratio = 6;
    } else if ratio <= 8 {
        *mclkdiv = WM8940_MCLKDIV_4;
        ratio = 8;
    } else if ratio <= 12 {
        *mclkdiv = WM8940_MCLKDIV_6;
        ratio = 12;
    } else if ratio <= 16 {
        *mclkdiv = WM8940_MCLKDIV_8;
        ratio = 16;
    } else {
        *mclkdiv = WM8940_MCLKDIV_12;
        ratio = 24;
    }

    f_out * ratio / 2
}

unsafe fn wm8940_update_clocks(dai: *mut snd_soc_dai) -> c_int {
    let codec = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(codec) as *mut wm8940_priv;
    let fs256: c_uint;
    let mut fpll: c_uint = 0;
    let mut f: c_uint;
    let mut mclkdiv: c_int = 0;

    if (*priv_).mclk == 0 || (*priv_).fs == 0 {
        return 0;
    }

    fs256 = 256 * (*priv_).fs;

    f = wm8940_get_mclkdiv((*priv_).mclk, fs256, &mut mclkdiv);
    if f != (*priv_).mclk {
        /* The PLL performs best around 90MHz */
        if fs256 % 8000 != 0 {
            f = 22579200;
        } else {
            f = 24576000;
        }

        fpll = wm8940_get_mclkdiv(f, fs256, &mut mclkdiv);
    }

    wm8940_set_dai_pll(dai, 0, 0, (*priv_).mclk, fpll);
    wm8940_set_dai_clkdiv(dai, WM8940_MCLKDIV, mclkdiv);

    0
}

unsafe extern "C" fn wm8940_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let codec = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(codec) as *mut wm8940_priv;

    if dir != SND_SOC_CLOCK_IN {
        return -EINVAL;
    }

    (*priv_).mclk = freq;

    wm8940_update_clocks(dai)
}

const fn WM8940_RATES() -> c_uint {
    unsafe { SNDRV_PCM_RATE_8000_48000 }
}

const fn WM8940_FORMATS() -> c_ulonglong {
    unsafe {
        SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
    }
}

static wm8940_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8940_i2s_hw_params),
    set_sysclk: Some(wm8940_set_dai_sysclk),
    mute_stream: Some(wm8940_mute),
    set_fmt: Some(wm8940_set_dai_fmt),
    set_clkdiv: Some(wm8940_set_dai_clkdiv),
    set_pll: Some(wm8940_set_dai_pll),
    no_capture_mute: 1,
};

static mut wm8940_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"wm8940-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WM8940_RATES(),
        formats: WM8940_FORMATS(),
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WM8940_RATES(),
        formats: WM8940_FORMATS(),
    },
    ops: &wm8940_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8940_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let pdata = (*(*component).dev).platform_data as *mut wm8940_setup_data;
    let mut ret: c_int;
    let mut reg: u16;

    /*
     * Check chip ID for wm8940 - value of 0x00 offset
     * SOFTWARE_RESET on write
     * CHIP_ID on read
     */
    reg = snd_soc_component_read(component, WM8940_SOFTRESET);
    if reg != WM8940_CHIP_ID {
        dev_err((*component).dev, c"Wrong wm8940 chip ID: 0x%x\n".as_ptr(), reg as c_uint);
        return -ENODEV;
    }

    ret = wm8940_reset(component);
    if ret < 0 {
        dev_err((*component).dev, c"Failed to issue reset\n".as_ptr());
        return ret;
    }

    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);

    ret = snd_soc_component_write(component, WM8940_POWER1, 0x180);
    if ret < 0 {
        return ret;
    }

    if !pdata.is_null() {
        reg = snd_soc_component_read(component, WM8940_OUTPUTCTL);
        ret = snd_soc_component_write(component, WM8940_OUTPUTCTL, reg | (*pdata).vroi);
        if ret < 0 {
            return ret;
        }
    }

    ret
}

static soc_component_dev_wm8940: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8940_probe),
    set_bias_level: Some(wm8940_set_bias_level),
    controls: wm8940_snd_controls.as_ptr(),
    num_controls: wm8940_snd_controls.len() as c_uint,
    dapm_widgets: wm8940_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8940_dapm_widgets.len() as c_uint,
    dapm_routes: wm8940_dapm_routes.as_ptr(),
    num_dapm_routes: wm8940_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8940_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,

    max_register: unsafe { WM8940_MONOMIX },
    reg_defaults: wm8940_reg_defaults.as_ptr(),
    num_reg_defaults: wm8940_reg_defaults.len() as c_uint,
    cache_type: unsafe { REGCACHE_MAPLE },

    readable_reg: Some(wm8940_readable_register),
    volatile_reg: Some(wm8940_volatile_register),
};

unsafe extern "C" fn wm8940_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8940: *mut wm8940_priv;
    let ret: c_int;

    wm8940 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm8940_priv>(),
        GFP_KERNEL,
    ) as *mut wm8940_priv;
    if wm8940.is_null() {
        return -ENOMEM;
    }

    (*wm8940).regmap = devm_regmap_init_i2c(i2c, &wm8940_regmap);
    if IS_ERR((*wm8940).regmap as *const c_void) {
        return PTR_ERR((*wm8940).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, wm8940 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8940,
        &raw mut wm8940_dai,
        1,
    );

    ret
}

static wm8940_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'w' as c_char,
            b'm' as c_char,
            b'8' as c_char,
            b'9' as c_char,
            b'4' as c_char,
            b'0' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
MODULE_DEVICE_TABLE!(i2c, wm8940_i2c_id);

static wm8940_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"wlf,wm8940".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, wm8940_of_match);

static mut wm8940_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_inner {
        name: c"wm8940".as_ptr(),
        of_match_table: wm8940_of_match.as_ptr(),
    },
    probe: Some(wm8940_i2c_probe),
    id_table: wm8940_i2c_id.as_ptr(),
};

module_i2c_driver!(wm8940_i2c_driver);

MODULE_DESCRIPTION!(c"ASoC WM8940 driver".as_ptr());
MODULE_AUTHOR!(c"Jonathan Cameron".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
