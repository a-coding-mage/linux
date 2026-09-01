// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_admaif.c - Tegra ADMAIF driver

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params;
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub id: c_int,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_soc_component;

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
    pub item: [c_uint; 4],
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
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
#[derive(Copy, Clone)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: c_uint,
    pub chan_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
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
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub info: Option<unsafe extern "C" fn()>,
    pub name: *const c_char,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
    pub pcm_new: Option<unsafe extern "C" fn()>,
    pub open: Option<unsafe extern "C" fn()>,
    pub close: Option<unsafe extern "C" fn()>,
    pub hw_params: Option<unsafe extern "C" fn()>,
    pub pointer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
    pub client_ch: c_int,
    pub audio_ch: c_int,
    pub mono_conv: c_uint,
    pub stereo_conv: c_uint,
}

#[repr(C)]
pub struct tegra_admaif_soc_data {
    pub num_ch: c_uint,
    pub max_stream_ch: c_uint,
    pub cmpnt: *const snd_soc_component_driver,
    pub dais: *mut snd_soc_dai_driver,
    pub regmap_conf: *const regmap_config,
    pub global_base: c_uint,
    pub tx_base: c_uint,
    pub rx_base: c_uint,
}

#[repr(C)]
pub struct tegra_admaif {
    pub regmap: *mut regmap,
    pub soc_data: *const tegra_admaif_soc_data,
    pub playback_dma_data: *mut snd_dmaengine_dai_dma_data,
    pub mono_to_stereo: [*mut c_uint; ADMAIF_PATHS as usize],
    pub stereo_to_mono: [*mut c_uint; ADMAIF_PATHS as usize],
    pub capture_dma_data: [snd_dmaengine_dai_dma_data; 0],
}

pub type resource_size_t = u64;
pub type dma_addr_t = u64;

