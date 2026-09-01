/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
 *
 * This code is based on code copyrighted by Freescale,
 * Liam Girdwood, Javier Martin and probably others.
 */

/* Dependency intent from C include: <linux/dma/imx-dma.h> */

/*
 * Do not change this as the FIQ handler depends on this size
 */
pub const IMX_SSI_DMABUF_SIZE: usize = 64 * 1024;

pub const IMX_DEFAULT_DMABUF_SIZE: usize = 64 * 1024;

#[repr(C)]
pub struct imx_pcm_fiq_params {
    pub irq: ::core::ffi::c_int,
    pub base: *mut ::core::ffi::c_void,

    /* Pointer to original ssi driver to setup tx rx sizes */
    pub dma_params_rx: *mut snd_dmaengine_dai_dma_data,
    pub dma_params_tx: *mut snd_dmaengine_dai_dma_data,
}

/* Original C conditional: IS_ENABLED(CONFIG_SND_SOC_IMX_PCM_DMA) */
#[cfg(CONFIG_SND_SOC_IMX_PCM_DMA)]
extern "C" {
    pub fn imx_pcm_dma_init(pdev: *mut platform_device) -> ::core::ffi::c_int;
}

/* Original C fallback when CONFIG_SND_SOC_IMX_PCM_DMA is not enabled. */
#[cfg(not(CONFIG_SND_SOC_IMX_PCM_DMA))]
#[inline]
pub unsafe fn imx_pcm_dma_init(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let _ = pdev;
    -ENODEV
}

/* Original C conditional: IS_ENABLED(CONFIG_SND_SOC_IMX_PCM_FIQ) */
#[cfg(CONFIG_SND_SOC_IMX_PCM_FIQ)]
extern "C" {
    pub fn imx_pcm_fiq_init(
        pdev: *mut platform_device,
        params: *mut imx_pcm_fiq_params,
    ) -> ::core::ffi::c_int;
    pub fn imx_pcm_fiq_exit(pdev: *mut platform_device);
}

/* Original C fallback when CONFIG_SND_SOC_IMX_PCM_FIQ is not enabled. */
#[cfg(not(CONFIG_SND_SOC_IMX_PCM_FIQ))]
#[inline]
pub unsafe fn imx_pcm_fiq_init(
    pdev: *mut platform_device,
    params: *mut imx_pcm_fiq_params,
) -> ::core::ffi::c_int {
    let _ = pdev;
    let _ = params;
    -ENODEV
}

#[cfg(not(CONFIG_SND_SOC_IMX_PCM_FIQ))]
#[inline]
pub unsafe fn imx_pcm_fiq_exit(pdev: *mut platform_device) {
    let _ = pdev;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
