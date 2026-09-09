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
 */

// C headers and build-provided register/macro definitions are external dependencies.

#[repr(C)]
#[derive(Copy, Clone)]
enum HqdDequeueRequestType {
    NO_ACTION = 0,
    DRAIN_PIPE,
    RESET_WAVES,
}

const MAX_TRAPID: u32 = 8;
const MAX_WATCH_ADDRESSES: u32 = 4;

unsafe fn lock_srbm(adev: *mut amdgpu_device, mec: u32, pipe: u32, queue: u32, vmid: u32) {
    let value = PIPEID(pipe) | MEID(mec) | VMID(vmid) | QUEUEID(queue);
    mutex_lock(&mut (*adev).srbm_mutex);
    WREG32(adev, mmSRBM_GFX_CNTL, value);
}

unsafe fn unlock_srbm(adev: *mut amdgpu_device) {
    WREG32(adev, mmSRBM_GFX_CNTL, 0);
    mutex_unlock(&mut (*adev).srbm_mutex);
}

unsafe fn acquire_queue(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32) {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, queue_id, 0);
}

unsafe fn release_queue(adev: *mut amdgpu_device) { unlock_srbm(adev); }

unsafe fn kgd_program_sh_mem_settings(adev: *mut amdgpu_device, vmid: u32, sh_mem_config: u32,
    sh_mem_ape1_base: u32, sh_mem_ape1_limit: u32, sh_mem_bases: u32, _inst: u32) {
    lock_srbm(adev, 0, 0, 0, vmid);
    WREG32(adev, mmSH_MEM_CONFIG, sh_mem_config);
    WREG32(adev, mmSH_MEM_APE1_BASE, sh_mem_ape1_base);
    WREG32(adev, mmSH_MEM_APE1_LIMIT, sh_mem_ape1_limit);
    WREG32(adev, mmSH_MEM_BASES, sh_mem_bases);
    unlock_srbm(adev);
}

unsafe fn kgd_set_pasid_vmid_mapping(adev: *mut amdgpu_device, pasid: u32, vmid: u32, _inst: u32) -> i32 {
    let pasid_mapping = if pasid == 0 { 0 } else { pasid | ATC_VMID0_PASID_MAPPING__VALID_MASK };
    WREG32(adev, mmATC_VMID0_PASID_MAPPING + vmid, pasid_mapping);
    while RREG32(adev, mmATC_VMID_PASID_MAPPING_UPDATE_STATUS) & (1u32 << vmid) == 0 { cpu_relax(); }
    WREG32(adev, mmATC_VMID_PASID_MAPPING_UPDATE_STATUS, 1u32 << vmid);
    WREG32(adev, mmIH_VMID_0_LUT + vmid, pasid_mapping);
    0
}

unsafe fn kgd_init_interrupts(adev: *mut amdgpu_device, pipe_id: u32, _inst: u32) -> i32 {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, 0, 0);
    WREG32(adev, mmCPC_INT_CNTL, CP_INT_CNTL_RING0__TIME_STAMP_INT_ENABLE_MASK |
        CP_INT_CNTL_RING0__OPCODE_ERROR_INT_ENABLE_MASK);
    unlock_srbm(adev);
    0
}

unsafe fn get_sdma_rlc_reg_offset(m: *const cik_sdma_rlc_registers) -> u32 {
    let retval = (*m).sdma_engine_id * SDMA1_REGISTER_OFFSET + (*m).sdma_queue_id * KFD_CIK_SDMA_QUEUE_OFFSET;
    pr_debug!("RLC register offset for SDMA{} RLC{}: 0x{:x}\n", (*m).sdma_engine_id, (*m).sdma_queue_id, retval);
    retval
}

unsafe fn get_mqd(mqd: *mut core::ffi::c_void) -> *mut cik_mqd { mqd as *mut cik_mqd }
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut cik_sdma_rlc_registers { mqd as *mut cik_sdma_rlc_registers }

unsafe fn kgd_hqd_load(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, pipe_id: u32, queue_id: u32,
    wptr: *mut u32, wptr_shift: u32, wptr_mask: u32, mm: *mut mm_struct, _inst: u32) -> i32 {
    let m = get_mqd(mqd);
    acquire_queue(adev, pipe_id, queue_id);
    let mqd_hqd = &(*m).cp_mqd_base_addr_lo as *const u32;
    let mut reg = mmCP_MQD_BASE_ADDR;
    while reg <= mmCP_MQD_CONTROL { WREG32(adev, reg, *mqd_hqd.add((reg - mmCP_MQD_BASE_ADDR) as usize)); reg += 1; }
    let data = REG_SET_FIELD((*m).cp_hqd_pq_doorbell_control, CP_HQD_PQ_DOORBELL_CONTROL, DOORBELL_EN, 1);
    WREG32(adev, mmCP_HQD_PQ_DOORBELL_CONTROL, data);
    release_queue(adev);
    let mut wptr_val = 0u32;
    let valid_wptr = read_user_wptr(mm, wptr, &mut wptr_val);
    acquire_queue(adev, pipe_id, queue_id);
    if valid_wptr { WREG32(adev, mmCP_HQD_PQ_WPTR, (wptr_val << wptr_shift) & wptr_mask); }
    let data = REG_SET_FIELD((*m).cp_hqd_active, CP_HQD_ACTIVE, ACTIVE, 1);
    WREG32(adev, mmCP_HQD_ACTIVE, data);
    release_queue(adev);
    0
}

