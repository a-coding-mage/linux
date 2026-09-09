// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// C dependencies:
// #include "os_types.h"
// #include "dml_top_soc_parameter_types.h"
// #include "dml_top_display_cfg_types.h"

// Forward declarations from the C header. Their definitions are supplied by
// the corresponding Rust translations/dependencies.
#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_configuration_options {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_context {
    _private: [u8; 0],
}

#[repr(C)]
pub enum dc_validate_mode {}

/**
 * dml21_init - Initialize DML21 context
 * @in_dc: dc.
 * @dml_ctx: DML21 context to initialize.
 * @config: dml21 configuration options.
 *
 * Performs FPU-requiring initialization. Must be called with FPU protection.
 */
unsafe extern "C" {
    pub fn dml21_init(
        in_dc: *const dc,
        dml_ctx: *mut dml2_context,
        config: *const dml2_configuration_options,
    );

    /**
     * dml21_validate - Determines if a display configuration is supported or not.
     * @in_dc: dc.
     * @context: dc_state to be validated.
     * @dml_ctx: dml21 context.
     * @validate_mode: DC_VALIDATE_MODE_ONLY and DC_VALIDATE_MODE_AND_STATE_INDEX
     *           will not populate context.res_ctx.
     *
     * Based on fast_validate option internally would call:
     *
     * -dml21_mode_check_and_programming - for DC_VALIDATE_MODE_AND_PROGRAMMING option
     * Calculates if dc_state can be supported on the input display
     * configuration. If supported, generates the necessary HW
     * programming for the new dc_state.
     *
     * -dml21_check_mode_support - for DC_VALIDATE_MODE_ONLY and DC_VALIDATE_MODE_AND_STATE_INDEX option
     * Calculates if dc_state can be supported for the input display
     * config.
     *
     * Context: Two threads may not invoke this function concurrently unless they reference
     *          separate dc_states for validation.
     * Return: True if mode is supported, false otherwise.
     */
    pub fn dml21_validate(
        in_dc: *const dc,
        context: *mut dc_state,
        dml_ctx: *mut dml2_context,
        validate_mode: dc_validate_mode,
    ) -> bool;

    pub fn dml21_reinit(
        in_dc: *const dc,
        dml_ctx: *mut dml2_context,
        config: *const dml2_configuration_options,
    );

    // Prepare hubp mcache_regs for hubp mcache ID and split coordinate programming
    pub fn dml21_prepare_mcache_programming(
        in_dc: *mut dc,
        context: *mut dc_state,
        dml_ctx: *mut dml2_context,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
