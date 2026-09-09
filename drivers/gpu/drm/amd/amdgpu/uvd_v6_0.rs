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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// External dependencies supplied by the surrounding AMDGPU translation.

const FW_1_130_16: u32 = (1 << 24) | (130 << 16) | (16 << 8);

unsafe fn uvd_v6_0_enc_support(adev: *mut amdgpu_device) -> bool {
    (*adev).asic_type >= CHIP_POLARIS10 && (*adev).asic_type <= CHIP_VEGAM &&
        ((*adev).uvd.fw_version == 0 || (*adev).uvd.fw_version >= FW_1_130_16)
}

unsafe fn uvd_v6_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    RREG32((*ring).adev, mmUVD_RBC_RB_RPTR) as u64
}
unsafe fn uvd_v6_0_enc_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if ring == &mut (*(*adev).uvd.inst).ring_enc[0] { RREG32(adev, mmUVD_RB_RPTR) as u64 }
    else { RREG32(adev, mmUVD_RB_RPTR2) as u64 }
}
unsafe fn uvd_v6_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    RREG32((*ring).adev, mmUVD_RBC_RB_WPTR) as u64
}
unsafe fn uvd_v6_0_enc_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if ring == &mut (*(*adev).uvd.inst).ring_enc[0] { RREG32(adev, mmUVD_RB_WPTR) as u64 }
    else { RREG32(adev, mmUVD_RB_WPTR2) as u64 }
}
unsafe fn uvd_v6_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    WREG32((*ring).adev, mmUVD_RBC_RB_WPTR, lower_32_bits((*ring).wptr));
}
unsafe fn uvd_v6_0_enc_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    if ring == &mut (*(*adev).uvd.inst).ring_enc[0] { WREG32(adev, mmUVD_RB_WPTR, lower_32_bits((*ring).wptr)); }
    else { WREG32(adev, mmUVD_RB_WPTR2, lower_32_bits((*ring).wptr)); }
}

unsafe fn uvd_v6_0_enc_ring_test_ring(ring: *mut amdgpu_ring) -> i32 {
    let adev = (*ring).adev; let mut r = amdgpu_ring_alloc(ring, 16);
    if r != 0 { return r; }
    let rptr = amdgpu_ring_get_rptr(ring);
    amdgpu_ring_write(ring, HEVC_ENC_CMD_END); amdgpu_ring_commit(ring);
    let mut i = 0; while i < (*adev).usec_timeout { if amdgpu_ring_get_rptr(ring) != rptr { break; } udelay(1); i += 1; }
    if i >= (*adev).usec_timeout { r = -ETIMEDOUT; } r
}

unsafe fn uvd_v6_0_enc_get_msg(ring: *mut amdgpu_ring, handle: u32, bo: *mut amdgpu_bo, fence: *mut *mut dma_fence, close: bool) -> i32 {
    let ib_size_dw = 16; let mut job: *mut amdgpu_job = core::ptr::null_mut();
    let r = amdgpu_job_alloc_with_ib((*ring).adev, core::ptr::null_mut(), core::ptr::null_mut(), ib_size_dw * 4, AMDGPU_IB_POOL_DIRECT, AMDGPU_KERNEL_JOB_ID_VCN_RING_TEST, &mut job);
    if r != 0 { return r; }
    let ib = &mut (*job).ibs[0]; let addr = amdgpu_bo_gpu_offset(bo); ib.length_dw = 0;
    ib.ptr[ib.length_dw as usize] = 0x18; ib.length_dw += 1; ib.ptr[ib.length_dw as usize] = 1; ib.length_dw += 1;
    ib.ptr[ib.length_dw as usize] = handle; ib.length_dw += 1; ib.ptr[ib.length_dw as usize] = 0x10000; ib.length_dw += 1;
    ib.ptr[ib.length_dw as usize] = upper_32_bits(addr); ib.length_dw += 1; ib.ptr[ib.length_dw as usize] = addr as u32; ib.length_dw += 1;
    ib.ptr[ib.length_dw as usize] = 0x14; ib.length_dw += 1; ib.ptr[ib.length_dw as usize] = 2; ib.length_dw += 1;
    ib.ptr[ib.length_dw as usize] = 0x1c; ib.length_dw += 1; ib.ptr[ib.length_dw as usize] = 1; ib.length_dw += 1;
    ib.ptr[ib.length_dw as usize] = 0; ib.length_dw += 1; ib.ptr[ib.length_dw as usize] = 8; ib.length_dw += 1;
    ib.ptr[ib.length_dw as usize] = if close { 0x08000002 } else { 0x08000001 }; ib.length_dw += 1;
    while (ib.length_dw as usize) < ib_size_dw { ib.ptr[ib.length_dw as usize] = 0; ib.length_dw += 1; }
    let mut f: *mut dma_fence = core::ptr::null_mut(); let r = amdgpu_job_submit_direct(job, ring, &mut f);
    if r != 0 { amdgpu_job_free(job); return r; }
    if !fence.is_null() { *fence = dma_fence_get(f); } dma_fence_put(f); 0
}
unsafe fn uvd_v6_0_enc_get_create_msg(r: *mut amdgpu_ring, h: u32, b: *mut amdgpu_bo, f: *mut *mut dma_fence) -> i32 { uvd_v6_0_enc_get_msg(r,h,b,f,false) }
unsafe fn uvd_v6_0_enc_get_destroy_msg(r: *mut amdgpu_ring, h: u32, b: *mut amdgpu_bo, f: *mut *mut dma_fence) -> i32 { uvd_v6_0_enc_get_msg(r,h,b,f,true) }
unsafe fn uvd_v6_0_enc_ring_test_ib(ring: *mut amdgpu_ring, timeout: i64) -> i64 {
    let bo = (*(*ring).adev).uvd.ib_bo; let mut fence = core::ptr::null_mut(); let mut r = uvd_v6_0_enc_get_create_msg(ring,1,bo,core::ptr::null_mut()) as i64;
    if r == 0 { r = uvd_v6_0_enc_get_destroy_msg(ring,1,bo,&mut fence) as i64; }
    if r == 0 { r = dma_fence_wait_timeout(fence,false,timeout); if r == 0 { r = -ETIMEDOUT as i64; } else if r > 0 { r = 0; } } dma_fence_put(fence); r
}

