/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C headers supply the declarations used below.

static const dml_funcs dml20_funcs: dml_funcs = dml_funcs {
    validate: dml20_ModeSupportAndSystemConfigurationFull,
    recalculate: dml20_recalculate,
    rq_dlg_get_dlg_reg: dml20_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg: dml20_rq_dlg_get_rq_reg,
};
static const dml_funcs dml20v2_funcs: dml_funcs = dml_funcs {
    validate: dml20v2_ModeSupportAndSystemConfigurationFull,
    recalculate: dml20v2_recalculate,
    rq_dlg_get_dlg_reg: dml20v2_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg: dml20v2_rq_dlg_get_rq_reg,
};
static const dml_funcs dml21_funcs: dml_funcs = dml_funcs {
    validate: dml21_ModeSupportAndSystemConfigurationFull,
    recalculate: dml21_recalculate,
    rq_dlg_get_dlg_reg: dml21_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg: dml21_rq_dlg_get_rq_reg,
};
static const dml_funcs dml30_funcs: dml_funcs = dml_funcs {
    validate: dml30_ModeSupportAndSystemConfigurationFull,
    recalculate: dml30_recalculate,
    rq_dlg_get_dlg_reg: dml30_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg: dml30_rq_dlg_get_rq_reg,
};
static const dml_funcs dml31_funcs: dml_funcs = dml_funcs {
    validate: dml31_ModeSupportAndSystemConfigurationFull,
    recalculate: dml31_recalculate,
    rq_dlg_get_dlg_reg: dml31_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg: dml31_rq_dlg_get_rq_reg,
};
static const dml_funcs dml314_funcs: dml_funcs = dml_funcs {
    validate: dml314_ModeSupportAndSystemConfigurationFull,
    recalculate: dml314_recalculate,
    rq_dlg_get_dlg_reg: dml314_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg: dml314_rq_dlg_get_rq_reg,
};
static const dml_funcs dml32_funcs: dml_funcs = dml_funcs {
    validate: dml32_ModeSupportAndSystemConfigurationFull,
    recalculate: dml32_recalculate,
    rq_dlg_get_dlg_reg_v2: dml32_rq_dlg_get_dlg_reg,
    rq_dlg_get_rq_reg_v2: dml32_rq_dlg_get_rq_reg,
};

pub unsafe fn dml_init_instance(
    lib: *mut display_mode_lib,
    soc_bb: *const _vcs_dpi_soc_bounding_box_st,
    ip_params: *const _vcs_dpi_ip_params_st,
    project: dml_project,
) {
    (*lib).soc = *soc_bb;
    (*lib).ip = *ip_params;
    (*lib).project = project;
    match project {
        DML_PROJECT_NAVI10 | DML_PROJECT_DCN201 => (*lib).funcs = dml20_funcs,
        DML_PROJECT_NAVI10v2 => (*lib).funcs = dml20v2_funcs,
        DML_PROJECT_DCN21 => (*lib).funcs = dml21_funcs,
        DML_PROJECT_DCN30 => (*lib).funcs = dml30_funcs,
        DML_PROJECT_DCN31 | DML_PROJECT_DCN315 => (*lib).funcs = dml31_funcs,
        DML_PROJECT_DCN314 => (*lib).funcs = dml314_funcs,
        DML_PROJECT_DCN32 => (*lib).funcs = dml32_funcs,
        _ => (),
    }
}

