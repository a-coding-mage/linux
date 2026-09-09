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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// External declarations and definitions are supplied by the surrounding DML code.

const EPSILON: f64 = 0.01;
const DBL_EPSILON: f64 = 2.2204460492503131e-16;
const C_FRL_CB: i32 = 510;
const OVERHEAD_M: f64 = 0.003;
const TOLERANCE_PIXEL_CLOCK: f64 = 0.005;
const TOLERANCE_AUDIO_CLOCK: f64 = 1000.0;
const TOLERANCE_FRL_BIT: f64 = 300.0;
const ACR_RATE_MAX: f64 = 1500.0;
const TB_BORROWED_MAX: f64 = 400.0;

unsafe fn frl_cap_chk_common(inter: *mut frl_cap_chk_intermediates, params: *mut frl_cap_chk_params) -> frl_cap_chk_result {
    let audio_bw_reserve = if (*params).compressed { 192000.0 } else { 0.0 };

    (*inter).c_frl_sb = 4 * C_FRL_CB + (*params).lanes;
    (*inter).overhead_sb = (*params).lanes as f64 / (*inter).c_frl_sb as f64;
    (*inter).overhead_rs = 8.0 * 4.0 / (*inter).c_frl_sb as f64;
    (*inter).overhead_map = 2.5 / (*inter).c_frl_sb as f64;
    (*inter).overhead_min = (*inter).overhead_sb + (*inter).overhead_rs + (*inter).overhead_map;
    (*inter).overhead_max = (*inter).overhead_min + OVERHEAD_M;
    (*inter).f_pixel_clock_max = (*params).f_pixel_clock_nominal * (1.0 + TOLERANCE_PIXEL_CLOCK);
    (*inter).t_line = ((*params).h_active + (*params).h_blank) as f64 / (*inter).f_pixel_clock_max;
    (*inter).r_bit_min = (*params).r_bit_nominal * (1.0 - TOLERANCE_FRL_BIT / 1000000.0);
    (*inter).r_frl_char_min = (*inter).r_bit_min / 18.0;
    (*inter).c_frl_line = dcn_bw_floor((*inter).t_line * (*inter).r_frl_char_min * (*params).lanes as f64);

    match (*params).audio_packet_type {
        0x02 => {
            if (*params).layout == 0 { (*inter).ap = 0.25; }
            else if (*params).layout == 1 { (*inter).ap = 1.0; }
        }
        0x08 => (*inter).ap = 0.25,
        0x09 => (*inter).ap = 1.0,
        0x07 | 0x0e | 0x0f | 0x0b | 0x0c => return FRL_CAP_CHK_ERROR_UNSUPPORTED_AUDIO,
        _ => (*inter).ap = 0.0,
    }

    (*inter).r_ap = (dml_max(audio_bw_reserve, (*params).f_audio * (*inter).ap) + 2.0 * ACR_RATE_MAX)
        * (1.0 + TOLERANCE_AUDIO_CLOCK / 1000000.0);
    (*inter).avg_audio_packets_line = (*inter).r_ap * (*inter).t_line;
    (*inter).audio_packets_line = dcn_bw_ceil((*inter).avg_audio_packets_line) as i32;
    (*inter).blank_audio_min = 32 + 32 * (*inter).audio_packets_line;
    (*params).audio_packets_line = (*inter).audio_packets_line;
    FRL_CAP_CHK_OK
}

