// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2021-2024 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_mixer.c - Tegra210 MIXER driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct soc_mixer_control {
    pub reg: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
    pub id: c_int,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; TEGRA210_MIXER_RX_MAX as usize],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub audio_ch: c_uint,
    pub client_ch: c_uint,
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
}

#[repr(C)]
pub struct tegra210_mixer_gain_params {
    pub poly_coeff: [c_uint; NUM_GAIN_POLY_COEFFS as usize],
    pub gain_value: c_uint,
    pub duration: [c_uint; NUM_DURATION_PARMS as usize],
}

#[repr(C)]
pub struct tegra210_mixer {
    pub regmap: *mut regmap,
    pub gain_value: [c_uint; TEGRA210_MIXER_RX_MAX as usize],
    pub fade_gain: [c_long; TEGRA210_MIXER_RX_MAX as usize],
    pub duration: [c_uint; TEGRA210_MIXER_RX_MAX as usize],
    pub fade_pending: [bool_; TEGRA210_MIXER_RX_MAX as usize],
    pub in_fade: [bool_; TEGRA210_MIXER_RX_MAX as usize],
}

#[repr(C)]
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
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub reg_default_cb: Option<unsafe extern "C" fn(c_uint) -> c_uint>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn div_u64(dividend: u64, divisor: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt: *const snd_soc_component_driver,
        dais: *mut snd_soc_dai_driver,
        num_dais: c_uint,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn regmap_default_zero_cb(reg: c_uint) -> c_uint;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

unsafe extern "C" {
    static TEGRA210_MIXER_REG_STRIDE: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_0: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_STRIDE: c_uint;
    static TEGRA210_MIXER_RX1_CIF_CTRL: c_uint;
    static TEGRA210_MIXER_RX1_CTRL: c_uint;
    static TEGRA210_MIXER_RX1_PEAK_CTRL: c_uint;
    static TEGRA210_MIXER_TX1_INT_MASK: c_uint;
    static TEGRA210_MIXER_TX1_CIF_CTRL: c_uint;
    static TEGRA210_MIXER_ENABLE: c_uint;
    static TEGRA210_MIXER_CG: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_CTRL: c_uint;
    static TEGRA210_MIXER_PEAKM_RAM_CTRL: c_uint;
    static TEGRA210_MIXER_EN: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_SHIFT: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_MASK: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_INIT_EN: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_RW_WRITE: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_SEQ_ACCESS_EN: c_uint;
    static TEGRA210_MIXER_GAIN_CFG_RAM_DATA: c_uint;
    static NUM_GAIN_POLY_COEFFS: c_uint;
    static NUM_DURATION_PARMS: c_uint;
    static DURATION_N3_ID: c_uint;
    static DURATION_INV_N3_ID: c_uint;
    static TEGRA210_MIXER_PRESCALAR: c_uint;
    static REG_CFG_DONE_TRIGGER: c_uint;
    static VAL_CFG_DONE_TRIGGER: c_uint;
    static TEGRA210_MIXER_FADE_DURATION_MIN: c_uint;
    static TEGRA210_MIXER_FADE_DURATION_MAX: c_uint;
    static TEGRA210_MIXER_RX_MAX: c_uint;
    static TEGRA210_MIXER_GAIN_MAX: c_uint;
    static TEGRA210_MIXER_SAMPLE_COUNT_ENABLE: c_uint;
    static TEGRA210_MIXER_RX1_SAMPLE_COUNT: c_uint;
    static TEGRA210_MIXER_FADE_IDLE: c_uint;
    static TEGRA210_MIXER_FADE_ACTIVE: c_uint;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_LE: c_uint;
    static SNDRV_PCM_FORMAT_S32_LE: c_uint;
    static TEGRA_ACIF_BITS_16: c_uint;
    static TEGRA_ACIF_BITS_32: c_uint;
    static TEGRA210_MIXER_TX1_ENABLE: c_uint;
    static TEGRA210_MIXER_TX2_ENABLE: c_uint;
    static TEGRA210_MIXER_TX3_ENABLE: c_uint;
    static TEGRA210_MIXER_TX4_ENABLE: c_uint;
    static TEGRA210_MIXER_TX5_ENABLE: c_uint;
    static TEGRA210_MIXER_TX1_ADDER_CONFIG: c_uint;
    static TEGRA210_MIXER_TX2_ADDER_CONFIG: c_uint;
    static TEGRA210_MIXER_TX3_ADDER_CONFIG: c_uint;
    static TEGRA210_MIXER_TX4_ADDER_CONFIG: c_uint;
    static TEGRA210_MIXER_TX5_ADDER_CONFIG: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;
    static SND_SOC_NOPM: c_uint;
    static TEGRA210_MIXER_RX_LIMIT: c_uint;
    static TEGRA210_MIXER_TX_LIMIT: c_uint;
    static TEGRA210_MIXER_RX1_SOFT_RESET: c_uint;
    static TEGRA210_MIXER_TX1_SOFT_RESET: c_uint;
    static TEGRA210_MIXER_TX1_STATUS: c_uint;
    static TEGRA210_MIXER_TX1_INT_STATUS: c_uint;
    static TEGRA210_MIXER_TX1_INT_SET: c_uint;
    static TEGRA210_MIXER_SOFT_RESET: c_uint;
    static TEGRA210_MIXER_STATUS: c_uint;
    static TEGRA210_MIXER_INT_STATUS: c_uint;
    static TEGRA210_MIXER_PEAKM_RAM_DATA: c_uint;
    static TEGRA210_MIXER_RX1_STATUS: c_uint;
    static TEGRA210_MIXER_CTRL: c_uint;
    static REGCACHE_FLAT: c_uint;
    static GFP_KERNEL: c_uint;
}

unsafe fn MIXER_REG(reg: c_uint, id: c_uint) -> c_uint {
    reg.wrapping_add(id.wrapping_mul(TEGRA210_MIXER_REG_STRIDE))
}

unsafe fn MIXER_REG_BASE(reg: c_uint) -> c_uint {
    reg % TEGRA210_MIXER_REG_STRIDE
}

unsafe fn MIXER_GAIN_CFG_RAM_ADDR(id: c_uint) -> c_uint {
    TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_0
        .wrapping_add(id.wrapping_mul(TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_STRIDE))
}

unsafe fn REG_DURATION_PARAM(reg: c_uint, i: c_int) -> c_uint {
    reg.wrapping_add(NUM_GAIN_POLY_COEFFS)
        .wrapping_add(1)
        .wrapping_add(i as c_uint)
}

unsafe fn BIT_ULL(n: c_uint) -> u64 {
    1u64 << n
}

static mut tegra210_mixer_reg_defaults: [reg_default; 49] = [
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00010823 },
    reg_default { reg: 0, def: 0x000012c0 },
    reg_default { reg: 0, def: 0x00000001 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00000001 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00000001 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00000001 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x00000001 },
    reg_default { reg: 0, def: 0x00007700 },
    reg_default { reg: 0, def: 0x0 },
    reg_default { reg: 0, def: 0x00000001 },
    reg_default { reg: 0, def: 0x00004000 },
    reg_default { reg: 0, def: 0x00004000 },
    reg_default { reg: 0, def: 0 },
    reg_default { reg: 0, def: 0 },
    reg_default { reg: 0, def: 0 },
    reg_default { reg: 0, def: 0 },
    reg_default { reg: 0, def: 0 },
];

