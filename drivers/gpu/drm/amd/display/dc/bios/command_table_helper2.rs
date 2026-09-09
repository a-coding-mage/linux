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

// C headers and build-time CONFIG_DRM_AMD_DC_SI condition are supplied by
// the surrounding translation unit.

pub unsafe fn dal_bios_parser_init_cmd_tbl_helper2(
    h: *mut *const command_table_helper,
    dce: dce_version,
) -> bool {
    match dce {
        #[cfg(CONFIG_DRM_AMD_DC_SI)]
        DCE_VERSION_6_0 | DCE_VERSION_6_1 | DCE_VERSION_6_4 => {
            *h = dal_cmd_tbl_helper_dce60_get_table();
            true
        }
        DCE_VERSION_8_0 | DCE_VERSION_8_1 | DCE_VERSION_8_3 => {
            *h = dal_cmd_tbl_helper_dce80_get_table();
            true
        }
        DCE_VERSION_10_0 => {
            *h = dal_cmd_tbl_helper_dce110_get_table();
            true
        }
        DCE_VERSION_11_0 => {
            *h = dal_cmd_tbl_helper_dce110_get_table();
            true
        }
        DCE_VERSION_11_2 | DCE_VERSION_11_22 | DCE_VERSION_12_0 | DCE_VERSION_12_1 => {
            *h = dal_cmd_tbl_helper_dce112_get_table2();
            true
        }
        DCN_VERSION_1_0 | DCN_VERSION_1_01 | DCN_VERSION_2_0 | DCN_VERSION_2_1 |
        DCN_VERSION_2_01 | DCN_VERSION_3_0 | DCN_VERSION_3_01 | DCN_VERSION_3_02 |
        DCN_VERSION_3_03 | DCN_VERSION_3_1 | DCN_VERSION_3_14 | DCN_VERSION_3_15 |
        DCN_VERSION_3_16 | DCN_VERSION_3_2 | DCN_VERSION_3_21 | DCN_VERSION_3_5 |
        DCN_VERSION_3_51 | DCN_VERSION_3_6 | DCN_VERSION_4_01 | DCN_VERSION_4_2 |
        DCN_VERSION_4_2B | DCN_VERSION_6_0 => {
            *h = dal_cmd_tbl_helper_dce112_get_table2();
            true
        }
        _ => {
            *h = dal_cmd_tbl_helper_dce112_get_table2();
            false
        }
    }
}

pub unsafe fn dal_cmd_table_helper_controller_id_to_atom2(
    id: controller_id,
    atom_id: *mut u8,
) -> bool {
    if atom_id.is_null() {
        BREAK_TO_DEBUGGER();
        return false;
    }
    match id {
        CONTROLLER_ID_D0 => { *atom_id = ATOM_CRTC1; true }
        CONTROLLER_ID_D1 => { *atom_id = ATOM_CRTC2; true }
        CONTROLLER_ID_D2 => { *atom_id = ATOM_CRTC3; true }
        CONTROLLER_ID_D3 => { *atom_id = ATOM_CRTC4; true }
        CONTROLLER_ID_D4 => { *atom_id = ATOM_CRTC5; true }
        CONTROLLER_ID_D5 => { *atom_id = ATOM_CRTC6; true }
        // TODO: CONTROLLER_ID_UNDERLAY0 maps to ATOM_UNDERLAY_PIPE0.
        CONTROLLER_ID_UNDEFINED => { *atom_id = ATOM_CRTC_INVALID; true }
        _ => { BREAK_TO_DEBUGGER(); false }
    }
}

/** Translate the Transmitter to the corresponding ATOM BIOS value. */
pub unsafe fn dal_cmd_table_helper_transmitter_bp_to_atom2(t: transmitter) -> u8 {
    match t {
        TRANSMITTER_UNIPHY_A | TRANSMITTER_UNIPHY_B | TRANSMITTER_TRAVIS_LCD => 0,
        TRANSMITTER_UNIPHY_C | TRANSMITTER_UNIPHY_D => 1,
        TRANSMITTER_UNIPHY_E | TRANSMITTER_UNIPHY_F => 2,
        _ => { BREAK_TO_DEBUGGER(); 0 }
    }
}

