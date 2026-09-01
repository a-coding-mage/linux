// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC Texas Instruments TAS6424 Quad-Channel Audio Amplifier
 *
 * Copyright (C) 2016-2017 Texas Instruments Incorporated - https://www.ti.com/
 *	Author: Andreas Dannenberg <dannenberg@ti.com>
 *	Andrew F. Davis <afd@ti.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

// Dependencies supplied by Linux/ALSA headers in the original C source:
// linux/module.h, linux/errno.h, linux/device.h, linux/i2c.h, linux/regmap.h,
// linux/slab.h, linux/regulator/consumer.h, linux/delay.h,
// linux/gpio/consumer.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-dapm.h, sound/tlv.h, and "tas6424.h".

type u8 = u8;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_layout {
    _private: [u8; 0],
}
type snd_soc_dapm_widget_item = snd_soc_dapm_widget_layout;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
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
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

type snd_soc_bias_level = c_uint;

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn msleep(msecs: c_uint);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn __ffs(word: c_uint) -> c_uint;
    fn __fls(word: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

type c_long = i64;

unsafe extern "C" {
    static TAS6424_CH1_VOL_CTRL: c_uint;
    static TAS6424_CH2_VOL_CTRL: c_uint;
    static TAS6424_CH3_VOL_CTRL: c_uint;
    static TAS6424_CH4_VOL_CTRL: c_uint;
    static TAS6424_DC_DIAG_CTRL1: c_uint;
    static TAS6424_LDGBYPASS_SHIFT: c_uint;
    static TAS6424_SAP_RATE_44100: u8;
    static TAS6424_SAP_RATE_48000: u8;
    static TAS6424_SAP_RATE_96000: u8;
    static TAS6424_SAP_TDM_SLOT_SZ_16: u8;
    static TAS6424_SAP_CTRL: c_uint;
    static TAS6424_SAP_RATE_MASK: c_uint;
    static TAS6424_SAP_FMT_MASK: c_uint;
    static TAS6424_SAP_I2S: u8;
    static TAS6424_SAP_DSP: u8;
    static TAS6424_SAP_LEFTJ: u8;
    static TAS6424_SAP_TDM_SLOT_LAST: c_uint;
    static TAS6424_ALL_STATE_MUTE: c_uint;
    static TAS6424_ALL_STATE_PLAY: c_uint;
    static TAS6424_CH_STATE_CTRL: c_uint;
    static TAS6424_ALL_STATE_HIZ: c_uint;
    static TAS6424_LDGBYPASS_MASK: c_uint;
    static TAS6424_CHANNEL_FAULT: c_uint;
    static TAS6424_FAULT_OC_CH1: c_uint;
    static TAS6424_FAULT_OC_CH2: c_uint;
    static TAS6424_FAULT_OC_CH3: c_uint;
    static TAS6424_FAULT_OC_CH4: c_uint;
    static TAS6424_FAULT_DC_CH1: c_uint;
    static TAS6424_FAULT_DC_CH2: c_uint;
    static TAS6424_FAULT_DC_CH3: c_uint;
    static TAS6424_FAULT_DC_CH4: c_uint;
    static TAS6424_GLOB_FAULT1: c_uint;
    static TAS6424_FAULT_PVDD_OV: c_uint;
    static TAS6424_FAULT_VBAT_OV: c_uint;
    static TAS6424_FAULT_PVDD_UV: c_uint;
    static TAS6424_FAULT_VBAT_UV: c_uint;
    static TAS6424_GLOB_FAULT2: c_uint;
    static TAS6424_FAULT_OTSD: c_uint;
    static TAS6424_FAULT_OTSD_CH1: c_uint;
    static TAS6424_FAULT_OTSD_CH2: c_uint;
    static TAS6424_FAULT_OTSD_CH3: c_uint;
    static TAS6424_FAULT_OTSD_CH4: c_uint;
    static TAS6424_WARN: c_uint;
    static TAS6424_WARN_VDD_UV: c_uint;
    static TAS6424_WARN_VDD_POR: c_uint;
    static TAS6424_WARN_VDD_OTW: c_uint;
    static TAS6424_WARN_VDD_OTW_CH1: c_uint;
    static TAS6424_WARN_VDD_OTW_CH2: c_uint;
    static TAS6424_WARN_VDD_OTW_CH3: c_uint;
    static TAS6424_WARN_VDD_OTW_CH4: c_uint;
    static TAS6424_MISC_CTRL3: c_uint;
    static TAS6424_CLEAR_FAULT: c_uint;
    static TAS6424_MODE_CTRL: c_uint;
    static TAS6424_MISC_CTRL1: c_uint;
    static TAS6424_MISC_CTRL2: c_uint;
    static TAS6424_DC_DIAG_CTRL2: c_uint;
    static TAS6424_DC_DIAG_CTRL3: c_uint;
    static TAS6424_PIN_CTRL: c_uint;
    static TAS6424_AC_DIAG_CTRL1: c_uint;
    static TAS6424_CLIP_CTRL: c_uint;
    static TAS6424_CLIP_WINDOW: c_uint;
    static TAS6424_CLIP_WARN: c_uint;
    static TAS6424_CBC_STAT: c_uint;
    static TAS6424_MISC_CTRL4: c_uint;
    static TAS6424_DC_LOAD_DIAG_REP12: c_uint;
    static TAS6424_DC_LOAD_DIAG_REP34: c_uint;
    static TAS6424_DC_LOAD_DIAG_REPLO: c_uint;
    static TAS6424_CHANNEL_STATE: c_uint;
    static TAS6424_AC_LOAD_DIAG_REP1: c_uint;
    static TAS6424_AC_LOAD_DIAG_REP2: c_uint;
    static TAS6424_AC_LOAD_DIAG_REP3: c_uint;
    static TAS6424_AC_LOAD_DIAG_REP4: c_uint;
    static TAS6424_MAX: c_uint;
    static TAS6424_RATES: c_uint;
    static TAS6424_FORMATS: c_ulong;
    static TAS6424_RESET: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static REGCACHE_RBTREE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static GPIOD_OUT_HIGH: c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;

/* Define how often to check (and clear) the fault status register (in ms) */
const TAS6424_FAULT_CHECK_INTERVAL: c_uint = 200;

static tas6424_supply_names: [*const c_char; 3] = [
    b"dvdd\0".as_ptr() as *const c_char, /* Digital power supply. Connect to 3.3-V supply. */
    b"vbat\0".as_ptr() as *const c_char, /* Supply used for higher voltage analog circuits. */
    b"pvdd\0".as_ptr() as *const c_char, /* Class-D amp output FETs supply. */
];
const TAS6424_NUM_SUPPLIES: usize = tas6424_supply_names.len();

#[repr(C)]
struct tas6424_data {
    dev: *mut device,
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; TAS6424_NUM_SUPPLIES],
    fault_check_work: delayed_work,
    last_cfault: c_uint,
    last_fault1: c_uint,
    last_fault2: c_uint,
    last_warn: c_uint,
    standby_gpio: *mut gpio_desc,
    mute_gpio: *mut gpio_desc,
}

/*
 * DAC digital volumes. From -103.5 to 24 dB in 0.5 dB steps. Note that
 * setting the gain below -100 dB (register value <0x7) is effectively a MUTE
 * as per device datasheet.
 */
// static DECLARE_TLV_DB_SCALE(dac_tlv, -10350, 50, 0);
static dac_tlv: [c_uint; 4] = [0, (-10350i32) as c_uint, 50, 0];

// The C array below is initialized with ALSA macro constructors:
// SOC_SINGLE_TLV(..., dac_tlv) for four channel volumes and
// SOC_SINGLE_STROBE("Auto Diagnostics Switch", TAS6424_DC_DIAG_CTRL1,
// TAS6424_LDGBYPASS_SHIFT, 1).
static tas6424_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn tas6424_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas6424 = snd_soc_component_get_drvdata(component) as *mut tas6424_data;

    dev_dbg((*component).dev, b"%s() event=0x%0x\n\0".as_ptr() as *const c_char, b"tas6424_dac_event\0".as_ptr(), event);

    if event & SND_SOC_DAPM_POST_PMU != 0 {
        /* Observe codec shutdown-to-active time */
        msleep(12);

        /* Turn on TAS6424 periodic fault checking/handling */
        (*tas6424).last_fault1 = 0;
        (*tas6424).last_fault2 = 0;
        (*tas6424).last_warn = 0;
        schedule_delayed_work(
            &mut (*tas6424).fault_check_work,
            msecs_to_jiffies(TAS6424_FAULT_CHECK_INTERVAL),
        );
    } else if event & SND_SOC_DAPM_PRE_PMD != 0 {
        /* Disable TAS6424 periodic fault checking/handling */
        cancel_delayed_work_sync(&mut (*tas6424).fault_check_work);
    }

    0
}

