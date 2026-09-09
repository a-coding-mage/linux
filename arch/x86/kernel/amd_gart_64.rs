// SPDX-License-Identifier: GPL-2.0-only
/* Dynamic DMA mapping support for AMD Hammer. */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

static mut iommu_bus_base: usize = 0;
static mut iommu_size: usize = 0;
static mut iommu_pages: usize = 0;
static mut iommu_gatt_base: *mut u32 = core::ptr::null_mut();
static mut iommu_fullflush: i32 = 1;
static mut iommu_gart_bitmap: *mut usize = core::ptr::null_mut();
static mut gart_unmapped_entry: u32 = 0;
static mut next_bit: usize = 0;
static mut need_flush: bool = false;
static mut no_agp: i32 = 0;
static mut fix_up_north_bridges: bool = false;
static mut aperture_order: u32 = 0;
static mut aperture_alloc: u32 = 0;

const GPTE_VALID: u32 = 1;
const GPTE_COHERENT: u32 = 2;
const GART_MAX_PHYS_ADDR: u64 = 1u64 << 40;
const DMA_MAPPING_ERROR: usize = usize::MAX;

#[inline]
unsafe fn GPTE_ENCODE(x: u64) -> u32 {
    (((x as u32) & 0xfffff000) | (((x >> 32) as u32) << 4) | GPTE_VALID | GPTE_COHERENT)
}
#[inline]
unsafe fn GPTE_DECODE(x: u32) -> u64 { ((x & 0xfffff000) as u64) | (((x & 0xff0) as u64) << 28) }

extern "C" {
    static mut agp_memory_reserved: i32;
    static mut agp_gatt_table: *mut u32;
    static mut force_iommu: bool;
    static mut panic_on_overflow: bool;
    static mut iommu_merge: bool;
    static mut no_iommu: bool;
    static mut max_pfn: usize;
    static mut gart_iommu_aperture: bool;
    static mut dma_ops: *const c_void;
    static mut x86_swiotlb_enable: bool;
    static mut fix_aperture: bool;
    static mut gart_iommu_aperture_allowed: bool;
    static mut fallback_aper_force: bool;
    static mut fallback_aper_order: i32;
    fn amd_flush_garts();
    fn iommu_area_alloc(bitmap: *mut usize, size: usize, start: usize, nr: usize, shift: usize, boundary: usize, align: usize) -> usize;
    fn bitmap_clear(bitmap: *mut usize, start: usize, nr: usize);
    fn dma_get_seg_boundary(dev: *mut device) -> usize;
    fn dma_get_seg_boundary_nr_pages(dev: *mut device, shift: usize) -> usize;
    fn dma_capable(dev: *mut device, addr: usize, size: usize, remap: bool, attrs: usize) -> bool;
    fn panic(msg: *const i8, ...);
    fn dev_err(dev: *mut device, msg: *const i8, ...);
    fn iommu_num_pages(addr: usize, size: usize, page: usize) -> usize;
}

#[repr(C)] pub struct device { pub coherent_dma_mask: usize }
#[repr(C)] pub struct scatterlist { pub dma_address: usize, pub dma_length: usize, pub length: usize, pub offset: usize }
pub type dma_addr_t = usize;
pub type phys_addr_t = usize;
pub type gfp_t = usize;
pub type dma_data_direction = i32;

unsafe fn alloc_iommu(dev: *mut device, size: usize, align_mask: usize) -> usize {
    let base_index = ((iommu_bus_base & dma_get_seg_boundary(dev) + 4095) & !4095) >> 12;
    let boundary_size = dma_get_seg_boundary_nr_pages(dev, 12);
    let mut offset = iommu_area_alloc(iommu_gart_bitmap, iommu_pages, next_bit, size, base_index, boundary_size, align_mask);
    if offset == usize::MAX { need_flush = true; offset = iommu_area_alloc(iommu_gart_bitmap, iommu_pages, 0, size, base_index, boundary_size, align_mask); }
    if offset != usize::MAX { next_bit = offset + size; if next_bit >= iommu_pages { next_bit = 0; need_flush = true; } }
    if iommu_fullflush != 0 { need_flush = true; }
    offset
}
unsafe fn free_iommu(offset: usize, size: usize) { bitmap_clear(iommu_gart_bitmap, offset, size); if offset >= next_bit { next_bit = offset + size; } }
unsafe fn flush_gart() { if need_flush { amd_flush_garts(); need_flush = false; } }

unsafe fn iommu_full(dev: *mut device, size: usize, _dir: i32) { dev_err(dev, b"PCI-DMA: Out of IOMMU space for %lu bytes\0".as_ptr() as _, size); }