/* Default gain parameters */
static gain_params: tegra210_mixer_gain_params = tegra210_mixer_gain_params {
    /* Polynomial coefficients */
    poly_coeff: [0, 0, 0, 0, 0, 0, 0, 0x1000000, 0],
    /* Gain value */
    gain_value: 0x10000,
    /* Duration Parameters */
    duration: [0, 0, 0x400, 0x8000000],
};

unsafe extern "C" fn tegra210_mixer_runtime_suspend(dev: *mut device) -> c_int {
    let mixer = dev_get_drvdata(dev) as *mut tegra210_mixer;

    regcache_cache_only((*mixer).regmap, true);
    regcache_mark_dirty((*mixer).regmap);

    0
}

unsafe extern "C" fn tegra210_mixer_runtime_resume(dev: *mut device) -> c_int {
    let mixer = dev_get_drvdata(dev) as *mut tegra210_mixer;
    let err: c_int;

    regcache_cache_only((*mixer).regmap, false);
    err = regcache_sync((*mixer).regmap);
    if err != 0 {
        return err;
    }

    regmap_write((*mixer).regmap, TEGRA210_MIXER_ENABLE, TEGRA210_MIXER_EN)
}

unsafe fn regmap_read_poll_timeout_gain_cfg(map: *mut regmap, val: *mut c_uint) -> c_int {
    let mut err: c_int;
    let mut timeout = 10000;
    loop {
        err = regmap_read(map, TEGRA210_MIXER_GAIN_CFG_RAM_CTRL, val);
        if err < 0 {
            return err;
        }
        if (*val & 0x80000000) == 0 {
            return 0;
        }
        if timeout <= 0 {
            return -110;
        }
        timeout -= 10;
    }
}

