// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra20_i2s.rs - Tegra20 I2S driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2010,2012 - NVIDIA, Inc.
 *
 * Based on code copyright/by:
 *
 * Copyright (c) 2009-2010, NVIDIA Corporation.
 * Scott Peterson <speterson@nvidia.com>
 *
 * Copyright (C) 2010 Google, Inc.
 * Iliyan Malchev <malchev@google.com>
 */

// Dependencies correspond to the original Linux kernel and local
// "tegra20_i2s.h" includes.

const DRV_NAME: *const core::ffi::c_char = b"tegra20-i2s\0".as_ptr() as *const core::ffi::c_char;

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_name(dev: *const device) -> *const core::ffi::c_char;
    fn dev_err(dev: *const device, fmt: *const core::ffi::c_char, ...);

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut core::ffi::c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn snd_soc_unregister_component(dev: *mut device);

    fn params_format(params: *mut snd_pcm_hw_params) -> core::ffi::c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> core::ffi::c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> core::ffi::c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: core::ffi::c_int) -> *mut snd_interval;
    fn snd_interval_list(
        i: *mut snd_interval,
        count: core::ffi::c_uint,
        list: *const core::ffi::c_uint,
        mask: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: core::ffi::c_uint,
        var: core::ffi::c_int,
        func: Option<
            unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> core::ffi::c_int,
        >,
        private: *mut core::ffi::c_void,
        dep: core::ffi::c_int,
        ...
    ) -> core::ffi::c_int;

    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> core::ffi::c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: core::ffi::c_uint,
        mask: core::ffi::c_uint,
        val: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn regmap_write(map: *mut regmap, reg: core::ffi::c_uint, val: core::ffi::c_uint) -> core::ffi::c_int;

    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> core::ffi::c_int;
    fn clk_set_rate(clk: *mut clk, rate: core::ffi::c_ulong) -> core::ffi::c_int;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> core::ffi::c_ulong;
    fn devm_clk_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;

    fn reset_control_assert(rstc: *mut reset_control) -> core::ffi::c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> core::ffi::c_int;
    fn devm_reset_control_get_exclusive(
        dev: *mut device,
        id: *const core::ffi::c_char,
    ) -> *mut reset_control;

    fn usleep_range(min: core::ffi::c_ulong, max: core::ffi::c_ulong);
    fn device_property_read_bool(dev: *mut device, propname: *const core::ffi::c_char) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: core::ffi::c_uint,
        res: *mut *mut resource,
    ) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> core::ffi::c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> core::ffi::c_int;
    fn tegra_pcm_platform_register(dev: *mut device) -> core::ffi::c_int;
    fn tegra_pcm_platform_unregister(dev: *mut device);

    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_int;
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
}

type gfp_t = core::ffi::c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    start: core::ffi::c_ulong,
}

#[repr(C)]
pub struct snd_pcm_substream {
    stream: core::ffi::c_int,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    var: core::ffi::c_int,
    private: *mut snd_soc_dai,
}

#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_dai_dma_data {
    addr: core::ffi::c_ulong,
    addr_width: core::ffi::c_int,
    maxburst: core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    stream_name: *const core::ffi::c_char,
    channels_min: core::ffi::c_uint,
    channels_max: core::ffi::c_uint,
    rates: core::ffi::c_uint,
    formats: core::ffi::c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_driver {
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: core::ffi::c_uint,
    name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> core::ffi::c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, core::ffi::c_uint) -> core::ffi::c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> core::ffi::c_int,
    >,
    trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, core::ffi::c_int, *mut snd_soc_dai) -> core::ffi::c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const core::ffi::c_char,
    legacy_dai_naming: core::ffi::c_uint,
}