pub fn dml_get_status_message(status: dm_validation_status) -> &'static str {
    match status {
        DML_VALIDATION_OK => "Validation OK",
        DML_FAIL_SCALE_RATIO_TAP => "Scale ratio/tap",
        DML_FAIL_SOURCE_PIXEL_FORMAT => "Source pixel format",
        DML_FAIL_VIEWPORT_SIZE => "Viewport size",
        DML_FAIL_TOTAL_V_ACTIVE_BW => "Total vertical active bandwidth",
        DML_FAIL_DIO_SUPPORT => "DIO support",
        DML_FAIL_NOT_ENOUGH_DSC => "Not enough DSC Units",
        DML_FAIL_DSC_CLK_REQUIRED => "DSC clock required",
        DML_FAIL_URGENT_LATENCY => "Urgent latency",
        DML_FAIL_REORDERING_BUFFER => "Re-ordering buffer",
        DML_FAIL_DISPCLK_DPPCLK => "Dispclk and Dppclk",
        DML_FAIL_TOTAL_AVAILABLE_PIPES => "Total available pipes",
        DML_FAIL_NUM_OTG => "Number of OTG",
        DML_FAIL_WRITEBACK_MODE => "Writeback mode",
        DML_FAIL_WRITEBACK_LATENCY => "Writeback latency",
        DML_FAIL_WRITEBACK_SCALE_RATIO_TAP => "Writeback scale ratio/tap",
        DML_FAIL_CURSOR_SUPPORT => "Cursor support",
        DML_FAIL_PITCH_SUPPORT => "Pitch support",
        DML_FAIL_PTE_BUFFER_SIZE => "PTE buffer size",
        DML_FAIL_DSC_INPUT_BPC => "DSC input bpc",
        DML_FAIL_PREFETCH_SUPPORT => "Prefetch support",
        DML_FAIL_V_RATIO_PREFETCH => "Vertical ratio prefetch",
        _ => "Unknown Status",
    }
}

