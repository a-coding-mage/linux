// SPDX-License-Identifier: GPL-2.0
/*
 * jh7110_tdm.rs -- StarFive JH7110 TDM driver
 *
 * Copyright (C) 2023 StarFive Technology Co., Ltd.
 *
 * Author: Walker Chen <walker.chen@starfivetech.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u16 = u16;
type u32 = u32;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const TDM_PCMGBCR: u16 = 0x00;
const PCMGBCR_ENABLE: u32 = BIT(0);
const CLKPOL_BIT: u32 = 5;
const ELM_BIT: u32 = 3;
const SYNCM_BIT: u32 = 2;
const MS_BIT: u32 = 1;
const TDM_PCMTXCR: u16 = 0x04;
const PCMTXCR_TXEN: u32 = BIT(0);
const IFL_BIT: u32 = 11;
const WL_BIT: u32 = 8;
const SSCALE_BIT: u32 = 4;
const SL_BIT: u32 = 2;
const LRJ_BIT: u32 = 1;
const TDM_PCMRXCR: u16 = 0x08;
const PCMRXCR_RXEN: u32 = BIT(0);
const TDM_PCMDIV: u16 = 0x0c;

const JH7110_TDM_FIFO: u32 = 0x170c0000;
const JH7110_TDM_FIFO_DEPTH: u32 = 32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S32_LE;
const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_PAUSE: u32 = 0;
const SNDRV_PCM_INFO_RESUME: u32 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0;
const SND_SOC_TRIGGER_ORDER_LDC: c_int = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0;
const DMA_SLAVE_BUSWIDTH_2_BYTES: c_uint = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const SND_DMAENGINE_PCM_FLAG_COMPAT: c_uint = 0;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

#[repr(C)]
struct clk_bulk_data {
    id: *const c_char,
    clk: *mut clk,
}

#[repr(C)]
struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
struct snd_soc_dai_link {
    trigger_stop: c_int,
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: u32,
    addr_width: c_uint,
    fifo_size: u32,
    maxburst: u32,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: u32,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: u32,
    buffer_bytes_max: u32,
    period_bytes_min: u32,
    period_bytes_max: u32,
    periods_min: u32,
    periods_max: u32,
    fifo_size: u32,
}

#[repr(C)]
struct snd_dmaengine_pcm_config {
    pcm_hardware: *const snd_pcm_hardware,
    prepare_slave_config: Option<unsafe extern "C" fn()>,
    prealloc_buffer_size: u32,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    system_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    system_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    driver: driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_MASTER_SLAVE_MODE {
    TDM_AS_MASTER = 0,
    TDM_AS_SLAVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_CLKPOL {
    /* tx raising and rx falling */
    TDM_TX_RASING_RX_FALLING = 0,
    /* tx falling and rx raising */
    TDM_TX_FALLING_RX_RASING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_ELM {
    /* only work while SYNCM=0 */
    TDM_ELM_LATE = 0,
    TDM_ELM_EARLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_SYNCM {
    /* short frame sync */
    TDM_SYNCM_SHORT = 0,
    /* long frame sync */
    TDM_SYNCM_LONG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_IFL {
    /* FIFO to send or received : half-1/2, Quarter-1/4 */
    TDM_FIFO_HALF = 0,
    TDM_FIFO_QUARTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_WL {
    /* send or received word length */
    TDM_8BIT_WORD_LEN = 0,
    TDM_16BIT_WORD_LEN,
    TDM_20BIT_WORD_LEN,
    TDM_24BIT_WORD_LEN,
    TDM_32BIT_WORD_LEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_SL {
    /* send or received slot length */
    TDM_8BIT_SLOT_LEN = 0,
    TDM_16BIT_SLOT_LEN,
    TDM_32BIT_SLOT_LEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum TDM_LRJ {
    /* left-justify or right-justify */
    TDM_RIGHT_JUSTIFY = 0,
    TDM_LEFT_JUSTIFT,
}

#[repr(C)]
struct tdm_chan_cfg {
    ifl: TDM_IFL,
    wl: TDM_WL,
    sscale: u8,
    sl: TDM_SL,
    lrj: TDM_LRJ,
    enable: u8,
}

#[repr(C)]
struct jh7110_tdm_dev {
    tdm_base: *mut c_void,
    dev: *mut device,
    clks: [clk_bulk_data; 6],
    resets: *mut reset_control,

    clkpolity: TDM_CLKPOL,
    elm: TDM_ELM,
    syncm: TDM_SYNCM,
    ms_mode: TDM_MASTER_SLAVE_MODE,

    tx: tdm_chan_cfg,
    rx: tdm_chan_cfg,

    syncdiv: u16,
    samplerate: u32,
    pcmclk: u32,

    /* data related to DMA transfers b/w tdm and DMAC */
    play_dma_data: snd_dmaengine_dai_dma_data,
    capture_dma_data: snd_dmaengine_dai_dma_data,
    saved_pcmgbcr: u32,
    saved_pcmtxcr: u32,
    saved_pcmrxcr: u32,
    saved_pcmdiv: u32,
}

extern "C" {
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(val: u32, addr: *mut c_void);
    fn clk_bulk_disable_unprepare(num_clks: usize, clks: *mut clk_bulk_data);
    fn clk_bulk_prepare_enable(num_clks: usize, clks: *mut clk_bulk_data) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_dai_set_drvdata(dai: *mut snd_soc_dai, data: *mut c_void);
    fn devm_clk_bulk_get(dev: *mut device, num_clks: usize, clks: *mut clk_bulk_data) -> c_int;
    fn devm_reset_control_array_get_exclusive(dev: *mut device) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_disable(dev: *mut device);
    fn snd_dmaengine_pcm_prepare_slave_config();
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[inline]
unsafe fn jh7110_tdm_readl(tdm: *mut jh7110_tdm_dev, reg: u16) -> u32 {
    readl_relaxed((*tdm).tdm_base.byte_add(reg as usize))
}

#[inline]
unsafe fn jh7110_tdm_writel(tdm: *mut jh7110_tdm_dev, reg: u16, val: u32) {
    writel_relaxed(val, (*tdm).tdm_base.byte_add(reg as usize));
}

unsafe extern "C" fn jh7110_tdm_save_context(
    tdm: *mut jh7110_tdm_dev,
    substream: *mut snd_pcm_substream,
) {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*tdm).saved_pcmtxcr = jh7110_tdm_readl(tdm, TDM_PCMTXCR);
    } else {
        (*tdm).saved_pcmrxcr = jh7110_tdm_readl(tdm, TDM_PCMRXCR);
    }
}

unsafe extern "C" fn jh7110_tdm_start(
    tdm: *mut jh7110_tdm_dev,
    substream: *mut snd_pcm_substream,
) {
    let data: u32;

    data = jh7110_tdm_readl(tdm, TDM_PCMGBCR);
    jh7110_tdm_writel(tdm, TDM_PCMGBCR, data | PCMGBCR_ENABLE);

    /* restore context */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        jh7110_tdm_writel(tdm, TDM_PCMTXCR, (*tdm).saved_pcmtxcr | PCMTXCR_TXEN);
    } else {
        jh7110_tdm_writel(tdm, TDM_PCMRXCR, (*tdm).saved_pcmrxcr | PCMRXCR_RXEN);
    }
}

unsafe extern "C" fn jh7110_tdm_stop(
    tdm: *mut jh7110_tdm_dev,
    substream: *mut snd_pcm_substream,
) {
    let mut val: c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        val = jh7110_tdm_readl(tdm, TDM_PCMTXCR);
        val &= !PCMTXCR_TXEN;
        jh7110_tdm_writel(tdm, TDM_PCMTXCR, val);
    } else {
        val = jh7110_tdm_readl(tdm, TDM_PCMRXCR);
        val &= !PCMRXCR_RXEN;
        jh7110_tdm_writel(tdm, TDM_PCMRXCR, val);
    }
}

unsafe extern "C" fn jh7110_tdm_syncdiv(tdm: *mut jh7110_tdm_dev) -> c_int {
    let sl: u32;
    let sscale: u32;
    let syncdiv: u32;

    sl = ((*tdm).rx.sl as u32).max((*tdm).tx.sl as u32);
    sscale = ((*tdm).rx.sscale as u32).max((*tdm).tx.sscale as u32);
    syncdiv = (*tdm).pcmclk / (*tdm).samplerate - 1;

    if (syncdiv + 1) < (sl * sscale) {
        dev_err((*tdm).dev, c"Failed to set syncdiv!\n".as_ptr());
        return -EINVAL;
    }

    if (*tdm).syncm == TDM_SYNCM::TDM_SYNCM_LONG
        && ((*tdm).rx.sscale <= 1 || (*tdm).tx.sscale <= 1)
        && ((syncdiv + 1) <= sl)
    {
        dev_err(
            (*tdm).dev,
            c"Wrong syncdiv! It must be (syncdiv+1) > max[tx.sl, rx.sl]\n".as_ptr(),
        );
        return -EINVAL;
    }

    jh7110_tdm_writel(tdm, TDM_PCMDIV, syncdiv);
    0
}

unsafe extern "C" fn jh7110_tdm_config(
    tdm: *mut jh7110_tdm_dev,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let datarx: u32;
    let datatx: u32;
    let ret: c_int;

    ret = jh7110_tdm_syncdiv(tdm);
    if ret != 0 {
        return ret;
    }

    datarx = ((*tdm).rx.ifl as u32) << IFL_BIT
        | ((*tdm).rx.wl as u32) << WL_BIT
        | ((*tdm).rx.sscale as u32) << SSCALE_BIT
        | ((*tdm).rx.sl as u32) << SL_BIT
        | ((*tdm).rx.lrj as u32) << LRJ_BIT;

    datatx = ((*tdm).tx.ifl as u32) << IFL_BIT
        | ((*tdm).tx.wl as u32) << WL_BIT
        | ((*tdm).tx.sscale as u32) << SSCALE_BIT
        | ((*tdm).tx.sl as u32) << SL_BIT
        | ((*tdm).tx.lrj as u32) << LRJ_BIT;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        jh7110_tdm_writel(tdm, TDM_PCMTXCR, datatx);
    } else {
        jh7110_tdm_writel(tdm, TDM_PCMRXCR, datarx);
    }

    0
}

unsafe extern "C" fn jh7110_tdm_clk_disable(tdm: *mut jh7110_tdm_dev) {
    clk_bulk_disable_unprepare((*tdm).clks.len(), (*tdm).clks.as_mut_ptr());
}

unsafe extern "C" fn jh7110_tdm_clk_enable(tdm: *mut jh7110_tdm_dev) -> c_int {
    let mut ret: c_int;

    ret = clk_bulk_prepare_enable((*tdm).clks.len(), (*tdm).clks.as_mut_ptr());
    if ret != 0 {
        dev_err((*tdm).dev, c"Failed to enable tdm clocks\n".as_ptr());
        return ret;
    }

    ret = reset_control_deassert((*tdm).resets);
    if ret != 0 {
        dev_err((*tdm).dev, c"Failed to deassert tdm resets\n".as_ptr());
        clk_bulk_disable_unprepare((*tdm).clks.len(), (*tdm).clks.as_mut_ptr());
        return ret;
    }

    /* select tdm_ext clock as the clock source for tdm */
    ret = clk_set_parent((*tdm).clks[5].clk, (*tdm).clks[4].clk);
    if ret != 0 {
        dev_err(
            (*tdm).dev,
            c"Can't set extern clock source for clk_tdm\n".as_ptr(),
        );
        clk_bulk_disable_unprepare((*tdm).clks.len(), (*tdm).clks.as_mut_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn jh7110_tdm_runtime_suspend(dev: *mut device) -> c_int {
    let tdm: *mut jh7110_tdm_dev = dev_get_drvdata(dev) as *mut jh7110_tdm_dev;

    jh7110_tdm_clk_disable(tdm);
    0
}

unsafe extern "C" fn jh7110_tdm_runtime_resume(dev: *mut device) -> c_int {
    let tdm: *mut jh7110_tdm_dev = dev_get_drvdata(dev) as *mut jh7110_tdm_dev;

    jh7110_tdm_clk_enable(tdm)
}

unsafe extern "C" fn jh7110_tdm_system_suspend(dev: *mut device) -> c_int {
    let tdm: *mut jh7110_tdm_dev = dev_get_drvdata(dev) as *mut jh7110_tdm_dev;

    /* save context */
    (*tdm).saved_pcmgbcr = jh7110_tdm_readl(tdm, TDM_PCMGBCR);
    (*tdm).saved_pcmdiv = jh7110_tdm_readl(tdm, TDM_PCMDIV);

    pm_runtime_force_suspend(dev)
}

unsafe extern "C" fn jh7110_tdm_system_resume(dev: *mut device) -> c_int {
    let tdm: *mut jh7110_tdm_dev = dev_get_drvdata(dev) as *mut jh7110_tdm_dev;

    /* restore context */
    jh7110_tdm_writel(tdm, TDM_PCMGBCR, (*tdm).saved_pcmgbcr);
    jh7110_tdm_writel(tdm, TDM_PCMDIV, (*tdm).saved_pcmdiv);

    pm_runtime_force_resume(dev)
}

static jh7110_tdm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"jh7110-tdm".as_ptr(),
};

unsafe extern "C" fn jh7110_tdm_startup(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dai_link: *mut snd_soc_dai_link = (*rtd).dai_link;

    (*dai_link).trigger_stop = SND_SOC_TRIGGER_ORDER_LDC;

    0
}

unsafe extern "C" fn jh7110_tdm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let tdm: *mut jh7110_tdm_dev = snd_soc_dai_get_drvdata(dai) as *mut jh7110_tdm_dev;
    let chan_wl: c_int;
    let chan_sl: c_int;
    let chan_nr: c_int;
    let data_width: c_uint;
    let dma_bus_width: c_uint;
    let mut dma_data: *mut snd_dmaengine_dai_dma_data = ptr::null_mut();
    let ret: c_int;

    data_width = params_width(params);

    (*tdm).samplerate = params_rate(params);
    (*tdm).pcmclk = params_channels(params) as u32 * (*tdm).samplerate * data_width;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            chan_wl = TDM_WL::TDM_16BIT_WORD_LEN as c_int;
            chan_sl = TDM_SL::TDM_16BIT_SLOT_LEN as c_int;
            dma_bus_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        }

        SNDRV_PCM_FORMAT_S32_LE => {
            chan_wl = TDM_WL::TDM_32BIT_WORD_LEN as c_int;
            chan_sl = TDM_SL::TDM_32BIT_SLOT_LEN as c_int;
            dma_bus_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        }

        _ => {
            dev_err((*tdm).dev, c"tdm: unsupported PCM fmt".as_ptr());
            return -EINVAL;
        }
    }

    chan_nr = params_channels(params);
    match chan_nr {
        1 | 2 | 4 | 6 | 8 => {}
        _ => {
            dev_err((*tdm).dev, c"channel not supported\n".as_ptr());
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*tdm).tx.wl = core::mem::transmute::<c_int, TDM_WL>(chan_wl);
        (*tdm).tx.sl = core::mem::transmute::<c_int, TDM_SL>(chan_sl);
        (*tdm).tx.sscale = chan_nr as u8;
        (*tdm).play_dma_data.addr_width = dma_bus_width;
        dma_data = &mut (*tdm).play_dma_data;
    } else {
        (*tdm).rx.wl = core::mem::transmute::<c_int, TDM_WL>(chan_wl);
        (*tdm).rx.sl = core::mem::transmute::<c_int, TDM_SL>(chan_sl);
        (*tdm).rx.sscale = chan_nr as u8;
        (*tdm).capture_dma_data.addr_width = dma_bus_width;
        dma_data = &mut (*tdm).capture_dma_data;
    }

    snd_soc_dai_set_dma_data(dai, substream, dma_data);

    ret = jh7110_tdm_config(tdm, substream);
    if ret != 0 {
        return ret;
    }

    jh7110_tdm_save_context(tdm, substream);
    0
}

