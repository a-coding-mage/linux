// SPDX-License-Identifier: GPL-2.0

// Types and symbols are supplied by the surrounding kernel implementation.
use linux_types::u32;

extern "C" {
    static mut __FPU_FPSCR: u32;
}

/// Read the floating-point status and control register into frD.
pub unsafe fn mffs(frD: *mut u32) -> i32 {
    *frD.add(1) = __FPU_FPSCR;

    // The original code is conditionally compiled when DEBUG is defined.
    // DEBUG-only printk support is supplied by the surrounding kernel.

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
