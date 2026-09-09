/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependency supplied by the surrounding translation unit: "link_service.h".

extern "C" {
    pub fn dp_get_panel_mode(link: *mut dc_link) -> dp_panel_mode;
    pub fn dp_set_panel_mode(link: *mut dc_link, panel_mode: dp_panel_mode);
    pub fn set_default_brightness_aux(link: *mut dc_link) -> bool;
    pub fn is_smartmux_suported(link: *mut dc_link) -> bool;
    pub fn edp_panel_backlight_power_on(link: *mut dc_link, wait_for_hpd: bool);
    pub fn edp_get_backlight_level(link: *const dc_link) -> i32;
    pub fn edp_get_backlight_level_nits(
        link: *mut dc_link,
        backlight_millinits_avg: *mut u32,
        backlight_millinits_peak: *mut u32,
    ) -> bool;
    pub fn edp_set_backlight_level(
        link: *const dc_link,
        backlight_level_params: *mut set_backlight_level_params,
    ) -> bool;
    pub fn edp_set_backlight_level_nits(
        link: *mut dc_link,
        isHDR: bool,
        backlight_millinits: u32,
        transition_time_in_ms: u32,
    ) -> bool;
    pub fn edp_get_target_backlight_pwm(link: *const dc_link) -> i32;
    pub fn edp_get_psr_state(link: *const dc_link, state: *mut dc_psr_state) -> bool;
    pub fn edp_set_psr_allow_active(
        link: *mut dc_link,
        allow_active: *const bool,
        wait: bool,
        force_static: bool,
        power_opts: *const u32,
    ) -> bool;
    pub fn edp_setup_psr(
        link: *mut dc_link,
        stream: *const dc_stream_state,
        psr_config: *mut psr_config,
        psr_context: *mut psr_context,
    ) -> bool;
    pub fn edp_set_sink_vtotal_in_psr_active(
        link: *const dc_link,
        psr_vtotal_idle: u16,
        psr_vtotal_su: u16,
    ) -> bool;
    pub fn edp_get_psr_residency(
        link: *const dc_link,
        residency: *mut u32,
        mode: psr_residency_mode,
    );
    pub fn edp_set_replay_allow_active(
        dc_link: *mut dc_link,
        enable: *const bool,
        wait: bool,
        force_static: bool,
        power_opts: *const u32,
    ) -> bool;
    pub fn edp_send_replay_cmd(
        link: *mut dc_link,
        msg: replay_FW_Message_type,
        cmd_data: *mut dmub_replay_cmd_set,
    ) -> bool;
    pub fn edp_set_coasting_vtotal(
        link: *mut dc_link,
        coasting_vtotal: u32,
        frame_skip_number: u16,
    ) -> bool;
    pub fn edp_replay_residency(
        link: *const dc_link,
        residency: *mut u32,
        is_start: bool,
        mode: pr_residency_mode,
    ) -> bool;
    pub fn edp_get_replay_state(link: *const dc_link, state: *mut u64) -> bool;
    pub fn edp_set_replay_power_opt_and_coasting_vtotal(
        link: *mut dc_link,
        power_opts: *const u32,
        coasting_vtotal: u32,
        frame_skip_number: u16,
    ) -> bool;
    pub fn edp_wait_for_t12(link: *mut dc_link) -> bool;
    pub fn edp_is_ilr_optimization_required(
        link: *mut dc_link,
        crtc_timing: *mut dc_crtc_timing,
    ) -> bool;
    pub fn edp_is_ilr_optimization_enabled(link: *mut dc_link) -> bool;
    pub fn get_max_edp_link_rate(link: *mut dc_link) -> dc_link_rate;
    pub fn edp_backlight_enable_aux(link: *mut dc_link, enable: bool) -> bool;
    pub fn edp_add_delay_for_T9(link: *mut dc_link);
    pub fn edp_receiver_ready_T9(link: *mut dc_link) -> bool;
    pub fn edp_receiver_ready_T7(link: *mut dc_link) -> bool;
    pub fn edp_power_alpm_dpcd_enable(link: *mut dc_link, enable: bool) -> bool;
    pub fn edp_setup_freesync_replay(
        link: *mut dc_link,
        stream: *const dc_stream_state,
    ) -> bool;
    pub fn edp_set_panel_power(link: *mut dc_link, powerOn: bool);
    pub fn edp_set_panel_assr(
        link: *mut dc_link,
        pipe_ctx: *mut pipe_ctx,
        panel_mode: *mut dp_panel_mode,
        enable: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
