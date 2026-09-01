// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Vista Silicon S.L.
 *
 * Author: Javier Martin <javier.martin@vista-silicon.com>
 *
 * Based on sound/soc/codecs/wm8974 and TI driver for kernel 2.6.27.
 */

// Rust translation of soc/codecs/tlv320aic32x4.c.
// C include dependencies are intentionally left as external kernel/ASoC symbols.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = ::core::ffi::c_uchar;
type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;
type bool_ = bool;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
type c_long = ::core::ffi::c_long;
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { pub _private: [usize; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { pub _private: [usize; 0] }
#[repr(C)] pub struct snd_soc_component_driver { pub _private: [usize; 0] }
#[repr(C)] pub struct regmap_range_cfg {
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
    pub range_min: c_uint,
    pub range_max: c_uint,
}
#[repr(C)] pub struct clk_bulk_data { pub id: *const c_char, pub clk: *mut clk }
#[repr(C)] pub struct device { pub of_node: *mut device_node }

#[repr(C)]
pub struct aic32x4_priv {
    pub regmap: *mut regmap,
    pub power_cfg: u32,
    pub micpga_routing: u32,
    pub swapdacs: bool_,
    pub rstn_gpio: *mut gpio_desc,
    pub mclk_name: *const c_char,
    pub supply_ldo: *mut regulator,
    pub supply_iov: *mut regulator,
    pub supply_dv: *mut regulator,
    pub supply_av: *mut regulator,
    pub gpio_func: [c_uint; 5],
    pub dev: *mut device,
    pub type_: aic32x4_type,
    pub fmt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic32x4_type {
    AIC32X4_TYPE_AIC32X4 = 0,
    AIC32X4_TYPE_TAS2505 = 1,
}

unsafe extern "C" {
    static mut AIC32X4_ADCSETUP: c_uint;
    static mut AIC32X4_DINCTL: c_uint;
    static mut AIC32X4_DOUTCTL: c_uint;
    static mut AIC32X4_SCLKCTL: c_uint;
    static mut AIC32X4_MISOCTL: c_uint;
    static mut AIC32X4_GPIOCTL: c_uint;
    static mut AIC32X4_CMMODE: c_uint;
    static mut AIC32X4_LPLAYBACK: c_uint;
    static mut AIC32X4_RPLAYBACK: c_uint;
    static mut AIC32X4_LDACVOL: c_uint;
    static mut AIC32X4_RDACVOL: c_uint;
    static mut AIC32X4_HPLGAIN: c_uint;
    static mut AIC32X4_HPRGAIN: c_uint;
    static mut AIC32X4_LOLGAIN: c_uint;
    static mut AIC32X4_LORGAIN: c_uint;
    static mut AIC32X4_LMICPGAVOL: c_uint;
    static mut AIC32X4_RMICPGAVOL: c_uint;
    static mut AIC32X4_ADCFGA: c_uint;
    static mut AIC32X4_LADCVOL: c_uint;
    static mut AIC32X4_RADCVOL: c_uint;
    static mut AIC32X4_DACMUTE: c_uint;
    static mut AIC32X4_LAGC1: c_uint;
    static mut AIC32X4_RAGC1: c_uint;
    static mut AIC32X4_LAGC2: c_uint;
    static mut AIC32X4_RAGC2: c_uint;
    static mut AIC32X4_LAGC3: c_uint;
    static mut AIC32X4_RAGC3: c_uint;
    static mut AIC32X4_LAGC4: c_uint;
    static mut AIC32X4_RAGC4: c_uint;
    static mut AIC32X4_LAGC5: c_uint;
    static mut AIC32X4_RAGC5: c_uint;
    static mut AIC32X4_LAGC6: c_uint;
    static mut AIC32X4_RAGC6: c_uint;
    static mut AIC32X4_LAGC7: c_uint;
    static mut AIC32X4_RAGC7: c_uint;
    static mut AIC32X4_HPLROUTE: c_uint;
    static mut AIC32X4_HPRROUTE: c_uint;
    static mut AIC32X4_LOLROUTE: c_uint;
    static mut AIC32X4_LORROUTE: c_uint;
    static mut AIC32X4_LMICPGAPIN: c_uint;
    static mut AIC32X4_LMICPGANIN: c_uint;
    static mut AIC32X4_RMICPGAPIN: c_uint;
    static mut AIC32X4_RMICPGANIN: c_uint;
    static mut AIC32X4_MICBIAS: c_uint;
    static mut AIC32X4_OUTPWRCTL: c_uint;
    static mut AIC32X4_REFPOWERUP: c_uint;
    static mut AIC32X4_IFACE1: c_uint;
    static mut AIC32X4_IFACE2: c_uint;
    static mut AIC32X4_IFACE3: c_uint;
    static mut AIC32X4_AOSR: c_uint;
    static mut AIC32X4_DOSRMSB: c_uint;
    static mut AIC32X4_DOSRLSB: c_uint;
    static mut AIC32X4_DACSPB: c_uint;
    static mut AIC32X4_ADCSPB: c_uint;
    static mut AIC32X4_DACSETUP: c_uint;
    static mut AIC32X4_PWRCFG: c_uint;
    static mut AIC32X4_LDOCTL: c_uint;
    static mut AIC32X4_RESET: c_uint;
    static mut TAS2505_SPKVOL1: c_uint;
    static mut TAS2505_SPKVOL2: c_uint;
    static mut TAS2505_SPK: c_uint;
    static mut TAS2505_REFPOWERUP: c_uint;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, control: *const snd_kcontrol_new, count: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_bulk_get(dev: *mut device, count: c_int, clocks: *mut clk_bulk_data) -> c_int;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_bulk_prepare_enable(count: c_int, clocks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_disable_unprepare(count: c_int, clocks: *mut clk_bulk_data);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn of_property_match_string(np: *mut device_node, propname: *const c_char, string: *const c_char) -> c_int;
    fn of_clk_get_parent_name(np: *mut device_node, index: c_int) -> *const c_char;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut c_uint, sz: usize) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn devm_regulator_get_optional(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn aic32x4_register_clocks(dev: *mut device, mclk_name: *const c_char) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_ulong);
    fn ndelay(nsecs: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "Rust" {
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_int = 1;

const SND_SOC_DAPM_POST_PMD: c_int = 0x1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x4;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 2;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_DSP_A: c_uint = 2;
const SND_SOC_DAIFMT_DSP_B: c_uint = 3;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 4;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 5;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 0;

const AIC32X4_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const AIC32X4_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE;

extern "Rust" {
    static AIC32X4_LADC_EN: c_uint; static AIC32X4_RADC_EN: c_uint;
    static AIC32x4_MICBIAS_MASK: c_uint; static AIC32X4_MICBIAS_LDOIN: c_uint; static AIC32X4_MICBIAS_2075V: c_uint;
    static AIC32X4_MFP_GPIO_ENABLED: c_uint; static AIC32X4_MFP2_GPIO_OUT_HIGH: c_uint; static AIC32X4_MFP5_GPIO_OUT_HIGH: c_uint; static AIC32X4_MFP5_GPIO_OUTPUT: c_uint;
    static AIC32X4_BCLKMASTER: c_uint; static AIC32X4_WCLKMASTER: c_uint; static AIC32X4_DSP_MODE: c_uint; static AIC32X4_IFACE1_DATATYPE_SHIFT: c_uint;
    static AIC32X4_BCLKINV_MASK: c_uint; static AIC32X4_RIGHT_JUSTIFIED_MODE: c_uint; static AIC32X4_LEFT_JUSTIFIED_MODE: c_uint;
    static AIC32X4_IFACE1_DATATYPE_MASK: c_uint; static AIC32X4_IFACE1_MASTER_MASK: c_uint; static AIC32X4_DATA_OFFSET_MASK: c_uint;
    static AIC32X4_MAX_DOSR_FREQ: c_uint; static AIC32X4_MIN_DOSR_FREQ: c_uint; static AIC32X4_MAX_CODEC_CLKIN_FREQ: c_uint;
    static AIC32X4_WORD_LEN_16BITS: c_uint; static AIC32X4_WORD_LEN_20BITS: c_uint; static AIC32X4_WORD_LEN_24BITS: c_uint; static AIC32X4_WORD_LEN_32BITS: c_uint;
    static AIC32X4_IFACE1_DATALEN_SHIFT: c_uint; static AIC32X4_IFACE1_DATALEN_MASK: c_uint;
    static AIC32X4_RDAC2LCHN: c_uint; static AIC32X4_LDAC2LCHN: c_uint; static AIC32X4_LDAC2RCHN: c_uint; static AIC32X4_RDAC2RCHN: c_uint; static AIC32X4_DAC_CHAN_MASK: c_uint;
    static AIC32X4_MUTEON: c_uint; static AIC32X4_MFPX_DEFAULT_VALUE: c_uint;
    static AIC32X4_PWR_MICBIAS_2075_LDOIN: c_uint; static AIC32X4_PWR_AVDD_DVDD_WEAK_DISABLE: c_uint; static AIC32X4_AVDDWEAKDISABLE: c_uint;
    static AIC32X4_PWR_AIC32X4_LDO_ENABLE: c_uint; static AIC32X4_LDOCTLEN: c_uint; static AIC32X4_PWR_CMMODE_LDOIN_RANGE_18_36: c_uint;
    static AIC32X4_LDOIN_18_36: c_uint; static AIC32X4_PWR_CMMODE_HP_LDOIN_POWERED: c_uint; static AIC32X4_LDOIN2HP: c_uint;
    static AIC32X4_MICPGA_ROUTE_LMIC_IN2R_10K: c_uint; static AIC32X4_LMICPGANIN_IN2R_10K: c_uint; static AIC32X4_LMICPGANIN_CM1L_10K: c_uint;
    static AIC32X4_MICPGA_ROUTE_RMIC_IN1L_10K: c_uint; static AIC32X4_RMICPGANIN_IN1L_10K: c_uint; static AIC32X4_RMICPGANIN_CM1R_10K: c_uint;
    static AIC32X4_REFPOWERUP_40MS: c_uint;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[inline]
fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint { (n + d - 1) / d }

#[inline]
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int { N as c_int }

unsafe fn aic32x4_reset_adc(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let adc_reg: u32;
    /*
     * Workaround: the datasheet does not mention a required programming
     * sequence but experiments show the ADC needs to be reset after each
     * capture to avoid audible artifacts.
     */
    match event {
        SND_SOC_DAPM_POST_PMD => {
            adc_reg = snd_soc_component_read(component, AIC32X4_ADCSETUP);
            snd_soc_component_write(component, AIC32X4_ADCSETUP, adc_reg | AIC32X4_LADC_EN | AIC32X4_RADC_EN);
            snd_soc_component_write(component, AIC32X4_ADCSETUP, adc_reg);
        }
        _ => {}
    }
    0
}

unsafe fn mic_bias_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Change Mic Bias Registor */
            snd_soc_component_update_bits(component, AIC32X4_MICBIAS, AIC32x4_MICBIAS_MASK,
                AIC32X4_MICBIAS_LDOIN | AIC32X4_MICBIAS_2075V);
            dev_dbg((*component).dev, c"Mic Bias will be turned ON\n".as_ptr());
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, AIC32X4_MICBIAS, AIC32x4_MICBIAS_MASK, 0);
            dev_dbg((*component).dev, c"Mic Bias will be turned OFF\n".as_ptr());
        }
        _ => {}
    }
    0
}

unsafe fn aic32x4_get_mfp1_gpio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let val: u8 = snd_soc_component_read(component, AIC32X4_DINCTL) as u8;
    (*ucontrol).value.integer.value[0] = (val & 0x01) as c_long;
    0
}

unsafe fn aic32x4_set_mfp2_gpio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut val: u8 = snd_soc_component_read(component, AIC32X4_DOUTCTL) as u8;
    let gpio_check: u8 = val & AIC32X4_MFP_GPIO_ENABLED as u8;
    if gpio_check != AIC32X4_MFP_GPIO_ENABLED as u8 {
        dev_err((*component).dev, c"MFP2 is not configure as a GPIO output\n".as_ptr());
        return -EINVAL;
    }
    if (*ucontrol).value.integer.value[0] == (val & AIC32X4_MFP2_GPIO_OUT_HIGH as u8) as c_long { return 0; }
    if (*ucontrol).value.integer.value[0] != 0 { val |= (*ucontrol).value.integer.value[0] as u8; } else { val &= !(AIC32X4_MFP2_GPIO_OUT_HIGH as u8); }
    snd_soc_component_write(component, AIC32X4_DOUTCTL, val as c_uint);
    0
}

