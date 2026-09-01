// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pxa2xx-i2s.c  --  ALSA Soc Audio Layer
 *
 * Copyright 2005 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood
 *         lrg@slimlogic.co.uk
 */

/*
 * C includes translated as external dependencies:
 * linux/init.h, linux/module.h, linux/device.h, linux/delay.h, linux/clk.h,
 * linux/platform_device.h, linux/io.h, sound/core.h, sound/pcm.h,
 * sound/initval.h, sound/soc.h, sound/dmaengine_pcm.h,
 * linux/platform_data/asoc-pxa.h, pxa2xx-lib.h, pxa2xx-i2s.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;
type dma_addr_t = u64;

/*
 * I2S Controller Register and Bit Definitions
 */
const SACR0: usize = 0x0000; /* Global Control Register */
const SACR1: usize = 0x0004; /* Serial Audio I 2 S/MSB-Justified Control Register */
const SASR0: usize = 0x000c; /* Serial Audio I 2 S/MSB-Justified Interface and FIFO Status Register */
const SAIMR: usize = 0x0014; /* Serial Audio Interrupt Mask Register */
const SAICR: usize = 0x0018; /* Serial Audio Interrupt Clear Register */
const SADIV: usize = 0x0060; /* Audio Clock Divider Register. */
const SADR: usize = 0x0080; /* Serial Audio Data Register (TX and RX FIFO access Register). */

const fn SACR0_RFTH(x: u32) -> u32 {
    x << 12 /* Rx FIFO Interrupt or DMA Trigger Threshold */
}
const fn SACR0_TFTH(x: u32) -> u32 {
    x << 8 /* Tx FIFO Interrupt or DMA Trigger Threshold */
}
const SACR0_STRF: u32 = 1 << 5; /* FIFO Select for EFWR Special Function */
const SACR0_EFWR: u32 = 1 << 4; /* Enable EFWR Function  */
const SACR0_RST: u32 = 1 << 3; /* FIFO, i2s Register Reset */
const SACR0_BCKD: u32 = 1 << 2; /* Bit Clock Direction */
const SACR0_ENB: u32 = 1 << 0; /* Enable I2S Link */
const SACR1_ENLBF: u32 = 1 << 5; /* Enable Loopback */
const SACR1_DRPL: u32 = 1 << 4; /* Disable Replaying Function */
const SACR1_DREC: u32 = 1 << 3; /* Disable Recording Function */
const SACR1_AMSL: u32 = 1 << 0; /* Specify Alternate Mode */

const SASR0_I2SOFF: u32 = 1 << 7; /* Controller Status */
const SASR0_ROR: u32 = 1 << 6; /* Rx FIFO Overrun */
const SASR0_TUR: u32 = 1 << 5; /* Tx FIFO Underrun */
const SASR0_RFS: u32 = 1 << 4; /* Rx FIFO Service Request */
const SASR0_TFS: u32 = 1 << 3; /* Tx FIFO Service Request */
const SASR0_BSY: u32 = 1 << 2; /* I2S Busy */
const SASR0_RNE: u32 = 1 << 1; /* Rx FIFO Not Empty */
const SASR0_TNF: u32 = 1 << 0; /* Tx FIFO Not Empty */

const SAICR_ROR: u32 = 1 << 6; /* Clear Rx FIFO Overrun Interrupt */
const SAICR_TUR: u32 = 1 << 5; /* Clear Tx FIFO Underrun Interrupt */

const SAIMR_ROR: u32 = 1 << 6; /* Enable Rx FIFO Overrun Condition Interrupt */
const SAIMR_TUR: u32 = 1 << 5; /* Enable Tx FIFO Underrun Condition Interrupt */
const SAIMR_RFS: u32 = 1 << 4; /* Enable Rx FIFO Service Interrupt */
const SAIMR_TFS: u32 = 1 << 3; /* Enable Tx FIFO Service Interrupt */

