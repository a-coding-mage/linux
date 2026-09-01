// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2021-2024 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_mvc.c - Tegra210 MVC driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::zeroed;
use core::ptr::{addr_of_mut, null, null_mut};

type bool_t = bool;
type u8 = u8;
type u32 = u32;
type s32 = i32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_mvc_gain_params {
    pub poly_coeff: [s32; NUM_GAIN_POLY_COEFFS as usize],
    pub poly_n1: c_uint,
    pub poly_n2: c_uint,
    pub duration: c_uint,
    pub duration_inv: c_uint,
}

#[repr(C)]
pub struct tegra210_mvc {
    pub regmap: *mut regmap,
    pub volume: [s32; TEGRA210_MVC_MAX_CHAN_COUNT as usize],
    pub curve_type: c_uint,
    pub ctrl_value: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cif_conf {
    pub audio_ch: c_uint,
    pub client_ch: c_uint,
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_enum {
    pub reg: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub reg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub soc_enum: *const soc_enum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: c_uint,
    pub invert: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub reg_default_cb: *const c_void,
    pub cache_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend: *const c_void,
    pub resume: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

extern "C" {
    static regmap_default_zero_cb: c_void;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
        change: *mut bool,
    ) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
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
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

extern "C" {
    static pm_runtime_force_suspend: c_void;
    static pm_runtime_force_resume: c_void;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

// Constants and register macros are supplied by tegra210_mvc.h, tegra_cif.h,
// and Linux/ASoC headers in the final repository.
extern "Rust" {
    static TEGRA210_MVC_RX_INT_MASK: c_uint;
    static TEGRA210_MVC_RX_CIF_CTRL: c_uint;
    static TEGRA210_MVC_TX_INT_MASK: c_uint;
    static TEGRA210_MVC_TX_CIF_CTRL: c_uint;
    static TEGRA210_MVC_CG: c_uint;
    static TEGRA210_MVC_CTRL: c_uint;
    static TEGRA210_MVC_CTRL_DEFAULT: c_uint;
    static TEGRA210_MVC_INIT_VOL: c_uint;
    static TEGRA210_MVC_TARGET_VOL: c_uint;
    static TEGRA210_MVC_DURATION: c_uint;
    static TEGRA210_MVC_DURATION_INV: c_uint;
    static TEGRA210_MVC_POLY_N1: c_uint;
    static TEGRA210_MVC_POLY_N2: c_uint;
    static TEGRA210_MVC_PEAK_CTRL: c_uint;
    static TEGRA210_MVC_CFG_RAM_CTRL: c_uint;
    static TEGRA210_MVC_CFG_RAM_DATA: c_uint;
    static TEGRA210_MVC_CFG_RAM_CTRL_SEQ_ACCESS_EN: c_uint;
    static TEGRA210_MVC_CFG_RAM_CTRL_ADDR_INIT_EN: c_uint;
    static TEGRA210_MVC_CFG_RAM_CTRL_RW_WRITE: c_uint;
    static TEGRA210_MVC_SWITCH: c_uint;
    static TEGRA210_MVC_VOLUME_SWITCH_MASK: c_uint;
    static TEGRA210_MVC_VOLUME_SWITCH_TRIGGER: c_uint;
    static TEGRA210_MVC_PER_CHAN_CTRL_EN: c_uint;
    static TEGRA210_MVC_PER_CHAN_CTRL_EN_MASK: c_uint;
    static TEGRA210_MVC_CH0_MUTE_EN: c_uint;
    static TEGRA210_MUTE_MASK_EN: c_uint;
    static TEGRA210_MVC_MUTE_MASK: c_uint;
    static TEGRA210_MVC_MUTE_SHIFT: c_uint;
    static CURVE_POLY: c_uint;
    static CURVE_LINEAR: c_uint;
    static TEGRA210_MVC_INIT_VOL_DEFAULT_POLY: s32;
    static TEGRA210_MVC_INIT_VOL_DEFAULT_LINEAR: s32;
    static TEGRA210_MVC_MAX_CHAN_COUNT: c_uint;
    static TEGRA210_MVC_CURVE_TYPE_MASK: c_uint;
    static TEGRA210_MVC_CURVE_TYPE_SHIFT: c_uint;
    static TEGRA210_MVC_ENABLE: c_uint;
    static TEGRA210_MVC_EN: c_uint;
    static TEGRA210_MVC_EN_SHIFT: c_uint;
    static TEGRA210_MVC_SOFT_RESET: c_uint;
    static TEGRA210_MVC_RX_STATUS: c_uint;
    static TEGRA210_MVC_CONFIG_ERR_TYPE: c_uint;
    static TEGRA210_MVC_TX_STATUS: c_uint;
    static TEGRA210_MVC_STATUS: c_uint;
    static TEGRA210_MVC_INT_STATUS: c_uint;
    static TEGRA210_MVC_RX_INT_STATUS: c_uint;
    static TEGRA210_MVC_RX_INT_SET: c_uint;
    static TEGRA210_MVC_TX_INT_STATUS: c_uint;
    static TEGRA210_MVC_TX_INT_SET: c_uint;
    static TEGRA210_MVC_PEAK_VALUE: c_uint;
    static TEGRA_ACIF_BITS_8: c_uint;
    static TEGRA_ACIF_BITS_16: c_uint;
    static TEGRA_ACIF_BITS_32: c_uint;
    static SNDRV_PCM_FORMAT_S8: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_LE: c_uint;
    static SNDRV_PCM_FORMAT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SND_SOC_NOPM: c_int;
    static GFP_KERNEL: c_uint;
    static REGCACHE_FLAT: c_uint;
    fn TEGRA210_GET_MUTE_VAL(val: c_uint) -> u8;
    fn TEGRA210_MVC_GET_CHAN(reg: c_uint, base: c_uint) -> u8;
    fn TEGRA210_MVC_REG_OFFSET(base: c_uint, chan: c_int) -> c_uint;
}

const NUM_GAIN_POLY_COEFFS: c_uint = 9;

static tegra210_mvc_reg_defaults: [reg_default; 14] = unsafe {
    [
        reg_default { reg: TEGRA210_MVC_RX_INT_MASK, def: 0x00000001 },
        reg_default { reg: TEGRA210_MVC_RX_CIF_CTRL, def: 0x00007700 },
        reg_default { reg: TEGRA210_MVC_TX_INT_MASK, def: 0x00000001 },
        reg_default { reg: TEGRA210_MVC_TX_CIF_CTRL, def: 0x00007700 },
        reg_default { reg: TEGRA210_MVC_CG, def: 0x1 },
        reg_default { reg: TEGRA210_MVC_CTRL, def: TEGRA210_MVC_CTRL_DEFAULT },
        reg_default { reg: TEGRA210_MVC_INIT_VOL, def: 0x00800000 },
        reg_default { reg: TEGRA210_MVC_TARGET_VOL, def: 0x00800000 },
        reg_default { reg: TEGRA210_MVC_DURATION, def: 0x000012c0 },
        reg_default { reg: TEGRA210_MVC_DURATION_INV, def: 0x0006d3a0 },
        reg_default { reg: TEGRA210_MVC_POLY_N1, def: 0x0000007d },
        reg_default { reg: TEGRA210_MVC_POLY_N2, def: 0x00000271 },
        reg_default { reg: TEGRA210_MVC_PEAK_CTRL, def: 0x000012c0 },
        reg_default { reg: TEGRA210_MVC_CFG_RAM_CTRL, def: 0x00004000 },
    ]
};

static gain_params: tegra210_mvc_gain_params = tegra210_mvc_gain_params {
    poly_coeff: [
        23738319, 659403, -3680, 15546680, 2530732, -120985, 12048422, 5527252,
        -785042,
    ],
    poly_n1: 16,
    poly_n2: 63,
    duration: 150,
    duration_inv: 14316558,
};

unsafe extern "C" fn tegra210_mvc_runtime_suspend(dev: *mut device) -> c_int {
    let mvc = dev_get_drvdata(dev) as *mut tegra210_mvc;

    regmap_read((*mvc).regmap, TEGRA210_MVC_CTRL, addr_of_mut!((*mvc).ctrl_value));

    regcache_cache_only((*mvc).regmap, true);
    regcache_mark_dirty((*mvc).regmap);

    0
}

unsafe extern "C" fn tegra210_mvc_runtime_resume(dev: *mut device) -> c_int {
    let mvc = dev_get_drvdata(dev) as *mut tegra210_mvc;

    regcache_cache_only((*mvc).regmap, false);
    regcache_sync((*mvc).regmap);

    regmap_write((*mvc).regmap, TEGRA210_MVC_CTRL, (*mvc).ctrl_value);
    regmap_update_bits(
        (*mvc).regmap,
        TEGRA210_MVC_SWITCH,
        TEGRA210_MVC_VOLUME_SWITCH_MASK,
        TEGRA210_MVC_VOLUME_SWITCH_TRIGGER,
    );

    0
}

unsafe fn tegra210_mvc_write_ram(regmap: *mut regmap) {
    let mut i: c_int;

    regmap_write(
        regmap,
        TEGRA210_MVC_CFG_RAM_CTRL,
        TEGRA210_MVC_CFG_RAM_CTRL_SEQ_ACCESS_EN
            | TEGRA210_MVC_CFG_RAM_CTRL_ADDR_INIT_EN
            | TEGRA210_MVC_CFG_RAM_CTRL_RW_WRITE,
    );

    i = 0;
    while i < NUM_GAIN_POLY_COEFFS as c_int {
        regmap_write(
            regmap,
            TEGRA210_MVC_CFG_RAM_DATA,
            gain_params.poly_coeff[i as usize] as c_uint,
        );
        i += 1;
    }
}

unsafe fn tegra210_mvc_conv_vol(mvc: *mut tegra210_mvc, chan: u8, mut val: s32) {
    /*
     * Volume control read from mixer control is with
     * 100x scaling; for CURVE_POLY the reg range
     * is 0-100 (linear, Q24) and for CURVE_LINEAR
     * it is -120dB to +40dB (Q8)
     */
    if (*mvc).curve_type == CURVE_POLY {
        if val > 10000 {
            val = 10000;
        }
        (*mvc).volume[chan as usize] = ((val * (1 << 8)) / 100) << 16;
    } else {
        val -= 12000;
        (*mvc).volume[chan as usize] = (val * (1 << 8)) / 100;
    }
}

unsafe fn tegra210_mvc_get_ctrl_reg(kcontrol: *mut snd_kcontrol) -> u32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;
    let mut val: u32 = 0;

    pm_runtime_get_sync((*cmpnt).dev);
    regmap_read((*mvc).regmap, TEGRA210_MVC_CTRL, &mut val);
    pm_runtime_put((*cmpnt).dev);

    val
}

unsafe extern "C" fn tegra210_mvc_get_mute(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let val = tegra210_mvc_get_ctrl_reg(kcontrol);
    let mute_mask = TEGRA210_GET_MUTE_VAL(val);

    /*
     * If per channel control is enabled, then return
     * exact mute/unmute setting of all channels.
     *
     * Else report setting based on CH0 bit to reflect
     * the correct HW state.
     */
    if val & TEGRA210_MVC_PER_CHAN_CTRL_EN != 0 {
        (*ucontrol).value.integer.value[0] = mute_mask as i64;
    } else if mute_mask & TEGRA210_MVC_CH0_MUTE_EN as u8 != 0 {
        (*ucontrol).value.integer.value[0] = TEGRA210_MUTE_MASK_EN as i64;
    } else {
        (*ucontrol).value.integer.value[0] = 0;
    }

    0
}

unsafe extern "C" fn tegra210_mvc_get_master_mute(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let val = tegra210_mvc_get_ctrl_reg(kcontrol);
    let mute_mask = TEGRA210_GET_MUTE_VAL(val);

    /*
     * If per channel control is disabled, then return
     * master mute/unmute setting based on CH0 bit.
     *
     * Else report settings based on state of all
     * channels.
     */
    if !(val & TEGRA210_MVC_PER_CHAN_CTRL_EN != 0) {
        (*ucontrol).value.integer.value[0] = (mute_mask & TEGRA210_MVC_CH0_MUTE_EN as u8) as i64;
    } else if mute_mask == TEGRA210_MUTE_MASK_EN as u8 {
        (*ucontrol).value.integer.value[0] = TEGRA210_MVC_CH0_MUTE_EN as i64;
    } else {
        (*ucontrol).value.integer.value[0] = 0;
    }

    0
}

unsafe fn tegra210_mvc_volume_switch_timeout(cmpnt: *mut snd_soc_component) -> c_int {
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;
    let mut value: u32 = 0;
    let mut err: c_int;

    err = 0;
    loop {
        regmap_read((*mvc).regmap, TEGRA210_MVC_SWITCH, &mut value);
        if !(value & TEGRA210_MVC_VOLUME_SWITCH_MASK != 0) {
            break;
        }
        err = -1;
        break;
    }
    if err < 0 {
        dev_err(
            (*cmpnt).dev,
            b"Volume switch trigger is still active, err = %d\n\0".as_ptr() as *const c_char,
            err,
        );
    }

    err
}

unsafe fn tegra210_mvc_update_mute(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    per_chan_ctrl: bool,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;
    let mut mute_val = (*ucontrol).value.integer.value[0] as u32;
    let per_ch_ctrl_val: u32;
    let mut change = false;
    let mut err: c_int;

    pm_runtime_get_sync((*cmpnt).dev);

    err = tegra210_mvc_volume_switch_timeout(cmpnt);
    if err < 0 {
        pm_runtime_put((*cmpnt).dev);
        return err;
    }

    if per_chan_ctrl {
        per_ch_ctrl_val = TEGRA210_MVC_PER_CHAN_CTRL_EN;
    } else {
        per_ch_ctrl_val = 0;

        if mute_val != 0 {
            mute_val = TEGRA210_MUTE_MASK_EN;
        }
    }

    regmap_update_bits_check(
        (*mvc).regmap,
        TEGRA210_MVC_CTRL,
        TEGRA210_MVC_MUTE_MASK,
        mute_val << TEGRA210_MVC_MUTE_SHIFT,
        &mut change,
    );

    if change {
        regmap_update_bits(
            (*mvc).regmap,
            TEGRA210_MVC_CTRL,
            TEGRA210_MVC_PER_CHAN_CTRL_EN_MASK,
            per_ch_ctrl_val,
        );

        regmap_update_bits(
            (*mvc).regmap,
            TEGRA210_MVC_SWITCH,
            TEGRA210_MVC_VOLUME_SWITCH_MASK,
            TEGRA210_MVC_VOLUME_SWITCH_TRIGGER,
        );
    }

    pm_runtime_put((*cmpnt).dev);

    if err < 0 {
        return err;
    }

    if change {
        return 1;
    }

    0
}

unsafe extern "C" fn tegra210_mvc_put_mute(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mvc_update_mute(kcontrol, ucontrol, true)
}

unsafe extern "C" fn tegra210_mvc_put_master_mute(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mvc_update_mute(kcontrol, ucontrol, false)
}

unsafe extern "C" fn tegra210_mvc_get_vol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;
    let chan = TEGRA210_MVC_GET_CHAN((*mc).reg, TEGRA210_MVC_TARGET_VOL);
    let mut val = (*mvc).volume[chan as usize];

    if (*mvc).curve_type == CURVE_POLY {
        val = ((val >> 16) * 100) >> 8;
    } else {
        val = (val * 100) >> 8;
        val += 12000;
    }

    (*ucontrol).value.integer.value[0] = val as i64;

    0
}

unsafe extern "C" fn tegra210_mvc_get_master_vol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mvc_get_vol(kcontrol, ucontrol)
}

unsafe fn tegra210_mvc_update_vol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    per_ch_enable: bool,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;
    let chan = TEGRA210_MVC_GET_CHAN((*mc).reg, TEGRA210_MVC_TARGET_VOL);
    let old_volume = (*mvc).volume[chan as usize];
    let mut err: c_int;
    let mut i: c_int;

