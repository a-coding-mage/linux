/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * PROM library functions for acquiring/using memory descriptors given to
 * us from the YAMON.
 *
 * Copyright (C) 1999,2000,2012  MIPS Technologies, Inc.
 * All rights reserved.
 * Authors: Carsten Langgaard <carstenl@mips.com>
 *          Steven J. Hill <sjhill@mips.com>
 */

// Dependencies supplied by the surrounding kernel build.

extern "C" {
    fn free_init_pages(name: *const core::ffi::c_char, begin: usize, end: usize);
    fn __pa_symbol(address: *mut usize) -> usize;

    static mut free_init_pages_eva:
        Option<unsafe extern "C" fn(begin: *mut core::ffi::c_void, end: *mut core::ffi::c_void)>;
}

/* determined physical memory size, not overridden by command line args */
pub static mut physical_memsize: usize = 0;

unsafe extern "C" fn free_init_pages_eva_malta(
    begin: *mut core::ffi::c_void,
    end: *mut core::ffi::c_void,
) {
    free_init_pages(
        b"unused kernel\0".as_ptr() as *const core::ffi::c_char,
        __pa_symbol(begin as *mut usize),
        __pa_symbol(end as *mut usize),
    );
}

pub unsafe extern "C" fn fw_meminit() {
    // IS_ENABLED(CONFIG_EVA): this build-time condition is represented by the
    // surrounding kernel configuration.
    let eva: bool = cfg!(feature = "CONFIG_EVA");

    free_init_pages_eva = if eva {
        Some(free_init_pages_eva_malta)
    } else {
        None
    };
}

pub extern "C" fn mips_cdmm_phys_base() -> usize {
    /* This address is "typically unused" */
    0x1fc10000
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
