/*
 * Copyright 2012-17 Advanced Micro Devices, Inc.
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

unsafe fn copy_pps_fields(to: &mut drm_dsc_config, from: &drm_dsc_config) {
    to.line_buf_depth = from.line_buf_depth;
    to.bits_per_component = from.bits_per_component;
    to.convert_rgb = from.convert_rgb;
    to.slice_width = from.slice_width;
    to.slice_height = from.slice_height;
    to.simple_422 = from.simple_422;
    to.native_422 = from.native_422;
    to.native_420 = from.native_420;
    to.pic_width = from.pic_width;
    to.pic_height = from.pic_height;
    to.rc_tgt_offset_high = from.rc_tgt_offset_high;
    to.rc_tgt_offset_low = from.rc_tgt_offset_low;
    to.bits_per_pixel = from.bits_per_pixel;
    to.rc_edge_factor = from.rc_edge_factor;
    to.rc_quant_incr_limit1 = from.rc_quant_incr_limit1;
    to.rc_quant_incr_limit0 = from.rc_quant_incr_limit0;
    to.initial_xmit_delay = from.initial_xmit_delay;
    to.initial_dec_delay = from.initial_dec_delay;
    to.block_pred_enable = from.block_pred_enable;
    to.first_line_bpg_offset = from.first_line_bpg_offset;
    to.second_line_bpg_offset = from.second_line_bpg_offset;
    to.initial_offset = from.initial_offset;
    to.rc_buf_thresh = from.rc_buf_thresh;
    to.rc_range_params = from.rc_range_params;
    to.rc_model_size = from.rc_model_size;
    to.flatness_min_qp = from.flatness_min_qp;
    to.flatness_max_qp = from.flatness_max_qp;
    to.initial_scale_value = from.initial_scale_value;
    to.scale_decrement_interval = from.scale_decrement_interval;
    to.scale_increment_interval = from.scale_increment_interval;
    to.nfl_bpg_offset = from.nfl_bpg_offset;
    to.nsl_bpg_offset = from.nsl_bpg_offset;
    to.slice_bpg_offset = from.slice_bpg_offset;
    to.final_offset = from.final_offset;
    to.vbr_enable = from.vbr_enable;
    to.slice_chunk_size = from.slice_chunk_size;
    to.second_line_offset_adj = from.second_line_offset_adj;
    to.dsc_version_minor = from.dsc_version_minor;
}

unsafe fn copy_rc_to_cfg(dsc_cfg: &mut drm_dsc_config, rc: &rc_params) {
    dsc_cfg.rc_quant_incr_limit0 = rc.rc_quant_incr_limit0 as u8;
    dsc_cfg.rc_quant_incr_limit1 = rc.rc_quant_incr_limit1 as u8;
    dsc_cfg.initial_offset = rc.initial_fullness_offset as u16;
    dsc_cfg.initial_xmit_delay = rc.initial_xmit_delay as u16;
    dsc_cfg.first_line_bpg_offset = rc.first_line_bpg_offset as u8;
    dsc_cfg.second_line_bpg_offset = rc.second_line_bpg_offset as u8;
    dsc_cfg.flatness_min_qp = rc.flatness_min_qp as u8;
    dsc_cfg.flatness_max_qp = rc.flatness_max_qp as u8;
    let mut i = 0;
    while i < QP_SET_SIZE {
        dsc_cfg.rc_range_params[i].range_min_qp = rc.qp_min[i] as u8;
        dsc_cfg.rc_range_params[i].range_max_qp = rc.qp_max[i] as u8;
        /* Truncate 8-bit signed value to 6-bit signed value */
        dsc_cfg.rc_range_params[i].range_bpg_offset = 0x3f & rc.ofs[i];
        i += 1;
    }
    dsc_cfg.rc_model_size = rc.rc_model_size as u16;
    dsc_cfg.rc_edge_factor = rc.rc_edge_factor as u8;
    dsc_cfg.rc_tgt_offset_high = rc.rc_tgt_offset_hi as u8;
    dsc_cfg.rc_tgt_offset_low = rc.rc_tgt_offset_lo as u8;

    i = 0;
    while i < QP_SET_SIZE - 1 {
        dsc_cfg.rc_buf_thresh[i] = rc.rc_buf_thresh[i] as u16;
        i += 1;
    }
}

pub unsafe fn dscc_compute_dsc_parameters(
    pps: &drm_dsc_config,
    rc: &rc_params,
    dsc_params: &mut dsc_parameters,
) -> i32 {
    let mut dsc_cfg: drm_dsc_config = core::mem::zeroed();

    dsc_params.pps = *pps;
    dsc_params.pps.initial_scale_value =
        (8 * rc.rc_model_size / (rc.rc_model_size - rc.initial_fullness_offset)) as u8;

    copy_pps_fields(&mut dsc_cfg, &dsc_params.pps);
    copy_rc_to_cfg(&mut dsc_cfg, rc);

    dsc_cfg.mux_word_size = if dsc_params.pps.bits_per_component <= 10 { 48 } else { 64 };

    let ret = drm_dsc_compute_rc_parameters(&mut dsc_cfg);
    dsc_params.bytes_per_pixel =
        div_u64(dsc_cfg.slice_chunk_size as u64 * 0x10000000 + (dsc_cfg.slice_width - 1) as u64,
                dsc_cfg.slice_width as u32) as u32;

    copy_pps_fields(&mut dsc_params.pps, &dsc_cfg);
    dsc_params.rc_buffer_model_size = dsc_cfg.rc_bits;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
