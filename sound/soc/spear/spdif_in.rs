// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC SPDIF In Audio Layer for spear processors
 *
 * Copyright (C) 2012 ST Microelectronics
 * Vipin Kumar <vipin.kumar@st.com>
 */

// Dependencies in the original C source:
// linux/clk.h, linux/delay.h, linux/device.h, linux/kernel.h, linux/init.h,
// linux/io.h, linux/ioport.h, linux/module.h, linux/platform_device.h,
// sound/dmaengine_pcm.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/spear_dma.h, sound/spear_spdif.h, spdif_in_regs.h, spear_pcm.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = c_uint;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
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
pub struct snd_soc_dai_ops {
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u32,
    pub formats: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct spear_dma_data {
    pub data: usize,
    pub addr: usize,
    pub max_burst: c_uint,
    pub addr_width: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub filter_data: *mut c_void,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spear_spdif_platform_data {
    pub dma_params: usize,
    pub reset_perip: Option<unsafe extern "C" fn()>,
    pub filter: *mut c_void,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
struct spdif_in_params {
    format: u32,
}

#[repr(C)]
struct spdif_in_dev {
    clk: *mut clk,
    dma_params: spear_dma_data,
    saved_params: spdif_in_params,
    io_base: *mut c_void,
    dev: *mut device,
    reset_perip: Option<unsafe extern "C" fn()>,
    irq: c_int,
    dma_params_rx: snd_dmaengine_dai_dma_data,
    config: snd_dmaengine_pcm_config,
}

unsafe extern "C" {
    static SPDIF_IN_PRTYEN: u32;
    static SPDIF_IN_STATEN: u32;
    static SPDIF_IN_USREN: u32;
    static SPDIF_IN_VALEN: u32;
    static SPDIF_IN_BLKEN: u32;
    static SPDIF_MODE_16BIT: u32;
    static SPDIF_FIFO_THRES_16: u32;
    static SPDIF_IN_CTRL: usize;
    static SPDIF_IN_IRQ_MASK: usize;
    static SNDRV_PCM_FORMAT_S16_LE: u32;
    static SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE: u32;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static EINVAL: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SPDIF_IN_SAMPLE: u32;
    static SPDIF_IN_ENB: u32;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_RATE_192000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u32;
    static SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u32;
    static SPDIF_IN_IRQ: usize;
    static IRQ_NONE: irqreturn_t;
    static SPDIF_IRQ_FIFOWRITE: u32;
    static SPDIF_IRQ_EMPTYFIFOREAD: u32;
    static SPDIF_IRQ_FIFOFULL: u32;
    static SPDIF_IRQ_OUTOFRANGE: u32;
    static IRQ_HANDLED: irqreturn_t;
    static IORESOURCE_IO: c_uint;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint;

    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_dma_data_set_capture(dai: *mut snd_soc_dai, data: *mut snd_dmaengine_dai_dma_data);
    fn params_format(params: *mut snd_pcm_hw_params) -> u32;
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_resource(pdev: *mut platform_device, kind: c_uint, num: c_uint) -> *mut resource;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_spear_pcm_platform_register(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        filter: *mut c_void,
    ) -> c_int;
}

const SPDIF_IN_COMPONENT_NAME: &[u8] = b"spdif-in\0";
const SPDIF_IN_IRQ_NAME: &[u8] = b"spdif-in\0";
const SPDIF_IN_DRIVER_NAME: &[u8] = b"spdif-in\0";
const MSG_FIFO_WRITE: &[u8] = b"spdif in: fifo write error\0";
const MSG_EMPTY_FIFO_READ: &[u8] = b"spdif in: empty fifo read error\0";
const MSG_FIFO_FULL: &[u8] = b"spdif in: fifo full error\0";
const MSG_OUT_OF_RANGE: &[u8] = b"spdif in: out of range error\0";

unsafe fn io_addr(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

unsafe extern "C" fn spdif_in_configure(host: *mut spdif_in_dev) {
    let mut ctrl: u32 = SPDIF_IN_PRTYEN | SPDIF_IN_STATEN | SPDIF_IN_USREN | SPDIF_IN_VALEN | SPDIF_IN_BLKEN;
    ctrl |= SPDIF_MODE_16BIT | SPDIF_FIFO_THRES_16;

    writel(ctrl, io_addr((*host).io_base, SPDIF_IN_CTRL));
    writel(0xF, io_addr((*host).io_base, SPDIF_IN_IRQ_MASK));
}

unsafe extern "C" fn spdif_in_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let host: *mut spdif_in_dev = snd_soc_dai_get_drvdata(dai) as *mut spdif_in_dev;

    (*host).dma_params_rx.filter_data = &mut (*host).dma_params as *mut spear_dma_data as *mut c_void;
    snd_soc_dai_dma_data_set_capture(dai, &mut (*host).dma_params_rx);

    0
}

unsafe extern "C" fn spdif_in_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let host: *mut spdif_in_dev = snd_soc_dai_get_drvdata(dai) as *mut spdif_in_dev;

    if (*substream).stream != SNDRV_PCM_STREAM_CAPTURE {
        return;
    }

    writel(0x0, io_addr((*host).io_base, SPDIF_IN_IRQ_MASK));
}

unsafe extern "C" fn spdif_in_format(host: *mut spdif_in_dev, format: u32) {
    let mut ctrl: u32 = readl(io_addr((*host).io_base, SPDIF_IN_CTRL));

    if format == SNDRV_PCM_FORMAT_S16_LE {
        ctrl |= SPDIF_XTRACT_16BIT;
    } else if format == SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE {
        ctrl &= !SPDIF_XTRACT_16BIT;
    }

    writel(ctrl, io_addr((*host).io_base, SPDIF_IN_CTRL));
}

unsafe extern "C" {
    static SPDIF_XTRACT_16BIT: u32;
}

unsafe extern "C" fn spdif_in_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let host: *mut spdif_in_dev = snd_soc_dai_get_drvdata(dai) as *mut spdif_in_dev;
    let format: u32;

    if (*substream).stream != SNDRV_PCM_STREAM_CAPTURE {
        return -EINVAL;
    }

    format = params_format(params);
    (*host).saved_params.format = format;

    0
}

unsafe extern "C" fn spdif_in_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let host: *mut spdif_in_dev = snd_soc_dai_get_drvdata(dai) as *mut spdif_in_dev;
    let mut ctrl: u32;
    let mut ret: c_int = 0;

