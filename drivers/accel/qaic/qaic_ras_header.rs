/* SPDX-License-Identifier: GPL-2.0-only */

/* Copyright (c) 2020, The Linux Foundation. All rights reserved. */

// C header guard: __QAIC_RAS_H__

unsafe extern "C" {
    pub fn qaic_ras_register() -> ::core::ffi::c_int;
    pub fn qaic_ras_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
