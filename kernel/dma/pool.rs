// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2020 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct dma_gen_pool {
    cc_shared: bool,
    pool: *mut gen_pool,
}

static mut ATOMIC_POOL_DMA: dma_gen_pool = dma_gen_pool { cc_shared: false, pool: core::ptr::null_mut() };
static mut POOL_SIZE_DMA: c_ulong = 0;
static mut ATOMIC_POOL_DMA32: dma_gen_pool = dma_gen_pool { cc_shared: false, pool: core::ptr::null_mut() };
static mut POOL_SIZE_DMA32: c_ulong = 0;
static mut ATOMIC_POOL_KERNEL: dma_gen_pool = dma_gen_pool { cc_shared: false, pool: core::ptr::null_mut() };
static mut POOL_SIZE_KERNEL: c_ulong = 0;

/* Size can be defined by the coherent_pool command line */
static mut ATOMIC_POOL_SIZE: usize = 0;

/* Dynamic background expansion when the atomic pool is near capacity */
static mut ATOMIC_POOL_WORK: work_struct = unsafe { core::mem::zeroed() };

unsafe fn early_coherent_pool(mut p: *mut c_char) -> c_int {
    ATOMIC_POOL_SIZE = memparse(p, &mut p);
    0
}
// early_param("coherent_pool", early_coherent_pool);

unsafe fn dma_atomic_pool_debugfs_init() {
    let root = debugfs_create_dir(c"dma_pools".as_ptr(), core::ptr::null_mut());
    debugfs_create_ulong(c"pool_size_dma".as_ptr(), 0o400, root, &mut POOL_SIZE_DMA);
    debugfs_create_ulong(c"pool_size_dma32".as_ptr(), 0o400, root, &mut POOL_SIZE_DMA32);
    debugfs_create_ulong(c"pool_size_kernel".as_ptr(), 0o400, root, &mut POOL_SIZE_KERNEL);
}

unsafe fn dma_atomic_pool_size_add(gfp: gfp_t, size: usize) {
    if gfp & __GFP_DMA != 0 { POOL_SIZE_DMA += size as c_ulong; }
    else if gfp & __GFP_DMA32 != 0 { POOL_SIZE_DMA32 += size as c_ulong; }
    else { POOL_SIZE_KERNEL += size as c_ulong; }
}

unsafe fn cma_in_zone(gfp: gfp_t) -> bool {
    let cma = dev_get_cma_area(core::ptr::null_mut());
    if cma.is_null() { return false; }
    let size = cma_get_size(cma);
    if size == 0 { return false; }
    let end = cma_get_base(cma) + size - 1;
    if IS_ENABLED(CONFIG_ZONE_DMA) && gfp & GFP_DMA != 0 { return end <= zone_dma_limit; }
    if IS_ENABLED(CONFIG_ZONE_DMA32) && gfp & GFP_DMA32 != 0 { return end <= core::cmp::max(DMA_BIT_MASK(32), zone_dma_limit); }
    true
}

unsafe fn atomic_pool_expand(dma_pool: *mut dma_gen_pool, mut pool_size: usize, gfp: gfp_t) -> c_int {
    let mut order = core::cmp::min(get_order(pool_size), MAX_PAGE_ORDER);
    let mut page: *mut page = core::ptr::null_mut();
    let mut leak_pages = false;
    let mut ret = -ENOMEM;
    let addr: *mut c_void;

    loop {
        pool_size = 1usize << (PAGE_SHIFT + order);
        if cma_in_zone(gfp) { page = dma_alloc_from_contiguous(core::ptr::null_mut(), 1usize << order, order, false); }
        if page.is_null() { page = alloc_pages(gfp | __GFP_NOWARN, order); }
        if !page.is_null() || order == 0 { break; }
        order -= 1;
    }
    if page.is_null() { return ret; }
    arch_dma_prep_coherent(page, pool_size);

    // CONFIG_DMA_DIRECT_REMAP conditionally selects the remap path.
    #[cfg(CONFIG_DMA_DIRECT_REMAP)]
    {
        let prot = if (*dma_pool).cc_shared { pgprot_decrypted(pgprot_dmacoherent(PAGE_KERNEL)) } else { pgprot_dmacoherent(PAGE_KERNEL) };
        addr = dma_common_contiguous_remap(page, pool_size, prot, builtin_return_address(0));
        if addr.is_null() { goto_free_page(page, order); }
    }
    #[cfg(not(CONFIG_DMA_DIRECT_REMAP))]
    { addr = page_to_virt(page); }

    if (*dma_pool).cc_shared {
        ret = set_memory_decrypted(page_to_virt(page) as c_ulong, 1usize << order);
        if ret != 0 { leak_pages = true; goto_remove_mapping(dma_pool, addr, page, order, pool_size, leak_pages); }
    }
    ret = gen_pool_add_virt((*dma_pool).pool, addr as c_ulong, page_to_phys(page), pool_size, NUMA_NO_NODE);
    if ret != 0 { goto_encrypt_mapping(dma_pool, addr, page, order, pool_size, leak_pages); }
    dma_atomic_pool_size_add(gfp, pool_size);
    0
}