extern "C" {
    static regmap_default_zero_cb: unsafe extern "C" fn(*mut regmap, c_uint) -> c_uint;
    static tegra_pcm_new: unsafe extern "C" fn();
    static tegra_pcm_open: unsafe extern "C" fn();
    static tegra_pcm_close: unsafe extern "C" fn();
    static tegra_pcm_hw_params: unsafe extern "C" fn();
    static tegra_pcm_pointer: unsafe extern "C" fn();
    static pm_runtime_force_suspend: unsafe extern "C" fn(*mut device) -> c_int;
    static pm_runtime_force_resume: unsafe extern "C" fn(*mut device) -> c_int;
    static snd_soc_info_enum_double: unsafe extern "C" fn();

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn tegra_isomgr_adma_setbw(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai, enable: bool) -> c_int;
    fn tegra_isomgr_adma_register(dev: *mut device) -> c_int;
    fn tegra_isomgr_adma_unregister(dev: *mut device);
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn tegra264_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn of_property_read_string_index(np: *mut device_node, propname: *const c_char, index: c_int, output: *mut *const c_char) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

extern "C" {
    static TEGRA210_ADMAIF_RX1_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX2_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX3_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX4_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX5_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX6_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX7_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX8_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX9_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_RX10_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX1_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX2_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX3_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX4_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX5_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX6_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX7_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX8_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX9_FIFO_CTRL_REG_DEFAULT: c_uint;
    static TEGRA210_ADMAIF_TX10_FIFO_CTRL_REG_DEFAULT: c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EOPNOTSUPP: c_int = 95;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: u64 = 1u64 << SNDRV_PCM_FORMAT_S8;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S32_LE;

extern "C" {
    static TEGRA_ADMAIF_CHANNEL_REG_STRIDE: c_uint;
    static TEGRA_ADMAIF_RX_INT_MASK: c_uint;
    static TEGRA_ADMAIF_CH_ACIF_RX_CTRL: c_uint;
    static TEGRA_ADMAIF_RX_FIFO_CTRL: c_uint;
    static TEGRA_ADMAIF_TX_INT_MASK: c_uint;
    static TEGRA_ADMAIF_CH_ACIF_TX_CTRL: c_uint;
    static TEGRA_ADMAIF_TX_FIFO_CTRL: c_uint;
    static TEGRA_ADMAIF_GLOBAL_CG_0: c_uint;
    static TEGRA_ADMAIF_RX_ENABLE: c_uint;
    static TEGRA_ADMAIF_RX_SOFT_RESET: c_uint;
    static TEGRA_ADMAIF_TX_ENABLE: c_uint;
    static TEGRA_ADMAIF_TX_SOFT_RESET: c_uint;
    static TEGRA_ADMAIF_GLOBAL_ENABLE: c_uint;
    static TEGRA_ADMAIF_RX_STATUS: c_uint;
    static TEGRA_ADMAIF_RX_INT_STATUS: c_uint;
    static TEGRA_ADMAIF_TX_STATUS: c_uint;
    static TEGRA_ADMAIF_TX_INT_STATUS: c_uint;
    static TEGRA_ADMAIF_GLOBAL_STATUS: c_uint;
    static TEGRA_ADMAIF_GLOBAL_RX_ENABLE_STATUS: c_uint;
    static TEGRA_ADMAIF_GLOBAL_TX_ENABLE_STATUS: c_uint;
    static TEGRA_ADMAIF_TX_FIFO_WRITE: c_uint;
    static TEGRA_ADMAIF_RX_FIFO_READ: c_uint;
    static TEGRA210_ADMAIF_GLOBAL_BASE: c_uint;
    static TEGRA210_ADMAIF_TX_BASE: c_uint;
    static TEGRA210_ADMAIF_RX_BASE: c_uint;
    static TEGRA210_ADMAIF_LAST_REG: c_uint;
    static TEGRA210_ADMAIF_CHANNEL_COUNT: c_uint;
    static TEGRA210_ADMAIF_MAX_CHANNEL: c_uint;
    static TEGRA186_ADMAIF_GLOBAL_BASE: c_uint;
    static TEGRA186_ADMAIF_TX_BASE: c_uint;
    static TEGRA186_ADMAIF_RX_BASE: c_uint;
    static TEGRA186_ADMAIF_LAST_REG: c_uint;
    static TEGRA186_ADMAIF_CHANNEL_COUNT: c_uint;
    static TEGRA186_ADMAIF_MAX_CHANNEL: c_uint;
    static TEGRA264_ADMAIF_GLOBAL_BASE: c_uint;
    static TEGRA264_ADMAIF_TX_BASE: c_uint;
    static TEGRA264_ADMAIF_RX_BASE: c_uint;
    static TEGRA264_ADMAIF_LAST_REG: c_uint;
    static TEGRA264_ADMAIF_CHANNEL_COUNT: c_uint;
    static TEGRA264_ADMAIF_MAX_CHANNEL: c_uint;
    static TEGRA264_ADMAIF_MAX_CHANNEL_CONST: c_uint;
    static TEGRA_ADMAIF_CIF_REG_DEFAULT: c_uint;
    static TEGRA_ACIF_BITS_8: c_uint;
    static TEGRA_ACIF_BITS_16: c_uint;
    static TEGRA_ACIF_BITS_24: c_uint;
    static TEGRA_ACIF_BITS_32: c_uint;
    static DATA_8BIT: c_int;
    static DATA_16BIT: c_int;
    static DATA_32BIT: c_int;
    static PACK8_EN_MASK: c_uint;
    static PACK8_EN: c_uint;
    static PACK16_EN_MASK: c_uint;
    static PACK16_EN: c_uint;
    static ADMAIF_TX_PATH: c_uint;
    static ADMAIF_RX_PATH: c_uint;
    static TX_ENABLE_MASK: c_uint;
    static TX_ENABLE: c_uint;
    static RX_ENABLE_MASK: c_uint;
    static RX_ENABLE: c_uint;
    static SW_RESET_MASK: c_uint;
    static SW_RESET: c_uint;
}

const ADMAIF_PATHS: c_uint = 2;

unsafe fn CH_REG(offset: c_uint, reg: c_uint, id: c_uint) -> c_uint {
    offset.wrapping_add(reg).wrapping_add(TEGRA_ADMAIF_CHANNEL_REG_STRIDE.wrapping_mul(id))
}

unsafe fn CH_TX_REG(admaif: *mut tegra_admaif, reg: c_uint, id: c_uint) -> c_uint {
    CH_REG((*(*admaif).soc_data).tx_base, reg, id)
}

unsafe fn CH_RX_REG(admaif: *mut tegra_admaif, reg: c_uint, id: c_uint) -> c_uint {
    CH_REG((*(*admaif).soc_data).rx_base, reg, id)
}

macro_rules! rx_reg_defaults {
    ($id:expr, $rx_ctrl:expr, $rx_base:expr, $cif_ctrl:expr) => {
        reg_default { reg: CH_REG($rx_base, TEGRA_ADMAIF_RX_INT_MASK, $id), def: 0x00000001 },
        reg_default { reg: CH_REG($rx_base, TEGRA_ADMAIF_CH_ACIF_RX_CTRL, $id), def: $cif_ctrl },
        reg_default { reg: CH_REG($rx_base, TEGRA_ADMAIF_RX_FIFO_CTRL, $id), def: $rx_ctrl },
    };
}

macro_rules! tx_reg_defaults {
    ($id:expr, $tx_ctrl:expr, $tx_base:expr, $cif_ctrl:expr) => {
        reg_default { reg: CH_REG($tx_base, TEGRA_ADMAIF_TX_INT_MASK, $id), def: 0x00000001 },
        reg_default { reg: CH_REG($tx_base, TEGRA_ADMAIF_CH_ACIF_TX_CTRL, $id), def: $cif_ctrl },
        reg_default { reg: CH_REG($tx_base, TEGRA_ADMAIF_TX_FIFO_CTRL, $id), def: $tx_ctrl },
    };
}

unsafe fn make_reg_defaults(base_rx: c_uint, base_tx: c_uint, global_base: c_uint, rx_ctrl: &[c_uint], tx_ctrl: &[c_uint], out: &mut [reg_default]) {
    let mut j = 0usize;
    let mut i = 0usize;
    while i < rx_ctrl.len() {
        out[j] = reg_default { reg: CH_REG(base_rx, TEGRA_ADMAIF_RX_INT_MASK, i as c_uint), def: 0x00000001 }; j += 1;
        out[j] = reg_default { reg: CH_REG(base_rx, TEGRA_ADMAIF_CH_ACIF_RX_CTRL, i as c_uint), def: TEGRA_ADMAIF_CIF_REG_DEFAULT }; j += 1;
        out[j] = reg_default { reg: CH_REG(base_rx, TEGRA_ADMAIF_RX_FIFO_CTRL, i as c_uint), def: rx_ctrl[i] }; j += 1;
        i += 1;
    }
    i = 0;
    while i < tx_ctrl.len() {
        out[j] = reg_default { reg: CH_REG(base_tx, TEGRA_ADMAIF_TX_INT_MASK, i as c_uint), def: 0x00000001 }; j += 1;
        out[j] = reg_default { reg: CH_REG(base_tx, TEGRA_ADMAIF_CH_ACIF_TX_CTRL, i as c_uint), def: TEGRA_ADMAIF_CIF_REG_DEFAULT }; j += 1;
        out[j] = reg_default { reg: CH_REG(base_tx, TEGRA_ADMAIF_TX_FIFO_CTRL, i as c_uint), def: tx_ctrl[i] }; j += 1;
        i += 1;
    }
    out[j] = reg_default { reg: TEGRA_ADMAIF_GLOBAL_CG_0.wrapping_add(global_base), def: 0x00000003 };
}

static mut tegra186_admaif_reg_defaults: [reg_default; 121] = [reg_default { reg: 0, def: 0 }; 121];
static mut tegra210_admaif_reg_defaults: [reg_default; 61] = [reg_default { reg: 0, def: 0 }; 61];
static mut tegra264_admaif_reg_defaults: [reg_default; 193] = [reg_default { reg: 0, def: 0 }; 193];

unsafe extern "C" fn tegra_admaif_wr_reg(dev: *mut device, mut reg: c_uint) -> bool {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;
    let ch_stride = TEGRA_ADMAIF_CHANNEL_REG_STRIDE;
    let num_ch = (*(*admaif).soc_data).num_ch;
    let rx_base = (*(*admaif).soc_data).rx_base;
    let tx_base = (*(*admaif).soc_data).tx_base;
    let global_base = (*(*admaif).soc_data).global_base;
    let reg_max = (*(*(*admaif).soc_data).regmap_conf).max_register;
    let rx_max = rx_base.wrapping_add(num_ch.wrapping_mul(ch_stride));
    let tx_max = tx_base.wrapping_add(num_ch.wrapping_mul(ch_stride));

    if reg >= rx_base && reg < rx_max {
        reg = reg.wrapping_sub(rx_base) % ch_stride;
        if reg == TEGRA_ADMAIF_RX_ENABLE ||
           reg == TEGRA_ADMAIF_RX_FIFO_CTRL ||
           reg == TEGRA_ADMAIF_RX_SOFT_RESET ||
           reg == TEGRA_ADMAIF_CH_ACIF_RX_CTRL {
            return true;
        }
    } else if reg >= tx_base && reg < tx_max {
        reg = reg.wrapping_sub(tx_base) % ch_stride;
        if reg == TEGRA_ADMAIF_TX_ENABLE ||
           reg == TEGRA_ADMAIF_TX_FIFO_CTRL ||
           reg == TEGRA_ADMAIF_TX_SOFT_RESET ||
           reg == TEGRA_ADMAIF_CH_ACIF_TX_CTRL {
            return true;
        }
    } else if reg >= global_base && reg < reg_max {
        if reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_ENABLE) {
            return true;
        }
    }

    false
}

unsafe extern "C" fn tegra_admaif_rd_reg(dev: *mut device, mut reg: c_uint) -> bool {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;
    let ch_stride = TEGRA_ADMAIF_CHANNEL_REG_STRIDE;
    let num_ch = (*(*admaif).soc_data).num_ch;
    let rx_base = (*(*admaif).soc_data).rx_base;
    let tx_base = (*(*admaif).soc_data).tx_base;
    let global_base = (*(*admaif).soc_data).global_base;
    let reg_max = (*(*(*admaif).soc_data).regmap_conf).max_register;
    let rx_max = rx_base.wrapping_add(num_ch.wrapping_mul(ch_stride));
    let tx_max = tx_base.wrapping_add(num_ch.wrapping_mul(ch_stride));

    if reg >= rx_base && reg < rx_max {
        reg = reg.wrapping_sub(rx_base) % ch_stride;
        if reg == TEGRA_ADMAIF_RX_ENABLE ||
           reg == TEGRA_ADMAIF_RX_STATUS ||
           reg == TEGRA_ADMAIF_RX_INT_STATUS ||
           reg == TEGRA_ADMAIF_RX_FIFO_CTRL ||
           reg == TEGRA_ADMAIF_RX_SOFT_RESET ||
           reg == TEGRA_ADMAIF_CH_ACIF_RX_CTRL {
            return true;
        }
    } else if reg >= tx_base && reg < tx_max {
        reg = reg.wrapping_sub(tx_base) % ch_stride;
        if reg == TEGRA_ADMAIF_TX_ENABLE ||
           reg == TEGRA_ADMAIF_TX_STATUS ||
           reg == TEGRA_ADMAIF_TX_INT_STATUS ||
           reg == TEGRA_ADMAIF_TX_FIFO_CTRL ||
           reg == TEGRA_ADMAIF_TX_SOFT_RESET ||
           reg == TEGRA_ADMAIF_CH_ACIF_TX_CTRL {
            return true;
        }
    } else if reg >= global_base && reg < reg_max {
        if reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_ENABLE) ||
           reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_CG_0) ||
           reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_STATUS) ||
           reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_RX_ENABLE_STATUS) ||
           reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_TX_ENABLE_STATUS) {
            return true;
        }
    }

    false
}