unsafe fn tegra210_mixer_write_ram(
    mixer: *mut tegra210_mixer,
    addr: c_uint,
    coef: c_uint,
) -> c_int {
    let mut reg: c_uint;
    let mut val: c_uint = 0;
    let mut err: c_int;

    /* Check if busy */
    err = regmap_read_poll_timeout_gain_cfg((*mixer).regmap, &mut val);
    if err < 0 {
        return err;
    }

    reg = (addr << TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_SHIFT)
        & TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_MASK;
    reg |= TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_INIT_EN;
    reg |= TEGRA210_MIXER_GAIN_CFG_RAM_RW_WRITE;
    reg |= TEGRA210_MIXER_GAIN_CFG_RAM_SEQ_ACCESS_EN;

    regmap_write((*mixer).regmap, TEGRA210_MIXER_GAIN_CFG_RAM_CTRL, reg);
    regmap_write((*mixer).regmap, TEGRA210_MIXER_GAIN_CFG_RAM_DATA, coef);

    0
}

unsafe fn tegra210_mixer_configure_gain(
    cmpnt: *mut snd_soc_component,
    id: c_uint,
    instant_gain: bool,
) -> c_int {
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let reg = MIXER_GAIN_CFG_RAM_ADDR(id);
    let mut err: c_int = 0;
    let mut i: c_int;

    pm_runtime_get_sync((*cmpnt).dev);

    /* Write default gain poly coefficients */
    i = 0;
    while i < NUM_GAIN_POLY_COEFFS as c_int {
        err = tegra210_mixer_write_ram(mixer, reg.wrapping_add(i as c_uint), gain_params.poly_coeff[i as usize]);

        if err < 0 {
            goto_rpm_put(cmpnt, err);
            return err;
        }
        i += 1;
    }

    /* Write stored gain value */
    err = tegra210_mixer_write_ram(
        mixer,
        reg.wrapping_add(NUM_GAIN_POLY_COEFFS),
        (*mixer).gain_value[id as usize],
    );
    if err < 0 {
        goto_rpm_put(cmpnt, err);
        return err;
    }

    /* Write duration parameters */
    i = 0;
    while i < NUM_DURATION_PARMS as c_int {
        let val: u32;

        if instant_gain {
            val = 1;
        } else if i == DURATION_N3_ID as c_int {
            val = (*mixer).duration[id as usize];
        } else if i == DURATION_INV_N3_ID as c_int {
            val = div_u64(
                BIT_ULL(31 + TEGRA210_MIXER_PRESCALAR),
                (*mixer).duration[id as usize],
            );
        } else {
            val = gain_params.duration[i as usize];
        }

        err = tegra210_mixer_write_ram(mixer, REG_DURATION_PARAM(reg, i), val);
        if err < 0 {
            goto_rpm_put(cmpnt, err);
            return err;
        }
        i += 1;
    }

    /* Trigger to apply gain configurations */
    err = tegra210_mixer_write_ram(
        mixer,
        reg.wrapping_add(REG_CFG_DONE_TRIGGER),
        VAL_CFG_DONE_TRIGGER,
    );

    pm_runtime_put((*cmpnt).dev);

    err
}

