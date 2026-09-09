// SPDX-License-Identifier: GPL-2.0
/* arch-independent dma-mapping routines */

// C includes and configuration-provided declarations are supplied by the surrounding kernel crate.

#[repr(C)]
pub struct DmaDevres {
    pub size: usize,
    pub vaddr: *mut core::ffi::c_void,
    pub dma_handle: dma_addr_t,
    pub attrs: c_ulong,
}

static mut DMA_DEFAULT_COHERENT: bool = false;

unsafe fn dmam_release(dev: *mut device, res: *mut core::ffi::c_void) {
    let this = res as *mut DmaDevres;
    dma_free_attrs(dev, (*this).size, (*this).vaddr, (*this).dma_handle, (*this).attrs);
}

unsafe fn dmam_match(_dev: *mut device, res: *mut core::ffi::c_void, match_data: *mut core::ffi::c_void) -> i32 {
    let this = res as *mut DmaDevres;
    let m = match_data as *mut DmaDevres;
    if (*this).vaddr == (*m).vaddr {
        WARN_ON((*this).size != (*m).size || (*this).dma_handle != (*m).dma_handle);
        return 1;
    }
    0
}

pub unsafe fn dmam_free_coherent(dev: *mut device, size: usize, vaddr: *mut core::ffi::c_void, dma_handle: dma_addr_t) {
    let mut match_data = DmaDevres { size, vaddr, dma_handle, attrs: 0 };
    WARN_ON(devres_destroy(dev, Some(dmam_release), Some(dmam_match), &mut match_data as *mut _ as *mut _));
    dma_free_coherent(dev, size, vaddr, dma_handle);
}

pub unsafe fn dmam_alloc_attrs(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t, attrs: c_ulong) -> *mut core::ffi::c_void {
    let dr = devres_alloc(Some(dmam_release), core::mem::size_of::<DmaDevres>(), gfp);
    if dr.is_null() { return core::ptr::null_mut(); }
    let vaddr = dma_alloc_attrs(dev, size, dma_handle, gfp, attrs);
    if vaddr.is_null() { devres_free(dr); return core::ptr::null_mut(); }
    let p = dr as *mut DmaDevres;
    (*p).vaddr = vaddr; (*p).dma_handle = *dma_handle; (*p).size = size; (*p).attrs = attrs;
    devres_add(dev, dr);
    vaddr
}

unsafe fn dma_go_direct(dev: *mut device, mask: dma_addr_t, ops: *const dma_map_ops) -> bool {
    if use_dma_iommu(dev) { return false; }
    if ops.is_null() { return true; }
    if dev_dma_ops_bypass(dev) {
        return min_not_zero(mask, (*dev).bus_dma_limit) >= dma_direct_get_required_mask(dev);
    }
    false
}
unsafe fn dma_alloc_direct(dev: *mut device, ops: *const dma_map_ops) -> bool { dma_go_direct(dev, (*dev).coherent_dma_mask, ops) }
unsafe fn dma_map_direct(dev: *mut device, ops: *const dma_map_ops) -> bool { dma_go_direct(dev, *(*dev).dma_mask, ops) }

pub unsafe fn dma_map_phys(dev: *mut device, phys: phys_addr_t, size: usize, dir: dma_data_direction, attrs: c_ulong) -> dma_addr_t {
    let ops = get_dma_ops(dev); let is_mmio = attrs & DMA_ATTR_MMIO != 0; let is_cc_shared = attrs & DMA_ATTR_CC_SHARED != 0;
    BUG_ON(!valid_dma_direction(dir));
    if (*dev).dma_mask.is_null() || (!dev_is_dma_coherent(dev) && attrs & DMA_ATTR_REQUIRE_COHERENT != 0) { return DMA_MAPPING_ERROR; }
    let addr = if dma_map_direct(dev, ops) || (!is_mmio && !is_cc_shared && arch_dma_map_phys_direct(dev, phys + size)) {
        dma_direct_map_phys(dev, phys, size, dir, attrs, true)
    } else if is_cc_shared { return DMA_MAPPING_ERROR; } else if use_dma_iommu(dev) { iommu_dma_map_phys(dev, phys, size, dir, attrs) } else { (*ops).map_phys.unwrap()(dev, phys, size, dir, attrs) };
    if !is_mmio { kmsan_handle_dma(phys, size, dir); }
    trace_dma_map_phys(dev, phys, addr, size, dir, attrs); debug_dma_map_phys(dev, phys, size, dir, addr, attrs); addr
}

pub unsafe fn dma_map_page_attrs(dev: *mut device, page: *mut page, offset: usize, size: usize, dir: dma_data_direction, attrs: c_ulong) -> dma_addr_t {
    if attrs & DMA_ATTR_MMIO != 0 { return DMA_MAPPING_ERROR; }
    dma_map_phys(dev, page_to_phys(page) + offset, size, dir, attrs)
}

