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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding kernel translation.
const UMC_6_INST_DIST: u32 = 0x40000;

pub static umc_v6_1_channel_idx_tbl: [[u32; UMC_V6_1_CHANNEL_INSTANCE_NUM as usize]; UMC_V6_1_UMC_INSTANCE_NUM as usize] = [
    [2, 18, 11, 27], [4, 20, 13, 29], [1, 17, 8, 24], [7, 23, 14, 30],
    [10, 26, 3, 19], [12, 28, 5, 21], [9, 25, 0, 16], [15, 31, 6, 22],
];

unsafe fn umc_v6_1_enable_umc_index_mode(adev: *mut amdgpu_device) {
    let addr = SOC15_REG_OFFSET(RSMU, 0, mmRSMU_UMC_INDEX_REGISTER_NBIF_VG20_GPU);
    let mut val = RREG32_PCIE(addr * 4);
    val = REG_SET_FIELD(val, RSMU_UMC_INDEX_REGISTER_NBIF_VG20_GPU, RSMU_UMC_INDEX_MODE_EN, 1);
    WREG32_PCIE(addr * 4, val);
}

unsafe fn umc_v6_1_disable_umc_index_mode(adev: *mut amdgpu_device) {
    let addr = SOC15_REG_OFFSET(RSMU, 0, mmRSMU_UMC_INDEX_REGISTER_NBIF_VG20_GPU);
    let mut val = RREG32_PCIE(addr * 4);
    val = REG_SET_FIELD(val, RSMU_UMC_INDEX_REGISTER_NBIF_VG20_GPU, RSMU_UMC_INDEX_MODE_EN, 0);
    WREG32_PCIE(addr * 4, val);
}

unsafe fn umc_v6_1_get_umc_index_mode_state(adev: *mut amdgpu_device) -> u32 {
    let addr = SOC15_REG_OFFSET(RSMU, 0, mmRSMU_UMC_INDEX_REGISTER_NBIF_VG20_GPU);
    let val = RREG32_PCIE(addr * 4);
    REG_GET_FIELD(val, RSMU_UMC_INDEX_REGISTER_NBIF_VG20_GPU, RSMU_UMC_INDEX_MODE_EN)
}

#[inline]
unsafe fn get_umc_6_reg_offset(adev: *mut amdgpu_device, umc_inst: u32, ch_inst: u32) -> u32 {
    (*adev).umc.channel_offs * ch_inst + UMC_6_INST_DIST * umc_inst
}

unsafe fn umc_v6_1_clear_error_count_per_channel(adev: *mut amdgpu_device, off: u32) {
    let (sel_addr, cnt_addr) = if (*adev).asic_type == CHIP_ARCTURUS {
        (SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCntSel_ARCT), SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCnt_ARCT))
    } else {
        (SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCntSel), SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCnt))
    };
    let mut sel = RREG32_PCIE((sel_addr + off) * 4);
    sel = REG_SET_FIELD(sel, UMCCH0_0_EccErrCntSel, EccErrCntCsSel, 0);
    WREG32_PCIE((sel_addr + off) * 4, sel);
    WREG32_PCIE((cnt_addr + off) * 4, UMC_V6_1_CE_CNT_INIT);
    sel = RREG32_PCIE((sel_addr + off) * 4);
    sel = REG_SET_FIELD(sel, UMCCH0_0_EccErrCntSel, EccErrCntCsSel, 1);
    WREG32_PCIE((sel_addr + off) * 4, sel);
    WREG32_PCIE((cnt_addr + off) * 4, UMC_V6_1_CE_CNT_INIT);
}

unsafe fn umc_v6_1_clear_error_count(adev: *mut amdgpu_device) {
    let state = umc_v6_1_get_umc_index_mode_state(adev);
    if state != 0 { umc_v6_1_disable_umc_index_mode(adev); }
    for umc_inst in 0..UMC_V6_1_UMC_INSTANCE_NUM { for ch_inst in 0..UMC_V6_1_CHANNEL_INSTANCE_NUM {
        umc_v6_1_clear_error_count_per_channel(adev, get_umc_6_reg_offset(adev, umc_inst, ch_inst));
    }}
    if state != 0 { umc_v6_1_enable_umc_index_mode(adev); }
}

