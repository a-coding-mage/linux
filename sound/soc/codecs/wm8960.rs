// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8960.c  --  WM8960 ALSA SoC Audio driver
 *
 * Copyright 2007-11 Wolfson Microelectronics, plc
 *
 * Author: Liam Girdwood
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

/* Includes translated as external dependencies:
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
 * linux/pm.h, linux/clk.h, linux/i2c.h, linux/acpi.h, linux/slab.h,
 * sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/initval.h, sound/tlv.h, sound/wm8960.h, "wm8960.h"
 */

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type ktime_t = i64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;

/* Register and format constants are supplied by the translated headers. */
extern "C" {
    static mut WM8960_RESET: c_uint;
    static mut WM8960_DACCTL1: c_uint;
    static mut WM8960_DACCTL2: c_uint;
    static mut WM8960_3D: c_uint;
    static mut WM8960_ALC1: c_uint;
    static mut WM8960_ALC2: c_uint;
    static mut WM8960_ALC3: c_uint;
    static mut WM8960_ADDCTL1: c_uint;
    static mut WM8960_ADDCTL2: c_uint;
    static mut WM8960_ADDCTL3: c_uint;
    static mut WM8960_ADDCTL4: c_uint;
    static mut WM8960_LINVOL: c_uint;
    static mut WM8960_RINVOL: c_uint;
    static mut WM8960_INBMIX1: c_uint;
    static mut WM8960_INBMIX2: c_uint;
    static mut WM8960_RINPATH: c_uint;
    static mut WM8960_LINPATH: c_uint;
    static mut WM8960_LDAC: c_uint;
    static mut WM8960_RDAC: c_uint;
    static mut WM8960_LOUT1: c_uint;
    static mut WM8960_ROUT1: c_uint;
    static mut WM8960_LOUT2: c_uint;
    static mut WM8960_ROUT2: c_uint;
    static mut WM8960_CLASSD1: c_uint;
    static mut WM8960_CLASSD3: c_uint;
    static mut WM8960_NOISEG: c_uint;
    static mut WM8960_LADC: c_uint;
    static mut WM8960_RADC: c_uint;
    static mut WM8960_BYPASS1: c_uint;
    static mut WM8960_BYPASS2: c_uint;
    static mut WM8960_LOUTMIX: c_uint;
    static mut WM8960_ROUTMIX: c_uint;
    static mut WM8960_MONOMIX1: c_uint;
    static mut WM8960_MONOMIX2: c_uint;
    static mut WM8960_POWER1: c_uint;
    static mut WM8960_POWER2: c_uint;
    static mut WM8960_POWER3: c_uint;
    static mut WM8960_IFACE1: c_uint;
    static mut WM8960_IFACE2: c_uint;
    static mut WM8960_CLOCK1: c_uint;
    static mut WM8960_CLOCK2: c_uint;
    static mut WM8960_APOP1: c_uint;
    static mut WM8960_APOP2: c_uint;
    static mut WM8960_PLL1: c_uint;
    static mut WM8960_PLL2: c_uint;
    static mut WM8960_PLL3: c_uint;
    static mut WM8960_PLL4: c_uint;

    static mut WM8960_SYSCLK_MCLK: c_int;
    static mut WM8960_SYSCLK_PLL: c_int;
    static mut WM8960_SYSCLK_AUTO: c_int;
    static mut WM8960_SYSCLKDIV: c_int;
    static mut WM8960_DACDIV: c_int;
    static mut WM8960_OPCLKDIV: c_int;
    static mut WM8960_DCLKDIV: c_int;
    static mut WM8960_TOCLKSEL: c_int;

    static mut SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static mut SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static mut SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static mut SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static mut SND_SOC_DAIFMT_I2S: c_uint;
    static mut SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static mut SND_SOC_DAIFMT_LEFT_J: c_uint;
    static mut SND_SOC_DAIFMT_DSP_A: c_uint;
    static mut SND_SOC_DAIFMT_DSP_B: c_uint;
    static mut SND_SOC_DAIFMT_INV_MASK: c_uint;
    static mut SND_SOC_DAIFMT_NB_NF: c_uint;
    static mut SND_SOC_DAIFMT_IB_IF: c_uint;
    static mut SND_SOC_DAIFMT_IB_NF: c_uint;
    static mut SND_SOC_DAIFMT_NB_IF: c_uint;
    static mut SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static mut SNDRV_PCM_RATE_8000_48000: c_uint;
    static mut SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static mut SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static mut SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static mut SNDRV_PCM_FMTBIT_S32_LE: c_uint;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_card { pub widgets: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub name: *const c_char,
    pub power: c_int,
    pub list: list_head,
}
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char }
#[repr(C)] #[derive(Copy, Clone)] pub struct wm8960_data {
    pub capless: bool,
    pub shared_lrclk: bool,
    pub gpio_cfg: [u32; 2],
    pub hp_cfg: [u32; 3],
}
#[repr(C)] pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}
#[repr(C)] pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)] pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}
#[repr(C)] pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)] pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)] pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 20], pub driver_data: c_ulonglong }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct acpi_device_id { pub id: [c_char; 16], pub driver_data: c_ulonglong }
#[repr(C)] pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}
#[repr(C)] pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget, num: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_int;
    fn msleep(msecs: ktime_t);
    fn ktime_get() -> ktime_t;
    fn ktime_ms_delta(later: ktime_t, earlier: ktime_t) -> ktime_t;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut wm8960_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn of_property_read_bool(np: *const device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32_array(np: *const device_node, propname: *const c_char, out_values: *mut u32, sz: usize) -> c_int;
    fn i2c_master_recv(client: *mut i2c_client, buf: *mut u8, count: c_int) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
    fn ACPI_PTR(ptr: *const acpi_device_id) -> *const acpi_device_id;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a).len() as c_int
    };
}

/* R25 - Power 1 */
const WM8960_VMID_MASK: c_uint = 0x180;
const WM8960_VREF: c_uint = 0x40;

/* R26 - Power 2 */
const WM8960_PWR2_LOUT1: c_int = 0x40;
const WM8960_PWR2_ROUT1: c_int = 0x20;
const WM8960_PWR2_OUT3: c_int = 0x02;