#[repr(C)]
struct pxa_i2s_port {
    sadiv: u32,
    sacr0: u32,
    sacr1: u32,
    saimr: u32,
    master: c_int,
    fmt: u32,
}

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
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
struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct resource {
    start: dma_addr_t,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: dma_addr_t,
    addr_width: c_uint,
    chan_name: *const c_char,
    maxburst: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    pcm_new: Option<unsafe extern "C" fn() -> c_int>,
    open: Option<unsafe extern "C" fn() -> c_int>,
    close: Option<unsafe extern "C" fn() -> c_int>,
    hw_params: Option<unsafe extern "C" fn() -> c_int>,
    prepare: Option<unsafe extern "C" fn() -> c_int>,
    trigger: Option<unsafe extern "C" fn() -> c_int>,
    pointer: Option<unsafe extern "C" fn() -> u64>,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
struct driver_private {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: driver_private,
}

const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0100;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0x0200;
const PXA2XX_I2S_SYSCLK: c_int = 0;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_11025: c_uint = 1 << 1;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 3;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 4;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 5;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 6;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;

unsafe extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(error: c_int) -> *mut clk;
    fn WARN_ON(condition: bool) -> bool;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        dma_data: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);

    fn pxa2xx_soc_pcm_new() -> c_int;
    fn pxa2xx_soc_pcm_open() -> c_int;
    fn pxa2xx_soc_pcm_close() -> c_int;
    fn pxa2xx_soc_pcm_hw_params() -> c_int;
    fn pxa2xx_soc_pcm_prepare() -> c_int;
    fn pxa2xx_soc_pcm_trigger() -> c_int;
    fn pxa2xx_soc_pcm_pointer() -> u64;
}

static mut pxa_i2s: pxa_i2s_port = pxa_i2s_port {
    sadiv: 0,
    sacr0: 0,
    sacr1: 0,
    saimr: 0,
    master: 0,
    fmt: 0,
};
static mut clk_i2s: *mut clk = core::ptr::null_mut();
static mut clk_ena: c_int = 0;
static mut i2s_reg_base: *mut c_void = core::ptr::null_mut();

static mut pxa2xx_i2s_pcm_stereo_out: snd_dmaengine_dai_dma_data =
    snd_dmaengine_dai_dma_data {
        addr: 0,
        addr_width: DMA_SLAVE_BUSWIDTH_4_BYTES,
        chan_name: c"tx".as_ptr(),
        maxburst: 32,
    };

static mut pxa2xx_i2s_pcm_stereo_in: snd_dmaengine_dai_dma_data =
    snd_dmaengine_dai_dma_data {
        addr: 0,
        addr_width: DMA_SLAVE_BUSWIDTH_4_BYTES,
        chan_name: c"rx".as_ptr(),
        maxburst: 32,
    };