unsafe extern "C" fn tegra_admaif_volatile_reg(dev: *mut device, mut reg: c_uint) -> bool {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;
    let ch_stride = TEGRA_ADMAIF_CHANNEL_REG_STRIDE;
    let num_ch = (*(*admaif).soc_data).num_ch;
    let rx_base = (*(*admaif).soc_data).rx_base;
    let tx_base = (*(*admaif).soc_data).tx_base;
    let global_base = (*(*admaif).soc_data).global_base;
    let reg_max = (*(*(*admaif).soc_data).regmap_conf).max_register;
    let rx_max = rx_base.wrapping_add(num_ch.wrapping_mul(ch_stride));
    let tx_max = tx_base.wrapping_add(num_ch.wrapping_mul(ch_stride));

    if reg >= rx_base && reg < rx_max {
        reg = reg.wrapping_sub(rx_base) % ch_stride;
        if reg == TEGRA_ADMAIF_RX_ENABLE ||
           reg == TEGRA_ADMAIF_RX_STATUS ||
           reg == TEGRA_ADMAIF_RX_INT_STATUS ||
           reg == TEGRA_ADMAIF_RX_SOFT_RESET {
            return true;
        }
    } else if reg >= tx_base && reg < tx_max {
        reg = reg.wrapping_sub(tx_base) % ch_stride;
        if reg == TEGRA_ADMAIF_TX_ENABLE ||
           reg == TEGRA_ADMAIF_TX_STATUS ||
           reg == TEGRA_ADMAIF_TX_INT_STATUS ||
           reg == TEGRA_ADMAIF_TX_SOFT_RESET {
            return true;
        }
    } else if reg >= global_base && reg < reg_max {
        if reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_STATUS) ||
           reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_RX_ENABLE_STATUS) ||
           reg == global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_TX_ENABLE_STATUS) {
            return true;
        }
    }

    false
}

static tegra210_admaif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0, /* TEGRA210_ADMAIF_LAST_REG: external constant in C headers */
    writeable_reg: Some(tegra_admaif_wr_reg),
    readable_reg: Some(tegra_admaif_rd_reg),
    volatile_reg: Some(tegra_admaif_volatile_reg),
    reg_defaults: unsafe { tegra210_admaif_reg_defaults.as_ptr() },
    num_reg_defaults: 0, /* TEGRA210_ADMAIF_CHANNEL_COUNT * 6 + 1 */
    reg_default_cb: unsafe { Some(regmap_default_zero_cb) },
    cache_type: REGCACHE_FLAT,
};

static tegra186_admaif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0, /* TEGRA186_ADMAIF_LAST_REG */
    writeable_reg: Some(tegra_admaif_wr_reg),
    readable_reg: Some(tegra_admaif_rd_reg),
    volatile_reg: Some(tegra_admaif_volatile_reg),
    reg_defaults: unsafe { tegra186_admaif_reg_defaults.as_ptr() },
    num_reg_defaults: 0, /* TEGRA186_ADMAIF_CHANNEL_COUNT * 6 + 1 */
    reg_default_cb: unsafe { Some(regmap_default_zero_cb) },
    cache_type: REGCACHE_FLAT,
};

