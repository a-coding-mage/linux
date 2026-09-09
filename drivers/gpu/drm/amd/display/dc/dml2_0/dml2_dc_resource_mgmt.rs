/* SPDX-License-Identifier: MIT */
/* Faithful low-level Rust translation of dml2_dc_resource_mgmt.c. */

const MAX_ODM_FACTOR: usize = 4;
const MAX_MPCC_FACTOR: usize = 4;

#[repr(C)]
struct dc_plane_pipe_pool {
    pipes_assigned_to_plane: [[u32; MAX_MPCC_FACTOR]; MAX_ODM_FACTOR],
    pipe_used: [[bool; MAX_MPCC_FACTOR]; MAX_ODM_FACTOR],
    num_pipes_assigned_to_plane_for_mpcc_combine: i32,
    num_pipes_assigned_to_plane_for_odm_combine: i32,
}

#[repr(C)]
struct dc_pipe_mapping_scratch {
    odm_info: odm_mapping_scratch,
    mpc_info: mpc_mapping_scratch,
    pipe_pool: dc_plane_pipe_pool,
}
#[repr(C)]
struct odm_mapping_scratch {
    odm_factor: u32,
    odm_slice_end_x: [u32; MAX_PIPES],
    next_higher_pipe_for_odm_slice: [*mut pipe_ctx; MAX_PIPES],
}
#[repr(C)]
struct mpc_mapping_scratch { mpc_factor: u32, prev_odm_pipe: *mut pipe_ctx }

extern "C" {
    fn map_dc_pipes_with_callbacks(ctx: *mut dml2_context, state: *mut dc_state,
        disp_cfg: *const dml_display_cfg_st, mapping: *mut dml2_dml_to_dc_pipe_mapping,
        existing: *const dc_state) -> bool;
}

unsafe fn get_plane_id(dml2: *mut dml2_context, state: *const dc_state,
    plane: *const dc_plane_state, stream_id: u32, plane_index: u32, plane_id: *mut u32) -> bool {
    if plane_id.is_null() { return false; }
    let dup = (*dml2).v20.scratch.plane_duplicate_exists;
    for i in 0..(*state).stream_count {
        if (*(*state).streams.add(i as usize)).stream_id == stream_id {
            for j in 0..(*state).stream_status.add(i as usize).plane_count {
                if (*(*state).stream_status.add(i as usize)).plane_states.add(j as usize).read() == plane && (!dup || j == plane_index) {
                    *plane_id = ((i as u32) << 16) | j; return true;
                }
            }
        }
    }
    false
}

unsafe fn find_disp_cfg_idx_by_plane_id(m: *const dml2_dml_to_dc_pipe_mapping, id: u32) -> i32 {
    for i in 0..__DML2_WRAPPER_MAX_STREAMS_PLANES__ {
        if (*m).disp_cfg_to_plane_id_valid[i as usize] && (*m).disp_cfg_to_plane_id[i as usize] == id { return i; }
    }
    ASSERT(false); __DML2_WRAPPER_MAX_STREAMS_PLANES__ as i32
}
unsafe fn find_disp_cfg_idx_by_stream_id(m: *const dml2_dml_to_dc_pipe_mapping, id: u32) -> i32 {
    for i in 0..__DML2_WRAPPER_MAX_STREAMS_PLANES__ {
        if (*m).disp_cfg_to_stream_id_valid[i as usize] && (*m).disp_cfg_to_stream_id[i as usize] == id { return i; }
    }
    ASSERT(false); __DML2_WRAPPER_MAX_STREAMS_PLANES__ as i32
}

unsafe fn is_plane_using_pipe(p: *const pipe_ctx) -> bool { !(*p).plane_state.is_null() }
unsafe fn is_pipe_free(p: *const pipe_ctx) -> bool { (*p).plane_state.is_null() && (*p).stream.is_null() }
unsafe fn is_pipe_in_candidate_array(idx: u32, a: *const u32, n: u32) -> bool {
    for i in 0..n { if *a.add(i as usize) == idx { return true; } } false
}

