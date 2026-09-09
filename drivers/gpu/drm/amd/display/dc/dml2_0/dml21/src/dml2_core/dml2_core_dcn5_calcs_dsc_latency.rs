// SPDX-License-Identifier: MIT
//
// Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.

// Shared DSC sub-functions (static) — identical for legacy and updated paths.

unsafe fn dscc_pcl_compute_delay(pixel_format: dml2_output_format_class, num_slices: i32) -> i32 {
    let mut dispclk_per_dscclk: i32;
    if num_slices == 1 { dispclk_per_dscclk = 3; } else { dispclk_per_dscclk = 6; }
    if pixel_format == dml2_420 || pixel_format == dml2_n422 { dispclk_per_dscclk *= 2; }
    let mut delay = 7;
    delay *= dispclk_per_dscclk;
    delay
}

unsafe fn dscc_bcl_compute_delay(p: *mut latency_t, bpc: i32, bpp: f32, slice_width: i32,
    num_slices: i32, pixel_format: dml2_output_format_class, initial_xmit_delay_offset: i32,
    group_delay_after_initial_xmit_delay_override_en: i32, group_delay_after_initial_xmit_delay: i32) {
    let rc_model_size = 8192;
    let pixels_per_clock: i32 = if pixel_format == dml2_n422 || pixel_format == dml2_420 { 2 } else { 1 };
    let mut initial_xmit_delay = math_round((rc_model_size as f64 / 2.0 / bpp as f64 / pixels_per_clock as f64)) as i32;
    let slice_width_modified = if pixel_format == dml2_n422 || pixel_format == dml2_420 { slice_width / 2 } else { slice_width };
    let padding_pixels = if slice_width_modified % 3 != 0 { (3 - slice_width_modified % 3) * (initial_xmit_delay / slice_width_modified) } else { 0 };
    if 3.0 * pixels_per_clock as f32 * bpp >= ((initial_xmit_delay + 2) / 3 * (3 + (pixel_format == dml2_n422) as i32)) as f32 {
        if (initial_xmit_delay + padding_pixels) % 3 == 1 { initial_xmit_delay += 1; }
    }
    initial_xmit_delay += initial_xmit_delay_offset;
    let ssm_group_priming_delay = match bpc {
        8 => 83, 10 => 91, 12 => 115, 14 => 123, 16 => 128,
        _ => { DML_LOG_VERBOSE!("ERROR: BPC is not a valid value. bpc = %d", bpc); 83 }
    };
    let slice_width_groups = (slice_width_modified + 2) / 3;
    let slice_padded_pixels = 3 * slice_width_groups - slice_width_modified;
    let lines_to_reach_ixd = initial_xmit_delay / slice_width_modified;
    let ixd_plus_padding = initial_xmit_delay + slice_padded_pixels * lines_to_reach_ixd;
    let ixd_plus_padding_groups = (ixd_plus_padding + 2) / 3;
    let groups_to_reach_ixd_adjusted = ixd_plus_padding_groups + ssm_group_priming_delay;
    let lines_to_reach_ixd_adjusted = (groups_to_reach_ixd_adjusted + slice_width_groups - 1) / slice_width_groups;
    let additional_group_delay = if (initial_xmit_delay - lines_to_reach_ixd * slice_width_modified) % 3 == 0 { 1 } else { 0 };
    let ssm_pipeline_delay = 4;
    let obsm_pipeline_delay = 1;
    let cycles_per_group = if pixel_format == dml2_n422 || pixel_format == dml2_420 { 6 } else { 3 };
    let groups_per_bcl_cycle = if num_slices > 1 { 2 } else { 1 };
    let syntax_elements_per_group = if pixel_format == dml2_n422 { 4 } else { 3 };
    let mut group_delay = lines_to_reach_ixd_adjusted * slice_width_groups * (num_slices - 1);
    group_delay += (lines_to_reach_ixd_adjusted - 1) * slice_width_groups;
    group_delay += groups_per_bcl_cycle * (groups_to_reach_ixd_adjusted - (lines_to_reach_ixd_adjusted - 1) * slice_width_groups);
    group_delay += additional_group_delay;
    if num_slices >= 2 && (num_slices % 2 == 0 || lines_to_reach_ixd_adjusted % 2 == 0) {
        group_delay += (slice_width_groups % 2 != 0) as i32;
    }
    if group_delay_after_initial_xmit_delay_override_en == 0 {
        if num_slices >= 3 && num_slices % 2 == 1 && lines_to_reach_ixd_adjusted % 2 == 1 { group_delay += 1; }
    } else {
        group_delay += group_delay_after_initial_xmit_delay;
        if num_slices >= 3 && num_slices % 2 == 1 && lines_to_reach_ixd_adjusted % 2 == 1 { group_delay -= -1; }
    }
    let pipeline_delay = ssm_pipeline_delay + obsm_pipeline_delay;
    let pixel_delay = (group_delay - groups_per_bcl_cycle) * cycles_per_group + syntax_elements_per_group * groups_per_bcl_cycle + pipeline_delay;
    (*p).groups = group_delay; (*p).pipeline = pipeline_delay; (*p).pixels = pixel_delay;
    (*p).additional_group_delay = additional_group_delay; (*p).lines_to_reach_ixd = lines_to_reach_ixd_adjusted;
    (*p).groups_to_reach_ixd = groups_to_reach_ixd_adjusted; (*p).slice_width_groups = slice_width_groups;
    (*p).initial_xmit_delay = initial_xmit_delay; (*p).number_of_lines_to_reach_ixd = lines_to_reach_ixd;
    (*p).slice_width_modified = slice_width_modified;
}

