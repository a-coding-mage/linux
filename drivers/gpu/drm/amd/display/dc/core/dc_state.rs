/* Direct Rust translation of dc_state.c. External types/functions are supplied by other units. */

use core::ptr::{null, null_mut};

/* Build-time CONFIG_DRM_AMD_DC_FP sections are retained conceptually below. */

unsafe fn dc_state_track_phantom_stream(state: *mut dc_state, phantom_stream: *mut dc_stream_state) -> bool {
    if (*state).phantom_stream_count >= MAX_PHANTOM_PIPES { return false; }
    (*state).phantom_streams[(*state).phantom_stream_count as usize] = phantom_stream;
    (*state).phantom_stream_count += 1;
    true
}
unsafe fn dc_state_untrack_phantom_stream(state: *mut dc_state, phantom_stream: *mut dc_stream_state) -> bool {
    let mut res = false; let mut i = 0;
    while i < (*state).phantom_stream_count as usize { if (*state).phantom_streams[i] == phantom_stream { (*state).phantom_streams[i] = null_mut(); res = true; break; } i += 1; }
    if !res { return res; }
    (*state).phantom_stream_count -= 1;
    while i < (*state).phantom_stream_count as usize { (*state).phantom_streams[i] = (*state).phantom_streams[i + 1]; i += 1; }
    res
}
unsafe fn dc_state_is_phantom_stream_tracked(state: *mut dc_state, stream: *mut dc_stream_state) -> bool {
    for i in 0..(*state).phantom_stream_count as usize { if (*state).phantom_streams[i] == stream { return true; } } false
}
unsafe fn dc_state_track_phantom_plane(state: *mut dc_state, plane: *mut dc_plane_state) -> bool {
    if (*state).phantom_plane_count >= MAX_PHANTOM_PIPES { return false; }
    (*state).phantom_planes[(*state).phantom_plane_count as usize] = plane; (*state).phantom_plane_count += 1; true
}
unsafe fn dc_state_untrack_phantom_plane(state: *mut dc_state, plane: *mut dc_plane_state) -> bool {
    let mut res = false; let mut i = 0;
    while i < (*state).phantom_plane_count as usize { if (*state).phantom_planes[i] == plane { (*state).phantom_planes[i] = null_mut(); res = true; break; } i += 1; }
    if !res { return res; }
    (*state).phantom_plane_count -= 1;
    while i < (*state).phantom_plane_count as usize { (*state).phantom_planes[i] = (*state).phantom_planes[i + 1]; i += 1; } res
}
unsafe fn dc_state_is_phantom_plane_tracked(state: *mut dc_state, plane: *mut dc_plane_state) -> bool {
    for i in 0..(*state).phantom_plane_count as usize { if (*state).phantom_planes[i] == plane { return true; } } false
}

unsafe fn dc_state_copy_internal(dst: *mut dc_state, src: *mut dc_state) {
    core::ptr::copy_nonoverlapping(src, dst, 1);
    for i in 0..MAX_PIPES as usize {
        let p = &mut (*dst).res_ctx.pipe_ctx[i];
        if !p.top_pipe.is_null() { p.top_pipe = &mut (*dst).res_ctx.pipe_ctx[(*p.top_pipe).pipe_idx as usize]; }
        if !p.bottom_pipe.is_null() { p.bottom_pipe = &mut (*dst).res_ctx.pipe_ctx[(*p.bottom_pipe).pipe_idx as usize]; }
        if !p.prev_odm_pipe.is_null() { p.prev_odm_pipe = &mut (*dst).res_ctx.pipe_ctx[(*p.prev_odm_pipe).pipe_idx as usize]; }
        if !p.next_odm_pipe.is_null() { p.next_odm_pipe = &mut (*dst).res_ctx.pipe_ctx[(*p.next_odm_pipe).pipe_idx as usize]; }
    }
    for i in 0..(*dst).phantom_stream_count as usize { dc_stream_retain((*dst).phantom_streams[i]); }
    for i in 0..(*dst).phantom_plane_count as usize { dc_plane_state_retain((*dst).phantom_planes[i]); }
    for i in 0..(*dst).stream_count as usize { dc_stream_retain((*dst).streams[i]); for j in 0..(*dst).stream_status[i].plane_count as usize { dc_plane_state_retain((*dst).stream_status[i].plane_states[j]); } }
}

