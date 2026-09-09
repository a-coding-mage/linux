/* Translated from dcn32_resource_helpers.c. External types and functions are supplied by dependencies. */

const MAX_STRETCHED_V_BLANK: u32 = 1000;
const V_SCALE: u32 = 10000 / MAX_STRETCHED_V_BLANK;

unsafe fn is_dual_plane(format: surface_pixel_format) -> bool {
    format >= SURFACE_PIXEL_FORMAT_VIDEO_BEGIN || format == SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA
}

pub unsafe fn dcn32_helper_calculate_mall_bytes_for_cursor(dc: *mut dc, pipe_ctx: *mut pipe_ctx, ignore_cursor_buf: bool) -> u32 {
    let _ = dc;
    let hubp = (*pipe_ctx).plane_res.hubp;
    let mut cursor_size = (*hubp).curs_attr.pitch * (*hubp).curs_attr.height;
    let mut cursor_mall_size_bytes = 0;
    match (*(*pipe_ctx).stream).cursor_attributes.color_format {
        CURSOR_MODE_MONO => cursor_size /= 2,
        CURSOR_MODE_COLOR_1BIT_AND | CURSOR_MODE_COLOR_PRE_MULTIPLIED_ALPHA | CURSOR_MODE_COLOR_UN_PRE_MULTIPLIED_ALPHA => cursor_size *= 4,
        CURSOR_MODE_COLOR_64BIT_FP_PRE_MULTIPLIED | CURSOR_MODE_COLOR_64BIT_FP_UN_PRE_MULTIPLIED => cursor_size *= 8,
        _ => {}
    }
    if (*(*pipe_ctx).stream).cursor_position.enable && (ignore_cursor_buf || cursor_size > 16384) {
        cursor_mall_size_bytes = ((cursor_size + DCN3_2_MALL_MBLK_SIZE_BYTES - 1) / DCN3_2_MALL_MBLK_SIZE_BYTES + 1) * DCN3_2_MALL_MBLK_SIZE_BYTES;
    }
    cursor_mall_size_bytes
}

pub unsafe fn dcn32_helper_calculate_num_ways_for_subvp(dc: *mut dc, context: *mut dc_state) -> u32 {
    if (*context).bw_ctx.bw.dcn.mall_subvp_size_bytes > 0 {
        if (*dc).debug.force_subvp_num_ways != 0 { (*dc).debug.force_subvp_num_ways }
        else if let Some(f) = (*(*dc).res_pool).funcs.calculate_mall_ways_from_bytes { f(dc, (*context).bw_ctx.bw.dcn.mall_subvp_size_bytes) }
        else { 0 }
    } else { 0 }
}

pub unsafe fn dcn32_merge_pipes_for_subvp(dc: *mut dc, context: *mut dc_state) {
    for i in 0..(*(*dc).res_pool).pipe_count {
        let pipe = &mut (*context).res_ctx.pipe_ctx[i as usize];
        if !pipe.prev_odm_pipe.is_null() {
            (*pipe.prev_odm_pipe).next_odm_pipe = pipe.next_odm_pipe;
            if !pipe.next_odm_pipe.is_null() { (*pipe.next_odm_pipe).prev_odm_pipe = pipe.prev_odm_pipe; }
            pipe.bottom_pipe = std::ptr::null_mut(); pipe.next_odm_pipe = std::ptr::null_mut(); pipe.plane_state = std::ptr::null_mut(); pipe.stream = std::ptr::null_mut(); pipe.top_pipe = std::ptr::null_mut(); pipe.prev_odm_pipe = std::ptr::null_mut();
            if !pipe.stream_res.dsc.is_null() { dcn20_release_dsc(&mut (*context).res_ctx, (*dc).res_pool, &mut pipe.stream_res.dsc); }
            std::ptr::write_bytes(&mut pipe.plane_res, 0, 1); std::ptr::write_bytes(&mut pipe.stream_res, 0, 1);
        } else if !pipe.top_pipe.is_null() && (*pipe.top_pipe).plane_state == pipe.plane_state {
            let top = pipe.top_pipe; let bottom = pipe.bottom_pipe;
            (*top).bottom_pipe = bottom; if !bottom.is_null() { (*bottom).top_pipe = top; }
            pipe.top_pipe = std::ptr::null_mut(); pipe.bottom_pipe = std::ptr::null_mut(); pipe.plane_state = std::ptr::null_mut(); pipe.stream = std::ptr::null_mut();
            std::ptr::write_bytes(&mut pipe.plane_res, 0, 1); std::ptr::write_bytes(&mut pipe.stream_res, 0, 1);
        }
    }
}