pub unsafe fn dml_log_pipe_params(
    mode_lib: *mut display_mode_lib,
    pipes: *mut display_e2e_pipe_params_st,
    pipe_cnt: i32,
) {
    let _ = mode_lib;
    for i in 0..pipe_cnt {
        let p = &mut *pipes.add(i as usize);
        let pipe_src = &p.pipe.src;
        let pipe_dest = &p.pipe.dest;
        let scale_ratio_depth = &p.pipe.scale_ratio_depth;
        let scale_taps = &p.pipe.scale_taps;
        let dout = &p.dout;
        let clks_cfg = &p.clks_cfg;
        dml_print!("DML PARAMS: =====================================\n");
        dml_print!("DML PARAMS: PIPE [{}] SOURCE PARAMS:\n", i);
        dml_print!("DML PARAMS:     source_format              = {}\n", pipe_src.source_format);
        dml_print!("DML PARAMS:     dcc                        = {}\n", pipe_src.dcc);
        dml_print!("DML PARAMS:     dcc_rate                   = {}\n", pipe_src.dcc_rate);
        dml_print!("DML PARAMS:     dcc_use_global             = {}\n", pipe_src.dcc_use_global);
        dml_print!("DML PARAMS:     vm                         = {}\n", pipe_src.vm);
        dml_print!("DML PARAMS:     gpuvm                      = {}\n", pipe_src.gpuvm);
        dml_print!("DML PARAMS:     hostvm                     = {}\n", pipe_src.hostvm);
        dml_print!("DML PARAMS:     gpuvm_levels_force_en      = {}\n", pipe_src.gpuvm_levels_force_en);
        dml_print!("DML PARAMS:     gpuvm_levels_force         = {}\n", pipe_src.gpuvm_levels_force);
        dml_print!("DML PARAMS:     source_scan                = {}\n", pipe_src.source_scan);
        dml_print!("DML PARAMS:     sw_mode                    = {}\n", pipe_src.sw_mode);
        dml_print!("DML PARAMS:     macro_tile_size            = {}\n", pipe_src.macro_tile_size);
        dml_print!("DML PARAMS:     viewport_width             = {}\n", pipe_src.viewport_width);
        dml_print!("DML PARAMS:     viewport_height            = {}\n", pipe_src.viewport_height);
        dml_print!("DML PARAMS:     viewport_y_y               = {}\n", pipe_src.viewport_y_y);
        dml_print!("DML PARAMS:     viewport_y_c               = {}\n", pipe_src.viewport_y_c);
        dml_print!("DML PARAMS:     viewport_width_c           = {}\n", pipe_src.viewport_width_c);
        dml_print!("DML PARAMS:     viewport_height_c          = {}\n", pipe_src.viewport_height_c);
        dml_print!("DML PARAMS:     data_pitch                 = {}\n", pipe_src.data_pitch);
        dml_print!("DML PARAMS:     data_pitch_c               = {}\n", pipe_src.data_pitch_c);
        dml_print!("DML PARAMS:     meta_pitch                 = {}\n", pipe_src.meta_pitch);
        dml_print!("DML PARAMS:     meta_pitch_c               = {}\n", pipe_src.meta_pitch_c);
        dml_print!("DML PARAMS:     cur0_src_width             = {}\n", pipe_src.cur0_src_width);
        dml_print!("DML PARAMS:     cur0_bpp                   = {}\n", pipe_src.cur0_bpp);
        dml_print!("DML PARAMS:     cur1_src_width             = {}\n", pipe_src.cur1_src_width);
        dml_print!("DML PARAMS:     cur1_bpp                   = {}\n", pipe_src.cur1_bpp);
        dml_print!("DML PARAMS:     num_cursors                = {}\n", pipe_src.num_cursors);
        dml_print!("DML PARAMS:     is_hsplit                  = {}\n", pipe_src.is_hsplit);
        dml_print!("DML PARAMS:     hsplit_grp                 = {}\n", pipe_src.hsplit_grp);
        dml_print!("DML PARAMS:     dynamic_metadata_enable    = {}\n", pipe_src.dynamic_metadata_enable);
        dml_print!("DML PARAMS:     dmdata_lines_before_active = {}\n", pipe_src.dynamic_metadata_lines_before_active);
        dml_print!("DML PARAMS:     dmdata_xmit_bytes          = {}\n", pipe_src.dynamic_metadata_xmit_bytes);
        dml_print!("DML PARAMS:     immediate_flip             = {}\n", pipe_src.immediate_flip);
        dml_print!("DML PARAMS:     v_total_min                = {}\n", pipe_src.v_total_min);
        dml_print!("DML PARAMS:     v_total_max                = {}\n", pipe_src.v_total_max);
        dml_print!("DML PARAMS: =====================================\n");
        dml_print!("DML PARAMS: PIPE [{}] DESTINATION PARAMS:\n", i);
        dml_print!("DML PARAMS:     recout_width               = {}\n", pipe_dest.recout_width);
        dml_print!("DML PARAMS:     recout_height              = {}\n", pipe_dest.recout_height);
        dml_print!("DML PARAMS:     full_recout_width          = {}\n", pipe_dest.full_recout_width);
        dml_print!("DML PARAMS:     full_recout_height         = {}\n", pipe_dest.full_recout_height);
        dml_print!("DML PARAMS:     hblank_start               = {}\n", pipe_dest.hblank_start);
        dml_print!("DML PARAMS:     hblank_end                 = {}\n", pipe_dest.hblank_end);
        dml_print!("DML PARAMS:     vblank_start               = {}\n", pipe_dest.vblank_start);
        dml_print!("DML PARAMS:     vblank_end                 = {}\n", pipe_dest.vblank_end);
        dml_print!("DML PARAMS:     htotal                     = {}\n", pipe_dest.htotal);
        dml_print!("DML PARAMS:     vtotal                     = {}\n", pipe_dest.vtotal);
        dml_print!("DML PARAMS:     vactive                    = {}\n", pipe_dest.vactive);
        dml_print!("DML PARAMS:     hactive                    = {}\n", pipe_dest.hactive);
        dml_print!("DML PARAMS:     vstartup_start             = {}\n", pipe_dest.vstartup_start);
        dml_print!("DML PARAMS:     vupdate_offset             = {}\n", pipe_dest.vupdate_offset);
        dml_print!("DML PARAMS:     vupdate_width              = {}\n", pipe_dest.vupdate_width);
        dml_print!("DML PARAMS:     vready_offset              = {}\n", pipe_dest.vready_offset);
        dml_print!("DML PARAMS:     interlaced                 = {}\n", pipe_dest.interlaced);
        dml_print!("DML PARAMS:     pixel_rate_mhz             = {}\n", pipe_dest.pixel_rate_mhz);
        dml_print!("DML PARAMS:     sync_vblank_all_planes     = {}\n", pipe_dest.synchronized_vblank_all_planes);
        dml_print!("DML PARAMS:     otg_inst                   = {}\n", pipe_dest.otg_inst);
        dml_print!("DML PARAMS:     odm_combine                = {}\n", pipe_dest.odm_combine);
        dml_print!("DML PARAMS:     use_maximum_vstartup       = {}\n", pipe_dest.use_maximum_vstartup);
        dml_print!("DML PARAMS:     vtotal_max                 = {}\n", pipe_dest.vtotal_max);
        dml_print!("DML PARAMS:     vtotal_min                 = {}\n", pipe_dest.vtotal_min);
        dml_print!("DML PARAMS: =====================================\n");
        dml_print!("DML PARAMS: PIPE [{}] SCALER PARAMS:\n", i);
        dml_print!("DML PARAMS:     hscl_ratio                 = {}\n", scale_ratio_depth.hscl_ratio);
        dml_print!("DML PARAMS:     vscl_ratio                 = {}\n", scale_ratio_depth.vscl_ratio);
        dml_print!("DML PARAMS:     hscl_ratio_c               = {}\n", scale_ratio_depth.hscl_ratio_c);
        dml_print!("DML PARAMS:     vscl_ratio_c               = {}\n", scale_ratio_depth.vscl_ratio_c);
        dml_print!("DML PARAMS:     vinit                      = {}\n", scale_ratio_depth.vinit);
        dml_print!("DML PARAMS:     vinit_c                    = {}\n", scale_ratio_depth.vinit_c);
        dml_print!("DML PARAMS:     vinit_bot                  = {}\n", scale_ratio_depth.vinit_bot);
        dml_print!("DML PARAMS:     vinit_bot_c                = {}\n", scale_ratio_depth.vinit_bot_c);
        dml_print!("DML PARAMS:     lb_depth                   = {}\n", scale_ratio_depth.lb_depth);
        dml_print!("DML PARAMS:     scl_enable                 = {}\n", scale_ratio_depth.scl_enable);
        dml_print!("DML PARAMS:     htaps                      = {}\n", scale_taps.htaps);
        dml_print!("DML PARAMS:     vtaps                      = {}\n", scale_taps.vtaps);
        dml_print!("DML PARAMS:     htaps_c                    = {}\n", scale_taps.htaps_c);
        dml_print!("DML PARAMS:     vtaps_c                    = {}\n", scale_taps.vtaps_c);
        dml_print!("DML PARAMS: =====================================\n");
        dml_print!("DML PARAMS: PIPE [{}] DISPLAY OUTPUT PARAMS:\n", i);
        dml_print!("DML PARAMS:     output_type                = {}\n", dout.output_type);
        dml_print!("DML PARAMS:     output_format              = {}\n", dout.output_format);
        dml_print!("DML PARAMS:     dsc_input_bpc              = {}\n", dout.dsc_input_bpc);
        dml_print!("DML PARAMS:     output_bpp                 = {}\n", dout.output_bpp);
        dml_print!("DML PARAMS:     dp_lanes                   = {}\n", dout.dp_lanes);
        dml_print!("DML PARAMS:     dsc_enable                 = {}\n", dout.dsc_enable);
        dml_print!("DML PARAMS:     dsc_slices                 = {}\n", dout.dsc_slices);
        dml_print!("DML PARAMS:     wb_enable                  = {}\n", dout.wb_enable);
        dml_print!("DML PARAMS:     num_active_wb              = {}\n", dout.num_active_wb);
        dml_print!("DML PARAMS: =====================================\n");
        dml_print!("DML PARAMS: PIPE [{}] CLOCK CONFIG PARAMS:\n", i);
        dml_print!("DML PARAMS:     voltage                    = {}\n", clks_cfg.voltage);
        dml_print!("DML PARAMS:     dppclk_mhz                 = {}\n", clks_cfg.dppclk_mhz);
        dml_print!("DML PARAMS:     refclk_mhz                 = {}\n", clks_cfg.refclk_mhz);
        dml_print!("DML PARAMS:     dispclk_mhz                = {}\n", clks_cfg.dispclk_mhz);
        dml_print!("DML PARAMS:     dcfclk_mhz                 = {}\n", clks_cfg.dcfclk_mhz);
        dml_print!("DML PARAMS:     socclk_mhz                 = {}\n", clks_cfg.socclk_mhz);
        dml_print!("DML PARAMS: =====================================\n");
    }
}

