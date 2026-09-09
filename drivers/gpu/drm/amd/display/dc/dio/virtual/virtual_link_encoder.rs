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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

unsafe fn virtual_link_encoder_validate_output_with_stream(
    enc: *mut link_encoder,
    stream: *const dc_stream_state,
) -> bool {
    let _ = enc;
    let _ = stream;
    true
}

unsafe fn virtual_link_encoder_hw_init(enc: *mut link_encoder) {
    let _ = enc;
}

unsafe fn virtual_link_encoder_setup(enc: *mut link_encoder, signal: signal_type) {
    let _ = enc;
    let _ = signal;
}

unsafe fn virtual_link_encoder_enable_tmds_output(
    enc: *mut link_encoder,
    clock_source: clock_source_id,
    color_depth: dc_color_depth,
    signal: signal_type,
    pixel_clock: u32,
) {
    let _ = enc;
    let _ = clock_source;
    let _ = color_depth;
    let _ = signal;
    let _ = pixel_clock;
}

unsafe fn virtual_link_encoder_enable_dp_output(
    enc: *mut link_encoder,
    link_settings: *const dc_link_settings,
    clock_source: clock_source_id,
) {
    let _ = enc;
    let _ = link_settings;
    let _ = clock_source;
}

unsafe fn virtual_link_encoder_enable_dp_mst_output(
    enc: *mut link_encoder,
    link_settings: *const dc_link_settings,
    clock_source: clock_source_id,
) {
    let _ = enc;
    let _ = link_settings;
    let _ = clock_source;
}

unsafe fn virtual_link_encoder_disable_output(
    link_enc: *mut link_encoder,
    signal: signal_type,
) {
    let _ = link_enc;
    let _ = signal;
}

unsafe fn virtual_link_encoder_dp_set_lane_settings(
    enc: *mut link_encoder,
    link_settings: *const dc_link_settings,
    lane_settings: *const dc_lane_settings,
) {
    let _ = enc;
    let _ = link_settings;
    let _ = lane_settings;
}

unsafe fn virtual_link_encoder_dp_set_phy_pattern(
    enc: *mut link_encoder,
    param: *const encoder_set_dp_phy_pattern_param,
) {
    let _ = enc;
    let _ = param;
}

unsafe fn virtual_link_encoder_update_mst_stream_allocation_table(
    enc: *mut link_encoder,
    table: *const link_mst_stream_allocation_table,
) {
    let _ = enc;
    let _ = table;
}

unsafe fn virtual_link_encoder_connect_dig_be_to_fe(
    enc: *mut link_encoder,
    engine: engine_id,
    connect: bool,
) {
    let _ = enc;
    let _ = engine;
    let _ = connect;
}

unsafe fn virtual_link_encoder_destroy(enc: *mut *mut link_encoder) {
    kfree(*enc);
    *enc = core::ptr::null_mut();
}

unsafe fn virtual_link_encoder_get_max_link_cap(
    enc: *mut link_encoder,
    link_settings: *mut dc_link_settings,
) {
    let _ = enc;
    /* Set Default link settings */
    let max_link_cap = dc_link_settings {
        lane_count: LANE_COUNT_FOUR,
        link_rate: LINK_RATE_HIGH,
        link_spread: LINK_SPREAD_05_DOWNSPREAD_30KHZ,
        use_link_rate: false,
        enhanced_framing: 0,
    };
    *link_settings = max_link_cap;
}

static virtual_lnk_enc_funcs: link_encoder_funcs = link_encoder_funcs {
    validate_output_with_stream: Some(virtual_link_encoder_validate_output_with_stream),
    hw_init: Some(virtual_link_encoder_hw_init),
    setup: Some(virtual_link_encoder_setup),
    enable_tmds_output: Some(virtual_link_encoder_enable_tmds_output),
    enable_dp_output: Some(virtual_link_encoder_enable_dp_output),
    enable_dp_mst_output: Some(virtual_link_encoder_enable_dp_mst_output),
    disable_output: Some(virtual_link_encoder_disable_output),
    get_max_link_cap: Some(virtual_link_encoder_get_max_link_cap),
    dp_set_lane_settings: Some(virtual_link_encoder_dp_set_lane_settings),
    dp_set_phy_pattern: Some(virtual_link_encoder_dp_set_phy_pattern),
    update_mst_stream_allocation_table: Some(virtual_link_encoder_update_mst_stream_allocation_table),
    connect_dig_be_to_fe: Some(virtual_link_encoder_connect_dig_be_to_fe),
    destroy: Some(virtual_link_encoder_destroy),
};

unsafe fn virtual_link_encoder_construct(
    enc: *mut link_encoder,
    init_data: *const encoder_init_data,
) -> bool {
    (*enc).funcs = &virtual_lnk_enc_funcs;
    (*enc).ctx = (*init_data).ctx;
    (*enc).id = (*init_data).encoder;

    (*enc).hpd_source = (*init_data).hpd_source;
    (*enc).connector = (*init_data).connector;

    (*enc).transmitter = (*init_data).transmitter;

    (*enc).output_signals = SIGNAL_TYPE_VIRTUAL;

    (*enc).preferred_engine = ENGINE_ID_VIRTUAL;

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