// SND_SOC_DAPM_AIF_IN, SND_SOC_DAPM_DAC_E, and SND_SOC_DAPM_OUTPUT expand to
// snd_soc_dapm_widget initializers in C.
static tas6424_dapm_widgets: [snd_soc_dapm_widget_item; 0] = [];

static tas6424_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"DAC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC IN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn tas6424_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rate = params_rate(params);
    let width = params_width(params);
    let mut sap_ctrl: u8 = 0;

    dev_dbg((*component).dev, b"%s() rate=%u width=%u\n\0".as_ptr() as *const c_char, b"tas6424_hw_params\0".as_ptr(), rate, width);

    match rate {
        44100 => sap_ctrl |= TAS6424_SAP_RATE_44100,
        48000 => sap_ctrl |= TAS6424_SAP_RATE_48000,
        96000 => sap_ctrl |= TAS6424_SAP_RATE_96000,
        _ => {
            dev_err((*component).dev, b"unsupported sample rate: %u\n\0".as_ptr() as *const c_char, rate);
            return -EINVAL;
        }
    }

    match width {
        16 => sap_ctrl |= TAS6424_SAP_TDM_SLOT_SZ_16,
        24 => {}
        _ => {
            dev_err((*component).dev, b"unsupported sample width: %u\n\0".as_ptr() as *const c_char, width);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(
        component,
        TAS6424_SAP_CTRL,
        TAS6424_SAP_RATE_MASK | TAS6424_SAP_TDM_SLOT_SZ_16 as c_uint,
        sap_ctrl as c_uint,
    );

    0
}

unsafe extern "C" fn tas6424_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut serial_format: u8 = 0;

    dev_dbg((*component).dev, b"%s() fmt=0x%0x\n\0".as_ptr() as *const c_char, b"tas6424_set_dai_fmt\0".as_ptr(), fmt);

    /* clock masters */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            dev_err((*component).dev, b"Invalid DAI clocking\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* signal polarity */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        _ => {
            dev_err((*component).dev, b"Invalid DAI clock signal polarity\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => serial_format |= TAS6424_SAP_I2S,
        x if x == SND_SOC_DAIFMT_DSP_A => serial_format |= TAS6424_SAP_DSP,
        x if x == SND_SOC_DAIFMT_DSP_B => {
            /*
             * We can use the fact that the TAS6424 does not care about the
             * LRCLK duty cycle during TDM to receive DSP_B formatted data
             * in LEFTJ mode (no delaying of the 1st data bit).
             */
            serial_format |= TAS6424_SAP_LEFTJ;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => serial_format |= TAS6424_SAP_LEFTJ,
        _ => {
            dev_err((*component).dev, b"Invalid DAI interface format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(
        component,
        TAS6424_SAP_CTRL,
        TAS6424_SAP_FMT_MASK,
        serial_format as c_uint,
    );

    0
}

unsafe extern "C" fn tas6424_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    _slots: c_int,
    _slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let first_slot: c_uint;
    let last_slot: c_uint;
    let sap_tdm_slot_last: bool;

    dev_dbg((*component).dev, b"%s() tx_mask=%d rx_mask=%d\n\0".as_ptr() as *const c_char, b"tas6424_set_dai_tdm_slot\0".as_ptr(), tx_mask, rx_mask);

    if tx_mask == 0 || rx_mask == 0 {
        return 0; /* nothing needed to disable TDM mode */
    }

    /*
     * Determine the first slot and last slot that is being requested so
     * we'll be able to more easily enforce certain constraints as the
     * TAS6424's TDM interface is not fully configurable.
     */
    first_slot = __ffs(tx_mask);
    last_slot = __fls(rx_mask);

    if last_slot.wrapping_sub(first_slot) != 4 {
        dev_err((*component).dev, b"tdm mask must cover 4 contiguous slots\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    match first_slot {
        0 => sap_tdm_slot_last = false,
        4 => sap_tdm_slot_last = true,
        _ => {
            dev_err((*component).dev, b"tdm mask must start at slot 0 or 4\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(
        component,
        TAS6424_SAP_CTRL,
        TAS6424_SAP_TDM_SLOT_LAST,
        if sap_tdm_slot_last { TAS6424_SAP_TDM_SLOT_LAST } else { 0 },
    );

    0
}

unsafe extern "C" fn tas6424_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas6424 = snd_soc_component_get_drvdata(component) as *mut tas6424_data;
    let val: c_uint;

    dev_dbg((*component).dev, b"%s() mute=%d\n\0".as_ptr() as *const c_char, b"tas6424_mute\0".as_ptr(), mute);

    if !(*tas6424).mute_gpio.is_null() {
        gpiod_set_value_cansleep((*tas6424).mute_gpio, mute);
        return 0;
    }

    if mute != 0 {
        val = TAS6424_ALL_STATE_MUTE;
    } else {
        val = TAS6424_ALL_STATE_PLAY;
    }

    snd_soc_component_write(component, TAS6424_CH_STATE_CTRL, val);

    0
}

unsafe extern "C" fn tas6424_power_off(component: *mut snd_soc_component) -> c_int {
    let tas6424 = snd_soc_component_get_drvdata(component) as *mut tas6424_data;
    let ret: c_int;

    snd_soc_component_write(component, TAS6424_CH_STATE_CTRL, TAS6424_ALL_STATE_HIZ);

    regcache_cache_only((*tas6424).regmap, true);
    regcache_mark_dirty((*tas6424).regmap);

    ret = regulator_bulk_disable(TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, b"failed to disable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tas6424_power_on(component: *mut snd_soc_component) -> c_int {
    let tas6424 = snd_soc_component_get_drvdata(component) as *mut tas6424_data;
    let mut ret: c_int;
    let chan_states: u8;
    let mut no_auto_diags: c_int = 0;
    let mut reg_val: c_uint = 0;

    if regmap_read((*tas6424).regmap, TAS6424_DC_DIAG_CTRL1, &mut reg_val) == 0 {
        no_auto_diags = (reg_val & TAS6424_LDGBYPASS_MASK) as c_int;
    }

    ret = regulator_bulk_enable(TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, b"failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regcache_cache_only((*tas6424).regmap, false);

    ret = regcache_sync((*tas6424).regmap);
    if ret < 0 {
        dev_err((*component).dev, b"failed to sync regcache: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    if !(*tas6424).mute_gpio.is_null() {
        gpiod_set_value_cansleep((*tas6424).mute_gpio, 0);
        /*
         * channels are muted via the mute pin.  Don't also mute
         * them via the registers so that subsequent register
         * access is not necessary to un-mute the channels
         */
        chan_states = TAS6424_ALL_STATE_PLAY as u8;
    } else {
        chan_states = TAS6424_ALL_STATE_MUTE as u8;
    }
    snd_soc_component_write(component, TAS6424_CH_STATE_CTRL, chan_states as c_uint);

    /* any time we come out of HIZ, the output channels automatically run DC
     * load diagnostics if autodiagnotics are enabled. wait here until this
     * completes.
     */
    if no_auto_diags == 0 {
        msleep(230);
    }

    0
}

unsafe extern "C" fn tas6424_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);

    dev_dbg((*component).dev, b"%s() level=%d\n\0".as_ptr() as *const c_char, b"tas6424_set_bias_level\0".as_ptr(), level);

    if level == SND_SOC_BIAS_ON || level == SND_SOC_BIAS_PREPARE {
    } else if level == SND_SOC_BIAS_STANDBY {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            tas6424_power_on(component);
        }
    } else if level == SND_SOC_BIAS_OFF {
        tas6424_power_off(component);
    }

    0
}

static soc_codec_dev_tas6424: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(tas6424_set_bias_level),
    controls: tas6424_snd_controls.as_ptr(),
    num_controls: tas6424_snd_controls.len() as c_uint,
    dapm_widgets: tas6424_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas6424_dapm_widgets.len() as c_uint,
    dapm_routes: tas6424_audio_map.as_ptr(),
    num_dapm_routes: tas6424_audio_map.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static tas6424_speaker_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tas6424_hw_params),
    set_fmt: Some(tas6424_set_dai_fmt),
    set_tdm_slot: Some(tas6424_set_dai_tdm_slot),
    mute_stream: Some(tas6424_mute),
    no_capture_mute: 1,
};

static mut tas6424_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"tas6424-amplifier\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 4,
        rates: unsafe { TAS6424_RATES },
        formats: unsafe { TAS6424_FORMATS },
    },
    ops: &tas6424_speaker_dai_ops,
}];

