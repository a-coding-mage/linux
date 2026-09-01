// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm2200.rs  --  WM2200 ALSA SoC Audio driver
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * Source-level Rust translation of wm2200.c.  Linux, ASoC, regmap, GPIO,
 * regulator, PM, and WM2200 register symbols are supplied by external
 * dependencies corresponding to the original C includes.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub irq: c_int }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char }
#[repr(C)] pub struct wm2200_pdata {
    pub gpio_defaults: [c_uint; 4],
    pub micbias: [wm2200_micbias_pdata; 2],
    pub in_mode: [c_uint; 3],
    pub dmic_sup: [c_uint; 3],
}
#[repr(C)] pub struct wm2200_micbias_pdata {
    pub mb_lvl: c_uint,
    pub bypass: bool,
    pub discharge: bool,
    pub fast_start: bool,
}
#[repr(C)] pub struct cs_dsp {
    pub type_: c_int,
    pub num: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub sysclk_reg: c_uint,
    pub sysclk_mask: c_uint,
    pub sysclk_shift: c_uint,
    pub base: c_uint,
    pub mem: *const cs_dsp_region,
    pub num_mems: c_int,
}
#[repr(C)] pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub part: *const c_char,
}
#[repr(C)] pub struct regmap_range_cfg {
    pub name: *const c_char,
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}
#[repr(C)] pub struct cs_dsp_region { pub type_: c_uint, pub base: c_uint }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)] pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)] pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}
#[repr(C)] pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)] pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}
#[repr(C)] pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_int,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
}
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 20] }
#[repr(C)] pub struct driver_private { pub name: *const c_char, pub pm: *const dev_pm_ops }
#[repr(C)] pub struct i2c_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn gcd(a: c_uint, b: c_uint) -> c_uint;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn try_wait_for_completion(x: *mut completion) -> bool;
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn msleep(msecs: c_uint);
    fn complete(x: *mut completion);
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn wm_adsp1_init(dsp: *mut wm_adsp);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut c_void;
    fn init_completion(x: *mut completion);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, name: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn regmap_register_patch(map: *mut regmap, patch: *const reg_sequence, num: c_int) -> c_int;
    fn request_threaded_irq(irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_request_idle(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TRIGGER_HIGH: c_uint = 0x00000004;
const IRQF_ONESHOT: c_uint = 0x00002000;
const GPIOD_OUT_HIGH: c_int = 1;
const GPIOD_OUT_LOW: c_int = 0;
const REGCACHE_MAPLE: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;

unsafe extern "C" {
    static WM2200_MAX_REGISTER: c_uint;
    static WM2200_SOFTWARE_RESET: c_uint;
    static WM2200_DEVICE_REVISION: c_uint;
    static WM2200_ADPS1_IRQ0: c_uint;
    static WM2200_ADPS1_IRQ1: c_uint;
    static WM2200_INTERRUPT_STATUS_1: c_uint;
    static WM2200_INTERRUPT_STATUS_2: c_uint;
    static WM2200_INTERRUPT_RAW_STATUS_2: c_uint;
}

const WM2200_DSP_CONTROL_1: c_uint = 0x00;
const WM2200_DSP_CONTROL_2: c_uint = 0x02;
const WM2200_DSP_CONTROL_3: c_uint = 0x03;
const WM2200_DSP_CONTROL_4: c_uint = 0x04;
const WM2200_DSP_CONTROL_5: c_uint = 0x06;
const WM2200_DSP_CONTROL_6: c_uint = 0x07;
const WM2200_DSP_CONTROL_7: c_uint = 0x08;
const WM2200_DSP_CONTROL_8: c_uint = 0x09;
const WM2200_DSP_CONTROL_9: c_uint = 0x0A;
const WM2200_DSP_CONTROL_10: c_uint = 0x0B;
const WM2200_DSP_CONTROL_11: c_uint = 0x0C;
const WM2200_DSP_CONTROL_12: c_uint = 0x0D;
const WM2200_DSP_CONTROL_13: c_uint = 0x0F;
const WM2200_DSP_CONTROL_14: c_uint = 0x10;
const WM2200_DSP_CONTROL_15: c_uint = 0x11;
const WM2200_DSP_CONTROL_16: c_uint = 0x12;
const WM2200_DSP_CONTROL_17: c_uint = 0x13;
const WM2200_DSP_CONTROL_18: c_uint = 0x14;
const WM2200_DSP_CONTROL_19: c_uint = 0x16;
const WM2200_DSP_CONTROL_20: c_uint = 0x17;
const WM2200_DSP_CONTROL_21: c_uint = 0x18;
const WM2200_DSP_CONTROL_22: c_uint = 0x1A;
const WM2200_DSP_CONTROL_23: c_uint = 0x1B;
const WM2200_DSP_CONTROL_24: c_uint = 0x1C;
const WM2200_DSP_CONTROL_25: c_uint = 0x1E;
const WM2200_DSP_CONTROL_26: c_uint = 0x20;
const WM2200_DSP_CONTROL_27: c_uint = 0x21;
const WM2200_DSP_CONTROL_28: c_uint = 0x22;
const WM2200_DSP_CONTROL_29: c_uint = 0x23;
const WM2200_DSP_CONTROL_30: c_uint = 0x24;
const WM2200_DSP_CONTROL_31: c_uint = 0x26;

/* The code assumes DCVDD is generated internally */
const WM2200_NUM_CORE_SUPPLIES: usize = 2;
static wm2200_core_supply_names: [*const c_char; WM2200_NUM_CORE_SUPPLIES] =
    [b"DBVDD\0".as_ptr() as *const c_char, b"LDOVDD\0".as_ptr() as *const c_char];

/* codec private data */
#[repr(C)]
pub struct wm2200_priv {
    pub dsp: [wm_adsp; 2],
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub pdata: wm2200_pdata,
    pub core_supplies: [regulator_bulk_data; WM2200_NUM_CORE_SUPPLIES],
    pub ldo_ena: *mut gpio_desc,
    pub reset: *mut gpio_desc,
    pub fll_lock: completion,
    pub fll_fout: c_int,
    pub fll_fref: c_int,
    pub fll_src: c_int,
    pub rev: c_int,
    pub sysclk: c_int,
    pub symmetric_rates: c_uint,
}

const WM2200_DSP_RANGE_BASE: c_uint = 0; /* WM2200_MAX_REGISTER + 1, external in C */
const WM2200_DSP_SPACING: c_uint = 12288;
const WM2200_DSP1_DM_BASE: c_uint = WM2200_DSP_RANGE_BASE + (0 * WM2200_DSP_SPACING);
const WM2200_DSP1_PM_BASE: c_uint = WM2200_DSP_RANGE_BASE + (1 * WM2200_DSP_SPACING);
const WM2200_DSP1_ZM_BASE: c_uint = WM2200_DSP_RANGE_BASE + (2 * WM2200_DSP_SPACING);
const WM2200_DSP2_DM_BASE: c_uint = WM2200_DSP_RANGE_BASE + (3 * WM2200_DSP_SPACING);
const WM2200_DSP2_PM_BASE: c_uint = WM2200_DSP_RANGE_BASE + (4 * WM2200_DSP_SPACING);
const WM2200_DSP2_ZM_BASE: c_uint = WM2200_DSP_RANGE_BASE + (5 * WM2200_DSP_SPACING);

/* Register range, DSP memory region, control, DAPM widget and route tables are
 * translated from the C static initializers.  Their element fields reference
 * WM2200 and ASoC constants/macros supplied by the original headers.
 */

static wm2200_dsp1_regions: [cs_dsp_region; 3] = [
    cs_dsp_region { type_: WMFW_ADSP1_PM, base: WM2200_DSP1_PM_BASE },
    cs_dsp_region { type_: WMFW_ADSP1_DM, base: WM2200_DSP1_DM_BASE },
    cs_dsp_region { type_: WMFW_ADSP1_ZM, base: WM2200_DSP1_ZM_BASE },
];

static wm2200_dsp2_regions: [cs_dsp_region; 3] = [
    cs_dsp_region { type_: WMFW_ADSP1_PM, base: WM2200_DSP2_PM_BASE },
    cs_dsp_region { type_: WMFW_ADSP1_DM, base: WM2200_DSP2_DM_BASE },
    cs_dsp_region { type_: WMFW_ADSP1_ZM, base: WM2200_DSP2_ZM_BASE },
];

const WMFW_ADSP1: c_int = 1;
const WMFW_ADSP1_PM: c_uint = 0;
const WMFW_ADSP1_DM: c_uint = 1;
const WMFW_ADSP1_ZM: c_uint = 2;

static wm2200_reg_defaults: [reg_default; 224] = [
    reg_default { reg: 0x000B, def: 0x0000 }, reg_default { reg: 0x0102, def: 0x0000 },
    reg_default { reg: 0x0103, def: 0x0011 }, reg_default { reg: 0x0111, def: 0x0000 },
    reg_default { reg: 0x0112, def: 0x0000 }, reg_default { reg: 0x0113, def: 0x0000 },
    reg_default { reg: 0x0114, def: 0x0000 }, reg_default { reg: 0x0116, def: 0x0177 },
    reg_default { reg: 0x0117, def: 0x0004 }, reg_default { reg: 0x0119, def: 0x0000 },
    reg_default { reg: 0x011A, def: 0x0002 }, reg_default { reg: 0x0200, def: 0x0000 },
    reg_default { reg: 0x0201, def: 0x03FF }, reg_default { reg: 0x0202, def: 0x9BDE },
    reg_default { reg: 0x020C, def: 0x0000 }, reg_default { reg: 0x020D, def: 0x0000 },
    reg_default { reg: 0x020F, def: 0x0000 }, reg_default { reg: 0x0210, def: 0x0000 },
    reg_default { reg: 0x0301, def: 0x0000 }, reg_default { reg: 0x0302, def: 0x2240 },
    reg_default { reg: 0x0303, def: 0x0040 }, reg_default { reg: 0x0304, def: 0x2240 },
    reg_default { reg: 0x0305, def: 0x0040 }, reg_default { reg: 0x0306, def: 0x2240 },
    reg_default { reg: 0x0307, def: 0x0040 }, reg_default { reg: 0x030A, def: 0x0000 },
    reg_default { reg: 0x030B, def: 0x0022 }, reg_default { reg: 0x030C, def: 0x0180 },
    reg_default { reg: 0x030D, def: 0x0180 }, reg_default { reg: 0x030E, def: 0x0180 },
    reg_default { reg: 0x030F, def: 0x0180 }, reg_default { reg: 0x0310, def: 0x0180 },
    reg_default { reg: 0x0311, def: 0x0180 }, reg_default { reg: 0x0400, def: 0x0000 },
    reg_default { reg: 0x0401, def: 0x0000 }, reg_default { reg: 0x0402, def: 0x0000 },
    reg_default { reg: 0x0403, def: 0x0000 }, reg_default { reg: 0x0404, def: 0x0000 },
    reg_default { reg: 0x0409, def: 0x0000 }, reg_default { reg: 0x040A, def: 0x0022 },
    reg_default { reg: 0x040B, def: 0x0180 }, reg_default { reg: 0x040C, def: 0x0180 },
    reg_default { reg: 0x040D, def: 0x0180 }, reg_default { reg: 0x040E, def: 0x0180 },
    reg_default { reg: 0x0417, def: 0x0069 }, reg_default { reg: 0x0418, def: 0x0000 },
    reg_default { reg: 0x0500, def: 0x0000 }, reg_default { reg: 0x0501, def: 0x0008 },
    reg_default { reg: 0x0502, def: 0x0000 }, reg_default { reg: 0x0503, def: 0x0000 },
    reg_default { reg: 0x0504, def: 0x0000 }, reg_default { reg: 0x0505, def: 0x0001 },
    reg_default { reg: 0x0506, def: 0x0001 }, reg_default { reg: 0x0507, def: 0x0000 },
    reg_default { reg: 0x0508, def: 0x0000 }, reg_default { reg: 0x0509, def: 0x0000 },
    reg_default { reg: 0x050A, def: 0x0000 }, reg_default { reg: 0x050B, def: 0x0000 },
    reg_default { reg: 0x050C, def: 0x0000 }, reg_default { reg: 0x050D, def: 0x0000 },
    reg_default { reg: 0x050E, def: 0x0000 }, reg_default { reg: 0x050F, def: 0x0000 },
    reg_default { reg: 0x0510, def: 0x0000 }, reg_default { reg: 0x0511, def: 0x0000 },
    reg_default { reg: 0x0512, def: 0x0000 }, reg_default { reg: 0x0513, def: 0x0000 },
    reg_default { reg: 0x0514, def: 0x0000 }, reg_default { reg: 0x0515, def: 0x0001 },
    reg_default { reg: 0x0600, def: 0x0000 }, reg_default { reg: 0x0601, def: 0x0080 },
    reg_default { reg: 0x0602, def: 0x0000 }, reg_default { reg: 0x0603, def: 0x0080 },
    reg_default { reg: 0x0604, def: 0x0000 }, reg_default { reg: 0x0605, def: 0x0080 },
    reg_default { reg: 0x0606, def: 0x0000 }, reg_default { reg: 0x0607, def: 0x0080 },
    reg_default { reg: 0x0608, def: 0x0000 }, reg_default { reg: 0x0609, def: 0x0080 },
    reg_default { reg: 0x060A, def: 0x0000 }, reg_default { reg: 0x060B, def: 0x0080 },
    reg_default { reg: 0x060C, def: 0x0000 }, reg_default { reg: 0x060D, def: 0x0080 },
    reg_default { reg: 0x060E, def: 0x0000 }, reg_default { reg: 0x060F, def: 0x0080 },
    reg_default { reg: 0x0610, def: 0x0000 }, reg_default { reg: 0x0611, def: 0x0080 },
    reg_default { reg: 0x0612, def: 0x0000 }, reg_default { reg: 0x0613, def: 0x0080 },
    reg_default { reg: 0x0614, def: 0x0000 }, reg_default { reg: 0x0615, def: 0x0080 },
    reg_default { reg: 0x0616, def: 0x0000 }, reg_default { reg: 0x0617, def: 0x0080 },
    reg_default { reg: 0x0618, def: 0x0000 }, reg_default { reg: 0x0619, def: 0x0080 },
    reg_default { reg: 0x061A, def: 0x0000 }, reg_default { reg: 0x061B, def: 0x0080 },
    reg_default { reg: 0x061C, def: 0x0000 }, reg_default { reg: 0x061D, def: 0x0080 },
    reg_default { reg: 0x061E, def: 0x0000 }, reg_default { reg: 0x061F, def: 0x0080 },
    reg_default { reg: 0x0620, def: 0x0000 }, reg_default { reg: 0x0621, def: 0x0080 },
    reg_default { reg: 0x0622, def: 0x0000 }, reg_default { reg: 0x0623, def: 0x0080 },
    reg_default { reg: 0x0624, def: 0x0000 }, reg_default { reg: 0x0625, def: 0x0080 },
    reg_default { reg: 0x0626, def: 0x0000 }, reg_default { reg: 0x0627, def: 0x0080 },
    reg_default { reg: 0x0628, def: 0x0000 }, reg_default { reg: 0x0629, def: 0x0080 },
    reg_default { reg: 0x062A, def: 0x0000 }, reg_default { reg: 0x062B, def: 0x0080 },
    reg_default { reg: 0x062C, def: 0x0000 }, reg_default { reg: 0x062D, def: 0x0080 },
    reg_default { reg: 0x062E, def: 0x0000 }, reg_default { reg: 0x062F, def: 0x0080 },
    reg_default { reg: 0x0630, def: 0x0000 }, reg_default { reg: 0x0631, def: 0x0080 },
    reg_default { reg: 0x0632, def: 0x0000 }, reg_default { reg: 0x0633, def: 0x0080 },
    reg_default { reg: 0x0634, def: 0x0000 }, reg_default { reg: 0x0635, def: 0x0080 },
    reg_default { reg: 0x0636, def: 0x0000 }, reg_default { reg: 0x0637, def: 0x0080 },
    reg_default { reg: 0x0638, def: 0x0000 }, reg_default { reg: 0x0639, def: 0x0080 },
    reg_default { reg: 0x063A, def: 0x0000 }, reg_default { reg: 0x063B, def: 0x0080 },
    reg_default { reg: 0x063C, def: 0x0000 }, reg_default { reg: 0x063D, def: 0x0080 },
    reg_default { reg: 0x063E, def: 0x0000 }, reg_default { reg: 0x063F, def: 0x0080 },
    reg_default { reg: 0x0640, def: 0x0000 }, reg_default { reg: 0x0641, def: 0x0080 },
    reg_default { reg: 0x0642, def: 0x0000 }, reg_default { reg: 0x0643, def: 0x0080 },
    reg_default { reg: 0x0644, def: 0x0000 }, reg_default { reg: 0x0645, def: 0x0080 },
    reg_default { reg: 0x0646, def: 0x0000 }, reg_default { reg: 0x0647, def: 0x0080 },
    reg_default { reg: 0x0648, def: 0x0000 }, reg_default { reg: 0x0649, def: 0x0080 },
    reg_default { reg: 0x064A, def: 0x0000 }, reg_default { reg: 0x064B, def: 0x0080 },
    reg_default { reg: 0x064C, def: 0x0000 }, reg_default { reg: 0x064D, def: 0x0080 },
    reg_default { reg: 0x064E, def: 0x0000 }, reg_default { reg: 0x064F, def: 0x0080 },
    reg_default { reg: 0x0650, def: 0x0000 }, reg_default { reg: 0x0651, def: 0x0080 },
    reg_default { reg: 0x0652, def: 0x0000 }, reg_default { reg: 0x0653, def: 0x0080 },
    reg_default { reg: 0x0654, def: 0x0000 }, reg_default { reg: 0x0655, def: 0x0080 },
    reg_default { reg: 0x0656, def: 0x0000 }, reg_default { reg: 0x0657, def: 0x0080 },
    reg_default { reg: 0x0658, def: 0x0000 }, reg_default { reg: 0x0659, def: 0x0080 },
    reg_default { reg: 0x065A, def: 0x0000 }, reg_default { reg: 0x065B, def: 0x0080 },
    reg_default { reg: 0x065C, def: 0x0000 }, reg_default { reg: 0x065D, def: 0x0080 },
    reg_default { reg: 0x065E, def: 0x0000 }, reg_default { reg: 0x065F, def: 0x0080 },
    reg_default { reg: 0x0660, def: 0x0000 }, reg_default { reg: 0x0661, def: 0x0080 },
    reg_default { reg: 0x0662, def: 0x0000 }, reg_default { reg: 0x0663, def: 0x0080 },
    reg_default { reg: 0x0664, def: 0x0000 }, reg_default { reg: 0x0665, def: 0x0080 },
    reg_default { reg: 0x0666, def: 0x0000 }, reg_default { reg: 0x0667, def: 0x0080 },
    reg_default { reg: 0x0668, def: 0x0000 }, reg_default { reg: 0x0669, def: 0x0080 },
    reg_default { reg: 0x066A, def: 0x0000 }, reg_default { reg: 0x066B, def: 0x0080 },
    reg_default { reg: 0x066C, def: 0x0000 }, reg_default { reg: 0x066D, def: 0x0080 },
    reg_default { reg: 0x066E, def: 0x0000 }, reg_default { reg: 0x066F, def: 0x0080 },
    reg_default { reg: 0x0670, def: 0x0000 }, reg_default { reg: 0x0671, def: 0x0080 },
    reg_default { reg: 0x0672, def: 0x0000 }, reg_default { reg: 0x0673, def: 0x0080 },
    reg_default { reg: 0x0674, def: 0x0000 }, reg_default { reg: 0x0675, def: 0x0080 },
    reg_default { reg: 0x0676, def: 0x0000 }, reg_default { reg: 0x0677, def: 0x0080 },
    reg_default { reg: 0x0678, def: 0x0000 }, reg_default { reg: 0x0679, def: 0x0080 },
    reg_default { reg: 0x067A, def: 0x0000 }, reg_default { reg: 0x067B, def: 0x0080 },
    reg_default { reg: 0x067C, def: 0x0000 }, reg_default { reg: 0x067D, def: 0x0080 },
    reg_default { reg: 0x067E, def: 0x0000 }, reg_default { reg: 0x067F, def: 0x0080 },
    reg_default { reg: 0x0680, def: 0x0000 }, reg_default { reg: 0x0681, def: 0x0000 },
    reg_default { reg: 0x0682, def: 0x0000 }, reg_default { reg: 0x0683, def: 0x0000 },
    reg_default { reg: 0x0684, def: 0x0000 }, reg_default { reg: 0x0685, def: 0x0000 },
    reg_default { reg: 0x0686, def: 0x0000 }, reg_default { reg: 0x0687, def: 0x0080 },
    reg_default { reg: 0x0688, def: 0x0000 }, reg_default { reg: 0x0689, def: 0x0080 },
    reg_default { reg: 0x068A, def: 0x0000 }, reg_default { reg: 0x068B, def: 0x0080 },
    reg_default { reg: 0x068C, def: 0x0000 }, reg_default { reg: 0x068D, def: 0x0080 },
    reg_default { reg: 0x068E, def: 0x0000 }, reg_default { reg: 0x068F, def: 0x0080 },
    reg_default { reg: 0x0690, def: 0x0000 }, reg_default { reg: 0x0691, def: 0x0080 },
    reg_default { reg: 0x0692, def: 0x0000 }, reg_default { reg: 0x0693, def: 0x0080 },
    reg_default { reg: 0x0694, def: 0x0000 }, reg_default { reg: 0x0695, def: 0x0080 },
    reg_default { reg: 0x0696, def: 0x0000 }, reg_default { reg: 0x0697, def: 0x0000 },
    reg_default { reg: 0x0698, def: 0x0000 }, reg_default { reg: 0x0699, def: 0x0000 },
    reg_default { reg: 0x069A, def: 0x0000 }, reg_default { reg: 0x069B, def: 0x0000 },
    reg_default { reg: 0x0700, def: 0xA101 }, reg_default { reg: 0x0701, def: 0xA101 },
    reg_default { reg: 0x0702, def: 0xA101 }, reg_default { reg: 0x0703, def: 0xA101 },
    reg_default { reg: 0x0709, def: 0x0000 }, reg_default { reg: 0x0801, def: 0x00FF },
    reg_default { reg: 0x0804, def: 0xFFFF }, reg_default { reg: 0x0808, def: 0x0000 },
    reg_default { reg: 0x0900, def: 0x0000 }, reg_default { reg: 0x0901, def: 0x0000 },
    reg_default { reg: 0x0902, def: 0x0000 }, reg_default { reg: 0x0903, def: 0x0000 },
    reg_default { reg: 0x0904, def: 0x0000 }, reg_default { reg: 0x0905, def: 0x0000 },
    reg_default { reg: 0x0906, def: 0x0000 }, reg_default { reg: 0x0907, def: 0x0000 },
    reg_default { reg: 0x0908, def: 0x0000 }, reg_default { reg: 0x0909, def: 0x0000 },
    reg_default { reg: 0x090A, def: 0x0000 }, reg_default { reg: 0x090B, def: 0x0000 },
    reg_default { reg: 0x090C, def: 0x0000 }, reg_default { reg: 0x090D, def: 0x0000 },
    reg_default { reg: 0x090E, def: 0x0000 }, reg_default { reg: 0x090F, def: 0x0000 },
    reg_default { reg: 0x0910, def: 0x0000 }, reg_default { reg: 0x0911, def: 0x0000 },
    reg_default { reg: 0x0912, def: 0x0000 }, reg_default { reg: 0x0913, def: 0x0000 },
    reg_default { reg: 0x0916, def: 0x0000 }, reg_default { reg: 0x0917, def: 0x0000 },
    reg_default { reg: 0x0918, def: 0x0000 }, reg_default { reg: 0x0919, def: 0x0000 },
    reg_default { reg: 0x091A, def: 0x0000 }, reg_default { reg: 0x091B, def: 0x0000 },
    reg_default { reg: 0x091C, def: 0x0000 }, reg_default { reg: 0x091D, def: 0x0000 },
    reg_default { reg: 0x091E, def: 0x0000 }, reg_default { reg: 0x091F, def: 0x0000 },
    reg_default { reg: 0x0920, def: 0x0000 }, reg_default { reg: 0x0921, def: 0x0000 },
    reg_default { reg: 0x0922, def: 0x0000 }, reg_default { reg: 0x0923, def: 0x0000 },
    reg_default { reg: 0x0924, def: 0x0000 }, reg_default { reg: 0x0925, def: 0x0000 },
    reg_default { reg: 0x0926, def: 0x0000 }, reg_default { reg: 0x0927, def: 0x0000 },
    reg_default { reg: 0x0928, def: 0x0000 }, reg_default { reg: 0x0929, def: 0x0000 },
    reg_default { reg: 0x093E, def: 0x0000 }, reg_default { reg: 0x093F, def: 0x0000 },
    reg_default { reg: 0x0942, def: 0x0000 }, reg_default { reg: 0x0943, def: 0x0000 },
];

static wm2200_reva_patch: [reg_sequence; 68] = [
    reg_sequence { reg: 0x07, def: 0x0003 }, reg_sequence { reg: 0x102, def: 0x0200 },
    reg_sequence { reg: 0x203, def: 0x0084 }, reg_sequence { reg: 0x201, def: 0x83FF },
    reg_sequence { reg: 0x20C, def: 0x0062 }, reg_sequence { reg: 0x20D, def: 0x0062 },
    reg_sequence { reg: 0x207, def: 0x2002 }, reg_sequence { reg: 0x208, def: 0x20C0 },
    reg_sequence { reg: 0x21D, def: 0x01C0 }, reg_sequence { reg: 0x50A, def: 0x0001 },
    reg_sequence { reg: 0x50B, def: 0x0002 }, reg_sequence { reg: 0x50C, def: 0x0003 },
    reg_sequence { reg: 0x50D, def: 0x0004 }, reg_sequence { reg: 0x50E, def: 0x0005 },
    reg_sequence { reg: 0x510, def: 0x0001 }, reg_sequence { reg: 0x511, def: 0x0002 },
    reg_sequence { reg: 0x512, def: 0x0003 }, reg_sequence { reg: 0x513, def: 0x0004 },
    reg_sequence { reg: 0x514, def: 0x0005 }, reg_sequence { reg: 0x515, def: 0x0000 },
    reg_sequence { reg: 0x201, def: 0x8084 }, reg_sequence { reg: 0x202, def: 0xBBDE },
    reg_sequence { reg: 0x203, def: 0x00EC }, reg_sequence { reg: 0x500, def: 0x8000 },
    reg_sequence { reg: 0x507, def: 0x1820 }, reg_sequence { reg: 0x508, def: 0x1820 },
    reg_sequence { reg: 0x505, def: 0x0300 }, reg_sequence { reg: 0x506, def: 0x0300 },
    reg_sequence { reg: 0x302, def: 0x2280 }, reg_sequence { reg: 0x303, def: 0x0080 },
    reg_sequence { reg: 0x304, def: 0x2280 }, reg_sequence { reg: 0x305, def: 0x0080 },
    reg_sequence { reg: 0x306, def: 0x2280 }, reg_sequence { reg: 0x307, def: 0x0080 },
    reg_sequence { reg: 0x401, def: 0x0080 }, reg_sequence { reg: 0x402, def: 0x0080 },
    reg_sequence { reg: 0x417, def: 0x3069 }, reg_sequence { reg: 0x900, def: 0x6318 },
    reg_sequence { reg: 0x901, def: 0x6300 }, reg_sequence { reg: 0x902, def: 0x0FC8 },
    reg_sequence { reg: 0x903, def: 0x03FE }, reg_sequence { reg: 0x904, def: 0x00E0 },
    reg_sequence { reg: 0x905, def: 0x1EC4 }, reg_sequence { reg: 0x906, def: 0xF136 },
    reg_sequence { reg: 0x907, def: 0x0409 }, reg_sequence { reg: 0x908, def: 0x04CC },
    reg_sequence { reg: 0x909, def: 0x1C9B }, reg_sequence { reg: 0x90A, def: 0xF337 },
    reg_sequence { reg: 0x90B, def: 0x040B }, reg_sequence { reg: 0x90C, def: 0x0CBB },
    reg_sequence { reg: 0x90D, def: 0x16F8 }, reg_sequence { reg: 0x90E, def: 0xF7D9 },
    reg_sequence { reg: 0x90F, def: 0x040A }, reg_sequence { reg: 0x910, def: 0x1F14 },
    reg_sequence { reg: 0x911, def: 0x058C }, reg_sequence { reg: 0x912, def: 0x0563 },
    reg_sequence { reg: 0x913, def: 0x4000 }, reg_sequence { reg: 0x916, def: 0x6318 },
    reg_sequence { reg: 0x917, def: 0x6300 }, reg_sequence { reg: 0x918, def: 0x0FC8 },
    reg_sequence { reg: 0x919, def: 0x03FE }, reg_sequence { reg: 0x91A, def: 0x00E0 },
    reg_sequence { reg: 0x91B, def: 0x1EC4 }, reg_sequence { reg: 0x91C, def: 0xF136 },
    reg_sequence { reg: 0x91D, def: 0x0409 }, reg_sequence { reg: 0x91E, def: 0x04CC },
    reg_sequence { reg: 0x91F, def: 0x1C9B }, reg_sequence { reg: 0x920, def: 0xF337 },
];

unsafe extern "C" fn wm2200_reset(wm2200: *mut wm2200_priv) -> c_int {
    if !(*wm2200).reset.is_null() {
        /* Descriptor flagged active low, so this will be inverted */
        gpiod_set_value_cansleep((*wm2200).reset, 1);
        gpiod_set_value_cansleep((*wm2200).reset, 0);
        0
    } else {
        regmap_write((*wm2200).regmap, WM2200_SOFTWARE_RESET, 0x2200)
    }
}

static wm2200_mixer_texts: [&[u8]; 31] = [
    b"None\0", b"Tone Generator\0", b"AEC Loopback\0", b"IN1L\0", b"IN1R\0",
    b"IN2L\0", b"IN2R\0", b"IN3L\0", b"IN3R\0", b"AIF1RX1\0", b"AIF1RX2\0",
    b"AIF1RX3\0", b"AIF1RX4\0", b"AIF1RX5\0", b"AIF1RX6\0", b"EQL\0",
    b"EQR\0", b"LHPF1\0", b"LHPF2\0", b"DSP1.1\0", b"DSP1.2\0", b"DSP1.3\0",
    b"DSP1.4\0", b"DSP1.5\0", b"DSP1.6\0", b"DSP2.1\0", b"DSP2.2\0",
    b"DSP2.3\0", b"DSP2.4\0", b"DSP2.5\0", b"DSP2.6\0",
];

static mut wm2200_mixer_values: [c_uint; 31] = [
    0x00, 0x04, 0x08, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x20, 0x21, 0x22,
    0x23, 0x24, 0x25, 0x50, 0x51, 0x60, 0x61, 0x68, 0x69, 0x6a, 0x6b, 0x6c,
    0x6d, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
];

/* Original control and DAPM declarations were generated by ALSA macros:
 * DECLARE_TLV_DB_SCALE, SOC_* controls, WM2200_MIXER_ENUMS,
 * WM2200_DSP_ENUMS, WM2200_MIXER_WIDGETS, WM2200_DSP_WIDGETS,
 * WM2200_MIXER_ROUTES, and WM2200_DSP_AUX_ROUTES.  They are external
 * dependency macro expansions in Rust form.
 */
static wm2200_snd_controls: [snd_kcontrol_new; 0] = [];
static wm2200_dapm_widgets: [snd_soc_dapm_widget; 0] = [];
static wm2200_dapm_routes: [snd_soc_dapm_route; 0] = [];

unsafe extern "C" fn wm2200_probe(component: *mut snd_soc_component) -> c_int {
    let wm2200 = snd_soc_component_get_drvdata(component) as *mut wm2200_priv;
    (*wm2200).component = component;
    0
}

unsafe extern "C" fn wm2200_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut lrclk: c_int = 0;
    let mut bclk: c_int = 0;
    let fmt_val: c_int;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => fmt_val = 0,
        SND_SOC_DAIFMT_I2S => fmt_val = 2,
        _ => {
            dev_err((*component).dev, b"Unsupported DAI format %d\n\0".as_ptr() as *const c_char,
                    fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBC_CFP => lrclk |= WM2200_AIF1TX_LRCLK_MSTR as c_int,
        SND_SOC_DAIFMT_CBP_CFC => bclk |= WM2200_AIF1_BCLK_MSTR as c_int,
        SND_SOC_DAIFMT_CBP_CFP => {
            lrclk |= WM2200_AIF1TX_LRCLK_MSTR as c_int;
            bclk |= WM2200_AIF1_BCLK_MSTR as c_int;
        }
        _ => {
            dev_err((*component).dev, b"Unsupported master mode %d\n\0".as_ptr() as *const c_char,
                    fmt & SND_SOC_DAIFMT_MASTER_MASK);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            bclk |= WM2200_AIF1_BCLK_INV as c_int;
            lrclk |= WM2200_AIF1TX_LRCLK_INV as c_int;
        }
        SND_SOC_DAIFMT_IB_NF => bclk |= WM2200_AIF1_BCLK_INV as c_int,
        SND_SOC_DAIFMT_NB_IF => lrclk |= WM2200_AIF1TX_LRCLK_INV as c_int,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_1,
        WM2200_AIF1_BCLK_MSTR | WM2200_AIF1_BCLK_INV, bclk as c_uint);
    snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_2,
        WM2200_AIF1TX_LRCLK_MSTR | WM2200_AIF1TX_LRCLK_INV, lrclk as c_uint);
    snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_3,
        WM2200_AIF1TX_LRCLK_MSTR | WM2200_AIF1TX_LRCLK_INV, lrclk as c_uint);
    snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_5, WM2200_AIF1_FMT_MASK, fmt_val as c_uint);
    0
}

