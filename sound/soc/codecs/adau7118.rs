// SPDX-License-Identifier: GPL-2.0
//
// Analog Devices ADAU7118 8 channel PDM-to-I2S/TDM Converter driver
//
// Copyright 2019 Analog Devices Inc.

// C dependencies translated from:
// <linux/bitfield.h>, <linux/module.h>, <linux/regmap.h>,
// <linux/regulator/consumer.h>, <sound/pcm_params.h>, <sound/soc.h>,
// and "adau7118.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u32 = u32;
pub type bool_ = bool;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

pub type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_channel_map: Option<
        unsafe extern "C" fn(
            *mut snd_soc_dai,
            c_uint,
            *const c_uint,
            c_uint,
            *const c_uint,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_component_cache_sync(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, map: *mut regmap);
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut u32) -> c_int;
    fn device_property_read_u32_array(
        dev: *mut device,
        propname: *const c_char,
        val: *mut u32,
        nval: usize,
    ) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static ADAU7118_REG_ENABLES: c_uint;
    static ADAU7118_REG_SPT_CTRL1: c_uint;
    static ADAU7118_REG_SPT_CTRL2: c_uint;
    static ADAU7118_REG_DEC_RATIO_CLK_MAP: c_uint;
    static ADAU7118_REG_RESET: c_uint;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S20_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_ulong;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static GFP_KERNEL: c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

const ADAU7118_DEC_RATIO_MASK: u32 = GENMASK(1, 0);
const fn ADAU7118_DEC_RATIO(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_DEC_RATIO_MASK, x)
}
const ADAU7118_CLK_MAP_MASK: u32 = GENMASK(7, 4);
const ADAU7118_SLOT_WIDTH_MASK: u32 = GENMASK(5, 4);
const fn ADAU7118_SLOT_WIDTH(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_SLOT_WIDTH_MASK, x)
}
const ADAU7118_TRISTATE_MASK: u32 = BIT(6);
const fn ADAU7118_TRISTATE(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_TRISTATE_MASK, x)
}
const ADAU7118_DATA_FMT_MASK: u32 = GENMASK(3, 1);
const fn ADAU7118_DATA_FMT(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_DATA_FMT_MASK, x)
}
const ADAU7118_SAI_MODE_MASK: u32 = BIT(0);
const fn ADAU7118_SAI_MODE(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_SAI_MODE_MASK, x)
}
const ADAU7118_LRCLK_BCLK_POL_MASK: u32 = GENMASK(1, 0);
const fn ADAU7118_LRCLK_BCLK_POL(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_LRCLK_BCLK_POL_MASK, x)
}
const ADAU7118_SPT_SLOT_MASK: u32 = GENMASK(7, 4);
const fn ADAU7118_SPT_SLOT(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_SPT_SLOT_MASK, x)
}
const ADAU7118_FULL_SOFT_R_MASK: u32 = BIT(1);
const fn ADAU7118_FULL_SOFT_R(x: u32) -> u32 {
    FIELD_PREP(ADAU7118_FULL_SOFT_R_MASK, x)
}

unsafe extern "C" {
    fn ADAU7118_REG_SPT_CX(x: c_int) -> c_uint;
}

#[repr(C)]
pub struct adau7118_data {
    pub map: *mut regmap,
    pub dev: *mut device,
    pub iovdd: *mut regulator,
    pub dvdd: *mut regulator,
    pub slot_width: u32,
    pub slots: u32,
    pub hw_mode: bool,
    pub right_j: bool,
}

/* Input Enable */
static adau7118_dapm_pdm_control: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Capture Switch", ADAU7118_REG_ENABLES, 0, 1, 0),
    SOC_DAPM_SINGLE!("Capture Switch", ADAU7118_REG_ENABLES, 1, 1, 0),
    SOC_DAPM_SINGLE!("Capture Switch", ADAU7118_REG_ENABLES, 2, 1, 0),
    SOC_DAPM_SINGLE!("Capture Switch", ADAU7118_REG_ENABLES, 3, 1, 0),
];