static tegra264_admaif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0, /* TEGRA264_ADMAIF_LAST_REG */
    writeable_reg: Some(tegra_admaif_wr_reg),
    readable_reg: Some(tegra_admaif_rd_reg),
    volatile_reg: Some(tegra_admaif_volatile_reg),
    reg_defaults: unsafe { tegra264_admaif_reg_defaults.as_ptr() },
    num_reg_defaults: 0, /* TEGRA264_ADMAIF_CHANNEL_COUNT * 6 + 1 */
    reg_default_cb: unsafe { Some(regmap_default_zero_cb) },
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn tegra_admaif_runtime_suspend(dev: *mut device) -> c_int {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;

    regcache_cache_only((*admaif).regmap, true);
    regcache_mark_dirty((*admaif).regmap);

    0
}

unsafe extern "C" fn tegra_admaif_runtime_resume(dev: *mut device) -> c_int {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;

    regcache_cache_only((*admaif).regmap, false);
    regcache_sync((*admaif).regmap);

    0
}

unsafe fn tegra_admaif_set_pack_mode(map: *mut regmap, reg: c_uint, valid_bit: c_int) -> c_int {
    if valid_bit == DATA_8BIT {
        regmap_update_bits(map, reg, PACK8_EN_MASK, PACK8_EN);
        regmap_update_bits(map, reg, PACK16_EN_MASK, 0);
    } else if valid_bit == DATA_16BIT {
        regmap_update_bits(map, reg, PACK16_EN_MASK, PACK16_EN);
        regmap_update_bits(map, reg, PACK8_EN_MASK, 0);
    } else if valid_bit == DATA_32BIT {
        regmap_update_bits(map, reg, PACK16_EN_MASK, 0);
        regmap_update_bits(map, reg, PACK8_EN_MASK, 0);
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn tegra_admaif_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    tegra_isomgr_adma_setbw(substream, dai, true)
}

unsafe extern "C" fn tegra_admaif_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    tegra_isomgr_adma_setbw(substream, dai, false);
}

unsafe extern "C" fn tegra_admaif_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let dev = (*dai).dev;
    let admaif = snd_soc_dai_get_drvdata(dai) as *mut tegra_admaif;
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();
    let reg: c_uint;
    let path: c_uint;
    let valid_bit: c_int;
    let channels: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_8;
            cif_conf.client_bits = TEGRA_ACIF_BITS_8;
            valid_bit = DATA_8BIT;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_16;
            cif_conf.client_bits = TEGRA_ACIF_BITS_16;
            valid_bit = DATA_16BIT;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_32;
            cif_conf.client_bits = TEGRA_ACIF_BITS_24;
            valid_bit = DATA_32BIT;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_32;
            cif_conf.client_bits = TEGRA_ACIF_BITS_32;
            valid_bit = DATA_32BIT;
        }
        _ => {
            dev_err(dev, b"unsupported format!\n\0".as_ptr() as *const c_char);
            return -EOPNOTSUPP;
        }
    }

    channels = params_channels(params);
    cif_conf.client_ch = channels;
    cif_conf.audio_ch = channels;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        path = ADMAIF_TX_PATH;
        reg = CH_TX_REG(admaif, TEGRA_ADMAIF_CH_ACIF_TX_CTRL, (*dai).id as c_uint);
    } else {
        path = ADMAIF_RX_PATH;
        reg = CH_RX_REG(admaif, TEGRA_ADMAIF_CH_ACIF_RX_CTRL, (*dai).id as c_uint);
    }

    cif_conf.mono_conv = *(*admaif).mono_to_stereo[path as usize].add((*dai).id as usize);
    cif_conf.stereo_conv = *(*admaif).stereo_to_mono[path as usize].add((*dai).id as usize);

    tegra_admaif_set_pack_mode((*admaif).regmap, reg, valid_bit);

    if (*(*admaif).soc_data).max_stream_ch == TEGRA264_ADMAIF_MAX_CHANNEL {
        tegra264_set_cif((*admaif).regmap, reg, &mut cif_conf);
    } else {
        tegra_set_cif((*admaif).regmap, reg, &mut cif_conf);
    }

    0
}

unsafe fn tegra_admaif_start(dai: *mut snd_soc_dai, direction: c_int) -> c_int {
    let admaif = snd_soc_dai_get_drvdata(dai) as *mut tegra_admaif;
    let reg: c_uint;
    let mask: c_uint;
    let val: c_uint;

    match direction {
        SNDRV_PCM_STREAM_PLAYBACK => {
            mask = TX_ENABLE_MASK;
            val = TX_ENABLE;
            reg = CH_TX_REG(admaif, TEGRA_ADMAIF_TX_ENABLE, (*dai).id as c_uint);
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            mask = RX_ENABLE_MASK;
            val = RX_ENABLE;
            reg = CH_RX_REG(admaif, TEGRA_ADMAIF_RX_ENABLE, (*dai).id as c_uint);
        }
        _ => {
            dev_err((*dai).dev, b"invalid stream direction: %d\n\0".as_ptr() as *const c_char, direction);
            return -EINVAL;
        }
    }

    regmap_update_bits((*admaif).regmap, reg, mask, val);
    0
}

unsafe fn regmap_read_poll_timeout_atomic_admaif(map: *mut regmap, reg: c_uint, val: &mut c_uint, enable: c_uint, reset_poll: bool) -> c_int {
    let mut elapsed = 0;
    loop {
        let err = regmap_read(map, reg, val as *mut c_uint);
        if err != 0 {
            return err;
        }
        if if reset_poll { (*val & SW_RESET_MASK & SW_RESET) == 0 } else { (*val & enable) == 0 } {
            return 0;
        }
        if elapsed >= 10000 {
            return -1;
        }
        elapsed += 10;
    }
}

