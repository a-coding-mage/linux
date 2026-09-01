// SPDX-License-Identifier: GPL-2.0-only
/*
 * IMG SPDIF output controller driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type phys_addr_t = u64;
type snd_pcm_format_t = c_uint;

const IMG_SPDIF_OUT_TX_FIFO: u32 = 0x0;

const IMG_SPDIF_OUT_CTL: u32 = 0x4;
const IMG_SPDIF_OUT_CTL_FS_MASK: u32 = 1u32 << 4;
const IMG_SPDIF_OUT_CTL_CLK_MASK: u32 = 1u32 << 2;
const IMG_SPDIF_OUT_CTL_SRT_MASK: u32 = 1u32 << 0;

const IMG_SPDIF_OUT_CSL: u32 = 0x14;

const IMG_SPDIF_OUT_CSH_UV: u32 = 0x18;
const IMG_SPDIF_OUT_CSH_UV_CSH_SHIFT: u32 = 0;
const IMG_SPDIF_OUT_CSH_UV_CSH_MASK: u32 = 0xff;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 5;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1u64 << SNDRV_PCM_FORMAT_S32_LE;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    addr: phys_addr_t,
    addr_width: c_uint,
    maxburst: c_uint,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    start: phys_addr_t,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
}

#[repr(C)]
pub struct snd_aes_iec958 {
    status: [u8; 24],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    iec958: snd_aes_iec958,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    access: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct img_spdif_out {
    lock: spinlock_t,
    base: *mut c_void,
    clk_sys: *mut clk,
    clk_ref: *mut clk,
    dma_data: snd_dmaengine_dai_dma_data,
    dev: *mut device,
    rst: *mut reset_control,
    suspend_ctl: u32,
    suspend_csl: u32,
    suspend_csh: u32,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_add_dai_controls(
        dai: *mut snd_soc_dai,
        controls: *mut snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: c_long) -> c_long;
    fn clk_set_rate(clk: *mut clk, rate: c_long) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_long;
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn writel(val: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> usize;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_long;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
}

unsafe extern "C" fn img_spdif_out_runtime_suspend(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut img_spdif_out;

    clk_disable_unprepare((*spdif).clk_ref);
    clk_disable_unprepare((*spdif).clk_sys);

    0
}

unsafe extern "C" fn img_spdif_out_runtime_resume(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut img_spdif_out;
    let mut ret: c_int;

    ret = clk_prepare_enable((*spdif).clk_sys);
    if ret != 0 {
        dev_err(dev, c"clk_enable failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = clk_prepare_enable((*spdif).clk_ref);
    if ret != 0 {
        dev_err(dev, c"clk_enable failed: %d\n".as_ptr(), ret);
        clk_disable_unprepare((*spdif).clk_sys);
        return ret;
    }

    0
}

#[inline]
unsafe fn img_spdif_out_writel(spdif: *mut img_spdif_out, val: u32, reg: u32) {
    writel(val, ((*spdif).base as *mut u8).add(reg as usize) as *mut c_void);
}

#[inline]
unsafe fn img_spdif_out_readl(spdif: *mut img_spdif_out, reg: u32) -> u32 {
    readl(((*spdif).base as *mut u8).add(reg as usize) as *mut c_void)
}

unsafe fn img_spdif_out_reset(spdif: *mut img_spdif_out) {
    let ctl: u32;
    let status_low: u32;
    let status_high: u32;

    ctl = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL) & !IMG_SPDIF_OUT_CTL_SRT_MASK;
    status_low = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSL);
    status_high = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);

    reset_control_assert((*spdif).rst);
    reset_control_deassert((*spdif).rst);

    img_spdif_out_writel(spdif, ctl, IMG_SPDIF_OUT_CTL);
    img_spdif_out_writel(spdif, status_low, IMG_SPDIF_OUT_CSL);
    img_spdif_out_writel(spdif, status_high, IMG_SPDIF_OUT_CSH_UV);
}

unsafe extern "C" fn img_spdif_out_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;

    0
}

unsafe extern "C" fn img_spdif_out_get_status_mask(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.iec958.status[0] = 0xff;
    (*ucontrol).value.iec958.status[1] = 0xff;
    (*ucontrol).value.iec958.status[2] = 0xff;
    (*ucontrol).value.iec958.status[3] = 0xff;
    (*ucontrol).value.iec958.status[4] = 0xff;

    0
}

unsafe extern "C" fn img_spdif_out_get_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif = snd_soc_dai_get_drvdata(cpu_dai) as *mut img_spdif_out;
    let mut reg: u32;
    let flags = spin_lock_irqsave(&mut (*spdif).lock);

    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSL);
    (*ucontrol).value.iec958.status[0] = (reg & 0xff) as u8;
    (*ucontrol).value.iec958.status[1] = ((reg >> 8) & 0xff) as u8;
    (*ucontrol).value.iec958.status[2] = ((reg >> 16) & 0xff) as u8;
    (*ucontrol).value.iec958.status[3] = ((reg >> 24) & 0xff) as u8;

    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);
    (*ucontrol).value.iec958.status[4] =
        ((reg & IMG_SPDIF_OUT_CSH_UV_CSH_MASK) >> IMG_SPDIF_OUT_CSH_UV_CSH_SHIFT) as u8;

    spin_unlock_irqrestore(&mut (*spdif).lock, flags);

    0
}

unsafe extern "C" fn img_spdif_out_set_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif = snd_soc_dai_get_drvdata(cpu_dai) as *mut img_spdif_out;
    let mut reg: u32;

    reg = ((*ucontrol).value.iec958.status[3] as u32) << 24;
    reg |= ((*ucontrol).value.iec958.status[2] as u32) << 16;
    reg |= ((*ucontrol).value.iec958.status[1] as u32) << 8;
    reg |= (*ucontrol).value.iec958.status[0] as u32;

    let flags = spin_lock_irqsave(&mut (*spdif).lock);

    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CSL);

    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);
    reg &= !IMG_SPDIF_OUT_CSH_UV_CSH_MASK;
    reg |= ((*ucontrol).value.iec958.status[4] as u32) << IMG_SPDIF_OUT_CSH_UV_CSH_SHIFT;
    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CSH_UV);

    spin_unlock_irqrestore(&mut (*spdif).lock, flags);

    0
}

static mut img_spdif_out_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Playback Mask".as_ptr(),
        info: Some(img_spdif_out_info),
        get: Some(img_spdif_out_get_status_mask),
        put: None,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Playback Default".as_ptr(),
        access: 0,
        info: Some(img_spdif_out_info),
        get: Some(img_spdif_out_get_status),
        put: Some(img_spdif_out_set_status),
    },
];

unsafe extern "C" fn img_spdif_out_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut img_spdif_out;
    let mut reg: u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL);
            reg |= IMG_SPDIF_OUT_CTL_SRT_MASK;
            img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CTL);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            let flags = spin_lock_irqsave(&mut (*spdif).lock);
            img_spdif_out_reset(spdif);
            spin_unlock_irqrestore(&mut (*spdif).lock, flags);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn img_spdif_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut img_spdif_out;
    let channels: c_uint;
    let mut pre_div_a: c_long;
    let mut pre_div_b: c_long;
    let mut diff_a: c_long;
    let mut diff_b: c_long;
    let rate: c_long;
    let clk_rate: c_long;
    let mut reg: u32;
    let format: snd_pcm_format_t;

    rate = params_rate(params);
    format = params_format(params);
    channels = params_channels(params);

    dev_dbg(
        (*spdif).dev,
        c"hw_params rate %ld channels %u format %u\n".as_ptr(),
        rate,
        channels,
        format,
    );

    if format != SNDRV_PCM_FORMAT_S32_LE {
        return -EINVAL;
    }

    if channels != 2 {
        return -EINVAL;
    }

    pre_div_a = clk_round_rate((*spdif).clk_ref, rate * 256);
    if pre_div_a < 0 {
        return pre_div_a as c_int;
    }
    pre_div_b = clk_round_rate((*spdif).clk_ref, rate * 384);
    if pre_div_b < 0 {
        return pre_div_b as c_int;
    }

    diff_a = ((pre_div_a / 256) - rate).abs();
    diff_b = ((pre_div_b / 384) - rate).abs();

    /* If diffs are equal, use lower clock rate */
    if diff_a > diff_b {
        clk_set_rate((*spdif).clk_ref, pre_div_b);
    } else {
        clk_set_rate((*spdif).clk_ref, pre_div_a);
    }

    /*
     * Another driver (eg machine driver) may have rejected the above
     * change. Get the current rate and set the register bit according to
     * the new min diff
     */
    clk_rate = clk_get_rate((*spdif).clk_ref);

    diff_a = ((clk_rate / 256) - rate).abs();
    diff_b = ((clk_rate / 384) - rate).abs();

    reg = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL);
    if diff_a <= diff_b {
        reg &= !IMG_SPDIF_OUT_CTL_CLK_MASK;
    } else {
        reg |= IMG_SPDIF_OUT_CTL_CLK_MASK;
    }
    img_spdif_out_writel(spdif, reg, IMG_SPDIF_OUT_CTL);

    0
}

