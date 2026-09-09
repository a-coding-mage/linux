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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/pci.h, amdgpu.h, amdgpu_ih.h, sid.h, si_ih.h,
// oss/oss_1_0_d.h, and oss/oss_1_0_sh_mask.h.

unsafe fn si_ih_enable_interrupts(adev: *mut amdgpu_device) {
    let mut ih_cntl: u32 = RREG32(IH_CNTL);
    let mut ih_rb_cntl: u32 = RREG32(IH_RB_CNTL);
    ih_cntl |= ENABLE_INTR;
    ih_rb_cntl |= IH_RB_ENABLE;
    WREG32(IH_CNTL, ih_cntl);
    WREG32(IH_RB_CNTL, ih_rb_cntl);
    (*adev).irq.ih.enabled = true;
}

unsafe fn si_ih_disable_interrupts(adev: *mut amdgpu_device) {
    let mut ih_rb_cntl: u32 = RREG32(IH_RB_CNTL);
    let mut ih_cntl: u32 = RREG32(IH_CNTL);
    ih_rb_cntl &= !IH_RB_ENABLE;
    ih_cntl &= !ENABLE_INTR;
    WREG32(IH_RB_CNTL, ih_rb_cntl);
    WREG32(IH_CNTL, ih_cntl);
    WREG32(IH_RB_RPTR, 0);
    WREG32(IH_RB_WPTR, 0);
    (*adev).irq.ih.enabled = false;
    (*adev).irq.ih.rptr = 0;
}

