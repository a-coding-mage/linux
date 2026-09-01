// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2021-2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_amx.c - Tegra210 AMX driver
//
// Rust source-level translation of soc/tegra/tegra210_amx.c.
// C include dependencies are expected to be supplied by surrounding bindings:
// linux/bits.h, linux/clk.h, linux/device.h, linux/io.h, linux/module.h,
// linux/platform_device.h, linux/pm_runtime.h, linux/regmap.h, sound headers,
// tegra210_amx.h, and tegra_cif.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u16 = u16;
type u32 = u32;
type bool_ = bool;

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
    pub id: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
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
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub reg_default_cb: Option<unsafe extern "C" fn(*mut regmap, c_uint) -> c_uint>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct tegra210_amx_soc_data {
    pub regmap_conf: *const regmap_config,
    pub auto_disable: bool_,
    pub max_ch: c_uint,
    pub ram_depth: c_uint,
    pub byte_mask_size: c_uint,
    pub reg_offset: c_uint,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
}

#[repr(C)]
pub struct tegra210_amx {
    pub regmap: *mut regmap,
    pub soc_data: *const tegra210_amx_soc_data,
    pub map: *mut u16,
    pub byte_mask: *mut c_uint,
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

const BITS_PER_BYTE: c_uint = 8;

/*
 * The counter is in terms of AHUB clock cycles. If a frame is not
 * received within these clock cycles, the AMX input channel gets
 * automatically disabled. For now the counter is calculated as a
 * function of sample rate (8 kHz) and AHUB clock (49.152 MHz).
 * If later an accurate number is needed, the counter needs to be
 * calculated at runtime.
 *
 *     count = ahub_clk / sample_rate
 */
const TEGRA194_MAX_FRAME_IDLE_COUNT: c_uint = 0x1800;

const fn AMX_CH_REG(id: c_int, reg: c_uint) -> c_uint {
    reg.wrapping_add((id as c_uint).wrapping_mul(TEGRA210_AMX_AUDIOCIF_CH_STRIDE))
}

extern "C" {
    static mut TEGRA210_AMX_AUDIOCIF_CH_STRIDE: c_uint;
    static mut TEGRA210_AMX_RX_INT_MASK: c_uint;
    static mut TEGRA210_AMX_RX1_CIF_CTRL: c_uint;
    static mut TEGRA210_AMX_RX2_CIF_CTRL: c_uint;
    static mut TEGRA210_AMX_RX3_CIF_CTRL: c_uint;
    static mut TEGRA210_AMX_RX4_CIF_CTRL: c_uint;
    static mut TEGRA210_AMX_TX_INT_MASK: c_uint;
    static mut TEGRA210_AMX_TX_CIF_CTRL: c_uint;
    static mut TEGRA210_AMX_CG: c_uint;
    static mut TEGRA210_AMX_CFG_RAM_CTRL: c_uint;
    static mut TEGRA264_AMX_CFG_RAM_CTRL: c_uint;
    static mut TEGRA210_AMX_CFG_RAM_CTRL_SEQ_ACCESS_EN: c_uint;
    static mut TEGRA210_AMX_CFG_RAM_CTRL_ADDR_INIT_EN: c_uint;
    static mut TEGRA210_AMX_CFG_RAM_CTRL_RW_WRITE: c_uint;
    static mut TEGRA210_AMX_CFG_RAM_DATA: c_uint;
    static mut TEGRA210_AMX_OUT_BYTE_EN0: c_uint;
    static mut TEGRA_AMX_SLOTS_PER_WORD: c_uint;
    static mut TEGRA210_AMX_STATUS: c_uint;
    static mut TEGRA210_AMX_SOFT_RESET: c_uint;
    static mut TEGRA210_AMX_SOFT_RESET_SOFT_RESET_MASK: c_uint;
    static mut TEGRA210_AMX_SOFT_RESET_SOFT_EN: c_uint;
    static mut TEGRA210_AMX_CTRL: c_uint;
    static mut TEGRA210_AMX_CTRL_RX_DEP_MASK: c_uint;
    static mut TEGRA210_AMX_WAIT_ON_ANY: c_uint;
    static mut TEGRA210_AMX_CTRL_RX_DEP_SHIFT: c_uint;
    static mut TEGRA264_AMX_MAX_CHANNEL: c_uint;
    static mut TEGRA210_AMX_MAX_CHANNEL: c_uint;
    static mut TEGRA194_AMX_RX1_FRAME_PERIOD: c_uint;
    static mut TEGRA194_AMX_RX4_FRAME_PERIOD: c_uint;
    static mut TEGRA210_AMX_CYA: c_uint;
    static mut TEGRA_ACIF_BITS_8: c_int;
    static mut TEGRA_ACIF_BITS_16: c_int;
    static mut TEGRA_ACIF_BITS_32: c_int;
    static mut SNDRV_PCM_FORMAT_S8: c_int;
    static mut SNDRV_PCM_FORMAT_S16_LE: c_int;
    static mut SNDRV_PCM_FORMAT_S24_LE: c_int;
    static mut SNDRV_PCM_FORMAT_S32_LE: c_int;
    static mut SNDRV_PCM_RATE_8000_192000: c_uint;
    static mut SNDRV_PCM_FMTBIT_S8: u64;
    static mut SNDRV_PCM_FMTBIT_S16_LE: u64;
    static mut SNDRV_PCM_FMTBIT_S24_LE: u64;
    static mut SNDRV_PCM_FMTBIT_S32_LE: u64;
    static mut TEGRA210_AMX_ENABLE: c_uint;
    static mut TEGRA210_AMX_ENABLE_SHIFT: c_uint;
    static mut TEGRA264_AMX_STREAMS_AUTO_DISABLE: c_uint;
    static mut TEGRA264_AMX_CFG_RAM_DATA: c_uint;
    static mut TEGRA264_AMX_RX1_FRAME_PERIOD: c_uint;
    static mut TEGRA264_AMX_RX4_FRAME_PERIOD: c_uint;
    static mut TEGRA210_AMX_RX_STATUS: c_uint;
    static mut TEGRA210_AMX_RX_INT_STATUS: c_uint;
    static mut TEGRA210_AMX_RX_INT_SET: c_uint;
    static mut TEGRA210_AMX_TX_STATUS: c_uint;
    static mut TEGRA210_AMX_TX_INT_STATUS: c_uint;
    static mut TEGRA210_AMX_TX_INT_SET: c_uint;
    static mut TEGRA210_AMX_INT_STATUS: c_uint;
    static mut TEGRA194_AMX_RX4_LAST_FRAME_PERIOD: c_uint;
    static mut TEGRA264_AMX_RX4_LAST_FRAME_PERIOD: c_uint;
    static mut TEGRA210_AMX_RAM_DEPTH: c_uint;
    static mut TEGRA210_AMX_BYTE_MASK_COUNT: c_uint;
    static mut TEGRA210_AMX_AUTO_DISABLE_OFFSET: c_uint;
    static mut TEGRA264_AMX_RAM_DEPTH: c_uint;
    static mut TEGRA264_AMX_BYTE_MASK_COUNT: c_uint;
    static mut TEGRA264_AMX_AUTO_DISABLE_OFFSET: c_uint;
    static mut TEGRA_AMX_OUT_DAI_ID: usize;
    static mut EINVAL: c_int;
    static mut ENOMEM: c_int;
    static mut GFP_KERNEL: c_uint;
    static mut REGCACHE_FLAT: c_uint;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_default_zero_cb(map: *mut regmap, reg: c_uint) -> c_uint;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *mut snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn tegra264_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn device_get_match_data(dev: *mut device) -> *const tegra210_amx_soc_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
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
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe fn regmap_read_poll_timeout(
    map: *mut regmap,
    reg: c_uint,
    val: &mut c_uint,
    timeout_us: c_uint,
    sleep_us: c_uint,
) -> c_int {
    extern "C" {
        fn regmap_read_poll_timeout_external(
            map: *mut regmap,
            reg: c_uint,
            val: *mut c_uint,
            timeout_us: c_uint,
            sleep_us: c_uint,
        ) -> c_int;
    }
    regmap_read_poll_timeout_external(map, reg, val as *mut c_uint, timeout_us, sleep_us)
}

unsafe fn snd_soc_dapm_aif_in(
    _name: *const c_char,
    _stname: *const c_char,
    _slot: c_int,
    _reg: c_uint,
    _shift: c_uint,
    _invert: c_uint,
) -> snd_soc_dapm_widget {
    zeroed()
}

unsafe fn snd_soc_dapm_aif_out(
    _name: *const c_char,
    _stname: *const c_char,
    _slot: c_int,
    _reg: c_uint,
    _shift: c_uint,
    _invert: c_uint,
) -> snd_soc_dapm_widget {
    zeroed()
}

unsafe fn soc_single_ext(
    _name: *const c_char,
    _reg: c_uint,
    _shift: c_uint,
    _max: c_uint,
    _invert: c_uint,
    _get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    _put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
) -> snd_kcontrol_new {
    zeroed()
}

static mut tegra210_amx_reg_defaults: [reg_default; 9] = [
    reg_default { reg: unsafe { TEGRA210_AMX_RX_INT_MASK }, def: 0x0000000f },
    reg_default { reg: unsafe { TEGRA210_AMX_RX1_CIF_CTRL }, def: 0x00007000 },
    reg_default { reg: unsafe { TEGRA210_AMX_RX2_CIF_CTRL }, def: 0x00007000 },
    reg_default { reg: unsafe { TEGRA210_AMX_RX3_CIF_CTRL }, def: 0x00007000 },
    reg_default { reg: unsafe { TEGRA210_AMX_RX4_CIF_CTRL }, def: 0x00007000 },
    reg_default { reg: unsafe { TEGRA210_AMX_TX_INT_MASK }, def: 0x00000001 },
    reg_default { reg: unsafe { TEGRA210_AMX_TX_CIF_CTRL }, def: 0x00007000 },
    reg_default { reg: unsafe { TEGRA210_AMX_CG }, def: 0x1 },
    reg_default { reg: unsafe { TEGRA210_AMX_CFG_RAM_CTRL }, def: 0x00004000 },
];

static mut tegra264_amx_reg_defaults: [reg_default; 9] = [
    reg_default { reg: unsafe { TEGRA210_AMX_RX_INT_MASK }, def: 0x0000000f },
    reg_default { reg: unsafe { TEGRA210_AMX_RX1_CIF_CTRL }, def: 0x00003800 },
    reg_default { reg: unsafe { TEGRA210_AMX_RX2_CIF_CTRL }, def: 0x00003800 },
    reg_default { reg: unsafe { TEGRA210_AMX_RX3_CIF_CTRL }, def: 0x00003800 },
    reg_default { reg: unsafe { TEGRA210_AMX_RX4_CIF_CTRL }, def: 0x00003800 },
    reg_default { reg: unsafe { TEGRA210_AMX_TX_INT_MASK }, def: 0x00000001 },
    reg_default { reg: unsafe { TEGRA210_AMX_TX_CIF_CTRL }, def: 0x00003800 },
    reg_default { reg: unsafe { TEGRA210_AMX_CG }, def: 0x1 },
    reg_default { reg: unsafe { TEGRA264_AMX_CFG_RAM_CTRL }, def: 0x00004000 },
];

unsafe extern "C" fn tegra210_amx_write_map_ram(amx: *mut tegra210_amx) {
    let bits_per_mask: c_uint = (size_of::<c_uint>() * BITS_PER_BYTE as usize) as c_uint;
    let mut i: c_int;

    memset(
        (*amx).byte_mask as *mut c_void,
        0,
        ((*(*amx).soc_data).byte_mask_size as usize) * size_of::<c_uint>(),
    );

    regmap_write(
        (*amx).regmap,
        TEGRA210_AMX_CFG_RAM_CTRL + (*(*amx).soc_data).reg_offset,
        TEGRA210_AMX_CFG_RAM_CTRL_SEQ_ACCESS_EN
            | TEGRA210_AMX_CFG_RAM_CTRL_ADDR_INIT_EN
            | TEGRA210_AMX_CFG_RAM_CTRL_RW_WRITE,
    );

    i = 0;
    while i < (*(*amx).soc_data).ram_depth as c_int {
        let mut word: u32 = 0;
        let mut b: c_int = 0;

        while b < TEGRA_AMX_SLOTS_PER_WORD as c_int {
            let slot: c_uint = (i as c_uint)
                .wrapping_mul(TEGRA_AMX_SLOTS_PER_WORD)
                .wrapping_add(b as c_uint);
            let val: u16 = *(*amx).map.add(slot as usize);

            if val < 256 {
                word |= (val as u32) << ((b as c_uint).wrapping_mul(BITS_PER_BYTE));
                *(*amx).byte_mask.add((slot / bits_per_mask) as usize) |=
                    1u32 << (slot % bits_per_mask);
            }

            b += 1;
        }

        regmap_write(
            (*amx).regmap,
            TEGRA210_AMX_CFG_RAM_DATA + (*(*amx).soc_data).reg_offset,
            word,
        );
        i += 1;
    }

    i = 0;
    while i < (*(*amx).soc_data).byte_mask_size as c_int {
        regmap_write(
            (*amx).regmap,
            TEGRA210_AMX_OUT_BYTE_EN0
                + ((i as c_uint).wrapping_mul(TEGRA210_AMX_AUDIOCIF_CH_STRIDE)),
            *(*amx).byte_mask.add(i as usize),
        );
        i += 1;
    }
}

unsafe extern "C" fn tegra210_amx_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let amx = snd_soc_dai_get_drvdata(dai) as *mut tegra210_amx;
    let mut val: c_uint = 0;
    let mut err: c_int;

    /* Ensure if AMX is disabled */
    err = regmap_read_poll_timeout((*amx).regmap, TEGRA210_AMX_STATUS, &mut val, 10, 10000);
    if err < 0 || (val & 0x1) != 0 {
        dev_err((*dai).dev, b"failed to stop AMX, err = %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    /*
     * Soft Reset: Below performs module soft reset which clears
     * all FSM logic, flushes flow control of FIFO and resets the
     * state register. It also brings module back to disabled
     * state (without flushing the data in the pipe).
     */
    regmap_update_bits(
        (*amx).regmap,
        TEGRA210_AMX_SOFT_RESET,
        TEGRA210_AMX_SOFT_RESET_SOFT_RESET_MASK,
        TEGRA210_AMX_SOFT_RESET_SOFT_EN,
    );

    err = regmap_read_poll_timeout((*amx).regmap, TEGRA210_AMX_SOFT_RESET, &mut val, 10, 10000);
    if err < 0 || (val & 0x1) != 0 {
        dev_err((*dai).dev, b"failed to reset AMX, err = %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    0
}

unsafe extern "C" fn tegra210_amx_runtime_suspend(dev: *mut device) -> c_int {
    let amx = dev_get_drvdata(dev) as *mut tegra210_amx;

    regcache_cache_only((*amx).regmap, true);
    regcache_mark_dirty((*amx).regmap);

    0
}

unsafe extern "C" fn tegra210_amx_runtime_resume(dev: *mut device) -> c_int {
    let amx = dev_get_drvdata(dev) as *mut tegra210_amx;

    regcache_cache_only((*amx).regmap, false);
    regcache_sync((*amx).regmap);

    regmap_update_bits(
        (*amx).regmap,
        TEGRA210_AMX_CTRL,
        TEGRA210_AMX_CTRL_RX_DEP_MASK,
        TEGRA210_AMX_WAIT_ON_ANY << TEGRA210_AMX_CTRL_RX_DEP_SHIFT,
    );

    tegra210_amx_write_map_ram(amx);

    0
}

unsafe extern "C" fn tegra210_amx_set_audio_cif(
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
    reg: c_uint,
) -> c_int {
    let amx = snd_soc_dai_get_drvdata(dai) as *mut tegra210_amx;
    let channels: c_int;
    let audio_bits: c_int;
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
            dev_err(
                (*dai).dev,
                b"unsupported format: %d\n\0".as_ptr() as *const c_char,
                params_format(params),
            );
            return -EINVAL;
        }
    }

    cif_conf.audio_ch = channels;
    cif_conf.client_ch = channels;
    cif_conf.audio_bits = audio_bits;
    cif_conf.client_bits = audio_bits;

    if (*(*amx).soc_data).max_ch == TEGRA264_AMX_MAX_CHANNEL {
        tegra264_set_cif((*amx).regmap, reg, &mut cif_conf);
    } else {
        tegra_set_cif((*amx).regmap, reg, &mut cif_conf);
    }

    0
}

unsafe extern "C" fn tegra210_amx_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let amx = snd_soc_dai_get_drvdata(dai) as *mut tegra210_amx;

    if (*(*amx).soc_data).auto_disable {
        regmap_write(
            (*amx).regmap,
            AMX_CH_REG(
                (*dai).id,
                TEGRA194_AMX_RX1_FRAME_PERIOD + (*(*amx).soc_data).reg_offset,
            ),
            TEGRA194_MAX_FRAME_IDLE_COUNT,
        );
        regmap_write(
            (*amx).regmap,
            TEGRA210_AMX_CYA + (*(*amx).soc_data).reg_offset,
            1,
        );
    }

    tegra210_amx_set_audio_cif(dai, params, AMX_CH_REG((*dai).id, TEGRA210_AMX_RX1_CIF_CTRL))
}

unsafe extern "C" fn tegra210_amx_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    tegra210_amx_set_audio_cif(dai, params, TEGRA210_AMX_TX_CIF_CTRL)
}

unsafe extern "C" fn tegra210_amx_get_byte_map(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let amx = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_amx;

    (*ucontrol).value.integer.value[0] = *(*amx).map.add((*mc).reg as usize) as i64;

    0
}

unsafe extern "C" fn tegra210_amx_put_byte_map(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let amx = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_amx;
    let mut value: c_uint = (*ucontrol).value.integer.value[0] as c_uint;

    /*
     * Match the previous behaviour: any value outside [0, 255] is
     * treated as the "disabled" sentinel (256). Negative values from
     * userspace fold in through the unsigned cast and are caught here.
     */
    if value > 255 {
        value = 256;
    }

    if *(*amx).map.add((*mc).reg as usize) == value as u16 {
        return 0;
    }

    *(*amx).map.add((*mc).reg as usize) = value as u16;

    1
}

static tegra210_amx_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_amx_out_hw_params),
    startup: Some(tegra210_amx_startup),
};