unsafe fn aic32x4_get_mfp3_gpio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let val: u8 = snd_soc_component_read(component, AIC32X4_SCLKCTL) as u8;
    (*ucontrol).value.integer.value[0] = (val & 0x01) as c_long;
    0
}

unsafe fn aic32x4_set_mfp4_gpio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut val: u8 = snd_soc_component_read(component, AIC32X4_MISOCTL) as u8;
    let gpio_check: u8 = val & AIC32X4_MFP_GPIO_ENABLED as u8;
    if gpio_check != AIC32X4_MFP_GPIO_ENABLED as u8 {
        dev_err((*component).dev, c"MFP4 is not configure as a GPIO output\n".as_ptr());
        return -EINVAL;
    }
    if (*ucontrol).value.integer.value[0] == (val & AIC32X4_MFP5_GPIO_OUT_HIGH as u8) as c_long { return 0; }
    if (*ucontrol).value.integer.value[0] != 0 { val |= (*ucontrol).value.integer.value[0] as u8; } else { val &= !(AIC32X4_MFP5_GPIO_OUT_HIGH as u8); }
    snd_soc_component_write(component, AIC32X4_MISOCTL, val as c_uint);
    0
}

unsafe fn aic32x4_get_mfp5_gpio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let val: u8 = snd_soc_component_read(component, AIC32X4_GPIOCTL) as u8;
    (*ucontrol).value.integer.value[0] = ((val & 0x2) >> 1) as c_long;
    0
}

