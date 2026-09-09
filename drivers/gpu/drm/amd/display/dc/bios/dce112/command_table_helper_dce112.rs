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

fn signal_type_to_atom_dig_mode(s: enum_signal_type) -> u8 {
    let mut atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V6_DP;

    match s {
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_EDP => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V6_DP;
        }
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V6_DVI;
        }
        SIGNAL_TYPE_HDMI_TYPE_A => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V6_HDMI;
        }
        SIGNAL_TYPE_HDMI_FRL => {
            atom_dig_mode = 4;
        }
        SIGNAL_TYPE_DISPLAY_PORT_MST => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V6_DP_MST;
        }
        _ => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V6_DVI;
        }
    }

    atom_dig_mode
}

fn hpd_sel_to_atom(id: enum_hpd_source_id) -> u8 {
    match id {
        HPD_SOURCEID1 => ATOM_TRANSMITTER_V6_HPD1_SEL,
        HPD_SOURCEID2 => ATOM_TRANSMITTER_V6_HPD2_SEL,
        HPD_SOURCEID3 => ATOM_TRANSMITTER_V6_HPD3_SEL,
        HPD_SOURCEID4 => ATOM_TRANSMITTER_V6_HPD4_SEL,
        HPD_SOURCEID5 => ATOM_TRANSMITTER_V6_HPD5_SEL,
        HPD_SOURCEID6 => ATOM_TRANSMITTER_V6_HPD6_SEL,
        HPD_SOURCEID_UNKNOWN | _ => 0,
    }
}

fn dig_encoder_sel_to_atom(id: enum_engine_id) -> u8 {
    let _ = id;
    /* On any ASIC after DCE80, we manually program the DIG_FE
     * selection (see connect_dig_be_to_fe function of the link
     * encoder), so translation should always return 0 (no FE).
     */
    0
}

fn clock_source_id_to_atom(id: enum_clock_source_id, atom_pll_id: *mut u32) -> bool {
    let mut result = true;

    if !atom_pll_id.is_null() {
        match id {
            CLOCK_SOURCE_COMBO_PHY_PLL0 => unsafe { *atom_pll_id = ATOM_COMBOPHY_PLL0 },
            CLOCK_SOURCE_COMBO_PHY_PLL1 => unsafe { *atom_pll_id = ATOM_COMBOPHY_PLL1 },
            CLOCK_SOURCE_COMBO_PHY_PLL2 => unsafe { *atom_pll_id = ATOM_COMBOPHY_PLL2 },
            CLOCK_SOURCE_COMBO_PHY_PLL3 => unsafe { *atom_pll_id = ATOM_COMBOPHY_PLL3 },
            CLOCK_SOURCE_COMBO_PHY_PLL4 => unsafe { *atom_pll_id = ATOM_COMBOPHY_PLL4 },
            CLOCK_SOURCE_COMBO_PHY_PLL5 => unsafe { *atom_pll_id = ATOM_COMBOPHY_PLL5 },
            CLOCK_SOURCE_COMBO_DISPLAY_PLL0 => unsafe { *atom_pll_id = ATOM_PPLL0 },
            CLOCK_SOURCE_ID_DFS => unsafe { *atom_pll_id = ATOM_GCK_DFS },
            CLOCK_SOURCE_ID_VCE | CLOCK_SOURCE_ID_DP_DTO => unsafe { *atom_pll_id = ATOM_DP_DTO },
            CLOCK_SOURCE_ID_UNDEFINED => {
                unsafe { *atom_pll_id = ATOM_PPLL_INVALID };
                result = false;
            }
            _ => result = false,
        }
    }

    result
}

fn encoder_action_to_atom(action: enum_bp_encoder_control_action) -> u8 {
    let mut atom_action = 0;
    match action {
        ENCODER_CONTROL_ENABLE => atom_action = ATOM_ENABLE,
        ENCODER_CONTROL_DISABLE => atom_action = ATOM_DISABLE,
        ENCODER_CONTROL_SETUP => atom_action = ATOM_ENCODER_CMD_STREAM_SETUP,
        ENCODER_CONTROL_INIT => atom_action = ATOM_ENCODER_INIT,
        _ => BREAK_TO_DEBUGGER(), /* Unhandle action in driver.!! */
    }
    atom_action
}

fn disp_power_gating_action_to_atom(action: enum_bp_pipe_control_action) -> u8 {
    let mut atom_pipe_action = 0;
    match action {
        ASIC_PIPE_DISABLE => atom_pipe_action = ATOM_DISABLE,
        ASIC_PIPE_ENABLE => atom_pipe_action = ATOM_ENABLE,
        ASIC_PIPE_INIT => atom_pipe_action = ATOM_INIT,
        _ => ASSERT_CRITICAL(false), /* Unhandle action in driver! */
    }
    atom_pipe_action
}

fn dc_clock_type_to_atom(id: enum_bp_dce_clock_type, atom_clock_type: *mut u32) -> bool {
    let ret_code = true;
    if !atom_clock_type.is_null() {
        match id {
            DCECLOCK_TYPE_DISPLAY_CLOCK => unsafe { *atom_clock_type = DCE_CLOCK_TYPE_DISPCLK },
            DCECLOCK_TYPE_DPREFCLK => unsafe { *atom_clock_type = DCE_CLOCK_TYPE_DPREFCLK },
            _ => ASSERT_CRITICAL(false), /* Unhandle action in driver! */
        }
    }
    ret_code
}

fn transmitter_color_depth_to_atom(id: enum_transmitter_color_depth) -> u8 {
    let mut atom_color_depth = 0;
    match id {
        TRANSMITTER_COLOR_DEPTH_24 => atom_color_depth = PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_DIS,
        TRANSMITTER_COLOR_DEPTH_30 => atom_color_depth = PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_5_4,
        TRANSMITTER_COLOR_DEPTH_36 => atom_color_depth = PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_3_2,
        TRANSMITTER_COLOR_DEPTH_48 => atom_color_depth = PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_2_1,
        _ => ASSERT_CRITICAL(false), /* Unhandle action in driver! */
    }
    atom_color_depth
}

/* function table */
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
    assign_control_parameter: None,
    clock_source_id_to_ref_clk_src: None,
    transmitter_bp_to_atom: None,
    encoder_id_to_atom: dal_cmd_table_helper_encoder_id_to_atom,
    encoder_mode_bp_to_atom: dal_cmd_table_helper_encoder_mode_bp_to_atom,
    dc_clock_type_to_atom,
    transmitter_color_depth_to_atom,
};

/*
 * dal_cmd_tbl_helper_dce110_get_table
 *
 * @brief
 * Initialize command table helper functions
 *
 * @param
 * const struct command_table_helper **h - [out] struct of functions
 */
fn dal_cmd_tbl_helper_dce112_get_table() -> *const command_table_helper {
    &command_table_helper_funcs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
