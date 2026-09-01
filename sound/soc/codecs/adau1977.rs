// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAU1977/ADAU1978/ADAU1979 driver
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u64_ = u64;

const fn BIT(x: c_uint) -> c_uint {
    1u32 << x
}

const ADAU1977_REG_POWER: c_uint = 0x00;
const ADAU1977_REG_PLL: c_uint = 0x01;
const ADAU1977_REG_BOOST: c_uint = 0x02;
const ADAU1977_REG_MICBIAS: c_uint = 0x03;
const ADAU1977_REG_BLOCK_POWER_SAI: c_uint = 0x04;
const ADAU1977_REG_SAI_CTRL0: c_uint = 0x05;
const ADAU1977_REG_SAI_CTRL1: c_uint = 0x06;
const ADAU1977_REG_CMAP12: c_uint = 0x07;
const ADAU1977_REG_CMAP34: c_uint = 0x08;
const ADAU1977_REG_SAI_OVERTEMP: c_uint = 0x09;
const fn ADAU1977_REG_POST_ADC_GAIN(x: c_uint) -> c_uint {
    0x0a + x
}
const ADAU1977_REG_MISC_CONTROL: c_uint = 0x0e;
const ADAU1977_REG_DIAG_CONTROL: c_uint = 0x10;
const fn ADAU1977_REG_STATUS(x: c_uint) -> c_uint {
    0x11 + x
}
const ADAU1977_REG_DIAG_IRQ1: c_uint = 0x15;
const ADAU1977_REG_DIAG_IRQ2: c_uint = 0x16;
const ADAU1977_REG_ADJUST1: c_uint = 0x17;
const ADAU1977_REG_ADJUST2: c_uint = 0x18;
const ADAU1977_REG_ADC_CLIP: c_uint = 0x19;
const ADAU1977_REG_DC_HPF_CAL: c_uint = 0x1a;

const ADAU1977_POWER_RESET: c_uint = BIT(7);
const ADAU1977_POWER_PWUP: c_uint = BIT(0);

const ADAU1977_PLL_CLK_S: c_uint = BIT(4);
const ADAU1977_PLL_MCS_MASK: c_uint = 0x7;

const ADAU1977_MICBIAS_MB_VOLTS_MASK: c_uint = 0xf0;
const ADAU1977_MICBIAS_MB_VOLTS_OFFSET: c_uint = 4;

const ADAU1977_BLOCK_POWER_SAI_LR_POL: c_uint = BIT(7);
const ADAU1977_BLOCK_POWER_SAI_BCLK_EDGE: c_uint = BIT(6);
const ADAU1977_BLOCK_POWER_SAI_LDO_EN: c_uint = BIT(5);

const ADAU1977_SAI_CTRL0_FMT_MASK: c_uint = 0x3 << 6;
const ADAU1977_SAI_CTRL0_FMT_I2S: c_uint = 0x0 << 6;
const ADAU1977_SAI_CTRL0_FMT_LJ: c_uint = 0x1 << 6;
const ADAU1977_SAI_CTRL0_FMT_RJ_24BIT: c_uint = 0x2 << 6;
const ADAU1977_SAI_CTRL0_FMT_RJ_16BIT: c_uint = 0x3 << 6;

const ADAU1977_SAI_CTRL0_SAI_MASK: c_uint = 0x7 << 3;
const ADAU1977_SAI_CTRL0_SAI_I2S: c_uint = 0x0 << 3;
const ADAU1977_SAI_CTRL0_SAI_TDM_2: c_uint = 0x1 << 3;
const ADAU1977_SAI_CTRL0_SAI_TDM_4: c_uint = 0x2 << 3;
const ADAU1977_SAI_CTRL0_SAI_TDM_8: c_uint = 0x3 << 3;
const ADAU1977_SAI_CTRL0_SAI_TDM_16: c_uint = 0x4 << 3;

const ADAU1977_SAI_CTRL0_FS_MASK: c_uint = 0x7;
const ADAU1977_SAI_CTRL0_FS_8000_12000: c_uint = 0x0;
const ADAU1977_SAI_CTRL0_FS_16000_24000: c_uint = 0x1;
const ADAU1977_SAI_CTRL0_FS_32000_48000: c_uint = 0x2;
const ADAU1977_SAI_CTRL0_FS_64000_96000: c_uint = 0x3;
const ADAU1977_SAI_CTRL0_FS_128000_192000: c_uint = 0x4;

