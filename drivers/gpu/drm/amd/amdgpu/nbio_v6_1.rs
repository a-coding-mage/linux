/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding kernel translation.

const smnPCIE_LC_CNTL: u32 = 0x11140280;
const smnPCIE_LC_CNTL3: u32 = 0x111402d4;
const smnPCIE_LC_CNTL6: u32 = 0x111402ec;
const smnPCIE_LC_CNTL7: u32 = 0x111402f0;
const smnNBIF_MGCG_CTRL_LCLK: u32 = 0x1013a05c;
const NBIF_MGCG_CTRL_LCLK__NBIF_MGCG_REG_DIS_LCLK_MASK: u32 = 0x00001000;
const RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER_MASK: u32 = 0x0000FFFF;
const RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER_MASK: u32 = 0xFFFF0000;
const smnRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL: u32 = 0x10123530;
const smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2: u32 = 0x1014008c;
const smnBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP: u32 = 0x10140324;
const smnPSWUSP0_PCIE_LC_CNTL2: u32 = 0x111402c4;
const smnRCC_BIF_STRAP2: u32 = 0x10123488;
const smnRCC_BIF_STRAP3: u32 = 0x1012348c;
const smnRCC_BIF_STRAP5: u32 = 0x10123494;
const BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK: u32 = 0x0400;
const RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER_MASK: u32 = 0x0000FFFF;
const RCC_BIF_STRAP2__STRAP_LTR_IN_ASPML1_DIS_MASK: u32 = 0x00004000;
const RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER__SHIFT: u32 = 0x0;
const RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER__SHIFT: u32 = 0x10;
const RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER__SHIFT: u32 = 0x0;

unsafe fn nbio_v6_1_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, mmREMAP_HDP_MEM_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15!(NBIO, 0, mmREMAP_HDP_REG_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v6_1_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    let mut tmp = RREG32_SOC15!(NBIO, 0, mmRCC_DEV0_EPF0_STRAP0);
    tmp &= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbio_v6_1_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    if enable { WREG32_SOC15!(NBIO, 0, mmBIF_FB_EN, BIF_FB_EN__FB_READ_EN_MASK | BIF_FB_EN__FB_WRITE_EN_MASK); }
    else { WREG32_SOC15!(NBIO, 0, mmBIF_FB_EN, 0); }
}

unsafe fn nbio_v6_1_get_memsize(adev: *mut amdgpu_device) -> u32 { RREG32_SOC15!(NBIO, 0, mmRCC_PF_0_0_RCC_CONFIG_MEMSIZE) }

unsafe fn nbio_v6_1_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32, use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    let reg = if instance == 0 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA0_DOORBELL_RANGE) } else { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA1_DOORBELL_RANGE) };
    let mut doorbell_range = RREG32!(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, doorbell_size);
    } else { doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, 0); }
    WREG32!(reg, doorbell_range);
}

unsafe fn nbio_v6_1_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) {
    WREG32_FIELD15!(NBIO, 0, RCC_PF_0_0_RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable { 1 } else { 0 });
}

unsafe fn nbio_v6_1_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp = 0;
    if enable {
        tmp = REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_EN, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_MODE, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_SIZE, 0);
        WREG32_SOC15!(NBIO, 0, mmBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_LOW, lower_32_bits((*adev).doorbell.base));
        WREG32_SOC15!(NBIO, 0, mmBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_HIGH, upper_32_bits((*adev).doorbell.base));
    }
    WREG32_SOC15!(NBIO, 0, mmBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, tmp);
}

unsafe fn nbio_v6_1_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let mut r = RREG32_SOC15!(NBIO, 0, mmBIF_IH_DOORBELL_RANGE);
    if use_doorbell {
        r = REG_SET_FIELD!(r, BIF_IH_DOORBELL_RANGE, OFFSET, doorbell_index);
        r = REG_SET_FIELD!(r, BIF_IH_DOORBELL_RANGE, SIZE, 6);
    } else { r = REG_SET_FIELD!(r, BIF_IH_DOORBELL_RANGE, SIZE, 0); }
    WREG32_SOC15!(NBIO, 0, mmBIF_IH_DOORBELL_RANGE, r);
}

unsafe fn nbio_v6_1_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut interrupt_cntl = RREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL);
    interrupt_cntl = REG_SET_FIELD!(interrupt_cntl, INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    interrupt_cntl = REG_SET_FIELD!(interrupt_cntl, INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL, interrupt_cntl);
}

