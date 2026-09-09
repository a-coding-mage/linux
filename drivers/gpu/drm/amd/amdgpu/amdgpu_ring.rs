/* Faithful low-level Rust translation of amdgpu_ring.c. */

/* The included Linux/DRM declarations and AMDGPU structures are supplied by
 * the surrounding translation unit. */

pub unsafe fn amdgpu_ring_max_ibs(ty: amdgpu_ring_type) -> u32 {
    match ty {
        AMDGPU_RING_TYPE_GFX => 192,
        AMDGPU_RING_TYPE_COMPUTE => 125,
        AMDGPU_RING_TYPE_VCN_JPEG => 16,
        _ => 49,
    }
}

pub unsafe fn amdgpu_ring_alloc(ring: *mut amdgpu_ring, mut ndw: u32) -> i32 {
    ndw = (ndw + (*(*ring).funcs).align_mask) & !(*(*ring).funcs).align_mask;
    if !(*ring).reemit && ndw > (*ring).max_dw { return -ENOMEM; }
    (*ring).count_dw = ndw as i32;
    (*ring).wptr_old = (*ring).wptr;
    if let Some(f) = (*(*ring).funcs).begin_use { f(ring); }
    0
}

pub unsafe fn amdgpu_ring_insert_nop(ring: *mut amdgpu_ring, count: u32) {
    let occupied = (*ring).wptr & (*ring).buf_mask;
    let chunk1 = core::cmp::min((*ring).buf_mask + 1 - occupied, count);
    let chunk2 = count - chunk1;
    if chunk1 != 0 { memset32((*ring).ring.add(occupied as usize), (*(*ring).funcs).nop, chunk1); }
    if chunk2 != 0 { memset32((*ring).ring, (*(*ring).funcs).nop, chunk2); }
    (*ring).wptr = ((*ring).wptr + count) & (*ring).ptr_mask;
    (*ring).count_dw -= count as i32;
}

pub unsafe fn amdgpu_ring_generic_pad_ib(ring: *mut amdgpu_ring, ib: *mut amdgpu_ib) {
    let mask = (*(*ring).funcs).align_mask;
    let mut count = (*ib).length_dw & mask;
    if count != 0 {
        count = mask + 1 - count;
        memset32((*ib).ptr.add((*ib).length_dw as usize), (*(*ring).funcs).nop, count);
        (*ib).length_dw += count;
    }
}

pub unsafe fn amdgpu_ring_commit(ring: *mut amdgpu_ring) {
    if (*ring).count_dw < 0 { drm_err(adev_to_drm((*ring).adev), "writing more dwords to the ring than expected!\n"); }
    let mask = (*(*ring).funcs).align_mask;
    let count = (mask + 1 - ((*ring).wptr & mask)) & mask;
    if count != 0 { ((*(*ring).funcs).insert_nop.unwrap())(ring, count); }
    mb(); amdgpu_ring_set_wptr(ring);
    if let Some(f) = (*(*ring).funcs).end_use { f(ring); }
}

pub unsafe fn amdgpu_ring_undo(ring: *mut amdgpu_ring) {
    (*ring).wptr = (*ring).wptr_old;
    if let Some(f) = (*(*ring).funcs).end_use { f(ring); }
}

