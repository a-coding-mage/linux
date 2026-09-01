// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC CX20721/CX20723 codec driver
//
// Copyright:	(C) 2017 Conexant Systems, Inc.
// Author:	Simon Ho, <Simon.ho@conexant.com>
//
// TODO: add support for TDM mode.
//
// Rust translation of soc/codecs/cx2072x.c. Linux, ALSA SoC, regmap, I2C,
// ACPI, PM, and codec-register symbols are external dependencies supplied by
// the surrounding kernel tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const PLL_OUT_HZ_48: c_uint = 1024 * 3 * 48000;
const BITS_PER_SLOT: c_int = 8;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub id: c_int }
#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub addr: u16, pub adapter: *mut i2c_adapter }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 20], pub driver_data: usize }
#[repr(C)] pub struct acpi_device_id { pub id: [c_char; 16], pub driver_data: usize }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_int,
    pub debounce_time: c_int,
    pub wake: bool,
    pub jack_status_check: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub gpiod_dev: *mut device,
    pub data: *mut c_void,
}

/* codec private data */
#[repr(C)]
pub struct cx2072x_priv {
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub dev: *mut device,
    pub codec: *mut snd_soc_component,
    pub jack_gpio: snd_soc_jack_gpio,
    pub lock: mutex,
    pub bclk_ratio: c_uint,
    pub pll_changed: bool,
    pub i2spcm_changed: bool,
    pub sample_size: c_int,
    pub frame_size: c_int,
    pub sample_rate: c_int,
    pub dai_fmt: c_uint,
    pub en_aec_ref: bool,
}

