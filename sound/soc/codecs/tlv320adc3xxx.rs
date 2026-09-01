// SPDX-License-Identifier: GPL-2.0-only
//
// Based on sound/soc/codecs/tlv320aic3x.c by  Vladimir Barinov
//
// Copyright (C) 2010 Mistral Solutions Pvt Ltd.
// Author: Shahina Shaik <shahina.s@mistralsolutions.com>
//
// Copyright (C) 2014-2018, Ambarella, Inc.
// Author: Dongge wu <dgwu@ambarella.com>
//
// Copyright (C) 2021 Axis Communications AB
// Author: Ricard Wanderlof <ricardw@axis.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint32_t = u32;

/*
 * External Linux/ASoC declarations provided by included headers in the C
 * source. They are declarations only; definitions are supplied elsewhere.
 */
#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct device_node { _priv: [u8; 0] }
#[repr(C)] pub struct clk { _priv: [u8; 0] }
#[repr(C)] pub struct regmap { _priv: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _priv: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)] pub struct snd_ctl_elem_info_integer { pub min: i64, pub max: i64 }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct i2c_client { pub dev: device }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default { pub reg: c_uint, pub def: c_uint }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_range_cfg {
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
    pub max_register: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub owner: *mut c_void,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub can_sleep: c_int,
    pub ngpio: c_uint,
    pub parent: *mut device,
    pub base: c_int,
}

#[repr(C)] pub struct snd_kcontrol_new { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _priv2: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>, pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_soc_component_driver { pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget, pub num_dapm_widgets: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_uint, pub endianness: c_uint }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 32], pub driver_data: c_ulong }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct i2c_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>, pub id_table: *const i2c_device_id }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub of_match_table: *const of_device_id }

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_dapm_del_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn gpiochip_remove(chip: *mut gpio_chip);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut c_uint) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut c_void;
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 3;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0;
const ADC3XXX_GPIO_GPI: c_uint = 0;
const ADC3XXX_GPIO_GPO: c_uint = 1;
const ADC3XXX_MICBIAS_OFF: c_uint = 0;
const ADC3XXX_MICBIAS_AVDD: c_uint = 3;

const ADC3XXX_MICBIAS_PINS: usize = 2;
const ADC3XXX_GPIO_PINS: usize = 2;
const ADC3XXX_GPIOS_MAX: usize = ADC3XXX_MICBIAS_PINS + ADC3XXX_GPIO_PINS;
const ADC3XXX_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const ADC3XXX_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE;
const ADC3XXX_PLL_AUTO: c_uint = 0;
const ADC3XXX_PLL_ENABLE: c_uint = 1;
const ADC3XXX_PLL_BYPASS: c_uint = 2;
const ADC3XXX_PAGE_SIZE: c_uint = 128;
const fn ADC3XXX_REG(page: c_uint, reg: c_uint) -> c_uint { page * ADC3XXX_PAGE_SIZE + reg }

const ADC3XXX_PAGE_SELECT: c_uint = ADC3XXX_REG(0, 0);
const ADC3XXX_RESET: c_uint = ADC3XXX_REG(0, 1);
const ADC3XXX_CLKGEN_MUX: c_uint = ADC3XXX_REG(0, 4);
const ADC3XXX_PLL_PROG_PR: c_uint = ADC3XXX_REG(0, 5);
const ADC3XXX_PLL_PROG_J: c_uint = ADC3XXX_REG(0, 6);
const ADC3XXX_PLL_PROG_D_MSB: c_uint = ADC3XXX_REG(0, 7);
const ADC3XXX_PLL_PROG_D_LSB: c_uint = ADC3XXX_REG(0, 8);
const ADC3XXX_ADC_NADC: c_uint = ADC3XXX_REG(0, 18);
const ADC3XXX_ADC_MADC: c_uint = ADC3XXX_REG(0, 19);
const ADC3XXX_ADC_AOSR: c_uint = ADC3XXX_REG(0, 20);
const ADC3XXX_ADC_IADC: c_uint = ADC3XXX_REG(0, 21);
const ADC3XXX_CLKOUT_MUX: c_uint = ADC3XXX_REG(0, 25);
const ADC3XXX_CLKOUT_M_DIV: c_uint = ADC3XXX_REG(0, 26);
const ADC3XXX_INTERFACE_CTRL_1: c_uint = ADC3XXX_REG(0, 27);
const ADC3XXX_CH_OFFSET_1: c_uint = ADC3XXX_REG(0, 28);
const ADC3XXX_INTERFACE_CTRL_2: c_uint = ADC3XXX_REG(0, 29);
const ADC3XXX_BCLK_N_DIV: c_uint = ADC3XXX_REG(0, 30);
const ADC3XXX_INTERFACE_CTRL_3: c_uint = ADC3XXX_REG(0, 31);
const ADC3XXX_INTERFACE_CTRL_4: c_uint = ADC3XXX_REG(0, 32);
const ADC3XXX_INTERFACE_CTRL_5: c_uint = ADC3XXX_REG(0, 33);
const ADC3XXX_I2S_SYNC: c_uint = ADC3XXX_REG(0, 34);
const ADC3XXX_ADC_FLAG: c_uint = ADC3XXX_REG(0, 36);
const ADC3XXX_CH_OFFSET_2: c_uint = ADC3XXX_REG(0, 37);
const ADC3XXX_I2S_TDM_CTRL: c_uint = ADC3XXX_REG(0, 38);
const ADC3XXX_INTR_FLAG_1: c_uint = ADC3XXX_REG(0, 42);
const ADC3XXX_INTR_FLAG_2: c_uint = ADC3XXX_REG(0, 43);
const ADC3XXX_INTR_FLAG_ADC1: c_uint = ADC3XXX_REG(0, 45);
const ADC3XXX_INTR_FLAG_ADC2: c_uint = ADC3XXX_REG(0, 47);
const ADC3XXX_INT1_CTRL: c_uint = ADC3XXX_REG(0, 48);
const ADC3XXX_INT2_CTRL: c_uint = ADC3XXX_REG(0, 49);
const ADC3XXX_GPIO2_CTRL: c_uint = ADC3XXX_REG(0, 51);
const ADC3XXX_GPIO1_CTRL: c_uint = ADC3XXX_REG(0, 52);
const ADC3XXX_DOUT_CTRL: c_uint = ADC3XXX_REG(0, 53);
const ADC3XXX_SYNC_CTRL_1: c_uint = ADC3XXX_REG(0, 57);
const ADC3XXX_SYNC_CTRL_2: c_uint = ADC3XXX_REG(0, 58);
const ADC3XXX_CIC_GAIN_CTRL: c_uint = ADC3XXX_REG(0, 59);
const ADC3XXX_PRB_SELECT: c_uint = ADC3XXX_REG(0, 61);
const ADC3XXX_INST_MODE_CTRL: c_uint = ADC3XXX_REG(0, 62);
const ADC3XXX_MIC_POLARITY_CTRL: c_uint = ADC3XXX_REG(0, 80);
const ADC3XXX_ADC_DIGITAL: c_uint = ADC3XXX_REG(0, 81);
const ADC3XXX_ADC_FGA: c_uint = ADC3XXX_REG(0, 82);
const ADC3XXX_LADC_VOL: c_uint = ADC3XXX_REG(0, 83);
const ADC3XXX_RADC_VOL: c_uint = ADC3XXX_REG(0, 84);
const ADC3XXX_ADC_PHASE_COMP: c_uint = ADC3XXX_REG(0, 85);
const ADC3XXX_LEFT_CHN_AGC_1: c_uint = ADC3XXX_REG(0, 86);
const ADC3XXX_LEFT_CHN_AGC_2: c_uint = ADC3XXX_REG(0, 87);
const ADC3XXX_LEFT_CHN_AGC_3: c_uint = ADC3XXX_REG(0, 88);
const ADC3XXX_LEFT_CHN_AGC_4: c_uint = ADC3XXX_REG(0, 89);
const ADC3XXX_LEFT_CHN_AGC_5: c_uint = ADC3XXX_REG(0, 90);
const ADC3XXX_LEFT_CHN_AGC_6: c_uint = ADC3XXX_REG(0, 91);
const ADC3XXX_LEFT_CHN_AGC_7: c_uint = ADC3XXX_REG(0, 92);
const ADC3XXX_LEFT_AGC_GAIN: c_uint = ADC3XXX_REG(0, 93);
const ADC3XXX_RIGHT_CHN_AGC_1: c_uint = ADC3XXX_REG(0, 94);
const ADC3XXX_RIGHT_CHN_AGC_2: c_uint = ADC3XXX_REG(0, 95);
const ADC3XXX_RIGHT_CHN_AGC_3: c_uint = ADC3XXX_REG(0, 96);
const ADC3XXX_RIGHT_CHN_AGC_4: c_uint = ADC3XXX_REG(0, 97);
const ADC3XXX_RIGHT_CHN_AGC_5: c_uint = ADC3XXX_REG(0, 98);
const ADC3XXX_RIGHT_CHN_AGC_6: c_uint = ADC3XXX_REG(0, 99);
const ADC3XXX_RIGHT_CHN_AGC_7: c_uint = ADC3XXX_REG(0, 100);
const ADC3XXX_RIGHT_AGC_GAIN: c_uint = ADC3XXX_REG(0, 101);
const ADC3XXX_DITHER_CTRL: c_uint = ADC3XXX_REG(1, 26);
const ADC3XXX_MICBIAS_CTRL: c_uint = ADC3XXX_REG(1, 51);
const ADC3XXX_LEFT_PGA_SEL_1: c_uint = ADC3XXX_REG(1, 52);
const ADC3XXX_LEFT_PGA_SEL_2: c_uint = ADC3XXX_REG(1, 54);
const ADC3XXX_RIGHT_PGA_SEL_1: c_uint = ADC3XXX_REG(1, 55);
const ADC3XXX_RIGHT_PGA_SEL_2: c_uint = ADC3XXX_REG(1, 57);
const ADC3XXX_LEFT_APGA_CTRL: c_uint = ADC3XXX_REG(1, 59);
const ADC3XXX_RIGHT_APGA_CTRL: c_uint = ADC3XXX_REG(1, 60);
const ADC3XXX_LOW_CURRENT_MODES: c_uint = ADC3XXX_REG(1, 61);
const ADC3XXX_ANALOG_PGA_FLAGS: c_uint = ADC3XXX_REG(1, 62);
const ADC3XXX_LEFT_ADC_IIR_COEFF_N0_MSB: c_uint = ADC3XXX_REG(4, 8);
const ADC3XXX_LEFT_ADC_IIR_COEFF_N0_LSB: c_uint = ADC3XXX_REG(4, 9);
const ADC3XXX_LEFT_ADC_IIR_COEFF_N1_MSB: c_uint = ADC3XXX_REG(4, 10);
const ADC3XXX_LEFT_ADC_IIR_COEFF_N1_LSB: c_uint = ADC3XXX_REG(4, 11);
const ADC3XXX_LEFT_ADC_IIR_COEFF_D1_MSB: c_uint = ADC3XXX_REG(4, 12);
const ADC3XXX_LEFT_ADC_IIR_COEFF_D1_LSB: c_uint = ADC3XXX_REG(4, 13);
const ADC3XXX_RIGHT_ADC_IIR_COEFF_N0_MSB: c_uint = ADC3XXX_REG(4, 72);
const ADC3XXX_RIGHT_ADC_IIR_COEFF_N0_LSB: c_uint = ADC3XXX_REG(4, 73);
const ADC3XXX_RIGHT_ADC_IIR_COEFF_N1_MSB: c_uint = ADC3XXX_REG(4, 74);
const ADC3XXX_RIGHT_ADC_IIR_COEFF_N1_LSB: c_uint = ADC3XXX_REG(4, 75);
const ADC3XXX_RIGHT_ADC_IIR_COEFF_D1_MSB: c_uint = ADC3XXX_REG(4, 76);
const ADC3XXX_RIGHT_ADC_IIR_COEFF_D1_LSB: c_uint = ADC3XXX_REG(4, 77);

