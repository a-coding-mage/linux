// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * imx-pcm-dma-mx2.c  --  ALSA Soc Audio Layer
 *
 * Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
 *
 * This code is based on code copyrighted by Freescale,
 * Liam Girdwood, Javier Martin and probably others.
 */

// C includes translated as external dependencies:
// linux/platform_device.h, linux/dmaengine.h, linux/types.h, linux/module.h
// sound/core.h, sound/pcm.h, sound/soc.h, sound/dmaengine_pcm.h
// "imx-pcm.h"

use core::ffi::c_void;

extern "C" {
    static GFP_KERNEL: gfp_t;
    static SND_DMAENGINE_PCM_FLAG_COMPAT: u32;

    fn imx_dma_is_general_purpose(chan: *mut dma_chan) -> bool;
    fn snd_dmaengine_pcm_prepare_slave_config();
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        flags: u32,
    ) -> i32;
}

pub type gfp_t = u32;

pub const ENOMEM: i32 = 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dma_chan {
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
    pub compat_filter_fn: Option<unsafe extern "C" fn(*mut dma_chan, *mut c_void) -> bool>,
}

static mut imx_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    compat_filter_fn: Some(filter),
};

unsafe extern "C" fn filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    if !imx_dma_is_general_purpose(chan) {
        return false;
    }

    (*chan).private = param;

    true
}

pub unsafe extern "C" fn imx_pcm_dma_init(pdev: *mut platform_device) -> i32 {
    let config: *mut snd_dmaengine_pcm_config;

    config = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<snd_dmaengine_pcm_config>(),
        GFP_KERNEL,
    ) as *mut snd_dmaengine_pcm_config;
    if config.is_null() {
        return -ENOMEM;
    }
    *config = imx_dmaengine_pcm_config;

    devm_snd_dmaengine_pcm_register(
        &mut (*pdev).dev,
        config,
        SND_DMAENGINE_PCM_FLAG_COMPAT,
    )
}

// EXPORT_SYMBOL_GPL(imx_pcm_dma_init);

// MODULE_DESCRIPTION("Freescale i.MX PCM DMA interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
