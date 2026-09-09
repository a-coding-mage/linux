// SPDX-License-Identifier: GPL-2.0+
/*
 * Contiguous Memory Allocator for DMA mapping framework
 * Copyright (c) 2010-2011 by Samsung Electronics.
 * Written by:
 *	Marek Szyprowski <m.szyprowski@samsung.com>
 *	Michal Nazarewicz <mina86@mina86.com>
 *
 * Contiguous Memory Allocator
 *
 * The Contiguous Memory Allocator (CMA) makes it possible to allocate big
 * contiguous chunks of memory after the system has booted.
 */

// C dependencies supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_CMA_SIZE_MBYTES")]
const CMA_SIZE_MBYTES: usize = CONFIG_CMA_SIZE_MBYTES;
#[cfg(not(feature = "CONFIG_CMA_SIZE_MBYTES"))]
const CMA_SIZE_MBYTES: usize = 0;

static mut DMA_CONTIGUOUS_AREAS: [*mut cma; MAX_CMA_AREAS] = [core::ptr::null_mut(); MAX_CMA_AREAS];
static mut DMA_CONTIGUOUS_AREAS_NUM: c_uint = 0;

unsafe fn dma_contiguous_insert_area(area: *mut cma) -> c_int {
    if DMA_CONTIGUOUS_AREAS_NUM as usize >= MAX_CMA_AREAS { return -EINVAL; }
    DMA_CONTIGUOUS_AREAS[DMA_CONTIGUOUS_AREAS_NUM as usize] = area;
    DMA_CONTIGUOUS_AREAS_NUM += 1;
    0
}

/// Get contiguous area at given index.
#[no_mangle]
pub unsafe extern "C" fn dma_contiguous_get_area_by_idx(idx: c_uint) -> *mut cma {
    if idx >= DMA_CONTIGUOUS_AREAS_NUM { return core::ptr::null_mut(); }
    DMA_CONTIGUOUS_AREAS[idx as usize]
}

static mut DMA_CONTIGUOUS_DEFAULT_AREA: *mut cma = core::ptr::null_mut();
const SIZE_BYTES: phys_addr_t = (CMA_SIZE_MBYTES as phys_addr_t) * SZ_1M;
static mut SIZE_CMDLINE: phys_addr_t = !0;
static mut BASE_CMDLINE: phys_addr_t = 0;
static mut LIMIT_CMDLINE: phys_addr_t = 0;

