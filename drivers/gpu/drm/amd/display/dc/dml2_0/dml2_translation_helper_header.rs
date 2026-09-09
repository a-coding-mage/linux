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

// Opaque types supplied by dependent translation units.
pub enum dml2_context {}
pub enum dc {}
pub enum ip_params_st {}
pub enum soc_bounding_box_st {}
pub enum soc_states_st {}
pub enum dc_state {}
pub enum dml_display_cfg_st {}
pub enum _vcs_dpi_dml_display_rq_regs_st {}
pub enum _vcs_dpi_dml_display_dlg_regs_st {}
pub enum _vcs_dpi_dml_display_ttu_regs_st {}
pub enum pipe_ctx {}

extern "C" {
    pub fn dml2_init_ip_params(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        out: *mut ip_params_st,
    );
    pub fn dml2_init_socbb_params(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        out: *mut soc_bounding_box_st,
    );
    pub fn dml2_init_soc_states(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        in_bbox: *const soc_bounding_box_st,
        out: *mut soc_states_st,
    );
    pub fn dml2_translate_ip_params(in_dc: *const dc, out: *mut ip_params_st);
    pub fn dml2_translate_socbb_params(in_dc: *const dc, out: *mut soc_bounding_box_st);
    pub fn dml2_translate_soc_states(
        in_dc: *const dc,
        out: *mut soc_states_st,
        num_states: i32,
    );
    pub fn map_dc_state_into_dml_display_cfg(
        dml2: *mut dml2_context,
        context: *mut dc_state,
        dml_dispcfg: *mut dml_display_cfg_st,
    );
    pub fn dml2_update_pipe_ctx_dchub_regs(
        rq_regs: *mut _vcs_dpi_dml_display_rq_regs_st,
        disp_dlg_regs: *mut _vcs_dpi_dml_display_dlg_regs_st,
        disp_ttu_regs: *mut _vcs_dpi_dml_display_ttu_regs_st,
        out: *mut pipe_ctx,
    );
    pub fn is_dp2p0_output_encoder(pipe: *const pipe_ctx) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
