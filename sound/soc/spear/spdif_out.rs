// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC SPDIF Out Audio Layer for spear processors
 *
 * Copyright (C) 2012 ST Microelectronics
 * Vipin Kumar <vipin.kumar@st.com>
 */

// C dependencies translated as external declarations/uses:
// <linux/clk.h>, <linux/delay.h>, <linux/device.h>, <linux/kernel.h>,
// <linux/init.h>, <linux/io.h>, <linux/ioport.h>, <linux/module.h>,
// <linux/platform_device.h>, <sound/dmaengine_pcm.h>, <sound/soc.h>,
// <sound/spear_dma.h>, <sound/spear_spdif.h>, "spdif_out_regs.h",
// and "spear_pcm.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type u32 = u32;
type bool_t = u32;

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
    pub start: c_ulong,
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct spear_dma_data {
    pub data: *mut c_void,
    pub addr: c_ulong,
    pub max_burst: u32,
    pub addr_width: u32,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub filter_data: *mut spear_dma_data,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spear_spdif_platform_data {
    pub dma_params: *mut c_void,
    pub filter: *mut c_void,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub index: u32,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub no_capture_mute: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
struct spdif_out_params {
    rate: u32,
    core_freq: u32,
    mute: u32,
}

#[repr(C)]
struct spdif_out_dev {
    clk: *mut clk,
    dma_params: spear_dma_data,
    saved_params: spdif_out_params,
    running: u32,
    io_base: *mut c_void,
    dma_params_tx: snd_dmaengine_dai_dma_data,
    config: snd_dmaengine_pcm_config,
}

unsafe extern "C" {
    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn mdelay(msecs: c_ulong);
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: u32) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_dma_data_set_playback(dai: *mut snd_soc_dai, data: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_add_dai_controls(
        dai: *mut snd_soc_dai,
        controls: *const snd_kcontrol_new,
        num_controls: c_int,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_int,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_spear_pcm_platform_register(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        filter: *mut c_void,
    ) -> c_int;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
}

// Constants/macros supplied by included Linux, ALSA, SPEAr, and register headers.
unsafe extern "C" {
    static SPDIF_OUT_RESET: u32;
    static SPDIF_OUT_SOFT_RST: usize;
    static SPDIF_OUT_FDMA_TRIG_16: u32;
    static SPDIF_OUT_MEMFMT_16_16: u32;
    static SPDIF_OUT_VALID_HW: u32;
    static SPDIF_OUT_USER_HW: u32;
    static SPDIF_OUT_CHNLSTA_HW: u32;
    static SPDIF_OUT_PARITY_HW: u32;
    static SPDIF_OUT_CFG: usize;
    static SPDIF_OUT_INT_STA_CLR: usize;
    static SPDIF_OUT_INT_EN_CLR: usize;
    static SPDIF_OUT_CTRL: usize;
    static SPDIF_DIVIDER_MASK: u32;
    static SPDIF_DIVIDER_SHIFT: u32;
    static SPDIF_OPMODE_MASK: u32;
    static SPDIF_OPMODE_AUD_DATA: u32;
    static SPDIF_STATE_NORMAL: u32;
    static SPDIF_OPMODE_MUTE_PCM: u32;
    static SPDIF_OPMODE_OFF: u32;
    static SPDIF_OUT_FIFO_DATA: c_ulong;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_RATE_192000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: u32;
    static GFP_KERNEL: u32;
    static EINVAL: c_int;
    static ENOMEM: c_int;
}

unsafe fn io_offset(base: *mut c_void, offset: usize) -> *mut c_void {
    (base as *mut u8).add(offset) as *mut c_void
}

unsafe fn DIV_ROUND_CLOSEST(x: c_ulong, divisor: u32) -> u32 {
    (((x as u64) + ((divisor as u64) / 2)) / (divisor as u64)) as u32
}

unsafe extern "C" fn spdif_out_configure(host: *mut spdif_out_dev) {
    unsafe {
        writel(SPDIF_OUT_RESET, io_offset((*host).io_base, SPDIF_OUT_SOFT_RST));
        mdelay(1);
        writel(
            readl(io_offset((*host).io_base, SPDIF_OUT_SOFT_RST)) & !SPDIF_OUT_RESET,
            io_offset((*host).io_base, SPDIF_OUT_SOFT_RST),
        );

        writel(
            SPDIF_OUT_FDMA_TRIG_16
                | SPDIF_OUT_MEMFMT_16_16
                | SPDIF_OUT_VALID_HW
                | SPDIF_OUT_USER_HW
                | SPDIF_OUT_CHNLSTA_HW
                | SPDIF_OUT_PARITY_HW,
            io_offset((*host).io_base, SPDIF_OUT_CFG),
        );

        writel(0x7F, io_offset((*host).io_base, SPDIF_OUT_INT_STA_CLR));
        writel(0x7F, io_offset((*host).io_base, SPDIF_OUT_INT_EN_CLR));
    }
}

unsafe extern "C" fn spdif_out_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let host = snd_soc_dai_get_drvdata(cpu_dai) as *mut spdif_out_dev;
        let ret: c_int;

        if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
            return -EINVAL;
        }

        ret = clk_enable((*host).clk);
        if ret != 0 {
            return ret;
        }

        (*host).running = 1 as bool_t;
        spdif_out_configure(host);

        0
    }
}

