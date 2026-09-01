// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2021-2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_adx.c - Tegra210 ADX driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u16 = u16;
type u32 = u32;
type bool_ = bool;
type snd_pcm_format_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 0;
const true_: bool = true;
const false_: bool = false;
const BITS_PER_BYTE: c_uint = 8;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

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
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub id: c_int,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub audio_ch: c_uint,
    pub client_ch: c_uint,
    pub audio_bits: c_int,
    pub client_bits: c_int,
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
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *mut snd_kcontrol_new,
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
    pub reg_default_cb: Option<unsafe extern "C" fn()>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct tegra210_adx_soc_data {
    pub regmap_conf: *const regmap_config,
    pub max_ch: c_uint,
    pub ram_depth: c_uint,
    pub byte_mask_size: c_uint,
    pub cya_offset: c_uint,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
}

#[repr(C)]
pub struct tegra210_adx {
    pub regmap: *mut regmap,
    pub map: *mut u16,
    pub byte_mask: *mut c_uint,
    pub soc_data: *const tegra210_adx_soc_data,
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
    static TEGRA210_ADX_RX_INT_MASK: c_uint;
    static TEGRA210_ADX_RX_CIF_CTRL: c_uint;
    static TEGRA210_ADX_TX_INT_MASK: c_uint;
    static TEGRA210_ADX_TX1_CIF_CTRL: c_uint;
    static TEGRA210_ADX_TX2_CIF_CTRL: c_uint;
    static TEGRA210_ADX_TX3_CIF_CTRL: c_uint;
    static TEGRA210_ADX_TX4_CIF_CTRL: c_uint;
    static TEGRA210_ADX_CG: c_uint;
    static TEGRA210_ADX_CFG_RAM_CTRL: c_uint;
    static TEGRA264_ADX_CFG_RAM_CTRL: c_uint;
    static TEGRA210_ADX_CFG_RAM_DATA: c_uint;
    static TEGRA264_ADX_CFG_RAM_DATA: c_uint;
    static TEGRA210_ADX_CFG_RAM_CTRL_SEQ_ACCESS_EN: c_uint;
    static TEGRA210_ADX_CFG_RAM_CTRL_ADDR_INIT_EN: c_uint;
    static TEGRA210_ADX_CFG_RAM_CTRL_RW_WRITE: c_uint;
    static TEGRA_ADX_SLOTS_PER_WORD: c_uint;
    static TEGRA210_ADX_IN_BYTE_EN0: c_uint;
    static TEGRA210_ADX_AUDIOCIF_CH_STRIDE: c_uint;
    static TEGRA210_ADX_STATUS: c_uint;
    static TEGRA210_ADX_SOFT_RESET: c_uint;
    static TEGRA210_ADX_SOFT_RESET_SOFT_RESET_MASK: c_uint;
    static TEGRA210_ADX_SOFT_RESET_SOFT_EN: c_uint;
    static SNDRV_PCM_FORMAT_S8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t;
    static TEGRA_ACIF_BITS_8: c_int;
    static TEGRA_ACIF_BITS_16: c_int;
    static TEGRA_ACIF_BITS_32: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static TEGRA210_ADX_ENABLE: c_uint;
    static TEGRA210_ADX_ENABLE_SHIFT: c_uint;
    static TEGRA210_ADX_CTRL: c_uint;
    static TEGRA210_ADX_IN_BYTE_EN1: c_uint;
    static TEGRA210_ADX_RX_STATUS: c_uint;
    static TEGRA210_ADX_RX_INT_STATUS: c_uint;
    static TEGRA210_ADX_RX_INT_SET: c_uint;
    static TEGRA210_ADX_TX_STATUS: c_uint;
    static TEGRA210_ADX_TX_INT_STATUS: c_uint;
    static TEGRA210_ADX_TX_INT_SET: c_uint;
    static TEGRA210_ADX_INT_STATUS: c_uint;
    static TEGRA264_ADX_CYA: c_uint;
    static TEGRA210_ADX_MAX_CHANNEL: c_uint;
    static TEGRA210_ADX_RAM_DEPTH: c_uint;
    static TEGRA210_ADX_BYTE_MASK_COUNT: c_uint;
    static TEGRA210_ADX_CYA_OFFSET: c_uint;
    static TEGRA264_ADX_MAX_CHANNEL: c_uint;
    static TEGRA264_ADX_RAM_DEPTH: c_uint;
    static TEGRA264_ADX_BYTE_MASK_COUNT: c_uint;
    static TEGRA264_ADX_CYA_OFFSET: c_uint;
    static TEGRA_ADX_IN_DAI_ID: usize;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut tegra210_adx;
    fn dev_get_drvdata(dev: *mut device) -> *mut tegra210_adx;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn tegra264_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut tegra210_adx;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *mut snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const tegra210_adx_soc_data;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
    fn regmap_default_zero_cb();
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

unsafe fn regmap_read_poll_timeout_adx(
    map: *mut regmap,
    reg: c_uint,
    val: *mut c_uint,
    delay_us: c_uint,
    timeout_us: c_uint,
) -> c_int {
    let mut err: c_int = 0;
    let mut elapsed: c_uint = 0;

    while elapsed <= timeout_us {
        err = regmap_read(map, reg, val);
        if err < 0 {
            return err;
        }
        if (*val & 0x1) == 0 {
            return 0;
        }
        elapsed = elapsed.wrapping_add(delay_us);
    }

    -1
}

unsafe static tegra210_adx_reg_defaults: [reg_default; 9] = [
    reg_default { reg: TEGRA210_ADX_RX_INT_MASK, def: 0x00000001 },
    reg_default { reg: TEGRA210_ADX_RX_CIF_CTRL, def: 0x00007000 },
    reg_default { reg: TEGRA210_ADX_TX_INT_MASK, def: 0x0000000f },
    reg_default { reg: TEGRA210_ADX_TX1_CIF_CTRL, def: 0x00007000 },
    reg_default { reg: TEGRA210_ADX_TX2_CIF_CTRL, def: 0x00007000 },
    reg_default { reg: TEGRA210_ADX_TX3_CIF_CTRL, def: 0x00007000 },
    reg_default { reg: TEGRA210_ADX_TX4_CIF_CTRL, def: 0x00007000 },
    reg_default { reg: TEGRA210_ADX_CG, def: 0x1 },
    reg_default { reg: TEGRA210_ADX_CFG_RAM_CTRL, def: 0x00004000 },
];

unsafe static tegra264_adx_reg_defaults: [reg_default; 9] = [
    reg_default { reg: TEGRA210_ADX_RX_INT_MASK, def: 0x00000001 },
    reg_default { reg: TEGRA210_ADX_RX_CIF_CTRL, def: 0x00003800 },
    reg_default { reg: TEGRA210_ADX_TX_INT_MASK, def: 0x0000000f },
    reg_default { reg: TEGRA210_ADX_TX1_CIF_CTRL, def: 0x00003800 },
    reg_default { reg: TEGRA210_ADX_TX2_CIF_CTRL, def: 0x00003800 },
    reg_default { reg: TEGRA210_ADX_TX3_CIF_CTRL, def: 0x00003800 },
    reg_default { reg: TEGRA210_ADX_TX4_CIF_CTRL, def: 0x00003800 },
    reg_default { reg: TEGRA210_ADX_CG, def: 0x1 },
    reg_default { reg: TEGRA264_ADX_CFG_RAM_CTRL, def: 0x00004000 },
];

unsafe extern "C" fn tegra210_adx_write_map_ram(adx: *mut tegra210_adx) {
    let bits_per_mask: c_uint = (size_of::<c_uint>() * 8) as c_uint;
    let mut i: c_int;

    memset(
        (*adx).byte_mask as *mut c_void,
        0,
        ((*(*adx).soc_data).byte_mask_size as usize) * size_of::<c_uint>(),
    );

    regmap_write(
        (*adx).regmap,
        TEGRA210_ADX_CFG_RAM_CTRL.wrapping_add((*(*adx).soc_data).cya_offset),
        TEGRA210_ADX_CFG_RAM_CTRL_SEQ_ACCESS_EN
            | TEGRA210_ADX_CFG_RAM_CTRL_ADDR_INIT_EN
            | TEGRA210_ADX_CFG_RAM_CTRL_RW_WRITE,
    );

    i = 0;
    while i < (*(*adx).soc_data).ram_depth as c_int {
        let mut word: u32 = 0;
        let mut b: c_int = 0;

        while b < TEGRA_ADX_SLOTS_PER_WORD as c_int {
            let slot: c_uint = (i as c_uint)
                .wrapping_mul(TEGRA_ADX_SLOTS_PER_WORD)
                .wrapping_add(b as c_uint);
            let val: u16 = *(*adx).map.add(slot as usize);

            if val < 256 {
                word |= (val as u32) << ((b as c_uint).wrapping_mul(BITS_PER_BYTE));
                let mask = (*adx).byte_mask.add((slot / bits_per_mask) as usize);
                *mask |= 1u32 << (slot % bits_per_mask);
            }

            b += 1;
        }

        regmap_write(
            (*adx).regmap,
            TEGRA210_ADX_CFG_RAM_DATA.wrapping_add((*(*adx).soc_data).cya_offset),
            word,
        );
        i += 1;
    }

    i = 0;
    while i < (*(*adx).soc_data).byte_mask_size as c_int {
        regmap_write(
            (*adx).regmap,
            TEGRA210_ADX_IN_BYTE_EN0
                .wrapping_add((i as c_uint).wrapping_mul(TEGRA210_ADX_AUDIOCIF_CH_STRIDE)),
            *(*adx).byte_mask.add(i as usize),
        );
        i += 1;
    }
}

unsafe extern "C" fn tegra210_adx_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let adx: *mut tegra210_adx = snd_soc_dai_get_drvdata(dai);
    let mut val: c_uint = 0;
    let mut err: c_int;

