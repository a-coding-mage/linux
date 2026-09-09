/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/dma-map-ops.h. */

/* Types and symbols from the included Linux headers are external dependencies. */

#[repr(C)]
pub struct dma_map_ops {
    pub alloc: Option<unsafe extern "C" fn(*mut device, usize, *mut dma_addr_t, gfp_t, c_ulong) -> *mut core::ffi::c_void>,
    pub free: Option<unsafe extern "C" fn(*mut device, usize, *mut core::ffi::c_void, dma_addr_t, c_ulong)>,
    pub alloc_pages_op: Option<unsafe extern "C" fn(*mut device, usize, *mut dma_addr_t, dma_data_direction, gfp_t) -> *mut page>,
    pub free_pages: Option<unsafe extern "C" fn(*mut device, usize, *mut page, dma_addr_t, dma_data_direction)>,
    pub mmap: Option<unsafe extern "C" fn(*mut device, *mut vm_area_struct, *mut core::ffi::c_void, dma_addr_t, usize, c_ulong) -> c_int>,
    pub get_sgtable: Option<unsafe extern "C" fn(*mut device, *mut sg_table, *mut core::ffi::c_void, dma_addr_t, usize, c_ulong) -> c_int>,
    pub map_phys: Option<unsafe extern "C" fn(*mut device, phys_addr_t, usize, dma_data_direction, c_ulong) -> dma_addr_t>,
    pub unmap_phys: Option<unsafe extern "C" fn(*mut device, dma_addr_t, usize, dma_data_direction, c_ulong)>,
    pub map_sg: Option<unsafe extern "C" fn(*mut device, *mut scatterlist, c_int, dma_data_direction, c_ulong) -> c_int>,
    pub unmap_sg: Option<unsafe extern "C" fn(*mut device, *mut scatterlist, c_int, dma_data_direction, c_ulong)>,
    pub sync_single_for_cpu: Option<unsafe extern "C" fn(*mut device, dma_addr_t, usize, dma_data_direction)>,
    pub sync_single_for_device: Option<unsafe extern "C" fn(*mut device, dma_addr_t, usize, dma_data_direction)>,
    pub sync_sg_for_cpu: Option<unsafe extern "C" fn(*mut device, *mut scatterlist, c_int, dma_data_direction)>,
    pub sync_sg_for_device: Option<unsafe extern "C" fn(*mut device, *mut scatterlist, c_int, dma_data_direction)>,
    pub cache_sync: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void, usize, dma_data_direction)>,
    pub dma_supported: Option<unsafe extern "C" fn(*mut device, u64) -> c_int>,
    pub get_required_mask: Option<unsafe extern "C" fn(*mut device) -> u64>,
    pub max_mapping_size: Option<unsafe extern "C" fn(*mut device) -> usize>,
    pub opt_mapping_size: Option<unsafe extern "C" fn() -> usize>,
    pub get_merge_boundary: Option<unsafe extern "C" fn(*mut device) -> c_ulong>,
}

#[cfg(CONFIG_ARCH_HAS_DMA_OPS)]
pub unsafe fn get_dma_ops(dev: *mut device) -> *const dma_map_ops {
    if !(*dev).dma_ops.is_null() { (*dev).dma_ops } else { get_arch_dma_ops() }
}
#[cfg(not(CONFIG_ARCH_HAS_DMA_OPS))]
pub unsafe fn get_dma_ops(_dev: *mut device) -> *const dma_map_ops { core::ptr::null() }

#[cfg(CONFIG_ARCH_HAS_DMA_OPS)]
pub unsafe fn set_dma_ops(dev: *mut device, dma_ops: *const dma_map_ops) { (*dev).dma_ops = dma_ops; }
#[cfg(not(CONFIG_ARCH_HAS_DMA_OPS))]
pub unsafe fn set_dma_ops(_dev: *mut device, _dma_ops: *const dma_map_ops) {}

