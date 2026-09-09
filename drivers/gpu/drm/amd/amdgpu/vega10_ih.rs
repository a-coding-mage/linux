/* Direct Rust translation of vega10_ih.c. */

const MAX_REARM_RETRY: u32 = 10;

unsafe fn vega10_ih_init_register_offset(adev: *mut amdgpu_device) {
    let mut ih_regs: *mut amdgpu_ih_regs;
    if (*(*adev).irq.ih).ring_size != 0 { ih_regs = &mut (*(*adev).irq.ih).ih_regs; (*ih_regs).ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE); (*ih_regs).ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_HI); (*ih_regs).ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_CNTL); (*ih_regs).ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR); (*ih_regs).ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_RPTR); (*ih_regs).ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_DOORBELL_RPTR); (*ih_regs).ih_rb_wptr_addr_lo = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_ADDR_LO); (*ih_regs).ih_rb_wptr_addr_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_ADDR_HI); (*ih_regs).psp_reg_id = PSP_REG_IH_RB_CNTL; }
    if (*(*adev).irq.ih1).ring_size != 0 { ih_regs = &mut (*(*adev).irq.ih1).ih_regs; (*ih_regs).ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_RING1); (*ih_regs).ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_HI_RING1); (*ih_regs).ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_CNTL_RING1); (*ih_regs).ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_RING1); (*ih_regs).ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_RPTR_RING1); (*ih_regs).ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_DOORBELL_RPTR_RING1); (*ih_regs).psp_reg_id = PSP_REG_IH_RB_CNTL_RING1; }
    if (*(*adev).irq.ih2).ring_size != 0 { ih_regs = &mut (*(*adev).irq.ih2).ih_regs; (*ih_regs).ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_RING2); (*ih_regs).ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_BASE_HI_RING2); (*ih_regs).ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_CNTL_RING2); (*ih_regs).ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_WPTR_RING2); (*ih_regs).ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_RB_RPTR_RING2); (*ih_regs).ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, mmIH_DOORBELL_RPTR_RING2); (*ih_regs).psp_reg_id = PSP_REG_IH_RB_CNTL_RING2; }
}

unsafe fn vega10_ih_toggle_ring_interrupts(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, enable: bool) -> i32 {
    let regs = &mut (*ih).ih_regs; let mut tmp = RREG32(regs.ih_rb_cntl);
    tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_ENABLE, if enable { 1 } else { 0 });
    tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_GPU_TS_ENABLE, 1);
    if ih == (*adev).irq.ih { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, ENABLE_INTR, if enable { 1 } else { 0 }); }
    if amdgpu_sriov_vf(adev) { if psp_reg_program(&mut (*adev).psp, regs.psp_reg_id, tmp) != 0 { dev_err((*adev).dev, "PSP program IH_RB_CNTL failed!\n"); return -ETIMEDOUT; } } else { WREG32(regs.ih_rb_cntl, tmp); }
    if enable { (*ih).enabled = true; } else { WREG32(regs.ih_rb_rptr, 0); WREG32(regs.ih_rb_wptr, 0); (*ih).enabled = false; (*ih).rptr = 0; } 0
}

unsafe fn vega10_ih_toggle_interrupts(adev: *mut amdgpu_device, enable: bool) -> i32 {
    let ih = [(*adev).irq.ih, (*adev).irq.ih1, (*adev).irq.ih2];
    for ring in ih { if (*ring).ring_size != 0 { let r = vega10_ih_toggle_ring_interrupts(adev, ring, enable); if r != 0 { return r; } } } 0
}

unsafe fn vega10_ih_rb_cntl(ih: *mut amdgpu_ih_ring, mut v: u32) -> u32 {
    let rb_bufsz = order_base_2((*ih).ring_size / 4);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_SPACE, if (*ih).use_bus_addr { 1 } else { 4 });
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1); v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_OVERFLOW_ENABLE, 1); v = REG_SET_FIELD(v, IH_RB_CNTL, RB_SIZE, rb_bufsz); v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_WRITEBACK_ENABLE, 1); v = REG_SET_FIELD(v, IH_RB_CNTL, MC_SNOOP, 1); v = REG_SET_FIELD(v, IH_RB_CNTL, MC_RO, 0); REG_SET_FIELD(v, IH_RB_CNTL, MC_VMID, 0)
}

