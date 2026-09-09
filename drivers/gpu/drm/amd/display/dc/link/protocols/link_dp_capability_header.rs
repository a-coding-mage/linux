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

// Dependency declarations are supplied by the translated link_service header.

extern "C" {
    pub fn detect_dp_sink_caps(link: *mut dc_link) -> bool;

    pub fn detect_edp_sink_caps(link: *mut dc_link);

    pub fn dp_get_max_link_cap(link: *mut dc_link) -> dc_link_settings;

    pub fn dp_get_max_link_enc_cap(
        link: *const dc_link,
        max_link_enc_cap: *mut dc_link_settings,
    ) -> bool;

    pub fn dp_get_verified_link_cap(link: *const dc_link) -> *const dc_link_settings;

    pub fn link_dp_get_encoding_format(
        link_settings: *const dc_link_settings,
    ) -> dp_link_encoding;

    pub fn dp_retrieve_lttpr_cap(link: *mut dc_link) -> dc_status;

    /* Convert PHY repeater count read from DPCD uint8_t. */
    pub fn dp_parse_lttpr_repeater_count(lttpr_repeater_count: u8) -> u8;

    /* Calculate embedded LTTPR address offset for vendor-specific behaviour */
    pub fn dp_get_closest_lttpr_offset(lttpr_count: u8) -> u32;

    pub fn dp_is_sink_present(link: *mut dc_link) -> bool;

    pub fn dp_is_lttpr_present(link: *mut dc_link) -> bool;

    pub fn dp_is_fec_supported(link: *const dc_link) -> bool;

    pub fn is_dp_active_dongle(link: *const dc_link) -> bool;

    pub fn is_dp_branch_device(link: *const dc_link) -> bool;

    pub fn dpcd_write_cable_id_to_dprx(link: *mut dc_link);

    pub fn dp_should_enable_fec(link: *const dc_link) -> bool;

    pub fn dp_is_128b_132b_signal(pipe_ctx: *mut pipe_ctx) -> bool;

    /* Initialize output parameter lt_settings. */
    pub fn dp_decide_training_settings(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_setting: *const dc_link_settings,
        lt_settings: *mut link_training_settings,
    );

    pub fn link_decide_link_settings(
        stream: *mut dc_stream_state,
        link_setting: *mut dc_link_settings,
    ) -> bool;

    pub fn edp_decide_link_settings(
        link: *mut dc_link,
        link_setting: *mut dc_link_settings,
        req_bw: u32,
    ) -> bool;

    pub fn decide_edp_link_settings_with_dsc(
        link: *mut dc_link,
        link_setting: *mut dc_link_settings,
        req_bw: u32,
        max_link_rate: dc_link_rate,
    ) -> bool;

    pub fn mst_decide_link_encoding_format(link: *const dc_link) -> dp_link_encoding;

    pub fn dpcd_set_source_specific_data(link: *mut dc_link);

    /* query dpcd for version and mst cap addresses */
    pub fn read_is_mst_supported(link: *mut dc_link) -> bool;

    pub fn decide_fallback_link_setting(
        link: *mut dc_link,
        max: *mut dc_link_settings,
        cur: *mut dc_link_settings,
        training_result: link_training_result,
    ) -> bool;

    pub fn dp_verify_link_cap_with_retries(
        link: *mut dc_link,
        known_limit_link_setting: *mut dc_link_settings,
        attempts: i32,
    ) -> bool;

    pub fn link_bw_kbps_from_raw_frl_link_rate_data(bw: u8) -> u32;

    pub fn dp_overwrite_extended_receiver_cap(link: *mut dc_link) -> bool;

    pub fn dp_get_lttpr_count(link: *mut dc_link) -> u8;

    pub fn edp_get_alpm_support(
        link: *mut dc_link,
        auxless_support: *mut bool,
        auxwake_support: *mut bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
