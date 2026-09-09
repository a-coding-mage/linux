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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// External kernel, AMDGPU, register, and platform definitions are supplied by
// the surrounding translation unit.

// Forward declaration: `tonga_ih_set_interrupt_funcs` is defined below.

unsafe fn tonga_ih_enable_interrupts(adev: *mut amdgpu_device) {
    let mut ih_rb_cntl: u32 = RREG32!(mmIH_RB_CNTL);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, RB_ENABLE, 1);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, ENABLE_INTR, 1);
    WREG32!(mmIH_RB_CNTL, ih_rb_cntl);
    (*adev).irq.ih.enabled = true;
}

unsafe fn tonga_ih_disable_interrupts(adev: *mut amdgpu_device) {
    let mut ih_rb_cntl: u32 = RREG32!(mmIH_RB_CNTL);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, RB_ENABLE, 0);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, ENABLE_INTR, 0);
    WREG32!(mmIH_RB_CNTL, ih_rb_cntl);
    WREG32!(mmIH_RB_RPTR, 0);
    WREG32!(mmIH_RB_WPTR, 0);
    (*adev).irq.ih.enabled = false;
    (*adev).irq.ih.rptr = 0;
}

unsafe fn tonga_ih_irq_init(adev: *mut amdgpu_device) -> i32 {
    let mut interrupt_cntl: u32;
    let mut ih_rb_cntl: u32;
    let mut ih_doorbell_rtpr: u32;
    let ih: *mut amdgpu_ih_ring = &mut (*adev).irq.ih;
    tonga_ih_disable_interrupts(adev);
    WREG32!(mmINTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    interrupt_cntl = RREG32!(mmINTERRUPT_CNTL);
    interrupt_cntl = REG_SET_FIELD!(interrupt_cntl, INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    interrupt_cntl = REG_SET_FIELD!(interrupt_cntl, INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32!(mmINTERRUPT_CNTL, interrupt_cntl);
    WREG32!(mmIH_RB_BASE, (*ih).gpu_addr >> 8);
    let rb_bufsz = order_base_2((*adev).irq.ih.ring_size / 4);
    ih_rb_cntl = REG_SET_FIELD!(0, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, RB_SIZE, rb_bufsz);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, WPTR_WRITEBACK_ENABLE, 1);
    ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, MC_VMID, 0);
    if (*adev).irq.msi_enabled { ih_rb_cntl = REG_SET_FIELD!(ih_rb_cntl, IH_RB_CNTL, RPTR_REARM, 1); }
    WREG32!(mmIH_RB_CNTL, ih_rb_cntl);
    WREG32!(mmIH_RB_WPTR_ADDR_LO, lower_32_bits((*ih).wptr_addr));
    WREG32!(mmIH_RB_WPTR_ADDR_HI, upper_32_bits((*ih).wptr_addr) & 0xff);
    WREG32!(mmIH_RB_RPTR, 0);
    WREG32!(mmIH_RB_WPTR, 0);
    ih_doorbell_rtpr = RREG32!(mmIH_DOORBELL_RPTR);
    if (*adev).irq.ih.use_doorbell {
        ih_doorbell_rtpr = REG_SET_FIELD!(ih_doorbell_rtpr, IH_DOORBELL_RPTR, OFFSET, (*adev).irq.ih.doorbell_index);
        ih_doorbell_rtpr = REG_SET_FIELD!(ih_doorbell_rtpr, IH_DOORBELL_RPTR, ENABLE, 1);
    } else { ih_doorbell_rtpr = REG_SET_FIELD!(ih_doorbell_rtpr, IH_DOORBELL_RPTR, ENABLE, 0); }
    WREG32!(mmIH_DOORBELL_RPTR, ih_doorbell_rtpr);
    pci_set_master((*adev).pdev);
    tonga_ih_enable_interrupts(adev);
    if (*adev).irq.ih_soft.ring_size != 0 { (*adev).irq.ih_soft.enabled = true; }
    0
}

unsafe fn tonga_ih_irq_disable(adev: *mut amdgpu_device) { tonga_ih_disable_interrupts(adev); mdelay(1); }

unsafe fn tonga_ih_get_wptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> u32 {
    let mut wptr = le32_to_cpu(*(*ih).wptr_cpu);
    let mut tmp: u32;
    if ih == &mut (*adev).irq.ih_soft { return wptr & (*ih).ptr_mask; }
    if REG_GET_FIELD!(wptr, IH_RB_WPTR, RB_OVERFLOW) == 0 { return wptr & (*ih).ptr_mask; }
    wptr = RREG32!(mmIH_RB_WPTR);
    if REG_GET_FIELD!(wptr, IH_RB_WPTR, RB_OVERFLOW) == 0 { return wptr & (*ih).ptr_mask; }
    wptr = REG_SET_FIELD!(wptr, IH_RB_WPTR, RB_OVERFLOW, 0);
    dev_warn!((*adev).dev, "IH ring buffer overflow (0x%08X, 0x%08X, 0x%08X)\n", wptr, (*ih).rptr, (wptr + 16) & (*ih).ptr_mask);
    (*ih).rptr = (wptr + 16) & (*ih).ptr_mask;
    tmp = RREG32!(mmIH_RB_CNTL);
    tmp = REG_SET_FIELD!(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1);
    WREG32!(mmIH_RB_CNTL, tmp);
    tmp = REG_SET_FIELD!(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0);
    WREG32!(mmIH_RB_CNTL, tmp);
    wptr & (*ih).ptr_mask
}

unsafe fn tonga_ih_decode_iv(_adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, entry: *mut amdgpu_iv_entry) {
    let ring_index = (*ih).rptr >> 2;
    let dw = [le32_to_cpu((*ih).ring.add((ring_index + 0) as usize)), le32_to_cpu((*ih).ring.add((ring_index + 1) as usize)), le32_to_cpu((*ih).ring.add((ring_index + 2) as usize)), le32_to_cpu((*ih).ring.add((ring_index + 3) as usize))];
    (*entry).client_id = AMDGPU_IRQ_CLIENTID_LEGACY;
    (*entry).src_id = dw[0] & 0xff;
    (*entry).src_data[0] = dw[1] & 0xfffffff;
    (*entry).ring_id = dw[2] & 0xff;
    (*entry).vmid = (dw[2] >> 8) & 0xff;
    (*entry).pasid = (dw[2] >> 16) & 0xffff;
    (*ih).rptr += 16;
}

unsafe fn tonga_ih_set_rptr(_adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) {
    if (*ih).use_doorbell { *(*ih).rptr_cpu = (*ih).rptr; WDOORBELL32!((*ih).doorbell_index, (*ih).rptr); }
    else { WREG32!(mmIH_RB_RPTR, (*ih).rptr); }
}

unsafe fn tonga_ih_early_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; let ret = amdgpu_irq_add_domain(adev); if ret != 0 { return ret; } tonga_ih_set_interrupt_funcs(adev); 0 }
unsafe fn tonga_ih_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; let mut r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih, 64 * 1024, true); if r != 0 { return r; } r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih_soft, IH_SW_RING_SIZE, true); if r != 0 { return r; } (*adev).irq.ih.use_doorbell = true; (*adev).irq.ih.doorbell_index = (*adev).doorbell_index.ih; amdgpu_irq_init(adev) }
unsafe fn tonga_ih_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; amdgpu_irq_fini_sw(adev); amdgpu_irq_remove_domain(adev); 0 }
unsafe fn tonga_ih_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 { tonga_ih_irq_init((*ip_block).adev) }
unsafe fn tonga_ih_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { tonga_ih_irq_disable((*ip_block).adev); 0 }
unsafe fn tonga_ih_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { tonga_ih_hw_fini(ip_block) }
unsafe fn tonga_ih_resume(ip_block: *mut amdgpu_ip_block) -> i32 { tonga_ih_hw_init(ip_block) }
unsafe fn tonga_ih_is_idle(ip_block: *mut amdgpu_ip_block) -> bool { REG_GET_FIELD!(RREG32!(mmSRBM_STATUS), SRBM_STATUS, IH_BUSY) == 0 }
unsafe fn tonga_ih_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; for _ in 0..(*adev).usec_timeout { if REG_GET_FIELD!(RREG32!(mmSRBM_STATUS), SRBM_STATUS, IH_BUSY) == 0 { return 0; } udelay(1); } -ETIMEDOUT }
unsafe fn tonga_ih_soft_reset(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; let s = (*adev).irq.srbm_soft_reset; if s == 0 { return 0; } let mut tmp = RREG32!(mmSRBM_SOFT_RESET) | s; dev_info!((*adev).dev, "SRBM_SOFT_RESET=0x%08X\n", tmp); WREG32!(mmSRBM_SOFT_RESET, tmp); tmp = RREG32!(mmSRBM_SOFT_RESET); udelay(50); tmp &= !s; WREG32!(mmSRBM_SOFT_RESET, tmp); let _ = RREG32!(mmSRBM_SOFT_RESET); udelay(50); 0 }
unsafe fn tonga_ih_set_clockgating_state(_ip_block: *mut amdgpu_ip_block, _state: amd_clockgating_state) -> i32 { 0 }
unsafe fn tonga_ih_set_powergating_state(_ip_block: *mut amdgpu_ip_block, _state: amd_powergating_state) -> i32 { 0 }