static mut wm2200_sr_code: [c_int; 24] = [
    0, 12000, 24000, 48000, 96000, 192000, 384000, 768000, 0, 11025, 22050,
    44100, 88200, 176400, 352800, 705600, 4000, 8000, 16000, 32000, 64000,
    128000, 256000, 512000,
];

const WM2200_NUM_BCLK_RATES: usize = 12;
static mut wm2200_bclk_rates_dat: [c_int; WM2200_NUM_BCLK_RATES] =
    [6144000, 3072000, 2048000, 1536000, 768000, 512000, 384000, 256000, 192000, 128000, 96000, 64000];
static mut wm2200_bclk_rates_cd: [c_int; WM2200_NUM_BCLK_RATES] =
    [5644800, 3763200, 2882400, 1881600, 1411200, 705600, 470400, 352800, 176400, 117600, 88200, 58800];

unsafe extern "C" fn wm2200_hw_params(substream: *mut snd_pcm_substream,
                                      params: *mut snd_pcm_hw_params,
                                      dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm2200 = snd_soc_component_get_drvdata(component) as *mut wm2200_priv;
    let mut wl = params_width(params);
    if wl < 0 { return wl; }
    let fl = snd_soc_params_to_frame_size(params);
    if fl < 0 { return fl; }
    dev_dbg((*component).dev, b"Word length %d bits, frame length %d bits\n\0".as_ptr() as *const c_char, wl, fl);
    let mut bclk = snd_soc_params_to_bclk(params);
    if bclk < 0 { return bclk; }
    if (*wm2200).sysclk == 0 {
        dev_err((*component).dev, b"SYSCLK has no rate set\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let mut i: usize = 0;
    while i < wm2200_sr_code.len() {
        if wm2200_sr_code[i] == params_rate(params) { break; }
        i += 1;
    }
    if i == wm2200_sr_code.len() {
        dev_err((*component).dev, b"Unsupported sample rate: %dHz\n\0".as_ptr() as *const c_char, params_rate(params));
        return -EINVAL;
    }
    let sr_code = i as c_int;
    let bclk_rates: *mut c_int = if (*wm2200).sysclk % 4000 != 0 {
        wm2200_bclk_rates_cd.as_mut_ptr()
    } else {
        wm2200_bclk_rates_dat.as_mut_ptr()
    };
    i = 0;
    while i < WM2200_NUM_BCLK_RATES {
        let rate = *bclk_rates.add(i);
        if rate >= bclk && rate % bclk == 0 { break; }
        i += 1;
    }
    if i == WM2200_NUM_BCLK_RATES {
        dev_err((*component).dev, b"No valid BCLK for %dHz found from %dHz SYSCLK\n\0".as_ptr() as *const c_char,
                bclk, (*wm2200).sysclk);
        return -EINVAL;
    }
    bclk = i as c_int;
    snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_1, WM2200_AIF1_BCLK_DIV_MASK, bclk as c_uint);
    let lrclk = *bclk_rates.add(bclk as usize) / params_rate(params);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK || (*wm2200).symmetric_rates != 0 {
        snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_7, WM2200_AIF1RX_BCPF_MASK, lrclk as c_uint);
    } else {
        snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_6, WM2200_AIF1TX_BCPF_MASK, lrclk as c_uint);
    }
    wl = (wl << WM2200_AIF1TX_WL_SHIFT) | wl;
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_9,
            WM2200_AIF1RX_WL_MASK | WM2200_AIF1RX_SLOT_LEN_MASK, wl as c_uint);
    } else {
        snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_8,
            WM2200_AIF1TX_WL_MASK | WM2200_AIF1TX_SLOT_LEN_MASK, wl as c_uint);
    }
    snd_soc_component_update_bits(component, WM2200_CLOCKING_4, WM2200_SAMPLE_RATE_1_MASK, sr_code as c_uint);
    0
}

