/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/dma-mapping.h. External kernel types and symbols
 * are supplied by the surrounding kernel translation unit. */

pub const DMA_ATTR_WEAK_ORDERING: usize = 1usize << 1;
pub const DMA_ATTR_WRITE_COMBINE: usize = 1usize << 2;
pub const DMA_ATTR_NO_KERNEL_MAPPING: usize = 1usize << 4;
pub const DMA_ATTR_SKIP_CPU_SYNC: usize = 1usize << 5;
pub const DMA_ATTR_FORCE_CONTIGUOUS: usize = 1usize << 6;
pub const DMA_ATTR_ALLOC_SINGLE_PAGES: usize = 1usize << 7;
pub const DMA_ATTR_NO_WARN: usize = 1usize << 8;
pub const DMA_ATTR_PRIVILEGED: usize = 1usize << 9;
pub const DMA_ATTR_MMIO: usize = 1usize << 10;
pub const DMA_ATTR_DEBUGGING_IGNORE_CACHELINES: usize = 1usize << 11;
pub const DMA_ATTR_REQUIRE_COHERENT: usize = 1usize << 12;
pub const DMA_ATTR_CC_SHARED: usize = 1usize << 13;
pub const __DMA_ATTR_ALLOC_CC_SHARED: usize = 1usize << 14;

#[repr(C)]
pub struct dma_iova_state {
    pub addr: dma_addr_t,
    pub __size: u64,
}
pub const DMA_IOVA_USE_SWIOTLB: u64 = 1u64 << 63;

#[inline]
pub unsafe fn dma_iova_size(state: *mut dma_iova_state) -> usize {
    ((*state).__size & !DMA_IOVA_USE_SWIOTLB) as usize
}

#[cfg(CONFIG_DMA_API_DEBUG)]
extern "C" {
    pub fn debug_dma_mapping_error(dev: *mut device, dma_addr: dma_addr_t);
    pub fn debug_dma_map_single(dev: *mut device, addr: *const core::ffi::c_void, len: usize);
}

#[inline] pub unsafe fn dma_get_mask(dev: *mut device) -> u64 {
    if !(*dev).dma_mask.is_null() && *(*dev).dma_mask != 0 { *(*dev).dma_mask } else { (1u64 << 32) - 1 }
}
#[inline] pub unsafe fn dma_set_mask_and_coherent(dev:*mut device, mask:u64)->i32 {
    let rc=dma_set_mask(dev,mask); if rc==0 { dma_set_coherent_mask(dev,mask); } rc
}
#[inline] pub unsafe fn dma_coerce_mask_and_coherent(dev:*mut device, mask:u64)->i32 {
    (*dev).dma_mask=&mut (*dev).coherent_dma_mask; dma_set_mask_and_coherent(dev,mask)
}
#[inline] pub unsafe fn dma_alloc_coherent(dev:*mut device,size:usize,h:*mut dma_addr_t,g:gfp_t)->*mut core::ffi::c_void { dma_alloc_attrs(dev,size,h,g,0) }
#[inline] pub unsafe fn dma_free_coherent(dev:*mut device,size:usize,p:*mut core::ffi::c_void,h:dma_addr_t) { dma_free_attrs(dev,size,p,h,0) }
#[inline] pub unsafe fn dmam_alloc_coherent(dev:*mut device,size:usize,h:*mut dma_addr_t,g:gfp_t)->*mut core::ffi::c_void { dmam_alloc_attrs(dev,size,h,g,0) }
#[inline] pub unsafe fn dma_get_max_seg_size(_: *mut device)->u32 { 65536 }
#[inline] pub unsafe fn dma_get_seg_boundary(_: *mut device)->usize { usize::MAX }
#[inline] pub unsafe fn dma_get_seg_boundary_nr_pages(dev:*mut device, shift:u32)->usize { if dev.is_null() { (u32::MAX >> shift) as usize + 1 } else { (dma_get_seg_boundary(dev)>>shift)+1 } }
#[inline] pub unsafe fn dma_get_min_align_mask(_: *mut device)->u32 { 0 }
#[inline] pub unsafe fn dma_get_cache_alignment()->i32 { 1 }

/* Configuration-dependent synchronization API. */
#[inline] pub unsafe fn dma_dev_need_sync(_: *const device)->bool { false }
#[inline] pub unsafe fn dma_sync_sg_for_cpu(_: *mut device, _: *mut scatterlist, _: i32, _: dma_data_direction) {}
#[inline] pub unsafe fn dma_sync_sg_for_device(_: *mut device, _: *mut scatterlist, _: i32, _: dma_data_direction) {}
#[inline] pub unsafe fn dma_need_sync(_: *mut device, _: dma_addr_t)->bool { false }
#[inline] pub unsafe fn dma_need_unmap(_: *mut device)->bool { false }

