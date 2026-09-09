/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2023 Intel Corporation
 */

// C dependency: <drm/drm_mm.h>

pub const IVPU_MMU_PGTABLE_ENTRIES: u64 = 512u64;

// Forward declarations supplied by other translation units.
pub enum ivpu_device {}
pub enum ivpu_file_priv {}
pub enum ivpu_addr_range {}
pub enum sg_table {}

// External kernel types supplied by other translation units.
pub type u64_t = u64;
pub type u32_t = u32;
pub type dma_addr_t = u64;
pub type size_t = usize;
pub enum mutex {}
pub enum drm_mm {}
pub enum drm_mm_node {}

#[repr(C)]
pub struct ivpu_mmu_pgtable {
    pub pte_ptrs: [*mut *mut *mut u64; IVPU_MMU_PGTABLE_ENTRIES as usize],
    pub pmd_ptrs: [*mut *mut u64; IVPU_MMU_PGTABLE_ENTRIES as usize],
    pub pud_ptrs: [*mut u64; IVPU_MMU_PGTABLE_ENTRIES as usize],
    pub pgd_dma_ptr: *mut u64,
    pub pgd_dma: dma_addr_t,
}

#[repr(C)]
pub struct ivpu_mmu_context {
    pub lock: mutex, // Protects: mm, pgtable, is_cd_valid
    pub mm: drm_mm,
    pub pgtable: ivpu_mmu_pgtable,
    pub is_cd_valid: bool,
    pub id: u32,
}

extern "C" {
    pub fn ivpu_mmu_context_init(
        vdev: *mut ivpu_device,
        ctx: *mut ivpu_mmu_context,
        context_id: u32,
    );
    pub fn ivpu_mmu_context_fini(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context);
    pub fn ivpu_mmu_global_context_init(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_global_context_fini(vdev: *mut ivpu_device);
    pub fn ivpu_mmu_reserved_context_init(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_mmu_reserved_context_fini(vdev: *mut ivpu_device);

    pub fn ivpu_mmu_context_insert_node(
        ctx: *mut ivpu_mmu_context,
        range: *const ivpu_addr_range,
        size: u64,
        node: *mut drm_mm_node,
    ) -> i32;
    pub fn ivpu_mmu_context_remove_node(ctx: *mut ivpu_mmu_context, node: *mut drm_mm_node);

    pub fn ivpu_mmu_context_map_sgt(
        vdev: *mut ivpu_device,
        ctx: *mut ivpu_mmu_context,
        vpu_addr: u64,
        sgt: *mut sg_table,
        bo_size: usize,
        llc_coherent: bool,
        read_only: bool,
    ) -> i32;
    pub fn ivpu_mmu_context_unmap_sgt(
        vdev: *mut ivpu_device,
        ctx: *mut ivpu_mmu_context,
        vpu_addr: u64,
        sgt: *mut sg_table,
    );
    pub fn ivpu_mmu_context_set_pages_ro(
        vdev: *mut ivpu_device,
        ctx: *mut ivpu_mmu_context,
        vpu_addr: u64,
        size: usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
