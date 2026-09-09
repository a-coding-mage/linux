// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Device tree support
 *
 * Copyright (C) 2013, 2015 Altera Corporation
 * Copyright (C) 2010 Thomas Chou <thomas@wytron.com.tw>
 *
 * Based on MIPS support for CONFIG_OF device tree support
 *
 * Copyright (C) 2010 Cisco Systems Inc. <dediao@cisco.com>
 */

use core::ffi::c_void;

// Linux kernel declarations supplied by the surrounding translation unit.
extern "C" {
    static __dtb_start: u8;
    fn early_init_dt_scan(params: *mut c_void, phys_addr: usize);
    fn __pa(addr: *mut c_void) -> usize;
    fn be32_to_cpup(addr: *const u32) -> u32;
    fn be32_to_cpu(value: u32) -> u32;
}

// OF_DT_HEADER and CONFIG_NIOS2_DTB_PHYS_ADDR are build-time definitions.
// CONFIG_NIOS2_DTB_AT_PHYS_ADDR and CONFIG_BUILTIN_DTB remain conditional.

pub unsafe fn early_init_devtree(mut params: *mut c_void) {
    let dtb: *mut u32 = &__dtb_start as *const u8 as *mut u32;

    #[cfg(CONFIG_NIOS2_DTB_AT_PHYS_ADDR)]
    {
        if be32_to_cpup(CONFIG_NIOS2_DTB_PHYS_ADDR as *const u32) == OF_DT_HEADER {
            params = CONFIG_NIOS2_DTB_PHYS_ADDR as *mut c_void;
            early_init_dt_scan(params, __pa(params));
            return;
        }
    }

    #[cfg(CONFIG_BUILTIN_DTB)]
    {
        if be32_to_cpu(dtb.read()) == OF_DT_HEADER {
            params = &__dtb_start as *const u8 as *mut c_void;
        }
    }

    early_init_dt_scan(params, __pa(params));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
