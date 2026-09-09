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
// amdgpu.h, nbio_v7_2.h, nbio/nbio_7_2_0_offset.h,
// nbio/nbio_7_2_0_sh_mask.h, and uapi/linux/kfd_ioctl.h.

const regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0_YC: u32 = 0x0015;
const regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0_YC_BASE_IDX: u32 = 2;
const regBIF_BX0_BIF_FB_EN_YC: u32 = 0x0100;
const regBIF_BX0_BIF_FB_EN_YC_BASE_IDX: u32 = 2;
const regBIF1_PCIE_MST_CTRL_3: u32 = 0x4601c6;
const regBIF1_PCIE_MST_CTRL_3_BASE_IDX: u32 = 5;
const BIF1_PCIE_MST_CTRL_3__CI_SWUS_MAX_READ_REQUEST_SIZE_MODE__SHIFT: u32 = 0x1b;
const BIF1_PCIE_MST_CTRL_3__CI_SWUS_MAX_READ_REQUEST_SIZE_PRIV__SHIFT: u32 = 0x1c;
const BIF1_PCIE_MST_CTRL_3__CI_SWUS_MAX_READ_REQUEST_SIZE_MODE_MASK: u32 = 0x08000000;
const BIF1_PCIE_MST_CTRL_3__CI_SWUS_MAX_READ_REQUEST_SIZE_PRIV_MASK: u32 = 0x30000000;
const regBIF1_PCIE_TX_POWER_CTRL_1: u32 = 0x460187;
const regBIF1_PCIE_TX_POWER_CTRL_1_BASE_IDX: u32 = 5;
const BIF1_PCIE_TX_POWER_CTRL_1__MST_MEM_LS_EN_MASK: u32 = 0x00000001;
const BIF1_PCIE_TX_POWER_CTRL_1__REPLAY_MEM_LS_EN_MASK: u32 = 0x00000008;

unsafe fn nbio_v7_2_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_REMAP_HDP_MEM_FLUSH_CNTL, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_REMAP_HDP_REG_FLUSH_CNTL, (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v7_2_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    let mut tmp: u32;
    match amdgpu_ip_version(adev, NBIO_HWIP, 0) {
        IP_VERSION!(7, 2, 1) | IP_VERSION!(7, 3, 0) | IP_VERSION!(7, 5, 0) => {
            tmp = RREG32_SOC15!(NBIO, 0, regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0_YC);
        }
        _ => { tmp = RREG32_SOC15!(NBIO, 0, regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0); }
    }
    tmp &= RCC_STRAP0_RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0_MASK;
    tmp >>= RCC_STRAP0_RCC_DEV0_EPF0_STRAP0__STRAP_ATI_REV_ID_DEV0_F0__SHIFT;
    tmp
}

unsafe fn nbio_v7_2_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    match amdgpu_ip_version(adev, NBIO_HWIP, 0) {
        IP_VERSION!(7, 2, 1) | IP_VERSION!(7, 3, 0) | IP_VERSION!(7, 5, 0) => {
            WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_FB_EN_YC, if enable { BIF_BX0_BIF_FB_EN__FB_READ_EN_MASK | BIF_BX0_BIF_FB_EN__FB_WRITE_EN_MASK } else { 0 });
        }
        _ => { WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_FB_EN, if enable { BIF_BX0_BIF_FB_EN__FB_READ_EN_MASK | BIF_BX0_BIF_FB_EN__FB_WRITE_EN_MASK } else { 0 }); }
    }
}

unsafe fn nbio_v7_2_get_memsize(adev: *mut amdgpu_device) -> u32 { RREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF0_0_RCC_CONFIG_MEMSIZE) }

