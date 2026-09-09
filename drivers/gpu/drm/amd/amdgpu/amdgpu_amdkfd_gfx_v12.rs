/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.

unsafe fn lock_srbm(adev: *mut amdgpu_device, mec: u32, pipe: u32, queue: u32, vmid: u32) {
    mutex_lock(unsafe { &mut (*adev).srbm_mutex });
    unsafe { soc24_grbm_select(adev, mec, pipe, queue, vmid); }
}

unsafe fn unlock_srbm(adev: *mut amdgpu_device) {
    soc24_grbm_select(adev, 0, 0, 0, 0);
    mutex_unlock(&mut (*adev).srbm_mutex);
}

unsafe fn acquire_queue(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32) {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, queue_id, 0);
}

unsafe fn release_queue(adev: *mut amdgpu_device) { unlock_srbm(adev); }

unsafe fn init_interrupts_v12(adev: *mut amdgpu_device, pipe_id: u32, _inst: u32) -> i32 {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, 0, 0);
    WREG32_SOC15!(GC, 0, regCPC_INT_CNTL,
        CP_INT_CNTL_RING0__TIME_STAMP_INT_ENABLE_MASK |
        CP_INT_CNTL_RING0__OPCODE_ERROR_INT_ENABLE_MASK);
    unlock_srbm(adev);
    0
}

unsafe fn get_sdma_rlc_reg_offset(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32) -> u32 {
    let mut base = 0;
    match engine_id {
        0 => base = SOC15_REG_OFFSET!(SDMA0, 0, regSDMA0_QUEUE0_RB_CNTL) - regSDMA0_QUEUE0_RB_CNTL,
        1 => base = SOC15_REG_OFFSET!(SDMA1, 0, regSDMA1_QUEUE0_RB_CNTL) - regSDMA0_QUEUE0_RB_CNTL,
        _ => { WARN!(1, "Invalid SDMA engine id %d\n", engine_id); }
    }
    let offset = base + queue_id * (regSDMA0_QUEUE1_RB_CNTL - regSDMA0_QUEUE0_RB_CNTL);
    pr_debug!("RLC register offset for SDMA%d RLC%d: 0x%x\n", engine_id, queue_id, offset);
    offset
}

unsafe fn hqd_dump_v12(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32,
                       dump: *mut *mut [[u32; 2]], n_regs: *mut u32, _inst: u32) -> i32 {
    const HQD_N_REGS: usize = 56;
    let mut i = 0usize;
    *dump = kmalloc_objs!(HQD_N_REGS);
    if (*dump).is_null() { return -ENOMEM; }
    acquire_queue(adev, pipe_id, queue_id);
    let mut reg = SOC15_REG_OFFSET!(GC, 0, regCP_MQD_BASE_ADDR);
    while reg <= SOC15_REG_OFFSET!(GC, 0, regCP_HQD_PQ_WPTR_HI) {
        if WARN_ON_ONCE!(i >= HQD_N_REGS) { break; }
        (**dump)[i][0] = reg << 2;
        (**dump)[i][1] = RREG32!(reg);
        i += 1;
        reg += 1;
    }
    release_queue(adev);
    WARN_ON_ONCE!(i != HQD_N_REGS);
    *n_regs = i as u32;
    0
}

unsafe fn hqd_sdma_dump_v12(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32,
                             dump: *mut *mut [[u32; 2]], n_regs: *mut u32) -> i32 {
    let offset = get_sdma_rlc_reg_offset(adev, engine_id, queue_id);
    let first = regSDMA0_QUEUE0_RB_CNTL;
    let last = regSDMA0_QUEUE0_CONTEXT_STATUS;
    let count = last - first + 1;
    let mut i = 0u32;
    *dump = kmalloc_objs!(count);
    if (*dump).is_null() { return -ENOMEM; }
    let mut reg = first;
    while reg <= last {
        if WARN_ON_ONCE!(i >= count) { break; }
        (**dump)[i as usize][0] = (offset + reg) << 2;
        (**dump)[i as usize][1] = RREG32!(offset + reg);
        i += 1;
        reg += 1;
    }
    WARN_ON_ONCE!(i != count);
    *n_regs = i;
    0
}

unsafe fn wave_control_execute_v12(adev: *mut amdgpu_device, gfx_index_val: u32, sq_cmd: u32, _inst: u32) -> i32 {
    let mut data = 0;
    mutex_lock(&mut (*adev).grbm_idx_mutex);
    WREG32!(SOC15_REG_OFFSET!(GC, 0, regGRBM_GFX_INDEX), gfx_index_val);
    WREG32!(SOC15_REG_OFFSET!(GC, 0, regSQ_CMD), sq_cmd);
    data = REG_SET_FIELD!(data, GRBM_GFX_INDEX, INSTANCE_BROADCAST_WRITES, 1);
    data = REG_SET_FIELD!(data, GRBM_GFX_INDEX, SA_BROADCAST_WRITES, 1);
    data = REG_SET_FIELD!(data, GRBM_GFX_INDEX, SE_BROADCAST_WRITES, 1);
    WREG32!(SOC15_REG_OFFSET!(GC, 0, regGRBM_GFX_INDEX), data);
    mutex_unlock(&mut (*adev).grbm_idx_mutex);
    0
}