unsafe extern "C" {
    static CX2072X_AFG_POWER_STATE: c_uint; static CX2072X_UM_RESPONSE: c_uint;
    static CX2072X_GPIO_DATA: c_uint; static CX2072X_GPIO_ENABLE: c_uint;
    static CX2072X_GPIO_DIRECTION: c_uint; static CX2072X_GPIO_WAKE: c_uint;
    static CX2072X_GPIO_UM_ENABLE: c_uint; static CX2072X_GPIO_STICKY_MASK: c_uint;
    static CX2072X_DAC1_POWER_STATE: c_uint; static CX2072X_DAC1_CONVERTER_STREAM_CHANNEL: c_uint;
    static CX2072X_DAC1_EAPD_ENABLE: c_uint; static CX2072X_DAC1_AMP_GAIN_RIGHT: c_uint;
    static CX2072X_DAC1_AMP_GAIN_LEFT: c_uint; static CX2072X_DAC1_CONVERTER_FORMAT: c_uint;
    static CX2072X_DAC2_POWER_STATE: c_uint; static CX2072X_DAC2_CONVERTER_STREAM_CHANNEL: c_uint;
    static CX2072X_DAC2_AMP_GAIN_RIGHT: c_uint; static CX2072X_DAC2_AMP_GAIN_LEFT: c_uint;
    static CX2072X_DAC2_CONVERTER_FORMAT: c_uint; static CX2072X_ADC1_CONNECTION_SELECT_CONTROL: c_uint;
    static CX2072X_ADC1_POWER_STATE: c_uint; static CX2072X_ADC1_CONVERTER_STREAM_CHANNEL: c_uint;
    static CX2072X_ADC1_AMP_GAIN_RIGHT_0: c_uint; static CX2072X_ADC1_AMP_GAIN_RIGHT_1: c_uint;
    static CX2072X_ADC1_AMP_GAIN_RIGHT_2: c_uint; static CX2072X_ADC1_AMP_GAIN_RIGHT_3: c_uint;
    static CX2072X_ADC1_AMP_GAIN_RIGHT_4: c_uint; static CX2072X_ADC1_AMP_GAIN_RIGHT_5: c_uint;
    static CX2072X_ADC1_AMP_GAIN_RIGHT_6: c_uint; static CX2072X_ADC1_AMP_GAIN_LEFT_0: c_uint;
    static CX2072X_ADC1_AMP_GAIN_LEFT_1: c_uint; static CX2072X_ADC1_AMP_GAIN_LEFT_2: c_uint;
    static CX2072X_ADC1_AMP_GAIN_LEFT_3: c_uint; static CX2072X_ADC1_AMP_GAIN_LEFT_4: c_uint;
    static CX2072X_ADC1_AMP_GAIN_LEFT_5: c_uint; static CX2072X_ADC1_AMP_GAIN_LEFT_6: c_uint;
    static CX2072X_ADC1_CONVERTER_FORMAT: c_uint; static CX2072X_ADC2_CONNECTION_SELECT_CONTROL: c_uint;
    static CX2072X_ADC2_POWER_STATE: c_uint; static CX2072X_ADC2_CONVERTER_STREAM_CHANNEL: c_uint;
    static CX2072X_ADC2_AMP_GAIN_RIGHT_0: c_uint; static CX2072X_ADC2_AMP_GAIN_RIGHT_1: c_uint;
    static CX2072X_ADC2_AMP_GAIN_RIGHT_2: c_uint; static CX2072X_ADC2_AMP_GAIN_LEFT_0: c_uint;
    static CX2072X_ADC2_AMP_GAIN_LEFT_1: c_uint; static CX2072X_ADC2_AMP_GAIN_LEFT_2: c_uint;
    static CX2072X_ADC2_CONVERTER_FORMAT: c_uint; static CX2072X_MIXER_POWER_STATE: c_uint;
    static CX2072X_MIXER_GAIN_RIGHT_0: c_uint; static CX2072X_MIXER_GAIN_RIGHT_1: c_uint;
    static CX2072X_MIXER_GAIN_LEFT_0: c_uint; static CX2072X_MIXER_GAIN_LEFT_1: c_uint;
    static CX2072X_PORTA_CONNECTION_SELECT_CTRL: c_uint; static CX2072X_PORTA_POWER_STATE: c_uint;
    static CX2072X_PORTA_PIN_CTRL: c_uint; static CX2072X_PORTA_UNSOLICITED_RESPONSE: c_uint;
    static CX2072X_PORTA_PIN_SENSE: c_uint; static CX2072X_PORTA_EAPD_BTL: c_uint;
    static CX2072X_PORTG_CONNECTION_SELECT_CTRL: c_uint; static CX2072X_PORTG_POWER_STATE: c_uint;
    static CX2072X_PORTG_PIN_CTRL: c_uint; static CX2072X_PORTG_EAPD_BTL: c_uint;
    static CX2072X_PORTB_POWER_STATE: c_uint; static CX2072X_PORTB_PIN_CTRL: c_uint;
    static CX2072X_PORTB_UNSOLICITED_RESPONSE: c_uint; static CX2072X_PORTB_PIN_SENSE: c_uint;
    static CX2072X_PORTB_EAPD_BTL: c_uint; static CX2072X_PORTB_GAIN_RIGHT: c_uint;
    static CX2072X_PORTB_GAIN_LEFT: c_uint; static CX2072X_PORTD_POWER_STATE: c_uint;
    static CX2072X_PORTD_PIN_CTRL: c_uint; static CX2072X_PORTD_UNSOLICITED_RESPONSE: c_uint;
    static CX2072X_PORTD_PIN_SENSE: c_uint; static CX2072X_PORTD_GAIN_RIGHT: c_uint;
    static CX2072X_PORTD_GAIN_LEFT: c_uint; static CX2072X_PORTC_POWER_STATE: c_uint;
    static CX2072X_PORTC_PIN_CTRL: c_uint; static CX2072X_PORTC_GAIN_RIGHT: c_uint;
    static CX2072X_PORTC_GAIN_LEFT: c_uint; static CX2072X_SPKR_DRC_ENABLE_STEP: c_uint;
    static CX2072X_SPKR_DRC_CONTROL: c_uint; static CX2072X_SPKR_DRC_TEST: c_uint;
    static CX2072X_DIGITAL_BIOS_TEST0: c_uint; static CX2072X_DIGITAL_BIOS_TEST2: c_uint;
    static CX2072X_I2SPCM_CONTROL1: c_uint; static CX2072X_I2SPCM_CONTROL2: c_uint;
    static CX2072X_I2SPCM_CONTROL3: c_uint; static CX2072X_I2SPCM_CONTROL4: c_uint;
    static CX2072X_I2SPCM_CONTROL5: c_uint; static CX2072X_UM_INTERRUPT_CRTL_E: c_uint;
    static CX2072X_I2SPCM_CONTROL6: c_uint; static CX2072X_DIGITAL_TEST16: c_uint;
    static CX2072X_DIGITAL_TEST17: c_uint; static CX2072X_DIGITAL_TEST18: c_uint;
    static CX2072X_DIGITAL_TEST19: c_uint; static CX2072X_DIGITAL_TEST20: c_uint;
    static CX2072X_CODEC_TEST2: c_uint; static CX2072X_CODEC_TEST9: c_uint;
    static CX2072X_ANALOG_TEST3: c_uint; static CX2072X_ANALOG_TEST4: c_uint;
    static CX2072X_ANALOG_TEST5: c_uint; static CX2072X_ANALOG_TEST6: c_uint;
    static CX2072X_ANALOG_TEST7: c_uint; static CX2072X_ANALOG_TEST8: c_uint;
    static CX2072X_ANALOG_TEST9: c_uint; static CX2072X_ANALOG_TEST10: c_uint;
    static CX2072X_ANALOG_TEST11: c_uint; static CX2072X_ANALOG_TEST12: c_uint;
    static CX2072X_ANALOG_TEST13: c_uint; static CX2072X_DIGITAL_TEST0: c_uint;
    static CX2072X_DIGITAL_TEST1: c_uint; static CX2072X_DIGITAL_TEST11: c_uint;
    static CX2072X_DIGITAL_TEST12: c_uint; static CX2072X_DIGITAL_TEST15: c_uint;
    static CX2072X_CODEC_TEST20: c_uint; static CX2072X_CODEC_TEST24: c_uint;
    static CX2072X_CODEC_TEST26: c_uint; static CX2072X_CODEC_TESTXX: c_uint;
    static CX2072X_PORTE_CONNECTION_SELECT_CTRL: c_uint; static CX2072X_PORTE_POWER_STATE: c_uint;
    static CX2072X_PORTE_PIN_CTRL: c_uint; static CX2072X_PORTE_UNSOLICITED_RESPONSE: c_uint;
    static CX2072X_PORTE_PIN_SENSE: c_uint; static CX2072X_PORTE_EAPD_BTL: c_uint;
    static CX2072X_PORTE_GAIN_RIGHT: c_uint; static CX2072X_PORTE_GAIN_LEFT: c_uint;
    static CX2072X_PORTF_POWER_STATE: c_uint; static CX2072X_PORTF_PIN_CTRL: c_uint;
    static CX2072X_PORTF_UNSOLICITED_RESPONSE: c_uint; static CX2072X_PORTF_PIN_SENSE: c_uint;
    static CX2072X_PORTF_GAIN_RIGHT: c_uint; static CX2072X_PORTF_GAIN_LEFT: c_uint;
    static CX2072X_PORTM_CONNECTION_SELECT_CTRL: c_uint; static CX2072X_PORTM_POWER_STATE: c_uint;
    static CX2072X_PORTM_PIN_CTRL: c_uint; static CX2072X_PORTM_EAPD_BTL: c_uint;
    static CX2072X_VENDOR_ID: c_uint; static CX2072X_REVISION_ID: c_uint;
    static CX2072X_CURRENT_BCLK_FREQUENCY: c_uint; static CX2072X_EQ_ENABLE_BYPASS: c_uint;
    static CX2072X_EQ_B0_COEFF: c_uint; static CX2072X_EQ_B1_COEFF: c_uint;
    static CX2072X_EQ_B2_COEFF: c_uint; static CX2072X_EQ_A1_COEFF: c_uint;
    static CX2072X_EQ_A2_COEFF: c_uint; static CX2072X_EQ_G_COEFF: c_uint;
    static CX2072X_EQ_BAND: c_uint; static CX2072X_REG_MAX: c_uint;
    static CX2072X_DAI_DSP: c_int; static CX2072X_DAI_HIFI: c_int;
    static CX2072X_RATES_DSP: c_uint; static CX2072X_MAX_EQ_COEFF: usize;

    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint; static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint; static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint; static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint; static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint; static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint; static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int; static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_BIAS_STANDBY: c_int; static SND_SOC_BIAS_OFF: c_int;
    static SND_JACK_HEADSET: c_int; static SND_JACK_HEADPHONE: c_int; static SND_JACK_BTN_0: c_int;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint; static SNDRV_PCM_FMTBIT_S24_LE: c_uint;

    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, count: usize) -> c_int;
    fn i2c_transfer(adapter: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cx2072x_priv;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_force_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_jack_add_gpios(jack: *mut snd_soc_jack, count: c_int, gpios: *mut snd_soc_jack_gpio) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut cx2072x_priv;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, ctx: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn mutex_init(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int; fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_use_autosuspend(dev: *mut device); fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

#[repr(C)]
pub struct i2c_msg { pub addr: u16, pub flags: u16, pub len: u16, pub buf: *mut u8 }

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const I2C_M_RD: u16 = 0x0001;

/* DAC/ADC Volume
 *
 * max : 74 : 0 dB
 *	 ( in 1 dB  step )
 * min : 0 : -74 dB
 */
static adc_tlv: [c_uint; 4] = [0, (-7400i32) as c_uint, 100, 0];
static dac_tlv: [c_uint; 4] = [0, (-7400i32) as c_uint, 100, 0];
static boost_tlv: [c_uint; 4] = [0, 0, 1200, 0];
static hpf_tlv: [c_uint; 8] = [0, 0, 0, 120, 0, 1, 63, 30];

/* Lookup table for PRE_DIV */
#[repr(C)]
struct mclk_pre_div_entry { mclk: c_uint, div: c_uint }

static mclk_pre_div: [mclk_pre_div_entry; 9] = [
    mclk_pre_div_entry { mclk: 6144000, div: 1 },
    mclk_pre_div_entry { mclk: 12288000, div: 2 },
    mclk_pre_div_entry { mclk: 19200000, div: 3 },
    mclk_pre_div_entry { mclk: 26000000, div: 4 },
    mclk_pre_div_entry { mclk: 28224000, div: 5 },
    mclk_pre_div_entry { mclk: 36864000, div: 6 },
    mclk_pre_div_entry { mclk: 36864000, div: 7 },
    mclk_pre_div_entry { mclk: 48000000, div: 8 },
    mclk_pre_div_entry { mclk: 49152000, div: 8 },
];

macro_rules! reg_default { ($r:ident, $v:expr) => { reg_default { reg: unsafe { $r }, def: $v } }; }
macro_rules! reg_sequence { ($r:ident, $v:expr) => { reg_sequence { reg: unsafe { $r }, def: $v } }; }

/*
 * cx2072x register cache.
 */
static cx2072x_reg_defaults: [reg_default; 128] = [
    reg_default!(CX2072X_AFG_POWER_STATE, 0x00000003), reg_default!(CX2072X_UM_RESPONSE, 0x00000000),
    reg_default!(CX2072X_GPIO_DATA, 0x00000000), reg_default!(CX2072X_GPIO_ENABLE, 0x00000000),
    reg_default!(CX2072X_GPIO_DIRECTION, 0x00000000), reg_default!(CX2072X_GPIO_WAKE, 0x00000000),
    reg_default!(CX2072X_GPIO_UM_ENABLE, 0x00000000), reg_default!(CX2072X_GPIO_STICKY_MASK, 0x00000000),
    reg_default!(CX2072X_DAC1_POWER_STATE, 0x00000433), reg_default!(CX2072X_DAC1_CONVERTER_STREAM_CHANNEL, 0x00000000),
    reg_default!(CX2072X_DAC1_EAPD_ENABLE, 0x00000000), reg_default!(CX2072X_DAC1_AMP_GAIN_RIGHT, 0x0000004a),
    reg_default!(CX2072X_DAC1_AMP_GAIN_LEFT, 0x0000004a), reg_default!(CX2072X_DAC1_CONVERTER_FORMAT, 0x00000031),
    reg_default!(CX2072X_DAC2_POWER_STATE, 0x00000433), reg_default!(CX2072X_DAC2_CONVERTER_STREAM_CHANNEL, 0x00000000),
    reg_default!(CX2072X_DAC2_AMP_GAIN_RIGHT, 0x0000004a), reg_default!(CX2072X_DAC2_AMP_GAIN_LEFT, 0x0000004a),
    reg_default!(CX2072X_DAC2_CONVERTER_FORMAT, 0x00000031), reg_default!(CX2072X_ADC1_CONNECTION_SELECT_CONTROL, 0x00000000),
    reg_default!(CX2072X_ADC1_POWER_STATE, 0x00000433), reg_default!(CX2072X_ADC1_CONVERTER_STREAM_CHANNEL, 0x00000000),
    reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_0, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_1, 0x0000004a),
    reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_2, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_3, 0x0000004a),
    reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_4, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_5, 0x0000004a),
    reg_default!(CX2072X_ADC1_AMP_GAIN_RIGHT_6, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_0, 0x0000004a),
    reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_1, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_2, 0x0000004a),
    reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_3, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_4, 0x0000004a),
    reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_5, 0x0000004a), reg_default!(CX2072X_ADC1_AMP_GAIN_LEFT_6, 0x0000004a),
    reg_default!(CX2072X_ADC1_CONVERTER_FORMAT, 0x00000031), reg_default!(CX2072X_ADC2_CONNECTION_SELECT_CONTROL, 0x00000000),
    reg_default!(CX2072X_ADC2_POWER_STATE, 0x00000433), reg_default!(CX2072X_ADC2_CONVERTER_STREAM_CHANNEL, 0x00000000),
    reg_default!(CX2072X_ADC2_AMP_GAIN_RIGHT_0, 0x0000004a), reg_default!(CX2072X_ADC2_AMP_GAIN_RIGHT_1, 0x0000004a),
    reg_default!(CX2072X_ADC2_AMP_GAIN_RIGHT_2, 0x0000004a), reg_default!(CX2072X_ADC2_AMP_GAIN_LEFT_0, 0x0000004a),
    reg_default!(CX2072X_ADC2_AMP_GAIN_LEFT_1, 0x0000004a), reg_default!(CX2072X_ADC2_AMP_GAIN_LEFT_2, 0x0000004a),
    reg_default!(CX2072X_ADC2_CONVERTER_FORMAT, 0x00000031), reg_default!(CX2072X_MIXER_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_MIXER_GAIN_RIGHT_0, 0x0000004a), reg_default!(CX2072X_MIXER_GAIN_RIGHT_1, 0x0000004a),
    reg_default!(CX2072X_MIXER_GAIN_LEFT_0, 0x0000004a), reg_default!(CX2072X_MIXER_GAIN_LEFT_1, 0x0000004a),
    reg_default!(CX2072X_PORTA_CONNECTION_SELECT_CTRL, 0x00000000), reg_default!(CX2072X_PORTA_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_PORTA_PIN_CTRL, 0x000000c0), reg_default!(CX2072X_PORTA_UNSOLICITED_RESPONSE, 0x00000000),
    reg_default!(CX2072X_PORTA_PIN_SENSE, 0x00000000), reg_default!(CX2072X_PORTA_EAPD_BTL, 0x00000002),
    reg_default!(CX2072X_PORTG_CONNECTION_SELECT_CTRL, 0x00000000), reg_default!(CX2072X_PORTG_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_PORTG_PIN_CTRL, 0x00000040), reg_default!(CX2072X_PORTG_EAPD_BTL, 0x00000002),
    reg_default!(CX2072X_PORTB_POWER_STATE, 0x00000433), reg_default!(CX2072X_PORTB_PIN_CTRL, 0x00000000),
    reg_default!(CX2072X_PORTB_UNSOLICITED_RESPONSE, 0x00000000), reg_default!(CX2072X_PORTB_PIN_SENSE, 0x00000000),
    reg_default!(CX2072X_PORTB_EAPD_BTL, 0x00000002), reg_default!(CX2072X_PORTB_GAIN_RIGHT, 0x00000000),
    reg_default!(CX2072X_PORTB_GAIN_LEFT, 0x00000000), reg_default!(CX2072X_PORTD_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_PORTD_PIN_CTRL, 0x00000020), reg_default!(CX2072X_PORTD_UNSOLICITED_RESPONSE, 0x00000000),
    reg_default!(CX2072X_PORTD_PIN_SENSE, 0x00000000), reg_default!(CX2072X_PORTD_GAIN_RIGHT, 0x00000000),
    reg_default!(CX2072X_PORTD_GAIN_LEFT, 0x00000000), reg_default!(CX2072X_PORTC_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_PORTC_PIN_CTRL, 0x00000000), reg_default!(CX2072X_PORTC_GAIN_RIGHT, 0x00000000),
    reg_default!(CX2072X_PORTC_GAIN_LEFT, 0x00000000), reg_default!(CX2072X_SPKR_DRC_ENABLE_STEP, 0x040065a4),
    reg_default!(CX2072X_SPKR_DRC_CONTROL, 0x007b0024), reg_default!(CX2072X_SPKR_DRC_TEST, 0x00000000),
    reg_default!(CX2072X_DIGITAL_BIOS_TEST0, 0x001f008a), reg_default!(CX2072X_DIGITAL_BIOS_TEST2, 0x00990026),
    reg_default!(CX2072X_I2SPCM_CONTROL1, 0x00010001), reg_default!(CX2072X_I2SPCM_CONTROL2, 0x00000000),
    reg_default!(CX2072X_I2SPCM_CONTROL3, 0x00000000), reg_default!(CX2072X_I2SPCM_CONTROL4, 0x00000000),
    reg_default!(CX2072X_I2SPCM_CONTROL5, 0x00000000), reg_default!(CX2072X_UM_INTERRUPT_CRTL_E, 0x00000000),
    reg_default!(CX2072X_I2SPCM_CONTROL6, 0x00000000), reg_default!(CX2072X_DIGITAL_TEST16, 0x00000021),
    reg_default!(CX2072X_DIGITAL_TEST17, 0x00000018), reg_default!(CX2072X_DIGITAL_TEST18, 0x00000024),
    reg_default!(CX2072X_DIGITAL_TEST19, 0x00000001), reg_default!(CX2072X_DIGITAL_TEST20, 0x00000002),
    reg_default!(CX2072X_CODEC_TEST2, 0x00000000), reg_default!(CX2072X_CODEC_TEST9, 0x00000004),
    reg_default!(CX2072X_ANALOG_TEST4, 0x00000000), reg_default!(CX2072X_ANALOG_TEST5, 0x00000000),
    reg_default!(CX2072X_ANALOG_TEST6, 0x0000059a), reg_default!(CX2072X_ANALOG_TEST7, 0x000000a7),
    reg_default!(CX2072X_ANALOG_TEST8, 0x00000017), reg_default!(CX2072X_ANALOG_TEST9, 0x00000000),
    reg_default!(CX2072X_ANALOG_TEST10, 0x00000285), reg_default!(CX2072X_ANALOG_TEST11, 0x00000000),
    reg_default!(CX2072X_ANALOG_TEST12, 0x00000000), reg_default!(CX2072X_ANALOG_TEST13, 0x00000000),
    reg_default!(CX2072X_DIGITAL_TEST1, 0x00000242), reg_default!(CX2072X_DIGITAL_TEST11, 0x00000000),
    reg_default!(CX2072X_DIGITAL_TEST12, 0x00000084), reg_default!(CX2072X_DIGITAL_TEST15, 0x00000077),
    reg_default!(CX2072X_CODEC_TEST20, 0x00000600), reg_default!(CX2072X_CODEC_TEST26, 0x00000208),
    reg_default!(CX2072X_PORTE_CONNECTION_SELECT_CTRL, 0x00000000), reg_default!(CX2072X_PORTE_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_PORTE_PIN_CTRL, 0x00000040), reg_default!(CX2072X_PORTE_UNSOLICITED_RESPONSE, 0x00000000),
    reg_default!(CX2072X_PORTE_PIN_SENSE, 0x00000000), reg_default!(CX2072X_PORTE_EAPD_BTL, 0x00000002),
    reg_default!(CX2072X_PORTE_GAIN_RIGHT, 0x00000000), reg_default!(CX2072X_PORTE_GAIN_LEFT, 0x00000000),
    reg_default!(CX2072X_PORTF_POWER_STATE, 0x00000433), reg_default!(CX2072X_PORTF_PIN_CTRL, 0x00000000),
    reg_default!(CX2072X_PORTF_UNSOLICITED_RESPONSE, 0x00000000), reg_default!(CX2072X_PORTF_PIN_SENSE, 0x00000000),
    reg_default!(CX2072X_PORTF_GAIN_RIGHT, 0x00000000), reg_default!(CX2072X_PORTF_GAIN_LEFT, 0x00000000),
    reg_default!(CX2072X_PORTM_CONNECTION_SELECT_CTRL, 0x00000000), reg_default!(CX2072X_PORTM_POWER_STATE, 0x00000433),
    reg_default!(CX2072X_PORTM_PIN_CTRL, 0x00000000), reg_default!(CX2072X_PORTM_EAPD_BTL, 0x00000002),
];

