// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2022-2024 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_ope.c - Tegra210 OPE driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cif_conf {
    pub audio_ch: c_int,
    pub client_ch: c_int,
    pub audio_bits: c_int,
    pub client_bits: c_int,
}

#[repr(C)]
pub struct tegra210_ope {
    pub regmap: *mut regmap,
    pub peq_regmap: *mut regmap,
    pub mbdrc_regmap: *mut regmap,
    pub peq_biquad_gains: *mut c_void,
    pub peq_biquad_shifts: *mut c_void,
    pub data_dir: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub invert: c_uint,
}

pub type c_uchar = u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: usize,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
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
    pub reg_default_cb: Option<unsafe extern "C" fn(*mut regmap, c_uint) -> c_uint>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
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
    static TEGRA210_OPE_RX_INT_MASK: c_uint;
    static TEGRA210_OPE_RX_CIF_CTRL: c_uint;
    static TEGRA210_OPE_TX_INT_MASK: c_uint;
    static TEGRA210_OPE_TX_CIF_CTRL: c_uint;
    static TEGRA210_OPE_CG: c_uint;
    static TEGRA210_OPE_ENABLE: c_uint;
    static TEGRA210_OPE_EN_SHIFT: c_uint;
    static TEGRA210_OPE_DIR: c_uint;
    static TEGRA210_OPE_DIR_SHIFT: c_uint;
    static TEGRA210_OPE_RX_STATUS: c_uint;
    static TEGRA210_OPE_RX_INT_STATUS: c_uint;
    static TEGRA210_OPE_TX_STATUS: c_uint;
    static TEGRA210_OPE_TX_INT_STATUS: c_uint;
    static TEGRA210_OPE_SOFT_RESET: c_uint;
    static TEGRA210_OPE_STATUS: c_uint;
    static TEGRA210_OPE_INT_STATUS: c_uint;

    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static TEGRA_ACIF_BITS_16: c_int;
    static TEGRA_ACIF_BITS_32: c_int;
    static SND_SOC_NOPM: c_int;
    static REGCACHE_FLAT: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn tegra_set_cif(regmap: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut tegra210_ope;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn tegra210_mbdrc_hw_params(component: *mut snd_soc_component);
    fn dev_get_drvdata(dev: *mut device) -> *mut tegra210_ope;
    fn tegra210_peq_component_init(cmpnt: *mut snd_soc_component);
    fn tegra210_mbdrc_component_init(cmpnt: *mut snd_soc_component);
    fn snd_soc_component_init_regmap(cmpnt: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut tegra210_ope;
    fn regmap_default_zero_cb(map: *mut regmap, reg: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn dev_set_drvdata(dev: *mut device, data: *mut tegra210_ope);
    fn tegra210_peq_regmap_init(pdev: *mut platform_device) -> c_int;
    fn tegra210_mbdrc_regmap_init(pdev: *mut platform_device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn tegra210_peq_save(regmap: *mut regmap, gains: *mut c_void, shifts: *mut c_void);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn tegra210_peq_restore(regmap: *mut regmap, gains: *mut c_void, shifts: *mut c_void);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static tegra210_ope_reg_defaults: [reg_default; 5] = unsafe {
    [
        reg_default { reg: TEGRA210_OPE_RX_INT_MASK, def: 0x00000001 },
        reg_default { reg: TEGRA210_OPE_RX_CIF_CTRL, def: 0x00007700 },
        reg_default { reg: TEGRA210_OPE_TX_INT_MASK, def: 0x00000001 },
        reg_default { reg: TEGRA210_OPE_TX_CIF_CTRL, def: 0x00007700 },
        reg_default { reg: TEGRA210_OPE_CG, def: 0x1 },
    ]
};

unsafe extern "C" fn tegra210_ope_set_audio_cif(
    ope: *mut tegra210_ope,
    params: *mut snd_pcm_hw_params,
    reg: c_uint,
) -> c_int {
    let mut channels: c_int;
    let mut audio_bits: c_int;
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();

    memset(
        &mut cif_conf as *mut tegra_cif_conf as *mut c_void,
        0,
        size_of::<tegra_cif_conf>(),
    );

    channels = params_channels(params);
    if channels < 2 {
        return -EINVAL;
    }

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            audio_bits = TEGRA_ACIF_BITS_16;
        }
        x if x == SNDRV_PCM_FORMAT_S24_LE || x == SNDRV_PCM_FORMAT_S32_LE => {
            audio_bits = TEGRA_ACIF_BITS_32;
        }
        _ => {
            return -EINVAL;
        }
    }

    cif_conf.audio_ch = channels;
    cif_conf.client_ch = channels;
    cif_conf.audio_bits = audio_bits;
    cif_conf.client_bits = audio_bits;

    tegra_set_cif((*ope).regmap, reg, &mut cif_conf);

    0
}

unsafe extern "C" fn tegra210_ope_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut device = (*dai).dev;
    let ope: *mut tegra210_ope = snd_soc_dai_get_drvdata(dai);
    let mut err: c_int;

    /* Set RX and TX CIF */
    err = tegra210_ope_set_audio_cif(ope, params, TEGRA210_OPE_RX_CIF_CTRL);
    if err != 0 {
        dev_err(dev, b"Can't set OPE RX CIF: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    err = tegra210_ope_set_audio_cif(ope, params, TEGRA210_OPE_TX_CIF_CTRL);
    if err != 0 {
        dev_err(dev, b"Can't set OPE TX CIF: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    tegra210_mbdrc_hw_params((*dai).component);

    err
}

unsafe extern "C" fn tegra210_ope_component_probe(cmpnt: *mut snd_soc_component) -> c_int {
    let ope: *mut tegra210_ope = dev_get_drvdata((*cmpnt).dev);

    tegra210_peq_component_init(cmpnt);
    tegra210_mbdrc_component_init(cmpnt);

    /*
     * The OPE, PEQ and MBDRC functionalities are combined under one
     * device registered by OPE driver. In fact OPE HW block includes
     * sub blocks PEQ and MBDRC. However driver registers separate
     * regmap interfaces for each of these. ASoC core depends on
     * dev_get_regmap() to populate the regmap field for a given ASoC
     * component. A component can have one regmap reference and since
     * the DAPM routes depend on OPE regmap only, below explicit
     * assignment is done to highlight this. This is needed for ASoC
     * core to access correct regmap during DAPM path setup.
     */
    snd_soc_component_init_regmap(cmpnt, (*ope).regmap);

    0
}

static tegra210_ope_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_ope_hw_params),
};

static mut tegra210_ope_dais: [snd_soc_dai_driver; 2] = unsafe {
    [
        snd_soc_dai_driver {
            name: b"OPE-RX-CIF\0".as_ptr() as *const c_char,
            playback: snd_soc_pcm_stream {
                stream_name: b"RX-CIF-Playback\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 8,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S24_LE
                    | SNDRV_PCM_FMTBIT_S32_LE,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"RX-CIF-Capture\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 8,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S24_LE
                    | SNDRV_PCM_FMTBIT_S32_LE,
            },
            ops: ptr::null(),
        },
        snd_soc_dai_driver {
            name: b"OPE-TX-CIF\0".as_ptr() as *const c_char,
            playback: snd_soc_pcm_stream {
                stream_name: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 8,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S24_LE
                    | SNDRV_PCM_FMTBIT_S32_LE,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: 8,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S24_LE
                    | SNDRV_PCM_FMTBIT_S32_LE,
            },
            ops: &tegra210_ope_dai_ops,
        },
    ]
};

static tegra210_ope_widgets: [snd_soc_dapm_widget; 2] = unsafe {
    [
        snd_soc_dapm_widget {
            id: 0,
            name: b"RX\0".as_ptr() as *const c_char,
            sname: ptr::null(),
            reg: SND_SOC_NOPM,
            shift: 0,
            invert: 0,
        },
        snd_soc_dapm_widget {
            id: 0,
            name: b"TX\0".as_ptr() as *const c_char,
            sname: ptr::null(),
            reg: TEGRA210_OPE_ENABLE as c_int,
            shift: TEGRA210_OPE_EN_SHIFT as c_uchar,
            invert: 0,
        },
    ]
};

// OPE_ROUTES(sname):
// { "RX XBAR-" sname, NULL, "XBAR-TX" },
// { "RX-CIF-" sname, NULL, "RX XBAR-" sname },
// { "RX", NULL, "RX-CIF-" sname },
// { "TX-CIF-" sname, NULL, "TX" },
// { "TX XBAR-" sname, NULL, "TX-CIF-" sname },
// { "XBAR-RX", NULL, "TX XBAR-" sname }
static tegra210_ope_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route {
        sink: b"TX\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"RX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX XBAR-Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"XBAR-TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX-CIF-Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"RX XBAR-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"RX-CIF-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX XBAR-Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"XBAR-RX\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TX XBAR-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX XBAR-Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"XBAR-TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX-CIF-Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"RX XBAR-Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"RX-CIF-Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX XBAR-Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"XBAR-RX\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"TX XBAR-Capture\0".as_ptr() as *const c_char,
    },
];

static tegra210_ope_data_dir_text: [*const c_char; 2] = [
    b"MBDRC to PEQ\0".as_ptr() as *const c_char,
    b"PEQ to MBDRC\0".as_ptr() as *const c_char,
];

static tegra210_ope_data_dir_enum: soc_enum = unsafe {
    soc_enum {
        reg: TEGRA210_OPE_DIR,
        shift_l: TEGRA210_OPE_DIR_SHIFT,
        items: 2,
        texts: tegra210_ope_data_dir_text.as_ptr(),
    }
};

unsafe extern "C" fn tegra210_ope_get_data_dir(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt);

    (*ucontrol).value.enumerated.item[0] = (*ope).data_dir;

    0
}

unsafe extern "C" fn tegra210_ope_put_data_dir(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let ope: *mut tegra210_ope = snd_soc_component_get_drvdata(cmpnt);
    let value: c_uint = (*ucontrol).value.enumerated.item[0];

    if value == (*ope).data_dir {
        return 0;
    }

    (*ope).data_dir = value;

    1
}

static tegra210_ope_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: b"Data Flow Direction\0".as_ptr() as *const c_char,
    private_value: &tegra210_ope_data_dir_enum as *const soc_enum as usize,
    get: Some(tegra210_ope_get_data_dir),
    put: Some(tegra210_ope_put_data_dir),
}];

