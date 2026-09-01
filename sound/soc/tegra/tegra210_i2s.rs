// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_i2s.c - Tegra210 I2S driver
//
// Rust translation of the isolated C implementation source. C include
// dependencies are expected to be supplied by surrounding bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type snd_pcm_format_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const GFP_KERNEL: c_uint = 0;
const INT_MAX: c_int = 2147483647;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_node {
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
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
    pub reg_default_cb: *const c_void,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub reg: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    pub value: [c_int; 4],
}

#[repr(C)]
pub struct snd_ctl_elem_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_integer,
    pub enumerated: snd_ctl_elem_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra_cif_conf {
    pub audio_ch: c_uint,
    pub client_ch: c_uint,
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
    pub threshold: c_uint,
    pub mono_conv: c_uint,
    pub stereo_conv: c_uint,
}

#[repr(C)]
pub struct simple_util_data {
    pub convert_channels: c_uint,
    pub convert_sample_format: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
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
    pub symmetric_rate: c_uint,
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
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct tegra_i2s_soc_data {
    pub regmap_conf: *const regmap_config,
    pub i2s_cmpnt: *const snd_soc_component_driver,
    pub max_ch: c_uint,
    pub enable_reg: c_uint,
    pub tx_offset: c_uint,
    pub i2s_ctrl_offset: c_uint,
    pub fsync_width_mask: c_uint,
    pub fsync_width_shift: c_uint,
    pub slot_mask: c_uint,
}

#[repr(C)]
pub struct tegra210_i2s {
    pub regmap: *mut regmap,
    pub clk_i2s: *mut clk,
    pub clk_sync_input: *mut clk,
    pub soc_data: *const tegra_i2s_soc_data,
    pub tx_mask: c_uint,
    pub rx_mask: c_uint,
    pub loopback: c_int,
    pub fsync_width: c_int,
    pub stereo_to_mono: [c_uint; 2],
    pub mono_to_stereo: [c_uint; 2],
    pub rx_fifo_th: c_int,
    pub bclk_ratio: c_uint,
    pub dai_fmt: c_uint,
    pub client_channels: c_uint,
    pub client_sample_format: c_int,
}

extern "C" {
    static regmap_default_zero_cb: c_void;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(compnt: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;

    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn tegra_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);
    fn tegra264_set_cif(map: *mut regmap, reg: c_uint, conf: *mut tegra_cif_conf);

    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_graph_get_endpoint_by_regs(parent: *mut device_node, port: c_int, reg: c_int) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn simple_util_parse_convert(np: *mut device_node, prefix: *const c_char, data: *mut simple_util_data);
    fn simple_util_get_sample_fmt(data: *mut simple_util_data) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dais: c_uint) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

unsafe fn regmap_read_poll_timeout(
    map: *mut regmap,
    reg: c_uint,
    val: &mut c_uint,
    cond: impl Fn(c_uint) -> bool,
    _sleep_us: c_uint,
    _timeout_us: c_uint,
) -> c_int {
    let err = regmap_read(map, reg, val as *mut c_uint);
    if err != 0 {
        return err;
    }
    if cond(*val) {
        0
    } else {
        -EINVAL
    }
}

static tegra210_i2s_reg_defaults: [reg_default; 9] = [
    reg_default { reg: TEGRA210_I2S_RX_INT_MASK, def: 0x00000003 },
    reg_default { reg: TEGRA210_I2S_RX_CIF_CTRL, def: 0x00007700 },
    reg_default { reg: TEGRA210_I2S_TX_INT_MASK, def: 0x00000003 },
    reg_default { reg: TEGRA210_I2S_TX_CIF_CTRL, def: 0x00007700 },
    reg_default { reg: TEGRA210_I2S_ENABLE, def: 0x0 },
    reg_default { reg: TEGRA210_I2S_CG, def: 0x1 },
    reg_default { reg: TEGRA210_I2S_TIMING, def: 0x0000001f },
    /*
     * Below update does not have any effect on Tegra186 and Tegra194.
     * On Tegra210, I2S4 has "i2s4a" and "i2s4b" pins and below update
     * is required to select i2s4b for it to be functional for I2S
     * operation.
     */
    reg_default { reg: TEGRA210_I2S_CYA, def: 0x1 },
];

static tegra264_i2s_reg_defaults: [reg_default; 9] = [
    reg_default { reg: TEGRA210_I2S_RX_INT_MASK, def: 0x00000003 },
    reg_default { reg: TEGRA210_I2S_RX_CIF_CTRL, def: 0x00003f00 },
    reg_default { reg: TEGRA264_I2S_RX_FIFO_WR_ACCESS_MODE, def: 0x1 },
    reg_default { reg: TEGRA264_I2S_TX_INT_MASK, def: 0x00000003 },
    reg_default { reg: TEGRA264_I2S_TX_CIF_CTRL, def: 0x00003f00 },
    reg_default { reg: TEGRA264_I2S_TX_FIFO_RD_ACCESS_MODE, def: 0x1 },
    reg_default { reg: TEGRA264_I2S_ENABLE, def: 0x0 },
    reg_default { reg: TEGRA264_I2S_CG, def: 0x1 },
    reg_default { reg: TEGRA264_I2S_TIMING, def: 0x0000001f },
];

unsafe extern "C" fn tegra210_i2s_set_slot_ctrl(i2s: *mut tegra210_i2s, total_slots: c_uint, tx_slot_mask: c_uint, rx_slot_mask: c_uint) {
    regmap_write((*i2s).regmap, TEGRA210_I2S_SLOT_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, total_slots - 1);
    regmap_write((*i2s).regmap, TEGRA210_I2S_TX_SLOT_CTRL + (*(*i2s).soc_data).tx_offset, tx_slot_mask);
    regmap_write((*i2s).regmap, TEGRA210_I2S_RX_SLOT_CTRL, rx_slot_mask);
}

unsafe extern "C" fn tegra210_i2s_set_clock_rate(dev: *mut device, clock_rate: c_uint) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    let mut val: c_uint = 0;
    let mut err: c_int;