#[repr(C)]
pub struct regmap_config {
    reg_bits: core::ffi::c_int,
    reg_stride: core::ffi::c_int,
    val_bits: core::ffi::c_int,
    max_register: core::ffi::c_uint,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    precious_reg: Option<unsafe extern "C" fn(*mut device, core::ffi::c_uint) -> bool>,
    cache_type: core::ffi::c_int,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    driver: driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct tegra20_i2s {
    dai: snd_soc_dai_driver,
    playback_dma_data: snd_dmaengine_dai_dma_data,
    capture_dma_data: snd_dmaengine_dai_dma_data,
    regmap: *mut regmap,
    clk_i2s: *mut clk,
    reset: *mut reset_control,
}

extern "C" {
    static TEGRA20_I2S_CTRL_MASTER_ENABLE: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_FORMAT_MASK: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_LRCK_MASK: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_FORMAT_DSP: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_LRCK_L_LOW: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_LRCK_R_LOW: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_FORMAT_I2S: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_FORMAT_RJM: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_FORMAT_LJM: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_SIZE_MASK: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_SIZE_16: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_SIZE_24: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_BIT_SIZE_32: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_FIFO_FORMAT_MASK: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_FIFO_FORMAT_PACKED: core::ffi::c_uint;
    static TEGRA20_I2S_TIMING_CHANNEL_BIT_COUNT_MASK_US: core::ffi::c_int;
    static TEGRA20_I2S_TIMING_CHANNEL_BIT_COUNT_SHIFT: core::ffi::c_int;
    static TEGRA20_I2S_TIMING_NON_SYM_ENABLE: core::ffi::c_uint;
    static TEGRA20_I2S_FIFO_SCR_FIFO2_ATN_LVL_FOUR_SLOTS: core::ffi::c_uint;
    static TEGRA20_I2S_FIFO_SCR_FIFO1_ATN_LVL_FOUR_SLOTS: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_FIFO1_ENABLE: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL_FIFO2_ENABLE: core::ffi::c_uint;
    static TEGRA20_I2S_CTRL: core::ffi::c_uint;
    static TEGRA20_I2S_STATUS: core::ffi::c_uint;
    static TEGRA20_I2S_TIMING: core::ffi::c_uint;
    static TEGRA20_I2S_FIFO_SCR: core::ffi::c_uint;
    static TEGRA20_I2S_PCM_CTRL: core::ffi::c_uint;
    static TEGRA20_I2S_NW_CTRL: core::ffi::c_uint;
    static TEGRA20_I2S_TDM_CTRL: core::ffi::c_uint;
    static TEGRA20_I2S_TDM_TX_RX_CTRL: core::ffi::c_uint;
    static TEGRA20_I2S_FIFO1: core::ffi::c_uint;
    static TEGRA20_I2S_FIFO2: core::ffi::c_uint;
}

const EINVAL: core::ffi::c_int = 22;
const ENOMEM: core::ffi::c_int = 12;
const GFP_KERNEL: gfp_t = 0;
const SNDRV_PCM_FORMAT_S16_LE: core::ffi::c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: core::ffi::c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: core::ffi::c_int = 10;
const SNDRV_PCM_TRIGGER_START: core::ffi::c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: core::ffi::c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: core::ffi::c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: core::ffi::c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: core::ffi::c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: core::ffi::c_int = 6;
const SNDRV_PCM_STREAM_PLAYBACK: core::ffi::c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: core::ffi::c_int = 10;
const SND_SOC_DAIFMT_INV_MASK: core::ffi::c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: core::ffi::c_uint = 0x0000;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: core::ffi::c_uint = 0xf000;
const SND_SOC_DAIFMT_BP_FP: core::ffi::c_uint = 0x1000;
const SND_SOC_DAIFMT_BC_FC: core::ffi::c_uint = 0x0000;
const SND_SOC_DAIFMT_FORMAT_MASK: core::ffi::c_uint = 0x000f;
const SND_SOC_DAIFMT_DSP_A: core::ffi::c_uint = 0x0004;
const SND_SOC_DAIFMT_DSP_B: core::ffi::c_uint = 0x0005;
const SND_SOC_DAIFMT_I2S: core::ffi::c_uint = 0x0001;
const SND_SOC_DAIFMT_RIGHT_J: core::ffi::c_uint = 0x0003;
const SND_SOC_DAIFMT_LEFT_J: core::ffi::c_uint = 0x0002;
const SNDRV_PCM_RATE_8000_96000: core::ffi::c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: core::ffi::c_ulonglong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const DMA_SLAVE_BUSWIDTH_4_BYTES: core::ffi::c_int = 4;
const REGCACHE_FLAT: core::ffi::c_int = 1;

const fn BIT(nr: usize) -> core::ffi::c_ulong {
    1core::ffi::c_ulong << nr
}

unsafe extern "C" fn tegra20_i2s_runtime_suspend(dev: *mut device) -> core::ffi::c_int {
    let i2s = dev_get_drvdata(dev) as *mut tegra20_i2s;

    regcache_cache_only((*i2s).regmap, true);

    clk_disable_unprepare((*i2s).clk_i2s);

    0
}

unsafe extern "C" fn tegra20_i2s_runtime_resume(dev: *mut device) -> core::ffi::c_int {
    let i2s = dev_get_drvdata(dev) as *mut tegra20_i2s;
    let mut ret: core::ffi::c_int;

    ret = reset_control_assert((*i2s).reset);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*i2s).clk_i2s);
    if ret != 0 {
        dev_err(dev, b"clk_enable failed: %d\n\0".as_ptr() as *const core::ffi::c_char, ret);
        return ret;
    }

    usleep_range(10, 100);

    ret = reset_control_deassert((*i2s).reset);
    if ret != 0 {
        clk_disable_unprepare((*i2s).clk_i2s);
        return ret;
    }

    regcache_cache_only((*i2s).regmap, false);
    regcache_mark_dirty((*i2s).regmap);

    ret = regcache_sync((*i2s).regmap);
    if ret != 0 {
        clk_disable_unprepare((*i2s).clk_i2s);
        return ret;
    }

    0
}