const ADC3XXX_ENABLE_PLL_SHIFT: c_uint = 7;
const ADC3XXX_ENABLE_PLL: c_uint = 1 << ADC3XXX_ENABLE_PLL_SHIFT;
const ADC3XXX_ENABLE_NADC_SHIFT: c_uint = 7;
const ADC3XXX_ENABLE_NADC: c_uint = 1 << ADC3XXX_ENABLE_NADC_SHIFT;
const ADC3XXX_ENABLE_MADC_SHIFT: c_uint = 7;
const ADC3XXX_ENABLE_MADC: c_uint = 1 << ADC3XXX_ENABLE_MADC_SHIFT;
const ADC3XXX_ENABLE_BCLK_SHIFT: c_uint = 7;
const ADC3XXX_ENABLE_BCLK: c_uint = 1 << ADC3XXX_ENABLE_BCLK_SHIFT;
const ADC3XXX_LADC_PWR_ON: c_uint = 0x80;
const ADC3XXX_RADC_PWR_ON: c_uint = 0x40;
const ADC3XXX_SOFT_RESET: c_uint = 0x01;
const ADC3XXX_BCLK_MASTER: c_uint = 0x08;
const ADC3XXX_WCLK_MASTER: c_uint = 0x04;
const ADC3XXX_FORMAT_MASK: c_uint = 0xc0;
const ADC3XXX_FORMAT_SHIFT: c_uint = 6;
const ADC3XXX_WLENGTH_MASK: c_uint = 0x30;
const ADC3XXX_WLENGTH_SHIFT: c_uint = 4;
const ADC3XXX_CLKDIR_MASK: c_uint = 0x0c;
const ADC3XXX_CLKDIR_SHIFT: c_uint = 2;
const ADC3XXX_FORMAT_I2S: c_uint = 0 << ADC3XXX_FORMAT_SHIFT;
const ADC3XXX_FORMAT_DSP: c_uint = 1 << ADC3XXX_FORMAT_SHIFT;
const ADC3XXX_FORMAT_RJF: c_uint = 2 << ADC3XXX_FORMAT_SHIFT;
const ADC3XXX_FORMAT_LJF: c_uint = 3 << ADC3XXX_FORMAT_SHIFT;
const ADC3XXX_IFACE_16BITS: c_uint = 0 << ADC3XXX_WLENGTH_SHIFT;
const ADC3XXX_IFACE_20BITS: c_uint = 1 << ADC3XXX_WLENGTH_SHIFT;
const ADC3XXX_IFACE_24BITS: c_uint = 2 << ADC3XXX_WLENGTH_SHIFT;
const ADC3XXX_IFACE_32BITS: c_uint = 3 << ADC3XXX_WLENGTH_SHIFT;
const ADC3XXX_PLLP_SHIFT: c_uint = 4;
const ADC3XXX_PLLR_SHIFT: c_uint = 0;
const ADC3XXX_PLL_PR_MASK: c_uint = 0x7f;
const ADC3XXX_PLLJ_MASK: c_uint = 0x3f;
const ADC3XXX_PLLD_MSB_MASK: c_uint = 0x3f;
const ADC3XXX_PLLD_LSB_MASK: c_uint = 0xff;
const ADC3XXX_NADC_MASK: c_uint = 0x7f;
const ADC3XXX_MADC_MASK: c_uint = 0x7f;
const ADC3XXX_AOSR_MASK: c_uint = 0xff;
const ADC3XXX_IADC_MASK: c_uint = 0xff;
const ADC3XXX_BDIV_MASK: c_uint = 0x7f;
const ADC3XXX_PLL_CLKIN_SHIFT: c_uint = 2;
const ADC3XXX_PLL_CLKIN_MCLK: c_uint = 0x0;
const ADC3XXX_PLL_CLKIN_BCLK: c_uint = 0x1;
const ADC3XXX_PLL_CLKIN_ZERO: c_uint = 0x3;
const ADC3XXX_CODEC_CLKIN_SHIFT: c_uint = 0;
const ADC3XXX_CODEC_CLKIN_MCLK: c_uint = 0x0;
const ADC3XXX_CODEC_CLKIN_BCLK: c_uint = 0x1;
const ADC3XXX_CODEC_CLKIN_PLL_CLK: c_uint = 0x3;
const ADC3XXX_USE_PLL: c_uint = (ADC3XXX_PLL_CLKIN_MCLK << ADC3XXX_PLL_CLKIN_SHIFT) | (ADC3XXX_CODEC_CLKIN_PLL_CLK << ADC3XXX_CODEC_CLKIN_SHIFT);
const ADC3XXX_NO_PLL: c_uint = (ADC3XXX_PLL_CLKIN_ZERO << ADC3XXX_PLL_CLKIN_SHIFT) | (ADC3XXX_CODEC_CLKIN_MCLK << ADC3XXX_CODEC_CLKIN_SHIFT);
const ADC3XXX_LPGA_MUTE: c_uint = 0x80;
const ADC3XXX_RPGA_MUTE: c_uint = 0x80;
const ADC3XXX_LPGA_GAIN_MASK: c_uint = 0x7f;
const ADC3XXX_RPGA_GAIN_MASK: c_uint = 0x7f;
const ADC3XXX_ADC_LOW_CURR_MODE: c_uint = 0x01;
const ADC3XXX_LCH_SEL1_SHIFT: c_uint = 0;
const ADC3XXX_LCH_SEL2_SHIFT: c_uint = 2;
const ADC3XXX_LCH_SEL3_SHIFT: c_uint = 4;
const ADC3XXX_LCH_SEL4_SHIFT: c_uint = 6;
const ADC3XXX_LCH_SEL1X_SHIFT: c_uint = 0;
const ADC3XXX_LCH_SEL2X_SHIFT: c_uint = 2;
const ADC3XXX_LCH_SEL3X_SHIFT: c_uint = 4;
const ADC3XXX_LCH_COMMON_MODE: c_uint = 0x40;
const ADC3XXX_BYPASS_LPGA: c_uint = 0x80;
const ADC3XXX_RCH_SEL1_SHIFT: c_uint = 0;
const ADC3XXX_RCH_SEL2_SHIFT: c_uint = 2;
const ADC3XXX_RCH_SEL3_SHIFT: c_uint = 4;
const ADC3XXX_RCH_SEL4_SHIFT: c_uint = 6;
const ADC3XXX_RCH_SEL1X_SHIFT: c_uint = 0;
const ADC3XXX_RCH_SEL2X_SHIFT: c_uint = 2;
const ADC3XXX_RCH_SEL3X_SHIFT: c_uint = 4;
const ADC3XXX_RCH_COMMON_MODE: c_uint = 0x40;
const ADC3XXX_BYPASS_RPGA: c_uint = 0x80;
const ADC3XXX_MICBIAS_MASK: c_uint = 0x3;
const ADC3XXX_MICBIAS1_SHIFT: c_uint = 5;
const ADC3XXX_MICBIAS2_SHIFT: c_uint = 3;
const ADC3XXX_ADC_MAX_VOLUME: c_uint = 64;
const ADC3XXX_ADC_POS_VOL: c_uint = 24;
const ADC3XXX_GPIO_CTRL_CFG_MASK: c_uint = 0x3c;
const ADC3XXX_GPIO_CTRL_CFG_SHIFT: c_uint = 2;
const ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_MASK: c_uint = 0x01;
const ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_SHIFT: c_uint = 0;
const ADC3XXX_GPIO_CTRL_INPUT_VALUE_MASK: c_uint = 0x02;
const ADC3XXX_GPIO_CTRL_INPUT_VALUE_SHIFT: c_uint = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum adc3xxx_type {
    ADC3001 = 0,
    ADC3101 = 1,
}

