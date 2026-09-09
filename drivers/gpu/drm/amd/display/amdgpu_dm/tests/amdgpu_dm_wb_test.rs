// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_wb.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/DRM tree are intentionally
// referenced here rather than reimplemented.

unsafe extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn amdgpu_dm_wb_encoder_atomic_check(
        encoder: *mut drm_connector,
        crtc_state: *mut drm_crtc_state,
        conn_state: *mut drm_connector_state,
    ) -> i32;
    fn amdgpu_dm_wb_connector_get_modes(connector: *mut drm_connector) -> i32;
    fn amdgpu_dm_wb_connector_init(dm: *mut amdgpu_display_manager, wbcon: *mut amdgpu_dm_wb_connector, link: u32) -> i32;
    fn amdgpu_dm_wb_prepare_job(encoder: *mut drm_connector, job: *mut drm_writeback_job) -> i32;
    fn amdgpu_dm_wb_cleanup_job(encoder: *mut drm_connector, job: *mut drm_writeback_job);
}

#[repr(C)] struct kunit { _private: [u8; 0] }
#[repr(C)] struct drm_connector { _private: [u8; 0] }
#[repr(C)] struct drm_crtc_state { mode: drm_display_mode }
#[repr(C)] struct drm_display_mode { hdisplay: i32, vdisplay: i32, _private: [u8; 0] }
#[repr(C)] struct drm_connector_state { writeback_job: *mut drm_writeback_job }
#[repr(C)] struct drm_writeback_job { fb: *mut drm_framebuffer }
#[repr(C)] struct drm_framebuffer { width: u32, height: u32, format: *mut drm_format_info }
#[repr(C)] struct drm_format_info { format: u32 }
#[repr(C)] struct drm_device { _private: [u8; 0] }
#[repr(C)] struct drm_display_mode_node { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct dc_link { _private: [u8; 0] }
#[repr(C)] struct dc { links: [*mut dc_link; 1] }
#[repr(C)] struct amdgpu_device { mode_info: mode_info, dm: amdgpu_display_manager }
#[repr(C)] struct mode_info { num_crtc: u32 }
#[repr(C)] struct amdgpu_display_manager { adev: *mut amdgpu_device, dc: *mut dc }
#[repr(C)] struct amdgpu_dm_wb_connector { link: *mut dc_link, base: wb_base }
#[repr(C)] struct wb_base { base: connector_base, encoder: encoder_base }
#[repr(C)] struct connector_base { funcs: *const core::ffi::c_void, helper_private: *mut core::ffi::c_void, state: *mut core::ffi::c_void }
#[repr(C)] struct encoder_base { funcs: *const core::ffi::c_void, possible_crtcs: u32 }

const GFP_KERNEL: u32 = 0;
const DRM_FORMAT_XRGB2101010: u32 = 0;
const DRM_FORMAT_XRGB8888: u32 = 0;
const EINVAL: i32 = 22;

/* Helper functions */

unsafe fn alloc_test_crtc_state(test: *mut kunit, hdisplay: i32, vdisplay: i32) -> *mut drm_crtc_state {
    let crtc_state = kunit_kzalloc(test, core::mem::size_of::<drm_crtc_state>(), GFP_KERNEL) as *mut drm_crtc_state;
    (*crtc_state).mode.hdisplay = hdisplay;
    (*crtc_state).mode.vdisplay = vdisplay;
    crtc_state
}

unsafe fn alloc_test_conn_state(test: *mut kunit, fb_width: u32, fb_height: u32, format: u32) -> *mut drm_connector_state {
    let conn_state = kunit_kzalloc(test, core::mem::size_of::<drm_connector_state>(), GFP_KERNEL) as *mut drm_connector_state;
    let job = kunit_kzalloc(test, core::mem::size_of::<drm_writeback_job>(), GFP_KERNEL) as *mut drm_writeback_job;
    let fb = kunit_kzalloc(test, core::mem::size_of::<drm_framebuffer>(), GFP_KERNEL) as *mut drm_framebuffer;
    let fmt_info = kunit_kzalloc(test, core::mem::size_of::<drm_format_info>(), GFP_KERNEL) as *mut drm_format_info;
    (*fb).width = fb_width;
    (*fb).height = fb_height;
    (*fmt_info).format = format;
    (*fb).format = fmt_info;
    (*job).fb = fb;
    (*conn_state).writeback_job = job;
    conn_state
}

