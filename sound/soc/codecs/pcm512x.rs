// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the PCM512x CODECs
 *
 * Author:	Mark Brown <broonie@kernel.org>
 *		Copyright 2014 Linaro Ltd
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct device {
    pub of_node: *const device_node,
}
#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct regulator;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct snd_kcontrol;
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_context;
#[repr(C)]
pub struct snd_pcm_hw_params;
#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
    pub var: c_int,
}
#[repr(C)]
pub struct snd_interval {
    pub min: u32,
    pub max: u32,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime;
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_ratnum {
    pub num: c_ulong,
    pub den_min: c_ulong,
    pub den_max: c_ulong,
    pub den_step: c_ulong,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_int,
    pub rats: *mut snd_ratnum,
}
#[repr(C)]
pub struct reg_default {
    pub reg: u32,
    pub def: u32,
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
    pub consumer: *mut regulator,
}
#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}
#[repr(C)]
pub struct soc_enum {
    pub reg: u32,
    pub shift_l: u32,
    pub items: u32,
    pub texts: *const *const c_char,
    pub values: *const u32,
    pub mask: u32,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub index: u32,
    pub access: u32,
    pub info: Option<unsafe extern "C" fn()>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
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
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
    pub no_capture_mute: c_int,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: u32,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: u32,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: u32,
    pub use_pmdown_time: c_int,
    pub endianness: c_int,
}
#[repr(C)]
pub struct regmap_range_cfg {
    pub name: *const c_char,
    pub range_min: u32,
    pub range_max: u32,
    pub selector_reg: u32,
    pub selector_mask: u32,
    pub window_start: u32,
    pub window_len: u32,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool_>,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_int,
    pub max_register: u32,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: u32,
    pub cache_type: c_int,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: u32 = 0;
const REGULATOR_EVENT_DISABLE: c_ulong = 0x2;
const REGCACHE_RBTREE: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: u32 = 0x3;
const SND_SOC_NOPM: u32 = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_FRAME_BITS: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;

extern "C" {
    static snd_ctl_boolean_stereo_info: unsafe extern "C" fn();

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_interval_ranges(interval: *mut snd_interval, count: u32, ranges: *mut snd_interval, mask: c_int) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: u32, var: c_int,
                           func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
                           private: *mut c_void, dep1: c_int, dep2: c_int, end: c_int) -> c_int;
    fn snd_pcm_hw_constraint_ratnums(runtime: *mut snd_pcm_runtime, cond: u32, var: c_int,
                                     rats: *mut snd_pcm_hw_constraint_ratnums) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: u32, var: c_int,
                                  list: *const snd_pcm_hw_constraint_list) -> c_int;

    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_regulator_register_notifier(consumer: *mut regulator, nb: *mut notifier_block) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver,
                                        dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn of_property_read_u32(np: *const device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn gcd(a: c_ulong, b: c_ulong) -> c_ulong;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! dev_err {
    ($dev:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{}};
}
macro_rules! dev_dbg {
    ($dev:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{}};
}
macro_rules! dev_info_once {
    ($dev:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{}};
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}
unsafe fn PTR_ERR<T>(ptr: *mut T) -> isize {
    ptr as isize
}
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> u32 {
    N as u32
}
fn DIV_ROUND_UP(n: c_ulong, d: c_ulong) -> c_ulong {
    n.wrapping_add(d).wrapping_sub(1) / d
}
fn DIV_ROUND_CLOSEST(n: c_ulong, d: c_ulong) -> c_ulong {
    n.wrapping_add(d / 2) / d
}
fn DIV_ROUND_CLOSEST_ULL(n: u64, d: u64) -> u64 {
    n.wrapping_add(d / 2) / d
}
fn DIV_ROUND_DOWN_ULL(n: u64, d: u64) -> u64 {
    n / d
}
fn rounddown(x: c_ulong, y: c_ulong) -> c_ulong {
    x - x % y
}
fn fls(x: c_ulong) -> c_int {
    if x == 0 { 0 } else { (usize::BITS - x.leading_zeros()) as c_int }
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: u32,
    pub list: *const u32,
}

const PCM512x_NUM_SUPPLIES: usize = 3;
static pcm512x_supply_names: [*const c_char; PCM512x_NUM_SUPPLIES] = [
    cstr!("AVDD"),
    cstr!("DVDD"),
    cstr!("CPVDD"),
];

#[repr(C)]
pub struct pcm512x_priv {
    pub regmap: *mut regmap,
    pub sclk: *mut clk,
    pub supplies: [regulator_bulk_data; PCM512x_NUM_SUPPLIES],
    pub supply_nb: [notifier_block; PCM512x_NUM_SUPPLIES],
    pub fmt: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub pll_r: c_int,
    pub pll_j: c_int,
    pub pll_d: c_int,
    pub pll_p: c_int,
    pub real_pll: c_ulong,
    pub overclock_pll: c_ulong,
    pub overclock_dac: c_ulong,
    pub overclock_dsp: c_ulong,
    pub mute: c_int,
    pub mutex: mutex,
    pub bclk_ratio: u32,
    pub force_pll_on: c_int,
}

/*
 * We can't use the same notifier block for more than one supply and
 * there's no way I can see to get from a callback to the caller
 * except container_of().
 */
unsafe extern "C" fn pcm512x_regulator_event_0(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let pcm512x = (nb as *mut u8).sub(offset_of_supply_nb(0)) as *mut pcm512x_priv;
    if event & REGULATOR_EVENT_DISABLE != 0 {
        regcache_mark_dirty((*pcm512x).regmap);
        regcache_cache_only((*pcm512x).regmap, true);
    }
    0
}
unsafe extern "C" fn pcm512x_regulator_event_1(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let pcm512x = (nb as *mut u8).sub(offset_of_supply_nb(1)) as *mut pcm512x_priv;
    if event & REGULATOR_EVENT_DISABLE != 0 {
        regcache_mark_dirty((*pcm512x).regmap);
        regcache_cache_only((*pcm512x).regmap, true);
    }
    0
}
unsafe extern "C" fn pcm512x_regulator_event_2(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let pcm512x = (nb as *mut u8).sub(offset_of_supply_nb(2)) as *mut pcm512x_priv;
    if event & REGULATOR_EVENT_DISABLE != 0 {
        regcache_mark_dirty((*pcm512x).regmap);
        regcache_cache_only((*pcm512x).regmap, true);
    }
    0
}
fn offset_of_supply_nb(n: usize) -> usize {
    let uninit = core::mem::MaybeUninit::<pcm512x_priv>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&(*base).supply_nb[n] as *const _ as usize) - (base as usize) }
}

/* Register, bit-field, DAI-format, and sample-format constants are supplied by
 * the Rust translation of pcm512x.h and the surrounding ASoC/kernel bindings.
 */

