// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by dc.h and dcn42_resource_fpu.h are intentionally
// referenced but not reimplemented here.

// #define DC_LOGGER_INIT(logger)

pub unsafe fn dcn42_decide_zstate_support(
    dc: *mut dc,
    context: *mut dc_state,
) {
    let mut support: dcn_zstate_support_state = DCN_ZSTATE_SUPPORT_DISALLOW;
    let mut i: u32;
    let mut plane_count: u32 = 0;

    // DC_LOGGER_INIT((*dc).ctx.logger);

    dc_assert_fp_enabled();
    i = 0;
    while i < (*(*dc).res_pool).pipe_count {
        if !(*(*context).res_ctx.pipe_ctx[i as usize]).plane_state.is_null() {
            plane_count += 1;
        }
        i += 1;
    }
    /* dcn42 has no z10 */
    if (*context).stream_count == 0 || plane_count == 0 {
        support = DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY;
    } else if (*context).stream_count == 1
        && (*(*context).streams[0]).signal == SIGNAL_TYPE_EDP
    {
        let link: *mut dc_link = (*(*(*context).streams[0]).sink).link;
        let is_psr = !link.is_null()
            && (((*link).psr_settings.psr_version == DC_PSR_VERSION_1)
                || ((*link).psr_settings.psr_version == DC_PSR_VERSION_SU_1))
            && !(*link).panel_config.psr.disable_psr;
        let is_replay = !link.is_null() && (*link).replay_settings.replay_feature_enabled;

        if is_psr || is_replay {
            support = DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY;
        } else {
            /* here we allow z8 for eDP based on dml21 output */
            support = if (*context).bw_ctx.bw.dcn.clk.zstate_support {
                DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY
            } else {
                DCN_ZSTATE_SUPPORT_DISALLOW
            };
        }

        DC_LOG_SMU(
            "zstate_support: %d, StutterPeriod: %d\n, z8_stutter_efficiency: %d\n",
            support,
            (*context).bw_ctx.bw.dcn.clk.stutter_efficiency.z8_stutter_period as i32,
            (*context).bw_ctx.bw.dcn.clk.stutter_efficiency.z8_stutter_efficiency as i32,
        );
    }
    (*context).bw_ctx.bw.dcn.clk.zstate_support = support;
}

pub unsafe fn dcn42_decide_odm_override(
    dc: *mut dc,
    context: *mut dc_state,
) -> bool {
    let mut odm_override = false;

    // DC_LOGGER_INIT((*dc).ctx.logger);
    if (*(*dc).ctx).dce_environment == DCE_ENV_DIAG {
        return false;
    }

    if (*context).stream_count == 1
        && (*(*context).streams[0]).signal == SIGNAL_TYPE_EDP
    {
        if (*dc).debug.force_odm2to1_for_edp_pixclk_mhz != 0
            && (*(*context).streams[0]).timing.pix_clk_100hz
                > (*dc).debug.force_odm2to1_for_edp_pixclk_mhz * 10000
        {
            odm_override = true;
            (*(*context).streams[0]).debug.force_odm_combine_segments = 2;
        }
        DC_LOG_SMU(
            "odm_override: %d, eDP pixelclock: %d, force_odm2to1_for_edp_pixclk_mhz: %d\n",
            odm_override,
            (*(*context).streams[0]).timing.pix_clk_100hz / 10000,
            (*dc).debug.force_odm2to1_for_edp_pixclk_mhz,
        );
    }
    odm_override
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
