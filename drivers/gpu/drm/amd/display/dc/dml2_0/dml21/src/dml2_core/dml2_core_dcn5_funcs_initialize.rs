// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding translation unit.

static mut core_dcn5_ip_caps_base: dml2_core_ip_params = unsafe { core::mem::zeroed() };

unsafe fn patch_ip_caps_with_explicit_ip_params(
    ip_caps: *mut dml2_ip_capabilities,
    ip_params: *const dml2_core_ip_params,
) {
    (*ip_caps).pipe_count = (*ip_params).max_num_dpp;
    (*ip_caps).otg_count = (*ip_params).max_num_otg;
    (*ip_caps).num_dsc = (*ip_params).num_dsc;
    (*ip_caps).max_num_dp2p0_streams = (*ip_params).max_num_dp2p0_streams;
    (*ip_caps).max_num_dp2p0_outputs = (*ip_params).max_num_dp2p0_outputs;
    (*ip_caps).max_num_hdmi_frl_outputs = (*ip_params).max_num_hdmi_frl_outputs;
    (*ip_caps).max_num_wb = (*ip_params).max_num_wb;
    (*ip_caps).rob_buffer_size_kbytes = (*ip_params).rob_buffer_size_kbytes;
    (*ip_caps).config_return_buffer_size_in_kbytes = (*ip_params).config_return_buffer_size_in_kbytes;
    (*ip_caps).config_return_buffer_segment_size_in_kbytes = (*ip_params).config_return_buffer_segment_size_in_kbytes;
    (*ip_caps).meta_fifo_size_in_kentries = (*ip_params).meta_fifo_size_in_kentries;
    (*ip_caps).compressed_buffer_segment_size_in_kbytes = (*ip_params).compressed_buffer_segment_size_in_kbytes;
    (*ip_caps).cursor_buffer_size = (*ip_params).cursor_buffer_size;
    (*ip_caps).max_flip_time_us = (*ip_params).max_flip_time_us;
    (*ip_caps).max_flip_time_lines = (*ip_params).max_flip_time_lines;
    (*ip_caps).hostvm_mode = (*ip_params).hostvm_mode;
    (*ip_caps).vblank_nom_default_us = (*ip_params).vblank_nom_default_us;
}

unsafe fn patch_ip_params_with_ip_caps(
    ip_params: *mut dml2_core_ip_params,
    ip_caps: *const dml2_ip_capabilities,
) {
    (*ip_params).max_num_dpp = (*ip_caps).pipe_count;
    (*ip_params).max_num_opp = (*ip_caps).pipe_count;
    (*ip_params).max_num_otg = (*ip_caps).otg_count;
    (*ip_params).num_dsc = (*ip_caps).num_dsc;
    (*ip_params).max_num_dp2p0_streams = (*ip_caps).max_num_dp2p0_streams;
    (*ip_params).max_num_dp2p0_outputs = (*ip_caps).max_num_dp2p0_outputs;
    (*ip_params).max_num_hdmi_frl_outputs = (*ip_caps).max_num_hdmi_frl_outputs;
    (*ip_params).max_num_wb = (*ip_caps).max_num_wb;
    (*ip_params).rob_buffer_size_kbytes = (*ip_caps).rob_buffer_size_kbytes;
    (*ip_params).config_return_buffer_size_in_kbytes = (*ip_caps).config_return_buffer_size_in_kbytes;
    (*ip_params).config_return_buffer_segment_size_in_kbytes = (*ip_caps).config_return_buffer_segment_size_in_kbytes;
    (*ip_params).meta_fifo_size_in_kentries = (*ip_caps).meta_fifo_size_in_kentries;
    (*ip_params).compressed_buffer_segment_size_in_kbytes = (*ip_caps).compressed_buffer_segment_size_in_kbytes;
    (*ip_params).cursor_buffer_size = (*ip_caps).cursor_buffer_size;
    (*ip_params).max_flip_time_us = (*ip_caps).max_flip_time_us;
    (*ip_params).max_flip_time_lines = (*ip_caps).max_flip_time_lines;
    (*ip_params).hostvm_mode = (*ip_caps).hostvm_mode;
    (*ip_params).vblank_nom_default_us = (*ip_caps).vblank_nom_default_us;
}

pub unsafe fn dml2_core_dcn5_funcs_initialize(
    in_out: *mut dml2_core_initialize_in_out,
) -> bool {
    let core = (*in_out).instance;

    // DML_LOG_DEBUG("DML_CORE::%s enter\n", __func__);
    if !(*in_out).explicit_ip_bb.is_null() && (*in_out).explicit_ip_bb_size > 0 {
        core::ptr::copy_nonoverlapping(
            (*in_out).explicit_ip_bb as *const u8,
            &mut (*core).clean_me_up.mode_lib.ip as *mut _ as *mut u8,
            (*in_out).explicit_ip_bb_size,
        );

        patch_ip_caps_with_explicit_ip_params(
            (*in_out).ip_caps,
            (*in_out).explicit_ip_bb,
        );
    } else {
        core::ptr::copy_nonoverlapping(
            &core_dcn5_ip_caps_base as *const dml2_core_ip_params as *const u8,
            &mut (*core).clean_me_up.mode_lib.ip as *mut _ as *mut u8,
            core::mem::size_of::<dml2_core_ip_params>(),
        );
        patch_ip_params_with_ip_caps(
            &mut (*core).clean_me_up.mode_lib.ip,
            (*in_out).ip_caps,
        );

        (*core).clean_me_up.mode_lib.ip.imall_supported = false;
    }

    core::ptr::copy_nonoverlapping(
        (*in_out).ip_caps as *const u8,
        &mut (*core).clean_me_up.mode_lib.ip_caps as *mut _ as *mut u8,
        core::mem::size_of::<dml2_ip_capabilities>(),
    );
    (*core).utm_soc_bb = (*in_out).utm_soc_bb;

    (*core).clean_me_up.mode_lib.ip.use_legacy_dsc_delay_formula =
        (*in_out).project_id != dml2_project_dcn5x_utm;

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