static tegra210_amx_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_amx_in_hw_params),
    startup: None,
};

unsafe fn dai_formats() -> u64 {
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

unsafe fn in_dai(id: c_uint) -> snd_soc_dai_driver {
    let name = match id {
        1 => b"AMX-RX-CIF1\0".as_ptr() as *const c_char,
        2 => b"AMX-RX-CIF2\0".as_ptr() as *const c_char,
        3 => b"AMX-RX-CIF3\0".as_ptr() as *const c_char,
        _ => b"AMX-RX-CIF4\0".as_ptr() as *const c_char,
    };
    let playback = match id {
        1 => b"RX1-CIF-Playback\0".as_ptr() as *const c_char,
        2 => b"RX2-CIF-Playback\0".as_ptr() as *const c_char,
        3 => b"RX3-CIF-Playback\0".as_ptr() as *const c_char,
        _ => b"RX4-CIF-Playback\0".as_ptr() as *const c_char,
    };
    let capture = match id {
        1 => b"RX1-CIF-Capture\0".as_ptr() as *const c_char,
        2 => b"RX2-CIF-Capture\0".as_ptr() as *const c_char,
        3 => b"RX3-CIF-Capture\0".as_ptr() as *const c_char,
        _ => b"RX4-CIF-Capture\0".as_ptr() as *const c_char,
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
        ops: &tegra210_amx_in_dai_ops,
    }
}

unsafe fn out_dai() -> snd_soc_dai_driver {
    snd_soc_dai_driver {
        name: b"AMX-TX-CIF\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: dai_formats(),
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: dai_formats(),
        },
        ops: &tegra210_amx_out_dai_ops,
    }
}