unsafe fn nbio_v7_2_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32, use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    let reg = SOC15_REG_OFFSET!(NBIO, 0, regGDC0_BIF_SDMA0_DOORBELL_RANGE);
    let mut doorbell_range = RREG32_PCIE_PORT!(reg);
    if use_doorbell { doorbell_range = REG_SET_FIELD!(doorbell_range, GDC0_BIF_SDMA0_DOORBELL_RANGE, OFFSET, doorbell_index); doorbell_range = REG_SET_FIELD!(doorbell_range, GDC0_BIF_SDMA0_DOORBELL_RANGE, SIZE, doorbell_size); }
    else { doorbell_range = REG_SET_FIELD!(doorbell_range, GDC0_BIF_SDMA0_DOORBELL_RANGE, SIZE, 0); }
    WREG32_PCIE_PORT!(reg, doorbell_range);
}

unsafe fn nbio_v7_2_vcn_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32, instance: i32) {
    let reg = SOC15_REG_OFFSET!(NBIO, 0, regGDC0_BIF_VCN0_DOORBELL_RANGE);
    let mut doorbell_range = RREG32_PCIE_PORT!(reg);
    if use_doorbell { doorbell_range = REG_SET_FIELD!(doorbell_range, GDC0_BIF_VCN0_DOORBELL_RANGE, OFFSET, doorbell_index); doorbell_range = REG_SET_FIELD!(doorbell_range, GDC0_BIF_VCN0_DOORBELL_RANGE, SIZE, 8); }
    else { doorbell_range = REG_SET_FIELD!(doorbell_range, GDC0_BIF_VCN0_DOORBELL_RANGE, SIZE, 0); }
    WREG32_PCIE_PORT!(reg, doorbell_range);
}

unsafe fn nbio_v7_2_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut reg = RREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF0_0_RCC_DOORBELL_APER_EN);
    reg = REG_SET_FIELD!(reg, RCC_DEV0_EPF0_0_RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable { 1 } else { 0 });
    WREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF0_0_RCC_DOORBELL_APER_EN, reg);
}

unsafe fn nbio_v7_2_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp: u32 = 0;
    if enable {
        tmp = REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_EN, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_MODE, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_SIZE, 0);
        WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_LOW, lower_32_bits((*adev).doorbell.base));
        WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_HIGH, upper_32_bits((*adev).doorbell.base));
    }
    WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, tmp);
}

unsafe fn nbio_v7_2_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let reg = SOC15_REG_OFFSET!(NBIO, 0, regGDC0_BIF_IH_DOORBELL_RANGE);
    let mut ih_doorbell_range = RREG32_PCIE_PORT!(reg);
    if use_doorbell { ih_doorbell_range = REG_SET_FIELD!(ih_doorbell_range, GDC0_BIF_IH_DOORBELL_RANGE, OFFSET, doorbell_index); ih_doorbell_range = REG_SET_FIELD!(ih_doorbell_range, GDC0_BIF_IH_DOORBELL_RANGE, SIZE, 2); }
    else { ih_doorbell_range = REG_SET_FIELD!(ih_doorbell_range, GDC0_BIF_IH_DOORBELL_RANGE, SIZE, 0); }
    WREG32_PCIE_PORT!(reg, ih_doorbell_range);
}

unsafe fn nbio_v7_2_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut interrupt_cntl = RREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL);
    interrupt_cntl = REG_SET_FIELD!(interrupt_cntl, BIF_BX0_INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    interrupt_cntl = REG_SET_FIELD!(interrupt_cntl, BIF_BX0_INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL, interrupt_cntl);
}

unsafe fn nbio_v7_2_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let def = RREG32_PCIE_PORT!(SOC15_REG_OFFSET!(NBIO, 0, regCPM_CONTROL));
    let mut data = def;
    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_MGCG) != 0 { data |= CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_LCNT_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_REGS_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PRBS_GATE_ENABLE_MASK | CPM_CONTROL__REFCLK_REGS_GATE_ENABLE_MASK; }
    else { data &= !(CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_DYN_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_LCNT_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_REGS_GATE_ENABLE_MASK | CPM_CONTROL__TXCLK_PRBS_GATE_ENABLE_MASK | CPM_CONTROL__REFCLK_REGS_GATE_ENABLE_MASK); }
    if def != data { WREG32_PCIE_PORT!(SOC15_REG_OFFSET!(NBIO, 0, regCPM_CONTROL), data); }
}

