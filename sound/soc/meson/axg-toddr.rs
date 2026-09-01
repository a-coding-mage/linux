// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

/* This driver implements the frontend capture DAI of AXG based SoCs */

// C dependencies:
// linux/bitfield.h, linux/clk.h, linux/regmap.h, linux/module.h,
// linux/of_platform.h, sound/pcm_params.h, sound/soc.h, sound/soc-dai.h,
// "axg-fifo.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const fn bit(n: u32) -> c_uint {
    1u32 << n
}

const fn genmask(h: u32, l: u32) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const CTRL0_TODDR_SEL_RESAMPLE: c_uint = bit(30);
const CTRL0_TODDR_EXT_SIGNED: c_uint = bit(29);
const CTRL0_TODDR_PP_MODE: c_uint = bit(28);
const CTRL0_TODDR_SYNC_CH: c_uint = bit(27);
const CTRL0_TODDR_TYPE: c_uint = genmask(15, 13);
const CTRL0_TODDR_MSB_POS: c_uint = genmask(12, 8);
const CTRL0_TODDR_LSB_POS: c_uint = genmask(7, 3);
const CTRL1_TODDR_FORCE_FINISH: c_uint = bit(25);
const CTRL1_SEL_SHIFT: c_uint = 28;

const TODDR_MSB_POS: c_uint = 31;

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
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
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct axg_fifo {
    pub map: *mut regmap,
    pub pclk: *mut clk,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct soc_enum {
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
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub close: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn() -> c_int>,
    pub pointer: Option<unsafe extern "C" fn() -> usize>,
    pub trigger: Option<unsafe extern "C" fn() -> c_int>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct reg_field {
    pub reg: c_uint,
    pub lsb: c_uint,
    pub msb: c_uint,
}

#[repr(C)]
pub struct axg_fifo_match_data {
    pub field_threshold: reg_field,
    pub component_drv: *const snd_soc_component_driver,
    pub dai_drv: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn() -> c_int>,
    pub driver: device_driver,
}

const FIFO_CTRL0: c_uint = 0;
const FIFO_CTRL1: c_uint = 0;
const CTRL0_SEL_SHIFT: c_uint = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 0;
const AXG_FIFO_CH_MAX: c_uint = 0;
const AXG_FIFO_FORMATS: u64 = 0;
const EINVAL: c_int = 22;

const fn reg_field_init(reg: c_uint, lsb: c_uint, msb: c_uint) -> reg_field {
    reg_field { reg, lsb, msb }
}

unsafe extern "C" {
    fn axg_fifo_pcm_new(rtd: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn axg_fifo_pcm_open() -> c_int;
    fn axg_fifo_pcm_close() -> c_int;
    fn axg_fifo_pcm_hw_params() -> c_int;
    fn g12a_fifo_pcm_hw_params() -> c_int;
    fn axg_fifo_pcm_hw_free() -> c_int;
    fn axg_fifo_pcm_pointer() -> usize;
    fn axg_fifo_pcm_trigger() -> c_int;
    fn axg_fifo_probe() -> c_int;
}

unsafe extern "C" fn axg_toddr_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    unsafe { axg_fifo_pcm_new(rtd, SNDRV_PCM_STREAM_CAPTURE) }
}

unsafe extern "C" fn g12a_toddr_dai_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let fifo = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_fifo };

    /* Reset the write pointer to the FIFO_INIT_ADDR */
    unsafe {
        regmap_update_bits((*fifo).map, FIFO_CTRL1, CTRL1_TODDR_FORCE_FINISH, 0);
        regmap_update_bits(
            (*fifo).map,
            FIFO_CTRL1,
            CTRL1_TODDR_FORCE_FINISH,
            CTRL1_TODDR_FORCE_FINISH,
        );
        regmap_update_bits((*fifo).map, FIFO_CTRL1, CTRL1_TODDR_FORCE_FINISH, 0);
    }

    0
}

