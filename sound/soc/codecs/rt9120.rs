// SPDX-License-Identifier: GPL-2.0

// Translated from Linux C implementation source. Kernel/ASoC/regmap symbols
// referenced here are external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type __be16 = u16;
type __be32 = u32;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const RT9120_REG_DEVID: c_uint = 0x00;
const RT9120_REG_I2SFMT: c_uint = 0x02;
const RT9120_REG_I2SWL: c_uint = 0x03;
const RT9120_REG_SDIOSEL: c_uint = 0x04;
const RT9120_REG_SYSCTL: c_uint = 0x05;
const RT9120_REG_SPKGAIN: c_uint = 0x07;
const RT9120_REG_VOLRAMP: c_uint = 0x0A;
const RT9120_REG_ERRRPT: c_uint = 0x10;
const RT9120_REG_MSVOL: c_uint = 0x20;
const RT9120_REG_SWRESET: c_uint = 0x40;
const RT9120_REG_INTERCFG: c_uint = 0x63;
const RT9120_REG_INTERNAL0: c_uint = 0x65;
const RT9120_REG_INTERNAL1: c_uint = 0x69;
const RT9120_REG_UVPOPT: c_uint = 0x6C;
const RT9120_REG_DIGCFG: c_uint = 0xF8;

const RT9120_VID_MASK: c_uint = GENMASK(15, 8);
const RT9120_SWRST_MASK: c_uint = BIT(7);
const RT9120_MUTE_MASK: c_uint = GENMASK(5, 4);
const RT9120_I2SFMT_MASK: c_uint = GENMASK(4, 2);
const RT9120_I2SFMT_SHIFT: c_uint = 2;
const RT9120_CFG_FMT_I2S: c_uint = 0;
const RT9120_CFG_FMT_LEFTJ: c_uint = 1;
const RT9120_CFG_FMT_RIGHTJ: c_uint = 2;
const RT9120_CFG_FMT_DSPA: c_uint = 3;
const RT9120_CFG_FMT_DSPB: c_uint = 7;
const RT9120_AUDBIT_MASK: c_uint = GENMASK(1, 0);
const RT9120_CFG_AUDBIT_16: c_uint = 0;
const RT9120_CFG_AUDBIT_20: c_uint = 1;
const RT9120_CFG_AUDBIT_24: c_uint = 2;
const RT9120_AUDWL_MASK: c_uint = GENMASK(5, 0);
const RT9120_CFG_WORDLEN_16: c_uint = 16;
const RT9120_CFG_WORDLEN_24: c_uint = 24;
const RT9120_CFG_WORDLEN_32: c_uint = 32;
const RT9120_DVDD_UVSEL_MASK: c_uint = GENMASK(5, 4);
const RT9120_AUTOSYNC_MASK: c_uint = BIT(6);

const RT9120_VENDOR_ID: c_uint = 0x42;
const RT9120S_VENDOR_ID: c_uint = 0x43;
const RT9120_RESET_WAITMS: c_uint = 20;
const RT9120_CHIPON_WAITMS: c_uint = 20;
const RT9120_AMPON_WAITMS: c_uint = 50;
const RT9120_AMPOFF_WAITMS: c_uint = 100;
const RT9120_LVAPP_THRESUV: c_int = 2000000;

/* 8000 to 192000 supported , only 176400 not support */
const RT9120_RATES_MASK: c_uint = SNDRV_PCM_RATE_8000_192000 & !SNDRV_PCM_RATE_176400;
const RT9120_FMTS_MASK: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

const CHIP_IDX_RT9120: c_int = 0;
const CHIP_IDX_RT9120S: c_int = 1;
const CHIP_IDX_MAX: c_int = 2;

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    dev: *mut device,
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct rt9120_data {
    dev: *mut device,
    regmap: *mut regmap,
    pwdnn_gpio: *mut gpio_desc,
    chip_idx: c_int,
}

#[repr(C)]
struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: usize,
    dapm_widgets: *const snd_soc_dapm_widget_desc,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    endianness: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    rates: c_uint,
    formats: c_uint,
    rate_max: c_uint,
    rate_min: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
}

#[repr(C)]
struct regmap_range {
    range_min: c_uint,
    range_max: c_uint,
}