unsafe extern "C" fn img_spdif_out_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut img_spdif_out;

    snd_soc_dai_init_dma_data(dai, &mut (*spdif).dma_data, ptr::null_mut());

    snd_soc_add_dai_controls(
        dai,
        img_spdif_out_controls.as_mut_ptr(),
        img_spdif_out_controls.len() as c_uint,
    );

    0
}

static img_spdif_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(img_spdif_out_dai_probe),
    trigger: Some(img_spdif_out_trigger),
    hw_params: Some(img_spdif_out_hw_params),
};

static mut img_spdif_out_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &img_spdif_out_dai_ops,
};

static img_spdif_out_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"img-spdif-out".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn img_spdif_out_probe(pdev: *mut platform_device) -> c_int {
    let spdif: *mut img_spdif_out;
    let mut res: *mut resource = ptr::null_mut();
    let base: *mut c_void;
    let mut ret: c_int;
    let dev = &mut (*pdev).dev as *mut device;

    spdif = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<img_spdif_out>(),
        GFP_KERNEL,
    ) as *mut img_spdif_out;
    if spdif.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, spdif as *mut c_void);

    (*spdif).dev = &mut (*pdev).dev;

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*spdif).base = base;

    (*spdif).rst = devm_reset_control_get_exclusive(&mut (*pdev).dev, c"rst".as_ptr());
    if IS_ERR((*spdif).rst as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*spdif).rst as *const c_void),
            c"No top level reset found\n".as_ptr(),
        );
    }

    (*spdif).clk_sys = devm_clk_get(&mut (*pdev).dev, c"sys".as_ptr());
    if IS_ERR((*spdif).clk_sys as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*spdif).clk_sys as *const c_void),
            c"Failed to acquire clock 'sys'\n".as_ptr(),
        );
    }

    (*spdif).clk_ref = devm_clk_get(&mut (*pdev).dev, c"ref".as_ptr());
    if IS_ERR((*spdif).clk_ref as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*spdif).clk_ref as *const c_void),
            c"Failed to acquire clock 'ref'\n".as_ptr(),
        );
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = img_spdif_out_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            goto_err_pm_disable(pdev);
            return ret;
        }
    }
    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret < 0 {
        goto_err_suspend(pdev);
        return ret;
    }

    img_spdif_out_writel(spdif, IMG_SPDIF_OUT_CTL_FS_MASK, IMG_SPDIF_OUT_CTL);

    img_spdif_out_reset(spdif);
    pm_runtime_put(&mut (*pdev).dev);

    spin_lock_init(&mut (*spdif).lock);

    (*spdif).dma_data.addr = (*res).start + IMG_SPDIF_OUT_TX_FIFO as phys_addr_t;
    (*spdif).dma_data.addr_width = 4;
    (*spdif).dma_data.maxburst = 4;

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &img_spdif_out_component,
        &mut img_spdif_out_dai,
        1,
    );
    if ret != 0 {
        goto_err_suspend(pdev);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        goto_err_suspend(pdev);
        return ret;
    }

    dev_dbg(&mut (*pdev).dev, c"Probe successful\n".as_ptr());

    0
}

