// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux suspend and architecture-specific TLB headers.

unsafe extern "C" {
    fn restore_image() -> i32;
    fn local_flush_tlb_all();
}

pub unsafe extern "C" fn swsusp_arch_resume() -> i32 {
    /* Avoid TLB mismatch during and after kernel resume */
    local_flush_tlb_all();
    restore_image()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
