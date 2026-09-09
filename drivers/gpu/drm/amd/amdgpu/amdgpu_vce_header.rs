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

pub const AMDGPU_MAX_VCE_HANDLES: usize = 16;
pub const AMDGPU_VCE_FIRMWARE_OFFSET: u32 = 256;

pub const AMDGPU_VCE_HARVEST_VCE0: u32 = 1 << 0;
pub const AMDGPU_VCE_HARVEST_VCE1: u32 = 1 << 1;

pub const AMDGPU_VCE_FW_53_45: u32 = (53 << 24) | (45 << 16);

#[repr(C)]
pub struct amdgpu_vce {
    pub vcpu_bo: *mut amdgpu_bo,
    pub gpu_addr: u64,
    pub cpu_addr: *mut core::ffi::c_void,
    pub saved_bo: *mut core::ffi::c_void,
    pub fw_version: core::ffi::c_uint,
    pub fb_version: core::ffi::c_uint,
    pub handles: [atomic_t; AMDGPU_MAX_VCE_HANDLES],
    pub filp: [*mut drm_file; AMDGPU_MAX_VCE_HANDLES],
    pub img_size: [u32; AMDGPU_MAX_VCE_HANDLES],
    pub idle_work: delayed_work,
    pub idle_mutex: mutex,
    pub fw: *const firmware,
    pub ring: [amdgpu_ring; AMDGPU_MAX_VCE_RINGS],
    pub irq: amdgpu_irq_src,
    pub harvest_config: core::ffi::c_uint,
    pub entity: drm_sched_entity,
    pub srbm_soft_reset: u32,
    pub num_rings: core::ffi::c_uint,
    pub keyselect: u32,
    pub gart_node: drm_mm_node,
}

extern "C" {
    pub fn amdgpu_vce_early_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_vce_sw_init(adev: *mut amdgpu_device, size: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn amdgpu_vce_sw_fini(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_vce_entity_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> core::ffi::c_int;
    pub fn amdgpu_vce_suspend(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_vce_resume(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_vce_free_handles(adev: *mut amdgpu_device, filp: *mut drm_file);
    pub fn amdgpu_vce_ring_parse_cs(p: *mut amdgpu_cs_parser, job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> core::ffi::c_int;
    pub fn amdgpu_vce_ring_parse_cs_vm(p: *mut amdgpu_cs_parser, job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> core::ffi::c_int;
    pub fn amdgpu_vce_ring_emit_ib(ring: *mut amdgpu_ring, job: *mut amdgpu_job, ib: *mut amdgpu_ib, flags: u32);
    pub fn amdgpu_vce_ring_emit_fence(ring: *mut amdgpu_ring, addr: u64, seq: u64, flags: core::ffi::c_uint);
    pub fn amdgpu_vce_ring_test_ring(ring: *mut amdgpu_ring) -> core::ffi::c_int;
    pub fn amdgpu_vce_ring_test_ib(ring: *mut amdgpu_ring, timeout: core::ffi::c_long) -> core::ffi::c_int;
    pub fn amdgpu_vce_ring_begin_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_vce_ring_end_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_vce_ring_get_emit_ib_size(ring: *mut amdgpu_ring) -> core::ffi::c_uint;
    pub fn amdgpu_vce_ring_get_dma_frame_size(ring: *mut amdgpu_ring) -> core::ffi::c_uint;
    pub fn amdgpu_vce_get_ring_prio(ring: core::ffi::c_int) -> amdgpu_ring_priority_level;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
