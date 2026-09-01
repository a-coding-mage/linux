// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Audio Layer - Samsung S/PDIF Controller driver
//
// Copyright (c) 2010 Samsung Electronics Co. Ltd
//		http://www.samsung.com/

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

/* Registers */
const CLKCON: usize = 0x00;
const CON: usize = 0x04;
const BSTAS: usize = 0x08;
const CSTAS: usize = 0x0C;
const DATA_OUTBUF: usize = 0x10;
const DCNT: usize = 0x14;
const BSTAS_S: usize = 0x18;
const DCNT_S: usize = 0x1C;

const CLKCTL_MASK: u32 = 0x7;
const CLKCTL_MCLK_EXT: u32 = 0x1 << 2;
const CLKCTL_PWR_ON: u32 = 0x1 << 0;

const CON_MASK: u32 = 0x3ffffff;
const CON_FIFO_TH_SHIFT: u32 = 19;
const CON_FIFO_TH_MASK: u32 = 0x7 << 19;
const CON_USERDATA_23RDBIT: u32 = 0x1 << 12;

const CON_SW_RESET: u32 = 0x1 << 5;

const CON_MCLKDIV_MASK: u32 = 0x3 << 3;
const CON_MCLKDIV_256FS: u32 = 0x0 << 3;
const CON_MCLKDIV_384FS: u32 = 0x1 << 3;
const CON_MCLKDIV_512FS: u32 = 0x2 << 3;

const CON_PCM_MASK: u32 = 0x3 << 1;
const CON_PCM_16BIT: u32 = 0x0 << 1;
const CON_PCM_20BIT: u32 = 0x1 << 1;
const CON_PCM_24BIT: u32 = 0x2 << 1;

const CON_PCM_DATA: u32 = 0x1 << 0;

const CSTAS_MASK: u32 = 0x3fffffff;
const CSTAS_SAMP_FREQ_MASK: u32 = 0xF << 24;
const CSTAS_SAMP_FREQ_44: u32 = 0x0 << 24;
const CSTAS_SAMP_FREQ_48: u32 = 0x2 << 24;
const CSTAS_SAMP_FREQ_32: u32 = 0x3 << 24;
const CSTAS_SAMP_FREQ_96: u32 = 0xA << 24;

const CSTAS_CATEGORY_MASK: u32 = 0xFF << 8;
const CSTAS_CATEGORY_CODE_CDP: u32 = 0x01 << 8;

const CSTAS_NO_COPYRIGHT: u32 = 0x1 << 2;

const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const IORESOURCE_MEM: c_uint = 0;
const SND_SOC_SPDIF_INT_MCLK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;

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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
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
pub struct snd_dmaengine_dai_dma_data {
    pub addr_width: c_int,
    pub addr: c_ulong,
    pub filter_data: *mut c_void,
}

pub type dma_filter_fn = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>;