unsafe fn nbio_v7_2_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) {
    match amdgpu_ip_version(adev, NBIO_HWIP, 0) {
        IP_VERSION!(7, 2, 1) | IP_VERSION!(7, 3, 0) | IP_VERSION!(7, 5, 0) => {
            let reg = SOC15_REG_OFFSET!(NBIO, 0, regPCIE_CNTL2); let def = RREG32_PCIE_PORT!(reg); let mut data = def;
            if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) != 0 { data |= PCIE_CNTL2__SLV_MEM_LS_EN_MASK; } else { data &= !PCIE_CNTL2__SLV_MEM_LS_EN_MASK; } if def != data { WREG32_PCIE_PORT!(reg, data); }
            let reg = SOC15_REG_OFFSET!(NBIO, 0, regBIF1_PCIE_TX_POWER_CTRL_1); let def = RREG32_PCIE_PORT!(reg); let mut data = def;
            if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) != 0 { data |= BIF1_PCIE_TX_POWER_CTRL_1__MST_MEM_LS_EN_MASK | BIF1_PCIE_TX_POWER_CTRL_1__REPLAY_MEM_LS_EN_MASK; } else { data &= !(BIF1_PCIE_TX_POWER_CTRL_1__MST_MEM_LS_EN_MASK | BIF1_PCIE_TX_POWER_CTRL_1__REPLAY_MEM_LS_EN_MASK); } if def != data { WREG32_PCIE_PORT!(reg, data); }
        }
        _ => {
            let reg = SOC15_REG_OFFSET!(NBIO, 0, regPCIE_CNTL2); let def = RREG32_PCIE_PORT!(reg); let mut data = def;
            if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_BIF_LS) != 0 { data |= PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK; } else { data &= !(PCIE_CNTL2__SLV_MEM_LS_EN_MASK | PCIE_CNTL2__MST_MEM_LS_EN_MASK | PCIE_CNTL2__REPLAY_MEM_LS_EN_MASK); } if def != data { WREG32_PCIE_PORT!(reg, data); }
        }
    }
}

unsafe fn nbio_v7_2_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let data = RREG32_PCIE_PORT!(SOC15_REG_OFFSET!(NBIO, 0, regCPM_CONTROL)); if data & CPM_CONTROL__LCLK_DYN_GATE_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_MGCG; }
    let data = RREG32_PCIE_PORT!(SOC15_REG_OFFSET!(NBIO, 0, regPCIE_CNTL2)); if data & PCIE_CNTL2__SLV_MEM_LS_EN_MASK != 0 { *flags |= AMD_CG_SUPPORT_BIF_LS; }
}

unsafe fn nbio_v7_2_get_hdp_flush_req_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_GPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v7_2_get_hdp_flush_done_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_GPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v7_2_get_pcie_index_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX0_PCIE_INDEX2) }
unsafe fn nbio_v7_2_get_pcie_data_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX0_PCIE_DATA2) }
unsafe fn nbio_v7_2_get_pcie_port_index_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_RSMU_INDEX) }
unsafe fn nbio_v7_2_get_pcie_port_data_offset(adev: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_RSMU_DATA) }

const MMIO_REG_HOLE_OFFSET: u32 = 0x80000 - PAGE_SIZE;

unsafe fn nbio_v7_2_set_reg_remap(adev: *mut amdgpu_device) {
    if !amdgpu_sriov_vf(adev) && PAGE_SIZE <= 4096 { (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET; (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET; }
    else { (*adev).rmmio_remap.reg_offset = SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_HDP_MEM_COHERENCY_FLUSH_CNTL) << 2; (*adev).rmmio_remap.bus_addr = 0; }
}

// The following aggregate definitions preserve the C ABI-facing objects and callback table.
pub static nbio_v7_2_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA1_MASK,
};

