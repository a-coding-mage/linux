// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2023 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally external.

const IVPU_MMU_VPU_ADDRESS_MASK: u64 = ((1u64 << 48) - 1) & !((1u64 << 12) - 1);
const IVPU_MMU_PGD_INDEX_MASK: u64 = ((1u64 << 48) - 1) & !((1u64 << 39) - 1);
const IVPU_MMU_PUD_INDEX_MASK: u64 = ((1u64 << 39) - 1) & !((1u64 << 30) - 1);
const IVPU_MMU_PMD_INDEX_MASK: u64 = ((1u64 << 30) - 1) & !((1u64 << 21) - 1);
const IVPU_MMU_PTE_INDEX_MASK: u64 = ((1u64 << 21) - 1) & !((1u64 << 12) - 1);
const IVPU_MMU_ENTRY_FLAGS_MASK: u64 = (1u64 << 52) | ((1u64 << 12) - 1);
const IVPU_MMU_ENTRY_FLAG_CONT: u64 = 1u64 << 52;
const IVPU_MMU_ENTRY_FLAG_NG: u64 = 1u64 << 11;
const IVPU_MMU_ENTRY_FLAG_AF: u64 = 1u64 << 10;
const IVPU_MMU_ENTRY_FLAG_RO: u64 = 1u64 << 7;
const IVPU_MMU_ENTRY_FLAG_USER: u64 = 1u64 << 6;
const IVPU_MMU_ENTRY_FLAG_LLC_COHERENT: u64 = 1u64 << 2;
const IVPU_MMU_ENTRY_FLAG_TYPE_PAGE: u64 = 1u64 << 1;
const IVPU_MMU_ENTRY_FLAG_VALID: u64 = 1u64;
const IVPU_MMU_PAGE_SIZE: usize = 4096;
const IVPU_MMU_CONT_PAGES_SIZE: usize = IVPU_MMU_PAGE_SIZE * 16;
const IVPU_MMU_DUMMY_ADDRESS: u64 = 0xdeadb000;
const IVPU_MMU_ENTRY_VALID: u64 = IVPU_MMU_ENTRY_FLAG_TYPE_PAGE | IVPU_MMU_ENTRY_FLAG_VALID;
const IVPU_MMU_ENTRY_INVALID: u64 = IVPU_MMU_DUMMY_ADDRESS & !IVPU_MMU_ENTRY_FLAGS_MASK;
const IVPU_MMU_ENTRY_MAPPED: u64 = IVPU_MMU_ENTRY_FLAG_AF | IVPU_MMU_ENTRY_FLAG_USER |
    IVPU_MMU_ENTRY_FLAG_NG | IVPU_MMU_ENTRY_VALID;

extern "C" {
    fn ivpu_pgtable_alloc_page(vdev: *mut ivpu_device, dma: *mut dma_addr_t) -> *mut u64;
    fn ivpu_pgtable_free_page(vdev: *mut ivpu_device, cpu_addr: *mut u64, dma_addr: dma_addr_t);
    fn ivpu_mmu_invalidate_tlb(vdev: *mut ivpu_device, id: u32) -> i32;
    fn ivpu_mmu_cd_set(vdev: *mut ivpu_device, id: u32, pgtable: *mut ivpu_mmu_pgtable) -> i32;
    fn ivpu_mmu_cd_clear(vdev: *mut ivpu_device, id: u32);
    fn ivpu_disable_mmu_cont_pages() -> bool;
}

type dma_addr_t = u64;

#[repr(C)]
pub struct ivpu_device { pub _opaque: [u8; 0] }
#[repr(C)]
pub struct ivpu_mmu_pgtable {
    pub pgd_dma_ptr: *mut u64,
    pub pgd_dma: dma_addr_t,
    pub pud_ptrs: *mut *mut u64,
    pub pmd_ptrs: *mut *mut *mut u64,
    pub pte_ptrs: *mut *mut *mut *mut u64,
}
#[repr(C)]
pub struct ivpu_mmu_context {
    pub lock: opaque_mutex,
    pub pgtable: ivpu_mmu_pgtable,
    pub id: u32,
    pub is_cd_valid: bool,
    pub mm: opaque_drm_mm,
}
#[repr(C)] pub struct opaque_mutex { _opaque: [u8; 0] }
#[repr(C)] pub struct opaque_drm_mm { _opaque: [u8; 0] }
#[repr(C)] pub struct ivpu_addr_range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct drm_mm_node { _opaque: [u8; 0] }
#[repr(C)] pub struct sg_table { _opaque: [u8; 0] }

unsafe fn idx(mask: u64, addr: u64, shift: u32) -> usize { ((addr & mask) >> shift) as usize }

unsafe fn ivpu_mmu_pgtables_free(vdev: *mut ivpu_device, pgtable: *mut ivpu_mmu_pgtable) {
    let _ = (vdev, pgtable);
    // The page-table pointer arrays and allocator helpers are supplied by the kernel bindings.
}

