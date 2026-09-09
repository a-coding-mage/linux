/* Translated from ih_v6_0.c. External kernel and driver symbols are provided by dependencies. */

const MAX_REARM_RETRY: i32 = 10;

unsafe fn ih_v6_0_init_register_offset(adev: *mut amdgpu_device) {
    if (*adev).irq.ih.ring_size != 0 {
        let r = &mut (*adev).irq.ih.ih_regs;
        r.ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_BASE);
        r.ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_BASE_HI);
        r.ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_CNTL);
        r.ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_WPTR);
        r.ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_RPTR);
        r.ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, regIH_DOORBELL_RPTR);
        r.ih_rb_wptr_addr_lo = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_WPTR_ADDR_LO);
        r.ih_rb_wptr_addr_hi = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_WPTR_ADDR_HI);
        r.psp_reg_id = PSP_REG_IH_RB_CNTL;
    }
    if (*adev).irq.ih1.ring_size != 0 {
        let r = &mut (*adev).irq.ih1.ih_regs;
        r.ih_rb_base = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_BASE_RING1);
        r.ih_rb_base_hi = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_BASE_HI_RING1);
        r.ih_rb_cntl = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_CNTL_RING1);
        r.ih_rb_wptr = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_WPTR_RING1);
        r.ih_rb_rptr = SOC15_REG_OFFSET(OSSSYS, 0, regIH_RB_RPTR_RING1);
        r.ih_doorbell_rptr = SOC15_REG_OFFSET(OSSSYS, 0, regIH_DOORBELL_RPTR_RING1);
        r.psp_reg_id = PSP_REG_IH_RB_CNTL_RING1;
    }
}

unsafe fn force_update_wptr_for_self_int(adev: *mut amdgpu_device, threshold: u32, timeout: u32, enabled: bool) {
    let mut ih_cntl = RREG32_SOC15(OSSSYS, 0, regIH_CNTL2);
    let mut rb = RREG32_SOC15(OSSSYS, 0, regIH_RB_CNTL_RING1);
    ih_cntl = REG_SET_FIELD(ih_cntl, IH_CNTL2, SELF_IV_FORCE_WPTR_UPDATE_TIMEOUT, timeout);
    ih_cntl = REG_SET_FIELD(ih_cntl, IH_CNTL2, SELF_IV_FORCE_WPTR_UPDATE_ENABLE, enabled);
    rb = REG_SET_FIELD(rb, IH_RB_CNTL_RING1, RB_USED_INT_THRESHOLD, threshold);
    if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) {
        if psp_reg_program(&mut (*adev).psp, PSP_REG_IH_RB_CNTL_RING1, rb) != 0 { return; }
    } else { WREG32_SOC15(OSSSYS, 0, regIH_RB_CNTL_RING1, rb); }
    WREG32_SOC15(OSSSYS, 0, regIH_CNTL2, ih_cntl);
}

unsafe fn ih_v6_0_toggle_ring_interrupts(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, enable: bool) -> i32 {
    let r = &(*ih).ih_regs;
    let mut tmp = RREG32(r.ih_rb_cntl);
    tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_ENABLE, if enable { 1 } else { 0 });
    if enable {
        tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0);
        if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, r.psp_reg_id, tmp) != 0 { return -ETIMEDOUT; } } else { WREG32_NO_KIQ(r.ih_rb_cntl, tmp); }
        tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1);
        if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, r.psp_reg_id, tmp) != 0 { return -ETIMEDOUT; } } else { WREG32_NO_KIQ(r.ih_rb_cntl, tmp); }
        tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0);
    }
    if ih == &mut (*adev).irq.ih { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, ENABLE_INTR, if enable { 1 } else { 0 }); }
    if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, r.psp_reg_id, tmp) != 0 { return -ETIMEDOUT; } } else { WREG32(r.ih_rb_cntl, tmp); }
    if enable { (*ih).enabled = true; } else { WREG32(r.ih_rb_rptr, 0); WREG32(r.ih_rb_wptr, 0); (*ih).enabled = false; (*ih).rptr = 0; }
    0
}

unsafe fn ih_v6_0_toggle_interrupts(adev: *mut amdgpu_device, enable: bool) -> i32 {
    let rings = [&mut (*adev).irq.ih as *mut _, &mut (*adev).irq.ih1 as *mut _];
    for ih in rings { if (*ih).ring_size != 0 { let r = ih_v6_0_toggle_ring_interrupts(adev, ih, enable); if r != 0 { return r; } } }
    0
}

