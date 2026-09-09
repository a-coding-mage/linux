/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/platform_data/uio_dmem_genirq.h
 *
 * Copyright (C) 2012 Damian Hobson-Garcia
 */

// Dependency supplied by the Linux UIO driver interface.

#[repr(C)]
pub struct uio_dmem_genirq_pdata {
    pub uioinfo: uio_info,
    pub dynamic_region_sizes: *mut core::ffi::c_uint,
    pub num_dynamic_regions: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
