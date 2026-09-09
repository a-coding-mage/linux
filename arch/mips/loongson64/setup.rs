// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// C dependencies supplied by the surrounding kernel translation.

/// External device-tree architecture setup routine.
unsafe extern "C" {
    fn __dt_setup_arch(blob: *mut core::ffi::c_void);
}

pub static mut loongson_fdt_blob: *mut core::ffi::c_void = core::ptr::null_mut();

// __init: kernel initialization function.
pub unsafe fn plat_mem_setup() {
    if !loongson_fdt_blob.is_null() {
        __dt_setup_arch(loongson_fdt_blob);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