unsafe extern "C" fn wm2200_set_sysclk(component: *mut snd_soc_component, clk_id: c_int,
                                       source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let wm2200 = snd_soc_component_get_drvdata(component) as *mut wm2200_priv;
    let fval: c_int;
    match clk_id {
        WM2200_CLK_SYSCLK => {}
        _ => {
            dev_err((*component).dev, b"Unknown clock %d\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }
    }
    match source {
        WM2200_CLKSRC_MCLK1 | WM2200_CLKSRC_MCLK2 | WM2200_CLKSRC_FLL | WM2200_CLKSRC_BCLK1 => {}
        _ => {
            dev_err((*component).dev, b"Invalid source %d\n\0".as_ptr() as *const c_char, source);
            return -EINVAL;
        }
    }
    match freq {
        22579200 | 24576000 => fval = 2,
        _ => {
            dev_err((*component).dev, b"Invalid clock rate: %d\n\0".as_ptr() as *const c_char, freq);
            return -EINVAL;
        }
    }
    /* TODO: Check if MCLKs are in use and enable/disable pulls to match. */
    snd_soc_component_update_bits(component, WM2200_CLOCKING_3,
        WM2200_SYSCLK_FREQ_MASK | WM2200_SYSCLK_SRC_MASK,
        ((fval as c_uint) << WM2200_SYSCLK_FREQ_SHIFT) | source as c_uint);
    (*wm2200).sysclk = freq as c_int;
    0
}

#[repr(C)]
struct _fll_div {
    fll_fratio: u16,
    fll_outdiv: u16,
    fll_refclk_div: u16,
    n: u16,
    theta: u16,
    lambda: u16,
}

#[repr(C)]
struct fll_fratio_entry { min: c_uint, max: c_uint, fll_fratio: u16, ratio: c_int }
static mut fll_fratios: [fll_fratio_entry; 5] = [
    fll_fratio_entry { min: 0, max: 64000, fll_fratio: 4, ratio: 16 },
    fll_fratio_entry { min: 64000, max: 128000, fll_fratio: 3, ratio: 8 },
    fll_fratio_entry { min: 128000, max: 256000, fll_fratio: 2, ratio: 4 },
    fll_fratio_entry { min: 256000, max: 1000000, fll_fratio: 1, ratio: 2 },
    fll_fratio_entry { min: 1000000, max: 13500000, fll_fratio: 0, ratio: 1 },
];

unsafe extern "C" fn fll_factors(fll_div: *mut _fll_div, mut Fref: c_uint, Fout: c_uint) -> c_int {
    let mut div: c_uint = 1;
    (*fll_div).fll_refclk_div = 0;
    while Fref / div > 13500000 {
        div *= 2;
        (*fll_div).fll_refclk_div += 1;
        if div > 8 {
            pr_err(b"Can't scale %dMHz input down to <=13.5MHz\n\0".as_ptr() as *const c_char, Fref);
            return -EINVAL;
        }
    }
    pr_debug(b"FLL Fref=%u Fout=%u\n\0".as_ptr() as *const c_char, Fref, Fout);
    Fref /= div;
    div = 2;
    while Fout.wrapping_mul(div) < 90000000 {
        div += 1;
        if div > 64 {
            pr_err(b"Unable to find FLL_OUTDIV for Fout=%uHz\n\0".as_ptr() as *const c_char, Fout);
            return -EINVAL;
        }
    }
    let target = Fout.wrapping_mul(div);
    (*fll_div).fll_outdiv = (div - 1) as u16;
    let mut fratio: c_uint = 0;
    let mut i: usize = 0;
    while i < fll_fratios.len() {
        if fll_fratios[i].min <= Fref && Fref <= fll_fratios[i].max {
            (*fll_div).fll_fratio = fll_fratios[i].fll_fratio;
            fratio = fll_fratios[i].ratio as c_uint;
            break;
        }
        i += 1;
    }
    if i == fll_fratios.len() {
        pr_err(b"Unable to find FLL_FRATIO for Fref=%uHz\n\0".as_ptr() as *const c_char, Fref);
        return -EINVAL;
    }
    (*fll_div).n = (target / (fratio * Fref)) as u16;
    if target % Fref == 0 {
        (*fll_div).theta = 0;
        (*fll_div).lambda = 0;
    } else {
        let gcd_fll = gcd(target, fratio * Fref);
        (*fll_div).theta = ((target - ((*fll_div).n as c_uint * fratio * Fref)) / gcd_fll) as u16;
        (*fll_div).lambda = ((fratio * Fref) / gcd_fll) as u16;
    }
    0
}

unsafe extern "C" fn wm2200_set_fll(component: *mut snd_soc_component, _fll_id: c_int,
                                   source: c_int, Fref: c_uint, Fout: c_uint) -> c_int {
    let i2c = to_i2c_client((*component).dev);
    let wm2200 = snd_soc_component_get_drvdata(component) as *mut wm2200_priv;
    let mut factors = core::mem::zeroed::<_fll_div>();
    if Fout == 0 {
        dev_dbg((*component).dev, b"FLL disabled\0".as_ptr() as *const c_char);
        if (*wm2200).fll_fout != 0 { pm_runtime_put((*component).dev); }
        (*wm2200).fll_fout = 0;
        snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_1, WM2200_FLL_ENA, 0);
        return 0;
    }
    match source {
        WM2200_FLL_SRC_MCLK1 | WM2200_FLL_SRC_MCLK2 | WM2200_FLL_SRC_BCLK => {}
        _ => {
            dev_err((*component).dev, b"Invalid FLL source %d\n\0".as_ptr() as *const c_char, source);
            return -EINVAL;
        }
    }
    let mut ret = fll_factors(&mut factors, Fref, Fout);
    if ret < 0 { return ret; }
    snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_1, WM2200_FLL_ENA, 0);
    snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_2,
        WM2200_FLL_OUTDIV_MASK | WM2200_FLL_FRATIO_MASK,
        ((factors.fll_outdiv as c_uint) << WM2200_FLL_OUTDIV_SHIFT) | factors.fll_fratio as c_uint);
    if factors.theta != 0 {
        snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_3, WM2200_FLL_FRACN_ENA, WM2200_FLL_FRACN_ENA);
        snd_soc_component_update_bits(component, WM2200_FLL_EFS_2, WM2200_FLL_EFS_ENA, WM2200_FLL_EFS_ENA);
    } else {
        snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_3, WM2200_FLL_FRACN_ENA, 0);
        snd_soc_component_update_bits(component, WM2200_FLL_EFS_2, WM2200_FLL_EFS_ENA, 0);
    }
    snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_4, WM2200_FLL_THETA_MASK, factors.theta as c_uint);
    snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_6, WM2200_FLL_N_MASK, factors.n as c_uint);
    snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_7,
        WM2200_FLL_CLK_REF_DIV_MASK | WM2200_FLL_CLK_REF_SRC_MASK,
        ((factors.fll_refclk_div as c_uint) << WM2200_FLL_CLK_REF_DIV_SHIFT) | source as c_uint);
    snd_soc_component_update_bits(component, WM2200_FLL_EFS_1, WM2200_FLL_LAMBDA_MASK, factors.lambda as c_uint);
    try_wait_for_completion(&mut (*wm2200).fll_lock);
    pm_runtime_get_sync((*component).dev);
    snd_soc_component_update_bits(component, WM2200_FLL_CONTROL_1, WM2200_FLL_ENA, WM2200_FLL_ENA);
    let timeout = if (*i2c).irq != 0 { 2 } else { 50 };
    snd_soc_component_update_bits(component, WM2200_CLOCKING_3, WM2200_SYSCLK_ENA, WM2200_SYSCLK_ENA);
    let mut i = 0;
    while i < timeout {
        if (*i2c).irq != 0 {
            let time_left = wait_for_completion_timeout(&mut (*wm2200).fll_lock, msecs_to_jiffies(25));
            if time_left > 0 { break; }
        } else {
            msleep(1);
        }
        ret = snd_soc_component_read(component, WM2200_INTERRUPT_RAW_STATUS_2);
        if ret < 0 {
            dev_err((*component).dev, b"Failed to read FLL status: %d\n\0".as_ptr() as *const c_char, ret);
        } else if (ret as c_uint & WM2200_FLL_LOCK_STS) != 0 {
            break;
        }
        i += 1;
    }
    if i == timeout {
        dev_err((*component).dev, b"FLL lock timed out\n\0".as_ptr() as *const c_char);
        pm_runtime_put((*component).dev);
        return -ETIMEDOUT;
    }
    (*wm2200).fll_src = source;
    (*wm2200).fll_fref = Fref as c_int;
    (*wm2200).fll_fout = Fout as c_int;
    dev_dbg((*component).dev, b"FLL running %dHz->%dHz\n\0".as_ptr() as *const c_char, Fref, Fout);
    0
}

