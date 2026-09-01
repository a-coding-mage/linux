/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * at91-pcm.h - ALSA PCM interface for the Atmel AT91 SoC.
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

/* Dependency intent from C header: #include <linux/atmel-ssc.h> */

pub const ATMEL_SSC_DMABUF_SIZE: usize = 64 * 1024;

/*
 * Registers and status bits that are required by the PCM driver.
 */
#[repr(C)]
pub struct atmel_pdc_regs {
    pub xpr: ::core::ffi::c_uint,  /* PDC recv/trans pointer */
    pub xcr: ::core::ffi::c_uint,  /* PDC recv/trans counter */
    pub xnpr: ::core::ffi::c_uint, /* PDC next recv/trans pointer */
    pub xncr: ::core::ffi::c_uint, /* PDC next recv/trans counter */
    pub ptcr: ::core::ffi::c_uint, /* PDC transfer control */
}

#[repr(C)]
pub struct atmel_ssc_mask {
    pub ssc_enable: u32,  /* SSC recv/trans enable */
    pub ssc_disable: u32, /* SSC recv/trans disable */
    pub ssc_error: u32,   /* SSC error conditions */
    pub ssc_endx: u32,    /* SSC ENDTX or ENDRX */
    pub ssc_endbuf: u32,  /* SSC TXBUFE or RXBUFF */
    pub pdc_enable: u32,  /* PDC recv/trans enable */
    pub pdc_disable: u32, /* PDC recv/trans disable */
}

#[repr(C)]
pub struct ssc_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

/*
 * This structure, shared between the PCM driver and the interface,
 * contains all information required by the PCM driver to perform the
 * PDC DMA operation.  All fields except dma_intr_handler() are initialized
 * by the interface.  The dma_intr_handler() pointer is set by the PCM
 * driver and called by the interface SSC interrupt handler if it is
 * non-NULL.
 */
#[repr(C)]
pub struct atmel_pcm_dma_params {
    pub name: *mut ::core::ffi::c_char, /* stream identifier */
    pub pdc_xfer_size: ::core::ffi::c_int, /* PDC counter increment in bytes */
    pub ssc: *mut ssc_device,          /* SSC device for stream */
    pub pdc: *mut atmel_pdc_regs,      /* PDC receive or transmit registers */
    pub mask: *mut atmel_ssc_mask,     /* SSC & PDC status bits */
    pub substream: *mut snd_pcm_substream,
    pub dma_intr_handler: Option<unsafe extern "C" fn(u32, *mut snd_pcm_substream)>,
}

/*
 * SSC register access (since ssc_writel() / ssc_readl() require literal name)
 */
#[inline]
pub unsafe fn ssc_readx(base: *const u8, reg: usize) -> u32 {
    unsafe { ::core::ptr::read_volatile(base.add(reg) as *const u32) }
}

#[inline]
pub unsafe fn ssc_writex(base: *mut u8, reg: usize, value: u32) {
    unsafe { ::core::ptr::write_volatile(base.add(reg) as *mut u32, value) }
}

/* C conditional intent: #if IS_ENABLED(CONFIG_SND_ATMEL_SOC_PDC) */
unsafe extern "C" {
    pub fn atmel_pcm_pdc_platform_register(dev: *mut device) -> ::core::ffi::c_int;
}
/* C fallback when CONFIG_SND_ATMEL_SOC_PDC is disabled:
 * static inline int atmel_pcm_pdc_platform_register(struct device *dev) { return 0; }
 */

/* C conditional intent: #if IS_ENABLED(CONFIG_SND_ATMEL_SOC_DMA) */
unsafe extern "C" {
    pub fn atmel_pcm_dma_platform_register(dev: *mut device) -> ::core::ffi::c_int;
}
/* C fallback when CONFIG_SND_ATMEL_SOC_DMA is disabled:
 * static inline int atmel_pcm_dma_platform_register(struct device *dev) { return 0; }
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
