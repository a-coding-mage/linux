// SPDX-License-Identifier: GPL-2.0-or-later
//
// Author: Kevin Wells <kevin.wells@nxp.com>
//
// Copyright (C) 2008 NXP Semiconductors
// Copyright 2023 Timesys Corporation <piotr.wojtaszczyk@timesys.com>

// Rust translation of the implementation source. C include dependencies:
// linux/init.h, linux/module.h, linux/interrupt.h, linux/device.h,
// linux/delay.h, linux/clk.h, linux/io.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/dmaengine_pcm.h, sound/initval.h, sound/soc.h,
// and "lpc3xxx-i2s.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;
type dma_addr_t = usize;

const I2S_PLAYBACK_FLAG: u32 = 0x1;
const I2S_CAPTURE_FLAG: u32 = 0x2;

const LPC3XXX_I2S_RATES: u32 = SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000;

const LPC3XXX_I2S_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
pub struct device {
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
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub maxburst: u32,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lpc3xxx_i2s_info {
    pub dev: *mut device,
    pub regs: *mut regmap,
    pub clk: *mut clk,
    pub clkrate: u32,
    pub freq: c_uint,
    pub streams_in_use: u32,
    pub lock: mutex,
    pub playback_dma_config: snd_dmaengine_dai_dma_data,
    pub capture_dma_config: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_inner,
}

extern "C" {
    static SNDRV_PCM_RATE_16000: u32;
    static SNDRV_PCM_RATE_22050: u32;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_FORMAT_S8: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static EBUSY: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static LPC3XXX_I2S_RESET: u32;
    static LPC3XXX_I2S_STOP: u32;
    static LPC3XXX_REG_I2S_TX_RATE: c_uint;
    static LPC3XXX_REG_I2S_RX_RATE: c_uint;
    static LPC3XXX_REG_I2S_DAO: c_uint;
    static LPC3XXX_REG_I2S_DAI: c_uint;
    static LPC3XXX_I2S_WW8: u32;
    static LPC3XXX_I2S_WW8_HP: u32;
    static LPC3XXX_I2S_WW16: u32;
    static LPC3XXX_I2S_WW16_HP: u32;
    static LPC3XXX_I2S_WW32: u32;
    static LPC3XXX_I2S_WW32_HP: u32;
    static LPC3XXX_I2S_MONO: u32;
    static LPC3XXX_REG_I2S_DMA1: c_uint;
    static LPC3XXX_I2S_DMA1_TX_EN: u32;
    static LPC3XXX_REG_I2S_DMA0: c_uint;
    static LPC3XXX_I2S_DMA0_RX_EN: u32;
    static LPC3XXX_REG_I2S_TX_FIFO: usize;
    static LPC3XXX_REG_I2S_RX_FIFO: usize;
}

extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn lpc3xxx_pcm_register(pdev: *mut platform_device) -> c_int;
}

fn LPC3XXX_I2S_WS_HP(x: u32) -> u32 {
    unsafe { lpc3xxx_i2s_ws_hp(x) }
}

fn LPC3XXX_I2S_DMA0_TX_DEPTH(x: u32) -> u32 {
    unsafe { lpc3xxx_i2s_dma0_tx_depth(x) }
}

fn LPC3XXX_I2S_DMA1_RX_DEPTH(x: u32) -> u32 {
    unsafe { lpc3xxx_i2s_dma1_rx_depth(x) }
}

extern "C" {
    fn lpc3xxx_i2s_ws_hp(x: u32) -> u32;
    fn lpc3xxx_i2s_dma0_tx_depth(x: u32) -> u32;
    fn lpc3xxx_i2s_dma1_rx_depth(x: u32) -> u32;
}

unsafe fn __lpc3xxx_find_clkdiv(
    clkx: *mut u32,
    clky: *mut u32,
    freq: c_int,
    xbytes: c_int,
    mut clkrate: u32,
) {
    let mut i2srate: u32;
    let mut idxx: u32;
    let mut idyy: u32;
    let mut diff: u32;
    let mut trate: u32;
    let baseclk: u32;

    /* Adjust rate for sample size (bits) and 2 channels and offset for
     * divider in clock output
     */
    i2srate = ((freq / 100) as u32)
        .wrapping_mul(2)
        .wrapping_mul((8 * xbytes) as u32);
    i2srate = i2srate << 1;
    clkrate = clkrate / 100;
    baseclk = clkrate;
    *clkx = 1;
    *clky = 1;

    /* Find the best divider */
    *clkx = 0;
    *clky = 0;
    diff = !0;
    idxx = 1;
    while idxx < 0xFF {
        idyy = 1;
        while idyy < 0xFF {
            trate = baseclk.wrapping_mul(idxx) / idyy;
            let delta = (trate as c_int).wrapping_sub(i2srate as c_int).abs() as u32;
            if delta < diff {
                diff = delta;
                *clkx = idxx;
                *clky = idyy;
            }
            idyy += 1;
        }
        idxx += 1;
    }
}