unsafe extern "C" fn axg_toddr_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let fifo = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_fifo };
    let type_: c_uint;
    let width: c_uint;

    match unsafe { params_physical_width(params) } {
        8 => {
            type_ = 0; /* 8 samples of 8 bits */
        }
        16 => {
            type_ = 2; /* 4 samples of 16 bits - right justified */
        }
        32 => {
            type_ = 4; /* 2 samples of 32 bits - right justified */
        }
        _ => return -EINVAL,
    }

    width = unsafe { params_width(params) };

    unsafe {
        regmap_update_bits(
            (*fifo).map,
            FIFO_CTRL0,
            CTRL0_TODDR_TYPE | CTRL0_TODDR_MSB_POS | CTRL0_TODDR_LSB_POS,
            field_prep(CTRL0_TODDR_TYPE, type_)
                | field_prep(CTRL0_TODDR_MSB_POS, TODDR_MSB_POS)
                | field_prep(
                    CTRL0_TODDR_LSB_POS,
                    TODDR_MSB_POS.wrapping_sub(width.wrapping_sub(1)),
                ),
        );
    }

    0
}

unsafe extern "C" fn axg_toddr_dai_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let fifo = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_fifo };
    let ret: c_int;

    /* Enable pclk to access registers and clock the fifo ip */
    ret = unsafe { clk_prepare_enable((*fifo).pclk) };
    if ret != 0 {
        return ret;
    }

    /* Select orginal data - resampling not supported ATM */
    unsafe {
        regmap_update_bits((*fifo).map, FIFO_CTRL0, CTRL0_TODDR_SEL_RESAMPLE, 0);
    }

    /* Only signed format are supported ATM */
    unsafe {
        regmap_update_bits(
            (*fifo).map,
            FIFO_CTRL0,
            CTRL0_TODDR_EXT_SIGNED,
            CTRL0_TODDR_EXT_SIGNED,
        );
    }

    /* Apply single buffer mode to the interface */
    unsafe {
        regmap_update_bits((*fifo).map, FIFO_CTRL0, CTRL0_TODDR_PP_MODE, 0);
    }

    0
}

unsafe extern "C" fn axg_toddr_dai_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let fifo = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_fifo };

    unsafe {
        clk_disable_unprepare((*fifo).pclk);
    }
}

static axg_toddr_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: None,
    hw_params: Some(axg_toddr_dai_hw_params),
    startup: Some(axg_toddr_dai_startup),
    shutdown: Some(axg_toddr_dai_shutdown),
    pcm_new: Some(axg_toddr_pcm_new),
};

static mut axg_toddr_dai_drv: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"TODDR".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: AXG_FIFO_CH_MAX,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5515,
        rate_max: 768000,
        formats: AXG_FIFO_FORMATS,
    },
    ops: &axg_toddr_ops,
};

static axg_toddr_sel_texts: [*const c_char; 8] = [
    c"IN 0".as_ptr(),
    c"IN 1".as_ptr(),
    c"IN 2".as_ptr(),
    c"IN 3".as_ptr(),
    c"IN 4".as_ptr(),
    c"IN 5".as_ptr(),
    c"IN 6".as_ptr(),
    c"IN 7".as_ptr(),
];

// static SOC_ENUM_SINGLE_DECL(axg_toddr_sel_enum, FIFO_CTRL0, CTRL0_SEL_SHIFT, axg_toddr_sel_texts);
static axg_toddr_sel_enum: soc_enum = soc_enum { _private: [] };

// static const struct snd_kcontrol_new axg_toddr_in_mux =
//      SOC_DAPM_ENUM("Input Source", axg_toddr_sel_enum);
static axg_toddr_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// SND_SOC_DAPM_* widget macro initializers are preserved as zero-sized external-layout placeholders.
static axg_toddr_dapm_widgets: [snd_soc_dapm_widget; 9] = [
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

static axg_toddr_dapm_routes: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: core::ptr::null(), source: c"SRC SEL".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 0".as_ptr(), source: c"IN 0".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 1".as_ptr(), source: c"IN 1".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 2".as_ptr(), source: c"IN 2".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 3".as_ptr(), source: c"IN 3".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 4".as_ptr(), source: c"IN 4".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 5".as_ptr(), source: c"IN 5".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 6".as_ptr(), source: c"IN 6".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 7".as_ptr(), source: c"IN 7".as_ptr() },
];

