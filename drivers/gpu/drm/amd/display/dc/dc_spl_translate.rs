// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Types, constants, callbacks, and functions referenced here are supplied by
// the corresponding translated headers and implementation units.

static mut DCN2_SPL_CALLBACKS: spl_callbacks = spl_callbacks {
    spl_calc_lb_num_partitions: dscl2_spl_calc_lb_num_partitions,
};
static mut DCN32_SPL_CALLBACKS: spl_callbacks = spl_callbacks {
    spl_calc_lb_num_partitions: dscl32_spl_calc_lb_num_partitions,
};
static mut DCN401_SPL_CALLBACKS: spl_callbacks = spl_callbacks {
    spl_calc_lb_num_partitions: dscl401_spl_calc_lb_num_partitions,
};
static mut DCN50_SPL_CALLBACKS: spl_callbacks = spl_callbacks {
    spl_calc_lb_num_partitions: dscl401_spl_calc_lb_num_partitions,
};

unsafe fn populate_splrect_from_rect(spl_rect: *mut spl_rect, rect: *const rect) {
    (*spl_rect).x = (*rect).x;
    (*spl_rect).y = (*rect).y;
    (*spl_rect).width = (*rect).width;
    (*spl_rect).height = (*rect).height;
}

unsafe fn populate_rect_from_splrect(rect: *mut rect, spl_rect: *const spl_rect) {
    (*rect).x = (*spl_rect).x;
    (*rect).y = (*spl_rect).y;
    (*rect).width = (*spl_rect).width;
    (*rect).height = (*spl_rect).height;
}

unsafe fn populate_spltaps_from_taps(
    spl_scaling_quality: *mut spl_taps,
    scaling_quality: *const scaling_taps,
) {
    (*spl_scaling_quality).h_taps_c = (*scaling_quality).h_taps_c;
    (*spl_scaling_quality).h_taps = (*scaling_quality).h_taps;
    (*spl_scaling_quality).v_taps_c = (*scaling_quality).v_taps_c;
    (*spl_scaling_quality).v_taps = (*scaling_quality).v_taps;
    (*spl_scaling_quality).integer_scaling = (*scaling_quality).integer_scaling;
}

unsafe fn populate_taps_from_spltaps(
    scaling_quality: *mut scaling_taps,
    spl_scaling_quality: *const spl_taps,
) {
    (*scaling_quality).h_taps_c = (*spl_scaling_quality).h_taps_c + 1;
    (*scaling_quality).h_taps = (*spl_scaling_quality).h_taps + 1;
    (*scaling_quality).v_taps_c = (*spl_scaling_quality).v_taps_c + 1;
    (*scaling_quality).v_taps = (*spl_scaling_quality).v_taps + 1;
}

unsafe fn populate_ratios_from_splratios(ratios: *mut scaling_ratios, spl_ratios: *const ratio) {
    (*ratios).horz = dc_fixpt_from_ux_dy((*spl_ratios).h_scale_ratio >> 5, 3, 19);
    (*ratios).vert = dc_fixpt_from_ux_dy((*spl_ratios).v_scale_ratio >> 5, 3, 19);
    (*ratios).horz_c = dc_fixpt_from_ux_dy((*spl_ratios).h_scale_ratio_c >> 5, 3, 19);
    (*ratios).vert_c = dc_fixpt_from_ux_dy((*spl_ratios).v_scale_ratio_c >> 5, 3, 19);
}

unsafe fn populate_inits_from_splinits(inits: *mut scl_inits, spl_inits: *const init) {
    (*inits).h = dc_fixpt_from_int_dy((*spl_inits).h_filter_init_int, (*spl_inits).h_filter_init_frac >> 5, 0, 19);
    (*inits).v = dc_fixpt_from_int_dy((*spl_inits).v_filter_init_int, (*spl_inits).v_filter_init_frac >> 5, 0, 19);
    (*inits).h_c = dc_fixpt_from_int_dy((*spl_inits).h_filter_init_int_c, (*spl_inits).h_filter_init_frac_c >> 5, 0, 19);
    (*inits).v_c = dc_fixpt_from_int_dy((*spl_inits).v_filter_init_int_c, (*spl_inits).v_filter_init_frac_c >> 5, 0, 19);
}

unsafe fn populate_splformat_from_format(spl_pixel_format: *mut spl_pixel_format, pixel_format: dc_pixel_format) {
    if pixel_format < PIXEL_FORMAT_INVALID {
        *spl_pixel_format = pixel_format as spl_pixel_format;
    } else {
        *spl_pixel_format = SPL_PIXEL_FORMAT_INVALID;
    }
}