    regmap_read((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, &mut val);

    /* No need to set rates if I2S is being operated in slave */
    if (val & I2S_CTRL_MASTER_EN) == 0 {
        return 0;
    }

    err = clk_set_rate((*i2s).clk_i2s, clock_rate);
    if err != 0 {
        dev_err(dev, b"can't set I2S bit clock rate %u, err: %d\n\0".as_ptr() as *const c_char, clock_rate, err);
        return err;
    }

    if !IS_ERR((*i2s).clk_sync_input as *const c_void) {
        /*
         * Other I/O modules in AHUB can use i2s bclk as reference
         * clock. Below sets sync input clock rate as per bclk,
         * which can be used as input to other I/O modules.
         */
        err = clk_set_rate((*i2s).clk_sync_input, clock_rate);
        if err != 0 {
            dev_err(dev, b"can't set I2S sync input rate %u, err = %d\n\0".as_ptr() as *const c_char, clock_rate, err);
            return err;
        }
    }

    0
}

unsafe extern "C" fn tegra210_i2s_sw_reset(compnt: *mut snd_soc_component, stream: c_int) -> c_int {
    let dev = (*compnt).dev;
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    let reset_mask: c_uint = I2S_SOFT_RESET_MASK;
    let reset_en: c_uint = I2S_SOFT_RESET_EN;
    let reset_reg: c_uint;
    let cif_reg: c_uint;
    let stream_reg: c_uint;
    let mut cif_ctrl: c_uint = 0;
    let mut stream_ctrl: c_uint = 0;
    let mut i2s_ctrl: c_uint = 0;
    let mut val: c_uint = 0;
    let err: c_int;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        reset_reg = TEGRA210_I2S_RX_SOFT_RESET;
        cif_reg = TEGRA210_I2S_RX_CIF_CTRL;
        stream_reg = TEGRA210_I2S_RX_CTRL;
    } else {
        reset_reg = TEGRA210_I2S_TX_SOFT_RESET + (*(*i2s).soc_data).tx_offset;
        cif_reg = TEGRA210_I2S_TX_CIF_CTRL + (*(*i2s).soc_data).tx_offset;
        stream_reg = TEGRA210_I2S_TX_CTRL + (*(*i2s).soc_data).tx_offset;
    }