    /* Ensure if ADX status is disabled */
    err = regmap_read_poll_timeout_adx((*adx).regmap, TEGRA210_ADX_STATUS, &mut val, 10, 10000);
    if err < 0 {
        dev_err((*dai).dev, c_str!("failed to stop ADX, err = %d\n"), err);
        return err;
    }

    /*
     * Soft Reset: Below performs module soft reset which clears
     * all FSM logic, flushes flow control of FIFO and resets the
     * state register. It also brings module back to disabled
     * state (without flushing the data in the pipe).
     */
    regmap_update_bits(
        (*adx).regmap,
        TEGRA210_ADX_SOFT_RESET,
        TEGRA210_ADX_SOFT_RESET_SOFT_RESET_MASK,
        TEGRA210_ADX_SOFT_RESET_SOFT_EN,
    );

    err = regmap_read_poll_timeout_adx((*adx).regmap, TEGRA210_ADX_SOFT_RESET, &mut val, 10, 10000);
    if err < 0 {
        dev_err((*dai).dev, c_str!("failed to reset ADX, err = %d\n"), err);
        return err;
    }

    0
}

unsafe extern "C" fn tegra210_adx_runtime_suspend(dev: *mut device) -> c_int {
    let adx: *mut tegra210_adx = dev_get_drvdata(dev);

    regcache_cache_only((*adx).regmap, true_);
    regcache_mark_dirty((*adx).regmap);

    0
}

