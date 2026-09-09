/*
 * Copyright (C) 2014 Red Hat
 * Copyright (C) 2014 Intel Corp.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors:
 * Rob Clark <robdclark@gmail.com>
 * Daniel Vetter <daniel.vetter@ffwll.ch>
 */

// Dependencies supplied by the surrounding DRM translation.

pub const DRM_PLANE_NO_SCALING: i32 = 1 << 16;

pub const DRM_PLANE_COMMIT_ACTIVE_ONLY: u32 = BIT(0);
pub const DRM_PLANE_COMMIT_NO_DISABLE_AFTER_MODESET: u32 = BIT(1);

#[repr(C)]
pub struct drm_atomic_commit;
#[repr(C)]
pub struct drm_private_obj;
#[repr(C)]
pub struct drm_private_state;
#[repr(C)]
pub struct drm_device;
#[repr(C)]
pub struct drm_connector;
#[repr(C)]
pub struct drm_plane_state;
#[repr(C)]
pub struct drm_crtc_state;
#[repr(C)]
pub struct drm_plane;
#[repr(C)]
pub struct drm_crtc;
#[repr(C)]
pub struct drm_framebuffer;
#[repr(C)]
pub struct drm_modeset_acquire_ctx;
#[repr(C)]
pub struct drm_mode_set;
#[repr(C)]
pub struct drm_pending_vblank_event;
#[repr(C)]
pub struct drm_bridge;
#[repr(C)]
pub struct drm_bridge_state;
#[repr(C)]
pub struct drm_connector_state;

