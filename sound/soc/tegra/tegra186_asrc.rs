// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2022-2025 NVIDIA CORPORATION. All rights reserved.
//
// tegra186_asrc.c - Tegra186 ASRC driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;

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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub reg: c_uint,
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub audio_ch: c_int,
    pub client_ch: c_int,
    pub audio_bits: c_int,
    pub client_bits: c_int,
}

#[repr(C)]
pub struct tegra_asrc_soc_data {
    pub aram_start_addr: c_uint,
}

#[repr(C)]
pub struct tegra186_asrc_lane {
    pub ratio_source: c_uint,
    pub int_part: c_uint,
    pub frac_part: c_uint,
    pub hwcomp_disable: c_int,
    pub input_thresh: c_int,
    pub output_thresh: c_int,
}

#[repr(C)]
pub struct tegra186_asrc {
    pub regmap: *mut regmap,
    pub soc_data: *const tegra_asrc_soc_data,
    pub lane: [tegra186_asrc_lane; TEGRA186_ASRC_STREAM_MAX as usize],
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
}

#[repr(C)]
pub struct soc_mreg_control {
    pub regbase: c_uint,
    pub regcount: c_uint,
    pub nbits: c_uint,
    pub invert: c_uint,
    pub min: c_int,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 4],
}

type c_long = isize;

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_init {
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_uint,
    pub shift: c_uint,
    pub invert: c_uint,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_uint,
}

type snd_soc_dapm_widget_item = snd_soc_dapm_widget_init;

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn() -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub reg_default_cb: Option<unsafe extern "C" fn() -> c_int>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

extern "C" {
    static mut regmap_default_zero_cb: Option<unsafe extern "C" fn() -> c_int>;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
        change: *mut bool_,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt: *const snd_soc_component_driver,
        dais: *mut snd_soc_dai_driver,
        num_dais: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
}

extern "C" {
    static tegra186_asrc_pm_ops_generated: dev_pm_ops;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

extern "C" {
    static TEGRA186_ASRC_CFG: c_uint;
    static TEGRA186_ASRC_STREAM_STRIDE: c_uint;
    static TEGRA186_ASRC_RATIO_INT_PART: c_uint;
    static TEGRA186_ASRC_RATIO_FRAC_PART: c_uint;
    static TEGRA186_ASRC_MUTE_UNMUTE_DURATION: c_uint;
    static TEGRA186_ASRC_RX_CIF_CTRL: c_uint;
    static TEGRA186_ASRC_TX_CIF_CTRL: c_uint;
    static TEGRA186_ASRC_GLOBAL_ENB: c_uint;
    static TEGRA186_ASRC_GLOBAL_SOFT_RESET: c_uint;
    static TEGRA186_ASRC_GLOBAL_CG: c_uint;
    static TEGRA186_ASRC_GLOBAL_CFG: c_uint;
    static TEGRA186_ASRC_GLOBAL_SCRATCH_ADDR: c_uint;
    static TEGRA186_ASRC_GLOBAL_SCRATCH_CFG: c_uint;
    static TEGRA186_ASRC_RATIO_UPD_RX_CIF_CTRL: c_uint;
    static TEGRA186_ASRC_GLOBAL_INT_MASK: c_uint;
    static TEGRA186_ASRC_GLOBAL_INT_SET: c_uint;
    static TEGRA186_ASRC_GLOBAL_INT_CLEAR: c_uint;
    static TEGRA186_ASRC_GLOBAL_APR_CTRL: c_uint;
    static TEGRA186_ASRC_GLOBAL_APR_CTRL_ACCESS_CTRL: c_uint;
    static TEGRA186_ASRC_GLOBAL_DISARM_APR: c_uint;
    static TEGRA186_ASRC_GLOBAL_DISARM_APR_ACCESS_CTRL: c_uint;
    static TEGRA186_ASRC_GLOBAL_RATIO_WR_ACCESS: c_uint;
    static TEGRA186_ASRC_GLOBAL_RATIO_WR_ACCESS_CTRL: c_uint;
    static TEGRA186_ASRC_CYA: c_uint;
    static TEGRA186_ASRC_RATIO_LOCK_STATUS: c_uint;
    static TEGRA186_ASRC_GLOBAL_EN: c_uint;
    static TEGRA186_ASRC_STREAM_MAX: c_uint;
    static TEGRA186_ASRC_RATIO_SOURCE_SW: c_uint;
    static TEGRA186_ASRC_RATIO_SOURCE_ARAD: c_uint;
    static TEGRA186_ASRC_RX_THRESHOLD: c_uint;
    static TEGRA186_ASRC_TX_THRESHOLD: c_uint;
    static TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_MASK: c_uint;
    static TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_DISABLE: c_uint;
    static TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_ENABLE: c_uint;
    static TEGRA186_ASRC_RATIO_COMP: c_uint;
    static TEGRA186_ASRC_STREAM_DEFAULT_HW_COMP_BIAS_VALUE: c_uint;
    static TEGRA186_ASRC_STREAM_RATIO_TYPE_MASK: c_uint;
    static TEGRA186_ASRC_STREAM_RATIO_INT_PART_MASK: c_uint;
    static TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK: c_uint;
    static TEGRA186_ASRC_ENABLE: c_uint;
    static TEGRA186_ASRC_SOFT_RESET: c_uint;
    static TEGRA186_ASRC_STREAM_EN_SHIFT: c_uint;
    static TEGRA186_ASRC_STREAM_LIMIT: c_uint;
    static TEGRA186_ASRC_RX_STATUS: c_uint;
    static TEGRA186_ASRC_TX_STATUS: c_uint;
    static TEGRA186_ASRC_STATUS: c_uint;
    static TEGRA186_ASRC_OUTSAMPLEBUF_CFG: c_uint;
    static TEGRA186_ASRC_RATIO_UPD_RX_STATUS: c_uint;
    static TEGRA186_ASRC_GLOBAL_STATUS: c_uint;
    static TEGRA186_ASRC_GLOBAL_INT_STATUS: c_uint;
    static TEGRA186_ASRC_GLOBAL_TRANSFER_ERROR_LOG: c_uint;
    static TEGRA186_ASRC_GLOBAL_STREAM_ENABLE_STATUS: c_uint;
    static TEGRA186_ASRC_ARAM_START_ADDR: c_uint;
    static TEGRA264_ASRC_ARAM_START_ADDR: c_uint;
    static TEGRA186_ASRC_GLOBAL_CFG_FRAC_32BIT_PRECISION: c_uint;
    static TEGRA186_ASRC_STREAM_DEFAULT_INPUT_HW_COMP_THRESH_CFG: c_uint;
    static TEGRA186_ASRC_STREAM_DEFAULT_OUTPUT_HW_COMP_THRESH_CFG: c_uint;

    static TEGRA_ACIF_BITS_16: c_int;
    static TEGRA_ACIF_BITS_24: c_int;
    static TEGRA_ACIF_BITS_32: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAPM_POST_PMD: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static REGCACHE_FLAT: c_uint;
}

unsafe fn ASRC_STREAM_SOURCE_SELECT(id: c_uint) -> c_uint {
    TEGRA186_ASRC_CFG + id.wrapping_mul(TEGRA186_ASRC_STREAM_STRIDE)
}

unsafe fn ASRC_STREAM_REG(reg: c_uint, id: c_uint) -> c_uint {
    reg + id.wrapping_mul(TEGRA186_ASRC_STREAM_STRIDE)
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASRC_STREAM_REG_DEFAULTS {
    ($id:expr) => {
        reg_default { reg: ASRC_STREAM_REG(TEGRA186_ASRC_CFG, $id), def: (($id + 1) << 4) },
        reg_default { reg: ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, $id), def: 0x1 },
        reg_default { reg: ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, $id), def: 0x0 },
        reg_default { reg: ASRC_STREAM_REG(TEGRA186_ASRC_MUTE_UNMUTE_DURATION, $id), def: 0x400 },
        reg_default { reg: ASRC_STREAM_REG(TEGRA186_ASRC_RX_CIF_CTRL, $id), def: 0x7500 },
        reg_default { reg: ASRC_STREAM_REG(TEGRA186_ASRC_TX_CIF_CTRL, $id), def: 0x7500 }
    };
}