unsafe extern "C" fn tegra210_adx_runtime_resume(dev: *mut device) -> c_int {
    let adx: *mut tegra210_adx = dev_get_drvdata(dev);

    regcache_cache_only((*adx).regmap, false_);
    regcache_sync((*adx).regmap);

    tegra210_adx_write_map_ram(adx);

    0
}

unsafe extern "C" fn tegra210_adx_set_audio_cif(
    dai: *mut snd_soc_dai,
    channels: c_uint,
    format: snd_pcm_format_t,
    reg: c_uint,
) -> c_int {
    let adx: *mut tegra210_adx = snd_soc_dai_get_drvdata(dai);
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();
    let audio_bits: c_int;

    memset(
        &mut cif_conf as *mut tegra_cif_conf as *mut c_void,
        0,
        size_of::<tegra_cif_conf>(),
    );

    if channels < 1 || channels > (*(*adx).soc_data).max_ch {
        dev_err(
            (*dai).dev,
            c_str!("invalid channels: %u (max %u)\n"),
            channels,
            (*(*adx).soc_data).max_ch,
        );
        return -EINVAL;
    }

    if format == SNDRV_PCM_FORMAT_S8 {
        audio_bits = TEGRA_ACIF_BITS_8;
    } else if format == SNDRV_PCM_FORMAT_S16_LE {
        audio_bits = TEGRA_ACIF_BITS_16;
    } else if format == SNDRV_PCM_FORMAT_S24_LE || format == SNDRV_PCM_FORMAT_S32_LE {
        audio_bits = TEGRA_ACIF_BITS_32;
    } else {
        dev_err((*dai).dev, c_str!("unsupported format: %d\n"), format);
        return -EINVAL;
    }

    cif_conf.audio_ch = channels;
    cif_conf.client_ch = channels;
    cif_conf.audio_bits = audio_bits;
    cif_conf.client_bits = audio_bits;

    if (*(*adx).soc_data).max_ch == 32 {
        tegra264_set_cif((*adx).regmap, reg, &mut cif_conf);
    } else {
        tegra_set_cif((*adx).regmap, reg, &mut cif_conf);
    }

    0
}

