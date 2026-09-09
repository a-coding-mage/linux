/* Translated from amdgpu_amdkfd_gfx_v12_1.c. */
/* Kernel and generated-register includes are external dependencies. */

unsafe fn lock_srbm(adev: *mut amdgpu_device, mec: u32, pipe: u32, queue: u32, vmid: u32, inst: u32) {
    mutex_lock(&mut (*adev).srbm_mutex);
    amdgpu_gfx_select_me_pipe_q(adev, mec, pipe, queue, vmid, inst);
}

unsafe fn unlock_srbm(adev: *mut amdgpu_device, inst: u32) {
    amdgpu_gfx_select_me_pipe_q(adev, 0, 0, 0, 0, inst);
    mutex_unlock(&mut (*adev).srbm_mutex);
}

unsafe fn acquire_queue(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, inst: u32) {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, queue_id, 0, inst);
}

unsafe fn release_queue(adev: *mut amdgpu_device, inst: u32) { unlock_srbm(adev, inst); }

unsafe fn init_interrupts_v12_1(adev: *mut amdgpu_device, pipe_id: u32, inst: u32) -> i32 {
    let mec = pipe_id / (*adev).gfx.mec.num_pipe_per_mec + 1;
    let pipe = pipe_id % (*adev).gfx.mec.num_pipe_per_mec;
    lock_srbm(adev, mec, pipe, 0, 0, inst);
    WREG32_SOC15(GC, GET_INST(GC, inst), regCPC_INT_CNTL,
        CP_INT_CNTL_RING0__TIME_STAMP_INT_ENABLE_MASK |
        CP_INT_CNTL_RING0__OPCODE_ERROR_INT_ENABLE_MASK);
    unlock_srbm(adev, inst);
    0
}

unsafe fn get_sdma_rlc_reg_offset(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32) -> u32 {
    let mut base = 0;
    let dev_inst = GET_INST(SDMA0, engine_id);
    match dev_inst % (*adev).sdma.num_inst_per_xcc {
        0 => { base = SOC15_REG_OFFSET(SDMA0, dev_inst / (*adev).sdma.num_inst_per_xcc, regSDMA0_SDMA_QUEUE0_RB_CNTL) - regSDMA0_SDMA_QUEUE0_RB_CNTL; }
        1 => { base = SOC15_REG_OFFSET(SDMA1, dev_inst / (*adev).sdma.num_inst_per_xcc, regSDMA1_SDMA_QUEUE0_RB_CNTL) - regSDMA0_SDMA_QUEUE0_RB_CNTL; }
        _ => { WARN(1, "Invalid SDMA engine id %d\n", engine_id); }
    }
    let off = base + queue_id * (regSDMA0_SDMA_QUEUE1_RB_CNTL - regSDMA0_SDMA_QUEUE0_RB_CNTL);
    pr_debug!("RLC register offset for SDMA%d RLC%d: 0x%x\n", engine_id, queue_id, off);
    off
}

unsafe fn hqd_dump_v12_1(adev: *mut amdgpu_device, pipe_id: u32, queue_id: u32, dump: *mut *mut [[u32; 2]], n_regs: *mut u32, inst: u32) -> i32 {
    const HQD_N_REGS: usize = 56;
    let mut i = 0usize;
    *dump = kmalloc(HQD_N_REGS * 2 * core::mem::size_of::<u32>(), GFP_KERNEL) as *mut [[u32; 2]];
    if (*dump).is_null() { return -ENOMEM; }
    acquire_queue(adev, pipe_id, queue_id, inst);
    let mut reg = SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regCP_MQD_BASE_ADDR);
    let last = SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regCP_HQD_PQ_WPTR_HI);
    while reg <= last {
        if WARN_ON_ONCE(i >= HQD_N_REGS) { break; }
        (**dump)[i][0] = reg << 2;
        (**dump)[i][1] = RREG32(reg);
        i += 1; reg += 1;
    }
    release_queue(adev, inst); WARN_ON_ONCE(i != HQD_N_REGS); *n_regs = i as u32; 0
}