unsafe fn goto_rpm_put(cmpnt: *mut snd_soc_component, _err: c_int) {
    pm_runtime_put((*cmpnt).dev);
}

unsafe extern "C" fn tegra210_mixer_fade_duration_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = TEGRA210_MIXER_FADE_DURATION_MIN as c_long;
    (*uinfo).value.integer.max = TEGRA210_MIXER_FADE_DURATION_MAX as c_long;

    0
}

unsafe extern "C" fn tegra210_mixer_get_fade_duration(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;

    (*ucontrol).value.integer.value[0] = (*mixer).duration[(*mc).reg as usize] as c_long;

    0
}

unsafe extern "C" fn tegra210_mixer_put_fade_duration(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let id = (*mc).reg;
    let duration = (*ucontrol).value.integer.value[0];

    if duration < TEGRA210_MIXER_FADE_DURATION_MIN as c_long
        || duration > TEGRA210_MIXER_FADE_DURATION_MAX as c_long
    {
        return -EINVAL;
    }

    if (*mixer).duration[id as usize] == duration as c_uint {
        return 0;
    }

    (*mixer).duration[id as usize] = duration as c_uint;
    (*mixer).fade_pending[id as usize] = true;

    1
}

unsafe extern "C" fn tegra210_mixer_get_fade_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;

    (*ucontrol).value.integer.value[0] = (*mixer).fade_gain[(*mc).reg as usize];

    0
}

unsafe extern "C" fn tegra210_mixer_put_fade_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let id = (*mc).reg;

    if (*ucontrol).value.integer.value[0] < 0
        || (*ucontrol).value.integer.value[0] > TEGRA210_MIXER_GAIN_MAX as c_long
    {
        return -EINVAL;
    }

    if (*mixer).fade_gain[id as usize] == (*ucontrol).value.integer.value[0] {
        return 0;
    }

    (*mixer).fade_gain[id as usize] = (*ucontrol).value.integer.value[0];
    (*mixer).fade_pending[id as usize] = true;

    1
}

unsafe extern "C" fn tegra210_mixer_get_fade_switch(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.integer.value[0] = 0;

    0
}

unsafe extern "C" fn tegra210_mixer_put_fade_switch(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let mut id: c_int;
    let mut err: c_int;
    let mut changed: c_int = 0;

    err = pm_runtime_resume_and_get((*cmpnt).dev);
    if err < 0 {
        return err;
    }

    /* Switch off: disable sample count for all active fades */
    if (*ucontrol).value.integer.value[0] == 0 {
        id = 0;
        while id < TEGRA210_MIXER_RX_MAX as c_int {
            if !(*mixer).in_fade[id as usize] {
                id += 1;
                continue;
            }

            regmap_update_bits(
                (*mixer).regmap,
                MIXER_REG(TEGRA210_MIXER_RX1_CTRL, id as c_uint),
                TEGRA210_MIXER_SAMPLE_COUNT_ENABLE,
                0,
            );
            (*mixer).in_fade[id as usize] = false;
            changed = 1;
            id += 1;
        }

        pm_runtime_put((*cmpnt).dev);
        return changed;
    }

    /* Stop active fades on pending streams before reconfiguring */
    id = 0;
    while id < TEGRA210_MIXER_RX_MAX as c_int {
        if !(*mixer).fade_pending[id as usize] {
            id += 1;
            continue;
        }

        if (*mixer).in_fade[id as usize] {
            regmap_update_bits(
                (*mixer).regmap,
                MIXER_REG(TEGRA210_MIXER_RX1_CTRL, id as c_uint),
                TEGRA210_MIXER_SAMPLE_COUNT_ENABLE,
                0,
            );
            (*mixer).in_fade[id as usize] = false;
        }

        (*mixer).gain_value[id as usize] = (*mixer).fade_gain[id as usize] as c_uint;
        err = tegra210_mixer_configure_gain(cmpnt, id as c_uint, false);
        if err != 0 {
            dev_err((*cmpnt).dev, b"Failed to configure fade for RX%d\n\0".as_ptr() as *const c_char, id + 1);
            pm_runtime_put((*cmpnt).dev);
            return err;
        }

        changed = 1;
        id += 1;
    }

    if changed == 0 {
        pm_runtime_put((*cmpnt).dev);
        return 0;
    }

    /* Enable sample count for all pending streams */
    id = 0;
    while id < TEGRA210_MIXER_RX_MAX as c_int {
        if !(*mixer).fade_pending[id as usize] {
            id += 1;
            continue;
        }

        err = regmap_update_bits(
            (*mixer).regmap,
            MIXER_REG(TEGRA210_MIXER_RX1_CTRL, id as c_uint),
            TEGRA210_MIXER_SAMPLE_COUNT_ENABLE,
            TEGRA210_MIXER_SAMPLE_COUNT_ENABLE,
        );
        if err != 0 {
            dev_err((*cmpnt).dev, b"Failed to enable sample count for RX%d\n\0".as_ptr() as *const c_char, id + 1);
            pm_runtime_put((*cmpnt).dev);
            return err;
        }

        (*mixer).in_fade[id as usize] = true;
        (*mixer).fade_pending[id as usize] = false;
        id += 1;
    }

    pm_runtime_put((*cmpnt).dev);

    1
}

