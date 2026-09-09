/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding C/Rust translation units:
// display_mode_core_structs.h, cmntypes.h, dml_assert.h, dml_logging.h
// The __DML_DLL_EXPORT__ annotation is represented by public declarations.

unsafe extern "C" {
    pub fn dml_util_is_420(source_format: dml_source_format_class) -> dml_bool_t;
    pub fn dml_ceil(x: dml_float_t, granularity: dml_float_t) -> dml_float_t;
    pub fn dml_floor(x: dml_float_t, granularity: dml_float_t) -> dml_float_t;
    pub fn dml_min(x: dml_float_t, y: dml_float_t) -> dml_float_t;
    pub fn dml_min3(x: dml_float_t, y: dml_float_t, z: dml_float_t) -> dml_float_t;
    pub fn dml_min4(x: dml_float_t, y: dml_float_t, z: dml_float_t, w: dml_float_t) -> dml_float_t;
    pub fn dml_max(x: dml_float_t, y: dml_float_t) -> dml_float_t;
    pub fn dml_max3(x: dml_float_t, y: dml_float_t, z: dml_float_t) -> dml_float_t;
    pub fn dml_max4(a: dml_float_t, b: dml_float_t, c: dml_float_t, d: dml_float_t) -> dml_float_t;
    pub fn dml_max5(a: dml_float_t, b: dml_float_t, c: dml_float_t, d: dml_float_t, e: dml_float_t) -> dml_float_t;
    pub fn dml_log(x: dml_float_t, base: dml_float_t) -> dml_float_t;
    pub fn dml_log2(x: dml_float_t) -> dml_float_t;
    pub fn dml_round(val: dml_float_t, bankers_rounding: dml_bool_t) -> dml_float_t;
    pub fn dml_pow(base: dml_float_t, exp: ::core::ffi::c_int) -> dml_float_t;
    pub fn dml_round_to_multiple(num: dml_uint_t, multiple: dml_uint_t, up: dml_bool_t) -> dml_uint_t;
    pub fn dml_is_vertical_rotation(scan: dml_rotation_angle) -> dml_bool_t;
    pub fn dml_get_cursor_bit_per_pixel(ebpp: dml_cursor_bpp) -> dml_uint_t;
    pub fn dml_print_data_rq_regs_st(data_rq_regs: *const dml_display_plane_rq_regs_st);
    pub fn dml_print_rq_regs_st(rq_regs: *const dml_display_rq_regs_st);
    pub fn dml_print_dlg_regs_st(dlg_regs: *const dml_display_dlg_regs_st);
    pub fn dml_print_ttu_regs_st(ttu_regs: *const dml_display_ttu_regs_st);
    pub fn dml_print_dml_policy(policy: *const dml_mode_eval_policy_st);
    pub fn dml_print_mode_support(mode_lib: *mut display_mode_lib_st, j: dml_uint_t);
    pub fn dml_print_dml_mode_support_info(support: *const dml_mode_support_info_st, fail_only: dml_bool_t);
    pub fn dml_print_dml_display_cfg_timing(timing: *const dml_timing_cfg_st, num_plane: dml_uint_t);
    pub fn dml_print_dml_display_cfg_plane(plane: *const dml_plane_cfg_st, num_plane: dml_uint_t);
    pub fn dml_print_dml_display_cfg_surface(surface: *const dml_surface_cfg_st, num_plane: dml_uint_t);
    pub fn dml_print_dml_display_cfg_hw_resource(hw: *const dml_hw_resource_st, num_plane: dml_uint_t);
    pub fn dml_print_soc_state_bounding_box(state: *const soc_state_bounding_box_st);
    pub fn dml_print_soc_bounding_box(soc: *const soc_bounding_box_st);
    pub fn dml_print_clk_cfg(clk_cfg: *const dml_clk_cfg_st);

    pub fn dml_get_num_active_planes(display_cfg: *const dml_display_cfg_st) -> dml_uint_t;
    pub fn dml_get_num_active_pipes(display_cfg: *const dml_display_cfg_st) -> dml_uint_t;
    pub fn dml_get_plane_idx(mode_lib: *const display_mode_lib_st, pipe_idx: dml_uint_t) -> dml_uint_t;
    pub fn dml_get_pipe_idx(mode_lib: *const display_mode_lib_st, plane_idx: dml_uint_t) -> dml_uint_t;
    pub fn dml_calc_pipe_plane_mapping(hw: *const dml_hw_resource_st, pipe_plane: *mut dml_uint_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
