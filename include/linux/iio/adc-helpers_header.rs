/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The industrial I/O ADC firmware property parsing helpers
 *
 * Copyright (c) 2025 Matti Vaittinen <mazziesaccount@gmail.com>
 */

use core::ffi::c_int;

// Dependency supplied by linux/property.h.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_chan_spec {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn device_get_named_child_node_count(
        dev: *mut device,
        name: *const core::ffi::c_char,
    ) -> c_int;

    pub fn devm_iio_adc_device_alloc_chaninfo_se(
        dev: *mut device,
        template: *const iio_chan_spec,
        max_chan_id: c_int,
        cs: *mut *mut iio_chan_spec,
    ) -> c_int;
}

pub unsafe fn iio_adc_device_num_channels(dev: *mut device) -> c_int {
    let name = b"channel\0";
    unsafe { device_get_named_child_node_count(dev, name.as_ptr() as *const core::ffi::c_char) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