/* These aliases preserve the source-level cacheline grouping macros. */
#[macro_export] macro_rules! DEFINE_DMA_UNMAP_ADDR { ($name:ident) => { pub $name: dma_addr_t }; }
#[macro_export] macro_rules! DEFINE_DMA_UNMAP_LEN { ($name:ident) => { pub $name: u32 }; }
#[cfg(not(CONFIG_DMA_API_DEBUG))]
#[inline] pub unsafe fn debug_dma_mapping_error(_: *mut device, _: dma_addr_t) {}
#[cfg(not(CONFIG_DMA_API_DEBUG))]
#[inline] pub unsafe fn debug_dma_map_single(_: *mut device, _: *const core::ffi::c_void, _: usize) {}

#[cfg(CONFIG_HAS_DMA)]
extern "C" {
    pub fn dma_map_page_attrs(_: *mut device, _: *mut page, _: usize, _: usize, _: dma_data_direction, _: usize) -> dma_addr_t;
    pub fn dma_unmap_page_attrs(_: *mut device, _: dma_addr_t, _: usize, _: dma_data_direction, _: usize);
    pub fn dma_map_phys(_: *mut device, _: phys_addr_t, _: usize, _: dma_data_direction, _: usize) -> dma_addr_t;
    pub fn dma_unmap_phys(_: *mut device, _: dma_addr_t, _: usize, _: dma_data_direction, _: usize);
    pub fn dma_map_sg_attrs(_: *mut device, _: *mut scatterlist, _: i32, _: dma_data_direction, _: usize) -> u32;
    pub fn dma_unmap_sg_attrs(_: *mut device, _: *mut scatterlist, _: i32, _: dma_data_direction, _: usize);
    pub fn dma_map_sgtable(_: *mut device, _: *mut sg_table, _: dma_data_direction, _: usize) -> i32;
    pub fn dma_map_resource(_: *mut device, _: phys_addr_t, _: usize, _: dma_data_direction, _: usize) -> dma_addr_t;
    pub fn dma_unmap_resource(_: *mut device, _: dma_addr_t, _: usize, _: dma_data_direction, _: usize);
    pub fn dma_alloc_attrs(_: *mut device, _: usize, _: *mut dma_addr_t, _: gfp_t, _: usize) -> *mut core::ffi::c_void;
    pub fn dma_free_attrs(_: *mut device, _: usize, _: *mut core::ffi::c_void, _: dma_addr_t, _: usize);
    pub fn dmam_alloc_attrs(_: *mut device, _: usize, _: *mut dma_addr_t, _: gfp_t, _: usize) -> *mut core::ffi::c_void;
    pub fn dmam_free_coherent(_: *mut device, _: usize, _: *mut core::ffi::c_void, _: dma_addr_t);
    pub fn dma_get_sgtable_attrs(_: *mut device, _: *mut sg_table, _: *mut core::ffi::c_void, _: dma_addr_t, _: usize, _: usize) -> i32;
    pub fn dma_mmap_attrs(_: *mut device, _: *mut vm_area_struct, _: *mut core::ffi::c_void, _: dma_addr_t, _: usize, _: usize) -> i32;
    pub fn dma_can_mmap(_: *mut device) -> bool;
    pub fn dma_pci_p2pdma_supported(_: *mut device) -> bool;
    pub fn dma_set_mask(_: *mut device, _: u64) -> i32;
    pub fn dma_set_coherent_mask(_: *mut device, _: u64) -> i32;
    pub fn dma_get_required_mask(_: *mut device) -> u64;
    pub fn dma_addressing_limited(_: *mut device) -> bool;
    pub fn dma_max_mapping_size(_: *mut device) -> usize;
    pub fn dma_opt_mapping_size(_: *mut device) -> usize;
    pub fn dma_get_merge_boundary(_: *mut device) -> usize;
    pub fn dma_alloc_noncontiguous(_: *mut device, _: usize, _: dma_data_direction, _: gfp_t, _: usize) -> *mut sg_table;
    pub fn dma_free_noncontiguous(_: *mut device, _: usize, _: *mut sg_table, _: dma_data_direction);
    pub fn dma_vmap_noncontiguous(_: *mut device, _: usize, _: *mut sg_table) -> *mut core::ffi::c_void;
    pub fn dma_vunmap_noncontiguous(_: *mut device, _: *mut core::ffi::c_void);
    pub fn dma_mmap_noncontiguous(_: *mut device, _: *mut vm_area_struct, _: usize, _: *mut sg_table) -> i32;
}

#[inline]
pub unsafe fn dma_mapping_error(dev: *mut device, addr: dma_addr_t) -> i32 {
    debug_dma_mapping_error(dev, addr);
    if addr == !0 as dma_addr_t { return -12; }
    0
}