unsafe extern "C" fn lpc3xxx_i2s_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let i2s_info_p = snd_soc_dai_get_drvdata(cpu_dai) as *mut lpc3xxx_i2s_info;
    let dev = (*i2s_info_p).dev;
    let flag: u32;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*i2s_info_p).lock);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        flag = I2S_PLAYBACK_FLAG;
    } else {
        flag = I2S_CAPTURE_FLAG;
    }

    if (flag & (*i2s_info_p).streams_in_use) != 0 {
        dev_warn(dev, c"I2S channel is busy\n".as_ptr());
        ret = -EBUSY;
        mutex_unlock(&mut (*i2s_info_p).lock);
        return ret;
    }

    if (*i2s_info_p).streams_in_use == 0 {
        ret = clk_prepare_enable((*i2s_info_p).clk);
        if ret != 0 {
            dev_err(dev, c"Can't enable clock, err=%d\n".as_ptr(), ret);
            mutex_unlock(&mut (*i2s_info_p).lock);
            return ret;
        }
    }

    (*i2s_info_p).streams_in_use |= flag;
    mutex_unlock(&mut (*i2s_info_p).lock);
    0
}

unsafe extern "C" fn lpc3xxx_i2s_shutdown(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) {
    let i2s_info_p = snd_soc_dai_get_drvdata(cpu_dai) as *mut lpc3xxx_i2s_info;
    let regs = (*i2s_info_p).regs;
    let stop_bits: u32 = LPC3XXX_I2S_RESET | LPC3XXX_I2S_STOP;
    let flag: u32;

    mutex_lock(&mut (*i2s_info_p).lock);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        flag = I2S_PLAYBACK_FLAG;
        regmap_write(regs, LPC3XXX_REG_I2S_TX_RATE, 0);
        regmap_update_bits(regs, LPC3XXX_REG_I2S_DAO, stop_bits, stop_bits);
    } else {
        flag = I2S_CAPTURE_FLAG;
        regmap_write(regs, LPC3XXX_REG_I2S_RX_RATE, 0);
        regmap_update_bits(regs, LPC3XXX_REG_I2S_DAI, stop_bits, stop_bits);
    }
    (*i2s_info_p).streams_in_use &= !flag;

    if (*i2s_info_p).streams_in_use == 0 {
        clk_disable_unprepare((*i2s_info_p).clk);
    }

    mutex_unlock(&mut (*i2s_info_p).lock);
}

unsafe extern "C" fn lpc3xxx_i2s_set_dai_sysclk(
    cpu_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let i2s_info_p = snd_soc_dai_get_drvdata(cpu_dai) as *mut lpc3xxx_i2s_info;

    /* Will use in HW params later */
    (*i2s_info_p).freq = freq;

    0
}