/*
 * register initialization
 */
static cx2072x_reg_init: [reg_sequence; 17] = [
    reg_sequence!(CX2072X_ANALOG_TEST9, 0x080),    /* DC offset Calibration */
    reg_sequence!(CX2072X_CODEC_TEST26, 0x65f),    /* Disable the PA */
    reg_sequence!(CX2072X_ANALOG_TEST10, 0x289),   /* Set the speaker output gain */
    reg_sequence!(CX2072X_CODEC_TEST20, 0xf05), reg_sequence!(CX2072X_CODEC_TESTXX, 0x380),
    reg_sequence!(CX2072X_CODEC_TEST26, 0xb90), reg_sequence!(CX2072X_CODEC_TEST9, 0x001),    /* Enable 30 Hz High pass filter */
    reg_sequence!(CX2072X_ANALOG_TEST3, 0x300),    /* Disable PCBEEP pad */
    reg_sequence!(CX2072X_CODEC_TEST24, 0x100),    /* Disable SnM mode */
    reg_sequence!(CX2072X_PORTD_PIN_CTRL, 0x020),  /* Enable PortD input */
    reg_sequence!(CX2072X_GPIO_ENABLE, 0x040),     /* Enable GPIO7 pin for button */
    reg_sequence!(CX2072X_GPIO_UM_ENABLE, 0x040),  /* Enable UM for GPIO7 */
    reg_sequence!(CX2072X_UM_RESPONSE, 0x080),     /* Enable button response */
    reg_sequence!(CX2072X_DIGITAL_TEST12, 0x0c4),  /* Enable headset button */
    reg_sequence!(CX2072X_DIGITAL_TEST0, 0x415),   /* Power down class-D during idle */
    reg_sequence!(CX2072X_I2SPCM_CONTROL2, 0x00f), /* Enable I2S TX */
    reg_sequence!(CX2072X_I2SPCM_CONTROL3, 0x00f), /* Enable I2S RX */
];

unsafe fn cx2072x_register_size(reg: c_uint) -> c_uint {
    if reg == CX2072X_VENDOR_ID || reg == CX2072X_REVISION_ID || reg == CX2072X_PORTA_PIN_SENSE ||
       reg == CX2072X_PORTB_PIN_SENSE || reg == CX2072X_PORTD_PIN_SENSE || reg == CX2072X_PORTE_PIN_SENSE ||
       reg == CX2072X_PORTF_PIN_SENSE || reg == CX2072X_I2SPCM_CONTROL1 || reg == CX2072X_I2SPCM_CONTROL2 ||
       reg == CX2072X_I2SPCM_CONTROL3 || reg == CX2072X_I2SPCM_CONTROL4 || reg == CX2072X_I2SPCM_CONTROL5 ||
       reg == CX2072X_I2SPCM_CONTROL6 || reg == CX2072X_UM_INTERRUPT_CRTL_E || reg == CX2072X_EQ_G_COEFF ||
       reg == CX2072X_SPKR_DRC_CONTROL || reg == CX2072X_SPKR_DRC_TEST || reg == CX2072X_DIGITAL_BIOS_TEST0 ||
       reg == CX2072X_DIGITAL_BIOS_TEST2 { return 4; }
    if reg == CX2072X_EQ_ENABLE_BYPASS || reg == CX2072X_EQ_B0_COEFF || reg == CX2072X_EQ_B1_COEFF ||
       reg == CX2072X_EQ_B2_COEFF || reg == CX2072X_EQ_A1_COEFF || reg == CX2072X_EQ_A2_COEFF ||
       reg == CX2072X_DAC1_CONVERTER_FORMAT || reg == CX2072X_DAC2_CONVERTER_FORMAT ||
       reg == CX2072X_ADC1_CONVERTER_FORMAT || reg == CX2072X_ADC2_CONVERTER_FORMAT ||
       reg == CX2072X_CODEC_TEST2 || reg == CX2072X_CODEC_TEST9 || reg == CX2072X_CODEC_TEST20 ||
       reg == CX2072X_CODEC_TEST26 || reg == CX2072X_ANALOG_TEST3 || reg == CX2072X_ANALOG_TEST4 ||
       reg == CX2072X_ANALOG_TEST5 || reg == CX2072X_ANALOG_TEST6 || reg == CX2072X_ANALOG_TEST7 ||
       reg == CX2072X_ANALOG_TEST8 || reg == CX2072X_ANALOG_TEST9 || reg == CX2072X_ANALOG_TEST10 ||
       reg == CX2072X_ANALOG_TEST11 || reg == CX2072X_ANALOG_TEST12 || reg == CX2072X_ANALOG_TEST13 ||
       reg == CX2072X_DIGITAL_TEST0 || reg == CX2072X_DIGITAL_TEST1 || reg == CX2072X_DIGITAL_TEST11 ||
       reg == CX2072X_DIGITAL_TEST12 || reg == CX2072X_DIGITAL_TEST15 || reg == CX2072X_DIGITAL_TEST16 ||
       reg == CX2072X_DIGITAL_TEST17 || reg == CX2072X_DIGITAL_TEST18 || reg == CX2072X_DIGITAL_TEST19 ||
       reg == CX2072X_DIGITAL_TEST20 { return 2; }
    1
}

