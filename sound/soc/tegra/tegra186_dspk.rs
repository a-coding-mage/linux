// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2020-2024 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//
// tegra186_dspk.c - Tegra186 DSPK driver
//
// Translated from C. Original include dependencies:
// <linux/clk.h>, <linux/device.h>, <linux/module.h>, <linux/of.h>,
// <linux/platform_device.h>, <linux/pm_runtime.h>, <linux/regmap.h>,
// <sound/core.h>, <sound/pcm_params.h>, <sound/soc.h>,
// "tegra186_dspk.h", "tegra_cif.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::zeroed;
use core::ptr::null;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[allow(non_camel_case_types)]
pub type c_long = i64;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
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
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub threshold: c_uint,
    pub audio_ch: c_uint,
    pub client_ch: c_uint,
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
    pub expand: c_uint,
    pub stereo_conv: c_uint,
    pub replicate: c_uint,
    pub truncate: c_uint,
    pub mono_conv: c_uint,
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
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
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
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
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
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub reg_default_cb: *const c_void,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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

#[repr(C)]
pub struct tegra186_dspk {
    pub regmap: *mut regmap,
    pub clk_dspk: *mut clk,
    pub rx_fifo_th: c_uint,
    pub osr_val: c_uint,
    pub lrsel: c_uint,
    pub ch_sel: c_uint,
    pub mono_to_stereo: c_uint,
    pub stereo_to_mono: c_uint,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
}

unsafe extern "C" {
    static regmap_default_zero_cb: c_void;
}

const TEGRA186_DSPK_RX_INT_MASK: c_uint = 0x0;
const TEGRA186_DSPK_RX_CIF_CTRL: c_uint = 0x24;
const TEGRA186_DSPK_CG: c_uint = 0x88;
const TEGRA186_DSPK_CORE_CTRL: c_uint = 0xa0;
const TEGRA186_DSPK_CODEC_CTRL: c_uint = 0xa4;
const TEGRA186_DSPK_ENABLE: c_uint = 0x80;
const TEGRA186_DSPK_RX_STATUS: c_uint = 0x0c;
const TEGRA186_DSPK_RX_INT_STATUS: c_uint = 0x10;
const TEGRA186_DSPK_STATUS: c_uint = 0x8c;
const TEGRA186_DSPK_INT_STATUS: c_uint = 0x90;
const TEGRA186_DSPK_RX_FIFO_DEPTH: c_uint = 64;
const TEGRA186_DSPK_OSR_MASK: c_uint = 0x3 << DSPK_OSR_SHIFT;
const TEGRA186_DSPK_CHANNEL_SELECT_MASK: c_uint = 0x3 << CH_SEL_SHIFT;
const TEGRA186_DSPK_CTRL_LRSEL_POLARITY_MASK: c_uint = 0x1 << LRSEL_POL_SHIFT;
const DSPK_OSR_SHIFT: c_uint = 8;
const CH_SEL_SHIFT: c_uint = 4;
const LRSEL_POL_SHIFT: c_uint = 0;
const DSPK_OSR_FACTOR: c_uint = 32;
const DSPK_CLK_RATIO: c_uint = 4;
const DSPK_OSR_64: c_uint = 1;
const DSPK_LRSEL_LEFT: c_uint = 0;
const DSPK_CH_SELECT_LEFT: c_uint = 0;
const DSPK_CH_SELECT_RIGHT: c_uint = 1;
const DSPK_CH_SELECT_STEREO: c_uint = 2;
const TEGRA_ACIF_BITS_16: c_uint = 3;
const TEGRA_ACIF_BITS_24: c_uint = 5;
const TEGRA_ACIF_BITS_32: c_uint = 7;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0x7fc;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << SNDRV_PCM_FORMAT_S32_LE;
const SND_SOC_NOPM: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const ENOMEM: c_int = 12;

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        ($array.len() as c_uint)
    };
}

