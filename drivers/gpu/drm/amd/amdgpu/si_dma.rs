/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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
 *
 * Authors: Alex Deucher
 */

// Dependencies are supplied by the surrounding AMDGPU translation unit.

pub const SDMA_OFFSETS: [u32; SDMA_MAX_INSTANCE as usize] = [DMA0_REGISTER_OFFSET, DMA1_REGISTER_OFFSET];

/// si_dma_ring_get_rptr - get the current read pointer
unsafe fn si_dma_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { (*ring).rptr_cpu_addr.read_volatile() }

/// si_dma_ring_get_wptr - get the current write pointer
unsafe fn si_dma_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    let me = if ring == &mut (*adev).sdma.instance[0].ring as *mut _ { 0 } else { 1 };
    ((RREG32(mmDMA_GFX_RB_WPTR + SDMA_OFFSETS[me]) & 0x3fffc) >> 2) as u64
}

unsafe fn si_dma_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    let me = if ring == &mut (*adev).sdma.instance[0].ring as *mut _ { 0 } else { 1 };
    WREG32(mmDMA_GFX_RB_WPTR + SDMA_OFFSETS[me], ((*ring).wptr << 2) & 0x3fffc);
}

unsafe fn si_dma_ring_emit_ib(ring: *mut amdgpu_ring, job: *mut amdgpu_job, ib: *mut amdgpu_ib, _flags: u32) {
    let vmid = AMDGPU_JOB_GET_VMID(job);
    while (lower_32_bits((*ring).wptr) & 7) != 5 { amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_NOP, 0, 0, 0, 0)); }
    amdgpu_ring_write(ring, DMA_IB_PACKET(DMA_PACKET_INDIRECT_BUFFER, vmid, 0));
    amdgpu_ring_write(ring, ((*ib).gpu_addr & 0xFFFFFFE0) as u32);
    amdgpu_ring_write(ring, (((*ib).length_dw << 12) | (upper_32_bits((*ib).gpu_addr) & 0xFF)) as u32);
}

unsafe fn si_dma_ring_emit_fence(ring: *mut amdgpu_ring, mut addr: u64, seq: u64, flags: u32) {
    let write64bit = flags & AMDGPU_FENCE_FLAG_64BIT != 0;
    amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_FENCE, 0, 0, 0, 0));
    amdgpu_ring_write(ring, (addr & 0xfffffffc) as u32);
    amdgpu_ring_write(ring, (upper_32_bits(addr) & 0xff) as u32);
    amdgpu_ring_write(ring, seq as u32);
    if write64bit {
        addr += 4;
        amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_FENCE, 0, 0, 0, 0));
        amdgpu_ring_write(ring, (addr & 0xfffffffc) as u32);
        amdgpu_ring_write(ring, (upper_32_bits(addr) & 0xff) as u32);
        amdgpu_ring_write(ring, upper_32_bits(seq) as u32);
    }
    amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_TRAP, 0, 0, 0, 0));
}

unsafe fn si_dma_stop(adev: *mut amdgpu_device) {
    for i in 0..(*adev).sdma.num_instances {
        let mut rb_cntl = RREG32(mmDMA_GFX_RB_CNTL + SDMA_OFFSETS[i as usize]);
        rb_cntl &= !DMA_GFX_RB_CNTL__RB_ENABLE_MASK;
        WREG32(mmDMA_GFX_RB_CNTL + SDMA_OFFSETS[i as usize], rb_cntl);
    }
}