pub unsafe fn dcn32_all_pipes_have_stream_and_plane(dc: *mut dc, context: *mut dc_state) -> bool {
    for i in 0..(*(*dc).res_pool).pipe_count { let p = &(*context).res_ctx.pipe_ctx[i as usize]; if !p.stream.is_null() && p.plane_state.is_null() { return false; } } true
}
pub unsafe fn dcn32_subvp_in_use(dc: *mut dc, context: *mut dc_state) -> bool {
    for i in 0..(*(*dc).res_pool).pipe_count { let p = &mut (*context).res_ctx.pipe_ctx[i as usize]; if dc_state_get_pipe_subvp_type(context, p) != SUBVP_NONE { return true; } } false
}
pub unsafe fn dcn32_mpo_in_use(context: *mut dc_state) -> bool { for i in 0..(*context).stream_count { if (*context).stream_status[i as usize].plane_count > 1 { return true; } } false }
pub unsafe fn dcn32_any_surfaces_rotated(dc: *mut dc, context: *mut dc_state) -> bool { for i in 0..(*(*dc).res_pool).pipe_count { let p=&(*context).res_ctx.pipe_ctx[i as usize]; if !p.stream.is_null() && !p.plane_state.is_null() && (*p.plane_state).rotation != ROTATION_ANGLE_0 { return true; } } false }
pub unsafe fn dcn32_is_center_timing(pipe: *mut pipe_ctx) -> bool { if pipe.is_null() || (*pipe).stream.is_null() { return false } let s=&*(*pipe).stream; let mut r=s.timing.v_addressable != s.dst.height || s.timing.v_addressable != s.src.height; if !(*pipe).plane_state.is_null() { let p=&*(*pipe).plane_state; r |= s.timing.v_addressable != p.dst_rect.height && s.timing.v_addressable != p.src_rect.height; } r }
pub unsafe fn dcn32_is_psr_capable(pipe: *mut pipe_ctx) -> bool { !pipe.is_null() && !(*pipe).stream.is_null() && (*(*(*pipe).stream).link).psr_settings.psr_version != DC_PSR_VERSION_UNSUPPORTED }

unsafe fn override_det_for_subvp(dc: *mut dc, context: *mut dc_state, pipe_segments: &mut [u8; MAX_PIPES]) {
    let mut fhd_count=0u8; let mut high=0u8; let mut streams=0u8;
    for i in 0..(*context).stream_count { if (*context).stream_status[i as usize].plane_count > 1 { return; } if dc_state_get_stream_subvp_type(context, (*context).streams[i as usize]) != SUBVP_PHANTOM { streams+=1; } }
    for i in 0..(*(*dc).res_pool).pipe_count { let p=&(*context).res_ctx.pipe_ctx[i as usize]; if !p.stream.is_null() && !p.plane_state.is_null() && dc_state_get_pipe_subvp_type(context,p)!=SUBVP_PHANTOM && dcn32_allow_subvp_high_refresh_rate(dc,context,p) { if (*p.stream).timing.v_addressable==1080 && (*p.stream).timing.h_addressable==1920 { fhd_count+=1; } high+=1; } }
    if streams==2 && high==2 && fhd_count==1 { for i in 0..(*(*dc).res_pool).pipe_count { let p=&(*context).res_ctx.pipe_ctx[i as usize]; if !p.stream.is_null() && !p.plane_state.is_null() && dc_state_get_pipe_subvp_type(context,p)!=SUBVP_PHANTOM && (*p.stream).timing.v_addressable==1080 && (*p.stream).timing.h_addressable==1920 && pipe_segments[i as usize]>4 { pipe_segments[i as usize]=4; } } }
}