static tegra210_ope_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tegra210_ope_component_probe),
    dapm_widgets: tegra210_ope_widgets.as_ptr(),
    num_dapm_widgets: array_size(&tegra210_ope_widgets),
    dapm_routes: tegra210_ope_routes.as_ptr(),
    num_dapm_routes: array_size(&tegra210_ope_routes),
    controls: tegra210_ope_controls.as_ptr(),
    num_controls: array_size(&tegra210_ope_controls),
};

unsafe extern "C" fn tegra210_ope_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg >= TEGRA210_OPE_RX_INT_MASK && reg <= TEGRA210_OPE_RX_CIF_CTRL {
        return true;
    }
    if reg >= TEGRA210_OPE_TX_INT_MASK && reg <= TEGRA210_OPE_TX_CIF_CTRL {
        return true;
    }
    if reg >= TEGRA210_OPE_ENABLE && reg <= TEGRA210_OPE_CG {
        return true;
    }
    if reg == TEGRA210_OPE_DIR {
        return true;
    }

    false
}

unsafe extern "C" fn tegra210_ope_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    if tegra210_ope_wr_reg(dev, reg) {
        return true;
    }

    if reg == TEGRA210_OPE_RX_STATUS
        || reg == TEGRA210_OPE_RX_INT_STATUS
        || reg == TEGRA210_OPE_TX_STATUS
        || reg == TEGRA210_OPE_TX_INT_STATUS
        || reg == TEGRA210_OPE_STATUS
        || reg == TEGRA210_OPE_INT_STATUS
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra210_ope_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg == TEGRA210_OPE_RX_STATUS
        || reg == TEGRA210_OPE_RX_INT_STATUS
        || reg == TEGRA210_OPE_TX_STATUS
        || reg == TEGRA210_OPE_TX_INT_STATUS
        || reg == TEGRA210_OPE_SOFT_RESET
        || reg == TEGRA210_OPE_STATUS
        || reg == TEGRA210_OPE_INT_STATUS
    {
        return true;
    }

    false
}

