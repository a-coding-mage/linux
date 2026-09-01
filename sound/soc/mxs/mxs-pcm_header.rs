// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 */

// Original C dependency: struct device.
use crate::device;

unsafe extern "C" {
    pub fn mxs_pcm_platform_register(dev: *mut device) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
