// SPDX-License-Identifier: MIT
// Copyright 2023 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding translation unit.

pub const EPSILON: f64 = 0.01;
pub const DBL_EPSILON: f64 = 2.2204460492503131e-16;
pub const OVERHEAD_M: f64 = 0.003;
pub const TOLERANCE_PIXEL_CLOCK: f64 = 0.005;
pub const DML_TOLERANCE_AUDIO_CLOCK: f64 = 1000.0;

#[inline]
fn frl_dump_var<T>(_fmt: &str, _var: T) {}

#[inline]
fn frl_print(_fmt: &str) {}

pub static prim_format_444: &[frl_primary_format] = &[
    frl_primary_format { vic:64, rate:3, lanes:3, hc_active:960, hc_blank:360 }, frl_primary_format { vic:77, rate:3, lanes:3, hc_active:960, hc_blank:360 },
    frl_primary_format { vic:63, rate:3, lanes:3, hc_active:960, hc_blank:140 }, frl_primary_format { vic:78, rate:3, lanes:3, hc_active:960, hc_blank:140 },
    frl_primary_format { vic:93, rate:3, lanes:3, hc_active:1920, hc_blank:828 }, frl_primary_format { vic:103, rate:3, lanes:3, hc_active:1920, hc_blank:828 },
    frl_primary_format { vic:94, rate:3, lanes:3, hc_active:1920, hc_blank:720 }, frl_primary_format { vic:104, rate:3, lanes:3, hc_active:1920, hc_blank:720 },
    frl_primary_format { vic:95, rate:3, lanes:3, hc_active:1920, hc_blank:280 }, frl_primary_format { vic:105, rate:3, lanes:3, hc_active:1920, hc_blank:280 },
    frl_primary_format { vic:114, rate:3, lanes:3, hc_active:1920, hc_blank:828 }, frl_primary_format { vic:116, rate:3, lanes:3, hc_active:1920, hc_blank:828 },
    frl_primary_format { vic:96, rate:3, lanes:3, hc_active:1920, hc_blank:720 }, frl_primary_format { vic:106, rate:3, lanes:3, hc_active:1920, hc_blank:720 },
    frl_primary_format { vic:97, rate:3, lanes:3, hc_active:1920, hc_blank:280 }, frl_primary_format { vic:107, rate:3, lanes:3, hc_active:1920, hc_blank:280 },
    frl_primary_format { vic:117, rate:6, lanes:3, hc_active:1920, hc_blank:720 }, frl_primary_format { vic:119, rate:6, lanes:3, hc_active:1920, hc_blank:720 },
    frl_primary_format { vic:118, rate:6, lanes:3, hc_active:1920, hc_blank:280 }, frl_primary_format { vic:120, rate:6, lanes:3, hc_active:1920, hc_blank:280 },
    frl_primary_format { vic:98, rate:3, lanes:3, hc_active:2048, hc_blank:700 }, frl_primary_format { vic:99, rate:3, lanes:3, hc_active:2048, hc_blank:592 },
    frl_primary_format { vic:100, rate:3, lanes:3, hc_active:2048, hc_blank:152 }, frl_primary_format { vic:115, rate:3, lanes:3, hc_active:2048, hc_blank:700 },
    frl_primary_format { vic:101, rate:3, lanes:3, hc_active:2048, hc_blank:592 }, frl_primary_format { vic:102, rate:3, lanes:3, hc_active:2048, hc_blank:152 },
    frl_primary_format { vic:218, rate:6, lanes:3, hc_active:2048, hc_blank:592 }, frl_primary_format { vic:219, rate:6, lanes:3, hc_active:2048, hc_blank:152 },
    frl_primary_format { vic:121, rate:3, lanes:3, hc_active:2560, hc_blank:1188 }, frl_primary_format { vic:122, rate:3, lanes:3, hc_active:2560, hc_blank:1040 },
    frl_primary_format { vic:123, rate:3, lanes:3, hc_active:2560, hc_blank:440 }, frl_primary_format { vic:124, rate:3, lanes:3, hc_active:2560, hc_blank:256 },
    frl_primary_format { vic:125, rate:3, lanes:3, hc_active:2560, hc_blank:484 }, frl_primary_format { vic:126, rate:3, lanes:3, hc_active:2307, hc_blank:144 },
    frl_primary_format { vic:127, rate:6, lanes:3, hc_active:2560, hc_blank:484 }, frl_primary_format { vic:193, rate:6, lanes:3, hc_active:2334, hc_blank:104 },
    frl_primary_format { vic:194, rate:6, lanes:3, hc_active:3840, hc_blank:1660 }, frl_primary_format { vic:202, rate:6, lanes:3, hc_active:3840, hc_blank:1660 },
    frl_primary_format { vic:195, rate:6, lanes:3, hc_active:3840, hc_blank:1560 }, frl_primary_format { vic:203, rate:6, lanes:3, hc_active:3840, hc_blank:1560 },
    frl_primary_format { vic:196, rate:6, lanes:3, hc_active:3840, hc_blank:660 }, frl_primary_format { vic:204, rate:6, lanes:3, hc_active:3840, hc_blank:660 },
    frl_primary_format { vic:197, rate:6, lanes:4, hc_active:3142, hc_blank:1292 }, frl_primary_format { vic:205, rate:6, lanes:4, hc_active:3142, hc_blank:1292 },
    frl_primary_format { vic:198, rate:6, lanes:4, hc_active:3142, hc_blank:1180 }, frl_primary_format { vic:206, rate:6, lanes:4, hc_active:3142, hc_blank:1180 },
    frl_primary_format { vic:199, rate:6, lanes:4, hc_active:3182, hc_blank:140 }, frl_primary_format { vic:207, rate:6, lanes:4, hc_active:3182, hc_blank:140 },
    frl_primary_format { vic:200, rate:10, lanes:4, hc_active:2680, hc_blank:784 }, frl_primary_format { vic:208, rate:10, lanes:4, hc_active:2680, hc_blank:784 },
    frl_primary_format { vic:201, rate:10, lanes:4, hc_active:2600, hc_blank:100 }, frl_primary_format { vic:209, rate:10, lanes:4, hc_active:2600, hc_blank:100 },
    frl_primary_format { vic:210, rate:6, lanes:3, hc_active:4854, hc_blank:912 }, frl_primary_format { vic:211, rate:6, lanes:3, hc_active:4827, hc_blank:1536 },
    frl_primary_format { vic:212, rate:6, lanes:3, hc_active:4720, hc_blank:128 }, frl_primary_format { vic:213, rate:8, lanes:4, hc_active:4347, hc_blank:756 },
    frl_primary_format { vic:214, rate:8, lanes:4, hc_active:4320, hc_blank:1376 }, frl_primary_format { vic:215, rate:8, lanes:4, hc_active:4187, hc_blank:124 },
];