static pcm512x_reg_defaults: [reg_default; 45] = [
    reg_default { reg: PCM512x_RESET,             def: 0x00 },
    reg_default { reg: PCM512x_POWER,             def: 0x00 },
    reg_default { reg: PCM512x_MUTE,              def: 0x00 },
    reg_default { reg: PCM512x_DSP,               def: 0x00 },
    reg_default { reg: PCM512x_BCLK_LRCLK_CFG,    def: 0x00 },
    reg_default { reg: PCM512x_MASTER_MODE,       def: 0x7c },
    reg_default { reg: PCM512x_PLL_REF,           def: 0x00 },
    reg_default { reg: PCM512x_DAC_REF,           def: 0x00 },
    reg_default { reg: PCM512x_GPIO_DACIN,        def: 0x00 },
    reg_default { reg: PCM512x_GPIO_PLLIN,        def: 0x00 },
    reg_default { reg: PCM512x_SYNCHRONIZE,       def: 0x10 },
    reg_default { reg: PCM512x_PLL_COEFF_0,       def: 0x00 },
    reg_default { reg: PCM512x_PLL_COEFF_1,       def: 0x00 },
    reg_default { reg: PCM512x_PLL_COEFF_2,       def: 0x00 },
    reg_default { reg: PCM512x_PLL_COEFF_3,       def: 0x00 },
    reg_default { reg: PCM512x_PLL_COEFF_4,       def: 0x00 },
    reg_default { reg: PCM512x_DSP_CLKDIV,        def: 0x00 },
    reg_default { reg: PCM512x_DAC_CLKDIV,        def: 0x00 },
    reg_default { reg: PCM512x_NCP_CLKDIV,        def: 0x00 },
    reg_default { reg: PCM512x_OSR_CLKDIV,        def: 0x00 },
    reg_default { reg: PCM512x_MASTER_CLKDIV_1,   def: 0x00 },
    reg_default { reg: PCM512x_MASTER_CLKDIV_2,   def: 0x00 },
    reg_default { reg: PCM512x_FS_SPEED_MODE,     def: 0x00 },
    reg_default { reg: PCM512x_IDAC_1,            def: 0x01 },
    reg_default { reg: PCM512x_IDAC_2,            def: 0x00 },
    reg_default { reg: PCM512x_ERROR_DETECT,      def: 0x00 },
    reg_default { reg: PCM512x_I2S_1,             def: 0x02 },
    reg_default { reg: PCM512x_I2S_2,             def: 0x00 },
    reg_default { reg: PCM512x_DAC_ROUTING,       def: 0x11 },
    reg_default { reg: PCM512x_DSP_PROGRAM,       def: 0x01 },
    reg_default { reg: PCM512x_CLKDET,            def: 0x00 },
    reg_default { reg: PCM512x_AUTO_MUTE,         def: 0x00 },
    reg_default { reg: PCM512x_DIGITAL_VOLUME_1,  def: 0x00 },
    reg_default { reg: PCM512x_DIGITAL_VOLUME_2,  def: 0x30 },
    reg_default { reg: PCM512x_DIGITAL_VOLUME_3,  def: 0x30 },
    reg_default { reg: PCM512x_DIGITAL_MUTE_1,    def: 0x22 },
    reg_default { reg: PCM512x_DIGITAL_MUTE_2,    def: 0x00 },
    reg_default { reg: PCM512x_DIGITAL_MUTE_3,    def: 0x07 },
    reg_default { reg: PCM512x_OUTPUT_AMPLITUDE,  def: 0x00 },
    reg_default { reg: PCM512x_ANALOG_GAIN_CTRL,  def: 0x00 },
    reg_default { reg: PCM512x_UNDERVOLTAGE_PROT, def: 0x00 },
    reg_default { reg: PCM512x_ANALOG_MUTE_CTRL,  def: 0x00 },
    reg_default { reg: PCM512x_ANALOG_GAIN_BOOST, def: 0x00 },
    reg_default { reg: PCM512x_VCOM_CTRL_1,       def: 0x00 },
    reg_default { reg: PCM512x_VCOM_CTRL_2,       def: 0x01 },
];

unsafe extern "C" fn pcm512x_readable(_dev: *mut device, reg: u32) -> bool {
    match reg {
        PCM512x_RESET | PCM512x_POWER | PCM512x_MUTE | PCM512x_PLL_EN |
        PCM512x_SPI_MISO_FUNCTION | PCM512x_DSP | PCM512x_GPIO_EN |
        PCM512x_BCLK_LRCLK_CFG | PCM512x_DSP_GPIO_INPUT | PCM512x_MASTER_MODE |
        PCM512x_PLL_REF | PCM512x_DAC_REF | PCM512x_GPIO_DACIN |
        PCM512x_GPIO_PLLIN | PCM512x_SYNCHRONIZE | PCM512x_PLL_COEFF_0 |
        PCM512x_PLL_COEFF_1 | PCM512x_PLL_COEFF_2 | PCM512x_PLL_COEFF_3 |
        PCM512x_PLL_COEFF_4 | PCM512x_DSP_CLKDIV | PCM512x_DAC_CLKDIV |
        PCM512x_NCP_CLKDIV | PCM512x_OSR_CLKDIV | PCM512x_MASTER_CLKDIV_1 |
        PCM512x_MASTER_CLKDIV_2 | PCM512x_FS_SPEED_MODE | PCM512x_IDAC_1 |
        PCM512x_IDAC_2 | PCM512x_ERROR_DETECT | PCM512x_I2S_1 | PCM512x_I2S_2 |
        PCM512x_DAC_ROUTING | PCM512x_DSP_PROGRAM | PCM512x_CLKDET |
        PCM512x_AUTO_MUTE | PCM512x_DIGITAL_VOLUME_1 | PCM512x_DIGITAL_VOLUME_2 |
        PCM512x_DIGITAL_VOLUME_3 | PCM512x_DIGITAL_MUTE_1 | PCM512x_DIGITAL_MUTE_2 |
        PCM512x_DIGITAL_MUTE_3 | PCM512x_GPIO_OUTPUT_1 | PCM512x_GPIO_OUTPUT_2 |
        PCM512x_GPIO_OUTPUT_3 | PCM512x_GPIO_OUTPUT_4 | PCM512x_GPIO_OUTPUT_5 |
        PCM512x_GPIO_OUTPUT_6 | PCM512x_GPIO_CONTROL_1 | PCM512x_GPIO_CONTROL_2 |
        PCM512x_OVERFLOW | PCM512x_RATE_DET_1 | PCM512x_RATE_DET_2 |
        PCM512x_RATE_DET_3 | PCM512x_RATE_DET_4 | PCM512x_CLOCK_STATUS |
        PCM512x_ANALOG_MUTE_DET | PCM512x_GPIN | PCM512x_DIGITAL_MUTE_DET |
        PCM512x_OUTPUT_AMPLITUDE | PCM512x_ANALOG_GAIN_CTRL |
        PCM512x_UNDERVOLTAGE_PROT | PCM512x_ANALOG_MUTE_CTRL |
        PCM512x_ANALOG_GAIN_BOOST | PCM512x_VCOM_CTRL_1 | PCM512x_VCOM_CTRL_2 |
        PCM512x_CRAM_CTRL | PCM512x_FLEX_A | PCM512x_FLEX_B => true,
        _ => reg < 0xff,
    }
}

unsafe extern "C" fn pcm512x_volatile(_dev: *mut device, reg: u32) -> bool {
    match reg {
        PCM512x_PLL_EN | PCM512x_OVERFLOW | PCM512x_RATE_DET_1 |
        PCM512x_RATE_DET_2 | PCM512x_RATE_DET_3 | PCM512x_RATE_DET_4 |
        PCM512x_CLOCK_STATUS | PCM512x_ANALOG_MUTE_DET | PCM512x_GPIN |
        PCM512x_DIGITAL_MUTE_DET | PCM512x_CRAM_CTRL => true,
        _ => reg < 0xff,
    }
}