const ADAU1977_SAI_CTRL1_SLOT_WIDTH_MASK: c_uint = 0x3 << 5;
const ADAU1977_SAI_CTRL1_SLOT_WIDTH_32: c_uint = 0x0 << 5;
const ADAU1977_SAI_CTRL1_SLOT_WIDTH_24: c_uint = 0x1 << 5;
const ADAU1977_SAI_CTRL1_SLOT_WIDTH_16: c_uint = 0x2 << 5;
const ADAU1977_SAI_CTRL1_DATA_WIDTH_MASK: c_uint = 0x1 << 4;
const ADAU1977_SAI_CTRL1_DATA_WIDTH_16BIT: c_uint = 0x1 << 4;
const ADAU1977_SAI_CTRL1_DATA_WIDTH_24BIT: c_uint = 0x0 << 4;
const ADAU1977_SAI_CTRL1_LRCLK_PULSE: c_uint = BIT(3);
const ADAU1977_SAI_CTRL1_MSB: c_uint = BIT(2);
const ADAU1977_SAI_CTRL1_BCLKRATE_16: c_uint = 0x1 << 1;
const ADAU1977_SAI_CTRL1_BCLKRATE_32: c_uint = 0x0 << 1;
const ADAU1977_SAI_CTRL1_BCLKRATE_MASK: c_uint = 0x1 << 1;
const ADAU1977_SAI_CTRL1_MASTER: c_uint = BIT(0);

const fn ADAU1977_SAI_OVERTEMP_DRV_C(x: c_uint) -> c_uint {
    BIT(4 + x)
}
const ADAU1977_SAI_OVERTEMP_DRV_HIZ: c_uint = BIT(3);

const ADAU1977_MISC_CONTROL_SUM_MODE_MASK: c_uint = 0x3 << 6;
const ADAU1977_MISC_CONTROL_SUM_MODE_1CH: c_uint = 0x2 << 6;
const ADAU1977_MISC_CONTROL_SUM_MODE_2CH: c_uint = 0x1 << 6;
const ADAU1977_MISC_CONTROL_SUM_MODE_4CH: c_uint = 0x0 << 6;
const ADAU1977_MISC_CONTROL_MMUTE: c_uint = BIT(4);
const ADAU1977_MISC_CONTROL_DC_CAL: c_uint = BIT(0);

const ADAU1977_CHAN_MAP_SECOND_SLOT_OFFSET: c_uint = 4;
const ADAU1977_CHAN_MAP_FIRST_SLOT_OFFSET: c_uint = 0;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const ADAU1977_SYSCLK: c_int = 0;
const ADAU1977_SYSCLK_SRC_MCLK: c_int = 0;
const ADAU1977_SYSCLK_SRC_LRCLK: c_int = 1;
const ADAU1977: adau1977_type = 0;
const ADAU1977_MICBIAS_8V5: c_uint = 0;
const ADAU1977_MICBIAS_9V0: c_uint = 0;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64_ = 0;
const SNDRV_PCM_FMTBIT_S16_BE: u64_ = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64_ = 0;
const SNDRV_PCM_FMTBIT_S24_BE: u64_ = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64_ = 0;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_uint = 0;

type adau1977_sysclk_src = c_int;
type adau1977_type = c_int;
type snd_soc_bias_level = c_int;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator {
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
    runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
    mask: c_uint,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64_,
    sig_bits: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    max_register: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    cache_type: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
}

#[repr(C)]
struct adau1977 {
    regmap: *mut regmap,
    right_j: bool_,
    sysclk: c_uint,
    sysclk_src: adau1977_sysclk_src,
    reset_gpio: *mut gpio_desc,
    type_: adau1977_type,
    avdd_reg: *mut regulator,
    dvdd_reg: *mut regulator,
    constraints: snd_pcm_hw_constraint_list,
    dev: *mut device,
    switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    max_clock_provider_fs: c_uint,
    slot_width: c_uint,
    enabled: bool_,
    clock_provider: bool_,
}

extern "C" {
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut adau1977;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn __ffs(word: c_uint) -> c_uint;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_uint, min: c_uint, max: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_mask64(runtime: *mut snd_pcm_runtime, var: c_uint, mask: u64_) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget, num: c_uint) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_regulator_get_optional(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn ndelay(nsecs: c_uint);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

static adau1977_reg_defaults: [reg_default; 21] = [
    reg_default { reg: 0x00, def: 0x00 },
    reg_default { reg: 0x01, def: 0x41 },
    reg_default { reg: 0x02, def: 0x4a },
    reg_default { reg: 0x03, def: 0x7d },
    reg_default { reg: 0x04, def: 0x3d },
    reg_default { reg: 0x05, def: 0x02 },
    reg_default { reg: 0x06, def: 0x00 },
    reg_default { reg: 0x07, def: 0x10 },
    reg_default { reg: 0x08, def: 0x32 },
    reg_default { reg: 0x09, def: 0xf0 },
    reg_default { reg: 0x0a, def: 0xa0 },
    reg_default { reg: 0x0b, def: 0xa0 },
    reg_default { reg: 0x0c, def: 0xa0 },
    reg_default { reg: 0x0d, def: 0xa0 },
    reg_default { reg: 0x0e, def: 0x02 },
    reg_default { reg: 0x10, def: 0x0f },
    reg_default { reg: 0x15, def: 0x20 },
    reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x17, def: 0x00 },
    reg_default { reg: 0x18, def: 0x00 },
    reg_default { reg: 0x1a, def: 0x00 },
];

