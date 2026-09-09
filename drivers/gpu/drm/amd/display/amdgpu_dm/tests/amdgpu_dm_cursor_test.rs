// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_cursor.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the kernel and amdgpu headers are intentionally
// left as external Rust declarations.

use core::ffi::c_void;

#[repr(C)]
pub struct kunit;
#[repr(C)]
pub struct drm_crtc_state;
#[repr(C)]
pub struct drm_crtc {
    pub index: u32,
}
#[repr(C)]
pub struct dm_crtc_state {
    pub base: drm_crtc_state,
    pub cursor_mode: u32,
}
#[repr(C)]
pub struct drm_atomic_crtc {
    pub old_state: *mut drm_crtc_state,
    pub new_state: *mut drm_crtc_state,
}
#[repr(C)]
pub struct drm_atomic_commit {
    pub crtcs: *mut drm_atomic_crtc,
}
#[repr(C)]
pub struct drm_plane_state {
    pub rotation: u32,
    pub src_w: i32,
    pub src_h: i32,
    pub crtc_w: i32,
    pub crtc_h: i32,
}
#[repr(C)]
pub struct kunit_case;
#[repr(C)]
pub struct kunit_suite {
    pub name: *const u8,
    pub test_cases: *mut kunit_case,
}

pub const DM_CURSOR_NATIVE_MODE: u32 = 0;
pub const DM_CURSOR_OVERLAY_MODE: u32 = 1;
pub const DRM_MODE_ROTATE_0: u32 = 1;
pub const DRM_MODE_ROTATE_90: u32 = 2;
pub const DRM_MODE_ROTATE_180: u32 = 4;
pub const DRM_MODE_ROTATE_270: u32 = 8;

unsafe extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn amdgpu_dm_should_update_native_cursor(
        state: *mut drm_atomic_commit,
        old_crtc: *mut drm_crtc,
        new_crtc: *mut drm_crtc,
        enable: bool,
    ) -> bool;
    fn dm_get_oriented_plane_size(state: *const drm_plane_state, src_w: *mut i32, src_h: *mut i32);
    fn dm_get_plane_scale(state: *const drm_plane_state, scale_w: *mut i32, scale_h: *mut i32);
}

/* Tests for amdgpu_dm_should_update_native_cursor() */

/// dm_test_should_update_native_cursor_without_crtc - Test NULL crtc cases update native cursor
/// @test: The KUnit test context
pub unsafe fn dm_test_should_update_native_cursor_without_crtc(_test: *mut kunit) {
    assert!(amdgpu_dm_should_update_native_cursor(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), false));
    assert!(amdgpu_dm_should_update_native_cursor(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), true));
}

/// dm_test_should_update_native_cursor_disable_native - Test disable path reads old crtc cursor mode
/// @test: The KUnit test context
pub unsafe fn dm_test_should_update_native_cursor_disable_native(test: *mut kunit) {
    let state = kunit_kzalloc(test, core::mem::size_of::<drm_atomic_commit>(), 0) as *mut drm_atomic_commit;
    assert!(!state.is_null());
    let crtc = kunit_kzalloc(test, core::mem::size_of::<drm_crtc>(), 0) as *mut drm_crtc;
    assert!(!crtc.is_null());
    let dm_state = kunit_kzalloc(test, core::mem::size_of::<dm_crtc_state>(), 0) as *mut dm_crtc_state;
    assert!(!dm_state.is_null());
    (*state).crtcs = kunit_kzalloc(test, core::mem::size_of::<drm_atomic_crtc>(), 0) as *mut drm_atomic_crtc;
    assert!(!(*state).crtcs.is_null());
    (*crtc).index = 0;
    (*dm_state).cursor_mode = DM_CURSOR_NATIVE_MODE;
    (*(*state).crtcs).old_state = &mut (*dm_state).base;
    assert!(amdgpu_dm_should_update_native_cursor(state, crtc, core::ptr::null_mut(), false));
}