unsafe extern "C" fn pcm512x_overclock_pll_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    (*ucontrol).value.integer.value[0] = (*pcm512x).overclock_pll as i64;
    0
}
unsafe extern "C" fn pcm512x_overclock_pll_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    match snd_soc_dapm_get_bias_level(dapm) {
        snd_soc_bias_level::SND_SOC_BIAS_OFF | snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {}
        _ => return -EBUSY,
    }
    (*pcm512x).overclock_pll = (*ucontrol).value.integer.value[0] as c_ulong;
    0
}
unsafe extern "C" fn pcm512x_overclock_dsp_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    (*ucontrol).value.integer.value[0] = (*pcm512x).overclock_dsp as i64;
    0
}
unsafe extern "C" fn pcm512x_overclock_dsp_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    match snd_soc_dapm_get_bias_level(dapm) {
        snd_soc_bias_level::SND_SOC_BIAS_OFF | snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {}
        _ => return -EBUSY,
    }
    (*pcm512x).overclock_dsp = (*ucontrol).value.integer.value[0] as c_ulong;
    0
}
unsafe extern "C" fn pcm512x_overclock_dac_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    (*ucontrol).value.integer.value[0] = (*pcm512x).overclock_dac as i64;
    0
}
unsafe extern "C" fn pcm512x_overclock_dac_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    match snd_soc_dapm_get_bias_level(dapm) {
        snd_soc_bias_level::SND_SOC_BIAS_OFF | snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {}
        _ => return -EBUSY,
    }
    (*pcm512x).overclock_dac = (*ucontrol).value.integer.value[0] as c_ulong;
    0
}

/* DECLARE_TLV_DB_SCALE(digital_tlv, -10350, 50, 1); */
static digital_tlv: [u32; 4] = [0, (-10350i32) as u32, 50, 1];
static analog_tlv: [u32; 4] = [0, (-600i32) as u32, 600, 0];
static boost_tlv: [u32; 4] = [0, 0, 80, 0];

static pcm512x_dsp_program_texts: [*const c_char; 5] = [
    cstr!("FIR interpolation with de-emphasis"),
    cstr!("Low latency IIR with de-emphasis"),
    cstr!("High attenuation with de-emphasis"),
    cstr!("Fixed process flow"),
    cstr!("Ringing-less low latency FIR"),
];
static pcm512x_dsp_program_values: [u32; 5] = [1, 2, 3, 5, 7];
static pcm512x_dsp_program: soc_enum = soc_enum {
    reg: PCM512x_DSP_PROGRAM, shift_l: 0, items: 5,
    texts: pcm512x_dsp_program_texts.as_ptr(), values: pcm512x_dsp_program_values.as_ptr(), mask: 0x1f,
};
static pcm512x_clk_missing_text: [*const c_char; 8] = [
    cstr!("1s"), cstr!("2s"), cstr!("3s"), cstr!("4s"),
    cstr!("5s"), cstr!("6s"), cstr!("7s"), cstr!("8s"),
];
static pcm512x_clk_missing: soc_enum = soc_enum {
    reg: PCM512x_CLKDET, shift_l: 0, items: 8,
    texts: pcm512x_clk_missing_text.as_ptr(), values: ptr::null(), mask: 0,
};
static pcm512x_autom_text: [*const c_char; 8] = [
    cstr!("21ms"), cstr!("106ms"), cstr!("213ms"), cstr!("533ms"),
    cstr!("1.07s"), cstr!("2.13s"), cstr!("5.33s"), cstr!("10.66s"),
];
static pcm512x_autom_l: soc_enum = soc_enum { reg: PCM512x_AUTO_MUTE, shift_l: PCM512x_ATML_SHIFT, items: 8, texts: pcm512x_autom_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_autom_r: soc_enum = soc_enum { reg: PCM512x_AUTO_MUTE, shift_l: PCM512x_ATMR_SHIFT, items: 8, texts: pcm512x_autom_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_ramp_rate_text: [*const c_char; 4] = [
    cstr!("1 sample/update"), cstr!("2 samples/update"),
    cstr!("4 samples/update"), cstr!("Immediate"),
];
static pcm512x_vndf: soc_enum = soc_enum { reg: PCM512x_DIGITAL_MUTE_1, shift_l: PCM512x_VNDF_SHIFT, items: 4, texts: pcm512x_ramp_rate_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_vnuf: soc_enum = soc_enum { reg: PCM512x_DIGITAL_MUTE_1, shift_l: PCM512x_VNUF_SHIFT, items: 4, texts: pcm512x_ramp_rate_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_vedf: soc_enum = soc_enum { reg: PCM512x_DIGITAL_MUTE_2, shift_l: PCM512x_VEDF_SHIFT, items: 4, texts: pcm512x_ramp_rate_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_ramp_step_text: [*const c_char; 4] = [
    cstr!("4dB/step"), cstr!("2dB/step"), cstr!("1dB/step"), cstr!("0.5dB/step"),
];
static pcm512x_vnds: soc_enum = soc_enum { reg: PCM512x_DIGITAL_MUTE_1, shift_l: PCM512x_VNDS_SHIFT, items: 4, texts: pcm512x_ramp_step_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_vnus: soc_enum = soc_enum { reg: PCM512x_DIGITAL_MUTE_1, shift_l: PCM512x_VNUS_SHIFT, items: 4, texts: pcm512x_ramp_step_text.as_ptr(), values: ptr::null(), mask: 0 };
static pcm512x_veds: soc_enum = soc_enum { reg: PCM512x_DIGITAL_MUTE_2, shift_l: PCM512x_VEDS_SHIFT, items: 4, texts: pcm512x_ramp_step_text.as_ptr(), values: ptr::null(), mask: 0 };

unsafe fn pcm512x_update_mute(pcm512x: *mut pcm512x_priv) -> c_int {
    regmap_update_bits(
        (*pcm512x).regmap, PCM512x_MUTE, PCM512x_RQML | PCM512x_RQMR,
        (((*pcm512x).mute & 0x5 != 0) as u32) << PCM512x_RQML_SHIFT
            | (((*pcm512x).mute & 0x3 != 0) as u32) << PCM512x_RQMR_SHIFT)
}

unsafe extern "C" fn pcm512x_digital_playback_switch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    (*ucontrol).value.integer.value[0] = (((*pcm512x).mute & 0x4) == 0) as i64;
    (*ucontrol).value.integer.value[1] = (((*pcm512x).mute & 0x2) == 0) as i64;
    0
}

unsafe extern "C" fn pcm512x_digital_playback_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let mut changed = 0;
    if ((*pcm512x).mute & 0x4) == (((*ucontrol).value.integer.value[0] as c_int) << 2) {
        (*pcm512x).mute ^= 0x4;
        changed = 1;
    }
    if ((*pcm512x).mute & 0x2) == (((*ucontrol).value.integer.value[1] as c_int) << 1) {
        (*pcm512x).mute ^= 0x2;
        changed = 1;
    }
    if changed != 0 {
        let ret = pcm512x_update_mute(pcm512x);
        if ret != 0 {
            dev_err!((*component).dev, "Failed to update digital mute: %d\n", ret);
            return ret;
        }
    }
    changed
}

/* ALSA control macro initializers translated as file-local data where direct layout is known. */
static pcm512x_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: cstr!("Digital Playback Switch"), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READWRITE, info: Some(snd_ctl_boolean_stereo_info), get: Some(pcm512x_digital_playback_switch_get), put: Some(pcm512x_digital_playback_switch_put) },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: cstr!("Max Overclock PLL"), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READWRITE, info: None, get: Some(pcm512x_overclock_pll_get), put: Some(pcm512x_overclock_pll_put) },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: cstr!("Max Overclock DSP"), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READWRITE, info: None, get: Some(pcm512x_overclock_dsp_get), put: Some(pcm512x_overclock_dsp_put) },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: cstr!("Max Overclock DAC"), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READWRITE, info: None, get: Some(pcm512x_overclock_dac_get), put: Some(pcm512x_overclock_dac_put) },
];
/* Other SOC_* controls from C are dependent on external ASoC macro layouts:
 * Digital/Analogue volumes, Deemphasis, DSP Program, Clock Missing Period,
 * Auto Mute controls, and Volume Ramp controls.
 */

