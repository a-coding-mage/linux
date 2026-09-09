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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

const UMC_8_INST_DIST: u32 = 0x40000;

pub static umc_v8_7_channel_idx_tbl: [[u32; UMC_V8_7_CHANNEL_INSTANCE_NUM]; UMC_V8_7_UMC_INSTANCE_NUM] = [
    [2, 11], [4, 13], [1, 8], [7, 14],
    [10, 3], [12, 5], [9, 0], [15, 6],
];

unsafe fn get_umc_v8_7_reg_offset(adev: *mut amdgpu_device, umc_inst: u32, ch_inst: u32) -> u32 {
    (*adev).umc.channel_offs * ch_inst + UMC_8_INST_DIST * umc_inst
}

unsafe fn umc_v8_7_ecc_info_query_correctable_error_count(adev: *mut amdgpu_device, umc_inst: u32, ch_inst: u32, error_count: *mut c_ulong) {
    let ras = amdgpu_ras_get_context(adev);
    let idx = umc_inst * (*adev).umc.channel_inst_num + ch_inst;
    let status = (*ras).umc_ecc.ecc[idx as usize].mca_umc_status;
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, CECC) == 1 {
        *error_count += 1;
    }
}

unsafe fn umc_v8_7_ecc_info_querry_uncorrectable_error_count(adev: *mut amdgpu_device, umc_inst: u32, ch_inst: u32, error_count: *mut c_ulong) {
    let ras = amdgpu_ras_get_context(adev);
    let idx = umc_inst * (*adev).umc.channel_inst_num + ch_inst;
    let status = (*ras).umc_ecc.ecc[idx as usize].mca_umc_status;
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 &&
        (REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Deferred) == 1 ||
         REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 ||
         REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, PCC) == 1 ||
         REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 1 ||
         REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, TCC) == 1) {
        *error_count += 1;
    }
}

unsafe fn umc_v8_7_ecc_info_query_ras_error_count(adev: *mut amdgpu_device, ras_error_status: *mut c_void) {
    let data = ras_error_status as *mut ras_err_data;
    let mut umc_inst = 0;
    while umc_inst < UMC_V8_7_UMC_INSTANCE_NUM as u32 {
        let mut ch_inst = 0;
        while ch_inst < (*adev).umc.channel_inst_num {
            umc_v8_7_ecc_info_query_correctable_error_count(adev, umc_inst, ch_inst, &mut (*data).ce_count);
            umc_v8_7_ecc_info_querry_uncorrectable_error_count(adev, umc_inst, ch_inst, &mut (*data).ue_count);
            ch_inst += 1;
        }
        umc_inst += 1;
    }
}

unsafe fn umc_v8_7_convert_error_address(adev: *mut amdgpu_device, data: *mut ras_err_data, err_addr: u64, ch_inst: u32, umc_inst: u32) {
    let idx = umc_inst * (*adev).umc.channel_inst_num + ch_inst;
    let channel_index = (*adev).umc.channel_idx_tbl[idx as usize];
    let retired_page = ADDR_OF_4KB_BLOCK(err_addr) | ADDR_OF_256B_BLOCK(channel_index) | OFFSET_IN_256B_BLOCK(err_addr);
    amdgpu_umc_fill_error_record(data, err_addr, retired_page, channel_index, umc_inst);
}

unsafe fn umc_v8_7_ecc_info_query_error_address(adev: *mut amdgpu_device, data: *mut ras_err_data, ch_inst: u32, umc_inst: u32) {
    let ras = amdgpu_ras_get_context(adev);
    let idx = umc_inst * (*adev).umc.channel_inst_num + ch_inst;
    let status = (*ras).umc_ecc.ecc[idx as usize].mca_umc_status;
    if status == 0 || (*data).err_addr == 0 { return; }
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 {
        let addr = REG_GET_FIELD((*ras).umc_ecc.ecc[idx as usize].mca_umc_addr, MCA_UMC_UMC0_MCUMC_ADDRT0, ErrorAddr);
        umc_v8_7_convert_error_address(adev, data, addr, ch_inst, umc_inst);
    }
}

unsafe fn umc_v8_7_ecc_info_query_ras_error_address(adev: *mut amdgpu_device, status: *mut c_void) {
    let data = status as *mut ras_err_data;
    let mut umc_inst = 0;
    while umc_inst < UMC_V8_7_UMC_INSTANCE_NUM as u32 {
        let mut ch_inst = 0;
        while ch_inst < (*adev).umc.channel_inst_num {
            umc_v8_7_ecc_info_query_error_address(adev, data, ch_inst, umc_inst);
            ch_inst += 1;
        }
        umc_inst += 1;
    }
}

unsafe fn umc_v8_7_clear_error_count_per_channel(adev: *mut amdgpu_device, off: u32) {
    let sel_addr = SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_GeccErrCntSel);
    let cnt_addr = SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_GeccErrCnt);
    let mut sel = RREG32_PCIE((sel_addr + off) * 4);
    sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrCntCsSel, 0);
    WREG32_PCIE((sel_addr + off) * 4, sel);
    WREG32_PCIE((cnt_addr + off) * 4, UMC_V8_7_CE_CNT_INIT);
    sel = RREG32_PCIE((sel_addr + off) * 4);
    sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrCntCsSel, 1);
    WREG32_PCIE((sel_addr + off) * 4, sel);
    WREG32_PCIE((cnt_addr + off) * 4, UMC_V8_7_CE_CNT_INIT);
}

unsafe fn umc_v8_7_clear_error_count(adev: *mut amdgpu_device) {
    let mut i = 0; while i < UMC_V8_7_UMC_INSTANCE_NUM as u32 { let mut c = 0; while c < (*adev).umc.channel_inst_num { umc_v8_7_clear_error_count_per_channel(adev, get_umc_v8_7_reg_offset(adev, i, c)); c += 1; } i += 1; }
}