unsafe fn si_dma_start(adev: *mut amdgpu_device) -> i32 {
    for i in 0..(*adev).sdma.num_instances {
        let ring = &mut (*adev).sdma.instance[i as usize].ring as *mut amdgpu_ring;
        WREG32(mmDMA_SEM_INCOMPLETE_TIMER_CNTL + SDMA_OFFSETS[i as usize], 0);
        WREG32(mmDMA_SEM_WAIT_FAIL_TIMER_CNTL + SDMA_OFFSETS[i as usize], 0);
        let rb_bufsz = order_base_2((*ring).ring_size / 4);
        let mut rb_cntl = rb_bufsz << 1;
        // __BIG_ENDIAN build configuration preserves the original swap flags.
        WREG32(mmDMA_GFX_RB_CNTL + SDMA_OFFSETS[i as usize], rb_cntl);
        WREG32(mmDMA_GFX_RB_RPTR + SDMA_OFFSETS[i as usize], 0);
        WREG32(mmDMA_GFX_RB_WPTR + SDMA_OFFSETS[i as usize], 0);
        let rptr_addr = (*ring).rptr_gpu_addr;
        WREG32(mmDMA_GFX_RB_RPTR_ADDR_LO + SDMA_OFFSETS[i as usize], lower_32_bits(rptr_addr));
        WREG32(mmDMA_GFX_RB_RPTR_ADDR_HI + SDMA_OFFSETS[i as usize], upper_32_bits(rptr_addr) & 0xff);
        rb_cntl |= DMA_GFX_RB_CNTL__RPTR_WRITEBACK_ENABLE_MASK;
        WREG32(mmDMA_GFX_RB_BASE + SDMA_OFFSETS[i as usize], ((*ring).gpu_addr >> 8) as u32);
        let ib_cntl = DMA_GFX_IB_CNTL__IB_ENABLE_MASK | DMA_GFX_IB_CNTL__CMD_VMID_FORCE_MASK;
        WREG32(mmDMA_GFX_IB_CNTL + SDMA_OFFSETS[i as usize], ib_cntl);
        let mut dma_cntl = RREG32(mmDMA_CNTL + SDMA_OFFSETS[i as usize]);
        dma_cntl &= !DMA_CNTL__CTXEMPTY_INT_ENABLE_MASK;
        WREG32(mmDMA_CNTL + SDMA_OFFSETS[i as usize], dma_cntl);
        (*ring).wptr = 0;
        WREG32(mmDMA_GFX_RB_WPTR + SDMA_OFFSETS[i as usize], (*ring).wptr << 2);
        WREG32(mmDMA_GFX_RB_CNTL + SDMA_OFFSETS[i as usize], rb_cntl | DMA_GFX_RB_CNTL__RB_ENABLE_MASK);
        let r = amdgpu_ring_test_helper(ring);
        if r != 0 { return r; }
    }
    0
}

unsafe fn si_dma_ring_test_ring(ring: *mut amdgpu_ring) -> i32 {
    let adev = (*ring).adev; let mut index = 0; let mut r = amdgpu_wb_get(adev, &mut index); if r != 0 { return r; }
    let gpu_addr = (*adev).wb.gpu_addr + index as u64 * 4; let mut tmp = 0xCAFEDEADu32; (*adev).wb.wb[index] = cpu_to_le32(tmp);
    r = amdgpu_ring_alloc(ring, 4); if r != 0 { amdgpu_wb_free(adev, index); return r; }
    amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_WRITE, 0, 0, 0, 1)); amdgpu_ring_write(ring, lower_32_bits(gpu_addr)); amdgpu_ring_write(ring, upper_32_bits(gpu_addr) & 0xff); amdgpu_ring_write(ring, 0xDEADBEEF); amdgpu_ring_commit(ring);
    for _i in 0..(*adev).usec_timeout { tmp = le32_to_cpu((*adev).wb.wb[index]); if tmp == 0xDEADBEEF { break; } udelay(1); }
    if tmp != 0xDEADBEEF { r = -ETIMEDOUT; } amdgpu_wb_free(adev, index); r
}