/* static const DECLARE_TLV_DB_MINMAX_MUTE(adau1977_adc_gain, -3562, 6000); */

/* SND_SOC_DAPM_* and SOC_* macro initializers require external kernel macro definitions. */
static adau1977_micbias_dapm_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget { _private: [] }];
static adau1977_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];
static adau1977_dapm_routes: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: b"ADC1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIN1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIN2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC3\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIN3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC4\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIN4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Vref\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Vref\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC3\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Vref\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC4\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Vref\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VREF\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Vref\0".as_ptr() as *const c_char },
];
static adau1977_snd_controls: [snd_kcontrol_new; 12] = [
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn adau1977_reset(adau1977: *mut adau1977) -> c_int {
    let ret: c_int;
    /*
     * The reset bit is obviously volatile, but we need to be able to cache
     * the other bits in the register, so we can't just mark the whole
     * register as volatile. Since this is the only place where we'll ever
     * touch the reset bit just bypass the cache for this operation.
     */
    regcache_cache_bypass((*adau1977).regmap, true);
    ret = regmap_write((*adau1977).regmap, ADAU1977_REG_POWER, ADAU1977_POWER_RESET);
    regcache_cache_bypass((*adau1977).regmap, false);
    ret
}

/*
 * Returns the appropriate setting for ths FS field in the CTRL0 register
 * depending on the rate.
 */
fn adau1977_lookup_fs(rate: c_uint) -> c_int {
    if rate >= 8000 && rate <= 12000 {
        ADAU1977_SAI_CTRL0_FS_8000_12000 as c_int
    } else if rate >= 16000 && rate <= 24000 {
        ADAU1977_SAI_CTRL0_FS_16000_24000 as c_int
    } else if rate >= 32000 && rate <= 48000 {
        ADAU1977_SAI_CTRL0_FS_32000_48000 as c_int
    } else if rate >= 64000 && rate <= 96000 {
        ADAU1977_SAI_CTRL0_FS_64000_96000 as c_int
    } else if rate >= 128000 && rate <= 192000 {
        ADAU1977_SAI_CTRL0_FS_128000_192000 as c_int
    } else {
        -EINVAL
    }
}

unsafe fn adau1977_lookup_mcs(adau1977: *mut adau1977, mut rate: c_uint, fs: c_uint) -> c_int {
    let mut mcs: c_uint;
    /*
     * rate = sysclk / (512 * mcs_lut[mcs]) * 2**fs
     * => mcs_lut[mcs] = sysclk / (512 * rate) * 2**fs
     * => mcs_lut[mcs] = sysclk / ((512 / 2**fs) * rate)
     */
    rate = rate.wrapping_mul(512 >> fs);
    if (*adau1977).sysclk % rate != 0 {
        return -EINVAL;
    }
    mcs = (*adau1977).sysclk / rate;
    /* The factors configured by MCS are 1, 2, 3, 4, 6 */
    if mcs < 1 || mcs > 6 || mcs == 5 {
        return -EINVAL;
    }
    mcs = mcs - 1;
    if mcs == 5 {
        mcs = 4;
    }
    mcs as c_int
}

unsafe extern "C" fn adau1977_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let adau1977 = snd_soc_component_get_drvdata(component);
    let rate = params_rate(params);
    let mut slot_width: c_uint;
    let mut ctrl0: c_uint;
    let mut ctrl0_mask: c_uint;
    let mut ctrl1: c_uint;
    let mut mcs: c_int;
    let fs: c_int;
    let mut ret: c_int;

    fs = adau1977_lookup_fs(rate);
    if fs < 0 {
        return fs;
    }
    if (*adau1977).sysclk_src == ADAU1977_SYSCLK_SRC_MCLK {
        mcs = adau1977_lookup_mcs(adau1977, rate, fs as c_uint);
        if mcs < 0 {
            return mcs;
        }
    } else {
        mcs = 0;
    }
    ctrl0_mask = ADAU1977_SAI_CTRL0_FS_MASK;
    ctrl0 = fs as c_uint;
    if (*adau1977).right_j {
        match params_width(params) {
            16 => ctrl0 |= ADAU1977_SAI_CTRL0_FMT_RJ_16BIT,
            24 => ctrl0 |= ADAU1977_SAI_CTRL0_FMT_RJ_24BIT,
            _ => return -EINVAL,
        }
        ctrl0_mask |= ADAU1977_SAI_CTRL0_FMT_MASK;
    }
    if (*adau1977).clock_provider {
        match params_width(params) {
            16 => {
                ctrl1 = ADAU1977_SAI_CTRL1_DATA_WIDTH_16BIT;
                slot_width = 16;
            }
            24 | 32 => {
                ctrl1 = ADAU1977_SAI_CTRL1_DATA_WIDTH_24BIT;
                slot_width = 32;
            }
            _ => return -EINVAL,
        }
        /* In TDM mode there is a fixed slot width */
        if (*adau1977).slot_width != 0 {
            slot_width = (*adau1977).slot_width;
        }
        if slot_width == 16 {
            ctrl1 |= ADAU1977_SAI_CTRL1_BCLKRATE_16;
        } else {
            ctrl1 |= ADAU1977_SAI_CTRL1_BCLKRATE_32;
        }
        ret = regmap_update_bits(
            (*adau1977).regmap,
            ADAU1977_REG_SAI_CTRL1,
            ADAU1977_SAI_CTRL1_DATA_WIDTH_MASK | ADAU1977_SAI_CTRL1_BCLKRATE_MASK,
            ctrl1,
        );
        if ret < 0 {
            return ret;
        }
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_SAI_CTRL0, ctrl0_mask, ctrl0);
    if ret < 0 {
        return ret;
    }
    regmap_update_bits((*adau1977).regmap, ADAU1977_REG_PLL, ADAU1977_PLL_MCS_MASK, mcs as c_uint)
}