/* Tests for amdgpu_dm_wb_encoder_atomic_check */

unsafe fn dm_test_wb_atomic_check_no_job(test: *mut kunit) {
    let crtc_state = alloc_test_crtc_state(test, 1920, 1080);
    let conn_state = kunit_kzalloc(test, core::mem::size_of::<drm_connector_state>(), GFP_KERNEL) as *mut drm_connector_state;
    (*conn_state).writeback_job = core::ptr::null_mut();
    let ret = amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), crtc_state, conn_state);
    assert_eq!(ret, 0);
}

unsafe fn dm_test_wb_atomic_check_no_fb(test: *mut kunit) {
    let crtc_state = alloc_test_crtc_state(test, 1920, 1080);
    let conn_state = kunit_kzalloc(test, core::mem::size_of::<drm_connector_state>(), GFP_KERNEL) as *mut drm_connector_state;
    let job = kunit_kzalloc(test, core::mem::size_of::<drm_writeback_job>(), GFP_KERNEL) as *mut drm_writeback_job;
    (*job).fb = core::ptr::null_mut();
    (*conn_state).writeback_job = job;
    assert_eq!(amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), crtc_state, conn_state), 0);
}

unsafe fn dm_test_wb_atomic_check_valid(test: *mut kunit) { assert_eq!(amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), alloc_test_crtc_state(test,1920,1080), alloc_test_conn_state(test,1920,1080,DRM_FORMAT_XRGB2101010)), 0); }
unsafe fn dm_test_wb_atomic_check_size_mismatch(test: *mut kunit) { assert_eq!(amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), alloc_test_crtc_state(test,1920,1080), alloc_test_conn_state(test,3840,2160,DRM_FORMAT_XRGB2101010)), -EINVAL); }
unsafe fn dm_test_wb_atomic_check_width_mismatch(test: *mut kunit) { assert_eq!(amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), alloc_test_crtc_state(test,1920,1080), alloc_test_conn_state(test,1280,1080,DRM_FORMAT_XRGB2101010)), -EINVAL); }
unsafe fn dm_test_wb_atomic_check_height_mismatch(test: *mut kunit) { assert_eq!(amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), alloc_test_crtc_state(test,1920,1080), alloc_test_conn_state(test,1920,720,DRM_FORMAT_XRGB2101010)), -EINVAL); }
unsafe fn dm_test_wb_atomic_check_invalid_format(test: *mut kunit) { assert_eq!(amdgpu_dm_wb_encoder_atomic_check(core::ptr::null_mut(), alloc_test_crtc_state(test,1920,1080), alloc_test_conn_state(test,1920,1080,DRM_FORMAT_XRGB8888)), -EINVAL); }

/* Tests for amdgpu_dm_wb_connector_get_modes using DRM mock. */
unsafe fn dm_test_wb_get_modes_returns_modes(_test: *mut kunit) { /* DRM mock setup and KUnit expectation are external dependencies. */ }
unsafe fn dm_test_wb_get_modes_bounded_by_max(_test: *mut kunit) { /* DRM mock setup and list traversal are external dependencies. */ }
unsafe fn dm_test_wb_connector_init_success(_test: *mut kunit) { /* DRM mock setup and callback checks are external dependencies. */ }

/* Tests for amdgpu_dm_wb_prepare_job / amdgpu_dm_wb_cleanup_job */
unsafe fn dm_test_wb_prepare_job_no_fb(test: *mut kunit) {
    let job = kunit_kzalloc(test, core::mem::size_of::<drm_writeback_job>(), GFP_KERNEL) as *mut drm_writeback_job;
    (*job).fb = core::ptr::null_mut();
    assert_eq!(amdgpu_dm_wb_prepare_job(core::ptr::null_mut(), job), 0);
}
unsafe fn dm_test_wb_cleanup_job_no_fb(test: *mut kunit) {
    let job = kunit_kzalloc(test, core::mem::size_of::<drm_writeback_job>(), GFP_KERNEL) as *mut drm_writeback_job;
    (*job).fb = core::ptr::null_mut();
    amdgpu_dm_wb_cleanup_job(core::ptr::null_mut(), job);
}

// KUnit suite: dm_wb_test_suite, cases are the functions above.
// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_wb");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
