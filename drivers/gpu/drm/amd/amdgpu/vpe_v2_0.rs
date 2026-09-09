/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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

// External Linux/AMDGPU declarations and register definitions are supplied by
// the surrounding translation unit.

const VPE_THREAD1_UCODE_OFFSET: u32 = 0x8000;

unsafe fn vpe_v2_0_get_reg_offset(vpe: *mut amdgpu_vpe, inst: u32, offset: u32) -> u32 {
    let base: u32 = (*(*vpe).ring.adev).reg_offset[VPE_HWIP][inst as usize][0];
    base + offset
}

unsafe fn vpe_v2_0_irq_init(vpe: *mut amdgpu_vpe) -> i32 {
    let adev: *mut amdgpu_device = container_of!(vpe, amdgpu_device, vpe);
    let ret = amdgpu_irq_add_id(
        adev,
        SOC21_IH_CLIENTID_VPE,
        VPE_6_1_SRCID__VPE_TRAP,
        &mut (*adev).vpe.trap_irq,
    );
    if ret != 0 { return ret; }
    0
}

unsafe fn vpe_v2_0_load_microcode(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = (*vpe).ring.adev;
    let mut ucode_offset = [0u32; 2];
    let mut ucode_size = [0u32; 2];
    let mut f32_offset: u32;
    let mut f32_cntl: u32;
    let mut reg_data: u32;
    let mut ret = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_CNTL));
    ret = REG_SET_FIELD!(ret, VPEC_CNTL, UMSCH_INT_ENABLE, 0);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_CNTL), ret);
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_CNTL2));
    reg_data = REG_SET_FIELD!(reg_data, VPEC_CNTL2, IB_FIFO_WATERMARK, 1);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_CNTL2), reg_data);
    if amdgpu_vpe_configure_dpm(vpe) != 0 { dev_warn!((*adev).dev, "VPE DPM not enabled.\n"); }
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP {
        f32_offset = vpe_get_reg_offset(vpe, 0, regVPEC_F32_CNTL);
        f32_cntl = RREG32(f32_offset);
        f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, HALT, 0);
        f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH1_RESET, 0);
        (*adev).vpe.cmdbuf_cpu_addr[0] = f32_offset as u64;
        (*adev).vpe.cmdbuf_cpu_addr[1] = f32_cntl as u64;
        return amdgpu_vpe_psp_update_sram(adev);
    }
    f32_offset = vpe_get_reg_offset(vpe, 0, regVPEC_F32_CNTL);
    f32_cntl = RREG32(f32_offset);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, HALT, 1);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH1_RESET, 1);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH1_CHECKSUM_CLR, 1);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH0_CHECKSUM_CLR, 1);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_F32_CNTL), f32_cntl);
    f32_cntl = RREG32(f32_offset);
    if REG_GET_FIELD!(f32_cntl, VPEC_F32_CNTL, HALT) == 0 { dev_err!((*adev).dev, "VPEC is not halted"); return -EBUSY; }
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH1_CHECKSUM_CLR, 0);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH0_CHECKSUM_CLR, 0);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_F32_CNTL), f32_cntl);
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_UCODE_CHECKSUM));
    if reg_data != 0 { dev_err!((*adev).dev, "VPE FW checksum 0 not clean"); return -EBUSY; }
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_UCODE1_CHECKSUM));
    if reg_data != 0 { dev_err!((*adev).dev, "VPE FW checksum 1 not clean"); return -EBUSY; }
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_STATUS2));
    if REG_GET_FIELD!(reg_data, VPEC_STATUS2, TH0F32_INSTR_PTR) != 0 { dev_err!((*adev).dev, "VPE FW initial status not clean"); return -EBUSY; }
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_STATUS6));
    if REG_GET_FIELD!(reg_data, VPEC_STATUS6, TH1F32_INSTR_PTR) != 0 { dev_err!((*adev).dev, "VPE FW initial status not clean"); return -EBUSY; }
    let vpe_hdr = (*adev).vpe.fw.data as *const vpe_firmware_header_v1_0;
    ucode_offset[0] = le32_to_cpu((*vpe_hdr).header.ucode_array_offset_bytes);
    ucode_size[0] = le32_to_cpu((*vpe_hdr).ctx_ucode_size_bytes);
    ucode_offset[1] = le32_to_cpu((*vpe_hdr).ctl_ucode_offset);
    ucode_size[1] = le32_to_cpu((*vpe_hdr).ctl_ucode_size_bytes);
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_PG_CNTL));
    reg_data = REG_SET_FIELD!(reg_data, VPEC_PG_CNTL, PG_EN, 0);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_PG_CNTL), reg_data);
    for j in 0..(*vpe).num_instances {
        for i in 0..2 {
            WREG32(vpe_get_reg_offset(vpe, j as u32, regVPEC_UCODE_ADDR), if i > 0 { VPE_THREAD1_UCODE_OFFSET } else { 0 });
            let mut data = ((*adev).vpe.fw.data as *const u8).add(ucode_offset[i] as usize) as *const u32;
            let mut size_dw = ucode_size[i] / core::mem::size_of::<u32>() as u32;
            while size_dw != 0 {
                size_dw -= 1;
                if amdgpu_emu_mode && size_dw % 500 == 0 { msleep(1); }
                WREG32(vpe_get_reg_offset(vpe, j as u32, regVPEC_UCODE_DATA), le32_to_cpu(*data));
                data = data.add(1);
            }
        }
    }
    reg_data = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_PG_CNTL));
    reg_data = REG_SET_FIELD!(reg_data, VPEC_PG_CNTL, PG_EN, 1);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_PG_CNTL), reg_data);
    f32_cntl = RREG32(f32_offset);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, HALT, 0);
    f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH1_RESET, 0);
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_F32_CNTL), f32_cntl);
    0
}

