// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8523.rs  --  WM8523 ALSA SoC Audio driver
 *
 * Copyright 2009 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* C include dependencies translated as external declarations:
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
 * linux/pm.h, linux/i2c.h, linux/regmap.h, linux/regulator/consumer.h,
 * linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
 * sound/soc.h, sound/initval.h, sound/tlv.h, and "wm8523.h".
 */

type bool_ = bool;
type u16 = u16;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_uint,
    pub mask: c_uint,
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
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

extern "C" {
    static WM8523_DEVICE_ID: c_uint;
    static WM8523_REVISION: c_uint;
    static WM8523_ZERO_DETECT: c_uint;
    static WM8523_AIF_CTRL1: c_uint;
    static WM8523_AIF_CTRL2: c_uint;
    static WM8523_DAC_CTRL3: c_uint;
    static WM8523_DAC_GAINL: c_uint;
    static WM8523_DAC_GAINR: c_uint;
    static WM8523_PSCTRL1: c_uint;
    static WM8523_SR_MASK: u16;
    static WM8523_AIF_MSTR: u16;
    static WM8523_BCLKDIV_MASK: u16;
    static WM8523_BCLKDIV_SHIFT: c_uint;
    static WM8523_WL_MASK: u16;
    static WM8523_BCLK_INV_MASK: u16;
    static WM8523_LRCLK_INV_MASK: u16;
    static WM8523_FMT_MASK: u16;
    static WM8523_AIF_MSTR_MASK: u16;
    static WM8523_BCLK_INV: u16;
    static WM8523_LRCLK_INV: u16;
    static WM8523_SYS_ENA_MASK: c_uint;
    static WM8523_DACR_VU: c_uint;
    static WM8523_ZC: c_uint;
    static WM8523_CHIP_REV_MASK: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_NOPM: c_int;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_uint,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;

    fn SOC_DOUBLE_R_TLV(
        name: *const c_char,
        reg_left: c_uint,
        reg_right: c_uint,
        xshift: c_uint,
        xmax: c_uint,
        xinvert: c_uint,
        tlv_array: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DOUBLE(
        name: *const c_char,
        reg: c_uint,
        shift_left: c_uint,
        shift_right: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_ENUM(name: *const c_char, e: snd_soc_enum) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_DAC(
        wname: *const c_char,
        stname: *const c_char,
        wreg: c_int,
        wshift: c_uint,
        winvert: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(wname: *const c_char) -> snd_soc_dapm_widget;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

const WM8523_NUM_SUPPLIES: usize = 2;
static wm8523_supply_names: [*const c_char; WM8523_NUM_SUPPLIES] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"LINEVDD\0".as_ptr() as *const c_char,
];

const WM8523_NUM_RATES: usize = 7;

/* codec private data */
#[repr(C)]
pub struct wm8523_priv {
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; WM8523_NUM_SUPPLIES],
    pub sysclk: c_uint,
    pub rate_constraint_list: [c_uint; WM8523_NUM_RATES],
    pub rate_constraint: snd_pcm_hw_constraint_list,
}

static wm8523_reg_defaults: [reg_default; 7] = [
    reg_default { reg: 2, def: 0x0000 },     /* R2 - PSCTRL1 */
    reg_default { reg: 3, def: 0x1812 },     /* R3 - AIF_CTRL1 */
    reg_default { reg: 4, def: 0x0000 },     /* R4 - AIF_CTRL2 */
    reg_default { reg: 5, def: 0x0001 },     /* R5 - DAC_CTRL3 */
    reg_default { reg: 6, def: 0x0190 },     /* R6 - DAC_GAINL */
    reg_default { reg: 7, def: 0x0190 },     /* R7 - DAC_GAINR */
    reg_default { reg: 8, def: 0x0000 },     /* R8 - ZERO_DETECT */
];

unsafe extern "C" fn wm8523_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        x if x == WM8523_DEVICE_ID => true,
        x if x == WM8523_REVISION => true,
        _ => false,
    }
}

