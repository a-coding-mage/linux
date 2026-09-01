// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * atmel-pcm.c  --  ALSA PCM interface for the Atmel atmel SoC.
 *
 *  Copyright (C) 2005 SAN People
 *  Copyright (C) 2008 Atmel
 *
 * Authors: Sedji Gaouaou <sedji.gaouaou@atmel.com>
 *
 * Based on at91-pcm. by:
 * Frank Mandarino <fmandarino@endrelia.com>
 * Copyright 2006 Endrelia Technologies Inc.
 *
 * Based on pxa2xx-pcm.c by:
 *
 * Author:	Nicolas Pitre
 * Created:	Nov 30, 2004
 * Copyright:	(C) 2004 MontaVista Software, Inc.
 */

// C dependencies:
// linux/module.h, linux/init.h, linux/platform_device.h, linux/slab.h,
// linux/dma-mapping.h, linux/atmel_pdc.h, linux/atmel-ssc.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// and "atmel-pcm.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = c_uint;
type size_t = usize;
type dma_addr_t = usize;
type snd_pcm_uframes_t = usize;

extern "C" {
    static atmel_pcm_hardware: snd_pcm_hardware;

    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        min: size_t,
        max: size_t,
    );
    fn ssc_writex(regs: *mut c_void, reg: c_uint, value: dma_addr_t);
    fn ssc_readx(regs: *mut c_void, reg: c_uint) -> c_uint;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut atmel_pcm_dma_params;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hardware: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: dma_addr_t) -> snd_pcm_uframes_t;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

const GFP_KERNEL: c_uint = 0;

const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;

const SNDRV_PCM_INFO_MMAP: c_uint = 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0;
const SNDRV_PCM_INFO_PAUSE: c_uint = 0;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 5;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const ATMEL_SSC_DMABUF_SIZE: size_t = 0;
const ATMEL_PDC_PTCR: c_uint = 0;
const SSC_PDC_PTCR: c_uint = 0;
const SSC_IDR: c_uint = 0;
const SSC_IER: c_uint = 0;
const SSC_SR: c_uint = 0;

const fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0u64
    } else {
        (1u64 << nr) - 1
    }
}

#[repr(C)]
pub struct device {
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
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: size_t,
    pub buffer_size: snd_pcm_uframes_t,
    pub dma_area: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct atmel_ssc {
    pub regs: *mut c_void,
}

#[repr(C)]
pub struct atmel_pcm_pdc {
    pub xpr: c_uint,
    pub xcr: c_uint,
    pub xnpr: c_uint,
    pub xncr: c_uint,
}

#[repr(C)]
pub struct atmel_pcm_mask {
    pub ssc_endbuf: u32,
    pub pdc_disable: dma_addr_t,
    pub pdc_enable: dma_addr_t,
    pub ssc_endx: u32,
}

#[repr(C)]
pub struct atmel_pcm_dma_params {
    pub mask: *mut atmel_pcm_mask,
    pub name: *const c_char,
    pub ssc: *mut atmel_ssc,
    pub pdc: *mut atmel_pcm_pdc,
    pub pdc_xfer_size: size_t,
    pub dma_intr_handler: Option<unsafe extern "C" fn(u32, *mut snd_pcm_substream)>,
}

#[repr(C)]
struct atmel_runtime_data {
    params: *mut atmel_pcm_dma_params,
    dma_buffer: dma_addr_t,     /* physical address of dma buffer */
    dma_buffer_end: dma_addr_t, /* first address beyond DMA buffer */
    period_size: size_t,

    period_ptr: dma_addr_t, /* physical address of next period */
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub hw_free: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
        ) -> snd_pcm_uframes_t,
    >,
    pub pcm_new: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int,
    >,
}

/*
 * Hardware definition
 */
/* TODO: These values were taken from the AT91 platform driver, check
 *	 them against real values for AT32
 */
#[no_mangle]
static ATMEL_PCM_HARDWARE_VALUE: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE,
    period_bytes_min: 32,
    period_bytes_max: 8192,
    periods_min: 2,
    periods_max: 1024,
    buffer_bytes_max: ATMEL_SSC_DMABUF_SIZE,
};

unsafe extern "C" fn atmel_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        ATMEL_SSC_DMABUF_SIZE,
        ATMEL_SSC_DMABUF_SIZE,
    );

    0
}

/*
 * ISR
 */