unsafe fn kgd_gfx_v12_enable_debug_trap(_adev: *mut amdgpu_device, _restore: bool, _vmid: u32) -> u32 {
    let mut data = 0;
    data = REG_SET_FIELD!(data, SPI_GDBG_PER_VMID_CNTL, TRAP_EN, 1);
    data = REG_SET_FIELD!(data, SPI_GDBG_PER_VMID_CNTL, EXCP_EN, 0);
    REG_SET_FIELD!(data, SPI_GDBG_PER_VMID_CNTL, EXCP_REPLACE, 0)
}

unsafe fn kgd_gfx_v12_disable_debug_trap(adev: *mut amdgpu_device, keep: bool, vmid: u32) -> u32 {
    kgd_gfx_v12_enable_debug_trap(adev, keep, vmid)
}

unsafe fn kgd_gfx_v12_validate_trap_override_request(_adev: *mut amdgpu_device, trap_override: u32, supported: *mut u32) -> i32 {
    *supported &= KFD_DBG_TRAP_MASK_FP_INVALID | KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL |
        KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_FP_OVERFLOW |
        KFD_DBG_TRAP_MASK_FP_UNDERFLOW | KFD_DBG_TRAP_MASK_FP_INEXACT |
        KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH |
        KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION | KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START |
        KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END;
    if trap_override != KFD_DBG_TRAP_OVERRIDE_OR && trap_override != KFD_DBG_TRAP_OVERRIDE_REPLACE { return -EPERM; }
    0
}

unsafe fn trap_mask_map_sw_to_hw(mask: u32) -> u32 {
    let start = if mask & KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START != 0 { 1 } else { 0 };
    let end = if mask & KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END != 0 { 1 } else { 0 };
    let exceptions = mask & (KFD_DBG_TRAP_MASK_FP_INVALID | KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL |
        KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_FP_OVERFLOW | KFD_DBG_TRAP_MASK_FP_UNDERFLOW |
        KFD_DBG_TRAP_MASK_FP_INEXACT | KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO |
        KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH | KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION);
    let mut ret = REG_SET_FIELD!(0, SPI_GDBG_PER_VMID_CNTL, EXCP_EN, exceptions);
    ret = REG_SET_FIELD!(ret, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_START, start);
    REG_SET_FIELD!(ret, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_END, end)
}

unsafe fn trap_mask_map_hw_to_sw(mask: u32) -> u32 {
    let mut ret = REG_GET_FIELD!(mask, SPI_GDBG_PER_VMID_CNTL, EXCP_EN);
    if REG_GET_FIELD!(mask, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_START) != 0 { ret |= KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START; }
    if REG_GET_FIELD!(mask, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_END) != 0 { ret |= KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END; }
    ret
}

unsafe fn kgd_gfx_v12_set_wave_launch_trap_override(_adev: *mut amdgpu_device, _vmid: u32, override_: u32,
        bits: u32, request: u32, previous: *mut u32, previous_cntl: u32) -> u32 {
    *previous = trap_mask_map_hw_to_sw(previous_cntl);
    let mut data = trap_mask_map_sw_to_hw((bits & request) | (*previous & !request));
    data = REG_SET_FIELD!(data, SPI_GDBG_PER_VMID_CNTL, TRAP_EN, 1);
    REG_SET_FIELD!(data, SPI_GDBG_PER_VMID_CNTL, EXCP_REPLACE, override_)
}

unsafe fn kgd_gfx_v12_set_wave_launch_mode(_adev: *mut amdgpu_device, mode: u8, _vmid: u32) -> u32 {
    if mode == 4 { REG_SET_FIELD!(0, SPI_GDBG_PER_VMID_CNTL, STALL_VMID, 1) }
    else { REG_SET_FIELD!(0, SPI_GDBG_PER_VMID_CNTL, LAUNCH_MODE, mode) }
}

const TCP_WATCH_STRIDE: u32 = regTCP_WATCH1_ADDR_H - regTCP_WATCH0_ADDR_H;
unsafe fn kgd_gfx_v12_set_address_watch(_adev: *mut amdgpu_device, address: u64, mask: u32, id: u32, mode: u32, _vmid: u32, _inst: u32) -> u32 {
    let high = (upper_32_bits(address) & 0xffff) as u32;
    let low = lower_32_bits(address);
    let mut cntl = REG_SET_FIELD!(0, TCP_WATCH0_CNTL, MODE, mode);
    cntl = REG_SET_FIELD!(cntl, TCP_WATCH0_CNTL, MASK, mask >> 7);
    cntl = REG_SET_FIELD!(cntl, TCP_WATCH0_CNTL, VALID, 1);
    WREG32_RLC!(SOC15_REG_OFFSET!(GC, 0, regTCP_WATCH0_ADDR_H) + id * TCP_WATCH_STRIDE, high);
    WREG32_RLC!(SOC15_REG_OFFSET!(GC, 0, regTCP_WATCH0_ADDR_L) + id * TCP_WATCH_STRIDE, low);
    cntl
}

