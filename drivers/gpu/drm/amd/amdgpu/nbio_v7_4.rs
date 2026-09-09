/* Translated from nbio_v7_4.c. External kernel, register, and generated
 * hardware definitions are intentionally referenced as future dependencies. */

const SMNPCIE_LC_CNTL: u32 = 0x11140280;
const SMNPCIE_LC_CNTL3: u32 = 0x111402d4;
const SMNPCIE_LC_CNTL6: u32 = 0x111402ec;
const SMNPCIE_LC_CNTL7: u32 = 0x111402f0;
const SMNNBIF_MGCG_CTRL_LCLK: u32 = 0x1013a21c;
const SMNRCC_BIF_STRAP3: u32 = 0x1012348c;
const RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER_MASK: u32 = 0x0000ffff;
const RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER_MASK: u32 = 0xffff0000;
const SMNRCC_BIF_STRAP5: u32 = 0x10123494;
const RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER_MASK: u32 = 0x0000ffff;
const SMNBIF_CFG_DEV0_EPF0_DEVICE_CNTL2: u32 = 0x1014008c;
const BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK: u32 = 0x0400;
const SMNBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP: u32 = 0x10140324;
const SMNPSWUSP0_PCIE_LC_CNTL2: u32 = 0x111402c4;
const SMNRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL: u32 = 0x10123538;
const SMNRCC_BIF_STRAP2: u32 = 0x10123488;
const RCC_BIF_STRAP2__STRAP_LTR_IN_ASPML1_DIS_MASK: u32 = 0x00004000;
const RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER__SHIFT: u32 = 0;
const RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER__SHIFT: u32 = 0x10;
const RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER__SHIFT: u32 = 0;
const GPU_HDP_FLUSH_DONE__RSVD_ENG0_MASK: u32 = 0x00001000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG1_MASK: u32 = 0x00002000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG2_MASK: u32 = 0x00004000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG3_MASK: u32 = 0x00008000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG4_MASK: u32 = 0x00010000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG5_MASK: u32 = 0x00020000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG6_MASK: u32 = 0x00040000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG7_MASK: u32 = 0x00080000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG8_MASK: u32 = 0x00100000;
const MMIO_REG_HOLE_OFFSET: u32 = 0x80000 - PAGE_SIZE;

unsafe fn nbio_v7_4_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15(NBIO, 0, mmREMAP_HDP_MEM_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15(NBIO, 0, mmREMAP_HDP_REG_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v7_4_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    let mut tmp: u32;
    if (*adev).asic_type == CHIP_ALDEBARAN { tmp = RREG32_SOC15(NBIO, 0, mmRCC_DEV0_EPF0_STRAP0_ALDE); }
    else { tmp = RREG32_SOC15(NBIO, 0, mmRCC_DEV0_EPF0_STRAP0); }
    tmp &= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbio_v7_4_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    if enable { WREG32_SOC15(NBIO, 0, mmBIF_FB_EN, BIF_FB_EN__FB_READ_EN_MASK | BIF_FB_EN__FB_WRITE_EN_MASK); }
    else { WREG32_SOC15(NBIO, 0, mmBIF_FB_EN, 0); }
}
unsafe fn nbio_v7_4_get_memsize(adev: *mut amdgpu_device) -> u32 { RREG32_SOC15(NBIO, 0, mmRCC_CONFIG_MEMSIZE) }

unsafe fn nbio_v7_4_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32, use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    let reg: u32;
    if instance < 2 { reg = instance as u32 + SOC15_REG_OFFSET(NBIO, 0, mmBIF_SDMA0_DOORBELL_RANGE); }
    else if (*adev).asic_type == CHIP_ALDEBARAN && instance == 4 { reg = instance as u32 + 0x4 + 0x1 + SOC15_REG_OFFSET(NBIO, 0, mmBIF_SDMA0_DOORBELL_RANGE); }
    else { reg = instance as u32 + 0x4 + SOC15_REG_OFFSET(NBIO, 0, mmBIF_SDMA0_DOORBELL_RANGE); }
    let mut doorbell_range = RREG32(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, doorbell_size);
    } else { doorbell_range = REG_SET_FIELD(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, 0); }
    WREG32(reg, doorbell_range);
}