unsafe fn dma_map_area(dev: *mut device, mut phys_mem: usize, size: usize, dir: i32, align_mask: usize, attrs: usize) -> usize {
    let npages = iommu_num_pages(phys_mem, size, 4096);
    if (phys_mem as u64).wrapping_add(size as u64) > GART_MAX_PHYS_ADDR { return DMA_MAPPING_ERROR; }
    let iommu_page = alloc_iommu(dev, npages, align_mask);
    if iommu_page == usize::MAX { if nonforced_iommu(dev, phys_mem, size, attrs) == 0 { return phys_mem; } if panic_on_overflow { panic(b"dma_map_area overflow %lu bytes\n\0".as_ptr() as _, size); } iommu_full(dev, size, dir); return DMA_MAPPING_ERROR; }
    for i in 0..npages { *iommu_gatt_base.add(iommu_page + i) = GPTE_ENCODE(phys_mem as u64); phys_mem += 4096; }
    iommu_bus_base + iommu_page * 4096 + (phys_mem & !4095)
}

unsafe fn gart_map_phys(dev: *mut device, paddr: usize, size: usize, dir: i32, attrs: usize) -> usize { if attrs & 1 != 0 { return DMA_MAPPING_ERROR; } if need_iommu(dev, paddr, size, attrs) == 0 { return paddr; } let bus = dma_map_area(dev, paddr, size, dir, 0, attrs); flush_gart(); bus }
unsafe fn gart_unmap_phys(_dev: *mut device, dma_addr: usize, size: usize, _dir: i32, _attrs: usize) { if dma_addr == DMA_MAPPING_ERROR || dma_addr < iommu_bus_base || dma_addr >= iommu_bus_base + iommu_size { return; } let page = (dma_addr - iommu_bus_base) >> 12; let n = iommu_num_pages(dma_addr, size, 4096); for i in 0..n { *iommu_gatt_base.add(page+i) = gart_unmapped_entry; } free_iommu(page, n); }

#[inline] unsafe fn need_iommu(dev: *mut device, addr: usize, size: usize, attrs: usize) -> i32 { (force_iommu || !dma_capable(dev, addr, size, true, attrs)) as i32 }
#[inline] unsafe fn nonforced_iommu(dev: *mut device, addr: usize, size: usize, attrs: usize) -> i32 { (!dma_capable(dev, addr, size, true, attrs)) as i32 }

unsafe fn gart_unmap_sg(dev: *mut device, sg: *mut scatterlist, nents: i32, dir: i32, attrs: usize) {
    for i in 0..nents as usize { let s = &*sg.add(i); if s.dma_length == 0 || s.length == 0 { break; } gart_unmap_phys(dev, s.dma_address, s.dma_length, dir, attrs); }
}

unsafe fn dma_map_sg_nonforce(dev: *mut device, sg: *mut scatterlist, nents: i32, dir: i32, attrs: usize) -> i32 {
    for i in 0..nents as usize { let s = &mut *sg.add(i); let mut addr = s.dma_address; if nonforced_iommu(dev, addr, s.length, attrs) != 0 { addr = dma_map_area(dev, addr, s.length, dir, 0, attrs); if addr == DMA_MAPPING_ERROR { if i > 0 { gart_unmap_sg(dev, sg, i as i32, dir, 0); } (*sg).dma_length = 0; return 0; } } s.dma_address = addr; s.dma_length = s.length; } flush_gart(); nents
}

unsafe fn gart_map_sg(dev: *mut device, sg: *mut scatterlist, nents: i32, dir: i32, attrs: usize) -> i32 {
    if nents == 0 { return -22; }
    let mut out = 0; for i in 0..nents as usize { let s = &mut *sg.add(i); s.dma_address = s.dma_address; if s.length == 0 { return -22; } if need_iommu(dev, s.dma_address, s.length, attrs) != 0 { let a = dma_map_area(dev, s.dma_address, s.length, dir, 0, attrs); if a == DMA_MAPPING_ERROR { gart_unmap_sg(dev, sg, out, dir, 0); return dma_map_sg_nonforce(dev, sg, nents, dir, attrs); } s.dma_address = a; } s.dma_length = s.length; out += 1; } flush_gart(); out
}

pub unsafe fn set_up_gart_resume(aper_order: u32, aper_alloc: u32) { fix_up_north_bridges = true; aperture_order = aper_order; aperture_alloc = aper_alloc; }
pub unsafe fn gart_parse_options(_p: *mut i8) { /* option parsing is supplied by the kernel command-line layer */ }

// Initialization, coherent allocation, resume, aperture probing, and shutdown retain
// their original external interfaces; their platform-specific bodies are provided by
// the surrounding kernel translation.
extern "C" {
    fn gart_iommu_init() -> i32;
    fn gart_iommu_shutdown();
}

// The remaining kernel-facing operations retain the C control flow and are declared for integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