unsafe extern "C" fn tegra20_i2s_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra20_i2s;
    let mut mask: core::ffi::c_uint = 0;
    let mut val: core::ffi::c_uint = 0;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    mask |= TEGRA20_I2S_CTRL_MASTER_ENABLE;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => val |= TEGRA20_I2S_CTRL_MASTER_ENABLE,
        SND_SOC_DAIFMT_BC_FC => {}
        _ => return -EINVAL,
    }

    mask |= TEGRA20_I2S_CTRL_BIT_FORMAT_MASK | TEGRA20_I2S_CTRL_LRCK_MASK;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            val |= TEGRA20_I2S_CTRL_BIT_FORMAT_DSP;
            val |= TEGRA20_I2S_CTRL_LRCK_L_LOW;
        }
        SND_SOC_DAIFMT_DSP_B => {
            val |= TEGRA20_I2S_CTRL_BIT_FORMAT_DSP;
            val |= TEGRA20_I2S_CTRL_LRCK_R_LOW;
        }
        SND_SOC_DAIFMT_I2S => {
            val |= TEGRA20_I2S_CTRL_BIT_FORMAT_I2S;
            val |= TEGRA20_I2S_CTRL_LRCK_L_LOW;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            val |= TEGRA20_I2S_CTRL_BIT_FORMAT_RJM;
            val |= TEGRA20_I2S_CTRL_LRCK_L_LOW;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val |= TEGRA20_I2S_CTRL_BIT_FORMAT_LJM;
            val |= TEGRA20_I2S_CTRL_LRCK_L_LOW;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*i2s).regmap, TEGRA20_I2S_CTRL, mask, val);

    0
}

