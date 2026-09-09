/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// External declarations supplied by the DRM scheduler and AMDGPU headers.

pub const AMDGPU_PREAMBLE_IB_PRESENT: u32 = 1 << 0;
pub const AMDGPU_PREAMBLE_IB_PRESENT_FIRST: u32 = 1 << 1;
pub const AMDGPU_HAVE_CTX_SWITCH: u32 = 1 << 2;
pub const AMDGPU_IB_PREEMPTED: u32 = 1 << 3;

// Preserves the C container_of macro dependency.
#[macro_export]
macro_rules! to_amdgpu_job {
    ($sched_job:expr) => {
        container_of!($sched_job, amdgpu_job, base)
    };
}

#[inline]
pub unsafe fn AMDGPU_JOB_GET_VMID(job: *const amdgpu_job) -> u32 {
    if !job.is_null() { (*job).vmid as u32 } else { 0 }
}

pub const AMDGPU_KERNEL_JOB_ID_VM_UPDATE: u64 = 18446744073709551615u64;
pub const AMDGPU_KERNEL_JOB_ID_VM_UPDATE_PDES: u64 = 18446744073709551614u64;
pub const AMDGPU_KERNEL_JOB_ID_VM_UPDATE_RANGE: u64 = 18446744073709551613u64;
pub const AMDGPU_KERNEL_JOB_ID_VM_PT_CLEAR: u64 = 18446744073709551612u64;
pub const AMDGPU_KERNEL_JOB_ID_TTM_MAP_BUFFER: u64 = 18446744073709551611u64;
pub const AMDGPU_KERNEL_JOB_ID_TTM_ACCESS_MEMORY_SDMA: u64 = 18446744073709551610u64;
pub const AMDGPU_KERNEL_JOB_ID_TTM_COPY_BUFFER: u64 = 18446744073709551609u64;
pub const AMDGPU_KERNEL_JOB_ID_CLEAR_ON_RELEASE: u64 = 18446744073709551608u64;
pub const AMDGPU_KERNEL_JOB_ID_MOVE_BLIT: u64 = 18446744073709551607u64;
pub const AMDGPU_KERNEL_JOB_ID_TTM_CLEAR_BUFFER: u64 = 18446744073709551606u64;
pub const AMDGPU_KERNEL_JOB_ID_CLEANER_SHADER: u64 = 18446744073709551605u64;
pub const AMDGPU_KERNEL_JOB_ID_FLUSH_GPU_TLB: u64 = 18446744073709551604u64;
pub const AMDGPU_KERNEL_JOB_ID_KFD_GART_MAP: u64 = 18446744073709551603u64;
pub const AMDGPU_KERNEL_JOB_ID_VCN_RING_TEST: u64 = 18446744073709551602u64;
pub const AMDGPU_KERNEL_JOB_ID_GFX_RING_TEST: u64 = 18446744073709551601u64;
pub const AMDGPU_KERNEL_JOB_ID_SDMA_RING_TEST: u64 = 18446744073709551600u64;
pub const AMDGPU_KERNEL_JOB_ID_VPE_RING_TEST: u64 = 18446744073709551599u64;
pub const AMDGPU_KERNEL_JOB_ID_RUN_SHADER: u64 = 18446744073709551598u64;

#[repr(C)]
pub struct amdgpu_job {
    pub base: drm_sched_job,
    pub vm: *mut amdgpu_vm,
    pub explicit_sync: amdgpu_sync,
    pub hw_fence: *mut amdgpu_fence,
    pub hw_vm_fence: *mut amdgpu_fence,
    pub gang_submit: *mut dma_fence,
    pub preamble_status: u32,
    pub preemption_status: u32,
    pub vm_needs_flush: bool,
    pub gds_switch_needed: bool,
    pub spm_update_needed: bool,
    pub vm_pd_addr: u64,
    pub vmid: u32,
    pub pasid: u32,
    pub gds_base: u32,
    pub gds_size: u32,
    pub gws_base: u32,
    pub gws_size: u32,
    pub oa_base: u32,
    pub oa_size: u32,
    pub generation: u64,
    pub uf_addr: u64,
    pub uf_sequence: u64,
    pub shadow_va: u64,
    pub csa_va: u64,
    pub gds_va: u64,
    pub init_shadow: bool,
    pub job_run_counter: u32,
    pub enforce_isolation: bool,
    pub run_cleaner_shader: bool,
    pub num_ibs: u32,
    pub ibs: [amdgpu_ib; 0],
}

#[inline]
pub unsafe fn amdgpu_job_ring(job: *mut amdgpu_job) -> *mut amdgpu_ring {
    to_amdgpu_ring((*(*(*job).base.entity).rq).sched)
}

extern "C" {
    pub fn amdgpu_job_alloc(adev: *mut amdgpu_device, vm: *mut amdgpu_vm,
        entity: *mut drm_sched_entity, owner: *mut core::ffi::c_void,
        num_ibs: u32, drm_client_id: u64, gfp_flags: gfp_t,
        job: *mut *mut amdgpu_job) -> i32;
    pub fn amdgpu_job_alloc_with_ib(adev: *mut amdgpu_device,
        entity: *mut drm_sched_entity, owner: *mut core::ffi::c_void,
        size: usize, pool_type: amdgpu_ib_pool_type, k_job_id: u64,
        job: *mut *mut amdgpu_job) -> i32;
    pub fn amdgpu_job_set_resources(job: *mut amdgpu_job, gds: *mut amdgpu_bo,
        gws: *mut amdgpu_bo, oa: *mut amdgpu_bo);
    pub fn amdgpu_job_free_resources(job: *mut amdgpu_job);
    pub fn amdgpu_job_set_gang_leader(job: *mut amdgpu_job, leader: *mut amdgpu_job);
    pub fn amdgpu_job_free(job: *mut amdgpu_job);
    pub fn amdgpu_job_submit(job: *mut amdgpu_job) -> *mut dma_fence;
    pub fn amdgpu_job_submit_direct(job: *mut amdgpu_job, ring: *mut amdgpu_ring,
        fence: *mut *mut dma_fence) -> i32;
    pub fn amdgpu_job_stop_all_jobs_on_sched(sched: *mut drm_gpu_scheduler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
