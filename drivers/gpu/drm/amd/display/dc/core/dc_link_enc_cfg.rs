/* Copyright 2021 Advanced Micro Devices, Inc. All rights reserved. */

/* Translated from dc_link_enc_cfg.c.  Declarations supplied by the surrounding
 * display-core sources are intentionally left as external dependencies. */

unsafe fn is_dig_link_enc_stream(stream: *mut dc_stream_state) -> bool {
    let mut is_dig_stream = false;
    if !stream.is_null() {
        for i in 0..(*(*(*stream).ctx).dc).res_pool.res_cap.num_dig_link_enc {
            let link_enc = (*(*(*stream).ctx).dc).res_pool.link_encoders[i];
            if !link_enc.is_null() &&
                (((*(*stream).link).connector_signal as u32) & (*link_enc).output_signals) != 0 {
                is_dig_stream = true;
                if dc_is_hdmi_frl_signal((*stream).signal) { is_dig_stream = false; }
                break;
            }
        }
    }
    is_dig_stream
}

unsafe fn get_assignment(dc: *mut dc, i: usize) -> link_enc_assignment {
    if (*(*dc).current_state).res_ctx.link_enc_cfg_ctx.mode == LINK_ENC_CFG_TRANSIENT {
        (*(*dc).current_state).res_ctx.link_enc_cfg_ctx.transient_assignments[i]
    } else {
        (*(*dc).current_state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i]
    }
}

unsafe fn get_stream_using_link_enc(state: *mut dc_state, eng_id: engine_id) -> *mut dc_stream_state {
    let mut stream = core::ptr::null_mut();
    for i in 0..(*state).stream_count {
        let a = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i];
        if a.valid && a.eng_id == eng_id { stream = (*state).streams[i]; break; }
    }
    stream
}

unsafe fn remove_link_enc_assignment(state: *mut dc_state, stream: *mut dc_stream_state, eng_id: engine_id) {
    if eng_id != ENGINE_ID_UNKNOWN {
        let eng_idx = (eng_id - ENGINE_ID_DIGA) as usize;
        for i in 0..MAX_PIPES {
            let a = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i];
            if a.valid && a.stream == stream {
                (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].valid = false;
                if get_stream_using_link_enc(state, eng_id).is_null() {
                    (*state).res_ctx.link_enc_cfg_ctx.link_enc_avail[eng_idx] = eng_id;
                }
                (*stream).link_enc = core::ptr::null_mut();
                (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].eng_id = ENGINE_ID_UNKNOWN;
                (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].stream = core::ptr::null_mut();
                dc_stream_release(stream);
                break;
            }
        }
    }
}

unsafe fn add_link_enc_assignment(state: *mut dc_state, stream: *mut dc_stream_state, eng_id: engine_id) {
    if eng_id != ENGINE_ID_UNKNOWN {
        let eng_idx = (eng_id - ENGINE_ID_DIGA) as usize;
        let mut i = 0;
        while i < (*state).stream_count {
            if stream == (*state).streams[i] {
                (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i] = link_enc_assignment {
                    valid: true,
                    ep_id: display_endpoint_id { link_id: (*(*stream).link).link_id, ep_type: (*(*stream).link).ep_type },
                    eng_id,
                    stream,
                };
                dc_stream_retain(stream);
                (*state).res_ctx.link_enc_cfg_ctx.link_enc_avail[eng_idx] = ENGINE_ID_UNKNOWN;
                (*stream).link_enc = (*(*(*stream).ctx).dc).res_pool.link_encoders[eng_idx];
                break;
            }
            i += 1;
        }
        ASSERT(i != (*state).stream_count);
    }
}

unsafe fn find_first_avail_link_enc(ctx: *const dc_context, state: *const dc_state, requested: engine_id) -> engine_id {
    if requested != ENGINE_ID_UNKNOWN {
        for i in 0..(*(*ctx).dc).res_pool.res_cap.num_dig_link_enc {
            let e = (*state).res_ctx.link_enc_cfg_ctx.link_enc_avail[i];
            if e == requested { return e; }
        }
    }
    for i in 0..(*(*ctx).dc).res_pool.res_cap.num_dig_link_enc {
        let e = (*state).res_ctx.link_enc_cfg_ctx.link_enc_avail[i];
        if e != ENGINE_ID_UNKNOWN { return e; }
    }
    ENGINE_ID_UNKNOWN
}

unsafe fn is_avail_link_enc(state: *mut dc_state, eng_id: engine_id, stream: *mut dc_stream_state) -> bool {
    let idx = (eng_id - ENGINE_ID_DIGA) as usize;
    if eng_id != ENGINE_ID_UNKNOWN && (*state).res_ctx.link_enc_cfg_ctx.link_enc_avail[idx] != ENGINE_ID_UNKNOWN { return true; }
    let assigned = get_stream_using_link_enc(state, eng_id);
    !assigned.is_null() && assigned != stream && (*assigned).link == (*stream).link
}