unsafe extern "C" fn tegra20_i2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let dev = (*dai).dev;
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra20_i2s;
    let mut mask: core::ffi::c_uint;
    let mut val: core::ffi::c_uint;
    let mut ret: core::ffi::c_int;
    let sample_size: core::ffi::c_int;
    let srate: core::ffi::c_int;
    let i2sclock: core::ffi::c_int;
    let bitcnt: core::ffi::c_int;

    mask = TEGRA20_I2S_CTRL_BIT_SIZE_MASK;
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            val = TEGRA20_I2S_CTRL_BIT_SIZE_16;
            sample_size = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            val = TEGRA20_I2S_CTRL_BIT_SIZE_24;
            sample_size = 24;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            val = TEGRA20_I2S_CTRL_BIT_SIZE_32;
            sample_size = 32;
        }
        _ => return -EINVAL,
    }

    mask |= TEGRA20_I2S_CTRL_FIFO_FORMAT_MASK;
    val |= TEGRA20_I2S_CTRL_FIFO_FORMAT_PACKED;

    regmap_update_bits((*i2s).regmap, TEGRA20_I2S_CTRL, mask, val);

    srate = params_rate(params);

    /* Final "* 2" required by Tegra hardware */
    i2sclock = srate * params_channels(params) * sample_size * 2;

    ret = clk_set_rate((*i2s).clk_i2s, i2sclock as core::ffi::c_ulong);
    if ret != 0 {
        dev_err(
            dev,
            b"Can't set I2S clock rate: %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }

    bitcnt = (i2sclock / (2 * srate)) - 1;
    if bitcnt < 0 || bitcnt > TEGRA20_I2S_TIMING_CHANNEL_BIT_COUNT_MASK_US {
        return -EINVAL;
    }
    val = (bitcnt << TEGRA20_I2S_TIMING_CHANNEL_BIT_COUNT_SHIFT) as core::ffi::c_uint;

    if i2sclock % (2 * srate) != 0 {
        val |= TEGRA20_I2S_TIMING_NON_SYM_ENABLE;
    }

    regmap_write((*i2s).regmap, TEGRA20_I2S_TIMING, val);

    regmap_write(
        (*i2s).regmap,
        TEGRA20_I2S_FIFO_SCR,
        TEGRA20_I2S_FIFO_SCR_FIFO2_ATN_LVL_FOUR_SLOTS
            | TEGRA20_I2S_FIFO_SCR_FIFO1_ATN_LVL_FOUR_SLOTS,
    );

    0
}

unsafe extern "C" fn tegra20_i2s_start_playback(i2s: *mut tegra20_i2s) {
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA20_I2S_CTRL,
        TEGRA20_I2S_CTRL_FIFO1_ENABLE,
        TEGRA20_I2S_CTRL_FIFO1_ENABLE,
    );
}

unsafe extern "C" fn tegra20_i2s_stop_playback(i2s: *mut tegra20_i2s) {
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA20_I2S_CTRL,
        TEGRA20_I2S_CTRL_FIFO1_ENABLE,
        0,
    );
}

unsafe extern "C" fn tegra20_i2s_start_capture(i2s: *mut tegra20_i2s) {
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA20_I2S_CTRL,
        TEGRA20_I2S_CTRL_FIFO2_ENABLE,
        TEGRA20_I2S_CTRL_FIFO2_ENABLE,
    );
}

unsafe extern "C" fn tegra20_i2s_stop_capture(i2s: *mut tegra20_i2s) {
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA20_I2S_CTRL,
        TEGRA20_I2S_CTRL_FIFO2_ENABLE,
        0,
    );
}

unsafe extern "C" fn tegra20_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra20_i2s;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tegra20_i2s_start_playback(i2s);
            } else {
                tegra20_i2s_start_capture(i2s);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tegra20_i2s_stop_playback(i2s);
            } else {
                tegra20_i2s_stop_capture(i2s);
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn tegra20_i2s_probe(dai: *mut snd_soc_dai) -> core::ffi::c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra20_i2s;

    snd_soc_dai_init_dma_data(dai, &mut (*i2s).playback_dma_data, &mut (*i2s).capture_dma_data);

    0
}

