// SPDX-License-Identifier: GPL-2.0-only
/*
 * IMG I2S output controller driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

// Dependencies from the original Linux and ALSA include files are declared as
// external symbols/types here and are expected to be supplied by surrounding code.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const IMG_I2S_OUT_TX_FIFO: u32 = 0x0;

const IMG_I2S_OUT_CTL: u32 = 0x4;
const IMG_I2S_OUT_CTL_DATA_EN_MASK: u32 = BIT(24);
const IMG_I2S_OUT_CTL_ACTIVE_CHAN_MASK: u32 = 0xffe000;
const IMG_I2S_OUT_CTL_ACTIVE_CHAN_SHIFT: u32 = 13;
const IMG_I2S_OUT_CTL_FRM_SIZE_MASK: u32 = BIT(8);
const IMG_I2S_OUT_CTL_MASTER_MASK: u32 = BIT(6);
const IMG_I2S_OUT_CTL_CLK_MASK: u32 = BIT(5);
const IMG_I2S_OUT_CTL_CLK_EN_MASK: u32 = BIT(4);
const IMG_I2S_OUT_CTL_FRM_CLK_POL_MASK: u32 = BIT(3);
const IMG_I2S_OUT_CTL_BCLK_POL_MASK: u32 = BIT(2);
const IMG_I2S_OUT_CTL_ME_MASK: u32 = BIT(0);

const IMG_I2S_OUT_CH_CTL: u32 = 0x4;
const IMG_I2S_OUT_CHAN_CTL_CH_MASK: u32 = BIT(11);
const IMG_I2S_OUT_CHAN_CTL_LT_MASK: u32 = BIT(10);
const IMG_I2S_OUT_CHAN_CTL_FMT_MASK: u32 = 0xf0;
const IMG_I2S_OUT_CHAN_CTL_FMT_SHIFT: u32 = 4;
const IMG_I2S_OUT_CHAN_CTL_JUST_MASK: u32 = BIT(3);
const IMG_I2S_OUT_CHAN_CTL_CLKT_MASK: u32 = BIT(1);
const IMG_I2S_OUT_CHAN_CTL_ME_MASK: u32 = BIT(0);

const IMG_I2S_OUT_CH_STRIDE: u32 = 0x20;

type u32 = u32;
type snd_pcm_format_t = c_int;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
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
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: c_ulong,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct dma_slave_config {
    pub dst_addr: c_ulong,
    pub dst_addr_width: c_uint,
    pub dst_maxburst: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut dma_slave_config) -> c_int>,
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

#[repr(C)]
pub struct img_i2s_out {
    pub base: *mut c_void,
    pub clk_sys: *mut clk,
    pub clk_ref: *mut clk,
    pub dma_data: snd_dmaengine_dai_dma_data,
    pub dev: *mut device,
    pub max_i2s_chan: c_uint,
    pub channel_base: *mut c_void,
    pub force_clk_active: bool,
    pub active_channels: c_uint,
    pub rst: *mut reset_control,
    pub dai_driver: snd_soc_dai_driver,
    pub suspend_ctl: u32,
    pub suspend_ch_ctl: *mut u32,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 0;
const SND_SOC_DAIFMT_CLOCK_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CONT: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 0;

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: c_long) -> c_long;
    fn clk_set_rate(clk: *mut clk, rate: c_long) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_long;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn writel(val: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_substream_to_rtd(st: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, st: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_hwparams_to_dma_slave_config(st: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, sc: *mut dma_slave_config) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn get_count_order(count: c_uint) -> c_int;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
}

unsafe extern "C" fn img_i2s_out_runtime_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_out;

    clk_disable_unprepare((*i2s).clk_ref);
    clk_disable_unprepare((*i2s).clk_sys);

    0
}

unsafe extern "C" fn img_i2s_out_runtime_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_out;
    let mut ret: c_int;

    ret = clk_prepare_enable((*i2s).clk_sys);
    if ret != 0 {
        dev_err(dev, c"clk_enable failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = clk_prepare_enable((*i2s).clk_ref);
    if ret != 0 {
        dev_err(dev, c"clk_enable failed: %d\n".as_ptr(), ret);
        clk_disable_unprepare((*i2s).clk_sys);
        return ret;
    }

    0
}

unsafe fn img_i2s_out_writel(i2s: *mut img_i2s_out, val: u32, reg: u32) {
    writel(val, ((*i2s).base as *mut u8).add(reg as usize) as *mut c_void);
}

unsafe fn img_i2s_out_readl(i2s: *mut img_i2s_out, reg: u32) -> u32 {
    readl(((*i2s).base as *mut u8).add(reg as usize) as *mut c_void)
}

unsafe fn img_i2s_out_ch_writel(i2s: *mut img_i2s_out, chan: u32, val: u32, reg: u32) {
    writel(
        val,
        ((*i2s).channel_base as *mut u8)
            .add(chan.wrapping_mul(IMG_I2S_OUT_CH_STRIDE).wrapping_add(reg) as usize)
            as *mut c_void,
    );
}

unsafe fn img_i2s_out_ch_readl(i2s: *mut img_i2s_out, chan: u32, reg: u32) -> u32 {
    readl(
        ((*i2s).channel_base as *mut u8)
            .add(chan.wrapping_mul(IMG_I2S_OUT_CH_STRIDE).wrapping_add(reg) as usize)
            as *mut c_void,
    )
}

unsafe fn img_i2s_out_ch_disable(i2s: *mut img_i2s_out, chan: u32) {
    let mut reg: u32;

    reg = img_i2s_out_ch_readl(i2s, chan, IMG_I2S_OUT_CH_CTL);
    reg &= !IMG_I2S_OUT_CHAN_CTL_ME_MASK;
    img_i2s_out_ch_writel(i2s, chan, reg, IMG_I2S_OUT_CH_CTL);
}

unsafe fn img_i2s_out_ch_enable(i2s: *mut img_i2s_out, chan: u32) {
    let mut reg: u32;

    reg = img_i2s_out_ch_readl(i2s, chan, IMG_I2S_OUT_CH_CTL);
    reg |= IMG_I2S_OUT_CHAN_CTL_ME_MASK;
    img_i2s_out_ch_writel(i2s, chan, reg, IMG_I2S_OUT_CH_CTL);
}

unsafe fn img_i2s_out_disable(i2s: *mut img_i2s_out) {
    let mut reg: u32;

    reg = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL);
    reg &= !IMG_I2S_OUT_CTL_ME_MASK;
    img_i2s_out_writel(i2s, reg, IMG_I2S_OUT_CTL);
}

unsafe fn img_i2s_out_enable(i2s: *mut img_i2s_out) {
    let mut reg: u32;

    reg = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL);
    reg |= IMG_I2S_OUT_CTL_ME_MASK;
    img_i2s_out_writel(i2s, reg, IMG_I2S_OUT_CTL);
}

unsafe fn img_i2s_out_reset(i2s: *mut img_i2s_out) {
    let mut i: c_int;
    let mut core_ctl: u32;
    let chan_ctl: u32;

    core_ctl = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL)
        & !IMG_I2S_OUT_CTL_ME_MASK
        & !IMG_I2S_OUT_CTL_DATA_EN_MASK;

    if !(*i2s).force_clk_active {
        core_ctl &= !IMG_I2S_OUT_CTL_CLK_EN_MASK;
    }

    chan_ctl = img_i2s_out_ch_readl(i2s, 0, IMG_I2S_OUT_CH_CTL)
        & !IMG_I2S_OUT_CHAN_CTL_ME_MASK;

    reset_control_assert((*i2s).rst);
    reset_control_deassert((*i2s).rst);

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        img_i2s_out_ch_writel(i2s, i as u32, chan_ctl, IMG_I2S_OUT_CH_CTL);
        i += 1;
    }

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_out_ch_enable(i2s, i as u32);
        i += 1;
    }

    img_i2s_out_writel(i2s, core_ctl, IMG_I2S_OUT_CTL);
    img_i2s_out_enable(i2s);
}

unsafe extern "C" fn img_i2s_out_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_out;
    let mut reg: u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            reg = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL);
            if !(*i2s).force_clk_active {
                reg |= IMG_I2S_OUT_CTL_CLK_EN_MASK;
            }
            reg |= IMG_I2S_OUT_CTL_DATA_EN_MASK;
            img_i2s_out_writel(i2s, reg, IMG_I2S_OUT_CTL);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            img_i2s_out_reset(i2s);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn img_i2s_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_out;
    let channels: c_uint;
    let i2s_channels: c_uint;
    let mut pre_div_a: c_long;
    let mut pre_div_b: c_long;
    let mut diff_a: c_long;
    let mut diff_b: c_long;
    let rate: c_long;
    let clk_rate: c_long;
    let mut i: c_int;
    let mut reg: u32;
    let control_mask: u32;
    let mut control_set: u32 = 0;
    let format: snd_pcm_format_t;

    rate = params_rate(params) as c_long;
    format = params_format(params);
    channels = params_channels(params);
    i2s_channels = channels / 2;

    if format != SNDRV_PCM_FORMAT_S32_LE {
        return -EINVAL;
    }

    if channels < 2 || channels > ((*i2s).max_i2s_chan * 2) || channels % 2 != 0 {
        return -EINVAL;
    }

    pre_div_a = clk_round_rate((*i2s).clk_ref, rate * 256);
    if pre_div_a < 0 {
        return pre_div_a as c_int;
    }
    pre_div_b = clk_round_rate((*i2s).clk_ref, rate * 384);
    if pre_div_b < 0 {
        return pre_div_b as c_int;
    }

    diff_a = ((pre_div_a / 256) - rate).abs();
    diff_b = ((pre_div_b / 384) - rate).abs();

    /* If diffs are equal, use lower clock rate */
    if diff_a > diff_b {
        clk_set_rate((*i2s).clk_ref, pre_div_b);
    } else {
        clk_set_rate((*i2s).clk_ref, pre_div_a);
    }

    /*
     * Another driver (eg alsa machine driver) may have rejected the above
     * change. Get the current rate and set the register bit according to
     * the new minimum diff
     */
    clk_rate = clk_get_rate((*i2s).clk_ref);

    diff_a = ((clk_rate / 256) - rate).abs();
    diff_b = ((clk_rate / 384) - rate).abs();

    if diff_a > diff_b {
        control_set |= IMG_I2S_OUT_CTL_CLK_MASK;
    }

    control_set |= ((i2s_channels - 1) << IMG_I2S_OUT_CTL_ACTIVE_CHAN_SHIFT)
        & IMG_I2S_OUT_CTL_ACTIVE_CHAN_MASK;

    control_mask = IMG_I2S_OUT_CTL_CLK_MASK | IMG_I2S_OUT_CTL_ACTIVE_CHAN_MASK;

    img_i2s_out_disable(i2s);

    reg = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL);
    reg = (reg & !control_mask) | control_set;
    img_i2s_out_writel(i2s, reg, IMG_I2S_OUT_CTL);

    i = 0;
    while i < i2s_channels as c_int {
        img_i2s_out_ch_enable(i2s, i as u32);
        i += 1;
    }

    while i < (*i2s).max_i2s_chan as c_int {
        img_i2s_out_ch_disable(i2s, i as u32);
        i += 1;
    }

    img_i2s_out_enable(i2s);

    (*i2s).active_channels = i2s_channels;

    0
}