static mut tegra186_asrc_reg_defaults: [reg_default; 58] = unsafe {
    [
        ASRC_STREAM_REG_DEFAULTS!(0),
        ASRC_STREAM_REG_DEFAULTS!(1),
        ASRC_STREAM_REG_DEFAULTS!(2),
        ASRC_STREAM_REG_DEFAULTS!(3),
        ASRC_STREAM_REG_DEFAULTS!(4),
        ASRC_STREAM_REG_DEFAULTS!(5),
        reg_default { reg: TEGRA186_ASRC_GLOBAL_ENB, def: 0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_SOFT_RESET, def: 0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_CG, def: 0x1 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_CFG, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_SCRATCH_ADDR, def: 0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_SCRATCH_CFG, def: 0x0c207980 },
        reg_default { reg: TEGRA186_ASRC_RATIO_UPD_RX_CIF_CTRL, def: 0x00115500 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_INT_MASK, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_INT_SET, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_INT_CLEAR, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_APR_CTRL, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_APR_CTRL_ACCESS_CTRL, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_DISARM_APR, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_DISARM_APR_ACCESS_CTRL, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_RATIO_WR_ACCESS, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_GLOBAL_RATIO_WR_ACCESS_CTRL, def: 0x0 },
        reg_default { reg: TEGRA186_ASRC_CYA, def: 0x0 },
    ]
};

unsafe extern "C" fn tegra186_asrc_lock_stream(asrc: *mut tegra186_asrc, id: c_uint) {
    regmap_write(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_LOCK_STATUS, id),
        1,
    );
}

unsafe extern "C" fn tegra186_asrc_runtime_suspend(dev: *mut device) -> c_int {
    let asrc = dev_get_drvdata(dev) as *mut tegra186_asrc;

    regcache_cache_only((*asrc).regmap, true);
    regcache_mark_dirty((*asrc).regmap);

    0
}

unsafe extern "C" fn tegra186_asrc_runtime_resume(dev: *mut device) -> c_int {
    let asrc = dev_get_drvdata(dev) as *mut tegra186_asrc;
    let mut id: c_int;

    regcache_cache_only((*asrc).regmap, false);

    /*
     * Below sequence is recommended after a runtime PM cycle.
     * This otherwise leads to transfer failures. The cache
     * sync is done after this to restore other settings.
     */
    regmap_write(
        (*asrc).regmap,
        TEGRA186_ASRC_GLOBAL_SCRATCH_ADDR,
        (*(*asrc).soc_data).aram_start_addr,
    );
    regmap_write((*asrc).regmap, TEGRA186_ASRC_GLOBAL_ENB, TEGRA186_ASRC_GLOBAL_EN);

    regcache_sync((*asrc).regmap);

    id = 0;
    while id < TEGRA186_ASRC_STREAM_MAX as c_int {
        if (*asrc).lane[id as usize].ratio_source != TEGRA186_ASRC_RATIO_SOURCE_SW {
            id += 1;
            continue;
        }

        regmap_write(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, id as c_uint),
            (*asrc).lane[id as usize].int_part,
        );

        regmap_write(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, id as c_uint),
            (*asrc).lane[id as usize].frac_part,
        );

        tegra186_asrc_lock_stream(asrc, id as c_uint);
        id += 1;
    }

    0
}

