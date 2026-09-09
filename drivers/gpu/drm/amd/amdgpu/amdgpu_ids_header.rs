/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.
pub struct list_head;
pub struct mutex;
pub struct dma_fence;
pub struct dma_resv;

/* maximum number of VMIDs */
pub const AMDGPU_NUM_VMID: usize = 16;

pub struct amdgpu_device;
pub struct amdgpu_fpriv;
pub struct amdgpu_vm;
pub struct amdgpu_ring;
pub struct amdgpu_sync;
pub struct amdgpu_job;

#[repr(C)]
pub struct amdgpu_vmid {
    pub list: list_head,
    pub active: amdgpu_sync,
    pub last_flush: *mut dma_fence,
    pub owner: u64,

    pub pd_gpu_addr: u64,
    /* last flushed PD/PT update */
    pub flushed_updates: u64,

    pub current_gpu_reset_count: u32,

    pub gds_base: u32,
    pub gds_size: u32,
    pub gws_base: u32,
    pub gws_size: u32,
    pub oa_base: u32,
    pub oa_size: u32,

    pub pasid: u32,
    pub pasid_mapping: *mut dma_fence,
}

#[repr(C)]
pub struct amdgpu_vmid_mgr {
    pub lock: mutex,
    pub num_ids: u32,
    pub ids_lru: list_head,
    pub ids: [amdgpu_vmid; AMDGPU_NUM_VMID],
    pub reserved_vmid: bool,
}

extern "C" {
    pub fn amdgpu_pasid_alloc(bits: u32, fpriv: *mut amdgpu_fpriv) -> i32;
    pub fn amdgpu_pasid_lock(flags: *mut ::core::ffi::c_ulong);
    pub fn amdgpu_pasid_unlock(flags: ::core::ffi::c_ulong);
    pub fn amdgpu_pasid_get_fpriv_locked(pasid: u32) -> *mut amdgpu_fpriv;
    pub fn amdgpu_pasid_free(pasid: u32);
    pub fn amdgpu_pasid_free_delayed(resv: *mut dma_resv, pasid: u32);
    pub fn amdgpu_pasid_mgr_cleanup();

    pub fn amdgpu_vmid_had_gpu_reset(
        adev: *mut amdgpu_device,
        id: *mut amdgpu_vmid,
    ) -> bool;
    pub fn amdgpu_vmid_uses_reserved(vm: *mut amdgpu_vm, vmhub: u32) -> bool;
    pub fn amdgpu_vmid_alloc_reserved(
        adev: *mut amdgpu_device,
        vm: *mut amdgpu_vm,
        vmhub: u32,
    ) -> i32;
    pub fn amdgpu_vmid_free_reserved(
        adev: *mut amdgpu_device,
        vm: *mut amdgpu_vm,
        vmhub: u32,
    );
    pub fn amdgpu_vmid_grab(
        vm: *mut amdgpu_vm,
        ring: *mut amdgpu_ring,
        job: *mut amdgpu_job,
        fence: *mut *mut dma_fence,
    ) -> i32;
    pub fn amdgpu_vmid_reset(adev: *mut amdgpu_device, vmhub: u32, vmid: u32);
    pub fn amdgpu_vmid_reset_all(adev: *mut amdgpu_device);

    pub fn amdgpu_vmid_mgr_init(adev: *mut amdgpu_device);
    pub fn amdgpu_vmid_mgr_fini(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