unsafe fn ih_v6_0_rb_cntl(ih: *mut amdgpu_ih_ring, mut v: u32) -> u32 {
    let size = order_base_2((*ih).ring_size / 4);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_SPACE, if (*ih).use_bus_addr { 2 } else { 4 });
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_OVERFLOW_ENABLE, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, RB_SIZE, size);
    v = REG_SET_FIELD(v, IH_RB_CNTL, WPTR_WRITEBACK_ENABLE, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_SNOOP, 1);
    v = REG_SET_FIELD(v, IH_RB_CNTL, MC_RO, 0);
    REG_SET_FIELD(v, IH_RB_CNTL, MC_VMID, 0)
}

unsafe fn ih_v6_0_doorbell_rptr(ih: *mut amdgpu_ih_ring) -> u32 {
    let mut v = 0;
    if (*ih).use_doorbell { v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, OFFSET, (*ih).doorbell_index); v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, ENABLE, 1); } else { v = REG_SET_FIELD(v, IH_DOORBELL_RPTR, ENABLE, 0); }
    v
}

unsafe fn ih_v6_0_enable_ring(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> i32 {
    let r = &(*ih).ih_regs;
    WREG32(r.ih_rb_base, (*ih).gpu_addr >> 8); WREG32(r.ih_rb_base_hi, ((*ih).gpu_addr >> 40) & 0xff);
    let mut tmp = ih_v6_0_rb_cntl(ih, RREG32(r.ih_rb_cntl));
    if ih == &mut (*adev).irq.ih { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RPTR_REARM, if (*adev).irq.msi_enabled { 1 } else { 0 }); }
    if ih == &mut (*adev).irq.ih1 { tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, WPTR_OVERFLOW_ENABLE, 0); tmp = REG_SET_FIELD(tmp, IH_RB_CNTL, RB_FULL_DRAIN_ENABLE, 1); }
    if amdgpu_sriov_vf(adev) && amdgpu_sriov_reg_indirect_ih(adev) { if psp_reg_program(&mut (*adev).psp, r.psp_reg_id, tmp) != 0 { DRM_ERROR!("PSP program IH_RB_CNTL failed!\n"); return -ETIMEDOUT; } } else { WREG32(r.ih_rb_cntl, tmp); }
    if ih == &mut (*adev).irq.ih { WREG32(r.ih_rb_wptr_addr_lo, lower_32_bits((*ih).wptr_addr)); WREG32(r.ih_rb_wptr_addr_hi, upper_32_bits((*ih).wptr_addr) & 0xFFFF); }
    WREG32(r.ih_rb_wptr, 0); WREG32(r.ih_rb_rptr, 0); WREG32(r.ih_doorbell_rptr, ih_v6_0_doorbell_rptr(ih)); 0
}