static tegra210_ope_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        reg_stride: 4,
        val_bits: 32,
        max_register: TEGRA210_OPE_DIR,
        writeable_reg: Some(tegra210_ope_wr_reg),
        readable_reg: Some(tegra210_ope_rd_reg),
        volatile_reg: Some(tegra210_ope_volatile_reg),
        reg_defaults: tegra210_ope_reg_defaults.as_ptr(),
        num_reg_defaults: array_size(&tegra210_ope_reg_defaults),
        reg_default_cb: Some(regmap_default_zero_cb),
        cache_type: REGCACHE_FLAT,
    }
};

unsafe extern "C" fn tegra210_ope_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let ope: *mut tegra210_ope;
    let regs: *mut c_void;
    let mut err: c_int;

    ope = devm_kzalloc(dev, size_of::<tegra210_ope>(), GFP_KERNEL) as *mut tegra210_ope;
    if ope.is_null() {
        return -ENOMEM;
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*ope).regmap = devm_regmap_init_mmio(dev, regs, &tegra210_ope_regmap_config);
    if IS_ERR((*ope).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*ope).regmap as *const c_void),
            b"regmap init failed\n\0".as_ptr() as *const c_char,
        );
    }

    regcache_cache_only((*ope).regmap, true);

    dev_set_drvdata(dev, ope);

    err = tegra210_peq_regmap_init(pdev);
    if err < 0 {
        return err;
    }

    err = tegra210_mbdrc_regmap_init(pdev);
    if err < 0 {
        return err;
    }

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_ope_cmpnt,
        tegra210_ope_dais.as_mut_ptr(),
        array_size(&tegra210_ope_dais) as c_int,
    );
    if err != 0 {
        return dev_err_probe(
            dev,
            err,
            b"can't register OPE component\n\0".as_ptr() as *const c_char,
        );
    }

    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn tegra210_ope_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn tegra210_ope_runtime_suspend(dev: *mut device) -> c_int {
    let ope: *mut tegra210_ope = dev_get_drvdata(dev);

    tegra210_peq_save(
        (*ope).peq_regmap,
        (*ope).peq_biquad_gains,
        (*ope).peq_biquad_shifts,
    );

    regcache_cache_only((*ope).mbdrc_regmap, true);
    regcache_cache_only((*ope).peq_regmap, true);
    regcache_cache_only((*ope).regmap, true);

    regcache_mark_dirty((*ope).regmap);
    regcache_mark_dirty((*ope).peq_regmap);
    regcache_mark_dirty((*ope).mbdrc_regmap);

    0
}

