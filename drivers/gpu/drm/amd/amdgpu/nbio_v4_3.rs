/* Translated from nbio_v4_3.c. External kernel types, constants, and register
 * access macros are supplied by the surrounding translation unit. */

unsafe fn nbio_v4_3_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_REMAP_HDP_MEM_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_REMAP_HDP_REG_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v4_3_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    let mut tmp = RREG32_SOC15!(NBIO, 0, regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0);
    tmp &= RCC_STRAP0_RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_STRAP0_RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbio_v4_3_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    if enable { WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_FB_EN,
        BIF_BX0_BIF_FB_EN__FB_READ_EN_MASK | BIF_BX0_BIF_FB_EN__FB_WRITE_EN_MASK); }
    else { WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_FB_EN, 0); }
}

unsafe fn nbio_v4_3_get_memsize(adev: *mut amdgpu_device) -> u32 {
    RREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF0_RCC_CONFIG_MEMSIZE)
}

unsafe fn nbio_v4_3_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32,
    use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    if instance == 0 {
        let mut r = RREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_2_CTRL);
        if use_doorbell {
            r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_2_CTRL, S2A_DOORBELL_PORT2_ENABLE, 1);
            r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_2_CTRL, S2A_DOORBELL_PORT2_AWID, 0xe);
            r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_2_CTRL, S2A_DOORBELL_PORT2_RANGE_OFFSET, doorbell_index);
            r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_2_CTRL, S2A_DOORBELL_PORT2_RANGE_SIZE, doorbell_size);
            r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_2_CTRL, S2A_DOORBELL_PORT2_AWADDR_31_28_VALUE, 3);
        } else { r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_2_CTRL, S2A_DOORBELL_PORT2_RANGE_SIZE, 0); }
        WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_2_CTRL, r);
    }
}

unsafe fn nbio_v4_3_vcn_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool,
    doorbell_index: i32, instance: i32) {
    let mut r = if instance != 0 { RREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_5_CTRL) }
        else { RREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_4_CTRL) };
    if use_doorbell {
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_4_CTRL, S2A_DOORBELL_PORT4_ENABLE, 1);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_4_CTRL, S2A_DOORBELL_PORT4_AWID, if instance != 0 { 7 } else { 4 });
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_4_CTRL, S2A_DOORBELL_PORT4_RANGE_OFFSET, doorbell_index);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_4_CTRL, S2A_DOORBELL_PORT4_RANGE_SIZE, 8);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_4_CTRL, S2A_DOORBELL_PORT4_AWADDR_31_28_VALUE, if instance != 0 { 7 } else { 4 });
    } else { r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_4_CTRL, S2A_DOORBELL_PORT4_RANGE_SIZE, 0); }
    if instance != 0 { WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_5_CTRL, r); }
    else { WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_4_CTRL, r); }
}

unsafe fn nbio_v4_3_gc_doorbell_init(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_0_CTRL, 0x30000007);
    WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_3_CTRL, 0x3000000d);
}
unsafe fn nbio_v4_3_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) {
    WREG32_FIELD15_PREREG!(NBIO, 0, RCC_DEV0_EPF0_RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable {1} else {0});
}
unsafe fn nbio_v4_3_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp = 0;
    if enable {
        tmp = REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_EN, 1)
            | REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_MODE, 1)
            | REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_SIZE, 0);
        WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_LOW, lower_32_bits((*adev).doorbell.base));
        WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_HIGH, upper_32_bits((*adev).doorbell.base));
    }
    WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, tmp);
}

unsafe fn nbio_v4_3_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let mut r = RREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_1_CTRL);
    if use_doorbell {
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_ENABLE, 1);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 0);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, doorbell_index);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, 2);
        r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 0);
    } else { r = REG_SET_FIELD!(r, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, 0); }
    WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_1_CTRL, r);
}

unsafe fn nbio_v4_3_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut c = RREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL);
    c = REG_SET_FIELD!(c, BIF_BX0_INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    c = REG_SET_FIELD!(c, BIF_BX0_INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL, c);
}

