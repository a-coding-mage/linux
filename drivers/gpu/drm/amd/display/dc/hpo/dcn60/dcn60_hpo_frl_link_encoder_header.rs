// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency: link_encoder.h
// Dependency: dcn30/dcn30_hpo_frl_link_encoder.h

extern "C" {
    pub fn hpo_frl_link_encoder60_construct(
        enc3: *mut dcn30_hpo_frl_link_encoder,
        ctx: *mut dc_context,
        inst: u32,
        hpo_le_regs: *const dcn30_hpo_frl_link_encoder_registers,
        hpo_le_shift: *const dcn30_hpo_frl_link_encoder_shift,
        hpo_le_mask: *const dcn30_hpo_frl_link_encoder_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