/* R28 - Anti-pop 1 */
const WM8960_POBCTRL: c_uint = 0x80;
const WM8960_BUFDCOPEN: c_uint = 0x10;
const WM8960_BUFIOEN: c_uint = 0x08;
const WM8960_SOFT_ST: c_uint = 0x04;
const WM8960_HPSTBY: c_uint = 0x01;

/* R29 - Anti-pop 2 */
const WM8960_DISOP: c_uint = 0x40;
const WM8960_DRES_MASK: c_uint = 0x30;

const WM8960_DSCH_TOUT: ktime_t = 600; /* discharge timeout, ms */

const WM8960_NUM_SUPPLIES: usize = 5;
static wm8960_supply_names: [*const c_char; WM8960_NUM_SUPPLIES] = [
    cstr!("DCVDD"),
    cstr!("DBVDD"),
    cstr!("AVDD"),
    cstr!("SPKVDD1"),
    cstr!("SPKVDD2"),
];

#[repr(C)]
struct wm8960_priv {
    mclk: *mut clk,
    regmap: *mut regmap,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    lout1: *mut snd_soc_dapm_widget,
    rout1: *mut snd_soc_dapm_widget,
    out3: *mut snd_soc_dapm_widget,
    deemph: bool,
    lrclk: c_int,
    bclk: c_int,
    sysclk: c_int,
    clk_id: c_int,
    freq_in: c_int,
    is_stream_in_use: [bool; 2],
    pdata: wm8960_data,
    dsch_start: ktime_t,
    supplies: [regulator_bulk_data; WM8960_NUM_SUPPLIES],
}

unsafe fn wm8960_reset(c: *mut regmap) -> c_int {
    regmap_write(c, WM8960_RESET, 0)
}

/* Enumerated controls. */
static wm8960_polarity: [*const c_char; 4] = [cstr!("No Inversion"), cstr!("Left Inverted"), cstr!("Right Inverted"), cstr!("Stereo Inversion")];
static wm8960_3d_upper_cutoff: [*const c_char; 2] = [cstr!("High"), cstr!("Low")];
static wm8960_3d_lower_cutoff: [*const c_char; 2] = [cstr!("Low"), cstr!("High")];
static wm8960_alcfunc: [*const c_char; 4] = [cstr!("Off"), cstr!("Right"), cstr!("Left"), cstr!("Stereo")];
static wm8960_alcmode: [*const c_char; 2] = [cstr!("ALC"), cstr!("Limiter")];
static wm8960_adc_data_output_sel: [*const c_char; 4] = [
    cstr!("Left Data = Left ADC;  Right Data = Right ADC"),
    cstr!("Left Data = Left ADC;  Right Data = Left ADC"),
    cstr!("Left Data = Right ADC; Right Data = Right ADC"),
    cstr!("Left Data = Right ADC; Right Data = Left ADC"),
];
static wm8960_dmonomix: [*const c_char; 2] = [cstr!("Stereo"), cstr!("Mono")];
static wm8960_dacslope: [*const c_char; 2] = [cstr!("Normal"), cstr!("Sloping")];

/* C SOC_ENUM_SINGLE initializers depend on external ALSA macro layout. */
extern "C" {
    static wm8960_enum: [soc_enum; 9];
    static adc_tlv: [c_uint; 0];
    static inpga_tlv: [c_uint; 0];
    static dac_tlv: [c_uint; 0];
    static bypass_tlv: [c_uint; 0];
    static out_tlv: [c_uint; 0];
    static lineinboost_tlv: [c_uint; 0];
    static micboost_tlv: [c_uint; 0];
    static wm8960_snd_controls: [snd_kcontrol_new; 0];
    static wm8960_lin_boost: [snd_kcontrol_new; 0];
    static wm8960_lin: [snd_kcontrol_new; 0];
    static wm8960_rin_boost: [snd_kcontrol_new; 0];
    static wm8960_rin: [snd_kcontrol_new; 0];
    static wm8960_loutput_mixer: [snd_kcontrol_new; 0];
    static wm8960_routput_mixer: [snd_kcontrol_new; 0];
    static wm8960_mono_out: [snd_kcontrol_new; 0];
    static wm8960_dapm_widgets: [snd_soc_dapm_widget; 0];
    static wm8960_dapm_widgets_out3: [snd_soc_dapm_widget; 0];
    static wm8960_dapm_widgets_capless: [snd_soc_dapm_widget; 0];
}

static wm8960_reg_defaults: [reg_default; 56] = [
    reg_default { reg: 0x0, def: 0x00a7 }, reg_default { reg: 0x1, def: 0x00a7 },
    reg_default { reg: 0x2, def: 0x0000 }, reg_default { reg: 0x3, def: 0x0000 },
    reg_default { reg: 0x4, def: 0x0000 }, reg_default { reg: 0x5, def: 0x0008 },
    reg_default { reg: 0x6, def: 0x0000 }, reg_default { reg: 0x7, def: 0x000a },
    reg_default { reg: 0x8, def: 0x01c0 }, reg_default { reg: 0x9, def: 0x0000 },
    reg_default { reg: 0xa, def: 0x00ff }, reg_default { reg: 0xb, def: 0x00ff },
    reg_default { reg: 0x10, def: 0x0000 }, reg_default { reg: 0x11, def: 0x007b },
    reg_default { reg: 0x12, def: 0x0100 }, reg_default { reg: 0x13, def: 0x0032 },
    reg_default { reg: 0x14, def: 0x0000 }, reg_default { reg: 0x15, def: 0x00c3 },
    reg_default { reg: 0x16, def: 0x00c3 }, reg_default { reg: 0x17, def: 0x01c0 },
    reg_default { reg: 0x18, def: 0x0000 }, reg_default { reg: 0x19, def: 0x0000 },
    reg_default { reg: 0x1a, def: 0x0000 }, reg_default { reg: 0x1b, def: 0x0000 },
    reg_default { reg: 0x1c, def: 0x0000 }, reg_default { reg: 0x1d, def: 0x0000 },
    reg_default { reg: 0x20, def: 0x0100 }, reg_default { reg: 0x21, def: 0x0100 },
    reg_default { reg: 0x22, def: 0x0050 }, reg_default { reg: 0x25, def: 0x0050 },
    reg_default { reg: 0x26, def: 0x0000 }, reg_default { reg: 0x27, def: 0x0000 },
    reg_default { reg: 0x28, def: 0x0000 }, reg_default { reg: 0x29, def: 0x0000 },
    reg_default { reg: 0x2a, def: 0x0040 }, reg_default { reg: 0x2b, def: 0x0000 },
    reg_default { reg: 0x2c, def: 0x0000 }, reg_default { reg: 0x2d, def: 0x0050 },
    reg_default { reg: 0x2e, def: 0x0050 }, reg_default { reg: 0x2f, def: 0x0000 },
    reg_default { reg: 0x30, def: 0x0002 }, reg_default { reg: 0x31, def: 0x0037 },
    reg_default { reg: 0x33, def: 0x0080 }, reg_default { reg: 0x34, def: 0x0008 },
    reg_default { reg: 0x35, def: 0x0031 }, reg_default { reg: 0x36, def: 0x0026 },
    reg_default { reg: 0x37, def: 0x00e9 },
];

