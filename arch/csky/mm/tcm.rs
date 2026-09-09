// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the kernel and architecture-specific Rust bindings.

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pte_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmd_t {
    _private: [u8; 0],
}

extern "C" {
    static mut __tcm_start: core::ffi::c_char;
    static mut __tcm_end: core::ffi::c_char;
    static mut __dtcm_start: core::ffi::c_char;

    fn pfn_valid(pfn: usize) -> bool;
    fn pgd_offset_k(vaddr: usize) -> *mut core::ffi::c_void;
    fn pte_offset_kernel(pmd: *mut pmd_t, vaddr: usize) -> *mut pte_t;
    fn set_pte(pte: *mut pte_t, entry: usize);
    fn pfn_pte(pfn: usize, prot: usize) -> usize;
    fn __phys_to_pfn(addr: usize) -> usize;
    fn __fix_to_virt(index: usize) -> usize;
    fn flush_tlb_one(vaddr: usize);
    fn gen_pool_create(order: i32, nid: i32) -> *mut gen_pool;
    fn gen_pool_add(pool: *mut gen_pool, start: u32, size: u32, nid: i32) -> i32;
    fn gen_pool_alloc(pool: *mut gen_pool, size: usize) -> usize;
    fn gen_pool_free(pool: *mut gen_pool, addr: usize, size: usize);
    fn panic(message: *const core::ffi::c_char) -> !;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn pr_err(format: *const core::ffi::c_char, ...);
}

static mut tcm_pool: *mut gen_pool = core::ptr::null_mut();