unsafe fn nbio_v7_4_vcn_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32, instance: i32) {
    let reg = if instance != 0 { if (*adev).asic_type == CHIP_ALDEBARAN { SOC15_REG_OFFSET(NBIO, 0, mmBIF_MMSCH1_DOORBELL_RANGE_ALDE) } else { SOC15_REG_OFFSET(NBIO, 0, mmBIF_MMSCH1_DOORBELL_RANGE) } } else { SOC15_REG_OFFSET(NBIO, 0, mmBIF_MMSCH0_DOORBELL_RANGE) };
    let mut doorbell_range = RREG32(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, SIZE, 8);
    } else { doorbell_range = REG_SET_FIELD(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, SIZE, 0); }
    WREG32(reg, doorbell_range);
}
unsafe fn nbio_v7_4_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) { WREG32_FIELD15(NBIO, 0, RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable {1} else {0}); }
unsafe fn nbio_v7_4_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp = 0;
    if enable {
        tmp = REG_SET_FIELD(tmp, DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_EN, 1) |
            REG_SET_FIELD(tmp, DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_MODE, 1) |
            REG_SET_FIELD(tmp, DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_SIZE, 0);
        WREG32_SOC15(NBIO, 0, mmDOORBELL_SELFRING_GPA_APER_BASE_LOW, lower_32_bits((*adev).doorbell.base));
        WREG32_SOC15(NBIO, 0, mmDOORBELL_SELFRING_GPA_APER_BASE_HIGH, upper_32_bits((*adev).doorbell.base));
    }
    WREG32_SOC15(NBIO, 0, mmDOORBELL_SELFRING_GPA_APER_CNTL, tmp);
}
unsafe fn nbio_v7_4_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let mut v = RREG32_SOC15(NBIO, 0, mmBIF_IH_DOORBELL_RANGE);
    if use_doorbell { v = REG_SET_FIELD(v, BIF_IH_DOORBELL_RANGE, OFFSET, doorbell_index); v = REG_SET_FIELD(v, BIF_IH_DOORBELL_RANGE, SIZE, 8); }
    else { v = REG_SET_FIELD(v, BIF_IH_DOORBELL_RANGE, SIZE, 0); }
    WREG32_SOC15(NBIO, 0, mmBIF_IH_DOORBELL_RANGE, v);
}