static axg_toddr_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: axg_toddr_dapm_widgets.as_ptr(),
    num_dapm_widgets: array_size(&axg_toddr_dapm_widgets) as c_uint,
    dapm_routes: axg_toddr_dapm_routes.as_ptr(),
    num_dapm_routes: array_size(&axg_toddr_dapm_routes) as c_uint,
    open: Some(axg_fifo_pcm_open),
    close: Some(axg_fifo_pcm_close),
    hw_params: Some(axg_fifo_pcm_hw_params),
    hw_free: Some(axg_fifo_pcm_hw_free),
    pointer: Some(axg_fifo_pcm_pointer),
    trigger: Some(axg_fifo_pcm_trigger),
    legacy_dai_naming: 1,
};

static axg_toddr_match_data: axg_fifo_match_data = axg_fifo_match_data {
    field_threshold: reg_field_init(FIFO_CTRL1, 16, 23),
    component_drv: &axg_toddr_component_drv,
    dai_drv: unsafe { &raw mut axg_toddr_dai_drv },
};

unsafe extern "C" fn g12a_toddr_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let fifo = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_fifo };
    let ret: c_int;

    ret = unsafe { axg_toddr_dai_startup(substream, dai) };
    if ret != 0 {
        return ret;
    }

    /*
     * Make sure the first channel ends up in the at beginning of the output
     * As weird as it looks, without this the first channel may be misplaced
     * in memory, with a random shift of 2 channels.
     */
    unsafe {
        regmap_update_bits((*fifo).map, FIFO_CTRL0, CTRL0_TODDR_SYNC_CH, CTRL0_TODDR_SYNC_CH);
    }

    0
}

static g12a_toddr_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(g12a_toddr_dai_prepare),
    hw_params: Some(axg_toddr_dai_hw_params),
    startup: Some(g12a_toddr_dai_startup),
    shutdown: Some(axg_toddr_dai_shutdown),
    pcm_new: Some(axg_toddr_pcm_new),
};

static mut g12a_toddr_dai_drv: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"TODDR".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: AXG_FIFO_CH_MAX,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5515,
        rate_max: 768000,
        formats: AXG_FIFO_FORMATS,
    },
    ops: &g12a_toddr_ops,
};

static g12a_toddr_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: axg_toddr_dapm_widgets.as_ptr(),
    num_dapm_widgets: array_size(&axg_toddr_dapm_widgets) as c_uint,
    dapm_routes: axg_toddr_dapm_routes.as_ptr(),
    num_dapm_routes: array_size(&axg_toddr_dapm_routes) as c_uint,
    open: Some(axg_fifo_pcm_open),
    close: Some(axg_fifo_pcm_close),
    hw_params: Some(g12a_fifo_pcm_hw_params),
    hw_free: Some(axg_fifo_pcm_hw_free),
    pointer: Some(axg_fifo_pcm_pointer),
    trigger: Some(axg_fifo_pcm_trigger),
    legacy_dai_naming: 1,
};

static g12a_toddr_match_data: axg_fifo_match_data = axg_fifo_match_data {
    field_threshold: reg_field_init(FIFO_CTRL1, 16, 23),
    component_drv: &g12a_toddr_component_drv,
    dai_drv: unsafe { &raw mut g12a_toddr_dai_drv },
};

static sm1_toddr_sel_texts: [*const c_char; 16] = [
    c"IN 0".as_ptr(), c"IN 1".as_ptr(), c"IN 2".as_ptr(), c"IN 3".as_ptr(),
    c"IN 4".as_ptr(), c"IN 5".as_ptr(), c"IN 6".as_ptr(), c"IN 7".as_ptr(),
    c"IN 8".as_ptr(), c"IN 9".as_ptr(), c"IN 10".as_ptr(), c"IN 11".as_ptr(),
    c"IN 12".as_ptr(), c"IN 13".as_ptr(), c"IN 14".as_ptr(), c"IN 15".as_ptr(),
];