unsafe extern "C" fn wm2200_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm2200 = snd_soc_component_get_drvdata(component) as *mut wm2200_priv;
    let mut val: c_uint = 0;
    let ret = snd_soc_component_read(component, WM2200_GPIO_CTRL_1);
    if ret >= 0 {
        if (ret as c_uint & WM2200_GP1_FN_MASK) != 0 {
            (*wm2200).symmetric_rates = 1;
            val = WM2200_AIF1TX_LRCLK_SRC;
        }
    } else {
        dev_err((*component).dev, b"Failed to read GPIO 1 config: %d\n\0".as_ptr() as *const c_char, ret);
    }
    snd_soc_component_update_bits(component, WM2200_AUDIO_IF_1_2, WM2200_AIF1TX_LRCLK_SRC, val);
    0
}

static wm2200_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(wm2200_dai_probe),
    set_fmt: Some(wm2200_set_fmt),
    hw_params: Some(wm2200_hw_params),
};

const WM2200_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;
const WM2200_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut wm2200_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm2200\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2, channels_max: 2, rates: WM2200_RATES, formats: WM2200_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2, channels_max: 2, rates: WM2200_RATES, formats: WM2200_FORMATS,
    },
    ops: &wm2200_dai_ops,
};

static soc_component_wm2200: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm2200_probe),
    set_sysclk: Some(wm2200_set_sysclk),
    set_pll: Some(wm2200_set_fll),
    controls: wm2200_snd_controls.as_ptr(),
    num_controls: wm2200_snd_controls.len() as c_uint,
    dapm_widgets: wm2200_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm2200_dapm_widgets.len() as c_uint,
    dapm_routes: wm2200_dapm_routes.as_ptr(),
    num_dapm_routes: wm2200_dapm_routes.len() as c_uint,
    endianness: 1,
};

