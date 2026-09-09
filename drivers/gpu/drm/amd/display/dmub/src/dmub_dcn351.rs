/* SPDX-License-Identifier: MIT */
/* Copyright 2024 Advanced Micro Devices, Inc. */

// Dependencies supplied by the corresponding DMUB headers and generated register files.

/// Initialize the DCN 3.5.1 DMUB register offsets, masks, and shifts.
///
/// The `DMUB_DCN35_REGS`, `DMCUB_INTERNAL_REGS`, and `DMUB_DCN35_FIELDS`
/// invocations below represent the generated declaration lists from the C
/// headers.  Their expansion is intentionally left to the surrounding build.
pub unsafe fn dmub_srv_dcn351_regs_init(
    dmub: *mut dmub_srv,
    ctx: *mut dc_context,
) {
    let regs: *mut dmub_srv_dcn35_regs = (*dmub).regs_dcn35;

    // C: BASE_INNER(seg) ctx->dcn_reg_offsets[seg]
    // C: REG_OFFSET_EXP(reg_name) BASE(reg##reg_name##_BASE_IDX) + reg##reg_name
    // C: REG_STRUCT->offset.reg = REG_OFFSET_EXP(reg)
    dmub_dcn35_regs!(regs, ctx);
    dmcub_internal_regs!(regs, ctx);

    // C: REG_STRUCT->mask.reg##__##field = FD_MASK(reg, field)
    dmub_dcn35_fields_mask!(regs);

    // C: REG_STRUCT->shift.reg##__##field = FD_SHIFT(reg, field)
    dmub_dcn35_fields_shift!(regs);
}

// External C-layout types and generated register-list macros are provided by
// the translated DMUB headers/dependencies.
extern "C" {
    pub type dmub_srv;
    pub type dc_context;
    pub type dmub_srv_dcn35_regs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
