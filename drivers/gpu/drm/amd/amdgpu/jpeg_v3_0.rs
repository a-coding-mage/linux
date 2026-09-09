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

// External kernel headers, register definitions, macros, types, and functions
// are supplied by the surrounding translation unit.

const MMUVD_JPEG_PITCH_INTERNAL_OFFSET: u32 = 0x401f;

static mut jpeg_reg_list_3_0: [amdgpu_hwip_reg_entry; 13] = [
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_POWER_STATUS),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_INT_STAT),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_RPTR),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_WPTR),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_CNTL),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_RB_SIZE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JRBC_STATUS),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_ADDR_MODE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_GFX10_ADDR_CONFIG),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_Y_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmJPEG_DEC_UV_GFX10_TILING_SURFACE),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_PITCH),
    SOC15_REG_ENTRY_STR!(JPEG, 0, mmUVD_JPEG_UV_PITCH),
];

unsafe fn jpeg_v3_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let harvest: u32;
    match amdgpu_ip_version(adev, UVD_HWIP, 0) {
        IP_VERSION!(3, 1, 1) | IP_VERSION!(3, 1, 2) => {}
        _ => {
            harvest = RREG32_SOC15!(JPEG, 0, mmCC_UVD_HARVESTING);
            if harvest & CC_UVD_HARVESTING__UVD_DISABLE_MASK != 0 { return -ENOENT; }
        }
    }
    (*adev).jpeg.num_jpeg_inst = 1;
    (*adev).jpeg.num_jpeg_rings = 1;
    jpeg_v3_0_set_dec_ring_funcs(adev);
    jpeg_v3_0_set_irq_funcs(adev);
    0
}

unsafe fn jpeg_v3_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let ring: *mut amdgpu_ring;
    let mut r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_VCN, VCN_2_0__SRCID__JPEG_DECODE, &mut (*(*adev).jpeg.inst).irq);
    if r != 0 { return r; }
    r = amdgpu_jpeg_sw_init(adev); if r != 0 { return r; }
    r = amdgpu_jpeg_resume(adev); if r != 0 { return r; }
    ring = (*(*adev).jpeg.inst).ring_dec;
    (*ring).use_doorbell = true;
    (*ring).doorbell_index = ((*adev).doorbell_index.vcn.vcn_ring0_1 << 1) + 1;
    (*ring).vm_hub = AMDGPU_MMHUB0!(0);
    sprintf!((*ring).name, "jpeg_dec");
    r = amdgpu_ring_init(adev, ring, 512, &mut (*(*adev).jpeg.inst).irq, 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut());
    if r != 0 { return r; }
    (*adev).jpeg.internal.jpeg_pitch[0] = MMUVD_JPEG_PITCH_INTERNAL_OFFSET;
    (*(*adev).jpeg.inst).external.jpeg_pitch[0] = SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_PITCH);
    r = amdgpu_jpeg_reg_dump_init(adev, jpeg_reg_list_3_0.as_ptr(), ARRAY_SIZE!(jpeg_reg_list_3_0));
    if r != 0 { return r; }
    (*adev).jpeg.supported_reset = amdgpu_get_soft_full_reset_mask((*(*adev).jpeg.inst).ring_dec);
    if !amdgpu_sriov_vf(adev) { (*adev).jpeg.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE; }
    amdgpu_jpeg_sysfs_reset_mask_init(adev)
}

unsafe fn jpeg_v3_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let r = amdgpu_jpeg_suspend(adev); if r != 0 { return r; }
    amdgpu_jpeg_sysfs_reset_mask_fini(adev);
    amdgpu_jpeg_sw_fini(adev)
}

unsafe fn jpeg_v3_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let ring = (*(*adev).jpeg.inst).ring_dec;
    ((*adev).nbio.funcs->vcn_doorbell_range)(adev, (*ring).use_doorbell, (*adev).doorbell_index.vcn.vcn_ring0_1 << 1, 0);
    amdgpu_ring_test_helper(ring)
}

unsafe fn jpeg_v3_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    cancel_delayed_work_sync!(&mut (*adev).jpeg.idle_work);
    if (*adev).jpeg.cur_state != AMD_PG_STATE_GATE && RREG32_SOC15!(JPEG, 0, mmUVD_JRBC_STATUS) != 0 { jpeg_v3_0_set_powergating_state(ip_block, AMD_PG_STATE_GATE); }
    0
}

unsafe fn jpeg_v3_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32 {
    let mut r = jpeg_v3_0_hw_fini(ip_block); if r != 0 { return r; }
    r = amdgpu_jpeg_suspend((*ip_block).adev); r
}