    if (*substream).stream != SNDRV_PCM_STREAM_CAPTURE {
        return -EINVAL;
    }

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        clk_enable((*host).clk);
        spdif_in_configure(host);
        spdif_in_format(host, (*host).saved_params.format);

        ctrl = readl(io_addr((*host).io_base, SPDIF_IN_CTRL));
        ctrl |= SPDIF_IN_SAMPLE | SPDIF_IN_ENB;
        writel(ctrl, io_addr((*host).io_base, SPDIF_IN_CTRL));
        writel(0xF, io_addr((*host).io_base, SPDIF_IN_IRQ_MASK));
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        ctrl = readl(io_addr((*host).io_base, SPDIF_IN_CTRL));
        ctrl &= !(SPDIF_IN_SAMPLE | SPDIF_IN_ENB);
        writel(ctrl, io_addr((*host).io_base, SPDIF_IN_CTRL));
        writel(0x0, io_addr((*host).io_base, SPDIF_IN_IRQ_MASK));

        if let Some(reset_perip) = (*host).reset_perip {
            reset_perip();
        }
        clk_disable((*host).clk);
    } else {
        ret = -EINVAL;
    }
    ret
}

static spdif_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    shutdown: Some(spdif_in_shutdown),
    probe: Some(spdif_in_dai_probe),
    trigger: Some(spdif_in_trigger),
    hw_params: Some(spdif_in_hw_params),
};