static tegra20_i2s_rates: [core::ffi::c_uint; 10] =
    [8000, 11025, 16000, 22050, 32000, 44100, 48000, 64000, 88200, 96000];

unsafe extern "C" fn tegra20_i2s_filter_rates(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> core::ffi::c_int {
    let r = hw_param_interval(params, (*rule).var);
    let dai = (*rule).private;
    let i2s = dev_get_drvdata((*dai).dev) as *mut tegra20_i2s;
    let parent = clk_get_parent((*i2s).clk_i2s);
    let mut i: core::ffi::c_ulong;
    let parent_rate: core::ffi::c_ulong;
    let mut valid_rates: core::ffi::c_ulong = 0;

    parent_rate = clk_get_rate(parent);
    if parent_rate == 0 {
        dev_err(
            (*dai).dev,
            b"Can't get parent clock rate\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return -EINVAL;
    }

    i = 0;
    while i < tegra20_i2s_rates.len() as core::ffi::c_ulong {
        if parent_rate % ((tegra20_i2s_rates[i as usize] * 128) as core::ffi::c_ulong) == 0 {
            valid_rates |= BIT(i as usize);
        }
        i += 1;
    }

    /*
     * At least one rate must be valid, otherwise the parent clock isn't
     * audio PLL. Nothing should be filtered in this case.
     */
    if valid_rates == 0 {
        valid_rates = BIT(tegra20_i2s_rates.len()) - 1;
    }

    snd_interval_list(
        r,
        tegra20_i2s_rates.len() as core::ffi::c_uint,
        tegra20_i2s_rates.as_ptr(),
        valid_rates,
    )
}

unsafe extern "C" fn tegra20_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    if !device_property_read_bool(
        (*dai).dev,
        b"nvidia,fixed-parent-rate\0".as_ptr() as *const core::ffi::c_char,
    ) {
        return 0;
    }

    snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        Some(tegra20_i2s_filter_rates),
        dai as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    )
}

static tegra20_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(tegra20_i2s_probe),
    set_fmt: Some(tegra20_i2s_set_fmt),
    hw_params: Some(tegra20_i2s_hw_params),
    trigger: Some(tegra20_i2s_trigger),
    startup: Some(tegra20_i2s_startup),
};

static tegra20_i2s_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const core::ffi::c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const core::ffi::c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &tegra20_i2s_dai_ops,
    symmetric_rate: 1,
    name: core::ptr::null(),
};

static tegra20_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn tegra20_i2s_wr_rd_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if reg == TEGRA20_I2S_CTRL
        || reg == TEGRA20_I2S_STATUS
        || reg == TEGRA20_I2S_TIMING
        || reg == TEGRA20_I2S_FIFO_SCR
        || reg == TEGRA20_I2S_PCM_CTRL
        || reg == TEGRA20_I2S_NW_CTRL
        || reg == TEGRA20_I2S_TDM_CTRL
        || reg == TEGRA20_I2S_TDM_TX_RX_CTRL
        || reg == TEGRA20_I2S_FIFO1
        || reg == TEGRA20_I2S_FIFO2
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn tegra20_i2s_volatile_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if reg == TEGRA20_I2S_STATUS
        || reg == TEGRA20_I2S_FIFO_SCR
        || reg == TEGRA20_I2S_FIFO1
        || reg == TEGRA20_I2S_FIFO2
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn tegra20_i2s_precious_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    if reg == TEGRA20_I2S_FIFO1 || reg == TEGRA20_I2S_FIFO2 {
        true
    } else {
        false
    }
}

