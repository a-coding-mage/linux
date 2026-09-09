/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

/* C headers removed; their declarations are supplied by the surrounding crate. */

pub unsafe fn dc_plane_construct(ctx: *mut dc_context, plane_state: *mut dc_plane_state) {
    (*plane_state).ctx = ctx;
    (*plane_state).gamma_correction.is_identity = true;
    (*plane_state).in_transfer_func.type_ = TF_TYPE_BYPASS;
    (*plane_state).pre_multiplied_alpha = true;
    (*plane_state).cm.shaper_func.type_ = TF_TYPE_BYPASS;
    (*plane_state).cm.blend_func.type_ = TF_TYPE_BYPASS;
    (*plane_state).cm.lut3d_func.state.raw = 0;
    (*plane_state).cm.flags.all = 0;
}

pub unsafe fn dc_plane_destruct(plane_state: *mut dc_plane_state) {
    let _ = plane_state;
    // no more pointers to free within dc_plane_state
}

/* dc_state is passed separately since it may differ from the current dc state accessible from plane_state. */
pub unsafe fn dc_plane_get_pipe_mask(
    dc_state: *mut dc_state,
    plane_state: *const dc_plane_state,
) -> u8 {
    let mut pipe_mask: u8 = 0;
    let mut i: u32 = 0;
    while i < (*(*plane_state).ctx).dc.as_ref().unwrap().res_pool.pipe_count {
        let pipe_ctx = &mut (*dc_state).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.plane_state == plane_state && !pipe_ctx.plane_res.hubp.is_null() {
            pipe_mask |= (1u8).wrapping_shl((*pipe_ctx.plane_res.hubp).inst as u32);
        }
        i += 1;
    }
    pipe_mask
}

pub unsafe fn dc_create_plane_state(dc: *const dc) -> *mut dc_plane_state {
    let plane_state = kvzalloc_obj::<dc_plane_state>();
    if plane_state.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*plane_state).refcount);
    dc_plane_construct((*dc).ctx, plane_state);
    plane_state
}

pub unsafe fn dc_plane_get_status(
    plane_state: *const dc_plane_state,
    flags: dc_plane_status_update_flags,
) -> *const dc_plane_status {
    if plane_state.is_null() || (*plane_state).ctx.is_null() || (*(*plane_state).ctx).dc.is_null() {
        ASSERT(0);
        return core::ptr::null();
    }
    let plane_status = &(*plane_state).status as *const dc_plane_status;
    let dc = (*plane_state).ctx.as_ref().unwrap().dc;
    if (*dc).current_state.is_null() { return core::ptr::null(); }
    let mut i = 0;
    while i < (*dc).res_pool.pipe_count {
        let pipe_ctx = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.plane_state != plane_state { i += 1; continue; }
        if !pipe_ctx.plane_state.is_null() && flags.bits.address { (*pipe_ctx.plane_state).status.is_flip_pending = false; }
        if !pipe_ctx.plane_state.is_null() && flags.bits.histogram { memset(&mut (*pipe_ctx.plane_state).status.cm_hist, 0, core::mem::size_of_val(&(*pipe_ctx.plane_state).status.cm_hist)); }
        break;
    }
    dc_exit_ips_for_hw_access(dc);
    i = 0;
    while i < (*dc).res_pool.pipe_count {
        let pipe_ctx = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.plane_state != plane_state { i += 1; continue; }
        if flags.bits.address { ((*dc).hwss.update_pending_status)(pipe_ctx); }
        if flags.bits.histogram {
            let dpp = pipe_ctx.plane_res.dpp;
            if !dpp.is_null() && !(*dpp).funcs.dpp_cm_hist_read.is_none() { ((*dpp).funcs.dpp_cm_hist_read.unwrap())(dpp, &mut (*pipe_ctx.plane_state).status.cm_hist); }
        }
        i += 1;
    }
    plane_status
}

pub unsafe fn dc_plane_state_retain(plane_state: *mut dc_plane_state) { kref_get(&mut (*plane_state).refcount); }

unsafe fn dc_plane_state_free(kref: *mut kref) {
    let plane_state = container_of!(kref, dc_plane_state, refcount);
    dc_plane_destruct(plane_state);
    kvfree(plane_state as *mut core::ffi::c_void);
}

