// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the included C headers:
//   inc/core_types.h, dc.h, dc_stream.h, hw_sequencer_private.h,
//   dcn401/dcn401_dccg.h

// Opaque declarations corresponding to types defined by those dependencies.
#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct block_sequence_state {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn50_init_hw(dc: *mut dc);

    pub fn dcn50_update_dchubp_dpp(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
    );

    pub fn dcn50_update_dchubp_dpp_sequence(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
        seq_state: *mut block_sequence_state,
    );

    pub fn dcn50_update_mpcc_sequence(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        seq_state: *mut block_sequence_state,
    );

    pub fn dcn50_program_front_end_for_ctx(dc: *mut dc, context: *mut dc_state);

    pub fn dcn50_post_unlock_program_front_end(dc: *mut dc, context: *mut dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
