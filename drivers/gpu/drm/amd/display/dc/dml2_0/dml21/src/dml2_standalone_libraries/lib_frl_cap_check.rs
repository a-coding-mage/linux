// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by lib_float_math.h and lib_frl_cap_check.h are
// intentionally referenced here as external items.

const EPSILON: f64 = 0.01;
const DBL_EPSILON: f64 = 2.2204460492503131e-16;
const C_FRL_CB: i32 = 510;
const OVERHEAD_M: f64 = 0.003;
const TOLERANCE_PIXEL_CLOCK: f64 = 0.005;
const TOLERANCE_AUDIO_CLOCK: f64 = 1000.0;
const TOLERANCE_FRL_BIT: f64 = 300.0;
const ACR_RATE_MAX: i32 = 1500;
pub const DML2_FRL_CHK_TB_BORROWED_MAX: i32 = 400;

// C macros frl_dump_var and frl_print expand to no-ops.
macro_rules! frl_dump_var { ($($arg:tt)*) => {}; }
macro_rules! frl_print { ($($arg:tt)*) => {}; }

unsafe fn frl_cap_check_common(
    inter: *mut lib_frl_cap_check_intermediates,
    params: *mut lib_frl_cap_check_params,
) -> lib_frl_cap_check_status {
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
    (*inter).c_frl_line = math_floor((*inter).t_line * (*inter).r_frl_char_min * (*params).lanes as f64);

    match (*params).audio_packet_type {
        0x02 => {
            if (*params).layout == 0 { (*inter).ap = 0.25; }
            else if (*params).layout == 1 { (*inter).ap = 1.0; }
        }
        0x08 => (*inter).ap = 0.25,
        0x09 => (*inter).ap = 1.0,
        0x07 | 0x0e | 0x0f | 0x0b | 0x0c => return LIB_FRL_CAP_CHECK_ERROR_UNSUPPORTED_AUDIO,
        _ => (*inter).ap = 0.0,
    }
    (*inter).r_ap = (math_max2(audio_bw_reserve, (*params).f_audio * (*inter).ap)
        + 2.0 * ACR_RATE_MAX as f64) * (1.0 + TOLERANCE_AUDIO_CLOCK / 1000000.0);
    (*inter).avg_audio_packets_line = (*inter).r_ap * (*inter).t_line;
    (*inter).audio_packets_line = math_ceil((*inter).avg_audio_packets_line) as i32;
    (*inter).blank_audio_min = 32 + 32 * (*inter).audio_packets_line;
    (*params).audio_packets_line = (*inter).audio_packets_line;
    LIB_FRL_CAP_CHECK_OK
}

unsafe fn frl_cap_check_uncompressed(params: *mut lib_frl_cap_check_params, inter: *mut lib_frl_cap_check_intermediates) -> lib_frl_cap_check_status {
    let res = frl_cap_check_common(inter, params);
    if res != LIB_FRL_CAP_CHECK_OK { return res; }
    let k_420 = if (*params).pixel_encoding == LIB_FRL_CAP_CHECK_PIXEL_ENCODING_420 { 2.0 } else { 1.0 };
    let k_cd = if (*params).pixel_encoding == LIB_FRL_CAP_CHECK_PIXEL_ENCODING_422 { 1.0 } else { (*params).bpc as f64 / 8.0 };
    let c_frl_free = math_max2((*params).h_blank as f64 * k_cd / k_420 - 32.0 * (1.0 + (*inter).audio_packets_line as f64) - 7.0, 0.0) as i32;
    let c_frl_rc_savings = math_floor(math_max2((7.0 / 8.0) * c_frl_free as f64 - 4.0, 0.0)) as i32;
    let bpp = (24.0 * k_cd / k_420) as i32;
    let bytes_line = bpp as f64 * (*params).h_active as f64 / 8.0;
    let tb_active = math_ceil(bytes_line / 3.0) as i32;
    let tb_blank = math_ceil((*params).h_blank as f64 * k_cd / k_420) as i32;
    if (*inter).blank_audio_min > tb_blank { return LIB_FRL_CAP_CHECK_ERROR_AUDIO_BW; }
    let f_tb_average = (*inter).f_pixel_clock_max / ((*params).h_active + (*params).h_blank) as f64 * (tb_active + tb_blank) as f64;
    let t_active_ref = (*inter).t_line * (*params).h_active as f64 / ((*params).h_active + (*params).h_blank) as f64;
    let t_blank_ref = (*inter).t_line * (*params).h_blank as f64 / ((*params).h_active + (*params).h_blank) as f64;
    let t_active_min = 1.5 * tb_active as f64 / ((*params).lanes as f64 * (*inter).r_frl_char_min * (1.0 - (*inter).overhead_max));
    let t_blank_min = tb_blank as f64 / ((*params).lanes as f64 * (*inter).r_frl_char_min * (1.0 - (*inter).overhead_max));
    let t_borrowed;
    if t_active_ref >= t_active_min && t_blank_ref >= t_blank_min { t_borrowed = 0.0; (*params).borrow_mode = LIB_FRL_CAP_CHECK_BORROW_MODE_NONE; }
    else if t_active_ref < t_active_min && t_blank_ref >= t_blank_min { t_borrowed = t_active_min - t_active_ref; (*params).borrow_mode = LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_BLANK; }
    else { return LIB_FRL_CAP_CHECK_ERROR_BORROW; }
    if math_ceil(t_borrowed * f_tb_average) > DML2_FRL_CHK_TB_BORROWED_MAX as f64 { return LIB_FRL_CAP_CHECK_ERROR_MAX_BORROW; }
    let payload = math_ceil(1.5 * tb_active as f64) as f64 + tb_blank as f64 - c_frl_rc_savings as f64;
    let margin = 1.0 - (payload / (*inter).c_frl_line + (*inter).overhead_max);
    if margin < 0.0 && math_fabs(margin) > EPSILON { return LIB_FRL_CAP_CHECK_ERROR_MARGIN; }
    LIB_FRL_CAP_CHECK_OK
}