pub unsafe fn dcn32_determine_det_override(dc:*mut dc, context:*mut dc_state, pipes:*mut display_e2e_pipe_params_st) {
    let mut seg=[0u8;MAX_PIPES]; let mut counted=[0u8;MAX_PIPES]; let mut stream_count=0u8;
    for i in 0..(*context).stream_count { if dc_state_get_stream_subvp_type(context,(*context).streams[i as usize])!=SUBVP_PHANTOM { stream_count+=1; } }
    if stream_count>0 { let stream_seg=18/stream_count; for i in 0..(*context).stream_count { let st=(*context).streams[i as usize]; if dc_state_get_stream_subvp_type(context,st)==SUBVP_PHANTOM {continue} let planes=(*context).stream_status[i as usize].plane_count; let plane_seg=if planes>0 {stream_seg/planes} else {stream_seg}; for j in 0..(*(*dc).res_pool).pipe_count { if (*context).res_ctx.pipe_ctx[j as usize].stream==st && counted[j as usize]!=1 { counted[j as usize]=1; let plane=(*context).res_ctx.pipe_ctx[j as usize].plane_state; let mut n=1u8; for k in 0..(*(*dc).res_pool).pipe_count { if k!=j && (*context).res_ctx.pipe_ctx[k as usize].stream==st && (*context).res_ctx.pipe_ctx[k as usize].plane_state==plane { counted[k as usize]=1;n+=1; } } seg[j as usize]=plane_seg/n; for k in 0..(*(*dc).res_pool).pipe_count { if k!=j && (*context).res_ctx.pipe_ctx[k as usize].stream==st && (*context).res_ctx.pipe_ctx[k as usize].plane_state==plane {seg[k as usize]=plane_seg/n;} } } } } override_det_for_subvp(dc,context,&mut seg); let mut n=0; for i in 0..(*(*dc).res_pool).pipe_count { if !(*context).res_ctx.pipe_ctx[i as usize].stream.is_null() { (*pipes.add(n)).pipe.src.det_size_override=seg[i as usize] as u32*DCN3_2_DET_SEG_SIZE;n+=1; } } } else { for i in 0..(*(*dc).res_pool).pipe_count { (*pipes.add(i as usize)).pipe.src.det_size_override=4*DCN3_2_DET_SEG_SIZE; } }
}

pub unsafe fn dcn32_set_det_allocations(dc:*mut dc, context:*mut dc_state, pipes:*mut display_e2e_pipe_params_st) { let mut count=0; let mut pipe=std::ptr::null_mut(); let disable=(*dc).debug.disable_z9_mpc||(*dc).debug.disable_unbounded_requesting; for i in 0..(*(*dc).res_pool).pipe_count { let p=&mut (*context).res_ctx.pipe_ctx[i as usize]; if !p.stream.is_null(){pipe=p;count+=1;} } if count==1 { (*pipes).pipe.src.det_size_override=DCN3_2_MAX_DET_SIZE; if !pipe.is_null() && !(*pipe).plane_state.is_null() && !disable && (*pipe).plane_state.tiling_info.gfx9.swizzle!=DC_SW_LINEAR && !is_dual_plane((*pipe).plane_state.format) { (*pipes).pipe.src.det_size_override=DCN3_2_DEFAULT_DET_SIZE; (*pipes).pipe.src.unbounded_req_mode=true; if (*pipe).plane_state.src_rect.width>=5120 && (*pipe).plane_state.src_rect.height>=2880 {(*pipes).pipe.src.det_size_override=320;} } } else {dcn32_determine_det_override(dc,context,pipes);} }

