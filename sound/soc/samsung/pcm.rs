// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Audio Layer - S3C PCM-Controller driver
//
// Copyright (c) 2009 Samsung Electronics Co. Ltd
// Author: Jaswinder Singh <jassisinghbrar@gmail.com>
// based upon I2S drivers by Ben Dooks.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u32 = u32;
type dma_addr_t = c_ulong;
type dma_filter_fn = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id: c_int,
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
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: c_uint,
    pub filter_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub symmetric_rate: c_uint,
    pub ops: *const snd_soc_dai_ops,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct s3c_audio_pdata {
    pub cfg_gpio: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub dma_capture: *mut c_void,
    pub dma_playback: *mut c_void,
    pub dma_filter: dma_filter_fn,
}

extern "C" {
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_CLOCK_MASK: c_uint;
    static SND_SOC_DAIFMT_CONT: c_uint;
    static SND_SOC_DAIFMT_GATED: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static S3C_PCM_SCLK_PER_FS: c_int;
    static S3C_PCM_CLKSRC_PCLK: c_int;
    static S3C_PCM_CLKSRC_MUX: c_int;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;

    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn samsung_asoc_dma_platform_register(
        dev: *mut device,
        filter: dma_filter_fn,
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: *mut c_void,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn __platform_driver_register(driver: *mut platform_driver, owner: *mut c_void) -> c_int;
}

/*Register Offsets */
const S3C_PCM_CTL: usize = 0x00;
const S3C_PCM_CLKCTL: usize = 0x04;
const S3C_PCM_TXFIFO: dma_addr_t = 0x08;
const S3C_PCM_RXFIFO: dma_addr_t = 0x0C;
const S3C_PCM_IRQCTL: usize = 0x10;
const S3C_PCM_IRQSTAT: usize = 0x14;
const S3C_PCM_FIFOSTAT: usize = 0x18;
const S3C_PCM_CLRINT: usize = 0x20;

/* PCM_CTL Bit-Fields */
const S3C_PCM_CTL_TXDIPSTICK_MASK: u32 = 0x3f;
const S3C_PCM_CTL_TXDIPSTICK_SHIFT: u32 = 13;
const S3C_PCM_CTL_RXDIPSTICK_MASK: u32 = 0x3f;
const S3C_PCM_CTL_RXDIPSTICK_SHIFT: u32 = 7;
const S3C_PCM_CTL_TXDMA_EN: u32 = 0x1 << 6;
const S3C_PCM_CTL_RXDMA_EN: u32 = 0x1 << 5;
const S3C_PCM_CTL_TXMSB_AFTER_FSYNC: u32 = 0x1 << 4;
const S3C_PCM_CTL_RXMSB_AFTER_FSYNC: u32 = 0x1 << 3;
const S3C_PCM_CTL_TXFIFO_EN: u32 = 0x1 << 2;
const S3C_PCM_CTL_RXFIFO_EN: u32 = 0x1 << 1;
const S3C_PCM_CTL_ENABLE: u32 = 0x1 << 0;

/* PCM_CLKCTL Bit-Fields */
const S3C_PCM_CLKCTL_SERCLK_EN: u32 = 0x1 << 19;
const S3C_PCM_CLKCTL_SERCLKSEL_PCLK: u32 = 0x1 << 18;
const S3C_PCM_CLKCTL_SCLKDIV_MASK: u32 = 0x1ff;
const S3C_PCM_CLKCTL_SYNCDIV_MASK: u32 = 0x1ff;
const S3C_PCM_CLKCTL_SCLKDIV_SHIFT: u32 = 9;
const S3C_PCM_CLKCTL_SYNCDIV_SHIFT: u32 = 0;

/* PCM_TXFIFO Bit-Fields */
const S3C_PCM_TXFIFO_DVALID: u32 = 0x1 << 16;
const S3C_PCM_TXFIFO_DATA_MSK: u32 = 0xffff << 0;

/* PCM_RXFIFO Bit-Fields */
const S3C_PCM_RXFIFO_DVALID: u32 = 0x1 << 16;
const S3C_PCM_RXFIFO_DATA_MSK: u32 = 0xffff << 0;

/* PCM_IRQCTL Bit-Fields */
const S3C_PCM_IRQCTL_IRQEN: u32 = 0x1 << 14;
const S3C_PCM_IRQCTL_WRDEN: u32 = 0x1 << 12;
const S3C_PCM_IRQCTL_TXEMPTYEN: u32 = 0x1 << 11;
const S3C_PCM_IRQCTL_TXALMSTEMPTYEN: u32 = 0x1 << 10;
const S3C_PCM_IRQCTL_TXFULLEN: u32 = 0x1 << 9;
const S3C_PCM_IRQCTL_TXALMSTFULLEN: u32 = 0x1 << 8;
const S3C_PCM_IRQCTL_TXSTARVEN: u32 = 0x1 << 7;
const S3C_PCM_IRQCTL_TXERROVRFLEN: u32 = 0x1 << 6;
const S3C_PCM_IRQCTL_RXEMPTEN: u32 = 0x1 << 5;
const S3C_PCM_IRQCTL_RXALMSTEMPTEN: u32 = 0x1 << 4;
const S3C_PCM_IRQCTL_RXFULLEN: u32 = 0x1 << 3;
const S3C_PCM_IRQCTL_RXALMSTFULLEN: u32 = 0x1 << 2;
const S3C_PCM_IRQCTL_RXSTARVEN: u32 = 0x1 << 1;
const S3C_PCM_IRQCTL_RXERROVRFLEN: u32 = 0x1 << 0;

/* PCM_IRQSTAT Bit-Fields */
const S3C_PCM_IRQSTAT_IRQPND: u32 = 0x1 << 13;
const S3C_PCM_IRQSTAT_WRD_XFER: u32 = 0x1 << 12;
const S3C_PCM_IRQSTAT_TXEMPTY: u32 = 0x1 << 11;
const S3C_PCM_IRQSTAT_TXALMSTEMPTY: u32 = 0x1 << 10;
const S3C_PCM_IRQSTAT_TXFULL: u32 = 0x1 << 9;
const S3C_PCM_IRQSTAT_TXALMSTFULL: u32 = 0x1 << 8;
const S3C_PCM_IRQSTAT_TXSTARV: u32 = 0x1 << 7;
const S3C_PCM_IRQSTAT_TXERROVRFL: u32 = 0x1 << 6;
const S3C_PCM_IRQSTAT_RXEMPT: u32 = 0x1 << 5;
const S3C_PCM_IRQSTAT_RXALMSTEMPT: u32 = 0x1 << 4;
const S3C_PCM_IRQSTAT_RXFULL: u32 = 0x1 << 3;
const S3C_PCM_IRQSTAT_RXALMSTFULL: u32 = 0x1 << 2;
const S3C_PCM_IRQSTAT_RXSTARV: u32 = 0x1 << 1;
const S3C_PCM_IRQSTAT_RXERROVRFL: u32 = 0x1 << 0;

/* PCM_FIFOSTAT Bit-Fields */
const S3C_PCM_FIFOSTAT_TXCNT_MSK: u32 = 0x3f << 14;
const S3C_PCM_FIFOSTAT_TXFIFOEMPTY: u32 = 0x1 << 13;
const S3C_PCM_FIFOSTAT_TXFIFOALMSTEMPTY: u32 = 0x1 << 12;
const S3C_PCM_FIFOSTAT_TXFIFOFULL: u32 = 0x1 << 11;
const S3C_PCM_FIFOSTAT_TXFIFOALMSTFULL: u32 = 0x1 << 10;
const S3C_PCM_FIFOSTAT_RXCNT_MSK: u32 = 0x3f << 4;
const S3C_PCM_FIFOSTAT_RXFIFOEMPTY: u32 = 0x1 << 3;
const S3C_PCM_FIFOSTAT_RXFIFOALMSTEMPTY: u32 = 0x1 << 2;
const S3C_PCM_FIFOSTAT_RXFIFOFULL: u32 = 0x1 << 1;
const S3C_PCM_FIFOSTAT_RXFIFOALMSTFULL: u32 = 0x1 << 0;

/**
 * struct s3c_pcm_info - S3C PCM Controller information
 * @lock: Spin lock
 * @dev: The parent device passed to use from the probe.
 * @regs: The pointer to the device register block.
 * @sclk_per_fs: number of sclk per frame sync
 * @idleclk: Whether to keep PCMSCLK enabled even when idle (no active xfer)
 * @pclk: the PCLK_PCM (pcm) clock pointer
 * @cclk: the SCLK_AUDIO (audio-bus) clock pointer
 * @dma_playback: DMA information for playback channel.
 * @dma_capture: DMA information for capture channel.
 */
#[repr(C)]
pub struct s3c_pcm_info {
    pub lock: spinlock_t,
    pub dev: *mut device,
    pub regs: *mut c_void,

    pub sclk_per_fs: c_uint,

    /* Whether to keep PCMSCLK enabled even when idle(no active xfer) */
    pub idleclk: c_uint,

    pub pclk: *mut clk,
    pub cclk: *mut clk,

    pub dma_playback: *mut snd_dmaengine_dai_dma_data,
    pub dma_capture: *mut snd_dmaengine_dai_dma_data,
}

const fn dma_data(addr_width: c_uint) -> snd_dmaengine_dai_dma_data {
    snd_dmaengine_dai_dma_data {
        addr: 0,
        addr_width,
        filter_data: ptr::null_mut(),
    }
}

static mut S3C_PCM_STEREO_OUT: [snd_dmaengine_dai_dma_data; 2] = [dma_data(4), dma_data(4)];
static mut S3C_PCM_STEREO_IN: [snd_dmaengine_dai_dma_data; 2] = [dma_data(4), dma_data(4)];

const fn s3c_pcm_info_zero() -> s3c_pcm_info {
    s3c_pcm_info {
        lock: spinlock_t { _private: [] },
        dev: ptr::null_mut(),
        regs: ptr::null_mut(),
        sclk_per_fs: 0,
        idleclk: 0,
        pclk: ptr::null_mut(),
        cclk: ptr::null_mut(),
        dma_playback: ptr::null_mut(),
        dma_capture: ptr::null_mut(),
    }
}

static mut S3C_PCM: [s3c_pcm_info; 2] = [s3c_pcm_info_zero(), s3c_pcm_info_zero()];

unsafe fn reg(regs: *mut c_void, offset: usize) -> *mut c_void {
    (regs as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn s3c_pcm_snd_txctrl(pcm: *mut s3c_pcm_info, on: c_int) {
    let regs = (*pcm).regs;
    let mut clkctl: u32;
    let mut ctl: u32;

    clkctl = readl(reg(regs, S3C_PCM_CLKCTL));
    ctl = readl(reg(regs, S3C_PCM_CTL));
    ctl &= !(S3C_PCM_CTL_TXDIPSTICK_MASK << S3C_PCM_CTL_TXDIPSTICK_SHIFT);

    if on != 0 {
        ctl |= S3C_PCM_CTL_TXDMA_EN;
        ctl |= S3C_PCM_CTL_TXFIFO_EN;
        ctl |= S3C_PCM_CTL_ENABLE;
        ctl |= 0x4 << S3C_PCM_CTL_TXDIPSTICK_SHIFT;
        clkctl |= S3C_PCM_CLKCTL_SERCLK_EN;
    } else {
        ctl &= !S3C_PCM_CTL_TXDMA_EN;
        ctl &= !S3C_PCM_CTL_TXFIFO_EN;

        if (ctl & S3C_PCM_CTL_RXFIFO_EN) == 0 {
            ctl &= !S3C_PCM_CTL_ENABLE;
            if (*pcm).idleclk == 0 {
                clkctl |= S3C_PCM_CLKCTL_SERCLK_EN;
            }
        }
    }

    writel(clkctl, reg(regs, S3C_PCM_CLKCTL));
    writel(ctl, reg(regs, S3C_PCM_CTL));
}

unsafe extern "C" fn s3c_pcm_snd_rxctrl(pcm: *mut s3c_pcm_info, on: c_int) {
    let regs = (*pcm).regs;
    let mut ctl: u32;
    let mut clkctl: u32;

    ctl = readl(reg(regs, S3C_PCM_CTL));
    clkctl = readl(reg(regs, S3C_PCM_CLKCTL));
    ctl &= !(S3C_PCM_CTL_RXDIPSTICK_MASK << S3C_PCM_CTL_RXDIPSTICK_SHIFT);

    if on != 0 {
        ctl |= S3C_PCM_CTL_RXDMA_EN;
        ctl |= S3C_PCM_CTL_RXFIFO_EN;
        ctl |= S3C_PCM_CTL_ENABLE;
        ctl |= 0x20 << S3C_PCM_CTL_RXDIPSTICK_SHIFT;
        clkctl |= S3C_PCM_CLKCTL_SERCLK_EN;
    } else {
        ctl &= !S3C_PCM_CTL_RXDMA_EN;
        ctl &= !S3C_PCM_CTL_RXFIFO_EN;

        if (ctl & S3C_PCM_CTL_TXFIFO_EN) == 0 {
            ctl &= !S3C_PCM_CTL_ENABLE;
            if (*pcm).idleclk == 0 {
                clkctl |= S3C_PCM_CLKCTL_SERCLK_EN;
            }
        }
    }

    writel(clkctl, reg(regs, S3C_PCM_CLKCTL));
    writel(ctl, reg(regs, S3C_PCM_CTL));
}

unsafe extern "C" fn s3c_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let pcm = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut s3c_pcm_info;

    dev_dbg((*pcm).dev, c"Entered %s\n".as_ptr(), c"s3c_pcm_trigger".as_ptr());

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*pcm).lock, &mut flags);
        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            s3c_pcm_snd_rxctrl(pcm, 1);
        } else {
            s3c_pcm_snd_txctrl(pcm, 1);
        }
        spin_unlock_irqrestore(&mut (*pcm).lock, flags);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*pcm).lock, &mut flags);
        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            s3c_pcm_snd_rxctrl(pcm, 0);
        } else {
            s3c_pcm_snd_txctrl(pcm, 0);
        }
        spin_unlock_irqrestore(&mut (*pcm).lock, flags);
    } else {
        return -22;
    }

    0
}