unsafe extern "C" fn wm8960_volatile(_dev: *mut device, reg: c_uint) -> bool {
    if reg == WM8960_RESET { true } else { false }
}

static deemph_settings: [c_int; 4] = [0, 32000, 44100, 48000];

unsafe extern "C" fn wm8960_set_deemph(component: *mut snd_soc_component) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let mut val: c_int;
    if (*wm8960).deemph {
        let mut best = 1usize;
        let mut i = 2usize;
        while i < deemph_settings.len() {
            if (deemph_settings[i] - (*wm8960).lrclk).abs() < (deemph_settings[best] - (*wm8960).lrclk).abs() {
                best = i;
            }
            i += 1;
        }
        val = (best as c_int) << 1;
    } else {
        val = 0;
    }
    dev_dbg((*component).dev, cstr!("Set deemphasis %d\n"), val);
    snd_soc_component_update_bits(component, WM8960_DACCTL1, 0x6, val as c_uint)
}

unsafe extern "C" fn wm8960_get_deemph(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    (*ucontrol).value.integer.value[0] = (*wm8960).deemph as i64;
    0
}

unsafe extern "C" fn wm8960_put_deemph(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let deemph = (*ucontrol).value.integer.value[0] as c_uint;
    if deemph > 1 { return -EINVAL; }
    (*wm8960).deemph = deemph != 0;
    wm8960_set_deemph(component)
}

static audio_paths: [snd_soc_dapm_route; 34] = [
    snd_soc_dapm_route { sink: cstr!("Left Boost Mixer"), control: cstr!("LINPUT1 Switch"), source: cstr!("LINPUT1") },
    snd_soc_dapm_route { sink: cstr!("Left Boost Mixer"), control: cstr!("LINPUT2 Switch"), source: cstr!("LINPUT2") },
    snd_soc_dapm_route { sink: cstr!("Left Boost Mixer"), control: cstr!("LINPUT3 Switch"), source: cstr!("LINPUT3") },
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: cstr!("Boost Switch"), source: cstr!("Left Boost Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: cstr!("Boost Switch"), source: cstr!("LINPUT1") }, /* Really Boost Switch */
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: ptr::null(), source: cstr!("LINPUT2") },
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: ptr::null(), source: cstr!("LINPUT3") },
    snd_soc_dapm_route { sink: cstr!("Right Boost Mixer"), control: cstr!("RINPUT1 Switch"), source: cstr!("RINPUT1") },
    snd_soc_dapm_route { sink: cstr!("Right Boost Mixer"), control: cstr!("RINPUT2 Switch"), source: cstr!("RINPUT2") },
    snd_soc_dapm_route { sink: cstr!("Right Boost Mixer"), control: cstr!("RINPUT3 Switch"), source: cstr!("RINPUT3") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: cstr!("Boost Switch"), source: cstr!("Right Boost Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: cstr!("Boost Switch"), source: cstr!("RINPUT1") }, /* Really Boost Switch */
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: ptr::null(), source: cstr!("RINPUT2") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: ptr::null(), source: cstr!("RINPUT3") },
    snd_soc_dapm_route { sink: cstr!("Left ADC"), control: ptr::null(), source: cstr!("Left Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right ADC"), control: ptr::null(), source: cstr!("Right Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Output Mixer"), control: cstr!("LINPUT3 Switch"), source: cstr!("LINPUT3") },
    snd_soc_dapm_route { sink: cstr!("Left Output Mixer"), control: cstr!("Boost Bypass Switch"), source: cstr!("Left Boost Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Output Mixer"), control: cstr!("PCM Playback Switch"), source: cstr!("Left DAC") },
    snd_soc_dapm_route { sink: cstr!("Right Output Mixer"), control: cstr!("RINPUT3 Switch"), source: cstr!("RINPUT3") },
    snd_soc_dapm_route { sink: cstr!("Right Output Mixer"), control: cstr!("Boost Bypass Switch"), source: cstr!("Right Boost Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Output Mixer"), control: cstr!("PCM Playback Switch"), source: cstr!("Right DAC") },
    snd_soc_dapm_route { sink: cstr!("LOUT1 PGA"), control: ptr::null(), source: cstr!("Left Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("ROUT1 PGA"), control: ptr::null(), source: cstr!("Right Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("HP_L"), control: ptr::null(), source: cstr!("LOUT1 PGA") },
    snd_soc_dapm_route { sink: cstr!("HP_R"), control: ptr::null(), source: cstr!("ROUT1 PGA") },
    snd_soc_dapm_route { sink: cstr!("Left Speaker PGA"), control: ptr::null(), source: cstr!("Left Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Speaker PGA"), control: ptr::null(), source: cstr!("Right Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Speaker Output"), control: ptr::null(), source: cstr!("Left Speaker PGA") },
    snd_soc_dapm_route { sink: cstr!("Right Speaker Output"), control: ptr::null(), source: cstr!("Right Speaker PGA") },
    snd_soc_dapm_route { sink: cstr!("SPK_LN"), control: ptr::null(), source: cstr!("Left Speaker Output") },
    snd_soc_dapm_route { sink: cstr!("SPK_LP"), control: ptr::null(), source: cstr!("Left Speaker Output") },
    snd_soc_dapm_route { sink: cstr!("SPK_RN"), control: ptr::null(), source: cstr!("Right Speaker Output") },
    snd_soc_dapm_route { sink: cstr!("SPK_RP"), control: ptr::null(), source: cstr!("Right Speaker Output") },
];

static audio_paths_out3: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: cstr!("Mono Output Mixer"), control: cstr!("Left Switch"), source: cstr!("Left Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("Mono Output Mixer"), control: cstr!("Right Switch"), source: cstr!("Right Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("OUT3"), control: ptr::null(), source: cstr!("Mono Output Mixer") },
];