unsafe extern "C" fn tegra210_adx_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    tegra210_adx_set_audio_cif(
        dai,
        params_channels(params),
        params_format(params),
        TEGRA210_ADX_TX1_CIF_CTRL.wrapping_add(
            (((*dai).id - 1) as c_uint).wrapping_mul(TEGRA210_ADX_AUDIOCIF_CH_STRIDE),
        ),
    )
}

unsafe extern "C" fn tegra210_adx_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    tegra210_adx_set_audio_cif(
        dai,
        params_channels(params),
        params_format(params),
        TEGRA210_ADX_RX_CIF_CTRL,
    )
}

unsafe extern "C" fn tegra210_adx_get_byte_map(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let adx: *mut tegra210_adx = snd_soc_component_get_drvdata(cmpnt);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;

    (*ucontrol).value.integer.value[0] = *(*adx).map.add((*mc).reg as usize) as i64;

    0
}

unsafe extern "C" fn tegra210_adx_put_byte_map(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let adx: *mut tegra210_adx = snd_soc_component_get_drvdata(cmpnt);
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut value: c_uint = (*ucontrol).value.integer.value[0] as c_uint;

    /*
     * Match the previous behaviour: any value outside [0, 255] is
     * treated as the "disabled" sentinel (256). Negative values from
     * userspace fold in through the unsigned cast and are caught here.
     */
    if value > 255 {
        value = 256;
    }

    if *(*adx).map.add((*mc).reg as usize) == value as u16 {
        return 0;
    }

    *(*adx).map.add((*mc).reg as usize) = value as u16;

    1
}

static tegra210_adx_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_adx_in_hw_params),
    startup: Some(tegra210_adx_startup),
};

static tegra210_adx_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_adx_out_hw_params),
    startup: None,
};

