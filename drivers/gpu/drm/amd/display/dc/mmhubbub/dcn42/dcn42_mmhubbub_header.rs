/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Dependencies supplied by the corresponding C headers:
// - mcif_wb.h
// - dcn32/dcn32_mmhubbub.h
// - dcn35/dcn35_mmhubbub.h
// - dcn401/dcn401_mmhubbub.h

extern "C" {
    pub fn dcn42_mmhubbub_set_fgcg(
        mcif_wb30: *mut dcn30_mmhubbub,
        enabled: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