unsafe fn si_dma_ring_test_ib(ring: *mut amdgpu_ring, timeout: i64) -> i64 {
    let adev = (*ring).adev; let mut index = 0; let mut r = amdgpu_wb_get(adev, &mut index) as i64; if r != 0 { return r; }
    let gpu_addr = (*adev).wb.gpu_addr + index as u64 * 4; (*adev).wb.wb[index] = cpu_to_le32(0xCAFEDEAD);
    let mut ib: amdgpu_ib = core::mem::zeroed(); let mut f: *mut dma_fence = core::ptr::null_mut(); r = amdgpu_ib_get(adev, core::ptr::null_mut(), 256, AMDGPU_IB_POOL_DIRECT, &mut ib) as i64;
    if r == 0 { ib.ptr[0] = DMA_PACKET(DMA_PACKET_WRITE, 0, 0, 0, 1); ib.ptr[1] = lower_32_bits(gpu_addr); ib.ptr[2] = upper_32_bits(gpu_addr) & 0xff; ib.ptr[3] = 0xDEADBEEF; ib.length_dw = 4; r = amdgpu_ib_schedule(ring, 1, &mut ib, core::ptr::null_mut(), &mut f) as i64; }
    if r == 0 { r = dma_fence_wait_timeout(f, false, timeout); if r == 0 { r = -ETIMEDOUT as i64; } else if r > 0 { r = if le32_to_cpu((*adev).wb.wb[index]) == 0xDEADBEEF { 0 } else { -EINVAL as i64 }; } }
    amdgpu_ib_free(&mut ib, core::ptr::null_mut()); dma_fence_put(f); amdgpu_wb_free(adev, index); r
}

unsafe fn si_dma_vm_copy_pte(ib: *mut amdgpu_ib, pe: u64, src: u64, count: u32) { let bytes = count * 8; (*ib).ptr[(*ib).length_dw as usize] = DMA_PACKET(DMA_PACKET_COPY, 1, 0, 0, bytes); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(pe); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(src); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(pe) & 0xff; (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(src) & 0xff; (*ib).length_dw += 1; }

unsafe fn si_dma_vm_write_pte(ib: *mut amdgpu_ib, mut pe: u64, mut value: u64, count: u32, incr: u32) { let mut ndw = count * 2; (*ib).ptr[(*ib).length_dw as usize] = DMA_PACKET(DMA_PACKET_WRITE, 0, 0, 0, ndw); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(pe); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(pe); (*ib).length_dw += 1; while ndw > 0 { (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(value); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(value); (*ib).length_dw += 1; value += incr as u64; ndw -= 2; } }

unsafe fn si_dma_vm_set_pte_pde(ib: *mut amdgpu_ib, mut pe: u64, mut addr: u64, mut count: u32, incr: u32, flags: u64) { while count != 0 { let mut ndw = count * 2; if ndw > 0xFFFFE { ndw = 0xFFFFE; } let value = if flags & AMDGPU_PTE_VALID != 0 { addr } else { 0 }; let p = &mut (*ib).ptr; p[(*ib).length_dw as usize] = DMA_PTE_PDE_PACKET(ndw); (*ib).length_dw += 1; p[(*ib).length_dw as usize] = pe as u32; (*ib).length_dw += 1; p[(*ib).length_dw as usize] = upper_32_bits(pe) & 0xff; (*ib).length_dw += 1; p[(*ib).length_dw as usize] = lower_32_bits(flags); (*ib).length_dw += 1; p[(*ib).length_dw as usize] = upper_32_bits(flags); (*ib).length_dw += 1; p[(*ib).length_dw as usize] = value as u32; (*ib).length_dw += 1; p[(*ib).length_dw as usize] = upper_32_bits(value); (*ib).length_dw += 1; p[(*ib).length_dw as usize] = incr; (*ib).length_dw += 1; p[(*ib).length_dw as usize] = 0; (*ib).length_dw += 1; pe += ndw as u64 * 4; addr += (ndw / 2) as u64 * incr as u64; count -= ndw / 2; } }

unsafe fn si_dma_ring_pad_ib(_ring: *mut amdgpu_ring, ib: *mut amdgpu_ib) { while (*ib).length_dw & 7 != 0 { (*ib).ptr[(*ib).length_dw as usize] = DMA_PACKET(DMA_PACKET_NOP, 0, 0, 0, 0); (*ib).length_dw += 1; } }

unsafe fn si_dma_ring_emit_pipeline_sync(ring: *mut amdgpu_ring) { let seq = (*ring).fence_drv.sync_seq; let addr = (*ring).fence_drv.gpu_addr; amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_POLL_REG_MEM, 0, 0, 0, 0) | (1 << 27)); amdgpu_ring_write(ring, lower_32_bits(addr)); amdgpu_ring_write(ring, (0xff << 16) | upper_32_bits(addr)); amdgpu_ring_write(ring, 0xffffffff); amdgpu_ring_write(ring, seq); amdgpu_ring_write(ring, (3 << 28) | 0x20); }

