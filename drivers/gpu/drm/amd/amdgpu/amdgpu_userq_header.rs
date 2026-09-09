/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by amdgpu_eviction_fence.h.

pub const AMDGPU_MAX_USERQ_COUNT: u32 = 512;

// C container_of macros; their containing-type/layout-dependent behavior is
// supplied by the surrounding translation unit.
#[macro_export]
macro_rules! to_ev_fence { ($f:expr) => { container_of!($f, amdgpu_eviction_fence, base) }; }
#[macro_export]
macro_rules! uq_mgr_to_fpriv { ($u:expr) => { container_of!($u, amdgpu_fpriv, userq_mgr) }; }
#[macro_export]
macro_rules! work_to_uq_mgr { ($w:expr, $name:ident) => { container_of!($w, amdgpu_userq_mgr, $name) }; }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_userq_state {
    AMDGPU_USERQ_STATE_UNMAPPED = 0,
    AMDGPU_USERQ_STATE_MAPPED,
    AMDGPU_USERQ_STATE_PREEMPTED,
    AMDGPU_USERQ_STATE_HUNG,
    AMDGPU_USERQ_STATE_INVALID_VA,
}

extern "C" {
    pub type amdgpu_mqd_prop;
    pub type amdgpu_bo;
    pub type amdgpu_userq_mgr;
    pub type amdgpu_vm;
    pub type amdgpu_userq_fence_driver;
    pub type dma_fence;
    pub type dentry;
    pub type drm_amdgpu_userq_in;
    pub type drm_device;
    pub type drm_file;
    pub type amdgpu_device;
    pub type amdgpu_eviction_fence_mgr;
    pub type amdgpu_bo_va_mapping;
    pub type amdgpu_eviction_fence;
    pub type amdgpu_fpriv;
    pub type work_struct;
    pub type mutex;
    pub type xarray;
    pub type delayed_work;
    pub type kref;
    pub type atomic_t;
}

#[repr(C)]
pub struct amdgpu_userq_obj {
    pub cpu_ptr: *mut core::ffi::c_void,
    pub gpu_addr: u64,
    pub obj: *mut amdgpu_bo,
}

#[repr(C)]
pub struct amdgpu_usermode_queue {
    pub queue_type: core::ffi::c_int,
    pub state: amdgpu_userq_state,
    pub doorbell_handle: u64,
    pub doorbell_index: u64,
    pub doorbell_offset: u32,
    pub flags: u64,
    pub userq_prop: *mut amdgpu_mqd_prop,
    pub userq_mgr: *mut amdgpu_userq_mgr,
    pub vm: *mut amdgpu_vm,
    pub mqd: amdgpu_userq_obj,
    pub db_obj: amdgpu_userq_obj,
    pub fw_obj: amdgpu_userq_obj,
    pub wptr_obj: amdgpu_userq_obj,
    pub fence_drv_lock: mutex,
    pub fence_drv_xa: xarray,
    pub fence_drv: *mut amdgpu_userq_fence_driver,
    pub last_fence: *mut dma_fence,
    pub xcp_id: u32,
    pub priority: core::ffi::c_int,
    pub debugfs_queue: *mut dentry,
    pub hang_detect_work: delayed_work,
    pub refcount: kref,
    pub userq_vas: amdgpu_userq_vas,
    pub gang_ctx_array_index: u32,
}

#[repr(C)]
pub union amdgpu_userq_vas {
    pub va: amdgpu_userq_va,
    pub va_array: [u64; 6],
}

#[repr(C)]
pub struct amdgpu_userq_va {
    pub queue_rb: u64,
    pub wptr: u64,
    pub rptr: u64,
    pub eop: u64,
    pub shadow: u64,
    pub csa: u64,
}

#[repr(C)]
pub struct amdgpu_userq_funcs {
    pub mqd_create: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue, *mut drm_amdgpu_userq_in) -> core::ffi::c_int>,
    pub mqd_update: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue, *mut drm_amdgpu_userq_in) -> core::ffi::c_int>,
    pub mqd_destroy: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue)>,
    pub unmap: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue) -> core::ffi::c_int>,
    pub map: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue) -> core::ffi::c_int>,
    pub preempt: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue) -> core::ffi::c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue) -> core::ffi::c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut amdgpu_usermode_queue) -> core::ffi::c_int>,
}