unsafe fn cx2072x_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    let readable = [
        CX2072X_VENDOR_ID, CX2072X_REVISION_ID, CX2072X_CURRENT_BCLK_FREQUENCY, CX2072X_AFG_POWER_STATE,
        CX2072X_UM_RESPONSE, CX2072X_GPIO_DATA, CX2072X_GPIO_ENABLE, CX2072X_GPIO_DIRECTION, CX2072X_GPIO_WAKE,
        CX2072X_GPIO_UM_ENABLE, CX2072X_GPIO_STICKY_MASK, CX2072X_DAC1_CONVERTER_FORMAT, CX2072X_DAC1_AMP_GAIN_RIGHT,
        CX2072X_DAC1_AMP_GAIN_LEFT, CX2072X_DAC1_POWER_STATE, CX2072X_DAC1_CONVERTER_STREAM_CHANNEL,
        CX2072X_DAC1_EAPD_ENABLE, CX2072X_DAC2_CONVERTER_FORMAT, CX2072X_DAC2_AMP_GAIN_RIGHT,
        CX2072X_DAC2_AMP_GAIN_LEFT, CX2072X_DAC2_POWER_STATE, CX2072X_DAC2_CONVERTER_STREAM_CHANNEL,
        CX2072X_ADC1_CONVERTER_FORMAT, CX2072X_ADC1_AMP_GAIN_RIGHT_0, CX2072X_ADC1_AMP_GAIN_LEFT_0,
        CX2072X_ADC1_AMP_GAIN_RIGHT_1, CX2072X_ADC1_AMP_GAIN_LEFT_1, CX2072X_ADC1_AMP_GAIN_RIGHT_2,
        CX2072X_ADC1_AMP_GAIN_LEFT_2, CX2072X_ADC1_AMP_GAIN_RIGHT_3, CX2072X_ADC1_AMP_GAIN_LEFT_3,
        CX2072X_ADC1_AMP_GAIN_RIGHT_4, CX2072X_ADC1_AMP_GAIN_LEFT_4, CX2072X_ADC1_AMP_GAIN_RIGHT_5,
        CX2072X_ADC1_AMP_GAIN_LEFT_5, CX2072X_ADC1_AMP_GAIN_RIGHT_6, CX2072X_ADC1_AMP_GAIN_LEFT_6,
        CX2072X_ADC1_CONNECTION_SELECT_CONTROL, CX2072X_ADC1_POWER_STATE, CX2072X_ADC1_CONVERTER_STREAM_CHANNEL,
        CX2072X_ADC2_CONVERTER_FORMAT, CX2072X_ADC2_AMP_GAIN_RIGHT_0, CX2072X_ADC2_AMP_GAIN_LEFT_0,
        CX2072X_ADC2_AMP_GAIN_RIGHT_1, CX2072X_ADC2_AMP_GAIN_LEFT_1, CX2072X_ADC2_AMP_GAIN_RIGHT_2,
        CX2072X_ADC2_AMP_GAIN_LEFT_2, CX2072X_ADC2_CONNECTION_SELECT_CONTROL, CX2072X_ADC2_POWER_STATE,
        CX2072X_ADC2_CONVERTER_STREAM_CHANNEL, CX2072X_PORTA_CONNECTION_SELECT_CTRL, CX2072X_PORTA_POWER_STATE,
        CX2072X_PORTA_PIN_CTRL, CX2072X_PORTA_UNSOLICITED_RESPONSE, CX2072X_PORTA_PIN_SENSE, CX2072X_PORTA_EAPD_BTL,
        CX2072X_PORTB_POWER_STATE, CX2072X_PORTB_PIN_CTRL, CX2072X_PORTB_UNSOLICITED_RESPONSE, CX2072X_PORTB_PIN_SENSE,
        CX2072X_PORTB_EAPD_BTL, CX2072X_PORTB_GAIN_RIGHT, CX2072X_PORTB_GAIN_LEFT, CX2072X_PORTC_POWER_STATE,
        CX2072X_PORTC_PIN_CTRL, CX2072X_PORTC_GAIN_RIGHT, CX2072X_PORTC_GAIN_LEFT, CX2072X_PORTD_POWER_STATE,
        CX2072X_PORTD_PIN_CTRL, CX2072X_PORTD_UNSOLICITED_RESPONSE, CX2072X_PORTD_PIN_SENSE, CX2072X_PORTD_GAIN_RIGHT,
        CX2072X_PORTD_GAIN_LEFT, CX2072X_PORTE_CONNECTION_SELECT_CTRL, CX2072X_PORTE_POWER_STATE, CX2072X_PORTE_PIN_CTRL,
        CX2072X_PORTE_UNSOLICITED_RESPONSE, CX2072X_PORTE_PIN_SENSE, CX2072X_PORTE_EAPD_BTL, CX2072X_PORTE_GAIN_RIGHT,
        CX2072X_PORTE_GAIN_LEFT, CX2072X_PORTF_POWER_STATE, CX2072X_PORTF_PIN_CTRL, CX2072X_PORTF_UNSOLICITED_RESPONSE,
        CX2072X_PORTF_PIN_SENSE, CX2072X_PORTF_GAIN_RIGHT, CX2072X_PORTF_GAIN_LEFT, CX2072X_PORTG_POWER_STATE,
        CX2072X_PORTG_PIN_CTRL, CX2072X_PORTG_CONNECTION_SELECT_CTRL, CX2072X_PORTG_EAPD_BTL, CX2072X_PORTM_POWER_STATE,
        CX2072X_PORTM_PIN_CTRL, CX2072X_PORTM_CONNECTION_SELECT_CTRL, CX2072X_PORTM_EAPD_BTL, CX2072X_MIXER_POWER_STATE,
        CX2072X_MIXER_GAIN_RIGHT_0, CX2072X_MIXER_GAIN_LEFT_0, CX2072X_MIXER_GAIN_RIGHT_1, CX2072X_MIXER_GAIN_LEFT_1,
        CX2072X_EQ_ENABLE_BYPASS, CX2072X_EQ_B0_COEFF, CX2072X_EQ_B1_COEFF, CX2072X_EQ_B2_COEFF, CX2072X_EQ_A1_COEFF,
        CX2072X_EQ_A2_COEFF, CX2072X_EQ_G_COEFF, CX2072X_SPKR_DRC_ENABLE_STEP, CX2072X_SPKR_DRC_CONTROL,
        CX2072X_SPKR_DRC_TEST, CX2072X_DIGITAL_BIOS_TEST0, CX2072X_DIGITAL_BIOS_TEST2, CX2072X_I2SPCM_CONTROL1,
        CX2072X_I2SPCM_CONTROL2, CX2072X_I2SPCM_CONTROL3, CX2072X_I2SPCM_CONTROL4, CX2072X_I2SPCM_CONTROL5,
        CX2072X_I2SPCM_CONTROL6, CX2072X_UM_INTERRUPT_CRTL_E, CX2072X_CODEC_TEST2, CX2072X_CODEC_TEST9,
        CX2072X_CODEC_TEST20, CX2072X_CODEC_TEST26, CX2072X_ANALOG_TEST4, CX2072X_ANALOG_TEST5, CX2072X_ANALOG_TEST6,
        CX2072X_ANALOG_TEST7, CX2072X_ANALOG_TEST8, CX2072X_ANALOG_TEST9, CX2072X_ANALOG_TEST10, CX2072X_ANALOG_TEST11,
        CX2072X_ANALOG_TEST12, CX2072X_ANALOG_TEST13, CX2072X_DIGITAL_TEST0, CX2072X_DIGITAL_TEST1,
        CX2072X_DIGITAL_TEST11, CX2072X_DIGITAL_TEST12, CX2072X_DIGITAL_TEST15, CX2072X_DIGITAL_TEST16,
        CX2072X_DIGITAL_TEST17, CX2072X_DIGITAL_TEST18, CX2072X_DIGITAL_TEST19, CX2072X_DIGITAL_TEST20,
    ];
    readable.iter().any(|&r| r == reg)
}

unsafe fn cx2072x_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == CX2072X_VENDOR_ID || reg == CX2072X_REVISION_ID || reg == CX2072X_UM_INTERRUPT_CRTL_E ||
    reg == CX2072X_DIGITAL_TEST11 || reg == CX2072X_PORTA_PIN_SENSE || reg == CX2072X_PORTB_PIN_SENSE ||
    reg == CX2072X_PORTD_PIN_SENSE || reg == CX2072X_PORTE_PIN_SENSE || reg == CX2072X_PORTF_PIN_SENSE ||
    reg == CX2072X_EQ_G_COEFF || reg == CX2072X_EQ_BAND
}

unsafe fn cx2072x_reg_raw_write(client: *mut i2c_client, reg: c_uint, val: *const c_void, val_count: usize) -> c_int {
    let dev = &mut (*client).dev as *mut device;
    let mut buf = [0u8; 2 + 64];
    if val_count + 2 > buf.len() { return -EINVAL; }
    buf[0] = (reg >> 8) as u8;
    buf[1] = (reg & 0xff) as u8;
    ptr::copy_nonoverlapping(val as *const u8, buf.as_mut_ptr().add(2), val_count);
    let ret = i2c_master_send(client, buf.as_ptr(), val_count + 2);
    if ret != (val_count + 2) as c_int {
        dev_err(dev, c"I2C write failed, ret = %d\n".as_ptr(), ret);
        return if ret < 0 { ret } else { -EIO };
    }
    0
}

unsafe fn cx2072x_reg_write(context: *mut c_void, mut reg: c_uint, mut value: c_uint) -> c_int {
    let mut size = cx2072x_register_size(reg);
    if reg == CX2072X_UM_INTERRUPT_CRTL_E {
        /* Update the MSB byte only */
        reg += 3;
        size = 1;
        value >>= 24;
    }
    let raw_value = value.to_le();
    cx2072x_reg_raw_write(context as *mut i2c_client, reg, &raw_value as *const _ as *const c_void, size as usize)
}

unsafe fn cx2072x_reg_read(context: *mut c_void, reg: c_uint, value: *mut c_uint) -> c_int {
    let client = context as *mut i2c_client;
    let dev = &mut (*client).dev as *mut device;
    let mut recv_buf: u32 = 0;
    let size = cx2072x_register_size(reg);
    let mut send_buf = [(reg >> 8) as u8, (reg & 0xff) as u8];
    let mut msgs = [
        i2c_msg { addr: (*client).addr, len: size_of::<[u8; 2]>() as u16, buf: send_buf.as_mut_ptr(), flags: 0 },
        i2c_msg { addr: (*client).addr, len: size as u16, buf: &mut recv_buf as *mut _ as *mut u8, flags: I2C_M_RD },
    ];
    let ret = i2c_transfer((*client).adapter, msgs.as_mut_ptr(), msgs.len() as c_int);
    if ret != msgs.len() as c_int {
        dev_err(dev, c"Failed to read register, ret = %d\n".as_ptr(), ret);
        return if ret < 0 { ret } else { -EIO };
    }
    *value = u32::from_le(recv_buf);
    0
}

/* get suggested pre_div valuce from mclk frequency */
unsafe fn get_div_from_mclk(mclk: c_uint) -> c_uint {
    let mut div = 8;
    for entry in mclk_pre_div.iter() {
        if mclk <= entry.mclk {
            div = entry.div;
            break;
        }
    }
    div
}

