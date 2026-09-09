/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies are supplied by the surrounding translation unit.

/*
 * dcn42b_init_pipes - Initialize pipes for dcn42b
 *
 * This function is modeled after dcn10_init_pipes but handles the case
 * where num_timing_generator != num_pipes (e.g., 3 TGs but 4 pipes).
 *
 * For dcn42b:
 * - num_timing_generator = 3
 * - num_pipes (num_dpp) = 4
 *
 * The key difference is that we iterate over timing generators separately
 * from pipes to avoid accessing timing_generators[i] when i >= num_timing_generator.
 */
pub unsafe fn dcn42b_init_pipes(dc: *mut dc, context: *mut dc_state) {
    let mut i: u8;
    let hws: *mut dce_hwseq = (*dc).hwseq;
    let hubbub: *mut hubbub = (*(*dc).res_pool).hubbub;
    let mut can_apply_seamless_boot = false;
    let mut tg_enabled = [false; MAX_PIPES];

    i = 0;
    while i < (*context).stream_count {
        if (*(*context).streams[i as usize]).apply_seamless_boot_optimization {
            can_apply_seamless_boot = true;
            break;
        }
        i += 1;
    }

    i = 0;
    while i < (*(*dc).res_pool).timing_generator_count {
        let tg: *mut timing_generator = (*(*dc).res_pool).timing_generators[i as usize];
        let pipe_ctx: *mut pipe_ctx = &mut (*(*context).res_ctx.pipe_ctx)[i as usize];

        /* There is assumption that pipe_ctx is not mapping irregularly
         * to non-preferred front end. If pipe_ctx->stream is not NULL,
         * we will use the pipe, so don't disable
         */
        if !(*pipe_ctx).stream.is_null() && can_apply_seamless_boot { i += 1; continue; }

        /* Blank controller using driver code instead of command table. */
        if ((*(*tg).funcs).is_tg_enabled)(tg) {
            if !(*hws).funcs.init_blank.is_none() {
                ((*(*hws).funcs).init_blank.unwrap())(dc, tg);
                ((*(*tg).funcs).lock)(tg);
            } else {
                ((*(*tg).funcs).lock)(tg);
                ((*(*tg).funcs).set_blank)(tg, true);
                hwss_wait_for_blank_complete(tg);
            }
        }
        i += 1;
    }

    /* Reset det size */
    i = 0;
    while i < (*(*dc).res_pool).pipe_count {
        let pipe_ctx: *mut pipe_ctx = &mut (*(*context).res_ctx.pipe_ctx)[i as usize];
        let hubp: *mut hubp = (*(*dc).res_pool).hubps[i as usize];
        if !(*pipe_ctx).stream.is_null() && can_apply_seamless_boot { i += 1; continue; }
        if !hubbub.is_null() && !hubp.is_null() {
            if let Some(f) = (*(*hubbub).funcs).program_det_size { f(hubbub, (*hubp).inst, 0); }
            if let Some(f) = (*(*hubbub).funcs).program_det_segments { f(hubbub, (*hubp).inst, 0); }
        }
        i += 1;
    }

    /* num_opp will be equal to number of mpcc */
    i = 0;
    while i < (*(*dc).res_pool).res_cap.num_opp {
        let pipe_ctx: *mut pipe_ctx = &mut (*(*context).res_ctx.pipe_ctx)[i as usize];
        if !(*pipe_ctx).stream.is_null() && can_apply_seamless_boot { i += 1; continue; }
        ((*(*(*dc).res_pool).mpc).funcs.mpc_init_single_inst)((*(*dc).res_pool).mpc, i);
        i += 1;
    }

    /* initialize DWB pointer to MCIF_WB */
    i = 0;
    while i < (*(*dc).res_pool).res_cap.num_dwb {
        (*(*dc).res_pool).dwbc[i as usize].mcif = (*(*dc).res_pool).mcif_wb[i as usize];
        i += 1;
    }

    i = 0;
    while i < (*(*dc).res_pool).timing_generator_count {
        let tg = (*(*dc).res_pool).timing_generators[i as usize];
        let hubp = (*(*dc).res_pool).hubps[i as usize];
        let dpp = (*(*dc).res_pool).dpps[i as usize];
        let mut pipe_ctx = &mut (*(*context).res_ctx.pipe_ctx)[i as usize] as *mut pipe_ctx;

        /* There is assumption that pipe_ctx is not mapping irregularly
         * to non-preferred front end. If pipe_ctx->stream is not NULL,
         * we will use the pipe, so don't disable
         */
        if can_apply_seamless_boot && !(*pipe_ctx).stream.is_null() &&
            ((*(*(*pipe_ctx).stream_res.tg).funcs).is_tg_enabled)((*pipe_ctx).stream_res.tg) {
            // Enable double buffering for OTG_BLANK no matter if seamless boot is enabled or not to suppress global sync
            // signals when OTG blanked. This is to prevent pipe from requesting data while in PSR.
            ((*(*tg).funcs).tg_init)(tg);
            (*hubp).power_gated = true;
            tg_enabled[i as usize] = true;
            i += 1;
            continue;
        }

        /* Disable on the current state so the new one isn't cleared. */
        pipe_ctx = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize];
        ((*(*hubp).funcs).hubp_reset)(hubp);
        ((*(*dpp).funcs).dpp_reset)(dpp);
        (*pipe_ctx).stream_res.tg = tg;
        (*pipe_ctx).pipe_idx = i;
        (*pipe_ctx).plane_res.hubp = hubp;
        (*pipe_ctx).plane_res.dpp = dpp;
        (*pipe_ctx).plane_res.mpcc_inst = (*dpp).inst as u8;
        (*hubp).mpcc_id = (*dpp).inst;
        (*hubp).opp_id = OPP_ID_INVALID;
        (*hubp).power_gated = false;
        (*(*dc).res_pool).opps[i as usize].mpc_tree_params.opp_id = (*(*dc).res_pool).opps[i as usize].inst;
        (*(*dc).res_pool).opps[i as usize].mpc_tree_params.opp_list = core::ptr::null_mut();
        (*(*dc).res_pool).opps[i as usize].mpcc_disconnect_pending[(*pipe_ctx).plane_res.mpcc_inst as usize] = true;
        (*pipe_ctx).stream_res.opp = (*(*dc).res_pool).opps[i as usize];
        ((*(*hws).funcs).plane_atomic_disconnect)(dc, context, pipe_ctx);
        if ((*(*tg).funcs).is_tg_enabled)(tg) { ((*(*tg).funcs).unlock)(tg); }
        ((*(*dc).hwss).disable_plane)(dc, context, pipe_ctx);
        (*pipe_ctx).stream_res.tg = core::ptr::null_mut();
        (*pipe_ctx).plane_res.hubp = core::ptr::null_mut();
        if ((*(*tg).funcs).is_tg_enabled)(tg) { if let Some(f) = (*(*tg).funcs).init_odm { f(tg); } }
        ((*(*tg).funcs).tg_init)(tg);
        i += 1;
    }

    /* Clean up MPC tree */
    i = 0;
    while i < (*(*dc).res_pool).pipe_count {
        if tg_enabled[i as usize] {
            let opp = (*(*dc).res_pool).opps[i as usize];
            if !(*opp).mpc_tree_params.opp_list.is_null() {
                let list = (*opp).mpc_tree_params.opp_list;
                if !(*list).mpcc_bot.is_null() {
                    let bot_id = (*(*list).mpcc_bot).mpcc_id;
                    if bot_id < MAX_MPCC && bot_id < MAX_PIPES && !tg_enabled[bot_id as usize] {
                        (*opp).mpc_tree_params.opp_list = core::ptr::null_mut();
                    }
                }
            }
        }
        i += 1;
    }

    /* Power gate DSCs */
    if !(*hws).funcs.dsc_pg_control.is_none() {
        let mut num_opps = 0;
        let mut opp_id_src0 = OPP_ID_INVALID;
        let mut opp_id_src1 = OPP_ID_INVALID;
        // Step 1: To find out which OPTC is running & OPTC DSC is ON
        // We can't use res_pool->res_cap->num_timing_generator to check
        // Because it records display pipes default setting built in driver,
        // not display pipes of the current chip.
        // Some ASICs would be fused display pipes less than the default setting.
        // In dcnxx_resource_construct function, driver would obatin real information.
        i = 0;
        while i < (*(*dc).res_pool).timing_generator_count {
            let mut optc_dsc_state = 0;
            let tg = (*(*dc).res_pool).timing_generators[i as usize];
            if ((*(*tg).funcs).is_tg_enabled)(tg) {
                if let Some(f) = (*(*tg).funcs).get_dsc_status { f(tg, &mut optc_dsc_state); }
                // Only one OPTC with DSC is ON, so if we got one result, we would exit this block.
                // non-zero value is DSC enabled
                if optc_dsc_state != 0 {
                    ((*(*tg).funcs).get_optc_source)(tg, &mut num_opps, &mut opp_id_src0, &mut opp_id_src1);
                    break;
                }
            }
            i += 1;
        }
        // Step 2: To power down DSC but skip DSC of running OPTC
        i = 0;
        while i < (*(*dc).res_pool).res_cap.num_dsc {
            let dsc = (*(*dc).res_pool).dscs[i as usize];
            let mut s: dcn_dsc_state = core::mem::zeroed();
            ((*(*dsc).funcs).dsc_read_state)(dsc, &mut s);
            if ((*s).dsc_opp_source == opp_id_src0 || (*s).dsc_opp_source == opp_id_src1) && (*s).dsc_clock_en && (*s).dsc_fw_en { i += 1; continue; }
            ((*(*hws).funcs).dsc_pg_control)(hws, (*dsc).inst, false);
            i += 1;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
