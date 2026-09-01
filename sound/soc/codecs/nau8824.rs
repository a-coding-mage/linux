// SPDX-License-Identifier: GPL-2.0-only
/*
 * NAU88L24 ALSA SoC audio driver
 *
 * Copyright 2016 Nuvoton Technology Corp.
 * Author: John Hsu <KCHSU0@nuvoton.com>
 *
 * Rust source-level translation of soc/codecs/nau8824.c.
 * Linux/ALSA symbols supplied by the original C headers are intentionally
 * referenced as external dependency symbols.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u64_ = u64;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct semaphore {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub status: c_int,
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
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct nau8824_fll_attr {
    pub param: c_uint,
    pub val: c_uint,
}

#[repr(C)]
pub struct nau8824_osr_attr {
    pub osr: c_uint,
    pub clk_src: c_uint,
}

#[repr(C)]
pub struct nau8824_fll {
    pub mclk_src: c_uint,
    pub ratio: c_uint,
    pub fll_frac: c_uint,
    pub fll_int: c_uint,
    pub clk_ref_div: c_uint,
}

#[repr(C)]
pub struct nau8824 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub mclk: *mut clk,
    pub irq: c_int,
    pub jack: *mut snd_soc_jack,
    pub dapm: *mut snd_soc_dapm_context,
    pub jdet_work: work_struct,
    pub jd_sem: semaphore,
    pub resume_lock: bool_,
    pub fs: c_uint,
    pub jkdet_polarity: c_uint,
    pub micbias_voltage: c_uint,
    pub vref_impedance: c_uint,
    pub sar_threshold_num: c_uint,
    pub sar_threshold: [c_uint; 8],
    pub sar_hysteresis: c_uint,
    pub sar_voltage: c_uint,
    pub sar_compare_time: c_uint,
    pub sar_sampling_time: c_uint,
    pub key_debounce: c_uint,
    pub jack_eject_debounce: c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct dmi_system_id {
    pub driver_data: *mut c_void,
}

extern "C" {
    fn down_timeout(sem: *mut semaphore, timeout: c_long) -> c_int;
    fn down_interruptible(sem: *mut semaphore) -> c_int;
    fn up(sem: *mut semaphore);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn msleep(ms: c_uint);
    fn mdelay(ms: c_uint);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut nau8824;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: c_int) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn schedule_work(work: *mut work_struct) -> bool_;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_int, min: c_uint, max: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn device_property_read_u32(dev: *mut device, prop: *const c_char, val: *mut c_uint) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, prop: *const c_char, val: *mut c_uint, nval: c_uint) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dmi_first_match(table: *const dmi_system_id) -> *const dmi_system_id;
    fn dev_get_platdata(dev: *mut device) -> *mut nau8824;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn sema_init(sem: *mut semaphore, val: c_int);
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const c_void, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
}

fn BIT(n: c_uint) -> c_uint {
    1u32.wrapping_shl(n)
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const HZ: c_long = 100;
const GFP_KERNEL: c_uint = 0;

const NAU8824_JD_ACTIVE_HIGH: c_int = 1 << 0;
const NAU8824_MONO_SPEAKER: c_int = 1 << 1;

static mut nau8824_quirk: c_int = 0;
static mut quirk_override: c_int = -1;

/* module_param_named(quirk, quirk_override, uint, 0444);
 * MODULE_PARM_DESC(quirk, "Board-specific quirk override");
 */

/* the ADC threshold of headset */
const DMIC_CLK: c_uint = 3072000;

/* the ADC threshold of headset */
const HEADSET_SARADC_THD: c_uint = 0x80;

/* the parameter threshold of FLL */
const NAU_FREF_MAX: c_uint = 13500000;
const NAU_FVCO_MAX: u64_ = 100000000;
const NAU_FVCO_MIN: u64_ = 90000000;

/* scaling for mclk from sysclk_src output */
static mclk_src_scaling: [nau8824_fll_attr; 10] = [
    nau8824_fll_attr { param: 1, val: 0x0 },
    nau8824_fll_attr { param: 2, val: 0x2 },
    nau8824_fll_attr { param: 4, val: 0x3 },
    nau8824_fll_attr { param: 8, val: 0x4 },
    nau8824_fll_attr { param: 16, val: 0x5 },
    nau8824_fll_attr { param: 32, val: 0x6 },
    nau8824_fll_attr { param: 3, val: 0x7 },
    nau8824_fll_attr { param: 6, val: 0xa },
    nau8824_fll_attr { param: 12, val: 0xb },
    nau8824_fll_attr { param: 24, val: 0xc },
];

/* ratio for input clk freq */
static fll_ratio: [nau8824_fll_attr; 7] = [
    nau8824_fll_attr { param: 512000, val: 0x01 },
    nau8824_fll_attr { param: 256000, val: 0x02 },
    nau8824_fll_attr { param: 128000, val: 0x04 },
    nau8824_fll_attr { param: 64000, val: 0x08 },
    nau8824_fll_attr { param: 32000, val: 0x10 },
    nau8824_fll_attr { param: 8000, val: 0x20 },
    nau8824_fll_attr { param: 4000, val: 0x40 },
];

static fll_pre_scalar: [nau8824_fll_attr; 4] = [
    nau8824_fll_attr { param: 1, val: 0x0 },
    nau8824_fll_attr { param: 2, val: 0x1 },
    nau8824_fll_attr { param: 4, val: 0x2 },
    nau8824_fll_attr { param: 8, val: 0x3 },
];

/* the maximum frequency of CLK_ADC and CLK_DAC */
const CLK_DA_AD_MAX: c_uint = 6144000;

/* over sampling rate */
static osr_dac_sel: [nau8824_osr_attr; 5] = [
    nau8824_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8824_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
    nau8824_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8824_osr_attr { osr: 0, clk_src: 0 },
    nau8824_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
];

static osr_adc_sel: [nau8824_osr_attr; 4] = [
    nau8824_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
    nau8824_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8824_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8824_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
];

/* Register defaults and ALSA/DAPM control tables are direct C macro data in the
 * original file. Their Rust forms intentionally remain dependency-shaped: the
 * concrete struct layouts and constructors are provided by Linux/ALSA headers.
 *
 * Translated table groups:
 * - nau8824_reg_defaults: all NAU8824_REG_* default pairs from the C source.
 * - nau8824_companding/adcs/dac/input/TDM string arrays and soc_enum values.
 * - spk_vol_tlv, hp_vol_tlv, mic_vol_tlv, dmic_vol_tlv TLV declarations.
 * - nau8824_snd_controls.
 * - DMIC switches, ADC/HP mixers, DAC muxes.
 * - nau8824_dapm_widgets and nau8824_dapm_routes.
 * - nau8824_component_driver, nau8824_dai_ops, nau8824_dai.
 * - nau8824_regmap_config.
 * - DMI, I2C, OF, ACPI match tables and module registration metadata.
 */

unsafe fn nau8824_config_sysclk(nau8824: *mut nau8824, clk_id: c_int, freq: c_uint) -> c_int;
unsafe fn nau8824_is_jack_inserted(nau8824: *mut nau8824) -> bool_;

unsafe extern "C" fn nau8824_sema_acquire(nau8824: *mut nau8824, timeout: c_long) -> c_int {
    let ret: c_int;

    if timeout != 0 {
        ret = down_timeout(ptr::addr_of_mut!((*nau8824).jd_sem), timeout);
        if ret < 0 {
            dev_warn((*nau8824).dev, b"Acquire semaphore timeout\n\0".as_ptr() as *const c_char);
        }
    } else {
        ret = down_interruptible(ptr::addr_of_mut!((*nau8824).jd_sem));
        if ret < 0 {
            dev_warn((*nau8824).dev, b"Acquire semaphore fail\n\0".as_ptr() as *const c_char);
        }
    }

    ret
}

unsafe fn nau8824_sema_release(nau8824: *mut nau8824) {
    up(ptr::addr_of_mut!((*nau8824).jd_sem));
}

