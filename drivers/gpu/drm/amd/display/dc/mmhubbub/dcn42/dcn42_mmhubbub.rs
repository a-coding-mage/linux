// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding C/Rust headers:
// dcn35/dcn35_mmhubbub.h, dcn42_mmhubbub.h, and reg_helper.h.

// C macro REG(reg) expands to the register member of the dcn35 register
// block referenced by mcif_wb30->mcif_wb_regs.
// C macro CTX expands to mcif_wb30->base.ctx.
// C macro FN(reg_name, field_name) expands to the corresponding shift and
// mask members referenced by mcif_wb30->mcif_wb_shift and mcif_wb30->mcif_wb_mask.

/// Set the MMHUBBUB fine-grained clock-gating control.
pub unsafe fn dcn42_mmhubbub_set_fgcg(
    mcif_wb30: *mut dcn30_mmhubbub,
    enabled: bool,
) {
    // Equivalent to:
    // REG_UPDATE(MMHUBBUB_CLOCK_CNTL, MMHUBBUB_FGCG_REP_DIS, !enabled);
    REG_UPDATE!(
        mcif_wb30,
        MMHUBBUB_CLOCK_CNTL,
        MMHUBBUB_FGCG_REP_DIS,
        !enabled
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