    /* Store CIF and I2S control values */
    regmap_read((*i2s).regmap, cif_reg, &mut cif_ctrl);
    regmap_read((*i2s).regmap, stream_reg, &mut stream_ctrl);
    regmap_read((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, &mut i2s_ctrl);

    /* Reset to make sure the previous transactions are clean */
    regmap_update_bits((*i2s).regmap, reset_reg, reset_mask, reset_en);

    err = regmap_read_poll_timeout((*i2s).regmap, reset_reg, &mut val, |v| (v & reset_mask & reset_en) == 0, 10, 10000);
    if err != 0 {
        dev_err(dev, b"timeout: failed to reset I2S for %s\n\0".as_ptr() as *const c_char, snd_pcm_direction_name(stream));
        return err;
    }

    /* Restore CIF and I2S control values */
    regmap_write((*i2s).regmap, cif_reg, cif_ctrl);
    regmap_write((*i2s).regmap, stream_reg, stream_ctrl);
    regmap_write((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, i2s_ctrl);

    0
}

unsafe extern "C" fn tegra210_i2s_init(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    let compnt = snd_soc_dapm_to_component((*w).dapm);
    let dev = (*compnt).dev;
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    let mut val: c_uint = 0;
    let status_reg: c_uint;
    let stream: c_int;
    let err: c_int;

    if (*w).reg == TEGRA210_I2S_RX_ENABLE {
        stream = SNDRV_PCM_STREAM_PLAYBACK;
        status_reg = TEGRA210_I2S_RX_STATUS;
    } else if (*w).reg == TEGRA210_I2S_TX_ENABLE + (*(*i2s).soc_data).tx_offset {
        stream = SNDRV_PCM_STREAM_CAPTURE;
        status_reg = TEGRA210_I2S_TX_STATUS + (*(*i2s).soc_data).tx_offset;
    } else {
        dev_err(dev, b"invalid I2S direction register 0x%x\n\0".as_ptr() as *const c_char, (*w).reg);
        return -EINVAL;
    }

    /* Ensure I2S is in disabled state before new session */
    err = regmap_read_poll_timeout((*i2s).regmap, status_reg, &mut val, |v| (v & I2S_EN_MASK & I2S_EN) == 0, 10, 10000);
    if err != 0 {
        dev_err(dev, b"timeout: previous I2S %s is still active\n\0".as_ptr() as *const c_char, snd_pcm_direction_name(stream));
        return err;
    }

    tegra210_i2s_sw_reset(compnt, stream)
}

unsafe extern "C" fn tegra210_i2s_runtime_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    regcache_cache_only((*i2s).regmap, true);
    regcache_mark_dirty((*i2s).regmap);
    clk_disable_unprepare((*i2s).clk_i2s);
    0
}

unsafe extern "C" fn tegra210_i2s_runtime_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    let mut err = clk_prepare_enable((*i2s).clk_i2s);
    if err != 0 {
        dev_err(dev, b"failed to enable I2S bit clock, err: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    regcache_cache_only((*i2s).regmap, false);
    err = regcache_sync((*i2s).regmap);
    if err != 0 {
        regcache_cache_only((*i2s).regmap, true);
        clk_disable_unprepare((*i2s).clk_i2s);
        return err;
    }

    err = regmap_write((*i2s).regmap, (*(*i2s).soc_data).enable_reg, I2S_EN);
    if err != 0 {
        regcache_cache_only((*i2s).regmap, true);
        clk_disable_unprepare((*i2s).clk_i2s);
        return err;
    }

    0
}

unsafe extern "C" fn tegra210_i2s_set_data_offset(i2s: *mut tegra210_i2s, data_offset: c_uint) {
    /* Capture path */
    regmap_update_bits((*i2s).regmap, TEGRA210_I2S_TX_CTRL + (*(*i2s).soc_data).tx_offset, I2S_CTRL_DATA_OFFSET_MASK, data_offset << I2S_DATA_SHIFT);
    /* Playback path */
    regmap_update_bits((*i2s).regmap, TEGRA210_I2S_RX_CTRL, I2S_CTRL_DATA_OFFSET_MASK, data_offset << I2S_DATA_SHIFT);
}

unsafe extern "C" fn tegra210_i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra210_i2s;
    let mut mask: c_uint = I2S_CTRL_MASTER_EN_MASK;
    let mut val: c_uint;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => val = 0,
        SND_SOC_DAIFMT_BP_FP => val = I2S_CTRL_MASTER_EN,
        _ => {
            dev_err((*dai).dev, b"invalid clock provider format 0x%x\n\0".as_ptr() as *const c_char, fmt);
            return -EINVAL;
        }
    }

    mask |= I2S_CTRL_FRAME_FMT_MASK | I2S_CTRL_LRCK_POL_MASK;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            val |= I2S_CTRL_FRAME_FMT_FSYNC_MODE;
            val |= I2S_CTRL_LRCK_POL_HIGH;
            tegra210_i2s_set_data_offset(i2s, 1);
        }
        SND_SOC_DAIFMT_DSP_B => {
            val |= I2S_CTRL_FRAME_FMT_FSYNC_MODE;
            val |= I2S_CTRL_LRCK_POL_HIGH;
            tegra210_i2s_set_data_offset(i2s, 0);
        }
        /* I2S mode has data offset of 1 */
        SND_SOC_DAIFMT_I2S => {
            val |= I2S_CTRL_FRAME_FMT_LRCK_MODE;
            val |= I2S_CTRL_LRCK_POL_LOW;
            tegra210_i2s_set_data_offset(i2s, 1);
        }
        /*
         * For RJ mode data offset is dependent on the sample size
         * and the bclk ratio, and so is set when hw_params is called.
         */
        SND_SOC_DAIFMT_RIGHT_J => {
            val |= I2S_CTRL_FRAME_FMT_LRCK_MODE;
            val |= I2S_CTRL_LRCK_POL_HIGH;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val |= I2S_CTRL_FRAME_FMT_LRCK_MODE;
            val |= I2S_CTRL_LRCK_POL_HIGH;
            tegra210_i2s_set_data_offset(i2s, 0);
        }
        _ => {
            dev_err((*dai).dev, b"invalid I2S frame format 0x%x\n\0".as_ptr() as *const c_char, fmt);
            return -EINVAL;
        }
    }

    mask |= I2S_CTRL_EDGE_CTRL_MASK;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => val |= I2S_CTRL_EDGE_CTRL_POS_EDGE,
        SND_SOC_DAIFMT_NB_IF => {
            val |= I2S_CTRL_EDGE_CTRL_POS_EDGE;
            val ^= I2S_CTRL_LRCK_POL_MASK;
        }
        SND_SOC_DAIFMT_IB_NF => val |= I2S_CTRL_EDGE_CTRL_NEG_EDGE,
        SND_SOC_DAIFMT_IB_IF => {
            val |= I2S_CTRL_EDGE_CTRL_NEG_EDGE;
            val ^= I2S_CTRL_LRCK_POL_MASK;
        }
        _ => {
            dev_err((*dai).dev, b"invalid I2S clock inversion 0x%x\n\0".as_ptr() as *const c_char, fmt);
            return -EINVAL;
        }
    }

    regmap_update_bits((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, mask, val);
    (*i2s).dai_fmt = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    0
}

unsafe extern "C" fn tegra210_i2s_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, _slots: c_int, _slot_width: c_int) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra210_i2s;
    /* Copy the required tx and rx mask */
    (*i2s).tx_mask = if tx_mask > (*(*i2s).soc_data).slot_mask { (*(*i2s).soc_data).slot_mask } else { tx_mask };
    (*i2s).rx_mask = if rx_mask > (*(*i2s).soc_data).slot_mask { (*(*i2s).soc_data).slot_mask } else { rx_mask };
    0
}

