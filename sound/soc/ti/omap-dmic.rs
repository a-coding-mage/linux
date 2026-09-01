// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap-dmic.c  --  OMAP ASoC DMIC DAI driver
 *
 * Copyright (C) 2010 - 2011 Texas Instruments
 *
 * Author: David Lambert <dlambert@ti.com>
 *	   Misael Lopez Cruz <misael.lopez@ti.com>
 *	   Liam Girdwood <lrg@ti.com>
 *	   Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// C includes translated as external dependencies:
// linux/init.h, linux/module.h, linux/platform_device.h, linux/err.h,
// linux/clk.h, linux/io.h, linux/slab.h, linux/pm_runtime.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/initval.h,
// sound/soc.h, sound/dmaengine_pcm.h, omap-dmic.h, sdma-pcm.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

type u16 = u16;
type u32 = u32;
type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pm_qos_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: usize,
    pub filter_data: *const c_char,
    pub maxburst: c_int,
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
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct omap_dmic {
    dev: *mut device,
    io_base: *mut c_void,
    fclk: *mut clk,
    pm_qos_req: pm_qos_request,
    latency: c_int,
    fclk_freq: c_int,
    out_freq: c_int,
    clk_div: c_int,
    sysclk: c_int,
    threshold: c_int,
    ch_enabled: u32,
    active: bool_,
    mutex: mutex,

    dma_data: snd_dmaengine_dai_dma_data,
}

unsafe extern "C" {
    fn writel_relaxed(val: u32, addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn cpu_latency_qos_remove_request(req: *mut pm_qos_request);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    fn cpu_latency_qos_request_active(req: *mut pm_qos_request) -> c_int;
    fn cpu_latency_qos_update_request(req: *mut pm_qos_request, value: c_int);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn platform_get_resource_byname(
        pdev: *mut platform_device,
        ty: c_uint,
        name: *const c_char,
    ) -> *mut resource;
    fn devm_platform_ioremap_resource_byname(
        pdev: *mut platform_device,
        name: *const c_char,
    ) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn sdma_pcm_platform_register(
        dev: *mut device,
        config: *mut c_void,
        filter_data: *const c_char,
    ) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_MEM: c_uint = 0;
const USEC_PER_SEC: c_int = 1000000;

const OMAP_DMIC_CTRL_REG: u16 = 0;
const OMAP_DMIC_DMAENABLE_SET_REG: u16 = 0;
const OMAP_DMIC_DMAENABLE_CLR_REG: u16 = 0;
const OMAP_DMIC_FIFO_CTRL_REG: u16 = 0;
const OMAP_DMIC_DATA_REG: usize = 0;
const OMAP_DMIC_DMA_ENABLE: u32 = 0;
const OMAP_DMIC_UP_ENABLE_MASK: u32 = 0;
const OMAP_DMIC_UP1_ENABLE: u32 = 0;
const OMAP_DMIC_UP2_ENABLE: u32 = 0;
const OMAP_DMIC_UP3_ENABLE: u32 = 0;
const OMAP_DMIC_THRES_MAX: c_int = 0;
const OMAP_DMIC_FORMAT: u32 = 0;
const OMAP_DMIC_POLAR_MASK: u32 = 0;
const OMAP_DMICOUTFORMAT_LJUST: u32 = 0;
const OMAP_DMIC_POLAR1: u32 = 0;
const OMAP_DMIC_POLAR2: u32 = 0;
const OMAP_DMIC_POLAR3: u32 = 0;
const OMAP_DMIC_CLK_DIV_MASK: u32 = 0;
const OMAP_DMIC_SYSCLK_PAD_CLKS: c_int = 0;
const OMAP_DMIC_SYSCLK_SLIMBLUS_CLKS: c_int = 0;
const OMAP_DMIC_SYSCLK_SYNC_MUX_CLKS: c_int = 0;
const OMAP_DMIC_ABE_DMIC_CLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

const fn OMAP_DMIC_CLK_DIV(div: c_int) -> u32 {
    div as u32
}

unsafe fn omap_dmic_write(dmic: *mut omap_dmic, reg: u16, val: u32) {
    unsafe {
        writel_relaxed(val, ((*dmic).io_base as *mut u8).add(reg as usize) as *mut c_void);
    }
}

unsafe fn omap_dmic_read(dmic: *mut omap_dmic, reg: u16) -> c_int {
    unsafe { readl_relaxed(((*dmic).io_base as *mut u8).add(reg as usize) as *mut c_void) as c_int }
}

unsafe fn omap_dmic_start(dmic: *mut omap_dmic) {
    let ctrl: u32 = unsafe { omap_dmic_read(dmic, OMAP_DMIC_CTRL_REG) as u32 };

    /* Configure DMA controller */
    unsafe {
        omap_dmic_write(
            dmic,
            OMAP_DMIC_DMAENABLE_SET_REG,
            OMAP_DMIC_DMA_ENABLE,
        );

        omap_dmic_write(dmic, OMAP_DMIC_CTRL_REG, ctrl | (*dmic).ch_enabled);
    }
}

unsafe fn omap_dmic_stop(dmic: *mut omap_dmic) {
    let ctrl: u32 = unsafe { omap_dmic_read(dmic, OMAP_DMIC_CTRL_REG) as u32 };
    unsafe {
        omap_dmic_write(
            dmic,
            OMAP_DMIC_CTRL_REG,
            ctrl & !OMAP_DMIC_UP_ENABLE_MASK,
        );

        /* Disable DMA request generation */
        omap_dmic_write(
            dmic,
            OMAP_DMIC_DMAENABLE_CLR_REG,
            OMAP_DMIC_DMA_ENABLE,
        );
    }
}

unsafe fn dmic_is_enabled(dmic: *mut omap_dmic) -> c_int {
    unsafe { omap_dmic_read(dmic, OMAP_DMIC_CTRL_REG) & OMAP_DMIC_UP_ENABLE_MASK as c_int }
}

unsafe extern "C" fn omap_dmic_dai_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };

    unsafe {
        mutex_lock(&mut (*dmic).mutex);
        if snd_soc_dai_active(dai) != 0 {
            mutex_unlock(&mut (*dmic).mutex);
            return -EBUSY;
        }

        (*dmic).active = true;
        mutex_unlock(&mut (*dmic).mutex);
    }
    0
}