    pm_runtime_get_sync((*cmpnt).dev);

    err = tegra210_mvc_volume_switch_timeout(cmpnt);
    if err < 0 {
        pm_runtime_put((*cmpnt).dev);
        return err;
    }

    tegra210_mvc_conv_vol(mvc, chan, (*ucontrol).value.integer.value[0] as s32);

    if (*mvc).volume[chan as usize] == old_volume {
        err = 0;
        pm_runtime_put((*cmpnt).dev);
        return err;
    }

    if per_ch_enable {
        regmap_update_bits(
            (*mvc).regmap,
            TEGRA210_MVC_CTRL,
            TEGRA210_MVC_PER_CHAN_CTRL_EN_MASK,
            TEGRA210_MVC_PER_CHAN_CTRL_EN,
        );
    } else {
        regmap_update_bits(
            (*mvc).regmap,
            TEGRA210_MVC_CTRL,
            TEGRA210_MVC_PER_CHAN_CTRL_EN_MASK,
            0,
        );

        i = 1;
        while i < TEGRA210_MVC_MAX_CHAN_COUNT as c_int {
            (*mvc).volume[i as usize] = (*mvc).volume[chan as usize];
            i += 1;
        }
    }

    /* Configure init volume same as target volume */
    regmap_write(
        (*mvc).regmap,
        TEGRA210_MVC_REG_OFFSET(TEGRA210_MVC_INIT_VOL, chan as c_int),
        (*mvc).volume[chan as usize] as c_uint,
    );