unsafe fn vpe_v2_0_ring_start(vpe: *mut amdgpu_vpe) -> i32 {
    let ring = &mut (*vpe).ring;
    let adev = ring.adev;
    let mut ret = 0;
    for i in 0..(*vpe).num_instances {
        let rb_bufsz = order_base_2(ring.ring_size / 4);
        let mut rb_cntl = RREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_CNTL));
        rb_cntl = REG_SET_FIELD!(rb_cntl, VPEC_QUEUE0_RB_CNTL, RB_SIZE, rb_bufsz);
        rb_cntl = REG_SET_FIELD!(rb_cntl, VPEC_QUEUE0_RB_CNTL, RB_PRIV, 1);
        rb_cntl = REG_SET_FIELD!(rb_cntl, VPEC_QUEUE0_RB_CNTL, RB_VMID, 0);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_CNTL), rb_cntl);
        for reg in [regVPEC_QUEUE0_RB_RPTR, regVPEC_QUEUE0_RB_RPTR_HI, regVPEC_QUEUE0_RB_WPTR, regVPEC_QUEUE0_RB_WPTR_HI] { WREG32(vpe_get_reg_offset(vpe, i as u32, reg), 0); }
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_RPTR_ADDR_LO), lower_32_bits(ring.rptr_gpu_addr) & 0xFFFFFFFC);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_RPTR_ADDR_HI), upper_32_bits(ring.rptr_gpu_addr) & 0xFFFFFFFF);
        rb_cntl = REG_SET_FIELD!(rb_cntl, VPEC_QUEUE0_RB_CNTL, RPTR_WRITEBACK_ENABLE, 1);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_BASE), ring.gpu_addr >> 8);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_BASE_HI), ring.gpu_addr >> 40);
        ring.wptr = 0;
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_MINOR_PTR_UPDATE), 1);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_WPTR), lower_32_bits(ring.wptr) << 2);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_WPTR_HI), upper_32_bits(ring.wptr) << 2);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_MINOR_PTR_UPDATE), 0);
        let mut doorbell_offset = RREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_DOORBELL_OFFSET));
        doorbell_offset = REG_SET_FIELD!(doorbell_offset, VPEC_QUEUE0_DOORBELL_OFFSET, OFFSET, ring.doorbell_index + i * 4);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_DOORBELL_OFFSET), doorbell_offset);
        let mut doorbell = RREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_DOORBELL));
        doorbell = REG_SET_FIELD!(doorbell, VPEC_QUEUE0_DOORBELL, ENABLE, if ring.use_doorbell { 1 } else { 0 });
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_DOORBELL), doorbell);
        ((*adev).nbio.funcs.vpe_doorbell_range)(adev, i as u32, ring.use_doorbell, ring.doorbell_index + i * 4, 4);
        rb_cntl = REG_SET_FIELD!(rb_cntl, VPEC_QUEUE0_RB_CNTL, RPTR_WRITEBACK_ENABLE, 1);
        rb_cntl = REG_SET_FIELD!(rb_cntl, VPEC_QUEUE0_RB_CNTL, RB_ENABLE, 1);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_RB_CNTL), rb_cntl);
        let mut ib_cntl = RREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_IB_CNTL));
        ib_cntl = REG_SET_FIELD!(ib_cntl, VPEC_QUEUE0_IB_CNTL, IB_ENABLE, 1);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE0_IB_CNTL), ib_cntl);
    }
    ret = amdgpu_ring_test_helper(ring);
    if ret != 0 { return ret; }
    0
}

