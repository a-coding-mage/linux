/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: v10_structs.h.

pub const AMDGPU_MES_CTX_RPTR_OFFS: u32 = 0;
pub const AMDGPU_MES_CTX_WPTR_OFFS: u32 = 1;
pub const AMDGPU_MES_CTX_FENCE_OFFS: u32 = 2;
pub const AMDGPU_MES_CTX_COND_EXE_OFFS: u32 = 3;
pub const AMDGPU_MES_CTX_TRAIL_FENCE_OFFS: u32 = 4;
pub const AMDGPU_MES_CTX_MAX_OFFS: u32 = 5;

pub const AMDGPU_MES_CTX_RING_OFFS: u32 = AMDGPU_MES_CTX_MAX_OFFS;
pub const AMDGPU_MES_CTX_IB_OFFS: u32 = AMDGPU_MES_CTX_RING_OFFS + 1;
pub const AMDGPU_MES_CTX_PADDING_OFFS: u32 = AMDGPU_MES_CTX_IB_OFFS + 1;

pub const AMDGPU_MES_CTX_MAX_GFX_RINGS: usize = 1;
pub const AMDGPU_MES_CTX_MAX_COMPUTE_RINGS: usize = 4;
pub const AMDGPU_MES_CTX_MAX_SDMA_RINGS: usize = 2;
pub const AMDGPU_MES_CTX_MAX_RINGS: usize =
    AMDGPU_MES_CTX_MAX_GFX_RINGS + AMDGPU_MES_CTX_MAX_COMPUTE_RINGS + AMDGPU_MES_CTX_MAX_SDMA_RINGS;

pub const AMDGPU_CSA_SDMA_SIZE: usize = 64;
pub const GFX10_MEC_HPD_SIZE: usize = 2048;

#[repr(C)]
pub struct amdgpu_wb_slot {
    pub data: [u32; 8],
}

#[repr(C, align(256))]
pub struct amdgpu_mes_ctx_aligned_ib {
    pub data: [u32; 256],
}

#[repr(C)]
pub struct amdgpu_mes_ctx_meta_data_gfx {
    pub ring: [u8; PAGE_SIZE * 4],
    // gfx csa
    pub gfx_meta_data: v10_gfx_meta_data,
    pub gds_backup: [u8; 64 * 1024],
    pub slots: [amdgpu_wb_slot; AMDGPU_MES_CTX_MAX_OFFS as usize],
    // only for ib test
    pub ib: amdgpu_mes_ctx_aligned_ib,
    pub padding: [u32; 64],
}

#[repr(C)]
pub struct amdgpu_mes_ctx_meta_data_compute {
    pub ring: [u8; PAGE_SIZE * 4],
    pub mec_hpd: [u8; GFX10_MEC_HPD_SIZE],
    pub slots: [amdgpu_wb_slot; AMDGPU_MES_CTX_MAX_OFFS as usize],
    // only for ib test
    pub ib: amdgpu_mes_ctx_aligned_ib,
    pub padding: [u32; 64],
}

#[repr(C)]
pub struct amdgpu_mes_ctx_meta_data_sdma {
    pub ring: [u8; PAGE_SIZE * 4],
    // sdma csa for mcbp
    pub sdma_meta_data: [u8; AMDGPU_CSA_SDMA_SIZE],
    pub slots: [amdgpu_wb_slot; AMDGPU_MES_CTX_MAX_OFFS as usize],
    // only for ib test
    pub ib: amdgpu_mes_ctx_aligned_ib,
    pub padding: [u32; 64],
}

#[repr(C)]
pub struct amdgpu_mes_ctx_meta_data {
    // C: each nested array is __aligned(PAGE_SIZE).
    pub gfx: [amdgpu_mes_ctx_meta_data_gfx; AMDGPU_MES_CTX_MAX_GFX_RINGS],
    pub compute: [amdgpu_mes_ctx_meta_data_compute; AMDGPU_MES_CTX_MAX_COMPUTE_RINGS],
    pub sdma: [amdgpu_mes_ctx_meta_data_sdma; AMDGPU_MES_CTX_MAX_SDMA_RINGS],
}

#[repr(C)]
pub struct amdgpu_mes_ctx_data {
    pub meta_data_obj: *mut amdgpu_bo,
    pub meta_data_gpu_addr: u64,
    pub meta_data_mc_addr: u64,
    pub meta_data_va: *mut amdgpu_bo_va,
    pub meta_data_ptr: *mut core::ffi::c_void,
    pub gang_ids: [u32; AMDGPU_HW_IP_DMA as usize + 1],
}

pub const AMDGPU_FENCE_MES_QUEUE_FLAG: u32 = 0x1000000u32;
pub const AMDGPU_FENCE_MES_QUEUE_ID_MASK: u32 = AMDGPU_FENCE_MES_QUEUE_FLAG - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