unsafe extern "C" fn lpc3xxx_i2s_set_dai_fmt(
    cpu_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let i2s_info_p = snd_soc_dai_get_drvdata(cpu_dai) as *mut lpc3xxx_i2s_info;
    let dev = (*i2s_info_p).dev;

    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S {
        dev_warn(dev, c"unsupported bus format %d\n".as_ptr(), fmt);
        return -EINVAL;
    }

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_BP_FP {
        dev_warn(dev, c"unsupported clock provider %d\n".as_ptr(), fmt);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn lpc3xxx_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let i2s_info_p = snd_soc_dai_get_drvdata(cpu_dai) as *mut lpc3xxx_i2s_info;
    let dev = (*i2s_info_p).dev;
    let regs = (*i2s_info_p).regs;
    let xfersize: c_int;
    let mut tmp: u32;
    let mut clkx: u32 = 0;
    let mut clky: u32 = 0;

    tmp = LPC3XXX_I2S_RESET | LPC3XXX_I2S_STOP;
    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S8 => {
            tmp |= LPC3XXX_I2S_WW8 | LPC3XXX_I2S_WS_HP(LPC3XXX_I2S_WW8_HP);
            xfersize = 1;
        }
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            tmp |= LPC3XXX_I2S_WW16 | LPC3XXX_I2S_WS_HP(LPC3XXX_I2S_WW16_HP);
            xfersize = 2;
        }
        x if x == SNDRV_PCM_FORMAT_S32_LE => {
            tmp |= LPC3XXX_I2S_WW32 | LPC3XXX_I2S_WS_HP(LPC3XXX_I2S_WW32_HP);
            xfersize = 4;
        }
        _ => {
            dev_warn(
                dev,
                c"Unsupported audio data format %d\n".as_ptr(),
                params_format(params),
            );
            return -EINVAL;
        }
    }

    if params_channels(params) == 1 {
        tmp |= LPC3XXX_I2S_MONO;
    }

    __lpc3xxx_find_clkdiv(
        &mut clkx,
        &mut clky,
        (*i2s_info_p).freq as c_int,
        xfersize,
        (*i2s_info_p).clkrate,
    );

    dev_dbg(
        dev,
        c"Stream                : %s\n".as_ptr(),
        snd_pcm_direction_name((*substream).stream),
    );
    dev_dbg(dev, c"Desired clock rate    : %d\n".as_ptr(), (*i2s_info_p).freq);
    dev_dbg(dev, c"Base clock rate       : %d\n".as_ptr(), (*i2s_info_p).clkrate);
    dev_dbg(dev, c"Transfer size (bytes) : %d\n".as_ptr(), xfersize);
    dev_dbg(dev, c"Clock divider (x)     : %d\n".as_ptr(), clkx);
    dev_dbg(dev, c"Clock divider (y)     : %d\n".as_ptr(), clky);
    dev_dbg(dev, c"Channels              : %d\n".as_ptr(), params_channels(params));
    dev_dbg(dev, c"Data format           : %s\n".as_ptr(), c"I2S".as_ptr());

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_write(
            regs,
            LPC3XXX_REG_I2S_DMA1,
            LPC3XXX_I2S_DMA1_TX_EN | LPC3XXX_I2S_DMA0_TX_DEPTH(4),
        );
        regmap_write(regs, LPC3XXX_REG_I2S_TX_RATE, (clkx << 8) | clky);
        regmap_write(regs, LPC3XXX_REG_I2S_DAO, tmp);
    } else {
        regmap_write(
            regs,
            LPC3XXX_REG_I2S_DMA0,
            LPC3XXX_I2S_DMA0_RX_EN | LPC3XXX_I2S_DMA1_RX_DEPTH(4),
        );
        regmap_write(regs, LPC3XXX_REG_I2S_RX_RATE, (clkx << 8) | clky);
        regmap_write(regs, LPC3XXX_REG_I2S_DAI, tmp);
    }

    0
}

unsafe extern "C" fn lpc3xxx_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let i2s_info_p = snd_soc_dai_get_drvdata(cpu_dai) as *mut lpc3xxx_i2s_info;
    let regs = (*i2s_info_p).regs;
    let mut ret: c_int = 0;

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_STOP
            || x == SNDRV_PCM_TRIGGER_PAUSE_PUSH
            || x == SNDRV_PCM_TRIGGER_SUSPEND =>
        {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                regmap_update_bits(
                    regs,
                    LPC3XXX_REG_I2S_DAO,
                    LPC3XXX_I2S_STOP,
                    LPC3XXX_I2S_STOP,
                );
            } else {
                regmap_update_bits(
                    regs,
                    LPC3XXX_REG_I2S_DAI,
                    LPC3XXX_I2S_STOP,
                    LPC3XXX_I2S_STOP,
                );
            }
        }
        x if x == SNDRV_PCM_TRIGGER_START
            || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
            || x == SNDRV_PCM_TRIGGER_RESUME =>
        {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                regmap_update_bits(
                    regs,
                    LPC3XXX_REG_I2S_DAO,
                    LPC3XXX_I2S_RESET | LPC3XXX_I2S_STOP,
                    0,
                );
            } else {
                regmap_update_bits(
                    regs,
                    LPC3XXX_REG_I2S_DAI,
                    LPC3XXX_I2S_RESET | LPC3XXX_I2S_STOP,
                    0,
                );
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn lpc3xxx_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s_info_p = snd_soc_dai_get_drvdata(dai) as *mut lpc3xxx_i2s_info;

    snd_soc_dai_init_dma_data(
        dai,
        &mut (*i2s_info_p).playback_dma_config,
        &mut (*i2s_info_p).capture_dma_config,
    );
    0
}