unsafe fn adau1977_power_disable(adau1977: *mut adau1977) -> c_int {
    let mut ret: c_int = 0;
    if !(*adau1977).enabled {
        return 0;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_POWER, ADAU1977_POWER_PWUP, 0);
    if ret != 0 {
        return ret;
    }
    regcache_mark_dirty((*adau1977).regmap);
    gpiod_set_value_cansleep((*adau1977).reset_gpio, 0);
    regcache_cache_only((*adau1977).regmap, true);
    regulator_disable((*adau1977).avdd_reg);
    if !(*adau1977).dvdd_reg.is_null() {
        regulator_disable((*adau1977).dvdd_reg);
    }
    (*adau1977).enabled = false;
    0
}

unsafe fn adau1977_power_enable(adau1977: *mut adau1977) -> c_int {
    let mut val: c_uint = 0;
    let mut ret: c_int = 0;
    if (*adau1977).enabled {
        return 0;
    }
    ret = regulator_enable((*adau1977).avdd_reg);
    if ret != 0 {
        return ret;
    }
    if !(*adau1977).dvdd_reg.is_null() {
        ret = regulator_enable((*adau1977).dvdd_reg);
        if ret != 0 {
            regulator_disable((*adau1977).avdd_reg);
            return ret;
        }
    }
    gpiod_set_value_cansleep((*adau1977).reset_gpio, 1);
    regcache_cache_only((*adau1977).regmap, false);
    if let Some(switch_mode) = (*adau1977).switch_mode {
        switch_mode((*adau1977).dev);
    }
    ret = adau1977_reset(adau1977);
    if ret != 0 {
        if !(*adau1977).dvdd_reg.is_null() {
            regulator_disable((*adau1977).dvdd_reg);
        }
        regulator_disable((*adau1977).avdd_reg);
        return ret;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_POWER, ADAU1977_POWER_PWUP, ADAU1977_POWER_PWUP);
    if ret != 0 {
        if !(*adau1977).dvdd_reg.is_null() {
            regulator_disable((*adau1977).dvdd_reg);
        }
        regulator_disable((*adau1977).avdd_reg);
        return ret;
    }
    ret = regcache_sync((*adau1977).regmap);
    if ret != 0 {
        if !(*adau1977).dvdd_reg.is_null() {
            regulator_disable((*adau1977).dvdd_reg);
        }
        regulator_disable((*adau1977).avdd_reg);
        return ret;
    }
    /*
     * The PLL register is not affected by the software reset. It is
     * possible that the value of the register was changed to the
     * default value while we were in cache only mode. In this case
     * regcache_sync will skip over it and we have to manually sync
     * it.
     */
    ret = regmap_read((*adau1977).regmap, ADAU1977_REG_PLL, &mut val);
    if ret != 0 {
        if !(*adau1977).dvdd_reg.is_null() {
            regulator_disable((*adau1977).dvdd_reg);
        }
        regulator_disable((*adau1977).avdd_reg);
        return ret;
    }
    if val == 0x41 {
        regcache_cache_bypass((*adau1977).regmap, true);
        ret = regmap_write((*adau1977).regmap, ADAU1977_REG_PLL, 0x41);
        if ret != 0 {
            if !(*adau1977).dvdd_reg.is_null() {
                regulator_disable((*adau1977).dvdd_reg);
            }
            regulator_disable((*adau1977).avdd_reg);
            return ret;
        }
        regcache_cache_bypass((*adau1977).regmap, false);
    }
    (*adau1977).enabled = true;
    ret
}