    regmap_write((*mvc).regmap, (*mc).reg, (*mvc).volume[chan as usize] as c_uint);

    regmap_update_bits(
        (*mvc).regmap,
        TEGRA210_MVC_SWITCH,
        TEGRA210_MVC_VOLUME_SWITCH_MASK,
        TEGRA210_MVC_VOLUME_SWITCH_TRIGGER,
    );

    err = 1;

    pm_runtime_put((*cmpnt).dev);

    err
}

unsafe extern "C" fn tegra210_mvc_put_vol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mvc_update_vol(kcontrol, ucontrol, true)
}

unsafe extern "C" fn tegra210_mvc_put_master_vol(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mvc_update_vol(kcontrol, ucontrol, false)
}

unsafe fn tegra210_mvc_reset_vol_settings(mvc: *mut tegra210_mvc, dev: *mut device) {
    let mut i: c_int;

    /* Change volume to default init for new curve type */
    if (*mvc).curve_type == CURVE_POLY {
        i = 0;
        while i < TEGRA210_MVC_MAX_CHAN_COUNT as c_int {
            (*mvc).volume[i as usize] = TEGRA210_MVC_INIT_VOL_DEFAULT_POLY;
            i += 1;
        }
    } else {
        i = 0;
        while i < TEGRA210_MVC_MAX_CHAN_COUNT as c_int {
            (*mvc).volume[i as usize] = TEGRA210_MVC_INIT_VOL_DEFAULT_LINEAR;
            i += 1;
        }
    }

    pm_runtime_get_sync(dev);

    /* Program curve type */
    regmap_update_bits(
        (*mvc).regmap,
        TEGRA210_MVC_CTRL,
        TEGRA210_MVC_CURVE_TYPE_MASK,
        (*mvc).curve_type << TEGRA210_MVC_CURVE_TYPE_SHIFT,
    );

    /* Init volume for all channels */
    i = 0;
    while i < TEGRA210_MVC_MAX_CHAN_COUNT as c_int {
        regmap_write(
            (*mvc).regmap,
            TEGRA210_MVC_REG_OFFSET(TEGRA210_MVC_INIT_VOL, i),
            (*mvc).volume[i as usize] as c_uint,
        );
        regmap_write(
            (*mvc).regmap,
            TEGRA210_MVC_REG_OFFSET(TEGRA210_MVC_TARGET_VOL, i),
            (*mvc).volume[i as usize] as c_uint,
        );
        i += 1;
    }

    /* Trigger volume switch */
    regmap_update_bits(
        (*mvc).regmap,
        TEGRA210_MVC_SWITCH,
        TEGRA210_MVC_VOLUME_SWITCH_MASK,
        TEGRA210_MVC_VOLUME_SWITCH_TRIGGER,
    );

    pm_runtime_put(dev);
}

