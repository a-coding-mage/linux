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

// External kernel/amdgpu declarations and register definitions are supplied by
// the surrounding translation unit.

const smnPCIE_CONFIG_CNTL: u32 = 0x11180044;
const smnCPM_CONTROL: u32 = 0x11180460;
const smnPCIE_CNTL2: u32 = 0x11180070;
const smnPCIE_LC_CNTL: u32 = 0x11140280;
const smnPCIE_LC_CNTL3: u32 = 0x111402d4;
const smnPCIE_LC_CNTL6: u32 = 0x111402ec;
const smnPCIE_LC_CNTL7: u32 = 0x111402f0;
const smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2: u32 = 0x1014008c;
const smnRCC_EP_DEV0_0_EP_PCIE_TX_LTR_CNTL: u32 = 0x10123538;
const smnBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP: u32 = 0x10140324;
const smnPSWUSP0_PCIE_LC_CNTL2: u32 = 0x111402c4;
const smnNBIF_MGCG_CTRL_LCLK: u32 = 0x1013a21c;
const mmBIF_SDMA2_DOORBELL_RANGE: u32 = 0x01d6;
const mmBIF_SDMA2_DOORBELL_RANGE_BASE_IDX: u32 = 2;
const mmBIF_SDMA3_DOORBELL_RANGE: u32 = 0x01d7;
const mmBIF_SDMA3_DOORBELL_RANGE_BASE_IDX: u32 = 2;
const mmBIF_MMSCH1_DOORBELL_RANGE: u32 = 0x01d8;
const mmBIF_MMSCH1_DOORBELL_RANGE_BASE_IDX: u32 = 2;
const smnPCIE_LC_LINK_WIDTH_CNTL: u32 = 0x11140288;
const GPU_HDP_FLUSH_DONE__RSVD_ENG0_MASK: u32 = 0x00001000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG1_MASK: u32 = 0x00002000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG2_MASK: u32 = 0x00004000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG3_MASK: u32 = 0x00008000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG4_MASK: u32 = 0x00010000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG5_MASK: u32 = 0x00020000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG6_MASK: u32 = 0x00040000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG7_MASK: u32 = 0x00080000;
const GPU_HDP_FLUSH_DONE__RSVD_ENG8_MASK: u32 = 0x00100000;

unsafe fn nbio_v2_3_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, mmREMAP_HDP_MEM_FLUSH_CNTL, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15!(NBIO, 0, mmREMAP_HDP_REG_FLUSH_CNTL, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v2_3_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    // Guest VMs read 0xffffffff from the strap register, so force the default revision.
    if amdgpu_sriov_vf(adev) { return 0; }
    let mut tmp = RREG32_SOC15!(NBIO, 0, mmRCC_DEV0_EPF0_STRAP0);
    tmp &= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbio_v2_3_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    if enable { WREG32_SOC15!(NBIO, 0, mmBIF_FB_EN, BIF_FB_EN__FB_READ_EN_MASK | BIF_FB_EN__FB_WRITE_EN_MASK); }
    else { WREG32_SOC15!(NBIO, 0, mmBIF_FB_EN, 0); }
}
unsafe fn nbio_v2_3_get_memsize(adev: *mut amdgpu_device) -> u32 { RREG32_SOC15!(NBIO, 0, mmRCC_DEV0_EPF0_RCC_CONFIG_MEMSIZE) }

unsafe fn nbio_v2_3_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32, use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    let reg = match instance { 0 => SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA0_DOORBELL_RANGE), 1 => SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA1_DOORBELL_RANGE), 2 => SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA2_DOORBELL_RANGE), _ => SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA3_DOORBELL_RANGE) };
    let mut doorbell_range = RREG32!(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, doorbell_size);
    } else { doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, 0); }
    WREG32!(reg, doorbell_range);
}

unsafe fn nbio_v2_3_vcn_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32, instance: i32) {
    let reg = if instance != 0 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_MMSCH1_DOORBELL_RANGE) } else { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_MMSCH0_DOORBELL_RANGE) };
    let mut doorbell_range = RREG32!(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, SIZE, 8);
    } else { doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, SIZE, 0); }
    WREG32!(reg, doorbell_range);
}

unsafe fn nbio_v2_3_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) { WREG32_FIELD15!(NBIO, 0, RCC_DEV0_EPF0_RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable { 1 } else { 0 }); }

