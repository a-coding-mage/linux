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

// Dependency supplied by dml2_dc_types.h.

pub struct dml2_context;
pub struct dml2_dml_to_dc_pipe_mapping;
pub struct dml_display_cfg_st;
pub struct dc_state;

/*
 * dml2_map_dc_pipes - Creates a pipe linkage in dc_state based on current display config.
 * @ctx: Input dml2 context
 * @state: Current dc_state to be updated.
 * @disp_cfg: Current display config.
 * @mapping: Pipe mapping logic structure to keep a track of pipes to be used.
 *
 * Based on ODM and DPPPersurface outputs calculated by the DML for the current display
 * config, create a pipe linkage in dc_state which is then used by DC core.
 * Make this function generic to be used by multiple DML versions.
 *
 * Return: True if pipe mapping and linking is successful, false otherwise.
 */
extern "C" {
    pub fn dml2_map_dc_pipes(
        ctx: *mut dml2_context,
        state: *mut dc_state,
        disp_cfg: *const dml_display_cfg_st,
        mapping: *mut dml2_dml_to_dc_pipe_mapping,
        existing_state: *const dc_state,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