unsafe fn dsc_compute_input_pixel_delay(pixel_format: dml2_output_format_class, num_slices: i32, dispclk_dynamic_gating_en: i32) -> i32 {
    let mut delay = 2;
    if dispclk_dynamic_gating_en == 1 && pixel_format != dml2_420 { delay += 1; }
    if pixel_format == dml2_n422 { delay += 1; } else if pixel_format == dml2_s422 { delay += 4; }
    delay += 1;
    if num_slices >= 2 { delay += 4; }
    delay
}

unsafe fn legacy_dsc_compute_output_pixel_delay(pixel_format: dml2_output_format_class, num_slices: i32, dscclk_dynamic_gating_en: i32) -> delay_uncertainty_t {
    let dispclk_per_dscclk = (if num_slices == 1 { 3 } else { 6 }) * (if pixel_format == dml2_420 || pixel_format == dml2_n422 { 2 } else { 1 });
    let mut delay = if num_slices == 1 { 3 + if pixel_format == dml2_420 || pixel_format == dml2_n422 { 2 } else { 0 } } else { 6 };
    let mut uncertainty = 0;
    delay += 1; uncertainty += 3 * dispclk_per_dscclk; delay += 3 * dispclk_per_dscclk;
    if dscclk_dynamic_gating_en == 1 { delay += dispclk_per_dscclk; }
    uncertainty += 3; delay += dispclk_per_dscclk + 2 + 1;
    delay += 1;
    delay_uncertainty_t { delay, uncertainty }
}

unsafe fn dsc_compute_output_pixel_delay(pixel_format: dml2_output_format_class, num_slices: i32, dscclk_dynamic_gating_en: i32) -> delay_uncertainty_t {
    let dispclk_per_dscclk = (if num_slices == 1 { 3 } else { 6 }) * (if pixel_format == dml2_420 || pixel_format == dml2_n422 { 2 } else { 1 });
    let mut delay = if num_slices == 1 { 3 + if pixel_format == dml2_420 || pixel_format == dml2_n422 { 2 } else { 0 } } else if pixel_format == dml2_n422 { 12 } else { 6 };
    let mut uncertainty = 0;
    uncertainty += 2 * dispclk_per_dscclk; delay += 1 + 2 * dispclk_per_dscclk;
    if dscclk_dynamic_gating_en == 1 { uncertainty += 4 * dispclk_per_dscclk; }
    uncertainty += 3; delay += dispclk_per_dscclk + 2 + 1;
    delay += 1;
    delay_uncertainty_t { delay, uncertainty }
}

pub unsafe fn dcn5_dsc_compute_delay_legacy(p: *mut delay_uncertainty_t, bpc: i32, bpp: f32, slice_width: i32, num_slices: i32, pixel_format: dml2_output_format_class, dscclk_dynamic_gating_en: i32, dispclk_dynamic_gating_en: i32, initial_xmit_delay_offset: i32, group_delay_after_initial_xmit_delay_override_en: i32, group_delay_after_initial_xmit_delay: i32) {
    let mut total_delay = dsc_compute_input_pixel_delay(pixel_format, num_slices, dispclk_dynamic_gating_en) + dscc_pcl_compute_delay(pixel_format, num_slices);
    let mut bcl = core::mem::MaybeUninit::<latency_t>::uninit();
    dscc_bcl_compute_delay(bcl.as_mut_ptr(), bpc, bpp, slice_width, num_slices, pixel_format, initial_xmit_delay_offset, group_delay_after_initial_xmit_delay_override_en, group_delay_after_initial_xmit_delay);
    total_delay += (*bcl.as_ptr()).pixels;
    let mut d = legacy_dsc_compute_output_pixel_delay(pixel_format, num_slices, dscclk_dynamic_gating_en);
    d.delay += total_delay; (*p).delay = d.delay; (*p).uncertainty = d.uncertainty;
}

pub unsafe fn dcn5_dsc_compute_delay(p: *mut delay_uncertainty_t, bpc: i32, bpp: f32, slice_width: i32, num_slices: i32, pixel_format: dml2_output_format_class, dscclk_dynamic_gating_en: i32, dispclk_dynamic_gating_en: i32, initial_xmit_delay_offset: i32, group_delay_after_initial_xmit_delay_override_en: i32, group_delay_after_initial_xmit_delay: i32) {
    let mut total_delay = dsc_compute_input_pixel_delay(pixel_format, num_slices, dispclk_dynamic_gating_en) + dscc_pcl_compute_delay(pixel_format, num_slices);
    let mut bcl = core::mem::MaybeUninit::<latency_t>::uninit();
    dscc_bcl_compute_delay(bcl.as_mut_ptr(), bpc, bpp, slice_width, num_slices, pixel_format, initial_xmit_delay_offset, group_delay_after_initial_xmit_delay_override_en, group_delay_after_initial_xmit_delay);
    total_delay += (*bcl.as_ptr()).pixels;
    let mut d = dsc_compute_output_pixel_delay(pixel_format, num_slices, dscclk_dynamic_gating_en);
    d.delay += total_delay; (*p).delay = d.delay; (*p).uncertainty = d.uncertainty;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