unsafe fn nbio_v6_1_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let def = RREG32_PCIE!(smnCPM_CONTROL); let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG) != 0 { data |= CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PERM_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_LCNT_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_REGS_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PRBS_GATE_ENABLE_MASK | CPM_CONTROL__REFCLK_REGS_GATE_ENABLE_MASK; }
    else { data &= !(CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PERM_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_LCNT_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_REGS_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PRBS_GATE_ENABLE_MASK | CPM_CONTROL__REFCLK_REGS_GATE_ENABLE_MASK); }
    if def != data { WREG32_PCIE!(smnCPM_CONTROL, data); }
}

unsafe fn nbio_v6_1_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    let def = RREG32_PCIE!(smnPCIE_CNTL2); let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) != 0 { data |= PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK; }
    else { data &= !(PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK); }
    if def != data { WREG32_PCIE!(smnPCIE_CNTL2, data); }
}

unsafe fn nbio_v6_1_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let data = RREG32_PCIE!(smnCPM_CONTROL); if data & CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_MGCG as u64; }
    let data = RREG32_PCIE!(smnPCIE_CNTL2); if data & PCIE_CNTL2__SLV_MEM_LS_EN_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_LS as u64; }
}

unsafe fn nbio_v6_1_get_hdp_flush_req_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_PF0_GPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v6_1_get_hdp_flush_done_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_PF0_GPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v6_1_get_pcie_index_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmPCIE_INDEX2) }
unsafe fn nbio_v6_1_get_pcie_data_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmPCIE_DATA2) }

const MMIO_REG_HOLE_OFFSET: u64 = 0x80000 - PAGE_SIZE as u64;

unsafe fn nbio_v6_1_init_registers(adev: *mut amdgpu_device) {
    let def = RREG32_PCIE!(smnPCIE_CONFIG_CNTL); let mut data = def;
    data = REG_SET_FIELD!(data, PCIE_CONFIG_CNTL, CI_SWUS_MAX_READ_REQUEST_SIZE_MODE, 1);
    data = REG_SET_FIELD!(data, PCIE_CONFIG_CNTL, CI_SWUS_MAX_READ_REQUEST_SIZE_PRIV, 1);
    if def != data { WREG32_PCIE!(smnPCIE_CONFIG_CNTL, data); }
    let def = RREG32_PCIE!(smnPCIE_CI_CNTL); let mut data = REG_SET_FIELD!(def, PCIE_CI_CNTL, CI_SLV_ORDERING_DIS, 1);
    if def != data { WREG32_PCIE!(smnPCIE_CI_CNTL, data); }
}

#[cfg(CONFIG_PCIEASPM)]
unsafe fn nbio_v6_1_program_ltr(adev: *mut amdgpu_device) {
    WREG32_PCIE!(smnRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL, 0x75EB);
    let def = RREG32_PCIE!(smnRCC_BIF_STRAP2); let data = def & !RCC_BIF_STRAP2__STRAP_LTR_IN_ASPML1_DIS_MASK; if def != data { WREG32_PCIE!(smnRCC_BIF_STRAP2, data); }
    let def = RREG32_PCIE!(smnRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL); let data = def & !EP_PCIE_TX_LTR_CNTL__LTR_PRIV_MSG_DIS_IN_PM_NON_D0_MASK; if def != data { WREG32_PCIE!(smnRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL, data); }
    let def = RREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2); let data = def | BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK; if def != data { WREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2, data); }
}