#[repr(C)]
pub struct adc3xxx {
    pub dev: *mut device,
    pub type_: adc3xxx_type,
    pub mclk: *mut clk,
    pub regmap: *mut regmap,
    pub rst_pin: *mut gpio_desc,
    pub pll_mode: c_uint,
    pub sysclk: c_uint,
    pub gpio_cfg: [c_uint; ADC3XXX_GPIO_PINS],
    pub micbias_gpo: [c_uint; ADC3XXX_MICBIAS_PINS],
    pub micbias_vg: [c_uint; ADC3XXX_MICBIAS_PINS],
    pub master: c_int,
    pub page_no: u8,
    pub use_pll: c_int,
    pub gpio_chip: gpio_chip,
}

static adc3xxx_gpio_ctrl_reg: [c_uint; ADC3XXX_GPIO_PINS] = [ADC3XXX_GPIO1_CTRL, ADC3XXX_GPIO2_CTRL];
static adc3xxx_micbias_shift: [c_uint; ADC3XXX_MICBIAS_PINS] = [ADC3XXX_MICBIAS1_SHIFT, ADC3XXX_MICBIAS2_SHIFT];

static adc3xxx_defaults: [reg_default; 256] = [
    reg_default{reg:0,def:0x00},reg_default{reg:1,def:0x00},reg_default{reg:2,def:0x00},reg_default{reg:3,def:0x00},reg_default{reg:4,def:0x00},reg_default{reg:5,def:0x11},reg_default{reg:6,def:0x04},reg_default{reg:7,def:0x00},
    reg_default{reg:8,def:0x00},reg_default{reg:9,def:0x00},reg_default{reg:10,def:0x00},reg_default{reg:11,def:0x00},reg_default{reg:12,def:0x00},reg_default{reg:13,def:0x00},reg_default{reg:14,def:0x00},reg_default{reg:15,def:0x00},
    reg_default{reg:16,def:0x00},reg_default{reg:17,def:0x00},reg_default{reg:18,def:0x01},reg_default{reg:19,def:0x01},reg_default{reg:20,def:0x80},reg_default{reg:21,def:0x80},reg_default{reg:22,def:0x04},reg_default{reg:23,def:0x00},
    reg_default{reg:24,def:0x00},reg_default{reg:25,def:0x00},reg_default{reg:26,def:0x01},reg_default{reg:27,def:0x00},reg_default{reg:28,def:0x00},reg_default{reg:29,def:0x02},reg_default{reg:30,def:0x01},reg_default{reg:31,def:0x00},
    reg_default{reg:32,def:0x00},reg_default{reg:33,def:0x10},reg_default{reg:34,def:0x00},reg_default{reg:35,def:0x00},reg_default{reg:36,def:0x00},reg_default{reg:37,def:0x00},reg_default{reg:38,def:0x02},reg_default{reg:39,def:0x00},
    reg_default{reg:40,def:0x00},reg_default{reg:41,def:0x00},reg_default{reg:42,def:0x00},reg_default{reg:43,def:0x00},reg_default{reg:44,def:0x00},reg_default{reg:45,def:0x00},reg_default{reg:46,def:0x00},reg_default{reg:47,def:0x00},
    reg_default{reg:48,def:0x00},reg_default{reg:49,def:0x00},reg_default{reg:50,def:0x00},reg_default{reg:51,def:0x00},reg_default{reg:52,def:0x00},reg_default{reg:53,def:0x12},reg_default{reg:54,def:0x00},reg_default{reg:55,def:0x00},
    reg_default{reg:56,def:0x00},reg_default{reg:57,def:0x00},reg_default{reg:58,def:0x00},reg_default{reg:59,def:0x44},reg_default{reg:60,def:0x00},reg_default{reg:61,def:0x01},reg_default{reg:62,def:0x00},reg_default{reg:63,def:0x00},
    reg_default{reg:64,def:0x00},reg_default{reg:65,def:0x00},reg_default{reg:66,def:0x00},reg_default{reg:67,def:0x00},reg_default{reg:68,def:0x00},reg_default{reg:69,def:0x00},reg_default{reg:70,def:0x00},reg_default{reg:71,def:0x00},
    reg_default{reg:72,def:0x00},reg_default{reg:73,def:0x00},reg_default{reg:74,def:0x00},reg_default{reg:75,def:0x00},reg_default{reg:76,def:0x00},reg_default{reg:77,def:0x00},reg_default{reg:78,def:0x00},reg_default{reg:79,def:0x00},
    reg_default{reg:80,def:0x00},reg_default{reg:81,def:0x00},reg_default{reg:82,def:0x88},reg_default{reg:83,def:0x00},reg_default{reg:84,def:0x00},reg_default{reg:85,def:0x00},reg_default{reg:86,def:0x00},reg_default{reg:87,def:0x00},
    reg_default{reg:88,def:0x7f},reg_default{reg:89,def:0x00},reg_default{reg:90,def:0x00},reg_default{reg:91,def:0x00},reg_default{reg:92,def:0x00},reg_default{reg:93,def:0x00},reg_default{reg:94,def:0x00},reg_default{reg:95,def:0x00},
    reg_default{reg:96,def:0x7f},reg_default{reg:97,def:0x00},reg_default{reg:98,def:0x00},reg_default{reg:99,def:0x00},reg_default{reg:100,def:0x00},reg_default{reg:101,def:0x00},reg_default{reg:102,def:0x00},reg_default{reg:103,def:0x00},
    reg_default{reg:104,def:0x00},reg_default{reg:105,def:0x00},reg_default{reg:106,def:0x00},reg_default{reg:107,def:0x00},reg_default{reg:108,def:0x00},reg_default{reg:109,def:0x00},reg_default{reg:110,def:0x00},reg_default{reg:111,def:0x00},
    reg_default{reg:112,def:0x00},reg_default{reg:113,def:0x00},reg_default{reg:114,def:0x00},reg_default{reg:115,def:0x00},reg_default{reg:116,def:0x00},reg_default{reg:117,def:0x00},reg_default{reg:118,def:0x00},reg_default{reg:119,def:0x00},
    reg_default{reg:120,def:0x00},reg_default{reg:121,def:0x00},reg_default{reg:122,def:0x00},reg_default{reg:123,def:0x00},reg_default{reg:124,def:0x00},reg_default{reg:125,def:0x00},reg_default{reg:126,def:0x00},reg_default{reg:127,def:0x00},
    reg_default{reg:128,def:0x00},reg_default{reg:129,def:0x00},reg_default{reg:130,def:0x00},reg_default{reg:131,def:0x00},reg_default{reg:132,def:0x00},reg_default{reg:133,def:0x00},reg_default{reg:134,def:0x00},reg_default{reg:135,def:0x00},
    reg_default{reg:136,def:0x00},reg_default{reg:137,def:0x00},reg_default{reg:138,def:0x00},reg_default{reg:139,def:0x00},reg_default{reg:140,def:0x00},reg_default{reg:141,def:0x00},reg_default{reg:142,def:0x00},reg_default{reg:143,def:0x00},
    reg_default{reg:144,def:0x00},reg_default{reg:145,def:0x00},reg_default{reg:146,def:0x00},reg_default{reg:147,def:0x00},reg_default{reg:148,def:0x00},reg_default{reg:149,def:0x00},reg_default{reg:150,def:0x00},reg_default{reg:151,def:0x00},
    reg_default{reg:152,def:0x00},reg_default{reg:153,def:0x00},reg_default{reg:154,def:0x00},reg_default{reg:155,def:0x00},reg_default{reg:156,def:0x00},reg_default{reg:157,def:0x00},reg_default{reg:158,def:0x00},reg_default{reg:159,def:0x00},
    reg_default{reg:160,def:0x00},reg_default{reg:161,def:0x00},reg_default{reg:162,def:0x00},reg_default{reg:163,def:0x00},reg_default{reg:164,def:0x00},reg_default{reg:165,def:0x00},reg_default{reg:166,def:0x00},reg_default{reg:167,def:0x00},
    reg_default{reg:168,def:0x00},reg_default{reg:169,def:0x00},reg_default{reg:170,def:0x00},reg_default{reg:171,def:0x00},reg_default{reg:172,def:0x00},reg_default{reg:173,def:0x00},reg_default{reg:174,def:0x00},reg_default{reg:175,def:0x00},
    reg_default{reg:176,def:0x00},reg_default{reg:177,def:0x00},reg_default{reg:178,def:0x00},reg_default{reg:179,def:0x00},reg_default{reg:180,def:0xff},reg_default{reg:181,def:0x00},reg_default{reg:182,def:0x3f},reg_default{reg:183,def:0xff},
    reg_default{reg:184,def:0x00},reg_default{reg:185,def:0x3f},reg_default{reg:186,def:0x00},reg_default{reg:187,def:0x80},reg_default{reg:188,def:0x80},reg_default{reg:189,def:0x00},reg_default{reg:190,def:0x00},reg_default{reg:191,def:0x00},
    reg_default{reg:1024,def:0x00},reg_default{reg:1026,def:0x01},reg_default{reg:1027,def:0x17},reg_default{reg:1028,def:0x01},reg_default{reg:1029,def:0x17},reg_default{reg:1030,def:0x7d},reg_default{reg:1031,def:0xd3},reg_default{reg:1032,def:0x7f},
    reg_default{reg:1033,def:0xff},reg_default{reg:1034,def:0x00},reg_default{reg:1035,def:0x00},reg_default{reg:1036,def:0x00},reg_default{reg:1037,def:0x00},reg_default{reg:1038,def:0x7f},reg_default{reg:1039,def:0xff},reg_default{reg:1040,def:0x00},
    reg_default{reg:1041,def:0x00},reg_default{reg:1042,def:0x00},reg_default{reg:1043,def:0x00},reg_default{reg:1044,def:0x00},reg_default{reg:1045,def:0x00},reg_default{reg:1046,def:0x00},reg_default{reg:1047,def:0x00},reg_default{reg:1048,def:0x7f},
    reg_default{reg:1049,def:0xff},reg_default{reg:1050,def:0x00},reg_default{reg:1051,def:0x00},reg_default{reg:1052,def:0x00},reg_default{reg:1053,def:0x00},reg_default{reg:1054,def:0x00},reg_default{reg:1055,def:0x00},reg_default{reg:1056,def:0x00},
    reg_default{reg:1057,def:0x00},reg_default{reg:1058,def:0x7f},reg_default{reg:1059,def:0xff},reg_default{reg:1060,def:0x00},reg_default{reg:1061,def:0x00},reg_default{reg:1062,def:0x00},reg_default{reg:1063,def:0x00},reg_default{reg:1064,def:0x00},
    reg_default{reg:1065,def:0x00},reg_default{reg:1066,def:0x00},reg_default{reg:1067,def:0x00},reg_default{reg:1068,def:0x7f},reg_default{reg:1069,def:0xff},reg_default{reg:1070,def:0x00},reg_default{reg:1071,def:0x00},reg_default{reg:1072,def:0x00},
    reg_default{reg:1073,def:0x00},reg_default{reg:1074,def:0x00},reg_default{reg:1075,def:0x00},reg_default{reg:1076,def:0x00},reg_default{reg:1077,def:0x00},reg_default{reg:1078,def:0x7f},reg_default{reg:1079,def:0xff},reg_default{reg:1080,def:0x00},
    reg_default{reg:1081,def:0x00},reg_default{reg:1082,def:0x00},reg_default{reg:1083,def:0x00},reg_default{reg:1084,def:0x00},reg_default{reg:1085,def:0x00},reg_default{reg:1086,def:0x00},reg_default{reg:1087,def:0x00},reg_default{reg:1088,def:0x00},
    reg_default{reg:1089,def:0x00},reg_default{reg:1090,def:0x00},reg_default{reg:1091,def:0x00},reg_default{reg:1092,def:0x00},reg_default{reg:1093,def:0x00},reg_default{reg:1094,def:0x00},reg_default{reg:1095,def:0x00},reg_default{reg:1096,def:0x00},
    reg_default{reg:1097,def:0x00},reg_default{reg:1098,def:0x00},reg_default{reg:1099,def:0x00},reg_default{reg:1100,def:0x00},reg_default{reg:1101,def:0x00},reg_default{reg:1102,def:0x00},reg_default{reg:1103,def:0x00},reg_default{reg:1104,def:0x00},
    reg_default{reg:1105,def:0x00},reg_default{reg:1106,def:0x00},reg_default{reg:1107,def:0x00},reg_default{reg:1108,def:0x00},reg_default{reg:1109,def:0x00},reg_default{reg:1110,def:0x00},reg_default{reg:1111,def:0x00},reg_default{reg:1112,def:0x00},
    reg_default{reg:1113,def:0x00},reg_default{reg:1114,def:0x00},reg_default{reg:1115,def:0x00},reg_default{reg:1116,def:0x00},reg_default{reg:1117,def:0x00},reg_default{reg:1118,def:0x00},reg_default{reg:1119,def:0x00},reg_default{reg:1120,def:0x00},
    reg_default{reg:1121,def:0x00},reg_default{reg:1122,def:0x00},reg_default{reg:1123,def:0x00},reg_default{reg:1124,def:0x00},reg_default{reg:1125,def:0x00},reg_default{reg:1126,def:0x00},reg_default{reg:1127,def:0x00},reg_default{reg:1128,def:0x00},
    reg_default{reg:1129,def:0x00},reg_default{reg:1130,def:0x00},reg_default{reg:1131,def:0x00},reg_default{reg:1132,def:0x00},reg_default{reg:1133,def:0x00},reg_default{reg:1134,def:0x00},reg_default{reg:1135,def:0x00},reg_default{reg:1136,def:0x00},
    reg_default{reg:1137,def:0x00},reg_default{reg:1138,def:0x00},reg_default{reg:1139,def:0x00},reg_default{reg:1140,def:0x00},reg_default{reg:1141,def:0x00},reg_default{reg:1142,def:0x00},reg_default{reg:1143,def:0x00},reg_default{reg:1144,def:0x00},
    reg_default{reg:1145,def:0x00},reg_default{reg:1146,def:0x00},reg_default{reg:1147,def:0x00},reg_default{reg:1148,def:0x00},reg_default{reg:1149,def:0x00},reg_default{reg:1150,def:0x00},reg_default{reg:1151,def:0x00},
];