unsafe fn kgd_hqd_dump(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, dump: *mut *mut [[u32; 2]], n_regs: *mut u32, _inst: u32) -> i32 {
    const HQD_N_REGS: usize = 39;
    let mut i = 0usize;
    *dump = kmalloc_objs::<[[u32; 2]]>(HQD_N_REGS);
    if (*dump).is_null() { return -ENOMEM; }
    acquire_queue(adev, pipe_id, queue_id);
    let mut reg = mmCOMPUTE_STATIC_THREAD_MGMT_SE0;
    while reg <= mmCOMPUTE_STATIC_THREAD_MGMT_SE3 { (*dump).add(i).write([[reg << 2, RREG32(adev, reg)]]); i += 1; reg += 1; }
    reg = mmCP_MQD_BASE_ADDR;
    while reg <= mmCP_MQD_CONTROL { (*dump).add(i).write([[reg << 2, RREG32(adev, reg)]]); i += 1; reg += 1; }
    release_queue(adev);
    WARN_ON_ONCE(i != HQD_N_REGS);
    *n_regs = i as u32;
    0
}

unsafe fn kgd_hqd_sdma_load(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, wptr: *mut u32, mm: *mut mm_struct) -> i32 {
    let m = get_sdma_mqd(mqd);
    let off = get_sdma_rlc_reg_offset(m);
    WREG32(adev, off + mmSDMA0_RLC0_RB_CNTL, (*m).sdma_rlc_rb_cntl & !SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK);
    let end_jiffies = msecs_to_jiffies(2000) + jiffies;
    loop {
        let data = RREG32(adev, off + mmSDMA0_RLC0_CONTEXT_STATUS);
        if data & SDMA0_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; }
        if time_after(jiffies, end_jiffies) { pr_err!("SDMA RLC not idle in {}\n", __func__); return -ETIME; }
        usleep_range(500, 1000);
    }
    let data = REG_SET_FIELD((*m).sdma_rlc_doorbell, SDMA0_RLC0_DOORBELL, ENABLE, 1);
    WREG32(adev, off + mmSDMA0_RLC0_DOORBELL, data);
    WREG32(adev, off + mmSDMA0_RLC0_RB_RPTR, (*m).sdma_rlc_rb_rptr);
    let mut user_wptr = 0u32;
    if read_user_wptr(mm, wptr, &mut user_wptr) { WREG32(adev, off + mmSDMA0_RLC0_RB_WPTR, user_wptr); }
    else { WREG32(adev, off + mmSDMA0_RLC0_RB_WPTR, (*m).sdma_rlc_rb_rptr); }
    WREG32(adev, off + mmSDMA0_RLC0_VIRTUAL_ADDR, (*m).sdma_rlc_virtual_addr);
    WREG32(adev, off + mmSDMA0_RLC0_RB_BASE, (*m).sdma_rlc_rb_base);
    WREG32(adev, off + mmSDMA0_RLC0_RB_BASE_HI, (*m).sdma_rlc_rb_base_hi);
    WREG32(adev, off + mmSDMA0_RLC0_RB_RPTR_ADDR_LO, (*m).sdma_rlc_rb_rptr_addr_lo);
    WREG32(adev, off + mmSDMA0_RLC0_RB_RPTR_ADDR_HI, (*m).sdma_rlc_rb_rptr_addr_hi);
    let data = REG_SET_FIELD((*m).sdma_rlc_rb_cntl, SDMA0_RLC0_RB_CNTL, RB_ENABLE, 1);
    WREG32(adev, off + mmSDMA0_RLC0_RB_CNTL, data);
    0
}