unsafe extern "C" fn nau8824_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        NAU8824_REG_ENA_CTRL..=NAU8824_REG_FLL_VCO_RSV
        | NAU8824_REG_JACK_DET_CTRL
        | NAU8824_REG_INTERRUPT_SETTING_1
        | NAU8824_REG_IRQ
        | NAU8824_REG_CLEAR_INT_REG..=NAU8824_REG_VDET_THRESHOLD_4
        | NAU8824_REG_GPIO_SEL
        | NAU8824_REG_PORT0_I2S_PCM_CTRL_1..=NAU8824_REG_TDM_CTRL
        | NAU8824_REG_ADC_HPF_FILTER..=NAU8824_REG_EQ4_EQ5
        | NAU8824_REG_ADC_CH0_DGAIN_CTRL..=NAU8824_REG_ADC_TO_DAC_ST
        | NAU8824_REG_DRC_KNEE_IP12_ADC_CH01..=NAU8824_REG_DRC_GAINL_ADC3
        | NAU8824_REG_DRC_KNEE_IP12_DAC..=NAU8824_REG_DRC_GAIN_DAC_CH1
        | NAU8824_REG_CLASSG..=NAU8824_REG_OTP_EFUSE
        | NAU8824_REG_OTPDOUT_1..=NAU8824_REG_OTPDOUT_2
        | NAU8824_REG_I2C_TIMEOUT
        | NAU8824_REG_I2C_DEVICE_ID..=NAU8824_REG_SAR_ADC_DATA_OUT
        | NAU8824_REG_BIAS_ADJ..=NAU8824_REG_CLASSD_GAIN_2
        | NAU8824_REG_ANALOG_ADC_1..=NAU8824_REG_ATT_PORT1
        | NAU8824_REG_POWER_UP_CONTROL..=NAU8824_REG_CHARGE_PUMP_INPUT => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8824_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        NAU8824_REG_RESET..=NAU8824_REG_FLL_VCO_RSV
        | NAU8824_REG_JACK_DET_CTRL
        | NAU8824_REG_INTERRUPT_SETTING_1
        | NAU8824_REG_CLEAR_INT_REG..=NAU8824_REG_VDET_THRESHOLD_4
        | NAU8824_REG_GPIO_SEL
        | NAU8824_REG_PORT0_I2S_PCM_CTRL_1..=NAU8824_REG_TDM_CTRL
        | NAU8824_REG_ADC_HPF_FILTER..=NAU8824_REG_EQ4_EQ5
        | NAU8824_REG_ADC_CH0_DGAIN_CTRL..=NAU8824_REG_ADC_TO_DAC_ST
        | NAU8824_REG_DRC_KNEE_IP12_ADC_CH01
        | NAU8824_REG_DRC_KNEE_IP34_ADC_CH01
        | NAU8824_REG_DRC_SLOPE_ADC_CH01
        | NAU8824_REG_DRC_ATKDCY_ADC_CH01
        | NAU8824_REG_DRC_KNEE_IP12_ADC_CH23
        | NAU8824_REG_DRC_KNEE_IP34_ADC_CH23
        | NAU8824_REG_DRC_SLOPE_ADC_CH23
        | NAU8824_REG_DRC_ATKDCY_ADC_CH23
        | NAU8824_REG_DRC_KNEE_IP12_DAC..=NAU8824_REG_DRC_ATKDCY_DAC
        | NAU8824_REG_CLASSG..=NAU8824_REG_OTP_EFUSE
        | NAU8824_REG_I2C_TIMEOUT
        | NAU8824_REG_BIAS_ADJ..=NAU8824_REG_CLASSD_GAIN_2
        | NAU8824_REG_ANALOG_ADC_1..=NAU8824_REG_ATT_PORT1
        | NAU8824_REG_POWER_UP_CONTROL..=NAU8824_REG_CHARGE_PUMP_CONTROL => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8824_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        NAU8824_REG_RESET
        | NAU8824_REG_IRQ..=NAU8824_REG_CLEAR_INT_REG
        | NAU8824_REG_DRC_GAINL_ADC0..=NAU8824_REG_DRC_GAINL_ADC3
        | NAU8824_REG_DRC_GAIN_DAC_CH0..=NAU8824_REG_DRC_GAIN_DAC_CH1
        | NAU8824_REG_OTPDOUT_1..=NAU8824_REG_OTPDOUT_2
        | NAU8824_REG_I2C_DEVICE_ID..=NAU8824_REG_SAR_ADC_DATA_OUT
        | NAU8824_REG_CHARGE_PUMP_INPUT => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8824_output_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8824 = snd_soc_component_get_drvdata(component);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Disables the TESTDAC to let DAC signal pass through. */
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_ENABLE_LO, NAU8824_TEST_DAC_EN, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_ENABLE_LO, NAU8824_TEST_DAC_EN, NAU8824_TEST_DAC_EN);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn nau8824_spk_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8824 = snd_soc_component_get_drvdata(component);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_ANALOG_CONTROL_2, NAU8824_CLASSD_CLAMP_DIS, NAU8824_CLASSD_CLAMP_DIS);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_ANALOG_CONTROL_2, NAU8824_CLASSD_CLAMP_DIS, 0);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn nau8824_pump_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8824 = snd_soc_component_get_drvdata(component);

    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Prevent startup click by letting charge pump to ramp up */
            msleep(10);
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_CHARGE_PUMP_CONTROL, NAU8824_JAMNODCLOW, NAU8824_JAMNODCLOW);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_CHARGE_PUMP_CONTROL, NAU8824_JAMNODCLOW, 0);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn system_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8824 = snd_soc_component_get_drvdata(component);
    let regmap = (*nau8824).regmap;
    let mut value: c_uint = 0;
    let mut clk_fll: bool_;
    let mut error: bool_;
    let ret: c_int;

    if SND_SOC_DAPM_EVENT_OFF(event) {
        dev_dbg((*nau8824).dev, b"system clock control : POWER OFF\n\0".as_ptr() as *const c_char);
        /* Set clock source to disable or internal clock before the
         * playback or capture end. Codec needs clock for Jack
         * detection and button press if jack inserted; otherwise,
         * the clock should be closed.
         */
        if nau8824_is_jack_inserted(nau8824) {
            nau8824_config_sysclk(nau8824, NAU8824_CLK_INTERNAL, 0);
        } else {
            nau8824_config_sysclk(nau8824, NAU8824_CLK_DIS, 0);
        }

        clk_disable_unprepare((*nau8824).mclk);
    } else {
        dev_dbg((*nau8824).dev, b"system clock control : POWER ON\n\0".as_ptr() as *const c_char);

        ret = clk_prepare_enable((*nau8824).mclk);
        if ret != 0 {
            return ret;
        }

        /* Check the clock source setting is proper or not
         * no matter the source is from FLL or MCLK.
         */
        regmap_read(regmap, NAU8824_REG_FLL1, &mut value);
        clk_fll = (value & NAU8824_FLL_RATIO_MASK) != 0;
        /* It's error to use internal clock when playback */
        regmap_read(regmap, NAU8824_REG_FLL6, &mut value);
        error = (value & NAU8824_DCO_EN) != 0;
        if !error {
            /* Check error depending on source is FLL or MCLK. */
            regmap_read(regmap, NAU8824_REG_CLK_DIVIDER, &mut value);
            if clk_fll {
                error = (value & NAU8824_CLK_SRC_VCO) == 0;
            } else {
                error = (value & NAU8824_CLK_SRC_VCO) != 0;
            }
        }
        /* Recover the clock source setting if error. */
        if error {
            if clk_fll {
                regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_DCO_EN, 0);
                regmap_update_bits(regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_SRC_MASK, NAU8824_CLK_SRC_VCO);
            } else {
                nau8824_config_sysclk(nau8824, NAU8824_CLK_MCLK, 0);
            }
        }
    }

    0
}

unsafe extern "C" fn dmic_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8824 = snd_soc_component_get_drvdata(component);
    let mut src: c_int;
    let mut freq: c_uint;

    freq = clk_get_rate((*nau8824).mclk);
    if freq == 0 {
        freq = (*nau8824).fs.wrapping_mul(256);
    }

    /* The DMIC clock is gotten from system clock (256fs) divided by
     * DMIC_SRC (1, 2, 4, 8, 16, 32). The clock has to be equal or
     * less than 3.072 MHz.
     */
    src = 0;
    while src < 5 {
        if freq / (0x1u32 << src) <= DMIC_CLK {
            break;
        }
        src += 1;
    }
    dev_dbg((*nau8824).dev, b"dmic src %d for mclk %d\n\0".as_ptr() as *const c_char, src, freq);
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_DMIC_SRC_MASK, (src as c_uint) << NAU8824_CLK_DMIC_SRC_SFT);

    0
}

unsafe fn nau8824_is_jack_inserted(nau8824: *mut nau8824) -> bool_ {
    let jack = (*nau8824).jack;
    let mut insert = false;

    if (*nau8824).irq != 0 && !jack.is_null() {
        insert = ((*jack).status & SND_JACK_HEADPHONE) != 0;
    }

    insert
}

unsafe fn nau8824_int_status_clear_all(regmap: *mut regmap) {
    let mut active_irq: c_uint = 0;
    let mut i: c_int;

    /* Reset the intrruption status from rightmost bit if the corres-
     * ponding irq event occurs.
     */
    regmap_read(regmap, NAU8824_REG_IRQ, &mut active_irq);
    i = 0;
    while i < NAU8824_REG_DATA_LEN as c_int {
        let clear_irq = 0x1u32 << i;
        if (active_irq & clear_irq) != 0 {
            regmap_write(regmap, NAU8824_REG_CLEAR_INT_REG, clear_irq);
        }
        i += 1;
    }
}