unsafe fn umc_v6_1_query_correctable_error_count(adev: *mut amdgpu_device, off: u32, count: *mut c_ulong) {
    let (sel_addr, cnt_addr, status_addr) = if (*adev).asic_type == CHIP_ARCTURUS {
        (SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCntSel_ARCT), SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCnt_ARCT), SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0_ARCT))
    } else {
        (SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCntSel), SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCnt), SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0))
    };
    let mut sel = RREG32_PCIE((sel_addr + off) * 4);
    sel = REG_SET_FIELD(sel, UMCCH0_0_EccErrCntSel, EccErrCntCsSel, 0); WREG32_PCIE((sel_addr + off) * 4, sel);
    *count += (REG_GET_FIELD(RREG32_PCIE((cnt_addr + off) * 4), UMCCH0_0_EccErrCnt, EccErrCnt) - UMC_V6_1_CE_CNT_INIT) as c_ulong;
    sel = REG_SET_FIELD(sel, UMCCH0_0_EccErrCntSel, EccErrCntCsSel, 1); WREG32_PCIE((sel_addr + off) * 4, sel);
    *count += (REG_GET_FIELD(RREG32_PCIE((cnt_addr + off) * 4), UMCCH0_0_EccErrCnt, EccErrCnt) - UMC_V6_1_CE_CNT_INIT) as c_ulong;
    let status = RREG64_PCIE((status_addr + off) * 4);
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, ErrorCodeExt) == 6 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, CECC) == 1 { *count += 1; }
}

unsafe fn umc_v6_1_querry_uncorrectable_error_count(adev: *mut amdgpu_device, off: u32, count: *mut c_ulong) {
    let addr = if (*adev).asic_type == CHIP_ARCTURUS { SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0_ARCT) } else { SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0) };
    let s = RREG64_PCIE((addr + off) * 4);
    if REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && (REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, Deferred) == 1 || REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 || REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, PCC) == 1 || REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 1 || REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, TCC) == 1) { *count += 1; }
}

unsafe fn umc_v6_1_query_ras_error_count(adev: *mut amdgpu_device, status: *mut c_void) {
    let e = status as *mut ras_err_data; let state = umc_v6_1_get_umc_index_mode_state(adev);
    if state != 0 { umc_v6_1_disable_umc_index_mode(adev); }
    if (*adev).asic_type == CHIP_ARCTURUS && amdgpu_dpm_set_df_cstate(adev, DF_CSTATE_DISALLOW) != 0 { drm_warn(adev_to_drm(adev), "Fail to disable DF-Cstate.\n"); }
    for i in 0..UMC_V6_1_UMC_INSTANCE_NUM { for j in 0..UMC_V6_1_CHANNEL_INSTANCE_NUM { let off = get_umc_6_reg_offset(adev, i, j); umc_v6_1_query_correctable_error_count(adev, off, &mut (*e).ce_count); umc_v6_1_querry_uncorrectable_error_count(adev, off, &mut (*e).ue_count); }}
    if (*adev).asic_type == CHIP_ARCTURUS && amdgpu_dpm_set_df_cstate(adev, DF_CSTATE_ALLOW) != 0 { drm_warn(adev_to_drm(adev), "Fail to enable DF-Cstate\n"); }
    if state != 0 { umc_v6_1_enable_umc_index_mode(adev); } umc_v6_1_clear_error_count(adev);
}