#[cfg(CONFIG_DMA_CMA)]
extern "C" {
    pub fn dev_get_cma_area(dev: *mut device) -> *mut cma;
    pub fn dma_contiguous_get_area_by_idx(idx: c_uint) -> *mut cma;
    pub fn dma_contiguous_reserve(addr_limit: phys_addr_t);
    pub fn dma_contiguous_reserve_area(size: phys_addr_t, base: phys_addr_t, limit: phys_addr_t, res_cma: *mut *mut cma, fixed: bool) -> c_int;
    pub fn dma_alloc_from_contiguous(dev: *mut device, count: usize, order: c_uint, no_warn: bool) -> *mut page;
    pub fn dma_release_from_contiguous(dev: *mut device, pages: *mut page, count: c_int) -> bool;
    pub fn dma_alloc_contiguous(dev: *mut device, size: usize, gfp: gfp_t) -> *mut page;
    pub fn dma_free_contiguous(dev: *mut device, page: *mut page, size: usize);
    pub fn dma_contiguous_early_fixup(base: phys_addr_t, size: c_ulong);
}
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dev_get_cma_area(_dev: *mut device) -> *mut cma { core::ptr::null_mut() }
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_contiguous_get_area_by_idx(_idx: c_uint) -> *mut cma { core::ptr::null_mut() }
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_contiguous_reserve(_limit: phys_addr_t) {}
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_contiguous_reserve_area(_size: phys_addr_t, _base: phys_addr_t, _limit: phys_addr_t, _res_cma: *mut *mut cma, _fixed: bool) -> c_int { -ENOSYS }
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_alloc_from_contiguous(_dev: *mut device, _count: usize, _order: c_uint, _no_warn: bool) -> *mut page { core::ptr::null_mut() }
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_release_from_contiguous(_dev: *mut device, _pages: *mut page, _count: c_int) -> bool { false }
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_alloc_contiguous(_dev: *mut device, _size: usize, _gfp: gfp_t) -> *mut page { core::ptr::null_mut() }
#[cfg(not(CONFIG_DMA_CMA))]
pub unsafe fn dma_free_contiguous(_dev: *mut device, page: *mut page, size: usize) { __free_pages(page, get_order(size)); }