static mut tegra210_amx_dais: [snd_soc_dai_driver; 5] = unsafe {
    [in_dai(1), in_dai(2), in_dai(3), in_dai(4), out_dai()]
};

static mut tegra210_amx_widgets: [snd_soc_dapm_widget; 5] = unsafe {
    [
        snd_soc_dapm_aif_in(b"RX1\0".as_ptr() as *const c_char, ptr::null(), 0, TEGRA210_AMX_CTRL, 0, 0),
        snd_soc_dapm_aif_in(b"RX2\0".as_ptr() as *const c_char, ptr::null(), 0, TEGRA210_AMX_CTRL, 1, 0),
        snd_soc_dapm_aif_in(b"RX3\0".as_ptr() as *const c_char, ptr::null(), 0, TEGRA210_AMX_CTRL, 2, 0),
        snd_soc_dapm_aif_in(b"RX4\0".as_ptr() as *const c_char, ptr::null(), 0, TEGRA210_AMX_CTRL, 3, 0),
        snd_soc_dapm_aif_out(
            b"TX\0".as_ptr() as *const c_char,
            ptr::null(),
            0,
            TEGRA210_AMX_ENABLE,
            TEGRA210_AMX_ENABLE_SHIFT,
            0,
        ),
    ]
};

macro_rules! route {
    ($sink:expr, $source:expr) => {
        snd_soc_dapm_route {
            sink: concat!($sink, "\0").as_ptr() as *const c_char,
            control: ptr::null(),
            source: concat!($source, "\0").as_ptr() as *const c_char,
        }
    };
}

