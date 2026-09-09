/*
 * Copyright (C) 2018 Intel Corp.
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

// Dependency supplied by the surrounding translation unit: linux/types.h

pub enum drm_atomic_commit {}
pub enum drm_bridge {}
pub enum drm_bridge_state {}
pub enum drm_crtc {}
pub enum drm_crtc_state {}
pub enum drm_plane {}
pub enum drm_plane_state {}
pub enum drm_connector {}
pub enum drm_connector_state {}
pub enum drm_private_obj {}
pub enum drm_private_state {}
pub enum drm_modeset_acquire_ctx {}
pub enum drm_device {}

extern "C" {
    pub fn __drm_atomic_helper_crtc_state_init(state: *mut drm_crtc_state, crtc: *mut drm_crtc);
    pub fn __drm_atomic_helper_crtc_reset(crtc: *mut drm_crtc, state: *mut drm_crtc_state);
    pub fn drm_atomic_helper_crtc_reset(crtc: *mut drm_crtc);
    pub fn drm_atomic_helper_crtc_create_state(crtc: *mut drm_crtc) -> *mut drm_crtc_state;
    pub fn __drm_atomic_helper_crtc_duplicate_state(crtc: *mut drm_crtc, state: *mut drm_crtc_state);
    pub fn drm_atomic_helper_crtc_duplicate_state(crtc: *mut drm_crtc) -> *mut drm_crtc_state;
    pub fn __drm_atomic_helper_crtc_destroy_state(state: *mut drm_crtc_state);
    pub fn drm_atomic_helper_crtc_destroy_state(crtc: *mut drm_crtc, state: *mut drm_crtc_state);

    pub fn __drm_atomic_helper_plane_state_init(state: *mut drm_plane_state, plane: *mut drm_plane);
    pub fn drm_atomic_helper_plane_create_state(plane: *mut drm_plane) -> *mut drm_plane_state;
    pub fn __drm_atomic_helper_plane_reset(plane: *mut drm_plane, state: *mut drm_plane_state);
    pub fn drm_atomic_helper_plane_reset(plane: *mut drm_plane);
    pub fn __drm_atomic_helper_plane_duplicate_state(plane: *mut drm_plane, state: *mut drm_plane_state);
    pub fn drm_atomic_helper_plane_duplicate_state(plane: *mut drm_plane) -> *mut drm_plane_state;
    pub fn __drm_atomic_helper_plane_destroy_state(state: *mut drm_plane_state);
    pub fn drm_atomic_helper_plane_destroy_state(plane: *mut drm_plane, state: *mut drm_plane_state);

    pub fn __drm_atomic_helper_connector_state_init(
        conn_state: *mut drm_connector_state,
        connector: *mut drm_connector,
    );
    pub fn __drm_atomic_helper_connector_reset(
        connector: *mut drm_connector,
        conn_state: *mut drm_connector_state,
    );
    pub fn drm_atomic_helper_connector_reset(connector: *mut drm_connector);
    pub fn drm_atomic_helper_connector_create_state(
        connector: *mut drm_connector,
    ) -> *mut drm_connector_state;
    pub fn drm_atomic_helper_connector_tv_reset(connector: *mut drm_connector);
    pub fn drm_atomic_helper_connector_tv_check(
        connector: *mut drm_connector,
        state: *mut drm_atomic_commit,
    ) -> ::core::ffi::c_int;
    pub fn drm_atomic_helper_connector_tv_margins_reset(connector: *mut drm_connector);
    pub fn __drm_atomic_helper_connector_duplicate_state(
        connector: *mut drm_connector,
        state: *mut drm_connector_state,
    );
    pub fn drm_atomic_helper_connector_duplicate_state(
        connector: *mut drm_connector,
    ) -> *mut drm_connector_state;
    pub fn __drm_atomic_helper_connector_destroy_state(state: *mut drm_connector_state);
    pub fn drm_atomic_helper_connector_destroy_state(
        connector: *mut drm_connector,
        state: *mut drm_connector_state,
    );

    pub fn __drm_atomic_helper_private_obj_create_state(
        obj: *mut drm_private_obj,
        state: *mut drm_private_state,
    );
    pub fn __drm_atomic_helper_private_obj_duplicate_state(
        obj: *mut drm_private_obj,
        state: *mut drm_private_state,
    );

    pub fn __drm_atomic_helper_bridge_duplicate_state(
        bridge: *mut drm_bridge,
        state: *mut drm_bridge_state,
    );
    pub fn drm_atomic_helper_bridge_duplicate_state(
        bridge: *mut drm_bridge,
    ) -> *mut drm_bridge_state;
    pub fn drm_atomic_helper_bridge_destroy_state(
        bridge: *mut drm_bridge,
        state: *mut drm_bridge_state,
    );
    pub fn __drm_atomic_helper_bridge_state_init(
        state: *mut drm_bridge_state,
        bridge: *mut drm_bridge,
    );
    pub fn drm_atomic_helper_bridge_create_state(
        bridge: *mut drm_bridge,
    ) -> *mut drm_bridge_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