unsafe fn kgd_hqd_sdma_dump(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32, dump: *mut *mut [[u32; 2]], n_regs: *mut u32) -> i32 {
    const HQD_N_REGS: usize = 23;
    let off = engine_id * SDMA1_REGISTER_OFFSET + queue_id * KFD_CIK_SDMA_QUEUE_OFFSET;
    let mut i = 0usize;
    *dump = kmalloc_objs::<[[u32; 2]]>(HQD_N_REGS);
    if (*dump).is_null() { return -ENOMEM; }
    let mut reg = mmSDMA0_RLC0_RB_CNTL;
    while reg <= mmSDMA0_RLC0_DOORBELL { (*dump).add(i).write([[off + reg << 2, RREG32(adev, off + reg)]]); i += 1; reg += 1; }
    reg = mmSDMA0_RLC0_VIRTUAL_ADDR;
    while reg <= mmSDMA0_RLC0_WATERMARK { (*dump).add(i).write([[off + reg << 2, RREG32(adev, off + reg)]]); i += 1; reg += 1; }
    WARN_ON_ONCE(i != HQD_N_REGS);
    *n_regs = i as u32;
    0
}

unsafe fn kgd_hqd_is_occupied(adev: *mut amdgpu_device, queue_address: u64, pipe_id: u32, queue_id: u32, _inst: u32) -> bool {
    acquire_queue(adev, pipe_id, queue_id);
    let act = RREG32(adev, mmCP_HQD_ACTIVE);
    let mut result = false;
    if act != 0 {
        let low = (queue_address >> 8) as u32;
        let high = (queue_address >> 40) as u32;
        result = low == RREG32(adev, mmCP_HQD_PQ_BASE) && high == RREG32(adev, mmCP_HQD_PQ_BASE_HI);
    }
    release_queue(adev);
    result
}

unsafe fn kgd_hqd_sdma_is_occupied(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void) -> bool {
    let m = get_sdma_mqd(mqd);
    let value = RREG32(adev, get_sdma_rlc_reg_offset(m) + mmSDMA0_RLC0_RB_CNTL);
    value & SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK != 0
}

unsafe fn kgd_hqd_destroy(adev: *mut amdgpu_device, _mqd: *mut core::ffi::c_void, reset_type: kfd_preempt_type, utimeout: u32, pipe_id: u32, queue_id: u32, _inst: u32) -> i32 {
    if amdgpu_in_reset(adev) { return -EIO; }
    acquire_queue(adev, pipe_id, queue_id);
    WREG32(adev, mmCP_HQD_PQ_DOORBELL_CONTROL, 0);
    let request = match reset_type { KFD_PREEMPT_TYPE_WAVEFRONT_RESET => RESET_WAVES, _ => DRAIN_PIPE };
    let mut flags = 0ul;
    local_irq_save(&mut flags); preempt_disable();
    let mut retry = 5000;
    loop {
        let temp = RREG32(adev, mmCP_HQD_IQ_TIMER);
        if REG_GET_FIELD(temp, CP_HQD_IQ_TIMER, PROCESSING_IQ) != 0 { pr_debug!("HW is processing IQ\n"); }
        else if REG_GET_FIELD(temp, CP_HQD_IQ_TIMER, ACTIVE) == 0 || REG_GET_FIELD(temp, CP_HQD_IQ_TIMER, RETRY_TYPE) == 3 || REG_GET_FIELD(temp, CP_HQD_IQ_TIMER, WAIT_TIME) >= 10 { break; }
        else { pr_debug!("IQ timer is active\n"); }
        if retry == 0 { pr_err!("CP HQD IQ timer status time out\n"); break; }
        ndelay(100); retry -= 1;
    }
    retry = 1000;
    loop {
        let temp = RREG32(adev, mmCP_HQD_DEQUEUE_REQUEST);
        if temp & CP_HQD_DEQUEUE_REQUEST__IQ_REQ_PEND_MASK == 0 { break; }
        pr_debug!("Dequeue request is pending\n");
        if retry == 0 { pr_err!("CP HQD dequeue request time out\n"); break; }
        ndelay(100); retry -= 1;
    }
    local_irq_restore(flags); preempt_enable();
    WREG32(adev, mmCP_HQD_DEQUEUE_REQUEST, request as u32);
    let end_jiffies = utimeout * HZ / 1000 + jiffies;
    loop {
        if RREG32(adev, mmCP_HQD_ACTIVE) & CP_HQD_ACTIVE__ACTIVE_MASK == 0 { break; }
        if time_after(jiffies, end_jiffies) { pr_err!("cp queue preemption time out\n"); release_queue(adev); return -ETIME; }
        usleep_range(500, 1000);
    }
    release_queue(adev); 0
}