macro_rules! stream_routes {
    ($id:literal, $sname:literal) => {
        route!(concat!("RX", $id, " XBAR-", $sname), concat!("RX", $id, " XBAR-TX")),
        route!(concat!("RX", $id, "-CIF-", $sname), concat!("RX", $id, " XBAR-", $sname)),
        route!(concat!("RX", $id), concat!("RX", $id, "-CIF-", $sname)),
        route!("TX", concat!("RX", $id)),
        route!(concat!("TX-CIF-", $sname), "TX"),
        route!(concat!("XBAR-", $sname), concat!("TX-CIF-", $sname)),
        route!("XBAR-RX", concat!("XBAR-", $sname))
    };
}

static tegra210_amx_routes: [snd_soc_dapm_route; 56] = [
    stream_routes!("1", "Playback"),
    stream_routes!("1", "Capture"),
    stream_routes!("2", "Playback"),
    stream_routes!("2", "Capture"),
    stream_routes!("3", "Playback"),
    stream_routes!("3", "Capture"),
    stream_routes!("4", "Playback"),
    stream_routes!("4", "Capture"),
];

unsafe fn tegra210_amx_byte_map_ctrl(reg: c_uint) -> snd_kcontrol_new {
    match reg {
        0 => soc_single_ext(b"Byte Map 0\0".as_ptr() as *const c_char, reg, 0, 256, 0, tegra210_amx_get_byte_map, tegra210_amx_put_byte_map),
        1 => soc_single_ext(b"Byte Map 1\0".as_ptr() as *const c_char, reg, 0, 256, 0, tegra210_amx_get_byte_map, tegra210_amx_put_byte_map),
        2 => soc_single_ext(b"Byte Map 2\0".as_ptr() as *const c_char, reg, 0, 256, 0, tegra210_amx_get_byte_map, tegra210_amx_put_byte_map),
        3 => soc_single_ext(b"Byte Map 3\0".as_ptr() as *const c_char, reg, 0, 256, 0, tegra210_amx_get_byte_map, tegra210_amx_put_byte_map),
        _ => soc_single_ext(b"Byte Map\0".as_ptr() as *const c_char, reg, 0, 256, 0, tegra210_amx_get_byte_map, tegra210_amx_put_byte_map),
    }
}

