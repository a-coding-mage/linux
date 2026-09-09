/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Dependencies supplied by the surrounding driver translation unit:
// dmub_srv.h, dmub_reg.h, dmub_dcn42b.h,
// dcn/dcn_4_2_1_offset.h, and dcn/dcn_4_2_1_sh_mask.h.

/*
 * These macros correspond to the C register-generation macros.  The
 * referenced types, fields, and register-list macros are supplied by the
 * surrounding driver translation unit.
 */

pub unsafe fn dmub_srv_dcn42b_regs_init(
    dmub: *mut dmub_srv,
    ctx: *mut dc_context,
) {
    // #define BASE_INNER(seg) ctx->dcn_reg_offsets[seg]
    // #define CTX dmub
    // #define REGS dmub->regs_dcn42
    // #define REG_OFFSET_EXP(reg_name) BASE(reg##reg_name##_BASE_IDX) + reg##reg_name

    let regs: *mut dmub_srv_dcn42_regs = (*dmub).regs_dcn42;

    // C: #define REG_STRUCT regs
    // C: #define DMUB_SR(reg) REG_STRUCT->offset.reg = REG_OFFSET_EXP(reg);
    // C: DMUB_DCN42_REGS()
    // C: DMCUB_INTERNAL_REGS()
    // The register-list macros expand to the individual assignments above.
    DMUB_DCN42_REGS!(regs, ctx);
    DMCUB_INTERNAL_REGS!(regs, ctx);

    // C: #define DMUB_SF(reg, field) REG_STRUCT->mask.reg##__##field = FD_MASK(reg, field);
    // C: DMUB_DCN42_FIELDS()
    DMUB_DCN42_FIELDS_MASK!(regs);

    // C: #define DMUB_SF(reg, field) REG_STRUCT->shift.reg##__##field = FD_SHIFT(reg, field);
    // C: DMUB_DCN42_FIELDS()
    DMUB_DCN42_FIELDS_SHIFT!(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