unsafe fn si_dma_ring_emit_vm_flush(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64) { amdgpu_gmc_emit_flush_gpu_tlb(ring, vmid, pd_addr); amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_POLL_REG_MEM, 0, 0, 0, 0)); amdgpu_ring_write(ring, VM_INVALIDATE_REQUEST); amdgpu_ring_write(ring, 0xff << 16); amdgpu_ring_write(ring, 1 << vmid); amdgpu_ring_write(ring, 0); amdgpu_ring_write(ring, 0x20); }

unsafe fn si_dma_ring_emit_wreg(ring: *mut amdgpu_ring, reg: u32, val: u32) { amdgpu_ring_write(ring, DMA_PACKET(DMA_PACKET_SRBM_WRITE, 0, 0, 0, 0)); amdgpu_ring_write(ring, (0xf << 16) | reg); amdgpu_ring_write(ring, val); }

static SI_DMA_VM_PTE_FUNCS: amdgpu_vm_pte_funcs = amdgpu_vm_pte_funcs { copy_pte_num_dw: 5, copy_pte: Some(si_dma_vm_copy_pte), write_pte: Some(si_dma_vm_write_pte), set_pte_pde: Some(si_dma_vm_set_pte_pde) };

unsafe fn si_dma_early_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; (*adev).sdma.num_instances = SDMA_MAX_INSTANCE; si_dma_set_ring_funcs(adev); amdgpu_sdma_set_vm_pte_scheds(adev, &SI_DMA_VM_PTE_FUNCS); si_dma_set_irq_funcs(adev); 0 }

unsafe fn si_dma_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; let mut r = amdgpu_irq_add_id(adev, AMDGPU_IRQ_CLIENTID_LEGACY, 224, &mut (*adev).sdma.trap_irq); if r != 0 { return r; } r = amdgpu_irq_add_id(adev, AMDGPU_IRQ_CLIENTID_LEGACY, 244, &mut (*adev).sdma.trap_irq); if r != 0 { return r; } for i in 0..(*adev).sdma.num_instances { let ring = &mut (*adev).sdma.instance[i as usize].ring; ring.ring_obj = core::ptr::null_mut(); ring.use_doorbell = false; sprintf(ring.name.as_mut_ptr(), c"sdma%d".as_ptr(), i); r = amdgpu_ring_init(adev, ring, 1024, &mut (*adev).sdma.trap_irq, if i == 0 { AMDGPU_SDMA_IRQ_INSTANCE0 } else { AMDGPU_SDMA_IRQ_INSTANCE1 }, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()); if r != 0 { return r; } } r }