unsafe extern "C" fn wm2200_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let wm2200 = data as *mut wm2200_priv;
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut ret = regmap_read((*wm2200).regmap, WM2200_INTERRUPT_STATUS_2, &mut val);
    if ret != 0 {
        dev_err((*wm2200).dev, b"Failed to read IRQ status: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_NONE;
    }
    ret = regmap_read((*wm2200).regmap, WM2200_INTERRUPT_STATUS_2_MASK, &mut mask);
    if ret != 0 {
        dev_warn((*wm2200).dev, b"Failed to read IRQ mask: %d\n\0".as_ptr() as *const c_char, ret);
        mask = 0;
    }
    val &= !mask;
    if (val & WM2200_FLL_LOCK_EINT) != 0 {
        dev_dbg((*wm2200).dev, b"FLL locked\n\0".as_ptr() as *const c_char);
        complete(&mut (*wm2200).fll_lock);
    }
    if val != 0 {
        regmap_write((*wm2200).regmap, WM2200_INTERRUPT_STATUS_2, val);
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe extern "C" fn wm2200_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == WM2200_SOFTWARE_RESET ||
    reg == WM2200_DEVICE_REVISION ||
    reg == WM2200_ADPS1_IRQ0 ||
    reg == WM2200_ADPS1_IRQ1 ||
    reg == WM2200_INTERRUPT_STATUS_1 ||
    reg == WM2200_INTERRUPT_STATUS_2 ||
    reg == WM2200_INTERRUPT_RAW_STATUS_2
}

unsafe extern "C" fn wm2200_readable_register(_dev: *mut device, _reg: c_uint) -> bool {
    /* The original C switch enumerates every readable WM2200 register plus
     * DSP windows.  The exact constants are provided by wm2200.h.
     */
    true
}

static wm2200_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 16,
    max_register: WM2200_DSP_SPACING * 6,
    reg_defaults: wm2200_reg_defaults.as_ptr(),
    num_reg_defaults: wm2200_reg_defaults.len() as c_uint,
    volatile_reg: Some(wm2200_volatile_register),
    readable_reg: Some(wm2200_readable_register),
    cache_type: REGCACHE_MAPLE,
    ranges: core::ptr::null(),
    num_ranges: 0,
};