unsafe extern "C" fn tegra210_mixer_get_fade_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let mut count: u32 = 0;
    let mut id: c_int;
    let err: c_int;

    err = pm_runtime_resume_and_get((*cmpnt).dev);
    if err < 0 {
        return err;
    }

    id = 0;
    while id < TEGRA210_MIXER_RX_MAX as c_int {
        if !(*mixer).in_fade[id as usize] {
            (*ucontrol).value.integer.value[id as usize] = TEGRA210_MIXER_FADE_IDLE as c_long;
            id += 1;
            continue;
        }

        regmap_read(
            (*mixer).regmap,
            MIXER_REG(TEGRA210_MIXER_RX1_SAMPLE_COUNT, id as c_uint),
            &mut count,
        );

        if count >= (*mixer).duration[id as usize] {
            (*ucontrol).value.integer.value[id as usize] = TEGRA210_MIXER_FADE_IDLE as c_long;
        } else {
            (*ucontrol).value.integer.value[id as usize] = TEGRA210_MIXER_FADE_ACTIVE as c_long;
        }
        id += 1;
    }

    pm_runtime_put((*cmpnt).dev);

    0
}

unsafe extern "C" fn tegra210_mixer_get_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let reg = (*mc).reg;
    let i: c_uint;

    i = (reg - TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_0) / TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_STRIDE;

    (*ucontrol).value.integer.value[0] = (*mixer).gain_value[i as usize] as c_long;

    0
}

unsafe fn tegra210_mixer_apply_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    instant_gain: bool,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mixer = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_mixer;
    let reg = (*mc).reg;
    let id: c_uint;
    let err: c_int;

    /* Save gain value for specific MIXER input */
    id = (reg - TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_0) / TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_STRIDE;

    if (*mixer).gain_value[id as usize] == (*ucontrol).value.integer.value[0] as c_uint {
        return 0;
    }

    (*mixer).gain_value[id as usize] = (*ucontrol).value.integer.value[0] as c_uint;

    err = tegra210_mixer_configure_gain(cmpnt, id, instant_gain);
    if err != 0 {
        dev_err((*cmpnt).dev, b"Failed to apply gain\n\0".as_ptr() as *const c_char);
        return err;
    }

    1
}

unsafe extern "C" fn tegra210_mixer_put_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mixer_apply_gain(kcontrol, ucontrol, false)
}

unsafe extern "C" fn tegra210_mixer_put_instant_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    tegra210_mixer_apply_gain(kcontrol, ucontrol, true)
}

unsafe fn tegra210_mixer_set_audio_cif(
    mixer: *mut tegra210_mixer,
    params: *mut snd_pcm_hw_params,
    reg: c_uint,
    id: c_uint,
) -> c_int {
    let channels: c_uint;
    let audio_bits: c_uint;
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();

    channels = params_channels(params);

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

    tegra_set_cif(
        (*mixer).regmap,
        reg.wrapping_add(id.wrapping_mul(TEGRA210_MIXER_REG_STRIDE)),
        &mut cif_conf,
    );

    0
}

