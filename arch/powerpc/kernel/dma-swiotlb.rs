// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Contains routines needed to support swiotlb for ppc.
 *
 * Copyright (C) 2009-2010 Freescale Semiconductor, Inc.
 * Author: Becky Bruce
 */

// Declarations supplied by the Linux memory-management and PPC swiotlb
// dependencies.
extern "C" {
    fn memblock_end_of_DRAM() -> u64;
    fn swiotlb_print_info();
    fn swiotlb_exit();
}

pub static mut ppc_swiotlb_enable: u32 = 0;
pub static mut ppc_swiotlb_flags: u32 = 0;

pub unsafe extern "C" fn swiotlb_detect_4g() {
    if (memblock_end_of_DRAM().wrapping_sub(1)) > 0xffff_ffff {
        ppc_swiotlb_enable = 1;
    }
}

unsafe extern "C" fn check_swiotlb_enabled() -> i32 {
    if ppc_swiotlb_enable != 0 {
        swiotlb_print_info();
    } else {
        swiotlb_exit();
    }

    0
}

// Equivalent registration for the C subsys_initcall(check_swiotlb_enabled)
// macro; the initcall mechanism is supplied by the surrounding kernel.
#[allow(dead_code)]
const _: unsafe extern "C" fn() -> i32 = check_swiotlb_enabled;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
