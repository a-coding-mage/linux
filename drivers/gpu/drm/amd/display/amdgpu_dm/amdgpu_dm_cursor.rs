// SPDX-License-Identifier: MIT
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// External DRM, display, and AMDGPU declarations are supplied by other units.

unsafe fn dm_check_cursor_fb(
    new_acrtc: *mut amdgpu_crtc,
    new_plane_state: *mut drm_plane_state,
    fb: *mut drm_framebuffer,
) -> i32 {
    let adev = drm_to_adev((*new_acrtc).base.dev);
    let afb = to_amdgpu_framebuffer(fb);
    let pitch: u32;
    let linear: bool;

    if (*fb).width > (*new_acrtc).max_cursor_width ||
       (*fb).height > (*new_acrtc).max_cursor_height {
        drm_dbg_atomic(adev_to_drm(adev), "Bad cursor FB size %dx%d\n",
                       (*(*new_plane_state).fb).width,
                       (*(*new_plane_state).fb).height);
        return -EINVAL;
    }
    if (*new_plane_state).src_w != ((*fb).width << 16) ||
       (*new_plane_state).src_h != ((*fb).height << 16) {
        drm_dbg_atomic(adev_to_drm(adev), "Cropping not supported for cursor plane\n");
        return -EINVAL;
    }

    // Pitch in pixels
    pitch = (*fb).pitches[0] / (*(*fb).format).cpp[0];
    if (*fb).width != pitch {
        drm_dbg_atomic(adev_to_drm(adev), "Cursor FB width %d doesn't match pitch %d",
                       (*fb).width, pitch);
        return -EINVAL;
    }
    match pitch {
        64 | 128 | 256 => (),
        _ => {
            drm_dbg_atomic(adev_to_drm(adev), "Bad cursor FB pitch %d px\n", pitch);
            return -EINVAL;
        }
    }

    // Core DRM checks FB modifiers; check tiling flags only without a modifier.
    if ((*fb).flags & DRM_MODE_FB_MODIFIERS) == 0 {
        if (*adev).family == AMDGPU_FAMILY_GC_12_0_0 {
            linear = AMDGPU_TILING_GET((*afb).tiling_flags, GFX12_SWIZZLE_MODE) == 0;
        } else if *adev.family == AMDGPU_FAMILY_GC_13_0_1 {
            linear = AMDGPU_TILING_GET((*afb).tiling_flags, GFX12_SWIZZLE_MODE) == 0;
        } else if *adev.family >= AMDGPU_FAMILY_AI {
            linear = AMDGPU_TILING_GET((*afb).tiling_flags, SWIZZLE_MODE) == 0;
        } else {
            linear = AMDGPU_TILING_GET((*afb).tiling_flags, ARRAY_MODE) != DC_ARRAY_2D_TILED_THIN1 &&
                     AMDGPU_TILING_GET((*afb).tiling_flags, ARRAY_MODE) != DC_ARRAY_1D_TILED_THIN1 &&
                     AMDGPU_TILING_GET((*afb).tiling_flags, MICRO_TILE_MODE) == 0;
        }
        if !linear {
            drm_dbg_atomic(adev_to_drm(adev), "Cursor FB not linear");
            return -EINVAL;
        }
    }
    0
}

pub unsafe fn amdgpu_dm_check_native_cursor_state(
    new_plane_crtc: *mut drm_crtc, plane: *mut drm_plane,
    new_plane_state: *mut drm_plane_state, enable: bool,
) -> i32 {
    if !enable || new_plane_crtc.is_null() ||
       drm_atomic_plane_disabling((*plane).state, new_plane_state) { return 0; }
    let new_acrtc = to_amdgpu_crtc(new_plane_crtc);
    if (*new_plane_state).src_x != 0 || (*new_plane_state).src_y != 0 {
        drm_dbg_atomic((*new_plane_crtc).dev, "Cropping not supported for cursor plane\n");
        return -EINVAL;
    }
    if !(*new_plane_state).fb.is_null() {
        let ret = dm_check_cursor_fb(new_acrtc, new_plane_state, (*new_plane_state).fb);
        if ret != 0 { return ret; }
    }
    0
}

pub unsafe fn amdgpu_dm_should_update_native_cursor(
    state: *mut drm_atomic_commit, old_plane_crtc: *mut drm_crtc,
    new_plane_crtc: *mut drm_crtc, enable: bool,
) -> bool {
    if !enable {
        if old_plane_crtc.is_null() { return true; }
        let old_crtc_state = drm_atomic_get_old_crtc_state(state, old_plane_crtc);
        return (*to_dm_crtc_state(old_crtc_state)).cursor_mode == DM_CURSOR_NATIVE_MODE;
    }
    if new_plane_crtc.is_null() { return true; }
    let new_crtc_state = drm_atomic_get_new_crtc_state(state, new_plane_crtc);
    (*to_dm_crtc_state(new_crtc_state)).cursor_mode == DM_CURSOR_NATIVE_MODE
}

unsafe fn dm_get_oriented_plane_size(plane_state: *mut drm_plane_state, src_w: *mut i32, src_h: *mut i32) {
    match (*plane_state).rotation & DRM_MODE_ROTATE_MASK {
        DRM_MODE_ROTATE_90 | DRM_MODE_ROTATE_270 => { *src_w = (*plane_state).src_h >> 16; *src_h = (*plane_state).src_w >> 16; }
        _ => { *src_w = (*plane_state).src_w >> 16; *src_h = (*plane_state).src_h >> 16; }
    }
}