unsafe extern "C" fn tegra210_mixer_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mixer = snd_soc_dai_get_drvdata(dai) as *mut tegra210_mixer;
    let err: c_int;

    err = tegra210_mixer_set_audio_cif(mixer, params, TEGRA210_MIXER_RX1_CIF_CTRL, (*dai).id as c_uint);
    if err < 0 {
        return err;
    }

    tegra210_mixer_configure_gain((*dai).component, (*dai).id as c_uint, false)
}

unsafe extern "C" fn tegra210_mixer_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mixer = snd_soc_dai_get_drvdata(dai) as *mut tegra210_mixer;

    tegra210_mixer_set_audio_cif(
        mixer,
        params,
        TEGRA210_MIXER_TX1_CIF_CTRL,
        ((*dai).id as c_uint).wrapping_sub(TEGRA210_MIXER_RX_MAX),
    )
}

static tegra210_mixer_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_mixer_out_hw_params),
};

static tegra210_mixer_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_mixer_in_hw_params),
};

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

static mut tegra210_mixer_dais: [snd_soc_dai_driver; 15] = [
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF1\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX1-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX1-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF2\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX2-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX2-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF3\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX3-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX3-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF4\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX4-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX4-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF5\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX5-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX5-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF6\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX6-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX6-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF7\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX7-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX7-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF8\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX8-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX8-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF9\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX9-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX9-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-RX-CIF10\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"RX10-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"RX10-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_in_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-TX-CIF1\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"TX1-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"TX1-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_out_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-TX-CIF2\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"TX2-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"TX2-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_out_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-TX-CIF3\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"TX3-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"TX3-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_out_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-TX-CIF4\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"TX4-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"TX4-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_out_dai_ops },
    snd_soc_dai_driver { name: cstr(b"MIXER-TX-CIF5\0"), playback: snd_soc_pcm_stream { stream_name: cstr(b"TX5-CIF-Playback\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, capture: snd_soc_pcm_stream { stream_name: cstr(b"TX5-CIF-Capture\0"), channels_min: 1, channels_max: 8, rates: 0, formats: 0 }, ops: &tegra210_mixer_out_dai_ops },
];

/* ADDER_CTRL_DECL(), FADE_CTRL(), GAIN_CTRL(), SND_SOC_DAPM_*(), and
 * SOC_SINGLE_EXT() construct kernel-owned structures through C macros. Their
 * generated static tables are represented here as external-layout placeholders
 * while preserving the functions, names, routes, and component references.
 */
static adder1: [snd_kcontrol_new; 0] = [];
static adder2: [snd_kcontrol_new; 0] = [];
static adder3: [snd_kcontrol_new; 0] = [];
static adder4: [snd_kcontrol_new; 0] = [];
static adder5: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn tegra210_mixer_fade_status_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = TEGRA210_MIXER_RX_MAX;
    (*uinfo).value.integer.min = TEGRA210_MIXER_FADE_IDLE as c_long;
    (*uinfo).value.integer.max = TEGRA210_MIXER_FADE_ACTIVE as c_long;

    0
}

static tegra210_mixer_gain_ctls: [snd_kcontrol_new; 0] = [];
static tegra210_mixer_widgets: [snd_soc_dapm_widget; 0] = [];

static tegra210_mixer_routes: [snd_soc_dapm_route; 200] = [
    snd_soc_dapm_route { sink: cstr(b"RX1 XBAR-Playback\0"), control: ptr::null(), source: cstr(b"RX1 XBAR-TX\0") },
    snd_soc_dapm_route { sink: cstr(b"RX1-CIF-Playback\0"), control: ptr::null(), source: cstr(b"RX1 XBAR-Playback\0") },
    snd_soc_dapm_route { sink: cstr(b"RX1\0"), control: ptr::null(), source: cstr(b"RX1-CIF-Playback\0") },
    snd_soc_dapm_route { sink: cstr(b"RX1 XBAR-Capture\0"), control: ptr::null(), source: cstr(b"RX1 XBAR-TX\0") },
    snd_soc_dapm_route { sink: cstr(b"RX1-CIF-Capture\0"), control: ptr::null(), source: cstr(b"RX1 XBAR-Capture\0") },
    snd_soc_dapm_route { sink: cstr(b"RX1\0"), control: ptr::null(), source: cstr(b"RX1-CIF-Capture\0") },
    snd_soc_dapm_route { sink: cstr(b"RX2 XBAR-Playback\0"), control: ptr::null(), source: cstr(b"RX2 XBAR-TX\0") },
    snd_soc_dapm_route { sink: cstr(b"RX2-CIF-Playback\0"), control: ptr::null(), source: cstr(b"RX2 XBAR-Playback\0") },
    snd_soc_dapm_route { sink: cstr(b"RX2\0"), control: ptr::null(), source: cstr(b"RX2-CIF-Playback\0") },
    snd_soc_dapm_route { sink: cstr(b"RX2 XBAR-Capture\0"), control: ptr::null(), source: cstr(b"RX2 XBAR-TX\0") },
    snd_soc_dapm_route { sink: cstr(b"RX2-CIF-Capture\0"), control: ptr::null(), source: cstr(b"RX2 XBAR-Capture\0") },
    snd_soc_dapm_route { sink: cstr(b"RX2\0"), control: ptr::null(), source: cstr(b"RX2-CIF-Capture\0") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() }; 188
];

static tegra210_mixer_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra210_mixer_widgets.as_ptr(),
    num_dapm_widgets: tegra210_mixer_widgets.len() as c_uint,
    dapm_routes: tegra210_mixer_routes.as_ptr(),
    num_dapm_routes: tegra210_mixer_routes.len() as c_uint,
    controls: tegra210_mixer_gain_ctls.as_ptr(),
    num_controls: tegra210_mixer_gain_ctls.len() as c_uint,
};