unsafe fn dai_formats() -> u64 {
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

unsafe fn in_dai() -> snd_soc_dai_driver {
    snd_soc_dai_driver {
        name: c_str!("ADX-RX-CIF"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("RX-CIF-Playback"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: dai_formats(),
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("RX-CIF-Capture"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: dai_formats(),
        },
        ops: &tegra210_adx_in_dai_ops,
    }
}

unsafe fn out_dai(id: c_int) -> snd_soc_dai_driver {
    let name = match id {
        1 => c_str!("ADX-TX1-CIF"),
        2 => c_str!("ADX-TX2-CIF"),
        3 => c_str!("ADX-TX3-CIF"),
        _ => c_str!("ADX-TX4-CIF"),
    };
    let playback = match id {
        1 => c_str!("TX1-CIF-Playback"),
        2 => c_str!("TX2-CIF-Playback"),
        3 => c_str!("TX3-CIF-Playback"),
        _ => c_str!("TX4-CIF-Playback"),
    };
    let capture = match id {
        1 => c_str!("TX1-CIF-Capture"),
        2 => c_str!("TX2-CIF-Capture"),
        3 => c_str!("TX3-CIF-Capture"),
        _ => c_str!("TX4-CIF-Capture"),
    };

    snd_soc_dai_driver {
        name,
        playback: snd_soc_pcm_stream {
            stream_name: playback,
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: dai_formats(),
        },
        capture: snd_soc_pcm_stream {
            stream_name: capture,
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: dai_formats(),
        },
        ops: &tegra210_adx_out_dai_ops,
    }
}

static mut tegra210_adx_dais: [snd_soc_dai_driver; 5] = unsafe {
    [in_dai(), out_dai(1), out_dai(2), out_dai(3), out_dai(4)]
};

// SND_SOC_DAPM_AIF_IN/OUT macro initializers are preserved as dependency-provided widgets.
unsafe extern "C" {
    static tegra210_adx_widgets: [snd_soc_dapm_widget; 5];
}

macro_rules! route {
    ($sink:literal, $source:literal) => {
        snd_soc_dapm_route {
            sink: c_str!($sink),
            control: ptr::null(),
            source: c_str!($source),
        }
    };
}

static tegra210_adx_routes: [snd_soc_dapm_route; 56] = [
    route!("XBAR-Playback", "XBAR-TX"), route!("RX-CIF-Playback", "XBAR-Playback"), route!("RX", "RX-CIF-Playback"), route!("TX1", "RX"), route!("TX1-CIF-Playback", "TX1"), route!("TX1 XBAR-Playback", "TX1-CIF-Playback"), route!("TX1 XBAR-RX", "TX1 XBAR-Playback"),
    route!("XBAR-Capture", "XBAR-TX"), route!("RX-CIF-Capture", "XBAR-Capture"), route!("RX", "RX-CIF-Capture"), route!("TX1", "RX"), route!("TX1-CIF-Capture", "TX1"), route!("TX1 XBAR-Capture", "TX1-CIF-Capture"), route!("TX1 XBAR-RX", "TX1 XBAR-Capture"),
    route!("XBAR-Playback", "XBAR-TX"), route!("RX-CIF-Playback", "XBAR-Playback"), route!("RX", "RX-CIF-Playback"), route!("TX2", "RX"), route!("TX2-CIF-Playback", "TX2"), route!("TX2 XBAR-Playback", "TX2-CIF-Playback"), route!("TX2 XBAR-RX", "TX2 XBAR-Playback"),
    route!("XBAR-Capture", "XBAR-TX"), route!("RX-CIF-Capture", "XBAR-Capture"), route!("RX", "RX-CIF-Capture"), route!("TX2", "RX"), route!("TX2-CIF-Capture", "TX2"), route!("TX2 XBAR-Capture", "TX2-CIF-Capture"), route!("TX2 XBAR-RX", "TX2 XBAR-Capture"),
    route!("XBAR-Playback", "XBAR-TX"), route!("RX-CIF-Playback", "XBAR-Playback"), route!("RX", "RX-CIF-Playback"), route!("TX3", "RX"), route!("TX3-CIF-Playback", "TX3"), route!("TX3 XBAR-Playback", "TX3-CIF-Playback"), route!("TX3 XBAR-RX", "TX3 XBAR-Playback"),
    route!("XBAR-Capture", "XBAR-TX"), route!("RX-CIF-Capture", "XBAR-Capture"), route!("RX", "RX-CIF-Capture"), route!("TX3", "RX"), route!("TX3-CIF-Capture", "TX3"), route!("TX3 XBAR-Capture", "TX3-CIF-Capture"), route!("TX3 XBAR-RX", "TX3 XBAR-Capture"),
    route!("XBAR-Playback", "XBAR-TX"), route!("RX-CIF-Playback", "XBAR-Playback"), route!("RX", "RX-CIF-Playback"), route!("TX4", "RX"), route!("TX4-CIF-Playback", "TX4"), route!("TX4 XBAR-Playback", "TX4-CIF-Playback"), route!("TX4 XBAR-RX", "TX4 XBAR-Playback"),
    route!("XBAR-Capture", "XBAR-TX"), route!("RX-CIF-Capture", "XBAR-Capture"), route!("RX", "RX-CIF-Capture"), route!("TX4", "RX"), route!("TX4-CIF-Capture", "TX4"), route!("TX4 XBAR-Capture", "TX4-CIF-Capture"), route!("TX4 XBAR-RX", "TX4 XBAR-Capture"),
];

// SOC_SINGLE_EXT control construction depends on ASoC macro layout.
unsafe extern "C" {
    static mut tegra210_adx_controls: [snd_kcontrol_new; 64];
    static mut tegra264_adx_controls: [snd_kcontrol_new; 64];
}

unsafe extern "C" fn tegra210_adx_component_probe(component: *mut snd_soc_component) -> c_int {
    let adx: *mut tegra210_adx = snd_soc_component_get_drvdata(component);

    if (*(*adx).soc_data).num_controls != 0 {
        return snd_soc_add_component_controls(
            component,
            (*(*adx).soc_data).controls,
            (*(*adx).soc_data).num_controls,
        );
    }

    0
}

static tegra210_adx_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tegra210_adx_component_probe),
    dapm_widgets: unsafe { tegra210_adx_widgets.as_ptr() },
    num_dapm_widgets: 5,
    dapm_routes: tegra210_adx_routes.as_ptr(),
    num_dapm_routes: 56,
    controls: unsafe { tegra210_adx_controls.as_ptr() as *mut snd_kcontrol_new },
    num_controls: 64,
};

unsafe extern "C" fn tegra210_adx_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    if (reg >= TEGRA210_ADX_TX_INT_MASK && reg <= TEGRA210_ADX_TX4_CIF_CTRL)
        || (reg >= TEGRA210_ADX_RX_INT_MASK && reg <= TEGRA210_ADX_RX_CIF_CTRL)
        || (reg >= TEGRA210_ADX_ENABLE && reg <= TEGRA210_ADX_CG)
        || (reg >= TEGRA210_ADX_CTRL && reg <= TEGRA210_ADX_IN_BYTE_EN1)
        || (reg >= TEGRA210_ADX_CFG_RAM_CTRL && reg <= TEGRA210_ADX_CFG_RAM_DATA)
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra210_adx_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg >= TEGRA210_ADX_RX_STATUS && reg <= TEGRA210_ADX_CFG_RAM_DATA {
        return true;
    }

    false
}