unsafe fn frl_cap_chk_uncompressed(params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result {
    let res = frl_cap_chk_common(inter, params); if res != FRL_CAP_CHK_OK { return res; }
    let k_420 = if (*params).pixel_encoding == PIXEL_ENCODING_420 { 2 } else { 1 };
    let k_cd = if (*params).pixel_encoding == PIXEL_ENCODING_422 { 1.0 } else { (*params).bpc as f64 / 8.0 };
    let c_frl_free = dml_max((*params).h_blank as f64 * k_cd / k_420 as f64 - 32.0 * (1 + (*inter).audio_packets_line) as f64 - 7.0, 0.0) as i32;
    let c_frl_rc_savings = dcn_bw_floor(dml_max((7.0 / 8.0) * c_frl_free as f64 - 4.0, 0.0)) as i32;
    let bpp = (24.0 * k_cd / k_420 as f64) as i32;
    let bytes_line = bpp as f64 * (*params).h_active as f64 / 8.0;
    let tb_active = dcn_bw_ceil(bytes_line / 3.0) as i32;
    let tb_blank = dcn_bw_ceil((*params).h_blank as f64 * k_cd / k_420 as f64) as i32;
    if (*inter).blank_audio_min > tb_blank { return FRL_CAP_CHK_ERROR_AUDIO_BW; }
    let f_tb_average = (*inter).f_pixel_clock_max / ((*params).h_active + (*params).h_blank) as f64 * (tb_active + tb_blank) as f64;
    let t_active_ref = (*inter).t_line * (*params).h_active as f64 / ((*params).h_active + (*params).h_blank) as f64;
    let t_blank_ref = (*inter).t_line * (*params).h_blank as f64 / ((*params).h_active + (*params).h_blank) as f64;
    let t_active_min = 3.0 / 2.0 * tb_active as f64 / ((*params).lanes as f64 * (*inter).r_frl_char_min * (1.0 - (*inter).overhead_max));
    let t_blank_min = tb_blank as f64 / ((*params).lanes as f64 * (*inter).r_frl_char_min * (1.0 - (*inter).overhead_max));
    let t_borrowed;
    if t_active_ref >= t_active_min && t_blank_ref >= t_blank_min { t_borrowed = 0.0; (*params).borrow_mode = BORROW_MODE_NONE; }
    else if t_active_ref < t_active_min && t_blank_ref >= t_blank_min { t_borrowed = t_active_min - t_active_ref; (*params).borrow_mode = BORROW_MODE_FROM_BLANK; }
    else { return FRL_CAP_CHK_ERROR_BORROW; }
    if dcn_bw_ceil(t_borrowed * f_tb_average) > TB_BORROWED_MAX { return FRL_CAP_CHK_ERROR_MAX_BORROW; }
    let payload = dcn_bw_ceil(3.0 / 2.0 * tb_active as f64) as f64 + tb_blank as f64 - c_frl_rc_savings as f64;
    let margin = 1.0 - (payload / (*inter).c_frl_line + (*inter).overhead_max);
    if margin < 0.0 && dml_fabs(margin) > EPSILON { return FRL_CAP_CHK_ERROR_MARGIN; }
    FRL_CAP_CHK_OK
}

unsafe fn frl_cap_chk_compressed(params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result {
    let res = frl_cap_chk_common(inter, params); if res != FRL_CAP_CHK_OK { return res; }
    let available = dcn_bw_floor((1.0 - (*inter).overhead_max) * (*inter).c_frl_line) as i32;
    let _active_available = dcn_bw_floor(available as f64 * (*params).h_active as f64 / ((*params).h_active + (*params).h_blank) as f64) as i32;
    let _blank_available = dcn_bw_floor(available as f64 * (*params).h_blank as f64 / ((*params).h_active + (*params).h_blank) as f64) as i32;
    let bytes_target = (*params).slices * dcn_bw_ceil((*params).bpp_target * (*params).slice_width as f64 / 8.0) as i32;
    let hc_active = if !(*params).bypass_hc_target_calc { dcn_bw_ceil(bytes_target as f64 / 3.0) as i32 } else { (*params).hc_active_target };
    let est1 = dcn_bw_ceil(hc_active as f64 * (*params).h_blank as f64 / (*params).h_active as f64) as i32;
    let est2 = dml_max(est1 as f64, (*inter).blank_audio_min as f64);
    let hc_blank = if !(*params).bypass_hc_target_calc {
        let v = 4.0 * dcn_bw_floor(dml_min(est2, available as f64 - 3.0 / 2.0 * hc_active as f64) / 4.0) as i32;
        (*params).hc_active_target = hc_active; (*params).hc_blank_target = v; v
    } else { (*params).hc_blank_target };
    if (*inter).blank_audio_min > hc_blank { return FRL_CAP_CHK_ERROR_AUDIO_BW; }
    let f_tb_average = (*inter).f_pixel_clock_max / ((*params).h_active + (*params).h_blank) as f64 * (hc_active + hc_blank) as f64;
    let t_active_ref = (*inter).t_line * (*params).h_active as f64 / ((*params).h_active + (*params).h_blank) as f64;
    let t_blank_ref = (*inter).t_line - t_active_ref;
    let t_active_target = dml_max(hc_active as f64 / f_tb_average, 3.0 / 2.0 * hc_active as f64 / ((*params).lanes as f64 * (*inter).r_frl_char_min * (1.0 - (*inter).overhead_max)));
    let t_blank_target = (*inter).t_line - t_active_target;
    let tb_borrowed = t_active_target * f_tb_average - hc_active as f64;
    if t_blank_target - t_blank_ref > DBL_EPSILON { (*params).borrow_mode = BORROW_MODE_FROM_ACTIVE; }
    else if t_active_target - t_active_ref > DBL_EPSILON { (*params).borrow_mode = BORROW_MODE_FROM_BLANK; }
    else { (*params).borrow_mode = BORROW_MODE_NONE; }
    if tb_borrowed > TB_BORROWED_MAX { return FRL_CAP_CHK_ERROR_MAX_BORROW; }
    let payload = dcn_bw_ceil(3.0 / 2.0 * hc_active as f64) as f64 + hc_blank as f64;
    let margin = 1.0 - (payload / (*inter).c_frl_line + (*inter).overhead_max);
    if margin < 0.0 && dml_fabs(margin) > EPSILON { return FRL_CAP_CHK_ERROR_MARGIN; }
    FRL_CAP_CHK_OK
}

pub unsafe fn frl_cap_chk(params: *mut frl_cap_chk_params) -> frl_cap_chk_result {
    let mut inter = core::mem::MaybeUninit::<frl_cap_chk_intermediates>::uninit();
    frl_cap_chk_inter(params, inter.as_mut_ptr())
}

pub unsafe fn frl_cap_chk_inter(params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result {
    if (*params).compressed { frl_cap_chk_compressed(params, inter) } else { frl_cap_chk_uncompressed(params, inter) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
