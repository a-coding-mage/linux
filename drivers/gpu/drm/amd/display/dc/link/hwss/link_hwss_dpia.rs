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
 */

// Dependencies supplied by the surrounding translation unit:
// link_hwss_dpia.h, core_types.h, link_hwss_dio.h, and link_enc_cfg.h

unsafe fn update_dpia_stream_allocation_table(
    link: *mut dc_link,
    link_res: *const link_resource,
    table: *const link_mst_stream_allocation_table,
) {
    let mut link_enc = (*link_res).dio_link_enc;
    static mut status: dc_status = DC_OK;
    let mut mst_alloc_slots: u8 = 0;
    let mut prev_mst_slots_in_use: u8 = 0xff;

    if !(*(*link).dc).config.unify_link_enc_assignment {
        link_enc = link_enc_cfg_get_link_enc(link);
    }

    for i in 0..(*table).stream_count {
        mst_alloc_slots = mst_alloc_slots.wrapping_add((*table).stream_allocations[i as usize].slot_count);
    }

    status = dc_process_dmub_set_mst_slots(
        (*link).dc,
        (*link).link_index,
        mst_alloc_slots,
        &mut prev_mst_slots_in_use,
    );
    ASSERT(status == DC_OK);
    DC_LOG_MST("dpia : status[%d]: alloc_slots[%d]: used_slots[%d]\n", status, mst_alloc_slots, prev_mst_slots_in_use);

    if !link_enc.is_null() {
        ((*(*link_enc).funcs).update_mst_stream_allocation_table)(link_enc, table);
    }
}

unsafe fn set_dio_dpia_link_test_pattern(
    link: *mut dc_link,
    link_res: *const link_resource,
    tp_params: *mut encoder_set_dp_phy_pattern_param,
) {
    if (*tp_params).dp_phy_pattern != DP_TEST_PATTERN_VIDEO_MODE { return; }
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if link_enc.is_null() { return; }
    ((*(*link_enc).funcs).dp_set_phy_pattern)(link_enc, tp_params);
    ((*(*(*link).dc).link_srv).dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN);
}

unsafe fn set_dio_dpia_lane_settings(
    _link: *mut dc_link,
    _link_res: *const link_resource,
    _link_settings: *const dc_link_settings,
    _lane_settings: *const dc_lane_settings,
) {}

unsafe fn enable_dpia_link_output(
    link: *mut dc_link,
    link_res: *const link_resource,
    signal: signal_type,
    clock_source: clock_source_id,
    link_settings: *const dc_link_settings,
) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if !link_enc.is_null() {
        if (*(*link).dc).config.enable_dpia_pre_training || (*(*link).dc).config.unify_link_enc_assignment {
            let fec_rdy = ((*(*link).dc).link_srv).dp_should_enable_fec(link);
            let digmode = if dc_is_dp_sst_signal(signal) { DIG_SST_MODE } else { DIG_MST_MODE };
            if let Some(enable) = (*(*link_enc).funcs).enable_dpia_output {
                enable(link_enc, link_settings, (*link).ddc_hw_inst, digmode, fec_rdy);
            } else { DC_LOG_ERROR("%s: link encoder does not support enable_dpia_output\n", "enable_dpia_link_output"); }
        } else { enable_dio_dp_link_output(link, link_res, signal, clock_source, link_settings); }
    }
    ((*(*(*link).dc).link_srv).dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_ENABLE_LINK_PHY);
}

unsafe fn disable_dpia_link_output(link: *mut dc_link, link_res: *const link_resource, signal: signal_type) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if !link_enc.is_null() {
        if (*(*link).dc).config.enable_dpia_pre_training || (*(*link).dc).config.unify_link_enc_assignment {
            let digmode = if dc_is_dp_sst_signal(signal) { DIG_SST_MODE } else { DIG_MST_MODE };
            if let Some(disable) = (*(*link_enc).funcs).disable_dpia_output { disable(link_enc, (*link).ddc_hw_inst, digmode); }
            else { DC_LOG_ERROR("%s: link encoder does not support disable_dpia_output\n", "disable_dpia_link_output"); }
        } else { ((*(*link_enc).funcs).disable_output)(link_enc, signal); }
    }
    ((*(*(*link).dc).link_srv).dp_trace_source_sequence)(link, DPCD_SOURCE_SEQ_AFTER_DISABLE_LINK_PHY);
}

// Ensure initialization order matches the declaration in link_hwss.h.
static dpia_link_hwss: link_hwss = link_hwss {
    ext: link_hwss_ext {
        set_throttled_vcp_size: set_dio_throttled_vcp_size,
        enable_dp_link_output: enable_dpia_link_output,
        set_dp_link_test_pattern: set_dio_dpia_link_test_pattern,
        set_dp_lane_settings: set_dio_dpia_lane_settings,
        update_stream_allocation_table: update_dpia_stream_allocation_table,
    },
    setup_stream_encoder: setup_dio_stream_encoder,
    reset_stream_encoder: reset_dio_stream_encoder,
    setup_stream_attribute: setup_dio_stream_attribute,
    disable_link_output: disable_dpia_link_output,
    setup_audio_output: setup_dio_audio_output,
    enable_audio_packet: enable_dio_audio_packet,
    disable_audio_packet: disable_dio_audio_packet,
};

unsafe fn can_use_dpia_link_hwss(link: *const dc_link, link_res: *const link_resource) -> bool {
    if !(*(*link).dc).config.unify_link_enc_assignment {
        (*link).is_dig_mapping_flexible && !(*(*link).dc).res_pool.funcs.link_encs_assign.is_none()
    } else {
        (*link).is_dig_mapping_flexible && !(*link_res).dio_link_enc.is_null()
    }
}

unsafe fn get_dpia_link_hwss() -> *const link_hwss { &dpia_link_hwss }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