unsafe extern "C" fn tegra210_adx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg == TEGRA210_ADX_RX_STATUS
        || reg == TEGRA210_ADX_RX_INT_STATUS
        || reg == TEGRA210_ADX_RX_INT_SET
        || reg == TEGRA210_ADX_TX_STATUS
        || reg == TEGRA210_ADX_TX_INT_STATUS
        || reg == TEGRA210_ADX_TX_INT_SET
        || reg == TEGRA210_ADX_SOFT_RESET
        || reg == TEGRA210_ADX_STATUS
        || reg == TEGRA210_ADX_INT_STATUS
        || reg == TEGRA210_ADX_CFG_RAM_CTRL
        || reg == TEGRA210_ADX_CFG_RAM_DATA
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra264_adx_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    if (reg >= TEGRA210_ADX_TX_INT_MASK && reg <= TEGRA210_ADX_TX4_CIF_CTRL)
        || (reg >= TEGRA210_ADX_RX_INT_MASK && reg <= TEGRA210_ADX_RX_CIF_CTRL)
        || (reg >= TEGRA210_ADX_ENABLE && reg <= TEGRA210_ADX_CG)
        || (reg >= TEGRA210_ADX_CTRL && reg <= TEGRA264_ADX_CYA)
        || (reg >= TEGRA264_ADX_CFG_RAM_CTRL && reg <= TEGRA264_ADX_CFG_RAM_DATA)
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra264_adx_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    if (reg >= TEGRA210_ADX_RX_STATUS && reg <= TEGRA210_ADX_RX_CIF_CTRL)
        || (reg >= TEGRA210_ADX_TX_STATUS && reg <= TEGRA210_ADX_TX4_CIF_CTRL)
        || (reg >= TEGRA210_ADX_ENABLE && reg <= TEGRA210_ADX_INT_STATUS)
        || (reg >= TEGRA210_ADX_CTRL && reg <= TEGRA264_ADX_CFG_RAM_DATA)
    {
        return true;
    }

    false
}

