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

// Dependencies supplied by amdgpu.h and vcn_sw_ring.h remain external.

pub unsafe fn vcn_dec_sw_ring_emit_fence(
    ring: *mut amdgpu_ring,
    addr: u64,
    seq: u64,
    flags: u32,
) {
    WARN_ON(flags & AMDGPU_FENCE_FLAG_64BIT);

    amdgpu_ring_write(ring, VCN_DEC_SW_CMD_FENCE);
    amdgpu_ring_write(ring, addr);
    amdgpu_ring_write(ring, upper_32_bits(addr));
    amdgpu_ring_write(ring, seq);
    amdgpu_ring_write(ring, VCN_DEC_SW_CMD_TRAP);
}

pub unsafe fn vcn_dec_sw_ring_insert_end(ring: *mut amdgpu_ring) {
    amdgpu_ring_write(ring, VCN_DEC_SW_CMD_END);
}

pub unsafe fn vcn_dec_sw_ring_emit_ib(
    ring: *mut amdgpu_ring,
    job: *mut amdgpu_job,
    ib: *mut amdgpu_ib,
    _flags: u32,
) {
    let vmid: u32 = AMDGPU_JOB_GET_VMID(job);

    amdgpu_ring_write(ring, VCN_DEC_SW_CMD_IB);
    amdgpu_ring_write(ring, vmid);
    amdgpu_ring_write(ring, lower_32_bits((*ib).gpu_addr));
    amdgpu_ring_write(ring, upper_32_bits((*ib).gpu_addr));
    amdgpu_ring_write(ring, (*ib).length_dw);
}

pub unsafe fn vcn_dec_sw_ring_emit_reg_wait(
    ring: *mut amdgpu_ring,
    reg: u32,
    val: u32,
    mask: u32,
) {
    amdgpu_ring_write(ring, VCN_DEC_SW_CMD_REG_WAIT);
    amdgpu_ring_write(ring, reg << 2);
    amdgpu_ring_write(ring, mask);
    amdgpu_ring_write(ring, val);
}

pub unsafe fn vcn_dec_sw_ring_emit_vm_flush(
    ring: *mut amdgpu_ring,
    vmid: u32,
    mut pd_addr: u64,
) {
    let hub: *mut amdgpu_vmhub = &mut (*(*ring).adev).vmhub[(*ring).vm_hub];
    let data0: u32;
    let data1: u32;
    let mask: u32;

    pd_addr = amdgpu_gmc_emit_flush_gpu_tlb(ring, vmid, pd_addr);

    /* wait for register write */
    data0 = (*hub).ctx0_ptb_addr_lo32 + vmid * (*hub).ctx_addr_distance;
    data1 = lower_32_bits(pd_addr);
    mask = 0xffffffff;
    vcn_dec_sw_ring_emit_reg_wait(ring, data0, data1, mask);
}

pub unsafe fn vcn_dec_sw_ring_emit_wreg(
    ring: *mut amdgpu_ring,
    reg: u32,
    val: u32,
) {
    amdgpu_ring_write(ring, VCN_DEC_SW_CMD_REG_WRITE);
    amdgpu_ring_write(ring, reg << 2);
    amdgpu_ring_write(ring, val);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