static adau7118_widgets_sw: [snd_soc_dapm_widget; 14] = [
    /* Input Enable Switches */
    SND_SOC_DAPM_SWITCH!("PDM0", SND_SOC_NOPM, 0, 0, &adau7118_dapm_pdm_control[0]),
    SND_SOC_DAPM_SWITCH!("PDM1", SND_SOC_NOPM, 0, 0, &adau7118_dapm_pdm_control[1]),
    SND_SOC_DAPM_SWITCH!("PDM2", SND_SOC_NOPM, 0, 0, &adau7118_dapm_pdm_control[2]),
    SND_SOC_DAPM_SWITCH!("PDM3", SND_SOC_NOPM, 0, 0, &adau7118_dapm_pdm_control[3]),

    /* PDM Clocks */
    SND_SOC_DAPM_SUPPLY!("PDM_CLK0", ADAU7118_REG_ENABLES, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PDM_CLK1", ADAU7118_REG_ENABLES, 5, 0, core::ptr::null(), 0),

    /* Output channels */
    SND_SOC_DAPM_AIF_OUT!("AIF1TX1", "Capture", 0, ADAU7118_REG_SPT_CX(0), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX2", "Capture", 0, ADAU7118_REG_SPT_CX(1), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX3", "Capture", 0, ADAU7118_REG_SPT_CX(2), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX4", "Capture", 0, ADAU7118_REG_SPT_CX(3), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX5", "Capture", 0, ADAU7118_REG_SPT_CX(4), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX6", "Capture", 0, ADAU7118_REG_SPT_CX(5), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX7", "Capture", 0, ADAU7118_REG_SPT_CX(6), 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX8", "Capture", 0, ADAU7118_REG_SPT_CX(7), 0, 0),
];

static adau7118_routes_sw: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: c"PDM0".as_ptr(), control: c"Capture Switch".as_ptr(), source: c"PDM_DAT0".as_ptr() },
    snd_soc_dapm_route { sink: c"PDM1".as_ptr(), control: c"Capture Switch".as_ptr(), source: c"PDM_DAT1".as_ptr() },
    snd_soc_dapm_route { sink: c"PDM2".as_ptr(), control: c"Capture Switch".as_ptr(), source: c"PDM_DAT2".as_ptr() },
    snd_soc_dapm_route { sink: c"PDM3".as_ptr(), control: c"Capture Switch".as_ptr(), source: c"PDM_DAT3".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX1".as_ptr(), control: core::ptr::null(), source: c"PDM0".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX2".as_ptr(), control: core::ptr::null(), source: c"PDM0".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX3".as_ptr(), control: core::ptr::null(), source: c"PDM1".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX4".as_ptr(), control: core::ptr::null(), source: c"PDM1".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX5".as_ptr(), control: core::ptr::null(), source: c"PDM2".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX6".as_ptr(), control: core::ptr::null(), source: c"PDM2".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX7".as_ptr(), control: core::ptr::null(), source: c"PDM3".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX8".as_ptr(), control: core::ptr::null(), source: c"PDM3".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: core::ptr::null(), source: c"PDM_CLK0".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: core::ptr::null(), source: c"PDM_CLK1".as_ptr() },
];

static adau7118_widgets_hw: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_AIF_OUT!("AIF1TX", "Capture", 0, SND_SOC_NOPM, 0, 0),
];

static adau7118_routes_hw: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: core::ptr::null(), source: c"PDM_DAT0".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: core::ptr::null(), source: c"PDM_DAT1".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: core::ptr::null(), source: c"PDM_DAT2".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF1TX".as_ptr(), control: core::ptr::null(), source: c"PDM_DAT3".as_ptr() },
];

static adau7118_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_INPUT!("PDM_DAT0"),
    SND_SOC_DAPM_INPUT!("PDM_DAT1"),
    SND_SOC_DAPM_INPUT!("PDM_DAT2"),
    SND_SOC_DAPM_INPUT!("PDM_DAT3"),
];

unsafe extern "C" fn adau7118_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: c_uint,
    tx_slot: *const c_uint,
    _rx_num: c_uint,
    _rx_slot: *const c_uint,
) -> c_int {
    let st = snd_soc_component_get_drvdata((*dai).component) as *mut adau7118_data;
    let mut ret: c_int;

    dev_dbg((*st).dev, c"Set channel map, %d".as_ptr(), tx_num);

    let mut chan: c_int = 0;
    while chan < tx_num as c_int {
        ret = snd_soc_component_update_bits(
            (*dai).component,
            ADAU7118_REG_SPT_CX(chan),
            ADAU7118_SPT_SLOT_MASK,
            ADAU7118_SPT_SLOT(*tx_slot.add(chan as usize)),
        );
        if ret < 0 {
            return ret;
        }
        chan += 1;
    }

    0
}