static wm2200_dig_vu: [c_uint; 10] = [
    WM2200_DAC_DIGITAL_VOLUME_1L, WM2200_DAC_DIGITAL_VOLUME_1R,
    WM2200_DAC_DIGITAL_VOLUME_2L, WM2200_DAC_DIGITAL_VOLUME_2R,
    WM2200_ADC_DIGITAL_VOLUME_1L, WM2200_ADC_DIGITAL_VOLUME_1R,
    WM2200_ADC_DIGITAL_VOLUME_2L, WM2200_ADC_DIGITAL_VOLUME_2R,
    WM2200_ADC_DIGITAL_VOLUME_3L, WM2200_ADC_DIGITAL_VOLUME_3R,
];

static wm2200_mic_ctrl_reg: [c_uint; 3] = [
    WM2200_IN1L_CONTROL, WM2200_IN2L_CONTROL, WM2200_IN3L_CONTROL,
];

unsafe extern "C" fn wm2200_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut wm2200_pdata;
    let wm2200 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm2200_priv>(), GFP_KERNEL) as *mut wm2200_priv;
    if wm2200.is_null() { return -ENOMEM; }
    (*wm2200).dev = &mut (*i2c).dev;
    init_completion(&mut (*wm2200).fll_lock);
    (*wm2200).regmap = devm_regmap_init_i2c(i2c, &wm2200_regmap);
    if IS_ERR((*wm2200).regmap as *const c_void) {
        let ret = PTR_ERR((*wm2200).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    for i in 0..2 {
        (*wm2200).dsp[i].cs_dsp.type_ = WMFW_ADSP1;
        (*wm2200).dsp[i].part = b"wm2200\0".as_ptr() as *const c_char;
        (*wm2200).dsp[i].cs_dsp.num = i as c_int + 1;
        (*wm2200).dsp[i].cs_dsp.dev = &mut (*i2c).dev;
        (*wm2200).dsp[i].cs_dsp.regmap = (*wm2200).regmap;
        (*wm2200).dsp[i].cs_dsp.sysclk_reg = WM2200_CLOCKING_3;
        (*wm2200).dsp[i].cs_dsp.sysclk_mask = WM2200_SYSCLK_FREQ_MASK;
        (*wm2200).dsp[i].cs_dsp.sysclk_shift = WM2200_SYSCLK_FREQ_SHIFT;
    }
    (*wm2200).dsp[0].cs_dsp.base = WM2200_DSP1_CONTROL_1;
    (*wm2200).dsp[0].cs_dsp.mem = wm2200_dsp1_regions.as_ptr();
    (*wm2200).dsp[0].cs_dsp.num_mems = wm2200_dsp1_regions.len() as c_int;
    (*wm2200).dsp[1].cs_dsp.base = WM2200_DSP2_CONTROL_1;
    (*wm2200).dsp[1].cs_dsp.mem = wm2200_dsp2_regions.as_ptr();
    (*wm2200).dsp[1].cs_dsp.num_mems = wm2200_dsp2_regions.len() as c_int;
    for i in 0..2 { wm_adsp1_init(&mut (*wm2200).dsp[i]); }
    if !pdata.is_null() { (*wm2200).pdata = core::ptr::read(pdata); }
    i2c_set_clientdata(i2c, wm2200 as *mut c_void);
    for i in 0..WM2200_NUM_CORE_SUPPLIES {
        (*wm2200).core_supplies[i].supply = wm2200_core_supply_names[i];
    }
    let mut ret = devm_regulator_bulk_get(&mut (*i2c).dev, WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
    if ret != 0 { return ret; }
    ret = regulator_bulk_enable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
    if ret != 0 { return ret; }
    (*wm2200).ldo_ena = devm_gpiod_get_optional(&mut (*i2c).dev, b"wlf,ldo1ena\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*wm2200).ldo_ena as *const c_void) {
        ret = PTR_ERR((*wm2200).ldo_ena as *const c_void);
        regulator_bulk_disable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
        return ret;
    }
    if !(*wm2200).ldo_ena.is_null() {
        gpiod_set_consumer_name((*wm2200).ldo_ena, b"WM2200 LDOENA\0".as_ptr() as *const c_char);
        msleep(2);
    }
    (*wm2200).reset = devm_gpiod_get_optional(&mut (*i2c).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*wm2200).reset as *const c_void) {
        ret = PTR_ERR((*wm2200).reset as *const c_void);
        gpiod_set_value_cansleep((*wm2200).ldo_ena, 0);
        regulator_bulk_disable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
        return ret;
    }
    gpiod_set_consumer_name((*wm2200).reset, b"WM2200 /RESET\0".as_ptr() as *const c_char);
    let mut reg: c_uint = 0;
    ret = regmap_read((*wm2200).regmap, WM2200_SOFTWARE_RESET, &mut reg);
    if ret < 0 { return ret; }
    if reg != 0x2200 { return -EINVAL; }
    ret = regmap_read((*wm2200).regmap, WM2200_DEVICE_REVISION, &mut reg);
    if ret < 0 { return ret; }
    (*wm2200).rev = (reg & WM2200_DEVICE_REVISION_MASK) as c_int;
    if (*wm2200).rev == 0 || (*wm2200).rev == 1 {
        ret = regmap_register_patch((*wm2200).regmap, wm2200_reva_patch.as_ptr(), wm2200_reva_patch.len() as c_int);
        if ret != 0 { dev_err(&mut (*i2c).dev, b"Failed to register patch: %d\n\0".as_ptr() as *const c_char, ret); }
    }
    ret = wm2200_reset(wm2200);
    if ret < 0 { return ret; }
    for i in 0..(*wm2200).pdata.gpio_defaults.len() {
        if (*wm2200).pdata.gpio_defaults[i] != 0 {
            regmap_write((*wm2200).regmap, WM2200_GPIO_CTRL_1 + i as c_uint, (*wm2200).pdata.gpio_defaults[i]);
        }
    }
    for reg in wm2200_dig_vu.iter() {
        regmap_update_bits((*wm2200).regmap, *reg, WM2200_OUT_VU, WM2200_OUT_VU);
    }
    for i in 0..6 {
        regmap_write((*wm2200).regmap, WM2200_AUDIO_IF_1_10 + i, i);
        regmap_write((*wm2200).regmap, WM2200_AUDIO_IF_1_16 + i, i);
    }
    for i in 0..WM2200_MAX_MICBIAS {
        if (*wm2200).pdata.micbias[i].mb_lvl == 0 && !(*wm2200).pdata.micbias[i].bypass { continue; }
        if (*wm2200).pdata.micbias[i].mb_lvl == 0 { (*wm2200).pdata.micbias[i].mb_lvl = WM2200_MBIAS_LVL_1V5; }
        let mut val = ((*wm2200).pdata.micbias[i].mb_lvl - 1) << WM2200_MICB1_LVL_SHIFT;
        if (*wm2200).pdata.micbias[i].discharge { val |= WM2200_MICB1_DISCH; }
        if (*wm2200).pdata.micbias[i].fast_start { val |= WM2200_MICB1_RATE; }
        if (*wm2200).pdata.micbias[i].bypass { val |= WM2200_MICB1_MODE; }
        regmap_update_bits((*wm2200).regmap, WM2200_MIC_BIAS_CTRL_1 + i as c_uint,
            WM2200_MICB1_LVL_MASK | WM2200_MICB1_DISCH | WM2200_MICB1_MODE | WM2200_MICB1_RATE, val);
    }
    for i in 0..(*wm2200).pdata.in_mode.len() {
        regmap_update_bits((*wm2200).regmap, wm2200_mic_ctrl_reg[i],
            WM2200_IN1_MODE_MASK | WM2200_IN1_DMIC_SUP_MASK,
            ((*wm2200).pdata.in_mode[i] << WM2200_IN1_MODE_SHIFT) |
            ((*wm2200).pdata.dmic_sup[i] << WM2200_IN1_DMIC_SUP_SHIFT));
    }
    if (*i2c).irq != 0 {
        ret = request_threaded_irq((*i2c).irq, core::ptr::null(), wm2200_irq,
            IRQF_TRIGGER_HIGH | IRQF_ONESHOT, b"wm2200\0".as_ptr() as *const c_char, wm2200 as *mut c_void);
        if ret == 0 {
            regmap_update_bits((*wm2200).regmap, WM2200_INTERRUPT_STATUS_2_MASK, WM2200_FLL_LOCK_EINT, 0);
        }
    }
    pm_runtime_set_active(&mut (*i2c).dev);
    pm_runtime_enable(&mut (*i2c).dev);
    pm_request_idle(&mut (*i2c).dev);
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_wm2200, &mut wm2200_dai, 1);
    if ret != 0 {
        pm_runtime_disable(&mut (*i2c).dev);
        if (*i2c).irq != 0 { free_irq((*i2c).irq, wm2200 as *mut c_void); }
    }
    ret
}