macro_rules! SND_SOC_DAPM_AIF_IN {
    ($name:literal, $wname:expr, $slot:expr, $reg:expr, $shift:expr, $invert:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_SPK {
    ($name:literal, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

macro_rules! SOC_ENUM_SINGLE {
    ($reg:expr, $shift:expr, $items:expr, $texts:expr) => {
        soc_enum { _private: [] }
    };
}

macro_rules! SOC_SINGLE_EXT {
    ($name:literal, $reg:expr, $shift:expr, $max:expr, $invert:expr, $get:expr, $put:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SOC_ENUM_EXT {
    ($name:literal, $enum_:expr, $get:expr, $put:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! RUNTIME_PM_OPS {
    ($suspend:expr, $resume:expr, $idle:expr) => {
        dev_pm_ops { _private: [] }
    };
}

macro_rules! SYSTEM_SLEEP_PM_OPS {
    ($suspend:expr, $resume:expr) => {};
}

unsafe fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops {
    ops
}

static tegra186_dspk_reg_defaults: [reg_default; 5] = [
    reg_default {
        reg: TEGRA186_DSPK_RX_INT_MASK,
        def: 0x00000007,
    },
    reg_default {
        reg: TEGRA186_DSPK_RX_CIF_CTRL,
        def: 0x00007700,
    },
    reg_default {
        reg: TEGRA186_DSPK_CG,
        def: 0x00000001,
    },
    reg_default {
        reg: TEGRA186_DSPK_CORE_CTRL,
        def: 0x00000310,
    },
    reg_default {
        reg: TEGRA186_DSPK_CODEC_CTRL,
        def: 0x03000000,
    },
];

unsafe extern "C" fn tegra186_dspk_get_fifo_th(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;

    (*ucontrol).value.integer.value[0] = (*dspk).rx_fifo_th as c_long;

    0
}

unsafe extern "C" fn tegra186_dspk_put_fifo_th(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;
    let value = (*ucontrol).value.integer.value[0] as c_uint;

    if value == (*dspk).rx_fifo_th {
        return 0;
    }

    (*dspk).rx_fifo_th = value;

    1
}

unsafe extern "C" fn tegra186_dspk_get_osr_val(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;

    (*ucontrol).value.enumerated.item[0] = (*dspk).osr_val;

    0
}

unsafe extern "C" fn tegra186_dspk_put_osr_val(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;
    let value = (*ucontrol).value.enumerated.item[0];

    if value == (*dspk).osr_val {
        return 0;
    }

    (*dspk).osr_val = value;

    1
}

unsafe extern "C" fn tegra186_dspk_get_pol_sel(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;

    (*ucontrol).value.enumerated.item[0] = (*dspk).lrsel;

    0
}

unsafe extern "C" fn tegra186_dspk_put_pol_sel(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;
    let value = (*ucontrol).value.enumerated.item[0];

    if value == (*dspk).lrsel {
        return 0;
    }

    (*dspk).lrsel = value;

    1
}

unsafe extern "C" fn tegra186_dspk_get_ch_sel(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;

    (*ucontrol).value.enumerated.item[0] = (*dspk).ch_sel;

    0
}

unsafe extern "C" fn tegra186_dspk_put_ch_sel(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;
    let value = (*ucontrol).value.enumerated.item[0];

    if value == (*dspk).ch_sel {
        return 0;
    }

    (*dspk).ch_sel = value;

    1
}

unsafe extern "C" fn tegra186_dspk_get_mono_to_stereo(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;

    (*ucontrol).value.enumerated.item[0] = (*dspk).mono_to_stereo;

    0
}

unsafe extern "C" fn tegra186_dspk_put_mono_to_stereo(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;
    let value = (*ucontrol).value.enumerated.item[0];

    if value == (*dspk).mono_to_stereo {
        return 0;
    }

    (*dspk).mono_to_stereo = value;

    1
}

unsafe extern "C" fn tegra186_dspk_get_stereo_to_mono(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;

    (*ucontrol).value.enumerated.item[0] = (*dspk).stereo_to_mono;

    0
}

unsafe extern "C" fn tegra186_dspk_put_stereo_to_mono(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let dspk = snd_soc_component_get_drvdata(codec) as *mut tegra186_dspk;
    let value = (*ucontrol).value.enumerated.item[0];

    if value == (*dspk).stereo_to_mono {
        return 0;
    }

    (*dspk).stereo_to_mono = value;

    1
}

unsafe extern "C" fn tegra186_dspk_runtime_suspend(dev: *mut device) -> c_int {
    let dspk = dev_get_drvdata(dev) as *mut tegra186_dspk;

    regcache_cache_only((*dspk).regmap, true);
    regcache_mark_dirty((*dspk).regmap);

    clk_disable_unprepare((*dspk).clk_dspk);

    0
}

unsafe extern "C" fn tegra186_dspk_runtime_resume(dev: *mut device) -> c_int {
    let dspk = dev_get_drvdata(dev) as *mut tegra186_dspk;
    let err: c_int;

    err = clk_prepare_enable((*dspk).clk_dspk);
    if err != 0 {
        dev_err(
            dev,
            c_str!("failed to enable DSPK clock, err: %d\n"),
            err,
        );
        return err;
    }

    regcache_cache_only((*dspk).regmap, false);
    regcache_sync((*dspk).regmap);

    0
}

unsafe extern "C" fn tegra186_dspk_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dspk = snd_soc_dai_get_drvdata(dai) as *mut tegra186_dspk;
    let channels: c_uint;
    let srate: c_uint;
    let dspk_clk: c_uint;
    let dev = (*dai).dev;
    let mut cif_conf: tegra_cif_conf;
    let max_th: c_uint;
    let err: c_int;

    cif_conf = zeroed();

    channels = params_channels(params);
    cif_conf.audio_ch = channels;

    /* Client channel */
    match (*dspk).ch_sel {
        DSPK_CH_SELECT_LEFT | DSPK_CH_SELECT_RIGHT => {
            cif_conf.client_ch = 1;
        }
        DSPK_CH_SELECT_STEREO => {
            cif_conf.client_ch = 2;
        }
        _ => {
            dev_err(dev, c_str!("Invalid DSPK client channels\n"));
            return -EINVAL;
        }
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_16;
            cif_conf.client_bits = TEGRA_ACIF_BITS_16;
        }
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S32_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_32;
            cif_conf.client_bits = TEGRA_ACIF_BITS_24;
        }
        _ => {
            dev_err(dev, c_str!("unsupported format!\n"));
            return -EOPNOTSUPP;
        }
    }

    srate = params_rate(params);

    /* RX FIFO threshold in terms of frames */
    max_th = (TEGRA186_DSPK_RX_FIFO_DEPTH / cif_conf.audio_ch).wrapping_sub(1);

    if (*dspk).rx_fifo_th > max_th {
        (*dspk).rx_fifo_th = max_th;
    }

    cif_conf.threshold = (*dspk).rx_fifo_th;
    cif_conf.mono_conv = (*dspk).mono_to_stereo;
    cif_conf.stereo_conv = (*dspk).stereo_to_mono;

    tegra_set_cif((*dspk).regmap, TEGRA186_DSPK_RX_CIF_CTRL, &mut cif_conf);

    /*
     * DSPK clock and PDM codec clock should be synchronous with 4:1 ratio,
     * this is because it takes 4 clock cycles to send out one sample to
     * codec by sigma delta modulator. Finally the clock rate is a multiple
     * of 'Over Sampling Ratio', 'Sample Rate' and 'Interface Clock Ratio'.
     */
    dspk_clk = (DSPK_OSR_FACTOR << (*dspk).osr_val)
        .wrapping_mul(srate)
        .wrapping_mul(DSPK_CLK_RATIO);

    err = clk_set_rate((*dspk).clk_dspk, dspk_clk);
    if err != 0 {
        dev_err(
            dev,
            c_str!("can't set DSPK clock rate %u, err: %d\n"),
            dspk_clk,
            err,
        );

        return err;
    }

    regmap_update_bits(
        (*dspk).regmap,
        /* Reg */
        TEGRA186_DSPK_CORE_CTRL,
        /* Mask */
        TEGRA186_DSPK_OSR_MASK
            | TEGRA186_DSPK_CHANNEL_SELECT_MASK
            | TEGRA186_DSPK_CTRL_LRSEL_POLARITY_MASK,
        /* Value */
        ((*dspk).osr_val << DSPK_OSR_SHIFT)
            | (((*dspk).ch_sel + 1) << CH_SEL_SHIFT)
            | ((*dspk).lrsel << LRSEL_POL_SHIFT),
    );

    0
}

static tegra186_dspk_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra186_dspk_hw_params),
};

static mut tegra186_dspk_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c_str!("DSPK-CIF"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("CIF-Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: null(),
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: c_str!("DSPK-DAP"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DAP-Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &tegra186_dspk_dai_ops,
        symmetric_rate: 1,
    },
];

static tegra186_dspk_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_AIF_IN!("RX", null(), 0, TEGRA186_DSPK_ENABLE, 0, 0),
    SND_SOC_DAPM_SPK!("SPK", null()),
];

static tegra186_dspk_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: c_str!("XBAR-Playback"),
        control: null(),
        source: c_str!("XBAR-TX"),
    },
    snd_soc_dapm_route {
        sink: c_str!("CIF-Playback"),
        control: null(),
        source: c_str!("XBAR-Playback"),
    },
    snd_soc_dapm_route {
        sink: c_str!("RX"),
        control: null(),
        source: c_str!("CIF-Playback"),
    },
    snd_soc_dapm_route {
        sink: c_str!("DAP-Playback"),
        control: null(),
        source: c_str!("RX"),
    },
    snd_soc_dapm_route {
        sink: c_str!("SPK"),
        control: null(),
        source: c_str!("DAP-Playback"),
    },
];

static tegra186_dspk_ch_sel_text: [*const c_char; 3] =
    [c_str!("Left"), c_str!("Right"), c_str!("Stereo")];

static tegra186_dspk_ch_sel_enum: soc_enum = SOC_ENUM_SINGLE!(
    SND_SOC_NOPM,
    0,
    ARRAY_SIZE!(tegra186_dspk_ch_sel_text),
    tegra186_dspk_ch_sel_text
);

static tegra186_dspk_osr_text: [*const c_char; 4] = [
    c_str!("OSR_32"),
    c_str!("OSR_64"),
    c_str!("OSR_128"),
    c_str!("OSR_256"),
];

static tegra186_dspk_osr_enum: soc_enum = SOC_ENUM_SINGLE!(
    SND_SOC_NOPM,
    0,
    ARRAY_SIZE!(tegra186_dspk_osr_text),
    tegra186_dspk_osr_text
);

static tegra186_dspk_lrsel_text: [*const c_char; 2] = [c_str!("Left"), c_str!("Right")];

static tegra186_dspk_mono_conv_text: [*const c_char; 2] = [c_str!("Zero"), c_str!("Copy")];

static tegra186_dspk_mono_conv_enum: soc_enum = SOC_ENUM_SINGLE!(
    SND_SOC_NOPM,
    0,
    ARRAY_SIZE!(tegra186_dspk_mono_conv_text),
    tegra186_dspk_mono_conv_text
);

static tegra186_dspk_stereo_conv_text: [*const c_char; 3] =
    [c_str!("CH0"), c_str!("CH1"), c_str!("AVG")];

static tegra186_dspk_stereo_conv_enum: soc_enum = SOC_ENUM_SINGLE!(
    SND_SOC_NOPM,
    0,
    ARRAY_SIZE!(tegra186_dspk_stereo_conv_text),
    tegra186_dspk_stereo_conv_text
);

static tegra186_dspk_lrsel_enum: soc_enum = SOC_ENUM_SINGLE!(
    SND_SOC_NOPM,
    0,
    ARRAY_SIZE!(tegra186_dspk_lrsel_text),
    tegra186_dspk_lrsel_text
);

static tegrat186_dspk_controls: [snd_kcontrol_new; 6] = [
    SOC_SINGLE_EXT!(
        "FIFO Threshold",
        SND_SOC_NOPM,
        0,
        TEGRA186_DSPK_RX_FIFO_DEPTH - 1,
        0,
        tegra186_dspk_get_fifo_th,
        tegra186_dspk_put_fifo_th
    ),
    SOC_ENUM_EXT!(
        "OSR Value",
        tegra186_dspk_osr_enum,
        tegra186_dspk_get_osr_val,
        tegra186_dspk_put_osr_val
    ),
    SOC_ENUM_EXT!(
        "LR Polarity Select",
        tegra186_dspk_lrsel_enum,
        tegra186_dspk_get_pol_sel,
        tegra186_dspk_put_pol_sel
    ),
    SOC_ENUM_EXT!(
        "Channel Select",
        tegra186_dspk_ch_sel_enum,
        tegra186_dspk_get_ch_sel,
        tegra186_dspk_put_ch_sel
    ),
    SOC_ENUM_EXT!(
        "Mono To Stereo",
        tegra186_dspk_mono_conv_enum,
        tegra186_dspk_get_mono_to_stereo,
        tegra186_dspk_put_mono_to_stereo
    ),
    SOC_ENUM_EXT!(
        "Stereo To Mono",
        tegra186_dspk_stereo_conv_enum,
        tegra186_dspk_get_stereo_to_mono,
        tegra186_dspk_put_stereo_to_mono
    ),
];

static tegra186_dspk_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra186_dspk_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(tegra186_dspk_widgets),
    dapm_routes: tegra186_dspk_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(tegra186_dspk_routes),
    controls: tegrat186_dspk_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(tegrat186_dspk_controls),
};