unsafe extern "C" fn adc3xxx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        ADC3XXX_RESET => true,
        _ => false,
    }
}

static adc3xxx_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 5 * ADC3XXX_PAGE_SIZE,
    selector_reg: ADC3XXX_PAGE_SELECT,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: ADC3XXX_PAGE_SIZE,
}];

static adc3xxx_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    reg_defaults: adc3xxx_defaults.as_ptr(),
    num_reg_defaults: adc3xxx_defaults.len() as c_uint,
    volatile_reg: Some(adc3xxx_volatile_reg),
    cache_type: REGCACHE_RBTREE,
    ranges: adc3xxx_ranges.as_ptr(),
    num_ranges: adc3xxx_ranges.len() as c_uint,
    max_register: 5 * ADC3XXX_PAGE_SIZE,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc3xxx_rate_divs {
    pub mclk: u32,
    pub rate: u32,
    pub pll_p: u8,
    pub pll_r: u8,
    pub pll_j: u8,
    pub pll_d: u16,
    pub nadc: u8,
    pub madc: u8,
    pub aosr: u8,
}

static adc3xxx_divs: [adc3xxx_rate_divs; 16] = [
    adc3xxx_rate_divs{mclk:12000000,rate:8000,pll_p:1,pll_r:1,pll_j:7,pll_d:1680,nadc:42,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12288000,rate:8000,pll_p:1,pll_r:1,pll_j:7,pll_d:0000,nadc:42,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:11025,pll_p:1,pll_r:1,pll_j:6,pll_d:8208,nadc:29,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:16000,pll_p:1,pll_r:1,pll_j:7,pll_d:1680,nadc:21,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12288000,rate:16000,pll_p:1,pll_r:1,pll_j:7,pll_d:0000,nadc:21,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:22050,pll_p:1,pll_r:1,pll_j:7,pll_d:560,nadc:15,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:32000,pll_p:1,pll_r:1,pll_j:8,pll_d:1920,nadc:12,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12288000,rate:32000,pll_p:1,pll_r:1,pll_j:8,pll_d:0000,nadc:12,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:44100,pll_p:1,pll_r:1,pll_j:7,pll_d:5264,nadc:8,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:48000,pll_p:1,pll_r:1,pll_j:7,pll_d:1680,nadc:7,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12288000,rate:48000,pll_p:1,pll_r:1,pll_j:7,pll_d:0000,nadc:7,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:24576000,rate:48000,pll_p:1,pll_r:1,pll_j:3,pll_d:5000,nadc:7,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:24576000,rate:48000,pll_p:0,pll_r:0,pll_j:0,pll_d:0000,nadc:2,madc:2,aosr:128},
    adc3xxx_rate_divs{mclk:12000000,rate:88200,pll_p:1,pll_r:1,pll_j:7,pll_d:5264,nadc:4,madc:4,aosr:64},
    adc3xxx_rate_divs{mclk:12000000,rate:96000,pll_p:1,pll_r:1,pll_j:8,pll_d:1920,nadc:4,madc:4,aosr:64},
    adc3xxx_rate_divs{mclk:0,rate:0,pll_p:0,pll_r:0,pll_j:0,pll_d:0,nadc:0,madc:0,aosr:0},
];