unsafe fn set_linear_light_scaling_preference(plane_state: *const dc_plane_state, spl_in: *mut spl_in) {
    if !plane_state.is_null() {
        (*spl_in).lls_pref = match (*plane_state).scaling_linearity {
            DC_SCALING_LINEARITY_LINEAR => LLS_PREF_YES,
            DC_SCALING_LINEARITY_SOURCE => LLS_PREF_NO,
            _ => LLS_PREF_DONT_CARE,
        };
    } else {
        (*spl_in).lls_pref = LLS_PREF_DONT_CARE;
    }
}

/// @brief Translate SPL input parameters from pipe context
/// @param pipe_ctx
/// @param spl_in
pub unsafe fn translate_SPL_in_params_from_pipe_ctx(pipe_ctx: *mut pipe_ctx, spl_in: *mut spl_in) {
    let plane_state = (*pipe_ctx).plane_state;
    let stream = (*pipe_ctx).stream;
    let odm_slice_src = resource_get_odm_slice_src_rect(pipe_ctx);

    // Assign the function to calculate the number of partitions in the line buffer
    // This is used to determine the vtap support
    (*spl_in).callbacks = match (*(*plane_state).ctx).dce_version {
        DCN_VERSION_2_0 => DCN2_SPL_CALLBACKS,
        DCN_VERSION_3_2 => DCN32_SPL_CALLBACKS,
        DCN_VERSION_4_01 | DCN_VERSION_4_2 | DCN_VERSION_4_2B => DCN401_SPL_CALLBACKS,
        DCN_VERSION_6_0 => DCN50_SPL_CALLBACKS,
        _ => DCN2_SPL_CALLBACKS,
    };
    // Make format field from spl_in point to plane_res scl_data format
    populate_splformat_from_format(&mut (*spl_in).basic_in.format, (*pipe_ctx).plane_res.scl_data.format);
    // Make view_format from basic_out point to view_format from stream
    (*spl_in).basic_out.view_format = (*stream).view_format as spl_view_3d;
    populate_splrect_from_rect(&mut (*spl_in).basic_in.clip_rect, &(*plane_state).clip_rect);
    populate_splrect_from_rect(&mut (*spl_in).basic_out.src_rect, &(*stream).src);
    populate_splrect_from_rect(&mut (*spl_in).basic_out.dst_rect, &(*stream).dst);
    (*spl_in).basic_in.rotation = (*plane_state).rotation as spl_rotation_angle;
    populate_splrect_from_rect(&mut (*spl_in).basic_in.src_rect, &(*plane_state).src_rect);
    populate_splrect_from_rect(&mut (*spl_in).basic_in.dst_rect, &(*plane_state).dst_rect);
    (*spl_in).basic_in.horizontal_mirror = (*plane_state).horizontal_mirror;

    (*spl_in).basic_in.num_h_slices_recout_width_align.use_recout_width_aligned = false;
    (*spl_in).basic_in.num_h_slices_recout_width_align.num_slices_recout_width.mpc_num_h_slices = resource_get_mpc_slice_count(pipe_ctx);
    if (*stream).view_format == VIEW_3D_FORMAT_SIDE_BY_SIDE {
        (*spl_in).basic_in.mpc_h_slice_index = 0;
    } else {
        (*spl_in).basic_in.mpc_h_slice_index = resource_get_mpc_slice_index(pipe_ctx);
    }
    populate_splrect_from_rect(&mut (*spl_in).basic_out.odm_slice_rect, &odm_slice_src);
    (*spl_in).basic_out.odm_combine_factor = 0;
    (*spl_in).odm_slice_index = resource_get_odm_slice_index(pipe_ctx);
    (*spl_in).basic_out.output_size.width = (*stream).timing.h_addressable + (*stream).timing.h_border_left + (*stream).timing.h_border_right + (*pipe_ctx).dsc_padding_params.dsc_hactive_padding;
    (*spl_in).basic_out.output_size.height = (*stream).timing.v_addressable + (*stream).timing.v_border_bottom + (*stream).timing.v_border_top;
    (*spl_in).basic_out.max_downscale_src_width = (*(*(*stream).ctx).dc).debug.max_downscale_src_width;
    (*spl_in).basic_out.always_scale = (*(*(*stream).ctx).dc).debug.always_scale;
    (*spl_in).basic_out.alpha_en = (*pipe_ctx).plane_res.scl_data.lb_params.alpha_en;
    (*spl_in).basic_out.use_two_pixels_per_container = (*(*pipe_ctx).stream_res.tg).funcs.is_two_pixels_per_container.unwrap()(&(*stream).timing);
    populate_spltaps_from_taps(&mut (*spl_in).scaling_quality, &(*plane_state).scaling_quality);
    (*spl_in).prefer_easf = (*(*(*stream).ctx).dc).config.prefer_easf;
    (*spl_in).disable_easf = false;
    if (*(*(*stream).ctx).dc).debug.force_easf == 1 { (*spl_in).prefer_easf = false; }
    else if (*(*(*stream).ctx).dc).debug.force_easf == 2 { (*spl_in).disable_easf = true; }
    else if (*(*(*stream).ctx).dc).debug.force_easf == 3 { (*spl_in).override_easf = true; }

    let sharpness_setting = (*(*(*stream).ctx).dc).debug.force_sharpness;
    let mut force_sharpness_level = (*(*(*stream).ctx).dc).debug.force_sharpness_level;
    if sharpness_setting == SHARPNESS_HW_OFF { (*spl_in).adaptive_sharpness.enable = false; }
    else if sharpness_setting == SHARPNESS_ZERO { (*spl_in).adaptive_sharpness.enable = true; (*spl_in).adaptive_sharpness.sharpness_level = 0; }
    else if sharpness_setting == SHARPNESS_CUSTOM {
        (*spl_in).adaptive_sharpness.sharpness_range = (*plane_state).sharpness_range;
        if force_sharpness_level > 0 {
            if force_sharpness_level > 10 { force_sharpness_level = 10; }
            (*spl_in).adaptive_sharpness.enable = true;
            (*spl_in).adaptive_sharpness.sharpness_level = force_sharpness_level;
        } else if !(*plane_state).adaptive_sharpness_en {
            (*spl_in).adaptive_sharpness.enable = false;
            (*spl_in).adaptive_sharpness.sharpness_level = 0;
        } else {
            (*spl_in).adaptive_sharpness.enable = true;
            (*spl_in).adaptive_sharpness.sharpness_level = (*plane_state).sharpness_level;
        }
    }
    if (*(*(*stream).ctx).dc).debug.force_lls > 0 { (*spl_in).lls_pref = (*(*(*stream).ctx).dc).debug.force_lls; }
    else if (*plane_state).ctx.dce_version == DCN_VERSION_4_01 || (*plane_state).ctx.dce_version == DCN_VERSION_4_2 || (*plane_state).ctx.dce_version == DCN_VERSION_4_2B { (*spl_in).lls_pref = LLS_PREF_DONT_CARE; }
    else { set_linear_light_scaling_preference(plane_state, spl_in); }
    (*spl_in).upsp_mode = (*pipe_ctx).plane_res.scl_data.upsp;
    if (*(*(*stream).ctx).dc).debug.force_cositing != 0 { (*spl_in).basic_in.cositing = (*(*(*stream).ctx).dc).debug.force_cositing - 1; }
    else { (*spl_in).basic_in.cositing = (*plane_state).cositing; }
    (*spl_in).basic_in.tf_type = (*plane_state).in_transfer_func.type as spl_transfer_func_type;
    (*spl_in).basic_in.tf_predefined_type = (*plane_state).in_transfer_func.tf as spl_transfer_func_predefined;
    (*spl_in).h_active = (*pipe_ctx).plane_res.scl_data.h_active;
    (*spl_in).v_active = (*pipe_ctx).plane_res.scl_data.v_active;
    (*spl_in).sharpen_policy = (*plane_state).adaptive_sharpness_policy as sharpen_policy;
    (*spl_in).debug.scale_to_sharpness_policy = (*(*(*stream).ctx).dc).debug.scale_to_sharpness_policy as scale_to_sharpness_policy;
    (*spl_in).is_fullscreen = (*stream).sharpening_required;
    (*spl_in).is_hdr_on = dm_helpers_is_hdr_on((*stream).ctx, stream);
    (*spl_in).sdr_white_level_nits = (*plane_state).sdr_white_level_nits;
}

/// @brief Translate SPL output parameters to pipe context
/// @param pipe_ctx
/// @param spl_out
pub unsafe fn translate_SPL_out_params_to_pipe_ctx(pipe_ctx: *mut pipe_ctx, spl_out: *mut spl_out) {
    populate_rect_from_splrect(&mut (*pipe_ctx).plane_res.scl_data.recout, &(*(*spl_out).dscl_prog_data).recout);
    populate_ratios_from_splratios(&mut (*pipe_ctx).plane_res.scl_data.ratios, &(*(*spl_out).dscl_prog_data).ratios);
    populate_rect_from_splrect(&mut (*pipe_ctx).plane_res.scl_data.viewport, &(*(*spl_out).dscl_prog_data).viewport);
    populate_rect_from_splrect(&mut (*pipe_ctx).plane_res.scl_data.viewport_c, &(*(*spl_out).dscl_prog_data).viewport_c);
    populate_taps_from_spltaps(&mut (*pipe_ctx).plane_res.scl_data.taps, &(*(*spl_out).dscl_prog_data).taps);
    populate_inits_from_splinits(&mut (*pipe_ctx).plane_res.scl_data.inits, &(*(*spl_out).dscl_prog_data).init);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
