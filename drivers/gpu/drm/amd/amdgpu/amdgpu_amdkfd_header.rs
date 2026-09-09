/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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

/* amdgpu_amdkfd.h defines the private interface between amdgpu and amdkfd. */

use core::ffi::c_void;

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;

// Types supplied by the included kernel and driver headers.
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct dma_buf { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_bo { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_bo_va { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_hmm_range { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_sync { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_vm { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_xcp { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_reset_context { _private: [u8; 0] }
#[repr(C)] pub struct kfd_process_device { _private: [u8; 0] }
#[repr(C)] pub struct kfd_dev { _private: [u8; 0] }
#[repr(C)] pub struct drm_client_dev { _private: [u8; 0] }
#[repr(C)] pub struct dev_pagemap { _private: [u8; 0] }
#[repr(C)] pub struct mmu_interval_notifier { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct kfd_local_mem_info { _private: [u8; 0] }
#[repr(C)] pub struct kfd_vm_fault_info { _private: [u8; 0] }
#[repr(C)] pub struct tile_config { _private: [u8; 0] }
#[repr(C)] pub struct kgd2kfd_shared_resources { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_iv_entry { _private: [u8; 0] }
pub type pasid_notify = unsafe extern "C" fn(*mut c_void);

pub const MAX_XCP: usize = 8; // build-time value supplied by the driver
pub const TASK_COMM_LEN: usize = 16; // supplied by Linux

pub static mut amdgpu_amdkfd_total_mem_size: u64 = 0;

#[repr(C)] #[derive(Copy, Clone)]
pub enum TLB_FLUSH_TYPE { TLB_FLUSH_LEGACY = 0, TLB_FLUSH_LIGHTWEIGHT, TLB_FLUSH_HEAVYWEIGHT }

#[repr(C)] #[derive(Copy, Clone)]
pub enum kfd_mem_attachment_type { KFD_MEM_ATT_SHARED, KFD_MEM_ATT_USERPTR, KFD_MEM_ATT_DMABUF, KFD_MEM_ATT_SG }

#[repr(C)] pub struct kfd_mem_attachment {
    pub list: list_head, pub type_: kfd_mem_attachment_type, pub is_mapped: bool,
    pub bo_va: *mut amdgpu_bo_va, pub adev: *mut amdgpu_device, pub va: u64, pub pte_flags: u64,
}

#[repr(C)] pub struct kgd_mem {
    pub lock: mutex, pub bo: *mut amdgpu_bo, pub dmabuf: *mut dma_buf,
    pub range: *mut amdgpu_hmm_range, pub attachments: list_head, pub validate_list: list_head,
    pub domain: u32, pub mapped_to_gpu_memory: core::ffi::c_uint, pub va: u64, pub alloc_flags: u32,
    pub invalid: u32, pub process_info: *mut amdkfd_process_info, pub sync: amdgpu_sync,
    pub gem_handle: u32, pub aql_queue: bool, pub is_imported: bool,
}

#[repr(C)] pub struct amdgpu_amdkfd_fence {
    pub base: dma_fence, pub mm: *mut mm_struct, pub lock: spinlock_t,
    pub timeline_name: [u8; TASK_COMM_LEN], pub context_id: u16,
}
#[repr(C)] pub struct amdgpu_kfd_dev {
    pub dev: *mut kfd_dev, pub vram_used: [i64; MAX_XCP], pub vram_used_aligned: [u64; MAX_XCP],
    pub init_complete: bool, pub reset_work: work_struct, pub client: drm_client_dev, pub pgmap: dev_pagemap,
}

#[repr(C)] #[derive(Copy, Clone)] pub enum kgd_engine_type {
    KGD_ENGINE_PFP = 1, KGD_ENGINE_ME, KGD_ENGINE_CE, KGD_ENGINE_MEC1, KGD_ENGINE_MEC2,
    KGD_ENGINE_RLC, KGD_ENGINE_SDMA1, KGD_ENGINE_SDMA2, KGD_ENGINE_MAX,
}
#[repr(C)] pub struct amdkfd_process_info {
    pub vm_list_head: list_head, pub kfd_bo_list: list_head, pub userptr_valid_list: list_head,
    pub userptr_inval_list: list_head, pub lock: mutex, pub n_vms: core::ffi::c_uint,
    pub eviction_fence: *mut amdgpu_amdkfd_fence, pub notifier_lock: mutex, pub evicted_bos: u32,
    pub context_id: u16, pub restore_userptr_work: delayed_work, pub pid: *mut pid,
    pub block_mmu_notifications: bool,
}

extern "C" {
    pub fn amdgpu_amdkfd_init() -> i32; pub fn amdgpu_amdkfd_fini();
    pub fn amdgpu_amdkfd_teardown_processes(adev: *mut amdgpu_device);
    pub fn amdgpu_amdkfd_suspend(adev: *mut amdgpu_device, suspend_proc: bool);
    pub fn amdgpu_amdkfd_resume(adev: *mut amdgpu_device, resume_proc: bool) -> i32;
    pub fn amdgpu_amdkfd_suspend_process(adev: *mut amdgpu_device);
    pub fn amdgpu_amdkfd_resume_process(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_amdkfd_interrupt(adev: *mut amdgpu_device, ih_ring_entry: *const c_void);
    pub fn amdgpu_amdkfd_device_probe(adev: *mut amdgpu_device); pub fn amdgpu_amdkfd_device_init(adev: *mut amdgpu_device);
    pub fn amdgpu_amdkfd_device_fini_sw(adev: *mut amdgpu_device);
    pub fn amdgpu_amdkfd_check_and_lock_kfd(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_amdkfd_unlock_kfd(adev: *mut amdgpu_device);
    pub fn amdgpu_amdkfd_submit_ib(adev: *mut amdgpu_device, engine: kgd_engine_type, vmid: u32, gpu_addr: u64, ib_cmd: *mut u32, ib_len: u32) -> i32;
    pub fn amdgpu_amdkfd_set_compute_idle(adev: *mut amdgpu_device, idle: bool);
    pub fn amdgpu_amdkfd_have_atomics_support(adev: *mut amdgpu_device) -> bool;
    pub fn amdgpu_amdkfd_is_kfd_vmid(adev: *mut amdgpu_device, vmid: u32) -> bool;
    pub fn amdgpu_amdkfd_pre_reset(adev: *mut amdgpu_device, reset_context: *mut amdgpu_reset_context) -> i32;
    pub fn amdgpu_amdkfd_post_reset(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_amdkfd_gpu_reset(adev: *mut amdgpu_device);
    pub fn amdgpu_queue_mask_bit_to_set_resource_bit(adev: *mut amdgpu_device, queue_bit: i32) -> i32;
    pub fn amdgpu_amdkfd_fence_create(context: u64, mm: *mut mm_struct, context_id: u16) -> *mut amdgpu_amdkfd_fence;
    pub fn amdgpu_amdkfd_drm_client_create(adev: *mut amdgpu_device) -> i32;
}

// Configuration-gated declarations and inline fallbacks are preserved below.
// CONFIG_DEBUG_FS and CONFIG_HSA_AMD are build-time conditions supplied by the kernel.
#[cfg(feature = "CONFIG_DEBUG_FS")] extern "C" { pub fn kfd_debugfs_kfd_mem_limits(m: *mut seq_file, data: *mut c_void) -> i32; }

#[inline] pub unsafe fn amdkfd_fence_check_mm(_f: *mut dma_fence, _mm: *mut mm_struct) -> bool { false }
#[inline] pub unsafe fn to_amdgpu_amdkfd_fence(_f: *mut dma_fence) -> *mut amdgpu_amdkfd_fence { core::ptr::null_mut() }
#[inline] pub unsafe fn amdgpu_amdkfd_remove_all_eviction_fences(_bo: *mut amdgpu_bo) {}
#[inline] pub unsafe fn amdgpu_amdkfd_evict_userptr(_mni: *mut mmu_interval_notifier, _cur_seq: usize, _mem: *mut kgd_mem) -> i32 { 0 }
#[inline] pub unsafe fn amdgpu_amdkfd_bo_validate_and_fence(_bo: *mut amdgpu_bo, _domain: u32, _fence: *mut dma_fence) -> i32 { 0 }
#[inline] pub unsafe fn amdgpu_amdkfd_set_sigbus_delay(_task: *mut task_struct, _ms: u32) -> i32 { -95 }

// Shared API.
extern "C" {
    pub fn amdgpu_amdkfd_alloc_kernel_mem(adev: *mut amdgpu_device, size: usize, domain: u32, mem_obj: *mut *mut c_void, gpu_addr: *mut u64, cpu_ptr: *mut *mut c_void, mqd_gfx9: bool) -> i32;
    pub fn amdgpu_amdkfd_free_kernel_mem(adev: *mut amdgpu_device, mem_obj: *mut *mut c_void);
    pub fn amdgpu_amdkfd_alloc_gws(adev: *mut amdgpu_device, size: usize, mem_obj: *mut *mut c_void) -> i32;
    pub fn amdgpu_amdkfd_free_gws(adev: *mut amdgpu_device, mem_obj: *mut c_void);
    pub fn amdgpu_amdkfd_add_gws_to_process(info: *mut c_void, gws: *mut c_void, mem: *mut *mut kgd_mem) -> i32;
    pub fn amdgpu_amdkfd_remove_gws_from_process(info: *mut c_void, mem: *mut c_void) -> i32;
    pub fn amdgpu_amdkfd_get_fw_version(adev: *mut amdgpu_device, type_: kgd_engine_type) -> u32;
    pub fn amdgpu_amdkfd_get_local_mem_info(adev: *mut amdgpu_device, mem_info: *mut kfd_local_mem_info, xcp: *mut amdgpu_xcp);
    pub fn amdgpu_amdkfd_get_gpu_clock_counter(adev: *mut amdgpu_device) -> u64;
    pub fn amdgpu_amdkfd_get_max_engine_clock_in_mhz(adev: *mut amdgpu_device) -> u32;
    pub fn amdgpu_amdkfd_get_pcie_bandwidth_mbytes(adev: *mut amdgpu_device, is_min: bool) -> i32;
    pub fn amdgpu_amdkfd_compute_active(adev: *mut amdgpu_device, node_id: u32) -> bool;
}

#[inline] pub unsafe fn amdgpu_amdkfd_gpuvm_init_mem_limits() {}
#[inline] pub unsafe fn amdgpu_amdkfd_gpuvm_destroy_cb(_adev: *mut amdgpu_device, _vm: *mut amdgpu_vm) {}
#[inline] pub unsafe fn amdgpu_amdkfd_release_notify(_bo: *mut amdgpu_bo) {}
#[inline] pub unsafe fn kgd2kfd_init_zone_device(_adev: *mut amdgpu_device) -> i32 { 0 }

// Remaining GPUVM and KGD2KFD declarations are external interfaces from the included headers.
extern "C" {
    pub fn amdgpu_amdkfd_xcp_memory_size(adev: *mut amdgpu_device, xcp_id: i32) -> u64;
    pub fn kgd2kfd_quiesce_mm(mm: *mut mm_struct, trigger: u32) -> i32;
    pub fn kgd2kfd_resume_mm(mm: *mut mm_struct) -> i32;
    pub fn kgd2kfd_schedule_evict_and_restore_process(mm: *mut mm_struct, context_id: u16, fence: *mut dma_fence) -> i32;
}

#[inline] pub unsafe fn KFD_XCP_MEM_ID(_adev: *mut amdgpu_device, _xcp_id: i32) -> i32 { -1 }
#[inline] pub unsafe fn KFD_XCP_MEMORY_SIZE(adev: *mut amdgpu_device, xcp_id: i32) -> u64 { amdgpu_amdkfd_xcp_memory_size(adev, xcp_id) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
