/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependency: dcn20/dcn20_hubbub.h

macro_rules! HUBBUB_HVM_REG_LIST {
    () => {
        SR!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_A), SR!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_B),
        SR!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_C), SR!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_D),
        SR!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A), SR!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B),
        SR!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C), SR!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D),
        SR!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A), SR!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B),
        SR!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C), SR!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D),
        SR!(DCHUBBUB_ARB_HOSTVM_CNTL), SR!(DCHVM_CTRL0), SR!(DCHVM_MEM_CTRL),
        SR!(DCHVM_CLK_CTRL), SR!(DCHVM_RIOMMU_CTRL0), SR!(DCHVM_RIOMMU_STAT0)
    };
}

macro_rules! HUBBUB_REG_LIST_DCN21 {
    () => { HUBBUB_REG_LIST_DCN20_COMMON!(), HUBBUB_SR_WATERMARK_REG_LIST!(), HUBBUB_HVM_REG_LIST!() };
}

macro_rules! HUBBUB_MASK_SH_LIST_HVM {
    ($mask_sh:expr) => {
        HUBBUB_SF!(DCHUBBUB_ARB_DF_REQ_OUTSTAND, DCHUBBUB_ARB_MIN_REQ_OUTSTAND_COMMIT_THRESHOLD, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, DCHUBBUB_ARB_FRAC_URG_BW_NOM_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, DCHUBBUB_ARB_FRAC_URG_BW_NOM_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_C, DCHUBBUB_ARB_FRAC_URG_BW_NOM_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_FRAC_URG_BW_NOM_D, DCHUBBUB_ARB_FRAC_URG_BW_NOM_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A, DCHUBBUB_ARB_VM_ROW_URGENCY_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B, DCHUBBUB_ARB_VM_ROW_URGENCY_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C, DCHUBBUB_ARB_VM_ROW_URGENCY_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D, DCHUBBUB_ARB_VM_ROW_URGENCY_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_ENTER_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_ENTER_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_ENTER_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_ENTER_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_EXIT_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_EXIT_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_EXIT_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D, DCHUBBUB_ARB_VM_ROW_ALLOW_SR_EXIT_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A, DCHUBBUB_ARB_VM_ROW_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B, DCHUBBUB_ARB_VM_ROW_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C, DCHUBBUB_ARB_VM_ROW_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D, DCHUBBUB_ARB_VM_ROW_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_HOSTVM_CNTL, DCHUBBUB_ARB_MAX_QOS_COMMIT_THRESHOLD, $mask_sh),
        HUBBUB_SF!(DCHVM_CTRL0, HOSTVM_INIT_REQ, $mask_sh), HUBBUB_SF!(DCHVM_MEM_CTRL, HVM_GPUVMRET_PWR_REQ_DIS, $mask_sh),
        HUBBUB_SF!(DCHVM_MEM_CTRL, HVM_GPUVMRET_FORCE_REQ, $mask_sh), HUBBUB_SF!(DCHVM_MEM_CTRL, HVM_GPUVMRET_POWER_STATUS, $mask_sh),
        HUBBUB_SF!(DCHVM_CLK_CTRL, HVM_DISPCLK_R_GATE_DIS, $mask_sh), HUBBUB_SF!(DCHVM_CLK_CTRL, HVM_DISPCLK_G_GATE_DIS, $mask_sh),
        HUBBUB_SF!(DCHVM_CLK_CTRL, HVM_DCFCLK_R_GATE_DIS, $mask_sh), HUBBUB_SF!(DCHVM_CLK_CTRL, HVM_DCFCLK_G_GATE_DIS, $mask_sh),
        HUBBUB_SF!(DCHVM_CLK_CTRL, TR_REQ_REQCLKREQ_MODE, $mask_sh), HUBBUB_SF!(DCHVM_CLK_CTRL, TW_RSP_COMPCLKREQ_MODE, $mask_sh),
        HUBBUB_SF!(DCHVM_RIOMMU_CTRL0, HOSTVM_PREFETCH_REQ, $mask_sh), HUBBUB_SF!(DCHVM_RIOMMU_CTRL0, HOSTVM_POWERSTATUS, $mask_sh),
        HUBBUB_SF!(DCHVM_RIOMMU_STAT0, RIOMMU_ACTIVE, $mask_sh), HUBBUB_SF!(DCHVM_RIOMMU_STAT0, HOSTVM_PREFETCH_DONE, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C, $mask_sh),
        HUBBUB_SF!(DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D, $mask_sh)
    };
}

// HUBBUB_MASK_SH_LIST_DCN21 additionally expands the common, stutter, and VM field lists.
macro_rules! HUBBUB_MASK_SH_LIST_DCN21 {
    ($mask_sh:expr) => {
        HUBBUB_MASK_SH_LIST_HVM!($mask_sh), HUBBUB_MASK_SH_LIST_DCN_COMMON!($mask_sh), HUBBUB_MASK_SH_LIST_STUTTER!($mask_sh),
        HUBBUB_SF!(DCHUBBUB_GLOBAL_TIMER_CNTL, DCHUBBUB_GLOBAL_TIMER_REFDIV, $mask_sh),
        HUBBUB_SF!(DCN_VM_FB_LOCATION_BASE, FB_BASE, $mask_sh), HUBBUB_SF!(DCN_VM_FB_LOCATION_TOP, FB_TOP, $mask_sh),
        HUBBUB_SF!(DCN_VM_FB_OFFSET, FB_OFFSET, $mask_sh), HUBBUB_SF!(DCN_VM_AGP_BOT, AGP_BOT, $mask_sh),
        HUBBUB_SF!(DCN_VM_AGP_TOP, AGP_TOP, $mask_sh), HUBBUB_SF!(DCN_VM_AGP_BASE, AGP_BASE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_ADDR_MSB, DCN_VM_FAULT_ADDR_MSB, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_ADDR_LSB, DCN_VM_FAULT_ADDR_LSB, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_CLEAR, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_STATUS_MODE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_ERROR_INTERRUPT_ENABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_RANGE_FAULT_DISABLE, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_CNTL, DCN_VM_PRQ_FAULT_DISABLE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_STATUS, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_VMID, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_TABLE_LEVEL, $mask_sh),
        HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_PIPE, $mask_sh), HUBBUB_SF!(DCN_VM_FAULT_STATUS, DCN_VM_ERROR_INTERRUPT_STATUS, $mask_sh)
    };
}

extern "C" {
    pub fn dcn21_dchvm_init(hubbub: *mut hubbub);
    pub fn hubbub21_init_dchub(hubbub: *mut hubbub, pa_config: *mut dcn_hubbub_phys_addr_config) -> ::core::ffi::c_int;
    pub fn hubbub21_program_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: ::core::ffi::c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub21_program_urgent_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: ::core::ffi::c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub21_program_stutter_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: ::core::ffi::c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub21_program_pstate_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, refclk_mhz: ::core::ffi::c_uint, safe_to_lower: bool) -> bool;
    pub fn hubbub21_wm_read_state(hubbub: *mut hubbub, wm: *mut dcn_hubbub_wm);
    pub fn hubbub21_construct(hubbub: *mut dcn20_hubbub, ctx: *mut dc_context, hubbub_regs: *const dcn_hubbub_registers, hubbub_shift: *const dcn_hubbub_shift, hubbub_mask: *const dcn_hubbub_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