unsafe extern "C" fn adc3xxx_get_divs(dev: *mut device, mclk: c_int, rate: c_int, pll_mode: c_int) -> c_int {
    dev_dbg(dev, c"mclk = %d, rate = %d, clock mode %u\n".as_ptr(), mclk, rate, pll_mode as c_uint);
    for i in 0..adc3xxx_divs.len() {
        let mode = &adc3xxx_divs[i];
        if (pll_mode as c_uint == ADC3XXX_PLL_BYPASS && mode.pll_p != 0) ||
           (pll_mode as c_uint == ADC3XXX_PLL_ENABLE && mode.pll_p == 0) {
            continue;
        }
        if mode.rate == rate as u32 && mode.mclk == mclk as u32 {
            return i as c_int;
        }
    }
    dev_info(dev, c"Master clock rate %d and sample rate %d is not supported\n".as_ptr(), mclk, rate);
    -EINVAL
}

unsafe extern "C" fn adc3xxx_pll_delay(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    usleep_range(10000, 20000);
    0
}

unsafe extern "C" fn adc3xxx_coefficient_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let numcoeff = ((*kcontrol).private_value >> 16) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = numcoeff as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 0xffff;
    0
}

unsafe extern "C" fn adc3xxx_coefficient_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let numcoeff = ((*kcontrol).private_value >> 16) as c_int;
    let mut reg = ((*kcontrol).private_value & 0xffff) as c_int;
    for index in 0..numcoeff {
        let value_msb = snd_soc_component_read(component, reg as c_uint);
        reg += 1;
        if (value_msb as c_int) < 0 { return value_msb as c_int; }
        let value_lsb = snd_soc_component_read(component, reg as c_uint);
        reg += 1;
        if (value_lsb as c_int) < 0 { return value_lsb as c_int; }
        let value = (value_msb << 8) | value_lsb;
        (*ucontrol).value.integer.value[index as usize] = value as i64;
    }
    0
}

unsafe extern "C" fn adc3xxx_coefficient_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let numcoeff = ((*kcontrol).private_value >> 16) as c_int;
    let mut reg = ((*kcontrol).private_value & 0xffff) as c_int;
    for index in 0..numcoeff {
        let value = (*ucontrol).value.integer.value[index as usize] as c_uint;
        let value_msb = (value >> 8) & 0xff;
        let value_lsb = value & 0xff;
        let mut ret = snd_soc_component_write(component, reg as c_uint, value_msb);
        reg += 1;
        if ret != 0 { return ret; }
        ret = snd_soc_component_write(component, reg as c_uint, value_lsb);
        reg += 1;
        if ret != 0 { return ret; }
    }
    0
}

/* TI_COEFFICIENTS(xname, reg, numcoeffs) expands to a snd_kcontrol_new with
 * adc3xxx_coefficient_info/get/put and private_value = reg | (numcoeffs << 16).
 * SOC_*, SND_SOC_DAPM_*, DECLARE_TLV_* and MODULE_* macro-generated objects
 * are represented below as declarations/comments because their concrete Rust
 * layout is supplied by external ASoC bindings, not by this file.
 */
static adc_softstepping_text: [*const c_char; 3] = [c"1 step".as_ptr(), c"2 step".as_ptr(), c"off".as_ptr()];
static multiplier_text: [*const c_char; 8] = [c"1".as_ptr(), c"2".as_ptr(), c"4".as_ptr(), c"8".as_ptr(), c"16".as_ptr(), c"32".as_ptr(), c"64".as_ptr(), c"128".as_ptr()];
static dither_dc_offset_text: [*const c_char; 15] = [c"0mV".as_ptr(),c"15mV".as_ptr(),c"30mV".as_ptr(),c"45mV".as_ptr(),c"60mV".as_ptr(),c"75mV".as_ptr(),c"90mV".as_ptr(),c"105mV".as_ptr(),c"-15mV".as_ptr(),c"-30mV".as_ptr(),c"-45mV".as_ptr(),c"-60mV".as_ptr(),c"-75mV".as_ptr(),c"-90mV".as_ptr(),c"-105mV".as_ptr()];
static dither_dc_offset_values: [c_uint; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15];