pub unsafe fn dma_unmap_phys(dev: *mut device, addr: dma_addr_t, size: usize, dir: dma_data_direction, attrs: c_ulong) {
    let ops = get_dma_ops(dev); let is_mmio = attrs & DMA_ATTR_MMIO != 0; let is_cc_shared = attrs & DMA_ATTR_CC_SHARED != 0; BUG_ON(!valid_dma_direction(dir));
    if dma_map_direct(dev, ops) || (!is_mmio && !is_cc_shared && arch_dma_unmap_phys_direct(dev, addr + size)) { dma_direct_unmap_phys(dev, addr, size, dir, attrs, true); }
    else if is_cc_shared { return; } else if use_dma_iommu(dev) { iommu_dma_unmap_phys(dev, addr, size, dir, attrs); } else if (*ops).unmap_phys.is_some() { (*ops).unmap_phys.unwrap()(dev, addr, size, dir, attrs); }
    trace_dma_unmap_phys(dev, addr, size, dir, attrs); debug_dma_unmap_phys(dev, addr, size, dir, attrs);
}
pub unsafe fn dma_unmap_page_attrs(dev: *mut device, addr: dma_addr_t, size: usize, dir: dma_data_direction, attrs: c_ulong) { if attrs & DMA_ATTR_MMIO == 0 { dma_unmap_phys(dev, addr, size, dir, attrs); } }

pub unsafe fn dma_map_resource(dev: *mut device, phys: phys_addr_t, size: usize, dir: dma_data_direction, attrs: c_ulong) -> dma_addr_t { dma_map_phys(dev, phys, size, dir, attrs | DMA_ATTR_MMIO) }
pub unsafe fn dma_unmap_resource(dev: *mut device, addr: dma_addr_t, size: usize, dir: dma_data_direction, attrs: c_ulong) { dma_unmap_phys(dev, addr, size, dir, attrs | DMA_ATTR_MMIO); }

pub unsafe fn dma_get_required_mask(dev: *mut device) -> u64 { let ops = get_dma_ops(dev); if dma_alloc_direct(dev, ops) { dma_direct_get_required_mask(dev) } else if use_dma_iommu(dev) { DMA_BIT_MASK(32) } else if (*ops).get_required_mask.is_some() { (*ops).get_required_mask.unwrap()(dev) } else { DMA_BIT_MASK(32) } }

pub unsafe fn dma_can_mmap(dev: *mut device) -> bool { let ops = get_dma_ops(dev); if dma_alloc_direct(dev, ops) { dma_direct_can_mmap(dev) } else if use_dma_iommu(dev) { true } else { (*ops).mmap.is_some() } }

pub unsafe fn dma_set_mask(dev: *mut device, mut mask: u64) -> i32 { mask = mask as dma_addr_t as u64; if (*dev).dma_mask.is_null() || !dma_supported(dev, mask) { return -EIO; } arch_dma_set_mask(dev, mask); *(*dev).dma_mask = mask as dma_addr_t; dma_setup_need_sync(dev); 0 }
pub unsafe fn dma_set_coherent_mask(dev: *mut device, mut mask: u64) -> i32 { mask = mask as dma_addr_t as u64; if !dma_supported(dev, mask) { return -EIO; } (*dev).coherent_dma_mask = mask; 0 }

pub unsafe fn dma_max_mapping_size(dev: *mut device) -> usize { let ops = get_dma_ops(dev); if (*dev).dma_mask.is_null() { return 0; } if dma_map_direct(dev, ops) { dma_direct_max_mapping_size(dev) } else if use_dma_iommu(dev) { iommu_dma_max_mapping_size(dev) } else if !ops.is_null() && (*ops).max_mapping_size.is_some() { (*ops).max_mapping_size.unwrap()(dev) } else { usize::MAX } }
pub unsafe fn dma_opt_mapping_size(dev: *mut device) -> usize { let ops = get_dma_ops(dev); let size = if use_dma_iommu(dev) { iommu_dma_opt_mapping_size() } else if !ops.is_null() && (*ops).opt_mapping_size.is_some() { (*ops).opt_mapping_size.unwrap()() } else { usize::MAX }; core::cmp::min(dma_max_mapping_size(dev), size) }
pub unsafe fn dma_get_merge_boundary(dev: *mut device) -> c_ulong { let ops = get_dma_ops(dev); if use_dma_iommu(dev) { return iommu_dma_get_merge_boundary(dev); } if ops.is_null() || (*ops).get_merge_boundary.is_none() { return 0; } (*ops).get_merge_boundary.unwrap()(dev) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
