// SPDX-License-Identifier: GPL-2.0-only
/*
 * IMG I2S input controller driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

/* Dependencies from Linux kernel and ALSA SoC headers:
 * linux/clk.h, linux/init.h, linux/kernel.h, linux/module.h, linux/of.h,
 * linux/platform_device.h, linux/pm_runtime.h, linux/reset.h,
 * sound/core.h, sound/dmaengine_pcm.h, sound/initval.h, sound/pcm.h,
 * sound/pcm_params.h, sound/soc.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type u32 = u32;
type resource_size_t = c_ulong;
type snd_pcm_format_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 6;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0x0000_0fff;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S32_LE;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0010;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0030;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0000;

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const IMG_I2S_IN_RX_FIFO: u32 = 0x0;

const IMG_I2S_IN_CTL: u32 = 0x4;
const IMG_I2S_IN_CTL_ACTIVE_CHAN_MASK: u32 = 0xfffffffc;
const IMG_I2S_IN_CTL_ACTIVE_CH_SHIFT: u32 = 2;
const IMG_I2S_IN_CTL_16PACK_MASK: u32 = BIT(1);
const IMG_I2S_IN_CTL_ME_MASK: u32 = BIT(0);

const IMG_I2S_IN_CH_CTL: u32 = 0x4;
const IMG_I2S_IN_CH_CTL_CCDEL_MASK: u32 = 0x38000;
const IMG_I2S_IN_CH_CTL_CCDEL_SHIFT: u32 = 15;
const IMG_I2S_IN_CH_CTL_FEN_MASK: u32 = BIT(14);
const IMG_I2S_IN_CH_CTL_FMODE_MASK: u32 = BIT(13);
const IMG_I2S_IN_CH_CTL_16PACK_MASK: u32 = BIT(12);
const IMG_I2S_IN_CH_CTL_JUST_MASK: u32 = BIT(10);
const IMG_I2S_IN_CH_CTL_PACKH_MASK: u32 = BIT(9);
const IMG_I2S_IN_CH_CTL_CLK_TRANS_MASK: u32 = BIT(8);
const IMG_I2S_IN_CH_CTL_BLKP_MASK: u32 = BIT(7);
const IMG_I2S_IN_CH_CTL_FIFO_FLUSH_MASK: u32 = BIT(6);
const IMG_I2S_IN_CH_CTL_LRD_MASK: u32 = BIT(3);
const IMG_I2S_IN_CH_CTL_FW_MASK: u32 = BIT(2);
const IMG_I2S_IN_CH_CTL_SW_MASK: u32 = BIT(1);
const IMG_I2S_IN_CH_CTL_ME_MASK: u32 = BIT(0);

const IMG_I2S_IN_CH_STRIDE: u32 = 0x20;

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
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: resource_size_t,
    pub addr_width: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_capture {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub capture: snd_soc_dai_capture,
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
    pub src_addr: resource_size_t,
    pub src_addr_width: c_uint,
    pub src_maxburst: c_uint,
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
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct img_i2s_in {
    base: *mut c_void,
    clk_sys: *mut clk,
    dma_data: snd_dmaengine_dai_dma_data,
    dev: *mut device,
    max_i2s_chan: c_uint,
    channel_base: *mut c_void,
    active_channels: c_uint,
    dai_driver: snd_soc_dai_driver,
    suspend_ctl: u32,
    suspend_ch_ctl: *mut u32,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *const snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_substream_to_rtd(st: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, st: *mut snd_pcm_substream) -> *mut snd_dmaengine_dai_dma_data;
    fn snd_hwparams_to_dma_slave_config(st: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, sc: *mut dma_slave_config) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: c_uint, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn get_count_order(count: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn reset_control_assert(rst: *mut reset_control);
    fn reset_control_deassert(rst: *mut reset_control);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe fn readl(addr: *mut c_void) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

unsafe fn writel(val: u32, addr: *mut c_void) {
    core::ptr::write_volatile(addr as *mut u32, val);
}

unsafe extern "C" fn img_i2s_in_runtime_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_in;

    clk_disable_unprepare((*i2s).clk_sys);

    0
}

unsafe extern "C" fn img_i2s_in_runtime_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_in;
    let ret: c_int;

    ret = clk_prepare_enable((*i2s).clk_sys);
    if ret != 0 {
        dev_err(dev, c"Unable to enable sys clock\n".as_ptr());
        return ret;
    }

    0
}

unsafe fn img_i2s_in_writel(i2s: *mut img_i2s_in, val: u32, reg: u32) {
    writel(val, ((*i2s).base as *mut u8).add(reg as usize) as *mut c_void);
}

unsafe fn img_i2s_in_readl(i2s: *mut img_i2s_in, reg: u32) -> u32 {
    readl(((*i2s).base as *mut u8).add(reg as usize) as *mut c_void)
}

unsafe fn img_i2s_in_ch_writel(i2s: *mut img_i2s_in, chan: u32, val: u32, reg: u32) {
    writel(
        val,
        ((*i2s).channel_base as *mut u8)
            .add((chan.wrapping_mul(IMG_I2S_IN_CH_STRIDE).wrapping_add(reg)) as usize)
            as *mut c_void,
    );
}

unsafe fn img_i2s_in_ch_readl(i2s: *mut img_i2s_in, chan: u32, reg: u32) -> u32 {
    readl(
        ((*i2s).channel_base as *mut u8)
            .add((chan.wrapping_mul(IMG_I2S_IN_CH_STRIDE).wrapping_add(reg)) as usize)
            as *mut c_void,
    )
}

unsafe fn img_i2s_in_ch_disable(i2s: *mut img_i2s_in, chan: u32) {
    let mut reg: u32;

    reg = img_i2s_in_ch_readl(i2s, chan, IMG_I2S_IN_CH_CTL);
    reg &= !IMG_I2S_IN_CH_CTL_ME_MASK;
    img_i2s_in_ch_writel(i2s, chan, reg, IMG_I2S_IN_CH_CTL);
}

unsafe fn img_i2s_in_ch_enable(i2s: *mut img_i2s_in, chan: u32) {
    let mut reg: u32;

    reg = img_i2s_in_ch_readl(i2s, chan, IMG_I2S_IN_CH_CTL);
    reg |= IMG_I2S_IN_CH_CTL_ME_MASK;
    img_i2s_in_ch_writel(i2s, chan, reg, IMG_I2S_IN_CH_CTL);
}

unsafe fn img_i2s_in_disable(i2s: *mut img_i2s_in) {
    let mut reg: u32;

    reg = img_i2s_in_readl(i2s, IMG_I2S_IN_CTL);
    reg &= !IMG_I2S_IN_CTL_ME_MASK;
    img_i2s_in_writel(i2s, reg, IMG_I2S_IN_CTL);
}

unsafe fn img_i2s_in_enable(i2s: *mut img_i2s_in) {
    let mut reg: u32;

    reg = img_i2s_in_readl(i2s, IMG_I2S_IN_CTL);
    reg |= IMG_I2S_IN_CTL_ME_MASK;
    img_i2s_in_writel(i2s, reg, IMG_I2S_IN_CTL);
}

unsafe fn img_i2s_in_flush(i2s: *mut img_i2s_in) {
    let mut i: c_int;
    let mut reg: u32;

    i = 0;
    while i < (*i2s).active_channels as c_int {
        reg = img_i2s_in_ch_readl(i2s, i as u32, IMG_I2S_IN_CH_CTL);
        reg |= IMG_I2S_IN_CH_CTL_FIFO_FLUSH_MASK;
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        reg &= !IMG_I2S_IN_CH_CTL_FIFO_FLUSH_MASK;
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        i += 1;
    }
}

unsafe extern "C" fn img_i2s_in_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_in;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            img_i2s_in_enable(i2s);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            img_i2s_in_disable(i2s);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn img_i2s_in_check_rate(
    i2s: *mut img_i2s_in,
    sample_rate: c_uint,
    frame_size: c_uint,
    bclk_filter_enable: *mut c_uint,
    bclk_filter_value: *mut c_uint,
) -> c_int {
    let bclk_freq: c_uint;
    let cur_freq: c_uint;

    bclk_freq = sample_rate.wrapping_mul(frame_size);

    cur_freq = clk_get_rate((*i2s).clk_sys);

    if cur_freq >= bclk_freq.wrapping_mul(8) {
        *bclk_filter_enable = 1;
        *bclk_filter_value = 0;
    } else if cur_freq >= bclk_freq.wrapping_mul(7) {
        *bclk_filter_enable = 1;
        *bclk_filter_value = 1;
    } else if cur_freq >= bclk_freq.wrapping_mul(6) {
        *bclk_filter_enable = 0;
        *bclk_filter_value = 0;
    } else {
        dev_err(
            (*i2s).dev,
            c"Sys clock rate %u insufficient for sample rate %u\n".as_ptr(),
            cur_freq,
            sample_rate,
        );
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn img_i2s_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_in;
    let rate: c_uint;
    let channels: c_uint;
    let i2s_channels: c_uint;
    let frame_size: c_uint;
    let mut bclk_filter_enable: c_uint = 0;
    let mut bclk_filter_value: c_uint = 0;
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut reg: u32;
    let control_mask: u32;
    let chan_control_mask: u32;
    let mut control_set: u32 = 0;
    let mut chan_control_set: u32 = 0;
    let format: snd_pcm_format_t;

    rate = params_rate(params);
    format = params_format(params);
    channels = params_channels(params);
    i2s_channels = channels / 2;

    match format {
        SNDRV_PCM_FORMAT_S32_LE => {
            frame_size = 64;
            chan_control_set |= IMG_I2S_IN_CH_CTL_SW_MASK;
            chan_control_set |= IMG_I2S_IN_CH_CTL_FW_MASK;
            chan_control_set |= IMG_I2S_IN_CH_CTL_PACKH_MASK;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            frame_size = 64;
            chan_control_set |= IMG_I2S_IN_CH_CTL_SW_MASK;
            chan_control_set |= IMG_I2S_IN_CH_CTL_FW_MASK;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            frame_size = 32;
            control_set |= IMG_I2S_IN_CTL_16PACK_MASK;
            chan_control_set |= IMG_I2S_IN_CH_CTL_16PACK_MASK;
        }
        _ => return -EINVAL,
    }

    if channels < 2 || channels > (*i2s).max_i2s_chan.wrapping_mul(2) || (channels % 2) != 0 {
        return -EINVAL;
    }

    control_set |= (i2s_channels - 1) << IMG_I2S_IN_CTL_ACTIVE_CH_SHIFT;

    ret = img_i2s_in_check_rate(
        i2s,
        rate,
        frame_size,
        &mut bclk_filter_enable,
        &mut bclk_filter_value,
    );
    if ret < 0 {
        return ret;
    }

    if bclk_filter_enable != 0 {
        chan_control_set |= IMG_I2S_IN_CH_CTL_FEN_MASK;
    }

    if bclk_filter_value != 0 {
        chan_control_set |= IMG_I2S_IN_CH_CTL_FMODE_MASK;
    }

    control_mask = IMG_I2S_IN_CTL_16PACK_MASK | IMG_I2S_IN_CTL_ACTIVE_CHAN_MASK;

    chan_control_mask = IMG_I2S_IN_CH_CTL_16PACK_MASK
        | IMG_I2S_IN_CH_CTL_FEN_MASK
        | IMG_I2S_IN_CH_CTL_FMODE_MASK
        | IMG_I2S_IN_CH_CTL_SW_MASK
        | IMG_I2S_IN_CH_CTL_FW_MASK
        | IMG_I2S_IN_CH_CTL_PACKH_MASK;

    reg = img_i2s_in_readl(i2s, IMG_I2S_IN_CTL);
    reg = (reg & !control_mask) | control_set;
    img_i2s_in_writel(i2s, reg, IMG_I2S_IN_CTL);

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_in_ch_disable(i2s, i as u32);
        i += 1;
    }

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = img_i2s_in_ch_readl(i2s, i as u32, IMG_I2S_IN_CH_CTL);
        reg = (reg & !chan_control_mask) | chan_control_set;
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        i += 1;
    }

    (*i2s).active_channels = i2s_channels;

    img_i2s_in_flush(i2s);

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_in_ch_enable(i2s, i as u32);
        i += 1;
    }

    0
}

unsafe extern "C" fn img_i2s_in_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_in;
    let mut i: c_int;
    let ret: c_int;
    let chan_control_mask: u32;
    let mut lrd_set: u32 = 0;
    let mut blkp_set: u32 = 0;
    let mut chan_control_set: u32 = 0;
    let mut reg: u32;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            lrd_set |= IMG_I2S_IN_CH_CTL_LRD_MASK;
        }
        SND_SOC_DAIFMT_NB_IF => {}
        SND_SOC_DAIFMT_IB_NF => {
            lrd_set |= IMG_I2S_IN_CH_CTL_LRD_MASK;
            blkp_set |= IMG_I2S_IN_CH_CTL_BLKP_MASK;
        }
        SND_SOC_DAIFMT_IB_IF => {
            blkp_set |= IMG_I2S_IN_CH_CTL_BLKP_MASK;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            chan_control_set |= IMG_I2S_IN_CH_CTL_CLK_TRANS_MASK;
        }
        SND_SOC_DAIFMT_LEFT_J => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        _ => return -EINVAL,
    }

    chan_control_mask = IMG_I2S_IN_CH_CTL_CLK_TRANS_MASK;

    ret = pm_runtime_resume_and_get((*i2s).dev);
    if ret < 0 {
        return ret;
    }

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_in_ch_disable(i2s, i as u32);
        i += 1;
    }

    /*
     * BLKP and LRD must be set during separate register writes
     */
    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = img_i2s_in_ch_readl(i2s, i as u32, IMG_I2S_IN_CH_CTL);
        reg = (reg & !chan_control_mask) | chan_control_set;
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        reg = (reg & !IMG_I2S_IN_CH_CTL_BLKP_MASK) | blkp_set;
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        reg = (reg & !IMG_I2S_IN_CH_CTL_LRD_MASK) | lrd_set;
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        i += 1;
    }

    i = 0;
    while i < (*i2s).active_channels as c_int {
        img_i2s_in_ch_enable(i2s, i as u32);
        i += 1;
    }

    pm_runtime_put((*i2s).dev);

    0
}