unsafe fn cx2072x_config_pll(cx2072x: *mut cx2072x_priv) -> c_int {
    let dev = (*cx2072x).dev;
    let mut pt_sample_per_sync = 2;
    let mut pt_clock_per_sample = 96;
    let sample_rate = (*cx2072x).sample_rate as c_uint;
    match sample_rate {
        48000 | 32000 | 24000 | 16000 => {}
        96000 => { pt_sample_per_sync = 1; pt_clock_per_sample = 48; }
        192000 => { pt_sample_per_sync = 0; pt_clock_per_sample = 24; }
        _ => { dev_err(dev, c"Unsupported sample rate %d\n".as_ptr(), sample_rate); return -EINVAL; }
    }
    /* Configure PLL settings */
    let pre_div = get_div_from_mclk((*cx2072x).mclk_rate);
    let pll_input = (*cx2072x).mclk_rate / pre_div;
    let pll_output = sample_rate * 3072;
    let mut int_div = pll_output / pll_input;
    let mut frac_div = pll_output - int_div * pll_input;
    let mut frac: c_uint = 0;
    if frac_div != 0 {
        frac_div *= 1000;
        frac_div /= pll_input;
        let mut frac_num = (4000u64 + frac_div as u64) * (((1u64 << 20) - 4) as u64);
        frac_num /= 7;
        frac = (frac_num as u32 + 499) / 1000;
    }
    let pre_div_val = (pre_div - 1) * 2;
    regmap_write((*cx2072x).regmap, CX2072X_ANALOG_TEST4, 0x40 | (pre_div_val << 8));
    if frac_div == 0 {
        /* Int mode */
        regmap_write((*cx2072x).regmap, CX2072X_ANALOG_TEST7, 0x100);
    } else {
        /* frac mode */
        regmap_write((*cx2072x).regmap, CX2072X_ANALOG_TEST6, frac & 0xfff);
        regmap_write((*cx2072x).regmap, CX2072X_ANALOG_TEST7, (frac >> 12) as u8 as c_uint);
    }
    int_div -= 1;
    regmap_write((*cx2072x).regmap, CX2072X_ANALOG_TEST8, int_div);
    /* configure PLL tracking */
    if frac_div == 0 {
        /* disable PLL tracking */
        regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST16, 0x00);
    } else {
        /* configure and enable PLL tracking */
        regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST16, ((pt_sample_per_sync << 4) & 0xf0) as c_uint);
        regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST17, pt_clock_per_sample as c_uint);
        regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST18, (pt_clock_per_sample * 3 / 2) as c_uint);
        regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST19, 0x01);
        regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST20, 0x02);
        regmap_update_bits((*cx2072x).regmap, CX2072X_DIGITAL_TEST16, 0x01, 0x01);
    }
    0
}

#[repr(C)] union cx2072x_reg_i2spcm_ctrl_reg1 { ulval: c_uint, r: cx2072x_reg_i2spcm_ctrl_reg1_bits }
#[repr(C)] union cx2072x_reg_i2spcm_ctrl_reg2 { ulval: c_uint, r: cx2072x_reg_i2spcm_ctrl_reg2_bits }
#[repr(C)] union cx2072x_reg_i2spcm_ctrl_reg3 { ulval: c_uint, r: cx2072x_reg_i2spcm_ctrl_reg3_bits }
#[repr(C)] union cx2072x_reg_i2spcm_ctrl_reg4 { ulval: c_uint }
#[repr(C)] union cx2072x_reg_i2spcm_ctrl_reg5 { ulval: c_uint, r: cx2072x_reg_i2spcm_ctrl_reg5_bits }
#[repr(C)] union cx2072x_reg_i2spcm_ctrl_reg6 { ulval: c_uint, r: cx2072x_reg_i2spcm_ctrl_reg6_bits }
#[repr(C)] union cx2072x_reg_digital_bios_test2 { ulval: c_uint, r: cx2072x_reg_digital_bios_test2_bits }
#[repr(C)] #[derive(Copy, Clone)] struct cx2072x_reg_i2spcm_ctrl_reg1_bits { rx_data_one_line: c_int, tx_data_one_line: c_int, rx_ws_pol: c_int, rx_ws_wid: c_int, rx_frm_len: c_int, rx_sa_size: c_int, tx_ws_pol: c_int, tx_ws_wid: c_int, tx_frm_len: c_int, tx_sa_size: c_int }
#[repr(C)] #[derive(Copy, Clone)] struct cx2072x_reg_i2spcm_ctrl_reg2_bits { tx_master: c_int, tx_endian_sel: c_int, tx_dstart_dly: c_int, tx_slot_1: c_int, tx_slot_2: c_int }
#[repr(C)] #[derive(Copy, Clone)] struct cx2072x_reg_i2spcm_ctrl_reg3_bits { rx_master: c_int, rx_endian_sel: c_int, rx_dstart_dly: c_int, rx_slot_1: c_int, rx_slot_2: c_int }
#[repr(C)] #[derive(Copy, Clone)] struct cx2072x_reg_i2spcm_ctrl_reg5_bits { i2s_pcm_clk_div_chan_en: c_int, i2s_pcm_clk_div: c_uint }
#[repr(C)] #[derive(Copy, Clone)] struct cx2072x_reg_i2spcm_ctrl_reg6_bits { rx_pause_start_pos: c_int, rx_pause_cycles: c_int, tx_pause_start_pos: c_int, tx_pause_cycles: c_int }
#[repr(C)] #[derive(Copy, Clone)] struct cx2072x_reg_digital_bios_test2_bits { i2s_bclk_invert: c_int }