unsafe fn umc_v8_7_query_correctable_error_count(_adev: *mut amdgpu_device, off: u32, count: *mut c_ulong) {
    let sa = SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_GeccErrCntSel); let ca = SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_GeccErrCnt); let ma = SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0);
    let mut sel = RREG32_PCIE((sa + off) * 4); sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrCntCsSel, 0); WREG32_PCIE((sa + off) * 4, sel);
    *count += (REG_GET_FIELD(RREG32_PCIE((ca + off) * 4), UMCCH0_0_GeccErrCnt, GeccErrCnt) - UMC_V8_7_CE_CNT_INIT) as c_ulong;
    sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrCntCsSel, 1); WREG32_PCIE((sa + off) * 4, sel);
    *count += (REG_GET_FIELD(RREG32_PCIE((ca + off) * 4), UMCCH0_0_GeccErrCnt, GeccErrCnt) - UMC_V8_7_CE_CNT_INIT) as c_ulong;
    let status = RREG64_PCIE((ma + off) * 4); if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, ErrorCodeExt) == 6 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, CECC) == 1 { *count += 1; }
}

unsafe fn umc_v8_7_querry_uncorrectable_error_count(_adev: *mut amdgpu_device, off: u32, count: *mut c_ulong) {
    let status = RREG64_PCIE((SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0) + off) * 4);
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && (REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Deferred) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, PCC) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, TCC) == 1) { *count += 1; }
}

unsafe fn umc_v8_7_query_ras_error_count(adev: *mut amdgpu_device, status: *mut c_void) {
    let data = status as *mut ras_err_data; let mut i = 0; while i < UMC_V8_7_UMC_INSTANCE_NUM as u32 { let mut c = 0; while c < (*adev).umc.channel_inst_num { let off = get_umc_v8_7_reg_offset(adev, i, c); umc_v8_7_query_correctable_error_count(adev, off, &mut (*data).ce_count); umc_v8_7_querry_uncorrectable_error_count(adev, off, &mut (*data).ue_count); c += 1; } i += 1; } umc_v8_7_clear_error_count(adev);
}

unsafe fn umc_v8_7_query_error_address(adev: *mut amdgpu_device, data: *mut ras_err_data, off: u32, ch: u32, inst: u32) {
    let sa = SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_STATUST0); let aa = SOC15_REG_OFFSET(UMC, 0, mmMCA_UMC_UMC0_MCUMC_ADDRT0); let status = RREG64_PCIE((sa + off) * 4); if status == 0 { return; }
    if (*data).err_addr == 0 { WREG64_PCIE((sa + off) * 4, 0); return; }
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 { let raw = RREG64_PCIE((aa + off) * 4); let lsb = REG_GET_FIELD(raw, MCA_UMC_UMC0_MCUMC_ADDRT0, LSB); let addr = REG_GET_FIELD(raw, MCA_UMC_UMC0_MCUMC_ADDRT0, ErrorAddr) & !((1u64 << lsb) - 1); umc_v8_7_convert_error_address(adev, data, addr, ch, inst); }
    WREG64_PCIE((sa + off) * 4, 0);
}

unsafe fn umc_v8_7_query_ras_error_address(adev: *mut amdgpu_device, status: *mut c_void) { let data = status as *mut ras_err_data; let mut i = 0; while i < UMC_V8_7_UMC_INSTANCE_NUM as u32 { let mut c = 0; while c < (*adev).umc.channel_inst_num { umc_v8_7_query_error_address(adev, data, get_umc_v8_7_reg_offset(adev, i, c), c, i); c += 1; } i += 1; } }

unsafe fn umc_v8_7_err_cnt_init_per_channel(_adev: *mut amdgpu_device, off: u32) { let sa = SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_GeccErrCntSel); let ca = SOC15_REG_OFFSET(UMC, 0, mmUMCCH0_0_GeccErrCnt); let mut sel = RREG32_PCIE((sa + off) * 4); sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrCntCsSel, 0); sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrInt, 1); WREG32_PCIE((sa + off) * 4, sel); WREG32_PCIE((ca + off) * 4, UMC_V8_7_CE_CNT_INIT); sel = REG_SET_FIELD(sel, UMCCH0_0_GeccErrCntSel, GeccErrCntCsSel, 1); WREG32_PCIE((sa + off) * 4, sel); WREG32_PCIE((ca + off) * 4, UMC_V8_7_CE_CNT_INIT); }
unsafe fn umc_v8_7_err_cnt_init(adev: *mut amdgpu_device) { let mut i = 0; while i < UMC_V8_7_UMC_INSTANCE_NUM as u32 { let mut c = 0; while c < (*adev).umc.channel_inst_num { umc_v8_7_err_cnt_init_per_channel(adev, get_umc_v8_7_reg_offset(adev, i, c)); c += 1; } i += 1; } }

pub static umc_v8_7_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops { query_ras_error_count: Some(umc_v8_7_query_ras_error_count), query_ras_error_address: Some(umc_v8_7_query_ras_error_address) };
pub static mut umc_v8_7_ras: amdgpu_umc_ras = amdgpu_umc_ras { ras_block: amdgpu_ras_block { hw_ops: &umc_v8_7_ras_hw_ops }, err_cnt_init: Some(umc_v8_7_err_cnt_init), ecc_info_query_ras_error_count: Some(umc_v8_7_ecc_info_query_ras_error_count), ecc_info_query_ras_error_address: Some(umc_v8_7_ecc_info_query_ras_error_address) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