unsafe extern "C" fn adau1977_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = adau1977_power_enable(adau1977);
            }
        }
        SND_SOC_BIAS_OFF => ret = adau1977_power_disable(adau1977),
        _ => {}
    }
    ret
}

unsafe extern "C" fn adau1977_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, mut rx_mask: c_uint, slots: c_int, width: c_int) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata((*dai).component);
    let ctrl0: c_uint;
    let ctrl1: c_uint;
    let mut drv: c_uint;
    let mut slot: [c_uint; 4] = [0; 4];
    let mut i: c_uint;
    let mut ret: c_int;
    if slots == 0 {
        /* 0 = No fixed slot width */
        (*adau1977).slot_width = 0;
        (*adau1977).max_clock_provider_fs = 192000;
        return regmap_update_bits((*adau1977).regmap, ADAU1977_REG_SAI_CTRL0, ADAU1977_SAI_CTRL0_SAI_MASK, ADAU1977_SAI_CTRL0_SAI_I2S);
    }
    if rx_mask == 0 || tx_mask != 0 {
        return -EINVAL;
    }
    drv = 0;
    i = 0;
    while i < 4 {
        slot[i as usize] = __ffs(rx_mask);
        drv |= ADAU1977_SAI_OVERTEMP_DRV_C(i);
        rx_mask &= !(1 << slot[i as usize]);
        if slot[i as usize] >= slots as c_uint {
            return -EINVAL;
        }
        if rx_mask == 0 {
            break;
        }
        i += 1;
    }
    if rx_mask != 0 {
        return -EINVAL;
    }
    match width {
        16 => ctrl1 = ADAU1977_SAI_CTRL1_SLOT_WIDTH_16,
        24 => {
            /* We can only generate 16 bit or 32 bit wide slots */
            if (*adau1977).clock_provider {
                return -EINVAL;
            }
            ctrl1 = ADAU1977_SAI_CTRL1_SLOT_WIDTH_24;
        }
        32 => ctrl1 = ADAU1977_SAI_CTRL1_SLOT_WIDTH_32,
        _ => return -EINVAL,
    }
    match slots {
        2 => ctrl0 = ADAU1977_SAI_CTRL0_SAI_TDM_2,
        4 => ctrl0 = ADAU1977_SAI_CTRL0_SAI_TDM_4,
        8 => ctrl0 = ADAU1977_SAI_CTRL0_SAI_TDM_8,
        16 => ctrl0 = ADAU1977_SAI_CTRL0_SAI_TDM_16,
        _ => return -EINVAL,
    }
    ret = regmap_update_bits(
        (*adau1977).regmap,
        ADAU1977_REG_SAI_OVERTEMP,
        ADAU1977_SAI_OVERTEMP_DRV_C(0) | ADAU1977_SAI_OVERTEMP_DRV_C(1) | ADAU1977_SAI_OVERTEMP_DRV_C(2) | ADAU1977_SAI_OVERTEMP_DRV_C(3),
        drv,
    );
    if ret != 0 {
        return ret;
    }
    ret = regmap_write(
        (*adau1977).regmap,
        ADAU1977_REG_CMAP12,
        (slot[1] << ADAU1977_CHAN_MAP_SECOND_SLOT_OFFSET) | (slot[0] << ADAU1977_CHAN_MAP_FIRST_SLOT_OFFSET),
    );
    if ret != 0 {
        return ret;
    }
    ret = regmap_write(
        (*adau1977).regmap,
        ADAU1977_REG_CMAP34,
        (slot[3] << ADAU1977_CHAN_MAP_SECOND_SLOT_OFFSET) | (slot[2] << ADAU1977_CHAN_MAP_FIRST_SLOT_OFFSET),
    );
    if ret != 0 {
        return ret;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_SAI_CTRL0, ADAU1977_SAI_CTRL0_SAI_MASK, ctrl0);
    if ret != 0 {
        return ret;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_SAI_CTRL1, ADAU1977_SAI_CTRL1_SLOT_WIDTH_MASK, ctrl1);
    if ret != 0 {
        return ret;
    }
    (*adau1977).slot_width = width as c_uint;
    /* In clock provider mode the maximum bitclock is 24.576 MHz */
    (*adau1977).max_clock_provider_fs = core::cmp::min(192000u32, 24576000u32 / width as c_uint / slots as c_uint);
    0
}

