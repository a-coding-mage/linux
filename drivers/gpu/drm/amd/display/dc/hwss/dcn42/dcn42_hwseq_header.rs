/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Dependencies supplied by the corresponding C headers:
// dc.h, hw_sequencer_private.h

extern "C" {
    pub fn dcn42_init_hw(dc: *mut dc);
    pub fn dcn42_update_mpcc(dc: *mut dc, pipe_ctx: *mut pipe_ctx);

    pub fn dcn42_program_cm_hist(
        dc: *mut dc,
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    );

    pub fn dcn42_set_mcm_luts(
        pipe_ctx: *mut pipe_ctx,
        plane_state: *const dc_plane_state,
    ) -> bool;

    pub fn dcn42_program_rmcm_luts(
        hubp: *mut hubp,
        pipe_ctx: *mut pipe_ctx,
        cm: *const dc_plane_cm,
        mpc: *mut mpc,
        mpcc_id: ::core::ffi::c_int,
    ) -> bool;

    pub fn dcn42_hardware_release(dc: *mut dc);

    pub fn dcn42_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dcn42_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);
    pub fn dcn42_calc_blocks_to_gate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    pub fn dcn42_calc_blocks_to_ungate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    pub fn dcn42_hw_block_power_down(
        dc: *mut dc,
        update_state: *mut pg_block_update,
    );
    pub fn dcn42_hw_block_power_up(
        dc: *mut dc,
        update_state: *mut pg_block_update,
    );
    pub fn dcn42_root_clock_control(
        dc: *mut dc,
        update_state: *mut pg_block_update,
        power_on: bool,
    );
    pub fn dcn42_dmub_hw_control_lock(
        dc: *mut dc,
        context: *mut dc_state,
        lock: bool,
    );
    pub fn dcn42_dmub_hw_control_lock_fast(params: *mut block_sequence_params);
    pub fn dcn42_setup_stereo(pipe_ctx: *mut pipe_ctx, dc: *mut dc);
    pub fn dcn42_power_down_on_boot(dc: *mut dc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