unsafe fn ih_v6_0_irq_init(adev: *mut amdgpu_device) -> i32 {
    let rings = [&mut (*adev).irq.ih as *mut _, &mut (*adev).irq.ih1 as *mut _];
    let mut r = ih_v6_0_toggle_interrupts(adev, false); if r != 0 { return r; }
    ((*adev).nbio.funcs.as_ref().unwrap().ih_control)(adev);
    if unlikely((*adev).firmware.load_type == AMDGPU_FW_LOAD_DIRECT || (*adev).firmware.load_type == AMDGPU_FW_LOAD_RLC_BACKDOOR_AUTO) && (*rings[0]).use_bus_addr { let mut c = RREG32_SOC15(OSSSYS, 0, regIH_CHICKEN); c = REG_SET_FIELD(c, IH_CHICKEN, MC_SPACE_GPA_ENABLE, 1); WREG32_SOC15(OSSSYS, 0, regIH_CHICKEN, c); }
    for ih in rings { if (*ih).ring_size != 0 { r = ih_v6_0_enable_ring(adev, ih); if r != 0 { return r; } } (*ih).overflow = false; }
    ((*adev).nbio.funcs.as_ref().unwrap().ih_doorbell_range)(adev, (*rings[0]).use_doorbell, (*rings[0]).doorbell_index);
    let mut t = RREG32_SOC15(OSSSYS, 0, regIH_STORM_CLIENT_LIST_CNTL); t = REG_SET_FIELD(t, IH_STORM_CLIENT_LIST_CNTL, CLIENT18_IS_STORM_CLIENT, 1); WREG32_SOC15(OSSSYS, 0, regIH_STORM_CLIENT_LIST_CNTL, t);
    t = RREG32_SOC15(OSSSYS, 0, regIH_INT_FLOOD_CNTL); t = REG_SET_FIELD(t, IH_INT_FLOOD_CNTL, FLOOD_CNTL_ENABLE, 1); WREG32_SOC15(OSSSYS, 0, regIH_INT_FLOOD_CNTL, t);
    t = RREG32_SOC15(OSSSYS, 0, regIH_MSI_STORM_CTRL); t = REG_SET_FIELD(t, IH_MSI_STORM_CTRL, DELAY, 3); WREG32_SOC15(OSSSYS, 0, regIH_MSI_STORM_CTRL, t);
    if (*adev).irq.ih1.ring_size != 0 { t = RREG32_SOC15(OSSSYS, 0, regIH_RING1_CLIENT_CFG_INDEX); t = REG_SET_FIELD(t, IH_RING1_CLIENT_CFG_INDEX, INDEX, 0); WREG32_SOC15(OSSSYS, 0, regIH_RING1_CLIENT_CFG_INDEX, t); t = RREG32_SOC15(OSSSYS, 0, regIH_RING1_CLIENT_CFG_DATA); t = REG_SET_FIELD(t, IH_RING1_CLIENT_CFG_DATA, CLIENT_ID, 0xa); t = REG_SET_FIELD(t, IH_RING1_CLIENT_CFG_DATA, SOURCE_ID, 0); t = REG_SET_FIELD(t, IH_RING1_CLIENT_CFG_DATA, SOURCE_ID_MATCH_ENABLE, 1); WREG32_SOC15(OSSSYS, 0, regIH_RING1_CLIENT_CFG_DATA, t); }
    pci_set_master((*adev).pdev); r = ih_v6_0_toggle_interrupts(adev, true); if r != 0 { return r; } force_update_wptr_for_self_int(adev, 0, 8, true); if (*adev).irq.ih_soft.ring_size != 0 { (*adev).irq.ih_soft.enabled = true; } 0
}

unsafe fn ih_v6_0_irq_disable(adev: *mut amdgpu_device) { force_update_wptr_for_self_int(adev, 0, 8, false); ih_v6_0_toggle_interrupts(adev, false); mdelay(1); }

unsafe fn ih_v6_0_get_wptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> u32 {
    let mut wptr = le32_to_cpu(*(*ih).wptr_cpu); let r = &(*ih).ih_regs;
    if REG_GET_FIELD(wptr, IH_RB_WPTR, RB_OVERFLOW) == 0 { return wptr & (*ih).ptr_mask; }
    wptr = RREG32_NO_KIQ(r.ih_rb_wptr); if REG_GET_FIELD(wptr, IH_RB_WPTR, RB_OVERFLOW) != 0 { if !amdgpu_sriov_vf(adev) { wptr = REG_SET_FIELD(wptr, IH_RB_WPTR, RB_OVERFLOW, 0); } else { (*ih).overflow = true; } let tmp = (wptr + 32) & (*ih).ptr_mask; dev_warn!((*adev).dev, "IH ring buffer overflow (0x%08X, 0x%08X, 0x%08X)\n", wptr, (*ih).rptr, tmp); (*ih).rptr = tmp; let mut c = RREG32_NO_KIQ(r.ih_rb_cntl); c = REG_SET_FIELD(c, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 1); WREG32_NO_KIQ(r.ih_rb_cntl, c); c = REG_SET_FIELD(c, IH_RB_CNTL, WPTR_OVERFLOW_CLEAR, 0); WREG32_NO_KIQ(r.ih_rb_cntl, c); } wptr & (*ih).ptr_mask
}

unsafe fn ih_v6_0_irq_rearm(_adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) { let r = &(*ih).ih_regs; for _ in 0..MAX_REARM_RETRY { let v = RREG32_NO_KIQ(r.ih_rb_rptr); if v < (*ih).ring_size && v != (*ih).rptr { WDOORBELL32((*ih).doorbell_index, (*ih).rptr); } else { break; } } }
unsafe fn ih_v6_0_set_rptr(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) { if (*ih).use_doorbell { *(*ih).rptr_cpu = (*ih).rptr; WDOORBELL32((*ih).doorbell_index, (*ih).rptr); if amdgpu_sriov_vf(adev) { ih_v6_0_irq_rearm(adev, ih); } } else { WREG32((*ih).ih_regs.ih_rb_rptr, (*ih).rptr); } }