unsafe fn hqd_sdma_dump_v12_1(adev: *mut amdgpu_device, engine_id: u32, queue_id: u32, dump: *mut *mut [[u32; 2]], n_regs: *mut u32) -> i32 {
    let off = get_sdma_rlc_reg_offset(adev, engine_id, queue_id);
    let first = regSDMA0_SDMA_QUEUE0_RB_CNTL;
    let last = regSDMA0_SDMA_QUEUE0_CONTEXT_STATUS;
    let count = last - first + 1;
    let mut i = 0u32;
    *dump = kmalloc(count as usize * 2 * core::mem::size_of::<u32>(), GFP_KERNEL) as *mut [[u32; 2]];
    if (*dump).is_null() { return -ENOMEM; }
    for reg in first..=last { (**dump)[i as usize][0] = (off + reg) << 2; (**dump)[i as usize][1] = RREG32(off + reg); i += 1; }
    WARN_ON_ONCE(i != count); *n_regs = i; 0
}

unsafe fn wave_control_execute_v12_1(adev: *mut amdgpu_device, gfx_index_val: u32, sq_cmd: u32, inst: u32) -> i32 {
    let mut data = 0;
    mutex_lock(&mut (*adev).grbm_idx_mutex);
    WREG32(SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regGRBM_GFX_INDEX), gfx_index_val);
    WREG32(SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regSQ_CMD), sq_cmd);
    data = REG_SET_FIELD(data, GRBM_GFX_INDEX, INSTANCE_BROADCAST_WRITES, 1);
    data = REG_SET_FIELD(data, GRBM_GFX_INDEX, SA_BROADCAST_WRITES, 1);
    data = REG_SET_FIELD(data, GRBM_GFX_INDEX, SE_BROADCAST_WRITES, 1);
    WREG32(SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regGRBM_GFX_INDEX), data);
    mutex_unlock(&mut (*adev).grbm_idx_mutex); 0
}

unsafe fn kgd_gfx_v12_1_enable_debug_trap(_adev: *mut amdgpu_device, _restore: bool, _vmid: u32) -> u32 {
    let mut data = 0; data = REG_SET_FIELD(data, SPI_GDBG_PER_VMID_CNTL, TRAP_EN, 1); data = REG_SET_FIELD(data, SPI_GDBG_PER_VMID_CNTL, EXCP_EN, 0); REG_SET_FIELD(data, SPI_GDBG_PER_VMID_CNTL, EXCP_REPLACE, 0)
}
unsafe fn kgd_gfx_v12_1_disable_debug_trap(adev: *mut amdgpu_device, keep: bool, vmid: u32) -> u32 { kgd_gfx_v12_1_enable_debug_trap(adev, keep, vmid) }

unsafe fn kgd_gfx_v12_1_validate_trap_override_request(_adev: *mut amdgpu_device, trap_override: u32, supported: *mut u32) -> i32 {
    *supported &= KFD_DBG_TRAP_MASK_FP_INVALID | KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL | KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_FP_OVERFLOW | KFD_DBG_TRAP_MASK_FP_UNDERFLOW | KFD_DBG_TRAP_MASK_FP_INEXACT | KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH | KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION | KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START | KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END;
    if trap_override != KFD_DBG_TRAP_OVERRIDE_OR && trap_override != KFD_DBG_TRAP_OVERRIDE_REPLACE { -EPERM } else { 0 }
}

unsafe fn trap_mask_map_sw_to_hw(mask: u32) -> u32 {
    let mut ret = REG_SET_FIELD(0, SPI_GDBG_PER_VMID_CNTL, EXCP_EN, mask & (KFD_DBG_TRAP_MASK_FP_INVALID | KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL | KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_FP_OVERFLOW | KFD_DBG_TRAP_MASK_FP_UNDERFLOW | KFD_DBG_TRAP_MASK_FP_INEXACT | KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO | KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH | KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION));
    ret = REG_SET_FIELD(ret, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_START, if mask & KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START != 0 { 1 } else { 0 });
    REG_SET_FIELD(ret, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_END, if mask & KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END != 0 { 1 } else { 0 })
}
unsafe fn trap_mask_map_hw_to_sw(mask: u32) -> u32 { let mut ret = REG_GET_FIELD(mask, SPI_GDBG_PER_VMID_CNTL, EXCP_EN); if REG_GET_FIELD(mask, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_START) != 0 { ret |= KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START; } if REG_GET_FIELD(mask, SPI_GDBG_PER_VMID_CNTL, TRAP_ON_END) != 0 { ret |= KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END; } ret }