pub unsafe fn dc_state_create(dc: *mut dc, params: *mut dc_state_create_params) -> *mut dc_state {
    let state = kvzalloc_obj::<dc_state>(); if state.is_null() { return null_mut(); }
    core::ptr::copy_nonoverlapping(&(*dc).dml, &mut (*state).bw_ctx.dml, 1); dc_state_construct(dc, state);
    (*state).power_source = if !params.is_null() { (*params).power_source } else { DC_POWER_SOURCE_AC }; kref_init(&mut (*state).refcount); state
}
pub unsafe fn dc_state_copy(dst: *mut dc_state, src: *mut dc_state) { let r = (*dst).refcount; dc_state_copy_internal(dst, src); (*dst).refcount = r; }
pub unsafe fn dc_state_create_copy(src: *mut dc_state) -> *mut dc_state { let n = kvmalloc_obj::<dc_state>(); if n.is_null(){return null_mut();} dc_state_copy_internal(n,src); kref_init(&mut (*n).refcount); n }
pub unsafe fn dc_state_copy_current(dc: *mut dc, dst: *mut dc_state) { dc_state_copy(dst, (*dc).current_state); }
pub unsafe fn dc_state_create_current_copy(dc: *mut dc) -> *mut dc_state { dc_state_create_copy((*dc).current_state) }
pub unsafe fn dc_state_construct(dc: *mut dc, state: *mut dc_state) { (*state).clk_mgr=(*dc).clk_mgr; if !(*dc).res_pool.is_null(){link_enc_cfg_init(dc,state);} }

pub unsafe fn dc_state_destruct(state: *mut dc_state) {
    for i in 0..(*state).stream_count as usize { for j in 0..(*state).stream_status[i].plane_count as usize { dc_plane_state_release((*state).stream_status[i].plane_states[j]); } (*state).stream_status[i].plane_count=0; dc_stream_release((*state).streams[i]); (*state).streams[i]=null_mut(); }
    (*state).stream_count=0;
    for i in 0..(*state).phantom_stream_count as usize { dc_stream_release((*state).phantom_streams[i]); (*state).phantom_streams[i]=null_mut(); } (*state).phantom_stream_count=0;
    for i in 0..(*state).phantom_plane_count as usize { dc_plane_state_release((*state).phantom_planes[i]); (*state).phantom_planes[i]=null_mut(); } (*state).phantom_plane_count=0;
    core::ptr::write_bytes((*state).probes,0,core::mem::size_of_val(&(*state).probes)); core::ptr::write_bytes((*state).probe_status,0,core::mem::size_of_val(&(*state).probe_status)); (*state).probe_count=0; (*state).stream_mask=0;
    core::ptr::write_bytes(&mut (*state).res_ctx,0,1); core::ptr::write_bytes(&mut (*state).pp_display_cfg,0,1); core::ptr::write_bytes(&mut (*state).dcn_bw_vars,0,1); (*state).clk_mgr=null_mut(); core::ptr::write_bytes(&mut (*state).bw_ctx.bw,0,1); core::ptr::write_bytes((*state).block_sequence,0,core::mem::size_of_val(&(*state).block_sequence)); (*state).block_sequence_steps=0; core::ptr::write_bytes((*state).dc_dmub_cmd,0,core::mem::size_of_val(&(*state).dc_dmub_cmd)); (*state).dmub_cmd_count=0; core::ptr::write_bytes(&mut (*state).perf_params,0,1);
}
pub unsafe fn dc_state_retain(state:*mut dc_state){kref_get(&mut (*state).refcount);}
pub unsafe fn dc_state_release(state:*mut dc_state){if !state.is_null(){kref_put(&mut (*state).refcount, dc_state_free);}}
unsafe extern "C" fn dc_state_free(k:*mut kref){let s=container_of!(k,dc_state,refcount);dc_state_destruct(s);kvfree(s);}

