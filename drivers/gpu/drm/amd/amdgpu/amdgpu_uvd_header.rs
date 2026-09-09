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

pub const AMDGPU_DEFAULT_UVD_HANDLES: usize = 10;
pub const AMDGPU_MAX_UVD_HANDLES: usize = 40;
pub const AMDGPU_UVD_STACK_SIZE: usize = 200 * 1024;
pub const AMDGPU_UVD_HEAP_SIZE: usize = 256 * 1024;
pub const AMDGPU_UVD_SESSION_SIZE: usize = 50 * 1024;
pub const AMDGPU_UVD_FIRMWARE_OFFSET: usize = 256;

pub const AMDGPU_MAX_UVD_INSTANCES: usize = 2;

// Build-time dependencies and the original macro's field access are preserved here.
#[macro_export]
macro_rules! AMDGPU_UVD_FIRMWARE_SIZE {
    ($adev:expr) => {
        (AMDGPU_GPU_PAGE_ALIGN(
            le32_to_cpu((((*$adev).uvd.fw).data as *const common_firmware_header)
                .as_ref().unwrap().ucode_size_bytes) + 8
        ) - AMDGPU_UVD_FIRMWARE_OFFSET)
    };
}

#[repr(C)]
pub struct amdgpu_uvd_inst {
    pub vcpu_bo: *mut amdgpu_bo,
    pub cpu_addr: *mut core::ffi::c_void,
    pub gpu_addr: u64,
    pub saved_bo: *mut core::ffi::c_void,
    pub ring: amdgpu_ring,
    pub ring_enc: [amdgpu_ring; AMDGPU_MAX_UVD_ENC_RINGS],
    pub irq: amdgpu_irq_src,
    pub srbm_soft_reset: u32,
}

pub const AMDGPU_UVD_HARVEST_UVD0: u32 = 1 << 0;
pub const AMDGPU_UVD_HARVEST_UVD1: u32 = 1 << 1;

#[repr(C)]
pub struct amdgpu_uvd {
    pub fw: *const firmware, /* UVD firmware */
    pub fw_version: u32,
    pub max_handles: u32,
    pub num_enc_rings: u32,
    pub num_uvd_inst: u8,
    pub address_64_bit: bool,
    pub use_ctx_buf: bool,
    pub inst: [amdgpu_uvd_inst; AMDGPU_MAX_UVD_INSTANCES],
    pub filp: [*mut drm_file; AMDGPU_MAX_UVD_HANDLES],
    pub handles: [atomic_t; AMDGPU_MAX_UVD_HANDLES],
    pub entity: drm_sched_entity,
    pub idle_work: delayed_work,
    pub harvest_config: u32,
    /* store image width to adjust nb memory state */
    pub decode_image_width: u32,
    pub keyselect: u32,
    pub ib_bo: *mut amdgpu_bo,
}

extern "C" {
    pub fn amdgpu_uvd_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_uvd_sw_fini(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_uvd_entity_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> i32;
    pub fn amdgpu_uvd_prepare_suspend(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_uvd_suspend(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_uvd_resume(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_uvd_get_create_msg(ring: *mut amdgpu_ring, handle: u32, fence: *mut *mut dma_fence) -> i32;
    pub fn amdgpu_uvd_get_destroy_msg(ring: *mut amdgpu_ring, handle: u32, direct: bool, fence: *mut *mut dma_fence) -> i32;
    pub fn amdgpu_uvd_free_handles(adev: *mut amdgpu_device, filp: *mut drm_file);
    pub fn amdgpu_uvd_ring_parse_cs(parser: *mut amdgpu_cs_parser, job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> i32;
    pub fn amdgpu_uvd_ring_begin_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_uvd_ring_end_use(ring: *mut amdgpu_ring);
    pub fn amdgpu_uvd_ring_test_ib(ring: *mut amdgpu_ring, timeout: libc::c_long) -> i32;
    pub fn amdgpu_uvd_used_handles(adev: *mut amdgpu_device) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