unsafe extern "C" fn s3c_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    _socdai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let pcm = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut s3c_pcm_info;
    let regs = (*pcm).regs;
    let clk: *mut clk;
    let sclk_div: c_int;
    let sync_div: c_int;
    let mut clkctl: u32;

    dev_dbg((*pcm).dev, c"Entered %s\n".as_ptr(), c"s3c_pcm_hw_params".as_ptr());

    /* Strictly check for sample size */
    match params_width(params) {
        16 => {}
        _ => return -22,
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*pcm).lock, &mut flags);

    /* Get hold of the PCMSOURCE_CLK */
    clkctl = readl(reg(regs, S3C_PCM_CLKCTL));
    if (clkctl & S3C_PCM_CLKCTL_SERCLKSEL_PCLK) != 0 {
        clk = (*pcm).pclk;
    } else {
        clk = (*pcm).cclk;
    }

    /* Set the SCLK divider */
    sclk_div = (clk_get_rate(clk) / (*pcm).sclk_per_fs as c_ulong / params_rate(params) as c_ulong
        / 2
        - 1) as c_int;

    clkctl &= !(S3C_PCM_CLKCTL_SCLKDIV_MASK << S3C_PCM_CLKCTL_SCLKDIV_SHIFT);
    clkctl |= ((sclk_div as u32) & S3C_PCM_CLKCTL_SCLKDIV_MASK)
        << S3C_PCM_CLKCTL_SCLKDIV_SHIFT;

    /* Set the SYNC divider */
    sync_div = (*pcm).sclk_per_fs as c_int - 1;

    clkctl &= !(S3C_PCM_CLKCTL_SYNCDIV_MASK << S3C_PCM_CLKCTL_SYNCDIV_SHIFT);
    clkctl |= ((sync_div as u32) & S3C_PCM_CLKCTL_SYNCDIV_MASK)
        << S3C_PCM_CLKCTL_SYNCDIV_SHIFT;

    writel(clkctl, reg(regs, S3C_PCM_CLKCTL));
    spin_unlock_irqrestore(&mut (*pcm).lock, flags);

    dev_dbg(
        (*pcm).dev,
        c"PCMSOURCE_CLK-%lu SCLK=%ufs SCLK_DIV=%d SYNC_DIV=%d\n".as_ptr(),
        clk_get_rate(clk),
        (*pcm).sclk_per_fs,
        sclk_div,
        sync_div,
    );

    0
}

