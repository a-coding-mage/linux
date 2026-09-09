// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Translated from dcn42_hubbub.c. External types, register helpers, and
// functions are supplied by the surrounding display-core bindings.

const DCN42_CRB_SEGMENT_SIZE_KB: i32 = 64;

unsafe fn hubbub42_program_urgent_watermarks(
    hubbub: *mut hubbub,
    watermarks: *mut dcn_watermark_set,
    safe_to_lower: bool,
) -> bool {
    let hubbub2 = TO_DCN20_HUBBUB(hubbub);
    let mut wm_pending = false;
    macro_rules! wm { ($s:ident, $f:ident, $r:ident) => {
        if safe_to_lower || (*watermarks).dcn4x.$s.$f > (*hubbub2).watermarks.dcn4x.$s.$f {
            (*hubbub2).watermarks.dcn4x.$s.$f = (*watermarks).dcn4x.$s.$f;
            REG_SET!($r, 0, $r, (*watermarks).dcn4x.$s.$f);
        } else if (*watermarks).dcn4x.$s.$f < (*hubbub2).watermarks.dcn4x.$s.$f { wm_pending = true; }
    }}
    wm!(a, urgent, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A);
    wm!(a, frac_urg_bw_flip, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A);
    wm!(a, frac_urg_bw_nom, DCHUBBUB_ARB_FRAC_URG_BW_NOM_A);
    wm!(a, refcyc_per_trip_to_mem, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A);
    wm!(b, urgent, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B);
    wm!(b, frac_urg_bw_flip, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B);
    wm!(b, frac_urg_bw_nom, DCHUBBUB_ARB_FRAC_URG_BW_NOM_B);
    wm!(b, refcyc_per_trip_to_mem, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B);
    wm!(c, urgent, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C);
    wm!(c, frac_urg_bw_flip, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C);
    wm!(c, frac_urg_bw_nom, DCHUBBUB_ARB_FRAC_URG_BW_NOM_C);
    wm!(c, refcyc_per_trip_to_mem, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C);
    wm!(d, urgent, DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D);
    wm!(d, frac_urg_bw_flip, DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D);
    wm!(d, frac_urg_bw_nom, DCHUBBUB_ARB_FRAC_URG_BW_NOM_D);
    wm!(d, refcyc_per_trip_to_mem, DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D);
    wm_pending
}

unsafe fn hubbub42_program_stutter_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, safe_to_lower: bool) -> bool {
    let hubbub2 = TO_DCN20_HUBBUB(hubbub); let mut wm_pending = false;
    macro_rules! wm { ($s:ident, $f:ident, $r:ident) => { if safe_to_lower || (*watermarks).dcn4x.$s.$f > (*hubbub2).watermarks.dcn4x.$s.$f { (*hubbub2).watermarks.dcn4x.$s.$f = (*watermarks).dcn4x.$s.$f; REG_SET!($r, 0, $r, (*watermarks).dcn4x.$s.$f); } else if (*watermarks).dcn4x.$s.$f < (*hubbub2).watermarks.dcn4x.$s.$f { wm_pending = true; } }}
    wm!(a,sr_enter,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A); wm!(a,sr_exit,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A);
    wm!(b,sr_enter,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B); wm!(b,sr_exit,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B);
    wm!(c,sr_enter,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C); wm!(c,sr_exit,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C);
    wm!(d,sr_enter,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D); wm!(d,sr_exit,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D); wm_pending
}

unsafe fn hubbub42_program_pstate_watermarks(hubbub: *mut hubbub, watermarks: *mut dcn_watermark_set, safe_to_lower: bool) -> bool {
    let hubbub2 = TO_DCN20_HUBBUB(hubbub); let mut wm_pending = false;
    macro_rules! wm { ($s:ident,$f:ident,$r:ident) => { if safe_to_lower || (*watermarks).dcn4x.$s.$f > (*hubbub2).watermarks.dcn4x.$s.$f { (*hubbub2).watermarks.dcn4x.$s.$f=(*watermarks).dcn4x.$s.$f; REG_SET!($r,0,$r,(*watermarks).dcn4x.$s.$f); } else if (*watermarks).dcn4x.$s.$f < (*hubbub2).watermarks.dcn4x.$s.$f { wm_pending=true; } }}
    wm!(a,uclk_pstate,DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A); wm!(b,uclk_pstate,DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B); wm!(c,uclk_pstate,DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_C); wm!(d,uclk_pstate,DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_D);
    wm!(a,fclk_pstate,DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A); wm!(b,fclk_pstate,DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B); wm!(c,fclk_pstate,DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_C); wm!(d,fclk_pstate,DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_D); wm_pending
}

