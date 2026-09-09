/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Dependency supplied by the corresponding C header: "link_encoder.h".

// Opaque declarations for types supplied by the dependency.
#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder_registers {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder_shift {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder_mask {
    _private: [u8; 0],
}

extern "C" {
    pub fn hpo_dp_link_encoder42_construct(
        enc31: *mut dcn31_hpo_dp_link_encoder,
        ctx: *mut dc_context,
        inst: u32,
        hpo_le_regs: *const dcn31_hpo_dp_link_encoder_registers,
        hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
        hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