unsafe fn jpeg_v3_0_resume(ip_block: *mut amdgpu_ip_block) -> i32 {
    let mut r = amdgpu_jpeg_resume((*ip_block).adev); if r != 0 { return r; }
    r = jpeg_v3_0_hw_init(ip_block); r
}

unsafe fn jpeg_v3_0_disable_clock_gating(adev: *mut amdgpu_device) {
    let mut data = RREG32_SOC15!(JPEG, 0, mmJPEG_CGC_CTRL);
    if (*adev).cg_flags & AMD_CG_SUPPORT_JPEG_MGCG != 0 { data |= 1 << JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT; } else { data &= !JPEG_CGC_CTRL__DYN_CLOCK_MODE__SHIFT; }
    data |= 1 << JPEG_CGC_CTRL__CLK_GATE_DLY_TIMER__SHIFT; data |= 4 << JPEG_CGC_CTRL__CLK_OFF_DELAY__SHIFT;
    WREG32_SOC15!(JPEG, 0, mmJPEG_CGC_CTRL, data);
    data = RREG32_SOC15!(JPEG, 0, mmJPEG_CGC_GATE);
    data &= !(JPEG_CGC_GATE__JPEG_DEC_MASK | JPEG_CGC_GATE__JPEG2_DEC_MASK | JPEG_CGC_GATE__JPEG_ENC_MASK | JPEG_CGC_GATE__JMCIF_MASK | JPEG_CGC_GATE__JRBBM_MASK);
    WREG32_SOC15!(JPEG, 0, mmJPEG_CGC_GATE, data);
    data = RREG32_SOC15!(JPEG, 0, mmJPEG_CGC_CTRL);
    data &= !(JPEG_CGC_CTRL__JPEG_DEC_MODE_MASK | JPEG_CGC_CTRL__JPEG2_DEC_MODE_MASK | JPEG_CGC_CTRL__JMCIF_MODE_MASK | JPEG_CGC_CTRL__JRBBM_MODE_MASK);
    WREG32_SOC15!(JPEG, 0, mmJPEG_CGC_CTRL, data);
}

unsafe fn jpeg_v3_0_enable_clock_gating(_adev: *mut amdgpu_device) {
    let mut data = RREG32_SOC15!(JPEG, 0, mmJPEG_CGC_GATE);
    data |= JPEG_CGC_GATE__JPEG_DEC_MASK | JPEG_CGC_GATE__JPEG2_DEC_MASK | JPEG_CGC_GATE__JPEG_ENC_MASK | JPEG_CGC_GATE__JMCIF_MASK | JPEG_CGC_GATE__JRBBM_MASK;
    WREG32_SOC15!(JPEG, 0, mmJPEG_CGC_GATE, data);
}

unsafe fn jpeg_v3_0_disable_static_power_gating(adev: *mut amdgpu_device) -> i32 {
    if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG != 0 {
        let data = 1 << UVD_PGFSM_CONFIG__UVDJ_PWR_CONFIG__SHIFT;
        WREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_PGFSM_CONFIG), data);
        let r = SOC15_WAIT_ON_RREG!(JPEG, 0, mmUVD_PGFSM_STATUS, UVD_PGFSM_STATUS_UVDJ_PWR_ON, UVD_PGFSM_STATUS__UVDJ_PWR_STATUS_MASK);
        if r != 0 { drm_err!(adev_to_drm!(adev), "failed to disable JPEG power gating\n"); return r; }
    }
    WREG32_P!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS), 0, !UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK);
    WREG32_P!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS), 0, !UVD_JPEG_POWER_STATUS__JPEG_PG_MODE_MASK);
    0
}

unsafe fn jpeg_v3_0_enable_static_power_gating(adev: *mut amdgpu_device) -> i32 {
    WREG32_P!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JPEG_POWER_STATUS), UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK, !UVD_JPEG_POWER_STATUS__JPEG_POWER_STATUS_MASK);
    if (*adev).pg_flags & AMD_PG_SUPPORT_JPEG != 0 {
        let data = 2 << UVD_PGFSM_CONFIG__UVDJ_PWR_CONFIG__SHIFT;
        WREG32!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_PGFSM_CONFIG), data);
        let r = SOC15_WAIT_ON_RREG!(JPEG, 0, mmUVD_PGFSM_STATUS, 2 << UVD_PGFSM_STATUS__UVDJ_PWR_STATUS__SHIFT, UVD_PGFSM_STATUS__UVDJ_PWR_STATUS_MASK);
        if r != 0 { drm_err!(adev_to_drm!(adev), "failed to enable JPEG power gating\n"); return r; }
    }
    0
}