unsafe extern "C" fn adau1977_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata((*dai).component);
    let val: c_uint = if mute != 0 { ADAU1977_MISC_CONTROL_MMUTE } else { 0 };
    regmap_update_bits((*adau1977).regmap, ADAU1977_REG_MISC_CONTROL, ADAU1977_MISC_CONTROL_MMUTE, val)
}

unsafe extern "C" fn adau1977_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata((*dai).component);
    let mut ctrl0: c_uint = 0;
    let mut ctrl1: c_uint = 0;
    let mut block_power: c_uint = 0;
    let mut invert_lrclk: bool_;
    let mut ret: c_int;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => (*adau1977).clock_provider = false,
        SND_SOC_DAIFMT_CBP_CFP => {
            ctrl1 |= ADAU1977_SAI_CTRL1_MASTER;
            (*adau1977).clock_provider = true;
        }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => invert_lrclk = false,
        SND_SOC_DAIFMT_IB_NF => {
            block_power |= ADAU1977_BLOCK_POWER_SAI_BCLK_EDGE;
            invert_lrclk = false;
        }
        SND_SOC_DAIFMT_NB_IF => invert_lrclk = true,
        SND_SOC_DAIFMT_IB_IF => {
            block_power |= ADAU1977_BLOCK_POWER_SAI_BCLK_EDGE;
            invert_lrclk = true;
        }
        _ => return -EINVAL,
    }
    (*adau1977).right_j = false;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl0 |= ADAU1977_SAI_CTRL0_FMT_I2S,
        SND_SOC_DAIFMT_LEFT_J => {
            ctrl0 |= ADAU1977_SAI_CTRL0_FMT_LJ;
            invert_lrclk = !invert_lrclk;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            ctrl0 |= ADAU1977_SAI_CTRL0_FMT_RJ_24BIT;
            (*adau1977).right_j = true;
            invert_lrclk = !invert_lrclk;
        }
        SND_SOC_DAIFMT_DSP_A => {
            ctrl1 |= ADAU1977_SAI_CTRL1_LRCLK_PULSE;
            ctrl0 |= ADAU1977_SAI_CTRL0_FMT_I2S;
            invert_lrclk = false;
        }
        SND_SOC_DAIFMT_DSP_B => {
            ctrl1 |= ADAU1977_SAI_CTRL1_LRCLK_PULSE;
            ctrl0 |= ADAU1977_SAI_CTRL0_FMT_LJ;
            invert_lrclk = false;
        }
        _ => return -EINVAL,
    }
    if invert_lrclk {
        block_power |= ADAU1977_BLOCK_POWER_SAI_LR_POL;
    }
    ret = regmap_update_bits(
        (*adau1977).regmap,
        ADAU1977_REG_BLOCK_POWER_SAI,
        ADAU1977_BLOCK_POWER_SAI_LR_POL | ADAU1977_BLOCK_POWER_SAI_BCLK_EDGE,
        block_power,
    );
    if ret != 0 {
        return ret;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_SAI_CTRL0, ADAU1977_SAI_CTRL0_FMT_MASK, ctrl0);
    if ret != 0 {
        return ret;
    }
    regmap_update_bits(
        (*adau1977).regmap,
        ADAU1977_REG_SAI_CTRL1,
        ADAU1977_SAI_CTRL1_MASTER | ADAU1977_SAI_CTRL1_LRCLK_PULSE,
        ctrl1,
    )
}

unsafe extern "C" fn adau1977_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata((*dai).component);
    let mut formats: u64_ = 0;
    if (*adau1977).slot_width == 16 {
        formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE;
    } else if (*adau1977).right_j || (*adau1977).slot_width == 24 {
        formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_BE;
    }
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &mut (*adau1977).constraints);
    if (*adau1977).clock_provider {
        snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 8000, (*adau1977).max_clock_provider_fs);
    }
    if formats != 0 {
        snd_pcm_hw_constraint_mask64((*substream).runtime, SNDRV_PCM_HW_PARAM_FORMAT, formats);
    }
    0
}