unsafe fn umc_v6_1_query_error_address(adev: *mut amdgpu_device, e: *mut ras_err_data, off: u32, ch: u32, inst: u32) {
    let (status_addr, addr_addr) = if (*adev).asic_type == CHIP_ARCTURUS { (SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0_ARCT), SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_ADDRT0_ARCT)) } else { (SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0), SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_ADDRT0)) };
    let status = RREG64_PCIE((status_addr + off) * 4); if status == 0 { return; }
    if (*e).err_addr == 0 { WREG64_PCIE((status_addr + off) * 4, 0); return; }
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 {
        let raw = RREG64_PCIE((addr_addr + off) * 4); let lsb = REG_GET_FIELD(raw, MCA_UMC_UMC0_MCUMC_ADDRT0, LSB); let a = REG_GET_FIELD(raw, MCA_UMC_UMC0_MCUMC_ADDRT0, ErrorAddr) & !((1u64 << lsb) - 1); let ci = (*adev).umc.channel_idx_tbl[(inst * (*adev).umc.channel_inst_num + ch) as usize]; let page = ADDR_OF_8KB_BLOCK(a) | ADDR_OF_256B_BLOCK(ci) | OFFSET_IN_256B_BLOCK(a); amdgpu_umc_fill_error_record(e, a, page, ci, inst);
    } WREG64_PCIE((status_addr + off) * 4, 0);
}

unsafe fn umc_v6_1_query_ras_error_address(adev: *mut amdgpu_device, status: *mut c_void) {
    let e = status as *mut ras_err_data; let state = umc_v6_1_get_umc_index_mode_state(adev); if state != 0 { umc_v6_1_disable_umc_index_mode(adev); }
    if (*adev).asic_type == CHIP_ARCTURUS && amdgpu_dpm_set_df_cstate(adev, DF_CSTATE_DISALLOW) != 0 { drm_warn(adev_to_drm(adev), "Fail to disable DF-Cstate.\n"); }
    for i in 0..UMC_V6_1_UMC_INSTANCE_NUM { for j in 0..UMC_V6_1_CHANNEL_INSTANCE_NUM { umc_v6_1_query_error_address(adev, e, get_umc_6_reg_offset(adev, i, j), j, i); }}
    if (*adev).asic_type == CHIP_ARCTURUS && amdgpu_dpm_set_df_cstate(adev, DF_CSTATE_ALLOW) != 0 { drm_warn(adev_to_drm(adev), "Fail to enable DF-Cstate\n"); } if state != 0 { umc_v6_1_enable_umc_index_mode(adev); }
}

unsafe fn umc_v6_1_err_cnt_init_per_channel(adev: *mut amdgpu_device, off: u32) {
    let (sel_addr, cnt_addr) = if (*adev).asic_type == CHIP_ARCTURUS { (SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCntSel_ARCT), SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCnt_ARCT)) } else { (SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCntSel), SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_EccErrCnt)) };
    let mut s = RREG32_PCIE((sel_addr + off) * 4); s = REG_SET_FIELD(s, UMCCH0_0_EccErrCntSel, EccErrCntCsSel, 0); s = REG_SET_FIELD(s, UMCCH0_0_EccErrCntSel, EccErrInt, 1); WREG32_PCIE((sel_addr + off) * 4, s); WREG32_PCIE((cnt_addr + off) * 4, UMC_V6_1_CE_CNT_INIT); s = REG_SET_FIELD(s, UMCCH0_0_EccErrCntSel, EccErrCntCsSel, 1); WREG32_PCIE((sel_addr + off) * 4, s); WREG32_PCIE((cnt_addr + off) * 4, UMC_V6_1_CE_CNT_INIT);
}

unsafe fn umc_v6_1_err_cnt_init(adev: *mut amdgpu_device) { let state = umc_v6_1_get_umc_index_mode_state(adev); if state != 0 { umc_v6_1_disable_umc_index_mode(adev); } for i in 0..UMC_V6_1_UMC_INSTANCE_NUM { for j in 0..UMC_V6_1_CHANNEL_INSTANCE_NUM { umc_v6_1_err_cnt_init_per_channel(adev, get_umc_6_reg_offset(adev, i, j)); }} if state != 0 { umc_v6_1_enable_umc_index_mode(adev); } }

pub static umc_v6_1_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops { query_ras_error_count: Some(umc_v6_1_query_ras_error_count), query_ras_error_address: Some(umc_v6_1_query_ras_error_address) };
pub static mut umc_v6_1_ras: amdgpu_umc_ras = amdgpu_umc_ras { ras_block: ras_block { hw_ops: &umc_v6_1_ras_hw_ops }, err_cnt_init: Some(umc_v6_1_err_cnt_init) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