unsafe extern "C" fn jh7110_tdm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let tdm: *mut jh7110_tdm_dev = snd_soc_dai_get_drvdata(dai) as *mut jh7110_tdm_dev;
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            jh7110_tdm_start(tdm, substream);
        }

        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            jh7110_tdm_stop(tdm, substream);
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn jh7110_tdm_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let tdm: *mut jh7110_tdm_dev = snd_soc_dai_get_drvdata(cpu_dai) as *mut jh7110_tdm_dev;
    let gbcr: c_uint;

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            /* cpu is master */
            (*tdm).ms_mode = TDM_MASTER_SLAVE_MODE::TDM_AS_MASTER;
        }
        SND_SOC_DAIFMT_BC_FC => {
            /* codec is master */
            (*tdm).ms_mode = TDM_MASTER_SLAVE_MODE::TDM_AS_SLAVE;
        }
        SND_SOC_DAIFMT_BC_FP | SND_SOC_DAIFMT_BP_FC => {
            return -EINVAL;
        }
        _ => {
            dev_dbg((*tdm).dev, c"dwc : Invalid clock provider format\n".as_ptr());
            return -EINVAL;
        }
    }

    gbcr = ((*tdm).clkpolity as c_uint) << CLKPOL_BIT
        | ((*tdm).elm as c_uint) << ELM_BIT
        | ((*tdm).syncm as c_uint) << SYNCM_BIT
        | ((*tdm).ms_mode as c_uint) << MS_BIT;
    jh7110_tdm_writel(tdm, TDM_PCMGBCR, gbcr);

    0
}

