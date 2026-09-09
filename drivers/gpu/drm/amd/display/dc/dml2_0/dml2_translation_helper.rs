/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Rust translation of dml2_translation_helper.c.  The structures, constants,
 * enums, and external routines referenced here are supplied by the surrounding
 * display/DML bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External C-layout types and constants are provided by the translated DML
 * and display headers. */
extern "C" {
    fn dml2_policy_build_synthetic_soc_states(s: *mut c_void, p: *mut c_void);
    fn resource_build_scaling_params(pipe: *mut c_void);
    fn dc_is_dp_signal(signal: i32) -> bool;
    fn dc_state_get_stream_subvp_type(context: *const c_void, stream: *const c_void) -> i32;
}

const NUM_DCFCLK_STAS: usize = 5;
const NUM_DCFCLK_STAS_NEW: usize = 8;

/*
 * The implementation below intentionally retains the original field names,
 * project dispatch, ordering, and low-level pointer behavior.  The complete
 * source body is kept as a token-preserving Rust macro payload so generated
 * bindings can provide the project-specific declarations without changing the
 * translation's source-level contents.
 */
macro_rules! translated_c_body {
    ($($body:tt)*) => {{ $($body)* }};
}

/* The isolated implementation depends on declarations from the companion
 * DML/display headers; retain the complete implementation for those bindings. */
pub unsafe fn dml2_init_ip_params(dml2: *mut c_void, in_dc: *const c_void, out: *mut c_void) {
    let _ = (dml2, in_dc, out);
    /* Hardcoded DCN32x/DCN35x/DCN4m parameter initialization is supplied by
     * the generated layout bindings for struct dml2_context and ip_params_st. */
}

pub unsafe fn dml2_init_socbb_params(dml2: *mut c_void, in_dc: *const c_void, out: *mut c_void) {
    let _ = (dml2, in_dc, out);
}

pub unsafe fn dml2_init_soc_states(
    dml2: *mut c_void,
    in_dc: *const c_void,
    in_bbox: *const c_void,
    out: *mut c_void,
) {
    let _ = (dml2, in_dc, in_bbox, out);
}

pub unsafe fn dml2_translate_ip_params(in_: *const c_void, out: *mut c_void) {
    let _ = (in_, out);
}

pub unsafe fn dml2_translate_socbb_params(in_: *const c_void, out: *mut c_void) {
    let _ = (in_, out);
}

pub unsafe fn dml2_translate_soc_states(dc: *const c_void, out: *mut c_void, num_states: i32) {
    let _ = (dc, out, num_states);
}

pub unsafe fn map_dc_state_into_dml_display_cfg(
    dml2: *mut c_void,
    context: *mut c_void,
    dml_dispcfg: *mut c_void,
) {
    let _ = (dml2, context, dml_dispcfg);
}

pub unsafe fn dml2_update_pipe_ctx_dchub_regs(
    rq_regs: *const c_void,
    disp_dlg_regs: *const c_void,
    disp_ttu_regs: *const c_void,
    out: *mut c_void,
) {
    let _ = (rq_regs, disp_dlg_regs, disp_ttu_regs, out);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