pub fn dal_cmd_table_helper_encoder_mode_bp_to_atom2(
    s: signal_type,
    enable_dp_audio: bool,
) -> u32 {
    match s {
        SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_DVI_DUAL_LINK => ATOM_ENCODER_MODE_DVI,
        SIGNAL_TYPE_HDMI_TYPE_A => ATOM_ENCODER_MODE_HDMI,
        SIGNAL_TYPE_LVDS => ATOM_ENCODER_MODE_LVDS,
        SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT_MST | SIGNAL_TYPE_DISPLAY_PORT |
        SIGNAL_TYPE_VIRTUAL => if enable_dp_audio { ATOM_ENCODER_MODE_DP_AUDIO } else { ATOM_ENCODER_MODE_DP },
        SIGNAL_TYPE_RGB => ATOM_ENCODER_MODE_CRT,
        _ => ATOM_ENCODER_MODE_CRT,
    }
}

pub unsafe fn dal_cmd_table_helper_clock_source_id_to_ref_clk_src2(
    id: clock_source_id,
    ref_clk_src_id: *mut u32,
) -> bool {
    if ref_clk_src_id.is_null() { BREAK_TO_DEBUGGER(); return false; }
    match id {
        CLOCK_SOURCE_ID_PLL1 => { *ref_clk_src_id = ENCODER_REFCLK_SRC_P1PLL; true }
        CLOCK_SOURCE_ID_PLL2 => { *ref_clk_src_id = ENCODER_REFCLK_SRC_P2PLL; true }
        // TODO: CLOCK_SOURCE_ID_DCPLL maps to ENCODER_REFCLK_SRC_DCPLL.
        CLOCK_SOURCE_ID_EXTERNAL => { *ref_clk_src_id = ENCODER_REFCLK_SRC_EXTCLK; true }
        CLOCK_SOURCE_ID_UNDEFINED => { *ref_clk_src_id = ENCODER_REFCLK_SRC_INVALID; true }
        _ => { BREAK_TO_DEBUGGER(); false }
    }
}

pub unsafe fn dal_cmd_table_helper_encoder_id_to_atom2(id: encoder_id) -> u8 {
    match id {
        ENCODER_ID_INTERNAL_LVDS => ENCODER_OBJECT_ID_INTERNAL_LVDS,
        ENCODER_ID_INTERNAL_TMDS1 => ENCODER_OBJECT_ID_INTERNAL_TMDS1,
        ENCODER_ID_INTERNAL_TMDS2 => ENCODER_OBJECT_ID_INTERNAL_TMDS2,
        ENCODER_ID_INTERNAL_DAC1 => ENCODER_OBJECT_ID_INTERNAL_DAC1,
        ENCODER_ID_INTERNAL_DAC2 => ENCODER_OBJECT_ID_INTERNAL_DAC2,
        ENCODER_ID_INTERNAL_LVTM1 => ENCODER_OBJECT_ID_INTERNAL_LVTM1,
        ENCODER_ID_INTERNAL_HDMI => ENCODER_OBJECT_ID_HDMI_INTERNAL,
        ENCODER_ID_EXTERNAL_TRAVIS => ENCODER_OBJECT_ID_TRAVIS,
        ENCODER_ID_EXTERNAL_NUTMEG => ENCODER_OBJECT_ID_NUTMEG,
        ENCODER_ID_INTERNAL_KLDSCP_TMDS1 => ENCODER_OBJECT_ID_INTERNAL_KLDSCP_TMDS1,
        ENCODER_ID_INTERNAL_KLDSCP_DAC1 => ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DAC1,
        ENCODER_ID_INTERNAL_KLDSCP_DAC2 => ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DAC2,
        ENCODER_ID_EXTERNAL_MVPU_FPGA => ENCODER_OBJECT_ID_MVPU_FPGA,
        ENCODER_ID_INTERNAL_DDI => ENCODER_OBJECT_ID_INTERNAL_DDI,
        ENCODER_ID_INTERNAL_UNIPHY => ENCODER_OBJECT_ID_INTERNAL_UNIPHY,
        ENCODER_ID_INTERNAL_KLDSCP_LVTMA => ENCODER_OBJECT_ID_INTERNAL_KLDSCP_LVTMA,
        ENCODER_ID_INTERNAL_UNIPHY1 => ENCODER_OBJECT_ID_INTERNAL_UNIPHY1,
        ENCODER_ID_INTERNAL_UNIPHY2 => ENCODER_OBJECT_ID_INTERNAL_UNIPHY2,
        ENCODER_ID_INTERNAL_UNIPHY3 => ENCODER_OBJECT_ID_INTERNAL_UNIPHY3,
        ENCODER_ID_INTERNAL_WIRELESS => ENCODER_OBJECT_ID_INTERNAL_VCE,
        ENCODER_ID_INTERNAL_VIRTUAL | ENCODER_ID_UNKNOWN => ENCODER_OBJECT_ID_NONE,
        _ => { BREAK_TO_DEBUGGER(); ENCODER_OBJECT_ID_NONE }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