unsafe extern "C" fn tas6424_fault_check_work(work: *mut work_struct) {
    let tas6424 = (work as *mut u8).sub(core::mem::offset_of!(tas6424_data, fault_check_work)) as *mut tas6424_data;
    let dev = (*tas6424).dev;
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_read((*tas6424).regmap, TAS6424_CHANNEL_FAULT, &mut reg);
    if ret < 0 {
        dev_err(dev, b"failed to read CHANNEL_FAULT register: %d\n\0".as_ptr() as *const c_char, ret);
        goto_out(tas6424);
        return;
    }

    if reg == 0 {
        (*tas6424).last_cfault = reg;
    } else {
        /*
         * Only flag errors once for a given occurrence. This is needed as
         * the TAS6424 will take time clearing the fault condition internally
         * during which we don't want to bombard the system with the same
         * error message over and over.
         */
        if (reg & TAS6424_FAULT_OC_CH1) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_OC_CH1) == 0 {
            dev_crit(dev, b"experienced a channel 1 overcurrent fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OC_CH2) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_OC_CH2) == 0 {
            dev_crit(dev, b"experienced a channel 2 overcurrent fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OC_CH3) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_OC_CH3) == 0 {
            dev_crit(dev, b"experienced a channel 3 overcurrent fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OC_CH4) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_OC_CH4) == 0 {
            dev_crit(dev, b"experienced a channel 4 overcurrent fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_DC_CH1) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_DC_CH1) == 0 {
            dev_crit(dev, b"experienced a channel 1 DC fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_DC_CH2) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_DC_CH2) == 0 {
            dev_crit(dev, b"experienced a channel 2 DC fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_DC_CH3) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_DC_CH3) == 0 {
            dev_crit(dev, b"experienced a channel 3 DC fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_DC_CH4) != 0 && ((*tas6424).last_cfault & TAS6424_FAULT_DC_CH4) == 0 {
            dev_crit(dev, b"experienced a channel 4 DC fault\n\0".as_ptr() as *const c_char);
        }

        /* Store current fault1 value so we can detect any changes next time */
        (*tas6424).last_cfault = reg;
    }

    ret = regmap_read((*tas6424).regmap, TAS6424_GLOB_FAULT1, &mut reg);
    if ret < 0 {
        dev_err(dev, b"failed to read GLOB_FAULT1 register: %d\n\0".as_ptr() as *const c_char, ret);
        goto_out(tas6424);
        return;
    }

    /*
     * Ignore any clock faults as there is no clean way to check for them.
     * We would need to start checking for those faults *after* the SAIF
     * stream has been setup, and stop checking *before* the stream is
     * stopped to avoid any false-positives. However there are no
     * appropriate hooks to monitor these events.
     */
    reg &= TAS6424_FAULT_PVDD_OV | TAS6424_FAULT_VBAT_OV | TAS6424_FAULT_PVDD_UV | TAS6424_FAULT_VBAT_UV;

    if reg == 0 {
        (*tas6424).last_fault1 = reg;
    } else {
        if (reg & TAS6424_FAULT_PVDD_OV) != 0 && ((*tas6424).last_fault1 & TAS6424_FAULT_PVDD_OV) == 0 {
            dev_crit(dev, b"experienced a PVDD overvoltage fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_VBAT_OV) != 0 && ((*tas6424).last_fault1 & TAS6424_FAULT_VBAT_OV) == 0 {
            dev_crit(dev, b"experienced a VBAT overvoltage fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_PVDD_UV) != 0 && ((*tas6424).last_fault1 & TAS6424_FAULT_PVDD_UV) == 0 {
            dev_crit(dev, b"experienced a PVDD undervoltage fault\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_VBAT_UV) != 0 && ((*tas6424).last_fault1 & TAS6424_FAULT_VBAT_UV) == 0 {
            dev_crit(dev, b"experienced a VBAT undervoltage fault\n\0".as_ptr() as *const c_char);
        }

        /* Store current fault1 value so we can detect any changes next time */
        (*tas6424).last_fault1 = reg;
    }

    ret = regmap_read((*tas6424).regmap, TAS6424_GLOB_FAULT2, &mut reg);
    if ret < 0 {
        dev_err(dev, b"failed to read GLOB_FAULT2 register: %d\n\0".as_ptr() as *const c_char, ret);
        goto_out(tas6424);
        return;
    }

    reg &= TAS6424_FAULT_OTSD | TAS6424_FAULT_OTSD_CH1 | TAS6424_FAULT_OTSD_CH2 | TAS6424_FAULT_OTSD_CH3 | TAS6424_FAULT_OTSD_CH4;

    if reg == 0 {
        (*tas6424).last_fault2 = reg;
    } else {
        if (reg & TAS6424_FAULT_OTSD) != 0 && ((*tas6424).last_fault2 & TAS6424_FAULT_OTSD) == 0 {
            dev_crit(dev, b"experienced a global overtemp shutdown\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OTSD_CH1) != 0 && ((*tas6424).last_fault2 & TAS6424_FAULT_OTSD_CH1) == 0 {
            dev_crit(dev, b"experienced an overtemp shutdown on CH1\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OTSD_CH2) != 0 && ((*tas6424).last_fault2 & TAS6424_FAULT_OTSD_CH2) == 0 {
            dev_crit(dev, b"experienced an overtemp shutdown on CH2\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OTSD_CH3) != 0 && ((*tas6424).last_fault2 & TAS6424_FAULT_OTSD_CH3) == 0 {
            dev_crit(dev, b"experienced an overtemp shutdown on CH3\n\0".as_ptr() as *const c_char);
        }
        if (reg & TAS6424_FAULT_OTSD_CH4) != 0 && ((*tas6424).last_fault2 & TAS6424_FAULT_OTSD_CH4) == 0 {
            dev_crit(dev, b"experienced an overtemp shutdown on CH4\n\0".as_ptr() as *const c_char);
        }

        /* Store current fault2 value so we can detect any changes next time */
        (*tas6424).last_fault2 = reg;
    }

    ret = regmap_read((*tas6424).regmap, TAS6424_WARN, &mut reg);
    if ret < 0 {
        dev_err(dev, b"failed to read WARN register: %d\n\0".as_ptr() as *const c_char, ret);
        goto_out(tas6424);
        return;
    }

    reg &= TAS6424_WARN_VDD_UV
        | TAS6424_WARN_VDD_POR
        | TAS6424_WARN_VDD_OTW
        | TAS6424_WARN_VDD_OTW_CH1
        | TAS6424_WARN_VDD_OTW_CH2
        | TAS6424_WARN_VDD_OTW_CH3
        | TAS6424_WARN_VDD_OTW_CH4;

    if reg == 0 {
        (*tas6424).last_warn = reg;
        goto_out(tas6424);
        return;
    }

    if (reg & TAS6424_WARN_VDD_UV) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_UV) == 0 {
        dev_warn(dev, b"experienced a VDD under voltage condition\n\0".as_ptr() as *const c_char);
    }
    if (reg & TAS6424_WARN_VDD_POR) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_POR) == 0 {
        dev_warn(dev, b"experienced a VDD POR condition\n\0".as_ptr() as *const c_char);
    }
    if (reg & TAS6424_WARN_VDD_OTW) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_OTW) == 0 {
        dev_warn(dev, b"experienced a global overtemp warning\n\0".as_ptr() as *const c_char);
    }
    if (reg & TAS6424_WARN_VDD_OTW_CH1) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_OTW_CH1) == 0 {
        dev_warn(dev, b"experienced an overtemp warning on CH1\n\0".as_ptr() as *const c_char);
    }
    if (reg & TAS6424_WARN_VDD_OTW_CH2) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_OTW_CH2) == 0 {
        dev_warn(dev, b"experienced an overtemp warning on CH2\n\0".as_ptr() as *const c_char);
    }
    if (reg & TAS6424_WARN_VDD_OTW_CH3) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_OTW_CH3) == 0 {
        dev_warn(dev, b"experienced an overtemp warning on CH3\n\0".as_ptr() as *const c_char);
    }
    if (reg & TAS6424_WARN_VDD_OTW_CH4) != 0 && ((*tas6424).last_warn & TAS6424_WARN_VDD_OTW_CH4) == 0 {
        dev_warn(dev, b"experienced an overtemp warning on CH4\n\0".as_ptr() as *const c_char);
    }

    /* Store current warn value so we can detect any changes next time */
    (*tas6424).last_warn = reg;

    /* Clear any warnings by toggling the CLEAR_FAULT control bit */
    ret = regmap_write_bits((*tas6424).regmap, TAS6424_MISC_CTRL3, TAS6424_CLEAR_FAULT, TAS6424_CLEAR_FAULT);
    if ret < 0 {
        dev_err(dev, b"failed to write MISC_CTRL3 register: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret = regmap_write_bits((*tas6424).regmap, TAS6424_MISC_CTRL3, TAS6424_CLEAR_FAULT, 0);
    if ret < 0 {
        dev_err(dev, b"failed to write MISC_CTRL3 register: %d\n\0".as_ptr() as *const c_char, ret);
    }

    goto_out(tas6424);
}

unsafe fn goto_out(tas6424: *mut tas6424_data) {
    /* Schedule the next fault check at the specified interval */
    schedule_delayed_work(
        &mut (*tas6424).fault_check_work,
        msecs_to_jiffies(TAS6424_FAULT_CHECK_INTERVAL),
    );
}

static tas6424_reg_defaults: [reg_default; 20] = [
    reg_default { reg: unsafe { TAS6424_MODE_CTRL }, def: 0x00 },
    reg_default { reg: unsafe { TAS6424_MISC_CTRL1 }, def: 0x32 },
    reg_default { reg: unsafe { TAS6424_MISC_CTRL2 }, def: 0x62 },
    reg_default { reg: unsafe { TAS6424_SAP_CTRL }, def: 0x04 },
    reg_default { reg: unsafe { TAS6424_CH_STATE_CTRL }, def: 0x55 },
    reg_default { reg: unsafe { TAS6424_CH1_VOL_CTRL }, def: 0xcf },
    reg_default { reg: unsafe { TAS6424_CH2_VOL_CTRL }, def: 0xcf },
    reg_default { reg: unsafe { TAS6424_CH3_VOL_CTRL }, def: 0xcf },
    reg_default { reg: unsafe { TAS6424_CH4_VOL_CTRL }, def: 0xcf },
    reg_default { reg: unsafe { TAS6424_DC_DIAG_CTRL1 }, def: 0x00 },
    reg_default { reg: unsafe { TAS6424_DC_DIAG_CTRL2 }, def: 0x11 },
    reg_default { reg: unsafe { TAS6424_DC_DIAG_CTRL3 }, def: 0x11 },
    reg_default { reg: unsafe { TAS6424_PIN_CTRL }, def: 0xff },
    reg_default { reg: unsafe { TAS6424_AC_DIAG_CTRL1 }, def: 0x00 },
    reg_default { reg: unsafe { TAS6424_MISC_CTRL3 }, def: 0x00 },
    reg_default { reg: unsafe { TAS6424_CLIP_CTRL }, def: 0x01 },
    reg_default { reg: unsafe { TAS6424_CLIP_WINDOW }, def: 0x14 },
    reg_default { reg: unsafe { TAS6424_CLIP_WARN }, def: 0x00 },
    reg_default { reg: unsafe { TAS6424_CBC_STAT }, def: 0x00 },
    reg_default { reg: unsafe { TAS6424_MISC_CTRL4 }, def: 0x40 },
];

unsafe extern "C" fn tas6424_is_writable_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == TAS6424_MODE_CTRL
        || reg == TAS6424_MISC_CTRL1
        || reg == TAS6424_MISC_CTRL2
        || reg == TAS6424_SAP_CTRL
        || reg == TAS6424_CH_STATE_CTRL
        || reg == TAS6424_CH1_VOL_CTRL
        || reg == TAS6424_CH2_VOL_CTRL
        || reg == TAS6424_CH3_VOL_CTRL
        || reg == TAS6424_CH4_VOL_CTRL
        || reg == TAS6424_DC_DIAG_CTRL1
        || reg == TAS6424_DC_DIAG_CTRL2
        || reg == TAS6424_DC_DIAG_CTRL3
        || reg == TAS6424_PIN_CTRL
        || reg == TAS6424_AC_DIAG_CTRL1
        || reg == TAS6424_MISC_CTRL3
        || reg == TAS6424_CLIP_CTRL
        || reg == TAS6424_CLIP_WINDOW
        || reg == TAS6424_CLIP_WARN
        || reg == TAS6424_CBC_STAT
        || reg == TAS6424_MISC_CTRL4
}

unsafe extern "C" fn tas6424_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == TAS6424_DC_LOAD_DIAG_REP12
        || reg == TAS6424_DC_LOAD_DIAG_REP34
        || reg == TAS6424_DC_LOAD_DIAG_REPLO
        || reg == TAS6424_CHANNEL_STATE
        || reg == TAS6424_CHANNEL_FAULT
        || reg == TAS6424_GLOB_FAULT1
        || reg == TAS6424_GLOB_FAULT2
        || reg == TAS6424_WARN
        || reg == TAS6424_AC_LOAD_DIAG_REP1
        || reg == TAS6424_AC_LOAD_DIAG_REP2
        || reg == TAS6424_AC_LOAD_DIAG_REP3
        || reg == TAS6424_AC_LOAD_DIAG_REP4
}

static tas6424_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    writeable_reg: Some(tas6424_is_writable_reg),
    volatile_reg: Some(tas6424_is_volatile_reg),
    max_register: unsafe { TAS6424_MAX },
    reg_defaults: tas6424_reg_defaults.as_ptr(),
    num_reg_defaults: tas6424_reg_defaults.len() as c_uint,
    cache_type: unsafe { REGCACHE_RBTREE },
};

// #if IS_ENABLED(CONFIG_OF)
static tas6424_of_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"ti,tas6424\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, tas6424_of_ids);
// #endif

unsafe extern "C" fn tas6424_i2c_probe(client: *mut i2c_client) -> c_int {
    let dev = &mut (*client).dev as *mut device;
    let tas6424: *mut tas6424_data;
    let mut ret: c_int;
    let mut i: c_int;

    tas6424 = devm_kzalloc(dev, core::mem::size_of::<tas6424_data>(), GFP_KERNEL) as *mut tas6424_data;
    if tas6424.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(dev, tas6424 as *mut c_void);

    (*tas6424).dev = dev;

    (*tas6424).regmap = devm_regmap_init_i2c(client, &tas6424_regmap_config);
    if IS_ERR((*tas6424).regmap as *const c_void) {
        ret = PTR_ERR((*tas6424).regmap as *const c_void) as c_int;
        dev_err(dev, b"unable to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /*
     * Get control of the standby pin and set it LOW to take the codec
     * out of the stand-by mode.
     * Note: The actual pin polarity is taken care of in the GPIO lib
     * according the polarity specified in the DTS.
     */
    (*tas6424).standby_gpio = devm_gpiod_get_optional(dev, b"standby\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*tas6424).standby_gpio as *const c_void) {
        if PTR_ERR((*tas6424).standby_gpio as *const c_void) == -(EPROBE_DEFER as c_long) {
            return -EPROBE_DEFER;
        }
        dev_info(
            dev,
            b"failed to get standby GPIO: %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR((*tas6424).standby_gpio as *const c_void),
        );
        (*tas6424).standby_gpio = ptr::null_mut();
    }

    /*
     * Get control of the mute pin and set it HIGH in order to start with
     * all the output muted.
     * Note: The actual pin polarity is taken care of in the GPIO lib
     * according the polarity specified in the DTS.
     */
    (*tas6424).mute_gpio = devm_gpiod_get_optional(dev, b"mute\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*tas6424).mute_gpio as *const c_void) {
        if PTR_ERR((*tas6424).mute_gpio as *const c_void) == -(EPROBE_DEFER as c_long) {
            return -EPROBE_DEFER;
        }
        dev_info(
            dev,
            b"failed to get nmute GPIO: %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR((*tas6424).mute_gpio as *const c_void),
        );
        (*tas6424).mute_gpio = ptr::null_mut();
    }

    i = 0;
    while i < TAS6424_NUM_SUPPLIES as c_int {
        (*tas6424).supplies[i as usize].supply = tas6424_supply_names[i as usize];
        i += 1;
    }
    ret = devm_regulator_bulk_get(dev, TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"unable to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regulator_bulk_enable(TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"unable to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* Reset device to establish well-defined startup state */
    ret = regmap_update_bits((*tas6424).regmap, TAS6424_MODE_CTRL, TAS6424_RESET, TAS6424_RESET);
    if ret != 0 {
        dev_err(dev, b"unable to reset device: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
        return ret;
    }

    INIT_DELAYED_WORK(&mut (*tas6424).fault_check_work, tas6424_fault_check_work);

    ret = devm_snd_soc_register_component(
        dev,
        &soc_codec_dev_tas6424,
        tas6424_dai.as_mut_ptr(),
        tas6424_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(dev, b"unable to register codec: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn tas6424_i2c_remove(client: *mut i2c_client) {
    let dev = &mut (*client).dev as *mut device;
    let tas6424 = dev_get_drvdata(dev) as *mut tas6424_data;
    let ret: c_int;

    cancel_delayed_work_sync(&mut (*tas6424).fault_check_work);

    /* put the codec in stand-by */
    if !(*tas6424).standby_gpio.is_null() {
        gpiod_set_value_cansleep((*tas6424).standby_gpio, 1);
    }

    ret = regulator_bulk_disable(TAS6424_NUM_SUPPLIES as c_int, (*tas6424).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err(dev, b"unable to disable supplies: %d\n\0".as_ptr() as *const c_char, ret);
    }
}

static tas6424_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id { name: b"tas6424\0".as_ptr() as *const c_char },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, tas6424_i2c_ids);

static mut tas6424_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tas6424\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(tas6424_of_ids.as_ptr()) },
    },
    probe: Some(tas6424_i2c_probe),
    remove: Some(tas6424_i2c_remove),
    id_table: tas6424_i2c_ids.as_ptr(),
};
// module_i2c_driver(tas6424_i2c_driver);

// MODULE_AUTHOR("Andreas Dannenberg <dannenberg@ti.com>");
// MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
// MODULE_DESCRIPTION("TAS6424 Audio amplifier driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
