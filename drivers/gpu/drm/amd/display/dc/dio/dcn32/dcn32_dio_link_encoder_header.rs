/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
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
 */

// Dependency supplied by dcn30/dcn30_dio_link_encoder.h.

extern "C" {
    pub fn dcn32_link_encoder_construct(
        enc20: *mut dcn20_link_encoder,
        init_data: *const encoder_init_data,
        enc_features: *const encoder_feature_support,
        link_regs: *const dcn10_link_enc_registers,
        aux_regs: *const dcn10_link_enc_aux_registers,
        hpd_regs: *const dcn10_link_enc_hpd_registers,
        link_shift: *const dcn10_link_enc_shift,
        link_mask: *const dcn10_link_enc_mask,
    );

    pub fn enc32_hw_init(enc: *mut link_encoder);

    pub fn dcn32_link_encoder_enable_dp_output(
        enc: *mut link_encoder,
        link_settings: *const dc_link_settings,
        clock_source: clock_source_id,
    );

    pub fn dcn32_link_encoder_is_in_alt_mode(enc: *mut link_encoder) -> bool;

    pub fn dcn32_link_encoder_get_max_link_cap(
        enc: *mut link_encoder,
        link_settings: *mut dc_link_settings,
    );

    pub fn dpcs32_program_eq_setting(
        enc: *mut link_encoder,
        FFE_Level: u8,
        de_emphasis_only: bool,
        pre_shoot_only: bool,
        no_ffe: bool,
        link_settings: *const dc_hdmi_frl_link_settings,
    );

    pub fn dpcs32_get_txffe(
        enc: *mut link_encoder,
        lane_settings: *mut frl_txffe,
    );

    pub fn dpcs32_set_txffe(
        enc: *mut link_encoder,
        lane_settings: *mut frl_txffe,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