unsafe fn goto_err_suspend(pdev: *mut platform_device) {
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        img_spdif_out_runtime_suspend(&mut (*pdev).dev);
    }
    goto_err_pm_disable(pdev);
}

unsafe fn goto_err_pm_disable(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn img_spdif_out_dev_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        img_spdif_out_runtime_suspend(&mut (*pdev).dev);
    }
}

unsafe extern "C" fn img_spdif_out_suspend(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut img_spdif_out;
    let mut ret: c_int;

    if pm_runtime_status_suspended(dev) {
        ret = img_spdif_out_runtime_resume(dev);
        if ret != 0 {
            return ret;
        }
    }

    (*spdif).suspend_ctl = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CTL);
    (*spdif).suspend_csl = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSL);
    (*spdif).suspend_csh = img_spdif_out_readl(spdif, IMG_SPDIF_OUT_CSH_UV);

    img_spdif_out_runtime_suspend(dev);

    0
}

unsafe extern "C" fn img_spdif_out_resume(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut img_spdif_out;
    let ret: c_int;

    ret = img_spdif_out_runtime_resume(dev);
    if ret != 0 {
        return ret;
    }

    img_spdif_out_writel(spdif, (*spdif).suspend_ctl, IMG_SPDIF_OUT_CTL);
    img_spdif_out_writel(spdif, (*spdif).suspend_csl, IMG_SPDIF_OUT_CSL);
    img_spdif_out_writel(spdif, (*spdif).suspend_csh, IMG_SPDIF_OUT_CSH_UV);

    if pm_runtime_status_suspended(dev) {
        img_spdif_out_runtime_suspend(dev);
    }

    0
}

static img_spdif_out_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"img,spdif-out".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, img_spdif_out_of_match); */

static img_spdif_out_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(img_spdif_out_runtime_suspend),
    runtime_resume: Some(img_spdif_out_runtime_resume),
    suspend: Some(img_spdif_out_suspend),
    resume: Some(img_spdif_out_resume),
};

static mut img_spdif_out_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"img-spdif-out".as_ptr(),
        of_match_table: img_spdif_out_of_match.as_ptr(),
        pm: &img_spdif_out_pm_ops,
    },
    probe: Some(img_spdif_out_probe),
    remove: Some(img_spdif_out_dev_remove),
};
/* module_platform_driver(img_spdif_out_driver); */

/* MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>"); */
/* MODULE_DESCRIPTION("IMG SPDIF Output driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