#[cfg(CONFIG_IOMMU_DMA)]
extern "C" {
    pub fn dma_iova_try_alloc(_: *mut device, _: *mut dma_iova_state, _: phys_addr_t, _: usize) -> bool;
    pub fn dma_iova_free(_: *mut device, _: *mut dma_iova_state);
    pub fn dma_iova_destroy(_: *mut device, _: *mut dma_iova_state, _: usize, _: dma_data_direction, _: usize);
    pub fn dma_iova_sync(_: *mut device, _: *mut dma_iova_state, _: usize, _: usize) -> i32;
    pub fn dma_iova_link(_: *mut device, _: *mut dma_iova_state, _: phys_addr_t, _: usize, _: usize, _: dma_data_direction, _: usize) -> i32;
    pub fn dma_iova_unlink(_: *mut device, _: *mut dma_iova_state, _: usize, _: usize, _: dma_data_direction, _: usize);
}
#[inline] pub unsafe fn dma_use_iova(s: *mut dma_iova_state) -> bool { (*s).__size != 0 }

extern "C" {
    pub fn dma_alloc_pages(_: *mut device, _: usize, _: *mut dma_addr_t, _: dma_data_direction, _: gfp_t) -> *mut page;
    pub fn dma_free_pages(_: *mut device, _: usize, _: *mut page, _: dma_addr_t, _: dma_data_direction);
    pub fn dma_mmap_pages(_: *mut device, _: *mut vm_area_struct, _: usize, _: *mut page) -> i32;
    pub fn dma_coherent_ok(_: *mut device, _: phys_addr_t, _: usize) -> bool;
}

#[inline]
pub unsafe fn dma_alloc_noncoherent(dev: *mut device, size: usize, handle: *mut dma_addr_t, dir: dma_data_direction, gfp: gfp_t) -> *mut core::ffi::c_void {
    let p = dma_alloc_pages(dev, size, handle, dir, gfp);
    if p.is_null() { core::ptr::null_mut() } else { page_address(p) }
}
#[inline] pub unsafe fn dma_free_noncoherent(dev: *mut device, size: usize, vaddr: *mut core::ffi::c_void, handle: dma_addr_t, dir: dma_data_direction) { dma_free_pages(dev, size, virt_to_page(vaddr), handle, dir); }
#[inline] pub unsafe fn dma_unmap_single_attrs(dev: *mut device, a: dma_addr_t, s: usize, d: dma_data_direction, x: usize) { dma_unmap_page_attrs(dev,a,s,d,x); }
#[inline] pub unsafe fn dma_sync_single_range_for_cpu(dev: *mut device, a: dma_addr_t, o: usize, s: usize, d: dma_data_direction) { dma_sync_single_for_cpu(dev,a.wrapping_add(o as dma_addr_t),s,d); }
#[inline] pub unsafe fn dma_sync_single_range_for_device(dev: *mut device, a: dma_addr_t, o: usize, s: usize, d: dma_data_direction) { dma_sync_single_for_device(dev,a.wrapping_add(o as dma_addr_t),s,d); }

/* C macro aliases, retained as Rust functions where Rust syntax permits. */
#[inline] pub unsafe fn dma_map_single(dev:*mut device,p:*mut core::ffi::c_void,s:usize,d:dma_data_direction)->dma_addr_t { dma_map_single_attrs(dev,p,s,d,0) }
#[inline] pub unsafe fn dma_unmap_single(dev:*mut device,a:dma_addr_t,s:usize,d:dma_data_direction) { dma_unmap_single_attrs(dev,a,s,d,0) }
#[inline] pub unsafe fn dma_map_sg(dev:*mut device,s:*mut scatterlist,n:i32,d:dma_data_direction)->u32 { dma_map_sg_attrs(dev,s,n,d,0) }
#[inline] pub unsafe fn dma_unmap_sg(dev:*mut device,s:*mut scatterlist,n:i32,d:dma_data_direction) { dma_unmap_sg_attrs(dev,s,n,d,0) }
#[inline] pub unsafe fn dma_map_page(dev:*mut device,p:*mut page,o:usize,s:usize,d:dma_data_direction)->dma_addr_t { dma_map_page_attrs(dev,p,o,s,d,0) }
#[inline] pub unsafe fn dma_unmap_page(dev:*mut device,a:dma_addr_t,s:usize,d:dma_data_direction) { dma_unmap_page_attrs(dev,a,s,d,0) }

/* External kernel declarations referenced by this header. */
extern "C" {
    pub fn dma_sync_single_for_cpu(_: *mut device,_: dma_addr_t,_: usize,_: dma_data_direction);
    pub fn dma_sync_single_for_device(_: *mut device,_: dma_addr_t,_: usize,_: dma_data_direction);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
