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

pub const mmUVD_JRBC_EXTERNAL_REG_INTERNAL_OFFSET: u32 = 0x1bfff;
pub const mmUVD_JPEG_GPCOM_CMD_INTERNAL_OFFSET: u32 = 0x4029;
pub const mmUVD_JPEG_GPCOM_DATA0_INTERNAL_OFFSET: u32 = 0x402a;
pub const mmUVD_JPEG_GPCOM_DATA1_INTERNAL_OFFSET: u32 = 0x402b;
pub const mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x40ea;
pub const mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x40eb;
pub const mmUVD_LMI_JRBC_IB_VMID_INTERNAL_OFFSET: u32 = 0x40cf;
pub const mmUVD_LMI_JPEG_VMID_INTERNAL_OFFSET: u32 = 0x40d1;
pub const mmUVD_LMI_JRBC_IB_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x40e8;
pub const mmUVD_LMI_JRBC_IB_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x40e9;
pub const mmUVD_JRBC_IB_SIZE_INTERNAL_OFFSET: u32 = 0x4082;
pub const mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_LOW_INTERNAL_OFFSET: u32 = 0x40ec;
pub const mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_HIGH_INTERNAL_OFFSET: u32 = 0x40ed;
pub const mmUVD_JRBC_RB_COND_RD_TIMER_INTERNAL_OFFSET: u32 = 0x4085;
pub const mmUVD_JRBC_RB_REF_DATA_INTERNAL_OFFSET: u32 = 0x4084;
pub const mmUVD_JRBC_STATUS_INTERNAL_OFFSET: u32 = 0x4089;
pub const mmUVD_JPEG_PITCH_INTERNAL_OFFSET: u32 = 0x401f;
pub const mmUVD_JPEG_IH_CTRL_INTERNAL_OFFSET: u32 = 0x4149;

pub const JRBC_DEC_EXTERNAL_REG_WRITE_ADDR: u32 = 0x18000;

extern "C" {
    pub fn jpeg_v2_0_dec_ring_insert_start(ring: *mut amdgpu_ring);
    pub fn jpeg_v2_0_dec_ring_insert_end(ring: *mut amdgpu_ring);
    pub fn jpeg_v2_0_dec_ring_emit_fence(
        ring: *mut amdgpu_ring,
        addr: u64,
        seq: u64,
        flags: u32,
    );
    pub fn jpeg_v2_0_dec_ring_emit_ib(
        ring: *mut amdgpu_ring,
        job: *mut amdgpu_job,
        ib: *mut amdgpu_ib,
        flags: u32,
    );
    pub fn jpeg_v2_0_dec_ring_emit_reg_wait(
        ring: *mut amdgpu_ring,
        reg: u32,
        val: u32,
        mask: u32,
    );
    pub fn jpeg_v2_0_dec_ring_emit_vm_flush(
        ring: *mut amdgpu_ring,
        vmid: u32,
        pd_addr: u64,
    );
    pub fn jpeg_v2_0_dec_ring_emit_wreg(ring: *mut amdgpu_ring, reg: u32, val: u32);
    pub fn jpeg_v2_0_dec_ring_nop(ring: *mut amdgpu_ring, count: u32);

    pub fn jpeg_v2_0_process_interrupt(
        adev: *mut amdgpu_device,
        source: *mut amdgpu_irq_src,
        entry: *mut amdgpu_iv_entry,
    ) -> i32;

    pub static jpeg_v2_0_ip_block: amdgpu_ip_block_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
