/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

pub const regUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET: u32 = 0x1bfff;
pub const regUVD_JPEG_GPCOM_CMD_INTERNAL_OFFSET: u32 = 0x404d;
pub const regUVD_JPEG_GPCOM_DATA0_INTERNAL_OFFSET: u32 = 0x404e;
pub const regUVD_JPEG_GPCOM_DATA1_INTERNAL_OFFSET: u32 = 0x404f;
pub const regUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x40ab;
pub const regUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x40ac;
pub const regUVD_LMI_JRBC_IB_VMID_INTERNAL_OFFSET: u32 = 0x40a4;
pub const regUVD_LMI_JPEG_VMID_INTERNAL_OFFSET: u32 = 0x40a6;
pub const regUVD_LMI_JRBC_IB_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x40b6;
pub const regUVD_LMI_JRBC_IB_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x40b7;
pub const regUVD_JRBC_IB_SIZE_INTERNAL_OFFSET: u32 = 0x4082;
pub const regUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x42d4;
pub const regUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x42d5;
pub const regUVD_JRBC_RB_COND_RD_TIMER_INTERNAL_OFFSET: u32 = 0x4085;
pub const regUVD_JRBC_RB_REF_DATA_INTERNAL_OFFSET: u32 = 0x4084;
pub const regUVD_JRBC_STATUS_INTERNAL_OFFSET: u32 = 0x4089;
pub const regUVD_JPEG_PITCH_INTERNAL_OFFSET: u32 = 0x4043;
pub const regUVD_JRBC0_UVD_JRBC_SCRATCH0_INTERNAL_OFFSET: u32 = 0x4094;
pub const regUVD_JRBC_EXTERNAL_MCM_ADDR_INTERNAL_OFFSET: u32 = 0x1bffe;

pub const JRBC_DEC_EXTERNAL_REG_WRITE_ADDR: u32 = 0x18000;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amdgpu_jpeg_v4_0_3_sub_block {
    AMDGPU_JPEG_V4_0_3_JPEG0 = 0,
    AMDGPU_JPEG_V4_0_3_JPEG1,
    AMDGPU_JPEG_V4_0_3_MAX_SUB_BLOCK,
}

extern "C" {
    pub static jpeg_v4_0_3_ip_block: crate::amdgpu_ip_block_version;

    pub fn jpeg_v4_0_3_dec_ring_emit_ib(
        ring: *mut crate::amdgpu_ring,
        job: *mut crate::amdgpu_job,
        ib: *mut crate::amdgpu_ib,
        flags: u32,
    );
    pub fn jpeg_v4_0_3_dec_ring_emit_fence(
        ring: *mut crate::amdgpu_ring,
        addr: u64,
        seq: u64,
        flags: core::ffi::c_uint,
    );
    pub fn jpeg_v4_0_3_dec_ring_emit_vm_flush(
        ring: *mut crate::amdgpu_ring,
        vmid: core::ffi::c_uint,
        pd_addr: u64,
    );
    pub fn jpeg_v4_0_3_ring_emit_hdp_flush(ring: *mut crate::amdgpu_ring);
    pub fn jpeg_v4_0_3_dec_ring_nop(ring: *mut crate::amdgpu_ring, count: u32);
    pub fn jpeg_v4_0_3_dec_ring_insert_start(ring: *mut crate::amdgpu_ring);
    pub fn jpeg_v4_0_3_dec_ring_insert_end(ring: *mut crate::amdgpu_ring);
    pub fn jpeg_v4_0_3_dec_ring_emit_wreg(
        ring: *mut crate::amdgpu_ring,
        reg: u32,
        val: u32,
    );
    pub fn jpeg_v4_0_3_dec_ring_emit_reg_wait(
        ring: *mut crate::amdgpu_ring,
        reg: u32,
        val: u32,
        mask: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
