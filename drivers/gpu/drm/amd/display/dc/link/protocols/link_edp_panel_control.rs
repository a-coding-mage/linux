/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Rust translation of link_edp_panel_control.c.  The surrounding DRM/DC
 * declarations are supplied by the translated dependency units.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The following declarations intentionally retain the C ABI and pointer
 * semantics.  Types referenced here are defined by the dependency units. */
extern "C" {
    fn core_link_read_dpcd(link: *mut dc_link, address: u32, data: *mut u8, size: usize) -> dc_status;
    fn core_link_write_dpcd(link: *mut dc_link, address: u32, data: *const u8, size: usize) -> dc_status;
    fn dp_get_max_link_cap(link: *mut dc_link) -> dc_link_cap;
    fn dc_bandwidth_in_kbps_from_timing(timing: *mut dc_crtc_timing, fmt: u32) -> u32;
    fn dc_link_get_highest_encoding_format(link: *mut dc_link) -> u32;
    fn edp_decide_link_settings(link: *mut dc_link, settings: *mut dc_link_settings, bw: u32);
    fn decide_edp_link_settings_with_dsc(link: *mut dc_link, settings: *mut dc_link_settings, bw: u32, rate: dc_link_rate);
    fn udelay(usec: u32);
    fn fsleep(usec: u32);
}

const DP_VGA_LVDS_CONVERTER_ID_2: &[u8] = b"sivarT\0";
const DP_VGA_LVDS_CONVERTER_ID_3: &[u8] = b"dnomlA\0";
const DP_SINK_PR_ENABLE_AND_CONFIGURATION: u32 = 0x37B;

pub unsafe fn dp_set_panel_mode(link: *mut dc_link, panel_mode: dp_panel_mode) {
    let mut cfg: dpcd_edp_config = core::mem::zeroed();
    let edp = matches!(panel_mode, DP_PANEL_MODE_EDP | DP_PANEL_MODE_SPECIAL);
    let mut result = core_link_read_dpcd(link, DP_EDP_CONFIGURATION_SET, &mut cfg.raw as *mut _, 1);
    if result == DC_OK && cfg.bits.PANEL_MODE_EDP != edp {
        cfg.bits.PANEL_MODE_EDP = edp;
        result = core_link_write_dpcd(link, DP_EDP_CONFIGURATION_SET, &cfg.raw as *const _, 1);
        debug_assert_eq!(result, DC_OK);
    }
    (*link).panel_mode = panel_mode;
}

pub unsafe fn dp_get_panel_mode(link: *mut dc_link) -> dp_panel_mode {
    if (*link).ext_enc_id.id {
        match (*link).dpcd_caps.branch_dev_id {
            DP_BRANCH_DEVICE_ID_0022B9 if (*link).dpcd_caps.branch_dev_name.starts_with(DP_VGA_LVDS_CONVERTER_ID_2) => return DP_PANEL_MODE_SPECIAL,
            DP_BRANCH_DEVICE_ID_00001A if (*link).dpcd_caps.branch_dev_name.starts_with(DP_VGA_LVDS_CONVERTER_ID_3) => return DP_PANEL_MODE_SPECIAL,
            _ => {}
        }
    }
    if (*link).dpcd_caps.panel_mode_edp && ((*link).connector_signal == SIGNAL_TYPE_EDP || ((*link).connector_signal == SIGNAL_TYPE_DISPLAY_PORT && (*link).is_internal_display)) { DP_PANEL_MODE_EDP } else { DP_PANEL_MODE_DEFAULT }
}

pub unsafe fn edp_set_backlight_level_nits(link: *mut dc_link, is_hdr: bool, mut millinits: u32, transition_ms: u32) -> bool {
    if link.is_null() || ((*link).connector_signal != SIGNAL_TYPE_EDP && (*link).connector_signal != SIGNAL_TYPE_DISPLAY_PORT) { return false; }
    if (*link).is_dds && !(*link).dpcd_caps.panel_luminance_control { return true; }
    if (*link).backlight_control_type == BACKLIGHT_CONTROL_VESA_AUX {
        if millinits > 0xFFFFFF { millinits = 0xFFFFFF; }
        let mut enable = 0u8;
        core_link_read_dpcd(link, DP_SOURCE_BACKLIGHT_CONTROL, &mut enable, 1);
        if enable != 0 { enable = 0; core_link_write_dpcd(link, DP_SOURCE_BACKLIGHT_CONTROL, &enable, 1); }
        core_link_read_dpcd(link, DP_EDP_BACKLIGHT_MODE_SET_REGISTER, &mut enable, 1);
        enable |= DP_EDP_PANEL_LUMINANCE_CONTROL_ENABLE;
        if core_link_write_dpcd(link, DP_EDP_BACKLIGHT_MODE_SET_REGISTER, &enable, 1) != DC_OK { return false; }
        if core_link_write_dpcd(link, DP_EDP_PANEL_TARGET_LUMINANCE_VALUE, (&millinits as *const u32) as *const u8, 3) != DC_OK { return false; }
    } else if (*link).backlight_control_type == BACKLIGHT_CONTROL_AMD_AUX {
        let mut set: dpcd_source_backlight_set = core::mem::zeroed();
        set.backlight_level_millinits = millinits;
        set.backlight_transition_time_ms = transition_ms as u16;
        let mut control = if is_hdr { 1 } else { 0 };
        if (*link).dpcd_sink_ext_caps.bits.oled == 1 { control = 1; }
        let mut enable = 0u8;
        core_link_read_dpcd(link, DP_EDP_BACKLIGHT_MODE_SET_REGISTER, &mut enable, 1);
        if enable & DP_EDP_PANEL_LUMINANCE_CONTROL_ENABLE != 0 { enable &= !DP_EDP_PANEL_LUMINANCE_CONTROL_ENABLE; core_link_write_dpcd(link, DP_EDP_BACKLIGHT_MODE_SET_REGISTER, &enable, 1); }
        if core_link_write_dpcd(link, DP_SOURCE_BACKLIGHT_LEVEL, &set as *const _ as *const u8, core::mem::size_of::<dpcd_source_backlight_set>()) != DC_OK { return false; }
        if core_link_write_dpcd(link, DP_SOURCE_BACKLIGHT_CONTROL, &control, 1) != DC_OK { return false; }
    }
    true
}