unsafe fn kgd_hqd_sdma_destroy(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, utimeout: u32) -> i32 {
    let m = get_sdma_mqd(mqd); let off = get_sdma_rlc_reg_offset(m);
    let end_jiffies = utimeout * HZ / 1000 + jiffies;
    let mut temp = RREG32(adev, off + mmSDMA0_RLC0_RB_CNTL) & !SDMA0_RLC0_RB_ENABLE_MASK;
    WREG32(adev, off + mmSDMA0_RLC0_RB_CNTL, temp);
    loop {
        temp = RREG32(adev, off + mmSDMA0_RLC0_CONTEXT_STATUS);
        if temp & SDMA0_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; }
        if time_after(jiffies, end_jiffies) { pr_err!("SDMA RLC not idle in {}\n", __func__); return -ETIME; }
        usleep_range(500, 1000);
    }
    WREG32(adev, off + mmSDMA0_RLC0_DOORBELL, 0);
    WREG32(adev, off + mmSDMA0_RLC0_RB_CNTL, RREG32(adev, off + mmSDMA0_RLC0_RB_CNTL) | SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK);
    (*m).sdma_rlc_rb_rptr = RREG32(adev, off + mmSDMA0_RLC0_RB_RPTR);
    0
}

unsafe fn kgd_wave_control_execute(adev: *mut amdgpu_device, gfx_index_val: u32, sq_cmd: u32, _inst: u32) -> i32 {
    mutex_lock(&mut (*adev).grbm_idx_mutex);
    WREG32(adev, mmGRBM_GFX_INDEX, gfx_index_val); WREG32(adev, mmSQ_CMD, sq_cmd);
    WREG32(adev, mmGRBM_GFX_INDEX, GRBM_GFX_INDEX__INSTANCE_BROADCAST_WRITES_MASK | GRBM_GFX_INDEX__SH_BROADCAST_WRITES_MASK | GRBM_GFX_INDEX__SE_BROADCAST_WRITES_MASK);
    mutex_unlock(&mut (*adev).grbm_idx_mutex); 0
}

unsafe fn get_atc_vmid_pasid_mapping_info(adev: *mut amdgpu_device, vmid: u8, p_pasid: *mut u16) -> bool {
    let value = RREG32(adev, mmATC_VMID0_PASID_MAPPING + vmid as u32);
    *p_pasid = (value & ATC_VMID0_PASID_MAPPING__PASID_MASK) as u16;
    value & ATC_VMID0_PASID_MAPPING__VALID_MASK != 0
}

unsafe fn set_scratch_backing_va(adev: *mut amdgpu_device, va: u64, vmid: u32) { lock_srbm(adev, 0, 0, 0, vmid); WREG32(adev, mmSH_HIDDEN_PRIVATE_BASE_VMID, va); unlock_srbm(adev); }

unsafe fn set_vm_context_page_table_base(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    if !amdgpu_amdkfd_is_kfd_vmid(adev, vmid) { pr_err!("trying to set page table base for wrong VMID\n"); return; }
    WREG32(adev, mmVM_CONTEXT8_PAGE_TABLE_BASE_ADDR + vmid - 8, page_table_base as u32);
}

/** read vmid from register (CIK). */
unsafe fn read_vmid_from_vmfault_reg(adev: *mut amdgpu_device) -> u32 {
    REG_GET_FIELD(RREG32(adev, mmVM_CONTEXT1_PROTECTION_FAULT_STATUS), VM_CONTEXT1_PROTECTION_FAULT_STATUS, VMID)
}

unsafe fn kgd_hqd_sdma_get_doorbell(_adev: *mut amdgpu_device, _engine: i32, _queue: i32) -> u32 { 0 }

#[no_mangle]
pub static gfx_v7_kfd2kgd: kfd2kgd_calls = kfd2kgd_calls {
    program_sh_mem_settings: Some(kgd_program_sh_mem_settings),
    set_pasid_vmid_mapping: Some(kgd_set_pasid_vmid_mapping),
    init_interrupts: Some(kgd_init_interrupts),
    hqd_load: Some(kgd_hqd_load),
    hqd_sdma_load: Some(kgd_hqd_sdma_load),
    hqd_dump: Some(kgd_hqd_dump),
    hqd_sdma_dump: Some(kgd_hqd_sdma_dump),
    hqd_is_occupied: Some(kgd_hqd_is_occupied),
    hqd_sdma_is_occupied: Some(kgd_hqd_sdma_is_occupied),
    hqd_destroy: Some(kgd_hqd_destroy),
    hqd_sdma_destroy: Some(kgd_hqd_sdma_destroy),
    wave_control_execute: Some(kgd_wave_control_execute),
    get_atc_vmid_pasid_mapping_info: Some(get_atc_vmid_pasid_mapping_info),
    set_scratch_backing_va: Some(set_scratch_backing_va),
    set_vm_context_page_table_base: Some(set_vm_context_page_table_base),
    read_vmid_from_vmfault_reg: Some(read_vmid_from_vmfault_reg),
    hqd_sdma_get_doorbell: Some(kgd_hqd_sdma_get_doorbell),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