unsafe fn get_frame_rate_at_max_stretch_100hz(s:*mut dc_stream_state, margin:u32)->i32 { if s.is_null(){return 0} let t=&(*s).timing; let scale=10000/(MAX_STRETCHED_V_BLANK+margin); let sec=t.pix_clk_100hz/t.h_total+1; let max=sec/scale+1; let curr=t.v_total-t.v_addressable; let stretch=if max>curr{max-curr}else{0}; let pixels=(stretch+t.v_total)*t.h_total; t.pix_clk_100hz/pixels/10000+1 }
unsafe fn is_refresh_rate_support_mclk_switch_using_fw_based_vblank_stretch(s:*mut dc_stream_state,margin:u32,current:i32)->bool { if s.is_null(){return false} let max=get_frame_rate_at_max_stretch_100hz(s,margin); let min=(*s).timing.min_refresh_in_uhz/10000; if max<min{return false} if (*(*s).ctx).dc.config.enable_fpo_flicker_detection==1 && !dc_stream_is_refresh_rate_range_flickerless(s,(max/100),current,false){return false} true }
unsafe fn get_refresh_rate(s:*mut dc_stream_state)->i32 { if s.is_null(){return 0} let t=&(*s).timing; let total=t.h_total*t.v_total; if total==0{0}else{(t.pix_clk_100hz*100/total)+1} }

pub unsafe fn dcn32_can_support_mclk_switch_using_fw_based_vblank_stretch(dc:*mut dc,context:*mut dc_state)->*mut dc_stream_state { if context.is_null()||(*dc).debug.disable_fams||!(*dc).caps.dmub_caps.mclk_sw||(*context).bw_ctx.bw.dcn.clk.fw_based_mclk_switching_shut_down{return std::ptr::null_mut()} if (*context).stream_count>2{return std::ptr::null_mut()} let mut s=if (*context).stream_count==1{(*context).streams[0]}else{let mut x=std::ptr::null_mut(); dcn32_assign_fpo_vactive_candidate(dc,context,&mut x); x}; let status=if !s.is_null(){dc_state_get_stream_status(context,s)}else{std::ptr::null_mut()}; let active=if (*context).stream_count==2{dcn32_find_vactive_pipe(dc,context,s,(*dc).debug.fpo_vactive_min_active_margin_us)}else{false}; if (*context).stream_count==2 && (!active||(*dc).debug.disable_fpo_vactive){return std::ptr::null_mut()} if s.is_null()||status.is_null()||(*status).plane_count==0||(*(*s).sink).edid_caps.panel_patch.disable_fams{return std::ptr::null_mut()} let rr=get_refresh_rate(s); if rr<120||!is_refresh_rate_support_mclk_switch_using_fw_based_vblank_stretch(s,if active{(*dc).debug.fpo_vactive_margin_us}else{0},rr)||!(*s).allow_freesync{return std::ptr::null_mut()} if (*s).vrr_active_variable && ((*dc).debug.disable_fams_gaming==INGAME_FAMS_DISABLE || ((*context).stream_count>1 && (*dc).debug.disable_fams_gaming!=INGAME_FAMS_MULTI_DISP_ENABLE)){return std::ptr::null_mut()} s }

pub unsafe fn dcn32_check_native_scaling_for_res(pipe:*mut pipe_ctx,width:u32,height:u32)->bool { let s=&*(*pipe).stream; let p=&*(*pipe).plane_state; s.timing.h_addressable==width&&s.timing.v_addressable==height&&p.src_rect.width==width&&p.src_rect.height==height&&p.dst_rect.width==width&&p.dst_rect.height==height }

unsafe fn disallow_subvp_in_active_plus_blank(pipe:*mut pipe_ctx)->bool { resource_is_pipe_type(pipe,OPP_HEAD)&&resource_is_pipe_type(pipe,DPP_PIPE)&&(*(*pipe).stream).timing.v_addressable==1080&&(*(*pipe).stream).timing.h_addressable==1920 }

