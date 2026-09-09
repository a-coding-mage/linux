// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding DC, resource, and DM helper
// modules are intentionally not implemented in this translation unit.

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spl_in {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spl_out {
    _private: [u8; 0],
}

/* Map SPL input parameters to pipe context
 * @pipe_ctx: pipe context
 * @spl_in: spl input structure
 */
extern "C" {
    pub fn translate_SPL_in_params_from_pipe_ctx(
        pipe_ctx: *mut pipe_ctx,
        spl_in: *mut spl_in,
    );
}

/* Map SPL output parameters to pipe context
 * @pipe_ctx: pipe context
 * @spl_out: spl output structure
 */
extern "C" {
    pub fn translate_SPL_out_params_to_pipe_ctx(
        pipe_ctx: *mut pipe_ctx,
        spl_out: *mut spl_out,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