unsafe extern "C" fn s3c_pcm_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let pcm = snd_soc_dai_get_drvdata(cpu_dai) as *mut s3c_pcm_info;
    let regs = (*pcm).regs;
    let mut ctl: u32;

    dev_dbg((*pcm).dev, c"Entered %s\n".as_ptr(), c"s3c_pcm_set_fmt".as_ptr());

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*pcm).lock, &mut flags);

    ctl = readl(reg(regs, S3C_PCM_CTL));

    if (fmt & SND_SOC_DAIFMT_INV_MASK) == SND_SOC_DAIFMT_IB_NF {
        /* Nothing to do, IB_NF by default */
    } else {
        dev_err((*pcm).dev, c"Unsupported clock inversion!\n".as_ptr());
        spin_unlock_irqrestore(&mut (*pcm).lock, flags);
        return -22;
    }

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP {
        /* Nothing to do, Master by default */
    } else {
        dev_err((*pcm).dev, c"Unsupported master/slave format!\n".as_ptr());
        spin_unlock_irqrestore(&mut (*pcm).lock, flags);
        return -22;
    }

    if (fmt & SND_SOC_DAIFMT_CLOCK_MASK) == SND_SOC_DAIFMT_CONT {
        (*pcm).idleclk = 1;
    } else if (fmt & SND_SOC_DAIFMT_CLOCK_MASK) == SND_SOC_DAIFMT_GATED {
        (*pcm).idleclk = 0;
    } else {
        dev_err((*pcm).dev, c"Invalid Clock gating request!\n".as_ptr());
        spin_unlock_irqrestore(&mut (*pcm).lock, flags);
        return -22;
    }

    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
        ctl |= S3C_PCM_CTL_TXMSB_AFTER_FSYNC;
        ctl |= S3C_PCM_CTL_RXMSB_AFTER_FSYNC;
    } else if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_B {
        ctl &= !S3C_PCM_CTL_TXMSB_AFTER_FSYNC;
        ctl &= !S3C_PCM_CTL_RXMSB_AFTER_FSYNC;
    } else {
        dev_err((*pcm).dev, c"Unsupported data format!\n".as_ptr());
        spin_unlock_irqrestore(&mut (*pcm).lock, flags);
        return -22;
    }

    writel(ctl, reg(regs, S3C_PCM_CTL));
    spin_unlock_irqrestore(&mut (*pcm).lock, flags);
    0
}