#[repr(C)]
pub struct s3c_audio_pdata {
    pub cfg_gpio: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub dma_playback: *mut c_void,
    pub dma_filter: dma_filter_fn,
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
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
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

/**
 * struct samsung_spdif_info - Samsung S/PDIF Controller information
 * @lock: Spin lock for S/PDIF.
 * @dev: The parent device passed to use from the probe.
 * @regs: The pointer to the device register block.
 * @clk_rate: Current clock rate for calcurate ratio.
 * @pclk: The peri-clock pointer for spdif master operation.
 * @sclk: The source clock pointer for making sync signals.
 * @saved_clkcon: Backup clkcon reg. in suspend.
 * @saved_con: Backup con reg. in suspend.
 * @saved_cstas: Backup cstas reg. in suspend.
 * @dma_playback: DMA information for playback channel.
 */
#[repr(C)]
pub struct samsung_spdif_info {
    lock: spinlock_t,
    dev: *mut device,
    regs: *mut c_void,
    clk_rate: c_ulong,
    pclk: *mut clk,
    sclk: *mut clk,
    saved_clkcon: u32,
    saved_con: u32,
    saved_cstas: u32,
    dma_playback: *mut snd_dmaengine_dai_dma_data,
}

static mut SPDIF_STEREO_OUT: snd_dmaengine_dai_dma_data = snd_dmaengine_dai_dma_data {
    addr_width: 0,
    addr: 0,
    filter_data: ptr::null_mut(),
};

static mut SPDIF_INFO: samsung_spdif_info = samsung_spdif_info {
    lock: spinlock_t { _private: [] },
    dev: ptr::null_mut(),
    regs: ptr::null_mut(),
    clk_rate: 0,
    pclk: ptr::null_mut(),
    sclk: ptr::null_mut(),
    saved_clkcon: 0,
    saved_con: 0,
    saved_cstas: 0,
    dma_playback: ptr::null_mut(),
};

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(cpu_dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn cpu_relax();
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn platform_get_resource(
        pdev: *mut platform_device,
        resource_type: c_uint,
        num: c_uint,
    ) -> *mut resource;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn samsung_asoc_dma_platform_register(
        dev: *mut device,
        filter: dma_filter_fn,
        arg1: *mut c_void,
        arg2: *mut c_void,
        arg3: *mut c_void,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[inline]
unsafe fn reg(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

#[inline]
unsafe fn component_to_info(component: *mut snd_soc_component) -> *mut samsung_spdif_info {
    snd_soc_component_get_drvdata(component) as *mut samsung_spdif_info
}

#[inline]
unsafe fn to_info(cpu_dai: *mut snd_soc_dai) -> *mut samsung_spdif_info {
    snd_soc_dai_get_drvdata(cpu_dai) as *mut samsung_spdif_info
}

unsafe fn spdif_snd_txctrl(spdif: *mut samsung_spdif_info, on: c_int) {
    let regs: *mut c_void = (*spdif).regs;
    let clkcon: u32;

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_snd_txctrl".as_ptr());

    clkcon = readl(reg(regs, CLKCON)) & CLKCTL_MASK;
    if on != 0 {
        writel(clkcon | CLKCTL_PWR_ON, reg(regs, CLKCON));
    } else {
        writel(clkcon & !CLKCTL_PWR_ON, reg(regs, CLKCON));
    }
}

unsafe extern "C" fn spdif_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let spdif: *mut samsung_spdif_info = to_info(cpu_dai);
    let mut clkcon: u32;

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_set_sysclk".as_ptr());

    clkcon = readl(reg((*spdif).regs, CLKCON));

    if clk_id == SND_SOC_SPDIF_INT_MCLK {
        clkcon &= !CLKCTL_MCLK_EXT;
    } else {
        clkcon |= CLKCTL_MCLK_EXT;
    }

    writel(clkcon, reg((*spdif).regs, CLKCON));

    (*spdif).clk_rate = freq as c_ulong;

    0
}

unsafe extern "C" fn spdif_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let spdif: *mut samsung_spdif_info = to_info(snd_soc_rtd_to_cpu(rtd, 0));

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_trigger".as_ptr());

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            let flags = spin_lock_irqsave(&mut (*spdif).lock);
            spdif_snd_txctrl(spdif, 1);
            spin_unlock_irqrestore(&mut (*spdif).lock, flags);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            let flags = spin_lock_irqsave(&mut (*spdif).lock);
            spdif_snd_txctrl(spdif, 0);
            spin_unlock_irqrestore(&mut (*spdif).lock, flags);
        }
        _ => return -EINVAL,
    }

    0
}

static mut SPDIF_SYSCLK_RATIOS: [c_int; 3] = [512, 384, 256];