/* static const DECLARE_TLV_DB_SCALE(dac_tlv, -10000, 25, 0); */
static dac_tlv: [c_uint; 4] = [0, (-10000i32) as c_uint, 25, 0];

static wm8523_zd_count_text: [*const c_char; 2] = [
    b"1024\0".as_ptr() as *const c_char,
    b"2048\0".as_ptr() as *const c_char,
];

/* static SOC_ENUM_SINGLE_DECL(wm8523_zc_count, WM8523_ZERO_DETECT, 0,
 *                             wm8523_zd_count_text);
 */
static wm8523_zc_count: snd_soc_enum = snd_soc_enum {
    reg: unsafe { WM8523_ZERO_DETECT },
    shift_l: 0,
    items: wm8523_zd_count_text.len() as c_uint,
    texts: wm8523_zd_count_text.as_ptr(),
};

static wm8523_controls: [snd_kcontrol_new; 7] = unsafe {
    [
        SOC_DOUBLE_R_TLV(
            b"Playback Volume\0".as_ptr() as *const c_char,
            WM8523_DAC_GAINL,
            WM8523_DAC_GAINR,
            0,
            448,
            0,
            dac_tlv.as_ptr(),
        ),
        SOC_SINGLE(
            b"ZC Switch\0".as_ptr() as *const c_char,
            WM8523_DAC_CTRL3,
            4,
            1,
            0,
        ),
        SOC_SINGLE(
            b"Playback Deemphasis Switch\0".as_ptr() as *const c_char,
            WM8523_AIF_CTRL1,
            8,
            1,
            0,
        ),
        SOC_DOUBLE(
            b"Playback Switch\0".as_ptr() as *const c_char,
            WM8523_DAC_CTRL3,
            2,
            3,
            1,
            1,
        ),
        SOC_SINGLE(
            b"Volume Ramp Up Switch\0".as_ptr() as *const c_char,
            WM8523_DAC_CTRL3,
            1,
            1,
            0,
        ),
        SOC_SINGLE(
            b"Volume Ramp Down Switch\0".as_ptr() as *const c_char,
            WM8523_DAC_CTRL3,
            0,
            1,
            0,
        ),
        SOC_ENUM(
            b"Zero Detect Count\0".as_ptr() as *const c_char,
            wm8523_zc_count,
        ),
    ]
};

static wm8523_dapm_widgets: [snd_soc_dapm_widget; 3] = unsafe {
    [
        SND_SOC_DAPM_DAC(
            b"DAC\0".as_ptr() as *const c_char,
            b"Playback\0".as_ptr() as *const c_char,
            SND_SOC_NOPM,
            0,
            0,
        ),
        SND_SOC_DAPM_OUTPUT(b"LINEVOUTL\0".as_ptr() as *const c_char),
        SND_SOC_DAPM_OUTPUT(b"LINEVOUTR\0".as_ptr() as *const c_char),
    ]
};

static wm8523_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"LINEVOUTL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"LINEVOUTR\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

#[repr(C)]
#[derive(Copy, Clone)]
struct ratio_entry {
    value: c_int,
    ratio: c_int,
}

static lrclk_ratios: [ratio_entry; WM8523_NUM_RATES] = [
    ratio_entry { value: 1, ratio: 128 },
    ratio_entry { value: 2, ratio: 192 },
    ratio_entry { value: 3, ratio: 256 },
    ratio_entry { value: 4, ratio: 384 },
    ratio_entry { value: 5, ratio: 512 },
    ratio_entry { value: 6, ratio: 768 },
    ratio_entry { value: 7, ratio: 1152 },
];

static bclk_ratios: [ratio_entry; 3] = [
    ratio_entry { value: 2, ratio: 32 },
    ratio_entry { value: 3, ratio: 64 },
    ratio_entry { value: 4, ratio: 128 },
];