unsafe fn nbio_v2_3_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp = 0u32;
    if enable {
        tmp = REG_SET_FIELD!(tmp, BIF_BX_PF_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_EN, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_MODE, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_SIZE, 0);
        WREG32_SOC15!(NBIO, 0, mmBIF_BX_PF_DOORBELL_SELFRING_GPA_APER_BASE_LOW, lower_32_bits((*adev).doorbell.base));
        WREG32_SOC15!(NBIO, 0, mmBIF_BX_PF_DOORBELL_SELFRING_GPA_APER_BASE_HIGH, upper_32_bits((*adev).doorbell.base));
    }
    WREG32_SOC15!(NBIO, 0, mmBIF_BX_PF_DOORBELL_SELFRING_GPA_APER_CNTL, tmp);
}

unsafe fn nbio_v2_3_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let mut v = RREG32_SOC15!(NBIO, 0, mmBIF_IH_DOORBELL_RANGE);
    if use_doorbell { v = REG_SET_FIELD!(v, BIF_IH_DOORBELL_RANGE, OFFSET, doorbell_index); v = REG_SET_FIELD!(v, BIF_IH_DOORBELL_RANGE, SIZE, 2); }
    else { v = REG_SET_FIELD!(v, BIF_IH_DOORBELL_RANGE, SIZE, 0); }
    WREG32_SOC15!(NBIO, 0, mmBIF_IH_DOORBELL_RANGE, v);
}

unsafe fn nbio_v2_3_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut v = RREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL);
    v = REG_SET_FIELD!(v, INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    v = REG_SET_FIELD!(v, INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL, v);
}

unsafe fn nbio_v2_3_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    if (*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG == 0 { return; }
    let def = RREG32_PCIE!(smnCPM_CONTROL); let mut data = def;
    let mask = CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_LCNT_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_REGS_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PRBS_GATE_ENABLE_MASK | CPM_CONTROL__REFCLK_REGS_GATE_ENABLE_MASK;
    if enable { data |= mask; } else { data &= !mask; } if def != data { WREG32_PCIE!(smnCPM_CONTROL, data); }
}

unsafe fn nbio_v2_3_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    if (*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS == 0 { return; }
    let def = RREG32_PCIE!(smnPCIE_CNTL2); let mut data = def;
    let mask = PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK;
    if enable { data |= mask; } else { data &= !mask; } if def != data { WREG32_PCIE!(smnPCIE_CNTL2, data); }
}

unsafe fn nbio_v2_3_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    if RREG32_PCIE!(smnCPM_CONTROL) & CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_MGCG as u64; }
    if RREG32_PCIE!(smnPCIE_CNTL2) & PCIE_CNTL2__SLV_MEM_LS_EN_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_LS as u64; }
}

unsafe fn nbio_v2_3_get_hdp_flush_req_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_PF_GPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v2_3_get_hdp_flush_done_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_PF_GPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v2_3_get_pcie_index_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmPCIE_INDEX2) }
unsafe fn nbio_v2_3_get_pcie_data_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmPCIE_DATA2) }

pub static nbio_v2_3_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: BIF_BX_PF_GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: BIF_BX_PF_GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: BIF_BX_PF_GPU_HDP_FLUSH_DONE__SDMA1_MASK,
};

const NAVI10_PCIE__LC_L0S_INACTIVITY_DEFAULT: u32 = 0x00000000;
const NAVI10_PCIE__LC_L1_INACTIVITY_DEFAULT: u32 = 0x0000000A;
const NAVI10_PCIE__LC_L1_INACTIVITY_TBT_DEFAULT: u32 = 0x0000000E;

unsafe fn nbio_v2_3_init_registers(adev: *mut amdgpu_device) {
    let def = RREG32_PCIE!(smnPCIE_CONFIG_CNTL); let mut data = def;
    data = REG_SET_FIELD!(data, PCIE_CONFIG_CNTL, CI_SWUS_MAX_READ_REQUEST_SIZE_MODE, 1);
    data = REG_SET_FIELD!(data, PCIE_CONFIG_CNTL, CI_SWUS_MAX_READ_REQUEST_SIZE_PRIV, 1);
    if def != data { WREG32_PCIE!(smnPCIE_CONFIG_CNTL, data); }
}