unsafe extern "C" fn tegra210_mvc_get_curve_type(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;

    (*ucontrol).value.enumerated.item[0] = (*mvc).curve_type;

    0
}

unsafe extern "C" fn tegra210_mvc_put_curve_type(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mvc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mvc;
    let mut value: c_uint = 0;

    regmap_read((*mvc).regmap, TEGRA210_MVC_ENABLE, &mut value);
    if value & TEGRA210_MVC_EN != 0 {
        dev_err(
            (*cmpnt).dev,
            b"Curve type can't be set when MVC is running\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    if (*mvc).curve_type == (*ucontrol).value.enumerated.item[0] {
        return 0;
    }

    (*mvc).curve_type = (*ucontrol).value.enumerated.item[0];

    tegra210_mvc_reset_vol_settings(mvc, (*cmpnt).dev);

    1
}

unsafe fn tegra210_mvc_set_audio_cif(
    mvc: *mut tegra210_mvc,
    params: *mut snd_pcm_hw_params,
    reg: c_uint,
) -> c_int {
    let channels: c_uint;
    let audio_bits: c_uint;
    let mut cif_conf: tegra_cif_conf = zeroed();

    channels = params_channels(params);

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S8 => {
            audio_bits = TEGRA_ACIF_BITS_8;
        }
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

    tegra_set_cif((*mvc).regmap, reg, &mut cif_conf);

    0
}

unsafe extern "C" fn tegra210_mvc_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = (*dai).dev;
    let mvc = snd_soc_dai_get_drvdata(dai) as *mut tegra210_mvc;
    let mut err: c_int;
    let mut val: c_uint = 0;

    /*
     * Soft Reset: Below performs module soft reset which clears
     * all FSM logic, flushes flow control of FIFO and resets the
     * state register. It also brings module back to disabled
     * state (without flushing the data in the pipe).
     */
    regmap_write((*mvc).regmap, TEGRA210_MVC_SOFT_RESET, 1);

    err = 0;
    regmap_read((*mvc).regmap, TEGRA210_MVC_SOFT_RESET, &mut val);
    if val != 0 {
        err = -1;
    }
    if err < 0 {
        dev_err(dev, b"SW reset failed, err = %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    /* Set RX CIF */
    err = tegra210_mvc_set_audio_cif(mvc, params, TEGRA210_MVC_RX_CIF_CTRL);
    if err != 0 {
        dev_err(dev, b"Can't set MVC RX CIF: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    /* Set TX CIF */
    err = tegra210_mvc_set_audio_cif(mvc, params, TEGRA210_MVC_TX_CIF_CTRL);
    if err != 0 {
        dev_err(dev, b"Can't set MVC TX CIF: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    tegra210_mvc_write_ram((*mvc).regmap);

    /* Program poly_n1, poly_n2, duration */
    regmap_write((*mvc).regmap, TEGRA210_MVC_POLY_N1, gain_params.poly_n1);
    regmap_write((*mvc).regmap, TEGRA210_MVC_POLY_N2, gain_params.poly_n2);
    regmap_write((*mvc).regmap, TEGRA210_MVC_DURATION, gain_params.duration);

    /* Program duration_inv */
    regmap_write(
        (*mvc).regmap,
        TEGRA210_MVC_DURATION_INV,
        gain_params.duration_inv,
    );

    0
}

static tegra210_mvc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_mvc_hw_params),
};

static tegra210_mvc_curve_type_text: [*const c_char; 2] = [
    b"Poly\0".as_ptr() as *const c_char,
    b"Linear\0".as_ptr() as *const c_char,
];

static tegra210_mvc_curve_type_ctrl: soc_enum = soc_enum {
    reg: 2,
    items: 2,
    texts: tegra210_mvc_curve_type_text.as_ptr(),
};

unsafe fn SOC_SINGLE_EXT(
    name: *const c_char,
    reg: c_uint,
    shift: c_uint,
    max: c_uint,
    invert: c_uint,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        name,
        reg,
        shift,
        max,
        invert,
        get,
        put,
        soc_enum: null(),
    }
}

unsafe fn SOC_ENUM_EXT(
    name: *const c_char,
    soc_enum: *const soc_enum,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        name,
        reg: 0,
        shift: 0,
        max: 0,
        invert: 0,
        get,
        put,
        soc_enum,
    }
}

// TEGRA210_MVC_VOL_CTRL(chan)
unsafe fn TEGRA210_MVC_VOL_CTRL(chan: c_int) -> snd_kcontrol_new {
    let name = match chan {
        1 => b"Channel1 Volume\0".as_ptr() as *const c_char,
        2 => b"Channel2 Volume\0".as_ptr() as *const c_char,
        3 => b"Channel3 Volume\0".as_ptr() as *const c_char,
        4 => b"Channel4 Volume\0".as_ptr() as *const c_char,
        5 => b"Channel5 Volume\0".as_ptr() as *const c_char,
        6 => b"Channel6 Volume\0".as_ptr() as *const c_char,
        7 => b"Channel7 Volume\0".as_ptr() as *const c_char,
        _ => b"Channel8 Volume\0".as_ptr() as *const c_char,
    };
    SOC_SINGLE_EXT(
        name,
        TEGRA210_MVC_REG_OFFSET(TEGRA210_MVC_TARGET_VOL, chan - 1),
        0,
        16000,
        0,
        Some(tegra210_mvc_get_vol),
        Some(tegra210_mvc_put_vol),
    )
}

static tegra210_mvc_vol_ctrl: [snd_kcontrol_new; 13] = unsafe {
    [
        /* Per channel volume control */
        TEGRA210_MVC_VOL_CTRL(1),
        TEGRA210_MVC_VOL_CTRL(2),
        TEGRA210_MVC_VOL_CTRL(3),
        TEGRA210_MVC_VOL_CTRL(4),
        TEGRA210_MVC_VOL_CTRL(5),
        TEGRA210_MVC_VOL_CTRL(6),
        TEGRA210_MVC_VOL_CTRL(7),
        TEGRA210_MVC_VOL_CTRL(8),
        /* Per channel mute */
        SOC_SINGLE_EXT(
            b"Per Chan Mute Mask\0".as_ptr() as *const c_char,
            TEGRA210_MVC_CTRL,
            0,
            TEGRA210_MUTE_MASK_EN,
            0,
            Some(tegra210_mvc_get_mute),
            Some(tegra210_mvc_put_mute),
        ),
        /* Master volume */
        SOC_SINGLE_EXT(
            b"Volume\0".as_ptr() as *const c_char,
            TEGRA210_MVC_TARGET_VOL,
            0,
            16000,
            0,
            Some(tegra210_mvc_get_master_vol),
            Some(tegra210_mvc_put_master_vol),
        ),
        /* Master mute */
        SOC_SINGLE_EXT(
            b"Mute\0".as_ptr() as *const c_char,
            TEGRA210_MVC_CTRL,
            0,
            1,
            0,
            Some(tegra210_mvc_get_master_mute),
            Some(tegra210_mvc_put_master_mute),
        ),
        SOC_ENUM_EXT(
            b"Curve Type\0".as_ptr() as *const c_char,
            &tegra210_mvc_curve_type_ctrl,
            Some(tegra210_mvc_get_curve_type),
            Some(tegra210_mvc_put_curve_type),
        ),
    ]
};

static mut tegra210_mvc_dais: [snd_soc_dai_driver; 2] = unsafe {
    [
        /* Input */
        snd_soc_dai_driver {
            name: b"MVC-RX-CIF\0".as_ptr() as *const c_char,
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
            ops: null(),
        },
        /* Output */
        snd_soc_dai_driver {
            name: b"MVC-TX-CIF\0".as_ptr() as *const c_char,
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
            ops: &tegra210_mvc_dai_ops,
        },
    ]
};

static tegra210_mvc_widgets: [snd_soc_dapm_widget; 2] = unsafe {
    [
        snd_soc_dapm_widget {
            name: b"RX\0".as_ptr() as *const c_char,
            sname: null(),
            reg: SND_SOC_NOPM,
            shift: 0,
            invert: 0,
        },
        snd_soc_dapm_widget {
            name: b"TX\0".as_ptr() as *const c_char,
            sname: null(),
            reg: TEGRA210_MVC_ENABLE as c_int,
            shift: TEGRA210_MVC_EN_SHIFT,
            invert: 0,
        },
    ]
};

// MVC_ROUTES(sname)
static tegra210_mvc_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route {
        sink: b"TX\0".as_ptr() as *const c_char,
        control: null(),
        source: b"RX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX XBAR-Playback\0".as_ptr() as *const c_char,
        control: null(),
        source: b"XBAR-TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX-CIF-Playback\0".as_ptr() as *const c_char,
        control: null(),
        source: b"RX XBAR-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX\0".as_ptr() as *const c_char,
        control: null(),
        source: b"RX-CIF-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
        control: null(),
        source: b"TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX XBAR-Playback\0".as_ptr() as *const c_char,
        control: null(),
        source: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"XBAR-RX\0".as_ptr() as *const c_char,
        control: null(),
        source: b"TX XBAR-Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX XBAR-Capture\0".as_ptr() as *const c_char,
        control: null(),
        source: b"XBAR-TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX-CIF-Capture\0".as_ptr() as *const c_char,
        control: null(),
        source: b"RX XBAR-Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RX\0".as_ptr() as *const c_char,
        control: null(),
        source: b"RX-CIF-Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
        control: null(),
        source: b"TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"TX XBAR-Capture\0".as_ptr() as *const c_char,
        control: null(),
        source: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"XBAR-RX\0".as_ptr() as *const c_char,
        control: null(),
        source: b"TX XBAR-Capture\0".as_ptr() as *const c_char,
    },
];

static tegra210_mvc_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra210_mvc_widgets.as_ptr(),
    num_dapm_widgets: tegra210_mvc_widgets.len() as c_uint,
    dapm_routes: tegra210_mvc_routes.as_ptr(),
    num_dapm_routes: tegra210_mvc_routes.len() as c_uint,
    controls: tegra210_mvc_vol_ctrl.as_ptr(),
    num_controls: tegra210_mvc_vol_ctrl.len() as c_uint,
};