unsafe extern "C" fn tegra186_dspk_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA186_DSPK_RX_INT_MASK..=TEGRA186_DSPK_RX_CIF_CTRL
        | TEGRA186_DSPK_ENABLE..=TEGRA186_DSPK_CG
        | TEGRA186_DSPK_CORE_CTRL..=TEGRA186_DSPK_CODEC_CTRL => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra186_dspk_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    if tegra186_dspk_wr_reg(dev, reg) {
        return true;
    }

    match reg {
        TEGRA186_DSPK_RX_STATUS
        | TEGRA186_DSPK_RX_INT_STATUS
        | TEGRA186_DSPK_STATUS
        | TEGRA186_DSPK_INT_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra186_dspk_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA186_DSPK_RX_STATUS
        | TEGRA186_DSPK_RX_INT_STATUS
        | TEGRA186_DSPK_STATUS
        | TEGRA186_DSPK_INT_STATUS => true,
        _ => false,
    }
}

static tegra186_dspk_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA186_DSPK_CODEC_CTRL,
    writeable_reg: Some(tegra186_dspk_wr_reg),
    readable_reg: Some(tegra186_dspk_rd_reg),
    volatile_reg: Some(tegra186_dspk_volatile_reg),
    reg_defaults: tegra186_dspk_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(tegra186_dspk_reg_defaults),
    reg_default_cb: unsafe { &regmap_default_zero_cb as *const c_void },
    cache_type: REGCACHE_FLAT,
};