unsafe fn hubbub42_program_usr_watermarks(hubbub:*mut hubbub, watermarks:*mut dcn_watermark_set, safe_to_lower:bool)->bool { let h=TO_DCN20_HUBBUB(hubbub); let mut p=false; macro_rules! w{($s:ident,$r:ident)=>{if safe_to_lower||(*watermarks).dcn4x.$s.usr>(*h).watermarks.dcn4x.$s.usr{(*h).watermarks.dcn4x.$s.usr=(*watermarks).dcn4x.$s.usr;REG_SET!($r,0,$r,(*watermarks).dcn4x.$s.usr);}else if (*watermarks).dcn4x.$s.usr<(*h).watermarks.dcn4x.$s.usr{p=true;}}} w!(a,DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A);w!(b,DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B);w!(c,DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_C);w!(d,DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_D);p }

unsafe fn hubbub42_program_stutter_z8_watermarks(hubbub:*mut hubbub, watermarks:*mut dcn_watermark_set, safe_to_lower:bool)->bool { let h=TO_DCN20_HUBBUB(hubbub); let mut p=false; macro_rules! w{($s:ident,$f:ident,$r:ident)=>{if safe_to_lower||(*watermarks).dcn4x.$s.$f>(*h).watermarks.dcn4x.$s.$f{(*h).watermarks.dcn4x.$s.$f=(*watermarks).dcn4x.$s.$f;REG_SET!($r,0,$r,(*watermarks).dcn4x.$s.$f);}else if (*watermarks).dcn4x.$s.$f<(*h).watermarks.dcn4x.$s.$f{p=true;}}} w!(a,sr_enter_z8,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_A);w!(a,sr_exit_z8,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A);w!(b,sr_enter_z8,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_B);w!(b,sr_exit_z8,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B);w!(c,sr_enter_z8,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_C);w!(c,sr_exit_z8,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C);w!(d,sr_enter_z8,DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_D);w!(d,sr_exit_z8,DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D);p }

unsafe fn hubbub42_allow_self_refresh_control(hubbub:*mut hubbub, allow:bool){let h=TO_DCN20_HUBBUB(hubbub);REG_UPDATE_2!(DCHUBBUB_ARB_DRAM_STATE_CNTL,DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_VALUE,0,DCHUBBUB_ARB_ALLOW_SELF_REFRESH_FORCE_ENABLE,!allow);}
unsafe fn hubbub42_set_sdp_control(hubbub:*mut hubbub, dc_control:bool){let _h=TO_DCN20_HUBBUB(hubbub);REG_UPDATE!(DCHUBBUB_SDPIF_CFG0,SDPIF_PORT_CONTROL,dc_control);}

unsafe fn hubbub42_program_watermarks(hubbub:*mut hubbub, watermarks:*mut dcn_watermark_set, refclk_mhz:u32, safe_to_lower:bool)->bool { let mut p=false; if !safe_to_lower&&(*(*hubbub).ctx).dc.debug.disable_stutter_for_wm_program{p=true;hubbub42_set_sdp_control(hubbub,false);hubbub42_allow_self_refresh_control(hubbub,false);} if hubbub42_program_urgent_watermarks(hubbub,watermarks,safe_to_lower){p=true;} if hubbub42_program_stutter_watermarks(hubbub,watermarks,safe_to_lower){p=true;} if hubbub42_program_pstate_watermarks(hubbub,watermarks,safe_to_lower){p=true;} if hubbub42_program_usr_watermarks(hubbub,watermarks,safe_to_lower){p=true;} if hubbub42_program_stutter_z8_watermarks(hubbub,watermarks,safe_to_lower){p=true;} REG_SET!(DCHUBBUB_ARB_SAT_LEVEL,0,DCHUBBUB_ARB_SAT_LEVEL,60*refclk_mhz);REG_UPDATE_2!(DCHUBBUB_ARB_DF_REQ_OUTSTAND,DCHUBBUB_ARB_MIN_REQ_OUTSTAND,0xff,DCHUBBUB_ARB_MIN_REQ_OUTSTAND_COMMIT_THRESHOLD,0xa);REG_UPDATE!(DCHUBBUB_ARB_HOSTVM_CNTL,DCHUBBUB_ARB_MAX_QOS_COMMIT_THRESHOLD,0xf); if safe_to_lower||(*(*hubbub).ctx).dc.debug.disable_stutter{hubbub42_allow_self_refresh_control(hubbub,!(*(*hubbub).ctx).dc.debug.disable_stutter);} if safe_to_lower&&(*(*hubbub).ctx).dc.debug.disable_stutter_for_wm_program{hubbub42_set_sdp_control(hubbub,true);} hubbub32_force_usr_retraining_allow(hubbub,(*(*hubbub).ctx).dc.debug.force_usr_allow);p }