unsafe fn jpeg_v3_0_start(adev: *mut amdgpu_device) -> i32 {
    let ring = (*(*adev).jpeg.inst).ring_dec;
    if (*adev).pm.dpm_enabled { amdgpu_dpm_enable_jpeg(adev, true); }
    let mut r = jpeg_v3_0_disable_static_power_gating(adev); if r != 0 { return r; }
    jpeg_v3_0_disable_clock_gating(adev);
    WREG32_SOC15!(JPEG, 0, mmJPEG_DEC_GFX10_ADDR_CONFIG, (*adev).gfx.config.gb_addr_config);
    WREG32_SOC15!(JPEG, 0, mmJPEG_ENC_GFX10_ADDR_CONFIG, (*adev).gfx.config.gb_addr_config);
    WREG32_P!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JMI_CNTL), 0, !UVD_JMI_CNTL__SOFT_RESET_MASK);
    WREG32_P!(SOC15_REG_OFFSET!(JPEG, 0, mmJPEG_SYS_INT_EN), JPEG_SYS_INT_EN__DJRBC_MASK, !JPEG_SYS_INT_EN__DJRBC_MASK);
    WREG32_SOC15!(JPEG, 0, mmUVD_LMI_JRBC_RB_VMID, 0);
    WREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_CNTL, 0x00000001u32 | 0x00000002u32);
    WREG32_SOC15!(JPEG, 0, mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW, lower_32_bits!((*ring).gpu_addr));
    WREG32_SOC15!(JPEG, 0, mmUVD_LMI_JRBC_RB_64BIT_BAR_HIGH, upper_32_bits!((*ring).gpu_addr));
    WREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_RPTR, 0); WREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_WPTR, 0);
    WREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_CNTL, 0x00000002u32); WREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_SIZE, (*ring).ring_size / 4);
    (*ring).wptr = RREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_WPTR); r = 0; r
}

unsafe fn jpeg_v3_0_stop(adev: *mut amdgpu_device) -> i32 {
    WREG32_P!(SOC15_REG_OFFSET!(JPEG, 0, mmUVD_JMI_CNTL), UVD_JMI_CNTL__SOFT_RESET_MASK, !UVD_JMI_CNTL__SOFT_RESET_MASK);
    jpeg_v3_0_enable_clock_gating(adev);
    let r = jpeg_v3_0_enable_static_power_gating(adev); if r != 0 { return r; }
    if (*adev).pm.dpm_enabled { amdgpu_dpm_enable_jpeg(adev, false); } 0
}