unsafe extern "C" fn jh7110_tdm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let tdm: *mut jh7110_tdm_dev = snd_soc_dai_get_drvdata(dai) as *mut jh7110_tdm_dev;

    snd_soc_dai_init_dma_data(dai, &mut (*tdm).play_dma_data, &mut (*tdm).capture_dma_data);
    snd_soc_dai_set_drvdata(dai, tdm as *mut c_void);
    0
}

static jh7110_tdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(jh7110_tdm_dai_probe),
    startup: Some(jh7110_tdm_startup),
    hw_params: Some(jh7110_tdm_hw_params),
    trigger: Some(jh7110_tdm_trigger),
    set_fmt: Some(jh7110_tdm_set_dai_fmt),
};

const JH7110_TDM_RATES: u32 = SNDRV_PCM_RATE_8000_48000;

const JH7110_TDM_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut jh7110_tdm_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"sf_tdm".as_ptr(),
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: JH7110_TDM_RATES,
        formats: JH7110_TDM_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: JH7110_TDM_RATES,
        formats: JH7110_TDM_FORMATS,
    },
    ops: &jh7110_tdm_dai_ops,
    symmetric_rate: 1,
};

static jh7110_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    buffer_bytes_max: 192512,
    period_bytes_min: 4096,
    period_bytes_max: 32768,
    periods_min: 1,
    periods_max: 48,
    fifo_size: 16,
};