pub unsafe fn amdgpu_ring_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring,
    max_dw: u32, irq_src: *mut amdgpu_irq_src, irq_type: u32, hw_prio: u32,
    sched_score: *mut atomic_t) -> i32 {
    let mut sched_hw_submission = amdgpu_sched_hw_submission;
    if (*(*ring).funcs).ty == AMDGPU_RING_TYPE_KIQ { sched_hw_submission = core::cmp::max(sched_hw_submission, 256); }
    if (*(*ring).funcs).ty == AMDGPU_RING_TYPE_MES { sched_hw_submission = 8; }
    else if ring == &mut (*(*adev).sdma).instance[0].page { sched_hw_submission = 256; }
    if (*ring).adev.is_null() {
        if (*adev).num_rings >= AMDGPU_MAX_RINGS { return -EINVAL; }
        (*ring).adev = adev; (*ring).num_hw_submission = sched_hw_submission; (*ring).sched_score = sched_score;
        (*ring).vmid_wait = dma_fence_get_stub(); (*ring).idx = (*adev).num_rings; (*adev).num_rings += 1;
        (*adev).rings[(*ring).idx as usize] = ring;
        let r = amdgpu_fence_driver_init_ring(ring); if r != 0 { return r; }
    }
    let mut r = amdgpu_wb_get(adev, &mut (*ring).rptr_offs); if r != 0 { return r; }
    r = amdgpu_wb_get(adev, &mut (*ring).wptr_offs); if r != 0 { return r; }
    r = amdgpu_wb_get(adev, &mut (*ring).fence_offs); if r != 0 { return r; }
    r = amdgpu_wb_get(adev, &mut (*ring).trail_fence_offs); if r != 0 { return r; }
    r = amdgpu_wb_get(adev, &mut (*ring).cond_exe_offs); if r != 0 { return r; }
    (*ring).fence_gpu_addr = (*adev).wb.gpu_addr + (*ring).fence_offs as u64 * 4;
    (*ring).fence_cpu_addr = (*adev).wb.wb.add((*ring).fence_offs as usize);
    (*ring).rptr_gpu_addr = (*adev).wb.gpu_addr + (*ring).rptr_offs as u64 * 4;
    (*ring).rptr_cpu_addr = (*adev).wb.wb.add((*ring).rptr_offs as usize);
    (*ring).wptr_gpu_addr = (*adev).wb.gpu_addr + (*ring).wptr_offs as u64 * 4;
    (*ring).wptr_cpu_addr = (*adev).wb.wb.add((*ring).wptr_offs as usize);
    (*ring).trail_fence_gpu_addr = (*adev).wb.gpu_addr + (*ring).trail_fence_offs as u64 * 4;
    (*ring).trail_fence_cpu_addr = (*adev).wb.wb.add((*ring).trail_fence_offs as usize);
    (*ring).cond_exe_gpu_addr = (*adev).wb.gpu_addr + (*ring).cond_exe_offs as u64 * 4;
    (*ring).cond_exe_cpu_addr = (*adev).wb.wb.add((*ring).cond_exe_offs as usize);
    *(*ring).cond_exe_cpu_addr = 1;
    if (*(*ring).funcs).ty != AMDGPU_RING_TYPE_CPER {
        r = amdgpu_fence_driver_start_ring(ring, irq_src, irq_type); if r != 0 { return r; }
        let mut max_ibs_dw = (*(*ring).funcs).emit_frame_size + amdgpu_ring_max_ibs((*(*ring).funcs).ty) * (*(*ring).funcs).emit_ib_size;
        max_ibs_dw = (max_ibs_dw + (*(*ring).funcs).align_mask) & !(*(*ring).funcs).align_mask;
        if max_ibs_dw > max_dw { max_dw = max_ibs_dw; }
        (*ring).ring_size = roundup_pow_of_two(max_dw * 4 * sched_hw_submission);
    } else {
        (*ring).ring_size = roundup_pow_of_two(max_dw * 4); (*ring).count_dw = (({(*ring).ring_size} - 4) >> 2) as i32;
        (*ring).wptr = 0; *(*ring).rptr_cpu_addr = 0;
    }
    (*ring).buf_mask = (*ring).ring_size / 4 - 1;
    (*ring).ptr_mask = if (*(*ring).funcs).support_64bit_ptrs { u64::MAX as u32 } else { (*ring).buf_mask };
    (*ring).cached_rptr = 0; (*ring).max_dw = max_dw; (*ring).hw_prio = hw_prio;
    0
}