unsafe fn tegra_admaif_stop(dai: *mut snd_soc_dai, direction: c_int) -> c_int {
    let admaif = snd_soc_dai_get_drvdata(dai) as *mut tegra_admaif;
    let enable_reg: c_uint;
    let status_reg: c_uint;
    let reset_reg: c_uint;
    let mask: c_uint;
    let enable: c_uint;
    let dir_name: *const c_char;
    let mut val: c_uint = 0;
    let mut err: c_int;

    match direction {
        SNDRV_PCM_STREAM_PLAYBACK => {
            mask = TX_ENABLE_MASK;
            enable = TX_ENABLE;
            dir_name = b"TX\0".as_ptr() as *const c_char;
            enable_reg = CH_TX_REG(admaif, TEGRA_ADMAIF_TX_ENABLE, (*dai).id as c_uint);
            status_reg = CH_TX_REG(admaif, TEGRA_ADMAIF_TX_STATUS, (*dai).id as c_uint);
            reset_reg = CH_TX_REG(admaif, TEGRA_ADMAIF_TX_SOFT_RESET, (*dai).id as c_uint);
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            mask = RX_ENABLE_MASK;
            enable = RX_ENABLE;
            dir_name = b"RX\0".as_ptr() as *const c_char;
            enable_reg = CH_RX_REG(admaif, TEGRA_ADMAIF_RX_ENABLE, (*dai).id as c_uint);
            status_reg = CH_RX_REG(admaif, TEGRA_ADMAIF_RX_STATUS, (*dai).id as c_uint);
            reset_reg = CH_RX_REG(admaif, TEGRA_ADMAIF_RX_SOFT_RESET, (*dai).id as c_uint);
        }
        _ => {
            dev_err((*dai).dev, b"invalid stream direction: %d\n\0".as_ptr() as *const c_char, direction);
            return -EINVAL;
        }
    }

    /* Disable TX/RX channel */
    regmap_update_bits((*admaif).regmap, enable_reg, mask, !enable);

    /* Wait until ADMAIF TX/RX status is disabled */
    err = regmap_read_poll_timeout_atomic_admaif((*admaif).regmap, status_reg, &mut val, enable, false);
    if err < 0 {
        dev_warn((*dai).dev, b"timeout: failed to disable ADMAIF%d_%s\n\0".as_ptr() as *const c_char, (*dai).id + 1, dir_name);
    }

    /* SW reset */
    regmap_update_bits((*admaif).regmap, reset_reg, SW_RESET_MASK, SW_RESET);

    /* Wait till SW reset is complete */
    err = regmap_read_poll_timeout_atomic_admaif((*admaif).regmap, reset_reg, &mut val, 0, true);
    if err != 0 {
        dev_err((*dai).dev, b"timeout: SW reset failed for ADMAIF%d_%s\n\0".as_ptr() as *const c_char, (*dai).id + 1, dir_name);
        return err;
    }

    0
}

unsafe extern "C" fn tegra_admaif_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let err = snd_dmaengine_pcm_trigger(substream, cmd);
    if err != 0 {
        return err;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            tegra_admaif_start(dai, (*substream).stream)
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            tegra_admaif_stop(dai, (*substream).stream)
        }
        _ => {
            dev_err((*dai).dev, b"invalid trigger command: %d\n\0".as_ptr() as *const c_char, cmd);
            -EINVAL
        }
    }
}

unsafe extern "C" fn tegra210_admaif_pget_mono_to_stereo(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    (*ucontrol).value.enumerated.item[0] = *(*admaif).mono_to_stereo[ADMAIF_TX_PATH as usize].add((*ec).reg as usize);
    0
}

unsafe extern "C" fn tegra210_admaif_pput_mono_to_stereo(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    let value = (*ucontrol).value.enumerated.item[0];
    let slot = (*admaif).mono_to_stereo[ADMAIF_TX_PATH as usize].add((*ec).reg as usize);
    if value == *slot {
        return 0;
    }
    *slot = value;
    1
}

unsafe extern "C" fn tegra210_admaif_cget_mono_to_stereo(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    (*ucontrol).value.enumerated.item[0] = *(*admaif).mono_to_stereo[ADMAIF_RX_PATH as usize].add((*ec).reg as usize);
    0
}

unsafe extern "C" fn tegra210_admaif_cput_mono_to_stereo(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    let value = (*ucontrol).value.enumerated.item[0];
    let slot = (*admaif).mono_to_stereo[ADMAIF_RX_PATH as usize].add((*ec).reg as usize);
    if value == *slot {
        return 0;
    }
    *slot = value;
    1
}

unsafe extern "C" fn tegra210_admaif_pget_stereo_to_mono(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    (*ucontrol).value.enumerated.item[0] = *(*admaif).stereo_to_mono[ADMAIF_TX_PATH as usize].add((*ec).reg as usize);
    0
}

unsafe extern "C" fn tegra210_admaif_pput_stereo_to_mono(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    let value = (*ucontrol).value.enumerated.item[0];
    let slot = (*admaif).stereo_to_mono[ADMAIF_TX_PATH as usize].add((*ec).reg as usize);
    if value == *slot {
        return 0;
    }
    *slot = value;
    1
}

unsafe extern "C" fn tegra210_admaif_cget_stereo_to_mono(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    (*ucontrol).value.enumerated.item[0] = *(*admaif).stereo_to_mono[ADMAIF_RX_PATH as usize].add((*ec).reg as usize);
    0
}

unsafe extern "C" fn tegra210_admaif_cput_stereo_to_mono(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let admaif = snd_soc_component_get_drvdata(cmpnt) as *mut tegra_admaif;
    let ec = (*kcontrol).private_value as *mut soc_enum;
    let value = (*ucontrol).value.enumerated.item[0];
    let slot = (*admaif).stereo_to_mono[ADMAIF_RX_PATH as usize].add((*ec).reg as usize);
    if value == *slot {
        return 0;
    }
    *slot = value;
    1
}

unsafe extern "C" fn tegra_admaif_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let admaif = snd_soc_dai_get_drvdata(dai) as *mut tegra_admaif;
    snd_soc_dai_init_dma_data(
        dai,
        (*admaif).playback_dma_data.add((*dai).id as usize),
        (*admaif).capture_dma_data.as_mut_ptr().add((*dai).id as usize),
    );
    0
}

static tegra_admaif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(tegra_admaif_dai_probe),
    hw_params: Some(tegra_admaif_hw_params),
    trigger: Some(tegra_admaif_trigger),
    shutdown: Some(tegra_admaif_shutdown),
    prepare: Some(tegra_admaif_prepare),
};

macro_rules! DAI {
    ($dai_name:expr, $channel:expr) => {
        snd_soc_dai_driver {
            name: concat!($dai_name, "\0").as_ptr() as *const c_char,
            playback: snd_soc_pcm_stream {
                stream_name: concat!($dai_name, " Playback\0").as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: $channel,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            },
            capture: snd_soc_pcm_stream {
                stream_name: concat!($dai_name, " Capture\0").as_ptr() as *const c_char,
                channels_min: 1,
                channels_max: $channel,
                rates: SNDRV_PCM_RATE_8000_192000,
                formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            },
            ops: &tegra_admaif_dai_ops,
        }
    };
}