static pcm512x_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { id: 0, name: cstr!("DACL") },
    snd_soc_dapm_widget { id: 0, name: cstr!("DACR") },
    snd_soc_dapm_widget { id: 0, name: cstr!("OUTL") },
    snd_soc_dapm_widget { id: 0, name: cstr!("OUTR") },
];
static pcm512x_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: cstr!("DACL"), control: ptr::null(), source: cstr!("Playback") },
    snd_soc_dapm_route { sink: cstr!("DACR"), control: ptr::null(), source: cstr!("Playback") },
    snd_soc_dapm_route { sink: cstr!("OUTL"), control: ptr::null(), source: cstr!("DACL") },
    snd_soc_dapm_route { sink: cstr!("OUTR"), control: ptr::null(), source: cstr!("DACR") },
];

unsafe fn pcm512x_pll_max(pcm512x: *mut pcm512x_priv) -> c_ulong {
    25000000 + 25000000 * (*pcm512x).overclock_pll / 100
}
unsafe fn pcm512x_dsp_max(pcm512x: *mut pcm512x_priv) -> c_ulong {
    50000000 + 50000000 * (*pcm512x).overclock_dsp / 100
}
unsafe fn pcm512x_dac_max(pcm512x: *mut pcm512x_priv, rate: c_ulong) -> c_ulong {
    rate + rate * (*pcm512x).overclock_dac / 100
}
unsafe fn pcm512x_sck_max(pcm512x: *mut pcm512x_priv) -> c_ulong {
    if (*pcm512x).pll_out == 0 { 25000000 } else { pcm512x_pll_max(pcm512x) }
}
unsafe fn pcm512x_ncp_target(pcm512x: *mut pcm512x_priv, dac_rate: c_ulong) -> c_ulong {
    /*
     * If the DAC is not actually overclocked, use the good old
     * NCP target rate...
     */
    if dac_rate <= 6144000 {
        return 1536000;
    }
    /*
     * ...but if the DAC is in fact overclocked, bump the NCP target
     * rate to get the recommended dividers even when overclocking.
     */
    pcm512x_dac_max(pcm512x, 1536000)
}

static pcm512x_dai_rates: [u32; 13] = [
    8000, 11025, 16000, 22050, 32000, 44100, 48000, 64000,
    88200, 96000, 176400, 192000, 384000,
];
static constraints_slave: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 13,
    list: pcm512x_dai_rates.as_ptr(),
};

unsafe extern "C" fn pcm512x_hw_rule_rate(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let pcm512x = (*rule).private as *mut pcm512x_priv;
    let mut ranges = [snd_interval { min: 0, max: 0 }, snd_interval { min: 0, max: 0 }];
    let frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 {
        return frame_size;
    }
    match frame_size {
        32 => 0,
        48 | 64 => {
            ranges[0].min = 8000;
            ranges[0].max = (pcm512x_sck_max(pcm512x) / frame_size as c_ulong / 2) as u32;
            ranges[1].min = DIV_ROUND_UP(16000000, frame_size as c_ulong) as u32;
            ranges[1].max = 384000;
            snd_interval_ranges(hw_param_interval(params, (*rule).var), ARRAY_SIZE(&ranges), ranges.as_mut_ptr(), 0)
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn pcm512x_dai_startup_master(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let dev = (*dai).dev;
    if IS_ERR((*pcm512x).sclk) {
        dev_err!(dev, "Need SCLK for master mode: %ld\n", PTR_ERR((*pcm512x).sclk));
        return PTR_ERR((*pcm512x).sclk) as c_int;
    }
    if (*pcm512x).pll_out != 0 {
        return snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
                                   pcm512x_hw_rule_rate, pcm512x as *mut c_void,
                                   SNDRV_PCM_HW_PARAM_FRAME_BITS, SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    }
    let constraints_no_pll = devm_kzalloc(dev, size_of::<snd_pcm_hw_constraint_ratnums>(), GFP_KERNEL) as *mut snd_pcm_hw_constraint_ratnums;
    if constraints_no_pll.is_null() {
        return -ENOMEM;
    }
    (*constraints_no_pll).nrats = 1;
    let rats_no_pll = devm_kzalloc(dev, size_of::<snd_ratnum>(), GFP_KERNEL) as *mut snd_ratnum;
    if rats_no_pll.is_null() {
        return -ENOMEM;
    }
    (*constraints_no_pll).rats = rats_no_pll;
    (*rats_no_pll).num = clk_get_rate((*pcm512x).sclk) / 64;
    (*rats_no_pll).den_min = 1;
    (*rats_no_pll).den_max = 128;
    (*rats_no_pll).den_step = 1;
    snd_pcm_hw_constraint_ratnums((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, constraints_no_pll)
}

unsafe extern "C" fn pcm512x_dai_startup_slave(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let dev = (*dai).dev;
    let regmap = (*pcm512x).regmap;
    if IS_ERR((*pcm512x).sclk) {
        dev_info_once!(dev, "No SCLK, using BCLK: %ld\n", PTR_ERR((*pcm512x).sclk));
        regmap_update_bits(regmap, PCM512x_ERROR_DETECT, PCM512x_IDCH, PCM512x_IDCH);
        regmap_update_bits(regmap, PCM512x_PLL_REF, PCM512x_SREF, PCM512x_SREF_BCK);
    }
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_slave)
}

unsafe extern "C" fn pcm512x_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    match (*pcm512x).fmt as u32 & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP | SND_SOC_DAIFMT_CBP_CFC => pcm512x_dai_startup_master(substream, dai),
        SND_SOC_DAIFMT_CBC_CFC => pcm512x_dai_startup_slave(substream, dai),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn pcm512x_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let pcm512x = dev_get_drvdata((*component).dev) as *mut pcm512x_priv;
    let mut ret;
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON | snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            ret = regmap_update_bits((*pcm512x).regmap, PCM512x_POWER, PCM512x_RQST, 0);
            if ret != 0 { dev_err!((*component).dev, "Failed to remove standby: %d\n", ret); return ret; }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            ret = regmap_update_bits((*pcm512x).regmap, PCM512x_POWER, PCM512x_RQST, PCM512x_RQST);
            if ret != 0 { dev_err!((*component).dev, "Failed to request standby: %d\n", ret); return ret; }
        }
    }
    0
}

unsafe fn pcm512x_find_sck(dai: *mut snd_soc_dai, bclk_rate: c_ulong) -> c_ulong {
    let dev = (*dai).dev;
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let mut sck_rate = 0;
    let mut pow2 = 1_i32 << fls((pcm512x_pll_max(pcm512x) - 16000000) / bclk_rate);
    while pow2 != 0 {
        sck_rate = rounddown(pcm512x_pll_max(pcm512x), bclk_rate * pow2 as c_ulong);
        if sck_rate >= 16000000 { break; }
        pow2 >>= 1;
    }
    if pow2 == 0 {
        dev_err!(dev, "Impossible to generate a suitable SCK\n");
        return 0;
    }
    dev_dbg!(dev, "sck_rate %lu\n", sck_rate);
    sck_rate
}

/* pll_rate = pllin_rate * R * J.D / P
 * 1 <= R <= 16
 * 1 <= J <= 63
 * 0 <= D <= 9999
 * 1 <= P <= 15
 * 64 MHz <= pll_rate <= 100 MHz
 * if D == 0
 *     1 MHz <= pllin_rate / P <= 20 MHz
 * else if D > 0
 *     6.667 MHz <= pllin_rate / P <= 20 MHz
 *     4 <= J <= 11
 *     R = 1
 */
