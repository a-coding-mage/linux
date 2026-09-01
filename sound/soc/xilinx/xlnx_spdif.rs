// SPDX-License-Identifier: GPL-2.0
//
// Xilinx ASoC SPDIF audio support
//
// Copyright (C) 2018 Xilinx, Inc.
//
// Author: Maruthi Srinivas Bayyavarapu <maruthis@xilinx.com>
//

// Linux kernel headers and bindings required:
// linux/clk.h, linux/io.h, linux/module.h, linux/property.h, linux/platform_device.h
// sound/pcm_params.h, sound/soc.h

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;

const XLNX_SPDIF_RATES: u32 =
    (SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 |
     SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 |
     SNDRV_PCM_RATE_192000);

const XLNX_SPDIF_FORMATS: u32 = (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE);

const XSPDIF_IRQ_STS_REG: usize = 0x20;
const XSPDIF_IRQ_ENABLE_REG: usize = 0x28;
const XSPDIF_SOFT_RESET_REG: usize = 0x40;
const XSPDIF_CONTROL_REG: usize = 0x44;
const XSPDIF_CHAN_0_STS_REG: usize = 0x4C;
const XSPDIF_GLOBAL_IRQ_ENABLE_REG: usize = 0x1C;
const XSPDIF_CH_A_USER_DATA_REG_0: usize = 0x64;

const XSPDIF_CORE_ENABLE_MASK: u32 = 1 << 0;
const XSPDIF_FIFO_FLUSH_MASK: u32 = 1 << 1;
const XSPDIF_CH_STS_MASK: u32 = 1 << 5;
const XSPDIF_GLOBAL_IRQ_ENABLE: u32 = 1 << 31;
const XSPDIF_CLOCK_CONFIG_BITS_MASK: u32 = 0xF << 2;
const XSPDIF_CLOCK_CONFIG_BITS_SHIFT: u32 = 2;
const XSPDIF_SOFT_RESET_VALUE: u32 = 0xA;

const MAX_CHANNELS: u32 = 2;
const AES_SAMPLE_WIDTH: u32 = 32;
const CH_STATUS_UPDATE_TIMEOUT: u32 = 40;

#[repr(C)]
pub struct spdif_dev_data {
    pub mode: u32,
    pub aclk: u32,
    pub rx_chsts_updated: bool,
    pub base: *mut c_void,
    pub axi_clk: *mut c_void,
    pub chsts_q: c_void,
}

// External kernel functions and types - provided by build environment
extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn dev_get_drvdata(dev: *mut c_void) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut c_void, data: *mut c_void);
    fn wait_event_interruptible_timeout(
        wq: *mut c_void,
        condition: bool,
        timeout: c_ulong,
    ) -> c_long;
    fn wake_up_interruptible(wq: *mut c_void);
    fn msecs_to_jiffies(msecs: u32) -> c_ulong;
}

type c_ulong = usize;
type c_long = isize;

const IRQ_NONE: u32 = 0;
const IRQ_HANDLED: u32 = 1;

unsafe extern "C" fn xlnx_spdifrx_irq_handler(irq: c_int, arg: *mut c_void) -> u32 {
    let mut val: u32;
    let ctx = arg as *mut spdif_dev_data;

    val = readl((*ctx).base.add(XSPDIF_IRQ_STS_REG) as *const c_void);
    if val & XSPDIF_CH_STS_MASK != 0 {
        writel(
            val & XSPDIF_CH_STS_MASK,
            (*ctx).base.add(XSPDIF_IRQ_STS_REG) as *mut c_void,
        );
        val = readl((*ctx).base.add(XSPDIF_IRQ_ENABLE_REG) as *const c_void);
        writel(
            val & !XSPDIF_CH_STS_MASK,
            (*ctx).base.add(XSPDIF_IRQ_ENABLE_REG) as *mut c_void,
        );

        (*ctx).rx_chsts_updated = true;
        wake_up_interruptible(&mut (*ctx).chsts_q as *mut c_void);
        return IRQ_HANDLED;
    }

    IRQ_NONE
}

unsafe extern "C" fn xlnx_spdif_startup(substream: *mut c_void, dai: *mut c_void) -> c_int {
    let mut val: u32;
    let ctx = dev_get_drvdata((*(dai as *mut snd_soc_dai)).dev) as *mut spdif_dev_data;

    val = readl((*ctx).base.add(XSPDIF_CONTROL_REG) as *const c_void);
    val |= XSPDIF_FIFO_FLUSH_MASK;
    writel(val, (*ctx).base.add(XSPDIF_CONTROL_REG) as *mut c_void);

    let substream_struct = substream as *mut snd_pcm_substream;
    if (*substream_struct).stream == SNDRV_PCM_STREAM_CAPTURE {
        writel(
            XSPDIF_CH_STS_MASK,
            (*ctx).base.add(XSPDIF_IRQ_ENABLE_REG) as *mut c_void,
        );
        writel(
            XSPDIF_GLOBAL_IRQ_ENABLE,
            (*ctx).base.add(XSPDIF_GLOBAL_IRQ_ENABLE_REG) as *mut c_void,
        );
    }

    0
}

