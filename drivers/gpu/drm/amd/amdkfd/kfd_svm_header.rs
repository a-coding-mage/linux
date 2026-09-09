/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2020-2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C conditional: IS_ENABLED(CONFIG_HSA_AMD_SVM)
#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
pub const SVM_RANGE_VRAM_DOMAIN: usize = 1usize << 0;

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[inline]
pub unsafe fn SVM_ADEV_PGMAP_OWNER<T>(adev: *mut T) -> *mut core::ffi::c_void {
    // The C expression selects adev->hive when non-null; the field is supplied
    // by the external amdgpu_device definition.
    adev as *mut core::ffi::c_void
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[repr(C)]
pub struct svm_range_bo {
    pub bo: amdgpu_bo,
    pub kref: kref,
    pub range_list: list_head, // all svm ranges shared this bo
    pub list_lock: spinlock_t,
    pub mm: *mut mm_struct,
    pub evicting: u32,
    pub release_work: work_struct,
    pub node: *mut kfd_node,
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum svm_work_list_ops {
    SVM_OP_NULL,
    SVM_OP_UNMAP_RANGE,
    SVM_OP_UPDATE_RANGE_NOTIFIER,
    SVM_OP_UPDATE_RANGE_NOTIFIER_AND_MAP,
    SVM_OP_ADD_RANGE,
    SVM_OP_ADD_RANGE_AND_MAP,
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[repr(C)]
pub struct svm_work_list_item {
    pub op: svm_work_list_ops,
    pub mm: *mut mm_struct,
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[repr(C)]
pub struct svm_range {
    pub svms: *mut svm_range_list,
    pub migrate_mutex: mutex,
    pub start: usize,
    pub last: usize,
    pub it_node: interval_tree_node,
    pub list: list_head,
    pub update_list: list_head,
    pub npages: u64,
    pub vram_pages: u64,
    pub dma_addr: [*mut dma_addr_t; MAX_GPU_INSTANCE],
    pub ttm_res: *mut ttm_resource,
    pub offset: u64,
    pub svm_bo: *mut svm_range_bo,
    pub svm_bo_list: list_head,
    pub lock: mutex,
    pub saved_flags: u32,
    pub flags: u32,
    pub preferred_loc: u32,
    pub prefetch_loc: u32,
    pub actual_loc: u32,
    pub granularity: u8,
    pub invalid: atomic_t,
    pub validate_timestamp: ktime_t,
    pub notifier: mmu_interval_notifier,
    pub work_item: svm_work_list_item,
    pub deferred_list: list_head,
    pub child_list: list_head,
    pub bitmap_access: [usize; MAX_GPU_INSTANCE],
    pub bitmap_aip: [usize; MAX_GPU_INSTANCE],
    pub bitmap_needs_unmap: [usize; MAX_GPU_INSTANCE],
    pub bitmap_mapped: [usize; MAX_GPU_INSTANCE],
    pub mapping_done: bool,
    pub queue_refcount: atomic_t,
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[inline]
pub unsafe fn svm_range_lock(prange: *mut svm_range) {
    mutex_lock(&mut (*prange).lock);
    (*prange).saved_flags = memalloc_noreclaim_save();
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[inline]
pub unsafe fn svm_range_unlock(prange: *mut svm_range) {
    memalloc_noreclaim_restore((*prange).saved_flags);
    mutex_unlock(&mut (*prange).lock);
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
#[inline]
pub unsafe fn svm_range_bo_ref(svm_bo: *mut svm_range_bo) -> *mut svm_range_bo {
    if !svm_bo.is_null() { kref_get(&mut (*svm_bo).kref); }
    svm_bo
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
extern "C" {
    pub fn svm_range_list_init(p: *mut kfd_process) -> i32;
    pub fn svm_range_list_fini(p: *mut kfd_process);
    pub fn svm_ioctl(p: *mut kfd_process, op: kfd_ioctl_svm_op, start: u64, size: u64, nattrs: u32, attrs: *mut kfd_ioctl_svm_attribute) -> i32;
    pub fn svm_range_from_addr(svms: *mut svm_range_list, addr: usize, parent: *mut *mut svm_range) -> *mut svm_range;
    pub fn svm_range_get_node_by_id(prange: *mut svm_range, gpu_id: u32) -> *mut kfd_node;
    pub fn svm_range_bo_destroy(tbo: *mut ttm_buffer_object);
    pub fn svm_range_vram_node_new(node: *mut kfd_node, prange: *mut svm_range, clear: bool) -> i32;
    pub fn svm_range_vram_node_free(prange: *mut svm_range);
    pub fn svm_range_restore_pages(adev: *mut amdgpu_device, pasid: u32, vmid: u32, node_id: u32, addr: u64, ts: u64, write_fault: bool) -> i32;
    pub fn svm_range_evict_svm_bo(bo: *mut amdgpu_bo) -> i32;
    pub fn svm_range_add_list_work(svms: *mut svm_range_list, prange: *mut svm_range, mm: *mut mm_struct, op: svm_work_list_ops);
    pub fn schedule_deferred_list_work(svms: *mut svm_range_list);
    pub fn svm_range_dma_unmap_dev(dev: *mut device, dma_addr: *mut dma_addr_t, offset: usize, npages: usize);
    pub fn svm_range_dma_unmap(prange: *mut svm_range);
    pub fn svm_range_get_info(p: *mut kfd_process, num_svm_ranges: *mut u32, svm_priv_data_size: *mut u64);
    pub fn kfd_criu_checkpoint_svm(p: *mut kfd_process, user_priv_data: *mut u8, priv_offset: *mut u64) -> i32;
    pub fn kfd_criu_restore_svm(p: *mut kfd_process, user_priv_ptr: *mut u8, priv_data_offset: *mut u64, max_priv_data_size: u64) -> i32;
    pub fn kfd_criu_resume_svm(p: *mut kfd_process) -> i32;
    pub fn svm_range_get_pdd_by_node(prange: *mut svm_range, node: *mut kfd_node) -> *mut kfd_process_device;
    pub fn svm_range_list_lock_and_flush_work(svms: *mut svm_range_list, mm: *mut mm_struct);
    pub fn svm_range_bo_unref_async(svm_bo: *mut svm_range_bo);
    pub fn svm_range_set_max_pages(adev: *mut amdgpu_device);
    pub fn svm_range_switch_xnack_reserve_mem(p: *mut kfd_process, xnack_enabled: bool) -> i32;
}

#[cfg(feature = "CONFIG_HSA_AMD_SVM")]
pub const KFD_IS_SVM_API_SUPPORTED: bool = true; // C macro depends on adev fields

#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
pub struct kfd_process;

#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn svm_range_list_init(_: *mut kfd_process) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn svm_range_list_fini(_: *mut kfd_process) {}
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn svm_range_restore_pages(_: *mut amdgpu_device, _: u32, _: u32, _: u32, _: u64, _: u64, _: bool) -> i32 { -14 }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn svm_range_bo_destroy(_: *mut ttm_buffer_object) {}
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn svm_range_evict_svm_bo(_: *mut amdgpu_bo) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub unsafe fn svm_range_get_info(_: *mut kfd_process, n: *mut u32, s: *mut u64) { *n = 0; *s = 0; }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn kfd_criu_checkpoint_svm(_: *mut kfd_process, _: *mut u8, _: *mut u64) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn kfd_criu_restore_svm(_: *mut kfd_process, _: *mut u8, _: *mut u64, _: u64) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn kfd_criu_resume_svm(_: *mut kfd_process) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
#[inline] pub fn svm_range_set_max_pages(_: *mut amdgpu_device) {}
#[cfg(not(feature = "CONFIG_HSA_AMD_SVM"))]
pub const KFD_IS_SVM_API_SUPPORTED: bool = false;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