static mut tegra210_amx_controls: [snd_kcontrol_new; 64] = unsafe {
    [
        tegra210_amx_byte_map_ctrl(0), tegra210_amx_byte_map_ctrl(1),
        tegra210_amx_byte_map_ctrl(2), tegra210_amx_byte_map_ctrl(3),
        tegra210_amx_byte_map_ctrl(4), tegra210_amx_byte_map_ctrl(5),
        tegra210_amx_byte_map_ctrl(6), tegra210_amx_byte_map_ctrl(7),
        tegra210_amx_byte_map_ctrl(8), tegra210_amx_byte_map_ctrl(9),
        tegra210_amx_byte_map_ctrl(10), tegra210_amx_byte_map_ctrl(11),
        tegra210_amx_byte_map_ctrl(12), tegra210_amx_byte_map_ctrl(13),
        tegra210_amx_byte_map_ctrl(14), tegra210_amx_byte_map_ctrl(15),
        tegra210_amx_byte_map_ctrl(16), tegra210_amx_byte_map_ctrl(17),
        tegra210_amx_byte_map_ctrl(18), tegra210_amx_byte_map_ctrl(19),
        tegra210_amx_byte_map_ctrl(20), tegra210_amx_byte_map_ctrl(21),
        tegra210_amx_byte_map_ctrl(22), tegra210_amx_byte_map_ctrl(23),
        tegra210_amx_byte_map_ctrl(24), tegra210_amx_byte_map_ctrl(25),
        tegra210_amx_byte_map_ctrl(26), tegra210_amx_byte_map_ctrl(27),
        tegra210_amx_byte_map_ctrl(28), tegra210_amx_byte_map_ctrl(29),
        tegra210_amx_byte_map_ctrl(30), tegra210_amx_byte_map_ctrl(31),
        tegra210_amx_byte_map_ctrl(32), tegra210_amx_byte_map_ctrl(33),
        tegra210_amx_byte_map_ctrl(34), tegra210_amx_byte_map_ctrl(35),
        tegra210_amx_byte_map_ctrl(36), tegra210_amx_byte_map_ctrl(37),
        tegra210_amx_byte_map_ctrl(38), tegra210_amx_byte_map_ctrl(39),
        tegra210_amx_byte_map_ctrl(40), tegra210_amx_byte_map_ctrl(41),
        tegra210_amx_byte_map_ctrl(42), tegra210_amx_byte_map_ctrl(43),
        tegra210_amx_byte_map_ctrl(44), tegra210_amx_byte_map_ctrl(45),
        tegra210_amx_byte_map_ctrl(46), tegra210_amx_byte_map_ctrl(47),
        tegra210_amx_byte_map_ctrl(48), tegra210_amx_byte_map_ctrl(49),
        tegra210_amx_byte_map_ctrl(50), tegra210_amx_byte_map_ctrl(51),
        tegra210_amx_byte_map_ctrl(52), tegra210_amx_byte_map_ctrl(53),
        tegra210_amx_byte_map_ctrl(54), tegra210_amx_byte_map_ctrl(55),
        tegra210_amx_byte_map_ctrl(56), tegra210_amx_byte_map_ctrl(57),
        tegra210_amx_byte_map_ctrl(58), tegra210_amx_byte_map_ctrl(59),
        tegra210_amx_byte_map_ctrl(60), tegra210_amx_byte_map_ctrl(61),
        tegra210_amx_byte_map_ctrl(62), tegra210_amx_byte_map_ctrl(63),
    ]
};