unsafe extern "C" fn tegra210_mixer_wr_reg(_dev: *mut device, mut reg: c_uint) -> bool {
    if reg < TEGRA210_MIXER_RX_LIMIT {
        reg = MIXER_REG_BASE(reg);
    } else if reg < TEGRA210_MIXER_TX_LIMIT {
        reg = MIXER_REG_BASE(reg).wrapping_add(TEGRA210_MIXER_TX1_ENABLE);
    }

    if reg == TEGRA210_MIXER_RX1_SOFT_RESET
        || (reg >= TEGRA210_MIXER_RX1_CIF_CTRL && reg <= TEGRA210_MIXER_RX1_PEAK_CTRL)
        || reg == TEGRA210_MIXER_TX1_ENABLE
        || reg == TEGRA210_MIXER_TX1_SOFT_RESET
        || (reg >= TEGRA210_MIXER_TX1_INT_MASK && reg <= TEGRA210_MIXER_TX1_ADDER_CONFIG)
        || (reg >= TEGRA210_MIXER_ENABLE && reg <= TEGRA210_MIXER_CG)
        || (reg >= TEGRA210_MIXER_GAIN_CFG_RAM_CTRL && reg <= TEGRA210_MIXER_CTRL)
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn tegra210_mixer_rd_reg(_dev: *mut device, mut reg: c_uint) -> bool {
    if reg < TEGRA210_MIXER_RX_LIMIT {
        reg = MIXER_REG_BASE(reg);
    } else if reg < TEGRA210_MIXER_TX_LIMIT {
        reg = MIXER_REG_BASE(reg).wrapping_add(TEGRA210_MIXER_TX1_ENABLE);
    }

    if (reg >= TEGRA210_MIXER_RX1_SOFT_RESET && reg <= TEGRA210_MIXER_RX1_SAMPLE_COUNT)
        || (reg >= TEGRA210_MIXER_TX1_ENABLE && reg <= TEGRA210_MIXER_TX1_ADDER_CONFIG)
        || (reg >= TEGRA210_MIXER_ENABLE && reg <= TEGRA210_MIXER_CTRL)
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn tegra210_mixer_volatile_reg(_dev: *mut device, mut reg: c_uint) -> bool {
    if reg < TEGRA210_MIXER_RX_LIMIT {
        reg = MIXER_REG_BASE(reg);
    } else if reg < TEGRA210_MIXER_TX_LIMIT {
        reg = MIXER_REG_BASE(reg).wrapping_add(TEGRA210_MIXER_TX1_ENABLE);
    }

    match reg {
        x if x == TEGRA210_MIXER_RX1_SOFT_RESET => true,
        x if x == TEGRA210_MIXER_RX1_STATUS => true,
        x if x == TEGRA210_MIXER_TX1_SOFT_RESET => true,
        x if x == TEGRA210_MIXER_TX1_STATUS => true,
        x if x == TEGRA210_MIXER_TX1_INT_STATUS => true,
        x if x == TEGRA210_MIXER_TX1_INT_SET => true,
        x if x == TEGRA210_MIXER_SOFT_RESET => true,
        x if x == TEGRA210_MIXER_STATUS => true,
        x if x == TEGRA210_MIXER_INT_STATUS => true,
        x if x == TEGRA210_MIXER_GAIN_CFG_RAM_CTRL => true,
        x if x == TEGRA210_MIXER_GAIN_CFG_RAM_DATA => true,
        x if x == TEGRA210_MIXER_PEAKM_RAM_CTRL => true,
        x if x == TEGRA210_MIXER_PEAKM_RAM_DATA => true,
        x if x == TEGRA210_MIXER_RX1_SAMPLE_COUNT => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_mixer_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == TEGRA210_MIXER_GAIN_CFG_RAM_DATA => true,
        x if x == TEGRA210_MIXER_PEAKM_RAM_DATA => true,
        _ => false,
    }
}

static tegra210_mixer_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0,
    writeable_reg: Some(tegra210_mixer_wr_reg),
    readable_reg: Some(tegra210_mixer_rd_reg),
    volatile_reg: Some(tegra210_mixer_volatile_reg),
    precious_reg: Some(tegra210_mixer_precious_reg),
    reg_defaults: unsafe { tegra210_mixer_reg_defaults.as_ptr() },
    num_reg_defaults: 49,
    reg_default_cb: Some(regmap_default_zero_cb),
    cache_type: 0,
};

static tegra210_mixer_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr(b"nvidia,tegra210-amixer\0") },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, tegra210_mixer_of_match); */