#[repr(C)]
struct regmap_access_table {
    yes_ranges: *const regmap_range,
    n_yes_ranges: c_uint,
}

#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    cache_type: c_uint,
    reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    wr_table: *const regmap_access_table,
    rd_table: *const regmap_access_table,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

extern "C" {
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;

    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_NOPM: c_int;
    static REGCACHE_RBTREE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_HIGH: c_uint;
    static EINVAL: c_int;
    static EIO: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_write(comp: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(comp: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(comp: *mut snd_soc_component, regmap: *mut regmap);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn snd_soc_component_update_bits(
        comp: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn params_width(param: *mut snd_pcm_hw_params) -> c_int;
    fn params_physical_width(param: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(param: *mut snd_pcm_hw_params) -> c_int;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_smbus_read_i2c_block_data(
        client: *mut i2c_client,
        command: u8,
        length: u8,
        values: *mut u8,
    ) -> c_int;
    fn i2c_smbus_write_i2c_block_data(
        client: *mut i2c_client,
        command: u8,
        length: u8,
        values: *const u8,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint)
        -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

/* 11bit [min,max,step] = [-103.9375dB, 24dB, 0.0625dB] */
// static const DECLARE_TLV_DB_SCALE(digital_tlv, -1039375, 625, 1);
static digital_tlv: [c_uint; 4] = [0, (-1039375i32) as c_uint, 625, 1];

/* {6, 8, 10, 12, 13, 14, 15, 16}dB */
// static const DECLARE_TLV_DB_RANGE(classd_tlv, ...);
static classd_tlv: [c_uint; 8] = [0, 3, 600, 200, 4, 7, 1300, 100];

static sdo_select_text: [*const c_char; 4] = [
    b"None\0".as_ptr() as *const c_char,
    b"INTF\0".as_ptr() as *const c_char,
    b"Final\0".as_ptr() as *const c_char,
    b"RMS Detect\0".as_ptr() as *const c_char,
];

// SOC_ENUM_SINGLE(RT9120_REG_SDIOSEL, 4, ARRAY_SIZE(sdo_select_text), sdo_select_text)
static sdo_select_enum: soc_enum = soc_enum { _private: [] };

// SOC_* control macro initializers are owned by ASoC headers.
static rt9120_snd_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn internal_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMU {
        snd_soc_component_write(comp, RT9120_REG_ERRRPT, 0);
    } else if event == SND_SOC_DAPM_POST_PMU {
        msleep(RT9120_AMPON_WAITMS);
    } else if event == SND_SOC_DAPM_POST_PMD {
        msleep(RT9120_AMPOFF_WAITMS);
    }

    0
}

// SND_SOC_DAPM_* widget macro initializers are represented as opaque entries.
static rt9120_dapm_widgets: [snd_soc_dapm_widget_desc; 8] = [
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
];

static rt9120_dapm_routes: [snd_soc_dapm_route; 11] = [
    snd_soc_dapm_route { sink: b"DMIX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIF Playback\0".as_ptr() as *const c_char },
    /* SPKL */
    snd_soc_dapm_route { sink: b"LDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PWND\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DMIX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKL PA\0".as_ptr() as *const c_char, control: ptr::null(), source: b"LDAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPKL PA\0".as_ptr() as *const c_char },
    /* SPKR */
    snd_soc_dapm_route { sink: b"RDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PWND\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RDAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DMIX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKR PA\0".as_ptr() as *const c_char, control: ptr::null(), source: b"RDAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPKR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPKR PA\0".as_ptr() as *const c_char },
    /* Cap */
    snd_soc_dapm_route { sink: b"AIF Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"LDAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIF Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"RDAC\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn rt9120_codec_probe(comp: *mut snd_soc_component) -> c_int {
    let data = snd_soc_component_get_drvdata(comp) as *mut rt9120_data;

    snd_soc_component_init_regmap(comp, (*data).regmap);
    pm_runtime_get_sync((*comp).dev);

    /* Internal setting */
    if (*data).chip_idx == CHIP_IDX_RT9120S {
        snd_soc_component_write(comp, RT9120_REG_INTERCFG, 0xde);
        snd_soc_component_write(comp, RT9120_REG_INTERNAL0, 0x66);
    } else {
        snd_soc_component_write(comp, RT9120_REG_INTERNAL0, 0x04);
    }

    pm_runtime_mark_last_busy((*comp).dev);
    pm_runtime_put((*comp).dev);

    0
}

unsafe extern "C" fn rt9120_codec_suspend(comp: *mut snd_soc_component) -> c_int {
    pm_runtime_force_suspend((*comp).dev)
}

unsafe extern "C" fn rt9120_codec_resume(comp: *mut snd_soc_component) -> c_int {
    pm_runtime_force_resume((*comp).dev)
}

static rt9120_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt9120_codec_probe),
    suspend: Some(rt9120_codec_suspend),
    resume: Some(rt9120_codec_resume),
    controls: rt9120_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&rt9120_snd_controls),
    dapm_widgets: rt9120_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&rt9120_dapm_widgets),
    dapm_routes: rt9120_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&rt9120_dapm_routes),
    endianness: 1,
};