unsafe extern "C" fn spdif_out_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    unsafe {
        let host = snd_soc_dai_get_drvdata(dai) as *mut spdif_out_dev;

        if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
            return;
        }

        clk_disable((*host).clk);
        (*host).running = 0 as bool_t;
    }
}

unsafe extern "C" fn spdif_out_clock(host: *mut spdif_out_dev, core_freq: u32, rate: u32) {
    unsafe {
        let divider: u32;
        let mut ctrl: u32;

        clk_set_rate((*host).clk, core_freq);
        divider = DIV_ROUND_CLOSEST(clk_get_rate((*host).clk), rate.wrapping_mul(128));

        ctrl = readl(io_offset((*host).io_base, SPDIF_OUT_CTRL));
        ctrl &= !SPDIF_DIVIDER_MASK;
        ctrl |= (divider << SPDIF_DIVIDER_SHIFT) & SPDIF_DIVIDER_MASK;
        writel(ctrl, io_offset((*host).io_base, SPDIF_OUT_CTRL));
    }
}

unsafe extern "C" fn spdif_out_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let host = snd_soc_dai_get_drvdata(dai) as *mut spdif_out_dev;
        let rate: u32;
        let core_freq: u32;

        if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
            return -EINVAL;
        }

        rate = params_rate(params);

        match rate {
            8000 | 16000 | 32000 | 64000 => {
                /*
                 * The clock is multiplied by 10 to bring it to feasible range
                 * of frequencies for sscg
                 */
                core_freq = 64000 * 128 * 10; /* 81.92 MHz */
            }
            5512 | 11025 | 22050 | 44100 | 88200 | 176400 => {
                core_freq = 176400 * 128; /* 22.5792 MHz */
            }
            48000 | 96000 | 192000 | _ => {
                core_freq = 192000 * 128; /* 24.576 MHz */
            }
        }

        spdif_out_clock(host, core_freq, rate);
        (*host).saved_params.core_freq = core_freq;
        (*host).saved_params.rate = rate;

        0
    }
}

unsafe extern "C" fn spdif_out_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let host = snd_soc_dai_get_drvdata(dai) as *mut spdif_out_dev;
        let mut ctrl: u32;
        let mut ret: c_int = 0;

        if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
            return -EINVAL;
        }

        if cmd == SNDRV_PCM_TRIGGER_START
            || cmd == SNDRV_PCM_TRIGGER_RESUME
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        {
            ctrl = readl(io_offset((*host).io_base, SPDIF_OUT_CTRL));
            ctrl &= !SPDIF_OPMODE_MASK;
            if (*host).saved_params.mute == 0 {
                ctrl |= SPDIF_OPMODE_AUD_DATA | SPDIF_STATE_NORMAL;
            } else {
                ctrl |= SPDIF_OPMODE_MUTE_PCM;
            }
            writel(ctrl, io_offset((*host).io_base, SPDIF_OUT_CTRL));
        } else if cmd == SNDRV_PCM_TRIGGER_STOP
            || cmd == SNDRV_PCM_TRIGGER_SUSPEND
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        {
            ctrl = readl(io_offset((*host).io_base, SPDIF_OUT_CTRL));
            ctrl &= !SPDIF_OPMODE_MASK;
            ctrl |= SPDIF_OPMODE_OFF;
            writel(ctrl, io_offset((*host).io_base, SPDIF_OUT_CTRL));
        } else {
            ret = -EINVAL;
        }

        ret
    }
}

unsafe extern "C" fn spdif_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    unsafe {
        let host = snd_soc_dai_get_drvdata(dai) as *mut spdif_out_dev;
        let mut val: u32;

        (*host).saved_params.mute = mute as u32;
        val = readl(io_offset((*host).io_base, SPDIF_OUT_CTRL));
        val &= !SPDIF_OPMODE_MASK;

        if mute != 0 {
            val |= SPDIF_OPMODE_MUTE_PCM;
        } else {
            if (*host).running != 0 {
                val |= SPDIF_OPMODE_AUD_DATA | SPDIF_STATE_NORMAL;
            } else {
                val |= SPDIF_OPMODE_OFF;
            }
        }

        writel(val, io_offset((*host).io_base, SPDIF_OUT_CTRL));
        0
    }
}

unsafe extern "C" fn spdif_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
        let host = snd_soc_dai_get_drvdata(cpu_dai) as *mut spdif_out_dev;

        (*ucontrol).value.integer.value[0] = (*host).saved_params.mute as i64;
        0
    }
}