static mut tegra210_admaif_cmpnt_dais: [snd_soc_dai_driver; 10] = [
    DAI!("ADMAIF1", 0), DAI!("ADMAIF2", 0), DAI!("ADMAIF3", 0), DAI!("ADMAIF4", 0), DAI!("ADMAIF5", 0),
    DAI!("ADMAIF6", 0), DAI!("ADMAIF7", 0), DAI!("ADMAIF8", 0), DAI!("ADMAIF9", 0), DAI!("ADMAIF10", 0),
];

static mut tegra186_admaif_cmpnt_dais: [snd_soc_dai_driver; 20] = [
    DAI!("ADMAIF1", 0), DAI!("ADMAIF2", 0), DAI!("ADMAIF3", 0), DAI!("ADMAIF4", 0), DAI!("ADMAIF5", 0),
    DAI!("ADMAIF6", 0), DAI!("ADMAIF7", 0), DAI!("ADMAIF8", 0), DAI!("ADMAIF9", 0), DAI!("ADMAIF10", 0),
    DAI!("ADMAIF11", 0), DAI!("ADMAIF12", 0), DAI!("ADMAIF13", 0), DAI!("ADMAIF14", 0), DAI!("ADMAIF15", 0),
    DAI!("ADMAIF16", 0), DAI!("ADMAIF17", 0), DAI!("ADMAIF18", 0), DAI!("ADMAIF19", 0), DAI!("ADMAIF20", 0),
];

static mut tegra264_admaif_cmpnt_dais: [snd_soc_dai_driver; 32] = [
    DAI!("ADMAIF1", 0), DAI!("ADMAIF2", 0), DAI!("ADMAIF3", 0), DAI!("ADMAIF4", 0), DAI!("ADMAIF5", 0),
    DAI!("ADMAIF6", 0), DAI!("ADMAIF7", 0), DAI!("ADMAIF8", 0), DAI!("ADMAIF9", 0), DAI!("ADMAIF10", 0),
    DAI!("ADMAIF11", 0), DAI!("ADMAIF12", 0), DAI!("ADMAIF13", 0), DAI!("ADMAIF14", 0), DAI!("ADMAIF15", 0),
    DAI!("ADMAIF16", 0), DAI!("ADMAIF17", 0), DAI!("ADMAIF18", 0), DAI!("ADMAIF19", 0), DAI!("ADMAIF20", 0),
    DAI!("ADMAIF21", 0), DAI!("ADMAIF22", 0), DAI!("ADMAIF23", 0), DAI!("ADMAIF24", 0), DAI!("ADMAIF25", 0),
    DAI!("ADMAIF26", 0), DAI!("ADMAIF27", 0), DAI!("ADMAIF28", 0), DAI!("ADMAIF29", 0), DAI!("ADMAIF30", 0),
    DAI!("ADMAIF31", 0), DAI!("ADMAIF32", 0),
];

static tegra_admaif_stereo_conv_text: [*const c_char; 3] = [
    b"CH0\0".as_ptr() as *const c_char,
    b"CH1\0".as_ptr() as *const c_char,
    b"AVG\0".as_ptr() as *const c_char,
];

static tegra_admaif_mono_conv_text: [*const c_char; 2] = [
    b"Zero\0".as_ptr() as *const c_char,
    b"Copy\0".as_ptr() as *const c_char,
];

/*
 * Below macro is added to avoid looping over all ADMAIFx controls related
 * to mono/stereo conversions in get()/put() callbacks.
 */
macro_rules! NV_SOC_ENUM_EXT {
    ($xname:expr, $xreg:expr, $xhandler_get:path, $xhandler_put:path, $xenum_text:ident) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            info: unsafe { Some(snd_soc_info_enum_double) },
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            get: Some($xhandler_get),
            put: Some($xhandler_put),
            private_value: $xreg as c_ulong,
        }
    };
}

macro_rules! TEGRA_ADMAIF_CIF_CTRL {
    ($reg:expr) => {
        NV_SOC_ENUM_EXT!(concat!("ADMAIF", stringify!($reg), " Playback Mono To Stereo"), $reg - 1, tegra210_admaif_pget_mono_to_stereo, tegra210_admaif_pput_mono_to_stereo, tegra_admaif_mono_conv_text),
        NV_SOC_ENUM_EXT!(concat!("ADMAIF", stringify!($reg), " Playback Stereo To Mono"), $reg - 1, tegra210_admaif_pget_stereo_to_mono, tegra210_admaif_pput_stereo_to_mono, tegra_admaif_stereo_conv_text),
        NV_SOC_ENUM_EXT!(concat!("ADMAIF", stringify!($reg), " Capture Mono To Stereo"), $reg - 1, tegra210_admaif_cget_mono_to_stereo, tegra210_admaif_cput_mono_to_stereo, tegra_admaif_mono_conv_text),
        NV_SOC_ENUM_EXT!(concat!("ADMAIF", stringify!($reg), " Capture Stereo To Mono"), $reg - 1, tegra210_admaif_cget_stereo_to_mono, tegra210_admaif_cput_stereo_to_mono, tegra_admaif_stereo_conv_text)
    };
}

static mut tegra210_admaif_controls: [snd_kcontrol_new; 40] = [
    TEGRA_ADMAIF_CIF_CTRL!(1), TEGRA_ADMAIF_CIF_CTRL!(2), TEGRA_ADMAIF_CIF_CTRL!(3), TEGRA_ADMAIF_CIF_CTRL!(4), TEGRA_ADMAIF_CIF_CTRL!(5),
    TEGRA_ADMAIF_CIF_CTRL!(6), TEGRA_ADMAIF_CIF_CTRL!(7), TEGRA_ADMAIF_CIF_CTRL!(8), TEGRA_ADMAIF_CIF_CTRL!(9), TEGRA_ADMAIF_CIF_CTRL!(10),
];