unsafe extern "C" fn tegra186_asrc_set_audio_cif(
    asrc: *mut tegra186_asrc,
    params: *mut snd_pcm_hw_params,
    reg: c_uint,
) -> c_int {
    let channels: c_int;
    let audio_bits: c_int;
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();

    channels = params_channels(params);

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            audio_bits = TEGRA_ACIF_BITS_16;
        }
        x if x == SNDRV_PCM_FORMAT_S24_LE || x == SNDRV_PCM_FORMAT_S32_LE => {
            audio_bits = TEGRA_ACIF_BITS_32;
        }
        _ => return -EINVAL,
    }

    cif_conf.audio_ch = channels;
    cif_conf.client_ch = channels;
    cif_conf.audio_bits = audio_bits;
    cif_conf.client_bits = TEGRA_ACIF_BITS_24;

    tegra_set_cif((*asrc).regmap, reg, &mut cif_conf);

    0
}

unsafe extern "C" fn tegra186_asrc_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = (*dai).dev;
    let asrc = snd_soc_dai_get_drvdata(dai) as *mut tegra186_asrc;
    let mut ret: c_int;
    let id = (*dai).id;

    /* Set input threshold */
    regmap_write(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, (*dai).id as c_uint),
        (*asrc).lane[id as usize].input_thresh as c_uint,
    );

    ret = tegra186_asrc_set_audio_cif(
        asrc,
        params,
        ASRC_STREAM_REG(TEGRA186_ASRC_RX_CIF_CTRL, (*dai).id as c_uint),
    );
    if ret != 0 {
        dev_err(dev, cstr!("Can't set ASRC RX%d CIF: %d\n"), (*dai).id, ret);
        return ret;
    }

    ret
}

unsafe extern "C" fn tegra186_asrc_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = (*dai).dev;
    let asrc = snd_soc_dai_get_drvdata(dai) as *mut tegra186_asrc;
    let mut ret: c_int;
    let id = (*dai).id - 7;

    /* Set output threshold */
    regmap_write(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, id as c_uint),
        (*asrc).lane[id as usize].output_thresh as c_uint,
    );

    ret = tegra186_asrc_set_audio_cif(
        asrc,
        params,
        ASRC_STREAM_REG(TEGRA186_ASRC_TX_CIF_CTRL, id as c_uint),
    );
    if ret != 0 {
        dev_err(dev, cstr!("Can't set ASRC TX%d CIF: %d\n"), id, ret);
        return ret;
    }

    /* Set ENABLE_HW_RATIO_COMP */
    if (*asrc).lane[id as usize].hwcomp_disable != 0 {
        regmap_update_bits(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_CFG, id as c_uint),
            TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_MASK,
            TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_DISABLE,
        );
    } else {
        regmap_update_bits(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_CFG, id as c_uint),
            TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_MASK,
            TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_ENABLE,
        );

        regmap_write(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_COMP, id as c_uint),
            TEGRA186_ASRC_STREAM_DEFAULT_HW_COMP_BIAS_VALUE,
        );
    }

    /* Set lock */
    regmap_update_bits(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_CFG, id as c_uint),
        1,
        (*asrc).lane[id as usize].ratio_source,
    );

    if (*asrc).lane[id as usize].ratio_source == TEGRA186_ASRC_RATIO_SOURCE_SW {
        regmap_write(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, id as c_uint),
            (*asrc).lane[id as usize].int_part,
        );
        regmap_write(
            (*asrc).regmap,
            ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, id as c_uint),
            (*asrc).lane[id as usize].frac_part,
        );
        tegra186_asrc_lock_stream(asrc, id as c_uint);
    }

    ret
}

unsafe extern "C" fn tegra186_asrc_get_ratio_source(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_enum;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;

    (*ucontrol).value.enumerated.item[0] = (*asrc).lane[id as usize].ratio_source;

    0
}

unsafe extern "C" fn tegra186_asrc_put_ratio_source(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_enum;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;
    let mut change = false;

    (*asrc).lane[id as usize].ratio_source = (*ucontrol).value.enumerated.item[0];

    regmap_update_bits_check(
        (*asrc).regmap,
        (*asrc_private).reg,
        TEGRA186_ASRC_STREAM_RATIO_TYPE_MASK,
        (*asrc).lane[id as usize].ratio_source,
        &mut change,
    );

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra186_asrc_get_ratio_int(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;

    regmap_read(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, id),
        &mut (*asrc).lane[id as usize].int_part,
    );

    (*ucontrol).value.integer.value[0] = (*asrc).lane[id as usize].int_part as c_long;

    0
}

unsafe extern "C" fn tegra186_asrc_put_ratio_int(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;
    let mut change = false;

    if (*asrc).lane[id as usize].ratio_source == TEGRA186_ASRC_RATIO_SOURCE_ARAD {
        dev_err(
            (*cmpnt).dev,
            cstr!("Lane %d ratio source is ARAD, invalid SW update\n"),
            id,
        );
        return -EINVAL;
    }

    (*asrc).lane[id as usize].int_part = (*ucontrol).value.integer.value[0] as c_uint;

    regmap_update_bits_check(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, id),
        TEGRA186_ASRC_STREAM_RATIO_INT_PART_MASK,
        (*asrc).lane[id as usize].int_part,
        &mut change,
    );

    tegra186_asrc_lock_stream(asrc, id);

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra186_asrc_get_ratio_frac(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mreg_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).regbase / TEGRA186_ASRC_STREAM_STRIDE;

    regmap_read(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, id),
        &mut (*asrc).lane[id as usize].frac_part,
    );

    (*ucontrol).value.integer.value[0] = (*asrc).lane[id as usize].frac_part as c_long;

    0
}

