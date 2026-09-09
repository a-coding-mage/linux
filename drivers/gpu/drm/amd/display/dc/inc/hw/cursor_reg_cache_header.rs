/* SPDX-License-Identifier: MIT */
/* Copyright © 2022 Advanced Micro Devices, Inc. All rights reserved. */

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cursor_control_cfg {
    pub bits: u32,
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_position_cache_hubp {
    pub cur_ctl: reg_cursor_control_cfg,
    pub position: reg_position_cfg,
    pub hot_spot: reg_hot_spot_cfg,
    pub dst_offset: reg_dst_offset_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_position_cfg {
    pub bits: u32, // x_pos:16, y_pos:16
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_hot_spot_cfg {
    pub bits: u32, // x_hot:16, y_hot:16
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_dst_offset_cfg {
    pub bits: u32, // dst_x_offset:13, reserved:19
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_attribute_cache_hubp {
    pub SURFACE_ADDR_HIGH: u32,
    pub SURFACE_ADDR: u32,
    pub cur_ctl: reg_cursor_control_cfg,
    pub size: reg_cursor_size_cfg,
    pub settings: reg_cursor_settings_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cursor_size_cfg {
    pub bits: u32, // width:16, height:16
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cursor_settings_cfg {
    pub bits: u32, // dst_y_offset:8, chunk_hdl_adjust:2, force_cursor_to_disp_pref:1, reserved:21
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_control_cfg {
    pub bits: u32, // cur0_enable:1, expansion_mode:1, reser0:1, cur0_rom_en:1, mode:3, reserved:25
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_position_cache_dpp {
    pub cur0_ctl: reg_cur0_control_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_attribute_cache_dpp {
    pub cur0_ctl: reg_cur0_control_cfg,
    pub fp_scale_bias: reg_cur0_fp_scale_bias,
    pub fp_scale_bias_g_y: reg_cur0_fp_scale_bias_g_y,
    pub fp_scale_bias_rb_crcb: reg_cur0_fp_scale_bias_rb_crcb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_fp_scale_bias {
    pub bits: u32, // fp_bias:16, fp_scale:16
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_fp_scale_bias_g_y {
    pub bits: u32, // fp_bias_g_y:16, fp_scale_g_y:16
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union reg_cur0_fp_scale_bias_rb_crcb {
    pub bits: u32, // fp_bias_rb_crcb:16, fp_scale_rb_crcb:16
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_attributes_cfg {
    pub aHubp: cursor_attribute_cache_hubp,
    pub aDpp: cursor_attribute_cache_dpp,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