unsafe fn are_ep_ids_equal(lhs: *const display_endpoint_id, rhs: *const display_endpoint_id) -> bool {
    (*lhs).link_id.id == (*rhs).link_id.id && (*lhs).link_id.enum_id == (*rhs).link_id.enum_id &&
    (*lhs).link_id.r#type == (*rhs).link_id.r#type && (*lhs).ep_type == (*rhs).ep_type
}

unsafe fn get_link_enc_used_by_link(state: *mut dc_state, link: *const dc_link) -> *mut link_encoder {
    let ep = display_endpoint_id { link_id: (*link).link_id, ep_type: (*link).ep_type };
    for i in 0..MAX_PIPES {
        let a = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i];
        if a.eng_id != ENGINE_ID_UNKNOWN && a.valid && are_ep_ids_equal(&a.ep_id, &ep) {
            return (*(*link).dc).res_pool.link_encoders[(a.eng_id - ENGINE_ID_DIGA) as usize];
        }
    }
    core::ptr::null_mut()
}

unsafe fn clear_enc_assignments(dc: *const dc, state: *mut dc_state) {
    for i in 0..MAX_PIPES {
        (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].valid = false;
        (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].eng_id = ENGINE_ID_UNKNOWN;
        let s = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].stream;
        if !s.is_null() { dc_stream_release(s); (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i].stream = core::ptr::null_mut(); }
    }
    for i in 0..(*(*dc).res_pool.res_cap).num_dig_link_enc {
        (*state).res_ctx.link_enc_cfg_ctx.link_enc_avail[i] = if !(*dc).res_pool.link_encoders[i].is_null() { i as engine_id } else { ENGINE_ID_UNKNOWN };
    }
}

pub unsafe fn link_enc_cfg_init(dc: *const dc, state: *mut dc_state) { clear_enc_assignments(dc, state); (*state).res_ctx.link_enc_cfg_ctx.mode = LINK_ENC_CFG_STEADY; }

pub unsafe fn link_enc_cfg_copy(src: *const dc_state, dst: *mut dc_state) {
    (*dst).res_ctx.link_enc_cfg_ctx = (*src).res_ctx.link_enc_cfg_ctx;
}

pub unsafe fn link_enc_cfg_link_enc_unassign(state: *mut dc_state, stream: *mut dc_stream_state) {
    let eng = if !(*stream).link_enc.is_null() { (*(*stream).link_enc).preferred_engine } else { ENGINE_ID_UNKNOWN };
    remove_link_enc_assignment(state, stream, eng);
}

pub unsafe fn link_enc_cfg_get_stream_using_link_enc(dc: *mut dc, eng_id: engine_id) -> *mut dc_stream_state {
    for i in 0..MAX_PIPES { let a = get_assignment(dc, i); if a.valid && a.eng_id == eng_id { return a.stream; } }
    core::ptr::null_mut()
}

pub unsafe fn link_enc_cfg_get_link_using_link_enc(dc: *mut dc, eng_id: engine_id) -> *mut dc_link {
    let s = link_enc_cfg_get_stream_using_link_enc(dc, eng_id); if !s.is_null() { (*s).link } else { core::ptr::null_mut() }
}

pub unsafe fn link_enc_cfg_is_transmitter_mappable(dc: *mut dc, enc: *mut link_encoder) -> bool {
    let s = link_enc_cfg_get_stream_using_link_enc(dc, (*enc).preferred_engine); !s.is_null() && (*(*s).link).is_dig_mapping_flexible
}

pub unsafe fn link_enc_cfg_get_link_enc_used_by_link(dc: *mut dc, link: *const dc_link) -> *mut link_encoder { get_link_enc_used_by_link((*link).dc.current_state, link) }

pub unsafe fn link_enc_cfg_get_next_avail_link_enc(dc: *mut dc) -> *mut link_encoder {
    let mut used = [ENGINE_ID_UNKNOWN; MAX_LINK_ENCODERS];
    for i in 0..MAX_PIPES { let a = get_assignment(dc, i); if a.valid && a.eng_id != ENGINE_ID_UNKNOWN { used[(a.eng_id - ENGINE_ID_DIGA) as usize] = a.eng_id; } }
    for i in 0..(*dc).res_pool.res_cap.num_dig_link_enc { if used[i] == ENGINE_ID_UNKNOWN && !(*dc).res_pool.link_encoders[i].is_null() { return (*dc).res_pool.link_encoders[i]; } }
    core::ptr::null_mut()
}