unsafe extern "C" fn omap_dmic_dai_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };

    unsafe {
        mutex_lock(&mut (*dmic).mutex);

        cpu_latency_qos_remove_request(&mut (*dmic).pm_qos_req);

        if snd_soc_dai_active(dai) == 0 {
            (*dmic).active = false;
        }

        mutex_unlock(&mut (*dmic).mutex);
    }
}

unsafe fn omap_dmic_select_divider(dmic: *mut omap_dmic, sample_rate: c_int) -> c_int {
    let mut divider: c_int = -EINVAL;

    /*
     * 192KHz rate is only supported with 19.2MHz/3.84MHz clock
     * configuration.
     */
    unsafe {
        if sample_rate == 192000 {
            if (*dmic).fclk_freq == 19200000 && (*dmic).out_freq == 3840000 {
                divider = 0x6; /* Divider: 5 (192KHz sampling rate) */
            } else {
                dev_err(
                    (*dmic).dev,
                    c"invalid clock configuration for 192KHz\n".as_ptr(),
                );
            }

            return divider;
        }

        match (*dmic).out_freq {
            1536000 => {
                if (*dmic).fclk_freq != 24576000 {
                    dev_err(
                        (*dmic).dev,
                        c"invalid out frequency %dHz for %dHz input\n".as_ptr(),
                        (*dmic).out_freq,
                        (*dmic).fclk_freq,
                    );
                    return -EINVAL;
                }
                divider = 0x4; /* Divider: 16 */
            }
            2400000 => match (*dmic).fclk_freq {
                12000000 => {
                    divider = 0x5; /* Divider: 5 */
                }
                19200000 => {
                    divider = 0x0; /* Divider: 8 */
                }
                24000000 => {
                    divider = 0x2; /* Divider: 10 */
                }
                _ => {
                    dev_err(
                        (*dmic).dev,
                        c"invalid out frequency %dHz for %dHz input\n".as_ptr(),
                        (*dmic).out_freq,
                        (*dmic).fclk_freq,
                    );
                    return -EINVAL;
                }
            },
            3072000 => {
                if (*dmic).fclk_freq != 24576000 {
                    dev_err(
                        (*dmic).dev,
                        c"invalid out frequency %dHz for %dHz input\n".as_ptr(),
                        (*dmic).out_freq,
                        (*dmic).fclk_freq,
                    );
                    return -EINVAL;
                }
                divider = 0x3; /* Divider: 8 */
            }
            3840000 => {
                if (*dmic).fclk_freq != 19200000 {
                    dev_err(
                        (*dmic).dev,
                        c"invalid out frequency %dHz for %dHz input\n".as_ptr(),
                        (*dmic).out_freq,
                        (*dmic).fclk_freq,
                    );
                    return -EINVAL;
                }
                divider = 0x1; /* Divider: 5 (96KHz sampling rate) */
            }
            _ => {
                dev_err(
                    (*dmic).dev,
                    c"invalid out frequency: %dHz\n".as_ptr(),
                    (*dmic).out_freq,
                );
            }
        }
    }

    divider
}