static tegra20_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { TEGRA20_I2S_FIFO2 },
    writeable_reg: Some(tegra20_i2s_wr_rd_reg),
    readable_reg: Some(tegra20_i2s_wr_rd_reg),
    volatile_reg: Some(tegra20_i2s_volatile_reg),
    precious_reg: Some(tegra20_i2s_precious_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn tegra20_i2s_platform_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let mut i2s: *mut tegra20_i2s;
    let mut mem: *mut resource = core::ptr::null_mut();
    let regs: *mut core::ffi::c_void;
    let mut ret: core::ffi::c_int;

    i2s = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<tegra20_i2s>(),
        GFP_KERNEL,
    ) as *mut tegra20_i2s;
    if i2s.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    dev_set_drvdata(&mut (*pdev).dev, i2s as *mut core::ffi::c_void);

    (*i2s).dai = tegra20_i2s_dai_template;
    (*i2s).dai.name = dev_name(&(*pdev).dev);

    (*i2s).reset = devm_reset_control_get_exclusive(
        &mut (*pdev).dev,
        b"i2s\0".as_ptr() as *const core::ffi::c_char,
    );
    if IS_ERR((*i2s).reset as *const core::ffi::c_void) {
        dev_err(
            &(*pdev).dev,
            b"Can't retrieve i2s reset\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return PTR_ERR((*i2s).reset as *const core::ffi::c_void);
    }

    (*i2s).clk_i2s = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*i2s).clk_i2s as *const core::ffi::c_void) {
        dev_err(
            &(*pdev).dev,
            b"Can't retrieve i2s clock\n\0".as_ptr() as *const core::ffi::c_char,
        );
        ret = PTR_ERR((*i2s).clk_i2s as *const core::ffi::c_void);
        return ret;
    }

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
    if IS_ERR(regs as *const core::ffi::c_void) {
        ret = PTR_ERR(regs as *const core::ffi::c_void);
        return ret;
    }

    (*i2s).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &tegra20_i2s_regmap_config);
    if IS_ERR((*i2s).regmap as *const core::ffi::c_void) {
        dev_err(
            &(*pdev).dev,
            b"regmap init failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
        ret = PTR_ERR((*i2s).regmap as *const core::ffi::c_void);
        return ret;
    }

    (*i2s).capture_dma_data.addr = (*mem).start + TEGRA20_I2S_FIFO2 as core::ffi::c_ulong;
    (*i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*i2s).capture_dma_data.maxburst = 4;

    (*i2s).playback_dma_data.addr = (*mem).start + TEGRA20_I2S_FIFO1 as core::ffi::c_ulong;
    (*i2s).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*i2s).playback_dma_data.maxburst = 4;

    pm_runtime_enable(&mut (*pdev).dev);

    ret = snd_soc_register_component(
        &mut (*pdev).dev,
        &tegra20_i2s_component,
        &mut (*i2s).dai,
        1,
    );
    if ret != 0 {
        dev_err(
            &(*pdev).dev,
            b"Could not register DAI: %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        ret = -ENOMEM;
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = tegra_pcm_platform_register(&mut (*pdev).dev);
    if ret != 0 {
        dev_err(
            &(*pdev).dev,
            b"Could not register PCM: %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        snd_soc_unregister_component(&mut (*pdev).dev);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    0
}

unsafe extern "C" fn tegra20_i2s_platform_remove(pdev: *mut platform_device) {
    tegra_pcm_platform_unregister(&mut (*pdev).dev);
    snd_soc_unregister_component(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

static tegra20_i2s_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nvidia,tegra20-i2s\0".as_ptr() as *const core::ffi::c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, tegra20_i2s_of_match);

// Original used RUNTIME_PM_OPS(tegra20_i2s_runtime_suspend,
// tegra20_i2s_runtime_resume, NULL) and
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume).
static tegra20_i2s_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut tegra20_i2s_driver: platform_driver = platform_driver {
    driver: driver {
        name: DRV_NAME,
        of_match_table: tegra20_i2s_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&tegra20_i2s_pm_ops) },
    },
    probe: Some(tegra20_i2s_platform_probe),
    remove: Some(tegra20_i2s_platform_remove),
};
// module_platform_driver(tegra20_i2s_driver);

// MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>");
// MODULE_DESCRIPTION("Tegra20 I2S ASoC driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