unsafe extern "C" fn atmel_pcm_dma_irq(ssc_sr: u32, substream: *mut snd_pcm_substream) {
    let prtd: *mut atmel_runtime_data = (*(*substream).runtime).private_data as *mut atmel_runtime_data;
    let params: *mut atmel_pcm_dma_params = (*prtd).params;
    static mut COUNT: c_int = 0;

    COUNT += 1;

    if (ssc_sr & (*(*params).mask).ssc_endbuf) != 0 {
        pr_warn(
            b"atmel-pcm: buffer %s on %s (SSC_SR=%#x, count=%d)\n\0".as_ptr() as *const c_char,
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                b"underrun\0".as_ptr()
            } else {
                b"overrun\0".as_ptr()
            },
            (*params).name,
            ssc_sr,
            COUNT,
        );

        /* re-start the PDC */
        ssc_writex((*(*params).ssc).regs, ATMEL_PDC_PTCR, (*(*params).mask).pdc_disable);
        (*prtd).period_ptr = (*prtd).period_ptr.wrapping_add((*prtd).period_size);
        if (*prtd).period_ptr >= (*prtd).dma_buffer_end {
            (*prtd).period_ptr = (*prtd).dma_buffer;
        }

        ssc_writex((*(*params).ssc).regs, (*(*params).pdc).xpr, (*prtd).period_ptr);
        ssc_writex(
            (*(*params).ssc).regs,
            (*(*params).pdc).xcr,
            (*prtd).period_size / (*params).pdc_xfer_size,
        );
        ssc_writex((*(*params).ssc).regs, ATMEL_PDC_PTCR, (*(*params).mask).pdc_enable);
    }

    if (ssc_sr & (*(*params).mask).ssc_endx) != 0 {
        /* Load the PDC next pointer and counter registers */
        (*prtd).period_ptr = (*prtd).period_ptr.wrapping_add((*prtd).period_size);
        if (*prtd).period_ptr >= (*prtd).dma_buffer_end {
            (*prtd).period_ptr = (*prtd).dma_buffer;
        }

        ssc_writex((*(*params).ssc).regs, (*(*params).pdc).xnpr, (*prtd).period_ptr);
        ssc_writex(
            (*(*params).ssc).regs,
            (*(*params).pdc).xncr,
            (*prtd).period_size / (*params).pdc_xfer_size,
        );
    }

    snd_pcm_period_elapsed(substream);
}

/*
 * PCM operations
 */
unsafe extern "C" fn atmel_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut atmel_runtime_data = (*runtime).private_data as *mut atmel_runtime_data;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);

    /* this may get called several times by oss emulation
     * with different params */

    (*prtd).params = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    (*(*prtd).params).dma_intr_handler = Some(atmel_pcm_dma_irq);

    (*prtd).dma_buffer = (*runtime).dma_addr;
    (*prtd).dma_buffer_end = (*runtime).dma_addr.wrapping_add((*runtime).dma_bytes);
    (*prtd).period_size = params_period_bytes(params);

    pr_debug(
        b"atmel-pcm: hw_params: DMA for %s initialized (dma_bytes=%zu, period_size=%zu)\n\0"
            .as_ptr() as *const c_char,
        (*(*prtd).params).name,
        (*runtime).dma_bytes,
        (*prtd).period_size,
    );
    0
}

unsafe extern "C" fn atmel_pcm_hw_free(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let prtd: *mut atmel_runtime_data =
        (*(*substream).runtime).private_data as *mut atmel_runtime_data;
    let params: *mut atmel_pcm_dma_params = (*prtd).params;

    if !params.is_null() {
        ssc_writex((*(*params).ssc).regs, SSC_PDC_PTCR, (*(*params).mask).pdc_disable);
        (*(*prtd).params).dma_intr_handler = None;
    }

    0
}

unsafe extern "C" fn atmel_pcm_prepare(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let prtd: *mut atmel_runtime_data =
        (*(*substream).runtime).private_data as *mut atmel_runtime_data;
    let params: *mut atmel_pcm_dma_params = (*prtd).params;

    ssc_writex(
        (*(*params).ssc).regs,
        SSC_IDR,
        ((*(*params).mask).ssc_endx | (*(*params).mask).ssc_endbuf) as dma_addr_t,
    );
    ssc_writex((*(*params).ssc).regs, ATMEL_PDC_PTCR, (*(*params).mask).pdc_disable);
    0
}

