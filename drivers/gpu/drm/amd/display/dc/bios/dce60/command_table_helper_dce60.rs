/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
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

unsafe fn encoder_action_to_atom(action: bp_encoder_control_action) -> u8 {
    let mut atom_action: u8 = 0;
    match action {
        ENCODER_CONTROL_ENABLE => atom_action = ATOM_ENABLE,
        ENCODER_CONTROL_DISABLE => atom_action = ATOM_DISABLE,
        ENCODER_CONTROL_SETUP => atom_action = ATOM_ENCODER_CMD_SETUP,
        ENCODER_CONTROL_INIT => atom_action = ATOM_ENCODER_INIT,
        _ => { BREAK_TO_DEBUGGER!(); }
    }
    atom_action
}

unsafe fn clock_source_id_to_atom(
    id: clock_source_id,
    atom_pll_id: *mut u32,
) -> bool {
    let mut result = true;
    if !atom_pll_id.is_null() {
        match id {
            CLOCK_SOURCE_ID_PLL0 => *atom_pll_id = ATOM_PPLL0,
            CLOCK_SOURCE_ID_PLL1 => *atom_pll_id = ATOM_PPLL1,
            CLOCK_SOURCE_ID_PLL2 => *atom_pll_id = ATOM_PPLL2,
            CLOCK_SOURCE_ID_EXTERNAL => *atom_pll_id = ATOM_PPLL_INVALID,
            CLOCK_SOURCE_ID_DFS => *atom_pll_id = ATOM_EXT_PLL1,
            CLOCK_SOURCE_ID_VCE => *atom_pll_id = ATOM_PPLL_INVALID,
            CLOCK_SOURCE_ID_DP_DTO => *atom_pll_id = ATOM_PPLL_INVALID,
            CLOCK_SOURCE_ID_UNDEFINED => {
                BREAK_TO_DEBUGGER!();
                *atom_pll_id = ATOM_PPLL_INVALID;
                result = false;
            }
            _ => result = false,
        }
    }
    result
}

unsafe fn signal_type_to_atom_dig_mode(s: signal_type) -> u8 {
    let mut atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DP;
    match s {
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_EDP => atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DP,
        SIGNAL_TYPE_LVDS => atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_LVDS,
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DVI,
        SIGNAL_TYPE_HDMI_TYPE_A => atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_HDMI,
        SIGNAL_TYPE_DISPLAY_PORT_MST => atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DP_MST,
        _ => atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DVI,
    }
    atom_dig_mode
}

unsafe fn hpd_sel_to_atom(id: hpd_source_id) -> u8 {
    let mut atom_hpd_sel: u8 = 0;
    match id {
        HPD_SOURCEID1 => atom_hpd_sel = ATOM_TRANSMITTER_CONFIG_V5_HPD1_SEL,
        HPD_SOURCEID2 => atom_hpd_sel = ATOM_TRANSMITTER_CONFIG_V5_HPD2_SEL,
        HPD_SOURCEID3 => atom_hpd_sel = ATOM_TRANSMITTER_CONFIG_V5_HPD3_SEL,
        HPD_SOURCEID4 => atom_hpd_sel = ATOM_TRANSMITTER_CONFIG_V5_HPD4_SEL,
        HPD_SOURCEID5 => atom_hpd_sel = ATOM_TRANSMITTER_CONFIG_V5_HPD5_SEL,
        HPD_SOURCEID6 => atom_hpd_sel = ATOM_TRANSMITTER_CONFIG_V5_HPD6_SEL,
        HPD_SOURCEID_UNKNOWN | _ => atom_hpd_sel = 0,
    }
    atom_hpd_sel >> 4
}

unsafe fn dig_encoder_sel_to_atom(id: engine_id) -> u8 {
    let mut atom_dig_encoder_sel: u8 = 0;
    match id {
        ENGINE_ID_DIGA => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGA_SEL,
        ENGINE_ID_DIGB => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGB_SEL,
        ENGINE_ID_DIGC => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGC_SEL,
        ENGINE_ID_DIGD => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGD_SEL,
        ENGINE_ID_DIGE => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGE_SEL,
        ENGINE_ID_DIGF => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGF_SEL,
        ENGINE_ID_DIGG => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGG_SEL,
        _ => atom_dig_encoder_sel = ATOM_TRANMSITTER_V5__DIGA_SEL,
    }
    atom_dig_encoder_sel
}

unsafe fn disp_power_gating_action_to_atom(action: bp_pipe_control_action) -> u8 {
    let mut atom_pipe_action: u8 = 0;
    match action {
        ASIC_PIPE_DISABLE => atom_pipe_action = ATOM_DISABLE,
        ASIC_PIPE_ENABLE => atom_pipe_action = ATOM_ENABLE,
        ASIC_PIPE_INIT => atom_pipe_action = ATOM_INIT,
        _ => { BREAK_TO_DEBUGGER!(); }
    }
    atom_pipe_action
}

static command_table_helper_funcs: command_table_helper = command_table_helper {
    controller_id_to_atom: dal_cmd_table_helper_controller_id_to_atom,
    encoder_action_to_atom,
    engine_bp_to_atom,
    clock_source_id_to_atom,
    clock_source_id_to_atom_phy_clk_src_id,
    signal_type_to_atom_dig_mode,
    hpd_sel_to_atom,
    dig_encoder_sel_to_atom,
    phy_id_to_atom,
    disp_power_gating_action_to_atom,
    assign_control_parameter: dal_cmd_table_helper_assign_control_parameter,
    clock_source_id_to_ref_clk_src: dal_cmd_table_helper_clock_source_id_to_ref_clk_src,
    transmitter_bp_to_atom: dal_cmd_table_helper_transmitter_bp_to_atom,
    encoder_id_to_atom: dal_cmd_table_helper_encoder_id_to_atom,
    encoder_mode_bp_to_atom: dal_cmd_table_helper_encoder_mode_bp_to_atom,
};

unsafe fn dal_cmd_tbl_helper_dce60_get_table() -> *const command_table_helper {
    &command_table_helper_funcs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
