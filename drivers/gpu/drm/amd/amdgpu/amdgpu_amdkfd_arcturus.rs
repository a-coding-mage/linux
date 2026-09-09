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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding kernel/amdgpu translation unit:
// linux/module.h, linux/uaccess.h, linux/firmware.h, amdgpu headers, SDMA,
// GC, SOC15, and v9 structure headers.

const HQD_N_REGS: usize = 56;

#[inline]
unsafe fn get_sdma_mqd(mqd: *mut core::ffi::c_void) -> *mut v9_sdma_mqd {
    mqd as *mut v9_sdma_mqd
}

unsafe fn get_sdma_rlc_reg_offset(
    adev: *mut amdgpu_device,
    engine_id: u32,
    queue_id: u32,
) -> u32 {
    let sdma_engine_reg_base = match engine_id {
        0 => SOC15_REG_OFFSET(SDMA0, 0, mmSDMA0_RLC0_RB_CNTL) - mmSDMA0_RLC0_RB_CNTL,
        1 => SOC15_REG_OFFSET(SDMA1, 0, mmSDMA1_RLC0_RB_CNTL) - mmSDMA1_RLC0_RB_CNTL,
        2 => SOC15_REG_OFFSET(SDMA2, 0, mmSDMA2_RLC0_RB_CNTL) - mmSDMA2_RLC0_RB_CNTL,
        3 => SOC15_REG_OFFSET(SDMA3, 0, mmSDMA3_RLC0_RB_CNTL) - mmSDMA3_RLC0_RB_CNTL,
        4 => SOC15_REG_OFFSET(SDMA4, 0, mmSDMA4_RLC0_RB_CNTL) - mmSDMA4_RLC0_RB_CNTL,
        5 => SOC15_REG_OFFSET(SDMA5, 0, mmSDMA5_RLC0_RB_CNTL) - mmSDMA5_RLC0_RB_CNTL,
        6 => SOC15_REG_OFFSET(SDMA6, 0, mmSDMA6_RLC0_RB_CNTL) - mmSDMA6_RLC0_RB_CNTL,
        7 => SOC15_REG_OFFSET(SDMA7, 0, mmSDMA7_RLC0_RB_CNTL) - mmSDMA7_RLC0_RB_CNTL,
        _ => {
            dev_warn((*adev).dev, "Invalid sdma engine id (%d), using engine id 0\n", engine_id);
            SOC15_REG_OFFSET(SDMA0, 0, mmSDMA0_RLC0_RB_CNTL) - mmSDMA0_RLC0_RB_CNTL
        }
    };
    let sdma_rlc_reg_offset = sdma_engine_reg_base
        + queue_id * (mmSDMA0_RLC1_RB_CNTL - mmSDMA0_RLC0_RB_CNTL);
    pr_debug!("RLC register offset for SDMA%d RLC%d: 0x%x\n", engine_id, queue_id, sdma_rlc_reg_offset);
    sdma_rlc_reg_offset
}

pub unsafe fn kgd_arcturus_hqd_sdma_load(
    adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void,
    wptr: *mut u32, mm: *mut mm_struct,
) -> i32 {
    let m = &mut *get_sdma_mqd(mqd);
    let off = get_sdma_rlc_reg_offset(adev, m.sdma_engine_id, m.sdma_queue_id);
    WREG32(off + mmSDMA0_RLC0_RB_CNTL, m.sdmax_rlcx_rb_cntl & !SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK);
    let end_jiffies = msecs_to_jiffies(2000) + jiffies;
    loop {
        let data = RREG32(off + mmSDMA0_RLC0_CONTEXT_STATUS);
        if data & SDMA0_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; }
        if time_after(jiffies, end_jiffies) { pr_err!("SDMA RLC not idle in kgd_arcturus_hqd_sdma_load\n"); return -ETIME; }
        usleep_range(500, 1000);
    }
    WREG32(off + mmSDMA0_RLC0_DOORBELL_OFFSET, m.sdmax_rlcx_doorbell_offset);
    let data = REG_SET_FIELD(m.sdmax_rlcx_doorbell, SDMA0_RLC0_DOORBELL, ENABLE, 1);
    WREG32(off + mmSDMA0_RLC0_DOORBELL, data);
    WREG32(off + mmSDMA0_RLC0_RB_RPTR, m.sdmax_rlcx_rb_rptr);
    WREG32(off + mmSDMA0_RLC0_RB_RPTR_HI, m.sdmax_rlcx_rb_rptr_hi);
    WREG32(off + mmSDMA0_RLC0_MINOR_PTR_UPDATE, 1);
    let mut data64 = 0u64;
    if read_user_wptr(mm, wptr as *mut u64, &mut data64) {
        WREG32(off + mmSDMA0_RLC0_RB_WPTR, lower_32_bits(data64));
        WREG32(off + mmSDMA0_RLC0_RB_WPTR_HI, upper_32_bits(data64));
    } else {
        WREG32(off + mmSDMA0_RLC0_RB_WPTR, m.sdmax_rlcx_rb_rptr);
        WREG32(off + mmSDMA0_RLC0_RB_WPTR_HI, m.sdmax_rlcx_rb_rptr_hi);
    }
    WREG32(off + mmSDMA0_RLC0_MINOR_PTR_UPDATE, 0);
    WREG32(off + mmSDMA0_RLC0_RB_BASE, m.sdmax_rlcx_rb_base);
    WREG32(off + mmSDMA0_RLC0_RB_BASE_HI, m.sdmax_rlcx_rb_base_hi);
    WREG32(off + mmSDMA0_RLC0_RB_RPTR_ADDR_LO, m.sdmax_rlcx_rb_rptr_addr_lo);
    WREG32(off + mmSDMA0_RLC0_RB_RPTR_ADDR_HI, m.sdmax_rlcx_rb_rptr_addr_hi);
    let data = REG_SET_FIELD(m.sdmax_rlcx_rb_cntl, SDMA0_RLC0_RB_CNTL, RB_ENABLE, 1);
    WREG32(off + mmSDMA0_RLC0_RB_CNTL, data);
    0
}