unsafe fn dm_get_plane_scale(plane_state: *mut drm_plane_state, out_plane_scale_w: *mut i32, out_plane_scale_h: *mut i32) {
    let mut plane_src_w = 0; let mut plane_src_h = 0;
    dm_get_oriented_plane_size(plane_state, &mut plane_src_w, &mut plane_src_h);
    *out_plane_scale_w = if plane_src_w != 0 { (*plane_state).crtc_w * 1000 / plane_src_w } else { 0 };
    *out_plane_scale_h = if plane_src_h != 0 { (*plane_state).crtc_h * 1000 / plane_src_h } else { 0 };
}

unsafe fn dm_plane_color_pipeline_active(state: *mut drm_atomic_commit, plane: *mut drm_plane, use_old: bool) -> bool {
    let mut colorop = core::ptr::null_mut();
    let mut old_colorop_state = core::ptr::null_mut();
    let mut new_colorop_state = core::ptr::null_mut();
    let mut i = 0;
    for_each_oldnew_colorop_in_state!(state, colorop, old_colorop_state, new_colorop_state, i, {
        let cstate = if use_old { old_colorop_state } else { new_colorop_state };
        if (*cstate).colorop.plane != plane { continue; }
        if !(*cstate).bypass { return true; }
    });
    false
}

pub unsafe fn amdgpu_dm_crtc_get_cursor_mode(adev: *mut amdgpu_device, state: *mut drm_atomic_commit, dm_crtc_state: *mut dm_crtc_state, cursor_mode: *mut amdgpu_dm_cursor_mode) -> i32 {
    let crtc_state = &mut (*dm_crtc_state).base;
    if amdgpu_ip_version(adev, DCE_HWIP, 0) == IP_VERSION(4, 0, 1) ||
       amdgpu_ip_version(adev, DCE_HWIP, 0) == IP_VERSION(4, 2, 0) ||
       amdgpu_ip_version(adev, DCE_HWIP, 0) == IP_VERSION(4, 2, 1) ||
       !crtc_state.enable {
        *cursor_mode = DM_CURSOR_NATIVE_MODE; return 0;
    }
    *cursor_mode = (*dm_crtc_state).cursor_mode;
    let mut consider_mode_change = false; let mut entire_crtc_covered = false; let mut cursor_changed = false;
    let mut plane = core::ptr::null_mut(); let mut old_plane_state = core::ptr::null_mut(); let mut plane_state = core::ptr::null_mut(); let mut i = 0;
    for_each_oldnew_plane_in_state!(state, plane, old_plane_state, plane_state, i, {
        if (drm_plane_mask(plane) & crtc_state.plane_mask) == 0 { continue; }
        if (*plane).type_ == DRM_PLANE_TYPE_CURSOR { cursor_changed = true; }
        if drm_atomic_plane_enabling(old_plane_state, plane_state) || drm_atomic_plane_disabling(old_plane_state, plane_state) || (*old_plane_state).fb.format != (*plane_state).fb.format { consider_mode_change = true; break; }
        let (mut nw, mut nh, mut ow, mut oh) = (0,0,0,0); dm_get_plane_scale(plane_state, &mut nw, &mut nh); dm_get_plane_scale(old_plane_state, &mut ow, &mut oh);
        if nw != ow || nh != oh { consider_mode_change = true; break; }
        if (*plane).type_ != DRM_PLANE_TYPE_CURSOR && ((*old_plane_state).crtc_x != (*plane_state).crtc_x || (*old_plane_state).crtc_y != (*plane_state).crtc_y || (*old_plane_state).crtc_w != (*plane_state).crtc_w || (*old_plane_state).crtc_h != (*plane_state).crtc_h) { consider_mode_change = true; break; }
        if dm_plane_color_pipeline_active(state, plane, true) != dm_plane_color_pipeline_active(state, plane, false) { consider_mode_change = true; break; }
    });
    if !consider_mode_change && !crtc_state.zpos_changed { return 0; }
    if !cursor_changed && (drm_plane_mask((*crtc_state.crtc).cursor) & crtc_state.plane_mask) == 0 { return 0; }
    let cursor_state = drm_atomic_get_plane_state(state, (*crtc_state.crtc).cursor);
    if IS_ERR(cursor_state) { return PTR_ERR(cursor_state); }
    if (*cursor_state).fb.is_null() { return 0; }
    let mut cursor_scale_w = 0; let mut cursor_scale_h = 0;
    for_each_oldnew_plane_in_descending_zpos!(state, plane, old_plane_state, plane_state, {
        if (drm_plane_mask(plane) & crtc_state.plane_mask) == 0 || (*plane).type_ == DRM_PLANE_TYPE_CURSOR { continue; }
        if amdgpu_dm_plane_is_video_format((*(*plane_state).fb).format.format) || dm_plane_color_pipeline_active(state, plane, false) { *cursor_mode = DM_CURSOR_OVERLAY_MODE; return 0; }
        let (mut underlying_scale_w, mut underlying_scale_h) = (0,0); dm_get_plane_scale(plane_state, &mut underlying_scale_w, &mut underlying_scale_h); dm_get_plane_scale(cursor_state, &mut cursor_scale_w, &mut cursor_scale_h);
        if cursor_scale_w != underlying_scale_w && cursor_scale_h != underlying_scale_h { *cursor_mode = DM_CURSOR_OVERLAY_MODE; return 0; }
        if (*plane_state).crtc_x <= 0 && (*plane_state).crtc_y <= 0 && (*plane_state).crtc_x + (*plane_state).crtc_w >= crtc_state.mode.hdisplay && (*plane_state).crtc_y + (*plane_state).crtc_h >= crtc_state.mode.vdisplay { entire_crtc_covered = true; break; }
    });
    *cursor_mode = if entire_crtc_covered { DM_CURSOR_NATIVE_MODE } else { DM_CURSOR_OVERLAY_MODE }; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