unsafe extern "C" fn atmel_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut atmel_runtime_data = (*rtd).private_data as *mut atmel_runtime_data;
    let params: *mut atmel_pcm_dma_params = (*prtd).params;
    let mut ret: c_int = 0;

    pr_debug(
        b"atmel-pcm:buffer_size = %ld,dma_area = %p, dma_bytes = %zu\n\0".as_ptr()
            as *const c_char,
        (*rtd).buffer_size,
        (*rtd).dma_area,
        (*rtd).dma_bytes,
    );

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*prtd).period_ptr = (*prtd).dma_buffer;

            ssc_writex((*(*params).ssc).regs, (*(*params).pdc).xpr, (*prtd).period_ptr);
            ssc_writex(
                (*(*params).ssc).regs,
                (*(*params).pdc).xcr,
                (*prtd).period_size / (*params).pdc_xfer_size,
            );

            (*prtd).period_ptr = (*prtd).period_ptr.wrapping_add((*prtd).period_size);
            ssc_writex((*(*params).ssc).regs, (*(*params).pdc).xnpr, (*prtd).period_ptr);
            ssc_writex(
                (*(*params).ssc).regs,
                (*(*params).pdc).xncr,
                (*prtd).period_size / (*params).pdc_xfer_size,
            );

            pr_debug(
                b"atmel-pcm: trigger: period_ptr=%lx, xpr=%u, xcr=%u, xnpr=%u, xncr=%u\n\0"
                    .as_ptr() as *const c_char,
                (*prtd).period_ptr as usize,
                ssc_readx((*(*params).ssc).regs, (*(*params).pdc).xpr),
                ssc_readx((*(*params).ssc).regs, (*(*params).pdc).xcr),
                ssc_readx((*(*params).ssc).regs, (*(*params).pdc).xnpr),
                ssc_readx((*(*params).ssc).regs, (*(*params).pdc).xncr),
            );

            ssc_writex(
                (*(*params).ssc).regs,
                SSC_IER,
                ((*(*params).mask).ssc_endx | (*(*params).mask).ssc_endbuf) as dma_addr_t,
            );
            ssc_writex((*(*params).ssc).regs, SSC_PDC_PTCR, (*(*params).mask).pdc_enable);

            pr_debug(
                b"sr=%u imr=%u\n\0".as_ptr() as *const c_char,
                ssc_readx((*(*params).ssc).regs, SSC_SR),
                ssc_readx((*(*params).ssc).regs, SSC_IER),
            );
        } /* SNDRV_PCM_TRIGGER_START */

        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ssc_writex((*(*params).ssc).regs, ATMEL_PDC_PTCR, (*(*params).mask).pdc_disable);
        }

        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ssc_writex((*(*params).ssc).regs, ATMEL_PDC_PTCR, (*(*params).mask).pdc_enable);
        }

        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn atmel_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut atmel_runtime_data = (*runtime).private_data as *mut atmel_runtime_data;
    let params: *mut atmel_pcm_dma_params = (*prtd).params;
    let ptr: dma_addr_t;
    let mut x: snd_pcm_uframes_t;

    ptr = ssc_readx((*(*params).ssc).regs, (*(*params).pdc).xpr) as dma_addr_t;
    x = bytes_to_frames(runtime, ptr.wrapping_sub((*prtd).dma_buffer));

    if x == (*runtime).buffer_size {
        x = 0;
    }

    x
}

unsafe extern "C" fn atmel_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut atmel_runtime_data;
    let mut ret: c_int = 0;

    snd_soc_set_runtime_hwparams(substream, &ATMEL_PCM_HARDWARE_VALUE);

    /* ensure that buffer size is a multiple of period size */
    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    prtd = kzalloc(core::mem::size_of::<atmel_runtime_data>(), GFP_KERNEL) as *mut atmel_runtime_data;
    if prtd.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    (*runtime).private_data = prtd as *mut c_void;

    ret
}

unsafe extern "C" fn atmel_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let prtd: *mut atmel_runtime_data =
        (*(*substream).runtime).private_data as *mut atmel_runtime_data;

    kfree(prtd as *mut c_void);
    0
}

static ATMEL_SOC_PLATFORM: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(atmel_pcm_open),
    close: Some(atmel_pcm_close),
    hw_params: Some(atmel_pcm_hw_params),
    hw_free: Some(atmel_pcm_hw_free),
    prepare: Some(atmel_pcm_prepare),
    trigger: Some(atmel_pcm_trigger),
    pointer: Some(atmel_pcm_pointer),
    pcm_new: Some(atmel_pcm_new),
};

#[no_mangle]
pub unsafe extern "C" fn atmel_pcm_pdc_platform_register(dev: *mut device) -> c_int {
    devm_snd_soc_register_component(dev, &ATMEL_SOC_PLATFORM, core::ptr::null_mut(), 0)
}

// EXPORT_SYMBOL(atmel_pcm_pdc_platform_register);

// MODULE_AUTHOR("Sedji Gaouaou <sedji.gaouaou@atmel.com>");
// MODULE_DESCRIPTION("Atmel PCM module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