unsafe extern "C" fn img_i2s_out_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_out;
    let mut i: c_int;
    let ret: c_int;
    let force_clk_active: bool;
    let chan_control_mask: u32;
    let control_mask: u32;
    let mut chan_control_set: u32 = 0;
    let mut reg: u32;
    let mut control_set: u32 = 0;

    force_clk_active = (fmt & SND_SOC_DAIFMT_CLOCK_MASK) == SND_SOC_DAIFMT_CONT;

    if force_clk_active {
        control_set |= IMG_I2S_OUT_CTL_CLK_EN_MASK;
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        SND_SOC_DAIFMT_BP_FP => {
            control_set |= IMG_I2S_OUT_CTL_MASTER_MASK;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            control_set |= IMG_I2S_OUT_CTL_BCLK_POL_MASK;
        }
        SND_SOC_DAIFMT_NB_IF => {
            control_set |= IMG_I2S_OUT_CTL_BCLK_POL_MASK;
            control_set |= IMG_I2S_OUT_CTL_FRM_CLK_POL_MASK;
        }
        SND_SOC_DAIFMT_IB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            control_set |= IMG_I2S_OUT_CTL_FRM_CLK_POL_MASK;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            chan_control_set |= IMG_I2S_OUT_CHAN_CTL_CLKT_MASK;
        }
        SND_SOC_DAIFMT_LEFT_J => {}
        _ => return -EINVAL,
    }

    control_mask = IMG_I2S_OUT_CTL_CLK_EN_MASK
        | IMG_I2S_OUT_CTL_MASTER_MASK
        | IMG_I2S_OUT_CTL_BCLK_POL_MASK
        | IMG_I2S_OUT_CTL_FRM_CLK_POL_MASK;

    chan_control_mask = IMG_I2S_OUT_CHAN_CTL_CLKT_MASK;

    ret = pm_runtime_resume_and_get((*i2s).dev);
    if ret < 0 {
        return ret;
    }

    img_i2s_out_disable(i2s);

    reg = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL);
    reg = (reg & !control_mask) | control_set;
    img_i2s_out_writel(i2s, reg, IMG_I2S_OUT_CTL);

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_out_ch_disable(i2s, i as u32);
        i += 1;
    }

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = img_i2s_out_ch_readl(i2s, i as u32, IMG_I2S_OUT_CH_CTL);
        reg = (reg & !chan_control_mask) | chan_control_set;
        img_i2s_out_ch_writel(i2s, i as u32, reg, IMG_I2S_OUT_CH_CTL);
        i += 1;
    }

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_out_ch_enable(i2s, i as u32);
        i += 1;
    }

    img_i2s_out_enable(i2s);
    pm_runtime_put((*i2s).dev);

    (*i2s).force_clk_active = force_clk_active;

    0
}