unsafe extern "C" fn wm8523_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8523 = snd_soc_component_get_drvdata(component) as *mut wm8523_priv;

    /* The set of sample rates that can be supported depends on the
     * MCLK supplied to the CODEC - enforce this.
     */
    if (*wm8523).sysclk == 0 {
        dev_err(
            (*component).dev,
            b"No MCLK configured, call set_sysclk() on init\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*wm8523).rate_constraint,
    );

    0
}

unsafe extern "C" fn wm8523_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8523 = snd_soc_component_get_drvdata(component) as *mut wm8523_priv;
    let mut i: usize;
    let mut aifctrl1: u16 = snd_soc_component_read(component, WM8523_AIF_CTRL1) as u16;
    let mut aifctrl2: u16 = snd_soc_component_read(component, WM8523_AIF_CTRL2) as u16;

    /* Find a supported LRCLK ratio */
    i = 0;
    while i < lrclk_ratios.len() {
        if (*wm8523).sysclk / params_rate(params) == lrclk_ratios[i].ratio as c_uint {
            break;
        }
        i += 1;
    }

    /* Should never happen, should be handled by constraints */
    if i == lrclk_ratios.len() {
        dev_err(
            (*component).dev,
            b"MCLK/fs ratio %d unsupported\n\0".as_ptr() as *const c_char,
            (*wm8523).sysclk / params_rate(params),
        );
        return -EINVAL;
    }

    aifctrl2 &= !WM8523_SR_MASK;
    aifctrl2 |= lrclk_ratios[i].value as u16;

    if (aifctrl1 & WM8523_AIF_MSTR) != 0 {
        /* Find a fs->bclk ratio */
        i = 0;
        while i < bclk_ratios.len() {
            if params_width(params) * 2 <= bclk_ratios[i].ratio {
                break;
            }
            i += 1;
        }

        if i == bclk_ratios.len() {
            dev_err(
                (*component).dev,
                b"No matching BCLK/fs ratio for word length %d\n\0".as_ptr() as *const c_char,
                params_width(params),
            );
            return -EINVAL;
        }

        aifctrl2 &= !WM8523_BCLKDIV_MASK;
        aifctrl2 |= (bclk_ratios[i].value as u16) << WM8523_BCLKDIV_SHIFT;
    }

    aifctrl1 &= !WM8523_WL_MASK;
    match params_width(params) {
        16 => {}
        20 => {
            aifctrl1 |= 0x8;
        }
        24 => {
            aifctrl1 |= 0x10;
        }
        32 => {
            aifctrl1 |= 0x18;
        }
        _ => {}
    }

    snd_soc_component_write(component, WM8523_AIF_CTRL1, aifctrl1 as c_uint);
    snd_soc_component_write(component, WM8523_AIF_CTRL2, aifctrl2 as c_uint);

    0
}

