// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO DMA driver.
//
// Copyright (c) 2016-2018 Socionext Inc.

// #include <linux/dma-mapping.h>
// #include <linux/errno.h>
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <sound/soc.h>
//
// #include "aio.h"

use core::mem;
use core::ptr;

// External types from Linux kernel and sound subsystem
extern "C" {
    // Types from sound/core.h and related headers
    type snd_pcm_runtime;
    type snd_compr_runtime;
    type snd_pcm_substream;
    type snd_compr_stream;
    type snd_soc_component;
    type snd_pcm;
    type snd_soc_pcm_runtime;
    type vm_area_struct;
    type platform_device;
    type device;
    type regmap;

    // External functions (declarations)
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: u32,
        var: u32,
        step: u32,
    ) -> i32;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, index: i32) -> *mut snd_soc_component;
    fn uniphier_priv(component: *mut snd_soc_component) -> *mut uniphier_aio;
    fn samples_to_bytes(runtime: *const snd_pcm_runtime, samples: u32) -> u32;
    fn bytes_to_frames(runtime: *const snd_pcm_runtime, bytes: u32) -> u32;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut uniphier_aio_chip;
    fn aiodma_rb_set_threshold(
        sub: *mut uniphier_aio_sub,
        dma_bytes: u32,
        threshold: u32,
    ) -> i32;
    fn aiodma_rb_sync(
        sub: *mut uniphier_aio_sub,
        dma_addr: u32,
        dma_bytes: u32,
        bytes: u32,
    );
    fn aiodma_rb_clear_irq(sub: *mut uniphier_aio_sub);
    fn aiodma_rb_is_irq(sub: *mut uniphier_aio_sub) -> i32;
    fn aiodma_ch_set_param(sub: *mut uniphier_aio_sub) -> i32;
    fn aiodma_rb_set_buffer(
        sub: *mut uniphier_aio_sub,
        start: u32,
        end: u32,
        bytes: u32,
    ) -> i32;
    fn aiodma_ch_set_enable(sub: *mut uniphier_aio_sub, enable: i32);
    fn pgprot_writecombine(prot: u32) -> u32;
    fn remap_pfn_range(
        vma: *mut vm_area_struct,
        addr: u32,
        pfn: u32,
        size: u32,
        prot: u32,
    ) -> i32;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: i32)
        -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn platform_get_irq(pdev: *mut platform_device, num: i32) -> i32;
    fn devm_request_irq(
        dev: *mut device,
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
        irqflags: u32,
        devname: *const i8,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn dev_name(dev: *const device) -> *const i8;
    fn dev_warn(dev: *const device, fmt: *const i8, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const core::ffi::c_void,
        num_dai: i32,
    ) -> i32;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: i32,
        dev: *mut device,
        prealloc: u32,
        max_prealloc: u32,
    );
}

// Constants
const SNDRV_PCM_INFO_MMAP: u32 = 0x00000001;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x00000002;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x00000100;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: u32 = 4;
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
const SNDRV_DMA_TYPE_DEV: i32 = 0;
const IRQF_SHARED: u32 = 0x00000080;
const PORT_DIR_OUTPUT: i32 = 0;
const IRQ_NONE: i32 = 0;
const IRQ_HANDLED: i32 = 1;
const PAGE_SHIFT: u32 = 12;
const REGCACHE_NONE: u32 = 0;
const DMA_BIT_MASK_33: u64 = (1u64 << 33) - 1;

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub period_bytes_min: u32,
    pub period_bytes_max: u32,
    pub periods_min: u32,
    pub periods_max: u32,
    pub buffer_bytes_max: u32,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
    pub cache_type: u32,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, i32) -> i32>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> u32>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut vm_area_struct) -> i32>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> i32>,
    pub compress_ops: *const core::ffi::c_void,
}

// External structures from aio.h
#[repr(C)]
pub struct uniphier_aio_sub {
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
    pub lock: u32, // spinlock_t (simplified)
    pub threshold: u32,
    pub running: i32,
    pub rd_offs: u32,
    pub wr_offs: u32,
    pub compr_addr: u32,
    pub compr_bytes: u32,
    pub swm: *mut uniphier_aio_swm,
}

#[repr(C)]
pub struct uniphier_aio_swm {
    pub dir: i32,
}