unsafe extern "C" fn rt9120_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let comp = (*dai).component;
    let format: c_uint;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => format = RT9120_CFG_FMT_I2S,
        x if x == SND_SOC_DAIFMT_LEFT_J => format = RT9120_CFG_FMT_LEFTJ,
        x if x == SND_SOC_DAIFMT_RIGHT_J => format = RT9120_CFG_FMT_RIGHTJ,
        x if x == SND_SOC_DAIFMT_DSP_A => format = RT9120_CFG_FMT_DSPA,
        x if x == SND_SOC_DAIFMT_DSP_B => format = RT9120_CFG_FMT_DSPB,
        _ => {
            dev_err((*dai).dev, b"Unknown dai format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(
        comp,
        RT9120_REG_I2SFMT,
        RT9120_I2SFMT_MASK,
        format << RT9120_I2SFMT_SHIFT,
    );
    0
}

unsafe extern "C" fn rt9120_hw_params(
    _substream: *mut snd_pcm_substream,
    param: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let comp = (*dai).component;
    let param_width: c_uint;
    let param_slot_width: c_uint;
    let auto_sync: c_uint;
    let mut width: c_int;
    let fs: c_int;

    width = params_width(param);
    match width {
        16 => param_width = RT9120_CFG_AUDBIT_16,
        20 => param_width = RT9120_CFG_AUDBIT_20,
        24 | 32 => param_width = RT9120_CFG_AUDBIT_24,
        _ => {
            dev_err(
                (*dai).dev,
                b"Unsupported data width [%d]\n\0".as_ptr() as *const c_char,
                width,
            );
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(comp, RT9120_REG_I2SFMT, RT9120_AUDBIT_MASK, param_width);

    width = params_physical_width(param);
    match width {
        16 => param_slot_width = RT9120_CFG_WORDLEN_16,
        24 => param_slot_width = RT9120_CFG_WORDLEN_24,
        32 => param_slot_width = RT9120_CFG_WORDLEN_32,
        _ => {
            dev_err(
                (*dai).dev,
                b"Unsupported slot width [%d]\n\0".as_ptr() as *const c_char,
                width,
            );
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(comp, RT9120_REG_I2SWL, RT9120_AUDWL_MASK, param_slot_width);

    fs = width * params_channels(param);
    /* If fs is divided by 48, disable auto sync */
    if fs % 48 == 0 {
        auto_sync = 0;
    } else {
        auto_sync = RT9120_AUTOSYNC_MASK;
    }

    snd_soc_component_update_bits(comp, RT9120_REG_DIGCFG, RT9120_AUTOSYNC_MASK, auto_sync);
    0
}

static rt9120_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(rt9120_set_fmt),
    hw_params: Some(rt9120_hw_params),
};

static mut rt9120_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"rt9120_aif\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"AIF Playback\0".as_ptr() as *const c_char,
        rates: RT9120_RATES_MASK,
        formats: RT9120_FMTS_MASK,
        rate_max: 192000,
        rate_min: 8000,
        channels_min: 1,
        channels_max: 2,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"AIF Capture\0".as_ptr() as *const c_char,
        rates: RT9120_RATES_MASK,
        formats: RT9120_FMTS_MASK,
        rate_max: 192000,
        rate_min: 8000,
        channels_min: 1,
        channels_max: 2,
    },
    ops: &rt9120_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

const fn regmap_reg_range(range_min: c_uint, range_max: c_uint) -> regmap_range {
    regmap_range { range_min, range_max }
}

static rt9120_rd_yes_ranges: [regmap_range; 10] = [
    regmap_reg_range(0x00, 0x0C),
    regmap_reg_range(0x10, 0x15),
    regmap_reg_range(0x20, 0x27),
    regmap_reg_range(0x30, 0x38),
    regmap_reg_range(0x3A, 0x40),
    regmap_reg_range(0x63, 0x63),
    regmap_reg_range(0x65, 0x65),
    regmap_reg_range(0x69, 0x69),
    regmap_reg_range(0x6C, 0x6C),
    regmap_reg_range(0xF8, 0xF8),
];

static rt9120_rd_table: regmap_access_table = regmap_access_table {
    yes_ranges: rt9120_rd_yes_ranges.as_ptr(),
    n_yes_ranges: ARRAY_SIZE(&rt9120_rd_yes_ranges) as c_uint,
};

static rt9120_wr_yes_ranges: [regmap_range; 12] = [
    regmap_reg_range(0x00, 0x00),
    regmap_reg_range(0x02, 0x0A),
    regmap_reg_range(0x10, 0x15),
    regmap_reg_range(0x20, 0x27),
    regmap_reg_range(0x30, 0x38),
    regmap_reg_range(0x3A, 0x3D),
    regmap_reg_range(0x40, 0x40),
    regmap_reg_range(0x63, 0x63),
    regmap_reg_range(0x65, 0x65),
    regmap_reg_range(0x69, 0x69),
    regmap_reg_range(0x6C, 0x6C),
    regmap_reg_range(0xF8, 0xF8),
];

static rt9120_wr_table: regmap_access_table = regmap_access_table {
    yes_ranges: rt9120_wr_yes_ranges.as_ptr(),
    n_yes_ranges: ARRAY_SIZE(&rt9120_wr_yes_ranges) as c_uint,
};

unsafe extern "C" fn rt9120_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00..=0x01 | 0x10 | 0x30..=0x40 => true,
        _ => false,
    }
}