unsafe extern "C" fn wm8523_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8523 = snd_soc_component_get_drvdata(component) as *mut wm8523_priv;
    let mut val: c_uint;
    let mut i: usize;

    (*wm8523).sysclk = freq;

    (*wm8523).rate_constraint.count = 0;
    i = 0;
    while i < lrclk_ratios.len() {
        val = freq / lrclk_ratios[i].ratio as c_uint;
        /* Check that it's a standard rate since core can't
         * cope with others and having the odd rates confuses
         * constraint matching.
         */
        match val {
            8000 | 11025 | 16000 | 22050 | 32000 | 44100 | 48000 | 64000 | 88200 | 96000
            | 176400 | 192000 => {
                dev_dbg(
                    (*component).dev,
                    b"Supported sample rate: %dHz\n\0".as_ptr() as *const c_char,
                    val,
                );
                (*wm8523).rate_constraint_list[i] = val;
                (*wm8523).rate_constraint.count += 1;
            }
            _ => {
                dev_dbg(
                    (*component).dev,
                    b"Skipping sample rate: %dHz\n\0".as_ptr() as *const c_char,
                    val,
                );
            }
        }
        i += 1;
    }

    /* Need at least one supported rate... */
    if (*wm8523).rate_constraint.count == 0 {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn wm8523_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut aifctrl1: u16 = snd_soc_component_read(component, WM8523_AIF_CTRL1) as u16;

    aifctrl1 &= !(WM8523_BCLK_INV_MASK
        | WM8523_LRCLK_INV_MASK
        | WM8523_FMT_MASK
        | WM8523_AIF_MSTR_MASK);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            aifctrl1 |= WM8523_AIF_MSTR;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            aifctrl1 |= 0x0002;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            aifctrl1 |= 0x0001;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            aifctrl1 |= 0x0003;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            aifctrl1 |= 0x0023;
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => {
            aifctrl1 |= WM8523_BCLK_INV | WM8523_LRCLK_INV;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            aifctrl1 |= WM8523_BCLK_INV;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            aifctrl1 |= WM8523_LRCLK_INV;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_write(component, WM8523_AIF_CTRL1, aifctrl1 as c_uint);

    0
}

unsafe extern "C" fn wm8523_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8523 = snd_soc_component_get_drvdata(component) as *mut wm8523_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}

        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            /* Full power on */
            snd_soc_component_update_bits(component, WM8523_PSCTRL1, WM8523_SYS_ENA_MASK, 3);
        }

        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if matches!(
                snd_soc_dapm_get_bias_level(dapm),
                snd_soc_bias_level::SND_SOC_BIAS_OFF
            ) {
                ret = regulator_bulk_enable(
                    (*wm8523).supplies.len() as c_uint,
                    (*wm8523).supplies.as_mut_ptr(),
                );
                if ret != 0 {
                    dev_err(
                        (*component).dev,
                        b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char,
                        ret,
                    );
                    return ret;
                }

                /* Sync back default/cached values */
                regcache_sync((*wm8523).regmap);

                /* Initial power up */
                snd_soc_component_update_bits(component, WM8523_PSCTRL1, WM8523_SYS_ENA_MASK, 1);

                msleep(100);
            }

            /* Power up to mute */
            snd_soc_component_update_bits(component, WM8523_PSCTRL1, WM8523_SYS_ENA_MASK, 2);
        }

        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* The chip runs through the power down sequence for us. */
            snd_soc_component_update_bits(component, WM8523_PSCTRL1, WM8523_SYS_ENA_MASK, 0);
            msleep(100);

            regulator_bulk_disable(
                (*wm8523).supplies.len() as c_uint,
                (*wm8523).supplies.as_mut_ptr(),
            );
        }
    }
    0
}

static WM8523_RATES: c_uint = unsafe { SNDRV_PCM_RATE_8000_192000 };

static WM8523_FORMATS: c_uint = unsafe {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE
};

static wm8523_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8523_startup),
    hw_params: Some(wm8523_hw_params),
    set_sysclk: Some(wm8523_set_dai_sysclk),
    set_fmt: Some(wm8523_set_dai_fmt),
};

static mut wm8523_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8523-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,  /* Mono modes not yet supported */
        channels_max: 2,
        rates: unsafe { WM8523_RATES },
        formats: unsafe { WM8523_FORMATS },
    },
    ops: &wm8523_dai_ops,
};

unsafe extern "C" fn wm8523_probe(component: *mut snd_soc_component) -> c_int {
    let wm8523 = snd_soc_component_get_drvdata(component) as *mut wm8523_priv;

    (*wm8523).rate_constraint.list = (*wm8523).rate_constraint_list.as_mut_ptr();
    (*wm8523).rate_constraint.count = (*wm8523).rate_constraint_list.len() as c_uint;

    /* Change some default settings - latch VU and enable ZC */
    snd_soc_component_update_bits(component, WM8523_DAC_GAINR, WM8523_DACR_VU, WM8523_DACR_VU);
    snd_soc_component_update_bits(component, WM8523_DAC_CTRL3, WM8523_ZC, WM8523_ZC);

    0
}