unsafe extern "C" fn adau1977_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata((*dai).component);
    let val: c_uint = if tristate != 0 { ADAU1977_SAI_OVERTEMP_DRV_HIZ } else { 0 };
    regmap_update_bits((*adau1977).regmap, ADAU1977_REG_SAI_OVERTEMP, ADAU1977_SAI_OVERTEMP_DRV_HIZ, val)
}

static adau1977_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(adau1977_startup),
    hw_params: Some(adau1977_hw_params),
    mute_stream: Some(adau1977_mute),
    set_fmt: Some(adau1977_set_dai_fmt),
    set_tdm_slot: Some(adau1977_set_tdm_slot),
    set_tristate: Some(adau1977_set_tristate),
};

static mut adau1977_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"adau1977-hifi\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 4,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        sig_bits: 24,
    },
    ops: &adau1977_dai_ops,
};

static adau1977_rates: [c_uint; 15] = [
    8000, 16000, 32000, 64000, 128000,
    11025, 22050, 44100, 88200, 172400,
    12000, 24000, 48000, 96000, 192000,
];

const ADAU1977_RATE_CONSTRAINT_MASK_32000: c_uint = 0x001f;
const ADAU1977_RATE_CONSTRAINT_MASK_44100: c_uint = 0x03e0;
const ADAU1977_RATE_CONSTRAINT_MASK_48000: c_uint = 0x7c00;
/* All rates >= 32000 */
const ADAU1977_RATE_CONSTRAINT_MASK_LRCLK: c_uint = 0x739c;

fn adau1977_check_sysclk(mclk: c_uint, base_freq: c_uint) -> bool_ {
    let mcs: c_uint;
    if mclk % (base_freq * 128) != 0 {
        return false;
    }
    mcs = mclk / (128 * base_freq);
    if mcs < 1 || mcs > 6 || mcs == 5 {
        return false;
    }
    true
}

unsafe extern "C" fn adau1977_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int {
    let adau1977 = snd_soc_component_get_drvdata(component);
    let mut mask: c_uint = 0;
    let clk_src: c_uint;
    let mut ret: c_int;
    if dir != SND_SOC_CLOCK_IN {
        return -EINVAL;
    }
    if clk_id != ADAU1977_SYSCLK {
        return -EINVAL;
    }
    match source {
        ADAU1977_SYSCLK_SRC_MCLK => clk_src = 0,
        ADAU1977_SYSCLK_SRC_LRCLK => clk_src = ADAU1977_PLL_CLK_S,
        _ => return -EINVAL,
    }
    if freq != 0 && source == ADAU1977_SYSCLK_SRC_MCLK {
        if freq < 4000000 || freq > 36864000 {
            return -EINVAL;
        }
        if adau1977_check_sysclk(freq, 32000) {
            mask |= ADAU1977_RATE_CONSTRAINT_MASK_32000;
        }
        if adau1977_check_sysclk(freq, 44100) {
            mask |= ADAU1977_RATE_CONSTRAINT_MASK_44100;
        }
        if adau1977_check_sysclk(freq, 48000) {
            mask |= ADAU1977_RATE_CONSTRAINT_MASK_48000;
        }
        if mask == 0 {
            return -EINVAL;
        }
    } else if source == ADAU1977_SYSCLK_SRC_LRCLK {
        mask = ADAU1977_RATE_CONSTRAINT_MASK_LRCLK;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_PLL, ADAU1977_PLL_CLK_S, clk_src);
    if ret != 0 {
        return ret;
    }
    (*adau1977).constraints.mask = mask;
    (*adau1977).sysclk_src = source;
    (*adau1977).sysclk = freq;
    0
}

unsafe extern "C" fn adau1977_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let adau1977 = snd_soc_component_get_drvdata(component);
    let mut ret: c_int;
    match (*adau1977).type_ {
        ADAU1977 => {
            ret = snd_soc_dapm_new_controls(dapm, adau1977_micbias_dapm_widgets.as_ptr(), adau1977_micbias_dapm_widgets.len() as c_uint);
            if ret < 0 {
                return ret;
            }
        }
        _ => {}
    }
    0
}