unsafe extern "C" fn img_i2s_in_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut img_i2s_in;

    snd_soc_dai_init_dma_data(dai, null(), &mut (*i2s).dma_data);

    0
}

static img_i2s_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(img_i2s_in_dai_probe),
    trigger: Some(img_i2s_in_trigger),
    hw_params: Some(img_i2s_in_hw_params),
    set_fmt: Some(img_i2s_in_set_fmt),
};

static img_i2s_in_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"img-i2s-in".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn img_i2s_in_dma_prepare_slave_config(
    st: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    sc: *mut dma_slave_config,
) -> c_int {
    let i2s_channels: c_uint = params_channels(params) / 2;
    let rtd = snd_soc_substream_to_rtd(st);
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let ret: c_int;

    dma_data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), st);

    ret = snd_hwparams_to_dma_slave_config(st, params, sc);
    if ret != 0 {
        return ret;
    }

    (*sc).src_addr = (*dma_data).addr;
    (*sc).src_addr_width = (*dma_data).addr_width;
    (*sc).src_maxburst = 4 * i2s_channels;

    0
}

static img_i2s_in_dma_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(img_i2s_in_dma_prepare_slave_config),
};

unsafe extern "C" fn img_i2s_in_probe(pdev: *mut platform_device) -> c_int {
    let i2s: *mut img_i2s_in;
    let mut res: *mut resource = null_mut();
    let base: *mut c_void;
    let mut ret: c_int;
    let mut i: c_int;
    let rst: *mut reset_control;
    let max_i2s_chan_pow_2: c_uint;
    let dev: *mut device = &mut (*pdev).dev;

    i2s = devm_kzalloc(dev, size_of::<img_i2s_in>(), GFP_KERNEL) as *mut img_i2s_in;
    if i2s.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, i2s as *mut c_void);

    (*i2s).dev = dev;

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
        dev_err(dev, c"No img,i2s-channels property\n".as_ptr());
        return -EINVAL;
    }

    max_i2s_chan_pow_2 = 1u32 << get_count_order((*i2s).max_i2s_chan);

    (*i2s).channel_base =
        (base as *mut u8).add(max_i2s_chan_pow_2.wrapping_mul(0x20) as usize) as *mut c_void;

    (*i2s).clk_sys = devm_clk_get(dev, c"sys".as_ptr());
    if IS_ERR((*i2s).clk_sys as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s).clk_sys),
            c"Failed to acquire clock 'sys'\n".as_ptr(),
        );
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = img_i2s_in_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            goto_err_pm_disable(pdev, ret);
            return ret;
        }
    }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret < 0 {
        goto_err_suspend(pdev);
        return ret;
    }

    (*i2s).active_channels = 1;
    (*i2s).dma_data.addr = (*res).start + IMG_I2S_IN_RX_FIFO as resource_size_t;
    (*i2s).dma_data.addr_width = 4;

    (*i2s).dai_driver.capture.channels_min = 2;
    (*i2s).dai_driver.capture.channels_max = (*i2s).max_i2s_chan * 2;
    (*i2s).dai_driver.capture.rates = SNDRV_PCM_RATE_8000_192000;
    (*i2s).dai_driver.capture.formats =
        SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE;
    (*i2s).dai_driver.ops = &img_i2s_in_dai_ops;

    rst = devm_reset_control_get_exclusive(dev, c"rst".as_ptr());
    if IS_ERR(rst as *const c_void) {
        if PTR_ERR(rst) == -EPROBE_DEFER {
            ret = -EPROBE_DEFER;
            pm_runtime_put(&mut (*pdev).dev);
            goto_err_suspend(pdev);
            return ret;
        }

        dev_dbg(dev, c"No top level reset found\n".as_ptr());

        img_i2s_in_disable(i2s);

        i = 0;
        while i < (*i2s).max_i2s_chan as c_int {
            img_i2s_in_ch_disable(i2s, i as u32);
            i += 1;
        }
    } else {
        reset_control_assert(rst);
        reset_control_deassert(rst);
    }

    img_i2s_in_writel(i2s, 0, IMG_I2S_IN_CTL);

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        img_i2s_in_ch_writel(
            i2s,
            i as u32,
            (4 << IMG_I2S_IN_CH_CTL_CCDEL_SHIFT)
                | IMG_I2S_IN_CH_CTL_JUST_MASK
                | IMG_I2S_IN_CH_CTL_FW_MASK,
            IMG_I2S_IN_CH_CTL,
        );
        i += 1;
    }

    pm_runtime_put(&mut (*pdev).dev);

    (*i2s).suspend_ch_ctl = devm_kcalloc(
        dev,
        (*i2s).max_i2s_chan,
        size_of::<u32>(),
        GFP_KERNEL,
    ) as *mut u32;
    if (*i2s).suspend_ch_ctl.is_null() {
        ret = -ENOMEM;
        goto_err_suspend(pdev);
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &img_i2s_in_component, &mut (*i2s).dai_driver, 1);
    if ret != 0 {
        goto_err_suspend(pdev);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(dev, &img_i2s_in_dma_config, 0);
    if ret != 0 {
        goto_err_suspend(pdev);
        return ret;
    }

    0
}

