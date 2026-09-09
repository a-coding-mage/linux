/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * adv7604 - Analog Devices ADV7604 video decoder driver
 *
 * Copyright 2012 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

// Dependency supplied by the surrounding kernel bindings: u8, V4L2_CID_DV_CLASS_BASE.

/* Analog input muxing modes (AFE register 0x02, [2:0]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7604_ain_sel {
    ADV7604_AIN1_2_3_NC_SYNC_1_2 = 0,
    ADV7604_AIN4_5_6_NC_SYNC_2_1 = 1,
    ADV7604_AIN7_8_9_NC_SYNC_3_1 = 2,
    ADV7604_AIN10_11_12_NC_SYNC_4_1 = 3,
    ADV7604_AIN9_4_5_6_SYNC_2_1 = 4,
}

/* Bus rotation and reordering. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7604_bus_order {
    ADV7604_BUS_ORDER_RGB,
    ADV7604_BUS_ORDER_GRB,
    ADV7604_BUS_ORDER_RBG,
    ADV7604_BUS_ORDER_BGR,
    ADV7604_BUS_ORDER_BRG,
    ADV7604_BUS_ORDER_GBR,
}

/* Input Color Space (IO register 0x02, [7:4]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv76xx_inp_color_space {
    ADV76XX_INP_COLOR_SPACE_LIM_RGB = 0,
    ADV76XX_INP_COLOR_SPACE_FULL_RGB = 1,
    ADV76XX_INP_COLOR_SPACE_LIM_YCbCr_601 = 2,
    ADV76XX_INP_COLOR_SPACE_LIM_YCbCr_709 = 3,
    ADV76XX_INP_COLOR_SPACE_XVYCC_601 = 4,
    ADV76XX_INP_COLOR_SPACE_XVYCC_709 = 5,
    ADV76XX_INP_COLOR_SPACE_FULL_YCbCr_601 = 6,
    ADV76XX_INP_COLOR_SPACE_FULL_YCbCr_709 = 7,
    ADV76XX_INP_COLOR_SPACE_AUTO = 0xf,
}

/* Select output format (IO register 0x03, [4:2]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv7604_op_format_mode_sel {
    ADV7604_OP_FORMAT_MODE0 = 0x00,
    ADV7604_OP_FORMAT_MODE1 = 0x04,
    ADV7604_OP_FORMAT_MODE2 = 0x08,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv76xx_drive_strength {
    ADV76XX_DR_STR_MEDIUM_LOW = 1,
    ADV76XX_DR_STR_MEDIUM_HIGH = 2,
    ADV76XX_DR_STR_HIGH = 3,
}

/* INT1 Configuration (IO register 0x40, [1:0]) */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv76xx_int1_config {
    ADV76XX_INT1_CONFIG_OPEN_DRAIN,
    ADV76XX_INT1_CONFIG_ACTIVE_LOW,
    ADV76XX_INT1_CONFIG_ACTIVE_HIGH,
    ADV76XX_INT1_CONFIG_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv76xx_page {
    ADV76XX_PAGE_IO,
    ADV7604_PAGE_AVLINK,
    ADV76XX_PAGE_CEC,
    ADV76XX_PAGE_INFOFRAME,
    ADV7604_PAGE_ESDP,
    ADV7604_PAGE_DPP,
    ADV76XX_PAGE_AFE,
    ADV76XX_PAGE_REP,
    ADV76XX_PAGE_EDID,
    ADV76XX_PAGE_HDMI,
    ADV76XX_PAGE_TEST,
    ADV76XX_PAGE_CP,
    ADV7604_PAGE_VDP,
    ADV76XX_PAGE_MAX,
}

/* Platform dependent definition */
#[repr(C)]
pub struct adv76xx_platform_data {
    /* C unsigned bit-fields are represented by their allocation unit. */
    pub disable_pwrdnb: u32,
    pub disable_cable_det_rst: u32,
    pub default_input: i32,
    pub ain_sel: adv7604_ain_sel,
    pub bus_order: adv7604_bus_order,
    pub op_format_mode_sel: adv7604_op_format_mode_sel,
    pub int1_config: adv76xx_int1_config,
    pub alt_gamma: u32,
    pub blank_data: u32,
    pub insert_av_codes: u32,
    pub replicate_av_codes: u32,
    pub inv_vs_pol: u32,
    pub inv_hs_pol: u32,
    pub inv_llc_pol: u32,
    pub dr_str_data: adv76xx_drive_strength,
    pub dr_str_clk: adv76xx_drive_strength,
    pub dr_str_sync: adv76xx_drive_strength,
    pub output_bus_lsb_to_msb: u32,
    pub hdmi_free_run_mode: u32,
    pub i2c_addresses: [u8; ADV76XX_PAGE_MAX as usize],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adv76xx_pad {
    ADV76XX_PAD_HDMI_PORT_A = 0,
    ADV7604_PAD_HDMI_PORT_B = 1,
    ADV7604_PAD_HDMI_PORT_C = 2,
    ADV7604_PAD_HDMI_PORT_D = 3,
    ADV7604_PAD_VGA_RGB = 4,
    ADV7604_PAD_VGA_COMP = 5,
    ADV7604_PAD_SOURCE = 6,
    ADV7611_PAD_SOURCE = 1,
    ADV76XX_PAD_MAX = 7,
}

pub const V4L2_CID_ADV_RX_ANALOG_SAMPLING_PHASE: u32 = V4L2_CID_DV_CLASS_BASE + 0x1000;
pub const V4L2_CID_ADV_RX_FREE_RUN_COLOR_MANUAL: u32 = V4L2_CID_DV_CLASS_BASE + 0x1001;
pub const V4L2_CID_ADV_RX_FREE_RUN_COLOR: u32 = V4L2_CID_DV_CLASS_BASE + 0x1002;

/* notify events */
pub const ADV76XX_HOTPLUG: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
