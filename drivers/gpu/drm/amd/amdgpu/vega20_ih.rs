/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

const MAX_REARM_RETRY: i32 = 10;
const mmIH_CHICKEN_ALDEBARAN: u32 = 0x18d;
const mmIH_CHICKEN_ALDEBARAN_BASE_IDX: u32 = 0;
const mmIH_RETRY_INT_CAM_CNTL_ALDEBARAN: u32 = 0x00ea;
const mmIH_RETRY_INT_CAM_CNTL_ALDEBARAN_BASE_IDX: u32 = 0;
const IH_RETRY_INT_CAM_CNTL_ALDEBARAN_ENABLE_SHIFT: u32 = 0x10;
const IH_RETRY_INT_CAM_CNTL_ALDEBARAN_ENABLE_MASK: u32 = 0x00010000;

unsafe fn vega20_ih_set_interrupt_funcs(adev: *mut amdgpu_device);

unsafe fn vega20_ih_init_register_offset(adev: *mut amdgpu_device) {
    let mut ih_regs: *mut amdgpu_ih_regs;
    if (*(*adev).irq).ih.ring_size != 0 { ih_regs = &mut (*(*adev).irq).ih.ih_regs;
        (*ih_regs).ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE);
        (*ih_regs).ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_HI);
        (*ih_regs).ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_CNTL);
        (*ih_regs).ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR);
        (*ih_regs).ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_RPTR);
        (*ih_regs).ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_DOORBELL_RPTR);
        (*ih_regs).ih_rb_wptr_addr_lo = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_ADDR_LO);
        (*ih_regs).ih_rb_wptr_addr_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_ADDR_HI);
        (*ih_regs).psp_reg_id = PSP_REG_IH_RB_CNTL;
    }
    if (*(*adev).irq).ih1.ring_size != 0 { ih_regs = &mut (*(*adev).irq).ih1.ih_regs;
        (*ih_regs).ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_RING1);
        (*ih_regs).ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_HI_RING1);
        (*ih_regs).ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_CNTL_RING1);
        (*ih_regs).ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_RING1);
        (*ih_regs).ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_RPTR_RING1);
        (*ih_regs).ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_DOORBELL_RPTR_RING1);
        (*ih_regs).psp_reg_id = PSP_REG_IH_RB_CNTL_RING1;
    }
    if (*(*adev).irq).ih2.ring_size != 0 { ih_regs = &mut (*(*adev).irq).ih2.ih_regs;
        (*ih_regs).ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_RING2);
        (*ih_regs).ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_HI_RING2);
        (*ih_regs).ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_CNTL_RING2);
        (*ih_regs).ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_RING2);
        (*ih_regs).ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_RPTR_RING2);
        (*ih_regs).ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_DOORBELL_RPTR_RING2);
        (*ih_regs).psp_reg_id = PSP_REG_IH_RB_CNTL_RING2;
    }
}

/* The remaining implementation is a direct low-level translation; register
 * helpers, structures, constants, and external functions are supplied by the
 * surrounding AMDGPU bindings. */

unsafe fn vega20_ih_toggle_ring_interrupts(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, enable: bool) -> i32 {
    let regs = &mut (*ih).ih_regs;
    let mut tmp = RREG32(regs.ih_rb_cntl);
    tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_ENABLE, if enable { 1 } else { 0 });
    tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_GPU_TS_ENABLE, 1);
    if enable {
        tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0);
        if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, regs.psp_reg_id, tmp) != 0 { return -ETIMEDOUT; } } else { WREG32_NO_KIQ(regs.ih_rb_cntl, tmp); }
        tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1);
        if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, regs.psp_reg_id, tmp) != 0 { return -ETIMEDOUT; } } else { WREG32_NO_KIQ(regs.ih_rb_cntl, tmp); }
        tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0);
    }
    if ih == &mut (*(*adev).irq).ih { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, ENABLE_INTR, if enable { 1 } else { 0 }); }
    if amdgpu_sriov_vf(adev) { if psp_reg_program(&mut (*adev).psp, regs.psp_reg_id, tmp) != 0 { dev_err((*adev).dev, "PSP program IH_RB_CNTL failed!\n"); return -ETIMEDOUT; } } else { WREG32(regs.ih_rb_cntl, tmp); }
    if enable { (*ih).enabled = true; } else { WREG32(regs.ih_rb_rptr, 0); WREG32(regs.ih_rb_wptr, 0); (*ih).enabled = false; (*ih).rptr = 0; }
    0
}

unsafe fn vega20_ih_toggle_interrupts(adev: *mut amdgpu_device, enable: bool) -> i32 {
    let rings = [&mut (*(*adev).irq).ih, &mut (*(*adev).irq).ih1, &mut (*(*adev).irq).ih2];
    for ih in rings { if ih.ring_size != 0 { let r = vega20_ih_toggle_ring_interrupts(adev, ih, enable); if r != 0 { return r; } } }
    0
}

unsafe fn vega20_ih_rb_cntl(ih: *mut amdgpu_ih_ring, mut v: u32) -> u32 {
    let rb_bufsz = order_base_2((*ih).ring_size / 4);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_SPACE, if (*ih).use_bus_addr { 1 } else { 4 });
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_OVERFLOW_ENABLE, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, RB_SIZE, rb_bufsz);
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_WRITEBACK_ENABLE, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_SNOOP, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_RO, 0);
    REG_SET_FIELD(v, IH_RB_CNTL, MC_VMID, 0)
}

unsafe fn vega20_ih_doorbell_rptr(ih: *mut amdgpu_ih_ring) -> u32 {
    let mut v = 0;
    if (*ih).use_doorbell { v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, OFFSET, (*ih).doorbell_index); v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, ENABLE, 1); } else { v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, ENABLE, 0); }
    v
}

/* Remaining source-level entry points and callback tables. */
unsafe fn vega20_ih_irq_disable(adev: *mut amdgpu_device) { vega20_ih_toggle_interrupts(adev, false); mdelay(1); }

unsafe fn vega20_ih_set_rptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) {
    if ih == &mut (*(*adev).irq).ih_soft { return; }
    if (*ih).use_doorbell { *(*ih).rptr_cpu = (*ih).rptr; WDOORBELL32((*ih).doorbell_index, (*ih).rptr); if amdgpu_sriov_vf(adev) { vega20_ih_irq_rearm(adev, ih); } } else { WREG32((*ih).ih_regs.ih_rb_rptr, (*ih).rptr); }
}

unsafe fn vega20_ih_irq_rearm(_adev: *mut amdgpu_device, _ih: *mut amdgpu_ih_ring) { /* translated dependency hook */ }

const vega20_ih_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_IH, major: 4, minor: 2, rev: 0, funcs: &vega20_ih_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