unsafe fn vega10_ih_doorbell_rptr(ih: *mut amdgpu_ih_ring) -> u32 {
    let mut v = 0; if (*ih).use_doorbell { v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, OFFSET, (*ih).doorbell_index); v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, ENABLE, 1); } else { v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, ENABLE, 0); } v
}

unsafe fn vega10_ih_enable_ring(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> i32 {
    let regs = &mut (*ih).ih_regs; WREG32(regs.ih_rb_base, (*ih).gpu_addr >> 8); WREG32(regs.ih_rb_base_hi, ((*ih).gpu_addr >> 40) & 0xff);
    let mut tmp = vega10_ih_rb_cntl(ih, RREG32(regs.ih_rb_cntl)); if ih == (*adev).irq.ih { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RPTR_REARM, if (*adev).irq.msi_enabled { 1 } else { 0 }); } if ih == (*adev).irq.ih1 { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_FULL_DRAIN_ENABLE, 1); }
    if amdgpu_sriov_vf(adev) { if psp_reg_program(&mut (*adev).psp, regs.psp_reg_id, tmp) != 0 { dev_err((*adev).dev, "PSP program IH_RB_CNTL failed!\n"); return -ETIMEDOUT; } } else { WREG32(regs.ih_rb_cntl, tmp); }
    if ih == (*adev).irq.ih { WREG32(regs.ih_rb_wptr_addr_lo, lower_32_bits((*ih).wptr_addr)); WREG32(regs.ih_rb_wptr_addr_hi, upper_32_bits((*ih).wptr_addr) & 0xffff); } WREG32(regs.ih_rb_wptr, 0); WREG32(regs.ih_rb_rptr, 0); WREG32(regs.ih_doorbell_rptr, vega10_ih_doorbell_rptr(ih)); 0
}

unsafe fn vega10_ih_irq_init(adev: *mut amdgpu_device) -> i32 {
    let rings = [(*adev).irq.ih, (*adev).irq.ih1, (*adev).irq.ih2]; let mut r = vega10_ih_toggle_interrupts(adev, false); if r != 0 { return r; } ((*adev).nbio.funcs).ih_control(adev);
    if (*adev).asic_type == CHIP_RENOIR { let mut c = RREG32_SOC15(OSSSYS, 0, mmIH_CHICKEN); if (*(*adev).irq.ih).use_bus_addr { c = REG_SET_FIELD(c, IH_CHICKEN, MC_SPACE_GPA_ENABLE, 1); } WREG32_SOC15(OSSSYS, 0, mmIH_CHICKEN, c); }
    for ih in rings { if (*ih).ring_size != 0 { r = vega10_ih_enable_ring(adev, ih); if r != 0 { return r; } } } if !amdgpu_sriov_vf(adev) { ((*adev).nbio.funcs).ih_doorbell_range(adev, (*(*adev).irq.ih).use_doorbell, (*(*adev).irq.ih).doorbell_index); } pci_set_master((*adev).pdev); r = vega10_ih_toggle_interrupts(adev, true); if r != 0 { return r; } if (*(*adev).irq.ih_soft).ring_size != 0 { (*(*adev).irq.ih_soft).enabled = true; } 0
}

unsafe fn vega10_ih_irq_disable(adev: *mut amdgpu_device) { vega10_ih_toggle_interrupts(adev, false); mdelay(1); }