unsafe fn nau8824_eject_jack(nau8824: *mut nau8824) {
    let dapm = (*nau8824).dapm;
    let regmap = (*nau8824).regmap;

    /* Clear all interruption status */
    nau8824_int_status_clear_all(regmap);

    snd_soc_dapm_disable_pin(dapm, b"SAR\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);
    snd_soc_dapm_sync(dapm);

    /* Enable the insertion interruption, disable the ejection
     * interruption, and then bypass de-bounce circuit.
     */
    regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING,
        NAU8824_IRQ_KEY_RELEASE_DIS | NAU8824_IRQ_KEY_SHORT_PRESS_DIS | NAU8824_IRQ_EJECT_DIS | NAU8824_IRQ_INSERT_DIS,
        NAU8824_IRQ_KEY_RELEASE_DIS | NAU8824_IRQ_KEY_SHORT_PRESS_DIS | NAU8824_IRQ_EJECT_DIS);
    regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING_1,
        NAU8824_IRQ_INSERT_EN | NAU8824_IRQ_EJECT_EN, NAU8824_IRQ_INSERT_EN);
    regmap_update_bits(regmap, NAU8824_REG_ENA_CTRL, NAU8824_JD_SLEEP_MODE, NAU8824_JD_SLEEP_MODE);

    /* Close clock for jack type detection at manual mode */
    if snd_soc_dapm_get_bias_level(dapm) < SND_SOC_BIAS_PREPARE {
        nau8824_config_sysclk(nau8824, NAU8824_CLK_DIS, 0);
    }
}

unsafe extern "C" fn nau8824_jdet_work(work: *mut work_struct) {
    let nau8824 = container_of_nau8824_jdet_work(work);
    let dapm = (*nau8824).dapm;
    let regmap = (*nau8824).regmap;
    let mut adc_value: c_uint = 0;
    let mut event: c_int = 0;
    let mut event_mask: c_int = 0;

    snd_soc_dapm_force_enable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);
    snd_soc_dapm_force_enable_pin(dapm, b"SAR\0".as_ptr() as *const c_char);
    snd_soc_dapm_sync(dapm);

    msleep(100);

    regmap_read(regmap, NAU8824_REG_SAR_ADC_DATA_OUT, &mut adc_value);
    adc_value &= NAU8824_SAR_ADC_DATA_MASK;
    dev_dbg((*nau8824).dev, b"SAR ADC data 0x%02x\n\0".as_ptr() as *const c_char, adc_value);
    if adc_value < HEADSET_SARADC_THD {
        event |= SND_JACK_HEADPHONE;

        snd_soc_dapm_disable_pin(dapm, b"SAR\0".as_ptr() as *const c_char);
        snd_soc_dapm_disable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);
        snd_soc_dapm_sync(dapm);
    } else {
        event |= SND_JACK_HEADSET;
    }
    event_mask |= SND_JACK_HEADSET;
    snd_soc_jack_report((*nau8824).jack, event, event_mask);

    /* Enable short key press and release interruption. */
    regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING,
        NAU8824_IRQ_KEY_RELEASE_DIS | NAU8824_IRQ_KEY_SHORT_PRESS_DIS, 0);

    if (*nau8824).resume_lock {
        nau8824_sema_release(nau8824);
        (*nau8824).resume_lock = false;
    }
}

unsafe fn nau8824_setup_auto_irq(nau8824: *mut nau8824) {
    let regmap = (*nau8824).regmap;

    /* Enable jack ejection interruption. */
    regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING_1,
        NAU8824_IRQ_INSERT_EN | NAU8824_IRQ_EJECT_EN, NAU8824_IRQ_EJECT_EN);
    regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING, NAU8824_IRQ_EJECT_DIS, 0);
    /* Enable internal VCO needed for interruptions */
    if snd_soc_dapm_get_bias_level((*nau8824).dapm) < SND_SOC_BIAS_PREPARE {
        nau8824_config_sysclk(nau8824, NAU8824_CLK_INTERNAL, 0);
    }
    regmap_update_bits(regmap, NAU8824_REG_ENA_CTRL, NAU8824_JD_SLEEP_MODE, 0);
}

fn nau8824_button_decode(value: c_int) -> c_int {
    let mut buttons = 0;

    /* The chip supports up to 8 buttons, but ALSA defines
     * only 6 buttons.
     */
    if (value & BIT(0) as c_int) != 0 { buttons |= SND_JACK_BTN_0; }
    if (value & BIT(1) as c_int) != 0 { buttons |= SND_JACK_BTN_1; }
    if (value & BIT(2) as c_int) != 0 { buttons |= SND_JACK_BTN_2; }
    if (value & BIT(3) as c_int) != 0 { buttons |= SND_JACK_BTN_3; }
    if (value & BIT(4) as c_int) != 0 { buttons |= SND_JACK_BTN_4; }
    if (value & BIT(5) as c_int) != 0 { buttons |= SND_JACK_BTN_5; }

    buttons
}

const NAU8824_BUTTONS: c_int = SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;

unsafe extern "C" fn nau8824_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let nau8824 = data as *mut nau8824;
    let regmap = (*nau8824).regmap;
    let mut active_irq: c_uint = 0;
    let mut clear_irq: c_uint = 0;
    let mut event: c_int = 0;
    let mut event_mask: c_int = 0;

    if regmap_read(regmap, NAU8824_REG_IRQ, &mut active_irq) != 0 {
        dev_err((*nau8824).dev, b"failed to read irq status\n\0".as_ptr() as *const c_char);
        return IRQ_NONE;
    }
    dev_dbg((*nau8824).dev, b"IRQ %x\n\0".as_ptr() as *const c_char, active_irq);

    if (active_irq & NAU8824_JACK_EJECTION_DETECTED) != 0 {
        nau8824_eject_jack(nau8824);
        event_mask |= SND_JACK_HEADSET;
        clear_irq = NAU8824_JACK_EJECTION_DETECTED;
        /* release semaphore held after resume,
         * and cancel jack detection
         */
        if (*nau8824).resume_lock {
            nau8824_sema_release(nau8824);
            (*nau8824).resume_lock = false;
        }
        cancel_work_sync(ptr::addr_of_mut!((*nau8824).jdet_work));
    } else if (active_irq & NAU8824_KEY_SHORT_PRESS_IRQ) != 0 {
        let mut key_status: c_uint = 0;

        regmap_read(regmap, NAU8824_REG_CLEAR_INT_REG, &mut key_status);

        /* lower 8 bits of the register are for pressed keys */
        let button_pressed = nau8824_button_decode(key_status as c_int);

        event |= button_pressed;
        dev_dbg((*nau8824).dev, b"button %x pressed\n\0".as_ptr() as *const c_char, event);
        event_mask |= NAU8824_BUTTONS;
        clear_irq = NAU8824_KEY_SHORT_PRESS_IRQ;
    } else if (active_irq & NAU8824_KEY_RELEASE_IRQ) != 0 {
        event_mask = NAU8824_BUTTONS;
        clear_irq = NAU8824_KEY_RELEASE_IRQ;
    } else if (active_irq & NAU8824_JACK_INSERTION_DETECTED) != 0 {
        /* Turn off insertion interruption at manual mode */
        regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING, NAU8824_IRQ_INSERT_DIS, NAU8824_IRQ_INSERT_DIS);
        regmap_update_bits(regmap, NAU8824_REG_INTERRUPT_SETTING_1, NAU8824_IRQ_INSERT_EN, 0);
        /* detect microphone and jack type */
        cancel_work_sync(ptr::addr_of_mut!((*nau8824).jdet_work));
        schedule_work(ptr::addr_of_mut!((*nau8824).jdet_work));

        /* Enable interruption for jack type detection at audo
         * mode which can detect microphone and jack type.
         */
        nau8824_setup_auto_irq(nau8824);
    }

    if clear_irq == 0 {
        clear_irq = active_irq;
    }
    /* clears the rightmost interruption */
    regmap_write(regmap, NAU8824_REG_CLEAR_INT_REG, clear_irq);

    if event_mask != 0 {
        snd_soc_jack_report((*nau8824).jack, event, event_mask);
    }

    IRQ_HANDLED
}

unsafe fn nau8824_get_osr(nau8824: *mut nau8824, stream: c_int) -> *const nau8824_osr_attr {
    let mut osr: c_uint = 0;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_read((*nau8824).regmap, NAU8824_REG_DAC_FILTER_CTRL_1, &mut osr);
        osr &= NAU8824_DAC_OVERSAMPLE_MASK;
        if osr as usize >= osr_dac_sel.len() {
            return ptr::null();
        }
        &osr_dac_sel[osr as usize] as *const nau8824_osr_attr
    } else {
        regmap_read((*nau8824).regmap, NAU8824_REG_ADC_FILTER_CTRL, &mut osr);
        osr &= NAU8824_ADC_SYNC_DOWN_MASK;
        if osr as usize >= osr_adc_sel.len() {
            return ptr::null();
        }
        &osr_adc_sel[osr as usize] as *const nau8824_osr_attr
    }
}

unsafe extern "C" fn nau8824_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let nau8824 = snd_soc_component_get_drvdata(component);
    let osr = nau8824_get_osr(nau8824, (*substream).stream);

    if osr.is_null() || (*osr).osr == 0 {
        return -EINVAL;
    }

    snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 0, CLK_DA_AD_MAX / (*osr).osr)
}