unsafe extern "C" fn omap_dmic_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let channels: c_int;

    unsafe {
        (*dmic).clk_div = omap_dmic_select_divider(dmic, params_rate(params));
        if (*dmic).clk_div < 0 {
            dev_err(
                (*dmic).dev,
                c"no valid divider for %dHz from %dHz\n".as_ptr(),
                (*dmic).out_freq,
                (*dmic).fclk_freq,
            );
            return -EINVAL;
        }

        (*dmic).ch_enabled = 0;
        channels = params_channels(params);
        match channels {
            6 => {
                (*dmic).ch_enabled |= OMAP_DMIC_UP3_ENABLE;
                (*dmic).ch_enabled |= OMAP_DMIC_UP2_ENABLE;
                (*dmic).ch_enabled |= OMAP_DMIC_UP1_ENABLE;
            }
            4 => {
                (*dmic).ch_enabled |= OMAP_DMIC_UP2_ENABLE;
                (*dmic).ch_enabled |= OMAP_DMIC_UP1_ENABLE;
            }
            2 => {
                (*dmic).ch_enabled |= OMAP_DMIC_UP1_ENABLE;
            }
            _ => {
                dev_err((*dmic).dev, c"invalid number of legacy channels\n".as_ptr());
                return -EINVAL;
            }
        }

        /* packet size is threshold * channels */
        dma_data = snd_soc_dai_get_dma_data(dai, substream);
        (*dma_data).maxburst = (*dmic).threshold * channels;
        (*dmic).latency =
            (OMAP_DMIC_THRES_MAX - (*dmic).threshold) * USEC_PER_SEC / params_rate(params);
    }

    0
}

unsafe extern "C" fn omap_dmic_dai_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };
    let mut ctrl: u32;

    unsafe {
        if cpu_latency_qos_request_active(&mut (*dmic).pm_qos_req) != 0 {
            cpu_latency_qos_update_request(&mut (*dmic).pm_qos_req, (*dmic).latency);
        }

        /* Configure uplink threshold */
        omap_dmic_write(dmic, OMAP_DMIC_FIFO_CTRL_REG, (*dmic).threshold as u32);

        ctrl = omap_dmic_read(dmic, OMAP_DMIC_CTRL_REG) as u32;

        /* Set dmic out format */
        ctrl &= !(OMAP_DMIC_FORMAT | OMAP_DMIC_POLAR_MASK);
        ctrl |= OMAP_DMICOUTFORMAT_LJUST | OMAP_DMIC_POLAR1 | OMAP_DMIC_POLAR2 | OMAP_DMIC_POLAR3;

        /* Configure dmic clock divider */
        ctrl &= !OMAP_DMIC_CLK_DIV_MASK;
        ctrl |= OMAP_DMIC_CLK_DIV((*dmic).clk_div);

        omap_dmic_write(dmic, OMAP_DMIC_CTRL_REG, ctrl);

        omap_dmic_write(
            dmic,
            OMAP_DMIC_CTRL_REG,
            ctrl | OMAP_DMICOUTFORMAT_LJUST | OMAP_DMIC_POLAR1 | OMAP_DMIC_POLAR2 | OMAP_DMIC_POLAR3,
        );
    }

    0
}

unsafe extern "C" fn omap_dmic_dai_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };

    unsafe {
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                omap_dmic_start(dmic);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                omap_dmic_stop(dmic);
            }
            _ => {}
        }
    }

    0
}