unsafe fn nbio_v7_2_init_registers(adev: *mut amdgpu_device) {
    match amdgpu_ip_version(adev, NBIO_HWIP, 0) {
        IP_VERSION!(7, 2, 1) | IP_VERSION!(7, 3, 0) | IP_VERSION!(7, 5, 0) => {
            let reg = SOC15_REG_OFFSET!(NBIO, 0, regBIF1_PCIE_MST_CTRL_3); let def = RREG32_PCIE_PORT!(reg); let mut data = def;
            data = REG_SET_FIELD!(data, BIF1_PCIE_MST_CTRL_3, CI_SWUS_MAX_READ_REQUEST_SIZE_MODE, 1);
            data = REG_SET_FIELD!(data, BIF1_PCIE_MST_CTRL_3, CI_SWUS_MAX_READ_REQUEST_SIZE_PRIV, 1);
            if def != data { WREG32_PCIE_PORT!(reg, data); }
        }
        _ => {
            let reg = SOC15_REG_OFFSET!(NBIO, 0, regPCIE_CONFIG_CNTL); let def = RREG32_PCIE_PORT!(reg); let mut data = def;
            data = REG_SET_FIELD!(data, PCIE_CONFIG_CNTL, CI_SWUS_MAX_READ_REQUEST_SIZE_MODE, 1);
            data = REG_SET_FIELD!(data, PCIE_CONFIG_CNTL, CI_SWUS_MAX_READ_REQUEST_SIZE_PRIV, 1);
            if def != data { WREG32_PCIE_PORT!(reg, data); }
        }
    }
    match amdgpu_ip_version(adev, NBIO_HWIP, 0) {
        IP_VERSION!(7, 3, 0) | IP_VERSION!(7, 5, 1) => {
            let mut data = RREG32_SOC15!(NBIO, 0, regRCC_DEV2_EPF0_STRAP2);
            data &= !RCC_DEV2_EPF0_STRAP2__STRAP_NO_SOFT_RESET_DEV2_F0_MASK;
            WREG32_SOC15!(NBIO, 0, regRCC_DEV2_EPF0_STRAP2, data);
        }
        _ => {}
    }
}

pub static nbio_v7_2_funcs: amdgpu_nbio_funcs = amdgpu_nbio_funcs {
    get_hdp_flush_req_offset: Some(nbio_v7_2_get_hdp_flush_req_offset),
    get_hdp_flush_done_offset: Some(nbio_v7_2_get_hdp_flush_done_offset),
    get_pcie_index_offset: Some(nbio_v7_2_get_pcie_index_offset),
    get_pcie_data_offset: Some(nbio_v7_2_get_pcie_data_offset),
    get_pcie_port_index_offset: Some(nbio_v7_2_get_pcie_port_index_offset),
    get_pcie_port_data_offset: Some(nbio_v7_2_get_pcie_port_data_offset),
    get_rev_id: Some(nbio_v7_2_get_rev_id), mc_access_enable: Some(nbio_v7_2_mc_access_enable),
    get_memsize: Some(nbio_v7_2_get_memsize), sdma_doorbell_range: Some(nbio_v7_2_sdma_doorbell_range),
    vcn_doorbell_range: Some(nbio_v7_2_vcn_doorbell_range), enable_doorbell_aperture: Some(nbio_v7_2_enable_doorbell_aperture),
    enable_doorbell_selfring_aperture: Some(nbio_v7_2_enable_doorbell_selfring_aperture), ih_doorbell_range: Some(nbio_v7_2_ih_doorbell_range),
    update_medium_grain_clock_gating: Some(nbio_v7_2_update_medium_grain_clock_gating), update_medium_grain_light_sleep: Some(nbio_v7_2_update_medium_grain_light_sleep),
    get_clockgating_state: Some(nbio_v7_2_get_clockgating_state), ih_control: Some(nbio_v7_2_ih_control),
    init_registers: Some(nbio_v7_2_init_registers), remap_hdp_registers: Some(nbio_v7_2_remap_hdp_registers),
    set_reg_remap: Some(nbio_v7_2_set_reg_remap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