unsafe fn hubbub42_set_request_limit(hubbub:*mut hubbub, _memory_channel_count:i32, _words_per_channel:i32){let _h=TO_DCN20_HUBBUB(hubbub);let request_limit:u32=96;REG_UPDATE!(SDPIF_REQUEST_RATE_LIMIT,SDPIF_REQUEST_RATE_LIMIT,request_limit);}
unsafe fn dcn42_program_arbiter(hubbub:*mut hubbub, arb_regs:*mut dml2_display_arb_regs, safe_to_lower:bool)->bool{let h=TO_DCN20_HUBBUB(hubbub);let mut p=false;let mut temp:u32=0;let allow=(*arb_regs).allow_sdpif_rate_limit_when_cstate_req!=0;if safe_to_lower||allow>(*h).allow_sdpif_rate_limit_when_cstate_req{(*h).allow_sdpif_rate_limit_when_cstate_req=allow;REG_GET!(DCHUBBUB_CTRL_STATUS,DCHUBBUB_HW_DEBUG,&mut temp);if allow{temp|=1<<5;}else{temp&=!(1<<5);}REG_UPDATE!(DCHUBBUB_CTRL_STATUS,DCHUBBUB_HW_DEBUG,temp);}else{p=true;}p}

static hubbub42_funcs: hubbub_funcs = hubbub_funcs { update_dchub:hubbub2_update_dchub, init_dchub_sys_ctx:hubbub31_init_dchub_sys_ctx, init_vm_ctx:hubbub2_init_vm_ctx, dcc_support_swizzle:hubbub3_dcc_support_swizzle, dcc_support_pixel_format:hubbub2_dcc_support_pixel_format, get_dcc_compression_cap:hubbub3_get_dcc_compression_cap, wm_read_state:hubbub35_wm_read_state, get_dchub_ref_freq:hubbub35_get_dchub_ref_freq, program_watermarks:hubbub42_program_watermarks, allow_self_refresh_control:hubbub42_allow_self_refresh_control, is_allow_self_refresh_enabled:hubbub1_is_allow_self_refresh_enabled, force_wm_propagate_to_pipes:hubbub32_force_wm_propagate_to_pipes, force_pstate_change_control:hubbub3_force_pstate_change_control, init_watermarks:hubbub35_init_watermarks, init_crb:dcn401_init_crb, dchvm_init:dcn35_dchvm_init, hubbub_read_state:hubbub2_read_state, force_usr_retraining_allow:hubbub32_force_usr_retraining_allow, set_request_limit:hubbub42_set_request_limit, program_det_segments:dcn401_program_det_segments, program_compbuf_segments:dcn401_program_compbuf_segments, wait_for_det_update:dcn401_wait_for_det_update, program_arbiter:dcn42_program_arbiter, hubbub_read_reg_state:hubbub3_read_reg_state };

unsafe fn hubbub42_construct(hubbub2:*mut dcn20_hubbub,ctx:*mut dc_context,hubbub_regs:*const dcn_hubbub_registers,hubbub_shift:*const dcn_hubbub_shift,hubbub_mask:*const dcn_hubbub_mask,det_size_kb:i32,pixel_chunk_size_kb:i32,config_return_buffer_size_kb:i32){(*hubbub2).base.ctx=ctx;(*hubbub2).base.funcs=&hubbub42_funcs;(*hubbub2).regs=hubbub_regs;(*hubbub2).shifts=hubbub_shift;(*hubbub2).masks=hubbub_mask;(*hubbub2).detile_buf_size=det_size_kb*1024;(*hubbub2).pixel_chunk_size=pixel_chunk_size_kb*1024;(*hubbub2).crb_size_segs=config_return_buffer_size_kb/DCN42_CRB_SEGMENT_SIZE_KB;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