unsafe fn aic32x4_set_mfp5_gpio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mut val: u8 = snd_soc_component_read(component, AIC32X4_GPIOCTL) as u8;
    let gpio_check: u8 = val & AIC32X4_MFP5_GPIO_OUTPUT as u8;
    if gpio_check != AIC32X4_MFP5_GPIO_OUTPUT as u8 {
        dev_err((*component).dev, c"MFP5 is not configure as a GPIO output\n".as_ptr());
        return -EINVAL;
    }
    if (*ucontrol).value.integer.value[0] == (val & 0x1) as c_long { return 0; }
    if (*ucontrol).value.integer.value[0] != 0 { val |= (*ucontrol).value.integer.value[0] as u8; } else { val &= 0xfe; }
    snd_soc_component_write(component, AIC32X4_GPIOCTL, val as c_uint);
    0
}

#[repr(C)]
struct aic32x4_mfp_cfg_entry { reg: c_uint, ctrl: snd_kcontrol_new }

// Static ALSA control/widget arrays from the C source are macro initializers:
// aic32x4_mfp_cfg, TLV DB scales, lo_cm_text, ptm_text, SOC enum declarations,
// aic32x4_snd_controls, output/input mixer controls, aic32x4_dapm_widgets,
// aic32x4_dapm_routes, TAS2505 controls/widgets/routes. Their C macro calls are
// preserved here as dependency intent and are expected to be supplied by ASoC
// Rust bindings in the target tree.
extern "Rust" {
    static aic32x4_mfp_cfg: [aic32x4_mfp_cfg_entry; 5];
    static aic32x4_snd_controls: [snd_kcontrol_new; 31];
    static hpl_output_mixer_controls: [snd_kcontrol_new; 2];
    static hpr_output_mixer_controls: [snd_kcontrol_new; 2];
    static lol_output_mixer_controls: [snd_kcontrol_new; 1];
    static lor_output_mixer_controls: [snd_kcontrol_new; 1];
    static aic32x4_dapm_widgets: [snd_soc_dapm_widget_desc; 38];
    static aic32x4_dapm_routes: [snd_soc_dapm_route; 76];
    static aic32x4_tas2505_snd_controls: [snd_kcontrol_new; 7];
    static hp_output_mixer_controls: [snd_kcontrol_new; 1];
    static aic32x4_tas2505_dapm_widgets: [snd_soc_dapm_widget_desc; 6];
    static aic32x4_tas2505_dapm_routes: [snd_soc_dapm_route; 5];
}