unsafe extern "C" fn spdif_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    _socdai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let spdif: *mut samsung_spdif_info = to_info(snd_soc_rtd_to_cpu(rtd, 0));
    let regs: *mut c_void = (*spdif).regs;
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let mut con: u32;
    let clkcon: u32;
    let mut cstas: u32;
    let mut i: usize;
    let ratio: c_int;

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_hw_params".as_ptr());

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dma_data = (*spdif).dma_playback;
    } else {
        dev_err((*spdif).dev, c"Capture is not supported\n".as_ptr());
        return -EINVAL;
    }

    snd_soc_dai_set_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream, dma_data);

    let flags = spin_lock_irqsave(&mut (*spdif).lock);

    con = readl(reg(regs, CON)) & CON_MASK;
    cstas = readl(reg(regs, CSTAS)) & CSTAS_MASK;
    clkcon = readl(reg(regs, CLKCON)) & CLKCTL_MASK;

    con &= !CON_FIFO_TH_MASK;
    con |= 0x7 << CON_FIFO_TH_SHIFT;
    con |= CON_USERDATA_23RDBIT;
    con |= CON_PCM_DATA;

    con &= !CON_PCM_MASK;
    match params_width(params) {
        16 => {
            con |= CON_PCM_16BIT;
        }
        _ => {
            dev_err((*spdif).dev, c"Unsupported data size.\n".as_ptr());
            spin_unlock_irqrestore(&mut (*spdif).lock, flags);
            return -EINVAL;
        }
    }

    ratio = ((*spdif).clk_rate / params_rate(params) as c_ulong) as c_int;
    i = 0;
    while i < SPDIF_SYSCLK_RATIOS.len() {
        if ratio == SPDIF_SYSCLK_RATIOS[i] {
            break;
        }
        i += 1;
    }
    if i == SPDIF_SYSCLK_RATIOS.len() {
        dev_err(
            (*spdif).dev,
            c"Invalid clock ratio %ld/%d\n".as_ptr(),
            (*spdif).clk_rate,
            params_rate(params),
        );
        spin_unlock_irqrestore(&mut (*spdif).lock, flags);
        return -EINVAL;
    }

    con &= !CON_MCLKDIV_MASK;
    match ratio {
        256 => {
            con |= CON_MCLKDIV_256FS;
        }
        384 => {
            con |= CON_MCLKDIV_384FS;
        }
        512 => {
            con |= CON_MCLKDIV_512FS;
        }
        _ => {}
    }

    cstas &= !CSTAS_SAMP_FREQ_MASK;
    match params_rate(params) {
        44100 => {
            cstas |= CSTAS_SAMP_FREQ_44;
        }
        48000 => {
            cstas |= CSTAS_SAMP_FREQ_48;
        }
        32000 => {
            cstas |= CSTAS_SAMP_FREQ_32;
        }
        96000 => {
            cstas |= CSTAS_SAMP_FREQ_96;
        }
        _ => {
            dev_err(
                (*spdif).dev,
                c"Invalid sampling rate %d\n".as_ptr(),
                params_rate(params),
            );
            spin_unlock_irqrestore(&mut (*spdif).lock, flags);
            return -EINVAL;
        }
    }

    cstas &= !CSTAS_CATEGORY_MASK;
    cstas |= CSTAS_CATEGORY_CODE_CDP;
    cstas |= CSTAS_NO_COPYRIGHT;

    writel(con, reg(regs, CON));
    writel(cstas, reg(regs, CSTAS));
    writel(clkcon, reg(regs, CLKCON));

    spin_unlock_irqrestore(&mut (*spdif).lock, flags);

    0
}

unsafe extern "C" fn spdif_shutdown(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let spdif: *mut samsung_spdif_info = to_info(snd_soc_rtd_to_cpu(rtd, 0));
    let regs: *mut c_void = (*spdif).regs;
    let con: u32;
    let clkcon: u32;

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_shutdown".as_ptr());

    con = readl(reg(regs, CON)) & CON_MASK;
    clkcon = readl(reg(regs, CLKCON)) & CLKCTL_MASK;

    writel(con | CON_SW_RESET, reg(regs, CON));
    cpu_relax();

    writel(clkcon & !CLKCTL_PWR_ON, reg(regs, CLKCON));
}

/* CONFIG_PM: suspend/resume callbacks are present when power management is enabled. */
unsafe extern "C" fn spdif_suspend(component: *mut snd_soc_component) -> c_int {
    let spdif: *mut samsung_spdif_info = component_to_info(component);
    let con: u32 = (*spdif).saved_con;

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_suspend".as_ptr());

    (*spdif).saved_clkcon = readl(reg((*spdif).regs, CLKCON)) & CLKCTL_MASK;
    (*spdif).saved_con = readl(reg((*spdif).regs, CON)) & CON_MASK;
    (*spdif).saved_cstas = readl(reg((*spdif).regs, CSTAS)) & CSTAS_MASK;

    writel(con | CON_SW_RESET, reg((*spdif).regs, CON));
    cpu_relax();

    0
}

unsafe extern "C" fn spdif_resume(component: *mut snd_soc_component) -> c_int {
    let spdif: *mut samsung_spdif_info = component_to_info(component);

    dev_dbg((*spdif).dev, c"Entered %s\n".as_ptr(), c"spdif_resume".as_ptr());

    writel((*spdif).saved_clkcon, reg((*spdif).regs, CLKCON));
    writel((*spdif).saved_con, reg((*spdif).regs, CON));
    writel((*spdif).saved_cstas, reg((*spdif).regs, CSTAS));

    0
}

static SPDIF_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(spdif_set_sysclk),
    trigger: Some(spdif_trigger),
    hw_params: Some(spdif_hw_params),
    shutdown: Some(spdif_shutdown),
};

static mut SAMSUNG_SPDIF_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"samsung-spdif".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"S/PDIF Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &SPDIF_DAI_OPS,
};