unsafe extern "C" fn img_i2s_out_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_out;

    snd_soc_dai_init_dma_data(dai, &mut (*i2s).dma_data, ptr::null_mut());

    0
}

static IMG_I2S_OUT_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(img_i2s_out_dai_probe),
    trigger: Some(img_i2s_out_trigger),
    hw_params: Some(img_i2s_out_hw_params),
    set_fmt: Some(img_i2s_out_set_fmt),
};

static IMG_I2S_OUT_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: c"img-i2s-out".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn img_i2s_out_dma_prepare_slave_config(
    st: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    sc: *mut dma_slave_config,
) -> c_int {
    let i2s_channels: c_uint = params_channels(params) / 2;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(st);
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let ret: c_int;

    dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), st) as *mut snd_dmaengine_dai_dma_data;

    ret = snd_hwparams_to_dma_slave_config(st, params, sc);
    if ret != 0 {
        return ret;
    }

    (*sc).dst_addr = (*dma_data).addr;
    (*sc).dst_addr_width = (*dma_data).addr_width;
    (*sc).dst_maxburst = 4 * i2s_channels;

    0
}

static IMG_I2S_OUT_DMA_CONFIG: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(img_i2s_out_dma_prepare_slave_config),
};

unsafe extern "C" fn img_i2s_out_probe(pdev: *mut platform_device) -> c_int {
    let mut i2s: *mut img_i2s_out;
    let mut res: *mut resource = ptr::null_mut();
    let base: *mut c_void;
    let mut i: c_int;
    let mut ret: c_int;
    let max_i2s_chan_pow_2: c_uint;
    let mut reg: u32;
    let dev: *mut device = &mut (*pdev).dev;

    i2s = devm_kzalloc(&mut (*pdev).dev, size_of::<img_i2s_out>(), GFP_KERNEL) as *mut img_i2s_out;
    if i2s.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, i2s as *mut c_void);

    (*i2s).dev = &mut (*pdev).dev;

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*i2s).base = base;

    if of_property_read_u32(
        (*pdev).dev.of_node,
        c"img,i2s-channels".as_ptr(),
        &mut (*i2s).max_i2s_chan,
    ) != 0
    {
        dev_err(&mut (*pdev).dev, c"No img,i2s-channels property\n".as_ptr());
        return -EINVAL;
    }

    max_i2s_chan_pow_2 = 1u32 << get_count_order((*i2s).max_i2s_chan);

    (*i2s).channel_base = (base as *mut u8).add((max_i2s_chan_pow_2 * 0x20) as usize) as *mut c_void;

    (*i2s).rst = devm_reset_control_get_exclusive(&mut (*pdev).dev, c"rst".as_ptr());
    if IS_ERR((*i2s).rst as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*i2s).rst as *const c_void),
            c"No top level reset found\n".as_ptr(),
        );
    }

    (*i2s).clk_sys = devm_clk_get(&mut (*pdev).dev, c"sys".as_ptr());
    if IS_ERR((*i2s).clk_sys as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s).clk_sys as *const c_void),
            c"Failed to acquire clock 'sys'\n".as_ptr(),
        );
    }

    (*i2s).clk_ref = devm_clk_get(&mut (*pdev).dev, c"ref".as_ptr());
    if IS_ERR((*i2s).clk_ref as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s).clk_ref as *const c_void),
            c"Failed to acquire clock 'ref'\n".as_ptr(),
        );
    }

    (*i2s).suspend_ch_ctl = devm_kcalloc(
        dev,
        (*i2s).max_i2s_chan as usize,
        size_of::<u32>(),
        GFP_KERNEL,
    ) as *mut u32;
    if (*i2s).suspend_ch_ctl.is_null() {
        return -ENOMEM;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = img_i2s_out_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            goto_err_pm_disable(pdev, ret);
            return ret;
        }
    }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret < 0 {
        goto_err_suspend(pdev, ret);
        return ret;
    }

    reg = IMG_I2S_OUT_CTL_FRM_SIZE_MASK;
    img_i2s_out_writel(i2s, reg, IMG_I2S_OUT_CTL);

    reg = IMG_I2S_OUT_CHAN_CTL_JUST_MASK
        | IMG_I2S_OUT_CHAN_CTL_LT_MASK
        | IMG_I2S_OUT_CHAN_CTL_CH_MASK
        | (8 << IMG_I2S_OUT_CHAN_CTL_FMT_SHIFT);

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        img_i2s_out_ch_writel(i2s, i as u32, reg, IMG_I2S_OUT_CH_CTL);
        i += 1;
    }

    img_i2s_out_reset(i2s);
    pm_runtime_put(&mut (*pdev).dev);

    (*i2s).active_channels = 1;
    (*i2s).dma_data.addr = (*res).start + IMG_I2S_OUT_TX_FIFO as c_ulong;
    (*i2s).dma_data.addr_width = 4;
    (*i2s).dma_data.maxburst = 4;

    (*i2s).dai_driver.playback.channels_min = 2;
    (*i2s).dai_driver.playback.channels_max = (*i2s).max_i2s_chan * 2;
    (*i2s).dai_driver.playback.rates = SNDRV_PCM_RATE_8000_192000;
    (*i2s).dai_driver.playback.formats = SNDRV_PCM_FMTBIT_S32_LE;
    (*i2s).dai_driver.ops = &IMG_I2S_OUT_DAI_OPS;

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &IMG_I2S_OUT_COMPONENT,
        &mut (*i2s).dai_driver,
        1,
    );
    if ret != 0 {
        goto_err_suspend(pdev, ret);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, &IMG_I2S_OUT_DMA_CONFIG, 0);
    if ret != 0 {
        goto_err_suspend(pdev, ret);
        return ret;
    }

    0
}