unsafe fn pcm512x_find_pll_coeff(dai: *mut snd_soc_dai, pllin_rate: c_ulong, pll_rate: c_ulong) -> c_int {
    let dev = (*dai).dev;
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let mut common = gcd(pll_rate, pllin_rate);
    let mut num = pll_rate / common;
    let mut den = pllin_rate / common;
    let mut R: c_int;
    let mut J: c_int;
    let mut D: c_int;
    let mut P: c_int;
    let mut K: c_ulong;
    dev_dbg!(dev, "pll %lu pllin %lu common %lu\n", pll_rate, pllin_rate, common);
    if pllin_rate / den > 20000000 && num < 8 {
        let q = DIV_ROUND_UP(pllin_rate / den, 20000000);
        num *= q;
        den *= q;
    }
    dev_dbg!(dev, "num / den = %lu / %lu\n", num, den);
    P = den as c_int;
    if den <= 15 && num <= 16 * 63 && 1000000 <= pllin_rate / P as c_ulong && pllin_rate / P as c_ulong <= 20000000 {
        D = 0;
        R = 16;
        while R != 0 {
            if num % R as c_ulong == 0 {
                J = (num / R as c_ulong) as c_int;
                if J != 0 && J <= 63 {
                    dev_dbg!(dev, "R * J / P = %d * %d / %d\n", R, J, P);
                    (*pcm512x).real_pll = pll_rate;
                    (*pcm512x).pll_r = R; (*pcm512x).pll_j = J; (*pcm512x).pll_d = D; (*pcm512x).pll_p = P;
                    return 0;
                }
            }
            R -= 1;
        }
    }
    R = 1;
    if num <= c_ulong::MAX / 10000 {
        common = gcd(10000 * num, den);
        num = 10000 * num / common;
        den /= common;
        dev_dbg!(dev, "num %lu den %lu common %lu\n", num, den, common);
        P = den as c_int;
        while P <= 15 {
            if !(pllin_rate / P as c_ulong < 6667000 || 200000000 < pllin_rate / P as c_ulong) &&
               (num * P as c_ulong) % den == 0 {
                K = num * P as c_ulong / den;
                if K >= 40000 && K <= 120000 {
                    J = (K / 10000) as c_int;
                    D = (K % 10000) as c_int;
                    dev_dbg!(dev, "J.D / P = %d.%04d / %d\n", J, D, P);
                    (*pcm512x).real_pll = pll_rate;
                    (*pcm512x).pll_r = R; (*pcm512x).pll_j = J; (*pcm512x).pll_d = D; (*pcm512x).pll_p = P;
                    return 0;
                }
            }
            P += 1;
        }
    }
    P = DIV_ROUND_UP(pllin_rate, 20000000) as c_int;
    if P == 0 {
        P = 1;
    } else if P > 15 {
        dev_err!(dev, "Need a slower clock as pll-input\n");
        return -EINVAL;
    }
    if pllin_rate / P as c_ulong < 6667000 {
        dev_err!(dev, "Need a faster clock as pll-input\n");
        return -EINVAL;
    }
    K = DIV_ROUND_CLOSEST_ULL(10000_u64 * pll_rate as u64 * P as u64, pllin_rate as u64) as c_ulong;
    if K < 40000 { K = 40000; }
    if K > 120000 { K = 120000; }
    J = (K / 10000) as c_int;
    D = (K % 10000) as c_int;
    dev_dbg!(dev, "J.D / P ~ %d.%04d / %d\n", J, D, P);
    (*pcm512x).real_pll = DIV_ROUND_DOWN_ULL(K as u64 * pllin_rate as u64, (10000 * P) as u64) as c_ulong;
    (*pcm512x).pll_r = R;
    (*pcm512x).pll_j = J;
    (*pcm512x).pll_d = D;
    (*pcm512x).pll_p = P;
    0
}

unsafe fn pcm512x_pllin_dac_rate(dai: *mut snd_soc_dai, osr_rate: c_ulong, pllin_rate: c_ulong) -> c_ulong {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    if (*pcm512x).pll_out == 0 { return 0; }
    if pllin_rate % osr_rate != 0 { return 0; }
    let mut dac_rate = rounddown(pcm512x_dac_max(pcm512x, 6144000), osr_rate);
    while dac_rate != 0 {
        if pllin_rate / dac_rate > 128 { return 0; }
        if pllin_rate % dac_rate == 0 { return dac_rate; }
        dac_rate -= osr_rate;
        dac_rate -= osr_rate;
    }
    0
}

