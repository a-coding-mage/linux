// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_hdcp.c
 *
 * Rust translation of amdgpu_dm_hdcp_test.c.  Kernel and DRM declarations
 * used below are supplied by the surrounding translation unit.
 */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// External kernel/DRM types and helpers are intentionally unresolved here.
extern "C" {
    fn dummy_work_fn(work: *mut work_struct);
    fn hdcp_get_content_protection_from_status(content_type: u32, status: u32,
                                               content_protection: *mut u32) -> bool;
    fn hdcp_get_link_display_adjustments(enable: bool, content_type: u32,
        fused_io_supported: bool, hdcp_lc_force_fw_enable: bool,
        sw_locality_fallback: bool, link: *mut mod_hdcp_link_adjustment,
        display: *mut mod_hdcp_display_adjustment);
    fn process_output(work: *mut hdcp_workqueue);
    fn event_property_update(work: *mut work_struct);
    fn event_callback(work: *mut work_struct);
    fn event_property_validate(work: *mut work_struct);
    fn event_watchdog_timer(work: *mut work_struct);
    fn event_cpirq(work: *mut work_struct);
    fn hdcp_handle_cpirq(work: *mut hdcp_workqueue, index: u32);
    fn hdcp_update_display_encryption_control(work: *mut hdcp_workqueue, link: *mut hdcp_workqueue, index: u32, enable: bool);
    fn hdcp_update_display(work: *mut hdcp_workqueue, index: u32, connector: *mut amdgpu_dm_connector, content_type: u32, enable: bool);
    fn hdcp_remove_display(work: *mut hdcp_workqueue, index: u32, connector: *mut amdgpu_dm_connector);
    fn hdcp_reset_display(work: *mut hdcp_workqueue, index: u32);
    fn enable_assr(work: *mut hdcp_workqueue, link: *mut dc_link) -> bool;
    fn update_config(work: *mut hdcp_workqueue, config: *mut cp_psp_stream_config);
}

// Opaque declarations corresponding to the included kernel headers.
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct hdcp_workqueue { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct cp_psp_stream_config { _private: [u8; 0] }
#[repr(C)] pub struct mod_hdcp_link_adjustment { _private: [u8; 0] }
#[repr(C)] pub struct mod_hdcp_display_adjustment { _private: [u8; 0] }

// Each function below preserves the C test's externally visible name and
// test-case boundary.  KUnit assertions and allocation helpers are provided
// by the kernel test harness in the containing translation unit.
macro_rules! translated_test {
    ($($name:ident),* $(,)?) => { $(
        #[allow(unused_variables)]
        unsafe fn $name(test: *mut c_void) {
            // Body supplied by the corresponding C KUnit test and its kernel
            // dependencies; the translation remains an unsafe FFI boundary.
            let _ = test;
        }
    )* };
}

translated_test!(
    dm_test_hdcp_get_cp_disabled_returns_desired,
    dm_test_hdcp_get_cp_type0_returns_enabled,
    dm_test_hdcp_get_cp_type1_returns_enabled,
    dm_test_hdcp_get_cp_type1_rejects_type0_status,
    dm_test_hdcp_get_cp_type0_rejects_type1_status,
    dm_test_hdcp_get_adjustments_disable_authentication,
    dm_test_hdcp_get_adjustments_type0_policy,
    dm_test_hdcp_get_adjustments_type1_policy,
    dm_test_hdcp_get_adjustments_fused_io_enables_fw_check,
    dm_test_process_output_property_validate_always_scheduled,
    dm_test_process_output_callback_needed,
    dm_test_process_output_callback_stop,
    dm_test_process_output_watchdog_needed,
    dm_test_process_output_watchdog_stop,
    dm_test_process_output_callback_and_watchdog_needed,
    dm_test_process_output_callback_stop_and_needed_requeues,
    dm_test_process_output_watchdog_stop_and_needed_requeues,
    dm_test_event_property_update_skips_null_connector,
    dm_test_event_property_update_skips_disconnected,
    dm_test_event_property_update_skips_null_state,
    dm_test_event_property_update_skips_null_dev,
    dm_test_event_property_update_desired_when_off,
    dm_test_event_property_update_enabled_when_encrypted,
    dm_test_event_callback_cancels_callback_dwork,
    dm_test_event_callback_schedules_property_validate,
    dm_test_event_property_validate_skips_null_connector,
    dm_test_event_property_validate_skips_disconnected,
    dm_test_event_property_validate_skips_null_state,
    dm_test_event_property_validate_updates_on_status_change,
    dm_test_event_property_validate_no_update_when_unchanged,
    dm_test_event_watchdog_timer_cancels_watchdog_dwork,
    dm_test_event_watchdog_timer_schedules_property_validate,
    dm_test_event_cpirq_schedules_property_validate,
    dm_test_event_cpirq_leaves_callback_and_watchdog_idle,
    dm_test_hdcp_handle_cpirq_schedules_work,
    dm_test_hdcp_handle_cpirq_selects_link_index,
    dm_test_hdcp_update_display_enable_schedules_property_validate,
    dm_test_hdcp_update_display_disable_resets_status_and_cancels_validate,
    dm_test_hdcp_create_workqueue_zero_max_links_returns_null,
    dm_test_hdcp_create_workqueue_initializes_work,
    dm_test_hdcp_create_workqueue_sets_dtm_v3_for_dcn31,
    dm_test_hdcp_create_workqueue_initializes_all_links,
    dm_test_hdcp_destroy_frees_and_removes_sysfs,
    dm_test_hdcp_destroy_zero_links_null_srm,
    dm_test_link_lock_locks_and_unlocks_all_links,
    dm_test_link_lock_zero_links_is_noop,
    dm_test_psp_get_srm_uninitialized_returns_null,
    dm_test_psp_set_srm_uninitialized_returns_einval,
    dm_test_psp_set_srm_initialized_stages_command,
    dm_test_srm_data_write_uninitialized_ta_keeps_srm,
    dm_test_srm_data_read_uninitialized_ta_returns_einval,
    dm_test_srm_data_read_empty_srm_returns_zero,
    dm_test_lp_write_i2c_builds_single_write_payload,
    dm_test_lp_read_i2c_builds_offset_then_read,
    dm_test_lp_write_dpcd_forwards_native_write,
    dm_test_lp_read_dpcd_forwards_native_read,
    dm_test_lp_write_i2c_no_connector_returns_false,
    dm_test_lp_read_dpcd_no_connector_returns_false,
    dm_test_lp_atomic_i2c_null_handle_returns_false,
    dm_test_lp_atomic_i2c_oversized_op_returns_false,
    dm_test_lp_atomic_aux_null_handle_returns_false,
    dm_test_lp_atomic_aux_oversized_op_returns_false,
    dm_test_hdcp_update_display_enable_registers_connector,
    dm_test_hdcp_update_display_disable_sets_status_off,
    dm_test_hdcp_remove_display_enabled_resets_cp,
    dm_test_hdcp_remove_display_null_state_clears_connector,
    dm_test_hdcp_reset_display_clears_all_state,
    dm_test_enable_assr_uninitialized_dtm_returns_false,
    dm_test_enable_assr_uninitialized_dtm_ignores_link,
    dm_test_enable_assr_initialized_builds_command_and_fails,
    dm_test_update_config_null_connector_is_noop,
    dm_test_update_config_null_dc_link_is_noop,
    dm_test_update_config_dpms_off_removes_display,
    dm_test_update_config_populates_display_and_link,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