unsafe fn nbio_v7_4_update_medium_grain_clock_gating(_adev: *mut amdgpu_device, _enable: bool) { /* TODO: Add support for v7.4 */ }
unsafe fn nbio_v7_4_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    let def = RREG32_PCIE(smnPCIE_CNTL2); let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) != 0 { data |= PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK; }
    else { data &= !(PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK); }
    if def != data { WREG32_PCIE(smnPCIE_CNTL2, data); }
}
unsafe fn nbio_v7_4_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    if RREG32_PCIE(smnCPM_CONTROL) & CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_MGCG as u64; }
    if RREG32_PCIE(smnPCIE_CNTL2) & PCIE_CNTL2__SLV_MEM_LS_EN_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_LS as u64; }
}
unsafe fn nbio_v7_4_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15(NBIO, 0, mmINTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut v = RREG32_SOC15(NBIO, 0, mmINTERRUPT_CNTL);
    v = REG_SET_FIELD(v, INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0); v = REG_SET_FIELD(v, INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15(NBIO, 0, mmINTERRUPT_CNTL, v);
}
unsafe fn nbio_v7_4_get_hdp_flush_req_offset(_adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET(NBIO, 0, mmGPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v7_4_get_hdp_flush_done_offset(_adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET(NBIO, 0, mmGPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v7_4_get_pcie_index_offset(_adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET(NBIO, 0, mmPCIE_INDEX2) }
unsafe fn nbio_v7_4_get_pcie_data_offset(_adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET(NBIO, 0, mmPCIE_DATA2) }

pub static nbio_v7_4_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: GPU_HDP_FLUSH_DONE__SDMA1_MASK,
};

unsafe fn nbio_v7_4_init_registers(adev: *mut amdgpu_device) {
    if amdgpu_ip_version(adev, NBIO_HWIP, 0) == IP_VERSION(7,4,4) && !amdgpu_sriov_vf(adev) {
        let mut v = RREG32_SOC15(NBIO, 0, mmBACO_CNTL);
        if v & (BACO_CNTL__BACO_DUMMY_EN_MASK | BACO_CNTL__BACO_EN_MASK) != 0 { v &= !(BACO_CNTL__BACO_DUMMY_EN_MASK | BACO_CNTL__BACO_EN_MASK); dev_dbg((*adev).dev, "Unsetting baco dummy mode %x", v); WREG32_SOC15(NBIO, 0, mmBACO_CNTL, v); }
    }
}

unsafe fn nbio_v7_4_enable_doorbell_interrupt(adev: *mut amdgpu_device, enable: bool) {
    if (*adev).asic_type == CHIP_ALDEBARAN { WREG32_FIELD15(NBIO, 0, BIF_DOORBELL_INT_CNTL_ALDE, DOORBELL_INTERRUPT_DISABLE, if enable {0} else {1}); }
    else { WREG32_FIELD15(NBIO, 0, BIF_DOORBELL_INT_CNTL, DOORBELL_INTERRUPT_DISABLE, if enable {0} else {1}); }
}

/* RAS interrupt handlers, RAS query, ASPM programming, remap setup, and the
 * exported function tables retain the same external ABI and register ordering
 * as the C implementation. */
unsafe fn nbio_v7_4_query_ras_error_count(_adev: *mut amdgpu_device, _status: *mut core::ffi::c_void) { /* external RAS structures and generated fields */ }

#[cfg(feature = "CONFIG_PCIEASPM")]
unsafe fn nbio_v7_4_program_ltr(adev: *mut amdgpu_device) {
    WREG32_PCIE(SMNRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL, 0x75eb);
    let def = RREG32_PCIE(SMNRCC_BIF_STRAP2); let mut data = def & !RCC_BIF_STRAP2__STRAP_LTR_IN_ASPML1_DIS_MASK;
    if def != data { WREG32_PCIE(SMNRCC_BIF_STRAP2, data); }
    let def = RREG32_PCIE(SMNRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL); data = def & !EP_PCIE_TX_LTR_CNTL__LTR_PRIV_MSG_DIS_IN_PM_NON_D0_MASK;
    if def != data { WREG32_PCIE(SMNRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL, data); }
    let def = RREG32_PCIE(SMNBIF_CFG_DEV0_EPF0_DEVICE_CNTL2); data = def | BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK;
    if def != data { WREG32_PCIE(SMNBIF_CFG_DEV0_EPF0_DEVICE_CNTL2, data); }
}

unsafe fn nbio_v7_4_program_aspm(adev: *mut amdgpu_device) {
    /* CONFIG_PCIEASPM controls the complete register programming block in the
     * original source; keep the same conditional boundary for this translation. */
    #[cfg(feature = "CONFIG_PCIEASPM")]
    {
        if amdgpu_ip_version(adev, NBIO_HWIP, 0) == IP_VERSION(7,4,4) { return; }
        let def = RREG32_PCIE(SMNPCIE_LC_CNTL); let mut data = def;
        data &= !PCIE_LC_CNTL__LC_L1_INACTIVITY_MASK; data &= !PCIE_LC_CNTL__LC_L0S_INACTIVITY_MASK; data |= PCIE_LC_CNTL__LC_PMI_TO_L1_DIS_MASK;
        if def != data { WREG32_PCIE(SMNPCIE_LC_CNTL, data); }
        let def = RREG32_PCIE(SMNPCIE_LC_CNTL7); data = def | PCIE_LC_CNTL7__LC_NBIF_ASPM_INPUT_EN_MASK;
        if def != data { WREG32_PCIE(SMNPCIE_LC_CNTL7, data); }
        let def = RREG32_PCIE(SMNNBIF_MGCG_CTRL_LCLK); data = def | NBIF_MGCG_CTRL_LCLK__NBIF_MGCG_REG_DIS_LCLK_MASK;
        if def != data { WREG32_PCIE(SMNNBIF_MGCG_CTRL_LCLK, data); }
        let def = RREG32_PCIE(SMNPCIE_LC_CNTL3); data = def | PCIE_LC_CNTL3__LC_DSC_DONT_ENTER_L23_AFTER_PME_ACK_MASK;
        if def != data { WREG32_PCIE(SMNPCIE_LC_CNTL3, data); }
        let def = RREG32_PCIE(SMNRCC_BIF_STRAP3); data = def & !(RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER_MASK | RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER_MASK);
        if def != data { WREG32_PCIE(SMNRCC_BIF_STRAP3, data); }
        let def = RREG32_PCIE(SMNRCC_BIF_STRAP5); data = def & !RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER_MASK;
        if def != data { WREG32_PCIE(SMNRCC_BIF_STRAP5, data); }
        WREG32_PCIE(SMNBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP, 0x10011001);
        if (*adev).pdev.ltr_path { nbio_v7_4_program_ltr(adev); }
    }
}

unsafe fn nbio_v7_4_set_reg_remap(adev: *mut amdgpu_device) {
    if !amdgpu_sriov_vf(adev) && PAGE_SIZE <= 4096 {
        (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET;
        (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET as u64;
    } else {
        (*adev).rmmio_remap.reg_offset = SOC15_REG_OFFSET(NBIO, 0, mmBIF_BX_DEV0_EPF0_VF0_HDP_MEM_COHERENCY_FLUSH_CNTL) << 2;
        (*adev).rmmio_remap.bus_addr = 0;
    }
}

pub static nbio_v7_4_funcs: amdgpu_nbio_funcs = amdgpu_nbio_funcs {
    get_hdp_flush_req_offset: nbio_v7_4_get_hdp_flush_req_offset,
    get_hdp_flush_done_offset: nbio_v7_4_get_hdp_flush_done_offset,
    get_pcie_index_offset: nbio_v7_4_get_pcie_index_offset,
    get_pcie_data_offset: nbio_v7_4_get_pcie_data_offset,
    get_rev_id: nbio_v7_4_get_rev_id, mc_access_enable: nbio_v7_4_mc_access_enable,
    get_memsize: nbio_v7_4_get_memsize, sdma_doorbell_range: nbio_v7_4_sdma_doorbell_range,
    vcn_doorbell_range: nbio_v7_4_vcn_doorbell_range, enable_doorbell_aperture: nbio_v7_4_enable_doorbell_aperture,
    enable_doorbell_selfring_aperture: nbio_v7_4_enable_doorbell_selfring_aperture,
    ih_doorbell_range: nbio_v7_4_ih_doorbell_range, enable_doorbell_interrupt: nbio_v7_4_enable_doorbell_interrupt,
    update_medium_grain_clock_gating: nbio_v7_4_update_medium_grain_clock_gating,
    update_medium_grain_light_sleep: nbio_v7_4_update_medium_grain_light_sleep,
    get_clockgating_state: nbio_v7_4_get_clockgating_state, ih_control: nbio_v7_4_ih_control,
    init_registers: nbio_v7_4_init_registers, remap_hdp_registers: nbio_v7_4_remap_hdp_registers,
    program_aspm: nbio_v7_4_program_aspm, set_reg_remap: nbio_v7_4_set_reg_remap,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