pub unsafe fn dcn32_subvp_drr_admissable(dc:*mut dc,context:*mut dc_state)->bool { let mut sub=0u8;let mut non=0u8;let mut found=false;let mut psr=false;let mut rr=0u64;let mut dis=false;for i in 0..(*(*dc).res_pool).pipe_count{let p=&mut (*context).res_ctx.pipe_ctx[i as usize];let typ=dc_state_get_pipe_subvp_type(context,p);if resource_is_pipe_type(p,OPP_HEAD)&&resource_is_pipe_type(p,DPP_PIPE){if typ==SUBVP_MAIN{sub+=1;dis|=disallow_subvp_in_active_plus_blank(p);rr=div_u64(div_u64((*p.stream).timing.pix_clk_100hz*100+(*p.stream).timing.v_total as u64*(*p.stream).timing.h_total as u64-1,(*p.stream).timing.v_total),(*p.stream).timing.h_total)}if typ==SUBVP_NONE{non+=1;psr|=dcn32_is_psr_capable(p);found|=(*p.stream).ignore_msa_timing_param&&((*p.stream).allow_freesync||(*p.stream).vrr_active_variable||(*p.stream).vrr_active_fixed)}}}sub==1&&!dis&&non==1&&found&&!psr&&rr<120}

pub unsafe fn dcn32_subvp_vblank_admissable(dc:*mut dc,context:*mut dc_state,vlevel:i32)->bool { let mut sub=0u8;let mut non=0u8;let mut found=false;let mut psr=false;let mut rr=0u64;let mut dis=false;for i in 0..(*(*dc).res_pool).pipe_count{let p=&mut (*context).res_ctx.pipe_ctx[i as usize];let typ=dc_state_get_pipe_subvp_type(context,p);if resource_is_pipe_type(p,OPP_HEAD)&&resource_is_pipe_type(p,DPP_PIPE){if typ==SUBVP_MAIN{sub+=1;dis|=disallow_subvp_in_active_plus_blank(p);rr=div_u64(div_u64((*p.stream).timing.pix_clk_100hz*100+(*p.stream).timing.v_total as u64*(*p.stream).timing.h_total as u64-1,(*p.stream).timing.v_total),(*p.stream).timing.h_total)}if typ==SUBVP_NONE{non+=1;psr|=dcn32_is_psr_capable(p);found|=(*p.stream).ignore_msa_timing_param&&((*p.stream).allow_freesync||(*p.stream).vrr_active_variable||(*p.stream).vrr_active_fixed)}}}let v=&(*context).bw_ctx.dml.vba;sub==1&&non==1&&!found&&!psr&&rr<120&&!dis&&v.DRAMClockChangeSupport[vlevel as usize][v.maxMpcComb as usize]==dm_dram_clock_change_vblank_w_mall_sub_vp}

pub unsafe fn dcn32_update_dml_pipes_odm_policy_based_on_context(dc:*mut dc,context:*mut dc_state,pipes:*mut display_e2e_pipe_params_st){let mut n=0;for i in 0..(*(*dc).res_pool).pipe_count{let p=&mut (*context).res_ctx.pipe_ctx[i as usize];if p.stream.is_null(){continue}match resource_get_odm_slice_count(p){1=>(*pipes.add(n)).pipe.dest.odm_combine_policy=dm_odm_combine_policy_dal,2=>(*pipes.add(n)).pipe.dest.odm_combine_policy=dm_odm_combine_policy_2to1,4=>(*pipes.add(n)).pipe.dest.odm_combine_policy=dm_odm_combine_policy_4to1,_=>{}}n+=1;}}
pub unsafe fn dcn32_override_min_req_dcfclk(dc:*mut dc,context:*mut dc_state){if dcn32_subvp_in_use(dc,context)&&(*context).bw_ctx.bw.dcn.clk.dcfclk_khz<=MIN_SUBVP_DCFCLK_KHZ{(*context).bw_ctx.bw.dcn.clk.dcfclk_khz=MIN_SUBVP_DCFCLK_KHZ;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