unsafe fn kgd_gfx_v12_1_set_wave_launch_trap_override(_adev: *mut amdgpu_device, _vmid: u32, override_: u32, bits: u32, request: u32, prev: *mut u32, old: u32) -> u32 { *prev = trap_mask_map_hw_to_sw(old); let mut data = trap_mask_map_sw_to_hw((bits & request) | (*prev & !request)); data = REG_SET_FIELD(data, SPI_GDBG_PER_VMID_CNTL, TRAP_EN, 1); REG_SET_FIELD(data, SPI_GDBG_PER_VMID_CNTL, EXCP_REPLACE, override_) }
unsafe fn kgd_gfx_v12_1_set_wave_launch_mode(_adev: *mut amdgpu_device, mode: u8, _vmid: u32) -> u32 { if mode == 4 { REG_SET_FIELD(0, SPI_GDBG_PER_VMID_CNTL, STALL_VMID, 1) } else { REG_SET_FIELD(0, SPI_GDBG_PER_VMID_CNTL, LAUNCH_MODE, mode) } }

const TCP_WATCH_STRIDE: u32 = regTCP_WATCH1_ADDR_H - regTCP_WATCH0_ADDR_H;
unsafe fn kgd_gfx_v12_1_set_address_watch(_adev: *mut amdgpu_device, addr: u64, mask: u32, id: u32, mode: u32, _vmid: u32, inst: u32) -> u32 {
    let high = upper_32_bits(addr) & 0x1ffffff; let low = lower_32_bits(addr); let mut ctl = REG_SET_FIELD(0, TCP_WATCH0_CNTL, MODE, mode);
    ctl = REG_SET_FIELD(ctl, TCP_WATCH0_CNTL, MASK, mask >> 7); ctl = REG_SET_FIELD(ctl, TCP_WATCH0_CNTL, VALID, 1);
    WREG32_XCC(SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regTCP_WATCH0_ADDR_H) + id * TCP_WATCH_STRIDE, high, inst);
    WREG32_XCC(SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regTCP_WATCH0_ADDR_L) + id * TCP_WATCH_STRIDE, low, inst); ctl
}
unsafe fn kgd_gfx_v12_1_clear_address_watch(_adev: *mut amdgpu_device, _id: u32) -> u32 { 0 }
unsafe fn kgd_gfx_v12_1_hqd_sdma_get_doorbell(_adev: *mut amdgpu_device, _engine: i32, _queue: i32) -> u32 { 0 }

unsafe fn lock_spi_csq_mutexes(adev: *mut amdgpu_device) { mutex_lock(&mut (*adev).srbm_mutex); mutex_lock(&mut (*adev).grbm_idx_mutex); }
unsafe fn unlock_spi_csq_mutexes(adev: *mut amdgpu_device) { mutex_unlock(&mut (*adev).grbm_idx_mutex); mutex_unlock(&mut (*adev).srbm_mutex); }

unsafe fn get_wave_count(adev: *mut amdgpu_device, queue_idx: i32, queue_cnt: *mut kfd_cu_occupancy, inst: u32) {
    let pipe = queue_idx / (*adev).gfx.mec.num_queue_per_pipe as i32; let slot = queue_idx % (*adev).gfx.mec.num_queue_per_pipe as i32;
    amdgpu_gfx_select_me_pipe_q(adev, 1, pipe as u32, slot as u32, 0, inst);
    let val = RREG32_SOC15_IP(GC, SOC15_REG_OFFSET(GC, GET_INST(GC, inst), regSPI_CSQ_WF_ACTIVE_COUNT_0) + slot as u32);
    let waves = val & SPI_CSQ_WF_ACTIVE_COUNT_0__COUNT_MASK;
    if waves != 0 { (*queue_cnt).wave_cnt += waves; (*queue_cnt).doorbell_off = (RREG32_SOC15(GC, GET_INST(GC, inst), regCP_HQD_PQ_DOORBELL_CONTROL) & CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET_MASK) >> CP_HQD_PQ_DOORBELL_CONTROL__DOORBELL_OFFSET__SHIFT; }
}