unsafe fn omap_dmic_select_fclk(
    dmic: *mut omap_dmic,
    clk_id: c_int,
    freq: c_uint,
) -> c_int {
    let parent_clk: *mut clk;
    let mux: *mut clk;
    let parent_clk_name: *const c_char;
    let mut ret: c_int = 0;

    unsafe {
        match freq {
            12000000 | 19200000 | 24000000 | 24576000 => {}
            _ => {
                dev_err((*dmic).dev, c"invalid input frequency: %dHz\n".as_ptr(), freq);
                (*dmic).fclk_freq = 0;
                return -EINVAL;
            }
        }

        if (*dmic).sysclk == clk_id {
            (*dmic).fclk_freq = freq as c_int;
            return 0;
        }

        /* re-parent not allowed if a stream is ongoing */
        if (*dmic).active && dmic_is_enabled(dmic) != 0 {
            dev_err((*dmic).dev, c"can't re-parent when DMIC active\n".as_ptr());
            return -EBUSY;
        }

        match clk_id {
            OMAP_DMIC_SYSCLK_PAD_CLKS => {
                parent_clk_name = c"pad_clks_ck".as_ptr();
            }
            OMAP_DMIC_SYSCLK_SLIMBLUS_CLKS => {
                parent_clk_name = c"slimbus_clk".as_ptr();
            }
            OMAP_DMIC_SYSCLK_SYNC_MUX_CLKS => {
                parent_clk_name = c"dmic_sync_mux_ck".as_ptr();
            }
            _ => {
                dev_err((*dmic).dev, c"fclk clk_id (%d) not supported\n".as_ptr(), clk_id);
                return -EINVAL;
            }
        }

        parent_clk = clk_get((*dmic).dev, parent_clk_name);
        if IS_ERR(parent_clk as *const c_void) != 0 {
            dev_err((*dmic).dev, c"can't get %s\n".as_ptr(), parent_clk_name);
            return -ENODEV;
        }

        mux = clk_get_parent((*dmic).fclk);
        if mux.is_null() {
            dev_err((*dmic).dev, c"can't get fck mux parent\n".as_ptr());
            clk_put(parent_clk);
            return -ENODEV;
        }

        mutex_lock(&mut (*dmic).mutex);
        if (*dmic).active {
            /* disable clock while reparenting */
            pm_runtime_put_sync((*dmic).dev);
            ret = clk_set_parent(mux, parent_clk);
            pm_runtime_get_sync((*dmic).dev);
        } else {
            ret = clk_set_parent(mux, parent_clk);
        }
        mutex_unlock(&mut (*dmic).mutex);

        if ret < 0 {
            dev_err((*dmic).dev, c"re-parent failed\n".as_ptr());
        } else {
            (*dmic).sysclk = clk_id;
            (*dmic).fclk_freq = freq as c_int;
        }

        clk_put(mux);
        clk_put(parent_clk);
    }

    ret
}

unsafe fn omap_dmic_select_outclk(
    dmic: *mut omap_dmic,
    clk_id: c_int,
    freq: c_uint,
) -> c_int {
    let mut ret: c_int = 0;

    unsafe {
        if clk_id != OMAP_DMIC_ABE_DMIC_CLK {
            dev_err(
                (*dmic).dev,
                c"output clk_id (%d) not supported\n".as_ptr(),
                clk_id,
            );
            return -EINVAL;
        }

        match freq {
            1536000 | 2400000 | 3072000 | 3840000 => {
                (*dmic).out_freq = freq as c_int;
            }
            _ => {
                dev_err((*dmic).dev, c"invalid out frequency: %dHz\n".as_ptr(), freq);
                (*dmic).out_freq = 0;
                ret = -EINVAL;
            }
        }
    }

    ret
}

unsafe extern "C" fn omap_dmic_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };

    unsafe {
        if dir == SND_SOC_CLOCK_IN {
            return omap_dmic_select_fclk(dmic, clk_id, freq);
        } else if dir == SND_SOC_CLOCK_OUT {
            return omap_dmic_select_outclk(dmic, clk_id, freq);
        }

        dev_err((*dmic).dev, c"invalid clock direction (%d)\n".as_ptr(), dir);
    }
    -EINVAL
}

