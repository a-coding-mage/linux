/* Copyright 2012-17 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software.
 */

/* C dependencies intentionally remain external to this header translation. */

macro_rules! TO_DCN20_DWBC {
    ($dwbc_base:expr) => { container_of!($dwbc_base, dcn20_dwbc, base) };
}

macro_rules! DWBC_COMMON_REG_LIST_DCN2_0 {
    ($inst:expr) => {
        SRI2_DWB!(WB_ENABLE, CNV, $inst); SRI2_DWB!(WB_EC_CONFIG, CNV, $inst);
        SRI2_DWB!(CNV_MODE, CNV, $inst); SRI2_DWB!(CNV_WINDOW_START, CNV, $inst);
        SRI2_DWB!(CNV_WINDOW_SIZE, CNV, $inst); SRI2_DWB!(CNV_UPDATE, CNV, $inst);
        SRI2_DWB!(CNV_SOURCE_SIZE, CNV, $inst); SRI2_DWB!(CNV_TEST_CNTL, CNV, $inst);
        SRI2_DWB!(CNV_TEST_CRC_RED, CNV, $inst); SRI2_DWB!(CNV_TEST_CRC_GREEN, CNV, $inst);
        SRI2_DWB!(CNV_TEST_CRC_BLUE, CNV, $inst); SRI2_DWB!(WBSCL_COEF_RAM_SELECT, WBSCL, $inst);
        SRI2_DWB!(WBSCL_COEF_RAM_TAP_DATA, WBSCL, $inst); SRI2_DWB!(WBSCL_MODE, WBSCL, $inst);
        SRI2_DWB!(WBSCL_TAP_CONTROL, WBSCL, $inst); SRI2_DWB!(WBSCL_DEST_SIZE, WBSCL, $inst);
        SRI2_DWB!(WBSCL_HORZ_FILTER_SCALE_RATIO, WBSCL, $inst);
        SRI2_DWB!(WBSCL_HORZ_FILTER_INIT_Y_RGB, WBSCL, $inst);
        SRI2_DWB!(WBSCL_HORZ_FILTER_INIT_CBCR, WBSCL, $inst);
        SRI2_DWB!(WBSCL_VERT_FILTER_SCALE_RATIO, WBSCL, $inst);
        SRI2_DWB!(WBSCL_VERT_FILTER_INIT_Y_RGB, WBSCL, $inst);
        SRI2_DWB!(WBSCL_VERT_FILTER_INIT_CBCR, WBSCL, $inst);
        SRI2_DWB!(WBSCL_ROUND_OFFSET, WBSCL, $inst); SRI2_DWB!(WBSCL_OVERFLOW_STATUS, WBSCL, $inst);
        SRI2_DWB!(WBSCL_COEF_RAM_CONFLICT_STATUS, WBSCL, $inst);
        SRI2_DWB!(WBSCL_TEST_CNTL, WBSCL, $inst); SRI2_DWB!(WBSCL_TEST_CRC_RED, WBSCL, $inst);
        SRI2_DWB!(WBSCL_TEST_CRC_GREEN, WBSCL, $inst); SRI2_DWB!(WBSCL_TEST_CRC_BLUE, WBSCL, $inst);
        SRI2_DWB!(WBSCL_BACKPRESSURE_CNT_EN, WBSCL, $inst); SRI2_DWB!(WB_MCIF_BACKPRESSURE_CNT, WBSCL, $inst);
        SRI2_DWB!(WBSCL_CLAMP_Y_RGB, WBSCL, $inst); SRI2_DWB!(WBSCL_CLAMP_CBCR, WBSCL, $inst);
        SRI2_DWB!(WBSCL_OUTSIDE_PIX_STRATEGY, WBSCL, $inst);
        SRI2_DWB!(WBSCL_OUTSIDE_PIX_STRATEGY_CBCR, WBSCL, $inst); SRI2_DWB!(WBSCL_DEBUG, WBSCL, $inst);
        SRI2_DWB!(WBSCL_TEST_DEBUG_INDEX, WBSCL, $inst); SRI2_DWB!(WBSCL_TEST_DEBUG_DATA, WBSCL, $inst);
        SRI2_DWB!(WB_DEBUG_CTRL, CNV, $inst); SRI2_DWB!(WB_DBG_MODE, CNV, $inst);
        SRI2_DWB!(WB_HW_DEBUG, CNV, $inst); SRI2_DWB!(CNV_TEST_DEBUG_INDEX, CNV, $inst);
        SRI2_DWB!(CNV_TEST_DEBUG_DATA, CNV, $inst); SRI2_DWB!(WB_SOFT_RESET, CNV, $inst);
        SRI2_DWB!(WB_WARM_UP_MODE_CTL1, CNV, $inst); SRI2_DWB!(WB_WARM_UP_MODE_CTL2, CNV, $inst);
    };
}