unsafe fn reg(offset: usize) -> *mut c_void {
    (i2s_reg_base as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn pxa2xx_i2s_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    if IS_ERR(clk_i2s as *const c_void) {
        return PTR_ERR(clk_i2s as *const c_void);
    }

    if snd_soc_dai_active(cpu_dai) == 0 {
        writel(0, reg(SACR0));
    }

    0
}

/* wait for I2S controller to be ready */
unsafe fn pxa_i2s_wait() -> c_int {
    let mut i: c_int = 0;

    /* flush the Rx FIFO */
    while i < 16 {
        readl(reg(SADR));
        i += 1;
    }
    0
}

unsafe extern "C" fn pxa2xx_i2s_set_dai_fmt(
    _cpu_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            pxa_i2s.fmt = 0;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            pxa_i2s.fmt = SACR1_AMSL;
        }
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            pxa_i2s.master = 1;
        }
        SND_SOC_DAIFMT_BC_FP => {
            pxa_i2s.master = 0;
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn pxa2xx_i2s_set_dai_sysclk(
    _cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    _freq: c_uint,
    _dir: c_int,
) -> c_int {
    if clk_id != PXA2XX_I2S_SYSCLK {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn pxa2xx_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dma_data: *mut snd_dmaengine_dai_dma_data;

    if WARN_ON(IS_ERR(clk_i2s as *const c_void)) {
        return -EINVAL;
    }
    clk_prepare_enable(clk_i2s);
    clk_ena = 1;
    pxa_i2s_wait();

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dma_data = &raw mut pxa2xx_i2s_pcm_stereo_out;
    } else {
        dma_data = &raw mut pxa2xx_i2s_pcm_stereo_in;
    }

    snd_soc_dai_set_dma_data(dai, substream, dma_data);

    /* is port used by another stream */
    if !(SACR0 & SACR0_ENB != 0) {
        writel(0, reg(SACR0));
        if pxa_i2s.master != 0 {
            writel(readl(reg(SACR0)) | SACR0_BCKD, reg(SACR0));
        }

        writel(
            readl(reg(SACR0)) | (SACR0_RFTH(14) | SACR0_TFTH(1)),
            reg(SACR0),
        );
        writel(readl(reg(SACR1)) | pxa_i2s.fmt, reg(SACR1));
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(readl(reg(SAIMR)) | SAIMR_TFS, reg(SAIMR));
    } else {
        writel(readl(reg(SAIMR)) | SAIMR_RFS, reg(SAIMR));
    }

    match params_rate(params) {
        8000 => {
            writel(0x48, reg(SADIV));
        }
        11025 => {
            writel(0x34, reg(SADIV));
        }
        16000 => {
            writel(0x24, reg(SADIV));
        }
        22050 => {
            writel(0x1a, reg(SADIV));
        }
        44100 => {
            writel(0x0d, reg(SADIV));
        }
        48000 => {
            writel(0x0c, reg(SADIV));
        }
        96000 => {
            /* not in manual and possibly slightly inaccurate */
            writel(0x06, reg(SADIV));
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn pxa2xx_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                writel(readl(reg(SACR1)) & !SACR1_DRPL, reg(SACR1));
            } else {
                writel(readl(reg(SACR1)) & !SACR1_DREC, reg(SACR1));
            }
            writel(readl(reg(SACR0)) | SACR0_ENB, reg(SACR0));
        }
        SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        | SNDRV_PCM_TRIGGER_STOP
        | SNDRV_PCM_TRIGGER_SUSPEND
        | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn pxa2xx_i2s_shutdown(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(readl(reg(SACR1)) | SACR1_DRPL, reg(SACR1));
        writel(readl(reg(SAIMR)) & !SAIMR_TFS, reg(SAIMR));
    } else {
        writel(readl(reg(SACR1)) | SACR1_DREC, reg(SACR1));
        writel(readl(reg(SAIMR)) & !SAIMR_RFS, reg(SAIMR));
    }

    if (readl(reg(SACR1)) & (SACR1_DREC | SACR1_DRPL)) == (SACR1_DREC | SACR1_DRPL) {
        writel(readl(reg(SACR0)) & !SACR0_ENB, reg(SACR0));
        pxa_i2s_wait();
        if clk_ena != 0 {
            clk_disable_unprepare(clk_i2s);
            clk_ena = 0;
        }
    }
}

/* CONFIG_PM conditional code translated from #ifdef CONFIG_PM. */
#[cfg(CONFIG_PM)]
unsafe extern "C" fn pxa2xx_soc_pcm_suspend(_component: *mut snd_soc_component) -> c_int {
    /* store registers */
    pxa_i2s.sacr0 = readl(reg(SACR0));
    pxa_i2s.sacr1 = readl(reg(SACR1));
    pxa_i2s.saimr = readl(reg(SAIMR));
    pxa_i2s.sadiv = readl(reg(SADIV));

    /* deactivate link */
    writel(readl(reg(SACR0)) & !SACR0_ENB, reg(SACR0));
    pxa_i2s_wait();
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn pxa2xx_soc_pcm_resume(_component: *mut snd_soc_component) -> c_int {
    pxa_i2s_wait();

    writel(pxa_i2s.sacr0 & !SACR0_ENB, reg(SACR0));
    writel(pxa_i2s.sacr1, reg(SACR1));
    writel(pxa_i2s.saimr, reg(SAIMR));
    writel(pxa_i2s.sadiv, reg(SADIV));

    writel(pxa_i2s.sacr0, reg(SACR0));

    0
}

#[cfg(not(CONFIG_PM))]
const pxa2xx_soc_pcm_suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> = None;
#[cfg(not(CONFIG_PM))]
const pxa2xx_soc_pcm_resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> = None;

unsafe extern "C" fn pxa2xx_i2s_probe(dai: *mut snd_soc_dai) -> c_int {
    clk_i2s = clk_get((*dai).dev, c"I2SCLK".as_ptr());
    if IS_ERR(clk_i2s as *const c_void) {
        return PTR_ERR(clk_i2s as *const c_void);
    }

    /*
     * PXA Developer's Manual:
     * If SACR0[ENB] is toggled in the middle of a normal operation,
     * the SACR0[RST] bit must also be set and cleared to reset all
     * I2S controller registers.
     */
    writel(SACR0_RST, reg(SACR0));
    writel(0, reg(SACR0));
    /* Make sure RPL and REC are disabled */
    writel(SACR1_DRPL | SACR1_DREC, reg(SACR1));
    /* Along with FIFO servicing */
    writel(readl(reg(SAIMR)) & !(SAIMR_RFS | SAIMR_TFS), reg(SAIMR));

    snd_soc_dai_init_dma_data(
        dai,
        &raw mut pxa2xx_i2s_pcm_stereo_out,
        &raw mut pxa2xx_i2s_pcm_stereo_in,
    );

    0
}

unsafe extern "C" fn pxa2xx_i2s_remove(_dai: *mut snd_soc_dai) -> c_int {
    clk_put(clk_i2s);
    clk_i2s = ERR_PTR(-ENOENT);
    0
}

const PXA2XX_I2S_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000;

static pxa_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(pxa2xx_i2s_probe),
    remove: Some(pxa2xx_i2s_remove),
    startup: Some(pxa2xx_i2s_startup),
    shutdown: Some(pxa2xx_i2s_shutdown),
    trigger: Some(pxa2xx_i2s_trigger),
    hw_params: Some(pxa2xx_i2s_hw_params),
    set_fmt: Some(pxa2xx_i2s_set_dai_fmt),
    set_sysclk: Some(pxa2xx_i2s_set_dai_sysclk),
};

static mut pxa_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: PXA2XX_I2S_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: PXA2XX_I2S_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &pxa_i2s_dai_ops,
    symmetric_rate: 1,
};