unsafe extern "C" fn tegra210_mvc_rd_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    if reg >= TEGRA210_MVC_RX_STATUS && reg <= TEGRA210_MVC_CONFIG_ERR_TYPE {
        true
    } else {
        false
    }
}

unsafe extern "C" fn tegra210_mvc_wr_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    if (reg >= TEGRA210_MVC_RX_INT_MASK && reg <= TEGRA210_MVC_RX_CIF_CTRL)
        || (reg >= TEGRA210_MVC_TX_INT_MASK && reg <= TEGRA210_MVC_TX_CIF_CTRL)
        || (reg >= TEGRA210_MVC_ENABLE && reg <= TEGRA210_MVC_CG)
        || (reg >= TEGRA210_MVC_CTRL && reg <= TEGRA210_MVC_CFG_RAM_DATA)
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn tegra210_mvc_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    if reg == TEGRA210_MVC_RX_STATUS
        || reg == TEGRA210_MVC_RX_INT_STATUS
        || reg == TEGRA210_MVC_RX_INT_SET
        || reg == TEGRA210_MVC_TX_STATUS
        || reg == TEGRA210_MVC_TX_INT_STATUS
        || reg == TEGRA210_MVC_TX_INT_SET
        || reg == TEGRA210_MVC_SOFT_RESET
        || reg == TEGRA210_MVC_STATUS
        || reg == TEGRA210_MVC_INT_STATUS
        || reg == TEGRA210_MVC_SWITCH
        || reg == TEGRA210_MVC_CFG_RAM_CTRL
        || reg == TEGRA210_MVC_CFG_RAM_DATA
        || reg == TEGRA210_MVC_PEAK_VALUE
        || reg == TEGRA210_MVC_CTRL
    {
        true
    } else {
        false
    }
}