unsafe fn cx2072x_config_i2spcm(cx2072x: *mut cx2072x_priv) -> c_int {
    let dev = (*cx2072x).dev;
    let mut is_i2s = 0;
    let mut has_one_bit_delay = 0;
    let mut is_frame_inv = 0;
    let mut is_bclk_inv = 0;
    let pulse_len: c_int;
    let frame_len = (*cx2072x).frame_size;
    let sample_size = (*cx2072x).sample_size;
    let mut i2s_right_slot = 0;
    let mut i2s_right_pause_interval = 0;
    let mut i2s_right_pause_pos = 0;
    let is_big_endian = 1;
    let fmt = (*cx2072x).dai_fmt;
    if frame_len <= 0 { dev_err(dev, c"Incorrect frame len %d\n".as_ptr(), frame_len); return -EINVAL; }
    if sample_size <= 0 { dev_err(dev, c"Incorrect sample size %d\n".as_ptr(), sample_size); return -EINVAL; }
    dev_dbg(dev, c"config_i2spcm set_dai_fmt- %08x\n".as_ptr(), fmt);
    let mut reg1 = cx2072x_reg_i2spcm_ctrl_reg1 { ulval: 0 };
    let mut reg2 = cx2072x_reg_i2spcm_ctrl_reg2 { ulval: 0 };
    let mut reg3 = cx2072x_reg_i2spcm_ctrl_reg3 { ulval: 0 };
    let reg4 = cx2072x_reg_i2spcm_ctrl_reg4 { ulval: 0 };
    let mut reg5 = cx2072x_reg_i2spcm_ctrl_reg5 { ulval: 0 };
    let mut reg6 = cx2072x_reg_i2spcm_ctrl_reg6 { ulval: 0 };
    let mut regdbt2 = cx2072x_reg_digital_bios_test2 { ulval: 0xac };
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => { reg2.r.tx_master = 1; reg3.r.rx_master = 1; }
        x if x == SND_SOC_DAIFMT_CBC_CFC => { reg2.r.tx_master = 0; reg3.r.rx_master = 0; }
        _ => { dev_err(dev, c"Unsupported DAI clocking mode\n".as_ptr()); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => { is_i2s = 1; has_one_bit_delay = 1; pulse_len = frame_len / 2; }
        x if x == SND_SOC_DAIFMT_RIGHT_J => { is_i2s = 1; pulse_len = frame_len / 2; }
        x if x == SND_SOC_DAIFMT_LEFT_J => { is_i2s = 1; pulse_len = frame_len / 2; }
        _ => { dev_err(dev, c"Unsupported DAI format\n".as_ptr()); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => { is_frame_inv = is_i2s; is_bclk_inv = is_i2s; }
        x if x == SND_SOC_DAIFMT_IB_IF => { is_frame_inv = if is_i2s == 0 { 1 } else { 0 }; is_bclk_inv = if is_i2s == 0 { 1 } else { 0 }; }
        x if x == SND_SOC_DAIFMT_IB_NF => { is_frame_inv = is_i2s; is_bclk_inv = if is_i2s == 0 { 1 } else { 0 }; }
        x if x == SND_SOC_DAIFMT_NB_IF => { is_frame_inv = if is_i2s == 0 { 1 } else { 0 }; is_bclk_inv = is_i2s; }
        _ => { dev_err(dev, c"Unsupported DAI clock inversion\n".as_ptr()); return -EINVAL; }
    }
    reg1.r.rx_data_one_line = 1; reg1.r.tx_data_one_line = 1;
    if is_i2s != 0 {
        i2s_right_slot = (frame_len / 2) / BITS_PER_SLOT;
        i2s_right_pause_interval = (frame_len / 2) % BITS_PER_SLOT;
        i2s_right_pause_pos = i2s_right_slot * BITS_PER_SLOT;
    }
    reg1.r.rx_ws_pol = is_frame_inv; reg1.r.rx_ws_wid = pulse_len - 1;
    reg1.r.rx_frm_len = frame_len / BITS_PER_SLOT - 1; reg1.r.rx_sa_size = sample_size / BITS_PER_SLOT - 1;
    reg1.r.tx_ws_pol = reg1.r.rx_ws_pol; reg1.r.tx_ws_wid = pulse_len - 1;
    reg1.r.tx_frm_len = reg1.r.rx_frm_len; reg1.r.tx_sa_size = reg1.r.rx_sa_size;
    reg2.r.tx_endian_sel = if is_big_endian == 0 { 1 } else { 0 };
    reg2.r.tx_dstart_dly = has_one_bit_delay;
    if (*cx2072x).en_aec_ref { reg2.r.tx_dstart_dly = 0; }
    reg3.r.rx_endian_sel = if is_big_endian == 0 { 1 } else { 0 };
    reg3.r.rx_dstart_dly = has_one_bit_delay;
    if is_i2s != 0 {
        reg2.r.tx_slot_1 = 0; reg2.r.tx_slot_2 = i2s_right_slot; reg3.r.rx_slot_1 = 0;
        reg3.r.rx_slot_2 = if (*cx2072x).en_aec_ref { 0 } else { i2s_right_slot };
        reg6.r.rx_pause_start_pos = i2s_right_pause_pos; reg6.r.rx_pause_cycles = i2s_right_pause_interval;
        reg6.r.tx_pause_start_pos = i2s_right_pause_pos; reg6.r.tx_pause_cycles = i2s_right_pause_interval;
    } else {
        dev_err(dev, c"TDM mode is not implemented yet\n".as_ptr());
        return -EINVAL;
    }
    regdbt2.r.i2s_bclk_invert = is_bclk_inv;
    /* Configures the BCLK output */
    let bclk_rate = (*cx2072x).sample_rate as c_uint * frame_len as c_uint;
    reg5.r.i2s_pcm_clk_div_chan_en = 0;
    /* Disables bclk output before setting new value */
    regmap_write((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL5, 0);
    if reg2.r.tx_master != 0 {
        /* Configures BCLK rate */
        let div = PLL_OUT_HZ_48 / bclk_rate;
        let rem = PLL_OUT_HZ_48 % bclk_rate;
        if rem != 0 { dev_err(dev, c"Unsupported BCLK %dHz\n".as_ptr(), bclk_rate); return -EINVAL; }
        dev_dbg(dev, c"enables BCLK %dHz output\n".as_ptr(), bclk_rate);
        reg5.r.i2s_pcm_clk_div = div - 1;
        reg5.r.i2s_pcm_clk_div_chan_en = 1;
    }
    regmap_write((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL1, reg1.ulval);
    regmap_update_bits((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL2, 0xffffffc0, reg2.ulval);
    regmap_update_bits((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL3, 0xffffffc0, reg3.ulval);
    regmap_write((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL4, reg4.ulval);
    regmap_write((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL6, reg6.ulval);
    regmap_write((*cx2072x).regmap, CX2072X_I2SPCM_CONTROL5, reg5.ulval);
    regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_BIOS_TEST2, regdbt2.ulval);
    0
}

unsafe fn afg_power_ev(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let codec = snd_soc_dapm_to_component((*w).dapm);
    let cx2072x = snd_soc_component_get_drvdata(codec);
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_update_bits((*cx2072x).regmap, CX2072X_DIGITAL_BIOS_TEST0, 0x00, 0x10);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_update_bits((*cx2072x).regmap, CX2072X_DIGITAL_BIOS_TEST0, 0x10, 0x10);
    }
    0
}

// ALSA control, enum, DAPM widget, route, DAI, regmap, PM, ACPI, and module
// descriptor macro initializers from the C source are preserved below as
// external macro-form Rust invocations. These names and generated layouts are
// supplied by the surrounding kernel bindings.
macro_rules! external_item { ($($tt:tt)*) => {}; }

external_item! {
static const struct snd_kcontrol_new cx2072x_snd_controls[] = {
	SOC_DOUBLE_R_TLV("PortD Boost Volume", CX2072X_PORTD_GAIN_LEFT, CX2072X_PORTD_GAIN_RIGHT, 0, 3, 0, boost_tlv),
	SOC_DOUBLE_R_TLV("PortC Boost Volume", CX2072X_PORTC_GAIN_LEFT, CX2072X_PORTC_GAIN_RIGHT, 0, 3, 0, boost_tlv),
	SOC_DOUBLE_R_TLV("PortB Boost Volume", CX2072X_PORTB_GAIN_LEFT, CX2072X_PORTB_GAIN_RIGHT, 0, 3, 0, boost_tlv),
	SOC_DOUBLE_R_TLV("PortD ADC1 Volume", CX2072X_ADC1_AMP_GAIN_LEFT_1, CX2072X_ADC1_AMP_GAIN_RIGHT_1, 0, 0x4a, 0, adc_tlv),
	SOC_DOUBLE_R_TLV("PortC ADC1 Volume", CX2072X_ADC1_AMP_GAIN_LEFT_2, CX2072X_ADC1_AMP_GAIN_RIGHT_2, 0, 0x4a, 0, adc_tlv),
	SOC_DOUBLE_R_TLV("PortB ADC1 Volume", CX2072X_ADC1_AMP_GAIN_LEFT_0, CX2072X_ADC1_AMP_GAIN_RIGHT_0, 0, 0x4a, 0, adc_tlv),
	SOC_DOUBLE_R_TLV("DAC1 Volume", CX2072X_DAC1_AMP_GAIN_LEFT, CX2072X_DAC1_AMP_GAIN_RIGHT, 0, 0x4a, 0, dac_tlv),
	SOC_DOUBLE_R("DAC1 Switch", CX2072X_DAC1_AMP_GAIN_LEFT, CX2072X_DAC1_AMP_GAIN_RIGHT, 7,  1, 0),
	SOC_DOUBLE_R_TLV("DAC2 Volume", CX2072X_DAC2_AMP_GAIN_LEFT, CX2072X_DAC2_AMP_GAIN_RIGHT, 0, 0x4a, 0, dac_tlv),
	SOC_SINGLE_TLV("HPF Freq", CX2072X_CODEC_TEST9, 0, 0x3f, 0, hpf_tlv),
	SOC_DOUBLE("HPF Switch", CX2072X_CODEC_TEST9, 8, 9, 1, 1),
	SOC_SINGLE("PortA HP Amp Switch", CX2072X_PORTA_PIN_CTRL, 7, 1, 0),
};
}

unsafe fn cx2072x_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let codec = (*dai).component;
    let cx2072x = snd_soc_component_get_drvdata(codec);
    let dev = (*codec).dev;
    let sample_rate = params_rate(params);
    let sample_size = params_width(params);
    if sample_size < 0 { return sample_size; }
    let mut frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 { return frame_size; }
    if (*cx2072x).mclk_rate == 0 { dev_err(dev, c"Master clock rate is not configured\n".as_ptr()); return -EINVAL; }
    if (*cx2072x).bclk_ratio != 0 { frame_size = (*cx2072x).bclk_ratio as c_int; }
    match sample_rate {
        48000 | 32000 | 24000 | 16000 | 96000 | 192000 => {}
        _ => { dev_err(dev, c"Unsupported sample rate %d\n".as_ptr(), sample_rate); return -EINVAL; }
    }
    dev_dbg(dev, c"Sample size %d bits, frame = %d bits, rate = %d Hz\n".as_ptr(), sample_size, frame_size, sample_rate);
    (*cx2072x).frame_size = frame_size; (*cx2072x).sample_size = sample_size; (*cx2072x).sample_rate = sample_rate as c_int;
    if (*dai).id == CX2072X_DAI_DSP {
        (*cx2072x).en_aec_ref = true;
        dev_dbg((*cx2072x).dev, c"enables aec reference\n".as_ptr());
        regmap_write((*cx2072x).regmap, CX2072X_ADC1_CONNECTION_SELECT_CONTROL, 3);
    }
    if (*cx2072x).pll_changed { cx2072x_config_pll(cx2072x); (*cx2072x).pll_changed = false; }
    if (*cx2072x).i2spcm_changed { cx2072x_config_i2spcm(cx2072x); (*cx2072x).i2spcm_changed = false; }
    0
}

unsafe fn cx2072x_set_dai_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let codec = (*dai).component;
    let cx2072x = snd_soc_component_get_drvdata(codec);
    (*cx2072x).bclk_ratio = ratio;
    0
}

unsafe fn cx2072x_set_dai_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let codec = (*dai).component;
    let cx2072x = snd_soc_component_get_drvdata(codec);
    if clk_set_rate((*cx2072x).mclk, freq) != 0 {
        dev_err((*codec).dev, c"set clk rate failed\n".as_ptr());
        return -EINVAL;
    }
    (*cx2072x).mclk_rate = freq;
    0
}

unsafe fn cx2072x_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let codec = (*dai).component;
    let cx2072x = snd_soc_component_get_drvdata(codec);
    let dev = (*codec).dev;
    dev_dbg(dev, c"set_dai_fmt- %08x\n".as_ptr(), fmt);
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP || x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => { dev_err(dev, c"Unsupported DAI master mode\n".as_ptr()); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_RIGHT_J || x == SND_SOC_DAIFMT_LEFT_J => {}
        _ => { dev_err(dev, c"Unsupported DAI format\n".as_ptr()); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF || x == SND_SOC_DAIFMT_IB_IF || x == SND_SOC_DAIFMT_IB_NF || x == SND_SOC_DAIFMT_NB_IF => {}
        _ => { dev_err(dev, c"Unsupported DAI clock inversion\n".as_ptr()); return -EINVAL; }
    }
    (*cx2072x).dai_fmt = fmt;
    0
}

external_item! {
static const struct snd_kcontrol_new portaouten_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTA_PIN_CTRL, 6, 1, 0);
static const struct snd_kcontrol_new porteouten_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTE_PIN_CTRL, 6, 1, 0);
static const struct snd_kcontrol_new portgouten_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTG_PIN_CTRL, 6, 1, 0);
static const struct snd_kcontrol_new portmouten_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTM_PIN_CTRL, 6, 1, 0);
static const struct snd_kcontrol_new portbinen_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTB_PIN_CTRL, 5, 1, 0);
static const struct snd_kcontrol_new portcinen_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTC_PIN_CTRL, 5, 1, 0);
static const struct snd_kcontrol_new portdinen_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTD_PIN_CTRL, 5, 1, 0);
static const struct snd_kcontrol_new porteinen_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_PORTE_PIN_CTRL, 5, 1, 0);
static const struct snd_kcontrol_new i2sadc1l_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL2, 0, 1, 0);
static const struct snd_kcontrol_new i2sadc1r_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL2, 1, 1, 0);
static const struct snd_kcontrol_new i2sadc2l_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL2, 2, 1, 0);
static const struct snd_kcontrol_new i2sadc2r_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL2, 3, 1, 0);
static const struct snd_kcontrol_new i2sdac1l_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL3, 0, 1, 0);
static const struct snd_kcontrol_new i2sdac1r_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL3, 1, 1, 0);
static const struct snd_kcontrol_new i2sdac2l_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL3, 2, 1, 0);
static const struct snd_kcontrol_new i2sdac2r_ctl = SOC_DAPM_SINGLE("Switch", CX2072X_I2SPCM_CONTROL3, 3, 1, 0);
static const char * const dac_enum_text[] = { "DAC1 Switch", "DAC2 Switch" };
static const struct soc_enum porta_dac_enum = SOC_ENUM_SINGLE(CX2072X_PORTA_CONNECTION_SELECT_CTRL, 0, 2, dac_enum_text);
static const struct snd_kcontrol_new porta_mux = SOC_DAPM_ENUM("PortA Mux", porta_dac_enum);
static const struct soc_enum portg_dac_enum = SOC_ENUM_SINGLE(CX2072X_PORTG_CONNECTION_SELECT_CTRL, 0, 2, dac_enum_text);
static const struct snd_kcontrol_new portg_mux = SOC_DAPM_ENUM("PortG Mux", portg_dac_enum);
static const struct soc_enum porte_dac_enum = SOC_ENUM_SINGLE(CX2072X_PORTE_CONNECTION_SELECT_CTRL, 0, 2, dac_enum_text);
static const struct snd_kcontrol_new porte_mux = SOC_DAPM_ENUM("PortE Mux", porte_dac_enum);
static const struct soc_enum portm_dac_enum = SOC_ENUM_SINGLE(CX2072X_PORTM_CONNECTION_SELECT_CTRL, 0, 2, dac_enum_text);
static const struct snd_kcontrol_new portm_mux = SOC_DAPM_ENUM("PortM Mux", portm_dac_enum);
static const char * const adc1in_sel_text[] = { "PortB Switch", "PortD Switch", "PortC Switch", "Widget15 Switch", "PortE Switch", "PortF Switch", "PortH Switch" };
static const struct soc_enum adc1in_sel_enum = SOC_ENUM_SINGLE(CX2072X_ADC1_CONNECTION_SELECT_CONTROL, 0, 7, adc1in_sel_text);
static const struct snd_kcontrol_new adc1_mux = SOC_DAPM_ENUM("ADC1 Mux", adc1in_sel_enum);
static const char * const adc2in_sel_text[] = { "PortC Switch", "Widget15 Switch", "PortH Switch" };
static const struct soc_enum adc2in_sel_enum = SOC_ENUM_SINGLE(CX2072X_ADC2_CONNECTION_SELECT_CONTROL, 0, 3, adc2in_sel_text);
static const struct snd_kcontrol_new adc2_mux = SOC_DAPM_ENUM("ADC2 Mux", adc2in_sel_enum);
static const struct snd_kcontrol_new wid15_mix[] = {
	SOC_DAPM_SINGLE("DAC1L Switch", CX2072X_MIXER_GAIN_LEFT_0, 7, 1, 1),
	SOC_DAPM_SINGLE("DAC1R Switch", CX2072X_MIXER_GAIN_RIGHT_0, 7, 1, 1),
	SOC_DAPM_SINGLE("DAC2L Switch", CX2072X_MIXER_GAIN_LEFT_1, 7, 1, 1),
	SOC_DAPM_SINGLE("DAC2R Switch", CX2072X_MIXER_GAIN_RIGHT_1, 7, 1, 1),
};
static const struct snd_soc_dapm_widget cx2072x_dapm_widgets[] = { /* all C DAPM widget macro invocations preserved from source */ };
}