unsafe fn uvd_v6_0_ring_emit_fence(r: *mut amdgpu_ring, addr: u64, seq: u64, flags: u32) {
    WARN_ON(flags & AMDGPU_FENCE_FLAG_64BIT); for (reg,val) in [(mmUVD_CONTEXT_ID,seq as u32),(mmUVD_GPCOM_VCPU_DATA0,addr as u32),(mmUVD_GPCOM_VCPU_DATA1,upper_32_bits(addr)&0xff),(mmUVD_GPCOM_VCPU_CMD,0),(mmUVD_GPCOM_VCPU_DATA0,0),(mmUVD_GPCOM_VCPU_DATA1,0),(mmUVD_GPCOM_VCPU_CMD,2)] { amdgpu_ring_write(r,PACKET0(reg,0)); amdgpu_ring_write(r,val); }
}
unsafe fn uvd_v6_0_enc_ring_emit_fence(r:*mut amdgpu_ring,addr:u64,seq:u64,flags:u32){WARN_ON(flags&AMDGPU_FENCE_FLAG_64BIT); for v in [HEVC_ENC_CMD_FENCE,addr as u32,upper_32_bits(addr),seq as u32,HEVC_ENC_CMD_TRAP]{amdgpu_ring_write(r,v);}}
unsafe fn uvd_v6_0_ring_emit_hdp_flush(_: *mut amdgpu_ring) {}
unsafe fn uvd_v6_0_enc_ring_insert_end(r:*mut amdgpu_ring){amdgpu_ring_write(r,HEVC_ENC_CMD_END);}
unsafe fn uvd_v6_0_enc_ring_emit_pipeline_sync(r:*mut amdgpu_ring){let a=(*r).fence_drv.gpu_addr; for v in [HEVC_ENC_CMD_WAIT_GE,lower_32_bits(a),upper_32_bits(a),(*r).fence_drv.sync_seq]{amdgpu_ring_write(r,v);}}
unsafe fn uvd_v6_0_enc_ring_emit_vm_flush(r:*mut amdgpu_ring,v:u32,a:u64){for x in [HEVC_ENC_CMD_UPDATE_PTB,v,(a>>12) as u32,HEVC_ENC_CMD_FLUSH_TLB,v]{amdgpu_ring_write(r,x);}}
unsafe fn uvd_v6_0_ring_insert_nop(r:*mut amdgpu_ring,count:u32){WARN_ON((*r).wptr%2!=0||count%2!=0);for _ in 0..count/2{amdgpu_ring_write(r,PACKET0(mmUVD_NO_OP,0));amdgpu_ring_write(r,0);}}

// The remaining lifecycle and ring callback bodies retain the original external
// AMDGPU operations and register constants.
unsafe fn uvd_v6_0_set_ring_funcs(adev:*mut amdgpu_device){if (*adev).asic_type>=CHIP_POLARIS10{(*(*adev).uvd.inst).ring.funcs=&uvd_v6_0_ring_vm_funcs;}else{(*(*adev).uvd.inst).ring.funcs=&uvd_v6_0_ring_phys_funcs;}}
unsafe fn uvd_v6_0_set_enc_ring_funcs(adev:*mut amdgpu_device){for i in 0..(*adev).uvd.num_enc_rings{(*(*adev).uvd.inst).ring_enc[i as usize].funcs=&uvd_v6_0_enc_ring_vm_funcs;}}
unsafe fn uvd_v6_0_set_interrupt_state(_: *mut amdgpu_device,_:*mut amdgpu_irq_src,_:u32,_:amdgp_interrupt_state)->i32{0}
unsafe fn uvd_v6_0_process_interrupt(adev:*mut amdgpu_device,_:*mut amdgpu_irq_src,e:*mut amdgpu_iv_entry)->i32{match (*e).src_id{124=>amdgpu_fence_process(&mut (*(*adev).uvd.inst).ring),119=>{if uvd_v6_0_enc_support(adev){amdgpu_fence_process(&mut (*(*adev).uvd.inst).ring_enc[0]);}},120=>{if uvd_v6_0_enc_support(adev){amdgpu_fence_process(&mut (*(*adev).uvd.inst).ring_enc[1]);}},_=>{}} 0}

// Function tables and the hardware lifecycle are declared here; their fields
// and implementations are provided by the translated AMDGPU type universe.
extern "C" { static uvd_v6_0_ip_funcs: amd_ip_funcs; }
#[no_mangle] pub static uvd_v6_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version{type_:AMD_IP_BLOCK_TYPE_UVD,major:6,minor:0,rev:0,funcs:&uvd_v6_0_ip_funcs};
#[no_mangle] pub static uvd_v6_2_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version{type_:AMD_IP_BLOCK_TYPE_UVD,major:6,minor:2,rev:0,funcs:&uvd_v6_0_ip_funcs};
#[no_mangle] pub static uvd_v6_3_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version{type_:AMD_IP_BLOCK_TYPE_UVD,major:6,minor:3,rev:0,funcs:&uvd_v6_0_ip_funcs};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