pub unsafe fn kgd_arcturus_hqd_sdma_dump(
    adev: *mut amdgpu_device, engine_id: u32, queue_id: u32,
    dump: *mut *mut [u32; 2], n_regs: *mut u32,
) -> i32 {
    let off = get_sdma_rlc_reg_offset(adev, engine_id, queue_id);
    let count = 19 + 6 + 7 + 10;
    *dump = kmalloc_objs::<[u32; 2]>(count);
    if (*dump).is_null() { return -ENOMEM; }
    let mut i = 0usize;
    let mut reg = mmSDMA0_RLC0_RB_CNTL;
    while reg <= mmSDMA0_RLC0_DOORBELL { if i >= count { break; } (*dump.add(0))[i] = [(off + reg) << 2, RREG32(off + reg)]; i += 1; reg += 1; }
    reg = mmSDMA0_RLC0_STATUS; while reg <= mmSDMA0_RLC0_CSA_ADDR_HI { if i >= count { break; } (*dump)[i] = [(off + reg) << 2, RREG32(off + reg)]; i += 1; reg += 1; }
    reg = mmSDMA0_RLC0_IB_SUB_REMAIN; while reg <= mmSDMA0_RLC0_MINOR_PTR_UPDATE { if i >= count { break; } (*dump)[i] = [(off + reg) << 2, RREG32(off + reg)]; i += 1; reg += 1; }
    reg = mmSDMA0_RLC0_MIDCMD_DATA0; while reg <= mmSDMA0_RLC0_MIDCMD_CNTL { if i >= count { break; } (*dump)[i] = [(off + reg) << 2, RREG32(off + reg)]; i += 1; reg += 1; }
    WARN_ON_ONCE(i != count); *n_regs = i as u32; 0
}

pub unsafe fn kgd_arcturus_hqd_sdma_is_occupied(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void) -> bool {
    let m = &*get_sdma_mqd(mqd);
    let off = get_sdma_rlc_reg_offset(adev, m.sdma_engine_id, m.sdma_queue_id);
    RREG32(off + mmSDMA0_RLC0_RB_CNTL) & SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK != 0
}

pub unsafe fn kgd_arcturus_hqd_sdma_destroy(adev: *mut amdgpu_device, mqd: *mut core::ffi::c_void, utimeout: u32) -> i32 {
    let m = &mut *get_sdma_mqd(mqd);
    let off = get_sdma_rlc_reg_offset(adev, m.sdma_engine_id, m.sdma_queue_id);
    let end_jiffies = (utimeout * HZ / 1000) + jiffies;
    let mut temp = RREG32(off + mmSDMA0_RLC0_RB_CNTL) & !SDMA0_RLC0_RB_ENABLE_MASK;
    WREG32(off + mmSDMA0_RLC0_RB_CNTL, temp);
    loop { temp = RREG32(off + mmSDMA0_RLC0_CONTEXT_STATUS); if temp & SDMA0_RLC0_CONTEXT_STATUS__IDLE_MASK != 0 { break; } if time_after(jiffies, end_jiffies) { pr_err!("SDMA RLC not idle in kgd_arcturus_hqd_sdma_destroy\n"); return -ETIME; } usleep_range(500, 1000); }
    WREG32(off + mmSDMA0_RLC0_DOORBELL, 0);
    WREG32(off + mmSDMA0_RLC0_RB_CNTL, RREG32(off + mmSDMA0_RLC0_RB_CNTL) | SDMA0_RLC0_RB_CNTL__RB_ENABLE_MASK);
    m.sdmax_rlcx_rb_rptr = RREG32(off + mmSDMA0_RLC0_RB_RPTR);
    m.sdmax_rlcx_rb_rptr_hi = RREG32(off + mmSDMA0_RLC0_RB_RPTR_HI);
    0
}

