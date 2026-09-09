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

// Translated from dcn30_mmhubbub.h.  The register/field-list macros below
// intentionally retain their source-level names and external dependencies.

#[macro_export]
macro_rules! TO_DCN30_MMHUBBUB {
    ($mcif_wb_base:expr) => { container_of!($mcif_wb_base, dcn30_mmhubbub, base) };
}

#[macro_export]
macro_rules! MCIF_WB_COMMON_REG_LIST_DCN3_0 {
    ($inst:expr) => {
        SRI!(MCIF_WB_BUFMGR_SW_CONTROL, MCIF_WB, $inst), SRI!(MCIF_WB_BUFMGR_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_PITCH, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_1_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_STATUS2, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_2_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_STATUS2, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_3_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_STATUS2, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_4_STATUS, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_STATUS2, MCIF_WB, $inst), SRI!(MCIF_WB_ARBITRATION_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_SCLK_CHANGE, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_1_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_2_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_3_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_4_ADDR_Y, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_C, MCIF_WB, $inst), SRI!(MCIF_WB_BUFMGR_VCE_CONTROL, MCIF_WB, $inst),
        SRI2!(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK, MMHUBBUB, $inst), SRI!(MCIF_WB_NB_PSTATE_CONTROL, MCIF_WB, $inst),
        SRI2!(MCIF_WB_WATERMARK, MMHUBBUB, $inst), SRI!(MCIF_WB_CLOCK_GATER_CONTROL, MCIF_WB, $inst),
        SRI!(MCIF_WB_SELF_REFRESH_CONTROL, MCIF_WB, $inst), SRI!(MULTI_LEVEL_QOS_CTRL, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_LUMA_SIZE, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_CHROMA_SIZE, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_ADDR_Y_HIGH, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_1_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_2_ADDR_Y_HIGH, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_2_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_ADDR_Y_HIGH, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_3_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_4_ADDR_Y_HIGH, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_4_ADDR_C_HIGH, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_1_RESOLUTION, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_2_RESOLUTION, MCIF_WB, $inst),
        SRI!(MCIF_WB_BUF_3_RESOLUTION, MCIF_WB, $inst), SRI!(MCIF_WB_BUF_4_RESOLUTION, MCIF_WB, $inst),
        SRI2!(MMHUBBUB_MEM_PWR_CNTL, MMHUBBUB, $inst), SRI2!(MMHUBBUB_WARMUP_ADDR_REGION, MMHUBBUB, $inst),
        SRI2!(MMHUBBUB_WARMUP_BASE_ADDR_HIGH, MMHUBBUB, $inst), SRI2!(MMHUBBUB_WARMUP_BASE_ADDR_LOW, MMHUBBUB, $inst),
        SRI2!(MMHUBBUB_WARMUP_CONTROL_STATUS, MMHUBBUB, $inst), SRI2!(MMHUBBUB_WARMUP_P_VMID, MMHUBBUB, $inst),
        SRI!(MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI, MCIF_WB, $inst)
    };
}

// The DCN2.0 field and variable lists are supplied by the included header;
// these macros append the DCN3.0 additions exactly as in the C header.
#[macro_export]
macro_rules! MCIF_WB_REG_FIELD_LIST_DCN3_0 {
    ($t:ty) => { MCIF_WB_REG_FIELD_LIST_DCN2_0!($t); $t WBIF_WHOLE_BUF_MODE; $t MMHUBBUB_WARMUP_ADDR_REGION; $t MMHUBBUB_WARMUP_BASE_ADDR_HIGH; $t MMHUBBUB_WARMUP_BASE_ADDR_LOW; $t MMHUBBUB_WARMUP_EN; $t MMHUBBUB_WARMUP_SW_INT_EN; $t MMHUBBUB_WARMUP_SW_INT_STATUS; $t MMHUBBUB_WARMUP_SW_INT_ACK; $t MMHUBBUB_WARMUP_INC_ADDR; $t MMHUBBUB_WARMUP_P_VMID; $t MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI; };
}

#[repr(C)]
pub struct dcn30_mmhubbub_registers {
    pub mcif_wb: mcif_wb_registers,
    pub MMHUBBUB_MEM_PWR_CNTL: u32,
    pub MMHUBBUB_WARMUP_ADDR_REGION: u32,
    pub MMHUBBUB_WARMUP_BASE_ADDR_HIGH: u32,
    pub MMHUBBUB_WARMUP_BASE_ADDR_LOW: u32,
    pub MMHUBBUB_WARMUP_CONTROL_STATUS: u32,
    pub MMHUBBUB_WARMUP_P_VMID: u32,
    pub MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI: u32,
}

#[repr(C)]
pub struct dcn30_mmhubbub_mask { pub mcif_wb: mcif_wb_mask; pub WBIF_WHOLE_BUF_MODE: u32; pub MMHUBBUB_WARMUP_ADDR_REGION: u32; pub MMHUBBUB_WARMUP_BASE_ADDR_HIGH: u32; pub MMHUBBUB_WARMUP_BASE_ADDR_LOW: u32; pub MMHUBBUB_WARMUP_EN: u32; pub MMHUBBUB_WARMUP_SW_INT_EN: u32; pub MMHUBBUB_WARMUP_SW_INT_STATUS: u32; pub MMHUBBUB_WARMUP_SW_INT_ACK: u32; pub MMHUBBUB_WARMUP_INC_ADDR: u32; pub MMHUBBUB_WARMUP_P_VMID: u32; pub MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI: u32 }
#[repr(C)]
pub struct dcn30_mmhubbub_shift { pub mcif_wb: mcif_wb_shift; pub WBIF_WHOLE_BUF_MODE: u8; pub MMHUBBUB_WARMUP_ADDR_REGION: u8; pub MMHUBBUB_WARMUP_BASE_ADDR_HIGH: u8; pub MMHUBBUB_WARMUP_BASE_ADDR_LOW: u8; pub MMHUBBUB_WARMUP_EN: u8; pub MMHUBBUB_WARMUP_SW_INT_EN: u8; pub MMHUBBUB_WARMUP_SW_INT_STATUS: u8; pub MMHUBBUB_WARMUP_SW_INT_ACK: u8; pub MMHUBBUB_WARMUP_INC_ADDR: u8; pub MMHUBBUB_WARMUP_P_VMID: u8; pub MCIF_WB_DRAM_SPEED_CHANGE_DURATION_VBI: u8 }

#[repr(C)]
pub struct dcn30_mmhubbub {
    pub base: mcif_wb,
    pub mcif_wb_regs: *const dcn30_mmhubbub_registers,
    pub mcif_wb_shift: *const dcn30_mmhubbub_shift,
    pub mcif_wb_mask: *const dcn30_mmhubbub_mask,
}

extern "C" {
    pub fn dcn30_mmhubbub_construct(
        mcif_wb30: *mut dcn30_mmhubbub,
        ctx: *mut dc_context,
        mcif_wb_regs: *const dcn30_mmhubbub_registers,
        mcif_wb_shift: *const dcn30_mmhubbub_shift,
        mcif_wb_mask: *const dcn30_mmhubbub_mask,
        inst: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