// Usermode queues for gfx.
#[repr(C)]
pub struct amdgpu_userq_mgr {
    pub userq_xa: xarray,
    pub userq_mutex: mutex,
    pub adev: *mut amdgpu_device,
    pub resume_work: delayed_work,
    pub file: *mut drm_file,
    pub proc_ctx_lock: mutex,
    pub proc_ctx_obj: amdgpu_userq_obj,
    pub proc_ctx_allocated: bool,
    pub proc_ctx_array_index: u32,
    pub reset_work: work_struct,
    pub userq_count: [atomic_t; AMDGPU_RING_TYPE_MAX],
}

#[repr(C)]
pub struct amdgpu_db_info {
    pub doorbell_handle: u64,
    pub queue_type: u32,
    pub doorbell_offset: u32,
    pub db_obj: *mut amdgpu_userq_obj,
}

extern "C" {
    pub fn amdgpu_userq_get(uq_mgr: *mut amdgpu_userq_mgr, qid: u32) -> *mut amdgpu_usermode_queue;
    pub fn amdgpu_userq_put(queue: *mut amdgpu_usermode_queue);
    pub fn amdgpu_userq_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> core::ffi::c_int;
    pub fn amdgpu_userq_mgr_init(userq_mgr: *mut amdgpu_userq_mgr, file_priv: *mut drm_file, adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_userq_mgr_cancel_reset_work(adev: *mut amdgpu_device);
    pub fn amdgpu_userq_mgr_cancel_resume(userq_mgr: *mut amdgpu_userq_mgr);
    pub fn amdgpu_userq_mgr_fini(userq_mgr: *mut amdgpu_userq_mgr);
    pub fn amdgpu_userq_evict(uq_mgr: *mut amdgpu_userq_mgr);
    pub fn amdgpu_userq_ensure_ev_fence(userq_mgr: *mut amdgpu_userq_mgr, evf_mgr: *mut amdgpu_eviction_fence_mgr);
    pub fn amdgpu_userq_get_supported_ip_mask(adev: *mut amdgpu_device) -> u32;
    pub fn amdgpu_userq_enabled(dev: *mut drm_device) -> bool;
    pub fn amdgpu_userq_suspend(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_userq_resume(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_userq_stop_sched_for_enforce_isolation(adev: *mut amdgpu_device, idx: u32) -> core::ffi::c_int;
    pub fn amdgpu_userq_start_sched_for_enforce_isolation(adev: *mut amdgpu_device, idx: u32) -> core::ffi::c_int;
    pub fn amdgpu_userq_reset_work(work: *mut work_struct);
    pub fn amdgpu_userq_pre_reset(adev: *mut amdgpu_device);
    pub fn amdgpu_userq_post_reset(adev: *mut amdgpu_device, vram_lost: bool) -> core::ffi::c_int;
    pub fn amdgpu_userq_start_hang_detect_work(queue: *mut amdgpu_usermode_queue);
    pub fn amdgpu_userq_process_fence_irq(adev: *mut amdgpu_device, doorbell: u32);
    pub fn amdgpu_userq_process_reset_irq(adev: *mut amdgpu_device, pasid: u32, doorbell_offset: u32);
    pub fn amdgpu_userq_input_va_validate(adev: *mut amdgpu_device, queue: *mut amdgpu_usermode_queue, addr: u64, expected_size: u64, va_out: *mut u64) -> core::ffi::c_int;
    pub fn amdgpu_userq_gem_va_unmap_validate(adev: *mut amdgpu_device, mapping: *mut amdgpu_bo_va_mapping);
}

// CP packs the per-process doorbell_id in CTXID0[9:0] on priv-fault.
pub const AMDGPU_CTXID0_DOORBELL_ID_MASK: u32 = 0x3ff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