macro_rules! get_int_control {
    ($name:ident, $field:ident) => {
        unsafe extern "C" fn $name(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
            let compnt = snd_kcontrol_chip(kcontrol);
            let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
            (*ucontrol).value.integer.value[0] = (*i2s).$field;
            0
        }
    };
}

get_int_control!(tegra210_i2s_get_loopback, loopback);
get_int_control!(tegra210_i2s_get_fsync_width, fsync_width);
get_int_control!(tegra210_i2s_pget_fifo_th, rx_fifo_th);

unsafe extern "C" fn tegra210_i2s_put_loopback(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let compnt = snd_kcontrol_chip(kcontrol);
    let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
    let value = (*ucontrol).value.integer.value[0];
    if value == (*i2s).loopback {
        return 0;
    }
    (*i2s).loopback = value;
    regmap_update_bits((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, I2S_CTRL_LPBK_MASK, ((*i2s).loopback as c_uint) << I2S_CTRL_LPBK_SHIFT);
    1
}

unsafe extern "C" fn tegra210_i2s_put_fsync_width(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let compnt = snd_kcontrol_chip(kcontrol);
    let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
    let value = (*ucontrol).value.integer.value[0];
    if value == (*i2s).fsync_width {
        return 0;
    }
    (*i2s).fsync_width = value;
    /*
     * Frame sync width is used only for FSYNC modes and not
     * applicable for LRCK modes. Reset value for this field is "0",
     * which means the width is one bit clock wide.
     * The width requirement may depend on the codec and in such
     * cases mixer control is used to update custom values. A value
     * of "N" here means, width is "N + 1" bit clock wide.
     */
    regmap_update_bits((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, (*(*i2s).soc_data).fsync_width_mask, ((*i2s).fsync_width as c_uint) << (*(*i2s).soc_data).fsync_width_shift);
    1
}

macro_rules! enum_get_put {
    ($get:ident, $put:ident, $field:ident, $path:expr) => {
        unsafe extern "C" fn $get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
            let compnt = snd_kcontrol_chip(kcontrol);
            let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
            (*ucontrol).value.enumerated.item[0] = (*i2s).$field[$path as usize];
            0
        }
        unsafe extern "C" fn $put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
            let compnt = snd_kcontrol_chip(kcontrol);
            let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
            let value = (*ucontrol).value.enumerated.item[0];
            if value == (*i2s).$field[$path as usize] {
                return 0;
            }
            (*i2s).$field[$path as usize] = value;
            1
        }
    };
}

enum_get_put!(tegra210_i2s_cget_stereo_to_mono, tegra210_i2s_cput_stereo_to_mono, stereo_to_mono, I2S_TX_PATH);
enum_get_put!(tegra210_i2s_cget_mono_to_stereo, tegra210_i2s_cput_mono_to_stereo, mono_to_stereo, I2S_TX_PATH);
enum_get_put!(tegra210_i2s_pget_stereo_to_mono, tegra210_i2s_pput_stereo_to_mono, stereo_to_mono, I2S_RX_PATH);
enum_get_put!(tegra210_i2s_pget_mono_to_stereo, tegra210_i2s_pput_mono_to_stereo, mono_to_stereo, I2S_RX_PATH);

unsafe extern "C" fn tegra210_i2s_pput_fifo_th(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let compnt = snd_kcontrol_chip(kcontrol);
    let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
    let value = (*ucontrol).value.integer.value[0];
    if value == (*i2s).rx_fifo_th {
        return 0;
    }
    (*i2s).rx_fifo_th = value;
    1
}

unsafe extern "C" fn tegra210_i2s_get_bclk_ratio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let compnt = snd_kcontrol_chip(kcontrol);
    let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
    (*ucontrol).value.integer.value[0] = (*i2s).bclk_ratio as c_int;
    0
}

unsafe extern "C" fn tegra210_i2s_put_bclk_ratio(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let compnt = snd_kcontrol_chip(kcontrol);
    let i2s = snd_soc_component_get_drvdata(compnt) as *mut tegra210_i2s;
    let value = (*ucontrol).value.integer.value[0];
    if value as c_uint == (*i2s).bclk_ratio {
        return 0;
    }
    (*i2s).bclk_ratio = value as c_uint;
    1
}

unsafe extern "C" fn tegra210_i2s_set_dai_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra210_i2s;
    (*i2s).bclk_ratio = ratio;
    0
}

