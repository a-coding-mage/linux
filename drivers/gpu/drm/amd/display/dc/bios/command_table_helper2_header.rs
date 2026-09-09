/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Conditional dependency: CONFIG_DRM_AMD_DC_SI selects the DCE60 helper.
// External dependencies supplied by the surrounding translation unit:
// dce80/command_table_helper_dce80.h
// dce110/command_table_helper_dce110.h
// dce112/command_table_helper2_dce112.h
// command_table_helper_struct.h

extern "C" {
    pub fn dal_bios_parser_init_cmd_tbl_helper2(
        h: *mut *const command_table_helper,
        dce: dce_version,
    ) -> bool;

    pub fn dal_cmd_table_helper_controller_id_to_atom2(
        id: controller_id,
        atom_id: *mut u8,
    ) -> bool;

    pub fn dal_cmd_table_helper_encoder_mode_bp_to_atom2(
        s: signal_type,
        enable_dp_audio: bool,
    ) -> u32;

    pub fn dal_cmd_table_helper_clock_source_id_to_ref_clk_src2(
        id: clock_source_id,
        ref_clk_src_id: *mut u32,
    ) -> bool;

    pub fn dal_cmd_table_helper_transmitter_bp_to_atom2(t: transmitter) -> u8;

    pub fn dal_cmd_table_helper_encoder_id_to_atom2(id: encoder_id) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