unsafe fn nbio_v6_1_program_aspm(adev: *mut amdgpu_device) {
    #[cfg(CONFIG_PCIEASPM)] {
        let def = RREG32_PCIE!(smnPCIE_LC_CNTL); let mut data = def & !PCIE_LC_CNTL__LC_L1_INACTIVITY_MASK & !PCIE_LC_CNTL__LC_L0S_INACTIVITY_MASK | PCIE_LC_CNTL__LC_PMI_TO_L1_DIS_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL, data); }
        let def = RREG32_PCIE!(smnPCIE_LC_CNTL7); let data = def | PCIE_LC_CNTL7__LC_NBIF_ASPM_INPUT_EN_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL7, data); }
        let def = RREG32_PCIE!(smnNBIF_MGCG_CTRL_LCLK); let data = def | NBIF_MGCG_CTRL_LCLK__NBIF_MGCG_REG_DIS_LCLK_MASK; if def != data { WREG32_PCIE!(smnNBIF_MGCG_CTRL_LCLK, data); }
        let def = RREG32_PCIE!(smnPCIE_LC_CNTL3); let data = def | PCIE_LC_CNTL3__LC_DSC_DONT_ENTER_L23_AFTER_PME_ACK_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL3, data); }
        let def = RREG32_PCIE!(smnRCC_BIF_STRAP3); let data = def & !RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER_MASK & !RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER_MASK; if def != data { WREG32_PCIE!(smnRCC_BIF_STRAP3, data); }
        let def = RREG32_PCIE!(smnRCC_BIF_STRAP5); let data = def & !RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER_MASK; if def != data { WREG32_PCIE!(smnRCC_BIF_STRAP5, data); }
        let def = RREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2); let data = def & !BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK; if def != data { WREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2, data); }
        WREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP, 0x10011001);
        let def = RREG32_PCIE!(smnPSWUSP0_PCIE_LC_CNTL2); let data = (def | PSWUSP0_PCIE_LC_CNTL2__LC_ALLOW_PDWN_IN_L1_MASK | PSWUSP0_PCIE_LC_CNTL2__LC_ALLOW_PDWN_IN_L23_MASK) & !PSWUSP0_PCIE_LC_CNTL2__LC_RCV_L0_TO_RCV_L0S_DIS_MASK; if def != data { WREG32_PCIE!(smnPSWUSP0_PCIE_LC_CNTL2, data); }
        let def = RREG32_PCIE!(smnPCIE_LC_CNTL6); let data = def | PCIE_LC_CNTL6__LC_L1_POWERDOWN_MASK | PCIE_LC_CNTL6__LC_RX_L0S_STANDBY_EN_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL6, data); }
        if (*adev).pdev.ltr_path { nbio_v6_1_program_ltr(adev); }
        let def = RREG32_PCIE!(smnRCC_BIF_STRAP3); let data = def | (0x5DE0 << RCC_BIF_STRAP3__STRAP_VLINK_ASPM_IDLE_TIMER__SHIFT) | (0x0010 << RCC_BIF_STRAP3__STRAP_VLINK_PM_L1_ENTRY_TIMER__SHIFT); if def != data { WREG32_PCIE!(smnRCC_BIF_STRAP3, data); }
        let def = RREG32_PCIE!(smnRCC_BIF_STRAP5); let data = def | (0x0010 << RCC_BIF_STRAP5__STRAP_VLINK_LDN_ENTRY_TIMER__SHIFT); if def != data { WREG32_PCIE!(smnRCC_BIF_STRAP5, data); }
        let def = RREG32_PCIE!(smnPCIE_LC_CNTL); let data = (def & !PCIE_LC_CNTL__LC_L0S_INACTIVITY_MASK) | (0x9 << PCIE_LC_CNTL__LC_L1_INACTIVITY__SHIFT) | (0x1 << PCIE_LC_CNTL__LC_PMI_TO_L1_DIS__SHIFT); if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL, data); }
        let def = RREG32_PCIE!(smnPCIE_LC_CNTL3); let data = def & !PCIE_LC_CNTL3__LC_DSC_DONT_ENTER_L23_AFTER_PME_ACK_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL3, data); }
    }
}

unsafe fn nbio_v6_1_set_reg_remap(adev: *mut amdgpu_device) {
    if !amdgpu_sriov_vf(adev) && PAGE_SIZE <= 4096 { (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET; (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET; }
    else { (*adev).rmmio_remap.reg_offset = SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_DEV0_EPF0_VF0_HDP_MEM_COHERENCY_FLUSH_CNTL) << 2; (*adev).rmmio_remap.bus_addr = 0; }
}

pub static nbio_v6_1_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA1_MASK,
};

pub static nbio_v6_1_funcs: amdgpu_nbio_funcs = amdgpu_nbio_funcs {
    get_hdp_flush_req_offset: Some(nbio_v6_1_get_hdp_flush_req_offset),
    get_hdp_flush_done_offset: Some(nbio_v6_1_get_hdp_flush_done_offset),
    get_pcie_index_offset: Some(nbio_v6_1_get_pcie_index_offset),
    get_pcie_data_offset: Some(nbio_v6_1_get_pcie_data_offset),
    get_rev_id: Some(nbio_v6_1_get_rev_id),
    mc_access_enable: Some(nbio_v6_1_mc_access_enable),
    get_memsize: Some(nbio_v6_1_get_memsize),
    sdma_doorbell_range: Some(nbio_v6_1_sdma_doorbell_range),
    enable_doorbell_aperture: Some(nbio_v6_1_enable_doorbell_aperture),
    enable_doorbell_selfring_aperture: Some(nbio_v6_1_enable_doorbell_selfring_aperture),
    ih_doorbell_range: Some(nbio_v6_1_ih_doorbell_range),
    update_medium_grain_clock_gating: Some(nbio_v6_1_update_medium_grain_clock_gating),
    update_medium_grain_light_sleep: Some(nbio_v6_1_update_medium_grain_light_sleep),
    get_clockgating_state: Some(nbio_v6_1_get_clockgating_state),
    ih_control: Some(nbio_v6_1_ih_control),
    init_registers: Some(nbio_v6_1_init_registers),
    remap_hdp_registers: Some(nbio_v6_1_remap_hdp_registers),
    program_aspm: Some(nbio_v6_1_program_aspm),
    set_reg_remap: Some(nbio_v6_1_set_reg_remap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
