/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm/pgalloc.h.  The included Linux and asm-generic
 * declarations are supplied by the surrounding translation unit. */

pub const __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL: bool = true;

extern "C" {
    fn set_pmd(pmd: *mut pmd_t, value: pmd_t);
    fn __pmd(value: usize) -> pmd_t;
    fn __pa(addr: *const core::ffi::c_void) -> usize;
    fn page_address(page: pgtable_t) -> *mut core::ffi::c_void;

    fn __pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t;
    fn __pgd_alloc(mm: *mut mm_struct, order: i32) -> *mut pgd_t;
    fn pgd_offset(mm: *mut mm_struct, address: usize) -> *mut pgd_t;
    fn pgd_init(p: *mut usize);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn smp_mb();
    fn dcache_wb_range(start: u32, end: u32);
    fn tlb_remove_ptdesc(tlb: *mut mmu_gather, ptdesc: *mut page_ptdesc);
    fn page_ptdesc(page: *mut page) -> *mut page_ptdesc;
}

extern "C" {
    static mut init_mm: mm_struct;
}

extern "C" {
    fn pagetable_init();
    fn mmu_init(min_pfn: usize, max_pfn: usize);
    fn pre_trap_init();
}

pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    let _ = mm;
    set_pmd(pmd, __pmd(__pa(pte as *const core::ffi::c_void)));
}

pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: pgtable_t) {
    let _ = mm;
    set_pmd(pmd, __pmd(__pa(page_address(pte) as *const core::ffi::c_void)));
}

pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    let pte = __pte_alloc_one_kernel(mm);
    if pte.is_null() {
        return core::ptr::null_mut();
    }

    let count = PAGE_SIZE / core::mem::size_of::<pte_t>();
    for i in 0..count {
        (*pte.add(i)).pte_low = _PAGE_GLOBAL;
    }

    pte
}

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let ret = __pgd_alloc(mm, 0);
    if !ret.is_null() {
        let init = pgd_offset(&mut init_mm, 0usize);
        pgd_init(ret as *mut usize);
        memcpy(
            ret.add(USER_PTRS_PER_PGD) as *mut core::ffi::c_void,
            init.add(USER_PTRS_PER_PGD) as *const core::ffi::c_void,
            (PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>(),
        );
        /* prevent out of order excute */
        smp_mb();
        /* CONFIG_CPU_NEED_TLBSYNC */
        #[cfg(CONFIG_CPU_NEED_TLBSYNC)]
        dcache_wb_range(ret as u32, ret.add(PTRS_PER_PGD) as u32);
    }

    ret
}

pub unsafe fn __pte_free_tlb(tlb: *mut mmu_gather, pte: *mut page) {
    tlb_remove_ptdesc(tlb, page_ptdesc(pte));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