unsafe extern "C" fn tegra264_adx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg == TEGRA210_ADX_RX_STATUS
        || reg == TEGRA210_ADX_RX_INT_STATUS
        || reg == TEGRA210_ADX_RX_INT_SET
        || reg == TEGRA210_ADX_TX_STATUS
        || reg == TEGRA210_ADX_TX_INT_STATUS
        || reg == TEGRA210_ADX_TX_INT_SET
        || reg == TEGRA210_ADX_SOFT_RESET
        || reg == TEGRA210_ADX_STATUS
        || reg == TEGRA210_ADX_INT_STATUS
        || reg == TEGRA264_ADX_CFG_RAM_CTRL
        || reg == TEGRA264_ADX_CFG_RAM_DATA
    {
        return true;
    }

    false
}

static tegra210_adx_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { TEGRA210_ADX_CFG_RAM_DATA },
    writeable_reg: Some(tegra210_adx_wr_reg),
    readable_reg: Some(tegra210_adx_rd_reg),
    volatile_reg: Some(tegra210_adx_volatile_reg),
    reg_defaults: unsafe { tegra210_adx_reg_defaults.as_ptr() },
    num_reg_defaults: 9,
    reg_default_cb: Some(regmap_default_zero_cb),
    cache_type: REGCACHE_FLAT,
};

static tegra264_adx_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { TEGRA264_ADX_CFG_RAM_DATA },
    writeable_reg: Some(tegra264_adx_wr_reg),
    readable_reg: Some(tegra264_adx_rd_reg),
    volatile_reg: Some(tegra264_adx_volatile_reg),
    reg_defaults: unsafe { tegra264_adx_reg_defaults.as_ptr() },
    num_reg_defaults: 9,
    reg_default_cb: Some(regmap_default_zero_cb),
    cache_type: REGCACHE_FLAT,
};

static soc_data_tegra210: tegra210_adx_soc_data = tegra210_adx_soc_data {
    regmap_conf: &tegra210_adx_regmap_config,
    max_ch: unsafe { TEGRA210_ADX_MAX_CHANNEL },
    ram_depth: unsafe { TEGRA210_ADX_RAM_DEPTH },
    byte_mask_size: unsafe { TEGRA210_ADX_BYTE_MASK_COUNT },
    cya_offset: unsafe { TEGRA210_ADX_CYA_OFFSET },
    controls: ptr::null_mut(),
    num_controls: 0,
};