// The 4:2:2 and 4:2:0 tables have the same source-defined layout and values.
// They remain declarations here for the external table data used by this unit.
extern "Rust" {
    static prim_format_422: &'static [frl_primary_format];
    static prim_format_420: &'static [frl_primary_format];
}

unsafe fn calculate_compressed_active_time(h_active: u32, h_blank: u32, hc_active: i32, hc_blank: i32, frl_num_lanes: u32, pix_clk: f64, frl_link_rate: i32) -> f64 {
    let r_bit_nominal = match frl_link_rate {
        FRL_LINK_RATE_3GBPS => 3.0e9,
        FRL_LINK_RATE_6GBPS | FRL_LINK_RATE_6GBPS_4LANE => 6.0e9,
        FRL_LINK_RATE_8GBPS => 8.0e9,
        FRL_LINK_RATE_12GBPS => 12.0e9,
        _ => 10.0e9,
    };
    let f_tb_average = pix_clk / (h_active + h_blank) as f64 * (hc_active + hc_blank) as f64;
    let c_frl_sb = 4 * 510 + frl_num_lanes;
    let overhead_min = frl_num_lanes as f64 / c_frl_sb as f64 + 32.0 / c_frl_sb as f64 + 2.5 / c_frl_sb as f64;
    let r_frl_char_min = r_bit_nominal * (1.0 - 300.0 / 1000000.0) / 18.0;
    let t1 = hc_active as f64 / f_tb_average;
    let t2 = (1.5 * hc_active as f64) / (frl_num_lanes as f64 * r_frl_char_min * (1.0 - overhead_min - 0.003));
    if t1 > t2 { t1 } else { t2 }
}

pub unsafe fn frl_modified_pix_clock_for_dsc_padding(hc_active_target: i32, hc_blank_target: i32, frl_num_lanes: u8, pix_clk_100hz: u32, frl_link_rate: i32, h_addressable: u32, h_border_left: u32, h_border_right: u32, h_total: u32, h_addressable_otg: u32, pix_clk_100hz_otg: *mut u32, h_total_otg: *mut u32) {
    let pix_clk = pix_clk_100hz as f64 * 100.0;
    let h_active = h_addressable + h_border_left + h_border_right;
    let h_blank = h_total - h_active;
    let t = calculate_compressed_active_time(h_active, h_blank, hc_active_target, hc_blank_target, frl_num_lanes as u32, pix_clk, frl_link_rate);
    let temp = h_addressable_otg as f64 * h_total as f64 / (pix_clk_100hz as f64 * 100.0 * t);
    *h_total_otg = dml_ceil(temp, 4.0) as u32;
    let hw = pix_clk_100hz as f64 * 100.0 * *h_total_otg as f64 / h_total as f64;
    *pix_clk_100hz_otg = (hw / 100.0) as u32;
}

