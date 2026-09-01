// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/sound/arm/pxa2xx-pcm.c -- ALSA PCM interface for the Intel PXA2xx chip
 *
 * Author:	Nicolas Pitre
 * Created:	Nov 30, 2004
 * Copyright:	(C) 2004 MontaVista Software, Inc.
 */

// C dependencies:
// linux/dma-mapping.h, linux/module.h, linux/dmaengine.h, linux/of.h
// sound/core.h, sound/soc.h, sound/dmaengine_pcm.h
// "pxa2xx-lib.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub pcm_new: Option<unsafe extern "C" fn() -> c_int>,
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub close: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub prepare: Option<unsafe extern "C" fn() -> c_int>,
    pub trigger: Option<unsafe extern "C" fn() -> c_int>,
    pub pointer: Option<unsafe extern "C" fn() -> c_uint>,
}

unsafe extern "C" {
    fn pxa2xx_soc_pcm_new() -> c_int;
    fn pxa2xx_soc_pcm_open() -> c_int;
    fn pxa2xx_soc_pcm_close() -> c_int;
    fn pxa2xx_soc_pcm_hw_params() -> c_int;
    fn pxa2xx_soc_pcm_prepare() -> c_int;
    fn pxa2xx_soc_pcm_trigger() -> c_int;
    fn pxa2xx_soc_pcm_pointer() -> c_uint;

    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

static pxa2xx_soc_platform: snd_soc_component_driver = snd_soc_component_driver {
    pcm_new: Some(pxa2xx_soc_pcm_new),
    open: Some(pxa2xx_soc_pcm_open),
    close: Some(pxa2xx_soc_pcm_close),
    hw_params: Some(pxa2xx_soc_pcm_hw_params),
    prepare: Some(pxa2xx_soc_pcm_prepare),
    trigger: Some(pxa2xx_soc_pcm_trigger),
    pointer: Some(pxa2xx_soc_pcm_pointer),
};

unsafe extern "C" fn pxa2xx_soc_platform_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &pxa2xx_soc_platform,
            ptr::null_mut(),
            0,
        )
    }
}

static mut pxa_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"pxa-pcm-audio".as_ptr(),
    },

    probe: Some(pxa2xx_soc_platform_probe),
};

// module_platform_driver(pxa_pcm_driver);
// MODULE_AUTHOR("Nicolas Pitre");
// MODULE_DESCRIPTION("Intel PXA2xx PCM DMA module");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:pxa-pcm-audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