unsafe extern "C" fn tegra210_i2s_set_timing_params(dev: *mut device, sample_size: c_uint, srate: c_uint, channels: c_uint) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    let mut val: c_uint = 0;
    let bit_count: c_uint;
    let mut num_bclk = sample_size;
    if (*i2s).bclk_ratio != 0 {
        num_bclk = num_bclk.wrapping_mul((*i2s).bclk_ratio);
    }
    if (*i2s).dai_fmt == SND_SOC_DAIFMT_RIGHT_J {
        tegra210_i2s_set_data_offset(i2s, num_bclk - sample_size);
    }

    /* I2S bit clock rate */
    let bclk_rate = srate.wrapping_mul(channels).wrapping_mul(num_bclk);
    let err = tegra210_i2s_set_clock_rate(dev, bclk_rate);
    if err != 0 {
        dev_err(dev, b"can't set I2S bit clock rate %u, err: %d\n\0".as_ptr() as *const c_char, bclk_rate, err);
        return err;
    }

    regmap_read((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, &mut val);
    /*
     * For LRCK mode, channel bit count depends on number of bit clocks
     * on the left channel, where as for FSYNC mode bit count depends on
     * the number of bit clocks in both left and right channels for DSP
     * mode or the number of bit clocks in one TDM frame.
     *
     */
    match val & I2S_CTRL_FRAME_FMT_MASK {
        I2S_CTRL_FRAME_FMT_LRCK_MODE => bit_count = (bclk_rate / (srate * 2)) - 1,
        I2S_CTRL_FRAME_FMT_FSYNC_MODE => {
            bit_count = (bclk_rate / srate) - 1;
            tegra210_i2s_set_slot_ctrl(i2s, channels, (*i2s).tx_mask, (*i2s).rx_mask);
        }
        _ => {
            dev_err(dev, b"invalid I2S frame format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if bit_count > I2S_TIMING_CH_BIT_CNT_MASK {
        dev_err(dev, b"invalid I2S channel bit count %u\n\0".as_ptr() as *const c_char, bit_count);
        return -EINVAL;
    }
    regmap_write((*i2s).regmap, TEGRA210_I2S_TIMING + (*(*i2s).soc_data).i2s_ctrl_offset, bit_count << I2S_TIMING_CH_BIT_CNT_SHIFT);
    0
}

unsafe extern "C" fn tegra210_i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let dev = (*dai).dev;
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra210_i2s;
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();
    let channels = params_channels(params);
    if channels < 1 {
        dev_err(dev, b"invalid I2S %d channel configuration\n\0".as_ptr() as *const c_char, channels);
        return -EINVAL;
    }

    cif_conf.audio_ch = channels;
    cif_conf.client_ch = channels;
    if (*i2s).client_channels != 0 {
        cif_conf.client_ch = (*i2s).client_channels;
    }

    /* AHUB CIF Audio bits configs */
    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => cif_conf.audio_bits = TEGRA_ACIF_BITS_8,
        SNDRV_PCM_FORMAT_S16_LE => cif_conf.audio_bits = TEGRA_ACIF_BITS_16,
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S32_LE => cif_conf.audio_bits = TEGRA_ACIF_BITS_32,
        _ => {
            dev_err(dev, b"unsupported params audio bit format!\n\0".as_ptr() as *const c_char);
            return -EOPNOTSUPP;
        }
    }

    let mut sample_format = params_format(params);
    if (*i2s).client_sample_format >= 0 {
        sample_format = (*i2s).client_sample_format as snd_pcm_format_t;
    }

    let val: c_uint;
    let sample_size: c_uint;
    /*
     * Format of the I2S for sending/receiving the audio
     * to/from external device.
     */
    match sample_format {
        SNDRV_PCM_FORMAT_S8 => {
            val = I2S_BITS_8;
            sample_size = 8;
            cif_conf.client_bits = TEGRA_ACIF_BITS_8;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            val = I2S_BITS_16;
            sample_size = 16;
            cif_conf.client_bits = TEGRA_ACIF_BITS_16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            val = I2S_BITS_24;
            sample_size = 32;
            cif_conf.client_bits = TEGRA_ACIF_BITS_24;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            val = I2S_BITS_32;
            sample_size = 32;
            cif_conf.client_bits = TEGRA_ACIF_BITS_32;
        }
        _ => {
            dev_err(dev, b"unsupported client bit format!\n\0".as_ptr() as *const c_char);
            return -EOPNOTSUPP;
        }
    }

    /* Program sample size */
    regmap_update_bits((*i2s).regmap, TEGRA210_I2S_CTRL + (*(*i2s).soc_data).i2s_ctrl_offset, I2S_CTRL_BIT_SIZE_MASK, val);
    let srate = params_rate(params);

    /* For playback I2S RX-CIF and for capture TX-CIF is used */
    let path = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { I2S_RX_PATH } else { I2S_TX_PATH };
    let reg: c_uint;
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        let max_th = (I2S_RX_FIFO_DEPTH / cif_conf.audio_ch) - 1;
        if ((*i2s).rx_fifo_th as c_uint) > max_th {
            (*i2s).rx_fifo_th = max_th as c_int;
        }
        cif_conf.threshold = (*i2s).rx_fifo_th as c_uint;
        reg = TEGRA210_I2S_RX_CIF_CTRL;
    } else {
        reg = TEGRA210_I2S_TX_CIF_CTRL + (*(*i2s).soc_data).tx_offset;
    }

    cif_conf.mono_conv = (*i2s).mono_to_stereo[path as usize];
    cif_conf.stereo_conv = (*i2s).stereo_to_mono[path as usize];
    if (*(*i2s).soc_data).max_ch == TEGRA264_I2S_MAX_CHANNEL {
        tegra264_set_cif((*i2s).regmap, reg, &mut cif_conf);
    } else {
        tegra_set_cif((*i2s).regmap, reg, &mut cif_conf);
    }

    tegra210_i2s_set_timing_params(dev, sample_size, srate, cif_conf.client_ch)
}