static soc_component_dev_wm8523: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8523_probe),
    set_bias_level: Some(wm8523_set_bias_level),
    controls: wm8523_controls.as_ptr(),
    num_controls: wm8523_controls.len() as c_uint,
    dapm_widgets: wm8523_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8523_dapm_widgets.len() as c_uint,
    dapm_routes: wm8523_dapm_routes.as_ptr(),
    num_dapm_routes: wm8523_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8523_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"wlf,wm8523\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, wm8523_of_match); */

static wm8523_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: unsafe { WM8523_ZERO_DETECT },

    reg_defaults: wm8523_reg_defaults.as_ptr(),
    num_reg_defaults: wm8523_reg_defaults.len() as c_uint,
    cache_type: unsafe { REGCACHE_MAPLE },

    volatile_reg: Some(wm8523_volatile_register),
};

unsafe extern "C" fn wm8523_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut wm8523: *mut wm8523_priv;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    let mut i: usize;

    wm8523 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<wm8523_priv>(),
        GFP_KERNEL,
    ) as *mut wm8523_priv;
    if wm8523.is_null() {
        return -ENOMEM;
    }

    (*wm8523).regmap = devm_regmap_init_i2c(i2c, &wm8523_regmap);
    if IS_ERR((*wm8523).regmap as *const c_void) {
        ret = PTR_ERR((*wm8523).regmap as *const c_void);
        dev_err(
            &mut (*i2c).dev,
            b"Failed to create regmap: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    i = 0;
    while i < (*wm8523).supplies.len() {
        (*wm8523).supplies[i].supply = wm8523_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*i2c).dev,
        (*wm8523).supplies.len() as c_uint,
        (*wm8523).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = regulator_bulk_enable(
        (*wm8523).supplies.len() as c_uint,
        (*wm8523).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = regmap_read((*wm8523).regmap, WM8523_DEVICE_ID, &mut val);
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to read ID register\n\0".as_ptr() as *const c_char,
        );
        goto_err_enable(wm8523);
        return ret;
    }
    if val != 0x8523 {
        dev_err(
            &mut (*i2c).dev,
            b"Device is not a WM8523, ID is %x\n\0".as_ptr() as *const c_char,
            ret,
        );
        ret = -EINVAL;
        goto_err_enable(wm8523);
        return ret;
    }

    ret = regmap_read((*wm8523).regmap, WM8523_REVISION, &mut val);
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to read revision register\n\0".as_ptr() as *const c_char,
        );
        goto_err_enable(wm8523);
        return ret;
    }
    dev_info(
        &mut (*i2c).dev,
        b"revision %c\n\0".as_ptr() as *const c_char,
        (val & WM8523_CHIP_REV_MASK) + b'A' as c_uint,
    );

    ret = regmap_write((*wm8523).regmap, WM8523_DEVICE_ID, 0x8523);
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to reset device: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        goto_err_enable(wm8523);
        return ret;
    }

    regulator_bulk_disable(
        (*wm8523).supplies.len() as c_uint,
        (*wm8523).supplies.as_mut_ptr(),
    );

    i2c_set_clientdata(i2c, wm8523 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8523,
        &mut wm8523_dai,
        1,
    );

    ret
}

unsafe fn goto_err_enable(wm8523: *mut wm8523_priv) {
    regulator_bulk_disable(
        (*wm8523).supplies.len() as c_uint,
        (*wm8523).supplies.as_mut_ptr(),
    );
}

static wm8523_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"wm8523\0".as_ptr() as *const c_char,
    },
    i2c_device_id {
        name: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(i2c, wm8523_i2c_id); */

static mut wm8523_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"wm8523\0".as_ptr() as *const c_char,
        of_match_table: wm8523_of_match.as_ptr(),
    },
    probe: Some(wm8523_i2c_probe),
    id_table: wm8523_i2c_id.as_ptr(),
};

/* module_i2c_driver(wm8523_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC WM8523 driver"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