static adc3xxx_snd_controls: [snd_kcontrol_new; 0] = [];
static left_input_mixer_controls: [snd_kcontrol_new; 0] = [];
static right_input_mixer_controls: [snd_kcontrol_new; 0] = [];
static left_input_dmic_controls: [snd_kcontrol_new; 0] = [];
static right_input_dmic_controls: [snd_kcontrol_new; 0] = [];
static adc3xxx_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static adc3xxx_intercon: [snd_soc_dapm_route; 38] = [
    route(c"Left Input", c"IN_1L Capture Switch", c"IN_1L"), route(c"Left Input", c"IN_2L Capture Switch", c"IN_2L"), route(c"Left Input", c"IN_3L Capture Switch", c"IN_3L"), route(c"Left Input", c"DIF_2L_3L Capture Switch", c"DIFL_2L_3L"),
    route(c"Left Input", c"DIF_1L_1R Capture Switch", c"DIFL_1L_1R"), route(c"Left Input", c"DIF_2R_3R Capture Switch", c"DIFL_2R_3R"), route(c"Left Input", c"IN_1R Capture Switch", c"IN_1R"), route(c"Left PGA", nullc(), c"Left Input"),
    route(c"Left ADC", nullc(), c"Left PGA"), route(c"Right Input", c"IN_1R Capture Switch", c"IN_1R"), route(c"Right Input", c"IN_2R Capture Switch", c"IN_2R"), route(c"Right Input", c"IN_3R Capture Switch", c"IN_3R"),
    route(c"Right Input", c"DIF_2R_3R Capture Switch", c"DIFR_2R_3R"), route(c"Right Input", c"DIF_1L_1R Capture Switch", c"DIFR_1L_1R"), route(c"Right Input", c"DIF_2L_3L Capture Switch", c"DIFR_2L_3L"), route(c"Right Input", c"IN_1L Capture Switch", c"IN_1L"),
    route(c"Right PGA", nullc(), c"Right Input"), route(c"Right ADC", nullc(), c"Right PGA"), route(c"Left DMic Input", c"Left ADC Capture Switch", c"DMic_L"), route(c"Left ADC", nullc(), c"Left DMic Input"),
    route(c"Right DMic Input", c"Right ADC Capture Switch", c"DMic_R"), route(c"Right ADC", nullc(), c"Right DMic Input"), route(c"AIF_OUT", nullc(), c"Left ADC"), route(c"AIF_OUT", nullc(), c"Right ADC"),
    route(c"ADC_MOD_CLK", nullc(), c"ADC_CLK"), route(c"Left ADC", nullc(), c"ADC_MOD_CLK"), route(c"Right ADC", nullc(), c"ADC_MOD_CLK"), route(c"BCLK", nullc(), c"ADC_CLK"),
    route(c"IN_1L", nullc(), nullc()), route(c"IN_1R", nullc(), nullc()), route(c"IN_2L", nullc(), nullc()), route(c"IN_2R", nullc(), nullc()),
    route(c"IN_3L", nullc(), nullc()), route(c"IN_3R", nullc(), nullc()), route(c"DMic_L", nullc(), nullc()), route(c"DMic_R", nullc(), nullc()),
    route(c"AIF_OUT", c"Capture", nullc()), route(c"PLL_CLK", nullc(), nullc()),
];
static adc3xxx_pll_intercon: [snd_soc_dapm_route; 1] = [route(c"ADC_CLK", nullc(), c"PLL_CLK")];
static adc3xxx_bclk_out_intercon: [snd_soc_dapm_route; 1] = [route(c"AIF_OUT", nullc(), c"BCLK")];

const fn nullc() -> *const c_char { core::ptr::null() }
const fn route(sink: &'static core::ffi::CStr, control: *const c_char, source: &'static core::ffi::CStr) -> snd_soc_dapm_route {
    snd_soc_dapm_route { sink: sink.as_ptr(), control, source: source.as_ptr() }
}

unsafe extern "C" fn adc3xxx_gpio_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let adc3xxx = gpiochip_get_data(chip) as *mut adc3xxx;
    if offset >= ADC3XXX_GPIOS_MAX as c_uint { return -EINVAL; }
    if offset < ADC3XXX_GPIO_PINS as c_uint {
        if (*adc3xxx).gpio_cfg[offset as usize] != 0 &&
           (*adc3xxx).gpio_cfg[offset as usize] != ADC3XXX_GPIO_GPO + 1 {
            return -EINVAL;
        }
    } else if offset >= ADC3XXX_GPIO_PINS as c_uint && offset < ADC3XXX_GPIOS_MAX as c_uint {
        if (*adc3xxx).micbias_gpo[(offset - ADC3XXX_GPIO_PINS as c_uint) as usize] == 0 {
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn adc3xxx_gpio_direction_out(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let adc3xxx = gpiochip_get_data(chip) as *mut adc3xxx;
    if offset >= ADC3XXX_GPIO_PINS as c_uint {
        let micbias = (offset - ADC3XXX_GPIO_PINS as c_uint) as usize;
        let vg = if value != 0 { (*adc3xxx).micbias_vg[micbias] } else { ADC3XXX_MICBIAS_OFF };
        return regmap_update_bits((*adc3xxx).regmap, ADC3XXX_MICBIAS_CTRL,
            ADC3XXX_MICBIAS_MASK << adc3xxx_micbias_shift[micbias],
            vg << adc3xxx_micbias_shift[micbias]);
    }
    regmap_update_bits((*adc3xxx).regmap, adc3xxx_gpio_ctrl_reg[offset as usize],
        ADC3XXX_GPIO_CTRL_CFG_MASK | ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_MASK,
        (ADC3XXX_GPIO_GPO << ADC3XXX_GPIO_CTRL_CFG_SHIFT) |
        (((value != 0) as c_uint) << ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_SHIFT))
}

unsafe extern "C" fn adc3xxx_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    adc3xxx_gpio_direction_out(chip, offset, value)
}

unsafe extern "C" fn adc3xxx_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let adc3xxx = gpiochip_get_data(chip) as *mut adc3xxx;
    let mut regval: c_uint = 0;
    let ret: c_int;
    if offset >= ADC3XXX_GPIO_PINS as c_uint {
        let micbias = (offset - ADC3XXX_GPIO_PINS as c_uint) as usize;
        ret = regmap_read((*adc3xxx).regmap, ADC3XXX_MICBIAS_CTRL, &mut regval);
        if ret != 0 { return ret; }
        return ((((regval >> adc3xxx_micbias_shift[micbias]) & ADC3XXX_MICBIAS_MASK) != ADC3XXX_MICBIAS_OFF) as c_int);
    }
    ret = regmap_read((*adc3xxx).regmap, adc3xxx_gpio_ctrl_reg[offset as usize], &mut regval);
    if ret != 0 { return ret; }
    ((regval & ADC3XXX_GPIO_CTRL_OUTPUT_CTRL_MASK) != 0) as c_int
}

static adc3xxx_gpio_chip: gpio_chip = gpio_chip {
    label: c"adc3xxx".as_ptr(),
    owner: core::ptr::null_mut(),
    request: Some(adc3xxx_gpio_request),
    direction_output: Some(adc3xxx_gpio_direction_out),
    set: Some(adc3xxx_gpio_set),
    get: Some(adc3xxx_gpio_get),
    can_sleep: 1,
    ngpio: 0,
    parent: core::ptr::null_mut(),
    base: 0,
};

unsafe extern "C" fn adc3xxx_free_gpio(adc3xxx: *mut adc3xxx) {
    /* CONFIG_GPIOLIB: gpiochip_remove is called when gpiolib is enabled. */
    gpiochip_remove(&mut (*adc3xxx).gpio_chip);
}

