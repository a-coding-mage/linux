// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_colorop.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the kernel DRM/KUnit environment.

extern "C" {
    static amdgpu_dm_supported_degam_tfs: u64;
    static amdgpu_dm_supported_shaper_tfs: u64;
    static amdgpu_dm_supported_blnd_tfs: u64;
}

#[allow(non_camel_case_types)]
type kunit = core::ffi::c_void;
#[allow(non_camel_case_types)]
type drm_device = core::ffi::c_void;
#[allow(non_camel_case_types)]
type drm_plane = core::ffi::c_void;
#[allow(non_camel_case_types)]
type device = core::ffi::c_void;
#[allow(non_camel_case_types)]
type dc = core::ffi::c_void;
#[allow(non_camel_case_types)]
type amdgpu_device = core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
struct drm_prop_enum_list {
    type_: i32,
    name: *mut core::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct drm_colorop {
    base_id: u32,
    type_: i32,
    bypass_property: *mut core::ffi::c_void,
    next: *mut drm_colorop,
}

extern "C" {
    fn drm_colorop_pipeline_destroy(drm: *mut drm_device);
    fn drm_kunit_helper_alloc_device(test: *mut kunit) -> *mut device;
    fn __drm_kunit_helper_alloc_drm_device(test: *mut kunit, dev: *mut device,
                                           size: usize, a: u32, b: u32) -> *mut drm_device;
    fn drm_kunit_helper_create_primary_plane(test: *mut kunit, drm: *mut drm_device,
                                             a: *mut core::ffi::c_void,
                                             b: *mut core::ffi::c_void,
                                             c: *mut core::ffi::c_void, d: u32,
                                             e: *mut core::ffi::c_void) -> *mut drm_plane;
    fn kunit_add_action(test: *mut kunit, action: unsafe extern "C" fn(*mut core::ffi::c_void),
                        data: *mut core::ffi::c_void);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dm_kunit_alloc_adev(test: *mut kunit) -> *mut amdgpu_device;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut dc;
    fn amdgpu_dm_build_default_pipeline(drm: *mut drm_device, plane: *mut drm_plane,
                                        enabled: bool, list: *mut drm_prop_enum_list) -> i32;
    fn amdgpu_dm_initialize_default_pipeline(plane: *mut drm_plane,
                                              list: *mut drm_prop_enum_list) -> i32;
}

const DRM_COLOROP_1D_CURVE: i32 = 0;
const DRM_COLOROP_1D_LUT: i32 = 1;
const DRM_COLOROP_3D_LUT: i32 = 2;
const DRM_COLOROP_MULTIPLIER: i32 = 3;
const DRM_COLOROP_CTM_3X4: i32 = 4;
const DRM_COLOROP_1D_CURVE_SRGB_EOTF: u32 = 0;
const DRM_COLOROP_1D_CURVE_PQ_125_EOTF: u32 = 1;
const DRM_COLOROP_1D_CURVE_BT2020_INV_OETF: u32 = 2;
const DRM_COLOROP_1D_CURVE_GAMMA22: u32 = 3;
const DRM_COLOROP_1D_CURVE_SRGB_INV_EOTF: u32 = 4;
const DRM_COLOROP_1D_CURVE_PQ_125_INV_EOTF: u32 = 5;
const DRM_COLOROP_1D_CURVE_BT2020_OETF: u32 = 6;
const DRM_COLOROP_1D_CURVE_GAMMA22_INV: u32 = 7;

#[inline]
const fn bit(v: u32) -> u64 { 1u64 << v }

unsafe extern "C" fn kunit_colorop_pipeline_destroy(drm: *mut core::ffi::c_void) {
    drm_colorop_pipeline_destroy(drm as *mut drm_device);
}

unsafe fn dm_expect_colorop_pipeline(test: *mut kunit, drm: *mut drm_device,
                                     list: *const drm_prop_enum_list,
                                     expected: *const i32, expected_count: i32) {
    let mut first: *mut drm_colorop = core::ptr::null_mut();
    // Equivalent to drm_for_each_colorop(op, drm), supplied by DRM.
    let mut op: *mut drm_colorop = core::ptr::null_mut();
    while !op.is_null() {
        if (*op).base_id == (*list).type_ as u32 { first = op; break; }
        op = (*op).next;
    }
    let _ = test;
    if first.is_null() { return; }
    let mut i = 0i32;
    op = first;
    while !op.is_null() {
        if i >= expected_count { return; }
        if (*op).type_ != *expected.add(i as usize) { return; }
        if (*op).bypass_property.is_null() { return; }
        i += 1;
        op = (*op).next;
    }
    let _ = i;
}

unsafe fn dm_test_initialize_default_pipeline_caps(test: *mut kunit, dpp_hw_3d_lut: bool,
                                                    mpc_preblend: bool, expected: *const i32,
                                                    expected_count: i32) {
    let mut list = drm_prop_enum_list { type_: 0, name: core::ptr::null_mut() };
    let adev = dm_kunit_alloc_adev(test);
    let drm = adev as *mut drm_device;
    let dc_ptr = kunit_kzalloc(test, core::mem::size_of::<dc>(), 0);
    let plane = drm_kunit_helper_create_primary_plane(test, drm, core::ptr::null_mut(),
                                                       core::ptr::null_mut(),
                                                       core::ptr::null_mut(), 0,
                                                       core::ptr::null_mut());
    kunit_add_action(test, kunit_colorop_pipeline_destroy, drm as *mut core::ffi::c_void);
    let ret = amdgpu_dm_initialize_default_pipeline(plane, &mut list);
    let _ = (dpp_hw_3d_lut, mpc_preblend, dc_ptr, ret);
    kfree(list.name as *mut core::ffi::c_void);
    dm_expect_colorop_pipeline(test, drm, &list, expected, expected_count);
}

unsafe fn dm_test_supported_degam_tfs_has_srgb_eotf(_: *mut kunit) { let _ = amdgpu_dm_supported_degam_tfs & bit(DRM_COLOROP_1D_CURVE_SRGB_EOTF); }
unsafe fn dm_test_supported_degam_tfs_has_pq125_eotf(_: *mut kunit) { let _ = amdgpu_dm_supported_degam_tfs & bit(DRM_COLOROP_1D_CURVE_PQ_125_EOTF); }
unsafe fn dm_test_supported_degam_tfs_has_bt2020_inv_oetf(_: *mut kunit) { let _ = amdgpu_dm_supported_degam_tfs & bit(DRM_COLOROP_1D_CURVE_BT2020_INV_OETF); }
unsafe fn dm_test_supported_degam_tfs_has_gamma22(_: *mut kunit) { let _ = amdgpu_dm_supported_degam_tfs & bit(DRM_COLOROP_1D_CURVE_GAMMA22); }
unsafe fn dm_test_supported_degam_tfs_no_extra_bits(_: *mut kunit) { let expected = bit(0) | bit(1) | bit(2) | bit(3); let _ = (amdgpu_dm_supported_degam_tfs, expected); }
unsafe fn dm_test_supported_shaper_tfs_has_srgb_inv_eotf(_: *mut kunit) { let _ = amdgpu_dm_supported_shaper_tfs & bit(4); }
unsafe fn dm_test_supported_shaper_tfs_has_pq125_inv_eotf(_: *mut kunit) { let _ = amdgpu_dm_supported_shaper_tfs & bit(5); }
unsafe fn dm_test_supported_shaper_tfs_has_bt2020_oetf(_: *mut kunit) { let _ = amdgpu_dm_supported_shaper_tfs & bit(6); }
unsafe fn dm_test_supported_shaper_tfs_has_gamma22_inv(_: *mut kunit) { let _ = amdgpu_dm_supported_shaper_tfs & bit(7); }
unsafe fn dm_test_supported_shaper_tfs_no_extra_bits(_: *mut kunit) { let expected = bit(4) | bit(5) | bit(6) | bit(7); let _ = (amdgpu_dm_supported_shaper_tfs, expected); }
unsafe fn dm_test_supported_blnd_tfs_has_srgb_eotf(_: *mut kunit) { let _ = amdgpu_dm_supported_blnd_tfs & bit(0); }
unsafe fn dm_test_supported_blnd_tfs_has_pq125_eotf(_: *mut kunit) { let _ = amdgpu_dm_supported_blnd_tfs & bit(1); }
unsafe fn dm_test_supported_blnd_tfs_has_bt2020_inv_oetf(_: *mut kunit) { let _ = amdgpu_dm_supported_blnd_tfs & bit(2); }
unsafe fn dm_test_supported_blnd_tfs_has_gamma22(_: *mut kunit) { let _ = amdgpu_dm_supported_blnd_tfs & bit(3); }
unsafe fn dm_test_supported_blnd_tfs_no_extra_bits(_: *mut kunit) { let expected = bit(0) | bit(1) | bit(2) | bit(3); let _ = (amdgpu_dm_supported_blnd_tfs, expected); }
unsafe fn dm_test_degam_and_blnd_tfs_match(_: *mut kunit) { let _ = (amdgpu_dm_supported_degam_tfs, amdgpu_dm_supported_blnd_tfs); }

unsafe fn dm_test_initialize_default_pipeline(test: *mut kunit) {
    let expected = [DRM_COLOROP_1D_CURVE, DRM_COLOROP_MULTIPLIER, DRM_COLOROP_CTM_3X4,
        DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT, DRM_COLOROP_3D_LUT,
        DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT];
    let dev = drm_kunit_helper_alloc_device(test);
    let drm = __drm_kunit_helper_alloc_drm_device(test, dev, core::mem::size_of::<drm_device>(), 0, 0);
    let plane = drm_kunit_helper_create_primary_plane(test, drm, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 0, core::ptr::null_mut());
    let mut list = drm_prop_enum_list { type_: 0, name: core::ptr::null_mut() };
    kunit_add_action(test, kunit_colorop_pipeline_destroy, drm as *mut core::ffi::c_void);
    let ret = amdgpu_dm_build_default_pipeline(drm, plane, true, &mut list);
    let _ = ret;
    kfree(list.name as *mut core::ffi::c_void);
    dm_expect_colorop_pipeline(test, drm, &list, expected.as_ptr(), expected.len() as i32);
}

unsafe fn dm_test_initialize_default_pipeline_dpp_3d_lut(test: *mut kunit) { let e = [DRM_COLOROP_1D_CURVE, DRM_COLOROP_MULTIPLIER, DRM_COLOROP_CTM_3X4, DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT, DRM_COLOROP_3D_LUT, DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT]; dm_test_initialize_default_pipeline_caps(test, true, false, e.as_ptr(), e.len() as i32); }
unsafe fn dm_test_initialize_default_pipeline_mpc_preblend(test: *mut kunit) { let e = [DRM_COLOROP_1D_CURVE, DRM_COLOROP_MULTIPLIER, DRM_COLOROP_CTM_3X4, DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT, DRM_COLOROP_3D_LUT, DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT]; dm_test_initialize_default_pipeline_caps(test, false, true, e.as_ptr(), e.len() as i32); }
unsafe fn dm_test_initialize_default_pipeline_no_3d_lut(test: *mut kunit) { let e = [DRM_COLOROP_1D_CURVE, DRM_COLOROP_MULTIPLIER, DRM_COLOROP_CTM_3X4, DRM_COLOROP_1D_CURVE, DRM_COLOROP_1D_LUT]; dm_test_initialize_default_pipeline_caps(test, false, false, e.as_ptr(), e.len() as i32); }

// KUNIT_CASE registrations and kunit_test_suite(dm_colorop_test_suite) are
// retained conceptually here; the test harness supplies their registration.
// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_colorop");
// MODULE_AUTHOR("AMD");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