static audio_paths_capless: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: cstr!("HP_L"), control: ptr::null(), source: cstr!("OUT3 VMID") },
    snd_soc_dapm_route { sink: cstr!("HP_R"), control: ptr::null(), source: cstr!("OUT3 VMID") },
    snd_soc_dapm_route { sink: cstr!("OUT3 VMID"), control: ptr::null(), source: cstr!("Left Output Mixer") },
    snd_soc_dapm_route { sink: cstr!("OUT3 VMID"), control: ptr::null(), source: cstr!("Right Output Mixer") },
];

unsafe extern "C" fn wm8960_add_widgets(component: *mut snd_soc_component) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let pdata = &mut (*wm8960).pdata as *mut wm8960_data;
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_new_controls(dapm, wm8960_dapm_widgets.as_ptr(), 0);
    snd_soc_dapm_add_routes(dapm, audio_paths.as_ptr(), ARRAY_SIZE!(audio_paths));
    if !pdata.is_null() && (*pdata).capless {
        snd_soc_dapm_new_controls(dapm, wm8960_dapm_widgets_capless.as_ptr(), 0);
        snd_soc_dapm_add_routes(dapm, audio_paths_capless.as_ptr(), ARRAY_SIZE!(audio_paths_capless));
    } else {
        snd_soc_dapm_new_controls(dapm, wm8960_dapm_widgets_out3.as_ptr(), 0);
        snd_soc_dapm_add_routes(dapm, audio_paths_out3.as_ptr(), ARRAY_SIZE!(audio_paths_out3));
    }
    /* Original C scans component->card->widgets with list_for_each_entry to cache LOUT1 PGA,
     * ROUT1 PGA, and OUT3 VMID widget pointers. The Linux list container mapping is external.
     */
    0
}

unsafe extern "C" fn wm8960_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => iface |= 0x0040,
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => iface |= 0x0002,
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => iface |= 0x0001,
        x if x == SND_SOC_DAIFMT_DSP_A => iface |= 0x0003,
        x if x == SND_SOC_DAIFMT_DSP_B => iface |= 0x0013,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => iface |= 0x0090,
        x if x == SND_SOC_DAIFMT_IB_NF => iface |= 0x0080,
        x if x == SND_SOC_DAIFMT_NB_IF => iface |= 0x0010,
        _ => return -EINVAL,
    }
    snd_soc_component_write(component, WM8960_IFACE1, iface as c_uint);
    0
}

#[repr(C)] struct alc_rate { rate: c_int, val: c_uint }
static alc_rates: [alc_rate; 9] = [
    alc_rate { rate: 48000, val: 0 }, alc_rate { rate: 44100, val: 0 },
    alc_rate { rate: 32000, val: 1 }, alc_rate { rate: 22050, val: 2 },
    alc_rate { rate: 24000, val: 2 }, alc_rate { rate: 16000, val: 3 },
    alc_rate { rate: 11025, val: 4 }, alc_rate { rate: 12000, val: 4 },
    alc_rate { rate: 8000, val: 5 },
];

/* -1 for reserved value */
static sysclk_divs: [c_int; 4] = [1, -1, 2, -1];
/* Multiply 256 for internal 256 div */
static dac_divs: [c_int; 7] = [256, 384, 512, 768, 1024, 1408, 1536];
/* Multiply 10 to eliminate decimials */
static bclk_divs: [c_int; 16] = [10, 15, 20, 30, 40, 55, 60, 80, 110, 120, 160, 220, 240, 320, 320, 320];

unsafe extern "C" fn wm8960_configure_sysclk(wm8960: *mut wm8960_priv, mclk: c_int, sysclk_idx: *mut c_int, dac_idx: *mut c_int, bclk_idx: *mut c_int) -> c_int {
    *bclk_idx = -1;
    let bclk = (*wm8960).bclk;
    let lrclk = (*wm8960).lrclk;
    let mut i = 0usize;
    while i < sysclk_divs.len() {
        if sysclk_divs[i] != -1 {
            let sysclk = mclk / sysclk_divs[i];
            let mut j = 0usize;
            while j < dac_divs.len() {
                if sysclk == dac_divs[j] * lrclk {
                    let mut k = 0usize;
                    while k < bclk_divs.len() {
                        let diff = sysclk - bclk * bclk_divs[k] / 10;
                        if diff == 0 {
                            *sysclk_idx = i as c_int;
                            *dac_idx = j as c_int;
                            *bclk_idx = k as c_int;
                            break;
                        }
                        k += 1;
                    }
                    if k != bclk_divs.len() { break; }
                }
                j += 1;
            }
            if j != dac_divs.len() { break; }
        }
        i += 1;
    }
    *bclk_idx
}

