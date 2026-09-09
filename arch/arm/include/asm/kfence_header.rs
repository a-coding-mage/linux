/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the ARM KFENCE header.
// The declarations referenced below are provided by the corresponding kernel
// dependencies (linux/kfence.h, asm/pgalloc.h, and asm/set_memory.h).

use core::ffi::c_void;

// C: static inline int split_pmd_page(pmd_t *pmd, unsigned long addr)
#[inline]
unsafe fn split_pmd_page(pmd: *mut pmd_t, addr: c_ulong) -> i32 {
    let mut i: i32;
    let pfn: c_ulong = PFN_DOWN(__pa(addr));
    let pte: *mut pte_t = pte_alloc_one_kernel(&init_mm);

    if pte.is_null() {
        return -(ENOMEM as i32);
    }

    i = 0;
    while i < PTRS_PER_PTE as i32 {
        set_pte_ext(
            pte.add(i as usize),
            pfn_pte(pfn + i as c_ulong, PAGE_KERNEL),
            0,
        );
        i += 1;
    }
    pmd_populate_kernel(&init_mm, pmd, pte);

    flush_tlb_kernel_range(addr, addr + PMD_SIZE);
    0
}

// C: static inline bool arch_kfence_init_pool(void)
#[inline]
unsafe fn arch_kfence_init_pool() -> bool {
    let mut addr: c_ulong;
    let mut pmd: *mut pmd_t;

    addr = &__kfence_pool as *const _ as c_ulong;
    while is_kfence_address(addr as *mut c_void) {
        pmd = pmd_off_k(addr);

        if pmd_leaf(*pmd) {
            if split_pmd_page(pmd, addr & PMD_MASK) != 0 {
                return false;
            }
        }
        addr += PAGE_SIZE;
    }

    true
}

// C: static inline bool kfence_protect_page(unsigned long addr, bool protect)
#[inline]
unsafe fn kfence_protect_page(addr: c_ulong, protect: bool) -> bool {
    set_memory_valid(addr, 1, !protect);

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