#[repr(C)]
pub struct uniphier_aio {
    pub chip: *mut uniphier_aio_chip,
    pub sub: [uniphier_aio_sub; 2],
}

#[repr(C)]
pub struct uniphier_aio_chip {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub num_aios: i32,
    pub aios: *mut uniphier_aio,
}

extern "C" {
    static mut uniphier_aio_compress_ops: core::ffi::c_void;
}

static mut UNIPHIER_AIODMA_HW: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED,
    period_bytes_min: 256,
    period_bytes_max: 4096,
    periods_min: 4,
    periods_max: 1024,
    buffer_bytes_max: 128 * 1024,
};

unsafe fn aiodma_pcm_irq(sub: *mut uniphier_aio_sub) {
    let sub = &mut *sub;
    let runtime = &*(*sub.substream).runtime;
    let bytes = runtime.period_size * runtime.channels * samples_to_bytes(runtime, 1);

    // Simplified scoped_guard(spinlock) - in real code this would use kernel's spinlock guard
    let ret = aiodma_rb_set_threshold(sub, runtime.dma_bytes, sub.threshold + bytes);
    if ret == 0 {
        sub.threshold += bytes;
    }

    aiodma_rb_sync(sub, runtime.dma_addr, runtime.dma_bytes, bytes);
    aiodma_rb_clear_irq(sub);

    snd_pcm_period_elapsed(sub.substream);
}

unsafe fn aiodma_compr_irq(sub: *mut uniphier_aio_sub) {
    let sub = &mut *sub;
    let runtime = &*(*sub.cstream).runtime;
    let bytes = runtime.fragment_size;

    // Simplified scoped_guard(spinlock) - in real code this would use kernel's spinlock guard
    let ret = aiodma_rb_set_threshold(sub, sub.compr_bytes, sub.threshold + bytes);
    if ret == 0 {
        sub.threshold += bytes;
    }

    aiodma_rb_sync(sub, sub.compr_addr, sub.compr_bytes, bytes);
    aiodma_rb_clear_irq(sub);

    snd_compr_fragment_elapsed(sub.cstream);
}

unsafe extern "C" fn aiodma_irq(irq: i32, p: *mut core::ffi::c_void) -> i32 {
    let pdev = p as *mut platform_device;
    let chip = &mut *(platform_get_drvdata(pdev) as *mut uniphier_aio_chip);
    let mut ret = IRQ_NONE;

    for i in 0..chip.num_aios {
        let aio = &mut *chip.aios.add(i as usize);

        for j in 0..2 {
            let sub = &mut aio.sub[j as usize];

            // Skip channel that does not trigger
            if sub.running == 0 || aiodma_rb_is_irq(sub) == 0 {
                continue;
            }

            if !sub.substream.is_null() {
                aiodma_pcm_irq(sub);
            }
            if !sub.cstream.is_null() {
                aiodma_compr_irq(sub);
            }

            ret = IRQ_HANDLED;
        }
    }

    ret
}

unsafe extern "C" fn uniphier_aiodma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime = &mut *(*substream).runtime;

    snd_soc_set_runtime_hwparams(substream, &UNIPHIER_AIODMA_HW);

    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 256)
}

unsafe extern "C" fn uniphier_aiodma_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> i32 {
    let runtime = &*(*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0)) as *mut uniphier_aio;
    let sub = &mut (*aio).sub[(*substream).stream as usize];
    let bytes = runtime.period_size * runtime.channels * samples_to_bytes(runtime, 1);

    let ret = aiodma_ch_set_param(sub);
    if ret != 0 {
        return ret;
    }

    // Simplified spinlock_irqsave guard
    let ret = aiodma_rb_set_buffer(
        sub,
        runtime.dma_addr,
        runtime.dma_addr + runtime.dma_bytes,
        bytes,
    );
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn uniphier_aiodma_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: i32,
) -> i32 {
    let runtime = &*(*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0)) as *mut uniphier_aio;
    let sub = &mut (*aio).sub[(*substream).stream as usize];
    let dev = &(*(*aio).chip).pdev;
    let bytes = runtime.period_size * runtime.channels * samples_to_bytes(runtime, 1);

    // Simplified spinlock_irqsave guard
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            aiodma_rb_sync(sub, runtime.dma_addr, runtime.dma_bytes, bytes);
            aiodma_ch_set_enable(sub, 1);
            sub.running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            sub.running = 0;
            aiodma_ch_set_enable(sub, 0);
        }
        _ => {
            dev_warn(dev, b"Unknown trigger(%d) ignored\n\0".as_ptr() as *const i8, cmd);
        }
    }

    0
}