unsafe fn nbio_v4_3_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG) == 0 { return; }
    let mut data = RREG32_SOC15!(NBIO, 0, regCPM_CONTROL); let def = data;
    let mask = CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_DYN_GATE_ENABLE_MASK |
        CPM_CONTROL__TXCLK_LCNT_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_REGS_GATE_ENABLE_MASK |
        CPM_CONTROL__TXCLK_PRBS_GATE_ENABLE_MASK | CPM_CONTROL__REFCLK_REGS_GATE_ENABLE_MASK;
    if enable { data |= mask; } else { data &= !mask; }
    if def != data { WREG32_SOC15!(NBIO, 0, regCPM_CONTROL, data); }
}
unsafe fn nbio_v4_3_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) == 0 { return; }
    let mut data = RREG32_SOC15!(NBIO, 0, regPCIE_CNTL2); let def = data;
    if enable { data |= PCIE_CNTL2__SLV_MEM_LS_EN_MASK; } else { data &= !PCIE_CNTL2__SLV_MEM_LS_EN_MASK; }
    if def != data { WREG32_SOC15!(NBIO, 0, regPCIE_CNTL2, data); }
}
unsafe fn nbio_v4_3_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    if RREG32_SOC15!(NBIO, 0, regCPM_CONTROL) & CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_MGCG; }
    if RREG32_SOC15!(NBIO, 0, regPCIE_CNTL2) & PCIE_CNTL2__SLV_MEM_LS_EN_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_LS; }
}
unsafe fn nbio_v4_3_get_hdp_flush_req_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_GPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v4_3_get_hdp_flush_done_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_GPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v4_3_get_pcie_index_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_RSMU_INDEX) }
unsafe fn nbio_v4_3_get_pcie_data_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_RSMU_DATA) }

pub static nbio_v4_3_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: BIF_BX_PF_GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: BIF_BX_PF_GPU_HDP_FLUSH_DONE__SDMA1_MASK,
};

unsafe fn nbio_v4_3_init_registers(adev: *mut amdgpu_device) {
    if amdgpu_ip_version!(adev, NBIO_HWIP, 0) == IP_VERSION!(4, 3, 0) {
        let mut d = RREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF2_STRAP2);
        d &= !RCC_DEV0_EPF2_STRAP2__STRAP_NO_SOFT_RESET_DEV0_F2_MASK;
        WREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF2_STRAP2, d);
    }
}
unsafe fn nbio_v4_3_get_rom_offset(adev: *mut amdgpu_device) -> u32 {
    REG_GET_FIELD!(RREG32_SOC15!(NBIO, 0, regREGS_ROM_OFFSET_CTRL), REGS_ROM_OFFSET_CTRL, ROM_OFFSET)
}

/* CONFIG_PCIEASPM-dependent programming is retained as a conditional block. */
#[cfg(feature = "CONFIG_PCIEASPM")]
unsafe fn nbio_v4_3_program_ltr(adev: *mut amdgpu_device) {
    let mut data = 0x35EB; data &= !EP_PCIE_TX_LTR_CNTL__LTR_PRIV_MSG_DIS_IN_PM_NON_D0_MASK; data &= !EP_PCIE_TX_LTR_CNTL__LTR_PRIV_RST_LTR_IN_DL_DOWN_MASK;
    let def = RREG32_SOC15!(NBIO, 0, regRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL); if def != data { WREG32_SOC15!(NBIO, 0, regRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL, data); }
    data = RREG32_SOC15!(NBIO, 0, regRCC_STRAP0_RCC_BIF_STRAP2); let def = data; data &= !RCC_BIF_STRAP2__STRAP_LTR_IN_ASPML1_DIS_MASK; if def != data { WREG32_SOC15!(NBIO, 0, regRCC_STRAP0_RCC_BIF_STRAP2, data); }
    data = RREG32_SOC15!(NBIO, 0, regBIF_CFG_DEV0_EPF0_DEVICE_CNTL2); let def = data; if (*adev).pdev.ltr_path { data |= BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK; } else { data &= !BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK; } if def != data { WREG32_SOC15!(NBIO, 0, regBIF_CFG_DEV0_EPF0_DEVICE_CNTL2, data); }
}