unsafe fn kgd_gfx_v12_clear_address_watch(_adev: *mut amdgpu_device, _id: u32) -> u32 { 0 }
unsafe fn kgd_gfx_v12_hqd_sdma_get_doorbell(_adev: *mut amdgpu_device, _engine: i32, _queue: i32) -> u32 { 0 }

unsafe fn lock_spi_csq_mutexes(adev: *mut amdgpu_device) {
    mutex_lock(&mut (*adev).srbm_mutex);
    mutex_lock(&mut (*adev).grbm_idx_mutex);
}
unsafe fn unlock_spi_csq_mutexes(adev: *mut amdgpu_device) {
    mutex_unlock(&mut (*adev).grbm_idx_mutex);
    mutex_unlock(&mut (*adev).srbm_mutex);
}

unsafe fn get_wave_count(adev: *mut amdgpu_device, queue_idx: i32, queue_cnt: *mut kfd_cu_occupancy, _inst: u32) {
    let pipe_idx = queue_idx / (*adev).gfx.mec.num_queue_per_pipe as i32;
    let slot = queue_idx % (*adev).gfx.mec.num_queue_per_pipe as i32;
    soc24_grbm_select(adev, 1, pipe_idx as u32, slot as u32, 0);
    let val = RREG32_SOC15_IP!(GC, SOC15_REG_OFFSET!(GC, 0, regSPI_CSQ_WF_ACTIVE_COUNT_0) + slot as u32);
    let waves = val & SPI_CSQ_WF_ACTIVE_COUNT_0__COUNT_MASK;
    if waves != 0 {
        (*queue_cnt).wave_cnt += waves;
        (*queue_cnt).doorbell_off = (RREG32_SOC15!(GC, 0, regCP_HQD_PQ_DOORBELL_CONTROL) & CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET_MASK) >> CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT;
    }
}

unsafe fn kgd_gfx_v12_get_cu_occupancy(adev: *mut amdgpu_device, occupancy: *mut kfd_cu_occupancy, max_waves: *mut i32, inst: u32) {
    let max_queue = (*adev).gfx.mec.num_pipe_per_mec * (*adev).gfx.mec.num_queue_per_pipe;
    lock_spi_csq_mutexes(adev);
    soc24_grbm_select(adev, 1, 0, 0, 0);
    let mut bitmap = DECLARE_BITMAP!(AMDGPU_MAX_QUEUES);
    bitmap_complement!(bitmap, (*adev).gfx.mec_bitmap[0].queue_bitmap, AMDGPU_MAX_QUEUES);
    for se in 0..(*adev).gfx.config.max_shader_engines {
        amdgpu_gfx_select_se_sh(adev, se, 0, 0xffffffff, inst);
        let map = RREG32_SOC15!(GC, 0, regSPI_CSQ_WF_ACTIVE_STATUS);
        for q in 0..max_queue {
            if !test_bit!(q, bitmap) || (map & (1 << q)) == 0 { continue; }
            get_wave_count(adev, q as i32, occupancy.add(q as usize), inst);
        }
    }
    amdgpu_gfx_select_se_sh(adev, 0xffffffff, 0xffffffff, 0xffffffff, inst);
    soc24_grbm_select(adev, 0, 0, 0, 0);
    unlock_spi_csq_mutexes(adev);
    *max_waves = ((*adev).gfx.cu_info.simd_per_cu * (*adev).gfx.cu_info.max_waves_per_simd) as i32;
}

#[no_mangle]
pub static gfx_v12_kfd2kgd: kfd2kgd_calls = kfd2kgd_calls {
    init_interrupts: Some(init_interrupts_v12), hqd_dump: Some(hqd_dump_v12), hqd_sdma_dump: Some(hqd_sdma_dump_v12),
    wave_control_execute: Some(wave_control_execute_v12), get_atc_vmid_pasid_mapping_info: None,
    enable_debug_trap: Some(kgd_gfx_v12_enable_debug_trap), disable_debug_trap: Some(kgd_gfx_v12_disable_debug_trap),
    validate_trap_override_request: Some(kgd_gfx_v12_validate_trap_override_request),
    set_wave_launch_trap_override: Some(kgd_gfx_v12_set_wave_launch_trap_override), set_wave_launch_mode: Some(kgd_gfx_v12_set_wave_launch_mode),
    set_address_watch: Some(kgd_gfx_v12_set_address_watch), clear_address_watch: Some(kgd_gfx_v12_clear_address_watch),
    hqd_sdma_get_doorbell: Some(kgd_gfx_v12_hqd_sdma_get_doorbell), get_cu_occupancy: Some(kgd_gfx_v12_get_cu_occupancy),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
