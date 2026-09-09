/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/**************************************************************************
 *
 * Copyright (c) 2018 VMware, Inc., Palo Alto, CA., USA
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors:
 * Deepak Rawat <drawat@vmware.com>
 *
 **************************************************************************/

// Dependency declarations supplied by drm/drm_atomic_helper.h remain external.

/**
 * drm_atomic_for_each_plane_damage - Iterator macro for plane damage.
 * @iter: The iterator to advance.
 * @rect: Return a rectangle in fb coordinate clipped to plane src.
 *
 * Note that if the first call to iterator macro return false then no need to do
 * plane update. Iterator will return full plane src when damage is not passed
 * by user-space.
 */
#[macro_export]
macro_rules! drm_atomic_for_each_plane_damage {
    ($iter:expr, $rect:expr) => {
        while drm_atomic_helper_damage_iter_next($iter, $rect)
    };
}

/**
 * struct drm_atomic_helper_damage_iter - Closure structure for damage iterator.
 *
 * This structure tracks state needed to walk the list of plane damage clips.
 */
#[repr(C)]
pub struct drm_atomic_helper_damage_iter {
    /* private: Plane src in whole number. */
    pub plane_src: drm_rect,
    /* private: Rectangles in plane damage blob. */
    pub clips: *const drm_rect,
    /* private: Number of rectangles in plane damage blob. */
    pub num_clips: u32,
    /* private: Current clip iterator is advancing on. */
    pub curr_clip: u32,
    /* private: Whether need full plane update. */
    pub full_update: bool,
}

extern "C" {
    pub fn drm_atomic_helper_check_plane_damage(
        state: *mut drm_atomic_commit,
        plane_state: *mut drm_plane_state,
    );
    pub fn drm_atomic_helper_dirtyfb(
        fb: *mut drm_framebuffer,
        file_priv: *mut drm_file,
        flags: core::ffi::c_uint,
        color: core::ffi::c_uint,
        clips: *mut drm_clip_rect,
        num_clips: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn drm_atomic_helper_damage_iter_init(
        iter: *mut drm_atomic_helper_damage_iter,
        old_state: *const drm_plane_state,
        new_state: *const drm_plane_state,
    );
    pub fn drm_atomic_helper_damage_iter_next(
        iter: *mut drm_atomic_helper_damage_iter,
        rect: *mut drm_rect,
    ) -> bool;
    pub fn drm_atomic_helper_damage_merged(
        old_state: *const drm_plane_state,
        state: *const drm_plane_state,
        rect: *mut drm_rect,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
