/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file discribe the STM32 DFSDM IIO driver API for audio part
 *
 * Copyright (C) 2017, STMicroelectronics - All Rights Reserved
 * Author(s): Arnaud Pouliquen <arnaud.pouliquen@st.com>.
 */

// Dependency supplied by <linux/iio/iio.h>.
use core::ffi::{c_int, c_void};

/// Opaque IIO device type supplied by the Linux IIO dependency.
pub struct iio_dev;

extern "C" {
    pub fn stm32_dfsdm_get_buff_cb(
        iio_dev: *mut iio_dev,
        cb: Option<unsafe extern "C" fn(data: *const c_void, size: usize, private: *mut c_void) -> c_int>,
        private: *mut c_void,
    ) -> c_int;

    pub fn stm32_dfsdm_release_buff_cb(iio_dev: *mut iio_dev) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