static tegra210_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(tegra210_i2s_set_fmt),
    hw_params: Some(tegra210_i2s_hw_params),
    set_bclk_ratio: Some(tegra210_i2s_set_dai_bclk_ratio),
    set_tdm_slot: Some(tegra210_i2s_set_tdm_slot),
};

static mut tegra210_i2s_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"I2S-CIF\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"CIF-Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 16, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
        capture: snd_soc_pcm_stream { stream_name: b"CIF-Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 16, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
        ops: ptr::null(),
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: b"I2S-DAP\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"DAP-Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 16, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
        capture: snd_soc_pcm_stream { stream_name: b"DAP-Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 16, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
        ops: &tegra210_i2s_dai_ops,
        symmetric_rate: 1,
    },
];

static tegra210_i2s_stereo_conv_text: [*const c_char; 3] = [
    b"CH0\0".as_ptr() as *const c_char,
    b"CH1\0".as_ptr() as *const c_char,
    b"AVG\0".as_ptr() as *const c_char,
];

static tegra210_i2s_mono_conv_text: [*const c_char; 2] = [
    b"Zero\0".as_ptr() as *const c_char,
    b"Copy\0".as_ptr() as *const c_char,
];

static tegra210_i2s_mono_conv_enum: soc_enum = soc_enum { reg: 0, shift_l: 0, items: ARRAY_SIZE(&tegra210_i2s_mono_conv_text), texts: tegra210_i2s_mono_conv_text.as_ptr() };
static tegra210_i2s_stereo_conv_enum: soc_enum = soc_enum { reg: 0, shift_l: 0, items: ARRAY_SIZE(&tegra210_i2s_stereo_conv_text), texts: tegra210_i2s_stereo_conv_text.as_ptr() };

static tegra210_i2s_controls: [snd_kcontrol_new; 8] = [
    snd_kcontrol_new { name: b"Loopback\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"FSYNC Width\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Capture Stereo To Mono\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Capture Mono To Stereo\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Playback Stereo To Mono\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Playback Mono To Stereo\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Playback FIFO Threshold\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"BCLK Ratio\0".as_ptr() as *const c_char },
];

/* TEGRA_I2S_WIDGETS(tx_enable_reg) expands to RX/TX AIF widgets plus MIC/SPK widgets. */
static tegra210_i2s_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: TEGRA210_I2S_RX_ENABLE },
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: TEGRA210_I2S_TX_ENABLE },
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: 0 },
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: 0 },
];

static tegra264_i2s_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: TEGRA210_I2S_RX_ENABLE },
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: TEGRA264_I2S_TX_ENABLE },
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: 0 },
    snd_soc_dapm_widget { dapm: ptr::null_mut(), reg: 0 },
];

static tegra210_i2s_routes: [snd_soc_dapm_route; 10] = [
    /* Playback route from XBAR */
    snd_soc_dapm_route { sink: b"XBAR-Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"XBAR-TX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CIF-Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"XBAR-Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CIF-Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAP-Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"RX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPK\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAP-Playback\0".as_ptr() as *const c_char },
    /* Capture route to XBAR */
    snd_soc_dapm_route { sink: b"XBAR-RX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"XBAR-Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"XBAR-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CIF-Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CIF-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAP-Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAP-Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MIC\0".as_ptr() as *const c_char },
];

static tegra210_i2s_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra210_i2s_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&tegra210_i2s_widgets),
    dapm_routes: tegra210_i2s_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&tegra210_i2s_routes),
    controls: tegra210_i2s_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tegra210_i2s_controls),
};

static tegra264_i2s_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra264_i2s_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&tegra264_i2s_widgets),
    dapm_routes: tegra210_i2s_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&tegra210_i2s_routes),
    controls: tegra210_i2s_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tegra210_i2s_controls),
};