unsafe extern "C" fn s3c_pcm_set_clkdiv(
    cpu_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let pcm = snd_soc_dai_get_drvdata(cpu_dai) as *mut s3c_pcm_info;

    if div_id == S3C_PCM_SCLK_PER_FS {
        (*pcm).sclk_per_fs = div as c_uint;
    } else {
        return -22;
    }

    0
}

unsafe extern "C" fn s3c_pcm_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let pcm = snd_soc_dai_get_drvdata(cpu_dai) as *mut s3c_pcm_info;
    let regs = (*pcm).regs;
    let mut clkctl = readl(reg(regs, S3C_PCM_CLKCTL));

    if clk_id == S3C_PCM_CLKSRC_PCLK {
        clkctl |= S3C_PCM_CLKCTL_SERCLKSEL_PCLK;
    } else if clk_id == S3C_PCM_CLKSRC_MUX {
        clkctl &= !S3C_PCM_CLKCTL_SERCLKSEL_PCLK;

        if clk_get_rate((*pcm).cclk) != freq as c_ulong {
            clk_set_rate((*pcm).cclk, freq);
        }
    } else {
        return -22;
    }

    writel(clkctl, reg(regs, S3C_PCM_CLKCTL));

    0
}

unsafe extern "C" fn s3c_pcm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let pcm = snd_soc_dai_get_drvdata(dai) as *mut s3c_pcm_info;

    snd_soc_dai_init_dma_data(dai, (*pcm).dma_playback, (*pcm).dma_capture);

    0
}