static jh7110_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &jh7110_pcm_hardware,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    prealloc_buffer_size: 192512,
};

unsafe extern "C" fn jh7110_tdm_init_params(tdm: *mut jh7110_tdm_dev) {
    (*tdm).clkpolity = TDM_CLKPOL::TDM_TX_RASING_RX_FALLING;
    (*tdm).elm = TDM_ELM::TDM_ELM_LATE;
    (*tdm).syncm = TDM_SYNCM::TDM_SYNCM_SHORT;

    (*tdm).rx.ifl = TDM_IFL::TDM_FIFO_HALF;
    (*tdm).tx.ifl = TDM_IFL::TDM_FIFO_HALF;
    (*tdm).rx.wl = TDM_WL::TDM_16BIT_WORD_LEN;
    (*tdm).tx.wl = TDM_WL::TDM_16BIT_WORD_LEN;
    (*tdm).rx.sscale = 2;
    (*tdm).tx.sscale = 2;
    (*tdm).rx.lrj = TDM_LRJ::TDM_LEFT_JUSTIFT;
    (*tdm).tx.lrj = TDM_LRJ::TDM_LEFT_JUSTIFT;

    (*tdm).play_dma_data.addr = JH7110_TDM_FIFO;
    (*tdm).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    (*tdm).play_dma_data.fifo_size = JH7110_TDM_FIFO_DEPTH / 2;
    (*tdm).play_dma_data.maxburst = 16;

    (*tdm).capture_dma_data.addr = JH7110_TDM_FIFO;
    (*tdm).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    (*tdm).capture_dma_data.fifo_size = JH7110_TDM_FIFO_DEPTH / 2;
    (*tdm).capture_dma_data.maxburst = 8;
}