unsafe extern "C" fn adau7118_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let st = snd_soc_component_get_drvdata((*dai).component) as *mut adau7118_data;
    let mut ret: c_int = 0;
    let regval: u32;

    dev_dbg((*st).dev, c"Set format, fmt:%d\n".as_ptr(), fmt);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            ret = snd_soc_component_update_bits(
                (*dai).component,
                ADAU7118_REG_SPT_CTRL1,
                ADAU7118_DATA_FMT_MASK,
                ADAU7118_DATA_FMT(0),
            );
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            ret = snd_soc_component_update_bits(
                (*dai).component,
                ADAU7118_REG_SPT_CTRL1,
                ADAU7118_DATA_FMT_MASK,
                ADAU7118_DATA_FMT(1),
            );
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            (*st).right_j = true;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            ret = snd_soc_component_update_bits(
                (*dai).component,
                ADAU7118_REG_SPT_CTRL1,
                ADAU7118_DATA_FMT_MASK,
                ADAU7118_DATA_FMT(1),
            );
        }
        _ => {
            dev_err(
                (*st).dev,
                c"Invalid format %d".as_ptr(),
                fmt & SND_SOC_DAIFMT_FORMAT_MASK,
            );
            return -EINVAL;
        }
    }

    if ret < 0 {
        return ret;
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {
            regval = ADAU7118_LRCLK_BCLK_POL(0);
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            regval = ADAU7118_LRCLK_BCLK_POL(2);
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            regval = ADAU7118_LRCLK_BCLK_POL(1);
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            regval = ADAU7118_LRCLK_BCLK_POL(3);
        }
        _ => {
            dev_err(
                (*st).dev,
                c"Invalid Inv mask %d".as_ptr(),
                fmt & SND_SOC_DAIFMT_INV_MASK,
            );
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        (*dai).component,
        ADAU7118_REG_SPT_CTRL2,
        ADAU7118_LRCLK_BCLK_POL_MASK,
        regval,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn adau7118_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let st = snd_soc_component_get_drvdata((*dai).component) as *mut adau7118_data;
    let ret: c_int;

    dev_dbg((*st).dev, c"Set tristate, %d\n".as_ptr(), tristate);

    ret = snd_soc_component_update_bits(
        (*dai).component,
        ADAU7118_REG_SPT_CTRL1,
        ADAU7118_TRISTATE_MASK,
        ADAU7118_TRISTATE(tristate as u32),
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn adau7118_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let st = snd_soc_component_get_drvdata((*dai).component) as *mut adau7118_data;
    let mut ret: c_int = 0;
    let regval: u32;

    dev_dbg((*st).dev, c"Set tdm, slots:%d width:%d\n".as_ptr(), slots, slot_width);

    match slot_width {
        32 => {
            regval = ADAU7118_SLOT_WIDTH(0);
        }
        24 => {
            regval = ADAU7118_SLOT_WIDTH(2);
        }
        16 => {
            regval = ADAU7118_SLOT_WIDTH(1);
        }
        _ => {
            dev_err((*st).dev, c"Invalid slot width:%d\n".as_ptr(), slot_width);
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        (*dai).component,
        ADAU7118_REG_SPT_CTRL1,
        ADAU7118_SLOT_WIDTH_MASK,
        regval,
    );
    if ret < 0 {
        return ret;
    }

    (*st).slot_width = slot_width as u32;
    (*st).slots = slots as u32;

    0
}

unsafe extern "C" fn adau7118_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let st = snd_soc_component_get_drvdata((*dai).component) as *mut adau7118_data;
    let data_width: u32 = params_width(params) as u32;
    let slots_width: u32;
    let mut ret: c_int;
    let regval: u32;

    if (*st).slots == 0 {
        /* set stereo mode */
        ret = snd_soc_component_update_bits(
            (*dai).component,
            ADAU7118_REG_SPT_CTRL1,
            ADAU7118_SAI_MODE_MASK,
            ADAU7118_SAI_MODE(0),
        );
        if ret < 0 {
            return ret;
        }

        slots_width = 32;
    } else {
        slots_width = (*st).slot_width;
    }

    if data_width > slots_width {
        dev_err(
            (*st).dev,
            c"Invalid data_width:%d, slots_width:%d".as_ptr(),
            data_width,
            slots_width,
        );
        return -EINVAL;
    }

    if (*st).right_j {
        match slots_width - data_width {
            8 => {
                /* delay bclck by 8 */
                regval = ADAU7118_DATA_FMT(2);
            }
            12 => {
                /* delay bclck by 12 */
                regval = ADAU7118_DATA_FMT(3);
            }
            16 => {
                /* delay bclck by 16 */
                regval = ADAU7118_DATA_FMT(4);
            }
            _ => {
                dev_err(
                    (*st).dev,
                    c"Cannot set right_j setting, slot_w:%d, data_w:%d\n".as_ptr(),
                    slots_width,
                    data_width,
                );
                return -EINVAL;
            }
        }

        ret = snd_soc_component_update_bits(
            (*dai).component,
            ADAU7118_REG_SPT_CTRL1,
            ADAU7118_DATA_FMT_MASK,
            regval,
        );
        if ret < 0 {
            return ret;
        }
    }

    0
}

unsafe extern "C" fn adau7118_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let st = snd_soc_component_get_drvdata(component) as *mut adau7118_data;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;

    dev_dbg((*st).dev, c"Set bias level %d\n".as_ptr(), level);

    if level == SND_SOC_BIAS_ON || level == SND_SOC_BIAS_PREPARE {
    } else if level == SND_SOC_BIAS_STANDBY {
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            /* power on */
            ret = regulator_enable((*st).iovdd);
            if ret != 0 {
                return ret;
            }

            /* there's no timing constraints before enabling dvdd */
            ret = regulator_enable((*st).dvdd);
            if ret != 0 {
                regulator_disable((*st).iovdd);
                return ret;
            }

            if (*st).hw_mode {
                return 0;
            }

            regcache_cache_only((*st).map, false);
            /* sync cache */
            ret = snd_soc_component_cache_sync(component);
        }
    } else if level == SND_SOC_BIAS_OFF {
        /* power off */
        ret = regulator_disable((*st).dvdd);
        if ret != 0 {
            return ret;
        }

        ret = regulator_disable((*st).iovdd);
        if ret != 0 {
            return ret;
        }

        if (*st).hw_mode {
            return 0;
        }

        /* cache only */
        regcache_mark_dirty((*st).map);
        regcache_cache_only((*st).map, true);
    }

    ret
}

