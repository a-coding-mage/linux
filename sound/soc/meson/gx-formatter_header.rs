// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/*
 * Copyright (c) 2026 Baylibre SAS.
 * Author: Valerio Setti <vsetti@baylibre.com>
 */

use core::ffi::{c_int, c_uint, c_void};

// C header dependency: #include "gx-interface.h"

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gx_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct gx_formatter_hw {
    pub skew_offset: c_uint,
}

#[repr(C)]
pub struct gx_formatter_ops {
    pub get_stream: Option<
        unsafe extern "C" fn(w: *mut snd_soc_dapm_widget) -> *mut gx_stream,
    >,
    pub enable: Option<unsafe extern "C" fn(map: *mut regmap)>,
    pub disable: Option<unsafe extern "C" fn(map: *mut regmap)>,
    pub prepare: Option<
        unsafe extern "C" fn(
            map: *mut regmap,
            quirks: *const gx_formatter_hw,
            ts: *mut gx_stream,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct gx_formatter_driver {
    pub component_drv: *const snd_soc_component_driver,
    pub regmap_cfg: *const regmap_config,
    pub ops: *const gx_formatter_ops,
    pub quirks: *const gx_formatter_hw,
}

unsafe extern "C" {
    pub fn gx_formatter_event(
        w: *mut snd_soc_dapm_widget,
        control: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;

    pub fn gx_formatter_probe(pdev: *mut platform_device) -> c_int;

    pub fn gx_formatter_create(
        dev: *mut device,
        w: *mut snd_soc_dapm_widget,
        drv: *const gx_formatter_driver,
        regmap: *mut regmap,
    ) -> c_int;
}

/*
 * Formatter data is already freed when the associated device is removed,
 * so we just need to remove the pointer from the widget.
 */
#[inline]
pub unsafe fn gx_formatter_free(w: *mut snd_soc_dapm_widget) {
    unsafe {
        (*w).priv_ = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