unsafe extern "C" fn nau8824_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let nau8824 = snd_soc_component_get_drvdata(component);
    let mut val_len: c_uint = 0;
    let mut ctrl_val: c_uint = 0;
    let bclk_fs: c_uint;
    let bclk_div: c_uint;
    let osr: *const nau8824_osr_attr;
    let mut err: c_int = -EINVAL;

    nau8824_sema_acquire(nau8824, HZ);

    /* CLK_DAC or CLK_ADC = OSR * FS
     * DAC or ADC clock frequency is defined as Over Sampling Rate (OSR)
     * multiplied by the audio sample rate (Fs). Note that the OSR and Fs
     * values must be selected such that the maximum frequency is less
     * than 6.144 MHz.
     */
    (*nau8824).fs = params_rate(params);
    osr = nau8824_get_osr(nau8824, (*substream).stream);
    if osr.is_null() || (*osr).osr == 0 {
        nau8824_sema_release(nau8824);
        return err;
    }
    if (*nau8824).fs.wrapping_mul((*osr).osr) > CLK_DA_AD_MAX {
        nau8824_sema_release(nau8824);
        return err;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*nau8824).regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_DAC_SRC_MASK, (*osr).clk_src << NAU8824_CLK_DAC_SRC_SFT);
    } else {
        regmap_update_bits((*nau8824).regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_ADC_SRC_MASK, (*osr).clk_src << NAU8824_CLK_ADC_SRC_SFT);
    }

    /* make BCLK and LRC divde configuration if the codec as master. */
    regmap_read((*nau8824).regmap, NAU8824_REG_PORT0_I2S_PCM_CTRL_2, &mut ctrl_val);
    if (ctrl_val & NAU8824_I2S_MS_MASTER) != 0 {
        /* get the bclk and fs ratio */
        bclk_fs = snd_soc_params_to_bclk(params) / (*nau8824).fs;
        if bclk_fs <= 32 {
            bclk_div = 0x3;
        } else if bclk_fs <= 64 {
            bclk_div = 0x2;
        } else if bclk_fs <= 128 {
            bclk_div = 0x1;
        } else if bclk_fs <= 256 {
            bclk_div = 0;
        } else {
            nau8824_sema_release(nau8824);
            return err;
        }
        regmap_update_bits((*nau8824).regmap, NAU8824_REG_PORT0_I2S_PCM_CTRL_2,
            NAU8824_I2S_LRC_DIV_MASK | NAU8824_I2S_BLK_DIV_MASK,
            (bclk_div << NAU8824_I2S_LRC_DIV_SFT) | bclk_div);
    }

    match params_width(params) {
        16 => val_len |= NAU8824_I2S_DL_16,
        20 => val_len |= NAU8824_I2S_DL_20,
        24 => val_len |= NAU8824_I2S_DL_24,
        32 => val_len |= NAU8824_I2S_DL_32,
        _ => {
            nau8824_sema_release(nau8824);
            return err;
        }
    }

    regmap_update_bits((*nau8824).regmap, NAU8824_REG_PORT0_I2S_PCM_CTRL_1, NAU8824_I2S_DL_MASK, val_len);
    err = 0;

    nau8824_sema_release(nau8824);
    err
}

unsafe extern "C" fn nau8824_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let nau8824 = snd_soc_component_get_drvdata(component);
    let mut ctrl1_val: c_uint = 0;
    let mut ctrl2_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => ctrl2_val |= NAU8824_I2S_MS_MASTER,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => ctrl1_val |= NAU8824_I2S_BP_INV,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl1_val |= NAU8824_I2S_DF_I2S,
        SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= NAU8824_I2S_DF_LEFT,
        SND_SOC_DAIFMT_RIGHT_J => ctrl1_val |= NAU8824_I2S_DF_RIGTH,
        SND_SOC_DAIFMT_DSP_A => ctrl1_val |= NAU8824_I2S_DF_PCM_AB,
        SND_SOC_DAIFMT_DSP_B => {
            ctrl1_val |= NAU8824_I2S_DF_PCM_AB;
            ctrl1_val |= NAU8824_I2S_PCMB_EN;
        }
        _ => return -EINVAL,
    }

    nau8824_sema_acquire(nau8824, HZ);

    regmap_update_bits((*nau8824).regmap, NAU8824_REG_PORT0_I2S_PCM_CTRL_1,
        NAU8824_I2S_DF_MASK | NAU8824_I2S_BP_MASK | NAU8824_I2S_PCMB_EN, ctrl1_val);
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_PORT0_I2S_PCM_CTRL_2, NAU8824_I2S_MS_MASK, ctrl2_val);

    nau8824_sema_release(nau8824);

    0
}

/**
 * nau8824_set_tdm_slot - configure DAI TDM.
 * @dai: DAI
 * @tx_mask: Bitmask representing active TX slots. Ex.
 *                 0xf for normal 4 channel TDM.
 *                 0xf0 for shifted 4 channel TDM
 * @rx_mask: Bitmask [0:1] representing active DACR RX slots.
 *                 Bitmask [2:3] representing active DACL RX slots.
 *                 00=CH0,01=CH1,10=CH2,11=CH3. Ex.
 *                 0xf for DACL/R selecting TDM CH3.
 *                 0xf0 for DACL/R selecting shifted TDM CH3.
 * @slots: Number of slots in use.
 * @slot_width: Width in bits for each slot.
 *
 * Configures a DAI for TDM operation. Only support 4 slots TDM.
 */
unsafe extern "C" fn nau8824_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let nau8824 = snd_soc_component_get_drvdata(component);
    let mut tslot_l: c_uint = 0;
    let mut ctrl_val: c_uint = 0;

    if slots > 4 || ((tx_mask & 0xf0) != 0 && (tx_mask & 0xf) != 0) ||
        ((rx_mask & 0xf0) != 0 && (rx_mask & 0xf) != 0) ||
        ((rx_mask & 0xf0) != 0 && (tx_mask & 0xf) != 0) ||
        ((rx_mask & 0xf) != 0 && (tx_mask & 0xf0) != 0) {
        return -EINVAL;
    }

    ctrl_val |= NAU8824_TDM_MODE | NAU8824_TDM_OFFSET_EN;
    if (tx_mask & 0xf0) != 0 {
        tslot_l = (4 * slot_width) as c_uint;
        ctrl_val |= tx_mask >> 4;
    } else {
        ctrl_val |= tx_mask;
    }
    if (rx_mask & 0xf0) != 0 {
        ctrl_val |= (rx_mask >> 4) << NAU8824_TDM_DACR_RX_SFT;
    } else {
        ctrl_val |= rx_mask << NAU8824_TDM_DACR_RX_SFT;
    }

    regmap_update_bits((*nau8824).regmap, NAU8824_REG_TDM_CTRL,
        NAU8824_TDM_MODE | NAU8824_TDM_OFFSET_EN | NAU8824_TDM_DACL_RX_MASK | NAU8824_TDM_DACR_RX_MASK | NAU8824_TDM_TX_MASK,
        ctrl_val);
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_PORT0_LEFT_TIME_SLOT, NAU8824_TSLOT_L_MASK, tslot_l);

    0
}

/**
 * nau8824_calc_fll_param - Calculate FLL parameters.
 * @fll_in: external clock provided to codec.
 * @fs: sampling rate.
 * @fll_param: Pointer to structure of FLL parameters.
 *
 * Calculate FLL parameters to configure codec.
 *
 * Returns 0 for success or negative error code.
 */
unsafe fn nau8824_calc_fll_param(fll_in: c_uint, fs: c_uint, fll_param: *mut nau8824_fll) -> c_int {
    let mut fvco: u64_;
    let mut fvco_max: u64_;
    let mut fref: c_uint = 0;
    let mut i: usize;
    let mut fvco_sel: usize;

    /* Ensure the reference clock frequency (FREF) is <= 13.5MHz by dividing
     * freq_in by 1, 2, 4, or 8 using FLL pre-scalar.
     * FREF = freq_in / NAU8824_FLL_REF_DIV_MASK
     */
    i = 0;
    while i < fll_pre_scalar.len() {
        fref = fll_in / fll_pre_scalar[i].param;
        if fref <= NAU_FREF_MAX {
            break;
        }
        i += 1;
    }
    if i == fll_pre_scalar.len() {
        return -EINVAL;
    }
    (*fll_param).clk_ref_div = fll_pre_scalar[i].val;

    /* Choose the FLL ratio based on FREF */
    i = 0;
    while i < fll_ratio.len() {
        if fref >= fll_ratio[i].param {
            break;
        }
        i += 1;
    }
    if i == fll_ratio.len() {
        return -EINVAL;
    }
    (*fll_param).ratio = fll_ratio[i].val;

    /* Calculate the frequency of DCO (FDCO) given freq_out = 256 * Fs.
     * FDCO must be within the 90MHz - 124MHz or the FFL cannot be
     * guaranteed across the full range of operation.
     * FDCO = freq_out * 2 * mclk_src_scaling
     */
    fvco_max = 0;
    fvco_sel = mclk_src_scaling.len();
    i = 0;
    while i < mclk_src_scaling.len() {
        fvco = 256u64.wrapping_mul(fs as u64).wrapping_mul(2).wrapping_mul(mclk_src_scaling[i].param as u64);
        if fvco > NAU_FVCO_MIN && fvco < NAU_FVCO_MAX && fvco_max < fvco {
            fvco_max = fvco;
            fvco_sel = i;
        }
        i += 1;
    }
    if mclk_src_scaling.len() == fvco_sel {
        return -EINVAL;
    }
    (*fll_param).mclk_src = mclk_src_scaling[fvco_sel].val;

    /* Calculate the FLL 10-bit integer input and the FLL 16-bit fractional
     * input based on FDCO, FREF and FLL ratio.
     */
    fvco = (fvco_max << 16) / (fref as u64 * (*fll_param).ratio as u64);
    (*fll_param).fll_int = ((fvco >> 16) & 0x3FF) as c_uint;
    (*fll_param).fll_frac = (fvco & 0xFFFF) as c_uint;
    0
}