unsafe extern "C" fn adau7118_component_probe(component: *mut snd_soc_component) -> c_int {
    let st = snd_soc_component_get_drvdata(component) as *mut adau7118_data;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;

    if (*st).hw_mode {
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau7118_widgets_hw.as_ptr(),
            adau7118_widgets_hw.len() as c_int,
        );
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dapm_add_routes(
            dapm,
            adau7118_routes_hw.as_ptr(),
            adau7118_routes_hw.len() as c_int,
        );
    } else {
        snd_soc_component_init_regmap(component, (*st).map);
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau7118_widgets_sw.as_ptr(),
            adau7118_widgets_sw.len() as c_int,
        );
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dapm_add_routes(
            dapm,
            adau7118_routes_sw.as_ptr(),
            adau7118_routes_sw.len() as c_int,
        );
    }

    ret
}

static adau7118_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(adau7118_hw_params),
    set_channel_map: Some(adau7118_set_channel_map),
    set_fmt: Some(adau7118_set_fmt),
    set_tdm_slot: Some(adau7118_set_tdm_slot),
    set_tristate: Some(adau7118_set_tristate),
};

static mut adau7118_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"adau7118-hifi-capture".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        formats: unsafe {
            SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S20_3LE
                | SNDRV_PCM_FMTBIT_S20_LE
                | SNDRV_PCM_FMTBIT_S24_LE
                | SNDRV_PCM_FMTBIT_S24_3LE
        },
        rates: unsafe { SNDRV_PCM_RATE_CONTINUOUS },
        rate_min: 4000,
        rate_max: 192000,
        sig_bits: 24,
    },
    ops: core::ptr::null(),
};