unsafe extern "C" fn tegra210_mixer_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mixer: *mut tegra210_mixer;
    let regs: *mut c_void;
    let mut err: c_int;
    let mut i: c_int;

    mixer = devm_kzalloc(dev, size_of::<tegra210_mixer>(), GFP_KERNEL) as *mut tegra210_mixer;
    if mixer.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, mixer as *mut c_void);

    /* Use default gain value for all MIXER inputs */
    i = 0;
    while i < TEGRA210_MIXER_RX_MAX as c_int {
        (*mixer).gain_value[i as usize] = gain_params.gain_value;
        (*mixer).fade_gain[i as usize] = gain_params.gain_value as c_long;
        (*mixer).duration[i as usize] = gain_params.duration[DURATION_N3_ID as usize];
        i += 1;
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*mixer).regmap = devm_regmap_init_mmio(dev, regs, &tegra210_mixer_regmap_config);
    if IS_ERR((*mixer).regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*mixer).regmap as *const c_void), b"regmap init failed\n\0".as_ptr() as *const c_char);
    }

    regcache_cache_only((*mixer).regmap, true);

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_mixer_cmpnt,
        tegra210_mixer_dais.as_mut_ptr(),
        tegra210_mixer_dais.len() as c_uint,
    );
    if err != 0 {
        return dev_err_probe(dev, err, b"can't register MIXER component\n\0".as_ptr() as *const c_char);
    }

    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn tegra210_mixer_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

static tegra210_mixer_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut tegra210_mixer_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: cstr(b"tegra210_mixer\0"),
        of_match_table: tegra210_mixer_of_match.as_ptr(),
        pm: &tegra210_mixer_pm_ops,
    },
    probe: Some(tegra210_mixer_platform_probe),
    remove: Some(tegra210_mixer_platform_remove),
};

/* module_platform_driver(tegra210_mixer_driver); */

/* MODULE_AUTHOR("Arun Shamanna Lakshmi <aruns@nvidia.com>"); */
/* MODULE_DESCRIPTION("Tegra210 MIXER ASoC driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