unsafe extern "C" fn jh7110_tdm_clk_reset_get(
    pdev: *mut platform_device,
    tdm: *mut jh7110_tdm_dev,
) -> c_int {
    let mut ret: c_int;

    (*tdm).clks[0].id = c"mclk_inner".as_ptr();
    (*tdm).clks[1].id = c"tdm_ahb".as_ptr();
    (*tdm).clks[2].id = c"tdm_apb".as_ptr();
    (*tdm).clks[3].id = c"tdm_internal".as_ptr();
    (*tdm).clks[4].id = c"tdm_ext".as_ptr();
    (*tdm).clks[5].id = c"tdm".as_ptr();

    ret = devm_clk_bulk_get(&mut (*pdev).dev, (*tdm).clks.len(), (*tdm).clks.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    (*tdm).resets = devm_reset_control_array_get_exclusive(&mut (*pdev).dev);
    if IS_ERR((*tdm).resets as *const c_void) {
        dev_err(&mut (*pdev).dev, c"Failed to get tdm resets\n".as_ptr());
        return PTR_ERR((*tdm).resets as *const c_void);
    }

    0
}

unsafe extern "C" fn jh7110_tdm_probe(pdev: *mut platform_device) -> c_int {
    let tdm: *mut jh7110_tdm_dev;
    let mut ret: c_int;

    tdm = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<jh7110_tdm_dev>(),
        GFP_KERNEL,
    ) as *mut jh7110_tdm_dev;
    if tdm.is_null() {
        return -ENOMEM;
    }

    (*tdm).tdm_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*tdm).tdm_base) {
        return PTR_ERR((*tdm).tdm_base);
    }

    (*tdm).dev = &mut (*pdev).dev;

    ret = jh7110_tdm_clk_reset_get(pdev, tdm);
    if ret != 0 {
        return ret;
    }

    jh7110_tdm_init_params(tdm);

    dev_set_drvdata(&mut (*pdev).dev, tdm as *mut c_void);
    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &jh7110_tdm_component,
        &raw mut jh7110_tdm_dai,
        1,
    );
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(
        &mut (*pdev).dev,
        &jh7110_dmaengine_pcm_config,
        SND_DMAENGINE_PCM_FLAG_COMPAT,
    );
    if ret != 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = jh7110_tdm_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            pm_runtime_disable(&mut (*pdev).dev);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn jh7110_tdm_dev_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

