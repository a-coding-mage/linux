/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Dependencies supplied by the corresponding C/Rust translation units:
// mcif_wb.h, dcn32/dcn32_mmhubbub.h, and dcn35/dcn35_mmhubbub.h.

// MCIF_WB_REG_VARIABLE_LIST_DCN4_01 expands to
// MCIF_WB_REG_VARIABLE_LIST_DCN3_5.
//
// MCIF_WB_COMMON_MASK_SH_LIST_DCN4_01(mask_sh) expands to
// MCIF_WB_COMMON_MASK_SH_LIST_DCN3_5(mask_sh), followed by
// SF(MCIF_WB_NB_PSTATE_LATENCY_WATERMARK,
//    NB_PSTATE_CHANGE_WATERMARK_TYPE, mask_sh).
//
// MCIF_WB_REG_FIELD_LIST_DCN4_01(type) expands to the DCN3.5 register field
// list followed by a field named NB_PSTATE_CHANGE_WATERMARK_TYPE of `type`.

#[repr(C)]
pub struct dcn401_mmhubbub_mask {
    // MCIF_WB_REG_FIELD_LIST_DCN3_5(u32)
    pub nb_pstate_change_watermark_type: u32,
}

#[repr(C)]
pub struct dcn401_mmhubbub_shift {
    // MCIF_WB_REG_FIELD_LIST_DCN3_5(u8)
    pub nb_pstate_change_watermark_type: u8,
}

extern "C" {
    pub fn dcn401_mmhubbub_construct(
        mcif_wb30: *mut dcn30_mmhubbub,
        ctx: *mut dc_context,
        mcif_wb_regs: *const dcn35_mmhubbub_registers,
        mcif_wb_shift: *const dcn401_mmhubbub_shift,
        mcif_wb_mask: *const dcn401_mmhubbub_mask,
        inst: ::core::ffi::c_int,
    );
}

// External types supplied by the included headers.
pub enum dcn30_mmhubbub {}
pub enum dc_context {}
pub enum dcn35_mmhubbub_registers {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
