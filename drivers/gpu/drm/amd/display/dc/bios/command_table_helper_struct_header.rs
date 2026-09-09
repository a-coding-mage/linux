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

// Dependencies supplied by the corresponding DCE command-table headers.

pub struct _DIG_ENCODER_CONTROL_PARAMETERS_V2;

#[repr(C)]
pub struct command_table_helper {
    pub controller_id_to_atom:
        Option<unsafe extern "C" fn(id: controller_id, atom_id: *mut u8) -> bool>,
    pub encoder_action_to_atom:
        Option<unsafe extern "C" fn(action: bp_encoder_control_action) -> u8>,
    pub encoder_mode_bp_to_atom:
        Option<unsafe extern "C" fn(s: signal_type, enable_dp_audio: bool) -> u32>,
    pub engine_bp_to_atom:
        Option<unsafe extern "C" fn(engine_id: engine_id, atom_engine_id: *mut u32) -> bool>,
    pub assign_control_parameter: Option<unsafe extern "C" fn(
        h: *const command_table_helper,
        control: *mut bp_encoder_control,
        ctrl_param: *mut _DIG_ENCODER_CONTROL_PARAMETERS_V2,
    )>,
    pub clock_source_id_to_atom:
        Option<unsafe extern "C" fn(id: clock_source_id, atom_pll_id: *mut u32) -> bool>,
    pub clock_source_id_to_ref_clk_src: Option<unsafe extern "C" fn(
        id: clock_source_id,
        ref_clk_src_id: *mut u32,
    ) -> bool>,
    pub transmitter_bp_to_atom:
        Option<unsafe extern "C" fn(t: transmitter) -> u8>,
    pub encoder_id_to_atom:
        Option<unsafe extern "C" fn(id: encoder_id) -> u8>,
    pub clock_source_id_to_atom_phy_clk_src_id:
        Option<unsafe extern "C" fn(id: clock_source_id) -> u8>,
    pub signal_type_to_atom_dig_mode:
        Option<unsafe extern "C" fn(s: signal_type) -> u8>,
    pub hpd_sel_to_atom:
        Option<unsafe extern "C" fn(id: hpd_source_id) -> u8>,
    pub dig_encoder_sel_to_atom:
        Option<unsafe extern "C" fn(engine_id: engine_id) -> u8>,
    pub phy_id_to_atom:
        Option<unsafe extern "C" fn(t: transmitter) -> u8>,
    pub disp_power_gating_action_to_atom:
        Option<unsafe extern "C" fn(action: bp_pipe_control_action) -> u8>,
    pub dc_clock_type_to_atom: Option<unsafe extern "C" fn(
        id: bp_dce_clock_type,
        atom_clock_type: *mut u32,
    ) -> bool>,
    pub transmitter_color_depth_to_atom:
        Option<unsafe extern "C" fn(id: transmitter_color_depth) -> u8>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
