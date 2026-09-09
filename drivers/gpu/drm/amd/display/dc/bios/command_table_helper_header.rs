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
 */

// The C header dependencies, including the optional CONFIG_DRM_AMD_DC_SI
// dependency, are supplied by other translated modules.

extern "C" {
    pub fn dal_bios_parser_init_cmd_tbl_helper(
        h: *mut *const command_table_helper,
        dce: dce_version,
    ) -> bool;

    pub fn dal_cmd_table_helper_controller_id_to_atom(
        id: controller_id,
        atom_id: *mut u8,
    ) -> bool;

    pub fn dal_cmd_table_helper_encoder_mode_bp_to_atom(
        s: signal_type,
        enable_dp_audio: bool,
    ) -> u32;

    pub fn dal_cmd_table_helper_assign_control_parameter(
        h: *const command_table_helper,
        control: *mut bp_encoder_control,
        ctrl_param: *mut DIG_ENCODER_CONTROL_PARAMETERS_V2,
    );

    pub fn dal_cmd_table_helper_clock_source_id_to_ref_clk_src(
        id: clock_source_id,
        ref_clk_src_id: *mut u32,
    ) -> bool;

    pub fn dal_cmd_table_helper_transmitter_bp_to_atom(t: transmitter) -> u8;

    pub fn dal_cmd_table_helper_encoder_id_to_atom(id: encoder_id) -> u8;

    pub fn phy_id_to_atom(t: transmitter) -> u8;

    pub fn clock_source_id_to_atom_phy_clk_src_id(id: clock_source_id) -> u8;

    pub fn engine_bp_to_atom(id: engine_id, atom_engine_id: *mut u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