unsafe extern "C" fn adc3xxx_init_gpio(adc3xxx: *mut adc3xxx) {
    (*adc3xxx).gpio_chip = adc3xxx_gpio_chip;
    (*adc3xxx).gpio_chip.ngpio = ADC3XXX_GPIOS_MAX as c_uint;
    (*adc3xxx).gpio_chip.parent = (*adc3xxx).dev;
    (*adc3xxx).gpio_chip.base = -1;
    let ret = gpiochip_add_data(&mut (*adc3xxx).gpio_chip, adc3xxx as *mut c_void);
    if ret != 0 {
        dev_err((*adc3xxx).dev, c"Failed to add gpios: %d\n".as_ptr(), ret);
    }
    for gpio in 0..ADC3XXX_GPIO_PINS {
        let mut cfg = (*adc3xxx).gpio_cfg[gpio];
        if cfg != 0 {
            cfg -= 1;
            regmap_update_bits((*adc3xxx).regmap, adc3xxx_gpio_ctrl_reg[gpio],
                ADC3XXX_GPIO_CTRL_CFG_MASK, cfg << ADC3XXX_GPIO_CTRL_CFG_SHIFT);
        }
    }
    for micbias in 0..ADC3XXX_MICBIAS_PINS {
        let vg = if (*adc3xxx).micbias_gpo[micbias] != 0 {
            ADC3XXX_MICBIAS_OFF
        } else {
            (*adc3xxx).micbias_vg[micbias]
        };
        regmap_update_bits((*adc3xxx).regmap, ADC3XXX_MICBIAS_CTRL,
            ADC3XXX_MICBIAS_MASK << adc3xxx_micbias_shift[micbias],
            vg << adc3xxx_micbias_shift[micbias]);
    }
}

unsafe extern "C" fn adc3xxx_parse_dt_gpio(adc3xxx: *mut adc3xxx, propname: *const c_char, cfg: *mut c_uint) -> c_int {
    let dev = (*adc3xxx).dev;
    let np = (*(dev as *mut device_with_node)).of_node;
    let mut val: c_uint = 0;
    if of_property_read_u32(np, propname, &mut val) == 0 {
        if (val & !15) != 0 || val == 7 || val >= 11 {
            dev_err(dev, c"Invalid property value for '%s'\n".as_ptr(), propname);
            return -EINVAL;
        }
        if val == ADC3XXX_GPIO_GPI {
            dev_warn(dev, c"GPIO Input read not yet implemented\n".as_ptr());
        }
        *cfg = val + 1;
    }
    0
}

#[repr(C)]
struct device_with_node { of_node: *mut device_node }

unsafe extern "C" fn adc3xxx_parse_dt_micbias_gpo(adc3xxx: *mut adc3xxx, propname: *const c_char, cfg: *mut c_uint) -> c_int {
    let dev = (*adc3xxx).dev;
    let np = (*(dev as *mut device_with_node)).of_node;
    *cfg = of_property_read_bool(np, propname) as c_uint;
    0
}

unsafe extern "C" fn adc3xxx_parse_dt_micbias_vg(adc3xxx: *mut adc3xxx, propname: *const c_char, vg: *mut c_uint) -> c_int {
    let dev = (*adc3xxx).dev;
    let np = (*(dev as *mut device_with_node)).of_node;
    let mut val: c_uint = 0;
    if of_property_read_u32(np, propname, &mut val) == 0 {
        if val > ADC3XXX_MICBIAS_AVDD {
            dev_err(dev, c"Invalid property value for '%s'\n".as_ptr(), propname);
            return -EINVAL;
        }
        *vg = val;
    }
    0
}

unsafe extern "C" fn adc3xxx_parse_pll_mode(val: uint32_t, pll_mode: *mut c_uint) -> c_int {
    if val != ADC3XXX_PLL_ENABLE && val != ADC3XXX_PLL_BYPASS && val != ADC3XXX_PLL_AUTO {
        return -EINVAL;
    }
    *pll_mode = val;
    0
}

unsafe extern "C" fn adc3xxx_setup_pll(component: *mut snd_soc_component, div_entry: c_int) {
    let i = div_entry as usize;
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_PR,
        ((adc3xxx_divs[i].pll_p as c_uint) << ADC3XXX_PLLP_SHIFT) |
        ((adc3xxx_divs[i].pll_r as c_uint) << ADC3XXX_PLLR_SHIFT));
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_J,
        (adc3xxx_divs[i].pll_j as c_uint) & ADC3XXX_PLLJ_MASK);
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_D_LSB,
        (adc3xxx_divs[i].pll_d as c_uint) & ADC3XXX_PLLD_LSB_MASK);
    snd_soc_component_write(component, ADC3XXX_PLL_PROG_D_MSB,
        ((adc3xxx_divs[i].pll_d as c_uint) >> 8) & ADC3XXX_PLLD_MSB_MASK);
}

unsafe extern "C" fn adc3xxx_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let dapm = snd_soc_component_to_dapm((*dai).component);
    let adc3xxx = snd_soc_component_get_drvdata(component) as *mut adc3xxx;
    let mut width: c_int = 16;
    let iface_len: c_uint;
    let i = adc3xxx_get_divs((*component).dev, (*adc3xxx).sysclk as c_int, params_rate(params), (*adc3xxx).pll_mode as c_int);
    if i < 0 { return i; }
    match params_width(params) {
        16 => { iface_len = ADC3XXX_IFACE_16BITS; width = 16; }
        20 => { iface_len = ADC3XXX_IFACE_20BITS; width = 20; }
        24 => { iface_len = ADC3XXX_IFACE_24BITS; width = 24; }
        32 => { iface_len = ADC3XXX_IFACE_32BITS; width = 32; }
        _ => {
            dev_err((*component).dev, c"Unsupported serial data format\n".as_ptr());
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, ADC3XXX_INTERFACE_CTRL_1, ADC3XXX_WLENGTH_MASK, iface_len);
    if adc3xxx_divs[i as usize].pll_p != 0 {
        adc3xxx_setup_pll(component, i);
        snd_soc_component_write(component, ADC3XXX_CLKGEN_MUX, ADC3XXX_USE_PLL);
        if (*adc3xxx).use_pll == 0 {
            snd_soc_dapm_add_routes(dapm, adc3xxx_pll_intercon.as_ptr(), adc3xxx_pll_intercon.len() as c_int);
            (*adc3xxx).use_pll = 1;
        }
    } else {
        snd_soc_component_write(component, ADC3XXX_CLKGEN_MUX, ADC3XXX_NO_PLL);
        if (*adc3xxx).use_pll != 0 {
            snd_soc_dapm_del_routes(dapm, adc3xxx_pll_intercon.as_ptr(), adc3xxx_pll_intercon.len() as c_int);
            (*adc3xxx).use_pll = 0;
        }
    }
    snd_soc_component_update_bits(component, ADC3XXX_ADC_NADC, ADC3XXX_NADC_MASK, adc3xxx_divs[i as usize].nadc as c_uint);
    snd_soc_component_update_bits(component, ADC3XXX_ADC_MADC, ADC3XXX_MADC_MASK, adc3xxx_divs[i as usize].madc as c_uint);
    snd_soc_component_update_bits(component, ADC3XXX_ADC_AOSR, ADC3XXX_AOSR_MASK, adc3xxx_divs[i as usize].aosr as c_uint);
    let bdiv = ((adc3xxx_divs[i as usize].aosr as c_int * adc3xxx_divs[i as usize].madc as c_int) / (2 * width)) as c_uint;
    snd_soc_component_update_bits(component, ADC3XXX_BCLK_N_DIV, ADC3XXX_BDIV_MASK, bdiv);
    0
}

unsafe extern "C" fn adc3xxx_pll_mode_text(pll_mode: c_int) -> *const c_char {
    match pll_mode as c_uint {
        ADC3XXX_PLL_AUTO => c"PLL auto".as_ptr(),
        ADC3XXX_PLL_ENABLE => c"PLL enable".as_ptr(),
        ADC3XXX_PLL_BYPASS => c"PLL bypass".as_ptr(),
        _ => c"PLL unknown".as_ptr(),
    }
}

unsafe extern "C" fn adc3xxx_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let adc3xxx = snd_soc_component_get_drvdata(component) as *mut adc3xxx;
    let ret = adc3xxx_parse_pll_mode(clk_id as uint32_t, &mut (*adc3xxx).pll_mode);
    if ret < 0 { return ret; }
    (*adc3xxx).sysclk = freq;
    dev_dbg((*component).dev, c"Set sysclk to %u Hz, %s\n".as_ptr(), freq, adc3xxx_pll_mode_text((*adc3xxx).pll_mode as c_int));
    0
}

