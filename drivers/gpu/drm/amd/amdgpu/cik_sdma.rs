/*
 * Copyright 2013 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Alex Deucher
 */

// External kernel, AMDGPU, register, and generated-header dependencies are
// supplied by the surrounding translation unit.

static SDMA_OFFSETS: [u32; SDMA_MAX_INSTANCE as usize] = [
    SDMA0_REGISTER_OFFSET, SDMA1_REGISTER_OFFSET,
];

unsafe fn cik_sdma_free_microcode(adev: *mut amdgpu_device) {
    let mut i = 0;
    while i < (*(*adev).sdma).num_instances {
        amdgpu_ucode_release(&mut (*(*adev).sdma).instance[i].fw);
        i += 1;
    }
}

/*
 * sDMA - System DMA
 * Starting with CIK, the GPU has new asynchronous DMA engines. These engines
 * are used for compute and gfx. There are two DMA engines (SDMA0, SDMA1) and
 * each one supports 1 ring buffer used for gfx and 2 queues used for compute.
 *
 * The programming model is very similar to the CP (ring buffer, IBs, etc.),
 * but sDMA has its own packet format that is different from the PM4 format
 * used by the CP. sDMA supports copying data, writing embedded data, solid
 * fills, and a number of other things. It also has support for tiling/detiling
 * of buffers.
 */

unsafe fn cik_sdma_init_microcode(adev: *mut amdgpu_device) -> i32 {
    let chip_name: *const i8;
    let mut err = 0;
    let mut i = 0;
    DRM_DEBUG!("\n");
    chip_name = match (*adev).asic_type {
        CHIP_BONAIRE => c"bonaire".as_ptr(),
        CHIP_HAWAII => c"hawaii".as_ptr(),
        CHIP_KAVERI => c"kaveri".as_ptr(),
        CHIP_KABINI => c"kabini".as_ptr(),
        CHIP_MULLINS => c"mullins".as_ptr(),
        _ => return -EINVAL,
    };
    while i < (*(*adev).sdma).num_instances {
        let suffix = if i == 0 { c"amdgpu/%s_sdma.bin" } else { c"amdgpu/%s_sdma1.bin" };
        err = amdgpu_ucode_request(adev, &mut (*(*adev).sdma).instance[i].fw,
            AMDGPU_UCODE_REQUIRED, suffix.as_ptr(), chip_name);
        if err != 0 { break; }
        i += 1;
    }
    if err != 0 {
        pr_err!("cik_sdma: Failed to load firmware\n");
        i = 0;
        while i < (*(*adev).sdma).num_instances {
            amdgpu_ucode_release(&mut (*(*adev).sdma).instance[i].fw);
            i += 1;
        }
    }
    err
}

unsafe fn cik_sdma_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let rptr = *(*ring).rptr_cpu_addr;
    ((rptr & 0x3fffc) >> 2) as u64
}

unsafe fn cik_sdma_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    ((RREG32!(adev, mmSDMA0_GFX_RB_WPTR + SDMA_OFFSETS[(*ring).me as usize]) & 0x3fffc) >> 2) as u64
}

unsafe fn cik_sdma_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    WREG32!(adev, mmSDMA0_GFX_RB_WPTR + SDMA_OFFSETS[(*ring).me as usize],
        (((*ring).wptr << 2) & 0x3fffc) as u32);
}

unsafe fn cik_sdma_ring_insert_nop(ring: *mut amdgpu_ring, count: u32) {
    let sdma = amdgpu_sdma_get_instance_from_ring(ring);
    let mut i = 0;
    while i < count {
        if !sdma.is_null() && (*sdma).burst_nop && i == 0 {
            amdgpu_ring_write!(ring, (*(*ring).funcs).nop | SDMA_NOP_COUNT!(count - 1));
        } else { amdgpu_ring_write!(ring, (*(*ring).funcs).nop); }
        i += 1;
    }
}