extern "C" {
    pub fn dma_declare_coherent_memory(dev: *mut device, phys_addr: phys_addr_t, device_addr: dma_addr_t, size: usize) -> c_int;
    pub fn dma_release_coherent_memory(dev: *mut device);
    pub fn dma_alloc_from_dev_coherent(dev: *mut device, size: isize, dma_handle: *mut dma_addr_t, ret: *mut *mut core::ffi::c_void) -> c_int;
    pub fn dma_release_from_dev_coherent(dev: *mut device, order: c_int, vaddr: *mut core::ffi::c_void) -> c_int;
    pub fn dma_mmap_from_dev_coherent(dev: *mut device, vma: *mut vm_area_struct, cpu_addr: *mut core::ffi::c_void, size: usize, ret: *mut c_int) -> c_int;
    pub fn dma_common_get_sgtable(dev: *mut device, sgt: *mut sg_table, cpu_addr: *mut core::ffi::c_void, dma_addr: dma_addr_t, size: usize, attrs: c_ulong) -> c_int;
    pub fn dma_common_mmap(dev: *mut device, vma: *mut vm_area_struct, cpu_addr: *mut core::ffi::c_void, dma_addr: dma_addr_t, size: usize, attrs: c_ulong) -> c_int;
    pub fn dma_common_alloc_pages(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, dir: dma_data_direction, gfp: gfp_t) -> *mut page;
    pub fn dma_common_free_pages(dev: *mut device, size: usize, vaddr: *mut page, dma_handle: dma_addr_t, dir: dma_data_direction);
    pub fn dma_common_find_pages(cpu_addr: *mut core::ffi::c_void) -> *mut *mut page;
    pub fn dma_common_contiguous_remap(page: *mut page, size: usize, prot: pgprot_t, caller: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn dma_common_pages_remap(pages: *mut *mut page, size: usize, prot: pgprot_t, caller: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn dma_common_free_remap(cpu_addr: *mut core::ffi::c_void, size: usize);
    pub fn dma_alloc_from_pool(dev: *mut device, size: usize, cpu_addr: *mut *mut core::ffi::c_void, flags: gfp_t, attrs: c_ulong, phys_addr_ok: Option<unsafe extern "C" fn(*mut device, phys_addr_t, usize) -> bool>) -> *mut page;
    pub fn dma_free_from_pool(dev: *mut device, start: *mut core::ffi::c_void, size: usize) -> bool;
    pub fn dma_free_from_pool_page(dev: *mut device, page: *mut page, size: usize) -> bool;
    pub fn dma_direct_set_offset(dev: *mut device, cpu_start: phys_addr_t, dma_start: dma_addr_t, size: u64) -> c_int;
    pub fn arch_dma_alloc(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t, attrs: c_ulong) -> *mut core::ffi::c_void;
    pub fn arch_dma_free(dev: *mut device, size: usize, cpu_addr: *mut core::ffi::c_void, dma_addr: dma_addr_t, attrs: c_ulong);
    pub fn arch_dma_set_uncached(addr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    pub fn arch_dma_clear_uncached(addr: *mut core::ffi::c_void, size: usize);
}

#[cfg(any(CONFIG_ARCH_HAS_SYNC_DMA_FOR_DEVICE, CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU, CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU_ALL))]
pub unsafe fn dev_is_dma_coherent(dev: *mut device) -> bool { dev_dma_coherent(dev) }
#[cfg(not(any(CONFIG_ARCH_HAS_SYNC_DMA_FOR_DEVICE, CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU, CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU_ALL)))]
pub unsafe fn dev_is_dma_coherent(_dev: *mut device) -> bool { true }

pub unsafe fn dma_reset_need_sync(dev: *mut device) {
    #[cfg(CONFIG_DMA_NEED_SYNC)]
    if unlikely(dev_dma_skip_sync(dev)) { dev_clear_dma_skip_sync(dev); }
}

pub unsafe fn dma_kmalloc_safe(dev: *mut device, dir: dma_data_direction) -> bool {
    if !IS_ENABLED(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) { return true; }
    dev_is_dma_coherent(dev) || dir == DMA_TO_DEVICE
}
pub unsafe fn dma_kmalloc_size_aligned(size: usize) -> bool {
    size >= 2 * ARCH_DMA_MINALIGN || IS_ALIGNED(kmalloc_size_roundup(size), dma_get_cache_alignment())
}
pub unsafe fn dma_kmalloc_needs_bounce(dev: *mut device, size: usize, dir: dma_data_direction) -> bool {
    !dma_kmalloc_safe(dev, dir) && !dma_kmalloc_size_aligned(size)
}

#[cfg(not(CONFIG_MMU))]
pub unsafe fn dma_pgprot(_dev: *mut device, prot: pgprot_t, _attrs: c_ulong) -> pgprot_t { prot }

#[cfg(not(CONFIG_ARCH_HAS_SYNC_DMA_FOR_DEVICE))]
pub unsafe fn arch_sync_dma_for_device(_paddr: phys_addr_t, _size: usize, _dir: dma_data_direction) {}
#[cfg(CONFIG_ARCH_HAS_SYNC_DMA_FOR_DEVICE)]
extern "C" { pub fn arch_sync_dma_for_device(paddr: phys_addr_t, size: usize, dir: dma_data_direction); }
#[cfg(not(CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU))]
pub unsafe fn arch_sync_dma_for_cpu(_paddr: phys_addr_t, _size: usize, _dir: dma_data_direction) {}
#[cfg(CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU)]
extern "C" { pub fn arch_sync_dma_for_cpu(paddr: phys_addr_t, size: usize, dir: dma_data_direction); }
#[cfg(not(CONFIG_ARCH_HAS_BATCHED_DMA_SYNC))]
pub unsafe fn arch_sync_dma_flush() {}
#[cfg(not(CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU_ALL))]
pub unsafe fn arch_sync_dma_for_cpu_all() {}
#[cfg(CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU_ALL)]
extern "C" { pub fn arch_sync_dma_for_cpu_all(); }
#[cfg(not(CONFIG_ARCH_HAS_DMA_PREP_COHERENT))]
pub unsafe fn arch_dma_prep_coherent(_page: *mut page, _size: usize) {}
#[cfg(CONFIG_ARCH_HAS_DMA_PREP_COHERENT)]
extern "C" { pub fn arch_dma_prep_coherent(page: *mut page, size: usize); }

#[cfg(CONFIG_ARCH_HAS_DMA_SET_MASK)]
extern "C" { pub fn arch_dma_set_mask(dev: *mut device, mask: u64); }
#[cfg(not(CONFIG_ARCH_HAS_DMA_SET_MASK))]
pub unsafe fn arch_dma_set_mask(_dev: *mut device, _mask: u64) {}

#[cfg(CONFIG_MMU)]
extern "C" { pub fn dma_pgprot(dev: *mut device, prot: pgprot_t, attrs: c_ulong) -> pgprot_t; }

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
extern "C" {
    pub fn arch_dma_map_phys_direct(dev: *mut device, addr: phys_addr_t) -> bool;
    pub fn arch_dma_unmap_phys_direct(dev: *mut device, dma_handle: dma_addr_t) -> bool;
    pub fn arch_dma_map_sg_direct(dev: *mut device, sg: *mut scatterlist, nents: c_int) -> bool;
    pub fn arch_dma_unmap_sg_direct(dev: *mut device, sg: *mut scatterlist, nents: c_int) -> bool;
    pub fn arch_dma_alloc_direct(dev: *mut device) -> bool;
    pub fn arch_dma_free_direct(dev: *mut device, dma_handle: dma_addr_t) -> bool;
}

#[cfg(not(CONFIG_ARCH_HAS_DMA_MAP_DIRECT))]
pub unsafe fn arch_dma_map_phys_direct(_d: *mut device, _a: phys_addr_t) -> bool { false }
#[cfg(not(CONFIG_ARCH_HAS_DMA_MAP_DIRECT))]
pub unsafe fn arch_dma_unmap_phys_direct(_d: *mut device, _a: dma_addr_t) -> bool { false }
#[cfg(not(CONFIG_ARCH_HAS_DMA_MAP_DIRECT))]
pub unsafe fn arch_dma_map_sg_direct(_d: *mut device, _s: *mut scatterlist, _n: c_int) -> bool { false }
#[cfg(not(CONFIG_ARCH_HAS_DMA_MAP_DIRECT))]
pub unsafe fn arch_dma_unmap_sg_direct(_d: *mut device, _s: *mut scatterlist, _n: c_int) -> bool { false }
#[cfg(not(CONFIG_ARCH_HAS_DMA_MAP_DIRECT))]
pub unsafe fn arch_dma_alloc_direct(_d: *mut device) -> bool { false }
#[cfg(not(CONFIG_ARCH_HAS_DMA_MAP_DIRECT))]
pub unsafe fn arch_dma_free_direct(_d: *mut device, _a: dma_addr_t) -> bool { false }

#[cfg(not(CONFIG_ARCH_HAS_SETUP_DMA_OPS))]
pub unsafe fn arch_setup_dma_ops(_dev: *mut device, _coherent: bool) {}
#[cfg(CONFIG_ARCH_HAS_SETUP_DMA_OPS)]
extern "C" { pub fn arch_setup_dma_ops(dev: *mut device, coherent: bool); }
#[cfg(not(CONFIG_ARCH_HAS_TEARDOWN_DMA_OPS))]
pub unsafe fn arch_teardown_dma_ops(_dev: *mut device) {}
#[cfg(CONFIG_ARCH_HAS_TEARDOWN_DMA_OPS)]
extern "C" { pub fn arch_teardown_dma_ops(dev: *mut device); }
#[cfg(not(CONFIG_DMA_API_DEBUG))]
pub unsafe fn dma_debug_add_bus(_bus: *const bus_type) {}
#[cfg(CONFIG_DMA_API_DEBUG)]
extern "C" { pub fn dma_debug_add_bus(bus: *const bus_type); }
#[cfg(not(CONFIG_DMA_API_DEBUG))]
pub unsafe fn debug_dma_dump_mappings(_dev: *mut device) {}
#[cfg(CONFIG_DMA_API_DEBUG)]
extern "C" { pub fn debug_dma_dump_mappings(dev: *mut device); }

extern "C" { pub static dma_dummy_ops: dma_map_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
