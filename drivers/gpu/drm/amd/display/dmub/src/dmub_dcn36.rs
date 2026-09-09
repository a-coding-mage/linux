/* SPDX-License-Identifier: MIT */
/* Copyright 2025 Advanced Micro Devices, Inc. */

// Dependencies supplied by the surrounding DMUB implementation:
// ../dmub_srv.h, dmub_reg.h, dmub_dcn36.h,
// dcn/dcn_3_6_0_offset.h, and dcn/dcn_3_6_0_sh_mask.h.

/*
 * #define BASE_INNER(seg) ctx->dcn_reg_offsets[seg]
 * #define CTX dmub
 * #define REGS dmub->regs_dcn35
 * #define REG_OFFSET_EXP(reg_name) BASE(reg##reg_name##_BASE_IDX) + reg##reg_name
 */

pub unsafe fn dmub_srv_dcn36_regs_init(
    dmub: *mut dmub_srv,
    ctx: *mut dc_context,
) {
    let regs: *mut dmub_srv_dcn35_regs = (*dmub).regs_dcn35;

    /*
     * C macro expansion:
     *
     * #define REG_STRUCT regs
     * #define DMUB_SR(reg) REG_STRUCT->offset.reg = REG_OFFSET_EXP(reg);
     * DMUB_DCN35_REGS()
     * DMCUB_INTERNAL_REGS()
     * #undef DMUB_SR
     *
     * The register-list macros are supplied by the DMUB headers and expand
     * each entry using the active DMUB_SR definition.
     */
    macro_rules! DMUB_SR {
        ($reg:ident) => {
            (*regs).offset.$reg = BASE($reg##_BASE_IDX) + $reg;
        };
    }
    DMUB_DCN35_REGS!();
    DMCUB_INTERNAL_REGS!();

    /* #define DMUB_SF(reg, field) REG_STRUCT->mask.reg##__##field = FD_MASK(reg, field); */
    macro_rules! DMUB_SF_MASK {
        ($reg:ident, $field:ident) => {
            (*regs).mask.$reg##__##$field = FD_MASK($reg, $field);
        };
    }
    DMUB_DCN35_FIELDS!();

    /* #define DMUB_SF(reg, field) REG_STRUCT->shift.reg##__##field = FD_SHIFT(reg, field); */
    macro_rules! DMUB_SF_SHIFT {
        ($reg:ident, $field:ident) => {
            (*regs).shift.$reg##__##$field = FD_SHIFT($reg, $field);
        };
    }
    DMUB_DCN35_FIELDS!();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