#[cfg(CONFIG_PM)]
const PXA2XX_SOC_PCM_SUSPEND_FIELD: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> =
    Some(pxa2xx_soc_pcm_suspend);
#[cfg(CONFIG_PM)]
const PXA2XX_SOC_PCM_RESUME_FIELD: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> =
    Some(pxa2xx_soc_pcm_resume);
#[cfg(not(CONFIG_PM))]
const PXA2XX_SOC_PCM_SUSPEND_FIELD: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> =
    pxa2xx_soc_pcm_suspend;
#[cfg(not(CONFIG_PM))]
const PXA2XX_SOC_PCM_RESUME_FIELD: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> =
    pxa2xx_soc_pcm_resume;

static pxa_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"pxa-i2s".as_ptr(),
    pcm_new: Some(pxa2xx_soc_pcm_new),
    open: Some(pxa2xx_soc_pcm_open),
    close: Some(pxa2xx_soc_pcm_close),
    hw_params: Some(pxa2xx_soc_pcm_hw_params),
    prepare: Some(pxa2xx_soc_pcm_prepare),
    trigger: Some(pxa2xx_soc_pcm_trigger),
    pointer: Some(pxa2xx_soc_pcm_pointer),
    suspend: PXA2XX_SOC_PCM_SUSPEND_FIELD,
    resume: PXA2XX_SOC_PCM_RESUME_FIELD,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn pxa2xx_i2s_drv_probe(pdev: *mut platform_device) -> c_int {
    let mut res: *mut resource = core::ptr::null_mut();

    i2s_reg_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(i2s_reg_base as *const c_void) {
        return PTR_ERR(i2s_reg_base as *const c_void);
    }

    pxa2xx_i2s_pcm_stereo_out.addr = (*res).start + SADR as dma_addr_t;
    pxa2xx_i2s_pcm_stereo_in.addr = (*res).start + SADR as dma_addr_t;

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &pxa_i2s_component,
        &raw mut pxa_i2s_dai,
        1,
    )
}

static mut pxa2xx_i2s_driver: platform_driver = platform_driver {
    probe: Some(pxa2xx_i2s_drv_probe),

    driver: driver_private {
        name: c"pxa2xx-i2s".as_ptr(),
    },
};

unsafe extern "C" fn pxa2xx_i2s_init() -> c_int {
    clk_i2s = ERR_PTR(-ENOENT);
    platform_driver_register(&raw mut pxa2xx_i2s_driver)
}

unsafe extern "C" fn pxa2xx_i2s_exit() {
    platform_driver_unregister(&raw mut pxa2xx_i2s_driver);
}

/* module_init(pxa2xx_i2s_init); */
/* module_exit(pxa2xx_i2s_exit); */

/* Module information */
/* MODULE_AUTHOR("Liam Girdwood, lrg@slimlogic.co.uk"); */
/* MODULE_DESCRIPTION("pxa2xx I2S SoC Interface"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:pxa2xx-i2s"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