static SAMSUNG_SPDIF_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: c"samsung-spdif".as_ptr(),
    suspend: Some(spdif_suspend),
    resume: Some(spdif_resume),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn spdif_probe(pdev: *mut platform_device) -> c_int {
    let spdif_pdata: *mut s3c_audio_pdata;
    let mem_res: *mut resource;
    let spdif: *mut samsung_spdif_info;
    let mut filter: dma_filter_fn;
    let mut ret: c_int;

    spdif_pdata = (*pdev).dev.platform_data as *mut s3c_audio_pdata;

    dev_dbg(&mut (*pdev).dev, c"Entered %s\n".as_ptr(), c"spdif_probe".as_ptr());

    mem_res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if mem_res.is_null() {
        dev_err(&mut (*pdev).dev, c"Unable to get register resource.\n".as_ptr());
        return -ENXIO;
    }

    if !spdif_pdata.is_null()
        && (*spdif_pdata).cfg_gpio.is_some()
        && ((*spdif_pdata).cfg_gpio.unwrap())(pdev) != 0
    {
        dev_err(&mut (*pdev).dev, c"Unable to configure GPIO pins\n".as_ptr());
        return -EINVAL;
    }

    spdif = &raw mut SPDIF_INFO;
    (*spdif).dev = &mut (*pdev).dev;

    spin_lock_init(&mut (*spdif).lock);

    (*spdif).pclk = devm_clk_get(&mut (*pdev).dev, c"spdif".as_ptr());
    if IS_ERR((*spdif).pclk as *const c_void) {
        ret = dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*spdif).pclk as *const c_void),
            c"failed to get peri-clock\n".as_ptr(),
        );
        return ret;
    }
    ret = clk_prepare_enable((*spdif).pclk);
    if ret != 0 {
        return ret;
    }

    (*spdif).sclk = devm_clk_get(&mut (*pdev).dev, c"sclk_spdif".as_ptr());
    if IS_ERR((*spdif).sclk as *const c_void) {
        ret = dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*spdif).sclk as *const c_void),
            c"failed to get internal source clock\n".as_ptr(),
        );
        clk_disable_unprepare((*spdif).pclk);
        return ret;
    }
    ret = clk_prepare_enable((*spdif).sclk);
    if ret != 0 {
        clk_disable_unprepare((*spdif).pclk);
        return ret;
    }

    (*spdif).regs = devm_ioremap_resource(&mut (*pdev).dev, mem_res);
    if IS_ERR((*spdif).regs as *const c_void) {
        ret = PTR_ERR((*spdif).regs as *const c_void);
        clk_disable_unprepare((*spdif).sclk);
        clk_disable_unprepare((*spdif).pclk);
        return ret;
    }

    SPDIF_STEREO_OUT.addr_width = 2;
    SPDIF_STEREO_OUT.addr = (*mem_res).start + DATA_OUTBUF as c_ulong;
    filter = None;
    if !spdif_pdata.is_null() {
        SPDIF_STEREO_OUT.filter_data = (*spdif_pdata).dma_playback;
        filter = (*spdif_pdata).dma_filter;
    }
    (*spdif).dma_playback = &raw mut SPDIF_STEREO_OUT;

    ret = samsung_asoc_dma_platform_register(
        &mut (*pdev).dev,
        filter,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to register DMA: %d\n".as_ptr(), ret);
        clk_disable_unprepare((*spdif).sclk);
        clk_disable_unprepare((*spdif).pclk);
        return ret;
    }

    dev_set_drvdata(&mut (*pdev).dev, spdif as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &SAMSUNG_SPDIF_COMPONENT,
        &raw mut SAMSUNG_SPDIF_DAI,
        1,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"fail to register dai\n".as_ptr());
        clk_disable_unprepare((*spdif).sclk);
        clk_disable_unprepare((*spdif).pclk);
        return ret;
    }

    0
}

unsafe extern "C" fn spdif_remove(_pdev: *mut platform_device) {
    let spdif: *mut samsung_spdif_info = &raw mut SPDIF_INFO;

    clk_disable_unprepare((*spdif).sclk);
    clk_disable_unprepare((*spdif).pclk);
}

static mut SAMSUNG_SPDIF_DRIVER: platform_driver = platform_driver {
    probe: Some(spdif_probe),
    remove: Some(spdif_remove),
    driver: platform_driver_inner {
        name: c"samsung-spdif".as_ptr(),
    },
};

/* module_platform_driver(samsung_spdif_driver); */
/* MODULE_AUTHOR("Seungwhan Youn, <sw.youn@samsung.com>"); */
/* MODULE_DESCRIPTION("Samsung S/PDIF Controller Driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:samsung-spdif"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