static mut tegra264_amx_controls: [snd_kcontrol_new; 64] = unsafe {
    [
        tegra210_amx_byte_map_ctrl(64), tegra210_amx_byte_map_ctrl(65),
        tegra210_amx_byte_map_ctrl(66), tegra210_amx_byte_map_ctrl(67),
        tegra210_amx_byte_map_ctrl(68), tegra210_amx_byte_map_ctrl(69),
        tegra210_amx_byte_map_ctrl(70), tegra210_amx_byte_map_ctrl(71),
        tegra210_amx_byte_map_ctrl(72), tegra210_amx_byte_map_ctrl(73),
        tegra210_amx_byte_map_ctrl(74), tegra210_amx_byte_map_ctrl(75),
        tegra210_amx_byte_map_ctrl(76), tegra210_amx_byte_map_ctrl(77),
        tegra210_amx_byte_map_ctrl(78), tegra210_amx_byte_map_ctrl(79),
        tegra210_amx_byte_map_ctrl(80), tegra210_amx_byte_map_ctrl(81),
        tegra210_amx_byte_map_ctrl(82), tegra210_amx_byte_map_ctrl(83),
        tegra210_amx_byte_map_ctrl(84), tegra210_amx_byte_map_ctrl(85),
        tegra210_amx_byte_map_ctrl(86), tegra210_amx_byte_map_ctrl(87),
        tegra210_amx_byte_map_ctrl(88), tegra210_amx_byte_map_ctrl(89),
        tegra210_amx_byte_map_ctrl(90), tegra210_amx_byte_map_ctrl(91),
        tegra210_amx_byte_map_ctrl(92), tegra210_amx_byte_map_ctrl(93),
        tegra210_amx_byte_map_ctrl(94), tegra210_amx_byte_map_ctrl(95),
        tegra210_amx_byte_map_ctrl(96), tegra210_amx_byte_map_ctrl(97),
        tegra210_amx_byte_map_ctrl(98), tegra210_amx_byte_map_ctrl(99),
        tegra210_amx_byte_map_ctrl(100), tegra210_amx_byte_map_ctrl(101),
        tegra210_amx_byte_map_ctrl(102), tegra210_amx_byte_map_ctrl(103),
        tegra210_amx_byte_map_ctrl(104), tegra210_amx_byte_map_ctrl(105),
        tegra210_amx_byte_map_ctrl(106), tegra210_amx_byte_map_ctrl(107),
        tegra210_amx_byte_map_ctrl(108), tegra210_amx_byte_map_ctrl(109),
        tegra210_amx_byte_map_ctrl(110), tegra210_amx_byte_map_ctrl(111),
        tegra210_amx_byte_map_ctrl(112), tegra210_amx_byte_map_ctrl(113),
        tegra210_amx_byte_map_ctrl(114), tegra210_amx_byte_map_ctrl(115),
        tegra210_amx_byte_map_ctrl(116), tegra210_amx_byte_map_ctrl(117),
        tegra210_amx_byte_map_ctrl(118), tegra210_amx_byte_map_ctrl(119),
        tegra210_amx_byte_map_ctrl(120), tegra210_amx_byte_map_ctrl(121),
        tegra210_amx_byte_map_ctrl(122), tegra210_amx_byte_map_ctrl(123),
        tegra210_amx_byte_map_ctrl(124), tegra210_amx_byte_map_ctrl(125),
        tegra210_amx_byte_map_ctrl(126), tegra210_amx_byte_map_ctrl(127),
    ]
};

unsafe extern "C" fn tegra210_amx_component_probe(component: *mut snd_soc_component) -> c_int {
    let amx = snd_soc_component_get_drvdata(component) as *mut tegra210_amx;

    if (*(*amx).soc_data).num_controls != 0 {
        return snd_soc_add_component_controls(
            component,
            (*(*amx).soc_data).controls,
            (*(*amx).soc_data).num_controls,
        );
    }

    0
}

static mut tegra210_amx_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tegra210_amx_component_probe),
    dapm_widgets: unsafe { tegra210_amx_widgets.as_ptr() },
    num_dapm_widgets: 5,
    dapm_routes: tegra210_amx_routes.as_ptr(),
    num_dapm_routes: 56,
    controls: unsafe { tegra210_amx_controls.as_mut_ptr() },
    num_controls: 64,
};

unsafe extern "C" fn tegra210_amx_wr_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r >= TEGRA210_AMX_RX_INT_MASK && r <= TEGRA210_AMX_RX4_CIF_CTRL => true,
        r if r >= TEGRA210_AMX_TX_INT_MASK && r <= TEGRA210_AMX_CG => true,
        r if r >= TEGRA210_AMX_CTRL && r <= TEGRA210_AMX_CYA => true,
        r if r >= TEGRA210_AMX_CFG_RAM_CTRL && r <= TEGRA210_AMX_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra194_amx_wr_reg(dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r >= TEGRA194_AMX_RX1_FRAME_PERIOD && r <= TEGRA194_AMX_RX4_FRAME_PERIOD => true,
        _ => tegra210_amx_wr_reg(dev, reg),
    }
}

