// SPDX-License-Identifier: GPL-2.0-only
/*
 * IMG parallel output controller driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null_mut};

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const IMG_PRL_OUT_TX_FIFO: u32 = 0;

const IMG_PRL_OUT_CTL: u32 = 0x4;
const IMG_PRL_OUT_CTL_CH_MASK: u32 = BIT(4);
const IMG_PRL_OUT_CTL_PACKH_MASK: u32 = BIT(3);
const IMG_PRL_OUT_CTL_EDGE_MASK: u32 = BIT(2);
const IMG_PRL_OUT_CTL_ME_MASK: u32 = BIT(1);
const IMG_PRL_OUT_CTL_SRST_MASK: u32 = BIT(0);

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;

const SNDRV_PCM_FORMAT_S32_LE: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 1;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 1;

const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 1;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
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
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: usize,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int,
    >,
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
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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
pub struct img_prl_out {
    pub base: *mut c_void,
    pub clk_sys: *mut clk,
    pub clk_ref: *mut clk,
    pub dma_data: snd_dmaengine_dai_dma_data,
    pub dev: *mut device,
    pub rst: *mut reset_control,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *mut c_void, flags: c_uint)
        -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_disable(dev: *mut device);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

unsafe extern "C" fn img_prl_out_suspend(dev: *mut device) -> c_int {
    let prl = dev_get_drvdata(dev) as *mut img_prl_out;

    clk_disable_unprepare((*prl).clk_ref);

    0
}

unsafe extern "C" fn img_prl_out_resume(dev: *mut device) -> c_int {
    let prl = dev_get_drvdata(dev) as *mut img_prl_out;
    let ret: c_int;

    ret = clk_prepare_enable((*prl).clk_ref);
    if ret != 0 {
        dev_err(dev, c"clk_enable failed: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

#[inline]
unsafe fn img_prl_out_writel(prl: *mut img_prl_out, val: u32, reg: u32) {
    writel(val, ((*prl).base as *mut u8).add(reg as usize) as *mut c_void);
}

#[inline]
unsafe fn img_prl_out_readl(prl: *mut img_prl_out, reg: u32) -> u32 {
    readl(((*prl).base as *mut u8).add(reg as usize) as *mut c_void)
}

unsafe fn img_prl_out_reset(prl: *mut img_prl_out) {
    let ctl: u32;

    ctl = img_prl_out_readl(prl, IMG_PRL_OUT_CTL) & !IMG_PRL_OUT_CTL_ME_MASK;

    reset_control_assert((*prl).rst);
    reset_control_deassert((*prl).rst);

    img_prl_out_writel(prl, ctl, IMG_PRL_OUT_CTL);
}

unsafe extern "C" fn img_prl_out_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let prl = snd_soc_dai_get_drvdata(dai) as *mut img_prl_out;
    let mut reg: u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            reg = img_prl_out_readl(prl, IMG_PRL_OUT_CTL);
            reg |= IMG_PRL_OUT_CTL_ME_MASK;
            img_prl_out_writel(prl, reg, IMG_PRL_OUT_CTL);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            img_prl_out_reset(prl);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn img_prl_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let prl = snd_soc_dai_get_drvdata(dai) as *mut img_prl_out;
    let rate: c_uint;
    let channels: c_uint;
    let mut reg: u32;
    let mut control_set: u32 = 0;

    rate = params_rate(params);
    channels = params_channels(params);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S32_LE => {
            control_set |= IMG_PRL_OUT_CTL_PACKH_MASK;
        }
        SNDRV_PCM_FORMAT_S24_LE => {}
        _ => return -EINVAL,
    }

    if channels != 2 {
        return -EINVAL;
    }

    clk_set_rate((*prl).clk_ref, rate.wrapping_mul(256));

    reg = img_prl_out_readl(prl, IMG_PRL_OUT_CTL);
    reg = (reg & !IMG_PRL_OUT_CTL_PACKH_MASK) | control_set;
    img_prl_out_writel(prl, reg, IMG_PRL_OUT_CTL);

    0
}

unsafe extern "C" fn img_prl_out_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let prl = snd_soc_dai_get_drvdata(dai) as *mut img_prl_out;
    let mut reg: u32;
    let mut control_set: u32 = 0;
    let ret: c_int;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => {
            control_set |= IMG_PRL_OUT_CTL_EDGE_MASK;
        }
        _ => return -EINVAL,
    }

    ret = pm_runtime_resume_and_get((*prl).dev);
    if ret < 0 {
        return ret;
    }

    reg = img_prl_out_readl(prl, IMG_PRL_OUT_CTL);
    reg = (reg & !IMG_PRL_OUT_CTL_EDGE_MASK) | control_set;
    img_prl_out_writel(prl, reg, IMG_PRL_OUT_CTL);
    pm_runtime_put((*prl).dev);

    0
}

unsafe extern "C" fn img_prl_out_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let prl = snd_soc_dai_get_drvdata(dai) as *mut img_prl_out;

    snd_soc_dai_init_dma_data(dai, addr_of_mut!((*prl).dma_data), null_mut());

    0
}

static img_prl_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(img_prl_out_dai_probe),
    trigger: Some(img_prl_out_trigger),
    hw_params: Some(img_prl_out_hw_params),
    set_fmt: Some(img_prl_out_set_fmt),
};

static mut img_prl_out_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &img_prl_out_dai_ops,
};

static img_prl_out_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"img-prl-out".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn img_prl_out_probe(pdev: *mut platform_device) -> c_int {
    let prl: *mut img_prl_out;
    let mut res: *mut resource = null_mut();
    let base: *mut c_void;
    let mut ret: c_int;
    let dev = addr_of_mut!((*pdev).dev);

    prl = devm_kzalloc(dev, size_of::<img_prl_out>(), GFP_KERNEL) as *mut img_prl_out;
    if prl.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, prl as *mut c_void);

    (*prl).dev = dev;

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*prl).base = base;

    (*prl).rst = devm_reset_control_get_exclusive(dev, c"rst".as_ptr());
    if IS_ERR((*prl).rst as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*prl).rst as *const c_void),
            c"No top level reset found\n".as_ptr(),
        );
    }

    (*prl).clk_sys = devm_clk_get(dev, c"sys".as_ptr());
    if IS_ERR((*prl).clk_sys as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*prl).clk_sys as *const c_void),
            c"Failed to acquire clock 'sys'\n".as_ptr(),
        );
    }

    (*prl).clk_ref = devm_clk_get(dev, c"ref".as_ptr());
    if IS_ERR((*prl).clk_ref as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*prl).clk_ref as *const c_void),
            c"Failed to acquire clock 'ref'\n".as_ptr(),
        );
    }

    ret = clk_prepare_enable((*prl).clk_sys);
    if ret != 0 {
        return ret;
    }

    img_prl_out_writel(prl, IMG_PRL_OUT_CTL_EDGE_MASK, IMG_PRL_OUT_CTL);
    img_prl_out_reset(prl);

    pm_runtime_enable(dev);
    if !pm_runtime_enabled(dev) {
        ret = img_prl_out_resume(dev);
        if ret != 0 {
            pm_runtime_disable(dev);
            clk_disable_unprepare((*prl).clk_sys);
            return ret;
        }
    }

    (*prl).dma_data.addr = (*res).start + IMG_PRL_OUT_TX_FIFO as usize;
    (*prl).dma_data.addr_width = 4;
    (*prl).dma_data.maxburst = 4;

    ret = devm_snd_soc_register_component(
        dev,
        &img_prl_out_component,
        addr_of_mut!(img_prl_out_dai),
        1,
    );
    if ret != 0 {
        if !pm_runtime_status_suspended(dev) {
            img_prl_out_suspend(dev);
        }
        pm_runtime_disable(dev);
        clk_disable_unprepare((*prl).clk_sys);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(dev, null_mut(), 0);
    if ret != 0 {
        if !pm_runtime_status_suspended(dev) {
            img_prl_out_suspend(dev);
        }
        pm_runtime_disable(dev);
        clk_disable_unprepare((*prl).clk_sys);
        return ret;
    }

    0
}

unsafe extern "C" fn img_prl_out_dev_remove(pdev: *mut platform_device) {
    let prl = platform_get_drvdata(pdev) as *mut img_prl_out;
    let dev = addr_of_mut!((*pdev).dev);

    pm_runtime_disable(dev);
    if !pm_runtime_status_suspended(dev) {
        img_prl_out_suspend(dev);
    }

    clk_disable_unprepare((*prl).clk_sys);
}

static img_prl_out_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"img,parallel-out".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, img_prl_out_of_match); */

static img_prl_out_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(img_prl_out_suspend),
    runtime_resume: Some(img_prl_out_resume),
};

static mut img_prl_out_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"img-parallel-out".as_ptr(),
        of_match_table: img_prl_out_of_match.as_ptr(),
        pm: &img_prl_out_pm_ops,
    },
    probe: Some(img_prl_out_probe),
    remove: Some(img_prl_out_dev_remove),
};

/* module_platform_driver(img_prl_out_driver); */

/* MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>"); */
/* MODULE_DESCRIPTION("IMG Parallel Output Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