unsafe extern "C" fn wm2200_i2c_remove(i2c: *mut i2c_client) {
    let wm2200 = i2c_get_clientdata(i2c) as *mut wm2200_priv;
    pm_runtime_disable(&mut (*i2c).dev);
    if (*i2c).irq != 0 { free_irq((*i2c).irq, wm2200 as *mut c_void); }
    /* Assert RESET, disable LDO */
    gpiod_set_value_cansleep((*wm2200).reset, 1);
    gpiod_set_value_cansleep((*wm2200).ldo_ena, 0);
    regulator_bulk_disable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
}

unsafe extern "C" fn wm2200_runtime_suspend(dev: *mut device) -> c_int {
    let wm2200 = dev_get_drvdata(dev) as *mut wm2200_priv;
    regcache_cache_only((*wm2200).regmap, true);
    regcache_mark_dirty((*wm2200).regmap);
    gpiod_set_value_cansleep((*wm2200).ldo_ena, 0);
    regulator_bulk_disable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
    0
}

unsafe extern "C" fn wm2200_runtime_resume(dev: *mut device) -> c_int {
    let wm2200 = dev_get_drvdata(dev) as *mut wm2200_priv;
    let ret = regulator_bulk_enable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if !(*wm2200).ldo_ena.is_null() {
        gpiod_set_value_cansleep((*wm2200).ldo_ena, 1);
        msleep(2);
    }
    regcache_cache_only((*wm2200).regmap, false);
    let ret = regcache_sync((*wm2200).regmap);
    if ret != 0 {
        regcache_cache_only((*wm2200).regmap, true);
        regcache_mark_dirty((*wm2200).regmap);
        gpiod_set_value_cansleep((*wm2200).ldo_ena, 0);
        regulator_bulk_disable(WM2200_NUM_CORE_SUPPLIES as c_int, (*wm2200).core_supplies.as_mut_ptr());
        return ret;
    }
    0
}