unsafe extern "C" fn tegra210_i2s_wr_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        TEGRA210_I2S_RX_ENABLE..=TEGRA210_I2S_RX_SOFT_RESET
        | TEGRA210_I2S_RX_INT_MASK..=TEGRA210_I2S_RX_CLK_TRIM
        | TEGRA210_I2S_TX_ENABLE..=TEGRA210_I2S_TX_SOFT_RESET
        | TEGRA210_I2S_TX_INT_MASK..=TEGRA210_I2S_TX_CLK_TRIM
        | TEGRA210_I2S_ENABLE..=TEGRA210_I2S_CG
        | TEGRA210_I2S_CTRL..=TEGRA210_I2S_CYA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_i2s_rd_reg(dev: *mut device, reg: c_uint) -> bool_ {
    if tegra210_i2s_wr_reg(dev, reg) {
        return true;
    }
    match reg {
        TEGRA210_I2S_RX_STATUS | TEGRA210_I2S_RX_INT_STATUS | TEGRA210_I2S_RX_CIF_FIFO_STATUS |
        TEGRA210_I2S_TX_STATUS | TEGRA210_I2S_TX_INT_STATUS | TEGRA210_I2S_TX_CIF_FIFO_STATUS |
        TEGRA210_I2S_STATUS | TEGRA210_I2S_INT_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        TEGRA210_I2S_RX_STATUS | TEGRA210_I2S_RX_INT_STATUS | TEGRA210_I2S_RX_CIF_FIFO_STATUS |
        TEGRA210_I2S_TX_STATUS | TEGRA210_I2S_TX_INT_STATUS | TEGRA210_I2S_TX_CIF_FIFO_STATUS |
        TEGRA210_I2S_STATUS | TEGRA210_I2S_INT_STATUS | TEGRA210_I2S_RX_SOFT_RESET |
        TEGRA210_I2S_TX_SOFT_RESET => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra264_i2s_wr_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        TEGRA210_I2S_RX_ENABLE..=TEGRA210_I2S_RX_SOFT_RESET
        | TEGRA210_I2S_RX_INT_MASK..=TEGRA264_I2S_RX_CYA
        | TEGRA264_I2S_TX_ENABLE..=TEGRA264_I2S_TX_SOFT_RESET
        | TEGRA264_I2S_TX_INT_MASK..=TEGRA264_I2S_TX_FIFO_RD_ACCESS_MODE
        | TEGRA264_I2S_TX_FIFO_THRESHOLD..=TEGRA264_I2S_TX_CYA
        | TEGRA264_I2S_ENABLE..=TEGRA264_I2S_CG
        | TEGRA264_I2S_INT_SET..=TEGRA264_I2S_INT_MASK
        | TEGRA264_I2S_CTRL..=TEGRA264_I2S_CYA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra264_i2s_rd_reg(dev: *mut device, reg: c_uint) -> bool_ {
    if tegra264_i2s_wr_reg(dev, reg) {
        return true;
    }
    match reg {
        TEGRA210_I2S_RX_STATUS | TEGRA210_I2S_RX_INT_STATUS | TEGRA264_I2S_RX_CIF_FIFO_STATUS |
        TEGRA264_I2S_TX_STATUS | TEGRA264_I2S_TX_INT_STATUS | TEGRA264_I2S_TX_FIFO_RD_DATA |
        TEGRA264_I2S_TX_CIF_FIFO_STATUS | TEGRA264_I2S_STATUS | TEGRA264_I2S_INT_STATUS |
        TEGRA264_I2S_PIO_MODE_ENABLE | TEGRA264_I2S_PAD_MACRO_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra264_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        TEGRA210_I2S_RX_SOFT_RESET | TEGRA210_I2S_RX_STATUS | TEGRA210_I2S_RX_INT_STATUS |
        TEGRA264_I2S_RX_CIF_FIFO_STATUS | TEGRA264_I2S_TX_STATUS | TEGRA264_I2S_TX_INT_STATUS |
        TEGRA264_I2S_TX_FIFO_RD_DATA | TEGRA264_I2S_TX_CIF_FIFO_STATUS | TEGRA264_I2S_STATUS |
        TEGRA264_I2S_INT_STATUS | TEGRA264_I2S_TX_SOFT_RESET | TEGRA264_I2S_PAD_MACRO_STATUS => true,
        _ => false,
    }
}

static tegra210_regmap_conf: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA210_I2S_CYA,
    writeable_reg: Some(tegra210_i2s_wr_reg),
    readable_reg: Some(tegra210_i2s_rd_reg),
    volatile_reg: Some(tegra210_i2s_volatile_reg),
    reg_defaults: tegra210_i2s_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tegra210_i2s_reg_defaults),
    reg_default_cb: unsafe { &regmap_default_zero_cb as *const c_void },
    cache_type: REGCACHE_FLAT,
};

/*
 * The AHUB HW modules are interconnected with CIF which are capable of
 * supporting Channel and Sample bit format conversion. This needs different
 * CIF Audio and client configuration. As one of the config comes from
 * params_channels() or params_format(), the extra configuration is passed from
 * CIF Port of DT I2S node which can help to perform this conversion.
 *
 *    4ch          audio = 4ch      client = 2ch       2ch
 *   -----> ADMAIF -----------> CIF -------------> I2S ---->
 */
unsafe extern "C" fn tegra210_parse_client_convert(dev: *mut device) {
    let i2s = dev_get_drvdata(dev) as *mut tegra210_i2s;
    let mut data: simple_util_data = core::mem::zeroed();
    let cif_port: c_int = 0;
    let ports = of_get_child_by_name((*dev).of_node, b"ports\0".as_ptr() as *const c_char);
    if !ports.is_null() {
        let ep = of_graph_get_endpoint_by_regs(ports, cif_port, -1);
        if !ep.is_null() {
            simple_util_parse_convert(ep, ptr::null(), &mut data);
            of_node_put(ep);
        }
        of_node_put(ports);
    }
    if data.convert_channels != 0 {
        (*i2s).client_channels = data.convert_channels;
    }
    if data.convert_sample_format != 0 {
        (*i2s).client_sample_format = simple_util_get_sample_fmt(&mut data);
    }
}