pub unsafe fn amdgpu_ring_emit_reg_write_reg_wait_helper(ring: *mut amdgpu_ring, reg0:u32, reg1:u32, ref_:u32, mask:u32) { amdgpu_ring_emit_wreg(ring,reg0,ref_); amdgpu_ring_emit_reg_wait(ring,reg1,mask,mask); }
pub unsafe fn amdgpu_ring_sched_ready(ring:*mut amdgpu_ring)->bool { !ring.is_null() && !(*ring).no_scheduler && drm_sched_wqueue_ready(&mut (*ring).sched) }

pub unsafe fn amdgpu_ring_soft_recovery(ring:*mut amdgpu_ring, vmid:u32, fence:*mut dma_fence)->bool {
    if amdgpu_sriov_vf((*ring).adev) || (*(*ring).funcs).soft_recovery.is_none() || fence.is_null() { return false; }
    let deadline = ktime_add_us(ktime_get(), 10000);
    let mut flags = 0u64;
    dma_fence_lock_irqsave(fence, &mut flags);
    if !dma_fence_is_signaled_locked(fence) { dma_fence_set_error(fence, -ENODATA); }
    dma_fence_unlock_irqrestore(fence, flags);
    while !dma_fence_is_signaled(fence) && ktime_to_ns(ktime_sub(deadline, ktime_get())) > 0 { (*(*ring).funcs).soft_recovery.unwrap()(ring, vmid); }
    let ret = dma_fence_is_signaled(fence); if ret { atomic_inc(&mut (*(*ring).adev).gpu_reset_counter); } ret
}

pub unsafe fn amdgpu_ring_ib_begin(ring:*mut amdgpu_ring) { if (*ring).is_sw_ring { amdgpu_sw_ring_ib_begin(ring); } }
pub unsafe fn amdgpu_ring_ib_end(ring:*mut amdgpu_ring) { if (*ring).is_sw_ring { amdgpu_sw_ring_ib_end(ring); } }
pub unsafe fn amdgpu_ring_ib_on_emit_cntl(ring:*mut amdgpu_ring) { if (*ring).is_sw_ring { amdgpu_sw_ring_ib_mark_offset(ring, AMDGPU_MUX_OFFSET_TYPE_CONTROL); } }
pub unsafe fn amdgpu_ring_ib_on_emit_ce(ring:*mut amdgpu_ring) { if (*ring).is_sw_ring { amdgpu_sw_ring_ib_mark_offset(ring, AMDGPU_MUX_OFFSET_TYPE_CE); } }
pub unsafe fn amdgpu_ring_ib_on_emit_de(ring:*mut amdgpu_ring) { if (*ring).is_sw_ring { amdgpu_sw_ring_ib_mark_offset(ring, AMDGPU_MUX_OFFSET_TYPE_DE); } }

pub unsafe fn amdgpu_ring_reset_helper_begin(ring:*mut amdgpu_ring, guilty:*mut amdgpu_fence) { amdgpu_ring_backup_unprocessed_commands(ring,guilty); }
pub unsafe fn amdgpu_ring_reset_helper_end(ring:*mut amdgpu_ring, guilty:*mut amdgpu_fence)->i32 { let r=amdgpu_ring_test_ring(ring); if r!=0{return r;} amdgpu_ring_set_fence_errors_and_reemit(ring,guilty); 0 }

pub unsafe fn amdgpu_ring_is_reset_type_supported(ring:*mut amdgpu_ring, reset_type:u32)->bool {
    match (*(*ring).funcs).ty {
        AMDGPU_RING_TYPE_GFX => (*(*ring).adev).gfx.gfx_supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_COMPUTE => (*(*ring).adev).gfx.compute_supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_SDMA => (*(*ring).adev).sdma.supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_VCN_DEC | AMDGPU_RING_TYPE_VCN_ENC => (*(*ring).adev).vcn.supported_reset & reset_type != 0,
        AMDGPU_RING_TYPE_VCN_JPEG => (*(*ring).adev).jpeg.supported_reset & reset_type != 0,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