unsafe fn nbio_v2_3_enable_aspm(adev: *mut amdgpu_device, enable: bool) {
    let def = RREG32_PCIE!(smnPCIE_LC_CNTL); let mut data = def;
    if enable {
        data &= !(PCIE_LC_CNTL__LC_L0S_INACTIVITY_MASK | PCIE_LC_CNTL__LC_L1_INACTIVITY_MASK);
        data |= NAVI10_PCIE__LC_L0S_INACTIVITY_DEFAULT << PCIE_LC_CNTL__LC_L0S_INACTIVITY__SHIFT;
        data |= (if dev_is_removable!(&(*adev).pdev.dev) { NAVI10_PCIE__LC_L1_INACTIVITY_TBT_DEFAULT } else { NAVI10_PCIE__LC_L1_INACTIVITY_DEFAULT }) << PCIE_LC_CNTL__LC_L1_INACTIVITY__SHIFT;
        data &= !PCIE_LC_CNTL__LC_PMI_TO_L1_DIS_MASK;
    } else { data &= !(PCIE_LC_CNTL__LC_L1_INACTIVITY_MASK | PCIE_LC_CNTL__LC_L0S_INACTIVITY_MASK); data |= PCIE_LC_CNTL__LC_PMI_TO_L1_DIS_MASK; }
    if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL, data); }
}

unsafe fn nbio_v2_3_program_aspm(adev: *mut amdgpu_device) {
    // CONFIG_PCIEASPM-gated implementation; register programming is preserved below.
    let def = RREG32_PCIE!(smnPCIE_LC_CNTL); let mut data = def;
    data &= !(PCIE_LC_CNTL__LC_L1_INACTIVITY_MASK | PCIE_LC_CNTL__LC_L0S_INACTIVITY_MASK); data |= PCIE_LC_CNTL__LC_PMI_TO_L1_DIS_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL, data); }
    let def = RREG32_PCIE!(smnPCIE_LC_CNTL7); let mut data = def | PCIE_LC_CNTL7__LC_NBIF_ASPM_INPUT_EN_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL7, data); }
    let def = RREG32_PCIE!(smnNBIF_MGCG_CTRL_LCLK); let mut data = def | NBIF_MGCG_CTRL_LCLK__NBIF_MGCG_REG_DIS_LCLK_MASK; if def != data { WREG32_PCIE!(smnNBIF_MGCG_CTRL_LCLK, data); }
    let def = RREG32_PCIE!(smnPCIE_LC_CNTL3); let mut data = def | PCIE_LC_CNTL3__LC_DSC_DONT_ENTER_L23_AFTER_PME_ACK_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL3, data); }
    let def = RREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2); let mut data = def & !BIF_CFG_DEV0_EPF0_DEVICE_CNTL2__LTR_EN_MASK; if def != data { WREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_DEVICE_CNTL2, data); }
    WREG32_PCIE!(smnBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP, 0x10011001);
    let def = RREG32_PCIE!(smnPSWUSP0_PCIE_LC_CNTL2); let mut data = def | PSWUSP0_PCIE_LC_CNTL2__LC_ALLOW_PDWN_IN_L1_MASK | PSWUSP0_PCIE_LC_CNTL2__LC_ALLOW_PDWN_IN_L23_MASK; data &= !PSWUSP0_PCIE_LC_CNTL2__LC_RCV_L0_TO_RCV_L0S_DIS_MASK; if def != data { WREG32_PCIE!(smnPSWUSP0_PCIE_LC_CNTL2, data); }
    let def = RREG32_PCIE!(smnPCIE_LC_CNTL6); let mut data = def | PCIE_LC_CNTL6__LC_L1_POWERDOWN_MASK | PCIE_LC_CNTL6__LC_RX_L0S_STANDBY_EN_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL6, data); }
    let def = RREG32_PCIE!(smnPCIE_LC_CNTL); let mut data = def | NAVI10_PCIE__LC_L0S_INACTIVITY_DEFAULT << PCIE_LC_CNTL__LC_L0S_INACTIVITY__SHIFT | (if dev_is_removable!(&(*adev).pdev.dev) { NAVI10_PCIE__LC_L1_INACTIVITY_TBT_DEFAULT } else { NAVI10_PCIE__LC_L1_INACTIVITY_DEFAULT }) << PCIE_LC_CNTL__LC_L1_INACTIVITY__SHIFT; data &= !PCIE_LC_CNTL__LC_PMI_TO_L1_DIS_MASK; if def != data { WREG32_PCIE!(smnPCIE_LC_CNTL, data); }
}

