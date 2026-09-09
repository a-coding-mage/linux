/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023-2024 Linaro Ltd.
 */

// Opaque forward declaration of the C `struct device`.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn qcom_tzmem_enable(dev: *mut device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