unsafe fn ih_v6_0_self_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 { let wptr = cpu_to_le32((*entry).src_data[0]); if (*entry).ring_id == 1 { *(*adev).irq.ih1.wptr_cpu = wptr; schedule_work(&mut (*adev).irq.ih1_work); } 0 }

unsafe fn ih_v6_0_early_init(ip: *mut amdgpu_ip_block) -> i32 { ih_v6_0_set_interrupt_funcs((*ip).adev); ih_v6_0_set_self_irq_funcs((*ip).adev); 0 }
unsafe fn ih_v6_0_sw_init(ip: *mut amdgpu_ip_block) -> i32 { let adev = (*ip).adev; let mut r = amdgpu_irq_add_id(adev, SOC21_IH_CLIENTID_IH, 0, &mut (*adev).irq.self_irq); if r != 0 { return r; } let bus = (*adev).firmware.load_type != AMDGPU_FW_LOAD_PSP; r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih, IH_RING_SIZE, bus); if r != 0 { return r; } (*adev).irq.ih.use_doorbell = true; (*adev).irq.ih.doorbell_index = (*adev).doorbell_index.ih << 1; if (*adev).flags & AMD_IS_APU == 0 { r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih1, IH_RING_SIZE, bus); if r != 0 { return r; } (*adev).irq.ih1.use_doorbell = true; (*adev).irq.ih1.doorbell_index = ((*adev).doorbell_index.ih + 1) << 1; } ih_v6_0_init_register_offset(adev); r = amdgpu_ih_ring_init(adev, &mut (*adev).irq.ih_soft, IH_SW_RING_SIZE, true); if r != 0 { return r; } amdgpu_irq_init(adev) }
unsafe fn ih_v6_0_sw_fini(ip: *mut amdgpu_ip_block) -> i32 { amdgpu_irq_fini_sw((*ip).adev); 0 }
unsafe fn ih_v6_0_hw_init(ip: *mut amdgpu_ip_block) -> i32 { ih_v6_0_irq_init((*ip).adev) }
unsafe fn ih_v6_0_hw_fini(ip: *mut amdgpu_ip_block) -> i32 { ih_v6_0_irq_disable((*ip).adev); 0 }
unsafe fn ih_v6_0_suspend(ip: *mut amdgpu_ip_block) -> i32 { ih_v6_0_hw_fini(ip) }
unsafe fn ih_v6_0_resume(ip: *mut amdgpu_ip_block) -> i32 { ih_v6_0_hw_init(ip) }
unsafe fn ih_v6_0_is_idle(_ip: *mut amdgpu_ip_block) -> bool { true }
unsafe fn ih_v6_0_wait_for_idle(_ip: *mut amdgpu_ip_block) -> i32 { -ETIMEDOUT }
unsafe fn ih_v6_0_soft_reset(_ip: *mut amdgpu_ip_block) -> i32 { 0 }