static tegra186_dspk_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("nvidia,tegra186-dspk"),
    },
    of_device_id { compatible: null() },
];

// MODULE_DEVICE_TABLE(of, tegra186_dspk_of_match);

unsafe extern "C" fn tegra186_dspk_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let dspk: *mut tegra186_dspk;
    let regs: *mut c_void;
    let err: c_int;

    dspk = devm_kzalloc(dev, core::mem::size_of::<tegra186_dspk>(), GFP_KERNEL) as *mut tegra186_dspk;
    if dspk.is_null() {
        return -ENOMEM;
    }

    (*dspk).osr_val = DSPK_OSR_64;
    (*dspk).lrsel = DSPK_LRSEL_LEFT;
    (*dspk).ch_sel = DSPK_CH_SELECT_STEREO;
    (*dspk).mono_to_stereo = 0; /* "Zero" */

    dev_set_drvdata(dev, dspk as *mut c_void);

    (*dspk).clk_dspk = devm_clk_get(dev, c_str!("dspk"));
    if IS_ERR((*dspk).clk_dspk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*dspk).clk_dspk as *const c_void),
            c_str!("can't retrieve DSPK clock\n"),
        );
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*dspk).regmap = devm_regmap_init_mmio(dev, regs, &tegra186_dspk_regmap);
    if IS_ERR((*dspk).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*dspk).regmap as *const c_void),
            c_str!("regmap init failed\n"),
        );
    }

    regcache_cache_only((*dspk).regmap, true);

    err = devm_snd_soc_register_component(
        dev,
        &tegra186_dspk_cmpnt,
        tegra186_dspk_dais.as_mut_ptr(),
        ARRAY_SIZE!(tegra186_dspk_dais) as c_int,
    );
    if err != 0 {
        return dev_err_probe(dev, err, c_str!("can't register DSPK component\n"));
    }

    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn tegra186_dspk_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev as *mut device);
}

static tegra186_dspk_pm_ops: dev_pm_ops = RUNTIME_PM_OPS!(
    tegra186_dspk_runtime_suspend,
    tegra186_dspk_runtime_resume,
    null()
);
SYSTEM_SLEEP_PM_OPS!(pm_runtime_force_suspend, pm_runtime_force_resume);

static mut tegra186_dspk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("tegra186-dspk"),
        of_match_table: tegra186_dspk_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&tegra186_dspk_pm_ops) },
    },
    probe: Some(tegra186_dspk_platform_probe),
    remove: Some(tegra186_dspk_platform_remove),
};

// module_platform_driver(tegra186_dspk_driver);

// MODULE_AUTHOR("Mohan Kumar <mkumard@nvidia.com>");
// MODULE_AUTHOR("Sameer Pujar <spujar@nvidia.com>");
// MODULE_DESCRIPTION("Tegra186 ASoC DSPK driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