unsafe fn vpe_v2_0_ring_stop(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = (*vpe).ring.adev;
    let mut ret = 0;
    for i in 0..(*vpe).num_instances {
        let mut queue_reset = RREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE_RESET_REQ));
        queue_reset = REG_SET_FIELD!(queue_reset, VPEC_QUEUE_RESET_REQ, QUEUE0_RESET, 1);
        WREG32(vpe_get_reg_offset(vpe, i as u32, regVPEC_QUEUE_RESET_REQ), queue_reset);
        ret = SOC15_WAIT_ON_RREG!(VPE, i, regVPEC_QUEUE_RESET_REQ, 0, VPEC_QUEUE_RESET_REQ__QUEUE0_RESET_MASK);
        if ret != 0 { dev_err!((*adev).dev, "VPE queue reset failed\n"); }
    }
    (*vpe).ring.sched.ready = false;
    ret
}

unsafe fn vpe_v2_0_set_trap_irq_state(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _type: u32, state: amdgpu_interrupt_state) -> i32 {
    let vpe = &mut (*adev).vpe;
    let mut vpe_cntl = RREG32(vpe_get_reg_offset(vpe, 0, regVPEC_CNTL));
    vpe_cntl = REG_SET_FIELD!(vpe_cntl, VPEC_CNTL, TRAP_ENABLE, if state == AMDGPU_IRQ_STATE_ENABLE { 1 } else { 0 });
    WREG32(vpe_get_reg_offset(vpe, 0, regVPEC_CNTL), vpe_cntl);
    0
}

unsafe fn vpe_v2_0_process_trap_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 {
    DRM_DEBUG!("IH: VPE trap\n");
    match (*entry).client_id { SOC21_IH_CLIENTID_VPE => amdgpu_fence_process(&mut (*adev).vpe.ring), _ => {} }
    0
}

unsafe fn vpe_v2_0_set_regs(vpe: *mut amdgpu_vpe) -> i32 {
    (*vpe).regs.queue0_rb_rptr_lo = regVPEC_QUEUE0_RB_RPTR;
    (*vpe).regs.queue0_rb_rptr_hi = regVPEC_QUEUE0_RB_RPTR_HI;
    (*vpe).regs.queue0_rb_wptr_lo = regVPEC_QUEUE0_RB_WPTR;
    (*vpe).regs.queue0_rb_wptr_hi = regVPEC_QUEUE0_RB_WPTR_HI;
    (*vpe).regs.queue0_preempt = regVPEC_QUEUE0_PREEMPT;
    (*vpe).regs.dpm_enable = regVPEC_PUB_DUMMY2;
    (*vpe).regs.dpm_pratio = regVPEC_QUEUE6_DUMMY4;
    (*vpe).regs.dpm_request_interval = regVPEC_QUEUE5_DUMMY3;
    (*vpe).regs.dpm_decision_threshold = regVPEC_QUEUE5_DUMMY4;
    (*vpe).regs.dpm_busy_clamp_threshold = regVPEC_QUEUE7_DUMMY2;
    (*vpe).regs.dpm_idle_clamp_threshold = regVPEC_QUEUE7_DUMMY3;
    (*vpe).regs.dpm_request_lv = regVPEC_QUEUE7_DUMMY1;
    (*vpe).regs.context_indicator = regVPEC_QUEUE6_DUMMY3;
    0
}

static mut vpe_v2_0_funcs: vpe_funcs = vpe_funcs {
    get_reg_offset: vpe_v2_0_get_reg_offset,
    set_regs: vpe_v2_0_set_regs,
    irq_init: vpe_v2_0_irq_init,
    init_microcode: amdgpu_vpe_init_microcode,
    load_microcode: vpe_v2_0_load_microcode,
    ring_init: amdgpu_vpe_ring_init,
    ring_start: vpe_v2_0_ring_start,
    ring_stop: vpe_v2_0_ring_stop,
    ring_fini: amdgpu_vpe_ring_fini,
};

static vpe_v2_0_trap_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs {
    set: vpe_v2_0_set_trap_irq_state,
    process: vpe_v2_0_process_trap_irq,
};

pub unsafe fn vpe_v2_0_set_funcs(vpe: *mut amdgpu_vpe) {
    (*vpe).funcs = &mut vpe_v2_0_funcs;
    (*vpe).trap_irq.funcs = &mut vpe_v2_0_trap_irq_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