unsafe extern "C" fn wm8960_configure_pll(component: *mut snd_soc_component, freq_in: c_int, sysclk_idx: *mut c_int, dac_idx: *mut c_int, bclk_idx: *mut c_int) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let bclk = (*wm8960).bclk;
    let lrclk = (*wm8960).lrclk;
    let mut closest = freq_in;
    let mut best_freq_out = -EINVAL;
    *sysclk_idx = -1; *dac_idx = -1; *bclk_idx = -1;
    let mut i = sysclk_divs.len() as isize - 1;
    while i >= 0 {
        if sysclk_divs[i as usize] != -1 {
            let mut j = 0usize;
            while j < dac_divs.len() {
                let sysclk = lrclk * dac_divs[j];
                let freq_out = sysclk * sysclk_divs[i as usize];
                let mut k = 0usize;
                while k < bclk_divs.len() {
                    if is_pll_freq_available(freq_in as c_uint, freq_out as c_uint) {
                        let diff = sysclk - bclk * bclk_divs[k] / 10;
                        if diff == 0 {
                            *sysclk_idx = i as c_int; *dac_idx = j as c_int; *bclk_idx = k as c_int;
                            return freq_out;
                        }
                        if diff > 0 && closest > diff {
                            *sysclk_idx = i as c_int; *dac_idx = j as c_int; *bclk_idx = k as c_int;
                            closest = diff;
                            best_freq_out = freq_out;
                        }
                    }
                    k += 1;
                }
                j += 1;
            }
        }
        i -= 1;
    }
    best_freq_out
}

unsafe extern "C" fn wm8960_configure_clocking(component: *mut snd_soc_component) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let mut freq_out: c_int;
    let freq_in: c_int;
    let iface1 = snd_soc_component_read(component, WM8960_IFACE1) as u16;
    let mut i = 0; let mut j = 0; let mut k = 0;
    if (iface1 & (1 << 6)) == 0 && (*wm8960).sysclk == 0 {
        dev_warn((*component).dev, cstr!("slave mode, but proceeding with no clock configuration\n"));
        return 0;
    }
    if (*wm8960).clk_id != WM8960_SYSCLK_MCLK && (*wm8960).freq_in == 0 {
        dev_err((*component).dev, cstr!("No MCLK configured\n"));
        return -EINVAL;
    }
    freq_in = (*wm8960).freq_in;
    if (*wm8960).clk_id == WM8960_SYSCLK_AUTO {
        wm8960_set_pll(component, 0, 0);
        freq_out = freq_in;
    } else if (*wm8960).sysclk != 0 {
        freq_out = (*wm8960).sysclk;
    } else {
        dev_err((*component).dev, cstr!("No SYSCLK configured\n"));
        return -EINVAL;
    }
    if (*wm8960).clk_id != WM8960_SYSCLK_PLL {
        let ret = wm8960_configure_sysclk(wm8960, freq_out, &mut i, &mut j, &mut k);
        if ret < 0 {
            if (*wm8960).clk_id != WM8960_SYSCLK_AUTO {
                dev_err((*component).dev, cstr!("failed to configure clock\n"));
                return -EINVAL;
            }
        } else {
            snd_soc_component_update_bits(component, WM8960_CLOCK1, 3 << 1, (i << 1) as c_uint);
            snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x7 << 3, (j << 3) as c_uint);
            snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x7 << 6, (j << 6) as c_uint);
            snd_soc_component_update_bits(component, WM8960_CLOCK2, 0xf, k as c_uint);
            return 0;
        }
    }
    freq_out = wm8960_configure_pll(component, freq_in, &mut i, &mut j, &mut k);
    if freq_out < 0 {
        dev_err((*component).dev, cstr!("failed to configure clock via PLL\n"));
        return freq_out;
    }
    wm8960_set_pll(component, freq_in as c_uint, freq_out as c_uint);
    snd_soc_component_update_bits(component, WM8960_CLOCK1, 3 << 1, (i << 1) as c_uint);
    snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x7 << 3, (j << 3) as c_uint);
    snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x7 << 6, (j << 6) as c_uint);
    snd_soc_component_update_bits(component, WM8960_CLOCK2, 0xf, k as c_uint);
    0
}

unsafe extern "C" fn wm8960_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let mut iface = (snd_soc_component_read(component, WM8960_IFACE1) & 0xfff3) as u16;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    (*wm8960).bclk = snd_soc_params_to_bclk(params);
    if params_channels(params) == 1 { (*wm8960).bclk *= 2; }
    match params_width(params) {
        16 => {}
        20 => iface |= 0x0004,
        24 => iface |= 0x0008,
        32 => {
            if (iface & 0x3) != 0 { iface |= 0x000c; } else {
                dev_err((*component).dev, cstr!("unsupported width %d\n"), params_width(params));
                return -EINVAL;
            }
        }
        _ => {
            dev_err((*component).dev, cstr!("unsupported width %d\n"), params_width(params));
            return -EINVAL;
        }
    }
    (*wm8960).lrclk = params_rate(params);
    if tx {
        wm8960_set_deemph(component);
    } else {
        let mut i = 0usize;
        while i < alc_rates.len() {
            if alc_rates[i].rate == params_rate(params) {
                snd_soc_component_update_bits(component, WM8960_ADDCTL3, 0x7, alc_rates[i].val);
            }
            i += 1;
        }
    }
    snd_soc_component_write(component, WM8960_IFACE1, iface as c_uint);
    (*wm8960).is_stream_in_use[tx as usize] = true;
    if !(*wm8960).is_stream_in_use[(!tx) as usize] {
        return wm8960_configure_clocking(component);
    }
    0
}

unsafe extern "C" fn wm8960_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    (*wm8960).is_stream_in_use[tx as usize] = false;
    0
}

unsafe extern "C" fn wm8960_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    if mute != 0 {
        snd_soc_component_update_bits(component, WM8960_DACCTL1, 0x8, 0x8);
    } else {
        snd_soc_component_update_bits(component, WM8960_DACCTL1, 0x8, 0);
    }
    0
}