static lpc3xxx_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(lpc3xxx_i2s_dai_probe),
    startup: Some(lpc3xxx_i2s_startup),
    shutdown: Some(lpc3xxx_i2s_shutdown),
    trigger: Some(lpc3xxx_i2s_trigger),
    hw_params: Some(lpc3xxx_i2s_hw_params),
    set_sysclk: Some(lpc3xxx_i2s_set_dai_sysclk),
    set_fmt: Some(lpc3xxx_i2s_set_dai_fmt),
};

static mut lpc3xxx_i2s_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: LPC3XXX_I2S_RATES,
        formats: LPC3XXX_I2S_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: LPC3XXX_I2S_RATES,
        formats: LPC3XXX_I2S_FORMATS,
    },
    ops: &lpc3xxx_i2s_dai_ops,
    symmetric_rate: 1,
    symmetric_channels: 1,
    symmetric_sample_bits: 1,
};

static lpc32xx_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"lpc32xx-i2s".as_ptr(),
    legacy_dai_naming: 1,
};

static lpc32xx_i2s_regconfig: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { LPC3XXX_REG_I2S_RX_RATE },
};

unsafe extern "C" fn lpc32xx_i2s_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let i2s_info_p: *mut lpc3xxx_i2s_info;
    let mut res: *mut resource = core::ptr::null_mut();
    let iomem: *mut c_void;
    let mut ret: c_int;

    i2s_info_p = devm_kzalloc(
        dev,
        core::mem::size_of::<lpc3xxx_i2s_info>(),
        GFP_KERNEL,
    ) as *mut lpc3xxx_i2s_info;
    if i2s_info_p.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, i2s_info_p as *mut c_void);
    (*i2s_info_p).dev = dev;

    iomem = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(iomem) {
        return dev_err_probe(dev, PTR_ERR(iomem), c"Can't map registers\n".as_ptr());
    }

    (*i2s_info_p).regs = devm_regmap_init_mmio(dev, iomem, &lpc32xx_i2s_regconfig);
    if IS_ERR((*i2s_info_p).regs as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s_info_p).regs as *const c_void),
            c"failed to init register map: %pe\n".as_ptr(),
            (*i2s_info_p).regs,
        );
    }

    (*i2s_info_p).clk = devm_clk_get(dev, core::ptr::null());
    if IS_ERR((*i2s_info_p).clk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s_info_p).clk as *const c_void),
            c"Can't get clock\n".as_ptr(),
        );
    }

    (*i2s_info_p).clkrate = clk_get_rate((*i2s_info_p).clk);
    if (*i2s_info_p).clkrate == 0 {
        return dev_err_probe(dev, -EINVAL as isize, c"Invalid returned clock rate\n".as_ptr());
    }

    mutex_init(&mut (*i2s_info_p).lock);

    ret = devm_snd_soc_register_component(
        dev,
        &lpc32xx_i2s_component,
        &mut lpc3xxx_i2s_dai_driver,
        1,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret as isize, c"Can't register cpu_dai component\n".as_ptr());
    }

    (*i2s_info_p).playback_dma_config.addr =
        ((*res).start + LPC3XXX_REG_I2S_TX_FIFO) as dma_addr_t;
    (*i2s_info_p).playback_dma_config.maxburst = 4;

    (*i2s_info_p).capture_dma_config.addr =
        ((*res).start + LPC3XXX_REG_I2S_RX_FIFO) as dma_addr_t;
    (*i2s_info_p).capture_dma_config.maxburst = 4;

    ret = lpc3xxx_pcm_register(pdev);
    if ret != 0 {
        return dev_err_probe(dev, ret as isize, c"Can't register pcm component\n".as_ptr());
    }

    0
}

static lpc32xx_i2s_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"nxp,lpc3220-i2s".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, lpc32xx_i2s_match);

static mut lpc32xx_i2s_driver: platform_driver = platform_driver {
    probe: Some(lpc32xx_i2s_probe),
    driver: platform_driver_inner {
        name: c"lpc3xxx-i2s".as_ptr(),
        of_match_table: lpc32xx_i2s_match.as_ptr(),
    },
};

// module_platform_driver(lpc32xx_i2s_driver);
// MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
// MODULE_AUTHOR("Piotr Wojtaszczyk <piotr.wojtaszczyk@timesys.com>");
// MODULE_DESCRIPTION("ASoC LPC3XXX I2S interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