unsafe fn vega10_ih_get_wptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> u32 {
    let mut wptr: u32; let mut tmp: u32; let regs = &mut (*ih).ih_regs;
    if ih == (*adev).irq.ih || ih == (*adev).irq.ih_soft { wptr = le32_to_cpu(*(*ih).wptr_cpu); if REG_GET_FIELD(wptr, IH_RB_WPTR, RB_OVERFLOW) == 0 { return wptr & (*ih).ptr_mask; } }
    wptr = RREG32_NO_KIQ(regs.ih_rb_wptr); if REG_GET_FIELD(wptr, IH_RB_WPTR, RB_OVERFLOW) == 0 { return wptr & (*ih).ptr_mask; } wptr = REG_SET_FIELD(wptr, IH_RB_WPTR, RB_OVERFLOW, 0); tmp = (wptr + 32) & (*ih).ptr_mask; dev_warn_ratelimited((*adev).dev, "%s ring buffer overflow (0x%08X, 0x%08X, 0x%08X)\n", amdgpu_ih_ring_name(adev, ih), wptr, (*ih).rptr, tmp); (*ih).rptr = tmp; tmp = REG_SET_FIELD(RREG32_NO_KIQ(regs.ih_rb_cntl), IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1); WREG32_NO_KIQ(regs.ih_rb_cntl, tmp); tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0); WREG32_NO_KIQ(regs.ih_rb_cntl, tmp); wptr & (*ih).ptr_mask
}

unsafe fn vega10_ih_irq_rearm(_adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) { let regs = &mut (*ih).ih_regs; for _ in 0..MAX_REARM_RETRY { let v = RREG32_NO_KIQ(regs.ih_rb_rptr); if v < (*ih).ring_size && v != (*ih).rptr { WDOORBELL32((*ih).doorbell_index, (*ih).rptr); } else { break; } } }
unsafe fn vega10_ih_set_rptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) { if ih == (*adev).irq.ih_soft { return; } if (*ih).use_doorbell { *(*ih).rptr_cpu = (*ih).rptr; WDOORBELL32((*ih).doorbell_index, (*ih).rptr); if amdgpu_sriov_vf(adev) { vega10_ih_irq_rearm(adev, ih); } } else { WREG32((*ih).ih_regs.ih_rb_rptr, (*ih).rptr); } }

unsafe fn vega10_ih_self_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 { match (*entry).ring_id { 1 => schedule_work(&mut (*adev).irq.ih1_work), 2 => schedule_work(&mut (*adev).irq.ih2_work), _ => {} }; 0 }
static vega10_ih_self_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { process: Some(vega10_ih_self_irq) };
unsafe fn vega10_ih_set_self_irq_funcs(adev: *mut amdgpu_device) { (*adev).irq.self_irq.num_types = 0; (*adev).irq.self_irq.funcs = &vega10_ih_self_irq_funcs; }
unsafe fn vega10_ih_early_init(ip: *mut amdgpu_ip_block) -> i32 { vega10_ih_set_interrupt_funcs((*ip).adev); vega10_ih_set_self_irq_funcs((*ip).adev); 0 }

