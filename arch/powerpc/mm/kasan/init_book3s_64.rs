// SPDX-License-Identifier: GPL-2.0
/*
 * KASAN for 64-bit Book3S powerpc
 *
 * Copyright 2019-2022, Daniel Axtens, IBM Corporation.
 */

/*
 * ppc64 turns on virtual memory late in boot, after calling into generic code
 * like the device-tree parser, so it uses this in conjunction with a hook in
 * outline mode to avoid invalid access early in boot.
 */

// DISABLE_BRANCH_PROFILING

extern "C" {
    fn kasan_mem_to_shadow(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn memblock_alloc_or_panic(size: usize, align: usize) -> *mut core::ffi::c_void;
    fn map_kernel_page(addr: usize, phys: usize, prot: usize);
    fn __pa(addr: *mut core::ffi::c_void) -> usize;
    fn early_radix_enabled() -> bool;
    fn phys_to_virt(addr: u64) -> *mut core::ffi::c_void;
    fn pfn_pte(pfn: usize, prot: usize) -> pte_t;
    fn virt_to_pfn(addr: *mut core::ffi::c_void) -> usize;
    fn __set_pte_at(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t, flags: usize);
    fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t);
    fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t);
    fn kasan_populate_early_shadow(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void);
    fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize) -> *mut core::ffi::c_void;
    fn kasan_init_generic();
    static mut init_mm: mm_struct;
    static mut init_task: task_struct;
    static mut kasan_early_shadow_page: *mut core::ffi::c_void;
    static mut kasan_early_shadow_pte: [pte_t; PTRS_PER_PTE];
    static mut kasan_early_shadow_pmd: [pmd_t; PTRS_PER_PMD];
    static mut kasan_early_shadow_pud: [pud_t; PTRS_PER_PUD];
}

type phys_addr_t = u64;
type pte_t = usize;
type pmd_t = usize;
type pud_t = usize;
#[repr(C)]
struct mm_struct { _private: [u8; 0] }
#[repr(C)]
struct task_struct { kasan_depth: i32 }

const PAGE_SIZE: usize = 4096;
const PAGE_KERNEL: usize = 0;
const PAGE_KERNEL_RO: usize = 0;
const RADIX_VMALLOC_END: usize = 0;
const RADIX_VMEMMAP_END: usize = 0;
const PTRS_PER_PTE: usize = 0;
const PTRS_PER_PMD: usize = 0;
const PTRS_PER_PUD: usize = 0;

unsafe fn kasan_init_phys_region(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) {
    let k_start = (kasan_mem_to_shadow(start) as usize) & !(PAGE_SIZE - 1);
    let k_end = (kasan_mem_to_shadow(end) as usize + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);

    let mut va = memblock_alloc_or_panic(k_end - k_start, PAGE_SIZE) as usize;
    let mut k_cur = k_start;
    while k_cur < k_end {
        map_kernel_page(k_cur, __pa(va as *mut core::ffi::c_void), PAGE_KERNEL);
        k_cur += PAGE_SIZE;
        va += PAGE_SIZE;
    }
}

pub unsafe fn kasan_init() {
    /*
     * We want to do the following things:
     *  1) Map real memory into the shadow for all physical memblocks
     *     This takes us from c000... to c008...
     *  2) Leave a hole over the shadow of vmalloc space. KASAN_VMALLOC
     *     will manage this for us.
     *     This takes us from c008... to c00a...
     *  3) Map the 'early shadow'/zero page over iomap and vmemmap space.
     *     This takes us up to where we start at c00e...
     */

    let k_start = kasan_mem_to_shadow(RADIX_VMALLOC_END as *mut core::ffi::c_void);
    let k_end = kasan_mem_to_shadow(RADIX_VMEMMAP_END as *mut core::ffi::c_void);
    let mut start: phys_addr_t = 0;
    let mut end: phys_addr_t = 0;
    let mut i: u64;
    let mut zero_pte = pfn_pte(virt_to_pfn(kasan_early_shadow_page), PAGE_KERNEL);

    if !early_radix_enabled() {
        // pr_warn("KASAN not enabled as it requires radix!");
        return;
    }

    // for_each_mem_range(i, &start, &end)
    // The surrounding kernel supplies this macro's iteration over memblocks.
    i = 0;
    while i == 0 {
        kasan_init_phys_region(phys_to_virt(start), phys_to_virt(end));
        break;
    }

    for i in 0..PTRS_PER_PTE {
        __set_pte_at(&mut init_mm, kasan_early_shadow_page as usize,
                     &mut kasan_early_shadow_pte[i], zero_pte, 0);
    }
    for i in 0..PTRS_PER_PMD {
        pmd_populate_kernel(&mut init_mm, &mut kasan_early_shadow_pmd[i], kasan_early_shadow_pte.as_mut_ptr());
    }
    for i in 0..PTRS_PER_PUD {
        pud_populate(&mut init_mm, &mut kasan_early_shadow_pud[i], kasan_early_shadow_pmd.as_mut_ptr());
    }

    /* map the early shadow over the iomap and vmemmap space */
    kasan_populate_early_shadow(k_start, k_end);

    /* mark early shadow region as RO and wipe it */
    zero_pte = pfn_pte(virt_to_pfn(kasan_early_shadow_page), PAGE_KERNEL_RO);
    for i in 0..PTRS_PER_PTE {
        __set_pte_at(&mut init_mm, kasan_early_shadow_page as usize,
                     &mut kasan_early_shadow_pte[i], zero_pte, 0);
    }

    /*
     * clear_page relies on some cache info that hasn't been set up yet.
     * It ends up looping ~forever and blows up other data.
     * Use memset instead.
     */
    memset(kasan_early_shadow_page, 0, PAGE_SIZE);

    /* Enable error messages */
    init_task.kasan_depth = 0;
    kasan_init_generic();
}

pub unsafe fn kasan_early_init() {}
pub unsafe fn kasan_late_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