unsafe fn ih_v6_0_update_clockgating_state(adev: *mut amdgpu_device, enable: bool) { if (*adev).cg_flags & AMD_CG_SUPPORT_IH_CG != 0 { let mut data = RREG32_SOC15(OSSSYS, 0, regIH_CLK_CTRL); let old = data; let v = if enable { 0 } else { 1 }; data = REG_SET_FIELD(data, IH_CLK_CTRL, DBUS_MUX_CLK_SOFT_OVERRIDE, v); data = REG_SET_FIELD(data, IH_CLK_CTRL, OSSSYS_SHARE_CLK_SOFT_OVERRIDE, v); data = REG_SET_FIELD(data, IH_CLK_CTRL, LIMIT_SMN_CLK_SOFT_OVERRIDE, v); data = REG_SET_FIELD(data, IH_CLK_CTRL, DYN_CLK_SOFT_OVERRIDE, v); data = REG_SET_FIELD(data, IH_CLK_CTRL, REG_CLK_SOFT_OVERRIDE, v); if old != data { WREG32_SOC15(OSSSYS, 0, regIH_CLK_CTRL, data); } } }
unsafe fn ih_v6_0_set_clockgating_state(ip: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32 { ih_v6_0_update_clockgating_state((*ip).adev, state == AMD_CG_STATE_GATE); 0 }
unsafe fn ih_v6_0_update_ih_mem_power_gating(adev: *mut amdgpu_device, enable: bool) { let mut v = RREG32_SOC15(OSSSYS, 0, regIH_MEM_POWER_CTRL); v = REG_SET_FIELD(v, IH_MEM_POWER_CTRL, IH_BUFFER_MEM_POWER_CTRL_EN, 0); WREG32_SOC15(OSSSYS, 0, regIH_MEM_POWER_CTRL, v); for f in [IH_BUFFER_MEM_POWER_LS_EN, IH_BUFFER_MEM_POWER_DS_EN, IH_BUFFER_MEM_POWER_SD_EN, IH_RETRY_INT_CAM_MEM_POWER_LS_EN, IH_RETRY_INT_CAM_MEM_POWER_DS_EN, IH_RETRY_INT_CAM_MEM_POWER_SD_EN] { v = REG_SET_FIELD(v, IH_MEM_POWER_CTRL, f, 0); } if enable { v = REG_SET_FIELD(v, IH_MEM_POWER_CTRL, IH_BUFFER_MEM_POWER_DS_EN, 1); v = REG_SET_FIELD(v, IH_MEM_POWER_CTRL, IH_RETRY_INT_CAM_MEM_POWER_DS_EN, 1); } v = REG_SET_FIELD(v, IH_MEM_POWER_CTRL, IH_BUFFER_MEM_POWER_CTRL_EN, 1); WREG32_SOC15(OSSSYS, 0, regIH_MEM_POWER_CTRL, v); }
unsafe fn ih_v6_0_set_powergating_state(ip: *mut amdgpu_ip_block, state: amd_powergating_state) -> i32 { let adev = (*ip).adev; if (*adev).pg_flags & AMD_PG_SUPPORT_IH_SRAM_PG != 0 { ih_v6_0_update_ih_mem_power_gating(adev, state == AMD_PG_STATE_GATE); } 0 }
unsafe fn ih_v6_0_get_clockgating_state(ip: *mut amdgpu_ip_block, flags: *mut u64) { if RREG32_SOC15(OSSSYS, 0, regIH_CLK_CTRL) == 0 { *flags |= AMD_CG_SUPPORT_IH_CG as u64; } }

unsafe fn ih_v6_0_set_self_irq_funcs(adev: *mut amdgpu_device) { (*adev).irq.self_irq.num_types = 0; (*adev).irq.self_irq.funcs = &ih_v6_0_self_irq_funcs; }
unsafe fn ih_v6_0_set_interrupt_funcs(adev: *mut amdgpu_device) { (*adev).irq.ih_funcs = &ih_v6_0_funcs; }

static ih_v6_0_self_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { process: Some(ih_v6_0_self_irq) };
static ih_v6_0_funcs: amdgpu_ih_funcs = amdgpu_ih_funcs { get_wptr: Some(ih_v6_0_get_wptr), decode_iv: Some(amdgpu_ih_decode_iv_helper), decode_iv_ts: Some(amdgpu_ih_decode_iv_ts_helper), set_rptr: Some(ih_v6_0_set_rptr) };
static ih_v6_0_ip_funcs: amd_ip_funcs = amd_ip_funcs { name: "ih_v6_0", early_init: Some(ih_v6_0_early_init), sw_init: Some(ih_v6_0_sw_init), sw_fini: Some(ih_v6_0_sw_fini), hw_init: Some(ih_v6_0_hw_init), hw_fini: Some(ih_v6_0_hw_fini), suspend: Some(ih_v6_0_suspend), resume: Some(ih_v6_0_resume), is_idle: Some(ih_v6_0_is_idle), wait_for_idle: Some(ih_v6_0_wait_for_idle), soft_reset: Some(ih_v6_0_soft_reset), set_clockgating_state: Some(ih_v6_0_set_clockgating_state), set_powergating_state: Some(ih_v6_0_set_powergating_state), get_clockgating_state: Some(ih_v6_0_get_clockgating_state) };
static ih_v6_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_IH, major: 6, minor: 0, rev: 0, funcs: &ih_v6_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