pub unsafe fn dc_plane_state_release(plane_state: *mut dc_plane_state) { kref_put(&mut (*plane_state).refcount, dc_plane_state_free); }
pub unsafe fn dc_gamma_retain(gamma: *mut dc_gamma) { kref_get(&mut (*gamma).refcount); }
unsafe fn dc_gamma_free(kref: *mut kref) { kvfree(container_of!(kref, dc_gamma, refcount) as *mut core::ffi::c_void); }
pub unsafe fn dc_gamma_release(gamma: *mut *mut dc_gamma) { kref_put(&mut (**gamma).refcount, dc_gamma_free); *gamma = core::ptr::null_mut(); }

pub unsafe fn dc_create_gamma() -> *mut dc_gamma {
    let gamma = kvzalloc_obj::<dc_gamma>();
    if gamma.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*gamma).refcount); gamma
}

pub unsafe fn dc_transfer_func_retain(tf: *mut dc_transfer_func) { kref_get(&mut (*tf).refcount); }
unsafe fn dc_transfer_func_free(kref: *mut kref) { kvfree(container_of!(kref, dc_transfer_func, refcount) as *mut core::ffi::c_void); }
pub unsafe fn dc_transfer_func_release(tf: *mut dc_transfer_func) { kref_put(&mut (*tf).refcount, dc_transfer_func_free); }
pub unsafe fn dc_create_transfer_func() -> *mut dc_transfer_func {
    let tf = kvzalloc_obj::<dc_transfer_func>(); if tf.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*tf).refcount); tf
}

unsafe fn dc_3dlut_func_free(kref: *mut kref) { kvfree(container_of!(kref, dc_3dlut, refcount) as *mut core::ffi::c_void); }
pub unsafe fn dc_create_3dlut_func() -> *mut dc_3dlut {
    let lut = kvzalloc_obj::<dc_3dlut>(); if lut.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*lut).refcount); (*lut).state.raw = 0; lut
}
pub unsafe fn dc_3dlut_func_release(lut: *mut dc_3dlut) { kref_put(&mut (*lut).refcount, dc_3dlut_func_free); }
pub unsafe fn dc_3dlut_func_retain(lut: *mut dc_3dlut) { kref_get(&mut (*lut).refcount); }

unsafe fn dc_plane_cm_free(kref: *mut kref) { kvfree(container_of!(kref, dc_plane_cm, refcount) as *mut core::ffi::c_void); }
pub unsafe fn dc_plane_cm_create() -> *mut dc_plane_cm {
    let cm = kvzalloc_obj::<dc_plane_cm>(); if cm.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*cm).refcount); cm
}
pub unsafe fn dc_plane_cm_release(cm: *mut dc_plane_cm) { kref_put(&mut (*cm).refcount, dc_plane_cm_free); }
pub unsafe fn dc_plane_cm_retain(cm: *mut dc_plane_cm) { kref_get(&mut (*cm).refcount); }

pub unsafe fn dc_plane_force_dcc_and_tiling_disable(plane_state: *mut dc_plane_state, clear_tiling: bool) {
    if plane_state.is_null() { return; }
    let dc = (*plane_state).ctx.as_ref().unwrap().dc;
    if dc.is_null() || (*dc).current_state.is_null() { return; }
    let mut i = 0;
    while i < (*dc).res_pool.pipe_count {
        let pipe_ctx = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize];
        if !pipe_ctx.is_null() { if let Some(f) = (*dc).hwss.clear_surface_dcc_and_tiling { f(pipe_ctx, plane_state, clear_tiling); } }
        i += 1;
    }
}

pub unsafe fn dc_plane_copy_config(dst: *mut dc_plane_state, src: *const dc_plane_state) {
    let mut temp_refcount: kref = core::mem::zeroed();
    memcpy(&mut temp_refcount, &(*dst).refcount, core::mem::size_of::<kref>());
    memcpy(dst, src, core::mem::size_of::<dc_plane_state>());
    memcpy(&mut (*dst).refcount, &temp_refcount, core::mem::size_of::<kref>());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
