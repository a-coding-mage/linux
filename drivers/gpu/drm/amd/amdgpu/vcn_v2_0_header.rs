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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

// Original header guard: __VCN_V2_0_H__

unsafe extern "C" {
    pub fn vcn_v2_0_dec_ring_insert_start(ring: *mut amdgpu_ring);
    pub fn vcn_v2_0_dec_ring_insert_end(ring: *mut amdgpu_ring);
    pub fn vcn_v2_0_dec_ring_insert_nop(ring: *mut amdgpu_ring, count: u32);
    pub fn vcn_v2_0_dec_ring_emit_fence(
        ring: *mut amdgpu_ring,
        addr: u64,
        seq: u64,
        flags: c_uint,
    );
    pub fn vcn_v2_0_dec_ring_emit_ib(
        ring: *mut amdgpu_ring,
        job: *mut amdgpu_job,
        ib: *mut amdgpu_ib,
        flags: u32,
    );
    pub fn vcn_v2_0_dec_ring_emit_reg_wait(
        ring: *mut amdgpu_ring,
        reg: u32,
        val: u32,
        mask: u32,
    );
    pub fn vcn_v2_0_dec_ring_emit_vm_flush(
        ring: *mut amdgpu_ring,
        vmid: c_uint,
        pd_addr: u64,
    );
    pub fn vcn_v2_0_dec_ring_emit_wreg(ring: *mut amdgpu_ring, reg: u32, val: u32);
    pub fn vcn_v2_0_dec_ring_test_ring(ring: *mut amdgpu_ring) -> i32;

    pub fn vcn_v2_0_enc_ring_insert_end(ring: *mut amdgpu_ring);
    pub fn vcn_v2_0_enc_ring_emit_fence(
        ring: *mut amdgpu_ring,
        addr: u64,
        seq: u64,
        flags: c_uint,
    );
    pub fn vcn_v2_0_enc_ring_emit_ib(
        ring: *mut amdgpu_ring,
        job: *mut amdgpu_job,
        ib: *mut amdgpu_ib,
        flags: u32,
    );
    pub fn vcn_v2_0_enc_ring_emit_reg_wait(
        ring: *mut amdgpu_ring,
        reg: u32,
        val: u32,
        mask: u32,
    );
    pub fn vcn_v2_0_enc_ring_emit_vm_flush(
        ring: *mut amdgpu_ring,
        vmid: c_uint,
        pd_addr: u64,
    );
    pub fn vcn_v2_0_enc_ring_emit_wreg(ring: *mut amdgpu_ring, reg: u32, val: u32);

    pub static vcn_v2_0_ip_block: amdgpu_ip_block_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