unsafe extern "C" fn tegra264_amx_wr_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r >= TEGRA210_AMX_RX_INT_MASK && r <= TEGRA210_AMX_RX4_CIF_CTRL => true,
        r if r >= TEGRA210_AMX_TX_INT_MASK && r <= TEGRA210_AMX_TX_CIF_CTRL => true,
        r if r >= TEGRA210_AMX_ENABLE && r <= TEGRA210_AMX_CG => true,
        r if r >= TEGRA210_AMX_CTRL && r <= TEGRA264_AMX_STREAMS_AUTO_DISABLE => true,
        r if r >= TEGRA264_AMX_CFG_RAM_CTRL && r <= TEGRA264_AMX_CFG_RAM_DATA => true,
        r if r >= TEGRA264_AMX_RX1_FRAME_PERIOD && r <= TEGRA264_AMX_RX4_FRAME_PERIOD => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_amx_rd_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r >= TEGRA210_AMX_RX_STATUS && r <= TEGRA210_AMX_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra194_amx_rd_reg(dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r >= TEGRA194_AMX_RX1_FRAME_PERIOD && r <= TEGRA194_AMX_RX4_FRAME_PERIOD => true,
        _ => tegra210_amx_rd_reg(dev, reg),
    }
}

unsafe extern "C" fn tegra264_amx_rd_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r >= TEGRA210_AMX_RX_STATUS && r <= TEGRA210_AMX_RX4_CIF_CTRL => true,
        r if r >= TEGRA210_AMX_TX_STATUS && r <= TEGRA210_AMX_TX_CIF_CTRL => true,
        r if r >= TEGRA210_AMX_ENABLE && r <= TEGRA210_AMX_INT_STATUS => true,
        r if r >= TEGRA210_AMX_CTRL && r <= TEGRA264_AMX_CFG_RAM_DATA => true,
        r if r >= TEGRA264_AMX_RX1_FRAME_PERIOD && r <= TEGRA264_AMX_RX4_FRAME_PERIOD => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_amx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r == TEGRA210_AMX_RX_STATUS => true,
        r if r == TEGRA210_AMX_RX_INT_STATUS => true,
        r if r == TEGRA210_AMX_RX_INT_SET => true,
        r if r == TEGRA210_AMX_TX_STATUS => true,
        r if r == TEGRA210_AMX_TX_INT_STATUS => true,
        r if r == TEGRA210_AMX_TX_INT_SET => true,
        r if r == TEGRA210_AMX_SOFT_RESET => true,
        r if r == TEGRA210_AMX_STATUS => true,
        r if r == TEGRA210_AMX_INT_STATUS => true,
        r if r == TEGRA210_AMX_CFG_RAM_CTRL => true,
        r if r == TEGRA210_AMX_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra264_amx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        r if r == TEGRA210_AMX_RX_STATUS => true,
        r if r == TEGRA210_AMX_RX_INT_STATUS => true,
        r if r == TEGRA210_AMX_RX_INT_SET => true,
        r if r == TEGRA210_AMX_TX_STATUS => true,
        r if r == TEGRA210_AMX_TX_INT_STATUS => true,
        r if r == TEGRA210_AMX_TX_INT_SET => true,
        r if r == TEGRA210_AMX_SOFT_RESET => true,
        r if r == TEGRA210_AMX_STATUS => true,
        r if r == TEGRA210_AMX_INT_STATUS => true,
        r if r == TEGRA264_AMX_CFG_RAM_CTRL => true,
        r if r == TEGRA264_AMX_CFG_RAM_DATA => true,
        _ => false,
    }
}

static mut tegra210_amx_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        reg_stride: 4,
        val_bits: 32,
        max_register: TEGRA210_AMX_CFG_RAM_DATA,
        writeable_reg: Some(tegra210_amx_wr_reg),
        readable_reg: Some(tegra210_amx_rd_reg),
        volatile_reg: Some(tegra210_amx_volatile_reg),
        reg_defaults: tegra210_amx_reg_defaults.as_ptr(),
        num_reg_defaults: 9,
        reg_default_cb: Some(regmap_default_zero_cb),
        cache_type: REGCACHE_FLAT,
    }
};

static mut tegra194_amx_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        reg_stride: 4,
        val_bits: 32,
        max_register: TEGRA194_AMX_RX4_LAST_FRAME_PERIOD,
        writeable_reg: Some(tegra194_amx_wr_reg),
        readable_reg: Some(tegra194_amx_rd_reg),
        volatile_reg: Some(tegra210_amx_volatile_reg),
        reg_defaults: tegra210_amx_reg_defaults.as_ptr(),
        num_reg_defaults: 9,
        reg_default_cb: Some(regmap_default_zero_cb),
        cache_type: REGCACHE_FLAT,
    }
};

static mut tegra264_amx_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        reg_stride: 4,
        val_bits: 32,
        max_register: TEGRA264_AMX_RX4_LAST_FRAME_PERIOD,
        writeable_reg: Some(tegra264_amx_wr_reg),
        readable_reg: Some(tegra264_amx_rd_reg),
        volatile_reg: Some(tegra264_amx_volatile_reg),
        reg_defaults: tegra264_amx_reg_defaults.as_ptr(),
        num_reg_defaults: 9,
        reg_default_cb: Some(regmap_default_zero_cb),
        cache_type: REGCACHE_FLAT,
    }
};

static mut soc_data_tegra210: tegra210_amx_soc_data = unsafe {
    tegra210_amx_soc_data {
        regmap_conf: &tegra210_amx_regmap_config,
        auto_disable: false,
        max_ch: TEGRA210_AMX_MAX_CHANNEL,
        ram_depth: TEGRA210_AMX_RAM_DEPTH,
        byte_mask_size: TEGRA210_AMX_BYTE_MASK_COUNT,
        reg_offset: TEGRA210_AMX_AUTO_DISABLE_OFFSET,
        controls: ptr::null_mut(),
        num_controls: 0,
    }
};

