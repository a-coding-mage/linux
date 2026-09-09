/* Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub enum abm_defines {
    abm_defines_max_level = 4,
    abm_defines_max_config = 4,
}

#[repr(C)]
pub struct dmcu_iram_parameters {
    pub backlight_lut_array: *mut ::std::os::raw::c_uint,
    pub backlight_lut_array_size: ::std::os::raw::c_uint,
    pub backlight_ramping_override: bool,
    pub backlight_ramping_reduction: ::std::os::raw::c_uint,
    pub backlight_ramping_start: ::std::os::raw::c_uint,
    pub min_abm_backlight: ::std::os::raw::c_uint,
    pub set: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct backlight_state {
    /* HW uses u16.16 format for backlight PWM */
    pub backlight_pwm: ::std::os::raw::c_uint,
    /* DM may call power module to set backlight targeting percent brightness */
    pub backlight_millipercent: ::std::os::raw::c_uint,
    /* DM may call power module to set backlight based on an explicit nits value. */
    pub backlight_millinit: ::std::os::raw::c_uint,
    pub frame_ramp: ::std::os::raw::c_uint,
    pub smooth_brightness_enabled: bool,
    pub isHDR: bool,
}

#[repr(C)]
pub struct power_entity {
    pub stream: *mut dc_stream_state,
    pub caps: *mut psr_caps,
    pub psr_context: *mut mod_power_psr_context,
    /* PSR cached properties */
    pub psr_enabled: bool,
    pub psr_events: ::std::os::raw::c_uint,
    pub psr_power_opt: ::std::os::raw::c_uint,
    pub replay_events: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct pwr_backlight_properties {
    pub use_nits_based_brightness: bool,
    pub disable_fractional_pwm: bool,
    pub min_abm_backlight: ::std::os::raw::c_uint,
    pub num_backlight_levels: ::std::os::raw::c_uint,
    pub backlight_ramping_override: bool,
    pub backlight_ramping_reduction: ::std::os::raw::c_uint,
    pub backlight_ramping_start: ::std::os::raw::c_uint,
    /* Backlight cached properties */
    pub ac_backlight_percent: ::std::os::raw::c_uint,
    pub dc_backlight_percent: ::std::os::raw::c_uint,
    /* backlight LUT stored in HW u16.16 format */
    pub backlight_lut: *mut ::std::os::raw::c_uint,
    pub min_backlight_pwm: ::std::os::raw::c_uint,
    pub max_backlight_pwm: ::std::os::raw::c_uint,
    pub backlight_range: ::std::os::raw::c_uint,
    /* Describes the panel's min and max luminance in millinits measured on full white screen, in min and max backlight settings. */
    pub min_brightness_millinits: ::std::os::raw::c_uint,
    pub max_brightness_millinits: ::std::os::raw::c_uint,
    pub nits_range: ::std::os::raw::c_uint,
    /* Cached backlight control type used by brightness translation helpers. */
    pub backlight_control_type: backlight_control_type,
    pub backlight_caps_valid: bool,
    pub use_custom_backlight_caps: bool,
    pub custom_backlight_caps_config_no: ::std::os::raw::c_uint,
    pub use_linear_backlight_curve: bool,
}

#[repr(C)]
pub struct dmcu_varibright_cached_properties {
    pub varibright_config_setting: ::std::os::raw::c_uint,
    pub varibright_level: ::std::os::raw::c_uint,
    pub varibright_hw_level: ::std::os::raw::c_uint,
    pub def_varibright_level: ::std::os::raw::c_uint,
    pub varibright_user_enable: bool,
    pub varibright_active: bool,
}

#[repr(C)]
pub struct core_power {
    pub mod_public: mod_power,
    pub dc: *mut dc,
    pub map: *mut power_entity,
    pub varibright_prop: dmcu_varibright_cached_properties,
    pub bl_prop: [pwr_backlight_properties; MAX_NUM_EDP],
    pub bl_state: [backlight_state; MAX_NUM_EDP],
    pub edp_num: ::std::os::raw::c_uint,
    pub psr_smu_optimizations_support: bool,
    pub multi_disp_optimizations_support: bool,
    pub num_entities: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct dmcu_abm_set_bl_params_bits {
    pub gradual_change: ::std::os::raw::c_uint,
    pub reserved: ::std::os::raw::c_uint,
    pub frame_ramp: ::std::os::raw::c_uint,
}

#[repr(C)]
pub union dmcu_abm_set_bl_params {
    pub bits: dmcu_abm_set_bl_params_bits,
    pub u32All: ::std::os::raw::c_uint,
}

extern "C" {
    pub fn dmcu_load_iram(dmcu: *mut dmcu, params: dmcu_iram_parameters) -> bool;
    pub fn dmub_init_abm_config(res_pool: *mut resource_pool, params: dmcu_iram_parameters, inst: ::std::os::raw::c_uint) -> bool;
    pub fn dmub_is_abm_supported(res_pool: *mut resource_pool, inst: ::std::os::raw::c_uint) -> bool;
    pub fn dmub_set_abm_event(res_pool: *mut resource_pool, full_screen: ::std::os::raw::c_uint, trans_info: ::std::os::raw::c_uint, hdr_mode: ::std::os::raw::c_uint, scaling_enable: ::std::os::raw::c_uint, scaling_strength_map: ::std::os::raw::c_uint, inst: ::std::os::raw::c_uint) -> bool;
    pub fn dmub_set_abm_strength(res_pool: *mut resource_pool, strength: ::std::os::raw::c_uint, inst: ::std::os::raw::c_uint) -> bool;
    pub fn init_replay_config(link: *mut dc_link, pr_config: *mut replay_config);
    pub fn set_replay_coasting_vtotal(link: *mut dc_link, type_: replay_coasting_vtotal_type, vtotal: u32);
    pub fn set_replay_defer_update_coasting_vtotal(link: *mut dc_link, type_: replay_coasting_vtotal_type, vtotal: u32);
    pub fn set_replay_frame_skip_number(link: *mut dc_link, type_: replay_coasting_vtotal_type, coasting_vtotal_refresh_rate_Mhz: u32, flicker_free_refresh_rate_Mhz: u32, is_defer: bool);
    pub fn update_replay_coasting_vtotal_from_defer(link: *mut dc_link, type_: replay_coasting_vtotal_type);
    pub fn set_replay_low_rr_full_screen_video_src_vtotal(link: *mut dc_link, vtotal: u16);
    pub fn calculate_replay_link_off_frame_count(link: *mut dc_link, vtotal: u16, htotal: u16);
    pub fn is_psr_su_specific_panel(link: *mut dc_link) -> bool;
    pub fn mod_power_calc_psr_configs(psr_config: *mut psr_config, link: *mut dc_link, stream: *const dc_stream_state);
    pub fn mod_power_only_edp(context: *const dc_state, stream: *const dc_stream_state) -> bool;
    pub fn psr_su_set_dsc_slice_height(dc: *mut dc, link: *mut dc_link, stream: *mut dc_stream_state, config: *mut psr_config) -> bool;
    pub fn fill_custom_backlight_caps(config_no: ::std::os::raw::c_uint, caps: *mut dm_acpi_atif_backlight_caps) -> bool;
    pub fn reset_replay_dsync_error_count(link: *mut dc_link);
    pub fn change_replay_to_psr(link: *mut dc_link);
    pub fn change_psr_to_replay(link: *mut dc_link);
    pub fn initialize_backlight_caps(core_power: *mut core_power, inst: ::std::os::raw::c_uint);
    pub fn mod_power_set_backlight_control_type(core_power: *mut core_power, inst: ::std::os::raw::c_uint, backlight_control_type: backlight_control_type);
    pub fn backlight_millipercent_to_pwm(core_power: *mut core_power, millipercent: ::std::os::raw::c_uint, inst: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    pub fn backlight_millipercent_to_millinit(core_power: *mut core_power, millipercent: ::std::os::raw::c_uint, inst: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    pub fn fill_backlight_level_params(core_power: *mut core_power, backlight_level_params: *mut set_backlight_level_params, panel_inst: i32, aux_inst: u8, backlight_pwm: ::std::os::raw::c_uint, backlight_control_type: backlight_control_type, backlight_millinit: ::std::os::raw::c_uint, transition_time_millisec: ::std::os::raw::c_uint, is_hdr: bool);
    pub fn mod_power_hw_init_backlight(mod_power: *mut mod_power) -> bool;
    pub fn mod_power_update_backlight_on_mode_change(core_power: *mut core_power, link: *mut dc_link, panel_inst: ::std::os::raw::c_uint, aux_inst: u8, is_hdr: bool);
    pub fn map_index_from_stream(core_power: *mut core_power, stream: *const dc_stream_state) -> ::std::os::raw::c_uint;
    pub fn mod_power_psr_notify_mode_change(mod_power: *mut mod_power, stream: *const dc_stream_state, link: *mut dc_link, stream_index: ::std::os::raw::c_uint) -> bool;
    pub fn mod_power_replay_notify_mode_change(mod_power: *mut mod_power, dc: *mut dc, link: *mut dc_link, stream: *const dc_stream_state, stream_index: ::std::os::raw::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