static adau1977_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adau1977_component_probe),
    set_bias_level: Some(adau1977_set_bias_level),
    set_sysclk: Some(adau1977_set_sysclk),
    controls: adau1977_snd_controls.as_ptr(),
    num_controls: adau1977_snd_controls.len() as c_uint,
    dapm_widgets: adau1977_dapm_widgets.as_ptr(),
    num_dapm_widgets: adau1977_dapm_widgets.len() as c_uint,
    dapm_routes: adau1977_dapm_routes.as_ptr(),
    num_dapm_routes: adau1977_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn adau1977_setup_micbias(adau1977: *mut adau1977) -> c_int {
    let mut micbias: c_uint = 0;
    if device_property_read_u32((*adau1977).dev, b"adi,micbias\0".as_ptr() as *const c_char, &mut micbias) != 0 {
        micbias = ADAU1977_MICBIAS_8V5;
    }
    if micbias > ADAU1977_MICBIAS_9V0 {
        dev_err((*adau1977).dev, b"Invalid value for 'adi,micbias'\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    regmap_update_bits(
        (*adau1977).regmap,
        ADAU1977_REG_MICBIAS,
        ADAU1977_MICBIAS_MB_VOLTS_MASK,
        micbias << ADAU1977_MICBIAS_MB_VOLTS_OFFSET,
    )
}

#[no_mangle]
pub unsafe extern "C" fn adau1977_probe(dev: *mut device, regmap: *mut regmap, type_: adau1977_type, switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>) -> c_int {
    let power_off_mask: c_uint;
    let adau1977: *mut adau1977;
    let mut ret: c_int;
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }
    adau1977 = devm_kzalloc(dev, size_of::<adau1977>(), GFP_KERNEL) as *mut adau1977;
    if adau1977.is_null() {
        return -ENOMEM;
    }
    (*adau1977).dev = dev;
    (*adau1977).type_ = type_;
    (*adau1977).regmap = regmap;
    (*adau1977).switch_mode = switch_mode;
    (*adau1977).max_clock_provider_fs = 192000;
    (*adau1977).constraints.list = adau1977_rates.as_ptr();
    (*adau1977).constraints.count = adau1977_rates.len() as c_uint;
    (*adau1977).avdd_reg = devm_regulator_get(dev, b"AVDD\0".as_ptr() as *const c_char);
    if IS_ERR((*adau1977).avdd_reg as *const c_void) {
        return PTR_ERR((*adau1977).avdd_reg as *const c_void);
    }
    (*adau1977).dvdd_reg = devm_regulator_get_optional(dev, b"DVDD\0".as_ptr() as *const c_char);
    if IS_ERR((*adau1977).dvdd_reg as *const c_void) {
        if PTR_ERR((*adau1977).dvdd_reg as *const c_void) != -ENODEV {
            return PTR_ERR((*adau1977).dvdd_reg as *const c_void);
        }
        (*adau1977).dvdd_reg = ptr::null_mut();
    }
    (*adau1977).reset_gpio = devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*adau1977).reset_gpio as *const c_void) {
        return PTR_ERR((*adau1977).reset_gpio as *const c_void);
    }
    dev_set_drvdata(dev, adau1977 as *mut c_void);
    if !(*adau1977).reset_gpio.is_null() {
        ndelay(100);
    }
    ret = adau1977_power_enable(adau1977);
    if ret != 0 {
        return ret;
    }
    if type_ == ADAU1977 {
        ret = adau1977_setup_micbias(adau1977);
        if ret != 0 {
            adau1977_power_disable(adau1977);
            return ret;
        }
    }
    if !(*adau1977).dvdd_reg.is_null() {
        power_off_mask = !0u32;
    } else {
        power_off_mask = !ADAU1977_BLOCK_POWER_SAI_LDO_EN;
    }
    ret = regmap_update_bits((*adau1977).regmap, ADAU1977_REG_BLOCK_POWER_SAI, power_off_mask, 0x00);
    if ret != 0 {
        adau1977_power_disable(adau1977);
        return ret;
    }
    ret = adau1977_power_disable(adau1977);
    if ret != 0 {
        return ret;
    }
    devm_snd_soc_register_component(dev, &adau1977_component_driver, &mut adau1977_dai, 1)
}
/* EXPORT_SYMBOL_GPL(adau1977_probe); */

unsafe extern "C" fn adau1977_register_volatile(dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        x if x == ADAU1977_REG_STATUS(0) => true,
        x if x == ADAU1977_REG_STATUS(1) => true,
        x if x == ADAU1977_REG_STATUS(2) => true,
        x if x == ADAU1977_REG_STATUS(3) => true,
        ADAU1977_REG_ADC_CLIP => true,
        _ => false,
    }
}

#[no_mangle]
pub static adau1977_regmap_config: regmap_config = regmap_config {
    max_register: ADAU1977_REG_DC_HPF_CAL,
    volatile_reg: Some(adau1977_register_volatile),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: adau1977_reg_defaults.as_ptr(),
    num_reg_defaults: adau1977_reg_defaults.len() as c_uint,
};
/* EXPORT_SYMBOL_GPL(adau1977_regmap_config); */

/* MODULE_DESCRIPTION("ASoC ADAU1977/ADAU1978/ADAU1979 driver"); */
/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