unsafe fn nau8824_fll_apply(regmap: *mut regmap, fll_param: *mut nau8824_fll) {
    regmap_update_bits(regmap, NAU8824_REG_CLK_DIVIDER,
        NAU8824_CLK_SRC_MASK | NAU8824_CLK_MCLK_SRC_MASK,
        NAU8824_CLK_SRC_MCLK | (*fll_param).mclk_src);
    regmap_update_bits(regmap, NAU8824_REG_FLL1, NAU8824_FLL_RATIO_MASK, (*fll_param).ratio);
    /* FLL 16-bit fractional input */
    regmap_write(regmap, NAU8824_REG_FLL2, (*fll_param).fll_frac);
    /* FLL 10-bit integer input */
    regmap_update_bits(regmap, NAU8824_REG_FLL3, NAU8824_FLL_INTEGER_MASK, (*fll_param).fll_int);
    /* FLL pre-scaler */
    regmap_update_bits(regmap, NAU8824_REG_FLL4, NAU8824_FLL_REF_DIV_MASK, (*fll_param).clk_ref_div << NAU8824_FLL_REF_DIV_SFT);
    /* select divided VCO input */
    regmap_update_bits(regmap, NAU8824_REG_FLL5, NAU8824_FLL_CLK_SW_MASK, NAU8824_FLL_CLK_SW_REF);
    /* Disable free-running mode */
    regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_DCO_EN, 0);
    if (*fll_param).fll_frac != 0 {
        regmap_update_bits(regmap, NAU8824_REG_FLL5,
            NAU8824_FLL_PDB_DAC_EN | NAU8824_FLL_LOOP_FTR_EN | NAU8824_FLL_FTR_SW_MASK,
            NAU8824_FLL_PDB_DAC_EN | NAU8824_FLL_LOOP_FTR_EN | NAU8824_FLL_FTR_SW_FILTER);
        regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_SDM_EN, NAU8824_SDM_EN);
    } else {
        regmap_update_bits(regmap, NAU8824_REG_FLL5,
            NAU8824_FLL_PDB_DAC_EN | NAU8824_FLL_LOOP_FTR_EN | NAU8824_FLL_FTR_SW_MASK,
            NAU8824_FLL_FTR_SW_ACCU);
        regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_SDM_EN, 0);
    }
}

/* freq_out must be 256*Fs in order to achieve the best performance */
unsafe extern "C" fn nau8824_set_pll(component: *mut snd_soc_component, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);
    let mut fll_param = nau8824_fll { mclk_src: 0, ratio: 0, fll_frac: 0, fll_int: 0, clk_ref_div: 0 };
    let fs = freq_out / 256;
    let ret = nau8824_calc_fll_param(freq_in, fs, &mut fll_param);
    if ret < 0 {
        dev_err((*nau8824).dev, b"Unsupported input clock %d\n\0".as_ptr() as *const c_char, freq_in);
        return ret;
    }
    dev_dbg((*nau8824).dev, b"mclk_src=%x ratio=%x fll_frac=%x fll_int=%x clk_ref_div=%x\n\0".as_ptr() as *const c_char,
        fll_param.mclk_src, fll_param.ratio, fll_param.fll_frac, fll_param.fll_int, fll_param.clk_ref_div);

    nau8824_fll_apply((*nau8824).regmap, &mut fll_param);
    mdelay(2);
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_SRC_MASK, NAU8824_CLK_SRC_VCO);

    0
}

unsafe fn nau8824_config_sysclk(nau8824: *mut nau8824, clk_id: c_int, freq: c_uint) -> c_int {
    let regmap = (*nau8824).regmap;

    match clk_id {
        NAU8824_CLK_DIS => {
            regmap_update_bits(regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_SRC_MASK, NAU8824_CLK_SRC_MCLK);
            regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_DCO_EN, 0);
        }
        NAU8824_CLK_MCLK => {
            nau8824_sema_acquire(nau8824, HZ);
            regmap_update_bits(regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_SRC_MASK, NAU8824_CLK_SRC_MCLK);
            regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_DCO_EN, 0);
            nau8824_sema_release(nau8824);
        }
        NAU8824_CLK_INTERNAL => {
            regmap_update_bits(regmap, NAU8824_REG_FLL6, NAU8824_DCO_EN, NAU8824_DCO_EN);
            regmap_update_bits(regmap, NAU8824_REG_CLK_DIVIDER, NAU8824_CLK_SRC_MASK, NAU8824_CLK_SRC_VCO);
        }
        NAU8824_CLK_FLL_MCLK => {
            nau8824_sema_acquire(nau8824, HZ);
            regmap_update_bits(regmap, NAU8824_REG_FLL3, NAU8824_FLL_CLK_SRC_MASK, NAU8824_FLL_CLK_SRC_MCLK);
            nau8824_sema_release(nau8824);
        }
        NAU8824_CLK_FLL_BLK => {
            nau8824_sema_acquire(nau8824, HZ);
            regmap_update_bits(regmap, NAU8824_REG_FLL3, NAU8824_FLL_CLK_SRC_MASK, NAU8824_FLL_CLK_SRC_BLK);
            nau8824_sema_release(nau8824);
        }
        NAU8824_CLK_FLL_FS => {
            nau8824_sema_acquire(nau8824, HZ);
            regmap_update_bits(regmap, NAU8824_REG_FLL3, NAU8824_FLL_CLK_SRC_MASK, NAU8824_FLL_CLK_SRC_FS);
            nau8824_sema_release(nau8824);
        }
        _ => {
            dev_err((*nau8824).dev, b"Invalid clock id (%d)\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }
    }

    dev_dbg((*nau8824).dev, b"Sysclk is %dHz and clock id is %d\n\0".as_ptr() as *const c_char, freq, clk_id);

    0
}

unsafe extern "C" fn nau8824_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);
    nau8824_config_sysclk(nau8824, clk_id, freq)
}

unsafe fn nau8824_resume_setup(nau8824: *mut nau8824) {
    nau8824_config_sysclk(nau8824, NAU8824_CLK_DIS, 0);
    if (*nau8824).irq != 0 {
        /* Clear all interruption status */
        nau8824_int_status_clear_all((*nau8824).regmap);
        /* Enable jack detection at sleep mode, insertion detection,
         * and ejection detection.
         */
        regmap_update_bits((*nau8824).regmap, NAU8824_REG_ENA_CTRL, NAU8824_JD_SLEEP_MODE, NAU8824_JD_SLEEP_MODE);
        regmap_update_bits((*nau8824).regmap, NAU8824_REG_INTERRUPT_SETTING_1,
            NAU8824_IRQ_EJECT_EN | NAU8824_IRQ_INSERT_EN, NAU8824_IRQ_EJECT_EN | NAU8824_IRQ_INSERT_EN);
        regmap_update_bits((*nau8824).regmap, NAU8824_REG_INTERRUPT_SETTING,
            NAU8824_IRQ_EJECT_DIS | NAU8824_IRQ_INSERT_DIS, 0);
    }
}

unsafe extern "C" fn nau8824_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level((*nau8824).dapm) == SND_SOC_BIAS_OFF {
                /* Setup codec configuration after resume */
                nau8824_resume_setup(nau8824);
            }
        }
        SND_SOC_BIAS_OFF => {
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_INTERRUPT_SETTING, 0x3ff, 0x3ff);
            regmap_update_bits((*nau8824).regmap, NAU8824_REG_INTERRUPT_SETTING_1,
                NAU8824_IRQ_EJECT_EN | NAU8824_IRQ_INSERT_EN, 0);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn nau8824_component_probe(component: *mut snd_soc_component) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);

    (*nau8824).dapm = dapm;

    0
}

unsafe extern "C" fn nau8824_suspend(component: *mut snd_soc_component) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);

    if (*nau8824).irq != 0 {
        disable_irq((*nau8824).irq);
        snd_soc_dapm_force_bias_level((*nau8824).dapm, SND_SOC_BIAS_OFF);
    }
    regcache_cache_only((*nau8824).regmap, true);
    regcache_mark_dirty((*nau8824).regmap);

    0
}