unsafe fn jpeg_v3_0_dec_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { let _adev = (*ring).adev; RREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_RPTR) as u64 }
unsafe fn jpeg_v3_0_dec_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 { let _adev = (*ring).adev; if (*ring).use_doorbell { *(*ring).wptr_cpu_addr as u64 } else { RREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_WPTR) as u64 } }
unsafe fn jpeg_v3_0_dec_ring_set_wptr(ring: *mut amdgpu_ring) { let _adev = (*ring).adev; if (*ring).use_doorbell { *(*ring).wptr_cpu_addr = lower_32_bits!((*ring).wptr); WDOORBELL32!((*ring).doorbell_index, lower_32_bits!((*ring).wptr)); } else { WREG32_SOC15!(JPEG, 0, mmUVD_JRBC_RB_WPTR, lower_32_bits!((*ring).wptr)); } }
unsafe fn jpeg_v3_0_is_idle(ip_block: *mut amdgpu_ip_block) -> bool { let _adev = (*ip_block).adev; (RREG32_SOC15!(JPEG, 0, mmUVD_JRBC_STATUS) & UVD_JRBC_STATUS__RB_JOB_DONE_MASK) == UVD_JRBC_STATUS__RB_JOB_DONE_MASK }
unsafe fn jpeg_v3_0_wait_for_idle(_ip_block: *mut amdgpu_ip_block) -> i32 { SOC15_WAIT_ON_RREG!(JPEG, 0, mmUVD_JRBC_STATUS, UVD_JRBC_STATUS__RB_JOB_DONE_MASK, UVD_JRBC_STATUS__RB_JOB_DONE_MASK) }
unsafe fn jpeg_v3_0_set_clockgating_state(ip_block: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32 { let adev = (*ip_block).adev; if state == AMD_CG_STATE_GATE { if !jpeg_v3_0_is_idle(ip_block) { return -EBUSY; } jpeg_v3_0_enable_clock_gating(adev); } else { jpeg_v3_0_disable_clock_gating(adev); } 0 }
unsafe fn jpeg_v3_0_set_powergating_state(ip_block: *mut amdgpu_ip_block, state: amd_powergating_state) -> i32 { let adev = (*ip_block).adev; if state == (*adev).jpeg.cur_state { return 0; } let ret = if state == AMD_PG_STATE_GATE { jpeg_v3_0_stop(adev) } else { jpeg_v3_0_start(adev) }; if ret == 0 { (*adev).jpeg.cur_state = state; } ret }
unsafe fn jpeg_v3_0_set_interrupt_state(_adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _type: u32, _state: amdgpu_interrupt_state) -> i32 { 0 }
unsafe fn jpeg_v3_0_ring_reset(ring: *mut amdgpu_ring, _vmid: u32, timedout_fence: *mut amdgpu_fence) -> i32 { amdgpu_ring_reset_helper_begin(ring, timedout_fence); let mut r = jpeg_v3_0_stop((*ring).adev); if r != 0 { return r; } r = jpeg_v3_0_start((*ring).adev); if r != 0 { return r; } amdgpu_ring_reset_helper_end(ring, timedout_fence) }

static jpeg_v3_0_ip_funcs: amd_ip_funcs = amd_ip_funcs { name: "jpeg_v3_0", early_init: jpeg_v3_0_early_init, sw_init: jpeg_v3_0_sw_init, sw_fini: jpeg_v3_0_sw_fini, hw_init: jpeg_v3_0_hw_init, hw_fini: jpeg_v3_0_hw_fini, suspend: jpeg_v3_0_suspend, resume: jpeg_v3_0_resume, is_idle: jpeg_v3_0_is_idle, wait_for_idle: jpeg_v3_0_wait_for_idle, set_clockgating_state: jpeg_v3_0_set_clockgating_state, set_powergating_state: jpeg_v3_0_set_powergating_state, dump_ip_state: amdgpu_jpeg_dump_ip_state, print_ip_state: amdgpu_jpeg_print_ip_state };

static jpeg_v3_0_dec_ring_vm_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs {
    type: AMDGPU_RING_TYPE_VCN_JPEG, align_mask: 0xf, no_user_fence: true,
    get_rptr: jpeg_v3_0_dec_ring_get_rptr, get_wptr: jpeg_v3_0_dec_ring_get_wptr, set_wptr: jpeg_v3_0_dec_ring_set_wptr,
    parse_cs: amdgpu_jpeg_dec_parse_cs, emit_frame_size: SOC15_FLUSH_GPU_TLB_NUM_WREG * 6 + SOC15_FLUSH_GPU_TLB_NUM_REG_WAIT * 8 + 8 + 18 + 18 + 8 + 16,
    emit_ib_size: 22, emit_ib: jpeg_v2_0_dec_ring_emit_ib, emit_fence: jpeg_v2_0_dec_ring_emit_fence, emit_vm_flush: jpeg_v2_0_dec_ring_emit_vm_flush,
    test_ring: amdgpu_jpeg_dec_ring_test_ring, test_ib: amdgpu_jpeg_dec_ring_test_ib, insert_nop: jpeg_v2_0_dec_ring_nop,
    insert_start: jpeg_v2_0_dec_ring_insert_start, insert_end: jpeg_v2_0_dec_ring_insert_end, pad_ib: amdgpu_ring_generic_pad_ib,
    begin_use: amdgpu_jpeg_ring_begin_use, end_use: amdgpu_jpeg_ring_end_use, emit_wreg: jpeg_v2_0_dec_ring_emit_wreg,
    emit_reg_wait: jpeg_v2_0_dec_ring_emit_reg_wait, emit_reg_write_reg_wait: amdgpu_ring_emit_reg_write_reg_wait_helper, reset: jpeg_v3_0_ring_reset,
};

unsafe fn jpeg_v3_0_set_dec_ring_funcs(adev: *mut amdgpu_device) { (*(*adev).jpeg.inst).ring_dec.funcs = &jpeg_v3_0_dec_ring_vm_funcs; }
static jpeg_v3_0_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: jpeg_v3_0_set_interrupt_state, process: jpeg_v2_0_process_interrupt };
unsafe fn jpeg_v3_0_set_irq_funcs(adev: *mut amdgpu_device) { (*(*adev).jpeg.inst).irq.num_types = 1; (*(*adev).jpeg.inst).irq.funcs = &jpeg_v3_0_irq_funcs; }

static jpeg_v3_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_JPEG, major: 3, minor: 0, rev: 0, funcs: &jpeg_v3_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