unsafe extern "C" fn tegra186_asrc_put_ratio_frac(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mreg_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).regbase / TEGRA186_ASRC_STREAM_STRIDE;
    let mut change = false;

    if (*asrc).lane[id as usize].ratio_source == TEGRA186_ASRC_RATIO_SOURCE_ARAD {
        dev_err(
            (*cmpnt).dev,
            cstr!("Lane %d ratio source is ARAD, invalid SW update\n"),
            id,
        );
        return -EINVAL;
    }

    (*asrc).lane[id as usize].frac_part = (*ucontrol).value.integer.value[0] as c_uint;

    regmap_update_bits_check(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, id),
        TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK,
        (*asrc).lane[id as usize].frac_part,
        &mut change,
    );

    tegra186_asrc_lock_stream(asrc, id);

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra186_asrc_get_hwcomp_disable(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;

    (*ucontrol).value.integer.value[0] = (*asrc).lane[id as usize].hwcomp_disable as c_long;

    0
}

unsafe extern "C" fn tegra186_asrc_put_hwcomp_disable(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;
    let value = (*ucontrol).value.integer.value[0] as c_int;

    if value == (*asrc).lane[id as usize].hwcomp_disable {
        return 0;
    }

    (*asrc).lane[id as usize].hwcomp_disable = value;

    1
}

unsafe extern "C" fn tegra186_asrc_get_input_threshold(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;

    (*ucontrol).value.integer.value[0] = ((*asrc).lane[id as usize].input_thresh & 0x3) as c_long;

    0
}

unsafe extern "C" fn tegra186_asrc_put_input_threshold(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;
    let value =
        ((*asrc).lane[id as usize].input_thresh & !(0x3)) | (*ucontrol).value.integer.value[0] as c_int;

    if value == (*asrc).lane[id as usize].input_thresh {
        return 0;
    }

    (*asrc).lane[id as usize].input_thresh = value;

    1
}

unsafe extern "C" fn tegra186_asrc_get_output_threshold(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;

    (*ucontrol).value.integer.value[0] = ((*asrc).lane[id as usize].output_thresh & 0x3) as c_long;

    0
}

unsafe extern "C" fn tegra186_asrc_put_output_threshold(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let asrc_private = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let asrc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra186_asrc;
    let id = (*asrc_private).reg / TEGRA186_ASRC_STREAM_STRIDE;
    let value =
        ((*asrc).lane[id as usize].output_thresh & !(0x3)) | (*ucontrol).value.integer.value[0] as c_int;

    if value == (*asrc).lane[id as usize].output_thresh {
        return 0;
    }

    (*asrc).lane[id as usize].output_thresh = value;

    1
}

unsafe extern "C" fn tegra186_asrc_widget_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let asrc = dev_get_drvdata((*cmpnt).dev) as *mut tegra186_asrc;
    let id = ((*w).reg - TEGRA186_ASRC_ENABLE) / TEGRA186_ASRC_STREAM_STRIDE;

    regmap_write(
        (*asrc).regmap,
        ASRC_STREAM_REG(TEGRA186_ASRC_SOFT_RESET, id),
        0x1,
    );

    0
}

static tegra186_asrc_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra186_asrc_in_hw_params),
};

static tegra186_asrc_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra186_asrc_out_hw_params),
};