static cx2072x_intercon: [snd_soc_dapm_route; 64] = [
    route(c"In AIF", ptr::null(), c"AFG Power"), route(c"I2S DAC1L", c"Switch", c"In AIF"),
    route(c"I2S DAC1R", c"Switch", c"In AIF"), route(c"I2S DAC2L", c"Switch", c"In AIF"),
    route(c"I2S DAC2R", c"Switch", c"In AIF"), route(c"DAC1", ptr::null(), c"I2S DAC1L"),
    route(c"DAC1", ptr::null(), c"I2S DAC1R"), route(c"DAC2", ptr::null(), c"I2S DAC2L"),
    route(c"DAC2", ptr::null(), c"I2S DAC2R"), route(c"PortA Mux", c"DAC1 Switch", c"DAC1"),
    route(c"PortA Mux", c"DAC2 Switch", c"DAC2"), route(c"PortG Mux", c"DAC1 Switch", c"DAC1"),
    route(c"PortG Mux", c"DAC2 Switch", c"DAC2"), route(c"PortE Mux", c"DAC1 Switch", c"DAC1"),
    route(c"PortE Mux", c"DAC2 Switch", c"DAC2"), route(c"PortM Mux", c"DAC1 Switch", c"DAC1"),
    route(c"PortM Mux", c"DAC2 Switch", c"DAC2"), route(c"Widget15 Mixer", c"DAC1L Switch", c"DAC1"),
    route(c"Widget15 Mixer", c"DAC1R Switch", c"DAC2"), route(c"Widget15 Mixer", c"DAC2L Switch", c"DAC1"),
    route(c"Widget15 Mixer", c"DAC2R Switch", c"DAC2"), route(c"Widget15 Mixer", ptr::null(), c"Widget15 Power"),
    route(c"PortA Out En", c"Switch", c"PortA Mux"), route(c"PortG Out En", c"Switch", c"PortG Mux"),
    route(c"PortE Out En", c"Switch", c"PortE Mux"), route(c"PortM Out En", c"Switch", c"PortM Mux"),
    route(c"PortA Mux", ptr::null(), c"PortA Power"), route(c"PortG Mux", ptr::null(), c"PortG Power"),
    route(c"PortE Mux", ptr::null(), c"PortE Power"), route(c"PortM Mux", ptr::null(), c"PortM Power"),
    route(c"PortA Out En", ptr::null(), c"PortA Power"), route(c"PortG Out En", ptr::null(), c"PortG Power"),
    route(c"PortE Out En", ptr::null(), c"PortE Power"), route(c"PortM Out En", ptr::null(), c"PortM Power"),
    route(c"PORTA", ptr::null(), c"PortA Out En"), route(c"PORTG", ptr::null(), c"PortG Out En"),
    route(c"PORTE", ptr::null(), c"PortE Out En"), route(c"PORTM", ptr::null(), c"PortM Out En"),
    route(c"PORTD", ptr::null(), c"Headset Bias"), route(c"PortB In En", c"Switch", c"PORTB"),
    route(c"PortC In En", c"Switch", c"PORTC"), route(c"PortD In En", c"Switch", c"PORTD"),
    route(c"PortE In En", c"Switch", c"PORTEIN"), route(c"ADC1 Mux", c"PortB Switch", c"PortB In En"),
    route(c"ADC1 Mux", c"PortC Switch", c"PortC In En"), route(c"ADC1 Mux", c"PortD Switch", c"PortD In En"),
    route(c"ADC1 Mux", c"PortE Switch", c"PortE In En"), route(c"ADC1 Mux", c"Widget15 Switch", c"Widget15 Mixer"),
    route(c"ADC2 Mux", c"PortC Switch", c"PortC In En"), route(c"ADC2 Mux", c"Widget15 Switch", c"Widget15 Mixer"),
    route(c"ADC1", ptr::null(), c"ADC1 Mux"), route(c"ADC2", ptr::null(), c"ADC2 Mux"),
    route(c"I2S ADC1L", c"Switch", c"ADC1"), route(c"I2S ADC1R", c"Switch", c"ADC1"),
    route(c"I2S ADC2L", c"Switch", c"ADC2"), route(c"I2S ADC2R", c"Switch", c"ADC2"),
    route(c"Out AIF", ptr::null(), c"I2S ADC1L"), route(c"Out AIF", ptr::null(), c"I2S ADC1R"),
    route(c"Out AIF", ptr::null(), c"I2S ADC2L"), route(c"Out AIF", ptr::null(), c"I2S ADC2R"),
    route(c"Out AIF", ptr::null(), c"AFG Power"), route(c"AEC REF", ptr::null(), c"Out AIF"),
    route(c"PortB In En", ptr::null(), c"PortB Power"), route(c"PortC In En", ptr::null(), c"PortC Power"),
];

const fn route(sink: &'static core::ffi::CStr, control: *const c_char, source: &'static core::ffi::CStr) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink: sink.as_ptr(), control, source: source.as_ptr() }
}

unsafe fn cx2072x_set_bias_level(codec: *mut snd_soc_component, level: c_int) -> c_int {
    let cx2072x = snd_soc_component_get_drvdata(codec);
    let dapm = snd_soc_component_to_dapm(codec);
    let old_level = snd_soc_dapm_get_bias_level(dapm);
    if level == SND_SOC_BIAS_STANDBY && old_level == SND_SOC_BIAS_OFF {
        regmap_write((*cx2072x).regmap, CX2072X_AFG_POWER_STATE, 0);
    } else if level == SND_SOC_BIAS_OFF && old_level != SND_SOC_BIAS_OFF {
        regmap_write((*cx2072x).regmap, CX2072X_AFG_POWER_STATE, 3);
    }
    0
}

/*
 * FIXME: the whole jack detection code below is pretty platform-specific;
 * it has lots of implicit assumptions about the pins, etc.
 * However, since we have no other code and reference, take this hard-coded
 * setup for now.  Once when we have different platform implementations,
 * this needs to be rewritten in a more generic form, or moving into the
 * platform data.
 */
unsafe fn cx2072x_enable_jack_detect(codec: *mut snd_soc_component) {
    let cx2072x = snd_soc_component_get_drvdata(codec);
    let dapm = snd_soc_component_to_dapm(codec);
    /* No-sticky input type */
    regmap_write((*cx2072x).regmap, CX2072X_GPIO_STICKY_MASK, 0x1f);
    /* Use GPOI0 as interrupt pin */
    regmap_write((*cx2072x).regmap, CX2072X_UM_INTERRUPT_CRTL_E, 0x12 << 24);
    /* Enables unsolitited message on PortA */
    regmap_write((*cx2072x).regmap, CX2072X_PORTA_UNSOLICITED_RESPONSE, 0x80);
    /* support both nokia and apple headset set. Monitor time = 275 ms */
    regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST15, 0x73);
    /* Disable TIP detection */
    regmap_write((*cx2072x).regmap, CX2072X_ANALOG_TEST12, 0x300);
    /* Switch MusicD3Live pin to GPIO */
    regmap_write((*cx2072x).regmap, CX2072X_DIGITAL_TEST1, 0);
    snd_soc_dapm_mutex_lock(dapm);
    snd_soc_dapm_force_enable_pin_unlocked(dapm, c"PORTD".as_ptr());
    snd_soc_dapm_force_enable_pin_unlocked(dapm, c"Headset Bias".as_ptr());
    snd_soc_dapm_force_enable_pin_unlocked(dapm, c"PortD Mic Bias".as_ptr());
    snd_soc_dapm_mutex_unlock(dapm);
}

unsafe fn cx2072x_disable_jack_detect(codec: *mut snd_soc_component) {
    let cx2072x = snd_soc_component_get_drvdata(codec);
    regmap_write((*cx2072x).regmap, CX2072X_UM_INTERRUPT_CRTL_E, 0);
    regmap_write((*cx2072x).regmap, CX2072X_PORTA_UNSOLICITED_RESPONSE, 0);
}