unsafe extern "C" fn nau8824_resume(component: *mut snd_soc_component) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);

    regcache_cache_only((*nau8824).regmap, false);
    regcache_sync((*nau8824).regmap);
    if (*nau8824).irq != 0 {
        /* Hold semaphore to postpone playback happening
         * until jack detection done.
         */
        (*nau8824).resume_lock = true;
        let ret = nau8824_sema_acquire(nau8824, 0);
        if ret != 0 {
            (*nau8824).resume_lock = false;
        }
        enable_irq((*nau8824).irq);
    }

    0
}

/**
 * nau8824_enable_jack_detect - Specify a jack for event reporting
 *
 * @component:  component to register the jack with
 * @jack: jack to use to report headset and button events on
 *
 * After this function has been called the headset insert/remove and button
 * events will be routed to the given jack.  Jack can be null to stop
 * reporting.
 */
#[no_mangle]
pub unsafe extern "C" fn nau8824_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int {
    let nau8824 = snd_soc_component_get_drvdata(component);
    (*nau8824).jack = jack;
    /* Initiate jack detection work queue */
    INIT_WORK(ptr::addr_of_mut!((*nau8824).jdet_work), nau8824_jdet_work);
    let ret = devm_request_threaded_irq((*nau8824).dev, (*nau8824).irq, ptr::null(),
        nau8824_interrupt, IRQF_TRIGGER_LOW | IRQF_ONESHOT,
        b"nau8824\0".as_ptr() as *const c_char, nau8824 as *mut c_void);
    if ret != 0 {
        dev_err((*nau8824).dev, b"Cannot request irq %d (%d)\n\0".as_ptr() as *const c_char, (*nau8824).irq, ret);
    }

    ret
}
/* EXPORT_SYMBOL_GPL(nau8824_enable_jack_detect); */

unsafe fn nau8824_reset_chip(regmap: *mut regmap) {
    regmap_write(regmap, NAU8824_REG_RESET, 0x00);
    regmap_write(regmap, NAU8824_REG_RESET, 0x00);
}

unsafe fn nau8824_setup_buttons(nau8824: *mut nau8824) {
    let regmap = (*nau8824).regmap;

    regmap_update_bits(regmap, NAU8824_REG_SAR_ADC, NAU8824_SAR_TRACKING_GAIN_MASK, (*nau8824).sar_voltage << NAU8824_SAR_TRACKING_GAIN_SFT);
    regmap_update_bits(regmap, NAU8824_REG_SAR_ADC, NAU8824_SAR_COMPARE_TIME_MASK, (*nau8824).sar_compare_time << NAU8824_SAR_COMPARE_TIME_SFT);
    regmap_update_bits(regmap, NAU8824_REG_SAR_ADC, NAU8824_SAR_SAMPLING_TIME_MASK, (*nau8824).sar_sampling_time << NAU8824_SAR_SAMPLING_TIME_SFT);

    regmap_update_bits(regmap, NAU8824_REG_VDET_COEFFICIENT, NAU8824_LEVELS_NR_MASK, ((*nau8824).sar_threshold_num - 1) << NAU8824_LEVELS_NR_SFT);
    regmap_update_bits(regmap, NAU8824_REG_VDET_COEFFICIENT, NAU8824_HYSTERESIS_MASK, (*nau8824).sar_hysteresis << NAU8824_HYSTERESIS_SFT);
    regmap_update_bits(regmap, NAU8824_REG_VDET_COEFFICIENT, NAU8824_SHORTKEY_DEBOUNCE_MASK, (*nau8824).key_debounce << NAU8824_SHORTKEY_DEBOUNCE_SFT);

    regmap_write(regmap, NAU8824_REG_VDET_THRESHOLD_1, ((*nau8824).sar_threshold[0] << 8) | (*nau8824).sar_threshold[1]);
    regmap_write(regmap, NAU8824_REG_VDET_THRESHOLD_2, ((*nau8824).sar_threshold[2] << 8) | (*nau8824).sar_threshold[3]);
    regmap_write(regmap, NAU8824_REG_VDET_THRESHOLD_3, ((*nau8824).sar_threshold[4] << 8) | (*nau8824).sar_threshold[5]);
    regmap_write(regmap, NAU8824_REG_VDET_THRESHOLD_4, ((*nau8824).sar_threshold[6] << 8) | (*nau8824).sar_threshold[7]);
}

unsafe fn nau8824_init_regs(nau8824: *mut nau8824) {
    let regmap = (*nau8824).regmap;

    /* Enable Bias/VMID/VMID Tieoff */
    regmap_update_bits(regmap, NAU8824_REG_BIAS_ADJ,
        NAU8824_VMID | NAU8824_VMID_SEL_MASK,
        NAU8824_VMID | ((*nau8824).vref_impedance << NAU8824_VMID_SEL_SFT));
    regmap_update_bits(regmap, NAU8824_REG_BOOST, NAU8824_GLOBAL_BIAS_EN, NAU8824_GLOBAL_BIAS_EN);
    mdelay(2);
    regmap_update_bits(regmap, NAU8824_REG_MIC_BIAS, NAU8824_MICBIAS_VOLTAGE_MASK, (*nau8824).micbias_voltage);
    /* Disable Boost Driver, Automatic Short circuit protection enable */
    regmap_update_bits(regmap, NAU8824_REG_BOOST,
        NAU8824_PRECHARGE_DIS | NAU8824_HP_BOOST_DIS | NAU8824_HP_BOOST_G_DIS | NAU8824_SHORT_SHUTDOWN_EN,
        NAU8824_PRECHARGE_DIS | NAU8824_HP_BOOST_DIS | NAU8824_HP_BOOST_G_DIS | NAU8824_SHORT_SHUTDOWN_EN);
    /* Scaling for ADC and DAC clock */
    regmap_update_bits(regmap, NAU8824_REG_CLK_DIVIDER,
        NAU8824_CLK_ADC_SRC_MASK | NAU8824_CLK_DAC_SRC_MASK,
        (0x1 << NAU8824_CLK_ADC_SRC_SFT) | (0x1 << NAU8824_CLK_DAC_SRC_SFT));
    regmap_update_bits(regmap, NAU8824_REG_DAC_MUTE_CTRL, NAU8824_DAC_ZC_EN, NAU8824_DAC_ZC_EN);
    regmap_update_bits(regmap, NAU8824_REG_ENA_CTRL,
        NAU8824_DAC_CH1_EN | NAU8824_DAC_CH0_EN | NAU8824_ADC_CH0_EN | NAU8824_ADC_CH1_EN | NAU8824_ADC_CH2_EN | NAU8824_ADC_CH3_EN,
        NAU8824_DAC_CH1_EN | NAU8824_DAC_CH0_EN | NAU8824_ADC_CH0_EN | NAU8824_ADC_CH1_EN | NAU8824_ADC_CH2_EN | NAU8824_ADC_CH3_EN);
    regmap_update_bits(regmap, NAU8824_REG_CLK_GATING_ENA,
        NAU8824_CLK_ADC_CH23_EN | NAU8824_CLK_ADC_CH01_EN | NAU8824_CLK_DAC_CH1_EN | NAU8824_CLK_DAC_CH0_EN | NAU8824_CLK_I2S_EN | NAU8824_CLK_GAIN_EN | NAU8824_CLK_SAR_EN | NAU8824_CLK_DMIC_CH23_EN,
        NAU8824_CLK_ADC_CH23_EN | NAU8824_CLK_ADC_CH01_EN | NAU8824_CLK_DAC_CH1_EN | NAU8824_CLK_DAC_CH0_EN | NAU8824_CLK_I2S_EN | NAU8824_CLK_GAIN_EN | NAU8824_CLK_SAR_EN | NAU8824_CLK_DMIC_CH23_EN);
    /* Class G timer 64ms */
    regmap_update_bits(regmap, NAU8824_REG_CLASSG, NAU8824_CLASSG_TIMER_MASK, 0x20 << NAU8824_CLASSG_TIMER_SFT);
    regmap_update_bits(regmap, NAU8824_REG_TRIM_SETTINGS, NAU8824_DRV_CURR_INC, NAU8824_DRV_CURR_INC);
    /* Disable DACR/L power */
    regmap_update_bits(regmap, NAU8824_REG_CHARGE_PUMP_CONTROL,
        NAU8824_SPKR_PULL_DOWN | NAU8824_SPKL_PULL_DOWN | NAU8824_POWER_DOWN_DACR | NAU8824_POWER_DOWN_DACL,
        NAU8824_SPKR_PULL_DOWN | NAU8824_SPKL_PULL_DOWN | NAU8824_POWER_DOWN_DACR | NAU8824_POWER_DOWN_DACL);
    /* Enable TESTDAC. This sets the analog DAC inputs to a '0' input
     * signal to avoid any glitches due to power up transients in both
     * the analog and digital DAC circuit.
     */
    regmap_update_bits(regmap, NAU8824_REG_ENABLE_LO, NAU8824_TEST_DAC_EN, NAU8824_TEST_DAC_EN);
    /* Config L/R channel */
    regmap_update_bits(regmap, NAU8824_REG_DAC_CH0_DGAIN_CTRL, NAU8824_DAC_CH0_SEL_MASK, NAU8824_DAC_CH0_SEL_I2S0);
    regmap_update_bits(regmap, NAU8824_REG_DAC_CH1_DGAIN_CTRL, NAU8824_DAC_CH1_SEL_MASK, NAU8824_DAC_CH1_SEL_I2S1);
    regmap_update_bits(regmap, NAU8824_REG_ENABLE_LO, NAU8824_DACR_HPR_EN | NAU8824_DACL_HPL_EN, NAU8824_DACR_HPR_EN | NAU8824_DACL_HPL_EN);
    /* Default oversampling/decimations settings are unusable
     * (audible hiss). Set it to something better.
     */
    regmap_update_bits(regmap, NAU8824_REG_ADC_FILTER_CTRL, NAU8824_ADC_SYNC_DOWN_MASK, NAU8824_ADC_SYNC_DOWN_64);
    regmap_update_bits(regmap, NAU8824_REG_DAC_FILTER_CTRL_1,
        NAU8824_DAC_CICCLP_OFF | NAU8824_DAC_OVERSAMPLE_MASK,
        NAU8824_DAC_CICCLP_OFF | NAU8824_DAC_OVERSAMPLE_64);
    /* DAC clock delay 2ns, VREF */
    regmap_update_bits(regmap, NAU8824_REG_RDAC,
        NAU8824_RDAC_CLK_DELAY_MASK | NAU8824_RDAC_VREF_MASK,
        (0x2 << NAU8824_RDAC_CLK_DELAY_SFT) | (0x3 << NAU8824_RDAC_VREF_SFT));
    /* PGA input mode selection */
    regmap_update_bits(regmap, NAU8824_REG_FEPGA,
        NAU8824_FEPGA_MODEL_SHORT_EN | NAU8824_FEPGA_MODER_SHORT_EN,
        NAU8824_FEPGA_MODEL_SHORT_EN | NAU8824_FEPGA_MODER_SHORT_EN);
    /* Digital microphone control */
    regmap_update_bits(regmap, NAU8824_REG_ANALOG_CONTROL_1,
        NAU8824_DMIC_CLK_DRV_STRG | NAU8824_DMIC_CLK_SLEW_FAST,
        NAU8824_DMIC_CLK_DRV_STRG | NAU8824_DMIC_CLK_SLEW_FAST);
    regmap_update_bits(regmap, NAU8824_REG_JACK_DET_CTRL, NAU8824_JACK_LOGIC,
        /* jkdet_polarity - 1  is for active-low */
        if (*nau8824).jkdet_polarity != 0 { 0 } else { NAU8824_JACK_LOGIC });
    regmap_update_bits(regmap, NAU8824_REG_JACK_DET_CTRL, NAU8824_JACK_EJECT_DT_MASK,
        (*nau8824).jack_eject_debounce << NAU8824_JACK_EJECT_DT_SFT);
    if (*nau8824).sar_threshold_num != 0 {
        nau8824_setup_buttons(nau8824);
    }
}