unsafe fn suspend_resume_compute_scheduler(adev: *mut amdgpu_device, suspend: bool) -> i32 {
    let mut r = 0;
    for i in 0..(*adev).gfx.num_compute_rings { let ring = &mut (*adev).gfx.compute_ring[i as usize]; if !amdgpu_ring_sched_ready(ring) { continue; } if suspend { drm_sched_stop(&mut ring.sched, core::ptr::null_mut()); r = amdgpu_fence_wait_empty(ring); if r != 0 { break; } } else { drm_sched_start(&mut ring.sched, 0); } }
    if !suspend || r != 0 { return r; }
    amdgpu_device_ip_wait_for_idle(adev, AMD_IP_BLOCK_TYPE_GFX)
}

unsafe fn set_barrier_auto_waitcnt(adev: *mut amdgpu_device, enable_waitcnt: bool) {
    WRITE_ONCE!((*adev).barrier_has_auto_waitcnt, enable_waitcnt);
    if !down_read_trylock((*adev).reset_domain.sem) { return; }
    amdgpu_amdkfd_suspend(adev, true);
    if suspend_resume_compute_scheduler(adev, true) == 0 { let mut data = RREG32(SOC15_REG_OFFSET(GC, 0, mmSQ_CONFIG)); data = REG_SET_FIELD(data, SQ_CONFIG, DISABLE_BARRIER_WAITCNT, !enable_waitcnt); WREG32(SOC15_REG_OFFSET(GC, 0, mmSQ_CONFIG), data); }
    suspend_resume_compute_scheduler(adev, false); amdgpu_amdkfd_resume(adev, true); up_read((*adev).reset_domain.sem);
}

unsafe fn kgd_arcturus_enable_debug_trap(adev: *mut amdgpu_device, _restore_dbg_registers: bool, vmid: u32) -> u32 { mutex_lock(&mut (*adev).grbm_idx_mutex); kgd_gfx_v9_set_wave_launch_stall(adev, vmid, true); set_barrier_auto_waitcnt(adev, true); WREG32(SOC15_REG_OFFSET(GC, 0, mmSPI_GDBG_TRAP_MASK), 0); kgd_gfx_v9_set_wave_launch_stall(adev, vmid, false); mutex_unlock(&mut (*adev).grbm_idx_mutex); 0 }
unsafe fn kgd_arcturus_disable_debug_trap(adev: *mut amdgpu_device, _keep_trap_enabled: bool, vmid: u32) -> u32 { mutex_lock(&mut (*adev).grbm_idx_mutex); kgd_gfx_v9_set_wave_launch_stall(adev, vmid, true); set_barrier_auto_waitcnt(adev, false); WREG32(SOC15_REG_OFFSET(GC, 0, mmSPI_GDBG_TRAP_MASK), 0); kgd_gfx_v9_set_wave_launch_stall(adev, vmid, false); mutex_unlock(&mut (*adev).grbm_idx_mutex); 0 }

pub static arcturus_kfd2kgd: kfd2kgd_calls = kfd2kgd_calls {
    program_sh_mem_settings: kgd_gfx_v9_program_sh_mem_settings, set_pasid_vmid_mapping: kgd_gfx_v9_set_pasid_vmid_mapping, init_interrupts: kgd_gfx_v9_init_interrupts, hqd_load: kgd_gfx_v9_hqd_load, hiq_mqd_load: kgd_gfx_v9_hiq_mqd_load, hqd_sdma_load: kgd_arcturus_hqd_sdma_load, hqd_dump: kgd_gfx_v9_hqd_dump, hqd_sdma_dump: kgd_arcturus_hqd_sdma_dump, hqd_is_occupied: kgd_gfx_v9_hqd_is_occupied, hqd_sdma_is_occupied: kgd_arcturus_hqd_sdma_is_occupied, hqd_destroy: kgd_gfx_v9_hqd_destroy, hqd_sdma_destroy: kgd_arcturus_hqd_sdma_destroy, wave_control_execute: kgd_gfx_v9_wave_control_execute, get_atc_vmid_pasid_mapping_info: kgd_gfx_v9_get_atc_vmid_pasid_mapping_info, set_vm_context_page_table_base: kgd_gfx_v9_set_vm_context_page_table_base, enable_debug_trap: kgd_arcturus_enable_debug_trap, disable_debug_trap: kgd_arcturus_disable_debug_trap, validate_trap_override_request: kgd_gfx_v9_validate_trap_override_request, set_wave_launch_trap_override: kgd_gfx_v9_set_wave_launch_trap_override, set_wave_launch_mode: kgd_gfx_v9_set_wave_launch_mode, set_address_watch: kgd_gfx_v9_set_address_watch, clear_address_watch: kgd_gfx_v9_clear_address_watch, get_iq_wait_times: kgd_gfx_v9_get_iq_wait_times, build_dequeue_wait_counts_packet_info: kgd_gfx_v9_build_dequeue_wait_counts_packet_info, get_cu_occupancy: kgd_gfx_v9_get_cu_occupancy, program_trap_handler_settings: kgd_gfx_v9_program_trap_handler_settings, hqd_get_pq_addr: kgd_gfx_v9_hqd_get_pq_addr, hqd_reset: kgd_gfx_v9_hqd_reset, hqd_sdma_get_doorbell: kgd_gfx_v9_hqd_sdma_get_doorbell,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
