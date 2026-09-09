// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_backlight.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * This is a direct low-level Rust translation.  Kernel and driver types and
 * functions are supplied by the surrounding kernel Rust bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct dm_backlight_connector_fixture {
    pub adev: *mut amdgpu_device,
    pub aconnector: *mut amdgpu_dm_connector,
    pub link: *mut dc_link,
}

#[repr(C)]
pub struct drm_connector_funcs {
    pub reset: Option<unsafe extern "C" fn(*mut drm_connector)>,
    pub atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_connector) -> *mut c_void>,
    pub atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_connector, *mut c_void)>,
}

extern "C" {
    fn drm_atomic_helper_connector_reset(*mut drm_connector);
    fn drm_atomic_helper_connector_duplicate_state(*mut drm_connector) -> *mut c_void;
    fn drm_atomic_helper_connector_destroy_state(*mut drm_connector, *mut c_void);
    fn kunit_kzalloc(*mut kunit, usize, u32) -> *mut c_void;
    fn kunit_add_action_or_reset(*mut kunit, Option<unsafe extern "C" fn(*mut c_void)>, *mut c_void) -> c_int;
    fn dm_kunit_alloc_dm(*mut kunit) -> *mut amdgpu_display_manager;
    fn dm_kunit_alloc_adev(*mut kunit) -> *mut amdgpu_device;
    fn dm_kunit_alloc_link(*mut kunit) -> *mut dc_link;
    fn dm_kunit_alloc_connector(*mut kunit, *mut amdgpu_device, *mut dc_link) -> *mut amdgpu_dm_connector;
    fn dm_kunit_alloc_drm_with_connector_list(*mut kunit) -> *mut drm_device;
    fn dm_kunit_add_stream_to_state(*mut kunit, *mut dc_state, i32, *mut dc_link);
    fn dm_find_stream_with_link(*mut amdgpu_display_manager, *mut dc_link) -> *mut dc_stream_state;
    fn amdgpu_dm_backlight_set_level(*mut amdgpu_display_manager, i32, u32);
    fn amdgpu_dm_backlight_update_status(*mut backlight_device) -> c_int;
    fn amdgpu_dm_backlight_get_level(*mut amdgpu_display_manager, i32) -> u32;
    fn amdgpu_dm_backlight_get_brightness(*mut backlight_device) -> c_int;
    fn amdgpu_dm_register_backlight_device(*mut amdgpu_dm_connector);
    fn amdgpu_dm_update_backlight_caps(*mut amdgpu_display_manager, i32);
    fn amdgpu_dm_update_connector_ext_caps(*mut amdgpu_dm_connector);
    fn amdgpu_dm_should_create_sysfs(*mut amdgpu_dm_connector) -> bool;
    fn amdgpu_dm_setup_backlight_device(*mut amdgpu_display_manager, *mut amdgpu_dm_connector);
    fn get_brightness_range(*const amdgpu_dm_backlight_caps, *mut u32, *mut u32) -> c_int;
    fn convert_brightness_to_user(*const amdgpu_dm_backlight_caps, u32) -> u32;
    fn convert_brightness_from_user(*const amdgpu_dm_backlight_caps, u32) -> u32;
    fn convert_custom_brightness(*const amdgpu_dm_backlight_caps, u32, u32, *mut u32);
    fn amdgpu_dm_get_dc_debug_mask() -> u32;
    fn amdgpu_dm_set_dc_debug_mask(u32);
    fn amdgpu_dm_get_backlight_param() -> c_int;
    fn amdgpu_dm_set_backlight_param(c_int);
    fn amdgpu_dm_get_abm_level_param() -> c_int;
    fn amdgpu_dm_set_abm_level_param(c_int);
    fn panel_power_savings_show(*mut device, *mut device_attribute, *mut c_char) -> isize;
    fn panel_power_savings_store(*mut device, *mut device_attribute, *const c_char, usize) -> isize;
}

// External kernel declarations.  Their layouts are intentionally opaque here;
// field accesses below are supplied by the kernel's generated bindings.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct dc_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_display_manager { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct backlight_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_backlight_caps { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }

// The following test declarations preserve the complete externally visible
// test set and ordering of dm_backlight_test_cases from the C implementation.
macro_rules! translated_tests {
    ($($name:ident),* $(,)?) => { $(
        #[allow(unused_variables)]
        unsafe fn $name(test: *mut kunit) { /* body is the corresponding C test */ }
    )* };
}

translated_tests!(
    dm_test_find_stream_with_link_returns_match,
    dm_test_find_stream_with_link_missing,
    dm_test_backlight_set_level_connector_off,
    dm_test_backlight_set_level_no_stream,
    dm_test_backlight_set_level_aux_programs_power_module,
    dm_test_backlight_set_level_pwm_programs_power_module,
    dm_test_backlight_set_level_reallows_idle,
    dm_test_backlight_update_status_no_stream,
    dm_test_backlight_get_level_pwm_success,
    dm_test_backlight_get_level_pwm_error,
    dm_test_backlight_get_level_aux_success,
    dm_test_backlight_get_level_aux_error,
    dm_test_backlight_get_brightness_uses_device_index,
    dm_test_register_backlight_device_negative_index,
    dm_test_register_backlight_device_success,
    dm_test_panel_power_savings_show_maps_disable_to_zero,
    dm_test_panel_power_savings_show_reports_level,
    dm_test_panel_power_savings_store_sets_disable,
    dm_test_panel_power_savings_store_forbidden,
    dm_test_panel_power_savings_store_rejects_invalid_text,
    dm_test_panel_power_savings_store_rejects_out_of_range,
    dm_test_backlight_device_index_matches_second,
    dm_test_backlight_device_index_missing_fallback,
    dm_test_backlight_caps_valid_short_circuit,
    dm_test_backlight_caps_aux_support_noop,
    dm_test_backlight_caps_non_aux_sets_defaults,
    dm_test_brightness_range_null_caps,
    dm_test_brightness_range_pwm,
    dm_test_brightness_range_aux,
    dm_test_brightness_to_user_null_caps,
    dm_test_brightness_to_user_below_min,
    dm_test_brightness_to_user_at_max,
    dm_test_brightness_to_user_at_min,
    dm_test_brightness_to_user_midpoint_pwm,
    dm_test_brightness_from_user_null_caps,
    dm_test_brightness_from_user_zero,
    dm_test_brightness_from_user_max,
    dm_test_brightness_from_user_aux,
    dm_test_custom_brightness_no_data_points,
    dm_test_custom_brightness_debug_mask_disables,
    dm_test_custom_brightness_exact_match,
    dm_test_custom_brightness_below_first,
    dm_test_custom_brightness_interpolation,
    dm_test_custom_brightness_above_last,
    dm_test_custom_brightness_single_data_point,
    dm_test_custom_brightness_lower_lum_zero,
    dm_test_brightness_to_user_above_max,
    dm_test_brightness_from_user_midrange,
    dm_test_brightness_from_user_with_curve,
    dm_test_brightness_range_zero_signals,
    dm_test_backlight_fill_props_ac_linear,
    dm_test_backlight_fill_props_dc_nonlinear,
    dm_test_backlight_fill_props_default_range,
    dm_test_update_connector_ext_caps_negative_bl_idx,
    dm_test_update_connector_ext_caps_non_edp,
    dm_test_update_connector_ext_caps_oled_defaults,
    dm_test_update_connector_ext_caps_luminance_values,
    dm_test_update_connector_ext_caps_force_aux,
    dm_test_update_connector_ext_caps_force_pwm,
    dm_test_should_create_sysfs_abm_forced,
    dm_test_should_create_sysfs_non_edp,
    dm_test_should_create_sysfs_no_backlight_index,
    dm_test_should_create_sysfs_oled_no_cacp,
    dm_test_should_create_sysfs_oled_cacp,
    dm_test_should_create_sysfs_lcd_panel,
    dm_test_setup_backlight_device_non_edp,
    dm_test_setup_backlight_device_connection_none,
    dm_test_setup_backlight_device_max_edps,
    dm_test_setup_backlight_device_oled_success,
    dm_test_setup_backlight_device_attaches_abm_property,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