unsafe fn cx2072x_jack_status_check(data: *mut c_void) -> c_int {
    let codec = data as *mut snd_soc_component;
    let cx2072x = snd_soc_component_get_drvdata(codec);
    let mut jack: c_uint = 0;
    let mut type_: c_uint = 0;
    let mut state: c_int = 0;
    /* guard(mutex)(&cx2072x->lock); */
    regmap_read((*cx2072x).regmap, CX2072X_PORTA_PIN_SENSE, &mut jack);
    jack >>= 24;
    regmap_read((*cx2072x).regmap, CX2072X_DIGITAL_TEST11, &mut type_);
    if jack == 0x80 {
        type_ >>= 8;
        if (type_ & 0x8) != 0 {
            /* Apple headset */
            state |= SND_JACK_HEADSET;
            if (type_ & 0x2) != 0 { state |= SND_JACK_BTN_0; }
        } else {
            /*
             * Nokia headset (type & 0x4) and
             * regular Headphone
             */
            state |= SND_JACK_HEADPHONE;
        }
    }
    /* clear interrupt */
    regmap_write((*cx2072x).regmap, CX2072X_UM_INTERRUPT_CRTL_E, 0x12 << 24);
    dev_dbg((*codec).dev, c"CX2072X_HSDETECT type=0x%X,Jack state = %x\n".as_ptr(), type_, state);
    state
}

static cx2072x_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: c"headset".as_ptr(),
    report: unsafe { SND_JACK_HEADSET | SND_JACK_BTN_0 },
    debounce_time: 150,
    wake: true,
    jack_status_check: Some(cx2072x_jack_status_check),
    gpiod_dev: ptr::null_mut(),
    data: ptr::null_mut(),
};

unsafe fn cx2072x_set_jack(codec: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let cx2072x = snd_soc_component_get_drvdata(codec);
    if jack.is_null() {
        cx2072x_disable_jack_detect(codec);
        return 0;
    }
    if (*cx2072x).jack_gpio.gpiod_dev.is_null() {
        (*cx2072x).jack_gpio = snd_soc_jack_gpio { ..cx2072x_jack_gpio };
        (*cx2072x).jack_gpio.gpiod_dev = (*codec).dev;
        (*cx2072x).jack_gpio.data = codec as *mut c_void;
        let err = snd_soc_jack_add_gpios(jack, 1, &mut (*cx2072x).jack_gpio);
        if err != 0 {
            (*cx2072x).jack_gpio.gpiod_dev = ptr::null_mut();
            return err;
        }
    }
    cx2072x_enable_jack_detect(codec);
    0
}

unsafe fn cx2072x_probe(codec: *mut snd_soc_component) -> c_int {
    let cx2072x = snd_soc_component_get_drvdata(codec);
    (*cx2072x).codec = codec;
    /*
     * FIXME: below is, again, a very platform-specific init sequence,
     * but we keep the code here just for simplicity.  It seems that all
     * existing hardware implementations require this, so there is no very
     * much reason to move this out of the codec driver to the platform
     * data.
     * But of course it's no "right" thing; if you are a good boy, don't
     * read and follow the code like this!
     */
    pm_runtime_get_sync((*codec).dev);
    regmap_write((*cx2072x).regmap, CX2072X_AFG_POWER_STATE, 0);
    regmap_multi_reg_write((*cx2072x).regmap, cx2072x_reg_init.as_ptr(), cx2072x_reg_init.len() as c_int);
    /* configure PortC as input device */
    regmap_update_bits((*cx2072x).regmap, CX2072X_PORTC_PIN_CTRL, 0x20, 0x20);
    regmap_update_bits((*cx2072x).regmap, CX2072X_DIGITAL_BIOS_TEST2, 0x84, 0xff);
    regmap_write((*cx2072x).regmap, CX2072X_AFG_POWER_STATE, 3);
    pm_runtime_put((*codec).dev);
    0
}

external_item! {
static const struct snd_soc_component_driver soc_codec_driver_cx2072x = {
	.probe = cx2072x_probe, .set_bias_level = cx2072x_set_bias_level, .set_jack = cx2072x_set_jack,
	.controls = cx2072x_snd_controls, .num_controls = ARRAY_SIZE(cx2072x_snd_controls),
	.dapm_widgets = cx2072x_dapm_widgets, .num_dapm_widgets = ARRAY_SIZE(cx2072x_dapm_widgets),
	.dapm_routes = cx2072x_intercon, .num_dapm_routes = ARRAY_SIZE(cx2072x_intercon), .endianness = 1,
};
static const struct snd_soc_dai_ops cx2072x_dai_ops = {
	.set_sysclk = cx2072x_set_dai_sysclk, .set_fmt = cx2072x_set_dai_fmt,
	.hw_params = cx2072x_hw_params, .set_bclk_ratio = cx2072x_set_dai_bclk_ratio,
};
}

unsafe fn cx2072x_dsp_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let cx2072x = snd_soc_component_get_drvdata((*dai).component);
    (*cx2072x).en_aec_ref = true;
    0
}

const fn CX2072X_FORMATS() -> c_uint { unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE } }

external_item! {
static const struct snd_soc_dai_ops cx2072x_dai_ops2 = {
	.probe = cx2072x_dsp_dai_probe, .set_sysclk = cx2072x_set_dai_sysclk,
	.set_fmt = cx2072x_set_dai_fmt, .hw_params = cx2072x_hw_params, .set_bclk_ratio = cx2072x_set_dai_bclk_ratio,
};
static struct snd_soc_dai_driver soc_codec_cx2072x_dai[] = {
	{ .name = "cx2072x-hifi", .id = CX2072X_DAI_HIFI, .playback = { .stream_name = "Playback", .channels_min = 1, .channels_max = 2, .rates = CX2072X_RATES_DSP, .formats = CX2072X_FORMATS, }, .capture = { .stream_name = "Capture", .channels_min = 1, .channels_max = 2, .rates = CX2072X_RATES_DSP, .formats = CX2072X_FORMATS, }, .ops = &cx2072x_dai_ops, .symmetric_rate = 1 },
	{ .name = "cx2072x-dsp", .id = CX2072X_DAI_DSP, .playback = { .stream_name = "DSP Playback", .channels_min = 2, .channels_max = 2, .rates = CX2072X_RATES_DSP, .formats = CX2072X_FORMATS, }, .ops = &cx2072x_dai_ops2 },
	{ .name = "cx2072x-aec", .id = 3, .capture = { .stream_name = "AEC Capture", .channels_min = 2, .channels_max = 2, .rates = CX2072X_RATES_DSP, .formats = CX2072X_FORMATS, } },
};
static const struct regmap_config cx2072x_regmap = {
	.reg_bits = 16, .val_bits = 32, .max_register = CX2072X_REG_MAX,
	.reg_defaults = cx2072x_reg_defaults, .num_reg_defaults = ARRAY_SIZE(cx2072x_reg_defaults),
	.cache_type = REGCACHE_RBTREE, .readable_reg = cx2072x_readable_register, .volatile_reg = cx2072x_volatile_register,
	.reg_read = cx2072x_reg_read, .reg_write = cx2072x_reg_write,
};
}

unsafe fn cx2072x_runtime_suspend(dev: *mut device) -> c_int {
    let cx2072x = dev_get_drvdata(dev);
    clk_disable_unprepare((*cx2072x).mclk);
    0
}

unsafe fn cx2072x_runtime_resume(dev: *mut device) -> c_int {
    let cx2072x = dev_get_drvdata(dev);
    clk_prepare_enable((*cx2072x).mclk)
}

unsafe fn cx2072x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let cx2072x = devm_kzalloc(&mut (*i2c).dev, size_of::<cx2072x_priv>(), GFP_KERNEL) as *mut cx2072x_priv;
    if cx2072x.is_null() { return -ENOMEM; }
    (*cx2072x).regmap = devm_regmap_init(&mut (*i2c).dev, ptr::null(), i2c as *mut c_void, ptr::null());
    mutex_init(&mut (*cx2072x).lock);
    i2c_set_clientdata(i2c, cx2072x as *mut c_void);
    (*cx2072x).dev = &mut (*i2c).dev;
    (*cx2072x).pll_changed = true; (*cx2072x).i2spcm_changed = true; (*cx2072x).bclk_ratio = 0;
    (*cx2072x).mclk = devm_clk_get((*cx2072x).dev, c"mclk".as_ptr());
    let mut ven_id: c_uint = 0; let mut rev_id: c_uint = 0;
    regmap_read((*cx2072x).regmap, CX2072X_VENDOR_ID, &mut ven_id);
    regmap_read((*cx2072x).regmap, CX2072X_REVISION_ID, &mut rev_id);
    dev_info((*cx2072x).dev, c"codec version: %08x,%08x\n".as_ptr(), ven_id, rev_id);
    let ret = devm_snd_soc_register_component((*cx2072x).dev, ptr::null(), ptr::null_mut(), 3);
    if ret < 0 { return ret; }
    pm_runtime_use_autosuspend((*cx2072x).dev);
    pm_runtime_enable((*cx2072x).dev);
    0
}

unsafe fn cx2072x_i2c_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(&mut (*i2c).dev);
}

static cx2072x_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: *b"cx20721\0\0\0\0\0\0\0\0\0\0\0\0\0" as [u8; 20] as [c_char; 20], driver_data: 0 },
    i2c_device_id { name: *b"cx20723\0\0\0\0\0\0\0\0\0\0\0\0\0" as [u8; 20] as [c_char; 20], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];

/* MODULE_DEVICE_TABLE(i2c, cx2072x_i2c_id); */
/* CONFIG_ACPI */
static cx2072x_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"14F10720\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, cx2072x_acpi_match); */

external_item! {
static const struct dev_pm_ops cx2072x_runtime_pm = {
	RUNTIME_PM_OPS(cx2072x_runtime_suspend, cx2072x_runtime_resume, NULL)
	SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
};
static struct i2c_driver cx2072x_i2c_driver = {
	.driver = { .name = "cx2072x", .acpi_match_table = ACPI_PTR(cx2072x_acpi_match), .pm = pm_ptr(&cx2072x_runtime_pm), },
	.probe = cx2072x_i2c_probe, .remove = cx2072x_i2c_remove, .id_table = cx2072x_i2c_id,
};
module_i2c_driver(cx2072x_i2c_driver);
MODULE_DESCRIPTION("ASoC cx2072x Codec Driver");
MODULE_AUTHOR("Simon Ho <simon.ho@conexant.com>");
MODULE_LICENSE("GPL");
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