#[no_mangle]
pub static aic32x4_regmap_pages: [regmap_range_cfg; 1] = [
    regmap_range_cfg {
        selector_reg: 0,
        selector_mask: 0xff,
        window_start: 0,
        window_len: 128,
        range_min: 0,
        range_max: unsafe { AIC32X4_REFPOWERUP },
    },
];
// EXPORT_SYMBOL_GPL(aic32x4_regmap_pages);

unsafe fn aic32x4_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let pll = devm_clk_get((*component).dev, c"pll".as_ptr());
    if IS_ERR(pll as *const c_void) { return PTR_ERR(pll as *const c_void); }
    let mclk = clk_get_parent(pll);
    clk_set_rate(mclk, freq as c_ulong)
}

unsafe fn aic32x4_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    let mut iface_reg_1: u8 = 0;
    let mut iface_reg_2: u8 = 0;
    let mut iface_reg_3: u8 = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface_reg_1 |= (AIC32X4_BCLKMASTER | AIC32X4_WCLKMASTER) as u8,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => { dev_err((*component).dev, c"invalid clock provider\n".as_ptr()); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_DSP_A => {
            iface_reg_1 |= (AIC32X4_DSP_MODE << AIC32X4_IFACE1_DATATYPE_SHIFT) as u8;
            iface_reg_3 |= AIC32X4_BCLKINV_MASK as u8; /* invert bit clock */
            iface_reg_2 = 0x01; /* add offset 1 */
        }
        SND_SOC_DAIFMT_DSP_B => {
            iface_reg_1 |= (AIC32X4_DSP_MODE << AIC32X4_IFACE1_DATATYPE_SHIFT) as u8;
            iface_reg_3 |= AIC32X4_BCLKINV_MASK as u8; /* invert bit clock */
        }
        SND_SOC_DAIFMT_RIGHT_J => iface_reg_1 |= (AIC32X4_RIGHT_JUSTIFIED_MODE << AIC32X4_IFACE1_DATATYPE_SHIFT) as u8,
        SND_SOC_DAIFMT_LEFT_J => iface_reg_1 |= (AIC32X4_LEFT_JUSTIFIED_MODE << AIC32X4_IFACE1_DATATYPE_SHIFT) as u8,
        _ => { dev_err((*component).dev, c"invalid DAI interface format\n".as_ptr()); return -EINVAL; }
    }
    (*aic32x4).fmt = fmt;
    snd_soc_component_update_bits(component, AIC32X4_IFACE1, AIC32X4_IFACE1_DATATYPE_MASK | AIC32X4_IFACE1_MASTER_MASK, iface_reg_1 as c_uint);
    snd_soc_component_update_bits(component, AIC32X4_IFACE2, AIC32X4_DATA_OFFSET_MASK, iface_reg_2 as c_uint);
    snd_soc_component_update_bits(component, AIC32X4_IFACE3, AIC32X4_BCLKINV_MASK, iface_reg_3 as c_uint);
    0
}

unsafe fn aic32x4_set_aosr(component: *mut snd_soc_component, aosr: u8) -> c_int {
    snd_soc_component_write(component, AIC32X4_AOSR, aosr as c_uint)
}

unsafe fn aic32x4_set_dosr(component: *mut snd_soc_component, dosr: u16) -> c_int {
    snd_soc_component_write(component, AIC32X4_DOSRMSB, (dosr >> 8) as c_uint);
    snd_soc_component_write(component, AIC32X4_DOSRLSB, (dosr & 0xff) as c_uint);
    0
}

unsafe fn aic32x4_set_processing_blocks(component: *mut snd_soc_component, r_block: u8, p_block: u8) -> c_int {
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    if (*aic32x4).type_ == aic32x4_type::AIC32X4_TYPE_TAS2505 {
        if r_block != 0 || p_block > 3 { return -EINVAL; }
        snd_soc_component_write(component, AIC32X4_DACSPB, p_block as c_uint);
    } else {
        if r_block > 18 || p_block > 25 { return -EINVAL; }
        snd_soc_component_write(component, AIC32X4_ADCSPB, r_block as c_uint);
        snd_soc_component_write(component, AIC32X4_DACSPB, p_block as c_uint);
    }
    0
}

unsafe fn aic32x4_configure_rate(component: *mut snd_soc_component, rate: c_uint, aosr: *mut u8, adc_rc: *mut u8, dac_rc: *mut u8, dosr_inc: *mut u8) -> c_int {
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    let mut prb_rx: u8;
    let prb_tx: u8;
    if rate <= 48000 {
        *aosr = 128; *adc_rc = 6; *dac_rc = 8; *dosr_inc = 8; prb_rx = 1; prb_tx = 1;
    } else if rate <= 96000 {
        *aosr = 64; *adc_rc = 6; *dac_rc = 8; *dosr_inc = 4; prb_rx = 1;
        prb_tx = if (*aic32x4).type_ == aic32x4_type::AIC32X4_TYPE_TAS2505 { 1 } else { 9 };
    } else if rate == 192000 {
        *aosr = 32; *adc_rc = 3; *dac_rc = 4; *dosr_inc = 2; prb_rx = 13;
        prb_tx = if (*aic32x4).type_ == aic32x4_type::AIC32X4_TYPE_TAS2505 { 1 } else { 19 };
    } else {
        dev_err((*component).dev, c"Sampling rate %u not supported\n".as_ptr(), rate);
        return -EINVAL;
    }
    if (*aic32x4).type_ == aic32x4_type::AIC32X4_TYPE_TAS2505 { prb_rx = 0; }
    aic32x4_set_processing_blocks(component, prb_rx, prb_tx)
}