static TONGA_IH_IP_FUNCS: amd_ip_funcs = amd_ip_funcs { name: "tonga_ih", early_init: Some(tonga_ih_early_init), sw_init: Some(tonga_ih_sw_init), sw_fini: Some(tonga_ih_sw_fini), hw_init: Some(tonga_ih_hw_init), hw_fini: Some(tonga_ih_hw_fini), suspend: Some(tonga_ih_suspend), resume: Some(tonga_ih_resume), is_idle: Some(tonga_ih_is_idle), wait_for_idle: Some(tonga_ih_wait_for_idle), soft_reset: Some(tonga_ih_soft_reset), set_clockgating_state: Some(tonga_ih_set_clockgating_state), set_powergating_state: Some(tonga_ih_set_powergating_state) };
static TONGA_IH_FUNCS: amdgpu_ih_funcs = amdgpu_ih_funcs { get_wptr: Some(tonga_ih_get_wptr), decode_iv: Some(tonga_ih_decode_iv), set_rptr: Some(tonga_ih_set_rptr) };
unsafe fn tonga_ih_set_interrupt_funcs(adev: *mut amdgpu_device) { (*adev).irq.ih_funcs = &TONGA_IH_FUNCS; }
static TONGA_IH_IP_BLOCK: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_IH, major: 3, minor: 0, rev: 0, funcs: &TONGA_IH_IP_FUNCS };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