unsafe fn goto_err_suspend(pdev: *mut platform_device) {
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        img_i2s_in_runtime_suspend(&mut (*pdev).dev);
    }
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe fn goto_err_pm_disable(pdev: *mut platform_device, _ret: c_int) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn img_i2s_in_dev_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        img_i2s_in_runtime_suspend(&mut (*pdev).dev);
    }
}

unsafe extern "C" fn img_i2s_in_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_in;
    let mut i: c_int;
    let ret: c_int;
    let reg: u32;

    if pm_runtime_status_suspended(dev) {
        ret = img_i2s_in_runtime_resume(dev);
        if ret != 0 {
            return ret;
        }
    }

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = img_i2s_in_ch_readl(i2s, i as u32, IMG_I2S_IN_CH_CTL);
        *(*i2s).suspend_ch_ctl.add(i as usize) = reg;
        i += 1;
    }

    (*i2s).suspend_ctl = img_i2s_in_readl(i2s, IMG_I2S_IN_CTL);

    img_i2s_in_runtime_suspend(dev);

    0
}

unsafe extern "C" fn img_i2s_in_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut img_i2s_in;
    let mut i: c_int;
    let ret: c_int;
    let reg: u32;

    ret = img_i2s_in_runtime_resume(dev);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < (*i2s).max_i2s_chan as c_int {
        reg = *(*i2s).suspend_ch_ctl.add(i as usize);
        img_i2s_in_ch_writel(i2s, i as u32, reg, IMG_I2S_IN_CH_CTL);
        i += 1;
    }

    img_i2s_in_writel(i2s, (*i2s).suspend_ctl, IMG_I2S_IN_CTL);

    if pm_runtime_status_suspended(dev) {
        img_i2s_in_runtime_suspend(dev);
    }

    0
}

static img_i2s_in_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"img,i2s-in".as_ptr(),
    },
    of_device_id { compatible: null() },
];
/* MODULE_DEVICE_TABLE(of, img_i2s_in_of_match); */

/* RUNTIME_PM_OPS(img_i2s_in_runtime_suspend, img_i2s_in_runtime_resume, NULL)
 * SYSTEM_SLEEP_PM_OPS(img_i2s_in_suspend, img_i2s_in_resume)
 */
static img_i2s_in_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut img_i2s_in_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"img-i2s-in".as_ptr(),
        of_match_table: img_i2s_in_of_match.as_ptr(),
        pm: &img_i2s_in_pm_ops,
    },
    probe: Some(img_i2s_in_probe),
    remove: Some(img_i2s_in_dev_remove),
};
/* module_platform_driver(img_i2s_in_driver); */

/* MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
 * MODULE_DESCRIPTION("IMG I2S Input Driver");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
