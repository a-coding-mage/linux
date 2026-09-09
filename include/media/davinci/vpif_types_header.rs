/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 Texas Instruments Inc
 */

// C dependency: <linux/i2c.h> and related V4L2 types are supplied externally.

pub const VPIF_CAPTURE_MAX_CHANNELS: usize = 2;
pub const VPIF_DISPLAY_MAX_CHANNELS: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vpif_if_type {
    VPIF_IF_BT656,
    VPIF_IF_BT1120,
    VPIF_IF_RAW_BAYER,
}

#[repr(C)]
pub struct vpif_interface {
    pub if_type: vpif_if_type,
    // C unsigned one-bit bit-fields; represented as unsigned storage fields.
    pub hd_pol: u32,
    pub vd_pol: u32,
    pub fid_pol: u32,
}

#[repr(C)]
pub struct vpif_subdev_info {
    pub name: *const core::ffi::c_char,
    pub board_info: i2c_board_info,
}

#[repr(C)]
pub struct vpif_output {
    pub output: v4l2_output,
    pub subdev_name: *const core::ffi::c_char,
    pub input_route: u32,
    pub output_route: u32,
}

#[repr(C)]
pub struct vpif_display_chan_config {
    pub outputs: *const vpif_output,
    pub output_count: core::ffi::c_int,
    pub clip_en: bool,
}

#[repr(C)]
pub struct vpif_display_config {
    pub set_clock: Option<unsafe extern "C" fn(core::ffi::c_int, core::ffi::c_int) -> core::ffi::c_int>,
    pub subdevinfo: *mut vpif_subdev_info,
    pub subdev_count: core::ffi::c_int,
    pub i2c_adapter_id: core::ffi::c_int,
    pub chan_config: [vpif_display_chan_config; VPIF_DISPLAY_MAX_CHANNELS],
    pub card_name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct vpif_input {
    pub input: v4l2_input,
    pub subdev_name: *mut core::ffi::c_char,
    pub input_route: u32,
    pub output_route: u32,
}

#[repr(C)]
pub struct vpif_capture_chan_config {
    pub vpif_if: vpif_interface,
    pub inputs: *mut vpif_input,
    pub input_count: core::ffi::c_int,
}

#[repr(C)]
pub struct vpif_capture_config {
    pub setup_input_channel_mode:
        Option<unsafe extern "C" fn(core::ffi::c_int) -> core::ffi::c_int>,
    pub setup_input_path: Option<
        unsafe extern "C" fn(core::ffi::c_int, *const core::ffi::c_char) -> core::ffi::c_int,
    >,
    pub chan_config: [vpif_capture_chan_config; VPIF_CAPTURE_MAX_CHANNELS],
    pub subdev_info: *mut vpif_subdev_info,
    pub subdev_count: core::ffi::c_int,
    pub i2c_adapter_id: core::ffi::c_int,
    pub card_name: *const core::ffi::c_char,
    pub asd: [*mut v4l2_async_connection; VPIF_CAPTURE_MAX_CHANNELS],
    pub asd_sizes: [core::ffi::c_int; VPIF_CAPTURE_MAX_CHANNELS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