unsafe fn kgd_gfx_v12_1_get_cu_occupancy(adev: *mut amdgpu_device, occ: *mut kfd_cu_occupancy, max_waves: *mut i32, inst: u32) {
    lock_spi_csq_mutexes(adev); amdgpu_gfx_select_me_pipe_q(adev, 1, 0, 0, 0, inst);
    let maxq = (*adev).gfx.mec.num_pipe_per_mec * (*adev).gfx.mec.num_queue_per_pipe;
    let mut bitmap = [0u64; (AMDGPU_MAX_QUEUES as usize + 63) / 64];
    for i in 0..AMDGPU_MAX_QUEUES as usize { if (*adev).gfx.mec_bitmap[0].queue_bitmap[i / 64] & (1 << (i % 64)) == 0 { bitmap[i / 64] |= 1 << (i % 64); } }
    for se in 0..(*adev).gfx.config.max_shader_engines { amdgpu_gfx_select_se_sh(adev, se, 0, 0xffffffff, inst); let map = RREG32_SOC15(GC, GET_INST(GC, inst), regSPI_CSQ_WF_ACTIVE_STATUS); for q in 0..maxq { if bitmap[q as usize / 64] & (1 << (q as usize % 64)) == 0 || map & (1 << q) == 0 { continue; } get_wave_count(adev, q as i32, occ.add(q as usize), inst); } }
    amdgpu_gfx_select_se_sh(adev, 0xffffffff, 0xffffffff, 0xffffffff, inst); amdgpu_gfx_select_me_pipe_q(adev, 0, 0, 0, 0, inst); unlock_spi_csq_mutexes(adev);
    *max_waves = ((*adev).gfx.cu_info.simd_per_cu * (*adev).gfx.cu_info.max_waves_per_simd) as i32;
}

#[repr(C)]
pub struct kfd2kgd_calls { pub init_interrupts: unsafe fn(*mut amdgpu_device,u32,u32)->i32, pub hqd_dump: unsafe fn(*mut amdgpu_device,u32,u32,*mut *mut [[u32;2]],*mut u32,u32)->i32, pub hqd_sdma_dump: unsafe fn(*mut amdgpu_device,u32,u32,*mut *mut [[u32;2]],*mut u32)->i32, pub wave_control_execute: unsafe fn(*mut amdgpu_device,u32,u32,u32)->i32, pub get_atc_vmid_pasid_mapping_info: Option<unsafe fn()>, pub enable_debug_trap: unsafe fn(*mut amdgpu_device,bool,u32)->u32, pub disable_debug_trap: unsafe fn(*mut amdgpu_device,bool,u32)->u32, pub validate_trap_override_request: unsafe fn(*mut amdgpu_device,u32,*mut u32)->i32, pub set_wave_launch_trap_override: unsafe fn(*mut amdgpu_device,u32,u32,u32,u32,*mut u32,u32)->u32, pub set_wave_launch_mode: unsafe fn(*mut amdgpu_device,u8,u32)->u32, pub set_address_watch: unsafe fn(*mut amdgpu_device,u64,u32,u32,u32,u32,u32)->u32, pub clear_address_watch: unsafe fn(*mut amdgpu_device,u32)->u32, pub hqd_sdma_get_doorbell: unsafe fn(*mut amdgpu_device,i32,i32)->u32, pub get_cu_occupancy: unsafe fn(*mut amdgpu_device,*mut kfd_cu_occupancy,*mut i32,u32) }

pub static gfx_v12_1_kfd2kgd: kfd2kgd_calls = kfd2kgd_calls { init_interrupts: init_interrupts_v12_1, hqd_dump: hqd_dump_v12_1, hqd_sdma_dump: hqd_sdma_dump_v12_1, wave_control_execute: wave_control_execute_v12_1, get_atc_vmid_pasid_mapping_info: None, enable_debug_trap: kgd_gfx_v12_1_enable_debug_trap, disable_debug_trap: kgd_gfx_v12_1_disable_debug_trap, validate_trap_override_request: kgd_gfx_v12_1_validate_trap_override_request, set_wave_launch_trap_override: kgd_gfx_v12_1_set_wave_launch_trap_override, set_wave_launch_mode: kgd_gfx_v12_1_set_wave_launch_mode, set_address_watch: kgd_gfx_v12_1_set_address_watch, clear_address_watch: kgd_gfx_v12_1_clear_address_watch, hqd_sdma_get_doorbell: kgd_gfx_v12_1_hqd_sdma_get_doorbell, get_cu_occupancy: kgd_gfx_v12_1_get_cu_occupancy };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