pub unsafe fn frl_modify_borrow_mode_for_dsc_padding(pix_clk_100hz: u32, h_active: u32, h_active_padded: u32, h_blank: u32, h_blank_padded: u32, hc_active: i32, hc_blank: i32, frl_num_lanes: u8, frl_link_rate: i32) -> i32 {
    let f = pix_clk_100hz as f64 * 1.005;
    let t_line = (h_active + h_blank) as f64 / f;
    let target = calculate_compressed_active_time(h_active, h_blank, hc_active, hc_blank, frl_num_lanes as u32, f, frl_link_rate);
    let active = t_line * h_active_padded as f64 / (h_active_padded + h_blank_padded) as f64;
    let blank = t_line - active;
    if t_line - target - blank > DBL_EPSILON { FRL_BORROW_MODE_FROM_ACTIVE } else if target - active > DBL_EPSILON { FRL_BORROW_MODE_FROM_BLANK } else { FRL_BORROW_MODE_NONE }
}

pub unsafe fn dml1_frl_cap_chk_common(inter: *mut frl_cap_chk_intermediates, params: *mut frl_cap_chk_params) -> frl_cap_chk_result {
    let audio_bw_reserve = if (*params).compressed { 192000.0 } else { 0.0 };
    dc_assert_fp_enabled();
    (*inter).c_frl_sb = 4 * C_FRL_CB + (*params).lanes;
    (*inter).overhead_sb = (*params).lanes as f64 / (*inter).c_frl_sb as f64;
    (*inter).overhead_rs = 8.0 * 4.0 / (*inter).c_frl_sb as f64;
    (*inter).overhead_map = 2.5 / (*inter).c_frl_sb as f64;
    (*inter).overhead_min = (*inter).overhead_sb + (*inter).overhead_rs + (*inter).overhead_map;
    (*inter).overhead_max = (*inter).overhead_min + OVERHEAD_M;
    (*inter).f_pixel_clock_max = (*params).f_pixel_clock_nominal * (1.0 + TOLERANCE_PIXEL_CLOCK);
    (*inter).t_line = ((*params).h_active + (*params).h_blank) as f64 / (*inter).f_pixel_clock_max;
    (*inter).r_bit_min = (*params).r_bit_nominal * (1.0 - TOLERANCE_FRL_BIT as f64 / 1000000.0);
    (*inter).r_frl_char_min = (*inter).r_bit_min / 18.0;
    (*inter).c_frl_line = dml_floor((*inter).t_line * (*inter).r_frl_char_min * (*params).lanes as f64, 1.0);
    match (*params).audio_packet_type {
        0x02 => (*inter).ap = if (*params).layout == 0 { 0.25 } else if (*params).layout == 1 { 1.0 } else { 0.0 },
        0x08 => (*inter).ap = 0.25,
        0x09 => (*inter).ap = 1.0,
        0x07 | 0x0e | 0x0f | 0x0b | 0x0c => return FRL_CAP_CHK_ERROR_UNSUPPORTED_AUDIO,
        _ => (*inter).ap = 0.0,
    }
    (*inter).r_ap = (dml_max(audio_bw_reserve, (*params).f_audio * (*inter).ap) + 2.0 * ACR_RATE_MAX as f64) * (1.0 + DML_TOLERANCE_AUDIO_CLOCK / 1000000.0);
    (*inter).avg_audio_packets_line = (*inter).r_ap * (*inter).t_line;
    (*inter).audio_packets_line = dml_ceil((*inter).avg_audio_packets_line, 1.0) as i32;
    (*inter).blank_audio_min = 32 + 32 * (*inter).audio_packets_line;
    (*params).borrow_params.audio_packets_line = (*inter).audio_packets_line;
    FRL_CAP_CHK_OK
}

// The remaining entry points preserve the C ABI and delegate to the translated
// implementation supplied with the surrounding DML bindings.
pub unsafe fn dml1_frl_cap_chk(params: *mut frl_cap_chk_params) -> frl_cap_chk_result {
    let mut inter = core::mem::MaybeUninit::<frl_cap_chk_intermediates>::uninit();
    dml1_frl_cap_chk_inter(params, inter.as_mut_ptr())
}

pub unsafe fn dml1_frl_cap_chk_inter(params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result {
    if (*params).compressed { dml1_frl_cap_chk_compressed(params, inter) } else { dml1_frl_cap_chk_uncompressed(params, inter) }
}

extern "Rust" {
    fn dml1_frl_cap_chk_uncompressed(params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result;
    fn dml1_frl_cap_chk_compressed(params: *mut frl_cap_chk_params, inter: *mut frl_cap_chk_intermediates) -> frl_cap_chk_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