static mut tegra186_admaif_controls: [snd_kcontrol_new; 80] = [
    TEGRA_ADMAIF_CIF_CTRL!(1), TEGRA_ADMAIF_CIF_CTRL!(2), TEGRA_ADMAIF_CIF_CTRL!(3), TEGRA_ADMAIF_CIF_CTRL!(4), TEGRA_ADMAIF_CIF_CTRL!(5),
    TEGRA_ADMAIF_CIF_CTRL!(6), TEGRA_ADMAIF_CIF_CTRL!(7), TEGRA_ADMAIF_CIF_CTRL!(8), TEGRA_ADMAIF_CIF_CTRL!(9), TEGRA_ADMAIF_CIF_CTRL!(10),
    TEGRA_ADMAIF_CIF_CTRL!(11), TEGRA_ADMAIF_CIF_CTRL!(12), TEGRA_ADMAIF_CIF_CTRL!(13), TEGRA_ADMAIF_CIF_CTRL!(14), TEGRA_ADMAIF_CIF_CTRL!(15),
    TEGRA_ADMAIF_CIF_CTRL!(16), TEGRA_ADMAIF_CIF_CTRL!(17), TEGRA_ADMAIF_CIF_CTRL!(18), TEGRA_ADMAIF_CIF_CTRL!(19), TEGRA_ADMAIF_CIF_CTRL!(20),
];

static mut tegra264_admaif_controls: [snd_kcontrol_new; 128] = [
    TEGRA_ADMAIF_CIF_CTRL!(1), TEGRA_ADMAIF_CIF_CTRL!(2), TEGRA_ADMAIF_CIF_CTRL!(3), TEGRA_ADMAIF_CIF_CTRL!(4), TEGRA_ADMAIF_CIF_CTRL!(5),
    TEGRA_ADMAIF_CIF_CTRL!(6), TEGRA_ADMAIF_CIF_CTRL!(7), TEGRA_ADMAIF_CIF_CTRL!(8), TEGRA_ADMAIF_CIF_CTRL!(9), TEGRA_ADMAIF_CIF_CTRL!(10),
    TEGRA_ADMAIF_CIF_CTRL!(11), TEGRA_ADMAIF_CIF_CTRL!(12), TEGRA_ADMAIF_CIF_CTRL!(13), TEGRA_ADMAIF_CIF_CTRL!(14), TEGRA_ADMAIF_CIF_CTRL!(15),
    TEGRA_ADMAIF_CIF_CTRL!(16), TEGRA_ADMAIF_CIF_CTRL!(17), TEGRA_ADMAIF_CIF_CTRL!(18), TEGRA_ADMAIF_CIF_CTRL!(19), TEGRA_ADMAIF_CIF_CTRL!(20),
    TEGRA_ADMAIF_CIF_CTRL!(21), TEGRA_ADMAIF_CIF_CTRL!(22), TEGRA_ADMAIF_CIF_CTRL!(23), TEGRA_ADMAIF_CIF_CTRL!(24), TEGRA_ADMAIF_CIF_CTRL!(25),
    TEGRA_ADMAIF_CIF_CTRL!(26), TEGRA_ADMAIF_CIF_CTRL!(27), TEGRA_ADMAIF_CIF_CTRL!(28), TEGRA_ADMAIF_CIF_CTRL!(29), TEGRA_ADMAIF_CIF_CTRL!(30),
    TEGRA_ADMAIF_CIF_CTRL!(31), TEGRA_ADMAIF_CIF_CTRL!(32),
];

static tegra210_admaif_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { tegra210_admaif_controls.as_mut_ptr() },
    num_controls: 40,
    pcm_new: unsafe { Some(tegra_pcm_new) },
    open: unsafe { Some(tegra_pcm_open) },
    close: unsafe { Some(tegra_pcm_close) },
    hw_params: unsafe { Some(tegra_pcm_hw_params) },
    pointer: unsafe { Some(tegra_pcm_pointer) },
};

static tegra186_admaif_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { tegra186_admaif_controls.as_mut_ptr() },
    num_controls: 80,
    pcm_new: unsafe { Some(tegra_pcm_new) },
    open: unsafe { Some(tegra_pcm_open) },
    close: unsafe { Some(tegra_pcm_close) },
    hw_params: unsafe { Some(tegra_pcm_hw_params) },
    pointer: unsafe { Some(tegra_pcm_pointer) },
};

static tegra264_admaif_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { tegra264_admaif_controls.as_mut_ptr() },
    num_controls: 128,
    pcm_new: unsafe { Some(tegra_pcm_new) },
    open: unsafe { Some(tegra_pcm_open) },
    close: unsafe { Some(tegra_pcm_close) },
    hw_params: unsafe { Some(tegra_pcm_hw_params) },
    pointer: unsafe { Some(tegra_pcm_pointer) },
};

static soc_data_tegra210: tegra_admaif_soc_data = tegra_admaif_soc_data {
    num_ch: 0, /* TEGRA210_ADMAIF_CHANNEL_COUNT */
    max_stream_ch: 0, /* TEGRA210_ADMAIF_MAX_CHANNEL */
    cmpnt: &tegra210_admaif_cmpnt,
    dais: unsafe { tegra210_admaif_cmpnt_dais.as_mut_ptr() },
    regmap_conf: &tegra210_admaif_regmap_config,
    global_base: 0, /* TEGRA210_ADMAIF_GLOBAL_BASE */
    tx_base: 0, /* TEGRA210_ADMAIF_TX_BASE */
    rx_base: 0, /* TEGRA210_ADMAIF_RX_BASE */
};

static soc_data_tegra186: tegra_admaif_soc_data = tegra_admaif_soc_data {
    num_ch: 0, /* TEGRA186_ADMAIF_CHANNEL_COUNT */
    max_stream_ch: 0, /* TEGRA186_ADMAIF_MAX_CHANNEL */
    cmpnt: &tegra186_admaif_cmpnt,
    dais: unsafe { tegra186_admaif_cmpnt_dais.as_mut_ptr() },
    regmap_conf: &tegra186_admaif_regmap_config,
    global_base: 0, /* TEGRA186_ADMAIF_GLOBAL_BASE */
    tx_base: 0, /* TEGRA186_ADMAIF_TX_BASE */
    rx_base: 0, /* TEGRA186_ADMAIF_RX_BASE */
};