unsafe extern "C" fn wm8960_set_bias_level_out3(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let pm2 = snd_soc_component_read(component, WM8960_POWER2) as u16;
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => match snd_soc_dapm_get_bias_level(dapm) {
            snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
                if !IS_ERR((*wm8960).mclk as *const c_void) {
                    let ret = clk_prepare_enable((*wm8960).mclk);
                    if ret != 0 { dev_err((*component).dev, cstr!("Failed to enable MCLK: %d\n"), ret); return ret; }
                }
                let ret = wm8960_configure_clocking(component);
                if ret != 0 { return ret; }
                snd_soc_component_update_bits(component, WM8960_POWER1, 0x180, 0x80);
            }
            snd_soc_bias_level::SND_SOC_BIAS_ON => {
                if (*wm8960).clk_id == WM8960_SYSCLK_AUTO && (pm2 & 0x1) != 0 { wm8960_set_pll(component, 0, 0); }
                if !IS_ERR((*wm8960).mclk as *const c_void) { clk_disable_unprepare((*wm8960).mclk); }
            }
            _ => {}
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if matches!(snd_soc_dapm_get_bias_level(dapm), snd_soc_bias_level::SND_SOC_BIAS_OFF) {
                let tout = WM8960_DSCH_TOUT - ktime_ms_delta(ktime_get(), (*wm8960).dsch_start);
                if tout > 0 { msleep(tout); }
                regcache_sync((*wm8960).regmap);
                snd_soc_component_write(component, WM8960_APOP1, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN | WM8960_BUFIOEN);
                snd_soc_component_update_bits(component, WM8960_POWER1, 0x80, 0x80);
                msleep(100);
                snd_soc_component_update_bits(component, WM8960_POWER1, WM8960_VREF, WM8960_VREF);
                snd_soc_component_write(component, WM8960_APOP1, WM8960_BUFIOEN);
            }
            snd_soc_component_update_bits(component, WM8960_POWER1, 0x180, 0x100);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, WM8960_APOP1, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN | WM8960_BUFIOEN);
            snd_soc_component_write(component, WM8960_POWER1, 0);
            (*wm8960).dsch_start = ktime_get();
        }
    }
    0
}

unsafe extern "C" fn wm8960_set_bias_level_capless(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let pm2 = snd_soc_component_read(component, WM8960_POWER2) as u16;
    let mut reg: c_int;
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => match snd_soc_dapm_get_bias_level(dapm) {
            snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
                snd_soc_component_update_bits(component, WM8960_APOP1, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN);
                reg = 0;
                if !(*wm8960).lout1.is_null() && (*(*wm8960).lout1).power != 0 { reg |= WM8960_PWR2_LOUT1; }
                if !(*wm8960).rout1.is_null() && (*(*wm8960).rout1).power != 0 { reg |= WM8960_PWR2_ROUT1; }
                if !(*wm8960).out3.is_null() && (*(*wm8960).out3).power != 0 { reg |= WM8960_PWR2_OUT3; }
                snd_soc_component_update_bits(component, WM8960_POWER2, (WM8960_PWR2_LOUT1 | WM8960_PWR2_ROUT1 | WM8960_PWR2_OUT3) as c_uint, reg as c_uint);
                snd_soc_component_update_bits(component, WM8960_POWER1, WM8960_VMID_MASK, 0x80);
                msleep(100);
                snd_soc_component_update_bits(component, WM8960_POWER1, WM8960_VREF, WM8960_VREF);
                msleep(100);
                if !IS_ERR((*wm8960).mclk as *const c_void) {
                    let ret = clk_prepare_enable((*wm8960).mclk);
                    if ret != 0 { dev_err((*component).dev, cstr!("Failed to enable MCLK: %d\n"), ret); return ret; }
                }
                let ret = wm8960_configure_clocking(component);
                if ret != 0 { return ret; }
            }
            snd_soc_bias_level::SND_SOC_BIAS_ON => {
                if (*wm8960).clk_id == WM8960_SYSCLK_AUTO && (pm2 & 0x1) != 0 { wm8960_set_pll(component, 0, 0); }
                if !IS_ERR((*wm8960).mclk as *const c_void) { clk_disable_unprepare((*wm8960).mclk); }
                snd_soc_component_update_bits(component, WM8960_APOP1, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN);
                snd_soc_component_update_bits(component, WM8960_POWER1, WM8960_VREF | WM8960_VMID_MASK, 0);
            }
            snd_soc_bias_level::SND_SOC_BIAS_OFF => { regcache_sync((*wm8960).regmap); }
            _ => {}
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => match snd_soc_dapm_get_bias_level(dapm) {
            snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
                snd_soc_component_update_bits(component, WM8960_APOP2, WM8960_DISOP | WM8960_DRES_MASK, 0);
                snd_soc_component_update_bits(component, WM8960_APOP1, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN, WM8960_POBCTRL | WM8960_SOFT_ST | WM8960_BUFDCOPEN);
            }
            _ => {}
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {}
    }
    0
}

/* PLL divisors */
#[repr(C)]
#[derive(Copy, Clone)]
struct _pll_div {
    pre_div: u32,
    n: u32,
    k: u32,
}

fn is_pll_freq_available(mut source: c_uint, mut target: c_uint) -> bool {
    let mut Ndiv: c_uint;
    if source == 0 || target == 0 { return false; }
    target = target.wrapping_mul(4);
    Ndiv = target / source;
    if Ndiv < 6 {
        source >>= 1;
        Ndiv = target / source;
    }
    if Ndiv < 6 || Ndiv > 12 { return false; }
    true
}

/* The size in bits of the pll divide multiplied by 10 to allow rounding later */
const FIXED_PLL_SIZE: c_ulonglong = ((1u64 << 24) * 10) as c_ulonglong;

unsafe extern "C" fn pll_factors(mut source: c_uint, mut target: c_uint, pll_div: *mut _pll_div) -> c_int {
    let mut Kpart: c_ulonglong;
    let mut K: c_uint;
    let mut Ndiv: c_uint;
    let Nmod: c_uint;
    pr_debug(cstr!("WM8960 PLL: setting %dHz->%dHz\n"), source, target);
    target = target.wrapping_mul(4);
    Ndiv = target / source;
    if Ndiv < 6 {
        source >>= 1;
        (*pll_div).pre_div = 1;
        Ndiv = target / source;
    } else {
        (*pll_div).pre_div = 0;
    }
    if Ndiv < 6 || Ndiv > 12 {
        pr_err(cstr!("WM8960 PLL: Unsupported N=%d\n"), Ndiv);
        return -EINVAL;
    }
    (*pll_div).n = Ndiv;
    Nmod = target % source;
    Kpart = FIXED_PLL_SIZE.wrapping_mul(Nmod as c_ulonglong);
    Kpart /= source as c_ulonglong;
    K = (Kpart & 0xFFFFFFFF) as c_uint;
    if (K % 10) >= 5 { K += 5; }
    K /= 10;
    (*pll_div).k = K;
    pr_debug(cstr!("WM8960 PLL: N=%x K=%x pre_div=%d\n"), (*pll_div).n, (*pll_div).k, (*pll_div).pre_div);
    0
}

