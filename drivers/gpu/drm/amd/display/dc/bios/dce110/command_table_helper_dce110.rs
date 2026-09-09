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

/* C includes are supplied by the surrounding translation unit. */

static unsafe fn signal_type_to_atom_dig_mode(s: signal_type) -> u8 {
    let mut atom_dig_mode: u8 = ATOM_TRANSMITTER_DIGMODE_V5_DP;
    match s {
        SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_EDP => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DP;
        }
        SIGNAL_TYPE_LVDS => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_LVDS;
        }
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DVI;
        }
        SIGNAL_TYPE_HDMI_TYPE_A => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_HDMI;
        }
        SIGNAL_TYPE_DISPLAY_PORT_MST => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DP_MST;
        }
        _ => {
            atom_dig_mode = ATOM_TRANSMITTER_DIGMODE_V5_DVI;
        }
    }
    atom_dig_mode
}

static unsafe fn hpd_sel_to_atom(id: hpd_source_id) -> u8 {
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

static unsafe fn dig_encoder_sel_to_atom(_id: engine_id) -> u8 {
    /* On any ASIC after DCE80, we manually program the DIG_FE
     * selection (see connect_dig_be_to_fe function of the link
     * encoder), so translation should always return 0 (no FE).
     */
    0
}

static unsafe fn clock_source_id_to_atom(id: clock_source_id, atom_pll_id: *mut u32) -> bool {
    let mut result = true;
    if !atom_pll_id.is_null() {
        match id {
            CLOCK_SOURCE_ID_PLL0 => *atom_pll_id = ATOM_PPLL0,
            CLOCK_SOURCE_ID_PLL1 => *atom_pll_id = ATOM_PPLL1,
            CLOCK_SOURCE_ID_PLL2 => *atom_pll_id = ATOM_PPLL2,
            CLOCK_SOURCE_ID_EXTERNAL => *atom_pll_id = ATOM_PPLL_INVALID,
            CLOCK_SOURCE_ID_DFS => *atom_pll_id = ATOM_EXT_PLL1,
            CLOCK_SOURCE_ID_VCE => {
                /* for VCE encoding, we need to pass in ATOM_PPLL_INVALID */
                *atom_pll_id = ATOM_PPLL_INVALID;
            }
            CLOCK_SOURCE_ID_DP_DTO => {
                /* When programming DP DTO PLL ID should be invalid */
                *atom_pll_id = ATOM_PPLL_INVALID;
            }
            CLOCK_SOURCE_ID_UNDEFINED => {
                /* Should not happen */
                *atom_pll_id = ATOM_PPLL_INVALID;
                result = false;
            }
            _ => result = false,
        }
    }
    result
}

static unsafe fn encoder_action_to_atom(action: bp_encoder_control_action) -> u8 {
    let mut atom_action: u8 = 0;
    match action {
        ENCODER_CONTROL_ENABLE => atom_action = ATOM_ENABLE,
        ENCODER_CONTROL_DISABLE => atom_action = ATOM_DISABLE,
        ENCODER_CONTROL_SETUP => atom_action = ATOM_ENCODER_CMD_SETUP,
        ENCODER_CONTROL_INIT => atom_action = ATOM_ENCODER_INIT,
        _ => {
            BREAK_TO_DEBUGGER(); /* Unhandle action in driver.!! */
        }
    }
    atom_action
}

static unsafe fn disp_power_gating_action_to_atom(action: bp_pipe_control_action) -> u8 {
    let mut atom_pipe_action: u8 = 0;
    match action {
        ASIC_PIPE_DISABLE => atom_pipe_action = ATOM_DISABLE,
        ASIC_PIPE_ENABLE => atom_pipe_action = ATOM_ENABLE,
        ASIC_PIPE_INIT => atom_pipe_action = ATOM_INIT,
        _ => {
            ASSERT_CRITICAL(false); /* Unhandle action in driver! */
        }
    }
    atom_pipe_action
}

/* function table */
static command_table_helper_funcs: command_table_helper = command_table_helper {
    controller_id_to_atom: Some(dal_cmd_table_helper_controller_id_to_atom),
    encoder_action_to_atom: Some(encoder_action_to_atom),
    engine_bp_to_atom: Some(engine_bp_to_atom),
    clock_source_id_to_atom: Some(clock_source_id_to_atom),
    clock_source_id_to_atom_phy_clk_src_id: Some(clock_source_id_to_atom_phy_clk_src_id),
    signal_type_to_atom_dig_mode: Some(signal_type_to_atom_dig_mode),
    hpd_sel_to_atom: Some(hpd_sel_to_atom),
    dig_encoder_sel_to_atom: Some(dig_encoder_sel_to_atom),
    phy_id_to_atom: Some(phy_id_to_atom),
    disp_power_gating_action_to_atom: Some(disp_power_gating_action_to_atom),
    assign_control_parameter: None,
    clock_source_id_to_ref_clk_src: None,
    transmitter_bp_to_atom: None,
    encoder_id_to_atom: Some(dal_cmd_table_helper_encoder_id_to_atom),
    encoder_mode_bp_to_atom: Some(dal_cmd_table_helper_encoder_mode_bp_to_atom),
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
unsafe fn dal_cmd_tbl_helper_dce110_get_table() -> *const command_table_helper {
    &command_table_helper_funcs
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