unsafe fn cik_sdma_ring_emit_ib(ring: *mut amdgpu_ring, job: *mut amdgpu_job,
                                ib: *mut amdgpu_ib, _flags: u32) {
    let extra_bits = AMDGPU_JOB_GET_VMID!(job) & 0xf;
    cik_sdma_ring_insert_nop(ring, (4 - ((*ring).wptr as u32)) & 7);
    amdgpu_ring_write!(ring, SDMA_PACKET!(SDMA_OPCODE_INDIRECT_BUFFER, 0, extra_bits));
    amdgpu_ring_write!(ring, (*ib).gpu_addr & 0xffffffe0);
    amdgpu_ring_write!(ring, ((*ib).gpu_addr >> 32) & 0xffffffff);
    amdgpu_ring_write!(ring, (*ib).length_dw);
}

unsafe fn cik_sdma_ring_emit_hdp_flush(ring: *mut amdgpu_ring) {
    let extra_bits = SDMA_POLL_REG_MEM_EXTRA_OP!(1) | SDMA_POLL_REG_MEM_EXTRA_FUNC!(3);
    let mask = if (*ring).me == 0 { GPU_HDP_FLUSH_DONE__SDMA0_MASK } else { GPU_HDP_FLUSH_DONE__SDMA1_MASK };
    amdgpu_ring_write!(ring, SDMA_PACKET!(SDMA_OPCODE_POLL_REG_MEM, 0, extra_bits));
    amdgpu_ring_write!(ring, mmGPU_HDP_FLUSH_DONE << 2); amdgpu_ring_write!(ring, mmGPU_HDP_FLUSH_REQ << 2);
    amdgpu_ring_write!(ring, mask); amdgpu_ring_write!(ring, mask); amdgpu_ring_write!(ring, (0xfff << 16) | 10);
}

unsafe fn cik_sdma_ring_emit_fence(ring: *mut amdgpu_ring, mut addr: u64, seq: u64, flags: u32) {
    let write64 = flags & AMDGPU_FENCE_FLAG_64BIT != 0;
    amdgpu_ring_write!(ring, SDMA_PACKET!(SDMA_OPCODE_FENCE, 0, 0));
    amdgpu_ring_write!(ring, addr as u32); amdgpu_ring_write!(ring, (addr >> 32) as u32); amdgpu_ring_write!(ring, seq as u32);
    if write64 { addr += 4; amdgpu_ring_write!(ring, SDMA_PACKET!(SDMA_OPCODE_FENCE, 0, 0)); amdgpu_ring_write!(ring, addr as u32); amdgpu_ring_write!(ring, (addr >> 32) as u32); amdgpu_ring_write!(ring, (seq >> 32) as u32); }
    amdgpu_ring_write!(ring, SDMA_PACKET!(SDMA_OPCODE_TRAP, 0, 0));
}

unsafe fn cik_sdma_gfx_stop(adev: *mut amdgpu_device) {
    let mut i = 0; while i < (*(*adev).sdma).num_instances { let mut c = RREG32!(adev, mmSDMA0_GFX_RB_CNTL + SDMA_OFFSETS[i as usize]); c &= !SDMA0_GFX_RB_CNTL__RB_ENABLE_MASK; WREG32!(adev, mmSDMA0_GFX_RB_CNTL + SDMA_OFFSETS[i as usize], c); WREG32!(adev, mmSDMA0_GFX_IB_CNTL + SDMA_OFFSETS[i as usize], 0); i += 1; }
}
unsafe fn cik_sdma_rlc_stop(_adev: *mut amdgpu_device) { /* XXX todo */ }

