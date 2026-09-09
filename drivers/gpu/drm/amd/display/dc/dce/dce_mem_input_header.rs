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

// C dependencies: dc_hw_types.h and mem_input.h.
// The register-list and field-list preprocessor macros are preserved as
// declarative macro names below; their token-pasting expressions depend on
// the platform register definitions supplied by those headers.

macro_rules! TO_DCE_MEM_INPUT { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE_BASE_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE_PTE_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE6_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE8_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE11_2_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE11_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE12_REG_LIST { ($($tt:tt)*) => {}; }
macro_rules! SFB { ($($tt:tt)*) => {}; }
macro_rules! MI_GFX6_TILE_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_GFX8_TILE_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCP_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCP_DCE6_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCP_DCE11_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCP_PTE_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DMIF_PG_MASK_SH_LIST_DCE6 { ($($tt:tt)*) => {}; }
macro_rules! MI_DMIF_PG_MASK_SH_DCE6 { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE6_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DMIF_PG_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DMIF_PG_MASK_SH_DCE { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE8_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE11_2_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE11_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_GFX9_TILE_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE12_DMIF_PG_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_GFX9_DCHUB_MASK_SH_LIST { ($($tt:tt)*) => {}; }
macro_rules! MI_DCE12_MASK_SH_LIST { ($($tt:tt)*) => {}; }

#[repr(C)]
pub struct dce_mem_input_registers {
    pub GRPH_ENABLE: u32, pub GRPH_CONTROL: u32, pub GRPH_X_START: u32,
    pub GRPH_Y_START: u32, pub GRPH_X_END: u32, pub GRPH_Y_END: u32,
    pub GRPH_PITCH: u32, pub HW_ROTATION: u32, pub GRPH_SWAP_CNTL: u32,
    pub PRESCALE_GRPH_CONTROL: u32, pub GRPH_PIPE_OUTSTANDING_REQUEST_LIMIT: u32,
    pub DVMM_PTE_CONTROL: u32, pub DVMM_PTE_ARB_CONTROL: u32,
    pub GRPH_UPDATE: u32, pub GRPH_FLIP_CONTROL: u32,
    pub GRPH_PRIMARY_SURFACE_ADDRESS: u32, pub GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: u32,
    pub GRPH_SECONDARY_SURFACE_ADDRESS: u32, pub GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: u32,
    pub DPG_PIPE_ARBITRATION_CONTROL1: u32,
    #[cfg(CONFIG_DRM_AMD_DC_SI)] pub DPG_PIPE_ARBITRATION_CONTROL3: u32,
    pub DPG_WATERMARK_MASK_CONTROL: u32, pub DPG_PIPE_URGENCY_CONTROL: u32,
    pub DPG_PIPE_URGENT_LEVEL_CONTROL: u32, pub DPG_PIPE_NB_PSTATE_CHANGE_CONTROL: u32,
    pub DPG_PIPE_LOW_POWER_CONTROL: u32, pub DPG_PIPE_STUTTER_CONTROL: u32,
    pub DPG_PIPE_STUTTER_CONTROL2: u32, pub DMIF_BUFFER_CONTROL: u32,
    pub MC_HUB_RDREQ_DMIF_LIMIT: u32, pub DCHUB_FB_LOCATION: u32,
    pub DCHUB_AGP_BASE: u32, pub DCHUB_AGP_BOT: u32, pub DCHUB_AGP_TOP: u32,
}

#[repr(C)]
pub struct dce_mem_input_shift {
    pub GRPH_ENABLE: u8, pub GRPH_X_START: u8, pub GRPH_Y_START: u8, pub GRPH_X_END: u8,
    pub GRPH_Y_END: u8, pub GRPH_PITCH: u8, pub GRPH_ROTATION_ANGLE: u8,
    pub GRPH_RED_CROSSBAR: u8, pub GRPH_BLUE_CROSSBAR: u8, pub GRPH_PRESCALE_SELECT: u8,
    pub GRPH_PRESCALE_R_SIGN: u8, pub GRPH_PRESCALE_G_SIGN: u8, pub GRPH_PRESCALE_B_SIGN: u8,
    pub GRPH_PIPE_OUTSTANDING_REQUEST_LIMIT: u8, pub DVMM_PAGE_WIDTH: u8, pub DVMM_PAGE_HEIGHT: u8,
    pub DVMM_MIN_PTE_BEFORE_FLIP: u8, pub DVMM_PTE_REQ_PER_CHUNK: u8, pub DVMM_MAX_PTE_REQ_OUTSTANDING: u8,
    pub GRPH_DEPTH: u8, pub GRPH_FORMAT: u8, pub GRPH_NUM_BANKS: u8, pub GRPH_BANK_WIDTH: u8,
    pub GRPH_BANK_HEIGHT: u8, pub GRPH_MACRO_TILE_ASPECT: u8, pub GRPH_TILE_SPLIT: u8,
    pub GRPH_MICRO_TILE_MODE: u8, pub GRPH_PIPE_CONFIG: u8, pub GRPH_ARRAY_MODE: u8,
    pub GRPH_COLOR_EXPANSION_MODE: u8, pub GRPH_SW_MODE: u8, pub GRPH_SE_ENABLE: u8,
    pub GRPH_NUM_SHADER_ENGINES: u8, pub GRPH_NUM_PIPES: u8,
    pub GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: u8, pub GRPH_SECONDARY_SURFACE_ADDRESS: u8,
    pub GRPH_SECONDARY_DFQ_ENABLE: u8, pub GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: u8,
    pub GRPH_PRIMARY_SURFACE_ADDRESS: u8, pub GRPH_SURFACE_UPDATE_PENDING: u8,
    pub GRPH_SURFACE_UPDATE_H_RETRACE_EN: u8, pub GRPH_UPDATE_LOCK: u8, pub PIXEL_DURATION: u8,
    pub URGENCY_WATERMARK_MASK: u8, pub PSTATE_CHANGE_WATERMARK_MASK: u8,
    pub NB_PSTATE_CHANGE_WATERMARK_MASK: u8, pub STUTTER_EXIT_SELF_REFRESH_WATERMARK_MASK: u8,
    pub URGENCY_LOW_WATERMARK: u8, pub URGENCY_HIGH_WATERMARK: u8,
    pub URGENT_LEVEL_LOW_WATERMARK: u8, pub URGENT_LEVEL_HIGH_WATERMARK: u8,
    pub NB_PSTATE_CHANGE_ENABLE: u8, pub NB_PSTATE_CHANGE_URGENT_DURING_REQUEST: u8,
    pub NB_PSTATE_CHANGE_NOT_SELF_REFRESH_DURING_REQUEST: u8, pub NB_PSTATE_CHANGE_WATERMARK: u8,
    pub PSTATE_CHANGE_ENABLE: u8, pub PSTATE_CHANGE_URGENT_DURING_REQUEST: u8,
    pub PSTATE_CHANGE_NOT_SELF_REFRESH_DURING_REQUEST: u8, pub PSTATE_CHANGE_WATERMARK: u8,
    pub STUTTER_ENABLE: u8, pub STUTTER_IGNORE_FBC: u8, pub STUTTER_EXIT_SELF_REFRESH_WATERMARK: u8,
    pub STUTTER_ENTER_SELF_REFRESH_WATERMARK: u8, pub DMIF_BUFFERS_ALLOCATED: u8,
    pub DMIF_BUFFERS_ALLOCATION_COMPLETED: u8, pub ENABLE: u8, pub FB_BASE: u8,
    pub FB_TOP: u8, pub AGP_BASE: u8, pub AGP_TOP: u8, pub AGP_BOT: u8,
}

#[repr(C)]
pub struct dce_mem_input_mask {
    // MI_REG_FIELD_LIST(uint32_t); field order is identical to dce_mem_input_shift.
    pub fields: [u32; 76],
}

#[repr(C)]
pub struct dce_mem_input_wa { pub single_head_rdreq_dmif_limit: u8 }

#[repr(C)]
pub struct dce_mem_input {
    pub base: mem_input,
    pub regs: *const dce_mem_input_registers,
    pub shifts: *const dce_mem_input_shift,
    pub masks: *const dce_mem_input_mask,
    pub wa: dce_mem_input_wa,
}

extern "C" {
    pub fn dce_mem_input_construct(dce_mi: *mut dce_mem_input, ctx: *mut dc_context, inst: i32,
        regs: *const dce_mem_input_registers, mi_shift: *const dce_mem_input_shift,
        mi_mask: *const dce_mem_input_mask);
    #[cfg(CONFIG_DRM_AMD_DC_SI)]
    pub fn dce60_mem_input_construct(dce_mi: *mut dce_mem_input, ctx: *mut dc_context, inst: i32,
        regs: *const dce_mem_input_registers, mi_shift: *const dce_mem_input_shift,
        mi_mask: *const dce_mem_input_mask);
    pub fn dce112_mem_input_construct(dce_mi: *mut dce_mem_input, ctx: *mut dc_context, inst: i32,
        regs: *const dce_mem_input_registers, mi_shift: *const dce_mem_input_shift,
        mi_mask: *const dce_mem_input_mask);
    pub fn dce120_mem_input_construct(dce_mi: *mut dce_mem_input, ctx: *mut dc_context, inst: i32,
        regs: *const dce_mem_input_registers, mi_shift: *const dce_mem_input_shift,
        mi_mask: *const dce_mem_input_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