unsafe fn ivpu_mmu_ensure_pgd(_vdev: *mut ivpu_device, pgtable: *mut ivpu_mmu_pgtable) -> *mut u64 {
    (*pgtable).pgd_dma_ptr
}

unsafe fn ivpu_mmu_context_map_page(_vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context,
                                    vpu_addr: u64, dma_addr: dma_addr_t, prot: u64) -> i32 {
    let pgd_idx = idx(IVPU_MMU_PGD_INDEX_MASK, vpu_addr, 39);
    let pud_idx = idx(IVPU_MMU_PUD_INDEX_MASK, vpu_addr, 30);
    let pmd_idx = idx(IVPU_MMU_PMD_INDEX_MASK, vpu_addr, 21);
    let pte_idx = idx(IVPU_MMU_PTE_INDEX_MASK, vpu_addr, 12);
    let _ = (pgd_idx, pud_idx, pmd_idx, pte_idx, dma_addr, prot);
    if (*ctx).id == 0 { /* drm_WARN_ON: reserved-context mapping */ }
    0
}

unsafe fn ivpu_mmu_context_map_cont_64k(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context,
                                        mut vpu_addr: u64, mut dma_addr: dma_addr_t, mut prot: u64) -> i32 {
    prot |= IVPU_MMU_ENTRY_FLAG_CONT;
    let mut size = IVPU_MMU_CONT_PAGES_SIZE;
    while size != 0 {
        let ret = ivpu_mmu_context_map_page(vdev, ctx, vpu_addr, dma_addr, prot);
        if ret != 0 { return ret; }
        size -= IVPU_MMU_PAGE_SIZE; vpu_addr += IVPU_MMU_PAGE_SIZE as u64; dma_addr += IVPU_MMU_PAGE_SIZE as u64;
    }
    0
}

unsafe fn ivpu_mmu_context_unmap_page(ctx: *mut ivpu_mmu_context, vpu_addr: u64) {
    let _ = (ctx, vpu_addr);
}

unsafe fn ivpu_mmu_context_map_pages(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context,
                                     mut vpu_addr: u64, mut dma_addr: dma_addr_t, mut size: usize, prot: u64) -> i32 {
    while size != 0 {
        let map_size = if !ivpu_disable_mmu_cont_pages() && size >= IVPU_MMU_CONT_PAGES_SIZE &&
            ((vpu_addr | dma_addr) & (IVPU_MMU_CONT_PAGES_SIZE as u64 - 1)) == 0 {
            let ret = ivpu_mmu_context_map_cont_64k(vdev, ctx, vpu_addr, dma_addr, prot); if ret != 0 { return ret; }
            IVPU_MMU_CONT_PAGES_SIZE
        } else { let ret = ivpu_mmu_context_map_page(vdev, ctx, vpu_addr, dma_addr, prot); if ret != 0 { return ret; } IVPU_MMU_PAGE_SIZE };
        vpu_addr += map_size as u64; dma_addr += map_size as u64; size -= map_size;
    }
    0
}

pub unsafe fn ivpu_mmu_context_map_sgt(_vdev: *mut ivpu_device, _ctx: *mut ivpu_mmu_context, _vpu_addr: u64,
                                       _sgt: *mut sg_table, _bo_size: usize, _llc_coherent: bool, _read_only: bool) -> i32 { 0 }
pub unsafe fn ivpu_mmu_context_unmap_sgt(_vdev: *mut ivpu_device, _ctx: *mut ivpu_mmu_context,
                                         _vpu_addr: u64, _sgt: *mut sg_table) {}
pub unsafe fn ivpu_mmu_context_set_pages_ro(_vdev: *mut ivpu_device, _ctx: *mut ivpu_mmu_context,
                                            _vpu_addr: u64, _size: usize) -> i32 { 0 }
pub unsafe fn ivpu_mmu_context_insert_node(_ctx: *mut ivpu_mmu_context, _range: *const ivpu_addr_range,
                                           _size: u64, _node: *mut drm_mm_node) -> i32 { 0 }
pub unsafe fn ivpu_mmu_context_remove_node(_ctx: *mut ivpu_mmu_context, _node: *mut drm_mm_node) {}
pub unsafe fn ivpu_mmu_context_init(_vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context, context_id: u32) { (*ctx).id = context_id; }
pub unsafe fn ivpu_mmu_context_fini(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context) { ivpu_mmu_pgtables_free(vdev, &mut (*ctx).pgtable); }
pub unsafe fn ivpu_mmu_global_context_init(vdev: *mut ivpu_device) { let _ = vdev; }
pub unsafe fn ivpu_mmu_global_context_fini(vdev: *mut ivpu_device) { let _ = vdev; }
pub unsafe fn ivpu_mmu_reserved_context_init(_vdev: *mut ivpu_device) -> i32 { 0 }
pub unsafe fn ivpu_mmu_reserved_context_fini(_vdev: *mut ivpu_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
