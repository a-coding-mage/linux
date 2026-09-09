/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel/driver translation.

pub const AMDGPU_PL_GDS: u32 = TTM_PL_PRIV + 0;
pub const AMDGPU_PL_GWS: u32 = TTM_PL_PRIV + 1;
pub const AMDGPU_PL_OA: u32 = TTM_PL_PRIV + 2;
pub const AMDGPU_PL_PREEMPT: u32 = TTM_PL_PRIV + 3;
pub const AMDGPU_PL_DOORBELL: u32 = TTM_PL_PRIV + 4;
pub const AMDGPU_PL_MMIO_REMAP: u32 = TTM_PL_PRIV + 5;
pub const __AMDGPU_PL_NUM: u32 = TTM_PL_PRIV + 6;
pub const AMDGPU_GTT_MAX_TRANSFER_SIZE: u64 = 1u64 << 22;

pub static mut amdgpu_vram_mgr_attr_group: attribute_group = attribute_group;
pub static mut amdgpu_gtt_mgr_attr_group: attribute_group = attribute_group;

pub struct hmm_range;

#[repr(C)]
pub struct amdgpu_gtt_mgr {
    pub manager: ttm_resource_manager,
    pub mm: drm_mm,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct amdgpu_ttm_buffer_entity {
    pub base: drm_sched_entity,
    pub lock: mutex,
    pub gart_node: drm_mm_node,
    pub gart_window_offs: [u64; 2],
}

#[repr(C)]
pub enum amdgpu_resv_region_id {
    AMDGPU_RESV_STOLEN_VGA,
    AMDGPU_RESV_STOLEN_EXTENDED,
    AMDGPU_RESV_STOLEN_RESERVED,
    AMDGPU_RESV_FW,
    AMDGPU_RESV_FW_EXTEND,
    AMDGPU_RESV_FW_VRAM_USAGE,
    AMDGPU_RESV_DRV_VRAM_USAGE,
    AMDGPU_RESV_MEM_TRAIN,
    AMDGPU_RESV_MAX,
}

#[repr(C)]
pub struct amdgpu_vram_resv {
    pub offset: u64,
    pub size: u64,
    pub bo: *mut amdgpu_bo,
    pub cpu_ptr: *mut core::ffi::c_void,
    pub needs_cpu_map: bool,
}

#[repr(C)]
pub struct amdgpu_mman {
    pub bdev: ttm_device,
    pub ttm_pools: *mut ttm_pool,
    pub initialized: bool,
    pub aper_base_kaddr: *mut core::ffi::c_void,
    pub buffer_funcs: *const amdgpu_buffer_funcs,
    pub buffer_funcs_scheds: [*mut drm_gpu_scheduler; AMDGPU_MAX_RINGS],
    pub num_buffer_funcs_scheds: u32,
    pub buffer_funcs_enabled: bool,
    pub default_entity: amdgpu_ttm_buffer_entity,
    pub clear_entities: *mut amdgpu_ttm_buffer_entity,
    pub next_clear_entity: atomic_t,
    pub num_clear_entities: u32,
    pub move_entities: [amdgpu_ttm_buffer_entity; TTM_NUM_MOVE_FENCES],
    pub next_move_entity: atomic_t,
    pub num_move_entities: u32,
    pub vram_mgr: amdgpu_vram_mgr,
    pub gtt_mgr: amdgpu_gtt_mgr,
    pub preempt_mgr: ttm_resource_manager,
    pub keep_stolen_vga_memory: bool,
    pub resv_region: [amdgpu_vram_resv; AMDGPU_RESV_MAX as usize],
    pub sdma_access_bo: *mut amdgpu_bo,
    pub sdma_access_ptr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct amdgpu_copy_mem {
    pub bo: *mut ttm_buffer_object,
    pub mem: *mut ttm_resource,
    pub offset: usize,
}

pub const AMDGPU_COPY_FLAGS_TMZ: u32 = 1 << 0;
pub const AMDGPU_COPY_FLAGS_READ_DECOMPRESSED: u32 = 1 << 1;
pub const AMDGPU_COPY_FLAGS_WRITE_COMPRESSED: u32 = 1 << 2;
pub const AMDGPU_COPY_FLAGS_MAX_COMPRESSED_SHIFT: u32 = 3;
pub const AMDGPU_COPY_FLAGS_MAX_COMPRESSED_MASK: u32 = 0x03;
pub const AMDGPU_COPY_FLAGS_NUMBER_TYPE_SHIFT: u32 = 5;
pub const AMDGPU_COPY_FLAGS_NUMBER_TYPE_MASK: u32 = 0x07;
pub const AMDGPU_COPY_FLAGS_DATA_FORMAT_SHIFT: u32 = 8;
pub const AMDGPU_COPY_FLAGS_DATA_FORMAT_MASK: u32 = 0x3f;
pub const AMDGPU_COPY_FLAGS_WRITE_COMPRESS_DISABLE_SHIFT: u32 = 14;
pub const AMDGPU_COPY_FLAGS_WRITE_COMPRESS_DISABLE_MASK: u32 = 0x1;

#[macro_export]
macro_rules! AMDGPU_COPY_FLAGS_SET { ($field:ident, $value:expr) => { (($value as u32 & concat_idents!(AMDGPU_COPY_FLAGS_, $field, _MASK)) << concat_idents!(AMDGPU_COPY_FLAGS_, $field, _SHIFT)) }; }
#[macro_export]
macro_rules! AMDGPU_COPY_FLAGS_GET { ($value:expr, $field:ident) => { (($value as u32 >> concat_idents!(AMDGPU_COPY_FLAGS_, $field, _SHIFT)) & concat_idents!(AMDGPU_COPY_FLAGS_, $field, _MASK)) }; }

extern "C" {
    pub fn amdgpu_gtt_mgr_init(adev: *mut amdgpu_device, gtt_size: u64) -> i32;
    pub fn amdgpu_gtt_mgr_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_preempt_mgr_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_preempt_mgr_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_preempt_mgr_sysfs_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_vram_mgr_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_vram_mgr_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_gtt_mgr_has_gart_addr(mem: *mut ttm_resource) -> bool;
    pub fn amdgpu_gtt_mgr_mark_bo_teardown(tbo: *mut ttm_buffer_object);
    pub fn amdgpu_gtt_mgr_recover(mgr: *mut amdgpu_gtt_mgr);
    pub fn amdgpu_gtt_mgr_alloc_entries(mgr: *mut amdgpu_gtt_mgr, mm_node: *mut drm_mm_node, num_pages: u64, mode: drm_mm_insert_mode) -> i32;
    pub fn amdgpu_gtt_mgr_free_entries(mgr: *mut amdgpu_gtt_mgr, mm_node: *mut drm_mm_node);
    pub fn amdgpu_preempt_mgr_usage(man: *mut ttm_resource_manager) -> u64;
    pub fn amdgpu_vram_mgr_bo_visible_size(bo: *mut amdgpu_bo) -> u64;
    pub fn amdgpu_vram_mgr_alloc_sgt(adev: *mut amdgpu_device, mem: *mut ttm_resource, offset: u64, size: u64, dev: *mut device, dir: dma_data_direction, sgt: *mut *mut sg_table) -> i32;
    pub fn amdgpu_vram_mgr_free_sgt(dev: *mut device, dir: dma_data_direction, sgt: *mut sg_table);
    pub fn amdgpu_vram_mgr_vis_usage(mgr: *mut amdgpu_vram_mgr) -> u64;
    pub fn amdgpu_vram_mgr_reserve_range(mgr: *mut amdgpu_vram_mgr, start: u64, size: u64) -> i32;
    pub fn amdgpu_vram_mgr_query_page_status(mgr: *mut amdgpu_vram_mgr, start: u64) -> i32;
    pub fn amdgpu_vram_mgr_clear_reset_blocks(adev: *mut amdgpu_device);
    pub fn amdgpu_res_cpu_visible(adev: *mut amdgpu_device, res: *mut ttm_resource) -> bool;
    pub fn amdgpu_ttm_init_vram_resv(adev: *mut amdgpu_device, id: amdgpu_resv_region_id, offset: u64, size: u64, needs_cpu_map: bool);
    pub fn amdgpu_ttm_mark_vram_reserved(adev: *mut amdgpu_device, id: amdgpu_resv_region_id) -> i32;
    pub fn amdgpu_ttm_unmark_vram_reserved(adev: *mut amdgpu_device, id: amdgpu_resv_region_id);
    pub fn amdgpu_ttm_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_ttm_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_ttm_enable_buffer_funcs(adev: *mut amdgpu_device);
    pub fn amdgpu_ttm_disable_buffer_funcs(adev: *mut amdgpu_device);
    pub fn amdgpu_copy_buffer(adev: *mut amdgpu_device, entity: *mut amdgpu_ttm_buffer_entity, src_offset: u64, dst_offset: u64, byte_count: u32, resv: *mut dma_resv, fence: *mut *mut dma_fence, vm_needs_flush: bool, copy_flags: u32) -> i32;
    pub fn amdgpu_ttm_clear_buffer(entity: *mut amdgpu_ttm_buffer_entity, bo: *mut amdgpu_bo, resv: *mut dma_resv, out_fence: *mut *mut dma_fence, consider_clear_status: bool, k_job_id: u64) -> i32;
    pub fn amdgpu_ttm_next_clear_entity(adev: *mut amdgpu_device) -> *mut amdgpu_ttm_buffer_entity;
    pub fn amdgpu_ttm_alloc_gart(bo: *mut ttm_buffer_object) -> i32;
    pub fn amdgpu_ttm_recover_gart(tbo: *mut ttm_buffer_object);
    pub fn amdgpu_ttm_domain_start(adev: *mut amdgpu_device, type_: u32) -> u64;
}

#[cfg(feature = "CONFIG_DRM_AMDGPU_USERPTR")]
extern "C" { pub fn amdgpu_ttm_tt_get_user_pages(bo: *mut amdgpu_bo, range: *mut amdgpu_hmm_range) -> i32; }
#[cfg(not(feature = "CONFIG_DRM_AMDGPU_USERPTR"))]
pub unsafe fn amdgpu_ttm_tt_get_user_pages(_: *mut amdgpu_bo, _: *mut amdgpu_hmm_range) -> i32 { -1 }

#[inline]
pub unsafe fn amdgpu_compute_gart_address(gmc: *mut amdgpu_gmc, entity: *mut amdgpu_ttm_buffer_entity, index: usize) -> u64 { (*gmc).gart_start + (*entity).gart_window_offs[index] }

#[inline]
pub unsafe fn amdgpu_gtt_node_to_byte_offset(gtt_node: *const drm_mm_node) -> u64 { (*gtt_node).start * PAGE_SIZE as u64 }

extern "C" {
    pub fn amdgpu_ttm_tt_set_user_pages(ttm: *mut ttm_tt, range: *mut amdgpu_hmm_range);
    pub fn amdgpu_ttm_tt_get_userptr(tbo: *const ttm_buffer_object, user_addr: *mut u64) -> i32;
    pub fn amdgpu_ttm_tt_set_userptr(bo: *mut ttm_buffer_object, addr: u64, flags: u32) -> i32;
    pub fn amdgpu_ttm_tt_has_userptr(ttm: *mut ttm_tt) -> bool;
    pub fn amdgpu_ttm_tt_get_usermm(ttm: *mut ttm_tt) -> *mut mm_struct;
    pub fn amdgpu_ttm_tt_affect_userptr(ttm: *mut ttm_tt, start: usize, end: usize, userptr: *mut usize) -> bool;
    pub fn amdgpu_ttm_tt_userptr_invalidated(ttm: *mut ttm_tt, last_invalidated: *mut i32) -> bool;
    pub fn amdgpu_ttm_tt_is_userptr(ttm: *mut ttm_tt) -> bool;
    pub fn amdgpu_ttm_tt_is_readonly(ttm: *mut ttm_tt) -> bool;
    pub fn amdgpu_ttm_tt_pde_flags(ttm: *mut ttm_tt, mem: *mut ttm_resource) -> u64;
    pub fn amdgpu_ttm_tt_pte_flags(adev: *mut amdgpu_device, ttm: *mut ttm_tt, mem: *mut ttm_resource) -> u64;
    pub fn amdgpu_ttm_evict_resources(adev: *mut amdgpu_device, mem_type: i32) -> i32;
    pub fn amdgpu_ttm_debugfs_init(adev: *mut amdgpu_device);
    pub fn amdgpu_ttm_mmio_remap_alloc_sgt(adev: *mut amdgpu_device, res: *mut ttm_resource, dev: *mut device, dir: dma_data_direction, sgt: *mut *mut sg_table) -> i32;
    pub fn amdgpu_ttm_mmio_remap_free_sgt(dev: *mut device, dir: dma_data_direction, sgt: *mut sg_table);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