unsafe fn tcm_mapping_init() {
    let mut tcm_pte: *mut pte_t;
    let mut vaddr: usize;
    let mut paddr: usize = CONFIG_ITCM_RAM_BASE;
    let mut i: i32;

    if pfn_valid(PFN_DOWN(CONFIG_ITCM_RAM_BASE)) {
        panic(c"TCM init error".as_ptr());
    }

    // CONFIG_HAVE_DTCM selects CONFIG_ITCM_NR_PAGES; otherwise TCM_NR_PAGES.
    #[cfg(feature = "CONFIG_HAVE_DTCM")]
    let nr_pages = CONFIG_ITCM_NR_PAGES;
    #[cfg(not(feature = "CONFIG_HAVE_DTCM"))]
    let nr_pages = TCM_NR_PAGES;

    i = 0;
    while i < nr_pages {
        vaddr = __fix_to_virt(FIX_TCM - i as usize);
        tcm_pte = pte_offset_kernel(pgd_offset_k(vaddr) as *mut pmd_t, vaddr);
        set_pte(tcm_pte, pfn_pte(__phys_to_pfn(paddr), PAGE_KERNEL));
        flush_tlb_one(vaddr);
        paddr = paddr.wrapping_add(PAGE_SIZE);
        i += 1;
    }

    #[cfg(feature = "CONFIG_HAVE_DTCM")]
    {
        if pfn_valid(PFN_DOWN(CONFIG_DTCM_RAM_BASE)) {
            panic(c"TCM init error".as_ptr());
        }
        paddr = CONFIG_DTCM_RAM_BASE;
        i = 0;
        while i < CONFIG_DTCM_NR_PAGES {
            vaddr = __fix_to_virt(FIX_TCM - CONFIG_ITCM_NR_PAGES - i as usize);
            tcm_pte = pte_offset_kernel(pgd_offset_k(vaddr) as *mut pmd_t, vaddr);
            set_pte(tcm_pte, pfn_pte(__phys_to_pfn(paddr), PAGE_KERNEL));
            flush_tlb_one(vaddr);
            paddr = paddr.wrapping_add(PAGE_SIZE);
            i += 1;
        }

        memcpy(__fix_to_virt(FIX_TCM) as *mut _, &__tcm_start as *const _ as *const _,
            &__dtcm_start as *const _ as usize - &__tcm_start as *const _ as usize);
        memcpy(__fix_to_virt(FIX_TCM - CONFIG_ITCM_NR_PAGES) as *mut _, &__dtcm_start as *const _ as *const _,
            &__tcm_end as *const _ as usize - &__dtcm_start as *const _ as usize);

        pr_info(c"%s: mapping itcm va:0x%08lx to pa:0x%08x\n".as_ptr(), c"tcm_mapping_init".as_ptr(), __fix_to_virt(FIX_TCM), CONFIG_ITCM_RAM_BASE);
        pr_info(c"%s: __itcm_start va:0x%08lx size:%d\n".as_ptr(), c"tcm_mapping_init".as_ptr(), &__tcm_start as *const _ as usize, &__dtcm_start as *const _ as usize - &__tcm_start as *const _ as usize);
        pr_info(c"%s: mapping dtcm va:0x%08lx to pa:0x%08x\n".as_ptr(), c"tcm_mapping_init".as_ptr(), __fix_to_virt(FIX_TCM - CONFIG_ITCM_NR_PAGES), CONFIG_DTCM_RAM_BASE);
        pr_info(c"%s: __dtcm_start va:0x%08lx size:%d\n".as_ptr(), c"tcm_mapping_init".as_ptr(), &__dtcm_start as *const _ as usize, &__tcm_end as *const _ as usize - &__dtcm_start as *const _ as usize);
    }

    #[cfg(not(feature = "CONFIG_HAVE_DTCM"))]
    {
        memcpy(__fix_to_virt(FIX_TCM) as *mut _, &__tcm_start as *const _ as *const _,
            &__tcm_end as *const _ as usize - &__tcm_start as *const _ as usize);
        pr_info(c"%s: mapping tcm va:0x%08lx to pa:0x%08x\n".as_ptr(), c"tcm_mapping_init".as_ptr(), __fix_to_virt(FIX_TCM), CONFIG_ITCM_RAM_BASE);
        pr_info(c"%s: __tcm_start va:0x%08lx size:%d\n".as_ptr(), c"tcm_mapping_init".as_ptr(), &__tcm_start as *const _ as usize, &__tcm_end as *const _ as usize - &__tcm_start as *const _ as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcm_alloc(len: usize) -> *mut core::ffi::c_void {
    if tcm_pool.is_null() { return core::ptr::null_mut(); }
    let vaddr = gen_pool_alloc(tcm_pool, len);
    if vaddr == 0 { return core::ptr::null_mut(); }
    vaddr as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn tcm_free(addr: *mut core::ffi::c_void, len: usize) {
    gen_pool_free(tcm_pool, addr as usize, len);
}

unsafe fn tcm_setup_pool() -> i32 {
    // The two pool layouts are selected by CONFIG_HAVE_DTCM.
    #[cfg(feature = "CONFIG_HAVE_DTCM")]
    let pool_size = (CONFIG_DTCM_NR_PAGES * PAGE_SIZE) as u32
        - (&__tcm_end as *const _ as u32 - &__dtcm_start as *const _ as u32);
    #[cfg(not(feature = "CONFIG_HAVE_DTCM"))]
    let pool_size = (TCM_NR_PAGES * PAGE_SIZE) as u32
        - (&__tcm_end as *const _ as u32 - &__tcm_start as *const _ as u32);
    #[cfg(feature = "CONFIG_HAVE_DTCM")]
    let tcm_pool_start = __fix_to_virt(FIX_TCM - CONFIG_ITCM_NR_PAGES) as u32
        + (&__tcm_end as *const _ as u32 - &__dtcm_start as *const _ as u32);
    #[cfg(not(feature = "CONFIG_HAVE_DTCM"))]
    let tcm_pool_start = __fix_to_virt(FIX_TCM) as u32
        + (&__tcm_end as *const _ as u32 - &__tcm_start as *const _ as u32);
    tcm_pool = gen_pool_create(2, -1);
    let ret = gen_pool_add(tcm_pool, tcm_pool_start, pool_size, -1);
    if ret != 0 {
        pr_err(c"%s: gen_pool add failed!\n".as_ptr(), c"tcm_setup_pool".as_ptr());
        return ret;
    }
    pr_info(c"%s: Added %d bytes @ 0x%08x to memory pool\n".as_ptr(), c"tcm_setup_pool".as_ptr(), pool_size, tcm_pool_start);
    0
}

unsafe fn tcm_init() -> i32 {
    tcm_mapping_init();
    tcm_setup_pool();
    0
}

// EXPORT_SYMBOL(tcm_alloc), EXPORT_SYMBOL(tcm_free), and arch_initcall(tcm_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