unsafe extern "C" fn tegra210_ope_runtime_resume(dev: *mut device) -> c_int {
    let ope: *mut tegra210_ope = dev_get_drvdata(dev);

    regcache_cache_only((*ope).regmap, false);
    regcache_cache_only((*ope).peq_regmap, false);
    regcache_cache_only((*ope).mbdrc_regmap, false);

    regcache_sync((*ope).regmap);
    regcache_sync((*ope).peq_regmap);
    regcache_sync((*ope).mbdrc_regmap);

    tegra210_peq_restore(
        (*ope).peq_regmap,
        (*ope).peq_biquad_gains,
        (*ope).peq_biquad_shifts,
    );

    0
}

static tegra210_ope_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(tegra210_ope_runtime_suspend),
    runtime_resume: Some(tegra210_ope_runtime_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static tegra210_ope_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nvidia,tegra210-ope\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, tegra210_ope_of_match);

static mut tegra210_ope_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"tegra210-ope\0".as_ptr() as *const c_char,
        of_match_table: tegra210_ope_of_match.as_ptr(),
        pm: &tegra210_ope_pm_ops,
    },
    probe: Some(tegra210_ope_probe),
    remove: Some(tegra210_ope_remove),
};

// module_platform_driver(tegra210_ope_driver)
#[no_mangle]
pub unsafe extern "C" fn tegra210_ope_driver_init() -> c_int {
    platform_driver_register(&mut tegra210_ope_driver)
}

#[no_mangle]
pub unsafe extern "C" fn tegra210_ope_driver_exit() {
    platform_driver_unregister(&mut tegra210_ope_driver);
}

// MODULE_AUTHOR("Sumit Bhattacharya <sumitb@nvidia.com>");
// MODULE_DESCRIPTION("Tegra210 OPE ASoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