unsafe fn nau8824_setup_irq(nau8824: *mut nau8824) -> c_int {
    /* Disable interruption before codec initiation done */
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_ENA_CTRL, NAU8824_JD_SLEEP_MODE, NAU8824_JD_SLEEP_MODE);
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_INTERRUPT_SETTING, 0x3ff, 0x3ff);
    regmap_update_bits((*nau8824).regmap, NAU8824_REG_INTERRUPT_SETTING_1,
        NAU8824_IRQ_EJECT_EN | NAU8824_IRQ_INSERT_EN, 0);

    0
}

unsafe fn nau8824_print_device_properties(nau8824: *mut nau8824) {
    let dev = (*nau8824).dev;
    let mut i: c_uint;

    dev_dbg(dev, b"jkdet-polarity:       %d\n\0".as_ptr() as *const c_char, (*nau8824).jkdet_polarity);
    dev_dbg(dev, b"micbias-voltage:      %d\n\0".as_ptr() as *const c_char, (*nau8824).micbias_voltage);
    dev_dbg(dev, b"vref-impedance:       %d\n\0".as_ptr() as *const c_char, (*nau8824).vref_impedance);

    dev_dbg(dev, b"sar-threshold-num:    %d\n\0".as_ptr() as *const c_char, (*nau8824).sar_threshold_num);
    i = 0;
    while i < (*nau8824).sar_threshold_num {
        dev_dbg(dev, b"sar-threshold[%d]=%x\n\0".as_ptr() as *const c_char, i, (*nau8824).sar_threshold[i as usize]);
        i += 1;
    }

    dev_dbg(dev, b"sar-hysteresis:       %d\n\0".as_ptr() as *const c_char, (*nau8824).sar_hysteresis);
    dev_dbg(dev, b"sar-voltage:          %d\n\0".as_ptr() as *const c_char, (*nau8824).sar_voltage);
    dev_dbg(dev, b"sar-compare-time:     %d\n\0".as_ptr() as *const c_char, (*nau8824).sar_compare_time);
    dev_dbg(dev, b"sar-sampling-time:    %d\n\0".as_ptr() as *const c_char, (*nau8824).sar_sampling_time);
    dev_dbg(dev, b"short-key-debounce:   %d\n\0".as_ptr() as *const c_char, (*nau8824).key_debounce);
    dev_dbg(dev, b"jack-eject-debounce:  %d\n\0".as_ptr() as *const c_char, (*nau8824).jack_eject_debounce);
}