unsafe fn aic32x4_setup_clocks(component: *mut snd_soc_component, sample_rate: c_uint, mut channels: c_uint, bit_depth: c_uint) -> c_int {
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    let (mut aosr, mut adc_resource_class, mut dac_resource_class, mut dosr_increment): (u8, u8, u8, u8) = (0, 0, 0, 0);
    let mut clocks = [
        clk_bulk_data { id: c"pll".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"nadc".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"madc".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"ndac".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"mdac".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"bdiv".as_ptr(), clk: core::ptr::null_mut() },
    ];
    let ret = devm_clk_bulk_get((*component).dev, ARRAY_SIZE(&clocks), clocks.as_mut_ptr());
    if ret != 0 { return ret; }
    let ret = aic32x4_configure_rate(component, sample_rate, &mut aosr, &mut adc_resource_class, &mut dac_resource_class, &mut dosr_increment);
    if ret != 0 { return ret; }
    /* PCM over I2S is always 2-channel */
    if ((*aic32x4).fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S { channels = 2; }
    let madc = DIV_ROUND_UP(32 * adc_resource_class as c_uint, aosr as c_uint) as u8;
    let max_dosr = ((AIC32X4_MAX_DOSR_FREQ / sample_rate / dosr_increment as c_uint) * dosr_increment as c_uint) as u16;
    let min_dosr = ((AIC32X4_MIN_DOSR_FREQ / sample_rate / dosr_increment as c_uint) * dosr_increment as c_uint) as u16;
    let max_nadc = (AIC32X4_MAX_CODEC_CLKIN_FREQ / (madc as c_uint * aosr as c_uint * sample_rate)) as u8;
    let mut nadc = max_nadc;
    while nadc > 0 {
        let adc_clock_rate = nadc as c_ulong * madc as c_ulong * aosr as c_ulong * sample_rate as c_ulong;
        let mut dosr = max_dosr;
        while dosr >= min_dosr {
            let min_mdac = DIV_ROUND_UP(32 * dac_resource_class as c_uint, dosr as c_uint) as u8;
            let max_ndac = (AIC32X4_MAX_CODEC_CLKIN_FREQ / (min_mdac as c_uint * dosr as c_uint * sample_rate)) as u8;
            let mut mdac = min_mdac;
            while mdac <= 128 {
                let mut ndac = max_ndac;
                while ndac > 0 {
                    let dac_clock_rate = ndac as c_ulong * mdac as c_ulong * dosr as c_ulong * sample_rate as c_ulong;
                    if dac_clock_rate == adc_clock_rate && clk_round_rate(clocks[0].clk, dac_clock_rate) != 0 {
                        clk_set_rate(clocks[0].clk, dac_clock_rate);
                        clk_set_rate(clocks[1].clk, (sample_rate * aosr as c_uint * madc as c_uint) as c_ulong);
                        clk_set_rate(clocks[2].clk, (sample_rate * aosr as c_uint) as c_ulong);
                        aic32x4_set_aosr(component, aosr);
                        clk_set_rate(clocks[3].clk, (sample_rate * dosr as c_uint * mdac as c_uint) as c_ulong);
                        clk_set_rate(clocks[4].clk, (sample_rate * dosr as c_uint) as c_ulong);
                        aic32x4_set_dosr(component, dosr);
                        clk_set_rate(clocks[5].clk, (sample_rate * channels * bit_depth) as c_ulong);
                        return 0;
                    }
                    ndac = ndac.wrapping_sub(1);
                }
                if mdac == 128 { break; }
                mdac = mdac.wrapping_add(1);
            }
            if dosr < min_dosr + dosr_increment as u16 { break; }
            dosr = dosr.wrapping_sub(dosr_increment as u16);
        }
        nadc = nadc.wrapping_sub(1);
    }
    dev_err((*component).dev, c"Could not set clocks to support sample rate.\n".as_ptr());
    -EINVAL
}

unsafe fn aic32x4_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    let mut iface1_reg: u8 = 0;
    let dacsetup_reg: u8;
    aic32x4_setup_clocks(component, params_rate(params), params_channels(params), params_physical_width(params));
    match params_physical_width(params) {
        16 => iface1_reg |= (AIC32X4_WORD_LEN_16BITS << AIC32X4_IFACE1_DATALEN_SHIFT) as u8,
        20 => iface1_reg |= (AIC32X4_WORD_LEN_20BITS << AIC32X4_IFACE1_DATALEN_SHIFT) as u8,
        24 => iface1_reg |= (AIC32X4_WORD_LEN_24BITS << AIC32X4_IFACE1_DATALEN_SHIFT) as u8,
        32 => iface1_reg |= (AIC32X4_WORD_LEN_32BITS << AIC32X4_IFACE1_DATALEN_SHIFT) as u8,
        _ => {}
    }
    snd_soc_component_update_bits(component, AIC32X4_IFACE1, AIC32X4_IFACE1_DATALEN_MASK, iface1_reg as c_uint);
    if params_channels(params) == 1 {
        dacsetup_reg = (AIC32X4_RDAC2LCHN | AIC32X4_LDAC2LCHN) as u8;
    } else if (*aic32x4).swapdacs {
        dacsetup_reg = (AIC32X4_RDAC2LCHN | AIC32X4_LDAC2RCHN) as u8;
    } else {
        dacsetup_reg = (AIC32X4_LDAC2LCHN | AIC32X4_RDAC2RCHN) as u8;
    }
    snd_soc_component_update_bits(component, AIC32X4_DACSETUP, AIC32X4_DAC_CHAN_MASK, dacsetup_reg as c_uint);
    0
}