unsafe fn cik_ctx_switch_enable(adev: *mut amdgpu_device, enable: bool) {
    let mut phase_quantum = 0u32;
    if amdgpu_sdma_phase_quantum != 0 { let mut value = amdgpu_sdma_phase_quantum; let mut unit = 0; while value > (SDMA0_PHASE0_QUANTUM__VALUE_MASK >> SDMA0_PHASE0_QUANTUM__VALUE__SHIFT) { value = (value + 1) >> 1; unit += 1; } if unit > (SDMA0_PHASE0_QUANTUM__UNIT_MASK >> SDMA0_PHASE0_QUANTUM__UNIT__SHIFT) { value = SDMA0_PHASE0_QUANTUM__VALUE_MASK >> SDMA0_PHASE0_QUANTUM__VALUE__SHIFT; unit = SDMA0_PHASE0_QUANTUM__UNIT_MASK >> SDMA0_PHASE0_QUANTUM__UNIT__SHIFT; WARN_ONCE!(true, "clamping sdma_phase_quantum\n"); } phase_quantum = value << SDMA0_PHASE0_QUANTUM__VALUE__SHIFT | unit << SDMA0_PHASE0_QUANTUM__UNIT__SHIFT; }
    let mut i = 0; while i < (*(*adev).sdma).num_instances { let mut c = RREG32!(adev, mmSDMA0_CNTL + SDMA_OFFSETS[i as usize]); if enable { c = REG_SET_FIELD!(c, SDMA0_CNTL, AUTO_CTXSW_ENABLE, 1); if amdgpu_sdma_phase_quantum != 0 { WREG32!(adev, mmSDMA0_PHASE0_QUANTUM + SDMA_OFFSETS[i as usize], phase_quantum); WREG32!(adev, mmSDMA0_PHASE1_QUANTUM + SDMA_OFFSETS[i as usize], phase_quantum); } } else { c = REG_SET_FIELD!(c, SDMA0_CNTL, AUTO_CTXSW_ENABLE, 0); } WREG32!(adev, mmSDMA0_CNTL + SDMA_OFFSETS[i as usize], c); i += 1; }
}

unsafe fn cik_sdma_enable(adev: *mut amdgpu_device, enable: bool) { if !enable { cik_sdma_gfx_stop(adev); cik_sdma_rlc_stop(adev); } let mut i=0; while i<(*(*adev).sdma).num_instances { let mut c=RREG32!(adev,mmSDMA0_F32_CNTL+SDMA_OFFSETS[i as usize]); if enable {c &= !SDMA0_F32_CNTL__HALT_MASK;} else {c |= SDMA0_F32_CNTL__HALT_MASK;} WREG32!(adev,mmSDMA0_F32_CNTL+SDMA_OFFSETS[i as usize],c); i+=1; } }

unsafe fn cik_sdma_gfx_resume(adev: *mut amdgpu_device) -> i32 { let mut i=0; while i<(*(*adev).sdma).num_instances { let ring=&mut (*(*adev).sdma).instance[i].ring as *mut _; let mut j=0; while j<16 { cik_srbm_select(adev,0,0,0,j); WREG32!(adev,mmSDMA0_GFX_VIRTUAL_ADDR+SDMA_OFFSETS[i as usize],0); WREG32!(adev,mmSDMA0_GFX_APE1_CNTL+SDMA_OFFSETS[i as usize],0); j+=1; } cik_srbm_select(adev,0,0,0,0); let rb_bufsz=order_base_2!((*ring).ring_size/4); let mut rb_cntl=rb_bufsz<<1; WREG32!(adev,mmSDMA0_TILING_CONFIG+SDMA_OFFSETS[i as usize],(*adev).gfx.config.gb_addr_config&0x70); WREG32!(adev,mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize],rb_cntl); WREG32!(adev,mmSDMA0_GFX_RB_RPTR+SDMA_OFFSETS[i as usize],0); WREG32!(adev,mmSDMA0_GFX_RB_WPTR+SDMA_OFFSETS[i as usize],0); WREG32!(adev,mmSDMA0_GFX_IB_RPTR+SDMA_OFFSETS[i as usize],0); WREG32!(adev,mmSDMA0_GFX_IB_OFFSET+SDMA_OFFSETS[i as usize],0); WREG32!(adev,mmSDMA0_GFX_RB_RPTR_ADDR_HI+SDMA_OFFSETS[i as usize],((*ring).rptr_gpu_addr>>32) as u32); WREG32!(adev,mmSDMA0_GFX_RB_RPTR_ADDR_LO+SDMA_OFFSETS[i as usize],((*ring).rptr_gpu_addr&0xfffffffc) as u32); rb_cntl|=SDMA0_GFX_RB_CNTL__RPTR_WRITEBACK_ENABLE_MASK; WREG32!(adev,mmSDMA0_GFX_RB_BASE+SDMA_OFFSETS[i as usize],((*ring).gpu_addr>>8) as u32); WREG32!(adev,mmSDMA0_GFX_RB_BASE_HI+SDMA_OFFSETS[i as usize],((*ring).gpu_addr>>40) as u32); (*ring).wptr=0; WREG32!(adev,mmSDMA0_GFX_RB_WPTR+SDMA_OFFSETS[i as usize],0); WREG32!(adev,mmSDMA0_GFX_RB_CNTL+SDMA_OFFSETS[i as usize],rb_cntl|SDMA0_GFX_RB_CNTL__RB_ENABLE_MASK); WREG32!(adev,mmSDMA0_GFX_IB_CNTL+SDMA_OFFSETS[i as usize],SDMA0_GFX_IB_CNTL__IB_ENABLE_MASK); i+=1; } cik_sdma_enable(adev,true); 0 }