static mut soc_data_tegra194: tegra210_amx_soc_data = unsafe {
    tegra210_amx_soc_data {
        regmap_conf: &tegra194_amx_regmap_config,
        auto_disable: true,
        max_ch: TEGRA210_AMX_MAX_CHANNEL,
        ram_depth: TEGRA210_AMX_RAM_DEPTH,
        byte_mask_size: TEGRA210_AMX_BYTE_MASK_COUNT,
        reg_offset: TEGRA210_AMX_AUTO_DISABLE_OFFSET,
        controls: ptr::null_mut(),
        num_controls: 0,
    }
};

static mut soc_data_tegra264: tegra210_amx_soc_data = unsafe {
    tegra210_amx_soc_data {
        regmap_conf: &tegra264_amx_regmap_config,
        auto_disable: true,
        max_ch: TEGRA264_AMX_MAX_CHANNEL,
        ram_depth: TEGRA264_AMX_RAM_DEPTH,
        byte_mask_size: TEGRA264_AMX_BYTE_MASK_COUNT,
        reg_offset: TEGRA264_AMX_AUTO_DISABLE_OFFSET,
        controls: tegra264_amx_controls.as_mut_ptr(),
        num_controls: 64,
    }
};

static mut tegra210_amx_of_match: [of_device_id; 4] = unsafe {
    [
        of_device_id {
            compatible: b"nvidia,tegra210-amx\0".as_ptr() as *const c_char,
            data: &soc_data_tegra210 as *const _ as *const c_void,
        },
        of_device_id {
            compatible: b"nvidia,tegra194-amx\0".as_ptr() as *const c_char,
            data: &soc_data_tegra194 as *const _ as *const c_void,
        },
        of_device_id {
            compatible: b"nvidia,tegra264-amx\0".as_ptr() as *const c_char,
            data: &soc_data_tegra264 as *const _ as *const c_void,
        },
        of_device_id {
            compatible: ptr::null(),
            data: ptr::null(),
        },
    ]
};
// MODULE_DEVICE_TABLE(of, tegra210_amx_of_match);

unsafe extern "C" fn tegra210_amx_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let amx: *mut tegra210_amx;
    let regs: *mut c_void;
    let mut err: c_int;
    let mut i: c_int;

    amx = devm_kzalloc(dev, size_of::<tegra210_amx>(), GFP_KERNEL) as *mut tegra210_amx;
    if amx.is_null() {
        return -ENOMEM;
    }

    (*amx).soc_data = device_get_match_data(dev);

    dev_set_drvdata(dev, amx as *mut c_void);

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*amx).regmap = devm_regmap_init_mmio(dev, regs, (*(*amx).soc_data).regmap_conf);
    if IS_ERR((*amx).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*amx).regmap as *const c_void),
            b"regmap init failed\n\0".as_ptr() as *const c_char,
        );
    }

    regcache_cache_only((*amx).regmap, true);

    (*amx).map = devm_kcalloc(
        dev,
        ((*(*amx).soc_data).ram_depth * TEGRA_AMX_SLOTS_PER_WORD) as usize,
        size_of::<u16>(),
        GFP_KERNEL,
    ) as *mut u16;
    if (*amx).map.is_null() {
        return -ENOMEM;
    }

    (*amx).byte_mask = devm_kcalloc(
        dev,
        (*(*amx).soc_data).byte_mask_size as usize,
        size_of::<c_uint>(),
        GFP_KERNEL,
    ) as *mut c_uint;
    if (*amx).byte_mask.is_null() {
        return -ENOMEM;
    }

    /* Initialise all byte map slots as disabled (value 256). */
    i = 0;
    while i < ((*(*amx).soc_data).ram_depth * TEGRA_AMX_SLOTS_PER_WORD) as c_int {
        *(*amx).map.add(i as usize) = 256;
        i += 1;
    }

    tegra210_amx_dais[TEGRA_AMX_OUT_DAI_ID].capture.channels_max = (*(*amx).soc_data).max_ch;

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_amx_cmpnt,
        tegra210_amx_dais.as_mut_ptr(),
        tegra210_amx_dais.len() as c_int,
    );
    if err != 0 {
        return dev_err_probe(
            dev,
            err,
            b"can't register AMX component\n\0".as_ptr() as *const c_char,
        );
    }

    pm_runtime_enable(dev);

    0
}

unsafe extern "C" fn tegra210_amx_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev as *mut device);
}

static tegra210_amx_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };
// RUNTIME_PM_OPS(tegra210_amx_runtime_suspend, tegra210_amx_runtime_resume, NULL)
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)

static mut tegra210_amx_driver: platform_driver = unsafe {
    platform_driver {
        driver: platform_driver_driver {
            name: b"tegra210-amx\0".as_ptr() as *const c_char,
            of_match_table: tegra210_amx_of_match.as_ptr(),
            pm: pm_ptr(&tegra210_amx_pm_ops),
        },
        probe: Some(tegra210_amx_platform_probe),
        remove: Some(tegra210_amx_platform_remove),
    }
};
// module_platform_driver(tegra210_amx_driver);

// MODULE_AUTHOR("Songhee Baek <sbaek@nvidia.com>");
// MODULE_DESCRIPTION("Tegra210 AMX ASoC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