static tegra210_mvc_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        reg_stride: 4,
        val_bits: 32,
        max_register: TEGRA210_MVC_CONFIG_ERR_TYPE,
        writeable_reg: Some(tegra210_mvc_wr_reg),
        readable_reg: Some(tegra210_mvc_rd_reg),
        volatile_reg: Some(tegra210_mvc_volatile_reg),
        reg_defaults: tegra210_mvc_reg_defaults.as_ptr(),
        num_reg_defaults: tegra210_mvc_reg_defaults.len() as c_uint,
        reg_default_cb: &regmap_default_zero_cb as *const c_void,
        cache_type: REGCACHE_FLAT,
    }
};

static tegra210_mvc_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nvidia,tegra210-mvc\0".as_ptr() as *const c_char,
    },
    of_device_id { compatible: null() },
];

// MODULE_DEVICE_TABLE(of, tegra210_mvc_of_match);

unsafe extern "C" fn tegra210_mvc_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = addr_of_mut!((*pdev).dev);
    let mvc: *mut tegra210_mvc;
    let regs: *mut c_void;
    let mut err: c_int;

    mvc = devm_kzalloc(dev, core::mem::size_of::<tegra210_mvc>(), GFP_KERNEL) as *mut tegra210_mvc;
    if mvc.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, mvc as *mut c_void);

    (*mvc).curve_type = CURVE_LINEAR;
    (*mvc).ctrl_value = TEGRA210_MVC_CTRL_DEFAULT;

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*mvc).regmap = devm_regmap_init_mmio(dev, regs, &tegra210_mvc_regmap_config);
    if IS_ERR((*mvc).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*mvc).regmap as *const c_void),
            b"regmap init failed\n\0".as_ptr() as *const c_char,
        );
    }

    regcache_cache_only((*mvc).regmap, true);

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_mvc_cmpnt,
        tegra210_mvc_dais.as_mut_ptr(),
        tegra210_mvc_dais.len() as c_int,
    );
    if err != 0 {
        return dev_err_probe(
            dev,
            err,
            b"can't register MVC component\n\0".as_ptr() as *const c_char,
        );
    }

    pm_runtime_enable(dev);

    tegra210_mvc_reset_vol_settings(mvc, addr_of_mut!((*pdev).dev));

    0
}

unsafe extern "C" fn tegra210_mvc_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(addr_of_mut!((*pdev).dev));
}

static tegra210_mvc_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(tegra210_mvc_runtime_suspend),
    runtime_resume: Some(tegra210_mvc_runtime_resume),
    suspend: unsafe { &pm_runtime_force_suspend as *const c_void },
    resume: unsafe { &pm_runtime_force_resume as *const c_void },
};

static mut tegra210_mvc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"tegra210-mvc\0".as_ptr() as *const c_char,
        of_match_table: tegra210_mvc_of_match.as_ptr(),
        pm: &tegra210_mvc_pm_ops,
    },
    probe: Some(tegra210_mvc_platform_probe),
    remove: Some(tegra210_mvc_platform_remove),
};

// module_platform_driver(tegra210_mvc_driver)
// MODULE_AUTHOR("Arun Shamanna Lakshmi <aruns@nvidia.com>");
// MODULE_DESCRIPTION("Tegra210 MVC ASoC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