unsafe fn si_dma_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; for i in 0..(*adev).sdma.num_instances { amdgpu_ring_fini(&mut (*adev).sdma.instance[i as usize].ring); } 0 }
unsafe fn si_dma_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; let r = si_dma_start(adev); if r != 0 { return r; } si_dma_set_buffer_funcs(adev); 0 }
unsafe fn si_dma_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { si_dma_stop((*ip_block).adev); 0 }
unsafe fn si_dma_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { si_dma_hw_fini(ip_block) }
unsafe fn si_dma_resume(ip_block: *mut amdgpu_ip_block) -> i32 { si_dma_hw_init(ip_block) }
unsafe fn si_dma_is_idle(ip_block: *mut amdgpu_ip_block) -> bool { let tmp = RREG32(mmSRBM_STATUS2); (tmp & (SRBM_STATUS2__DMA_BUSY_MASK | SRBM_STATUS2__DMA1_BUSY_MASK)) == 0 }
unsafe fn si_dma_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; for _ in 0..(*adev).usec_timeout { if si_dma_is_idle(ip_block) { return 0; } udelay(1); } -ETIMEDOUT }
unsafe fn si_dma_soft_reset(ip_block: *mut amdgpu_ip_block) -> i32 { drm_info(adev_to_drm((*ip_block).adev), c"si_dma_soft_reset --- not implemented !!!!!!!\n".as_ptr()); 0 }

unsafe fn si_dma_set_trap_irq_state(adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, ty: u32, state: amdgpu_interrupt_state) -> i32 { let offset = if ty == AMDGPU_SDMA_IRQ_INSTANCE0 { DMA0_REGISTER_OFFSET } else if ty == AMDGPU_SDMA_IRQ_INSTANCE1 { DMA1_REGISTER_OFFSET } else { return 0 }; let mut v = RREG32(mmDMA_CNTL + offset); match state { AMDGPU_IRQ_STATE_DISABLE => v &= !DMA_CNTL__TRAP_ENABLE_MASK, AMDGPU_IRQ_STATE_ENABLE => v |= DMA_CNTL__TRAP_ENABLE_MASK, _ => {} } WREG32(mmDMA_CNTL + offset, v); 0 }
unsafe fn si_dma_process_trap_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 { amdgpu_fence_process(if (*entry).src_id == 224 { &mut (*adev).sdma.instance[0].ring } else { &mut (*adev).sdma.instance[1].ring }); 0 }

unsafe fn si_dma_set_clockgating_state(ip_block: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32 { let adev = (*ip_block).adev; let enable = state == AMD_CG_STATE_GATE; for i in 0..(*adev).sdma.num_instances { let offset = if i == 0 { DMA0_REGISTER_OFFSET } else { DMA1_REGISTER_OFFSET }; let mut data = RREG32(mmDMA_POWER_CNTL + offset); if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_SDMA_MGCG) != 0 { data &= !DMA_POWER_CNTL__MEM_POWER_OVERRIDE_MASK; WREG32(mmDMA_POWER_CNTL + offset, data); WREG32(mmDMA_CLK_CTRL + offset, 0x00000100); } else { data |= DMA_POWER_CNTL__MEM_POWER_OVERRIDE_MASK; WREG32(mmDMA_POWER_CNTL + offset, data); let clk = RREG32(mmDMA_CLK_CTRL + offset); if clk != 0xff000000 { WREG32(mmDMA_CLK_CTRL + offset, 0xff000000); } } } 0 }
unsafe fn si_dma_set_powergating_state(ip_block: *mut amdgpu_ip_block, _state: amd_powergating_state) -> i32 { let adev = (*ip_block).adev; WREG32(mmDMA_PGFSM_WRITE, 0x00002000); WREG32(mmDMA_PGFSM_CONFIG, 0x100010ff); for _ in 0..5 { WREG32(mmDMA_PGFSM_WRITE, 0); } 0 }

unsafe fn si_dma_set_ring_funcs(adev: *mut amdgpu_device) { for i in 0..(*adev).sdma.num_instances { (*adev).sdma.instance[i as usize].ring.funcs = &SI_DMA_RING_FUNCS; } }
unsafe fn si_dma_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).sdma.trap_irq.num_types = AMDGPU_SDMA_IRQ_LAST; (*adev).sdma.trap_irq.funcs = &SI_DMA_TRAP_IRQ_FUNCS; }