unsafe fn pcm512x_set_dividers(dai: *mut snd_soc_dai, params: *mut snd_pcm_hw_params) -> c_int {
    let dev = (*dai).dev;
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let mut pllin_rate: c_ulong = 0;
    let pll_rate: c_ulong;
    let sck_rate: c_ulong;
    let mck_rate: c_ulong;
    let bclk_rate: c_ulong;
    let sample_rate: c_ulong;
    let osr_rate: c_ulong;
    let dacsrc_rate: c_ulong;
    let bclk_div: c_int;
    let lrclk_div: c_int;
    let dsp_div: c_int;
    let dac_div: c_int;
    let mut dac_rate: c_ulong;
    let mut ncp_div: c_int;
    let osr_div: c_int;
    let mut ret: c_int;
    let idac: c_int;
    let fssp: c_int;
    let mut gpio: c_int;

    if (*pcm512x).bclk_ratio > 0 {
        lrclk_div = (*pcm512x).bclk_ratio as c_int;
    } else {
        lrclk_div = snd_soc_params_to_frame_size(params);
        if lrclk_div == 0 {
            dev_err!(dev, "No LRCLK?\n");
            return -EINVAL;
        }
    }
    if (*pcm512x).pll_out == 0 {
        sck_rate = clk_get_rate((*pcm512x).sclk);
        bclk_rate = params_rate(params) * lrclk_div as c_ulong;
        bclk_div = DIV_ROUND_CLOSEST(sck_rate, bclk_rate) as c_int;
        mck_rate = sck_rate;
    } else {
        ret = snd_soc_params_to_bclk(params);
        if ret < 0 { dev_err!(dev, "Failed to find suitable BCLK: %d\n", ret); return ret; }
        if ret == 0 { dev_err!(dev, "No BCLK?\n"); return -EINVAL; }
        bclk_rate = ret as c_ulong;
        pllin_rate = clk_get_rate((*pcm512x).sclk);
        sck_rate = pcm512x_find_sck(dai, bclk_rate);
        if sck_rate == 0 { return -EINVAL; }
        pll_rate = 4 * sck_rate;
        ret = pcm512x_find_pll_coeff(dai, pllin_rate, pll_rate);
        if ret != 0 { return ret; }
        ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_0, ((*pcm512x).pll_p - 1) as u32);
        if ret != 0 { dev_err!(dev, "Failed to write PLL P: %d\n", ret); return ret; }
        ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_1, (*pcm512x).pll_j as u32);
        if ret != 0 { dev_err!(dev, "Failed to write PLL J: %d\n", ret); return ret; }
        ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_2, ((*pcm512x).pll_d >> 8) as u32);
        if ret != 0 { dev_err!(dev, "Failed to write PLL D msb: %d\n", ret); return ret; }
        ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_3, ((*pcm512x).pll_d & 0xff) as u32);
        if ret != 0 { dev_err!(dev, "Failed to write PLL D lsb: %d\n", ret); return ret; }
        ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_4, ((*pcm512x).pll_r - 1) as u32);
        if ret != 0 { dev_err!(dev, "Failed to write PLL R: %d\n", ret); return ret; }
        mck_rate = (*pcm512x).real_pll;
        bclk_div = DIV_ROUND_CLOSEST(sck_rate, bclk_rate) as c_int;
    }
    if bclk_div > 128 {
        dev_err!(dev, "Failed to find BCLK divider\n");
        return -EINVAL;
    }
    sample_rate = sck_rate / bclk_div as c_ulong / lrclk_div as c_ulong;
    osr_rate = 16 * sample_rate;
    dsp_div = if mck_rate > pcm512x_dsp_max(pcm512x) { 2 } else { 1 };
    dac_rate = pcm512x_pllin_dac_rate(dai, osr_rate, pllin_rate);
    if dac_rate != 0 {
        dev_dbg!(dev, "using pll input as dac input\n");
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_DAC_REF, PCM512x_SDAC, PCM512x_SDAC_GPIO);
        if ret != 0 { dev_err!((*component).dev, "Failed to set gpio as dacref: %d\n", ret); return ret; }
        gpio = PCM512x_GREF_GPIO1 as c_int + (*pcm512x).pll_in - 1;
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_GPIO_DACIN, PCM512x_GREF, gpio as u32);
        if ret != 0 { dev_err!((*component).dev, "Failed to set gpio %d as dacin: %d\n", (*pcm512x).pll_in, ret); return ret; }
        dacsrc_rate = pllin_rate;
    } else {
        let mut dac_mul = pcm512x_dac_max(pcm512x, 6144000) / osr_rate;
        let sck_mul = sck_rate / osr_rate;
        while dac_mul != 0 {
            if sck_mul % dac_mul == 0 { break; }
            dac_mul -= 1;
        }
        if dac_mul == 0 { dev_err!(dev, "Failed to find DAC rate\n"); return -EINVAL; }
        dac_rate = dac_mul * osr_rate;
        dev_dbg!(dev, "dac_rate %lu sample_rate %lu\n", dac_rate, sample_rate);
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_DAC_REF, PCM512x_SDAC, PCM512x_SDAC_SCK);
        if ret != 0 { dev_err!((*component).dev, "Failed to set sck as dacref: %d\n", ret); return ret; }
        dacsrc_rate = sck_rate;
    }
    osr_div = DIV_ROUND_CLOSEST(dac_rate, osr_rate) as c_int;
    if osr_div > 128 { dev_err!(dev, "Failed to find OSR divider\n"); return -EINVAL; }
    dac_div = DIV_ROUND_CLOSEST(dacsrc_rate, dac_rate) as c_int;
    if dac_div > 128 { dev_err!(dev, "Failed to find DAC divider\n"); return -EINVAL; }
    dac_rate = dacsrc_rate / dac_div as c_ulong;
    ncp_div = DIV_ROUND_CLOSEST(dac_rate, pcm512x_ncp_target(pcm512x, dac_rate)) as c_int;
    if ncp_div > 128 || dac_rate / ncp_div as c_ulong > 2048000 {
        ncp_div = DIV_ROUND_UP(dac_rate, 2048000) as c_int;
        if ncp_div > 128 { dev_err!(dev, "Failed to find NCP divider\n"); return -EINVAL; }
    }
    idac = (mck_rate / (dsp_div as c_ulong * sample_rate)) as c_int;
    ret = regmap_write((*pcm512x).regmap, PCM512x_DSP_CLKDIV, (dsp_div - 1) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write DSP divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_DAC_CLKDIV, (dac_div - 1) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write DAC divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_NCP_CLKDIV, (ncp_div - 1) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write NCP divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_OSR_CLKDIV, (osr_div - 1) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write OSR divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_MASTER_CLKDIV_1, (bclk_div - 1) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write BCLK divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_MASTER_CLKDIV_2, (lrclk_div - 1) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write LRCLK divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_IDAC_1, (idac >> 8) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write IDAC msb divider: %d\n", ret); return ret; }
    ret = regmap_write((*pcm512x).regmap, PCM512x_IDAC_2, (idac & 0xff) as u32);
    if ret != 0 { dev_err!(dev, "Failed to write IDAC lsb divider: %d\n", ret); return ret; }
    if sample_rate <= pcm512x_dac_max(pcm512x, 48000) { fssp = PCM512x_FSSP_48KHZ as c_int; }
    else if sample_rate <= pcm512x_dac_max(pcm512x, 96000) { fssp = PCM512x_FSSP_96KHZ as c_int; }
    else if sample_rate <= pcm512x_dac_max(pcm512x, 192000) { fssp = PCM512x_FSSP_192KHZ as c_int; }
    else { fssp = PCM512x_FSSP_384KHZ as c_int; }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_FS_SPEED_MODE, PCM512x_FSSP, fssp as u32);
    if ret != 0 { dev_err!((*component).dev, "Failed to set fs speed: %d\n", ret); return ret; }
    dev_dbg!((*component).dev, "DSP divider %d\n", dsp_div);
    dev_dbg!((*component).dev, "DAC divider %d\n", dac_div);
    dev_dbg!((*component).dev, "NCP divider %d\n", ncp_div);
    dev_dbg!((*component).dev, "OSR divider %d\n", osr_div);
    dev_dbg!((*component).dev, "BCK divider %d\n", bclk_div);
    dev_dbg!((*component).dev, "LRCK divider %d\n", lrclk_div);
    dev_dbg!((*component).dev, "IDAC %d\n", idac);
    dev_dbg!((*component).dev, "1<<FSSP %d\n", 1 << fssp);
    0
}

unsafe extern "C" fn pcm512x_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let mut gpio: c_int;
    let mut ret: c_int;
    dev_dbg!((*component).dev, "hw_params %u Hz, %u channels\n", params_rate(params), params_channels(params));
    let alen = match params_width(params) {
        16 => PCM512x_ALEN_16,
        20 => PCM512x_ALEN_20,
        24 => PCM512x_ALEN_24,
        32 => PCM512x_ALEN_32,
        _ => { dev_err!((*component).dev, "Bad frame size: %d\n", params_width(params)); return -EINVAL; }
    };
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_I2S_1, PCM512x_ALEN, alen);
    if ret != 0 { dev_err!((*component).dev, "Failed to set frame size: %d\n", ret); return ret; }
    if ((*pcm512x).fmt as u32 & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBC_CFC {
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_ERROR_DETECT, PCM512x_DCAS, 0);
        if ret != 0 { dev_err!((*component).dev, "Failed to enable clock divider autoset: %d\n", ret); return ret; }
        return 0;
    }
    if (*pcm512x).pll_out != 0 {
        ret = regmap_write((*pcm512x).regmap, PCM512x_FLEX_A, 0x11);
        if ret != 0 { dev_err!((*component).dev, "Failed to set FLEX_A: %d\n", ret); return ret; }
        ret = regmap_write((*pcm512x).regmap, PCM512x_FLEX_B, 0xff);
        if ret != 0 { dev_err!((*component).dev, "Failed to set FLEX_B: %d\n", ret); return ret; }
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_ERROR_DETECT,
                                 PCM512x_IDFS | PCM512x_IDBK | PCM512x_IDSK | PCM512x_IDCH | PCM512x_IDCM | PCM512x_DCAS | PCM512x_IPLK,
                                 PCM512x_IDFS | PCM512x_IDBK | PCM512x_IDSK | PCM512x_IDCH | PCM512x_DCAS);
        if ret != 0 { dev_err!((*component).dev, "Failed to ignore auto-clock failures: %d\n", ret); return ret; }
    } else {
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_ERROR_DETECT,
                                 PCM512x_IDFS | PCM512x_IDBK | PCM512x_IDSK | PCM512x_IDCH | PCM512x_IDCM | PCM512x_DCAS | PCM512x_IPLK,
                                 PCM512x_IDFS | PCM512x_IDBK | PCM512x_IDSK | PCM512x_IDCH | PCM512x_DCAS | PCM512x_IPLK);
        if ret != 0 { dev_err!((*component).dev, "Failed to ignore auto-clock failures: %d\n", ret); return ret; }
        if (*pcm512x).force_pll_on == 0 {
            ret = regmap_update_bits((*pcm512x).regmap, PCM512x_PLL_EN, PCM512x_PLLE, 0);
        } else {
            ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_0, 0x01);
            if ret != 0 { dev_err!((*component).dev, "Failed to set pll coefficient: %d\n", ret); return ret; }
            ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_COEFF_1, 0x04);
            if ret != 0 { dev_err!((*component).dev, "Failed to set pll coefficient: %d\n", ret); return ret; }
            ret = regmap_write((*pcm512x).regmap, PCM512x_PLL_EN, 0x01);
            dev_dbg!((*component).dev, "Enabling PLL for TAS575x\n");
        }
        if ret != 0 { dev_err!((*component).dev, "Failed to set pll mode: %d\n", ret); return ret; }
    }
    ret = pcm512x_set_dividers(dai, params);
    if ret != 0 { return ret; }
    if (*pcm512x).pll_out != 0 {
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_PLL_REF, PCM512x_SREF, PCM512x_SREF_GPIO);
        if ret != 0 { dev_err!((*component).dev, "Failed to set gpio as pllref: %d\n", ret); return ret; }
        gpio = PCM512x_GREF_GPIO1 as c_int + (*pcm512x).pll_in - 1;
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_GPIO_PLLIN, PCM512x_GREF, gpio as u32);
        if ret != 0 { dev_err!((*component).dev, "Failed to set gpio %d as pllin: %d\n", (*pcm512x).pll_in, ret); return ret; }
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_PLL_EN, PCM512x_PLLE, PCM512x_PLLE);
        if ret != 0 { dev_err!((*component).dev, "Failed to enable pll: %d\n", ret); return ret; }
        gpio = (PCM512x_G1OE << ((*pcm512x).pll_out - 1)) as c_int;
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_GPIO_EN, gpio as u32, gpio as u32);
        if ret != 0 { dev_err!((*component).dev, "Failed to enable gpio %d: %d\n", (*pcm512x).pll_out, ret); return ret; }
        gpio = PCM512x_GPIO_OUTPUT_1 as c_int + (*pcm512x).pll_out - 1;
        ret = regmap_update_bits((*pcm512x).regmap, gpio as u32, PCM512x_GxSL, PCM512x_GxSL_PLLCK);
        if ret != 0 { dev_err!((*component).dev, "Failed to output pll on %d: %d\n", ret, (*pcm512x).pll_out); return ret; }
    }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_SYNCHRONIZE, PCM512x_RQSY, PCM512x_RQSY_HALT);
    if ret != 0 { dev_err!((*component).dev, "Failed to halt clocks: %d\n", ret); return ret; }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_SYNCHRONIZE, PCM512x_RQSY, PCM512x_RQSY_RESUME);
    if ret != 0 { dev_err!((*component).dev, "Failed to resume clocks: %d\n", ret); return ret; }
    0
}

