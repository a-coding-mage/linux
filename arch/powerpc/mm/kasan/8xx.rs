// SPDX-License-Identifier: GPL-2.0

// DISABLE_BRANCH_PROFILING
// External dependencies corresponding to the Linux and PowerPC headers are
// supplied by the surrounding translation unit.

unsafe extern "C" {
    fn pmd_off_k(addr: usize) -> *mut pmd_t;
    fn pgd_addr_end(cur: usize, end: usize) -> usize;
    fn pmd_page_vaddr(pmd: pmd_t) -> *mut core::ffi::c_void;
    static kasan_early_shadow_pte: *mut core::ffi::c_void;
    fn memblock_alloc(size: usize, align: usize) -> *mut core::ffi::c_void;
    fn pfn_pte(pfn: usize, prot: usize) -> pte_t;
    fn pte_mkhuge(pte: pte_t) -> pte_t;
    fn phys_pfn(addr: usize) -> usize;
    fn pa(addr: *mut core::ffi::c_void) -> usize;
    fn set_pte_at(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t, urgent: i32);
    fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, ptep: *mut pte_t);
    fn pmd_val(pmd: pmd_t) -> usize;
    fn pmd(value: usize) -> pmd_t;
    fn kasan_mem_to_shadow(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn init_shadow_page_tables(start: usize, end: usize) -> i32;
    fn pte_offset_kernel(pmd: *mut pmd_t, addr: usize) -> *mut pte_t;
    fn flush_tlb_kernel_range(start: usize, end: usize);
    static mut init_mm: mm_struct;
}

// Types and constants are provided by the corresponding kernel headers.
#[allow(non_camel_case_types)]
type pmd_t = crate::pmd_t;
#[allow(non_camel_case_types)]
type pte_t = crate::pte_t;
#[allow(non_camel_case_types)]
type mm_struct = crate::mm_struct;

const SZ_4M: usize = crate::SZ_4M;
const SZ_8M: usize = crate::SZ_8M;
const SZ_512K: usize = crate::SZ_512K;
const PTE_FRAG_SIZE: usize = crate::PTE_FRAG_SIZE;
const PTRS_PER_PTE: usize = crate::PTRS_PER_PTE;
const PAGE_SIZE: usize = crate::PAGE_SIZE;
const PAGE_KERNEL: usize = crate::PAGE_KERNEL;
const PMD_PAGE_8M: usize = crate::_PMD_PAGE_8M;

#[inline]
unsafe fn __pa(addr: *mut core::ffi::c_void) -> usize {
    pa(addr)
}

#[inline]
unsafe fn PHYS_PFN(addr: usize) -> usize {
    phys_pfn(addr)
}

#[inline]
unsafe fn __set_pte_at(
    mm: *mut mm_struct,
    addr: usize,
    ptep: *mut pte_t,
    pte: pte_t,
    urgent: i32,
) {
    set_pte_at(mm, addr, ptep, pte, urgent)
}

#[inline]
unsafe fn __pmd(value: usize) -> pmd_t {
    pmd(value)
}

#[inline]
unsafe fn kasan_init_shadow_page_tables(start: usize, end: usize) -> i32 {
    init_shadow_page_tables(start, end)
}

#[inline]
unsafe fn ALIGN_DOWN(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

#[inline]
unsafe fn IS_ALIGNED(value: usize, alignment: usize) -> bool {
    value & (alignment - 1) == 0
}

#[allow(non_snake_case)]
unsafe fn kasan_init_shadow_8M(
    k_start: usize,
    k_end: usize,
    mut block: *mut core::ffi::c_void,
) -> i32 {
    let mut pmd = pmd_off_k(k_start);
    let mut k_cur: usize = k_start;
    let mut k_next: usize;

    while k_cur != k_end {
        k_next = pgd_addr_end(k_cur, k_end);

        if pmd_page_vaddr(*pmd) != kasan_early_shadow_pte {
            k_cur = k_next;
            pmd = pmd.add(1);
            block = (block as *mut u8).add(SZ_4M) as *mut core::ffi::c_void;
            continue;
        }

        let ptep = memblock_alloc(PTE_FRAG_SIZE, PTE_FRAG_SIZE) as *mut pte_t;
        if ptep.is_null() {
            return -12; // -ENOMEM
        }

        for i in 0..PTRS_PER_PTE {
            let page = (block as *mut u8).add(i * PAGE_SIZE) as *mut core::ffi::c_void;
            let pte = pte_mkhuge(pfn_pte(PHYS_PFN(__pa(page)), PAGE_KERNEL));
            __set_pte_at(&mut init_mm, k_cur, ptep.add(i), pte, 1);
        }
        pmd_populate_kernel(&mut init_mm, pmd, ptep);
        *pmd = __pmd(pmd_val(*pmd) | PMD_PAGE_8M);

        k_cur = k_next;
        pmd = pmd.add(1);
        block = (block as *mut u8).add(SZ_4M) as *mut core::ffi::c_void;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kasan_init_region(
    start: *mut core::ffi::c_void,
    size: usize,
) -> i32 {
    let k_start = kasan_mem_to_shadow(start) as usize;
    let k_end = kasan_mem_to_shadow((start as *mut u8).add(size) as *mut core::ffi::c_void)
        as usize;
    let mut k_cur: usize;
    let block = memblock_alloc(k_end - k_start, SZ_8M);

    if block.is_null() {
        return -12; // -ENOMEM
    }

    if IS_ALIGNED(k_start, SZ_8M) {
        kasan_init_shadow_8M(k_start, ALIGN_DOWN(k_end, SZ_8M), block);
        k_cur = ALIGN_DOWN(k_end, SZ_8M);
        if k_cur == k_end {
            flush_tlb_kernel_range(k_start, k_end);
            return 0;
        }
    } else {
        k_cur = k_start;
    }

    let ret = kasan_init_shadow_page_tables(k_start, k_end);
    if ret != 0 {
        return ret;
    }

    while k_cur < k_end {
        let pmd = pmd_off_k(k_cur);
        let va = (block as *mut u8).add(k_cur - k_start) as *mut core::ffi::c_void;
        let mut pte = pfn_pte(PHYS_PFN(__pa(va)), PAGE_KERNEL);

        if k_cur < ALIGN_DOWN(k_end, SZ_512K) {
            pte = pte_mkhuge(pte);
        }

        __set_pte_at(
            &mut init_mm,
            k_cur,
            pte_offset_kernel(pmd, k_cur),
            pte,
            0,
        );
        k_cur += PAGE_SIZE;
    }

    flush_tlb_kernel_range(k_start, k_end);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