static S3C_PCM_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(s3c_pcm_dai_probe),
    set_sysclk: Some(s3c_pcm_set_sysclk),
    set_clkdiv: Some(s3c_pcm_set_clkdiv),
    trigger: Some(s3c_pcm_trigger),
    hw_params: Some(s3c_pcm_hw_params),
    set_fmt: Some(s3c_pcm_set_fmt),
};

const fn stream_zero() -> snd_soc_pcm_stream {
    snd_soc_pcm_stream {
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
    }
}

const fn dai_driver_zero() -> snd_soc_dai_driver {
    snd_soc_dai_driver {
        name: ptr::null(),
        symmetric_rate: 0,
        ops: ptr::null(),
        playback: stream_zero(),
        capture: stream_zero(),
    }
}

/* #define S3C_PCM_RATES  SNDRV_PCM_RATE_8000_96000 */
/* S3C_PCM_DAI_DECLARE initializes common DAI fields below. */
static mut S3C_PCM_DAI: [snd_soc_dai_driver; 2] = [dai_driver_zero(), dai_driver_zero()];

static S3C_PCM_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: c"s3c-pcm".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe fn init_s3c_pcm_dai() {
    S3C_PCM_DAI[0].name = c"samsung-pcm.0".as_ptr();
    S3C_PCM_DAI[1].name = c"samsung-pcm.1".as_ptr();
    let mut i = 0;
    while i < 2 {
        S3C_PCM_DAI[i].symmetric_rate = 1;
        S3C_PCM_DAI[i].ops = &S3C_PCM_DAI_OPS;
        S3C_PCM_DAI[i].playback = snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        };
        S3C_PCM_DAI[i].capture = snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        };
        i += 1;
    }
}