unsafe fn si_ih_irq_init(adev: *mut amdgpu_device) -> i32 {
    let ih: *mut amdgpu_ih_ring = &mut (*adev).irq.ih;
    let mut rb_bufsz: i32;
    let mut interrupt_cntl: u32;
    let mut ih_cntl: u32;
    let mut ih_rb_cntl: u32;

    si_ih_disable_interrupts(adev);
    WREG32(INTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    interrupt_cntl = RREG32(INTERRUPT_CNTL);
    interrupt_cntl &= !IH_DUMMY_RD_OVERRIDE;
    interrupt_cntl &= !IH_REQ_NONSNOOP_EN;
    WREG32(INTERRUPT_CNTL, interrupt_cntl);
    WREG32(IH_RB_BASE, (*adev).irq.ih.gpu_addr >> 8);
    rb_bufsz = order_base_2((*adev).irq.ih.ring_size / 4);
    ih_rb_cntl = IH_WPTR_OVERFLOW_ENABLE | IH_WPTR_OVERFLOW_CLEAR |
        ((rb_bufsz as u32) << 1) | IH_WPTR_WRITEBACK_ENABLE;
    WREG32(IH_RB_WPTR_ADDR_LO, lower_32_bits((*ih).wptr_addr));
    WREG32(IH_RB_WPTR_ADDR_HI, upper_32_bits((*ih).wptr_addr) & 0xff);
    WREG32(IH_RB_CNTL, ih_rb_cntl);
    WREG32(IH_RB_RPTR, 0);
    WREG32(IH_RB_WPTR, 0);
    ih_cntl = MC_WRREQ_CREDIT(0x10) | MC_WR_CLEAN_CNT(0x10) | MC_VMID(0);
    if (*adev).irq.msi_enabled { ih_cntl |= RPTR_REARM; }
    WREG32(IH_CNTL, ih_cntl);
    pci_set_master((*adev).pdev);
    si_ih_enable_interrupts(adev);
    if (*adev).irq.ih_soft.ring_size != 0 { (*adev).irq.ih_soft.enabled = true; }
    0
}

unsafe fn si_ih_irq_disable(adev: *mut amdgpu_device) { si_ih_disable_interrupts(adev); mdelay(1); }

unsafe fn si_ih_get_wptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> u32 {
    let mut wptr = le32_to_cpu(*(*ih).wptr_cpu);
    let mut tmp: u32;
    if ih == &mut (*adev).irq.ih_soft { return wptr & (*ih).ptr_mask; }
    if wptr & IH_RB_WPTR__RB_OVERFLOW_MASK != 0 {
        wptr &= !IH_RB_WPTR__RB_OVERFLOW_MASK;
        dev_warn((*adev).dev, "IH ring buffer overflow (0x%08X, 0x%08X, 0x%08X)\n", wptr, (*ih).rptr, (wptr + 16) & (*ih).ptr_mask);
        (*ih).rptr = (wptr + 16) & (*ih).ptr_mask;
        tmp = RREG32(IH_RB_CNTL);
        tmp |= IH_RB_CNTL__WPTR_OVERFLOW_CLEAR_MASK;
        WREG32(IH_RB_CNTL, tmp);
        tmp &= !IH_RB_CNTL__WPTR_OVERFLOW_CLEAR_MASK;
        WREG32(IH_RB_CNTL, tmp);
    }
    wptr & (*ih).ptr_mask
}

unsafe fn si_ih_decode_iv(_adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, entry: *mut amdgpu_iv_entry) {
    let ring_index = (*ih).rptr >> 2;
    let mut dw = [0u32; 4];
    dw[0] = le32_to_cpu((*ih).ring[ring_index as usize + 0]);
    dw[1] = le32_to_cpu((*ih).ring[ring_index as usize + 1]);
    dw[2] = le32_to_cpu((*ih).ring[ring_index as usize + 2]);
    dw[3] = le32_to_cpu((*ih).ring[ring_index as usize + 3]);
    (*entry).client_id = AMDGPU_IRQ_CLIENTID_LEGACY;
    (*entry).src_id = dw[0] & 0xff;
    (*entry).src_data[0] = dw[1] & 0xfffffff;
    (*entry).ring_id = dw[2] & 0xff;
    (*entry).vmid = (dw[2] >> 8) & 0xff;
    (*ih).rptr += 16;
}

unsafe fn si_ih_set_rptr(_adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) { WREG32(IH_RB_RPTR, (*ih).rptr); }

unsafe fn si_ih_early_init(ip_block: *mut amdgpu_ip_block) -> i32 { si_ih_set_interrupt_funcs((*ip_block).adev); 0 }
unsafe fn si_ih_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih, 64 * 1024, false);
    if r != 0 { return r; }
    r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih_soft, IH_SW_RING_SIZE, true);
    if r != 0 { return r; }
    amdgpu_irq_init(adev)
}
unsafe fn si_ih_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { amdgpu_irq_fini_sw((*ip_block).adev); 0 }
unsafe fn si_ih_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 { si_ih_irq_init((*ip_block).adev) }
unsafe fn si_ih_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { si_ih_irq_disable((*ip_block).adev); 0 }
unsafe fn si_ih_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { si_ih_hw_fini(ip_block) }
unsafe fn si_ih_resume(ip_block: *mut amdgpu_ip_block) -> i32 { si_ih_hw_init(ip_block) }
unsafe fn si_ih_is_idle(ip_block: *mut amdgpu_ip_block) -> bool {
    let tmp = RREG32(mmSRBM_STATUS);
    !(tmp & SRBM_STATUS__IH_BUSY_MASK != 0)
}
unsafe fn si_ih_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> i32 {
    for _i in 0..(*(*ip_block).adev).usec_timeout { if si_ih_is_idle(ip_block) { return 0; } udelay(1); }
    -ETIMEDOUT
}
unsafe fn si_ih_soft_reset(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    let mut srbm_soft_reset = 0u32;
    let mut tmp = RREG32(mmSRBM_STATUS);
    if tmp & SRBM_STATUS__IH_BUSY_MASK != 0 { srbm_soft_reset |= SRBM_SOFT_RESET__SOFT_RESET_IH_MASK; }
    if srbm_soft_reset != 0 {
        tmp = RREG32(mmSRBM_SOFT_RESET); tmp |= srbm_soft_reset;
        dev_info((*adev).dev, "mmSRBM_SOFT_RESET=0x%08X\n", tmp); WREG32(mmSRBM_SOFT_RESET, tmp); tmp = RREG32(mmSRBM_SOFT_RESET);
        udelay(50); tmp &= !srbm_soft_reset; WREG32(mmSRBM_SOFT_RESET, tmp); tmp = RREG32(mmSRBM_SOFT_RESET); udelay(50);
    }
    0
}
unsafe fn si_ih_set_clockgating_state(_ip_block: *mut amdgpu_ip_block, _state: amd_clockgating_state) -> i32 { 0 }
unsafe fn si_ih_set_powergating_state(_ip_block: *mut amdgpu_ip_block, _state: amd_powergating_state) -> i32 { 0 }

static si_ih_ip_funcs: amd_ip_funcs = amd_ip_funcs {
    name: "si_ih", early_init: Some(si_ih_early_init), sw_init: Some(si_ih_sw_init), sw_fini: Some(si_ih_sw_fini),
    hw_init: Some(si_ih_hw_init), hw_fini: Some(si_ih_hw_fini), suspend: Some(si_ih_suspend), resume: Some(si_ih_resume),
    is_idle: Some(si_ih_is_idle), wait_for_idle: Some(si_ih_wait_for_idle), soft_reset: Some(si_ih_soft_reset),
    set_clockgating_state: Some(si_ih_set_clockgating_state), set_powergating_state: Some(si_ih_set_powergating_state),
};
static si_ih_funcs: amdgpu_ih_funcs = amdgpu_ih_funcs { get_wptr: Some(si_ih_get_wptr), decode_iv: Some(si_ih_decode_iv), set_rptr: Some(si_ih_set_rptr) };
unsafe fn si_ih_set_interrupt_funcs(adev: *mut amdgpu_device) { (*adev).irq.ih_funcs = &si_ih_funcs; }
static si_ih_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_IH, major: 1, minor: 0, rev: 0, funcs: &si_ih_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