static soc_data_tegra264: tegra_admaif_soc_data = tegra_admaif_soc_data {
    num_ch: 0, /* TEGRA264_ADMAIF_CHANNEL_COUNT */
    max_stream_ch: 0, /* TEGRA264_ADMAIF_MAX_CHANNEL */
    cmpnt: &tegra264_admaif_cmpnt,
    dais: unsafe { tegra264_admaif_cmpnt_dais.as_mut_ptr() },
    regmap_conf: &tegra264_admaif_regmap_config,
    global_base: 0, /* TEGRA264_ADMAIF_GLOBAL_BASE */
    tx_base: 0, /* TEGRA264_ADMAIF_TX_BASE */
    rx_base: 0, /* TEGRA264_ADMAIF_RX_BASE */
};

static tegra_admaif_of_match: [of_device_id; 4] = [
    of_device_id { compatible: b"nvidia,tegra210-admaif\0".as_ptr() as *const c_char, data: &soc_data_tegra210 as *const _ as *const c_void },
    of_device_id { compatible: b"nvidia,tegra186-admaif\0".as_ptr() as *const c_char, data: &soc_data_tegra186 as *const _ as *const c_void },
    of_device_id { compatible: b"nvidia,tegra264-admaif\0".as_ptr() as *const c_char, data: &soc_data_tegra264 as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, tegra_admaif_of_match); */

unsafe extern "C" fn tegra_admaif_probe(pdev: *mut platform_device) -> c_int {
    let soc_data = of_device_get_match_data(&mut (*pdev).dev) as *const tegra_admaif_soc_data;
    let mut res: *mut resource = ptr::null_mut();
    let alloc_size = size_of::<tegra_admaif>().wrapping_add(size_of::<snd_dmaengine_dai_dma_data>().wrapping_mul((*soc_data).num_ch as usize));
    let admaif = devm_kzalloc(&mut (*pdev).dev, alloc_size, GFP_KERNEL) as *mut tegra_admaif;
    if admaif.is_null() {
        return -ENOMEM;
    }

    (*admaif).playback_dma_data = (*admaif).capture_dma_data.as_mut_ptr().add((*soc_data).num_ch as usize);
    (*admaif).soc_data = soc_data;

    dev_set_drvdata(&mut (*pdev).dev, admaif as *mut c_void);

    let mut i = 0;
    while i < ADMAIF_PATHS as c_int {
        (*admaif).mono_to_stereo[i as usize] = devm_kcalloc(&mut (*pdev).dev, (*(*admaif).soc_data).num_ch as usize, size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
        if (*admaif).mono_to_stereo[i as usize].is_null() {
            return -ENOMEM;
        }

        (*admaif).stereo_to_mono[i as usize] = devm_kcalloc(&mut (*pdev).dev, (*(*admaif).soc_data).num_ch as usize, size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
        if (*admaif).stereo_to_mono[i as usize].is_null() {
            return -ENOMEM;
        }
        i += 1;
    }

    let regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*admaif).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, (*(*admaif).soc_data).regmap_conf);
    if IS_ERR((*admaif).regmap as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*admaif).regmap as *const c_void), b"regmap init failed\n\0".as_ptr() as *const c_char);
    }

    regcache_cache_only((*admaif).regmap, true);

    let mut err = tegra_isomgr_adma_register(&mut (*pdev).dev);
    if err != 0 {
        return err;
    }

    regmap_update_bits((*admaif).regmap, (*(*admaif).soc_data).global_base.wrapping_add(TEGRA_ADMAIF_GLOBAL_ENABLE), 1, 1);

    i = 0;
    while i < (*(*admaif).soc_data).num_ch as c_int {
        (*(*admaif).playback_dma_data.add(i as usize)).addr =
            (*res).start.wrapping_add(CH_TX_REG(admaif, TEGRA_ADMAIF_TX_FIFO_WRITE, i as c_uint) as resource_size_t);

        (*(*admaif).capture_dma_data.as_mut_ptr().add(i as usize)).addr =
            (*res).start.wrapping_add(CH_RX_REG(admaif, TEGRA_ADMAIF_RX_FIFO_READ, i as c_uint) as resource_size_t);

        (*(*admaif).playback_dma_data.add(i as usize)).addr_width = 32;

        if of_property_read_string_index((*pdev).dev.of_node, b"dma-names\0".as_ptr() as *const c_char, (i * 2) + 1, &mut (*(*admaif).playback_dma_data.add(i as usize)).chan_name) < 0 {
            dev_err(&mut (*pdev).dev, b"missing property nvidia,dma-names\n\0".as_ptr() as *const c_char);
            return -ENODEV;
        }

        (*(*admaif).capture_dma_data.as_mut_ptr().add(i as usize)).addr_width = 32;

        if of_property_read_string_index((*pdev).dev.of_node, b"dma-names\0".as_ptr() as *const c_char, i * 2, &mut (*(*admaif).capture_dma_data.as_mut_ptr().add(i as usize)).chan_name) < 0 {
            dev_err(&mut (*pdev).dev, b"missing property nvidia,dma-names\n\0".as_ptr() as *const c_char);
            return -ENODEV;
        }

        i += 1;
    }

    err = devm_snd_soc_register_component(&mut (*pdev).dev, (*(*admaif).soc_data).cmpnt, (*(*admaif).soc_data).dais, (*(*admaif).soc_data).num_ch as c_int);
    if err != 0 {
        return dev_err_probe(&mut (*pdev).dev, err, b"can't register ADMAIF component\n\0".as_ptr() as *const c_char);
    }

    pm_runtime_enable(&mut (*pdev).dev);

    0
}

unsafe extern "C" fn tegra_admaif_remove(pdev: *mut platform_device) {
    tegra_isomgr_adma_unregister(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

static tegra_admaif_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(tegra_admaif_runtime_suspend),
    runtime_resume: Some(tegra_admaif_runtime_resume),
    suspend: unsafe { Some(pm_runtime_force_suspend) },
    resume: unsafe { Some(pm_runtime_force_resume) },
};

static mut tegra_admaif_driver: platform_driver = platform_driver {
    probe: Some(tegra_admaif_probe),
    remove: Some(tegra_admaif_remove),
    driver: device_driver {
        name: b"tegra210-admaif\0".as_ptr() as *const c_char,
        of_match_table: tegra_admaif_of_match.as_ptr(),
        pm: &tegra_admaif_pm_ops,
    },
};
/* module_platform_driver(tegra_admaif_driver); */

/* MODULE_AUTHOR("Songhee Baek <sbaek@nvidia.com>"); */
/* MODULE_DESCRIPTION("Tegra210 ASoC ADMAIF driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
