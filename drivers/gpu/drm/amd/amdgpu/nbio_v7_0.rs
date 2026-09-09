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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const SMN_NBIF_MGCG_CTRL_LCLK: u32 = 0x1013a05c;

unsafe fn nbio_v7_0_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, mmREMAP_HDP_MEM_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15!(NBIO, 0, mmREMAP_HDP_REG_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v7_0_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    let mut tmp = RREG32_SOC15!(NBIO, 0, mmRCC_DEV0_EPF0_STRAP0);
    tmp &= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbio_v7_0_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    if enable {
        WREG32_SOC15!(NBIO, 0, mmBIF_FB_EN,
            BIF_FB_EN__FB_READ_EN_MASK | BIF_FB_EN__FB_WRITE_EN_MASK);
    } else { WREG32_SOC15!(NBIO, 0, mmBIF_FB_EN, 0); }
}

unsafe fn nbio_v7_0_get_memsize(adev: *mut amdgpu_device) -> u32 {
    RREG32_SOC15!(NBIO, 0, mmRCC_CONFIG_MEMSIZE)
}

unsafe fn nbio_v7_0_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32,
    use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    let reg = if instance == 0 { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA0_DOORBELL_RANGE) }
        else { SOC15_REG_OFFSET!(NBIO, 0, mmBIF_SDMA1_DOORBELL_RANGE) };
    let mut doorbell_range = RREG32!(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, doorbell_size);
    } else { doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_SDMA0_DOORBELL_RANGE, SIZE, 0); }
    WREG32!(reg, doorbell_range);
}

unsafe fn nbio_v7_0_vcn_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool,
    doorbell_index: i32, instance: i32) {
    let reg = SOC15_REG_OFFSET!(NBIO, 0, mmBIF_MMSCH0_DOORBELL_RANGE);
    let mut doorbell_range = RREG32!(reg);
    if use_doorbell {
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, OFFSET, doorbell_index);
        doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, SIZE, 8);
    } else { doorbell_range = REG_SET_FIELD!(doorbell_range, BIF_MMSCH0_DOORBELL_RANGE, SIZE, 0); }
    WREG32!(reg, doorbell_range);
}

unsafe fn nbio_v7_0_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) {
    WREG32_FIELD15!(NBIO, 0, RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable { 1 } else { 0 });
}

unsafe fn nbio_v7_0_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {}

unsafe fn nbio_v7_0_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let mut range = RREG32_SOC15!(NBIO, 0, mmBIF_IH_DOORBELL_RANGE);
    if use_doorbell {
        range = REG_SET_FIELD!(range, BIF_IH_DOORBELL_RANGE, OFFSET, doorbell_index);
        range = REG_SET_FIELD!(range, BIF_IH_DOORBELL_RANGE, SIZE, 2);
    } else { range = REG_SET_FIELD!(range, BIF_IH_DOORBELL_RANGE, SIZE, 0); }
    WREG32_SOC15!(NBIO, 0, mmBIF_IH_DOORBELL_RANGE, range);
}

unsafe fn nbio_7_0_read_syshub_ind_mmr(adev: *mut amdgpu_device, offset: u32) -> u32 {
    WREG32_SOC15!(NBIO, 0, mmSYSHUB_INDEX, offset);
    RREG32_SOC15!(NBIO, 0, mmSYSHUB_DATA)
}

unsafe fn nbio_7_0_write_syshub_ind_mmr(adev: *mut amdgpu_device, offset: u32, data: u32) {
    WREG32_SOC15!(NBIO, 0, mmSYSHUB_INDEX, offset);
    WREG32_SOC15!(NBIO, 0, mmSYSHUB_DATA, data);
}

unsafe fn nbio_v7_0_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let mut def = RREG32_PCIE!(SMN_NBIF_MGCG_CTRL_LCLK); let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG) != 0 { data |= NBIF_MGCG_CTRL_LCLK__NBIF_MGCG_EN_LCLK_MASK; }
    else { data &= !NBIF_MGCG_CTRL_LCLK__NBIF_MGCG_EN_LCLK_MASK; }
    if def != data { WREG32_PCIE!(SMN_NBIF_MGCG_CTRL_LCLK, data); }
    def = nbio_7_0_read_syshub_ind_mmr(adev, ixSYSHUB_MMREG_IND_SYSHUB_MGCG_CTRL_SOCCLK); data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG) != 0 { data |= SYSHUB_MMREG_DIRECT_SYSHUB_MGCG_CTRL_SOCCLK__SYSHUB_MGCG_EN_SOCCLK_MASK; }
    else { data &= !SYSHUB_MMREG_DIRECT_SYSHUB_MGCG_CTRL_SOCCLK__SYSHUB_MGCG_EN_SOCCLK_MASK; }
    if def != data { nbio_7_0_write_syshub_ind_mmr(adev, ixSYSHUB_MMREG_IND_SYSHUB_MGCG_CTRL_SOCCLK, data); }
    def = nbio_7_0_read_syshub_ind_mmr(adev, ixSYSHUB_MMREG_IND_SYSHUB_MGCG_CTRL_SHUBCLK); data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG) != 0 { data |= SYSHUB_MMREG_DIRECT_SYSHUB_MGCG_CTRL_SHUBCLK__SYSHUB_MGCG_EN_SHUBCLK_MASK; }
    else { data &= !SYSHUB_MMREG_DIRECT_SYSHUB_MGCG_CTRL_SHUBCLK__SYSHUB_MGCG_EN_SHUBCLK_MASK; }
    if def != data { nbio_7_0_write_syshub_ind_mmr(adev, ixSYSHUB_MMREG_IND_SYSHUB_MGCG_CTRL_SHUBCLK, data); }
}