unsafe extern "C" fn s3c_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let pcm: *mut s3c_pcm_info;
    let mut mem_res: *mut resource = ptr::null_mut();
    let pcm_pdata: *mut s3c_audio_pdata;
    let mut filter: dma_filter_fn;
    let mut ret: c_int;

    /* Check for valid device index */
    if (*pdev).id < 0 || (*pdev).id as usize >= S3C_PCM.len() {
        dev_err(
            &mut (*pdev).dev,
            c"id %d out of range\n".as_ptr(),
            (*pdev).id,
        );
        return -22;
    }

    pcm_pdata = (*pdev).dev.platform_data as *mut s3c_audio_pdata;

    if !pcm_pdata.is_null()
        && (*pcm_pdata).cfg_gpio.is_some()
        && ((*pcm_pdata).cfg_gpio.unwrap())(pdev) != 0
    {
        dev_err(&mut (*pdev).dev, c"Unable to configure gpio\n".as_ptr());
        return -22;
    }

    pcm = &mut S3C_PCM[(*pdev).id as usize];
    (*pcm).dev = &mut (*pdev).dev;

    spin_lock_init(&mut (*pcm).lock);

    /* Default is 128fs */
    (*pcm).sclk_per_fs = 128;

    (*pcm).regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem_res);
    if IS_ERR((*pcm).regs) {
        return PTR_ERR((*pcm).regs);
    }

    (*pcm).cclk = devm_clk_get(&mut (*pdev).dev, c"audio-bus".as_ptr());
    if IS_ERR((*pcm).cclk as *const c_void) {
        dev_err(&mut (*pdev).dev, c"failed to get audio-bus clock\n".as_ptr());
        return PTR_ERR((*pcm).cclk as *const c_void);
    }
    ret = clk_prepare_enable((*pcm).cclk);
    if ret != 0 {
        return ret;
    }

    /* record our pcm structure for later use in the callbacks */
    dev_set_drvdata(&mut (*pdev).dev, pcm as *mut c_void);

    (*pcm).pclk = devm_clk_get(&mut (*pdev).dev, c"pcm".as_ptr());
    if IS_ERR((*pcm).pclk as *const c_void) {
        dev_err(&mut (*pdev).dev, c"failed to get pcm clock\n".as_ptr());
        ret = PTR_ERR((*pcm).pclk as *const c_void);
        goto_err_dis_cclk(pcm);
        return ret;
    }
    ret = clk_prepare_enable((*pcm).pclk);
    if ret != 0 {
        goto_err_dis_cclk(pcm);
        return ret;
    }

    S3C_PCM_STEREO_IN[(*pdev).id as usize].addr = (*mem_res).start + S3C_PCM_RXFIFO;
    S3C_PCM_STEREO_OUT[(*pdev).id as usize].addr = (*mem_res).start + S3C_PCM_TXFIFO;

    filter = None;
    if !pcm_pdata.is_null() {
        S3C_PCM_STEREO_IN[(*pdev).id as usize].filter_data = (*pcm_pdata).dma_capture;
        S3C_PCM_STEREO_OUT[(*pdev).id as usize].filter_data = (*pcm_pdata).dma_playback;
        filter = (*pcm_pdata).dma_filter;
    }

    (*pcm).dma_capture = &mut S3C_PCM_STEREO_IN[(*pdev).id as usize];
    (*pcm).dma_playback = &mut S3C_PCM_STEREO_OUT[(*pdev).id as usize];

    ret = samsung_asoc_dma_platform_register(
        &mut (*pdev).dev,
        filter,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"failed to get register DMA: %d\n".as_ptr(),
            ret,
        );
        goto_err_dis_pclk(pcm);
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);

    init_s3c_pcm_dai();
    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &S3C_PCM_COMPONENT,
        &mut S3C_PCM_DAI[(*pdev).id as usize],
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"failed to get register DAI: %d\n".as_ptr(),
            ret,
        );
        goto_err_dis_pm(pdev, pcm);
        return ret;
    }

    0
}