unsafe extern "C" fn uniphier_aiodma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> u32 {
    let runtime = &*(*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let aio = uniphier_priv(snd_soc_rtd_to_cpu(rtd, 0)) as *mut uniphier_aio;
    let sub = &mut (*aio).sub[(*substream).stream as usize];
    let bytes = runtime.period_size * runtime.channels * samples_to_bytes(runtime, 1);
    let pos: u32;

    // Simplified spinlock_irqsave guard
    aiodma_rb_sync(sub, runtime.dma_addr, runtime.dma_bytes, bytes);

    if (*sub.swm).dir == PORT_DIR_OUTPUT {
        pos = bytes_to_frames(runtime, sub.rd_offs);
    } else {
        pos = bytes_to_frames(runtime, sub.wr_offs);
    }

    pos
}

unsafe extern "C" fn uniphier_aiodma_mmap(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    vma: *mut vm_area_struct,
) -> i32 {
    let vma = &mut *vma;
    vma.vm_page_prot = pgprot_writecombine(vma.vm_page_prot);

    remap_pfn_range(
        vma,
        vma.vm_start,
        (*substream).runtime.dma_addr >> PAGE_SHIFT,
        vma.vm_end - vma.vm_start,
        vma.vm_page_prot,
    )
}

unsafe extern "C" fn uniphier_aiodma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> i32 {
    let dev = (*(*rtd).card).snd_card.dev;
    let pcm = (*rtd).pcm;

    let ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK_33);
    if ret != 0 {
        return ret;
    }

    let hw_buffer_bytes_max = unsafe { UNIPHIER_AIODMA_HW.buffer_bytes_max };
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        dev,
        hw_buffer_bytes_max,
        hw_buffer_bytes_max,
    );
    0
}

static UNIPHIER_SOC_PLATFORM: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(uniphier_aiodma_open),
    prepare: Some(uniphier_aiodma_prepare),
    trigger: Some(uniphier_aiodma_trigger),
    pointer: Some(uniphier_aiodma_pointer),
    mmap: Some(uniphier_aiodma_mmap),
    pcm_new: Some(uniphier_aiodma_new),
    compress_ops: unsafe { &uniphier_aio_compress_ops as *const _ as *const core::ffi::c_void },
};

static AIODMA_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x7fffc,
    cache_type: REGCACHE_NONE,
};

/// uniphier_aiodma_soc_register_platform - register the AIO DMA
/// @pdev: the platform device
///
/// Register and setup the DMA of AIO to transfer the sound data to device.
/// This function need to call once at driver startup and need NOT to call
/// unregister function.
///
/// Return: Zero if successful, otherwise a negative value on error.
#[no_mangle]
pub unsafe extern "C" fn uniphier_aiodma_soc_register_platform(pdev: *mut platform_device) -> i32 {
    let chip = platform_get_drvdata(pdev) as *mut uniphier_aio_chip;
    let dev = &(*pdev).dev;
    let mut preg: *mut core::ffi::c_void;
    let mut irq: i32;
    let mut ret: i32;

    preg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(preg as *const core::ffi::c_void) != 0 {
        return PTR_ERR(preg as *const core::ffi::c_void);
    }

    (*chip).regmap = devm_regmap_init_mmio(dev, preg, &AIODMA_REGMAP_CONFIG);
    if IS_ERR((*chip).regmap as *const core::ffi::c_void) != 0 {
        return PTR_ERR((*chip).regmap as *const core::ffi::c_void);
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    ret = devm_request_irq(
        dev,
        irq as u32,
        aiodma_irq,
        IRQF_SHARED,
        dev_name(dev),
        pdev as *mut core::ffi::c_void,
    );
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(dev, &UNIPHIER_SOC_PLATFORM, ptr::null(), 0)
}

// Symbol exported for kernel module
// EXPORT_SYMBOL_GPL(uniphier_aiodma_soc_register_platform);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
