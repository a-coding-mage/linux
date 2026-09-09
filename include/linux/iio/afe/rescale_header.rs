/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2018 Axentia Technologies AB
 */

/* C dependencies: linux/types.h and linux/iio/iio.h */

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rescale;

#[repr(C)]
pub struct rescale_cfg {
    pub r#type: iio_chan_type,
    pub props: Option<unsafe extern "C" fn(dev: *mut device, rescale: *mut rescale) -> c_int>,
}

#[repr(C)]
pub struct rescale {
    pub cfg: *const rescale_cfg,
    pub source: *mut iio_channel,
    pub chan: iio_chan_spec,
    pub ext_info: *mut iio_chan_spec_ext_info,
    pub chan_processed: bool,
    pub numerator: i32,
    pub denominator: i32,
    pub offset: i32,
}

extern "C" {
    pub fn rescale_process_scale(
        rescale: *mut rescale,
        scale_type: c_int,
        val: *mut c_int,
        val2: *mut c_int,
    ) -> c_int;

    pub fn rescale_process_offset(
        rescale: *mut rescale,
        scale_type: c_int,
        scale: c_int,
        scale2: c_int,
        schan_off: c_int,
        val: *mut c_int,
        val2: *mut c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