unsafe extern "C" fn spdif_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
        let host = snd_soc_dai_get_drvdata(cpu_dai) as *mut spdif_out_dev;

        if (*host).saved_params.mute == (*ucontrol).value.integer.value[0] as u32 {
            return 0;
        }

        spdif_mute(
            cpu_dai,
            (*ucontrol).value.integer.value[0] as c_int,
            SNDRV_PCM_STREAM_PLAYBACK,
        );

        1
    }
}

static spdif_out_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: b"IEC958 Playback Switch\0".as_ptr() as *const c_char,
    index: 0,
    get: Some(spdif_mute_get),
    put: Some(spdif_mute_put),
}];

unsafe extern "C" fn spdif_soc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let host = snd_soc_dai_get_drvdata(dai) as *mut spdif_out_dev;

        (*host).dma_params_tx.filter_data = &mut (*host).dma_params;

        snd_soc_dai_dma_data_set_playback(dai, &mut (*host).dma_params_tx);

        snd_soc_add_dai_controls(dai, spdif_out_controls.as_ptr(), spdif_out_controls.len() as c_int)
    }
}

static spdif_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(spdif_soc_dai_probe),
    mute_stream: Some(spdif_mute),
    startup: Some(spdif_out_startup),
    shutdown: Some(spdif_out_shutdown),
    trigger: Some(spdif_out_trigger),
    hw_params: Some(spdif_out_hw_params),
    no_capture_mute: 1,
};

// The following static initializers depend on C preprocessor constants supplied
// by ALSA headers. They are preserved as runtime-initialized equivalents where
// direct Rust const initialization is not file-local.
static mut spdif_out_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: 0,
        formats: 0,
    },
    ops: &spdif_out_dai_ops,
};

static spdif_out_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"spdif-out\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn spdif_out_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut host: *mut spdif_out_dev;
        let pdata: *mut spear_spdif_platform_data;
        let mut res: *mut resource = ptr::null_mut();
        let ret: c_int;

        host = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<spdif_out_dev>(),
            GFP_KERNEL,
        ) as *mut spdif_out_dev;
        if host.is_null() {
            return -ENOMEM;
        }

        (*host).io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
        if IS_ERR((*host).io_base) != 0 {
            return PTR_ERR((*host).io_base);
        }

        (*host).clk = devm_clk_get(&mut (*pdev).dev, ptr::null());
        if IS_ERR((*host).clk as *const c_void) != 0 {
            return PTR_ERR((*host).clk as *const c_void);
        }

        pdata = dev_get_platdata(&mut (*pdev).dev) as *mut spear_spdif_platform_data;

        (*host).dma_params.data = (*pdata).dma_params;
        (*host).dma_params.addr = (*res).start + SPDIF_OUT_FIFO_DATA;
        (*host).dma_params.max_burst = 16;
        (*host).dma_params.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;

        dev_set_drvdata(&mut (*pdev).dev, host as *mut c_void);

        spdif_out_dai.playback.rates = SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_192000;
        spdif_out_dai.playback.formats = SNDRV_PCM_FMTBIT_S16_LE;

        ret = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &spdif_out_component,
            &mut spdif_out_dai,
            1,
        );
        if ret != 0 {
            return ret;
        }

        devm_spear_pcm_platform_register(&mut (*pdev).dev, &mut (*host).config, (*pdata).filter)
    }
}

// Original C condition: #ifdef CONFIG_PM
unsafe extern "C" fn spdif_out_suspend(dev: *mut device) -> c_int {
    unsafe {
        let pdev = to_platform_device(dev);
        let host = dev_get_drvdata(&mut (*pdev).dev) as *mut spdif_out_dev;

        if (*host).running != 0 {
            clk_disable((*host).clk);
        }

        0
    }
}

unsafe extern "C" fn spdif_out_resume(dev: *mut device) -> c_int {
    unsafe {
        let pdev = to_platform_device(dev);
        let host = dev_get_drvdata(&mut (*pdev).dev) as *mut spdif_out_dev;

        if (*host).running != 0 {
            clk_enable((*host).clk);
            spdif_out_configure(host);
            spdif_out_clock(host, (*host).saved_params.core_freq, (*host).saved_params.rate);
        }
        0
    }
}

static spdif_out_dev_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(spdif_out_suspend),
    resume: Some(spdif_out_resume),
};

// Original C fallback under #else: #define SPDIF_OUT_DEV_PM_OPS NULL
const SPDIF_OUT_DEV_PM_OPS: *const dev_pm_ops = &spdif_out_dev_pm_ops;

static spdif_out_driver: platform_driver = platform_driver {
    probe: Some(spdif_out_probe),
    driver: device_driver {
        name: b"spdif-out\0".as_ptr() as *const c_char,
        pm: SPDIF_OUT_DEV_PM_OPS,
    },
};

// module_platform_driver(spdif_out_driver);
// MODULE_AUTHOR("Vipin Kumar <vipin.kumar@st.com>");
// MODULE_DESCRIPTION("SPEAr SPDIF OUT SoC Interface");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:spdif_out");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
