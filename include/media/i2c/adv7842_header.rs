/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * adv7842 - Analog Devices ADV7842 video decoder driver
 *
 * Copyright 2013 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

/* Analog input muxing modes (AFE register 0x02, [2:0]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_ain_sel {
    ADV7842_AIN1_2_3_NC_SYNC_1_2 = 0,
    ADV7842_AIN4_5_6_NC_SYNC_2_1 = 1,
    ADV7842_AIN7_8_9_NC_SYNC_3_1 = 2,
    ADV7842_AIN10_11_12_NC_SYNC_4_1 = 3,
    ADV7842_AIN9_4_5_6_SYNC_2_1 = 4,
}

/* Bus rotation and reordering. This is used to specify component reordering on
 * the board and describes the components order on the bus when the ADV7842
 * outputs RGB. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_bus_order { ADV7842_BUS_ORDER_RGB, ADV7842_BUS_ORDER_GRB, ADV7842_BUS_ORDER_RBG, ADV7842_BUS_ORDER_BGR, ADV7842_BUS_ORDER_BRG, ADV7842_BUS_ORDER_GBR }

/* Input Color Space (IO register 0x02, [7:4]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_inp_color_space {
    ADV7842_INP_COLOR_SPACE_LIM_RGB = 0, ADV7842_INP_COLOR_SPACE_FULL_RGB = 1,
    ADV7842_INP_COLOR_SPACE_LIM_YCbCr_601 = 2, ADV7842_INP_COLOR_SPACE_LIM_YCbCr_709 = 3,
    ADV7842_INP_COLOR_SPACE_XVYCC_601 = 4, ADV7842_INP_COLOR_SPACE_XVYCC_709 = 5,
    ADV7842_INP_COLOR_SPACE_FULL_YCbCr_601 = 6, ADV7842_INP_COLOR_SPACE_FULL_YCbCr_709 = 7,
    ADV7842_INP_COLOR_SPACE_AUTO = 0xf,
}

/* Select output format (IO register 0x03, [4:2]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_op_format_mode_sel { ADV7842_OP_FORMAT_MODE0 = 0x00, ADV7842_OP_FORMAT_MODE1 = 0x04, ADV7842_OP_FORMAT_MODE2 = 0x08 }

/* Mode of operation */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_mode { ADV7842_MODE_SDP, ADV7842_MODE_COMP, ADV7842_MODE_RGB, ADV7842_MODE_HDMI }

/* Video standard select (IO register 0x00, [5:0]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_vid_std_select {
    ADV7842_SDP_VID_STD_CVBS_SD_4x1 = 0x01, ADV7842_SDP_VID_STD_YC_SD4_x1 = 0x09,
    ADV7842_RGB_VID_STD_AUTO_GRAPH_MODE = 0x07, ADV7842_HDMI_GR_VID_STD_AUTO_GRAPH_MODE = 0x02,
    ADV7842_HDMI_COMP_VID_STD_HD_1250P = 0x1e,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_select_input { ADV7842_SELECT_HDMI_PORT_A, ADV7842_SELECT_HDMI_PORT_B, ADV7842_SELECT_VGA_RGB, ADV7842_SELECT_VGA_COMP, ADV7842_SELECT_SDP_CVBS, ADV7842_SELECT_SDP_YC }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7842_drive_strength { ADV7842_DR_STR_LOW = 0, ADV7842_DR_STR_MEDIUM_LOW = 1, ADV7842_DR_STR_MEDIUM_HIGH = 2, ADV7842_DR_STR_HIGH = 3 }

#[repr(C)]
pub struct adv7842_sdp_csc_coeff { pub manual: bool, pub scaling: u16, pub A1: u16, pub A2: u16, pub A3: u16, pub A4: u16, pub B1: u16, pub B2: u16, pub B3: u16, pub B4: u16, pub C1: u16, pub C2: u16, pub C3: u16, pub C4: u16 }

#[repr(C)]
pub struct adv7842_sdp_io_sync_adjustment { pub adjust: bool, pub hs_beg: u16, pub hs_width: u16, pub de_beg: u16, pub de_end: u16, pub vs_beg_o: u8, pub vs_beg_e: u8, pub vs_end_o: u8, pub vs_end_e: u8, pub de_v_beg_o: u8, pub de_v_beg_e: u8, pub de_v_end_o: u8, pub de_v_end_e: u8 }

/* Platform dependent definition. C bit-fields are represented by their
 * underlying unsigned storage units; callers must apply the indicated widths. */
#[repr(C)]
pub struct adv7842_platform_data {
    pub chip_reset: u32, pub disable_pwrdnb: u32, pub disable_cable_det_rst: u32,
    pub ain_sel: adv7842_ain_sel, pub bus_order: adv7842_bus_order,
    pub op_format_mode_sel: adv7842_op_format_mode_sel, pub mode: adv7842_mode,
    pub input: u32, pub vid_std_select: adv7842_vid_std_select, pub alt_gamma: u32,
    pub blank_data: u32, pub insert_av_codes: u32, pub replicate_av_codes: u32,
    pub output_bus_lsb_to_msb: u32, pub dr_str_data: adv7842_drive_strength,
    pub dr_str_clk: adv7842_drive_strength, pub dr_str_sync: adv7842_drive_strength,
    pub llc_dll_phase: u32, pub sd_ram_size: u32, pub sd_ram_ddr: u32,
    pub hdmi_free_run_enable: u32, pub hdmi_free_run_mode: u32,
    pub sdp_free_run_auto: u32, pub sdp_free_run_man_col_en: u32,
    pub sdp_free_run_cbar_en: u32, pub sdp_free_run_force: u32, pub hpa_auto: u32,
    pub sdp_csc_coeff: adv7842_sdp_csc_coeff,
    pub sdp_io_sync_625: adv7842_sdp_io_sync_adjustment,
    pub sdp_io_sync_525: adv7842_sdp_io_sync_adjustment,
    pub i2c_sdp_io: u8, pub i2c_sdp: u8, pub i2c_cp: u8, pub i2c_vdp: u8,
    pub i2c_afe: u8, pub i2c_hdmi: u8, pub i2c_repeater: u8, pub i2c_edid: u8,
    pub i2c_infoframe: u8, pub i2c_cec: u8, pub i2c_avlink: u8,
}

pub const V4L2_CID_ADV_RX_ANALOG_SAMPLING_PHASE: u32 = V4L2_CID_DV_CLASS_BASE + 0x1000;
pub const V4L2_CID_ADV_RX_FREE_RUN_COLOR_MANUAL: u32 = V4L2_CID_DV_CLASS_BASE + 0x1001;
pub const V4L2_CID_ADV_RX_FREE_RUN_COLOR: u32 = V4L2_CID_DV_CLASS_BASE + 0x1002;
pub const ADV7842_CMD_RAM_TEST: u32 = _IO('V' as u32, BASE_VIDIOC_PRIVATE);
pub const ADV7842_EDID_PORT_A: u32 = 0;
pub const ADV7842_EDID_PORT_B: u32 = 1;
pub const ADV7842_EDID_PORT_VGA: u32 = 2;
pub const ADV7842_PAD_SOURCE: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