unsafe extern "C" fn early_cma(mut p: *mut c_char) -> c_int {
    if p.is_null() { pr_err!("Config string not provided\n"); return -EINVAL; }
    SIZE_CMDLINE = memparse(p, &mut p);
    if *p != b'@' as c_char { return 0; }
    BASE_CMDLINE = memparse(p.add(1), &mut p);
    if *p != b'-' as c_char { LIMIT_CMDLINE = BASE_CMDLINE + SIZE_CMDLINE; return 0; }
    LIMIT_CMDLINE = memparse(p.add(1), &mut p);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dev_get_cma_area(dev: *mut device) -> *mut cma {
    if !dev.is_null() && !(*dev).cma_area.is_null() { (*dev).cma_area } else { DMA_CONTIGUOUS_DEFAULT_AREA }
}

#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
static mut DMA_CONTIGUOUS_NUMA_AREA: [*mut cma; MAX_NUMNODES] = [core::ptr::null_mut(); MAX_NUMNODES];
#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
static mut NUMA_CMA_SIZE: [phys_addr_t; MAX_NUMNODES] = [0; MAX_NUMNODES];
#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
static mut PERNUMA_SIZE_BYTES: phys_addr_t = 0;
#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
static mut NUMA_CMA_CONFIGURED: bool = false;

#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
unsafe extern "C" fn early_numa_cma(mut p: *mut c_char) -> c_int {
    let mut s = p; let mut nid: c_int; let mut count: c_int = 0; let mut node: c_ulong;
    while *s != 0 {
        if sscanf(s, b"%lu%n\0".as_ptr() as *const c_char, &mut node, &mut count) != 1 { break; }
        if *s.add(count as usize) != b':' as c_char { break; }
        if node >= MAX_NUMNODES as c_ulong { break; }
        nid = array_index_nospec(node, MAX_NUMNODES as c_ulong) as c_int;
        s = s.add(count as usize + 1);
        NUMA_CMA_SIZE[nid as usize] = memparse(s, &mut s);
        if *s == b',' as c_char { s = s.add(1); } else { break; }
    }
    NUMA_CMA_CONFIGURED = true; 0
}

#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
unsafe extern "C" fn early_cma_pernuma(mut p: *mut c_char) -> c_int {
    PERNUMA_SIZE_BYTES = memparse(p, &mut p); NUMA_CMA_CONFIGURED = true; 0
}

#[cfg(feature = "CONFIG_CMA_SIZE_PERCENTAGE")]
unsafe fn cma_early_percent_memory() -> phys_addr_t {
    let total_pages: c_ulong = PHYS_PFN(memblock_phys_mem_size());
    (total_pages * CONFIG_CMA_SIZE_PERCENTAGE as c_ulong / 100) << PAGE_SHIFT
}
#[cfg(not(feature = "CONFIG_CMA_SIZE_PERCENTAGE"))]
unsafe fn cma_early_percent_memory() -> phys_addr_t { 0 }

#[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
unsafe fn dma_numa_cma_reserve() {
    if IS_ENABLED(CONFIG_CMA_SIZE_PERNUMA) && !NUMA_CMA_CONFIGURED && !DMA_CONTIGUOUS_DEFAULT_AREA.is_null() && nr_online_nodes > 1 {
        PERNUMA_SIZE_BYTES = cma_get_size(DMA_CONTIGUOUS_DEFAULT_AREA);
    }
    for_each_node!(nid => {
        let size = if NUMA_CMA_SIZE[nid as usize] != 0 { NUMA_CMA_SIZE[nid as usize] } else { PERNUMA_SIZE_BYTES };
        if !node_online(nid) || size == 0 { continue; }
        let mut name = [0 as c_char; CMA_MAX_NAME];
        snprintf(name.as_mut_ptr(), name.len(), b"numa%d\0".as_ptr() as *const c_char, nid);
        let ret = cma_declare_contiguous_nid(0, size, 0, 0, 0, false, name.as_ptr(), &mut DMA_CONTIGUOUS_NUMA_AREA[nid as usize], nid);
        if ret != 0 { pr_warn!("%s: reservation failed: err %d, node %d\n", __func__, ret, nid); }
    });
}
#[cfg(not(feature = "CONFIG_DMA_NUMA_CMA"))]
unsafe fn dma_numa_cma_reserve() {}

#[no_mangle]
pub unsafe extern "C" fn dma_contiguous_reserve(limit: phys_addr_t) {
    let mut selected_size = 0; let mut selected_base = 0; let mut selected_limit = limit; let mut fixed = false;
    if SIZE_CMDLINE != !0 {
        selected_size = SIZE_CMDLINE; selected_base = BASE_CMDLINE; selected_limit = if LIMIT_CMDLINE != 0 { LIMIT_CMDLINE } else { limit };
        if BASE_CMDLINE + SIZE_CMDLINE == LIMIT_CMDLINE { fixed = true; }
    } else {
        #[cfg(feature = "CONFIG_CMA_SIZE_SEL_MBYTES")] { selected_size = SIZE_BYTES; }
        #[cfg(feature = "CONFIG_CMA_SIZE_SEL_PERCENTAGE")] { selected_size = cma_early_percent_memory(); }
        #[cfg(feature = "CONFIG_CMA_SIZE_SEL_MIN")] { selected_size = core::cmp::min(SIZE_BYTES, cma_early_percent_memory()); }
        #[cfg(feature = "CONFIG_CMA_SIZE_SEL_MAX")] { selected_size = core::cmp::max(SIZE_BYTES, cma_early_percent_memory()); }
    }
    if selected_size != 0 && DMA_CONTIGUOUS_DEFAULT_AREA.is_null() {
        let ret = dma_contiguous_reserve_area(selected_size, selected_base, selected_limit, &mut DMA_CONTIGUOUS_DEFAULT_AREA, fixed);
        if ret != 0 { return; }
        if dma_contiguous_insert_area(DMA_CONTIGUOUS_DEFAULT_AREA) != 0 { pr_warn!("Couldn't queue default CMA region for heap creation."); }
    }
    dma_numa_cma_reserve();
}

#[no_mangle]
pub unsafe extern "C" fn dma_contiguous_early_fixup(_base: phys_addr_t, _size: c_ulong) {}

#[no_mangle]
pub unsafe extern "C" fn dma_contiguous_reserve_area(size: phys_addr_t, base: phys_addr_t, limit: phys_addr_t, res_cma: *mut *mut cma, fixed: bool) -> c_int {
    let ret = cma_declare_contiguous(base, size, limit, 0, 0, fixed, b"reserved\0".as_ptr() as *const c_char, res_cma);
    if ret != 0 { return ret; }
    dma_contiguous_early_fixup(cma_get_base(*res_cma), cma_get_size(*res_cma)); 0
}

#[no_mangle]
pub unsafe extern "C" fn dma_alloc_from_contiguous(dev: *mut device, count: usize, mut align: c_uint, no_warn: bool) -> *mut page {
    if align > CONFIG_CMA_ALIGNMENT { align = CONFIG_CMA_ALIGNMENT; }
    cma_alloc(dev_get_cma_area(dev), count, align, no_warn)
}

#[no_mangle]
pub unsafe extern "C" fn dma_release_from_contiguous(dev: *mut device, pages: *mut page, count: c_int) -> bool {
    cma_release(dev_get_cma_area(dev), pages, count)
}

unsafe fn cma_alloc_aligned(cma: *mut cma, size: usize, gfp: gfp_t) -> *mut page {
    let align = core::cmp::min(get_order(size), CONFIG_CMA_ALIGNMENT);
    cma_alloc(cma, size >> PAGE_SHIFT, align, gfp & __GFP_NOWARN)
}

#[no_mangle]
pub unsafe extern "C" fn dma_alloc_contiguous(dev: *mut device, size: usize, gfp: gfp_t) -> *mut page {
    if !gfpflags_allow_blocking(gfp) { return core::ptr::null_mut(); }
    if !dev.is_null() && !(*dev).cma_area.is_null() { return cma_alloc_aligned((*dev).cma_area, size, gfp); }
    if size <= PAGE_SIZE { return core::ptr::null_mut(); }
    #[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
    { let nid = dev_to_node(dev); if nid != NUMA_NO_NODE && (gfp & (GFP_DMA | GFP_DMA32)) == 0 { let c = DMA_CONTIGUOUS_NUMA_AREA[nid as usize]; if !c.is_null() { let p = cma_alloc_aligned(c, size, gfp); if !p.is_null() { return p; } } } }
    if DMA_CONTIGUOUS_DEFAULT_AREA.is_null() { return core::ptr::null_mut(); }
    cma_alloc_aligned(DMA_CONTIGUOUS_DEFAULT_AREA, size, gfp)
}

#[no_mangle]
pub unsafe extern "C" fn dma_free_contiguous(dev: *mut device, page: *mut page, size: usize) {
    let count = PAGE_ALIGN(size) >> PAGE_SHIFT;
    if !dev.is_null() && !(*dev).cma_area.is_null() { if cma_release((*dev).cma_area, page, count) { return; } }
    #[cfg(feature = "CONFIG_DMA_NUMA_CMA")]
    if cma_release(DMA_CONTIGUOUS_NUMA_AREA[page_to_nid(page) as usize], page, count) { return; }
    if cma_release(DMA_CONTIGUOUS_DEFAULT_AREA, page, count) { return; }
    __free_pages(page, get_order(size));
}

// Device-tree reserved-memory CMA support is retained under its original build-time gate.
#[cfg(feature = "CONFIG_OF_RESERVED_MEM")]
mod reserved_mem_cma {
    use super::*;
    unsafe extern "C" fn rmem_cma_device_init(rmem: *mut reserved_mem, dev: *mut device) -> c_int { (*dev).cma_area = (*rmem).priv_; 0 }
    unsafe extern "C" fn rmem_cma_device_release(_rmem: *mut reserved_mem, dev: *mut device) { (*dev).cma_area = core::ptr::null_mut(); }
    unsafe extern "C" fn __rmem_cma_verify_node(node: c_ulong) -> c_int { if of_get_flat_dt_prop(node, b"reusable\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null() || !of_get_flat_dt_prop(node, b"no-map\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null() { return -ENODEV; } if SIZE_CMDLINE != !0 && !of_get_flat_dt_prop(node, b"linux,cma-default\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null() { return -EBUSY; } 0 }
    unsafe extern "C" fn rmem_cma_validate(node: c_ulong, align: *mut phys_addr_t) -> c_int { let ret = __rmem_cma_verify_node(node); if ret != 0 { return ret; } if !align.is_null() { *align = core::cmp::max(*align, CMA_MIN_ALIGNMENT_BYTES); } 0 }
    unsafe extern "C" fn rmem_cma_fixup(node: c_ulong, base: phys_addr_t, size: phys_addr_t) -> c_int { let ret = __rmem_cma_verify_node(node); if ret != 0 { return ret; } dma_contiguous_early_fixup(base, size); 0 }
    unsafe extern "C" fn rmem_cma_setup(node: c_ulong, rmem: *mut reserved_mem) -> c_int { let default_cma = !of_get_flat_dt_prop(node, b"linux,cma-default\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null(); let mut c = core::ptr::null_mut(); let ret = __rmem_cma_verify_node(node); if ret != 0 { return ret; } if (( (*rmem).base | (*rmem).size) & (CMA_MIN_ALIGNMENT_BYTES - 1)) != 0 { return -EINVAL; } let ret = cma_init_reserved_mem((*rmem).base, (*rmem).size, 0, (*rmem).name, &mut c); if ret != 0 { return ret; } if default_cma { DMA_CONTIGUOUS_DEFAULT_AREA = c; } (*rmem).priv_ = c; dma_contiguous_insert_area(c); 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