unsafe fn frl_cap_check_compressed(params: *mut lib_frl_cap_check_params, inter: *mut lib_frl_cap_check_intermediates) -> lib_frl_cap_check_status {
    let res = frl_cap_check_common(inter, params);
    if res != LIB_FRL_CAP_CHECK_OK { return res; }
    let available = math_floor((1.0 - (*inter).overhead_max) * (*inter).c_frl_line) as i32;
    let bytes_target = (*params).slices * math_ceil((*params).bpp_target * (*params).slice_width as f64 / 8.0) as i32;
    let hc_active = if !(*params).bypass_hc_target_calc { math_ceil(bytes_target as f64 / 3.0) as i32 } else { (*params).hc_active_target };
    let est1 = math_ceil(hc_active as f64 * (*params).h_blank as f64 / (*params).h_active as f64) as i32;
    let est2 = math_max2(est1 as f64, (*inter).blank_audio_min as f64) as i32;
    let hc_blank = if !(*params).bypass_hc_target_calc { 4 * math_floor(math_min2(est2 as f64, available as f64 - 1.5 * hc_active as f64) / 4.0) as i32 } else { (*params).hc_blank_target };
    if !(*params).bypass_hc_target_calc { (*params).hc_active_target = hc_active; (*params).hc_blank_target = hc_blank; }
    if (*inter).blank_audio_min > hc_blank { return LIB_FRL_CAP_CHECK_ERROR_AUDIO_BW; }
    let f_tb = (*inter).f_pixel_clock_max / ((*params).h_active + (*params).h_blank) as f64 * (hc_active + hc_blank) as f64;
    let active_ref = (*inter).t_line * (*params).h_active as f64 / ((*params).h_active + (*params).h_blank) as f64;
    let blank_ref = (*inter).t_line - active_ref;
    let active_target = math_max2(hc_active as f64 / f_tb, 1.5 * hc_active as f64 / ((*params).lanes as f64 * (*inter).r_frl_char_min * (1.0 - (*inter).overhead_max)));
    let blank_target = (*inter).t_line - active_target;
    let borrowed = active_target * f_tb - hc_active as f64;
    if blank_target - blank_ref > DBL_EPSILON { (*params).borrow_mode = LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_ACTIVE; }
    else if active_target - active_ref > DBL_EPSILON { (*params).borrow_mode = LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_BLANK; }
    else { (*params).borrow_mode = LIB_FRL_CAP_CHECK_BORROW_MODE_NONE; }
    if borrowed > DML2_FRL_CHK_TB_BORROWED_MAX as f64 { return LIB_FRL_CAP_CHECK_ERROR_MAX_BORROW; }
    let margin = 1.0 - ((math_ceil(1.5 * hc_active as f64) + hc_blank as f64) / (*inter).c_frl_line + (*inter).overhead_max);
    if margin < 0.0 && math_fabs(margin) > EPSILON { return LIB_FRL_CAP_CHECK_ERROR_MARGIN; }
    LIB_FRL_CAP_CHECK_OK
}

pub unsafe fn frl_cap_check(params: *mut lib_frl_cap_check_params) -> lib_frl_cap_check_status {
    let mut inter = core::mem::MaybeUninit::<lib_frl_cap_check_intermediates>::uninit();
    frl_cap_check_intermediates(params, inter.as_mut_ptr())
}

pub unsafe fn frl_cap_check_intermediates(params: *mut lib_frl_cap_check_params, inter: *mut lib_frl_cap_check_intermediates) -> lib_frl_cap_check_status {
    if (*params).compressed { frl_cap_check_compressed(params, inter) } else { frl_cap_check_uncompressed(params, inter) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