pub unsafe fn dml_log_mode_support_params(mode_lib: *mut display_mode_lib) {
    let vba = &(*mode_lib).vba;
    let mut i = vba.soc.num_states;
    while i >= 0 {
        dml_print!("DML SUPPORT: ===============================================\n");
        dml_print!("DML SUPPORT: Voltage State {}\n", i);
        dml_print!("DML SUPPORT:     Mode Supported              : {}\n", if vba.ModeSupport[i][0] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Mode Supported (pipe split) : {}\n", if vba.ModeSupport[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Scale Ratio And Taps        : {}\n", if vba.ScaleRatioAndTapsSupport { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Source Format Pixel And Scan: {}\n", if vba.SourceFormatPixelAndScanSupport { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Viewport Size               : [{}, {}]\n", if vba.ViewportSizeSupport[i][0] { "Supported" } else { "NOT Supported" }, if vba.ViewportSizeSupport[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     DIO Support                 : {}\n", if vba.DIOSupport[i] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     ODM Combine 4To1 Support    : {}\n", if vba.ODMCombine4To1SupportCheckOK[i] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     DSC Units                   : {}\n", if vba.NotEnoughDSCUnits[i] { "Not Supported" } else { "Supported" });
        dml_print!("DML SUPPORT:     DSCCLK Required             : {}\n", if vba.DSCCLKRequiredMoreThanSupported[i] { "Not Supported" } else { "Supported" });
        dml_print!("DML SUPPORT:     DTBCLK Required             : {}\n", if vba.DTBCLKRequiredMoreThanSupported[i] { "Not Supported" } else { "Supported" });
        dml_print!("DML SUPPORT:     Re-ordering Buffer          : [{}, {}]\n", if vba.ROBSupport[i][0] { "Supported" } else { "NOT Supported" }, if vba.ROBSupport[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     DISPCLK and DPPCLK          : [{}, {}]\n", if vba.DISPCLK_DPPCLK_Support[i][0] { "Supported" } else { "NOT Supported" }, if vba.DISPCLK_DPPCLK_Support[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Total Available Pipes       : [{}, {}]\n", if vba.TotalAvailablePipesSupport[i][0] { "Supported" } else { "NOT Supported" }, if vba.TotalAvailablePipesSupport[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Writeback Latency           : {}\n", if vba.WritebackLatencySupport { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Writeback Scale Ratio/Taps  : {}\n", if vba.WritebackScaleRatioAndTapsSupport { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Cursor                      : {}\n", if vba.CursorSupport { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Pitch                       : {}\n", if vba.PitchSupport { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Prefetch                    : [{}, {}]\n", if vba.PrefetchSupported[i][0] { "Supported" } else { "NOT Supported" }, if vba.PrefetchSupported[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Dynamic Metadata            : [{}, {}]\n", if vba.DynamicMetadataSupported[i][0] { "Supported" } else { "NOT Supported" }, if vba.DynamicMetadataSupported[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     Total Vertical Active BW    : [{}, {}]\n", if vba.TotalVerticalActiveBandwidthSupport[i][0] { "Supported" } else { "NOT Supported" }, if vba.TotalVerticalActiveBandwidthSupport[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     VRatio In Prefetch          : [{}, {}]\n", if vba.VRatioInPrefetchSupported[i][0] { "Supported" } else { "NOT Supported" }, if vba.VRatioInPrefetchSupported[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     PTE Buffer Size              : [{}, {}]\n", if vba.PTEBufferSizeNotExceeded[i][0] { "Supported" } else { "NOT Supported" }, if vba.PTEBufferSizeNotExceeded[i][1] { "Supported" } else { "NOT Supported" });
        dml_print!("DML SUPPORT:     DSC Input BPC               : {}\n", if vba.NonsupportedDSCInputBPC { "Not Supported" } else { "Supported" });
        dml_print!("DML SUPPORT:     HostVMEnable                : {}\n", vba.HostVMEnable);
        dml_print!("DML SUPPORT:     ImmediateFlipSupported      : [{}, {}]\n", vba.ImmediateFlipSupportedForState[i][0], vba.ImmediateFlipSupportedForState[i][1]);
        i -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