static jh7110_tdm_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"starfive,jh7110-tdm".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, jh7110_tdm_of_match); */

static jh7110_tdm_pm_ops: dev_pm_ops = dev_pm_ops {
    /* RUNTIME_PM_OPS(jh7110_tdm_runtime_suspend, jh7110_tdm_runtime_resume, NULL) */
    runtime_suspend: Some(jh7110_tdm_runtime_suspend),
    runtime_resume: Some(jh7110_tdm_runtime_resume),
    /* SYSTEM_SLEEP_PM_OPS(jh7110_tdm_system_suspend, jh7110_tdm_system_resume) */
    system_suspend: Some(jh7110_tdm_system_suspend),
    system_resume: Some(jh7110_tdm_system_resume),
};

static mut jh7110_tdm_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"jh7110-tdm".as_ptr(),
        of_match_table: jh7110_tdm_of_match.as_ptr(),
        pm: &jh7110_tdm_pm_ops,
    },
    probe: Some(jh7110_tdm_probe),
    remove: Some(jh7110_tdm_dev_remove),
};

/* module_platform_driver(jh7110_tdm_driver); */

/* MODULE_DESCRIPTION("StarFive JH7110 TDM ASoC Driver"); */
/* MODULE_AUTHOR("Walker Chen <walker.chen@starfivetech.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