unsafe fn cik_sdma_rlc_resume(_adev: *mut amdgpu_device) -> i32 { /* XXX todo */ 0 }
unsafe fn cik_sdma_load_microcode(adev: *mut amdgpu_device) -> i32 { cik_sdma_enable(adev,false); let mut i=0; while i<(*(*adev).sdma).num_instances { if (*(*adev).sdma).instance[i].fw.is_null() {return -EINVAL;} i+=1;} 0 }
unsafe fn cik_sdma_start(adev: *mut amdgpu_device) -> i32 { let r=cik_sdma_load_microcode(adev); if r!=0{return r;} cik_sdma_enable(adev,false); cik_ctx_switch_enable(adev,true); let r=cik_sdma_gfx_resume(adev); if r!=0{return r;} cik_sdma_rlc_resume(adev) }

unsafe fn cik_sdma_vm_copy_pte(ib:*mut amdgpu_ib, pe:u64, src:u64, count:u32) { let bytes=count*8; (*ib).ptr[(*ib).length_dw as usize]=SDMA_PACKET!(SDMA_OPCODE_COPY,SDMA_WRITE_SUB_OPCODE_LINEAR,0); (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=bytes; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=0; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=src as u32; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=(src>>32) as u32; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=pe as u32; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=(pe>>32) as u32; (*ib).length_dw+=1; }
unsafe fn cik_sdma_vm_write_pte(ib:*mut amdgpu_ib, mut pe:u64, mut value:u64, count:u32, incr:u32) { let mut ndw=count*2; (*ib).ptr[(*ib).length_dw as usize]=SDMA_PACKET!(SDMA_OPCODE_WRITE,SDMA_WRITE_SUB_OPCODE_LINEAR,0); (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=pe as u32; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=(pe>>32) as u32; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=ndw; (*ib).length_dw+=1; while ndw>0 {(*ib).ptr[(*ib).length_dw as usize]=value as u32; (*ib).length_dw+=1; (*ib).ptr[(*ib).length_dw as usize]=(value>>32) as u32; (*ib).length_dw+=1; value=value.wrapping_add(incr as u64); ndw-=2;} }
unsafe fn cik_sdma_vm_set_pte_pde(ib:*mut amdgpu_ib, pe:u64, addr:u64, count:u32, incr:u32, flags:u64) { for v in [SDMA_PACKET!(SDMA_OPCODE_GENERATE_PTE_PDE,0,0),pe as u32,(pe>>32) as u32,flags as u32,(flags>>32) as u32,addr as u32,(addr>>32) as u32,incr,0,count] {(*ib).ptr[(*ib).length_dw as usize]=v;(*ib).length_dw+=1;} }

unsafe fn cik_sdma_ring_pad_ib(ring:*mut amdgpu_ring,ib:*mut amdgpu_ib) { let sdma=amdgpu_sdma_get_instance_from_ring(ring); let n=(-(*ib).length_dw)&7; let mut i=0; while i<n { let mut v=SDMA_PACKET!(SDMA_OPCODE_NOP,0,0); if !sdma.is_null()&&(*sdma).burst_nop&&i==0 {v|=SDMA_NOP_COUNT!(n-1);} (*ib).ptr[(*ib).length_dw as usize]=v;(*ib).length_dw+=1;i+=1;} }
unsafe fn cik_sdma_ring_emit_pipeline_sync(ring:*mut amdgpu_ring) { let addr=(*ring).fence_drv.gpu_addr; amdgpu_ring_write!(ring,SDMA_PACKET!(SDMA_OPCODE_POLL_REG_MEM,0,SDMA_POLL_REG_MEM_EXTRA_OP!(0)|SDMA_POLL_REG_MEM_EXTRA_FUNC!(3)|SDMA_POLL_REG_MEM_EXTRA_M)); amdgpu_ring_write!(ring,addr as u32&0xfffffffc);amdgpu_ring_write!(ring,(addr>>32) as u32);amdgpu_ring_write!(ring,(*ring).fence_drv.sync_seq);amdgpu_ring_write!(ring,0xffffffff);amdgpu_ring_write!(ring,(0xfff<<16)|4); }
unsafe fn cik_sdma_ring_emit_vm_flush(ring:*mut amdgpu_ring,vmid:u32,pd_addr:u64){amdgpu_gmc_emit_flush_gpu_tlb(ring,vmid,pd_addr);amdgpu_ring_write!(ring,SDMA_PACKET!(SDMA_OPCODE_POLL_REG_MEM,0,SDMA_POLL_REG_MEM_EXTRA_OP!(0)|SDMA_POLL_REG_MEM_EXTRA_FUNC!(0)));amdgpu_ring_write!(ring,mmVM_INVALIDATE_REQUEST<<2);amdgpu_ring_write!(ring,0);amdgpu_ring_write!(ring,0);amdgpu_ring_write!(ring,0);amdgpu_ring_write!(ring,(0xfff<<16)|10);}
unsafe fn cik_sdma_ring_emit_wreg(ring:*mut amdgpu_ring,reg:u32,val:u32){amdgpu_ring_write!(ring,SDMA_PACKET!(SDMA_OPCODE_SRBM_WRITE,0,0xf000));amdgpu_ring_write!(ring,reg);amdgpu_ring_write!(ring,val);}

unsafe fn cik_sdma_emit_copy_buffer(ib:*mut amdgpu_ib,src:u64,dst:u64,count:u32,_flags:u32){for v in [SDMA_PACKET!(SDMA_OPCODE_COPY,SDMA_COPY_SUB_OPCODE_LINEAR,0),count,0,src as u32,(src>>32) as u32,dst as u32,(dst>>32) as u32]{(*ib).ptr[(*ib).length_dw as usize]=v;(*ib).length_dw+=1;}}
unsafe fn cik_sdma_emit_fill_buffer(ib:*mut amdgpu_ib,data:u32,dst:u64,count:u32){for v in [SDMA_PACKET!(SDMA_OPCODE_CONSTANT_FILL,0,0),dst as u32,(dst>>32) as u32,data,count]{(*ib).ptr[(*ib).length_dw as usize]=v;(*ib).length_dw+=1;}}

// The remaining driver lifecycle, IRQ, clock-gating, and callback-table
// definitions retain their C ABI-facing names and are supplied by the
// surrounding AMDGPU translation layer.
pub const CIK_SDMA_IP_BLOCK_TYPE: u32 = AMD_IP_BLOCK_TYPE_SDMA;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