static adau7118_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adau7118_component_probe),
    set_bias_level: Some(adau7118_set_bias_level),
    dapm_widgets: adau7118_widgets.as_ptr(),
    num_dapm_widgets: adau7118_widgets.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn adau7118_regulator_setup(st: *mut adau7118_data) -> c_int {
    (*st).iovdd = devm_regulator_get((*st).dev, c"iovdd".as_ptr());
    if IS_ERR((*st).iovdd as *const c_void) {
        dev_err(
            (*st).dev,
            c"Could not get iovdd: %ld\n".as_ptr(),
            PTR_ERR((*st).iovdd as *const c_void),
        );
        return PTR_ERR((*st).iovdd as *const c_void);
    }

    (*st).dvdd = devm_regulator_get((*st).dev, c"dvdd".as_ptr());
    if IS_ERR((*st).dvdd as *const c_void) {
        dev_err(
            (*st).dev,
            c"Could not get dvdd: %ld\n".as_ptr(),
            PTR_ERR((*st).dvdd as *const c_void),
        );
        return PTR_ERR((*st).dvdd as *const c_void);
    }
    /* just assume the device is in reset */
    if !(*st).hw_mode {
        regcache_mark_dirty((*st).map);
        regcache_cache_only((*st).map, true);
    }

    0
}

unsafe fn adau7118_parset_dt(st: *const adau7118_data) -> c_int {
    let mut ret: c_int;
    let mut dec_ratio: u32 = 0;
    /* 4 inputs */
    let mut clk_map: [u32; 4] = [0; 4];
    let regval: u32;

    if (*st).hw_mode {
        return 0;
    }

    ret = device_property_read_u32(
        (*st).dev,
        c"adi,decimation-ratio".as_ptr(),
        &mut dec_ratio,
    );
    if ret == 0 {
        match dec_ratio {
            64 => {
                regval = ADAU7118_DEC_RATIO(0);
            }
            32 => {
                regval = ADAU7118_DEC_RATIO(1);
            }
            16 => {
                regval = ADAU7118_DEC_RATIO(2);
            }
            _ => {
                dev_err((*st).dev, c"Invalid dec ratio: %u".as_ptr(), dec_ratio);
                return -EINVAL;
            }
        }

        ret = regmap_update_bits(
            (*st).map,
            ADAU7118_REG_DEC_RATIO_CLK_MAP,
            ADAU7118_DEC_RATIO_MASK,
            regval,
        );
        if ret != 0 {
            return ret;
        }
    }

    ret = device_property_read_u32_array(
        (*st).dev,
        c"adi,pdm-clk-map".as_ptr(),
        clk_map.as_mut_ptr(),
        clk_map.len(),
    );
    if ret == 0 {
        let mut pdm: c_int;
        let mut _clk_map: u32 = 0;

        pdm = 0;
        while pdm < clk_map.len() as c_int {
            _clk_map |= clk_map[pdm as usize] << (pdm + 4);
            pdm += 1;
        }

        ret = regmap_update_bits(
            (*st).map,
            ADAU7118_REG_DEC_RATIO_CLK_MAP,
            ADAU7118_CLK_MAP_MASK,
            _clk_map,
        );
        if ret != 0 {
            return ret;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn adau7118_probe(
    dev: *mut device,
    map: *mut regmap,
    hw_mode: bool,
) -> c_int {
    let st: *mut adau7118_data;
    let mut ret: c_int;

    st = devm_kzalloc(dev, core::mem::size_of::<adau7118_data>(), GFP_KERNEL) as *mut adau7118_data;
    if st.is_null() {
        return -ENOMEM;
    }

    (*st).dev = dev;
    (*st).hw_mode = hw_mode;
    dev_set_drvdata(dev, st as *mut c_void);

    if !hw_mode {
        (*st).map = map;
        adau7118_dai.ops = &adau7118_ops;
        /*
         * Perform a full soft reset. This will set all register's
         * with their reset values.
         */
        ret = regmap_update_bits(
            map,
            ADAU7118_REG_RESET,
            ADAU7118_FULL_SOFT_R_MASK,
            ADAU7118_FULL_SOFT_R(1),
        );
        if ret != 0 {
            return ret;
        }
    }

    ret = adau7118_parset_dt(st);
    if ret != 0 {
        return ret;
    }

    ret = adau7118_regulator_setup(st);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(dev, &adau7118_component_driver, &raw mut adau7118_dai, 1)
}

EXPORT_SYMBOL_GPL!(adau7118_probe);

MODULE_AUTHOR!("Nuno Sa <nuno.sa@analog.com>");
MODULE_DESCRIPTION!("ADAU7118 8 channel PDM-to-I2S/TDM Converter driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