fn rt9120_get_reg_size(reg: c_uint) -> c_int {
    match reg {
        0x00 | 0x20..=0x27 => 2,
        0x30..=0x3D => 3,
        0x3E..=0x3F => 4,
        _ => 1,
    }
}

unsafe extern "C" fn rt9120_reg_read(
    context: *mut c_void,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let data = context as *mut rt9120_data;
    let i2c = to_i2c_client((*data).dev);
    let size = rt9120_get_reg_size(reg);
    let mut raw: [u8; 4] = [0; 4];
    let ret: c_int;

    ret = i2c_smbus_read_i2c_block_data(i2c, reg as u8, size as u8, raw.as_mut_ptr());
    if ret < 0 {
        return ret;
    } else if ret != size {
        return -EIO;
    }

    match size {
        4 => {
            *val = u32::from_be_bytes(raw);
        }
        3 => {
            *val = ((raw[0] as c_uint) << 16) | ((raw[1] as c_uint) << 8) | raw[2] as c_uint;
        }
        2 => {
            *val = u16::from_be_bytes([raw[0], raw[1]]) as c_uint;
        }
        _ => {
            *val = raw[0] as c_uint;
        }
    }

    0
}

unsafe extern "C" fn rt9120_reg_write(
    context: *mut c_void,
    reg: c_uint,
    val: c_uint,
) -> c_int {
    let data = context as *mut rt9120_data;
    let i2c = to_i2c_client((*data).dev);
    let size = rt9120_get_reg_size(reg);
    let be32_val: __be32 = val.to_be();
    let rawp = (&be32_val as *const __be32) as *const u8;
    let offs = 4 - size;

    i2c_smbus_write_i2c_block_data(i2c, reg as u8, size as u8, rawp.offset(offs as isize))
}