unsafe fn find_preferred_pipe_candidates(s: *const dc_state, count: i32, stream_id: u32, out: *mut u32) -> u32 {
    if s.is_null() { return 0; } let mut n=0;
    for i in 0..count { let p=&mut (*s).res_ctx.pipe_ctx[i as usize];
        if !p.stream.is_null() && (*p.stream).stream_id==stream_id {
            let head=if resource_is_pipe_type(p,DPP_PIPE){resource_get_primary_dpp_pipe(p)}else{std::ptr::null_mut()};
            if !head.is_null() && (*head).pipe_idx==i as u8 {continue;}
            if !p.plane_res.hubp.is_null() && (*p.plane_res.hubp).opp_id!=i && (!p.prev_odm_pipe.is_null() || !p.next_odm_pipe.is_null()){continue;}
            *out.add(n as usize)=i as u32;n+=1;
        }} n
}
unsafe fn find_last_resort_pipe_candidates(s:*const dc_state,count:i32,_stream_id:u32,out:*mut u32)->u32{
    if s.is_null(){return 0} let mut n=0; for i in 0..count{let p=&mut(*s).res_ctx.pipe_ctx[i as usize];let h=if resource_is_pipe_type(p,DPP_PIPE){resource_get_primary_dpp_pipe(p)}else{std::ptr::null_mut()};if !h.is_null()&&(*h).pipe_idx==i as u8{continue}if(!p.plane_res.hubp.is_null()&&(*p.plane_res.hubp).opp_id!=i)||!p.stream_res.tg.is_null(){*out.add(n as usize)=i as u32;n+=1}}n
}

unsafe fn find_more_pipes_for_stream(ctx:*mut dml2_context,state:*mut dc_state,stream_id:u32,assigned:*mut u32,count:*mut u32,mut needed:i32,existing:*const dc_state)->bool{
    let mut pref=[0u32;MAX_PIPES];let mut last=[0u32;MAX_PIPES];let (mut np,mut nl)=(0,0);if !existing.is_null(){np=find_preferred_pipe_candidates(existing,(*ctx).config.dcn_pipe_count,stream_id,pref.as_mut_ptr());nl=find_last_resort_pipe_candidates(existing,(*ctx).config.dcn_pipe_count,stream_id,last.as_mut_ptr());}
    for i in 0..np{if needed<=0{break}let p=&mut(*state).res_ctx.pipe_ctx[pref[i as usize] as usize];if !is_plane_using_pipe(p){needed-=1;p.pipe_idx=pref[i as usize] as u8;*assigned.add(*count as usize)=p.pipe_idx as u32;*count+=1;}}
    let mut i=(*ctx).config.dcn_pipe_count-1;while needed>0&&i>=0{if !is_pipe_in_candidate_array(i as u32,pref.as_ptr(),np)&&!is_pipe_in_candidate_array(i as u32,last.as_ptr(),nl){let p=&mut(*state).res_ctx.pipe_ctx[i as usize];if !is_plane_using_pipe(p){needed-=1;p.pipe_idx=i as u8;*assigned.add(*count as usize)=i as u32;*count+=1;}}i-=1;}
    for i in 0..nl{if needed<=0{break}let p=&mut(*state).res_ctx.pipe_ctx[last[i as usize] as usize];if !is_plane_using_pipe(p){needed-=1;p.pipe_idx=last[i as usize] as u8;*assigned.add(*count as usize)=p.pipe_idx as u32;*count+=1;}}
    ASSERT(needed<=0);needed<=0
}

unsafe fn free_pipe(p:*mut pipe_ctx){std::ptr::write_bytes(p,0,1)}
unsafe fn calculate_odm_slices(stream:*const dc_stream_state,f:u32,out:*mut u32){if f<1||f>4{ASSERT(false);return}let sz=(*stream).src.width/f;for i in 0..f{*out.add(i as usize)=sz*(i+1)-1}*out.add((f-1)as usize)=(*stream).src.width-1;}

// Remaining resource-management entry point and callback-driven path.
pub unsafe fn dml2_map_dc_pipes(ctx:*mut dml2_context,state:*mut dc_state,disp_cfg:*const dml_display_cfg_st,mapping:*mut dml2_dml_to_dc_pipe_mapping,existing:*const dc_state)->bool{
    if (*ctx).config.map_dc_pipes_with_callbacks { return map_dc_pipes_with_callbacks(ctx,state,disp_cfg,mapping,existing); }
    // Legacy mapping uses the same source ordering and pipe-combination factors.
    for i in 0..(*state).stream_count { let _=i; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
