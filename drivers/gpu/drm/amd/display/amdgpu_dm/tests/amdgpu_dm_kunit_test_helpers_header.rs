/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * KUnit test helpers for amdgpu_dm tests.
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// The corresponding C header includes <kunit/test.h> for this opaque type.
// These declarations rely on the translated KUnit definitions supplied by
// the surrounding build.

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_display_manager {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_dm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_mgr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dm_kunit_alloc_adev(test: *mut kunit) -> *mut amdgpu_device;
    pub fn dm_kunit_alloc_dc_with_ctx(test: *mut kunit) -> *mut dc;
    pub fn dm_kunit_alloc_link(test: *mut kunit) -> *mut dc_link;
    pub fn dm_kunit_alloc_link_with_ctx(test: *mut kunit) -> *mut dc_link;
    pub fn dm_kunit_alloc_dm(test: *mut kunit) -> *mut amdgpu_display_manager;
    pub fn dm_kunit_alloc_drm_with_connector_list(test: *mut kunit) -> *mut drm_device;
    pub fn dm_kunit_alloc_dc_state(test: *mut kunit) -> *mut dc_state;
    pub fn dm_kunit_alloc_clk_mgr(test: *mut kunit) -> *mut clk_mgr;
    pub fn dm_kunit_alloc_stream(test: *mut kunit, link: *mut dc_link) -> *mut dc_stream_state;
    pub fn dm_kunit_add_stream_to_state(
        test: *mut kunit,
        state: *mut dc_state,
        index: ::core::ffi::c_uint,
        link: *mut dc_link,
    );
    pub fn dm_kunit_alloc_connector(
        test: *mut kunit,
        adev: *mut amdgpu_device,
        link: *mut dc_link,
    ) -> *mut amdgpu_dm_connector;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