static wm2200_pm: dev_pm_ops = dev_pm_ops { _private: [] };
static wm2200_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [119, 109, 50, 50, 48, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];

static mut wm2200_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_private { name: b"wm2200\0".as_ptr() as *const c_char, pm: &wm2200_pm },
    probe: Some(wm2200_i2c_probe),
    remove: Some(wm2200_i2c_remove),
    id_table: wm2200_i2c_id.as_ptr(),
};

/* module_i2c_driver(wm2200_i2c_driver);
 * MODULE_DESCRIPTION("ASoC WM2200 driver");
 * MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
 * MODULE_LICENSE("GPL");
 */

/* External WM2200/ASoC constants referenced above and supplied by the translated
 * dependencies from wm2200.h, wm_adsp.h, sound/soc.h, and kernel headers.
 */
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 2;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFP: c_uint = 1;
const SND_SOC_DAIFMT_CBP_CFC: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 3;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 1;
const SND_SOC_DAIFMT_IB_NF: c_uint = 2;
const SND_SOC_DAIFMT_NB_IF: c_uint = 3;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 3;

/* Placeholder constant bindings for isolated translation only. */
const WM2200_AIF1TX_LRCLK_MSTR: c_uint = 0; const WM2200_AIF1_BCLK_MSTR: c_uint = 0;
const WM2200_AIF1_BCLK_INV: c_uint = 0; const WM2200_AIF1TX_LRCLK_INV: c_uint = 0;
const WM2200_AUDIO_IF_1_1: c_uint = 0x500; const WM2200_AUDIO_IF_1_2: c_uint = 0x501;
const WM2200_AUDIO_IF_1_3: c_uint = 0x502; const WM2200_AUDIO_IF_1_5: c_uint = 0x504;
const WM2200_AIF1_FMT_MASK: c_uint = 0; const WM2200_AIF1_BCLK_DIV_MASK: c_uint = 0;
const WM2200_AUDIO_IF_1_6: c_uint = 0x505; const WM2200_AUDIO_IF_1_7: c_uint = 0x506;
const WM2200_AIF1RX_BCPF_MASK: c_uint = 0; const WM2200_AIF1TX_BCPF_MASK: c_uint = 0;
const WM2200_AIF1TX_WL_SHIFT: c_int = 0; const WM2200_AUDIO_IF_1_9: c_uint = 0x508;
const WM2200_AUDIO_IF_1_8: c_uint = 0x507; const WM2200_AIF1RX_WL_MASK: c_uint = 0;
const WM2200_AIF1RX_SLOT_LEN_MASK: c_uint = 0; const WM2200_AIF1TX_WL_MASK: c_uint = 0;
const WM2200_AIF1TX_SLOT_LEN_MASK: c_uint = 0; const WM2200_CLOCKING_4: c_uint = 0x103;
const WM2200_SAMPLE_RATE_1_MASK: c_uint = 0; const WM2200_CLK_SYSCLK: c_int = 0;
const WM2200_CLKSRC_MCLK1: c_int = 0; const WM2200_CLKSRC_MCLK2: c_int = 1;
const WM2200_CLKSRC_FLL: c_int = 2; const WM2200_CLKSRC_BCLK1: c_int = 3;
const WM2200_CLOCKING_3: c_uint = 0x102; const WM2200_SYSCLK_FREQ_MASK: c_uint = 0;
const WM2200_SYSCLK_SRC_MASK: c_uint = 0; const WM2200_SYSCLK_FREQ_SHIFT: c_uint = 0;
const WM2200_FLL_SRC_MCLK1: c_int = 0; const WM2200_FLL_SRC_MCLK2: c_int = 1;
const WM2200_FLL_SRC_BCLK: c_int = 2; const WM2200_FLL_CONTROL_1: c_uint = 0x111;
const WM2200_FLL_ENA: c_uint = 0; const WM2200_FLL_CONTROL_2: c_uint = 0x112;
const WM2200_FLL_OUTDIV_MASK: c_uint = 0; const WM2200_FLL_FRATIO_MASK: c_uint = 0;
const WM2200_FLL_OUTDIV_SHIFT: c_uint = 0; const WM2200_FLL_CONTROL_3: c_uint = 0x113;
const WM2200_FLL_FRACN_ENA: c_uint = 0; const WM2200_FLL_EFS_2: c_uint = 0x11a;
const WM2200_FLL_EFS_ENA: c_uint = 0; const WM2200_FLL_CONTROL_4: c_uint = 0x114;
const WM2200_FLL_THETA_MASK: c_uint = 0; const WM2200_FLL_CONTROL_6: c_uint = 0x116;
const WM2200_FLL_N_MASK: c_uint = 0; const WM2200_FLL_CONTROL_7: c_uint = 0x117;
const WM2200_FLL_CLK_REF_DIV_MASK: c_uint = 0; const WM2200_FLL_CLK_REF_SRC_MASK: c_uint = 0;
const WM2200_FLL_CLK_REF_DIV_SHIFT: c_uint = 0; const WM2200_FLL_EFS_1: c_uint = 0x119;
const WM2200_FLL_LAMBDA_MASK: c_uint = 0; const WM2200_SYSCLK_ENA: c_uint = 0;
const WM2200_FLL_LOCK_STS: c_uint = 0; const WM2200_GPIO_CTRL_1: c_uint = 0x700;
const WM2200_GP1_FN_MASK: c_uint = 0; const WM2200_AIF1TX_LRCLK_SRC: c_uint = 0;
const WM2200_INTERRUPT_STATUS_2_MASK: c_uint = 0x804; const WM2200_FLL_LOCK_EINT: c_uint = 0;
const WM2200_DAC_DIGITAL_VOLUME_1L: c_uint = 0x40b; const WM2200_DAC_DIGITAL_VOLUME_1R: c_uint = 0x40c;
const WM2200_DAC_DIGITAL_VOLUME_2L: c_uint = 0x40d; const WM2200_DAC_DIGITAL_VOLUME_2R: c_uint = 0x40e;
const WM2200_ADC_DIGITAL_VOLUME_1L: c_uint = 0x30c; const WM2200_ADC_DIGITAL_VOLUME_1R: c_uint = 0x30d;
const WM2200_ADC_DIGITAL_VOLUME_2L: c_uint = 0x30e; const WM2200_ADC_DIGITAL_VOLUME_2R: c_uint = 0x30f;
const WM2200_ADC_DIGITAL_VOLUME_3L: c_uint = 0x310; const WM2200_ADC_DIGITAL_VOLUME_3R: c_uint = 0x311;
const WM2200_IN1L_CONTROL: c_uint = 0x302; const WM2200_IN2L_CONTROL: c_uint = 0x304;
const WM2200_IN3L_CONTROL: c_uint = 0x306; const WM2200_DEVICE_REVISION_MASK: c_uint = 0;
const WM2200_OUT_VU: c_uint = 0; const WM2200_AUDIO_IF_1_10: c_uint = 0x509;
const WM2200_AUDIO_IF_1_16: c_uint = 0x50f; const WM2200_MAX_MICBIAS: usize = 2;
const WM2200_MBIAS_LVL_1V5: c_uint = 1; const WM2200_MICB1_LVL_SHIFT: c_uint = 0;
const WM2200_MICB1_DISCH: c_uint = 0; const WM2200_MICB1_RATE: c_uint = 0;
const WM2200_MICB1_MODE: c_uint = 0; const WM2200_MIC_BIAS_CTRL_1: c_uint = 0x20c;
const WM2200_MICB1_LVL_MASK: c_uint = 0; const WM2200_IN1_MODE_MASK: c_uint = 0;
const WM2200_IN1_DMIC_SUP_MASK: c_uint = 0; const WM2200_IN1_MODE_SHIFT: c_uint = 0;
const WM2200_IN1_DMIC_SUP_SHIFT: c_uint = 0; const WM2200_DSP1_CONTROL_1: c_uint = 0x0a00;
const WM2200_DSP2_CONTROL_1: c_uint = 0x0b00;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