unsafe fn goto_err_dis_pm(pdev: *mut platform_device, pcm: *mut s3c_pcm_info) {
    pm_runtime_disable(&mut (*pdev).dev);
    goto_err_dis_pclk(pcm);
}

unsafe fn goto_err_dis_pclk(pcm: *mut s3c_pcm_info) {
    clk_disable_unprepare((*pcm).pclk);
    goto_err_dis_cclk(pcm);
}

unsafe fn goto_err_dis_cclk(pcm: *mut s3c_pcm_info) {
    clk_disable_unprepare((*pcm).cclk);
}

unsafe extern "C" fn s3c_pcm_dev_remove(pdev: *mut platform_device) {
    let pcm = &mut S3C_PCM[(*pdev).id as usize] as *mut s3c_pcm_info;

    pm_runtime_disable(&mut (*pdev).dev);
    clk_disable_unprepare((*pcm).cclk);
    clk_disable_unprepare((*pcm).pclk);
}

static mut S3C_PCM_DRIVER: platform_driver = platform_driver {
    probe: Some(s3c_pcm_dev_probe),
    remove: Some(s3c_pcm_dev_remove),
    driver: device_driver {
        name: c"samsung-pcm".as_ptr(),
    },
};

/* module_platform_driver(s3c_pcm_driver); */
#[no_mangle]
pub unsafe extern "C" fn init_module() -> c_int {
    __platform_driver_register(&mut S3C_PCM_DRIVER, ptr::null_mut())
}

/* Module information */
/* MODULE_AUTHOR("Jaswinder Singh, <jassisinghbrar@gmail.com>"); */
/* MODULE_DESCRIPTION("S3C PCM Controller Driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:samsung-pcm"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
