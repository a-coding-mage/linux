// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
 */

// C header guard omitted in Rust: __EP93XX_PCM_H__
// External dependency from C: struct device.

extern "C" {
    pub fn devm_ep93xx_pcm_platform_register(dev: *mut device) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
