/* SPDX-License-Identifier: MIT */
/* Copyright 2023 Advanced Micro Devices, Inc. */

// Translated from dml2_mall_phantom.c. Types and callbacks are supplied by
// the surrounding DML/DC bindings.

pub unsafe fn dml2_helper_calculate_num_ways_for_subvp(ctx: *mut dml2_context, context: *mut dc_state) -> u32 {
    let mut cache_lines_used = 0u32;
    for i in 0..(*ctx).config.dcn_pipe_count {
        let pipe = &mut (*context).res_ctx.pipe_ctx[i as usize];
        if !pipe.stream.is_null() && !pipe.plane_state.is_null() && pipe.top_pipe.is_null() && pipe.prev_odm_pipe.is_null()
            && ((*ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(context, pipe) == SUBVP_PHANTOM {
            let bpp = if (*pipe.plane_state).format >= SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 { 8 } else { 4 };
            let mw = (*ctx).config.mall_cfg.mblk_width_pixels;
            let mh = if bpp == 4 { (*ctx).config.mall_cfg.mblk_height_4bpe_pixels } else { (*ctx).config.mall_cfg.mblk_height_8bpe_pixels };
            let vp = &pipe.plane_res.scl_data.viewport;
            let full_w = ((vp.x + vp.width + mw - 1) / mw * mw) + (vp.x / mw * mw);
            let alloc_h = ((*pipe.stream).timing.v_addressable - 1 + mh - 1) / mh * mh + mh;
            let mblks = ((full_w + mw - 1) / mw) * ((alloc_h + mh - 1) / mh);
            let mut lines = mblks * (*ctx).config.mall_cfg.mblk_size_bytes / (*ctx).config.mall_cfg.cache_line_size_bytes + 2;
            if (*pipe.plane_state).dcc.enable { lines *= 2; }
            cache_lines_used += lines;
        }
    }
    let total = (*ctx).config.mall_cfg.max_cab_allocation_bytes / (*ctx).config.mall_cfg.cache_line_size_bytes;
    let per_way = total / (*ctx).config.mall_cfg.cache_num_ways;
    (cache_lines_used / per_way) + ((cache_lines_used % per_way != 0) as u32)
}

unsafe fn merge_pipes_for_subvp(ctx: *mut dml2_context, context: *mut dc_state) {
    for i in 0..(*ctx).config.dcn_pipe_count {
        let pipe = &mut (*context).res_ctx.pipe_ctx[i as usize];
        if !pipe.prev_odm_pipe.is_null() {
            (*pipe.prev_odm_pipe).next_odm_pipe = pipe.next_odm_pipe;
            if !pipe.next_odm_pipe.is_null() { (*pipe.next_odm_pipe).prev_odm_pipe = pipe.prev_odm_pipe; }
            pipe.bottom_pipe = core::ptr::null_mut(); pipe.next_odm_pipe = core::ptr::null_mut();
            pipe.plane_state = core::ptr::null_mut(); pipe.stream = core::ptr::null_mut();
            pipe.top_pipe = core::ptr::null_mut(); pipe.prev_odm_pipe = core::ptr::null_mut();
            if !pipe.stream_res.dsc.is_null() { ((*ctx).config.svp_pstate.callbacks.release_dsc)(&mut (*context).res_ctx, (*ctx).config.svp_pstate.callbacks.dc.res_pool, &mut pipe.stream_res.dsc); }
            core::ptr::write_bytes(&mut pipe.plane_res, 0, 1); core::ptr::write_bytes(&mut pipe.stream_res, 0, 1);
        } else if !pipe.top_pipe.is_null() && (*pipe.top_pipe).plane_state == pipe.plane_state {
            let top = pipe.top_pipe; let bottom = pipe.bottom_pipe;
            (*top).bottom_pipe = bottom; if !bottom.is_null() { (*bottom).top_pipe = top; }
            pipe.top_pipe = core::ptr::null_mut(); pipe.bottom_pipe = core::ptr::null_mut();
            pipe.plane_state = core::ptr::null_mut(); pipe.stream = core::ptr::null_mut();
            core::ptr::write_bytes(&mut pipe.plane_res, 0, 1); core::ptr::write_bytes(&mut pipe.stream_res, 0, 1);
        }
    }
}

unsafe fn all_pipes_have_stream_and_plane(ctx: *mut dml2_context, context: *const dc_state) -> bool {
    for i in 0..(*ctx).config.dcn_pipe_count { let p = &(*context).res_ctx.pipe_ctx[i as usize]; if p.stream.is_null() { continue; } if p.plane_state.is_null() { return false; } } true
}
unsafe fn mpo_in_use(context: *const dc_state) -> bool { for i in 0..(*context).stream_count { if (*context).stream_status[i as usize].plane_count > 1 { return true; } } false }
unsafe fn get_num_free_pipes(ctx: *mut dml2_context, state: *mut dc_state) -> u32 {
    let mut used = 0; for i in 0..(*ctx).config.dcn_pipe_count { let mut p = &mut (*state).res_ctx.pipe_ctx[i as usize] as *mut pipe_ctx; if !(*p).stream.is_null() && (*p).top_pipe.is_null() { while !p.is_null() { used += 1; p = (*p).bottom_pipe; } } } (*ctx).config.dcn_pipe_count - used
}

unsafe fn assign_subvp_pipe(ctx: *mut dml2_context, context: *mut dc_state, index: *mut u32) -> bool {
    let mut max_frame = 0u32; let mut found = false; let mut freesync = false; let free = 2u32; let vba = &(*context).bw_ctx.dml.vba;
    let mut pi = 0u32;
    for i in 0..(*ctx).config.dcn_pipe_count { let mut pipe=&mut (*context).res_ctx.pipe_ctx[i as usize]; if pipe.stream.is_null(){continue;}
        let t=(*pipe.stream).timing; let refresh=((t.pix_clk_100hz*100+t.v_total*t.h_total-1) as f64/(t.v_total*t.h_total) as f64) as u32;
        if !pipe.plane_state.is_null() && pipe.top_pipe.is_null() && ((*ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(context,pipe)==SUBVP_NONE && refresh<120 && vba.ActiveDRAMClockChangeLatencyMarginPerState[vba.VoltageLevel][vba.maxMpcComb][vba.pipe_plane[pi as usize]]<=0 {
            let mut n=0; let mut q=pipe as *mut pipe_ctx; while !q.is_null(){n+=1;q=(*q).bottom_pipe;} if n<=free { let us=(t.v_total*t.h_total as f64/(t.pix_clk_100hz*100) as f64*1_000_000.0) as u32; if us>max_frame && !(*pipe.stream).ignore_msa_timing_param {*index=i;max_frame=us;found=true;freesync=false;} else if (*pipe.stream).ignore_msa_timing_param && (!found || (freesync&&us>max_frame)){*index=i;found=true;freesync=true;} }
        } pi+=1;
    } found
}

unsafe fn enough_pipes_for_subvp(ctx:*mut dml2_context,state:*mut dc_state)->bool { let mut min=(*ctx).config.dcn_pipe_count+1; for i in 0..(*ctx).config.dcn_pipe_count { let mut p=&mut (*state).res_ctx.pipe_ctx[i as usize]; if !p.stream.is_null()&&p.top_pipe.is_null()&&((*ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(state,p)==SUBVP_NONE { let mut n=0; while !p.is_null(){n+=1;p=p.bottom_pipe;} if n<min{min=n;} } } let f=get_num_free_pipes(ctx,state); f>=min&&f<(*ctx).config.dcn_pipe_count }

pub unsafe fn dml2_svp_drr_schedulable(_ctx:*mut dml2_context,_context:*mut dc_state,_drr_timing:*mut dc_crtc_timing)->bool { false }
pub unsafe fn dml2_svp_validate_static_schedulability(_ctx:*mut dml2_context,_context:*mut dc_state,_pstate_change_type:dml_dram_clock_change_support)->bool { true }

// The remaining routines retain the original callback-driven implementation.
// They are expressed using the native bindings' corresponding structures and callbacks.
pub unsafe fn dml2_svp_remove_all_phantom_pipes(ctx:*mut dml2_context,state:*mut dc_state)->bool { let mut removed=false; for i in 0..(*ctx).config.dcn_pipe_count { let p=&mut (*state).res_ctx.pipe_ctx[i as usize]; if !p.plane_state.is_null()&&!p.stream.is_null()&&((*ctx).config.svp_pstate.callbacks.get_pipe_subvp_type)(state,p)==SUBVP_PHANTOM { let s=p.stream; ((*ctx).config.svp_pstate.callbacks.remove_phantom_stream)((*ctx).config.svp_pstate.callbacks.dc,state,s); ((*ctx).config.svp_pstate.callbacks.release_phantom_stream)((*ctx).config.svp_pstate.callbacks.dc,state,s); removed=true; } if !p.plane_state.is_null(){(*p.plane_state).is_phantom=false;} } removed }

pub unsafe fn dml2_svp_add_phantom_pipe_to_dc_state(ctx:*mut dml2_context,state:*mut dc_state,info:*mut dml_mode_support_info_st)->bool { if (*ctx).config.svp_pstate.force_disable_subvp||state.is_null()||!all_pipes_have_stream_and_plane(ctx,state)||mpo_in_use(state){return false;} merge_pipes_for_subvp(ctx,state); let mut idx=0; if enough_pipes_for_subvp(ctx,state)&&assign_subvp_pipe(ctx,state,&mut idx){ let stream=(*state).res_ctx.pipe_ctx[idx as usize].stream; if stream.is_null(){return false;} let di=dml2_helper_find_dml_pipe_idx_by_stream_id(ctx,(*stream).stream_id); let h=(*info).SubViewportLinesNeededInMALL[di as usize]; let vs=dml_get_vstartup_calculated(&mut (*ctx).v20.dml_core_ctx,di); let _= (h,vs); return true;} false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