unsafe fn nbio_v7_0_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    let def = RREG32_PCIE!(smnPCIE_CNTL2); let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) != 0 { data |= PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK; }
    else { data &= !(PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK); }
    if def != data { WREG32_PCIE!(smnPCIE_CNTL2, data); }
}

unsafe fn nbio_v7_0_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let data = RREG32_PCIE!(smnCPM_CONTROL); if data & CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_MGCG as u64; }
    let data = RREG32_PCIE!(smnPCIE_CNTL2); if data & PCIE_CNTL2__SLV_MEM_LS_EN_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_LS as u64; }
}

unsafe fn nbio_v7_0_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut c = RREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL);
    c = REG_SET_FIELD!(c, INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    c = REG_SET_FIELD!(c, INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15!(NBIO, 0, mmINTERRUPT_CNTL, c);
}

unsafe fn nbio_v7_0_get_hdp_flush_req_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmGPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v7_0_get_hdp_flush_done_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmGPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v7_0_get_pcie_index_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmPCIE_INDEX2) }
unsafe fn nbio_v7_0_get_pcie_data_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, mmPCIE_DATA2) }

pub static nbio_v7_0_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: GPU_HDP_FLUSH_DONE__SDMA1_MASK,
};

const REG_RCC_DEV0_EPF6_STRAP4: u32 = 0xd304;
const REG_RCC_DEV0_EPF6_STRAP4_BASE_IDX: u32 = 5;
const MMIO_REG_HOLE_OFFSET: u64 = 0x80000 - PAGE_SIZE as u64;

unsafe fn nbio_v7_0_init_registers(adev: *mut amdgpu_device) {
    match amdgpu_ip_version(adev, NBIO_HWIP, 0) {
        IP_VERSION!(2, 5, 0) => { let data = RREG32_SOC15!(NBIO, 0, REG_RCC_DEV0_EPF6_STRAP4) & !BIT!(23); WREG32_SOC15!(NBIO, 0, REG_RCC_DEV0_EPF6_STRAP4, data); }
        _ => {}
    }
}

unsafe fn nbio_v7_0_set_reg_remap(adev: *mut amdgpu_device) {
    if !amdgpu_sriov_vf(adev) && PAGE_SIZE <= 4096 {
        (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET as _;
        (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET;
    } else {
        (*adev).rmmio_remap.reg_offset = (SOC15_REG_OFFSET!(NBIO, 0, mmHDP_MEM_COHERENCY_FLUSH_CNTL) << 2) as _;
        (*adev).rmmio_remap.bus_addr = 0;
    }
}

pub static nbio_v7_0_funcs: amdgpu_nbio_funcs = amdgpu_nbio_funcs {
    get_hdp_flush_req_offset: Some(nbio_v7_0_get_hdp_flush_req_offset), get_hdp_flush_done_offset: Some(nbio_v7_0_get_hdp_flush_done_offset),
    get_pcie_index_offset: Some(nbio_v7_0_get_pcie_index_offset), get_pcie_data_offset: Some(nbio_v7_0_get_pcie_data_offset),
    get_rev_id: Some(nbio_v7_0_get_rev_id), mc_access_enable: Some(nbio_v7_0_mc_access_enable), get_memsize: Some(nbio_v7_0_get_memsize),
    sdma_doorbell_range: Some(nbio_v7_0_sdma_doorbell_range), vcn_doorbell_range: Some(nbio_v7_0_vcn_doorbell_range),
    enable_doorbell_aperture: Some(nbio_v7_0_enable_doorbell_aperture), enable_doorbell_selfring_aperture: Some(nbio_v7_0_enable_doorbell_selfring_aperture),
    ih_doorbell_range: Some(nbio_v7_0_ih_doorbell_range), update_medium_grain_clock_gating: Some(nbio_v7_0_update_medium_grain_clock_gating),
    update_medium_grain_light_sleep: Some(nbio_v7_0_update_medium_grain_light_sleep), get_clockgating_state: Some(nbio_v7_0_get_clockgating_state),
    ih_control: Some(nbio_v7_0_ih_control), init_registers: Some(nbio_v7_0_init_registers), remap_hdp_registers: Some(nbio_v7_0_remap_hdp_registers),
    set_reg_remap: Some(nbio_v7_0_set_reg_remap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