unsafe fn stream_formats() -> u64 {
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

macro_rules! IN_DAI {
    ($id:literal) => {
        snd_soc_dai_driver {
            name: cstr!(concat!("ASRC-RX-CIF", stringify!($id))),
            playback: snd_soc_pcm_stream {
                stream_name: cstr!(concat!("RX", stringify!($id), "-CIF-Playback")),
                channels_min: 1,
                channels_max: 12,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: stream_formats(),
            },
            capture: snd_soc_pcm_stream {
                stream_name: cstr!(concat!("RX", stringify!($id), "-CIF-Capture")),
                channels_min: 1,
                channels_max: 12,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: stream_formats(),
            },
            ops: &tegra186_asrc_in_dai_ops,
        }
    };
}

macro_rules! OUT_DAI {
    ($id:literal) => {
        snd_soc_dai_driver {
            name: cstr!(concat!("ASRC-TX-CIF", stringify!($id))),
            playback: snd_soc_pcm_stream {
                stream_name: cstr!(concat!("TX", stringify!($id), "-CIF-Playback")),
                channels_min: 1,
                channels_max: 12,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: stream_formats(),
            },
            capture: snd_soc_pcm_stream {
                stream_name: cstr!(concat!("TX", stringify!($id), "-CIF-Capture")),
                channels_min: 1,
                channels_max: 12,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: stream_formats(),
            },
            ops: &tegra186_asrc_out_dai_ops,
        }
    };
}

static mut tegra186_asrc_dais: [snd_soc_dai_driver; 13] = unsafe {
    [
        /* ASRC Input */
        IN_DAI!(1),
        IN_DAI!(2),
        IN_DAI!(3),
        IN_DAI!(4),
        IN_DAI!(5),
        IN_DAI!(6),
        IN_DAI!(7),
        /* ASRC Output */
        OUT_DAI!(1),
        OUT_DAI!(2),
        OUT_DAI!(3),
        OUT_DAI!(4),
        OUT_DAI!(5),
        OUT_DAI!(6),
    ]
};

macro_rules! SND_SOC_DAPM_AIF_IN {
    ($name:literal, $sname:expr, $wreg:expr, $reg:expr, $shift:expr, $invert:expr) => {
        snd_soc_dapm_widget_init {
            name: cstr!($name),
            sname: ptr::null(),
            reg: $reg,
            shift: $shift,
            invert: $invert,
            event: None,
            event_flags: 0,
        }
    };
}

macro_rules! SND_SOC_DAPM_AIF_OUT_E {
    ($name:literal, $sname:expr, $wreg:expr, $reg:expr, $shift:expr, $invert:expr, $event:expr, $flags:expr) => {
        snd_soc_dapm_widget_init {
            name: cstr!($name),
            sname: ptr::null(),
            reg: $reg,
            shift: $shift,
            invert: $invert,
            event: Some($event),
            event_flags: $flags,
        }
    };
}

macro_rules! SND_SOC_DAPM_SPK {
    ($name:literal, $event:expr) => {
        snd_soc_dapm_widget_init {
            name: cstr!($name),
            sname: ptr::null(),
            reg: 0,
            shift: 0,
            invert: 0,
            event: None,
            event_flags: 0,
        }
    };
}

static mut tegra186_asrc_widgets: [snd_soc_dapm_widget_item; 14] = unsafe {
    [
        SND_SOC_DAPM_AIF_IN!("RX1", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN!("RX2", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN!("RX3", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN!("RX4", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN!("RX5", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN!("RX6", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_IN!("RX7", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_AIF_OUT_E!("TX1", ptr::null(), 0, ASRC_STREAM_REG(TEGRA186_ASRC_ENABLE, 0), TEGRA186_ASRC_STREAM_EN_SHIFT, 0, tegra186_asrc_widget_event, SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_AIF_OUT_E!("TX2", ptr::null(), 0, ASRC_STREAM_REG(TEGRA186_ASRC_ENABLE, 1), TEGRA186_ASRC_STREAM_EN_SHIFT, 0, tegra186_asrc_widget_event, SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_AIF_OUT_E!("TX3", ptr::null(), 0, ASRC_STREAM_REG(TEGRA186_ASRC_ENABLE, 2), TEGRA186_ASRC_STREAM_EN_SHIFT, 0, tegra186_asrc_widget_event, SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_AIF_OUT_E!("TX4", ptr::null(), 0, ASRC_STREAM_REG(TEGRA186_ASRC_ENABLE, 3), TEGRA186_ASRC_STREAM_EN_SHIFT, 0, tegra186_asrc_widget_event, SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_AIF_OUT_E!("TX5", ptr::null(), 0, ASRC_STREAM_REG(TEGRA186_ASRC_ENABLE, 4), TEGRA186_ASRC_STREAM_EN_SHIFT, 0, tegra186_asrc_widget_event, SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_AIF_OUT_E!("TX6", ptr::null(), 0, ASRC_STREAM_REG(TEGRA186_ASRC_ENABLE, 5), TEGRA186_ASRC_STREAM_EN_SHIFT, 0, tegra186_asrc_widget_event, SND_SOC_DAPM_POST_PMD),
        SND_SOC_DAPM_SPK!("Depacketizer", ptr::null()),
    ]
};

macro_rules! route {
    ($sink:literal, $source:literal) => {
        snd_soc_dapm_route { sink: cstr!($sink), control: ptr::null(), source: cstr!($source) }
    };
}

macro_rules! ASRC_STREAM_ROUTE {
    ($id:literal, $sname:literal) => {
        route!(concat!("RX", stringify!($id), " XBAR-", $sname), concat!("RX", stringify!($id), " XBAR-TX")),
        route!(concat!("RX", stringify!($id), "-CIF-", $sname), concat!("RX", stringify!($id), " XBAR-", $sname)),
        route!(concat!("RX", stringify!($id)), concat!("RX", stringify!($id), "-CIF-", $sname)),
        route!(concat!("TX", stringify!($id)), concat!("RX", stringify!($id))),
        route!(concat!("TX", stringify!($id), "-CIF-", $sname), concat!("TX", stringify!($id))),
        route!(concat!("TX", stringify!($id), " XBAR-", $sname), concat!("TX", stringify!($id), "-CIF-", $sname)),
        route!(concat!("TX", stringify!($id), " XBAR-RX"), concat!("TX", stringify!($id), " XBAR-", $sname))
    };
}

macro_rules! ASRC_ROUTE {
    ($id:literal) => {
        ASRC_STREAM_ROUTE!($id, "Playback"),
        ASRC_STREAM_ROUTE!($id, "Capture")
    };
}

macro_rules! ASRC_RATIO_ROUTE {
    ($sname:literal) => {
        route!(concat!("RX7 XBAR-", $sname), "RX7 XBAR-TX"),
        route!(concat!("RX7-CIF-", $sname), concat!("RX7 XBAR-", $sname)),
        route!("RX7", concat!("RX7-CIF-", $sname)),
        route!("Depacketizer", "RX7")
    };
}

static tegra186_asrc_routes: [snd_soc_dapm_route; 92] = [
    ASRC_ROUTE!(1),
    ASRC_ROUTE!(2),
    ASRC_ROUTE!(3),
    ASRC_ROUTE!(4),
    ASRC_ROUTE!(5),
    ASRC_ROUTE!(6),
    ASRC_RATIO_ROUTE!("Playback"),
    ASRC_RATIO_ROUTE!("Capture"),
];

static tegra186_asrc_ratio_source_text: [*const c_char; 2] = [
    cstr!("ARAD"),
    cstr!("SW"),
];

macro_rules! ASRC_SOURCE_DECL {
    ($name:ident, $id:expr) => {
        static mut $name: soc_enum = unsafe { soc_enum { reg: ASRC_STREAM_SOURCE_SELECT($id) } };
    };
}

ASRC_SOURCE_DECL!(src_select1, 0);
ASRC_SOURCE_DECL!(src_select2, 1);
ASRC_SOURCE_DECL!(src_select3, 2);
ASRC_SOURCE_DECL!(src_select4, 3);
ASRC_SOURCE_DECL!(src_select5, 4);
ASRC_SOURCE_DECL!(src_select6, 5);

extern "C" {
    fn snd_soc_info_xr_sx() -> c_int;
}

macro_rules! SOC_SINGLE_EXT_CTL {
    ($xname:literal, $xreg:expr, $xget:expr, $xput:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: cstr!($xname),
            info: Some(snd_soc_info_xr_sx),
            get: Some($xget),
            put: Some($xput),
            private_value: &soc_mixer_control { reg: $xreg } as *const soc_mixer_control as c_ulong,
        }
    };
}

macro_rules! SOC_SINGLE_EXT_FRAC {
    ($xname:literal, $xregbase:expr, $xmax:expr, $xget:expr, $xput:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: cstr!($xname),
            info: Some(snd_soc_info_xr_sx),
            get: Some($xget),
            put: Some($xput),
            private_value: &soc_mreg_control {
                regbase: $xregbase,
                regcount: 1,
                nbits: 32,
                invert: 0,
                min: 0,
                max: $xmax,
            } as *const soc_mreg_control as c_ulong,
        }
    };
}

macro_rules! SOC_ENUM_EXT_CTL {
    ($xname:literal, $xenum:ident, $xget:expr, $xput:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: cstr!($xname),
            info: None,
            get: Some($xget),
            put: Some($xput),
            private_value: &$xenum as *const soc_enum as c_ulong,
        }
    };
}

static mut tegra186_asrc_controls: [snd_kcontrol_new; 36] = unsafe {
    [
        /* Controls for integer part of ratio */
        SOC_SINGLE_EXT_CTL!("Ratio1 Integer Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, 0), tegra186_asrc_get_ratio_int, tegra186_asrc_put_ratio_int),
        SOC_SINGLE_EXT_CTL!("Ratio2 Integer Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, 1), tegra186_asrc_get_ratio_int, tegra186_asrc_put_ratio_int),
        SOC_SINGLE_EXT_CTL!("Ratio3 Integer Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, 2), tegra186_asrc_get_ratio_int, tegra186_asrc_put_ratio_int),
        SOC_SINGLE_EXT_CTL!("Ratio4 Integer Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, 3), tegra186_asrc_get_ratio_int, tegra186_asrc_put_ratio_int),
        SOC_SINGLE_EXT_CTL!("Ratio5 Integer Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, 4), tegra186_asrc_get_ratio_int, tegra186_asrc_put_ratio_int),
        SOC_SINGLE_EXT_CTL!("Ratio6 Integer Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_INT_PART, 5), tegra186_asrc_get_ratio_int, tegra186_asrc_put_ratio_int),
        /* Controls for fractional part of ratio */
        SOC_SINGLE_EXT_FRAC!("Ratio1 Fractional Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, 0), TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK, tegra186_asrc_get_ratio_frac, tegra186_asrc_put_ratio_frac),
        SOC_SINGLE_EXT_FRAC!("Ratio2 Fractional Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, 1), TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK, tegra186_asrc_get_ratio_frac, tegra186_asrc_put_ratio_frac),
        SOC_SINGLE_EXT_FRAC!("Ratio3 Fractional Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, 2), TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK, tegra186_asrc_get_ratio_frac, tegra186_asrc_put_ratio_frac),
        SOC_SINGLE_EXT_FRAC!("Ratio4 Fractional Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, 3), TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK, tegra186_asrc_get_ratio_frac, tegra186_asrc_put_ratio_frac),
        SOC_SINGLE_EXT_FRAC!("Ratio5 Fractional Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, 4), TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK, tegra186_asrc_get_ratio_frac, tegra186_asrc_put_ratio_frac),
        SOC_SINGLE_EXT_FRAC!("Ratio6 Fractional Part", ASRC_STREAM_REG(TEGRA186_ASRC_RATIO_FRAC_PART, 5), TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK, tegra186_asrc_get_ratio_frac, tegra186_asrc_put_ratio_frac),
        /* Source of ratio provider */
        SOC_ENUM_EXT_CTL!("Ratio1 Source", src_select1, tegra186_asrc_get_ratio_source, tegra186_asrc_put_ratio_source),
        SOC_ENUM_EXT_CTL!("Ratio2 Source", src_select2, tegra186_asrc_get_ratio_source, tegra186_asrc_put_ratio_source),
        SOC_ENUM_EXT_CTL!("Ratio3 Source", src_select3, tegra186_asrc_get_ratio_source, tegra186_asrc_put_ratio_source),
        SOC_ENUM_EXT_CTL!("Ratio4 Source", src_select4, tegra186_asrc_get_ratio_source, tegra186_asrc_put_ratio_source),
        SOC_ENUM_EXT_CTL!("Ratio5 Source", src_select5, tegra186_asrc_get_ratio_source, tegra186_asrc_put_ratio_source),
        SOC_ENUM_EXT_CTL!("Ratio6 Source", src_select6, tegra186_asrc_get_ratio_source, tegra186_asrc_put_ratio_source),
        /* Disable HW managed overflow/underflow issue at input and output */
        SOC_SINGLE_EXT_CTL!("Stream1 HW Component Disable", ASRC_STREAM_REG(TEGRA186_ASRC_CFG, 0), tegra186_asrc_get_hwcomp_disable, tegra186_asrc_put_hwcomp_disable),
        SOC_SINGLE_EXT_CTL!("Stream2 HW Component Disable", ASRC_STREAM_REG(TEGRA186_ASRC_CFG, 1), tegra186_asrc_get_hwcomp_disable, tegra186_asrc_put_hwcomp_disable),
        SOC_SINGLE_EXT_CTL!("Stream3 HW Component Disable", ASRC_STREAM_REG(TEGRA186_ASRC_CFG, 2), tegra186_asrc_get_hwcomp_disable, tegra186_asrc_put_hwcomp_disable),
        SOC_SINGLE_EXT_CTL!("Stream4 HW Component Disable", ASRC_STREAM_REG(TEGRA186_ASRC_CFG, 3), tegra186_asrc_get_hwcomp_disable, tegra186_asrc_put_hwcomp_disable),
        SOC_SINGLE_EXT_CTL!("Stream5 HW Component Disable", ASRC_STREAM_REG(TEGRA186_ASRC_CFG, 4), tegra186_asrc_get_hwcomp_disable, tegra186_asrc_put_hwcomp_disable),
        SOC_SINGLE_EXT_CTL!("Stream6 HW Component Disable", ASRC_STREAM_REG(TEGRA186_ASRC_CFG, 5), tegra186_asrc_get_hwcomp_disable, tegra186_asrc_put_hwcomp_disable),
        /* Input threshold for watermark fields */
        SOC_SINGLE_EXT_CTL!("Stream1 Input Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, 0), tegra186_asrc_get_input_threshold, tegra186_asrc_put_input_threshold),
        SOC_SINGLE_EXT_CTL!("Stream2 Input Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, 1), tegra186_asrc_get_input_threshold, tegra186_asrc_put_input_threshold),
        SOC_SINGLE_EXT_CTL!("Stream3 Input Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, 2), tegra186_asrc_get_input_threshold, tegra186_asrc_put_input_threshold),
        SOC_SINGLE_EXT_CTL!("Stream4 Input Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, 3), tegra186_asrc_get_input_threshold, tegra186_asrc_put_input_threshold),
        SOC_SINGLE_EXT_CTL!("Stream5 Input Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, 4), tegra186_asrc_get_input_threshold, tegra186_asrc_put_input_threshold),
        SOC_SINGLE_EXT_CTL!("Stream6 Input Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_RX_THRESHOLD, 4), tegra186_asrc_get_input_threshold, tegra186_asrc_put_input_threshold),
        /* Output threshold for watermark fields */
        SOC_SINGLE_EXT_CTL!("Stream1 Output Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, 0), tegra186_asrc_get_output_threshold, tegra186_asrc_put_output_threshold),
        SOC_SINGLE_EXT_CTL!("Stream2 Output Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, 1), tegra186_asrc_get_output_threshold, tegra186_asrc_put_output_threshold),
        SOC_SINGLE_EXT_CTL!("Stream3 Output Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, 2), tegra186_asrc_get_output_threshold, tegra186_asrc_put_output_threshold),
        SOC_SINGLE_EXT_CTL!("Stream4 Output Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, 3), tegra186_asrc_get_output_threshold, tegra186_asrc_put_output_threshold),
        SOC_SINGLE_EXT_CTL!("Stream5 Output Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, 4), tegra186_asrc_get_output_threshold, tegra186_asrc_put_output_threshold),
        SOC_SINGLE_EXT_CTL!("Stream6 Output Threshold", ASRC_STREAM_REG(TEGRA186_ASRC_TX_THRESHOLD, 5), tegra186_asrc_get_output_threshold, tegra186_asrc_put_output_threshold),
    ]
};

static mut tegra186_asrc_cmpnt: snd_soc_component_driver = unsafe {
    snd_soc_component_driver {
        dapm_widgets: tegra186_asrc_widgets.as_ptr(),
        num_dapm_widgets: tegra186_asrc_widgets.len() as c_uint,
        dapm_routes: tegra186_asrc_routes.as_ptr(),
        num_dapm_routes: tegra186_asrc_routes.len() as c_uint,
        controls: tegra186_asrc_controls.as_ptr(),
        num_controls: tegra186_asrc_controls.len() as c_uint,
    }
};

unsafe extern "C" fn tegra186_asrc_wr_reg(_dev: *mut device, mut reg: c_uint) -> bool_ {
    if reg < TEGRA186_ASRC_STREAM_LIMIT {
        reg %= TEGRA186_ASRC_STREAM_STRIDE;
    }

    if (reg >= TEGRA186_ASRC_CFG && reg <= TEGRA186_ASRC_RATIO_COMP)
        || reg == TEGRA186_ASRC_RX_CIF_CTRL
        || reg == TEGRA186_ASRC_TX_CIF_CTRL
        || reg == TEGRA186_ASRC_ENABLE
        || reg == TEGRA186_ASRC_SOFT_RESET
        || (reg >= TEGRA186_ASRC_GLOBAL_ENB && reg <= TEGRA186_ASRC_RATIO_UPD_RX_CIF_CTRL)
        || (reg >= TEGRA186_ASRC_GLOBAL_INT_MASK && reg <= TEGRA186_ASRC_GLOBAL_INT_CLEAR)
        || (reg >= TEGRA186_ASRC_GLOBAL_APR_CTRL && reg <= TEGRA186_ASRC_CYA)
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra186_asrc_rd_reg(dev: *mut device, mut reg: c_uint) -> bool_ {
    if reg < TEGRA186_ASRC_STREAM_LIMIT {
        reg %= TEGRA186_ASRC_STREAM_STRIDE;
    }

    if tegra186_asrc_wr_reg(dev, reg) {
        return true;
    }

    if reg == TEGRA186_ASRC_RX_STATUS
        || reg == TEGRA186_ASRC_TX_STATUS
        || (reg >= TEGRA186_ASRC_STATUS && reg <= TEGRA186_ASRC_OUTSAMPLEBUF_CFG)
        || reg == TEGRA186_ASRC_RATIO_UPD_RX_STATUS
        || (reg >= TEGRA186_ASRC_GLOBAL_STATUS && reg <= TEGRA186_ASRC_GLOBAL_INT_STATUS)
        || reg == TEGRA186_ASRC_GLOBAL_TRANSFER_ERROR_LOG
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra186_asrc_volatile_reg(_dev: *mut device, mut reg: c_uint) -> bool_ {
    if reg < TEGRA186_ASRC_STREAM_LIMIT {
        reg %= TEGRA186_ASRC_STREAM_STRIDE;
    }

    match reg {
        x if x == TEGRA186_ASRC_RX_STATUS => true,
        x if x == TEGRA186_ASRC_TX_STATUS => true,
        x if x == TEGRA186_ASRC_SOFT_RESET => true,
        x if x == TEGRA186_ASRC_RATIO_INT_PART => true,
        x if x == TEGRA186_ASRC_RATIO_FRAC_PART => true,
        x if x == TEGRA186_ASRC_STATUS => true,
        x if x == TEGRA186_ASRC_RATIO_LOCK_STATUS => true,
        x if x == TEGRA186_ASRC_RATIO_UPD_RX_STATUS => true,
        x if x == TEGRA186_ASRC_GLOBAL_SOFT_RESET => true,
        x if x == TEGRA186_ASRC_GLOBAL_STATUS => true,
        x if x == TEGRA186_ASRC_GLOBAL_STREAM_ENABLE_STATUS => true,
        x if x == TEGRA186_ASRC_GLOBAL_INT_STATUS => true,
        x if x == TEGRA186_ASRC_GLOBAL_TRANSFER_ERROR_LOG => true,
        _ => false,
    }
}

static mut tegra186_asrc_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        reg_stride: 4,
        val_bits: 32,
        max_register: TEGRA186_ASRC_CYA,
        writeable_reg: Some(tegra186_asrc_wr_reg),
        readable_reg: Some(tegra186_asrc_rd_reg),
        volatile_reg: Some(tegra186_asrc_volatile_reg),
        reg_defaults: tegra186_asrc_reg_defaults.as_ptr(),
        num_reg_defaults: tegra186_asrc_reg_defaults.len() as c_uint,
        reg_default_cb: regmap_default_zero_cb,
        cache_type: REGCACHE_FLAT,
    }
};

static mut soc_data_tegra186: tegra_asrc_soc_data = unsafe {
    tegra_asrc_soc_data {
        aram_start_addr: TEGRA186_ASRC_ARAM_START_ADDR,
    }
};

static mut soc_data_tegra264: tegra_asrc_soc_data = unsafe {
    tegra_asrc_soc_data {
        aram_start_addr: TEGRA264_ASRC_ARAM_START_ADDR,
    }
};

static mut tegra186_asrc_of_match: [of_device_id; 3] = unsafe {
    [
        of_device_id {
            compatible: cstr!("nvidia,tegra186-asrc"),
            data: &soc_data_tegra186 as *const tegra_asrc_soc_data as *const c_void,
        },
        of_device_id {
            compatible: cstr!("nvidia,tegra264-asrc"),
            data: &soc_data_tegra264 as *const tegra_asrc_soc_data as *const c_void,
        },
        of_device_id {
            compatible: ptr::null(),
            data: ptr::null(),
        },
    ]
};
// MODULE_DEVICE_TABLE(of, tegra186_asrc_of_match);

unsafe extern "C" fn tegra186_asrc_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut asrc: *mut tegra186_asrc;
    let regs: *mut c_void;
    let mut i: c_uint;
    let err: c_int;

    asrc = devm_kzalloc(dev, size_of::<tegra186_asrc>(), GFP_KERNEL) as *mut tegra186_asrc;
    if asrc.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, asrc as *mut c_void);

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*asrc).regmap = devm_regmap_init_mmio(dev, regs, &tegra186_asrc_regmap_config);
    if IS_ERR((*asrc).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*asrc).regmap as *const c_void),
            cstr!("regmap init failed\n"),
        );
    }

    (*asrc).soc_data = of_device_get_match_data(&mut (*pdev).dev) as *const tegra_asrc_soc_data;

    regcache_cache_only((*asrc).regmap, true);

    regmap_write(
        (*asrc).regmap,
        TEGRA186_ASRC_GLOBAL_CFG,
        TEGRA186_ASRC_GLOBAL_CFG_FRAC_32BIT_PRECISION,
    );

    /* Initialize default output srate */
    i = 0;
    while i < TEGRA186_ASRC_STREAM_MAX {
        (*asrc).lane[i as usize].ratio_source = TEGRA186_ASRC_RATIO_SOURCE_SW;
        (*asrc).lane[i as usize].int_part = 1;
        (*asrc).lane[i as usize].frac_part = 0;
        (*asrc).lane[i as usize].hwcomp_disable = 0;
        (*asrc).lane[i as usize].input_thresh =
            TEGRA186_ASRC_STREAM_DEFAULT_INPUT_HW_COMP_THRESH_CFG as c_int;
        (*asrc).lane[i as usize].output_thresh =
            TEGRA186_ASRC_STREAM_DEFAULT_OUTPUT_HW_COMP_THRESH_CFG as c_int;
        i += 1;
    }

    err = devm_snd_soc_register_component(
        dev,
        &tegra186_asrc_cmpnt,
        tegra186_asrc_dais.as_mut_ptr(),
        tegra186_asrc_dais.len() as c_int,
    );
    if err != 0 {
        return dev_err_probe(dev, err, cstr!("can't register ASRC component\n"));
    }

    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn tegra186_asrc_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

// RUNTIME_PM_OPS(tegra186_asrc_runtime_suspend, tegra186_asrc_runtime_resume, NULL)
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
static tegra186_asrc_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut tegra186_asrc_driver: platform_driver = unsafe {
    platform_driver {
        driver: device_driver {
            name: cstr!("tegra186-asrc"),
            of_match_table: tegra186_asrc_of_match.as_ptr(),
            pm: pm_ptr(&tegra186_asrc_pm_ops),
        },
        probe: Some(tegra186_asrc_platform_probe),
        remove: Some(tegra186_asrc_platform_remove),
    }
};
// module_platform_driver(tegra186_asrc_driver)

// MODULE_AUTHOR("Junghyun Kim <juskim@nvidia.com>");
// MODULE_DESCRIPTION("Tegra186 ASRC ASoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
