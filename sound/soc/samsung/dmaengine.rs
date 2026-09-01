// SPDX-License-Identifier: GPL-2.0
//
// dmaengine.c - Samsung dmaengine wrapper
//
// Author: Mark Brown <broonie@linaro.org>
// Copyright 2013 Linaro

// C dependencies translated as external items:
// linux/module.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/dmaengine_pcm.h, sound/soc.h, and "dma.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const GFP_KERNEL: c_uint = 0;
pub const ENOMEM: c_int = 12;
pub const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
pub const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
pub const SND_DMAENGINE_PCM_FLAG_COMPAT: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type dma_filter_fn = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>;

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
    pub compat_filter_fn: dma_filter_fn,
    pub dma_dev: *mut device,
    pub chan_names: [*const c_char; 2],
}

unsafe extern "C" {
    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    pub static snd_dmaengine_pcm_prepare_slave_config: Option<unsafe extern "C" fn()>;
    pub fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn samsung_asoc_dma_platform_register(
    dev: *mut device,
    filter: dma_filter_fn,
    tx: *const c_char,
    rx: *const c_char,
    dma_dev: *mut device,
) -> c_int {
    let pcm_conf: *mut snd_dmaengine_pcm_config;

    pcm_conf = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<snd_dmaengine_pcm_config>(),
            GFP_KERNEL,
        ) as *mut snd_dmaengine_pcm_config
    };
    if pcm_conf.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*pcm_conf).prepare_slave_config = snd_dmaengine_pcm_prepare_slave_config;
        (*pcm_conf).compat_filter_fn = filter;
        (*pcm_conf).dma_dev = dma_dev;

        (*pcm_conf).chan_names[SNDRV_PCM_STREAM_PLAYBACK] = tx;
        (*pcm_conf).chan_names[SNDRV_PCM_STREAM_CAPTURE] = rx;

        devm_snd_dmaengine_pcm_register(dev, pcm_conf, SND_DMAENGINE_PCM_FLAG_COMPAT)
    }
}

// EXPORT_SYMBOL_GPL(samsung_asoc_dma_platform_register);

// MODULE_AUTHOR("Mark Brown <broonie@linaro.org>");
// MODULE_DESCRIPTION("Samsung dmaengine ASoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