static soc_data_tegra264: tegra210_adx_soc_data = tegra210_adx_soc_data {
    regmap_conf: &tegra264_adx_regmap_config,
    max_ch: unsafe { TEGRA264_ADX_MAX_CHANNEL },
    ram_depth: unsafe { TEGRA264_ADX_RAM_DEPTH },
    byte_mask_size: unsafe { TEGRA264_ADX_BYTE_MASK_COUNT },
    cya_offset: unsafe { TEGRA264_ADX_CYA_OFFSET },
    controls: unsafe { tegra264_adx_controls.as_ptr() as *mut snd_kcontrol_new },
    num_controls: 64,
};

static tegra210_adx_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c_str!("nvidia,tegra210-adx"),
        data: &soc_data_tegra210 as *const tegra210_adx_soc_data as *const c_void,
    },
    of_device_id {
        compatible: c_str!("nvidia,tegra264-adx"),
        data: &soc_data_tegra264 as *const tegra210_adx_soc_data as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, tegra210_adx_of_match);

unsafe extern "C" fn tegra210_adx_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let adx: *mut tegra210_adx;
    let soc_data: *const tegra210_adx_soc_data;
    let regs: *mut c_void;
    let mut err: c_int;
    let mut i: c_int;

    adx = devm_kzalloc(dev, size_of::<tegra210_adx>(), GFP_KERNEL) as *mut tegra210_adx;
    if adx.is_null() {
        return -ENOMEM;
    }

    soc_data = of_device_get_match_data(dev);
    (*adx).soc_data = soc_data;

    dev_set_drvdata(dev, adx as *mut c_void);

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*adx).regmap = devm_regmap_init_mmio(dev, regs, (*soc_data).regmap_conf);
    if IS_ERR((*adx).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*adx).regmap as *const c_void),
            c_str!("regmap init failed\n"),
        );
    }

    regcache_cache_only((*adx).regmap, true_);

    (*adx).map = devm_kcalloc(
        dev,
        ((*soc_data).ram_depth.wrapping_mul(TEGRA_ADX_SLOTS_PER_WORD)) as usize,
        size_of::<u16>(),
        GFP_KERNEL,
    ) as *mut u16;
    if (*adx).map.is_null() {
        return -ENOMEM;
    }

    (*adx).byte_mask = devm_kcalloc(
        dev,
        (*soc_data).byte_mask_size as usize,
        size_of::<c_uint>(),
        GFP_KERNEL,
    ) as *mut c_uint;
    if (*adx).byte_mask.is_null() {
        return -ENOMEM;
    }

    /* Initialise all byte map slots as disabled (value 256). */
    i = 0;
    while i < ((*soc_data).ram_depth.wrapping_mul(TEGRA_ADX_SLOTS_PER_WORD)) as c_int {
        *(*adx).map.add(i as usize) = 256;
        i += 1;
    }

    tegra210_adx_dais[TEGRA_ADX_IN_DAI_ID].playback.channels_max = (*(*adx).soc_data).max_ch;

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_adx_cmpnt,
        tegra210_adx_dais.as_mut_ptr(),
        ARRAY_SIZE(&tegra210_adx_dais) as c_int,
    );
    if err != 0 {
        return dev_err_probe(dev, err, c_str!("can't register ADX component\n"));
    }

    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn tegra210_adx_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

// RUNTIME_PM_OPS(tegra210_adx_runtime_suspend, tegra210_adx_runtime_resume, NULL)
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
static tegra210_adx_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut tegra210_adx_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c_str!("tegra210-adx"),
        of_match_table: tegra210_adx_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&tegra210_adx_pm_ops) },
    },
    probe: Some(tegra210_adx_platform_probe),
    remove: Some(tegra210_adx_platform_remove),
};
// module_platform_driver(tegra210_adx_driver);

// MODULE_AUTHOR("Arun Shamanna Lakshmi <aruns@nvidia.com>");
// MODULE_DESCRIPTION("Tegra210 ADX ASoC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