// static SOC_ENUM_SINGLE_DECL(sm1_toddr_sel_enum, FIFO_CTRL1, CTRL1_SEL_SHIFT, sm1_toddr_sel_texts);
static sm1_toddr_sel_enum: soc_enum = soc_enum { _private: [] };

// static const struct snd_kcontrol_new sm1_toddr_in_mux =
//      SOC_DAPM_ENUM("Input Source", sm1_toddr_sel_enum);
static sm1_toddr_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// SND_SOC_DAPM_* widget macro initializers are preserved as zero-sized external-layout placeholders.
static sm1_toddr_dapm_widgets: [snd_soc_dapm_widget; 17] = [
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
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static sm1_toddr_dapm_routes: [snd_soc_dapm_route; 17] = [
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: core::ptr::null(), source: c"SRC SEL".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 0".as_ptr(), source: c"IN 0".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 1".as_ptr(), source: c"IN 1".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 2".as_ptr(), source: c"IN 2".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 3".as_ptr(), source: c"IN 3".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 4".as_ptr(), source: c"IN 4".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 5".as_ptr(), source: c"IN 5".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 6".as_ptr(), source: c"IN 6".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 7".as_ptr(), source: c"IN 7".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 8".as_ptr(), source: c"IN 8".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 9".as_ptr(), source: c"IN 9".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 10".as_ptr(), source: c"IN 10".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 11".as_ptr(), source: c"IN 11".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 12".as_ptr(), source: c"IN 12".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 13".as_ptr(), source: c"IN 13".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 14".as_ptr(), source: c"IN 14".as_ptr() },
    snd_soc_dapm_route { sink: c"SRC SEL".as_ptr(), control: c"IN 15".as_ptr(), source: c"IN 15".as_ptr() },
];

static sm1_toddr_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: sm1_toddr_dapm_widgets.as_ptr(),
    num_dapm_widgets: array_size(&sm1_toddr_dapm_widgets) as c_uint,
    dapm_routes: sm1_toddr_dapm_routes.as_ptr(),
    num_dapm_routes: array_size(&sm1_toddr_dapm_routes) as c_uint,
    open: Some(axg_fifo_pcm_open),
    close: Some(axg_fifo_pcm_close),
    hw_params: Some(g12a_fifo_pcm_hw_params),
    hw_free: Some(axg_fifo_pcm_hw_free),
    pointer: Some(axg_fifo_pcm_pointer),
    trigger: Some(axg_fifo_pcm_trigger),
    legacy_dai_naming: 1,
};

static sm1_toddr_match_data: axg_fifo_match_data = axg_fifo_match_data {
    field_threshold: reg_field_init(FIFO_CTRL1, 12, 23),
    component_drv: &sm1_toddr_component_drv,
    dai_drv: unsafe { &raw mut g12a_toddr_dai_drv },
};

static axg_toddr_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: c"amlogic,axg-toddr".as_ptr(),
        data: &axg_toddr_match_data as *const axg_fifo_match_data as *const c_void,
    },
    of_device_id {
        compatible: c"amlogic,g12a-toddr".as_ptr(),
        data: &g12a_toddr_match_data as *const axg_fifo_match_data as *const c_void,
    },
    of_device_id {
        compatible: c"amlogic,sm1-toddr".as_ptr(),
        data: &sm1_toddr_match_data as *const axg_fifo_match_data as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, axg_toddr_of_match);

static mut axg_toddr_pdrv: platform_driver = platform_driver {
    probe: Some(axg_fifo_probe),
    driver: device_driver {
        name: c"axg-toddr".as_ptr(),
        of_match_table: axg_toddr_of_match.as_ptr(),
    },
};
// module_platform_driver(axg_toddr_pdrv);

// MODULE_DESCRIPTION("Amlogic AXG capture fifo driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