unsafe extern "C" fn pcm512x_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let mut offset = 0;
    let (clock_output, provider_mode) = match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => (0, 0),
        SND_SOC_DAIFMT_CBP_CFP => (PCM512x_BCKO | PCM512x_LRKO, PCM512x_RLRK | PCM512x_RBCK),
        SND_SOC_DAIFMT_CBP_CFC => (PCM512x_BCKO, PCM512x_RBCK),
        _ => return -EINVAL,
    };
    let mut ret = regmap_update_bits((*pcm512x).regmap, PCM512x_BCLK_LRCLK_CFG, PCM512x_BCKP | PCM512x_BCKO | PCM512x_LRKO, clock_output);
    if ret != 0 { dev_err!((*component).dev, "Failed to enable clock output: %d\n", ret); return ret; }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_MASTER_MODE, PCM512x_RLRK | PCM512x_RBCK, provider_mode);
    if ret != 0 { dev_err!((*component).dev, "Failed to enable provider mode: %d\n", ret); return ret; }
    let afmt = match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => PCM512x_AFMT_I2S,
        SND_SOC_DAIFMT_RIGHT_J => PCM512x_AFMT_RTJ,
        SND_SOC_DAIFMT_LEFT_J => PCM512x_AFMT_LTJ,
        SND_SOC_DAIFMT_DSP_A => { offset = 1; PCM512x_AFMT_DSP },
        SND_SOC_DAIFMT_DSP_B => PCM512x_AFMT_DSP,
        _ => { dev_err!((*component).dev, "unsupported DAI format: 0x%x\n", (*pcm512x).fmt); return -EINVAL; }
    };
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_I2S_1, PCM512x_AFMT, afmt);
    if ret != 0 { dev_err!((*component).dev, "Failed to set data format: %d\n", ret); return ret; }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_I2S_2, 0xFF, offset);
    if ret != 0 { dev_err!((*component).dev, "Failed to set data offset: %d\n", ret); return ret; }
    (*pcm512x).fmt = fmt as c_int;
    0
}

unsafe extern "C" fn pcm512x_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: u32) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    if ratio > 256 { return -EINVAL; }
    (*pcm512x).bclk_ratio = ratio;
    0
}

unsafe extern "C" fn pcm512x_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let pcm512x = snd_soc_component_get_drvdata(component) as *mut pcm512x_priv;
    let ret;
    let mut _mute_det: u32 = 0;
    if mute != 0 {
        (*pcm512x).mute |= 0x1;
        ret = regmap_update_bits((*pcm512x).regmap, PCM512x_MUTE, PCM512x_RQML | PCM512x_RQMR, PCM512x_RQML | PCM512x_RQMR);
        if ret != 0 { dev_err!((*component).dev, "Failed to set digital mute: %d\n", ret); return ret; }
        /* regmap_read_poll_timeout(..., (mute_det & 0x3) == 0, 200, 10000); */
    } else {
        (*pcm512x).mute &= !0x1;
        ret = pcm512x_update_mute(pcm512x);
        if ret != 0 { dev_err!((*component).dev, "Failed to update digital mute: %d\n", ret); return ret; }
        /* regmap_read_poll_timeout(..., (mute_det & 0x3) == ((~pcm512x->mute >> 1) & 0x3), 200, 10000); */
    }
    ret
}

static pcm512x_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S |
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J |
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J |
    SND_SOC_POSSIBLE_DAIFMT_DSP_A |
    SND_SOC_POSSIBLE_DAIFMT_DSP_B;

static pcm512x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(pcm512x_dai_startup),
    hw_params: Some(pcm512x_hw_params),
    set_fmt: Some(pcm512x_set_fmt),
    mute_stream: Some(pcm512x_mute),
    set_bclk_ratio: Some(pcm512x_set_bclk_ratio),
    auto_selectable_formats: &pcm512x_selectable_formats,
    num_auto_selectable_formats: 1,
    no_capture_mute: 1,
};

static mut pcm512x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("pcm512x-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 384000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &pcm512x_dai_ops,
};

static pcm512x_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(pcm512x_set_bias_level),
    controls: pcm512x_controls.as_ptr(),
    num_controls: 4,
    dapm_widgets: pcm512x_dapm_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: pcm512x_dapm_routes.as_ptr(),
    num_dapm_routes: 4,
    use_pmdown_time: 1,
    endianness: 1,
};

static pcm512x_range: regmap_range_cfg = regmap_range_cfg {
    name: cstr!("Pages"),
    range_min: PCM512x_VIRT_BASE,
    range_max: PCM512x_MAX_REGISTER,
    selector_reg: PCM512x_PAGE,
    selector_mask: 0xff,
    window_start: 0,
    window_len: 0x100,
};

#[no_mangle]
pub static pcm512x_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    readable_reg: Some(pcm512x_readable),
    volatile_reg: Some(pcm512x_volatile),
    ranges: &pcm512x_range,
    num_ranges: 1,
    max_register: PCM512x_MAX_REGISTER,
    reg_defaults: pcm512x_reg_defaults.as_ptr(),
    num_reg_defaults: 45,
    cache_type: REGCACHE_RBTREE,
};
/* EXPORT_SYMBOL_GPL(pcm512x_regmap); */