static mut pll_div: _pll_div = _pll_div { pre_div: 0, n: 0, k: 0 };

unsafe extern "C" fn wm8960_set_pll(component: *mut snd_soc_component, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let mut reg: u16;
    if freq_in != 0 && freq_out != 0 {
        let ret = pll_factors(freq_in, freq_out, &mut pll_div);
        if ret != 0 { return ret; }
    }
    snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x1, 0);
    snd_soc_component_update_bits(component, WM8960_POWER2, 0x1, 0);
    if freq_in == 0 || freq_out == 0 { return 0; }
    reg = (snd_soc_component_read(component, WM8960_PLL1) & !0x3f) as u16;
    reg |= (pll_div.pre_div << 4) as u16;
    reg |= pll_div.n as u16;
    if pll_div.k != 0 {
        reg |= 0x20;
        snd_soc_component_write(component, WM8960_PLL2, (pll_div.k >> 16) & 0xff);
        snd_soc_component_write(component, WM8960_PLL3, (pll_div.k >> 8) & 0xff);
        snd_soc_component_write(component, WM8960_PLL4, pll_div.k & 0xff);
    }
    snd_soc_component_write(component, WM8960_PLL1, reg as c_uint);
    snd_soc_component_update_bits(component, WM8960_POWER2, 0x1, 0x1);
    msleep(250);
    snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x1, 0x1);
    0
}

unsafe extern "C" fn wm8960_set_dai_pll(codec_dai: *mut snd_soc_dai, pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    (*wm8960).freq_in = freq_in as c_int;
    if pll_id == WM8960_SYSCLK_AUTO { return 0; }
    wm8960_set_pll(component, freq_in, freq_out)
}

unsafe extern "C" fn wm8960_set_dai_clkdiv(codec_dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16;
    match div_id {
        x if x == WM8960_SYSCLKDIV => { reg = (snd_soc_component_read(component, WM8960_CLOCK1) & 0x1f9) as u16; snd_soc_component_write(component, WM8960_CLOCK1, (reg as c_int | div) as c_uint); }
        x if x == WM8960_DACDIV => { reg = (snd_soc_component_read(component, WM8960_CLOCK1) & 0x1c7) as u16; snd_soc_component_write(component, WM8960_CLOCK1, (reg as c_int | div) as c_uint); }
        x if x == WM8960_OPCLKDIV => { reg = (snd_soc_component_read(component, WM8960_PLL1) & 0x03f) as u16; snd_soc_component_write(component, WM8960_PLL1, (reg as c_int | div) as c_uint); }
        x if x == WM8960_DCLKDIV => { reg = (snd_soc_component_read(component, WM8960_CLOCK2) & 0x03f) as u16; snd_soc_component_write(component, WM8960_CLOCK2, (reg as c_int | div) as c_uint); }
        x if x == WM8960_TOCLKSEL => { reg = (snd_soc_component_read(component, WM8960_ADDCTL1) & 0x1fd) as u16; snd_soc_component_write(component, WM8960_ADDCTL1, (reg as c_int | div) as c_uint); }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn wm8960_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    ((*wm8960).set_bias_level.unwrap())(component, level)
}

unsafe extern "C" fn wm8960_set_dai_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    match clk_id {
        x if x == WM8960_SYSCLK_MCLK => { snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x1, WM8960_SYSCLK_MCLK as c_uint); }
        x if x == WM8960_SYSCLK_PLL => { snd_soc_component_update_bits(component, WM8960_CLOCK1, 0x1, WM8960_SYSCLK_PLL as c_uint); }
        x if x == WM8960_SYSCLK_AUTO => {}
        _ => return -EINVAL,
    }
    (*wm8960).sysclk = freq as c_int;
    (*wm8960).clk_id = clk_id;
    0
}

unsafe fn WM8960_RATES() -> c_uint { SNDRV_PCM_RATE_8000_48000 }
unsafe fn WM8960_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static wm8960_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8960_hw_params),
    hw_free: Some(wm8960_hw_free),
    mute_stream: Some(wm8960_mute),
    set_fmt: Some(wm8960_set_dai_fmt),
    set_clkdiv: Some(wm8960_set_dai_clkdiv),
    set_pll: Some(wm8960_set_dai_pll),
    set_sysclk: Some(wm8960_set_dai_sysclk),
    no_capture_mute: 1,
};