unsafe fn vega10_ih_sw_init(ip: *mut amdgpu_ip_block) -> i32 { let adev = (*ip).adev; let mut r = amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_IH, 0, &mut (*adev).irq.self_irq); if r != 0 { return r; } r = amdgpu_ih_ring_init(adev, (*adev).irq.ih, IH_RING_SIZE, true); if r != 0 { return r; } (*(*adev).irq.ih).use_doorbell = true; (*(*adev).irq.ih).doorbell_index = (*adev).doorbell_index.ih << 1; if (*adev).flags & AMD_IS_APU == 0 { r = amdgpu_ih_ring_init(adev, (*adev).irq.ih1, PAGE_SIZE, true); if r != 0 { return r; } (*(*adev).irq.ih1).use_doorbell = true; (*(*adev).irq.ih1).doorbell_index = ((*adev).doorbell_index.ih + 1) << 1; r = amdgpu_ih_ring_init(adev, (*adev).irq.ih2, PAGE_SIZE, true); if r != 0 { return r; } (*(*adev).irq.ih2).use_doorbell = true; (*(*adev).irq.ih2).doorbell_index = ((*adev).doorbell_index.ih + 2) << 1; } vega10_ih_init_register_offset(adev); r = amdgpu_ih_ring_init(adev, (*adev).irq.ih_soft, IH_SW_RING_SIZE, true); if r != 0 { return r; } amdgpu_irq_init(adev) }
unsafe fn vega10_ih_sw_fini(ip: *mut amdgpu_ip_block) -> i32 { amdgpu_irq_fini_sw((*ip).adev); 0 }
unsafe fn vega10_ih_hw_init(ip: *mut amdgpu_ip_block) -> i32 { vega10_ih_irq_init((*ip).adev) }
unsafe fn vega10_ih_hw_fini(ip: *mut amdgpu_ip_block) -> i32 { vega10_ih_irq_disable((*ip).adev); 0 }
unsafe fn vega10_ih_suspend(ip: *mut amdgpu_ip_block) -> i32 { vega10_ih_hw_fini(ip) }
unsafe fn vega10_ih_resume(ip: *mut amdgpu_ip_block) -> i32 { vega10_ih_hw_init(ip) }
unsafe fn vega10_ih_is_idle(_ip: *mut amdgpu_ip_block) -> bool { true }
unsafe fn vega10_ih_wait_for_idle(_ip: *mut amdgpu_ip_block) -> i32 { -ETIMEDOUT }
unsafe fn vega10_ih_soft_reset(_ip: *mut amdgpu_ip_block) -> i32 { 0 }
unsafe fn vega10_ih_update_clockgating_state(adev: *mut amdgpu_device, enable: bool) { if (*adev).cg_flags & AMD_CG_SUPPORT_IH_CG != 0 { let mut data = RREG32_SOC15(OSSSYS, 0, mmIH_CLK_CTRL); let def = data; let f = if enable { 0 } else { 1 }; if (*adev).asic_type == CHIP_RENOIR { data = REG_SET_FIELD(data, IH_CLK_CTRL, IH_BUFFER_MEM_CLK_SOFT_OVERRIDE, f); } data = REG_SET_FIELD(data, IH_CLK_CTRL, DBUS_MUX_CLK_SOFT_OVERRIDE, f); data = REG_SET_FIELD(data, IH_CLK_CTRL, OSSSYS_SHARE_CLK_SOFT_OVERRIDE, f); data = REG_SET_FIELD(data, IH_CLK_CTRL, LIMIT_SMN_CLK_SOFT_OVERRIDE, f); data = REG_SET_FIELD(data, IH_CLK_CTRL, DYN_CLK_SOFT_OVERRIDE, f); data = REG_SET_FIELD(data, IH_CLK_CTRL, REG_CLK_SOFT_OVERRIDE, f); if def != data { WREG32_SOC15(OSSSYS, 0, mmIH_CLK_CTRL, data); } } }
unsafe fn vega10_ih_set_clockgating_state(ip: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32 { vega10_ih_update_clockgating_state((*ip).adev, state == AMD_CG_STATE_GATE); 0 }
unsafe fn vega10_ih_set_powergating_state(_ip: *mut amdgpu_ip_block, _state: amd_powergating_state) -> i32 { 0 }

static vega10_ih_ip_funcs: amd_ip_funcs = amd_ip_funcs { name: "vega10_ih", early_init: Some(vega10_ih_early_init), sw_init: Some(vega10_ih_sw_init), sw_fini: Some(vega10_ih_sw_fini), hw_init: Some(vega10_ih_hw_init), hw_fini: Some(vega10_ih_hw_fini), suspend: Some(vega10_ih_suspend), resume: Some(vega10_ih_resume), is_idle: Some(vega10_ih_is_idle), wait_for_idle: Some(vega10_ih_wait_for_idle), soft_reset: Some(vega10_ih_soft_reset), set_clockgating_state: Some(vega10_ih_set_clockgating_state), set_powergating_state: Some(vega10_ih_set_powergating_state) };
static vega10_ih_funcs: amdgpu_ih_funcs = amdgpu_ih_funcs { get_wptr: Some(vega10_ih_get_wptr), decode_iv: Some(amdgpu_ih_decode_iv_helper), decode_iv_ts: Some(amdgpu_ih_decode_iv_ts_helper), set_rptr: Some(vega10_ih_set_rptr) };
unsafe fn vega10_ih_set_interrupt_funcs(adev: *mut amdgpu_device) { (*adev).irq.ih_funcs = &vega10_ih_funcs; }
static vega10_ih_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_IH, major: 4, minor: 0, rev: 0, funcs: &vega10_ih_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