static tegra264_regmap_conf: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA264_I2S_PAD_MACRO_STATUS,
    writeable_reg: Some(tegra264_i2s_wr_reg),
    readable_reg: Some(tegra264_i2s_rd_reg),
    volatile_reg: Some(tegra264_i2s_volatile_reg),
    reg_defaults: tegra264_i2s_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tegra264_i2s_reg_defaults),
    reg_default_cb: unsafe { &regmap_default_zero_cb as *const c_void },
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn tegra210_i2s_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let i2s = devm_kzalloc(dev, size_of::<tegra210_i2s>(), GFP_KERNEL) as *mut tegra210_i2s;
    if i2s.is_null() {
        return -ENOMEM;
    }

    (*i2s).soc_data = of_device_get_match_data(&mut (*pdev).dev) as *const tegra_i2s_soc_data;
    (*i2s).rx_fifo_th = DEFAULT_I2S_RX_FIFO_THRESHOLD as c_int;
    (*i2s).tx_mask = (*(*i2s).soc_data).slot_mask;
    (*i2s).rx_mask = (*(*i2s).soc_data).slot_mask;
    (*i2s).loopback = 0;
    (*i2s).client_sample_format = -EINVAL;

    dev_set_drvdata(dev, i2s as *mut c_void);

    (*i2s).clk_i2s = devm_clk_get(dev, b"i2s\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).clk_i2s as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*i2s).clk_i2s as *const c_void), b"can't retrieve I2S bit clock\n\0".as_ptr() as *const c_char);
    }

    /*
     * This clock is optional and is only needed when another I/O uses
     * the current I2S instance as its input clock, as configured in DT.
     */
    (*i2s).clk_sync_input = devm_clk_get_optional(dev, b"sync_input\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).clk_sync_input as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*i2s).clk_sync_input as *const c_void), b"can't retrieve I2S sync input clock\n\0".as_ptr() as *const c_char);
    }

    let regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*i2s).regmap = devm_regmap_init_mmio(dev, regs, (*(*i2s).soc_data).regmap_conf);
    if IS_ERR((*i2s).regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*i2s).regmap as *const c_void), b"regmap init failed\n\0".as_ptr() as *const c_char);
    }

    tegra210_parse_client_convert(dev);
    regcache_cache_only((*i2s).regmap, true);

    /* Update the dais max channel as per soc */
    let mut id: usize = 0;
    while id < tegra210_i2s_dais.len() {
        tegra210_i2s_dais[id].playback.channels_max = (*(*i2s).soc_data).max_ch;
        tegra210_i2s_dais[id].capture.channels_max = (*(*i2s).soc_data).max_ch;
        id += 1;
    }

    let err = devm_snd_soc_register_component(dev, (*(*i2s).soc_data).i2s_cmpnt, tegra210_i2s_dais.as_mut_ptr(), ARRAY_SIZE(&tegra210_i2s_dais));
    if err != 0 {
        return dev_err_probe(dev, err, b"can't register I2S component\n\0".as_ptr() as *const c_char);
    }

    pm_runtime_enable(dev);
    0
}

unsafe extern "C" fn tegra210_i2s_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

/* RUNTIME_PM_OPS(tegra210_i2s_runtime_suspend, tegra210_i2s_runtime_resume, NULL)
 * SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
 */
static tegra210_i2s_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static soc_data_tegra210: tegra_i2s_soc_data = tegra_i2s_soc_data {
    regmap_conf: &tegra210_regmap_conf,
    i2s_cmpnt: &tegra210_i2s_cmpnt,
    max_ch: TEGRA210_I2S_MAX_CHANNEL,
    enable_reg: TEGRA210_I2S_ENABLE,
    tx_offset: TEGRA210_I2S_TX_OFFSET,
    i2s_ctrl_offset: TEGRA210_I2S_CTRL_OFFSET,
    fsync_width_mask: I2S_CTRL_FSYNC_WIDTH_MASK,
    fsync_width_shift: I2S_FSYNC_WIDTH_SHIFT,
    slot_mask: DEFAULT_I2S_SLOT_MASK,
};

static soc_data_tegra264: tegra_i2s_soc_data = tegra_i2s_soc_data {
    regmap_conf: &tegra264_regmap_conf,
    i2s_cmpnt: &tegra264_i2s_cmpnt,
    max_ch: TEGRA264_I2S_MAX_CHANNEL,
    enable_reg: TEGRA264_I2S_ENABLE,
    tx_offset: TEGRA264_I2S_TX_OFFSET,
    i2s_ctrl_offset: TEGRA264_I2S_CTRL_OFFSET,
    fsync_width_mask: TEGRA264_I2S_CTRL_FSYNC_WIDTH_MASK,
    fsync_width_shift: TEGRA264_I2S_FSYNC_WIDTH_SHIFT,
    slot_mask: TEGRA264_DEFAULT_I2S_SLOT_MASK,
};

static tegra210_i2s_of_match: [of_device_id; 3] = [
    of_device_id { compatible: b"nvidia,tegra210-i2s\0".as_ptr() as *const c_char, data: &soc_data_tegra210 as *const _ as *const c_void },
    of_device_id { compatible: b"nvidia,tegra264-i2s\0".as_ptr() as *const c_char, data: &soc_data_tegra264 as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, tegra210_i2s_of_match); */

static mut tegra210_i2s_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"tegra210-i2s\0".as_ptr() as *const c_char,
        of_match_table: tegra210_i2s_of_match.as_ptr(),
        pm: &tegra210_i2s_pm_ops,
    },
    probe: Some(tegra210_i2s_probe),
    remove: Some(tegra210_i2s_remove),
};
/* module_platform_driver(tegra210_i2s_driver) */

/* MODULE_AUTHOR("Songhee Baek <sbaek@nvidia.com>"); */
/* MODULE_DESCRIPTION("Tegra210 ASoC I2S driver"); */
/* MODULE_LICENSE("GPL v2"); */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
