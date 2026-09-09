/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PA-RISC KFENCE support.
 *
 * Copyright (C) 2021, Helge Deller <deller@gmx.de>
 */

// Dependencies supplied by the corresponding Linux/Rust translation units:
// linux/kfence.h, asm/pgtable.h, and asm/tlbflush.h.

/// Equivalent of `static inline bool arch_kfence_init_pool(void)`.
#[inline]
pub fn arch_kfence_init_pool() -> bool {
    true
}

/* Protect the given page and flush TLB. */
#[inline]
pub unsafe fn kfence_protect_page(addr: ::core::ffi::c_ulong, protect: bool) -> bool {
    let pte: *mut crate::pte_t = crate::virt_to_kpte(addr);

    if crate::WARN_ON(pte.is_null()) {
        return false;
    }

    /*
     * We need to avoid IPIs, as we may get KFENCE allocations or faults
     * with interrupts disabled.
     */

    if protect {
        crate::set_pte(
            pte,
            crate::__pte(crate::pte_val(*pte) & !crate::_PAGE_PRESENT),
        );
    } else {
        crate::set_pte(
            pte,
            crate::__pte(crate::pte_val(*pte) | crate::_PAGE_PRESENT),
        );
    }

    crate::flush_tlb_kernel_range(addr, addr + crate::PAGE_SIZE);

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