unsafe fn goto_err_suspend(pdev: *mut platform_device, ret: c_int) -> c_int {
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        img_i2s_out_runtime_suspend(&mut (*pdev).dev);
    }
    goto_err_pm_disable(pdev, ret)
}

unsafe fn goto_err_pm_disable(pdev: *mut platform_device, ret: c_int) -> c_int {
    pm_runtime_disable(&mut (*pdev).dev);

    ret
}

unsafe extern "C" fn img_i2s_out_dev_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        img_i2s_out_runtime_suspend(&mut (*pdev).dev);
    }
}

unsafe extern "C" fn img_i2s_out_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_out;
    let mut i: c_int;
    let mut ret: c_int;
    let mut reg: u32;

    if pm_runtime_status_suspended(dev) {
        ret = img_i2s_out_runtime_resume(dev);
        if ret != 0 {
            return ret;
        }
    }

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = img_i2s_out_ch_readl(i2s, i as u32, IMG_I2S_OUT_CH_CTL);
        *(*i2s).suspend_ch_ctl.add(i as usize) = reg;
        i += 1;
    }

    (*i2s).suspend_ctl = img_i2s_out_readl(i2s, IMG_I2S_OUT_CTL);

    img_i2s_out_runtime_suspend(dev);

    0
}

unsafe extern "C" fn img_i2s_out_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_out;
    let mut i: c_int;
    let ret: c_int;
    let mut reg: u32;

    ret = img_i2s_out_runtime_resume(dev);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = *(*i2s).suspend_ch_ctl.add(i as usize);
        img_i2s_out_ch_writel(i2s, i as u32, reg, IMG_I2S_OUT_CH_CTL);
        i += 1;
    }

    img_i2s_out_writel(i2s, (*i2s).suspend_ctl, IMG_I2S_OUT_CTL);

    if pm_runtime_status_suspended(dev) {
        img_i2s_out_runtime_suspend(dev);
    }

    0
}

static IMG_I2S_OUT_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"img,i2s-out".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, img_i2s_out_of_match);

// Original C used RUNTIME_PM_OPS and SYSTEM_SLEEP_PM_OPS initializers.
static IMG_I2S_OUT_PM_OPS: dev_pm_ops = dev_pm_ops { _private: [] };

static mut IMG_I2S_OUT_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"img-i2s-out".as_ptr(),
        of_match_table: IMG_I2S_OUT_OF_MATCH.as_ptr(),
        pm: &IMG_I2S_OUT_PM_OPS,
    },
    probe: Some(img_i2s_out_probe),
    remove: Some(img_i2s_out_dev_remove),
};
// module_platform_driver(img_i2s_out_driver);

// MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
// MODULE_DESCRIPTION("IMG I2S Output Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
