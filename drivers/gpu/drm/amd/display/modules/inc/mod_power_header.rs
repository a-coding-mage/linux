/* Copyright (c) 2019 Advanced Micro Devices, Inc. All rights reserved. */

// #include "dm_services.h"

use core::ffi::c_int;

#[repr(C)]
pub struct core_power {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mod_power_init_params {
    pub disable_fractional_pwm: bool,
    /* Use nits based brightness instead of brightness percentage */
    pub use_nits_based_brightness: bool,
    pub panel_min_millinits: u32,
    pub panel_max_millinits: u32,
    pub min_backlight_pwm: u32,
    pub max_backlight_pwm: u32,
    pub min_abm_backlight: u32,
    pub num_backlight_levels: u32,
    pub backlight_ramping_override: bool,
    pub backlight_ramping_reduction: u32,
    pub backlight_ramping_start: u32,
    pub def_varibright_enable: bool,
    pub def_varibright_level: u32,
    pub varibright_level: u32,
    pub abm_config_setting: u32,
    pub allow_psr_smu_optimizations: bool,
    pub allow_psr_multi_disp_optimizations: bool,
    pub use_custom_backlight_caps: bool,
    pub custom_backlight_caps_config_no: u32,
    pub use_linear_backlight_curve: bool,
}

#[repr(C)]
pub struct mod_power {
    pub dummy: c_int,
}

/* VariBright settings structure */
#[repr(C)]
pub struct varibright_info {
    pub level: u32,
    pub enable: bool,
    pub activate: bool,
}

#[repr(C)]
pub struct mod_power_psr_context {
    /* ddc line */
    pub channel: u32,
    /* Transmitter id */
    pub transmitter_id: u32,
    /* Engine Id is used for Dig Be source select */
    pub engine_id: u32,
    /* Controller Id used for Dig Fe source select */
    pub controller_id: u32,
    /* Pcie or Uniphy */
    pub phy_type: u32,
    /* Physical PHY Id used by SMU interpretation */
    pub smu_phy_id: u32,
    /* Vertical total pixels from crtc timing.
     * This is used for static screen detection.
     * ie. If we want to detect half a frame,
     * we use this to determine the hyst lines.
     */
    pub crtc_timing_vertical_total: u32,
    /* PSR supported from panel capabilities and
     * current display configuration
     */
    pub psr_supported_display_config: bool,
    /* Whether fast link training is supported by the panel */
    pub psr_exit_link_training_required: bool,
    /* If RFB setup time is greater than the total VBLANK time,
     * it is not possible for the sink to capture the video frame
     * in the same frame the SDP is sent. In this case,
     * the frame capture indication bit should be set and an extra
     * static frame should be transmitted to the sink.
     */
    pub psr_frame_capture_indication_req: bool,
    /* Set the last possible line SDP may be transmitted without violating
     * the RFB setup time or entering the active video frame.
     */
    pub sdp_transmit_line_num_deadline: u32,
    /* The VSync rate in Hz used to calculate the
     * step size for smooth brightness feature
     */
    pub vsync_rate_hz: u32,
    pub skip_psr_wait_for_pll_lock: u32,
    pub number_of_controllers: u32,
    /* Unused, for future use. To indicate that first changed frame from
     * state3 shouldn't result in psr_inactive, but rather to perform
     * an automatic single frame rfb_update.
     */
    pub rfb_update_auto_en: bool,
    /* Number of frame before entering static screen */
    pub timehyst_frames: u32,
    /* Partial frames before entering static screen */
    pub hyst_lines: u32,
    /* # of repeated AUX transaction attempts to make before
     * indicating failure to the driver
     */
    pub aux_repeats: u32,
    /* Controls hw blocks to power down during PSR active state */
    pub psr_level: u32,
    /* Controls additional delay after remote frame capture before
     * continuing powerd own
     */
    pub frame_delay: u32,
    pub allow_smu_optimizations: bool,
    pub allow_multi_disp_optimizations: bool,
    pub line_time_in_us: u32,
    /* Panel self refresh 2 selective update granularity required */
    pub su_granularity_required: bool,
    /* psr2 selective update y granularity capability */
    pub su_y_granularity: u8,
    pub rate_control_caps: u8,
    pub os_request_force_ffu: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psr_event {
    psr_event_invalid = 0x0,
    psr_event_vsync = 0x1,
    psr_event_full_screen = 0x2,
    psr_event_defer_enable = 0x4,
    psr_event_hw_programming = 0x8,
    psr_event_test_harness_enable_psr = 0x10,
    psr_event_test_harness_disable_psr = 0x20,
    psr_event_mpo_video_selective_update = 0x40,
    psr_event_edp_panel_off_disable_psr = 0x80,
    psr_event_dynamic_display_switch = 0x100,
    psr_event_big_screen_video = 0x200,
    psr_event_dds_defer_stream_enable = 0x800,
    psr_event_dynamic_link_rate_control = 0x1000,
    psr_event_vrr_transition = 0x2000,
    psr_event_pause = 0x4000,
    psr_event_immediate_flip = 0x8000,
    psr_event_os_request_disable = 0x10000,
    psr_event_os_request_force_ffu = 0x20000,
    psr_event_os_override_hold = 0x40000,
    psr_event_crc_window_active = 0x80000,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum replay_event {
    replay_event_invalid = 0x0,
    replay_event_vsync = 0x1,
    replay_event_full_screen = 0x2,
    replay_event_mpo_video_selective_update = 0x4,
    replay_event_big_screen_video = 0x8,
    replay_event_hw_programming = 0x10,
    replay_event_edp_panel_off_disable_psr = 0x20,
    replay_event_general_ui = 0x40,
    replay_event_vrr = 0x80,
    replay_event_prepare_vtotal = 0x100,
    replay_event_test_harness_enable_replay = 0x200,
    replay_event_test_harness_disable_replay = 0x400,
    replay_event_test_harness_ultra_sleep = 0x800,
    replay_event_immediate_flip = 0x1000,
    replay_event_vrr_transition = 0x2000,
    replay_event_pause = 0x4000,
    replay_event_disable_replay_while_DPMS = 0x8000,
    replay_event_test_harness_mode = 0x10000,
    replay_event_cursor_updating = 0x20000,
    replay_event_sleep_resume = 0x40000,
    replay_event_disable_in_AC = 0x80000,
    replay_event_disable_replay_while_detect_display = 0x100000,
    replay_event_disable_replay_while_switching_mux = 0x400000,
    replay_event_infopacket = 0x800000,
    replay_event_os_request_disable = 0x1000000,
    replay_event_os_request_force_ffu = 0x2000000,
    replay_event_os_override_hold = 0x4000000,
    replay_event_crc_window_active = 0x8000000,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum replay_enable_option {
    pr_enable_option_static_screen = 0x1,
    pr_enable_option_mpo_video = 0x2,
    pr_enable_option_full_screen_video = 0x4,
    pr_enable_option_general_ui = 0x8,
    pr_enable_option_full_screen = 0x10,
    pr_enable_option_static_screen_coasting = 0x10000,
    pr_enable_option_mpo_video_coasting = 0x20000,
    pr_enable_option_full_screen_video_coasting = 0x40000,
    pr_enable_option_full_screen_coasting = 0x100000,
}

#[repr(C)]
pub struct dc { _private: [u8; 0] }
#[repr(C)]
pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)]
pub struct psr_caps { _private: [u8; 0] }
#[repr(C)]
pub struct dc_link { _private: [u8; 0] }
#[repr(C)]
pub struct dc_stream_update { _private: [u8; 0] }
#[repr(C)]
pub enum dc_psr_state { }

extern "C" {
    pub fn mod_power_create(dc: *mut dc, init_params: *mut mod_power_init_params, edp_num: u32) -> *mut mod_power;
    pub fn mod_power_destroy(mod_power: *mut mod_power);
    pub fn mod_power_hw_init(mod_power: *mut mod_power) -> bool;
    pub fn mod_power_add_stream(mod_power: *mut mod_power, stream: *mut dc_stream_state, caps: *mut psr_caps) -> bool;
    pub fn mod_power_remove_stream(mod_power: *mut mod_power, stream: *const dc_stream_state) -> bool;
    pub fn mod_power_replace_stream(mod_power: *mut mod_power, current_stream: *const dc_stream_state, new_stream: *mut dc_stream_state, new_caps: *mut psr_caps) -> bool;
    pub fn mod_power_set_backlight_nits(mod_power: *mut mod_power, streams: *mut dc_stream_state, backlight_millinit: u32, transition_time_millisec: u32, skip_aux: bool, is_hdr: bool) -> bool;
    pub fn mod_power_set_backlight_percent(mod_power: *mut mod_power, stream: *mut dc_stream_state, backlight_millipercent: u32, transition_time_millisec: u32, is_hdr: bool) -> bool;
    pub fn mod_power_update_backlight(mod_power: *mut mod_power, stream: *mut dc_stream_state, backlight_millipercent: u32);
    pub fn mod_power_update_backlight_nits(mod_power: *mut mod_power, stream: *mut dc_stream_state, backlight_millinit: u32);
    pub fn mod_power_get_backlight_pwm(mod_power: *mut mod_power, backlight_pwm: *mut u32, inst: u32) -> bool;
    pub fn mod_power_get_backlight_nits(mod_power: *mut mod_power, backlight_millinit: *mut u32, inst: u32) -> bool;
    pub fn mod_power_get_backlight_percent(mod_power: *mut mod_power, backlight_millipercent: *mut u32, inst: u32) -> bool;
    pub fn mod_power_get_hw_target_backlight_pwm_nits(mod_power: *mut mod_power, link: *const dc_link, backlight_millinit: *mut u32, inst: u32) -> bool;
    pub fn mod_power_get_hw_target_backlight_pwm_percent(mod_power: *mut mod_power, link: *const dc_link, backlight_millipercent: *mut u32, inst: u32) -> bool;
    pub fn mod_power_get_hw_target_backlight_pwm(mod_power: *mut mod_power, link: *const dc_link, backlight_u16_16: *mut u32) -> bool;
    pub fn mod_power_get_hw_backlight_pwm(mod_power: *mut mod_power, link: *const dc_link, backlight: *mut u32) -> bool;
    pub fn mod_power_get_hw_backlight_pwm_nits(mod_power: *mut mod_power, link: *const dc_link, backlight_millinit: *mut u32, inst: u32) -> bool;
    pub fn mod_power_get_hw_backlight_aux_nits(mod_power: *mut mod_power, streams: *mut *mut dc_stream_state, num_streams: c_int, backlight_millinit_avg: *mut u32, backlight_millinit_peak: *mut u32) -> bool;
    pub fn mod_power_get_hw_backlight_pwm_percent(mod_power: *mut mod_power, link: *const dc_link, backlight_millipercent: *mut u32, inst: u32) -> bool;
    pub fn mod_power_initialize_backlight_caps(mod_power: *mut mod_power);
    pub fn mod_power_get_panel_backlight_boundaries(mod_power: *mut mod_power, out_min_backlight: *mut u32, out_max_backlight: *mut u32, out_ac_backlight_percent: *mut u32, out_dc_backlight_percent: *mut u32, inst: u32) -> bool;
    pub fn mod_power_set_smooth_brightness(mod_power: *mut mod_power, enable_brightness: bool, inst: u32) -> bool;
    pub fn mod_power_notify_mode_change(mod_power: *mut mod_power, stream: *const dc_stream_state, is_hdr: bool) -> bool;
    pub fn mod_power_get_varibright_level(mod_power: *mut mod_power, varibright_level: *mut u32) -> bool;
    pub fn mod_power_get_varibright_hw_level(mod_power: *mut mod_power, varibright_level: *mut u32) -> bool;
    pub fn mod_power_get_varibright_default_level(mod_power: *mut mod_power, varibright_level: *mut u32) -> bool;
    pub fn mod_power_get_varibright_enable(mod_power: *mut mod_power, varibright_enable: *mut bool) -> bool;
    pub fn mod_power_varibright_activate(mod_power: *mut mod_power, activate: bool, stream_update: *mut dc_stream_update) -> bool;
    pub fn mod_power_varibright_feature_enable(mod_power: *mut mod_power, enable: bool, stream_update: *mut dc_stream_update) -> bool;
    pub fn mod_power_varibright_set_level(mod_power: *mut mod_power, level: u32, stream_update: *mut dc_stream_update) -> bool;
    pub fn mod_power_varibright_set_hw_level(mod_power: *mut mod_power, level: u32, stream_update: *mut dc_stream_update) -> bool;
    pub fn mod_power_is_abm_active(mod_power: *mut mod_power, link: *const dc_link, inst: u32) -> bool;
    pub fn mod_power_is_abm_supported(mod_power: *mut mod_power, inst: u32) -> bool;
    pub fn mod_power_abm_set_event(mod_power: *mut mod_power, full_screen: u32, trans_info: u32, hdr_mode: u32, scaling_enable: u32, scaling_strength_map: u32, inst: u32) -> bool;
    pub fn mod_power_abm_set_strength(mod_power: *mut mod_power, strength: u32, inst: u32) -> bool;
    pub fn mod_power_set_psr_event(mod_power: *mut mod_power, stream: *mut dc_stream_state, set_event: bool, event: psr_event, wait: bool) -> bool;
    pub fn mod_power_get_psr_event(mod_power: *mut mod_power, stream: *mut dc_stream_state, active_psr_events: *mut u32) -> bool;
    pub fn mod_power_get_psr_state(mod_power: *mut mod_power, stream: *const dc_stream_state, state: *mut dc_psr_state) -> bool;
    pub fn mod_power_get_psr_enabled_status(mod_power: *mut mod_power, stream: *const dc_stream_state, psr_enabled: *mut bool) -> bool;
    pub fn mod_power_set_replay_event(mod_power: *mut mod_power, stream: *mut dc_stream_state, set_event: bool, event: replay_event, wait_for_disable: bool) -> bool;
    pub fn mod_power_get_replay_event(mod_power: *mut mod_power, stream: *mut dc_stream_state, active_replay_events: *mut u32) -> bool;
    pub fn mod_power_get_replay_active_status(stream: *const dc_stream_state, replay_active: *mut bool) -> bool;
    pub fn mod_power_replay_set_coasting_vtotal(mod_power: *mut mod_power, stream: *const dc_stream_state, coasting_vtotal: u32, frame_skip_number: u16) -> bool;
    pub fn mod_power_replay_residency(stream: *const dc_stream_state, residency: *mut u32, is_start: bool, is_alpm: bool);
    pub fn mod_power_replay_set_power_opt_and_coasting_vtotal(mod_power: *mut mod_power, stream: *const dc_stream_state, active_replay_events: u32, coasting_vtotal: u32, is_ultra_sleep_mode: bool, frame_skip_number: u16) -> bool;
    pub fn mod_power_replay_set_timing_sync_supported(mod_power: *mut mod_power, stream: *const dc_stream_state);
    pub fn mod_power_replay_disabled_adaptive_sync_sdp(mod_power: *mut mod_power, stream: *const dc_stream_state, force_disabled: bool);
    pub fn mod_power_replay_disabled_desync_error_detection(mod_power: *mut mod_power, stream: *const dc_stream_state, force_disabled: bool);
    pub fn mod_power_set_low_rr_activate(mod_power: *mut mod_power, stream: *const dc_stream_state, low_rr_supported: bool);
    pub fn mod_power_set_video_conferencing_activate(mod_power: *mut mod_power, stream: *const dc_stream_state, video_conferencing_activate: bool);
    pub fn mod_power_set_live_capture_with_cvt_activate(mod_power: *mut mod_power, stream: *const dc_stream_state, live_capture_with_cvt_activate: bool);
    pub fn mod_power_set_replay_continuously_resync(mod_power: *mut mod_power, stream: *const dc_stream_state, enable: bool);
    pub fn mod_power_set_coasting_vtotal_without_frame_update(mod_power: *mut mod_power, stream: *const dc_stream_state, coasting_vtotal: u32);
    pub fn mod_power_psr_residency(mod_power: *mut mod_power, stream: *const dc_stream_state, residency: *mut u32, mode: u8);
    pub fn mod_power_psr_get_active_psr_events(mod_power: *mut mod_power, stream: *const dc_stream_state, active_psr_events: *mut u32) -> bool;
    pub fn mod_power_psr_set_sink_vtotal_in_psr_active(mod_power: *mut mod_power, stream: *const dc_stream_state, psr_vtotal_idle: u16, psr_vtotal_su: u16) -> bool;
    pub fn mod_power_backlight_percent_to_nits(mod_power: *mut mod_power, stream: *mut dc_stream_state, backlight_millipercent: u32, backlight_millinit: *mut u32) -> bool;
    pub fn mod_power_backlight_nits_to_percent(mod_power: *mut mod_power, stream: *mut dc_stream_state, backlight_millinit: u32, backlight_millipercent: *mut u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