unsafe fn aic32x4_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    snd_soc_component_update_bits(component, AIC32X4_DACMUTE, AIC32X4_MUTEON, if mute != 0 { AIC32X4_MUTEON } else { 0 });
    0
}

unsafe fn aic32x4_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut clocks = [
        clk_bulk_data { id: c"madc".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"mdac".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"bdiv".as_ptr(), clk: core::ptr::null_mut() },
    ];
    let ret = devm_clk_bulk_get((*component).dev, ARRAY_SIZE(&clocks), clocks.as_mut_ptr());
    if ret != 0 { return ret; }
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            let ret = clk_bulk_prepare_enable(ARRAY_SIZE(&clocks), clocks.as_mut_ptr());
            if ret != 0 {
                dev_err((*component).dev, c"Failed to enable clocks\n".as_ptr());
                return ret;
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            /* Initial cold start */
            if snd_soc_dapm_get_bias_level(dapm) != snd_soc_bias_level::SND_SOC_BIAS_OFF {
                clk_bulk_disable_unprepare(ARRAY_SIZE(&clocks), clocks.as_mut_ptr());
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {}
    }
    0
}

extern "Rust" {
    static aic32x4_ops: snd_soc_dai_ops;
    static mut aic32x4_dai: snd_soc_dai_driver;
    static soc_component_dev_aic32x4: snd_soc_component_driver;
    static mut aic32x4_tas2505_dai: snd_soc_dai_driver;
    static soc_component_dev_aic32x4_tas2505: snd_soc_component_driver;
}

unsafe fn aic32x4_setup_gpios(component: *mut snd_soc_component) {
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    /* setup GPIO functions */
    let mut i = 0usize;
    while i < (*aic32x4).gpio_func.len() {
        if (*aic32x4).gpio_func[i] != AIC32X4_MFPX_DEFAULT_VALUE {
            snd_soc_component_write(component, aic32x4_mfp_cfg[i].reg, (*aic32x4).gpio_func[i]);
            snd_soc_add_component_controls(component, &aic32x4_mfp_cfg[i].ctrl, 1);
        }
        i += 1;
    }
}

unsafe fn aic32x4_component_probe(component: *mut snd_soc_component) -> c_int {
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    let mut tmp_reg: u32;
    let mut clocks = [
        clk_bulk_data { id: c"codec_clkin".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"pll".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"bdiv".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"mdac".as_ptr(), clk: core::ptr::null_mut() },
    ];
    let ret = devm_clk_bulk_get((*component).dev, ARRAY_SIZE(&clocks), clocks.as_mut_ptr());
    if ret != 0 { return ret; }
    aic32x4_setup_gpios(component);
    clk_set_parent(clocks[0].clk, clocks[1].clk);
    clk_set_parent(clocks[2].clk, clocks[3].clk);
    /* Power platform configuration */
    if ((*aic32x4).power_cfg & AIC32X4_PWR_MICBIAS_2075_LDOIN) != 0 {
        snd_soc_component_write(component, AIC32X4_MICBIAS, AIC32X4_MICBIAS_LDOIN | AIC32X4_MICBIAS_2075V);
    }
    if ((*aic32x4).power_cfg & AIC32X4_PWR_AVDD_DVDD_WEAK_DISABLE) != 0 {
        snd_soc_component_write(component, AIC32X4_PWRCFG, AIC32X4_AVDDWEAKDISABLE);
    }
    tmp_reg = if ((*aic32x4).power_cfg & AIC32X4_PWR_AIC32X4_LDO_ENABLE) != 0 { AIC32X4_LDOCTLEN } else { 0 };
    snd_soc_component_write(component, AIC32X4_LDOCTL, tmp_reg);
    tmp_reg = snd_soc_component_read(component, AIC32X4_CMMODE);
    if ((*aic32x4).power_cfg & AIC32X4_PWR_CMMODE_LDOIN_RANGE_18_36) != 0 { tmp_reg |= AIC32X4_LDOIN_18_36; }
    if ((*aic32x4).power_cfg & AIC32X4_PWR_CMMODE_HP_LDOIN_POWERED) != 0 { tmp_reg |= AIC32X4_LDOIN2HP; }
    snd_soc_component_write(component, AIC32X4_CMMODE, tmp_reg);
    /* Mic PGA routing */
    if ((*aic32x4).micpga_routing & AIC32X4_MICPGA_ROUTE_LMIC_IN2R_10K) != 0 {
        snd_soc_component_write(component, AIC32X4_LMICPGANIN, AIC32X4_LMICPGANIN_IN2R_10K);
    } else {
        snd_soc_component_write(component, AIC32X4_LMICPGANIN, AIC32X4_LMICPGANIN_CM1L_10K);
    }
    if ((*aic32x4).micpga_routing & AIC32X4_MICPGA_ROUTE_RMIC_IN1L_10K) != 0 {
        snd_soc_component_write(component, AIC32X4_RMICPGANIN, AIC32X4_RMICPGANIN_IN1L_10K);
    } else {
        snd_soc_component_write(component, AIC32X4_RMICPGANIN, AIC32X4_RMICPGANIN_CM1R_10K);
    }
    tmp_reg = snd_soc_component_read(component, AIC32X4_ADCSETUP);
    snd_soc_component_write(component, AIC32X4_ADCSETUP, tmp_reg | AIC32X4_LADC_EN | AIC32X4_RADC_EN);
    snd_soc_component_write(component, AIC32X4_ADCSETUP, tmp_reg);
    snd_soc_component_write(component, AIC32X4_REFPOWERUP, AIC32X4_REFPOWERUP_40MS);
    msleep(40);
    0
}

