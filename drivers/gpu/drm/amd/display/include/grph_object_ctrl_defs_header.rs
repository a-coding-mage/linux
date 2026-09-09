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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

/* Dependency: grph_object_defs.h */

/* These defines are shared between asic_control/bios_parser and other DAL components. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum display_output_bit_depth {
    PANEL_UNDEFINE = 0,
    PANEL_6BIT_COLOR = 1,
    PANEL_8BIT_COLOR = 2,
    PANEL_10BIT_COLOR = 3,
    PANEL_12BIT_COLOR = 4,
    PANEL_16BIT_COLOR = 5,
}

/* Device type as abstracted by ATOM BIOS */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dal_device_type {
    DEVICE_TYPE_UNKNOWN = 0,
    DEVICE_TYPE_LCD,
    DEVICE_TYPE_CRT,
    DEVICE_TYPE_DFP,
    DEVICE_TYPE_CV,
    DEVICE_TYPE_TV,
    DEVICE_TYPE_CF,
    DEVICE_TYPE_WIRELESS,
}

/* Device ID as abstracted by ATOM BIOS */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_id {
    pub device_type: dal_device_type, // C bit-field: 16 bits
    pub enum_id: u32,                 // C bit-field: 16 bits; 1 based enum
    pub raw_device_tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_i2c_info_gpio_info {
    pub clk_mask_register_index: u32,
    pub clk_en_register_index: u32,
    pub clk_y_register_index: u32,
    pub clk_a_register_index: u32,
    pub data_mask_register_index: u32,
    pub data_en_register_index: u32,
    pub data_y_register_index: u32,
    pub data_a_register_index: u32,
    pub clk_mask_shift: u32,
    pub clk_en_shift: u32,
    pub clk_y_shift: u32,
    pub clk_a_shift: u32,
    pub data_mask_shift: u32,
    pub data_en_shift: u32,
    pub data_y_shift: u32,
    pub data_a_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_i2c_info {
    pub gpio_info: graphics_object_i2c_info_gpio_info,
    pub i2c_hw_assist: bool,
    pub i2c_line: u32,
    pub i2c_engine_id: u32,
    pub i2c_slave_address: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_hpd_info {
    pub hpd_int_gpio_uid: u8,
    pub hpd_active: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connector_device_tag_info {
    pub acpi_device: u32,
    pub dev_id: device_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_timing_misc_info {
    pub HORIZONTAL_CUT_OFF: u32, // 1-bit C bit-field
    pub H_SYNC_POLARITY: u32,    // 1-bit C bit-field; 0=Active High, 1=Active Low
    pub V_SYNC_POLARITY: u32,    // 1-bit C bit-field; 0=Active High, 1=Active Low
    pub VERTICAL_CUT_OFF: u32,
    pub H_REPLICATION_BY2: u32,
    pub V_REPLICATION_BY2: u32,
    pub COMPOSITE_SYNC: u32,
    pub INTERLACE: u32,
    pub DOUBLE_CLOCK: u32,
    pub RGB888: u32,
    pub GREY_LEVEL: u32, // 2-bit C bit-field
    pub SPATIAL: u32,
    pub TEMPORAL: u32,
    pub API_ENABLED: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_timing {
    pub misc_info: device_timing_misc_info,
    pub pixel_clk: u32, // in KHz
    pub horizontal_addressable: u32,
    pub horizontal_blanking_time: u32,
    pub vertical_addressable: u32,
    pub vertical_blanking_time: u32,
    pub horizontal_sync_offset: u32,
    pub horizontal_sync_width: u32,
    pub vertical_sync_offset: u32,
    pub vertical_sync_width: u32,
    pub horizontal_border: u32,
    pub vertical_border: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct supported_refresh_rate {
    pub REFRESH_RATE_30HZ: u32,
    pub REFRESH_RATE_40HZ: u32,
    pub REFRESH_RATE_48HZ: u32,
    pub REFRESH_RATE_50HZ: u32,
    pub REFRESH_RATE_60HZ: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct embedded_panel_info {
    pub lcd_timing: device_timing,
    pub ss_id: u32,
    pub supported_rr: supported_refresh_rate,
    pub drr_enabled: u32,
    pub min_drr_refresh_rate: u32,
    pub realtek_eDPToLVDS: bool,
    pub panel_width_mm: u16,
    pub panel_height_mm: u16,
    pub fake_edid_size: u16,
    pub fake_edid: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_firmware_info_pll_info {
    pub crystal_frequency: u32, // in KHz
    pub min_input_pxl_clk_pll_frequency: u32, // in KHz
    pub max_input_pxl_clk_pll_frequency: u32, // in KHz
    pub min_output_pxl_clk_pll_frequency: u32, // in KHz
    pub max_output_pxl_clk_pll_frequency: u32, // in KHz
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_firmware_info_feature {
    pub memory_clk_ss_percentage: u32,
    pub engine_clk_ss_percentage: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_firmware_info {
    pub pll_info: dc_firmware_info_pll_info,
    pub feature: dc_firmware_info_feature,
    pub max_pixel_clock: u32, // in KHz
    pub default_display_engine_pll_frequency: u32, // in KHz
    pub external_clock_source_frequency_for_dp: u32, // in KHz
    pub smu_gpu_pll_output_freq: u32, // in KHz
    pub min_allowed_bl_level: u8,
    pub remote_display_config: u8,
    pub default_memory_clk: u32, // in KHz
    pub default_engine_clk: u32, // in KHz
    pub dp_phy_ref_clk: u32, // in KHz - DCE12 only
    pub i2c_engine_ref_clk: u32, // in KHz - DCE12 only
    pub oem_i2c_present: bool,
    pub oem_i2c_obj_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_vram_info {
    pub num_chans: core::ffi::c_uint,
    pub dram_channel_width_bytes: core::ffi::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct step_and_delay_info {
    pub step: u32,
    pub delay: u32,
    pub recommended_ref_div: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spread_spectrum_info_type {
    pub CENTER_MODE: bool,
    pub EXTERNAL: bool,
    pub STEP_AND_DELAY_INFO: bool,
}

#[repr(C)]
pub union spread_spectrum_info_data {
    pub step_and_delay_info: step_and_delay_info,
    pub target_clock_range: u32,
}

#[repr(C)]
pub struct spread_spectrum_info {
    pub r#type: spread_spectrum_info_type,
    // in unit of 0.01% (spreadPercentageDivider = 100),
    // otherwise in 0.001% units (spreadPercentageDivider = 1000);
    pub spread_spectrum_percentage: u32,
    pub spread_percentage_divider: u32, // 100 or 1000
    pub spread_spectrum_range: u32, // modulation freq (HZ)
    pub data: spread_spectrum_info_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_encoder_cap_info {
    pub dp_hbr2_cap: u32, // 1-bit C bit-field
    pub dp_hbr2_validated: u32, // 1-bit C bit-field
    /* TODO: added MST and HDMI 6G capable flags */
    pub reserved: u32, // 15-bit C bit-field
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct din_connector_info {
    pub gpio_id: u32,
    pub gpio_tv_active_state: bool,
}

/* Invalid channel mapping */
pub const INVALID_DDI_CHANNEL_MAPPING: u32 = 0x0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