static mut spdif_in_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: unsafe {
            SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_44100
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_96000
                | SNDRV_PCM_RATE_192000
        },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE },
    },
    ops: &spdif_in_dai_ops,
};

static spdif_in_component: snd_soc_component_driver = snd_soc_component_driver {
    name: SPDIF_IN_COMPONENT_NAME.as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn spdif_in_irq(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let host: *mut spdif_in_dev = arg as *mut spdif_in_dev;

    let irq_status: u32 = readl(io_addr((*host).io_base, SPDIF_IN_IRQ));

    if irq_status == 0 {
        return IRQ_NONE;
    }

    if irq_status & SPDIF_IRQ_FIFOWRITE != 0 {
        dev_err((*host).dev, MSG_FIFO_WRITE.as_ptr() as *const c_char);
    }
    if irq_status & SPDIF_IRQ_EMPTYFIFOREAD != 0 {
        dev_err((*host).dev, MSG_EMPTY_FIFO_READ.as_ptr() as *const c_char);
    }
    if irq_status & SPDIF_IRQ_FIFOFULL != 0 {
        dev_err((*host).dev, MSG_FIFO_FULL.as_ptr() as *const c_char);
    }
    if irq_status & SPDIF_IRQ_OUTOFRANGE != 0 {
        dev_err((*host).dev, MSG_OUT_OF_RANGE.as_ptr() as *const c_char);
    }

    writel(0, io_addr((*host).io_base, SPDIF_IN_IRQ));

    IRQ_HANDLED
}

unsafe extern "C" fn spdif_in_probe(pdev: *mut platform_device) -> c_int {
    let mut host: *mut spdif_in_dev;
    let mut pdata: *mut spear_spdif_platform_data;
    let mut res_fifo: *mut resource;
    let io_base: *mut c_void;
    let mut ret: c_int;

    io_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(io_base) {
        return PTR_ERR(io_base);
    }

    res_fifo = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if res_fifo.is_null() {
        return -EINVAL;
    }

    host = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<spdif_in_dev>(),
        GFP_KERNEL,
    ) as *mut spdif_in_dev;
    if host.is_null() {
        return -ENOMEM;
    }

    (*host).io_base = io_base;
    (*host).irq = platform_get_irq(pdev, 0);
    if (*host).irq < 0 {
        return (*host).irq;
    }

    (*host).clk = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*host).clk as *const c_void) {
        return PTR_ERR((*host).clk as *const c_void);
    }

    pdata = dev_get_platdata(&mut (*pdev).dev) as *mut spear_spdif_platform_data;

    if pdata.is_null() {
        return -EINVAL;
    }

    (*host).dma_params.data = (*pdata).dma_params;
    (*host).dma_params.addr = (*res_fifo).start;
    (*host).dma_params.max_burst = 16;
    (*host).dma_params.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*host).reset_perip = (*pdata).reset_perip;

    (*host).dev = &mut (*pdev).dev;
    dev_set_drvdata(&mut (*pdev).dev, host as *mut c_void);

    ret = devm_request_irq(
        &mut (*pdev).dev,
        (*host).irq,
        spdif_in_irq,
        0,
        SPDIF_IN_IRQ_NAME.as_ptr() as *const c_char,
        host as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &spdif_in_component,
        &mut spdif_in_dai,
        1,
    );
    if ret != 0 {
        return ret;
    }

    devm_spear_pcm_platform_register(&mut (*pdev).dev, &mut (*host).config, (*pdata).filter)
}

static mut spdif_in_driver: platform_driver = platform_driver {
    probe: Some(spdif_in_probe),
    driver: platform_driver_driver {
        name: SPDIF_IN_DRIVER_NAME.as_ptr() as *const c_char,
    },
};

// Original C registration and metadata:
// module_platform_driver(spdif_in_driver);
// MODULE_AUTHOR("Vipin Kumar <vipin.kumar@st.com>");
// MODULE_DESCRIPTION("SPEAr SPDIF IN SoC Interface");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:spdif_in");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