extern "C" {
    pub fn drm_atomic_helper_check_modeset(dev: *mut drm_device, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_check_wb_connector_state(connector: *mut drm_connector, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_check_plane_state(plane_state: *mut drm_plane_state, crtc_state: *const drm_crtc_state, min_scale: i32, max_scale: i32, can_position: bool, can_update_disabled: bool) -> i32;
    pub fn drm_atomic_helper_check_planes(dev: *mut drm_device, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_check_crtc_primary_plane(crtc_state: *mut drm_crtc_state) -> i32;
    pub fn drm_atomic_helper_commit_encoder_bridge_disable(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_crtc_disable(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_encoder_bridge_post_disable(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_check(dev: *mut drm_device, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_commit_tail(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_tail_rpm(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit(dev: *mut drm_device, state: *mut drm_atomic_commit, nonblock: bool) -> i32;
    pub fn drm_atomic_helper_async_check(dev: *mut drm_device, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_async_commit(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_wait_for_fences(dev: *mut drm_device, state: *mut drm_atomic_commit, pre_swap: bool) -> i32;
    pub fn drm_atomic_helper_wait_for_vblanks(dev: *mut drm_device, old_state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_wait_for_flip_done(dev: *mut drm_device, old_state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_update_legacy_modeset_state(dev: *mut drm_device, old_state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_calc_timestamping_constants(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_crtc_set_mode(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_modeset_disables(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_writebacks(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_encoder_bridge_pre_enable(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_crtc_enable(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_encoder_bridge_enable(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_modeset_enables(dev: *mut drm_device, old_state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_prepare_planes(dev: *mut drm_device, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_unprepare_planes(dev: *mut drm_device, state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_planes(dev: *mut drm_device, state: *mut drm_atomic_commit, flags: u32);
    pub fn drm_atomic_helper_cleanup_planes(dev: *mut drm_device, old_state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_planes_on_crtc(old_crtc_state: *mut drm_crtc_state);
    pub fn drm_atomic_helper_disable_planes_on_crtc(old_crtc_state: *mut drm_crtc_state, atomic: bool);
    pub fn drm_atomic_helper_swap_state(state: *mut drm_atomic_commit, stall: bool) -> i32;
    pub fn drm_atomic_helper_setup_commit(state: *mut drm_atomic_commit, nonblock: bool) -> i32;
    pub fn drm_atomic_helper_wait_for_dependencies(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_fake_vblank(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_hw_done(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_commit_cleanup_done(state: *mut drm_atomic_commit);
    pub fn drm_atomic_helper_update_plane(plane: *mut drm_plane, crtc: *mut drm_crtc, fb: *mut drm_framebuffer, crtc_x: i32, crtc_y: i32, crtc_w: u32, crtc_h: u32, src_x: u32, src_y: u32, src_w: u32, src_h: u32, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_disable_plane(plane: *mut drm_plane, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_set_config(set: *mut drm_mode_set, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_disable_all(dev: *mut drm_device, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_reset_crtc(crtc: *mut drm_crtc, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_shutdown(dev: *mut drm_device);
    pub fn drm_atomic_helper_duplicate_state(dev: *mut drm_device, ctx: *mut drm_modeset_acquire_ctx) -> *mut drm_atomic_commit;
    pub fn drm_atomic_helper_suspend(dev: *mut drm_device) -> *mut drm_atomic_commit;
    pub fn drm_atomic_helper_commit_duplicated_state(state: *mut drm_atomic_commit, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_resume(dev: *mut drm_device, state: *mut drm_atomic_commit) -> i32;
    pub fn drm_atomic_helper_page_flip(crtc: *mut drm_crtc, fb: *mut drm_framebuffer, event: *mut drm_pending_vblank_event, flags: u32, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_page_flip_target(crtc: *mut drm_crtc, fb: *mut drm_framebuffer, event: *mut drm_pending_vblank_event, flags: u32, target: u32, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_atomic_helper_bridge_propagate_bus_fmt(bridge: *mut drm_bridge, bridge_state: *mut drm_bridge_state, crtc_state: *mut drm_crtc_state, conn_state: *mut drm_connector_state, output_fmt: u32, num_input_fmts: *mut u32) -> *mut u32;
    pub fn drm_atomic_helper_bridge_get_hdmi_output_bus_fmts(bridge: *mut drm_bridge, bridge_state: *mut drm_bridge_state, crtc_state: *mut drm_crtc_state, conn_state: *mut drm_connector_state, num_output_fmts: *mut u32) -> *mut u32;
}

/// Checks whether a plane is being enabled.
pub unsafe fn drm_atomic_plane_enabling(old_plane_state: *mut drm_plane_state, new_plane_state: *mut drm_plane_state) -> bool {
    WARN_ON(((*new_plane_state).crtc.is_null() && !(*new_plane_state).fb.is_null()) ||
            (!(*new_plane_state).crtc.is_null() && (*new_plane_state).fb.is_null()));
    (*old_plane_state).crtc.is_null() && !(*new_plane_state).crtc.is_null()
}

/// Checks whether a plane is being disabled.
pub unsafe fn drm_atomic_plane_disabling(old_plane_state: *mut drm_plane_state, new_plane_state: *mut drm_plane_state) -> bool {
    WARN_ON(((*new_plane_state).crtc.is_null() && !(*new_plane_state).fb.is_null()) ||
            (!(*new_plane_state).crtc.is_null() && (*new_plane_state).fb.is_null()));
    !(*old_plane_state).crtc.is_null() && (*new_plane_state).crtc.is_null()
}

// C iteration macros, retained as Rust macro forms for use with the translated DRM helpers.
#[macro_export]
macro_rules! drm_atomic_crtc_for_each_plane {
    ($plane:ident, $crtc:expr) => { drm_for_each_plane_mask!($plane, (*$crtc).dev, (*$crtc).state.plane_mask) };
}
#[macro_export]
macro_rules! drm_atomic_crtc_state_for_each_plane {
    ($plane:ident, $crtc_state:expr) => { drm_for_each_plane_mask!($plane, (*$crtc_state).state.dev, (*$crtc_state).plane_mask) };
}
#[macro_export]
macro_rules! drm_atomic_crtc_state_for_each_plane_state {
    ($plane:ident, $plane_state:ident, $crtc_state:expr) => {
        drm_for_each_plane_mask!($plane, (*$crtc_state).state.dev, (*$crtc_state).plane_mask);
        for_each_if!($plane_state = __drm_atomic_get_current_plane_state((*$crtc_state).state, $plane));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