unsafe fn aic32x4_of_xlate_dai_id(_component: *mut snd_soc_component, _endpoint: *mut device_node) -> c_int {
    /* return dai id 0, whatever the endpoint index */
    0
}

unsafe fn aic32x4_tas2505_component_probe(component: *mut snd_soc_component) -> c_int {
    let aic32x4 = snd_soc_component_get_drvdata(component) as *mut aic32x4_priv;
    let mut tmp_reg: u32;
    let mut clocks = [
        clk_bulk_data { id: c"codec_clkin".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"pll".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"bdiv".as_ptr(), clk: core::ptr::null_mut() },
        clk_bulk_data { id: c"mdac".as_ptr(), clk: core::ptr::null_mut() },
    ];
    let ret = devm_clk_bulk_get((*component).dev, ARRAY_SIZE(&clocks), clocks.as_mut_ptr());
    if ret != 0 { return ret; }
    aic32x4_setup_gpios(component);
    clk_set_parent(clocks[0].clk, clocks[1].clk);
    clk_set_parent(clocks[2].clk, clocks[3].clk);
    /* Power platform configuration */
    if ((*aic32x4).power_cfg & AIC32X4_PWR_AVDD_DVDD_WEAK_DISABLE) != 0 {
        snd_soc_component_write(component, AIC32X4_PWRCFG, AIC32X4_AVDDWEAKDISABLE);
    }
    tmp_reg = if ((*aic32x4).power_cfg & AIC32X4_PWR_AIC32X4_LDO_ENABLE) != 0 { AIC32X4_LDOCTLEN } else { 0 };
    snd_soc_component_write(component, AIC32X4_LDOCTL, tmp_reg);
    tmp_reg = snd_soc_component_read(component, AIC32X4_CMMODE);
    if ((*aic32x4).power_cfg & AIC32X4_PWR_CMMODE_LDOIN_RANGE_18_36) != 0 { tmp_reg |= AIC32X4_LDOIN_18_36; }
    if ((*aic32x4).power_cfg & AIC32X4_PWR_CMMODE_HP_LDOIN_POWERED) != 0 { tmp_reg |= AIC32X4_LDOIN2HP; }
    snd_soc_component_write(component, AIC32X4_CMMODE, tmp_reg);
    /*
     * Enable the fast charging feature and ensure the needed 40ms elapsed
     * before using the analog circuits.
     */
    snd_soc_component_write(component, TAS2505_REFPOWERUP, AIC32X4_REFPOWERUP_40MS);
    msleep(40);
    0
}

unsafe fn aic32x4_parse_dt(aic32x4: *mut aic32x4_priv, np: *mut device_node) -> c_int {
    let ret = of_property_match_string(np, c"clock-names".as_ptr(), c"mclk".as_ptr());
    if ret < 0 { return -EINVAL; }
    (*aic32x4).mclk_name = of_clk_get_parent_name(np, ret);
    (*aic32x4).swapdacs = false;
    (*aic32x4).micpga_routing = 0;
    /* Assert reset using GPIOD_OUT_HIGH, because reset is GPIO_ACTIVE_LOW */
    (*aic32x4).rstn_gpio = devm_gpiod_get_optional((*aic32x4).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*aic32x4).rstn_gpio as *const c_void) {
        return dev_err_probe((*aic32x4).dev, PTR_ERR((*aic32x4).rstn_gpio as *const c_void), c"Failed to get reset gpio\n".as_ptr());
    } else {
        gpiod_set_consumer_name((*aic32x4).rstn_gpio, c"tlv320aic32x4_rstn".as_ptr());
    }
    let mut i = 0usize;
    while i < (*aic32x4).gpio_func.len() {
        (*aic32x4).gpio_func[i] = AIC32X4_MFPX_DEFAULT_VALUE;
        i += 1;
    }
    of_property_read_u32_array(np, c"aic32x4-gpio-func".as_ptr(), (*aic32x4).gpio_func.as_mut_ptr(), (*aic32x4).gpio_func.len());
    0
}

unsafe fn aic32x4_disable_regulators(aic32x4: *mut aic32x4_priv) {
    regulator_disable((*aic32x4).supply_iov);
    if !IS_ERR((*aic32x4).supply_ldo as *const c_void) { regulator_disable((*aic32x4).supply_ldo); }
    if !IS_ERR((*aic32x4).supply_dv as *const c_void) { regulator_disable((*aic32x4).supply_dv); }
    if !IS_ERR((*aic32x4).supply_av as *const c_void) { regulator_disable((*aic32x4).supply_av); }
}

