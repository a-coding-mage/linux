/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
    pub fn hdmi_frl_find_matching_phypll(link: *mut dc_link) -> clock_source_id;
    pub fn hdmi_frl_LTS_clear_Update_flag(ddc_service: *mut ddc_service);
    pub fn hdmi_frl_poll_start(ddc_service: *mut ddc_service);
    pub fn hdmi_frl_LTS_clear_Link_Setting(ddc_service: *mut ddc_service);
    pub fn hdmi_frl_retrieve_link_cap(link: *mut dc_link, sink: *mut dc_sink);
    pub fn hdmi_frl_perform_link_training_with_retries(link: *mut dc_link) -> link_result;
    pub fn hdmi_frl_perform_link_training_with_fallback(
        link: *mut dc_link,
        link_res: *mut link_resource,
        frl_phy_clock_source_id: clock_source_id,
    ) -> link_result;
    pub fn hdmi_frl_verify_link_cap(
        link: *mut dc_link,
        known_limit_link_setting: *mut dc_hdmi_frl_link_settings,
    );
    pub fn hdmi_frl_decide_link_settings(
        stream: *mut dc_stream_state,
        frl_link_settings: *mut dc_hdmi_frl_link_settings,
        dsc_paddding_params: *mut dsc_padding_params,
    );
    pub fn hdmi_frl_poll_status_flag(link: *mut dc_link) -> bool;
    pub fn hdmi_frl_get_verified_link_cap(
        link: *mut dc_link,
    ) -> *mut dc_hdmi_frl_link_settings;
    pub fn hdmi_frl_set_preferred_link_settings(
        dc: *mut dc,
        link_setting: *mut dc_hdmi_frl_link_settings,
        lt_overrides: *mut dc_hdmi_frl_link_training_overrides,
        link: *mut dc_link,
    );
    pub fn hdmi_frl_write_read_request_enable(ddc_service: *mut ddc_service);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