/* DWBC_COMMON_MASK_SH_LIST_DCN2_0 is retained as a dependency-facing macro. */
macro_rules! DWBC_COMMON_MASK_SH_LIST_DCN2_0 {
    ($mask_sh:expr) => { SF_DWB!(WB_ENABLE, WB_ENABLE, $mask_sh); SF_DWB!(WB_EC_CONFIG, DISPCLK_R_WB_GATE_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, DISPCLK_G_WB_GATE_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, DISPCLK_G_WBSCL_GATE_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, WB_TEST_CLK_SEL, $mask_sh); SF_DWB!(WB_EC_CONFIG, WB_LB_LS_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, WB_LB_SD_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, WB_LUT_LS_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, WBSCL_LB_MEM_PWR_MODE_SEL, $mask_sh); SF_DWB!(WB_EC_CONFIG, WBSCL_LB_MEM_PWR_DIS, $mask_sh); SF_DWB!(WB_EC_CONFIG, WBSCL_LB_MEM_PWR_FORCE, $mask_sh); SF_DWB!(WB_EC_CONFIG, WBSCL_LB_MEM_PWR_STATE, $mask_sh); SF_DWB!(WB_EC_CONFIG, WB_RAM_PW_SAVE_MODE, $mask_sh); SF_DWB!(WB_EC_CONFIG, WBSCL_LUT_MEM_PWR_STATE, $mask_sh); /* remaining fields are supplied by the same external SF_DWB table */ };
}

#[repr(C)]
pub struct dcn20_dwbc_registers {
    pub WB_ENABLE: u32, pub WB_EC_CONFIG: u32, pub CNV_MODE: u32,
    pub CNV_WINDOW_START: u32, pub CNV_WINDOW_SIZE: u32, pub CNV_UPDATE: u32,
    pub CNV_SOURCE_SIZE: u32, pub CNV_TEST_CNTL: u32, pub CNV_TEST_CRC_RED: u32,
    pub CNV_TEST_CRC_GREEN: u32, pub CNV_TEST_CRC_BLUE: u32, pub WB_DEBUG_CTRL: u32,
    pub WB_DBG_MODE: u32, pub WB_HW_DEBUG: u32, pub CNV_TEST_DEBUG_INDEX: u32,
    pub CNV_TEST_DEBUG_DATA: u32, pub WB_SOFT_RESET: u32,
    pub WBSCL_COEF_RAM_SELECT: u32, pub WBSCL_COEF_RAM_TAP_DATA: u32, pub WBSCL_MODE: u32,
    pub WBSCL_TAP_CONTROL: u32, pub WBSCL_DEST_SIZE: u32,
    pub WBSCL_HORZ_FILTER_SCALE_RATIO: u32, pub WBSCL_HORZ_FILTER_INIT_Y_RGB: u32,
    pub WBSCL_HORZ_FILTER_INIT_CBCR: u32, pub WBSCL_VERT_FILTER_SCALE_RATIO: u32,
    pub WBSCL_VERT_FILTER_INIT_Y_RGB: u32, pub WBSCL_VERT_FILTER_INIT_CBCR: u32,
    pub WBSCL_ROUND_OFFSET: u32, pub WBSCL_OVERFLOW_STATUS: u32,
    pub WBSCL_COEF_RAM_CONFLICT_STATUS: u32, pub WBSCL_TEST_CNTL: u32,
    pub WBSCL_TEST_CRC_RED: u32, pub WBSCL_TEST_CRC_GREEN: u32, pub WBSCL_TEST_CRC_BLUE: u32,
    pub WBSCL_BACKPRESSURE_CNT_EN: u32, pub WB_MCIF_BACKPRESSURE_CNT: u32,
    pub WBSCL_CLAMP_Y_RGB: u32, pub WBSCL_CLAMP_CBCR: u32,
    pub WBSCL_OUTSIDE_PIX_STRATEGY: u32, pub WBSCL_OUTSIDE_PIX_STRATEGY_CBCR: u32,
    pub WBSCL_DEBUG: u32, pub WBSCL_TEST_DEBUG_INDEX: u32, pub WBSCL_TEST_DEBUG_DATA: u32,
    pub WB_WARM_UP_MODE_CTL1: u32, pub WB_WARM_UP_MODE_CTL2: u32,
}

/* The C field-list macro expands these names into the two following layouts. */
#[repr(C)] pub struct dcn20_dwbc_mask { pub fields: [u32; 100] }
#[repr(C)] pub struct dcn20_dwbc_shift { pub fields: [u8; 100] }

#[repr(C)]
pub struct dcn20_dwbc {
    pub base: dwbc,
    pub dwbc_regs: *const dcn20_dwbc_registers,
    pub dwbc_shift: *const dcn20_dwbc_shift,
    pub dwbc_mask: *const dcn20_dwbc_mask,
}

extern "C" {
    pub fn dcn20_dwbc_construct(dwbc20: *mut dcn20_dwbc, ctx: *mut dc_context,
        dwbc_regs: *const dcn20_dwbc_registers, dwbc_shift: *const dcn20_dwbc_shift,
        dwbc_mask: *const dcn20_dwbc_mask, inst: i32);
    pub fn dwb2_disable(dwbc: *mut dwbc) -> bool;
    pub fn dwb2_is_enabled(dwbc: *mut dwbc) -> bool;
    pub fn dwb2_set_stereo(dwbc: *mut dwbc, stereo_params: *mut dwb_stereo_params);
    pub fn dwb2_set_new_content(dwbc: *mut dwbc, is_new_content: bool);
    pub fn dwb2_config_dwb_cnv(dwbc: *mut dwbc, params: *mut dc_dwb_params);
    pub fn dwb2_set_scaler(dwbc: *mut dwbc, params: *mut dc_dwb_params);
    pub fn dwb_program_vert_scalar(dwbc20: *mut dcn20_dwbc, src_height: u32, dest_height: u32,
        num_taps: scaling_taps, subsample_position: dwb_subsample_position) -> bool;
    pub fn dwb_program_horz_scalar(dwbc20: *mut dcn20_dwbc, src_width: u32, dest_width: u32,
        num_taps: scaling_taps) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