static rt9120_reg_defaults: [reg_default; 28] = [
    reg_default { reg: 0x02, def: 0x02 },
    reg_default { reg: 0x03, def: 0xf2 },
    reg_default { reg: 0x04, def: 0x01 },
    reg_default { reg: 0x05, def: 0xc0 },
    reg_default { reg: 0x06, def: 0x28 },
    reg_default { reg: 0x07, def: 0x04 },
    reg_default { reg: 0x08, def: 0xff },
    reg_default { reg: 0x09, def: 0x01 },
    reg_default { reg: 0x0a, def: 0x01 },
    reg_default { reg: 0x0b, def: 0x00 },
    reg_default { reg: 0x0c, def: 0x04 },
    reg_default { reg: 0x11, def: 0x30 },
    reg_default { reg: 0x12, def: 0x08 },
    reg_default { reg: 0x13, def: 0x12 },
    reg_default { reg: 0x14, def: 0x09 },
    reg_default { reg: 0x15, def: 0x00 },
    reg_default { reg: 0x20, def: 0x7ff },
    reg_default { reg: 0x21, def: 0x180 },
    reg_default { reg: 0x22, def: 0x180 },
    reg_default { reg: 0x23, def: 0x00 },
    reg_default { reg: 0x24, def: 0x80 },
    reg_default { reg: 0x25, def: 0x180 },
    reg_default { reg: 0x26, def: 0x640 },
    reg_default { reg: 0x27, def: 0x180 },
    reg_default { reg: 0x63, def: 0x5e },
    reg_default { reg: 0x65, def: 0x66 },
    reg_default { reg: 0x6c, def: 0xe0 },
    reg_default { reg: 0xf8, def: 0x44 },
];

static rt9120_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: RT9120_REG_DIGCFG,
    reg_defaults: rt9120_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&rt9120_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,

    reg_read: Some(rt9120_reg_read),
    reg_write: Some(rt9120_reg_write),

    volatile_reg: Some(rt9120_volatile_reg),
    wr_table: &rt9120_wr_table,
    rd_table: &rt9120_rd_table,
};

unsafe fn FIELD_GET(mask: c_uint, reg: c_uint) -> c_uint {
    (reg & mask) >> mask.trailing_zeros()
}

unsafe fn rt9120_check_vendor_info(data: *mut rt9120_data) -> c_int {
    let mut devid: c_uint = 0;
    let ret: c_int;

    ret = regmap_read((*data).regmap, RT9120_REG_DEVID, &mut devid);
    if ret != 0 {
        return ret;
    }

    devid = FIELD_GET(RT9120_VID_MASK, devid);
    match devid {
        RT9120_VENDOR_ID => {
            (*data).chip_idx = CHIP_IDX_RT9120;
        }
        RT9120S_VENDOR_ID => {
            (*data).chip_idx = CHIP_IDX_RT9120S;
        }
        _ => {
            dev_err(
                (*data).dev,
                b"DEVID not correct [0x%0x]\n\0".as_ptr() as *const c_char,
                devid,
            );
            return -ENODEV;
        }
    }

    0
}

unsafe fn rt9120_do_register_reset(data: *mut rt9120_data) -> c_int {
    let ret: c_int;

    ret = regmap_write((*data).regmap, RT9120_REG_SWRESET, RT9120_SWRST_MASK);
    if ret != 0 {
        return ret;
    }

    msleep(RT9120_RESET_WAITMS);
    0
}

