// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * atmel-pcm-dma.c  --  ALSA PCM DMA support for the Atmel SoC.
 *
 *  Copyright (C) 2012 Atmel
 *
 * Author: Bo Shen <voice.shen@atmel.com>
 *
 * Based on atmel-pcm by:
 * Sedji Gaouaou <sedji.gaouaou@atmel.com>
 * Copyright 2008 Atmel
 */

/* Dependency intent from C includes:
 * linux/module.h, linux/init.h, linux/platform_device.h, linux/slab.h,
 * linux/dma-mapping.h, linux/dmaengine.h, linux/atmel-ssc.h,
 * sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/dmaengine_pcm.h, and "atmel-pcm.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = c_uint;

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: usize,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            slave_config: *mut dma_slave_config,
        ) -> c_int,
    >,
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: usize,
}

#[repr(C)]
pub struct dma_slave_config {
    pub dst_addr: usize,
    pub dst_maxburst: c_uint,
    pub src_addr: usize,
    pub src_maxburst: c_uint,
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
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ssc_device {
    pub regs: *mut c_void,
    pub phybase: usize,
}

#[repr(C)]
pub struct atmel_ssc_mask {
    pub ssc_error: u32,
    pub ssc_disable: u32,
}

#[repr(C)]
pub struct atmel_pcm_dma_params {
    pub name: *const c_char,
    pub ssc: *mut ssc_device,
    pub mask: *const atmel_ssc_mask,
    pub dma_intr_handler:
        Option<unsafe extern "C" fn(ssc_sr: u32, substream: *mut snd_pcm_substream)>,
}

extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SSC_CR: c_uint;
    static SSC_RHR: c_uint;
    static SSC_SR: c_uint;
    static SSC_THR: usize;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut atmel_pcm_dma_params;
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn ssc_writex(regs: *mut c_void, reg: c_uint, value: u32);
    fn ssc_readx(regs: *mut c_void, reg: c_uint) -> u32;
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_hwparams_to_dma_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
}

/*--------------------------------------------------------------------------*\
 * Hardware definition
\*--------------------------------------------------------------------------*/
static mut atmel_pcm_dma_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_RESUME
            | SNDRV_PCM_INFO_PAUSE
    },
    period_bytes_min: 256,      /* lighting DMA overhead */
    period_bytes_max: 2 * 0xffff, /* if 2 bytes format */
    periods_min: 8,
    periods_max: 1024,         /* no limit */
    buffer_bytes_max: 512 * 1024,
};

/*
 * atmel_pcm_dma_irq: SSC interrupt handler for DMAENGINE enabled SSC
 *
 * We use DMAENGINE to send/receive data to/from SSC so this ISR is only to
 * check if any overrun occured.
 */
unsafe extern "C" fn atmel_pcm_dma_irq(ssc_sr: u32, substream: *mut snd_pcm_substream) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let prtd: *mut atmel_pcm_dma_params;

    prtd = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);

    if (ssc_sr & (*(*prtd).mask).ssc_error) != 0 {
        if snd_pcm_running(substream) != 0 {
            pr_warn(
                b"atmel-pcm: buffer %s on %s (SSC_SR=%#x)\n\0".as_ptr() as *const c_char,
                if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                    b"underrun\0".as_ptr() as *const c_char
                } else {
                    b"overrun\0".as_ptr() as *const c_char
                },
                (*prtd).name,
                ssc_sr,
            );
        }

        /* stop RX and capture: will be enabled again at restart */
        ssc_writex(
            (*(*prtd).ssc).regs,
            SSC_CR,
            (*(*prtd).mask).ssc_disable,
        );
        snd_pcm_stop_xrun(substream);

        /* now drain RHR and read status to remove xrun condition */
        ssc_readx((*(*prtd).ssc).regs, SSC_RHR);
        ssc_readx((*(*prtd).ssc).regs, SSC_SR);
    }
}

unsafe extern "C" fn atmel_pcm_configure_dma(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    slave_config: *mut dma_slave_config,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let prtd: *mut atmel_pcm_dma_params;
    let ssc: *mut ssc_device;
    let ret: c_int;

    prtd = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    ssc = (*prtd).ssc;

    ret = snd_hwparams_to_dma_slave_config(substream, params, slave_config);
    if ret != 0 {
        pr_err(b"atmel-pcm: hwparams to dma slave configure failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    (*slave_config).dst_addr = (*ssc).phybase + SSC_THR;
    (*slave_config).dst_maxburst = 1;

    (*slave_config).src_addr = (*ssc).phybase + SSC_RHR as usize;
    (*slave_config).src_maxburst = 1;

    (*prtd).dma_intr_handler = Some(atmel_pcm_dma_irq);

    0
}

static mut atmel_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(atmel_pcm_configure_dma),
    pcm_hardware: unsafe { &raw const atmel_pcm_dma_hardware },
    prealloc_buffer_size: 64 * 1024,
};

#[no_mangle]
pub unsafe extern "C" fn atmel_pcm_dma_platform_register(dev: *mut device) -> c_int {
    devm_snd_dmaengine_pcm_register(dev, &raw const atmel_dmaengine_pcm_config, 0)
}
/* EXPORT_SYMBOL(atmel_pcm_dma_platform_register); */

/* MODULE_AUTHOR("Bo Shen <voice.shen@atmel.com>"); */
/* MODULE_DESCRIPTION("Atmel DMA based PCM module"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