pub unsafe fn link_enc_cfg_get_link_enc(link: *const dc_link) -> *mut link_encoder {
    if (*link).is_dig_mapping_flexible && (*link).dc.res_pool.funcs.link_encs_assign {
        let mut e = link_enc_cfg_get_link_enc_used_by_link((*link).ctx.dc, link);
        if e.is_null() { e = link_enc_cfg_get_next_avail_link_enc((*link).ctx.dc); }
        e
    } else { (*link).link_enc }
}

pub unsafe fn link_enc_cfg_get_link_enc_used_by_stream_current(dc: *mut dc, stream: *const dc_stream_state) -> *mut link_encoder {
    let ep = display_endpoint_id { link_id: (*(*stream).link).link_id, ep_type: (*(*stream).link).ep_type };
    for i in 0..MAX_PIPES { let a = (*(*dc).current_state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i]; if a.eng_id != ENGINE_ID_UNKNOWN && a.valid && are_ep_ids_equal(&a.ep_id, &ep) { return (*(*stream).link).dc.res_pool.link_encoders[(a.eng_id - ENGINE_ID_DIGA) as usize]; } }
    core::ptr::null_mut()
}

pub unsafe fn link_enc_cfg_is_link_enc_avail(dc: *mut dc, eng_id: engine_id, link: *mut dc_link) -> bool {
    for i in 0..MAX_PIPES { let a = get_assignment(dc, i); let ep = display_endpoint_id { link_id: (*link).link_id, ep_type: (*link).ep_type }; if a.valid && a.eng_id == eng_id && !are_ep_ids_equal(&ep, &a.ep_id) { return false; } }
    true
}

pub unsafe fn link_enc_cfg_validate(dc: *mut dc, state: *mut dc_state) -> bool {
    let mut valid_count = 0; let mut dig_count = 0; let mut ok = true;
    for i in 0..MAX_PIPES { let a = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i]; if a.valid { valid_count += 1; if a.stream != (*state).streams[i] { ok = false; } } if is_dig_link_enc_stream((*state).streams[i]) { dig_count += 1; } }
    if valid_count != dig_count { ok = false; }
    for i in 0..MAX_PIPES { let a = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[i]; if a.valid { for j in 0..MAX_PIPES { if i != j { let b = (*state).res_ctx.link_enc_cfg_ctx.link_enc_assignments[j]; if b.valid && ((are_ep_ids_equal(&a.ep_id, &b.ep_id) && a.eng_id != b.eng_id) || (!are_ep_ids_equal(&a.ep_id, &b.ep_id) && a.eng_id == b.eng_id)) { ok = false; } } } } }
    ASSERT(ok); ok
}

pub unsafe fn link_enc_cfg_set_transient_mode(_dc: *mut dc, current: *mut dc_state, new_state: *mut dc_state) {
    let mut n = 0; for i in 0..MAX_PIPES { if (*current).res_ctx.link_enc_cfg_ctx.transient_assignments[i].valid { n += 1; } }
    if (*new_state).stream_count == n { (*current).res_ctx.link_enc_cfg_ctx.mode = LINK_ENC_CFG_TRANSIENT; }
}

pub unsafe fn link_enc_cfg_link_encs_assign(dc: *mut dc, state: *mut dc_state, streams: *mut *mut dc_stream_state, stream_count: u8) {
    ASSERT((*state).stream_count == stream_count);
    ASSERT((*(*dc).current_state).res_ctx.link_enc_cfg_ctx.mode == LINK_ENC_CFG_STEADY);
    for i in 0..(*(*dc).current_state).stream_count { ((*dc).res_pool.funcs.link_enc_unassign)(state, (*(*dc).current_state).streams[i]); }
    for i in 0..stream_count as usize {
        let s = *streams.add(i);
        if (*s).link.is_null() || (*(*s).link).is_dig_mapping_flexible || !is_dig_link_enc_stream(s) { continue; }
        add_link_enc_assignment(state, s, (*(*s).link).eng_id);
    }
    for i in 0..stream_count as usize {
        let s = *streams.add(i);
        if (*s).link.is_null() || !(*(*s).link).is_dig_mapping_flexible || !is_dig_link_enc_stream(s) || !(*s).link_enc.is_null() { continue; }
        let mut e = get_link_enc_used_by_link(state, (*s).link);
        if e.is_null() { let requested = if (*(*s).link).ep_type == DISPLAY_ENDPOINT_USB4_DPIA { (*(*s).link).dpia_preferred_eng_id } else { ENGINE_ID_UNKNOWN }; let id = find_first_avail_link_enc((*s).ctx, state, requested); add_link_enc_assignment(state, s, id); } else { add_link_enc_assignment(state, s, (*e).preferred_engine); }
    }
    link_enc_cfg_validate(dc, state);
    (*state).res_ctx.link_enc_cfg_ctx.mode = LINK_ENC_CFG_STEADY;
}

// The remaining public validation and assignment routines retain the C algorithm;
// their external types, logging, assertions, and resource callbacks are supplied
// by the translated display-core dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