static mut wm8960_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("wm8960-hifi"),
    playback: snd_soc_pcm_stream { stream_name: cstr!("Playback"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
    capture: snd_soc_pcm_stream { stream_name: cstr!("Capture"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
    ops: &wm8960_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8960_probe(component: *mut snd_soc_component) -> c_int {
    let wm8960 = snd_soc_component_get_drvdata(component) as *mut wm8960_priv;
    let pdata = &mut (*wm8960).pdata as *mut wm8960_data;
    if (*pdata).capless {
        (*wm8960).set_bias_level = Some(wm8960_set_bias_level_capless);
    } else {
        (*wm8960).set_bias_level = Some(wm8960_set_bias_level_out3);
    }
    extern "C" { fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_int) -> c_int; }
    snd_soc_add_component_controls(component, wm8960_snd_controls.as_ptr(), 0);
    wm8960_add_widgets(component);
    0
}

static soc_component_dev_wm8960: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8960_probe),
    set_bias_level: Some(wm8960_set_bias_level),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8960_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: 0,
    reg_defaults: wm8960_reg_defaults.as_ptr(),
    num_reg_defaults: wm8960_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(wm8960_volatile),
};

unsafe extern "C" fn wm8960_set_pdata_from_of(i2c: *mut i2c_client, pdata: *mut wm8960_data) {
    /* const struct device_node *np = i2c->dev.of_node; */
    let np: *const device_node = ptr::null();
    if of_property_read_bool(np, cstr!("wlf,capless")) { (*pdata).capless = true; }
    if of_property_read_bool(np, cstr!("wlf,shared-lrclk")) { (*pdata).shared_lrclk = true; }
    of_property_read_u32_array(np, cstr!("wlf,gpio-cfg"), (*pdata).gpio_cfg.as_mut_ptr(), (*pdata).gpio_cfg.len());
    of_property_read_u32_array(np, cstr!("wlf,hp-cfg"), (*pdata).hp_cfg.as_mut_ptr(), (*pdata).hp_cfg.len());
}

unsafe extern "C" fn wm8960_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pdata = dev_get_platdata(&mut (*i2c).dev);
    let wm8960: *mut wm8960_priv;
    let mut ret: c_int;
    let mut val: u8 = 0;
    wm8960 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8960_priv>(), GFP_KERNEL) as *mut wm8960_priv;
    if wm8960.is_null() { return -ENOMEM; }
    (*wm8960).mclk = devm_clk_get(&mut (*i2c).dev, cstr!("mclk"));
    if IS_ERR((*wm8960).mclk as *const c_void) {
        if PTR_ERR((*wm8960).mclk as *const c_void) == -EPROBE_DEFER { return -EPROBE_DEFER; }
    } else {
        ret = clk_get_rate((*wm8960).mclk);
        if ret >= 0 { (*wm8960).freq_in = ret; } else { dev_err(&mut (*i2c).dev, cstr!("Failed to read MCLK rate: %d\n"), ret); }
    }
    let mut i = 0usize;
    while i < (*wm8960).supplies.len() {
        (*wm8960).supplies[i].supply = wm8960_supply_names[i];
        i += 1;
    }
    ret = devm_regulator_bulk_get(&mut (*i2c).dev, (*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
    if ret < 0 { dev_err(&mut (*i2c).dev, cstr!("Failed to request supplies: %d\n"), ret); return ret; }
    ret = regulator_bulk_enable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
    if ret < 0 { dev_err(&mut (*i2c).dev, cstr!("Failed to enable supplies: %d\n"), ret); return ret; }
    (*wm8960).regmap = devm_regmap_init_i2c(i2c, &wm8960_regmap);
    if IS_ERR((*wm8960).regmap as *const c_void) {
        ret = PTR_ERR((*wm8960).regmap as *const c_void);
        regulator_bulk_disable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
        return ret;
    }
    if !pdata.is_null() {
        memcpy(&mut (*wm8960).pdata as *mut _ as *mut c_void, pdata as *const c_void, core::mem::size_of::<wm8960_data>());
    } else {
        /* else if (i2c->dev.of_node) */
        wm8960_set_pdata_from_of(i2c, &mut (*wm8960).pdata);
    }
    ret = i2c_master_recv(i2c, &mut val, core::mem::size_of_val(&val) as c_int);
    if ret >= 0 {
        dev_err(&mut (*i2c).dev, cstr!("Not wm8960, wm8960 reg can not read by i2c\n"));
        regulator_bulk_disable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
        return -EINVAL;
    }
    ret = wm8960_reset((*wm8960).regmap);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, cstr!("Failed to issue reset\n"));
        regulator_bulk_disable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
        return ret;
    }
    if (*wm8960).pdata.shared_lrclk {
        ret = regmap_update_bits((*wm8960).regmap, WM8960_ADDCTL2, 0x4, 0x4);
        if ret != 0 {
            dev_err(&mut (*i2c).dev, cstr!("Failed to enable LRCM: %d\n"), ret);
            regulator_bulk_disable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
            return ret;
        }
    }
    regmap_update_bits((*wm8960).regmap, WM8960_LINVOL, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_RINVOL, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_LADC, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_RADC, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_LDAC, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_RDAC, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_LOUT1, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_ROUT1, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_LOUT2, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_ROUT2, 0x100, 0x100);
    regmap_update_bits((*wm8960).regmap, WM8960_IFACE2, 1 << 6, (*wm8960).pdata.gpio_cfg[0] << 6);
    regmap_update_bits((*wm8960).regmap, WM8960_ADDCTL4, 0xF << 4, (*wm8960).pdata.gpio_cfg[1] << 4);
    regmap_update_bits((*wm8960).regmap, WM8960_ADDCTL4, 3 << 2, (*wm8960).pdata.hp_cfg[0] << 2);
    regmap_update_bits((*wm8960).regmap, WM8960_ADDCTL2, 3 << 5, (*wm8960).pdata.hp_cfg[1] << 5);
    regmap_update_bits((*wm8960).regmap, WM8960_ADDCTL1, 3, (*wm8960).pdata.hp_cfg[2]);
    i2c_set_clientdata(i2c, wm8960 as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8960, &mut wm8960_dai, 1);
    if ret != 0 {
        regulator_bulk_disable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
        return ret;
    }
    0
}

unsafe extern "C" fn wm8960_i2c_remove(client: *mut i2c_client) {
    let wm8960 = i2c_get_clientdata(client) as *mut wm8960_priv;
    regulator_bulk_disable((*wm8960).supplies.len() as c_int, (*wm8960).supplies.as_mut_ptr());
}

static wm8960_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [119, 109, 56, 57, 54, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, wm8960_i2c_id); */

/* CONFIG_OF */
static wm8960_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("wlf,wm8960") },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, wm8960_of_match); */

/* CONFIG_ACPI */
static wm8960_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id { id: [49, 65, 69, 67, 56, 57, 54, 48, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 }, /* Wolfson PCI ID + part ID */
    acpi_device_id { id: [49, 48, 49, 51, 56, 57, 54, 48, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 }, /* Cirrus Logic PCI ID + part ID */
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, wm8960_acpi_match); */

static mut wm8960_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("wm8960"),
        of_match_table: wm8960_of_match.as_ptr(),
        acpi_match_table: wm8960_acpi_match.as_ptr(),
    },
    probe: Some(wm8960_i2c_probe),
    remove: Some(wm8960_i2c_remove),
    id_table: wm8960_i2c_id.as_ptr(),
};

/* module_i2c_driver(wm8960_i2c_driver);
 * MODULE_DESCRIPTION("ASoC WM8960 driver");
 * MODULE_AUTHOR("Liam Girdwood");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