unsafe extern "C" fn xlnx_spdif_shutdown(substream: *mut c_void, dai: *mut c_void) {
    let ctx = dev_get_drvdata((*(dai as *mut snd_soc_dai)).dev) as *mut spdif_dev_data;

    writel(
        XSPDIF_SOFT_RESET_VALUE,
        (*ctx).base.add(XSPDIF_SOFT_RESET_REG) as *mut c_void,
    );
}

unsafe extern "C" fn xlnx_spdif_hw_params(
    substream: *mut c_void,
    params: *mut c_void,
    dai: *mut c_void,
) -> c_int {
    let mut val: u32;
    let mut clk_div: u32;
    let mut clk_cfg: u32;
    let ctx = dev_get_drvdata((*(dai as *mut snd_soc_dai)).dev) as *mut spdif_dev_data;

    clk_div = div_round_closest(
        (*ctx).aclk,
        MAX_CHANNELS.wrapping_mul(AES_SAMPLE_WIDTH).wrapping_mul(params_rate(params as *mut snd_pcm_hw_params)),
    );

    clk_cfg = match clk_div {
        4 => 0,
        8 => 1,
        16 => 2,
        24 => 3,
        32 => 4,
        48 => 5,
        64 => 6,
        _ => return -22, // -EINVAL
    };

    val = readl((*ctx).base.add(XSPDIF_CONTROL_REG) as *const c_void);
    val &= !XSPDIF_CLOCK_CONFIG_BITS_MASK;
    val |= clk_cfg << XSPDIF_CLOCK_CONFIG_BITS_SHIFT;
    writel(val, (*ctx).base.add(XSPDIF_CONTROL_REG) as *mut c_void);

    0
}

unsafe extern "C" fn rx_stream_detect(dai: *mut c_void) -> c_int {
    let mut err: c_long;
    let ctx = dev_get_drvdata((*(dai as *mut snd_soc_dai)).dev) as *mut spdif_dev_data;
    let jiffies = msecs_to_jiffies(CH_STATUS_UPDATE_TIMEOUT);

    err = wait_event_interruptible_timeout(&mut (*ctx).chsts_q as *mut c_void, (*ctx).rx_chsts_updated, jiffies);
    if err == 0 {
        // dev_err(dai->dev, "No streaming audio detected!\n");
        return -22; // -EINVAL
    }
    (*ctx).rx_chsts_updated = false;

    0
}

unsafe extern "C" fn xlnx_spdif_trigger(
    substream: *mut c_void,
    cmd: c_int,
    dai: *mut c_void,
) -> c_int {
    let mut val: u32;
    let mut ret: c_int = 0;
    let ctx = dev_get_drvdata((*(dai as *mut snd_soc_dai)).dev) as *mut spdif_dev_data;

    val = readl((*ctx).base.add(XSPDIF_CONTROL_REG) as *const c_void);
    match cmd {
        1 | 5 | 7 => { // SNDRV_PCM_TRIGGER_START | RESUME | PAUSE_RELEASE
            val |= XSPDIF_CORE_ENABLE_MASK;
            writel(val, (*ctx).base.add(XSPDIF_CONTROL_REG) as *mut c_void);
            let substream_struct = substream as *mut snd_pcm_substream;
            if (*substream_struct).stream == SNDRV_PCM_STREAM_CAPTURE {
                ret = rx_stream_detect(dai);
            }
        }
        0 | 6 | 8 => { // SNDRV_PCM_TRIGGER_STOP | SUSPEND | PAUSE_PUSH
            val &= !XSPDIF_CORE_ENABLE_MASK;
            writel(val, (*ctx).base.add(XSPDIF_CONTROL_REG) as *mut c_void);
        }
        _ => {
            ret = -22; // -EINVAL
        }
    }

    ret
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    pub trigger: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>,
}

static xlnx_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(xlnx_spdif_startup),
    shutdown: Some(xlnx_spdif_shutdown),
    trigger: Some(xlnx_spdif_trigger),
    hw_params: Some(xlnx_spdif_hw_params),
};

#[repr(C)]
pub struct snd_pcm_chmap_elem {
    pub channels: u32,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratdens {
    pub nrats: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params_sg {
    pub elems: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params_interval {
    pub min: u32,
    pub max: u32,
    pub openmin: c_uint,
    pub openmax: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub _private: [u8; 1024],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: u32,
    pub _private: [u8; 1024],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut c_void,
    pub _private: [u8; 2048],
}

const SNDRV_PCM_STREAM_CAPTURE: u32 = 1;

// External kernel functions for this file
extern "C" {
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: u32) -> *mut c_void;
    fn devm_clk_get_enabled(dev: *mut c_void, id: *const u8) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut c_void, index: c_uint) -> *mut c_void;
    fn device_property_read_u32(dev: *mut c_void, propname: *const u8, val: *mut u32) -> c_int;
    fn platform_get_irq(pdev: *mut c_void, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut c_void,
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> u32,
        irqflags: c_ulong,
        devname: *const u8,
        dev_id: *mut c_void,
    ) -> c_int;
    fn init_waitqueue_head(q: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut c_void,
        component_driver: *const c_void,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
    fn dev_info(dev: *mut c_void, fmt: *const u8, ...);
    fn params_rate(p: *mut snd_pcm_hw_params) -> u32;
    fn div_round_closest(x: u32, divisor: u32) -> u32;
}

const GFP_KERNEL: u32 = 0xd0;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const u8,
    pub legacy_dai_naming: c_int,
}

static xlnx_spdif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"xlnx-spdif\0" as *const u8,
    legacy_dai_naming: 1,
};

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const u8,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