#[no_mangle]
pub unsafe extern "C" fn pcm512x_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let pcm512x = devm_kzalloc(dev, size_of::<pcm512x_priv>(), GFP_KERNEL) as *mut pcm512x_priv;
    if pcm512x.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, pcm512x as *mut c_void);
    (*pcm512x).regmap = regmap;
    let mut i = 0;
    while i < PCM512x_NUM_SUPPLIES {
        (*pcm512x).supplies[i].supply = pcm512x_supply_names[i];
        i += 1;
    }
    let mut ret = devm_regulator_bulk_get(dev, PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
    if ret != 0 { dev_err!(dev, "Failed to get supplies: %d\n", ret); return ret; }
    (*pcm512x).supply_nb[0].notifier_call = Some(pcm512x_regulator_event_0);
    (*pcm512x).supply_nb[1].notifier_call = Some(pcm512x_regulator_event_1);
    (*pcm512x).supply_nb[2].notifier_call = Some(pcm512x_regulator_event_2);
    i = 0;
    while i < PCM512x_NUM_SUPPLIES {
        ret = devm_regulator_register_notifier((*pcm512x).supplies[i].consumer, &mut (*pcm512x).supply_nb[i]);
        if ret != 0 { dev_err!(dev, "Failed to register regulator notifier: %d\n", ret); }
        i += 1;
    }
    ret = regulator_bulk_enable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
    if ret != 0 { dev_err!(dev, "Failed to enable supplies: %d\n", ret); return ret; }
    ret = regmap_write(regmap, PCM512x_RESET, PCM512x_RSTM | PCM512x_RSTR);
    if ret != 0 { dev_err!(dev, "Failed to reset device: %d\n", ret); regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret; }
    ret = regmap_write(regmap, PCM512x_RESET, 0);
    if ret != 0 { dev_err!(dev, "Failed to reset device: %d\n", ret); regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret; }
    (*pcm512x).sclk = devm_clk_get(dev, ptr::null());
    if PTR_ERR((*pcm512x).sclk) == -(EPROBE_DEFER as isize) {
        ret = -EPROBE_DEFER;
        regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
        return ret;
    }
    if !IS_ERR((*pcm512x).sclk) {
        ret = clk_prepare_enable((*pcm512x).sclk);
        if ret != 0 { dev_err!(dev, "Failed to enable SCLK: %d\n", ret); regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret; }
    }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_POWER, PCM512x_RQST, PCM512x_RQST);
    if ret != 0 {
        dev_err!(dev, "Failed to request standby: %d\n", ret);
        if !IS_ERR((*pcm512x).sclk) { clk_disable_unprepare((*pcm512x).sclk); }
        regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
        return ret;
    }
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_idle(dev);
    /* CONFIG_OF: parse dev->of_node properties "pll-in", "pll-out", and TAS575x names when OF is enabled. */
    if !(*dev).of_node.is_null() {
        let np = (*dev).of_node;
        let mut val: u32 = 0;
        if of_property_read_u32(np, cstr!("pll-in"), &mut val) >= 0 {
            if val > 6 { dev_err!(dev, "Invalid pll-in\n"); ret = -EINVAL; pm_runtime_disable(dev); if !IS_ERR((*pcm512x).sclk) { clk_disable_unprepare((*pcm512x).sclk); } regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret; }
            (*pcm512x).pll_in = val as c_int;
        }
        if of_property_read_u32(np, cstr!("pll-out"), &mut val) >= 0 {
            if val > 6 { dev_err!(dev, "Invalid pll-out\n"); ret = -EINVAL; pm_runtime_disable(dev); if !IS_ERR((*pcm512x).sclk) { clk_disable_unprepare((*pcm512x).sclk); } regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret; }
            (*pcm512x).pll_out = val as c_int;
        }
        if (((*pcm512x).pll_in == 0) as c_int) != (((*pcm512x).pll_out == 0) as c_int) {
            dev_err!(dev, "Error: both pll-in and pll-out, or none\n");
            ret = -EINVAL; pm_runtime_disable(dev); if !IS_ERR((*pcm512x).sclk) { clk_disable_unprepare((*pcm512x).sclk); } regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret;
        }
        if (*pcm512x).pll_in != 0 && (*pcm512x).pll_in == (*pcm512x).pll_out {
            dev_err!(dev, "Error: pll-in == pll-out\n");
            ret = -EINVAL; pm_runtime_disable(dev); if !IS_ERR((*pcm512x).sclk) { clk_disable_unprepare((*pcm512x).sclk); } regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr()); return ret;
        }
        if strcmp((*np).name, cstr!("tas5756")) == 0 || strcmp((*np).name, cstr!("tas5754")) == 0 {
            (*pcm512x).force_pll_on = 1;
        }
        dev_dbg!(dev, "Device ID: %s\n", (*np).name);
    }
    ret = devm_snd_soc_register_component(dev, &pcm512x_component_driver, &mut pcm512x_dai, 1);
    if ret != 0 {
        dev_err!(dev, "Failed to register CODEC: %d\n", ret);
        pm_runtime_disable(dev);
        if !IS_ERR((*pcm512x).sclk) { clk_disable_unprepare((*pcm512x).sclk); }
        regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
        return ret;
    }
    0
}
/* EXPORT_SYMBOL_GPL(pcm512x_probe); */

#[no_mangle]
pub unsafe extern "C" fn pcm512x_remove(dev: *mut device) {
    let pcm512x = dev_get_drvdata(dev) as *mut pcm512x_priv;
    pm_runtime_disable(dev);
    if !IS_ERR((*pcm512x).sclk) {
        clk_disable_unprepare((*pcm512x).sclk);
    }
    regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
}
/* EXPORT_SYMBOL_GPL(pcm512x_remove); */

unsafe extern "C" fn pcm512x_suspend(dev: *mut device) -> c_int {
    let pcm512x = dev_get_drvdata(dev) as *mut pcm512x_priv;
    let mut ret = regmap_update_bits((*pcm512x).regmap, PCM512x_POWER, PCM512x_RQPD, PCM512x_RQPD);
    if ret != 0 { dev_err!(dev, "Failed to request power down: %d\n", ret); return ret; }
    ret = regulator_bulk_disable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
    if ret != 0 { dev_err!(dev, "Failed to disable supplies: %d\n", ret); return ret; }
    if !IS_ERR((*pcm512x).sclk) {
        clk_disable_unprepare((*pcm512x).sclk);
    }
    0
}

unsafe extern "C" fn pcm512x_resume(dev: *mut device) -> c_int {
    let pcm512x = dev_get_drvdata(dev) as *mut pcm512x_priv;
    let mut ret;
    if !IS_ERR((*pcm512x).sclk) {
        ret = clk_prepare_enable((*pcm512x).sclk);
        if ret != 0 { dev_err!(dev, "Failed to enable SCLK: %d\n", ret); return ret; }
    }
    ret = regulator_bulk_enable(PCM512x_NUM_SUPPLIES as c_int, (*pcm512x).supplies.as_mut_ptr());
    if ret != 0 { dev_err!(dev, "Failed to enable supplies: %d\n", ret); return ret; }
    regcache_cache_only((*pcm512x).regmap, false);
    ret = regcache_sync((*pcm512x).regmap);
    if ret != 0 { dev_err!(dev, "Failed to sync cache: %d\n", ret); return ret; }
    ret = regmap_update_bits((*pcm512x).regmap, PCM512x_POWER, PCM512x_RQPD, 0);
    if ret != 0 { dev_err!(dev, "Failed to remove power down: %d\n", ret); return ret; }
    0
}

/* EXPORT_GPL_DEV_PM_OPS(pcm512x_pm_ops) = {
 *	RUNTIME_PM_OPS(pcm512x_suspend, pcm512x_resume, NULL)
 * };
 *
 * MODULE_DESCRIPTION("ASoC PCM512x codec driver");
 * MODULE_AUTHOR("Mark Brown <broonie@kernel.org>");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
