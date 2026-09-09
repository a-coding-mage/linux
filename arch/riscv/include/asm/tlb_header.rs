/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// C header guard: _ASM_RISCV_TLB_H

use core::ffi::c_void;

// Opaque dependency supplied by asm-generic/tlb.h.
#[repr(C)]
pub struct mmu_gather {
    pub fullmm: bool,
    pub need_flush_all: bool,
    pub freed_tables: bool,
    pub mm: *mut c_void,
    pub start: usize,
    pub end: usize,
}

extern "C" {
    pub fn flush_tlb_mm(mm: *mut c_void);
    pub fn flush_tlb_mm_range(
        mm: *mut c_void,
        start: usize,
        end: usize,
        unmap_size: usize,
    );
    pub fn tlb_get_unmap_size(tlb: *mut mmu_gather) -> usize;
}

// #define tlb_flush tlb_flush

/// Translation of the C static inline `tlb_flush`.
#[inline]
pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    // The CONFIG_MMU conditional is preserved as a Rust configuration.
    #[cfg(CONFIG_MMU)]
    {
        if (*tlb).fullmm || (*tlb).need_flush_all || (*tlb).freed_tables {
            flush_tlb_mm((*tlb).mm);
        } else {
            flush_tlb_mm_range(
                (*tlb).mm,
                (*tlb).start,
                (*tlb).end,
                tlb_get_unmap_size(tlb),
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