/// dm_test_should_update_native_cursor_enable_overlay - Test enable path reads new crtc cursor mode
/// @test: The KUnit test context
pub unsafe fn dm_test_should_update_native_cursor_enable_overlay(test: *mut kunit) {
    let state = kunit_kzalloc(test, core::mem::size_of::<drm_atomic_commit>(), 0) as *mut drm_atomic_commit;
    assert!(!state.is_null());
    let crtc = kunit_kzalloc(test, core::mem::size_of::<drm_crtc>(), 0) as *mut drm_crtc;
    assert!(!crtc.is_null());
    let dm_state = kunit_kzalloc(test, core::mem::size_of::<dm_crtc_state>(), 0) as *mut dm_crtc_state;
    assert!(!dm_state.is_null());
    (*state).crtcs = kunit_kzalloc(test, core::mem::size_of::<drm_atomic_crtc>(), 0) as *mut drm_atomic_crtc;
    assert!(!(*state).crtcs.is_null());
    (*crtc).index = 0;
    (*dm_state).cursor_mode = DM_CURSOR_OVERLAY_MODE;
    (*(*state).crtcs).new_state = &mut (*dm_state).base;
    assert!(!amdgpu_dm_should_update_native_cursor(state, core::ptr::null_mut(), crtc, true));
}

/* Tests for dm_get_oriented_plane_size() */

unsafe fn oriented_test(rotation: u32, expected_w: i32, expected_h: i32, test: *mut kunit) {
    let mut plane_state = drm_plane_state { rotation, src_w: 1920 << 16, src_h: 1080 << 16, crtc_w: 0, crtc_h: 0 };
    let mut src_w = 0;
    let mut src_h = 0;
    dm_get_oriented_plane_size(&plane_state, &mut src_w, &mut src_h);
    assert_eq!(src_w, expected_w);
    assert_eq!(src_h, expected_h);
    let _ = test;
}

pub unsafe fn dm_test_oriented_plane_size_rotate_0(test: *mut kunit) { oriented_test(DRM_MODE_ROTATE_0, 1920, 1080, test); }
pub unsafe fn dm_test_oriented_plane_size_rotate_90(test: *mut kunit) { oriented_test(DRM_MODE_ROTATE_90, 1080, 1920, test); }
pub unsafe fn dm_test_oriented_plane_size_rotate_180(test: *mut kunit) { oriented_test(DRM_MODE_ROTATE_180, 1920, 1080, test); }
pub unsafe fn dm_test_oriented_plane_size_rotate_270(test: *mut kunit) { oriented_test(DRM_MODE_ROTATE_270, 1080, 1920, test); }

/* Tests for dm_get_plane_scale() */

unsafe fn scale_test(rotation: u32, src_w: i32, src_h: i32, crtc_w: i32, crtc_h: i32, expected_w: i32, expected_h: i32) {
    let plane_state = drm_plane_state { rotation, src_w, src_h, crtc_w, crtc_h };
    let mut scale_w = 0;
    let mut scale_h = 0;
    dm_get_plane_scale(&plane_state, &mut scale_w, &mut scale_h);
    assert_eq!(scale_w, expected_w);
    assert_eq!(scale_h, expected_h);
}

pub unsafe fn dm_test_get_plane_scale_identity(_test: *mut kunit) { scale_test(DRM_MODE_ROTATE_0, 1920 << 16, 1080 << 16, 1920, 1080, 1000, 1000); }
pub unsafe fn dm_test_get_plane_scale_rotate_90_identity(_test: *mut kunit) { scale_test(DRM_MODE_ROTATE_90, 1920 << 16, 1080 << 16, 1080, 1920, 1000, 1000); }
pub unsafe fn dm_test_get_plane_scale_zero_src_width(_test: *mut kunit) { scale_test(DRM_MODE_ROTATE_0, 0, 1080 << 16, 100, 200, 0, 185); }

// KUnit case registration and module metadata are provided by the kernel build system.
pub static mut amdgpu_dm_cursor_tests: [Option<unsafe fn(*mut kunit)>; 10] = [
    Some(dm_test_should_update_native_cursor_without_crtc), Some(dm_test_should_update_native_cursor_disable_native), Some(dm_test_should_update_native_cursor_enable_overlay),
    Some(dm_test_oriented_plane_size_rotate_0), Some(dm_test_oriented_plane_size_rotate_90), Some(dm_test_oriented_plane_size_rotate_180), Some(dm_test_oriented_plane_size_rotate_270),
    Some(dm_test_get_plane_scale_identity), Some(dm_test_get_plane_scale_rotate_90_identity), Some(dm_test_get_plane_scale_zero_src_width),
];

pub const MODULE_AUTHOR: &str = "AMD";
pub const MODULE_DESCRIPTION: &str = "KUnit tests for amdgpu_dm_cursor";
pub const MODULE_LICENSE: &str = "Dual MIT/GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
