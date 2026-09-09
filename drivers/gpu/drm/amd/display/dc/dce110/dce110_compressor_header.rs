/* Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependency: ../inc/compressor.h

#[macro_export]
macro_rules! TO_DCE110_COMPRESSOR {
    ($compressor:expr) => {
        container_of!($compressor, dce110_compressor, base)
    };
}

#[repr(C)]
pub struct dce110_compressor_reg_offsets {
    pub dcp_offset: u32,
    pub dmif_offset: u32,
}

#[repr(C)]
pub struct dce110_compressor {
    pub base: compressor,
    pub offsets: dce110_compressor_reg_offsets,
}

extern "C" {
    pub fn dce110_compressor_create(ctx: *mut dc_context) -> *mut compressor;

    pub fn dce110_compressor_construct(
        cp110: *mut dce110_compressor,
        ctx: *mut dc_context,
    );

    pub fn dce110_compressor_destroy(cp: *mut *mut compressor);

    /* FBC RELATED */
    pub fn dce110_compressor_power_up_fbc(cp: *mut compressor);

    pub fn dce110_compressor_enable_fbc(
        cp: *mut compressor,
        params: *mut compr_addr_and_pitch_params,
    );

    pub fn dce110_compressor_disable_fbc(cp: *mut compressor);

    pub fn dce110_compressor_set_fbc_invalidation_triggers(
        cp: *mut compressor,
        fbc_trigger: u32,
    );

    pub fn dce110_compressor_program_compressed_surface_address_and_pitch(
        cp: *mut compressor,
        params: *mut compr_addr_and_pitch_params,
    );

    pub fn dce110_compressor_is_fbc_enabled_in_hw(
        cp: *mut compressor,
        fbc_mapped_crtc_id: *mut u32,
    ) -> bool;

    /* LPT RELATED */
    pub fn dce110_compressor_enable_lpt(cp: *mut compressor);

    pub fn dce110_compressor_disable_lpt(cp: *mut compressor);

    pub fn dce110_compressor_program_lpt_control(
        cp: *mut compressor,
        params: *mut compr_addr_and_pitch_params,
    );

    pub fn dce110_compressor_is_lpt_enabled_in_hw(cp: *mut compressor) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