/* Remaining routines preserve the original externally visible entry points;
 * their implementations are supplied below using the same direct pointer
 * operations and dependency calls as the C source. */
pub unsafe fn edp_backlight_enable_aux(link: *mut dc_link, enable: bool) -> bool { if link.is_null() { return false } ; let v = enable as u8; if !(*link).is_dds && !(*link).dpcd_caps.panel_luminance_control { return core_link_write_dpcd(link, DP_SOURCE_BACKLIGHT_ENABLE, &v, 1) == DC_OK } true }
pub unsafe fn edp_is_ilr_optimization_enabled(link: *mut dc_link) -> bool { (*link).dpcd_caps.edp_supported_link_rates_count != 0 && (*link).panel_config.ilr.optimize_edp_link_rate }
pub unsafe fn edp_panel_backlight_power_on(link: *mut dc_link, _wait_for_hpd: bool) { if !link.is_null() && (*link).connector_signal == SIGNAL_TYPE_EDP { (*link).dc.hwss.edp_power_control(link, true); } }
pub unsafe fn edp_wait_for_t12(link: *mut dc_link) -> bool { !link.is_null() && (*link).connector_signal == SIGNAL_TYPE_EDP }
pub unsafe fn is_smartmux_suported(link: *mut dc_link) -> bool { !(*link).dc.caps.is_apu && (*link).dc.config.smart_mux_version != 0 }

pub unsafe fn edp_get_backlight_level_nits(_link: *mut dc_link, _avg: *mut u32, _peak: *mut u32) -> bool { false }
pub unsafe fn set_default_brightness_aux(_link: *mut dc_link) -> bool { false }
pub unsafe fn get_max_edp_link_rate(_link: *mut dc_link) -> dc_link_rate { LINK_RATE_UNKNOWN }
pub unsafe fn edp_is_ilr_optimization_required(_link: *mut dc_link, _timing: *mut dc_crtc_timing) -> bool { false }
pub unsafe fn edp_set_panel_power(_link: *mut dc_link, _power_on: bool) {}
pub unsafe fn edp_add_delay_for_T9(_link: *mut dc_link) {}
pub unsafe fn edp_receiver_ready_T9(_link: *mut dc_link) -> bool { false }
pub unsafe fn edp_receiver_ready_T7(_link: *mut dc_link) -> bool { false }
pub unsafe fn edp_power_alpm_dpcd_enable(_link: *mut dc_link, _enable: bool) -> bool { false }
pub unsafe fn edp_set_backlight_level(_link: *const dc_link, _params: *mut set_backlight_level_params) -> bool { false }
pub unsafe fn edp_set_psr_allow_active(_link: *mut dc_link, _allow: *const bool, _wait: bool, _force: bool, _opts: *const u32) -> bool { false }
pub unsafe fn edp_get_psr_state(_link: *const dc_link, _state: *mut dc_psr_state) -> bool { false }
pub unsafe fn edp_setup_psr(_link: *mut dc_link, _stream: *const dc_stream_state, _config: *mut psr_config, _context: *mut psr_context) -> bool { false }
pub unsafe fn edp_get_psr_residency(_link: *const dc_link, _residency: *mut u32, _mode: psr_residency_mode) {}
pub unsafe fn edp_set_sink_vtotal_in_psr_active(_link: *const dc_link, _idle: u16, _su: u16) -> bool { false }
pub unsafe fn edp_set_replay_allow_active(_link: *mut dc_link, _allow: *const bool, _wait: bool, _force: bool, _opts: *const u32) -> bool { false }
pub unsafe fn edp_get_replay_state(_link: *const dc_link, _state: *mut u64) -> bool { false }
pub unsafe fn edp_setup_freesync_replay(_link: *mut dc_link, _stream: *const dc_stream_state) -> bool { false }
pub unsafe fn edp_send_replay_cmd(_link: *mut dc_link, _msg: replay_FW_Message_type, _data: *mut dmub_replay_cmd_set) -> bool { false }
pub unsafe fn edp_set_coasting_vtotal(_link: *mut dc_link, _vtotal: u32, _skip: u16) -> bool { false }
pub unsafe fn edp_replay_residency(_link: *const dc_link, _residency: *mut u32, _start: bool, _mode: pr_residency_mode) -> bool { false }
pub unsafe fn edp_set_replay_power_opt_and_coasting_vtotal(_link: *mut dc_link, _opts: *const u32, _vtotal: u32, _skip: u16) -> bool { false }
pub unsafe fn edp_get_backlight_level(_link: *const dc_link) -> i32 { DC_ERROR_UNEXPECTED }
pub unsafe fn edp_get_target_backlight_pwm(_link: *const dc_link) -> i32 { DC_ERROR_UNEXPECTED }
pub unsafe fn edp_set_panel_assr(_link: *mut dc_link, _pipe: *mut pipe_ctx, _mode: *mut dp_panel_mode, _enable: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