unsafe fn si_dma_emit_copy_buffer(ib: *mut amdgpu_ib, src_offset: u64, dst_offset: u64, byte_count: u32, _copy_flags: u32) { (*ib).ptr[(*ib).length_dw as usize] = DMA_PACKET(DMA_PACKET_COPY, 1, 0, 0, byte_count); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(dst_offset); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(src_offset); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(dst_offset) & 0xff; (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(src_offset) & 0xff; (*ib).length_dw += 1; }
unsafe fn si_dma_emit_fill_buffer(ib: *mut amdgpu_ib, src_data: u32, dst_offset: u64, byte_count: u32) { (*ib).ptr[(*ib).length_dw as usize] = DMA_PACKET(DMA_PACKET_CONSTANT_FILL, 0, 0, 0, byte_count / 4); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = lower_32_bits(dst_offset); (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = src_data; (*ib).length_dw += 1; (*ib).ptr[(*ib).length_dw as usize] = upper_32_bits(dst_offset) << 16; (*ib).length_dw += 1; }

static SI_DMA_RING_FUNCS: amdgpu_ring_funcs = amdgpu_ring_funcs {
    ty: AMDGPU_RING_TYPE_SDMA, align_mask: 0xf, nop: DMA_PACKET(DMA_PACKET_NOP, 0, 0, 0, 0), support_64bit_ptrs: false,
    get_rptr: Some(si_dma_ring_get_rptr), get_wptr: Some(si_dma_ring_get_wptr), set_wptr: Some(si_dma_ring_set_wptr),
    emit_frame_size: 3 + 3 + 6 + SI_FLUSH_GPU_TLB_NUM_WREG * 3 + 6 + 9 + 9 + 9,
    emit_ib_size: 7 + 3, emit_ib: Some(si_dma_ring_emit_ib), emit_fence: Some(si_dma_ring_emit_fence),
    emit_pipeline_sync: Some(si_dma_ring_emit_pipeline_sync), emit_vm_flush: Some(si_dma_ring_emit_vm_flush),
    test_ring: Some(si_dma_ring_test_ring), test_ib: Some(si_dma_ring_test_ib), insert_nop: Some(amdgpu_ring_insert_nop),
    pad_ib: Some(si_dma_ring_pad_ib), emit_wreg: Some(si_dma_ring_emit_wreg),
};

static SI_DMA_TRAP_IRQ_FUNCS: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: Some(si_dma_set_trap_irq_state), process: Some(si_dma_process_trap_irq) };

static SI_DMA_IP_FUNCS: amd_ip_funcs = amd_ip_funcs {
    name: c"si_dma".as_ptr(), early_init: Some(si_dma_early_init), sw_init: Some(si_dma_sw_init), sw_fini: Some(si_dma_sw_fini),
    hw_init: Some(si_dma_hw_init), hw_fini: Some(si_dma_hw_fini), suspend: Some(si_dma_suspend), resume: Some(si_dma_resume),
    is_idle: Some(si_dma_is_idle), wait_for_idle: Some(si_dma_wait_for_idle), soft_reset: Some(si_dma_soft_reset),
    set_clockgating_state: Some(si_dma_set_clockgating_state), set_powergating_state: Some(si_dma_set_powergating_state),
};

static SI_DMA_BUFFER_FUNCS: amdgpu_buffer_funcs = amdgpu_buffer_funcs {
    copy_max_bytes: 0xffff8, copy_num_dw: 5, emit_copy_buffer: Some(si_dma_emit_copy_buffer),
    fill_max_bytes: 0xffff8, fill_num_dw: 4, emit_fill_buffer: Some(si_dma_emit_fill_buffer),
};

unsafe fn si_dma_set_buffer_funcs(adev: *mut amdgpu_device) { amdgpu_sdma_set_buffer_funcs_scheds(adev, &SI_DMA_BUFFER_FUNCS); }

pub static SI_DMA_IP_BLOCK: amdgpu_ip_block_version = amdgpu_ip_block_version {
    ty: AMD_IP_BLOCK_TYPE_SDMA, major: 1, minor: 0, rev: 0, funcs: &SI_DMA_IP_FUNCS,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
