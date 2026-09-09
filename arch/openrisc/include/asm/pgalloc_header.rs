/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependencies supplied by asm/page.h, linux/threads.h, linux/mm.h,
// linux/memblock.h, and asm-generic/pgalloc.h remain external.

pub const __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL: bool = true;

pub static mut mem_init_done: std::ffi::c_int;

#[inline]
pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut core::ffi::c_void,
) {
    let _ = mm;
    set_pmd(pmd, __pmd(_KERNPG_TABLE + __pa(pte)));
}

#[inline]
pub unsafe fn pmd_populate(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut page,
) {
    let _ = mm;
    set_pmd(
        pmd,
        __pmd(
            _KERNPG_TABLE
                + ((page_to_pfn(pte) as unsigned_long)
                    << (PAGE_SHIFT as unsigned_long)),
        ),
    );
}

/*
 * Allocate and free page tables.
 */
#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let ret: *mut pgd_t = __pgd_alloc(mm, 0);

    if !ret.is_null() {
        memcpy(
            ret.add(USER_PTRS_PER_PGD),
            swapper_pg_dir.add(USER_PTRS_PER_PGD),
            (PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>(),
        );
    }

    ret
}

/* FIXME: This seems to be the preferred style, but we are using
 * current_pgd (from mm->pgd) to load kernel pages so we need it
 * initialized.  This needs to be looked into.
 */
/*
pub unsafe fn pgd_alloc_preferred(mm: *mut mm_struct) -> *mut pgd_t {
    get_zeroed_page(GFP_KERNEL) as *mut pgd_t
}
*/

extern "C" {
    pub fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t;
}

#[inline]
pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pte: *mut page,
    addr: unsigned_long,
) {
    let _ = addr;
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