unsafe extern "C" fn omap_dmic_probe(dai: *mut snd_soc_dai) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };

    unsafe {
        pm_runtime_enable((*dmic).dev);

        /* Disable lines while request is ongoing */
        pm_runtime_get_sync((*dmic).dev);
        omap_dmic_write(dmic, OMAP_DMIC_CTRL_REG, 0x00);
        pm_runtime_put_sync((*dmic).dev);

        /* Configure DMIC threshold value */
        (*dmic).threshold = OMAP_DMIC_THRES_MAX - 3;

        snd_soc_dai_init_dma_data(dai, core::ptr::null_mut(), &mut (*dmic).dma_data);
    }

    0
}

unsafe extern "C" fn omap_dmic_remove(dai: *mut snd_soc_dai) -> c_int {
    let dmic: *mut omap_dmic = unsafe { snd_soc_dai_get_drvdata(dai) as *mut omap_dmic };

    unsafe {
        pm_runtime_disable((*dmic).dev);
    }

    0
}

static omap_dmic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(omap_dmic_probe),
    remove: Some(omap_dmic_remove),
    startup: Some(omap_dmic_dai_startup),
    shutdown: Some(omap_dmic_dai_shutdown),
    hw_params: Some(omap_dmic_dai_hw_params),
    prepare: Some(omap_dmic_dai_prepare),
    trigger: Some(omap_dmic_dai_trigger),
    set_sysclk: Some(omap_dmic_set_dai_sysclk),
};

static mut omap_dmic_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"omap-dmic".as_ptr(),
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 6,
        rates: SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
        sig_bits: 24,
    },
    ops: &omap_dmic_dai_ops,
};

static omap_dmic_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"omap-dmic".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn asoc_dmic_probe(pdev: *mut platform_device) -> c_int {
    let dmic: *mut omap_dmic;
    let res: *mut resource;
    let mut ret: c_int;

    unsafe {
        dmic = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<omap_dmic>(),
            GFP_KERNEL,
        ) as *mut omap_dmic;
        if dmic.is_null() {
            return -ENOMEM;
        }

        platform_set_drvdata(pdev, dmic as *mut c_void);
        (*dmic).dev = &mut (*pdev).dev;
        (*dmic).sysclk = OMAP_DMIC_SYSCLK_SYNC_MUX_CLKS;

        mutex_init(&mut (*dmic).mutex);

        (*dmic).fclk = devm_clk_get((*dmic).dev, c"fck".as_ptr());
        if IS_ERR((*dmic).fclk as *const c_void) != 0 {
            return dev_err_probe(
                (*dmic).dev,
                PTR_ERR((*dmic).fclk as *const c_void),
                c"can't get fck\n".as_ptr(),
            );
        }

        res = platform_get_resource_byname(pdev, IORESOURCE_MEM, c"dma".as_ptr());
        if res.is_null() {
            dev_err((*dmic).dev, c"invalid dma memory resource\n".as_ptr());
            return -ENODEV;
        }
        (*dmic).dma_data.addr = (*res).start + OMAP_DMIC_DATA_REG;

        (*dmic).dma_data.filter_data = c"up_link".as_ptr();

        (*dmic).io_base = devm_platform_ioremap_resource_byname(pdev, c"mpu".as_ptr());
        if IS_ERR((*dmic).io_base as *const c_void) != 0 {
            return PTR_ERR((*dmic).io_base as *const c_void) as c_int;
        }

        ret = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &omap_dmic_component,
            &raw mut omap_dmic_dai,
            1,
        );
        if ret != 0 {
            return ret;
        }

        ret = sdma_pcm_platform_register(&mut (*pdev).dev, core::ptr::null_mut(), c"up_link".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    0
}

static omap_dmic_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,omap4-dmic".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, omap_dmic_of_match);

static mut asoc_dmic_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"omap-dmic".as_ptr(),
        of_match_table: omap_dmic_of_match.as_ptr(),
    },
    probe: Some(asoc_dmic_probe),
};

// module_platform_driver(asoc_dmic_driver);

// MODULE_ALIAS("platform:omap-dmic");
// MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>");
// MODULE_DESCRIPTION("OMAP DMIC ASoC Interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