unsafe extern "C" fn adc3xxx_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let dapm = snd_soc_component_to_dapm(component);
    let adc3xxx = snd_soc_component_get_drvdata(component) as *mut adc3xxx;
    let mut clkdir: c_uint = 0;
    let format: c_uint;
    let master: c_int;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => { master = 1; clkdir = ADC3XXX_BCLK_MASTER | ADC3XXX_WCLK_MASTER; }
        SND_SOC_DAIFMT_CBC_CFC => { master = 0; }
        _ => {
            dev_err((*component).dev, c"Invalid DAI clock setup\n".as_ptr());
            return -EINVAL;
        }
    }
    match fmt & (SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK) {
        x if x == (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF) => format = ADC3XXX_FORMAT_I2S,
        x if x == (SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF) => format = ADC3XXX_FORMAT_DSP,
        x if x == (SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF) => format = ADC3XXX_FORMAT_DSP,
        x if x == (SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_NB_NF) => format = ADC3XXX_FORMAT_RJF,
        x if x == (SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF) => format = ADC3XXX_FORMAT_LJF,
        _ => {
            dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            return -EINVAL;
        }
    }
    if master != 0 && (*adc3xxx).master == 0 {
        snd_soc_dapm_add_routes(dapm, adc3xxx_bclk_out_intercon.as_ptr(), adc3xxx_bclk_out_intercon.len() as c_int);
    } else if master == 0 && (*adc3xxx).master != 0 {
        snd_soc_dapm_del_routes(dapm, adc3xxx_bclk_out_intercon.as_ptr(), adc3xxx_bclk_out_intercon.len() as c_int);
    }
    (*adc3xxx).master = master;
    let ret = snd_soc_component_update_bits(component, ADC3XXX_INTERFACE_CTRL_1,
        ADC3XXX_CLKDIR_MASK | ADC3XXX_FORMAT_MASK, clkdir | format);
    if ret < 0 { return ret; }
    0
}

static adc3xxx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(adc3xxx_hw_params),
    set_sysclk: Some(adc3xxx_set_dai_sysclk),
    set_fmt: Some(adc3xxx_set_dai_fmt),
};

static mut adc3xxx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"tlv320adc3xxx-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: ADC3XXX_RATES,
        formats: ADC3XXX_FORMATS,
    },
    ops: &adc3xxx_dai_ops,
};

static soc_component_dev_adc3xxx: snd_soc_component_driver = snd_soc_component_driver {
    controls: adc3xxx_snd_controls.as_ptr(),
    num_controls: adc3xxx_snd_controls.len() as c_uint,
    dapm_widgets: adc3xxx_dapm_widgets.as_ptr(),
    num_dapm_widgets: adc3xxx_dapm_widgets.len() as c_uint,
    dapm_routes: adc3xxx_intercon.as_ptr(),
    num_dapm_routes: adc3xxx_intercon.len() as c_uint,
    endianness: 1,
};

static adc3xxx_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: str32("tlv320adc3001"), driver_data: adc3xxx_type::ADC3001 as c_ulong },
    i2c_device_id { name: str32("tlv320adc3101"), driver_data: adc3xxx_type::ADC3101 as c_ulong },
    i2c_device_id { name: [0; 32], driver_data: 0 },
];

const fn str32(s: &str) -> [c_char; 32] {
    let bytes = s.as_bytes();
    let mut out = [0 as c_char; 32];
    let mut i = 0;
    while i < bytes.len() && i < 31 {
        out[i] = bytes[i] as c_char;
        i += 1;
    }
    out
}

unsafe extern "C" fn adc3xxx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let adc3xxx = devm_kzalloc(dev, core::mem::size_of::<adc3xxx>(), GFP_KERNEL) as *mut adc3xxx;
    if adc3xxx.is_null() { return -ENOMEM; }
    (*adc3xxx).dev = dev;
    (*adc3xxx).rst_pin = devm_gpiod_get(dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*adc3xxx).rst_pin as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*adc3xxx).rst_pin as *const c_void), c"Failed to request rst_pin\n".as_ptr());
    }
    (*adc3xxx).mclk = devm_clk_get(dev, core::ptr::null());
    if IS_ERR((*adc3xxx).mclk as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*adc3xxx).mclk as *const c_void), c"Failed to acquire MCLK\n".as_ptr());
    } else if !(*adc3xxx).mclk.is_null() {
        let ret = clk_prepare_enable((*adc3xxx).mclk);
        if ret < 0 { return ret; }
        dev_dbg(dev, c"Enabled MCLK, freq %lu Hz\n".as_ptr(), clk_get_rate((*adc3xxx).mclk));
    }
    let mut ret = adc3xxx_parse_dt_gpio(adc3xxx, c"ti,dmdin-gpio1".as_ptr(), &mut (*adc3xxx).gpio_cfg[0]);
    if ret < 0 { clk_disable_unprepare((*adc3xxx).mclk); return ret; }
    ret = adc3xxx_parse_dt_gpio(adc3xxx, c"ti,dmclk-gpio2".as_ptr(), &mut (*adc3xxx).gpio_cfg[1]);
    if ret < 0 { clk_disable_unprepare((*adc3xxx).mclk); return ret; }
    ret = adc3xxx_parse_dt_micbias_gpo(adc3xxx, c"ti,micbias1-gpo".as_ptr(), &mut (*adc3xxx).micbias_gpo[0]);
    if ret < 0 { clk_disable_unprepare((*adc3xxx).mclk); return ret; }
    ret = adc3xxx_parse_dt_micbias_gpo(adc3xxx, c"ti,micbias2-gpo".as_ptr(), &mut (*adc3xxx).micbias_gpo[1]);
    if ret < 0 { clk_disable_unprepare((*adc3xxx).mclk); return ret; }
    ret = adc3xxx_parse_dt_micbias_vg(adc3xxx, c"ti,micbias1-vg".as_ptr(), &mut (*adc3xxx).micbias_vg[0]);
    if ret < 0 { clk_disable_unprepare((*adc3xxx).mclk); return ret; }
    ret = adc3xxx_parse_dt_micbias_vg(adc3xxx, c"ti,micbias2-vg".as_ptr(), &mut (*adc3xxx).micbias_vg[1]);
    if ret < 0 { clk_disable_unprepare((*adc3xxx).mclk); return ret; }
    (*adc3xxx).regmap = devm_regmap_init_i2c(i2c, &adc3xxx_regmap);
    if IS_ERR((*adc3xxx).regmap as *const c_void) {
        ret = PTR_ERR((*adc3xxx).regmap as *const c_void);
        clk_disable_unprepare((*adc3xxx).mclk);
        return ret;
    }
    i2c_set_clientdata(i2c, adc3xxx as *mut c_void);
    (*adc3xxx).type_ = core::mem::transmute::<usize, adc3xxx_type>(i2c_get_match_data(i2c) as usize);
    gpiod_set_value_cansleep((*adc3xxx).rst_pin, 1);
    usleep_range(2000, 100000);
    gpiod_set_value_cansleep((*adc3xxx).rst_pin, 0);
    adc3xxx_init_gpio(adc3xxx);
    ret = snd_soc_register_component(dev, &soc_component_dev_adc3xxx, &mut adc3xxx_dai, 1);
    if ret < 0 {
        dev_err(dev, c"Failed to register codec: %d\n".as_ptr(), ret);
        clk_disable_unprepare((*adc3xxx).mclk);
        return ret;
    }
    0
}

unsafe extern "C" fn adc3xxx_i2c_remove(client: *mut i2c_client) {
    let adc3xxx = i2c_get_clientdata(client) as *mut adc3xxx;
    clk_disable_unprepare((*adc3xxx).mclk);
    adc3xxx_free_gpio(adc3xxx);
    snd_soc_unregister_component(&mut (*client).dev);
}

static tlv320adc3xxx_of_match: [of_device_id; 3] = [
    of_device_id { compatible: c"ti,tlv320adc3001".as_ptr() },
    of_device_id { compatible: c"ti,tlv320adc3101".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut adc3xxx_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tlv320adc3xxx-codec".as_ptr(),
        of_match_table: tlv320adc3xxx_of_match.as_ptr(),
    },
    probe: Some(adc3xxx_i2c_probe),
    remove: Some(adc3xxx_i2c_remove),
    id_table: adc3xxx_i2c_id.as_ptr(),
};

/* module_i2c_driver(adc3xxx_i2c_driver);
 * MODULE_DEVICE_TABLE(i2c, adc3xxx_i2c_id);
 * MODULE_DEVICE_TABLE(of, tlv320adc3xxx_of_match);
 * MODULE_DESCRIPTION("ASoC TLV320ADC3xxx codec driver");
 * MODULE_AUTHOR("shahina.s@mistralsolutions.com");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