unsafe fn nbio_v2_3_apply_lc_spc_mode_wa(adev: *mut amdgpu_device) {
    if (*adev).asic_type < CHIP_NAVI10 || (*adev).asic_type > CHIP_NAVI12 { return; }
    let mut d = RREG32_PCIE!(smnPCIE_LC_LINK_WIDTH_CNTL); let width = (d & PCIE_LC_LINK_WIDTH_CNTL__LC_LINK_WIDTH_RD_MASK) >> PCIE_LC_LINK_WIDTH_CNTL__LC_LINK_WIDTH_RD__SHIFT;
    if width == 0x3 { d = RREG32_PCIE!(smnPCIE_LC_CNTL6); d = (d & !PCIE_LC_CNTL6__LC_SPC_MODE_8GT_MASK) | (0x2 << PCIE_LC_CNTL6__LC_SPC_MODE_8GT__SHIFT); WREG32_PCIE!(smnPCIE_LC_CNTL6, d); }
}
unsafe fn nbio_v2_3_apply_l1_link_width_reconfig_wa(adev: *mut amdgpu_device) { if (*adev).asic_type == CHIP_NAVI10 { let d = RREG32_PCIE!(smnPCIE_LC_LINK_WIDTH_CNTL) | PCIE_LC_LINK_WIDTH_CNTL__LC_L1_RECONFIG_EN_MASK; WREG32_PCIE!(smnPCIE_LC_LINK_WIDTH_CNTL, d); } }
unsafe fn nbio_v2_3_clear_doorbell_interrupt(adev: *mut amdgpu_device) { if amdgpu_ip_version(adev, NBIO_HWIP, 0) != IP_VERSION!(3, 3, 0) { return; } if RREG32_SOC15!(NBIO, 0, mmBIF_RB_CNTL) & BIF_RB_CNTL__RB_ENABLE_MASK == 0 { let r = RREG32_SOC15!(NBIO, 0, mmBIF_DOORBELL_INT_CNTL); if r & BIF_DOORBELL_INT_CNTL__DOORBELL_INTERRUPT_STATUS_MASK != 0 { WREG32_SOC15!(NBIO, 0, mmBIF_DOORBELL_INT_CNTL, 1 << BIF_DOORBELL_INT_CNTL__DOORBELL_INTERRUPT_CLEAR__SHIFT); } } }
const MMIO_REG_HOLE_OFFSET: u64 = 0x80000 - PAGE_SIZE as u64;
unsafe fn nbio_v2_3_set_reg_remap(adev: *mut amdgpu_device) { if !amdgpu_sriov_vf(adev) && PAGE_SIZE <= 4096 { (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET; (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET; } else { (*adev).rmmio_remap.reg_offset = SOC15_REG_OFFSET!(NBIO, 0, mmBIF_BX_DEV0_EPF0_VF0_HDP_MEM_COHERENCY_FLUSH_CNTL) << 2; (*adev).rmmio_remap.bus_addr = 0; } }

pub static nbio_v2_3_funcs: amdgpu_nbio_funcs = amdgpu_nbio_funcs {
    get_hdp_flush_req_offset: Some(nbio_v2_3_get_hdp_flush_req_offset), get_hdp_flush_done_offset: Some(nbio_v2_3_get_hdp_flush_done_offset), get_pcie_index_offset: Some(nbio_v2_3_get_pcie_index_offset), get_pcie_data_offset: Some(nbio_v2_3_get_pcie_data_offset), get_rev_id: Some(nbio_v2_3_get_rev_id), mc_access_enable: Some(nbio_v2_3_mc_access_enable), get_memsize: Some(nbio_v2_3_get_memsize), sdma_doorbell_range: Some(nbio_v2_3_sdma_doorbell_range), vcn_doorbell_range: Some(nbio_v2_3_vcn_doorbell_range), enable_doorbell_aperture: Some(nbio_v2_3_enable_doorbell_aperture), enable_doorbell_selfring_aperture: Some(nbio_v2_3_enable_doorbell_selfring_aperture), ih_doorbell_range: Some(nbio_v2_3_ih_doorbell_range), update_medium_grain_clock_gating: Some(nbio_v2_3_update_medium_grain_clock_gating), update_medium_grain_light_sleep: Some(nbio_v2_3_update_medium_grain_light_sleep), get_clockgating_state: Some(nbio_v2_3_get_clockgating_state), ih_control: Some(nbio_v2_3_ih_control), init_registers: Some(nbio_v2_3_init_registers), remap_hdp_registers: Some(nbio_v2_3_remap_hdp_registers), enable_aspm: Some(nbio_v2_3_enable_aspm), program_aspm: Some(nbio_v2_3_program_aspm), apply_lc_spc_mode_wa: Some(nbio_v2_3_apply_lc_spc_mode_wa), apply_l1_link_width_reconfig_wa: Some(nbio_v2_3_apply_l1_link_width_reconfig_wa), clear_doorbell_interrupt: Some(nbio_v2_3_clear_doorbell_interrupt), set_reg_remap: Some(nbio_v2_3_set_reg_remap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