unsafe extern "C" fn rt9120_probe(i2c: *mut i2c_client) -> c_int {
    let data: *mut rt9120_data;
    let dvdd_supply: *mut regulator;
    let dvdd_supply_volt: c_int;
    let mut ret: c_int;

    data = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<rt9120_data>(),
        GFP_KERNEL,
    ) as *mut rt9120_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).dev = &mut (*i2c).dev;
    i2c_set_clientdata(i2c, data as *mut c_void);

    (*data).pwdnn_gpio =
        devm_gpiod_get_optional(&mut (*i2c).dev, b"pwdnn\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*data).pwdnn_gpio as *const c_void) {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to initialize 'pwdnn' gpio\n\0".as_ptr() as *const c_char,
        );
        return PTR_ERR((*data).pwdnn_gpio as *const c_void);
    } else if !(*data).pwdnn_gpio.is_null() {
        dev_dbg(
            &mut (*i2c).dev,
            b"'pwdnn' from low to high, wait chip on\n\0".as_ptr() as *const c_char,
        );
        msleep(RT9120_CHIPON_WAITMS);
    }

    (*data).regmap = devm_regmap_init(
        &mut (*i2c).dev,
        ptr::null(),
        data as *mut c_void,
        &rt9120_regmap_config,
    );
    if IS_ERR((*data).regmap as *const c_void) {
        ret = PTR_ERR((*data).regmap as *const c_void);
        dev_err(
            &mut (*i2c).dev,
            b"Failed to init regmap [%d]\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = rt9120_check_vendor_info(data);
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to check vendor info\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ret = rt9120_do_register_reset(data);
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to do register reset\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    dvdd_supply = devm_regulator_get(&mut (*i2c).dev, b"dvdd\0".as_ptr() as *const c_char);
    if IS_ERR(dvdd_supply as *const c_void) {
        dev_err(
            &mut (*i2c).dev,
            b"No dvdd regulator found\n\0".as_ptr() as *const c_char,
        );
        return PTR_ERR(dvdd_supply as *const c_void);
    }

    dvdd_supply_volt = regulator_get_voltage(dvdd_supply);
    if dvdd_supply_volt <= RT9120_LVAPP_THRESUV {
        dev_dbg(
            &mut (*i2c).dev,
            b"dvdd low voltage design\n\0".as_ptr() as *const c_char,
        );
        ret = regmap_update_bits((*data).regmap, RT9120_REG_UVPOPT, RT9120_DVDD_UVSEL_MASK, 0);
        if ret != 0 {
            dev_err(
                &mut (*i2c).dev,
                b"Failed to config dvdd uvsel\n\0".as_ptr() as *const c_char,
            );
            return ret;
        }
    }

    pm_runtime_set_autosuspend_delay(&mut (*i2c).dev, 1000);
    pm_runtime_use_autosuspend(&mut (*i2c).dev);
    pm_runtime_set_active(&mut (*i2c).dev);
    pm_runtime_mark_last_busy(&mut (*i2c).dev);
    pm_runtime_enable(&mut (*i2c).dev);

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &rt9120_component_driver,
        &mut rt9120_dai,
        1,
    )
}

unsafe extern "C" fn rt9120_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(&mut (*i2c).dev);
    pm_runtime_set_suspended(&mut (*i2c).dev);
}

unsafe extern "C" fn rt9120_runtime_suspend(dev: *mut device) -> c_int {
    let data = dev_get_drvdata(dev) as *mut rt9120_data;

    if !(*data).pwdnn_gpio.is_null() {
        regcache_cache_only((*data).regmap, true);
        regcache_mark_dirty((*data).regmap);
        gpiod_set_value((*data).pwdnn_gpio, 0);
    }

    0
}

unsafe extern "C" fn rt9120_runtime_resume(dev: *mut device) -> c_int {
    let data = dev_get_drvdata(dev) as *mut rt9120_data;
    let ret: c_int;

    if !(*data).pwdnn_gpio.is_null() {
        gpiod_set_value((*data).pwdnn_gpio, 1);
        msleep(RT9120_CHIPON_WAITMS);
        regcache_cache_only((*data).regmap, false);
        ret = regcache_sync((*data).regmap);
        if ret != 0 {
            regcache_cache_only((*data).regmap, true);
            regcache_mark_dirty((*data).regmap);
            gpiod_set_value((*data).pwdnn_gpio, 0);
            return ret;
        }
    }

    0
}

// RUNTIME_PM_OPS(rt9120_runtime_suspend, rt9120_runtime_resume, NULL)
static rt9120_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(rt9120_runtime_suspend),
    runtime_resume: Some(rt9120_runtime_resume),
};

static rt9120_device_table: [of_device_id; 2] = [
    of_device_id {
        compatible: b"richtek,rt9120\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, rt9120_device_table);

static mut rt9120_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rt9120\0".as_ptr() as *const c_char,
        of_match_table: rt9120_device_table.as_ptr(),
        pm: &rt9120_pm_ops,
    },
    probe: Some(rt9120_probe),
    remove: Some(rt9120_remove),
};
// module_i2c_driver(rt9120_driver);

// MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
// MODULE_DESCRIPTION("RT9120 Audio Amplifier Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