// The labels below retain the C cleanup ordering and are supplied by the kernel bindings.
unsafe fn atomic_pool_resize(dma_pool: *mut dma_gen_pool, gfp: gfp_t) {
    if !(*dma_pool).pool.is_null() && gen_pool_avail((*dma_pool).pool) < ATOMIC_POOL_SIZE { atomic_pool_expand(dma_pool, gen_pool_size((*dma_pool).pool), gfp); }
}

unsafe fn atomic_pool_work_fn(_work: *mut work_struct) {
    if IS_ENABLED(CONFIG_ZONE_DMA) { atomic_pool_resize(&mut ATOMIC_POOL_DMA, GFP_KERNEL | GFP_DMA); }
    if IS_ENABLED(CONFIG_ZONE_DMA32) { atomic_pool_resize(&mut ATOMIC_POOL_DMA32, GFP_KERNEL | GFP_DMA32); }
    atomic_pool_resize(&mut ATOMIC_POOL_KERNEL, GFP_KERNEL);
}

unsafe fn __dma_atomic_pool_init(dma_pool: *mut dma_gen_pool, pool_size: usize, gfp: gfp_t) -> *mut dma_gen_pool {
    (*dma_pool).pool = gen_pool_create(PAGE_SHIFT, NUMA_NO_NODE);
    if (*dma_pool).pool.is_null() { return core::ptr::null_mut(); }
    gen_pool_set_algo((*dma_pool).pool, gen_pool_first_fit_order_align, core::ptr::null_mut());
    (*dma_pool).cc_shared = cc_platform_has(CC_ATTR_MEM_ENCRYPT);
    if atomic_pool_expand(dma_pool, pool_size, gfp) != 0 {
        gen_pool_destroy((*dma_pool).pool);
        (*dma_pool).pool = core::ptr::null_mut();
        pr_err(c"DMA: failed to allocate atomic pool\n".as_ptr());
        return core::ptr::null_mut();
    }
    pr_info(c"DMA: preallocated atomic pool\n".as_ptr());
    dma_pool
}

unsafe fn dma_atomic_pool_init() -> c_int {
    let mut ret = 0;
    if ATOMIC_POOL_SIZE == 0 {
        let mut pages = totalram_pages() / (SZ_1G / SZ_128K);
        pages = core::cmp::min(pages, MAX_ORDER_NR_PAGES);
        ATOMIC_POOL_SIZE = core::cmp::max(pages << PAGE_SHIFT, SZ_128K);
    }
    INIT_WORK(&mut ATOMIC_POOL_WORK, atomic_pool_work_fn);
    if has_managed_zone(ZONE_NORMAL) {
        __dma_atomic_pool_init(&mut ATOMIC_POOL_KERNEL, ATOMIC_POOL_SIZE, GFP_KERNEL);
        if ATOMIC_POOL_KERNEL.pool.is_null() { ret = -ENOMEM; }
    }
    if has_managed_dma() {
        __dma_atomic_pool_init(&mut ATOMIC_POOL_DMA, ATOMIC_POOL_SIZE, GFP_KERNEL | GFP_DMA);
        if ATOMIC_POOL_DMA.pool.is_null() { ret = -ENOMEM; }
    }
    if has_managed_zone(ZONE_DMA32) {
        __dma_atomic_pool_init(&mut ATOMIC_POOL_DMA32, ATOMIC_POOL_SIZE, GFP_KERNEL | GFP_DMA32);
        if ATOMIC_POOL_DMA32.pool.is_null() { ret = -ENOMEM; }
    }
    dma_atomic_pool_debugfs_init();
    ret
}

