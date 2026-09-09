/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

unsafe fn get_umc_v8_14_reg_offset(
    adev: *mut amdgpu_device,
    umc_inst: u32,
    ch_inst: u32,
) -> u32 {
    (*adev).umc.channel_offs * ch_inst + UMC_V8_14_INST_DIST * umc_inst
}

unsafe fn umc_v8_14_clear_error_count_per_channel(
    adev: *mut amdgpu_device,
    _node_inst: u32,
    umc_inst: u32,
    ch_inst: u32,
    _data: *mut core::ffi::c_void,
) -> i32 {
    let ecc_err_cnt_addr: u32;
    let umc_reg_offset = get_umc_v8_14_reg_offset(adev, umc_inst, ch_inst);

    ecc_err_cnt_addr = SOC15_REG_OFFSET(UMC, 0, regUMCCH0_GeccErrCnt);

    /* clear error count */
    WREG32_PCIE((ecc_err_cnt_addr + umc_reg_offset) * 4, UMC_V8_14_CE_CNT_INIT);

    0
}

unsafe fn umc_v8_14_clear_error_count(adev: *mut amdgpu_device) {
    amdgpu_umc_loop_channels(adev, umc_v8_14_clear_error_count_per_channel, core::ptr::null_mut());
}

unsafe fn umc_v8_14_query_correctable_error_count(
    adev: *mut amdgpu_device,
    umc_reg_offset: u32,
    error_count: *mut usize,
) {
    let ecc_err_cnt_addr = SOC15_REG_OFFSET(UMC, 0, regUMCCH0_GeccErrCnt);

    /* UMC 8_14 registers */
    let ecc_err_cnt = RREG32_PCIE((ecc_err_cnt_addr + umc_reg_offset) * 4);
    *error_count += (REG_GET_FIELD(ecc_err_cnt, UMCCH0_GeccErrCnt, GeccErrCnt)
        - UMC_V8_14_CE_CNT_INIT) as usize;
}

unsafe fn umc_v8_14_query_uncorrectable_error_count(
    adev: *mut amdgpu_device,
    umc_reg_offset: u32,
    error_count: *mut usize,
) {
    let ecc_err_cnt_addr = SOC15_REG_OFFSET(UMC, 0, regUMCCH0_GeccErrCnt);
    /* UMC 8_14 registers */
    let ecc_err_cnt = RREG32_PCIE((ecc_err_cnt_addr + umc_reg_offset) * 4);
    *error_count += (REG_GET_FIELD(ecc_err_cnt, UMCCH0_GeccErrCnt, GeccUnCorrErrCnt)
        - UMC_V8_14_CE_CNT_INIT) as usize;
}

unsafe fn umc_v8_14_query_error_count_per_channel(
    adev: *mut amdgpu_device,
    _node_inst: u32,
    umc_inst: u32,
    ch_inst: u32,
    data: *mut core::ffi::c_void,
) -> i32 {
    let err_data = data as *mut ras_err_data;
    let umc_reg_offset = get_umc_v8_14_reg_offset(adev, umc_inst, ch_inst);

    umc_v8_14_query_correctable_error_count(adev, umc_reg_offset, &mut (*err_data).ce_count);
    umc_v8_14_query_uncorrectable_error_count(adev, umc_reg_offset, &mut (*err_data).ue_count);

    0
}

unsafe fn umc_v8_14_query_ras_error_count(
    adev: *mut amdgpu_device,
    ras_error_status: *mut core::ffi::c_void,
) {
    amdgpu_umc_loop_channels(adev, umc_v8_14_query_error_count_per_channel, ras_error_status);
    umc_v8_14_clear_error_count(adev);
}

unsafe fn umc_v8_14_err_cnt_init_per_channel(
    adev: *mut amdgpu_device,
    _node_inst: u32,
    umc_inst: u32,
    ch_inst: u32,
    _data: *mut core::ffi::c_void,
) -> i32 {
    let umc_reg_offset = get_umc_v8_14_reg_offset(adev, umc_inst, ch_inst);
    let ecc_err_cnt_sel_addr = SOC15_REG_OFFSET(UMC, 0, regUMCCH0_GeccErrCntSel);
    let ecc_err_cnt_addr = SOC15_REG_OFFSET(UMC, 0, regUMCCH0_GeccErrCnt);

    let mut ecc_err_cnt_sel = RREG32_PCIE((ecc_err_cnt_sel_addr + umc_reg_offset) * 4);

    /* set ce error interrupt type to APIC based interrupt */
    ecc_err_cnt_sel = REG_SET_FIELD(ecc_err_cnt_sel, UMCCH0_GeccErrCntSel, GeccErrInt, 0x1);
    WREG32_PCIE((ecc_err_cnt_sel_addr + umc_reg_offset) * 4, ecc_err_cnt_sel);
    /* set error count to initial value */
    WREG32_PCIE((ecc_err_cnt_addr + umc_reg_offset) * 4, UMC_V8_14_CE_CNT_INIT);

    0
}

unsafe fn umc_v8_14_err_cnt_init(adev: *mut amdgpu_device) {
    amdgpu_umc_loop_channels(adev, umc_v8_14_err_cnt_init_per_channel, core::ptr::null_mut());
}

pub static umc_v8_14_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(umc_v8_14_query_ras_error_count),
};

pub static mut umc_v8_14_ras: amdgpu_umc_ras = amdgpu_umc_ras {
    ras_block: amdgpu_ras_block {
        hw_ops: &umc_v8_14_ras_hw_ops,
    },
    err_cnt_init: Some(umc_v8_14_err_cnt_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