unsafe fn nau8824_read_device_properties(dev: *mut device, nau8824: *mut nau8824) -> c_int {
    let mut ret: c_int;

    ret = device_property_read_u32(dev, b"nuvoton,jkdet-polarity\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).jkdet_polarity));
    if ret != 0 { (*nau8824).jkdet_polarity = 1; }
    ret = device_property_read_u32(dev, b"nuvoton,micbias-voltage\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).micbias_voltage));
    if ret != 0 { (*nau8824).micbias_voltage = 6; }
    ret = device_property_read_u32(dev, b"nuvoton,vref-impedance\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).vref_impedance));
    if ret != 0 { (*nau8824).vref_impedance = 2; }
    ret = device_property_read_u32(dev, b"nuvoton,sar-threshold-num\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).sar_threshold_num));
    if ret != 0 { (*nau8824).sar_threshold_num = 4; }
    ret = device_property_read_u32_array(dev, b"nuvoton,sar-threshold\0".as_ptr() as *const c_char, (*nau8824).sar_threshold.as_mut_ptr(), (*nau8824).sar_threshold_num);
    if ret != 0 {
        (*nau8824).sar_threshold[0] = 0x0a;
        (*nau8824).sar_threshold[1] = 0x14;
        (*nau8824).sar_threshold[2] = 0x26;
        (*nau8824).sar_threshold[3] = 0x73;
    }
    ret = device_property_read_u32(dev, b"nuvoton,sar-hysteresis\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).sar_hysteresis));
    if ret != 0 { (*nau8824).sar_hysteresis = 0; }
    ret = device_property_read_u32(dev, b"nuvoton,sar-voltage\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).sar_voltage));
    if ret != 0 { (*nau8824).sar_voltage = 6; }
    ret = device_property_read_u32(dev, b"nuvoton,sar-compare-time\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).sar_compare_time));
    if ret != 0 { (*nau8824).sar_compare_time = 1; }
    ret = device_property_read_u32(dev, b"nuvoton,sar-sampling-time\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).sar_sampling_time));
    if ret != 0 { (*nau8824).sar_sampling_time = 1; }
    ret = device_property_read_u32(dev, b"nuvoton,short-key-debounce\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).key_debounce));
    if ret != 0 { (*nau8824).key_debounce = 0; }
    ret = device_property_read_u32(dev, b"nuvoton,jack-eject-debounce\0".as_ptr() as *const c_char, ptr::addr_of_mut!((*nau8824).jack_eject_debounce));
    if ret != 0 { (*nau8824).jack_eject_debounce = 1; }

    (*nau8824).mclk = devm_clk_get_optional(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*nau8824).mclk as *const c_void) {
        return PTR_ERR((*nau8824).mclk as *const c_void);
    }

    0
}

/* Please keep this list alphabetically sorted */
/* nau8824_quirk_table:
 * Cyberbook T116 rugged tablet: DMI_EXACT_MATCH board vendor "Default string",
 * board name "Cherry Trail CR", product SKU "20170531", driver_data
 * NAU8824_JD_ACTIVE_HIGH | NAU8824_MONO_SPEAKER.
 * CUBE iwork8 Air: DMI_MATCH sys vendor "cube", product name "i1-TF", board
 * name "Cherry Trail CR", driver_data NAU8824_MONO_SPEAKER.
 * Pipo W2S: DMI_MATCH sys vendor "PIPO", product name "W2S", driver_data
 * NAU8824_MONO_SPEAKER.
 * Positivo CW14Q01P/K1424G/N14ZP74G: DMI_MATCH vendor "Positivo Tecnologia SA"
 * and corresponding board name, driver_data NAU8824_JD_ACTIVE_HIGH.
 */
static nau8824_quirk_table: [dmi_system_id; 1] = [
    dmi_system_id { driver_data: ptr::null_mut() },
];

unsafe fn nau8824_check_quirks() {
    let dmi_id: *const dmi_system_id;

    if quirk_override != -1 {
        nau8824_quirk = quirk_override;
        return;
    }

    dmi_id = dmi_first_match(nau8824_quirk_table.as_ptr());
    if !dmi_id.is_null() {
        nau8824_quirk = (*dmi_id).driver_data as c_ulong as c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nau8824_components() -> *const c_char {
    nau8824_check_quirks();

    if (nau8824_quirk & NAU8824_MONO_SPEAKER) != 0 {
        b"cfg-spk:1\0".as_ptr() as *const c_char
    } else {
        b"cfg-spk:2\0".as_ptr() as *const c_char
    }
}
/* EXPORT_SYMBOL_GPL(nau8824_components); */

unsafe extern "C" fn nau8824_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = ptr::addr_of_mut!((*i2c).dev);
    let mut nau8824 = dev_get_platdata(dev);
    let mut ret: c_int;
    let mut value: c_uint = 0;

    if nau8824.is_null() {
        nau8824 = devm_kzalloc(dev, core::mem::size_of::<nau8824>(), GFP_KERNEL) as *mut nau8824;
        if nau8824.is_null() {
            return -ENOMEM;
        }
        ret = nau8824_read_device_properties(dev, nau8824);
        if ret != 0 {
            return ret;
        }
    }
    i2c_set_clientdata(i2c, nau8824 as *mut c_void);

    (*nau8824).regmap = devm_regmap_init_i2c(i2c, ptr::null());
    if IS_ERR((*nau8824).regmap as *const c_void) {
        return PTR_ERR((*nau8824).regmap as *const c_void);
    }
    (*nau8824).resume_lock = false;
    (*nau8824).dev = dev;
    (*nau8824).irq = (*i2c).irq;
    sema_init(ptr::addr_of_mut!((*nau8824).jd_sem), 1);

    nau8824_check_quirks();

    if (nau8824_quirk & NAU8824_JD_ACTIVE_HIGH) != 0 {
        (*nau8824).jkdet_polarity = 0;
    }

    nau8824_print_device_properties(nau8824);

    ret = regmap_read((*nau8824).regmap, NAU8824_REG_I2C_DEVICE_ID, &mut value);
    if ret < 0 {
        dev_err(dev, b"Failed to read device id from the NAU8824: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    nau8824_reset_chip((*nau8824).regmap);
    nau8824_init_regs(nau8824);

    if (*i2c).irq != 0 {
        nau8824_setup_irq(nau8824);
    }

    devm_snd_soc_register_component(dev, ptr::null(), ptr::null_mut(), 1)
}

/* #ifdef CONFIG_OF
 * static const struct of_device_id nau8824_of_ids[] = {
 *      { .compatible = "nuvoton,nau8824", },
 *      {}
 * };
 * MODULE_DEVICE_TABLE(of, nau8824_of_ids);
 * #endif
 *
 * #ifdef CONFIG_ACPI
 * static const struct acpi_device_id nau8824_acpi_match[] = {
 *      { "10508824", 0 },
 *      {},
 * };
 * MODULE_DEVICE_TABLE(acpi, nau8824_acpi_match);
 * #endif
 *
 * static struct i2c_driver nau8824_i2c_driver = {
 *      .driver = {
 *          .name = "nau8824",
 *          .of_match_table = of_match_ptr(nau8824_of_ids),
 *          .acpi_match_table = ACPI_PTR(nau8824_acpi_match),
 *      },
 *      .probe = nau8824_i2c_probe,
 *      .id_table = nau8824_i2c_ids,
 * };
 * module_i2c_driver(nau8824_i2c_driver);
 *
 * MODULE_DESCRIPTION("ASoC NAU88L24 driver");
 * MODULE_AUTHOR("John Hsu <KCHSU0@nuvoton.com>");
 * MODULE_LICENSE("GPL v2");
 */

/* External constants/macros from nau8824.h and Linux/ALSA headers. */
extern "Rust" {
    static NAU8824_REG_ENA_CTRL: c_uint; static NAU8824_REG_FLL_VCO_RSV: c_uint; static NAU8824_REG_JACK_DET_CTRL: c_uint;
    static NAU8824_REG_INTERRUPT_SETTING_1: c_uint; static NAU8824_REG_IRQ: c_uint; static NAU8824_REG_CLEAR_INT_REG: c_uint;
    static NAU8824_REG_VDET_THRESHOLD_4: c_uint; static NAU8824_REG_GPIO_SEL: c_uint; static NAU8824_REG_PORT0_I2S_PCM_CTRL_1: c_uint;
    static NAU8824_REG_TDM_CTRL: c_uint; static NAU8824_REG_ADC_HPF_FILTER: c_uint; static NAU8824_REG_EQ4_EQ5: c_uint;
    static NAU8824_REG_ADC_CH0_DGAIN_CTRL: c_uint; static NAU8824_REG_ADC_TO_DAC_ST: c_uint; static NAU8824_REG_DRC_KNEE_IP12_ADC_CH01: c_uint;
    static NAU8824_REG_DRC_GAINL_ADC3: c_uint; static NAU8824_REG_DRC_KNEE_IP12_DAC: c_uint; static NAU8824_REG_DRC_GAIN_DAC_CH1: c_uint;
    static NAU8824_REG_CLASSG: c_uint; static NAU8824_REG_OTP_EFUSE: c_uint; static NAU8824_REG_OTPDOUT_1: c_uint;
    static NAU8824_REG_OTPDOUT_2: c_uint; static NAU8824_REG_I2C_TIMEOUT: c_uint; static NAU8824_REG_I2C_DEVICE_ID: c_uint;
    static NAU8824_REG_SAR_ADC_DATA_OUT: c_uint; static NAU8824_REG_BIAS_ADJ: c_uint; static NAU8824_REG_CLASSD_GAIN_2: c_uint;
    static NAU8824_REG_ANALOG_ADC_1: c_uint; static NAU8824_REG_ATT_PORT1: c_uint; static NAU8824_REG_POWER_UP_CONTROL: c_uint;
    static NAU8824_REG_CHARGE_PUMP_INPUT: c_uint; static NAU8824_REG_RESET: c_uint; static NAU8824_REG_DRC_KNEE_IP34_ADC_CH01: c_uint;
    static NAU8824_REG_DRC_SLOPE_ADC_CH01: c_uint; static NAU8824_REG_DRC_ATKDCY_ADC_CH01: c_uint; static NAU8824_REG_DRC_KNEE_IP12_ADC_CH23: c_uint;
    static NAU8824_REG_DRC_KNEE_IP34_ADC_CH23: c_uint; static NAU8824_REG_DRC_SLOPE_ADC_CH23: c_uint; static NAU8824_REG_DRC_ATKDCY_ADC_CH23: c_uint;
    static NAU8824_REG_DRC_ATKDCY_DAC: c_uint; static NAU8824_REG_CHARGE_PUMP_CONTROL: c_uint; static NAU8824_REG_DRC_GAINL_ADC0: c_uint;
    static NAU8824_REG_DRC_GAIN_DAC_CH0: c_uint; static NAU8824_REG_FLL1: c_uint; static NAU8824_REG_FLL6: c_uint;
    static NAU8824_REG_CLK_DIVIDER: c_uint; static NAU8824_REG_ENABLE_LO: c_uint; static NAU8824_REG_ANALOG_CONTROL_2: c_uint;
    static NAU8824_REG_SAR_ADC: c_uint; static NAU8824_REG_DAC_FILTER_CTRL_1: c_uint; static NAU8824_REG_ADC_FILTER_CTRL: c_uint;
    static NAU8824_REG_PORT0_I2S_PCM_CTRL_2: c_uint; static NAU8824_REG_PORT0_LEFT_TIME_SLOT: c_uint; static NAU8824_REG_FLL2: c_uint;
    static NAU8824_REG_FLL3: c_uint; static NAU8824_REG_FLL4: c_uint; static NAU8824_REG_FLL5: c_uint; static NAU8824_REG_DAC_MUTE_CTRL: c_uint;
    static NAU8824_REG_BOOST: c_uint; static NAU8824_REG_MIC_BIAS: c_uint; static NAU8824_REG_CLK_GATING_ENA: c_uint; static NAU8824_REG_TRIM_SETTINGS: c_uint;
    static NAU8824_REG_DAC_CH0_DGAIN_CTRL: c_uint; static NAU8824_REG_DAC_CH1_DGAIN_CTRL: c_uint; static NAU8824_REG_RDAC: c_uint;
    static NAU8824_REG_FEPGA: c_uint; static NAU8824_REG_ANALOG_CONTROL_1: c_uint; static NAU8824_REG_VDET_COEFFICIENT: c_uint;
    static NAU8824_REG_VDET_THRESHOLD_1: c_uint; static NAU8824_REG_VDET_THRESHOLD_2: c_uint; static NAU8824_REG_VDET_THRESHOLD_3: c_uint;
}

/* The original C file obtains these names from headers; this translation keeps
 * them as unresolved external Rust names so dependency resolution remains in the
 * surrounding repository pass.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