static mut xlnx_spdif_tx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"xlnx_spdif_tx\0" as *const u8,
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: XLNX_SPDIF_RATES,
        formats: XLNX_SPDIF_FORMATS as u64,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
    },
    ops: &xlnx_spdif_dai_ops,
};

static mut xlnx_spdif_rx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"xlnx_spdif_rx\0" as *const u8,
    playback: snd_soc_pcm_stream {
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: XLNX_SPDIF_RATES,
        formats: XLNX_SPDIF_FORMATS as u64,
    },
    ops: &xlnx_spdif_dai_ops,
};

#[repr(C)]
pub struct of_device_id {
    pub compatible: [u8; 128],
    pub data: *const c_void,
}

static xlnx_spdif_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: *b"xlnx,spdif-2.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        data: ptr::null(),
    },
    of_device_id {
        compatible: [0; 128],
        data: ptr::null(),
    },
];

#[repr(C)]
pub struct platform_device {
    pub dev: c_void,
}

unsafe extern "C" fn xlnx_spdif_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut dai_drv: *mut snd_soc_dai_driver;
    let mut ctx: *mut spdif_dev_data;

    let dev = &mut (*pdev).dev as *mut c_void;

    ctx = devm_kzalloc(dev, core::mem::size_of::<spdif_dev_data>(), GFP_KERNEL) as *mut spdif_dev_data;
    if ctx.is_null() {
        return -12; // -ENOMEM
    }

    (*ctx).axi_clk = devm_clk_get_enabled(dev, b"s_axi_aclk\0" as *const u8);
    if IS_ERR((*ctx).axi_clk) {
        // return dev_err_probe(dev, PTR_ERR(ctx->axi_clk), "failed to get s_axi_aclk\n");
        return PTR_ERR((*ctx).axi_clk);
    }

    (*ctx).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*ctx).base) {
        return PTR_ERR((*ctx).base);
    }

    ret = device_property_read_u32(dev, b"xlnx,spdif-mode\0" as *const u8, &mut (*ctx).mode);
    if ret < 0 {
        // return dev_err_probe(dev, ret, "cannot get SPDIF mode\n");
        return ret;
    }

    if (*ctx).mode != 0 {
        dai_drv = &mut xlnx_spdif_tx_dai;
    } else {
        ret = platform_get_irq(pdev as *mut c_void, 0);
        if ret < 0 {
            return ret;
        }

        ret = devm_request_irq(
            dev,
            ret as c_uint,
            xlnx_spdifrx_irq_handler,
            0,
            b"XLNX_SPDIF_RX\0" as *const u8,
            ctx as *mut c_void,
        );
        if ret != 0 {
            return ret;
        }

        init_waitqueue_head(&mut (*ctx).chsts_q as *mut c_void);
        dai_drv = &mut xlnx_spdif_rx_dai;
    }

    ret = device_property_read_u32(dev, b"xlnx,aud_clk_i\0" as *const u8, &mut (*ctx).aclk);
    if ret < 0 {
        // return dev_err_probe(dev, ret, "cannot get aud_clk_i value\n");
        return ret;
    }

    dev_set_drvdata(dev, ctx as *mut c_void);

    ret = devm_snd_soc_register_component(
        dev,
        &xlnx_spdif_component as *const _ as *const c_void,
        dai_drv as *const _ as *const c_void,
        1,
    );
    if ret != 0 {
        return ret;
    }

    writel(
        XSPDIF_SOFT_RESET_VALUE,
        (*ctx).base.add(XSPDIF_SOFT_RESET_REG) as *mut c_void,
    );
    dev_info(dev, b"%s DAI registered\n\0" as *const u8, (*dai_drv).name);

    0
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

static xlnx_spdif_driver: platform_driver = platform_driver {
    probe: Some(xlnx_spdif_probe),
    driver: device_driver {
        name: b"xlnx-spdif\0" as *const u8,
        of_match_table: &xlnx_spdif_of_match as *const _,
    },
};

// module_platform_driver macro expansion - in Rust, this would be handled by a kernel crate
// The probe function is registered via the platform_driver structure above

// Module metadata - typically handled by kernel module system
const _MODULE_AUTHOR: &[u8] = b"Maruthi Srinivas Bayyavarapu <maruthis@xilinx.com>\0";
const _MODULE_DESCRIPTION: &[u8] = b"XILINX SPDIF driver\0";
const _MODULE_LICENSE: &[u8] = b"GPL v2\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