unsafe fn aic32x4_setup_regulators(dev: *mut device, aic32x4: *mut aic32x4_priv) -> c_int {
    let mut ret: c_int;
    (*aic32x4).supply_ldo = devm_regulator_get_optional(dev, c"ldoin".as_ptr());
    (*aic32x4).supply_iov = devm_regulator_get(dev, c"iov".as_ptr());
    (*aic32x4).supply_dv = devm_regulator_get_optional(dev, c"dv".as_ptr());
    (*aic32x4).supply_av = devm_regulator_get_optional(dev, c"av".as_ptr());
    /* Check if the regulator requirements are fulfilled */
    if IS_ERR((*aic32x4).supply_iov as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*aic32x4).supply_iov as *const c_void), c"Missing supply 'iov'\n".as_ptr());
    }
    if IS_ERR((*aic32x4).supply_ldo as *const c_void) {
        if PTR_ERR((*aic32x4).supply_ldo as *const c_void) == -EPROBE_DEFER { return -EPROBE_DEFER; }
        if IS_ERR((*aic32x4).supply_dv as *const c_void) {
            return dev_err_probe(dev, PTR_ERR((*aic32x4).supply_dv as *const c_void), c"Missing supply 'dv' or 'ldoin'\n".as_ptr());
        }
        if IS_ERR((*aic32x4).supply_av as *const c_void) {
            return dev_err_probe(dev, PTR_ERR((*aic32x4).supply_av as *const c_void), c"Missing supply 'av' or 'ldoin'\n".as_ptr());
        }
    } else {
        if PTR_ERR((*aic32x4).supply_dv as *const c_void) == -EPROBE_DEFER { return -EPROBE_DEFER; }
        if PTR_ERR((*aic32x4).supply_av as *const c_void) == -EPROBE_DEFER { return -EPROBE_DEFER; }
    }
    ret = regulator_enable((*aic32x4).supply_iov);
    if ret != 0 { dev_err(dev, c"Failed to enable regulator iov\n".as_ptr()); return ret; }
    if !IS_ERR((*aic32x4).supply_ldo as *const c_void) {
        ret = regulator_enable((*aic32x4).supply_ldo);
        if ret != 0 { dev_err(dev, c"Failed to enable regulator ldo\n".as_ptr()); goto_error_ldo(aic32x4); return ret; }
    }
    if !IS_ERR((*aic32x4).supply_dv as *const c_void) {
        ret = regulator_enable((*aic32x4).supply_dv);
        if ret != 0 {
            dev_err(dev, c"Failed to enable regulator dv\n".as_ptr());
            if !IS_ERR((*aic32x4).supply_ldo as *const c_void) { regulator_disable((*aic32x4).supply_ldo); }
            regulator_disable((*aic32x4).supply_iov);
            return ret;
        }
    }
    if !IS_ERR((*aic32x4).supply_av as *const c_void) {
        ret = regulator_enable((*aic32x4).supply_av);
        if ret != 0 {
            dev_err(dev, c"Failed to enable regulator av\n".as_ptr());
            if !IS_ERR((*aic32x4).supply_dv as *const c_void) { regulator_disable((*aic32x4).supply_dv); }
            if !IS_ERR((*aic32x4).supply_ldo as *const c_void) { regulator_disable((*aic32x4).supply_ldo); }
            regulator_disable((*aic32x4).supply_iov);
            return ret;
        }
    }
    if !IS_ERR((*aic32x4).supply_ldo as *const c_void) && IS_ERR((*aic32x4).supply_av as *const c_void) {
        (*aic32x4).power_cfg |= AIC32X4_PWR_AIC32X4_LDO_ENABLE;
    }
    0
}

unsafe fn goto_error_ldo(aic32x4: *mut aic32x4_priv) {
    regulator_disable((*aic32x4).supply_iov);
}

#[no_mangle]
pub unsafe extern "C" fn aic32x4_probe(dev: *mut device, regmap: *mut regmap, type_: aic32x4_type) -> c_int {
    let np = (*dev).of_node;
    let aic32x4 = devm_kzalloc(dev, core::mem::size_of::<aic32x4_priv>(), GFP_KERNEL) as *mut aic32x4_priv;
    if aic32x4.is_null() { return -ENOMEM; }
    (*aic32x4).dev = dev;
    (*aic32x4).type_ = type_;
    (*aic32x4).regmap = regmap;
    dev_set_drvdata(dev, aic32x4 as *mut c_void);
    let mut ret: c_int;
    if !np.is_null() {
        ret = aic32x4_parse_dt(aic32x4, np);
        if ret != 0 { dev_err(dev, c"Failed to parse DT node\n".as_ptr()); return ret; }
    } else {
        (*aic32x4).power_cfg = 0;
        (*aic32x4).swapdacs = false;
        (*aic32x4).micpga_routing = 0;
        (*aic32x4).rstn_gpio = core::ptr::null_mut();
        (*aic32x4).mclk_name = c"mclk".as_ptr();
    }
    ret = aic32x4_setup_regulators(dev, aic32x4);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to setup regulators\n".as_ptr()); }
    if !(*aic32x4).rstn_gpio.is_null() {
        ndelay(10);
        /* deassert reset */
        gpiod_set_value_cansleep((*aic32x4).rstn_gpio, 0);
        mdelay(1);
    }
    ret = regmap_write(regmap, AIC32X4_RESET, 0x01);
    if ret != 0 { aic32x4_disable_regulators(aic32x4); return ret; }
    ret = aic32x4_register_clocks(dev, (*aic32x4).mclk_name);
    if ret != 0 { aic32x4_disable_regulators(aic32x4); return ret; }
    match (*aic32x4).type_ {
        aic32x4_type::AIC32X4_TYPE_TAS2505 => {
            ret = devm_snd_soc_register_component(dev, &soc_component_dev_aic32x4_tas2505, &mut aic32x4_tas2505_dai, 1);
        }
        _ => {
            ret = devm_snd_soc_register_component(dev, &soc_component_dev_aic32x4, &mut aic32x4_dai, 1);
        }
    }
    if ret != 0 {
        dev_err(dev, c"Failed to register component\n".as_ptr());
        aic32x4_disable_regulators(aic32x4);
        return ret;
    }
    0
}
// EXPORT_SYMBOL(aic32x4_probe);

#[no_mangle]
pub unsafe extern "C" fn aic32x4_remove(dev: *mut device) {
    let aic32x4 = dev_get_drvdata(dev) as *mut aic32x4_priv;
    aic32x4_disable_regulators(aic32x4);
}
// EXPORT_SYMBOL(aic32x4_remove);

// MODULE_DESCRIPTION("ASoC tlv320aic32x4 codec driver");
// MODULE_AUTHOR("Javier Martin <javier.martin@vista-silicon.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