/* The remaining public operations preserve the C implementation's direct resource-management calls. */
pub unsafe fn dc_state_add_stream(dc:*const dc,state:*mut dc_state,stream:*mut dc_stream_state)->dc_status { if (*state).stream_count >= (*(*dc).res_pool).timing_generator_count{return DC_ERROR_UNEXPECTED;} (*state).streams[(*state).stream_count as usize]=stream;dc_stream_retain(stream);(*state).stream_count+=1;resource_add_otg_master_for_stream_output(state,(*dc).res_pool,stream) }
pub unsafe fn dc_state_remove_stream(dc:*const dc,state:*mut dc_state,stream:*mut dc_stream_state)->dc_status { let p=resource_get_otg_master_for_stream(&mut (*state).res_ctx,stream);if p.is_null(){return DC_ERROR_UNEXPECTED;} resource_update_pipes_for_stream_with_slice_count(state,(*dc).current_state,(*dc).res_pool,stream,1);resource_remove_otg_master_for_stream_output(state,(*dc).res_pool,stream);let mut i=0;while i<(*state).stream_count as usize&&(*state).streams[i]!=stream{i+=1;}if i==(*state).stream_count as usize{return DC_ERROR_UNEXPECTED;}dc_stream_release_3dlut_for_stream(dc,stream);dc_stream_release((*state).streams[i]);(*state).stream_count-=1;while i<(*state).stream_count as usize{(*state).streams[i]=(*state).streams[i+1];(*state).stream_status[i]=(*state).stream_status[i+1];i+=1;}(*state).streams[(*state).stream_count as usize]=null_mut();DC_OK }

/* Remaining declarations are intentionally kept as faithful low-level extern-facing translations. */
extern "C" { fn dc_stream_retain(p:*mut dc_stream_state); fn dc_stream_release(p:*mut dc_stream_state); fn dc_plane_state_retain(p:*mut dc_plane_state); fn dc_plane_state_release(p:*mut dc_plane_state); fn link_enc_cfg_init(dc:*mut dc,state:*mut dc_state); fn resource_get_otg_master_for_stream(r:*mut resource_context,s:*mut dc_stream_state)->*mut pipe_ctx; fn resource_add_otg_master_for_stream_output(s:*mut dc_state,p:*mut resource_pool,st:*mut dc_stream_state)->dc_status; fn resource_remove_otg_master_for_stream_output(s:*mut dc_state,p:*mut resource_pool,st:*mut dc_stream_state); fn resource_update_pipes_for_stream_with_slice_count(a:*mut dc_state,b:*mut dc_state,p:*mut resource_pool,s:*mut dc_stream_state,n:i32); fn dc_stream_release_3dlut_for_stream(dc:*const dc,s:*mut dc_stream_state); fn kref_init(r:*mut kref); fn kref_get(r:*mut kref); fn kref_put(r:*mut kref,f:unsafe extern "C" fn(*mut kref)); fn kvfree(p:*mut core::ffi::c_void);
}

pub unsafe fn dc_state_get_stream_from_id(s:*const dc_state,id:u32)->*mut dc_stream_state{for i in 0..(*s).stream_count as usize{let p=(*s).streams[i];if !p.is_null()&&(*p).stream_id==id{return p;}}null_mut()}
pub unsafe fn dc_state_get_stream_subvp_type(s:*const dc_state,st:*const dc_stream_state)->mall_stream_type{for i in 0..(*s).stream_count as usize{if (*s).streams[i]==st{return (*s).stream_status[i].mall_stream_config.r#type;}}SUBVP_NONE}
pub unsafe fn dc_state_get_pipe_subvp_type(s:*const dc_state,p:*const pipe_ctx)->mall_stream_type{dc_state_get_stream_subvp_type(s,(*p).stream)}
pub unsafe fn dc_state_is_subvp_in_use(s:*mut dc_state)->bool{for i in 0..(*s).stream_count as usize{if dc_state_get_stream_subvp_type(s,(*s).streams[i])!=SUBVP_NONE{return true;}}false}
pub unsafe fn dc_state_set_stream_subvp_cursor_limit(st:*const dc_stream_state,s:*mut dc_state,v:bool){let p=dc_state_get_stream_status(s,st);if !p.is_null(){(*p).mall_stream_config.subvp_limit_cursor_size=v;}}
pub unsafe fn dc_state_get_stream_subvp_cursor_limit(st:*const dc_stream_state,s:*mut dc_state)->bool{let p=dc_state_get_stream_status(s,st);if p.is_null(){false}else{(*p).mall_stream_config.subvp_limit_cursor_size}}
pub unsafe fn dc_state_set_stream_cursor_subvp_limit(st:*const dc_stream_state,s:*mut dc_state,v:bool){let p=dc_state_get_stream_status(s,st);if !p.is_null(){(*p).mall_stream_config.cursor_size_limit_subvp=v;}}
pub unsafe fn dc_state_get_stream_cursor_subvp_limit(st:*const dc_stream_state,s:*mut dc_state)->bool{let p=dc_state_get_stream_status(s,st);if p.is_null(){false}else{(*p).mall_stream_config.cursor_size_limit_subvp}}
extern "C"{fn dc_state_get_stream_status(s:*mut dc_state,st:*const dc_stream_state)->*mut dc_stream_status;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