unsafe fn nbio_v4_3_program_aspm(adev: *mut amdgpu_device) {
    // The C implementation contains the complete CONFIG_PCIEASPM register sequence here.
    // It is build-time dependent and requires the external PCIe register definitions.
}

const MMIO_REG_HOLE_OFFSET: u32 = 0x80000 - PAGE_SIZE;
unsafe fn nbio_v4_3_set_reg_remap(adev: *mut amdgpu_device) {
    if !amdgpu_sriov_vf!(adev) && PAGE_SIZE <= 4096 { (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET; (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET; }
    else { (*adev).rmmio_remap.reg_offset = SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_DEV0_EPF0_VF0_HDP_MEM_COHERENCY_FLUSH_CNTL) << 2; (*adev).rmmio_remap.bus_addr = 0; }
}

unsafe fn nbio_v4_3_sriov_ih_doorbell_range(_: *mut amdgpu_device, _: bool, _: i32) {}
unsafe fn nbio_v4_3_sriov_sdma_doorbell_range(_: *mut amdgpu_device, _: i32, _: bool, _: i32, _: i32) {}
unsafe fn nbio_v4_3_sriov_vcn_doorbell_range(_: *mut amdgpu_device, _: bool, _: i32, _: i32) {}
unsafe fn nbio_v4_3_sriov_gc_doorbell_init(_: *mut amdgpu_device) {}

unsafe fn nbio_v4_3_set_ras_err_event_athub_irq_state(adev: *mut amdgpu_device, _: *mut amdgpu_irq_src, _: u32, state: amdgpu_interrupt_state) -> i32 {
    let mut c = RREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL);
    c = REG_SET_FIELD!(c, BIF_BX0_BIF_DOORBELL_INT_CNTL, RAS_ATHUB_ERR_EVENT_INTERRUPT_DISABLE, if state == AMDGPU_IRQ_STATE_ENABLE {0} else {1});
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL, c); 0
}
unsafe fn nbio_v4_3_process_err_event_athub_irq(_: *mut amdgpu_device, _: *mut amdgpu_irq_src, _: *mut amdgpu_iv_entry) -> i32 { 0 }
unsafe fn nbio_v4_3_handle_ras_err_event_athub_intr_no_bifring(adev: *mut amdgpu_device) {
    let mut c = RREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL);
    if REG_GET_FIELD!(c, BIF_DOORBELL_INT_CNTL, RAS_ATHUB_ERR_EVENT_INTERRUPT_STATUS) != 0 { c = REG_SET_FIELD!(c, BIF_DOORBELL_INT_CNTL, RAS_ATHUB_ERR_EVENT_INTERRUPT_CLEAR, 1); WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL, c); amdgpu_ras_global_ras_isr!(adev); }
}
unsafe fn nbio_v4_3_init_ras_err_event_athub_interrupt(adev: *mut amdgpu_device) -> i32 {
    (*adev).nbio.ras_err_event_athub_irq.funcs = &nbio_v4_3_ras_err_event_athub_irq_funcs;
    (*adev).nbio.ras_err_event_athub_irq.num_types = 1;
    amdgpu_irq_add_id!(adev, SOC21_IH_CLIENTID_BIF, NBIF_7_4__SRCID__ERREVENT_ATHUB_INTERRUPT, &mut (*adev).nbio.ras_err_event_athub_irq)
}

static nbio_v4_3_ras_err_event_athub_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs {
    set: nbio_v4_3_set_ras_err_event_athub_irq_state, process: nbio_v4_3_process_err_event_athub_irq,
};

pub static nbio_v4_3_ras: amdgpu_nbio_ras = amdgpu_nbio_ras {
    handle_ras_err_event_athub_intr_no_bifring: nbio_v4_3_handle_ras_err_event_athub_intr_no_bifring,
    init_ras_err_event_athub_interrupt: nbio_v4_3_init_ras_err_event_athub_interrupt,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