unsafe fn __dma_guess_pool(first: *mut dma_gen_pool, second: *mut dma_gen_pool, third: *mut dma_gen_pool) -> *mut dma_gen_pool {
    if !(*first).pool.is_null() { return first; }
    if !second.is_null() && !(*second).pool.is_null() { return second; }
    if !third.is_null() && !(*third).pool.is_null() { return third; }
    core::ptr::null_mut()
}

unsafe fn dma_guess_pool(prev: *mut dma_gen_pool, gfp: gfp_t) -> *mut dma_gen_pool {
    if prev.is_null() {
        if gfp & GFP_DMA != 0 { return __dma_guess_pool(&mut ATOMIC_POOL_DMA, &mut ATOMIC_POOL_DMA32, &mut ATOMIC_POOL_KERNEL); }
        if gfp & GFP_DMA32 != 0 { return __dma_guess_pool(&mut ATOMIC_POOL_DMA32, &mut ATOMIC_POOL_DMA, &mut ATOMIC_POOL_KERNEL); }
        return __dma_guess_pool(&mut ATOMIC_POOL_KERNEL, &mut ATOMIC_POOL_DMA32, &mut ATOMIC_POOL_DMA);
    }
    if prev == &mut ATOMIC_POOL_KERNEL { return __dma_guess_pool(&mut ATOMIC_POOL_DMA32, &mut ATOMIC_POOL_DMA, core::ptr::null_mut()); }
    if prev == &mut ATOMIC_POOL_DMA32 { return __dma_guess_pool(&mut ATOMIC_POOL_DMA, core::ptr::null_mut(), core::ptr::null_mut()); }
    core::ptr::null_mut()
}

unsafe fn __dma_alloc_from_pool(dev: *mut device, size: usize, pool: *mut gen_pool, cpu_addr: *mut *mut c_void, phys_addr_ok: Option<unsafe extern "C" fn(*mut device, phys_addr_t, usize) -> bool>) -> *mut page {
    let addr = gen_pool_alloc(pool, size);
    if addr == 0 { return core::ptr::null_mut(); }
    let phys = gen_pool_virt_to_phys(pool, addr);
    if let Some(ok) = phys_addr_ok { if !ok(dev, phys, size) { gen_pool_free(pool, addr, size); return core::ptr::null_mut(); } }
    if gen_pool_avail(pool) < ATOMIC_POOL_SIZE { schedule_work(&mut ATOMIC_POOL_WORK); }
    *cpu_addr = addr as *mut c_void;
    memset(*cpu_addr, 0, size);
    pfn_to_page(__phys_to_pfn(phys))
}

pub unsafe fn dma_alloc_from_pool(dev: *mut device, size: usize, cpu_addr: *mut *mut c_void, gfp: gfp_t, attrs: c_ulong, phys_addr_ok: Option<unsafe extern "C" fn(*mut device, phys_addr_t, usize) -> bool>) -> *mut page {
    let mut pool = core::ptr::null_mut();
    let mut found = false;
    while { pool = dma_guess_pool(pool, gfp); !pool.is_null() } {
        if (*pool).cc_shared != (attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0) { continue; }
        found = true;
        let page = __dma_alloc_from_pool(dev, size, (*pool).pool, cpu_addr, phys_addr_ok);
        if !page.is_null() { return page; }
    }
    WARN(!found || gfp & __GFP_NOWARN == 0, c"DMA pool unavailable\n".as_ptr());
    core::ptr::null_mut()
}

pub unsafe fn dma_free_from_pool(_dev: *mut device, start: *mut c_void, size: usize) -> bool {
    let mut pool = core::ptr::null_mut();
    while { pool = dma_guess_pool(pool, 0); !pool.is_null() } {
        if gen_pool_has_addr((*pool).pool, start as c_ulong, size) { gen_pool_free((*pool).pool, start as c_ulong, size); return true; }
    }
    false
}

pub unsafe fn dma_free_from_pool_page(dev: *mut device, page: *mut page, size: usize) -> bool {
    if !IS_ENABLED(CONFIG_DMA_DIRECT_REMAP) { return dma_free_from_pool(dev, page_address(page), size); }
    let phys = page_to_phys(page);
    let mut pool = core::ptr::null_mut();
    while { pool = dma_guess_pool(pool, 0); !pool.is_null() } {
        let _ = phys;
        if dma_free_from_pool(pool, page_address(page) as phys_addr_t, size) { return true; }
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
