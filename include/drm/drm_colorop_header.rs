/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2023 Advanced Micro Devices, Inc. All rights reserved.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C header dependencies are supplied by other translated headers.

pub const DRM_COLOROP_FLAG_ALLOW_BYPASS: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_colorop_curve_1d_type {
    DRM_COLOROP_1D_CURVE_SRGB_EOTF,
    DRM_COLOROP_1D_CURVE_SRGB_INV_EOTF,
    DRM_COLOROP_1D_CURVE_PQ_125_EOTF,
    DRM_COLOROP_1D_CURVE_PQ_125_INV_EOTF,
    DRM_COLOROP_1D_CURVE_BT2020_INV_OETF,
    DRM_COLOROP_1D_CURVE_BT2020_OETF,
    DRM_COLOROP_1D_CURVE_GAMMA22,
    DRM_COLOROP_1D_CURVE_GAMMA22_INV,
    DRM_COLOROP_1D_CURVE_COUNT,
}

#[repr(C)]
pub struct drm_colorop_state {
    pub colorop: *mut drm_colorop,
    pub bypass: bool,
    pub curve_1d_type: drm_colorop_curve_1d_type,
    pub multiplier: u64,
    pub data: *mut drm_property_blob,
    pub lut1d_interpolation: drm_colorop_lut1d_interpolation_type,
    pub lut3d_interpolation: drm_colorop_lut3d_interpolation_type,
    pub state: *mut drm_atomic_commit,
}

#[repr(C)]
pub struct drm_colorop_funcs {
    pub destroy: Option<unsafe extern "C" fn(colorop: *mut drm_colorop)>,
}

#[repr(C)]
pub struct drm_colorop {
    pub dev: *mut drm_device,
    pub head: list_head,
    pub index: c_uint,
    pub base: drm_mode_object,
    pub plane: *mut drm_plane,
    pub state: *mut drm_colorop_state,
    pub properties: drm_object_properties,
    pub type_: drm_colorop_type,
    pub next: *mut drm_colorop,
    pub type_property: *mut drm_property,
    pub bypass_property: *mut drm_property,
    pub size: u32,
    pub lut1d_interpolation_property: *mut drm_property,
    pub curve_1d_type_property: *mut drm_property,
    pub multiplier_property: *mut drm_property,
    pub size_property: *mut drm_property,
    pub lut3d_interpolation_property: *mut drm_property,
    pub data_property: *mut drm_property,
    pub next_property: *mut drm_property,
    pub funcs: *const drm_colorop_funcs,
}

#[macro_export]
macro_rules! obj_to_colorop {
    ($x:expr) => { container_of!($x, drm_colorop, base) };
}

pub unsafe fn drm_colorop_find(
    dev: *mut drm_device,
    file_priv: *mut drm_file,
    id: u32,
) -> *mut drm_colorop {
    let mo = drm_mode_object_find(dev, file_priv, id, DRM_MODE_OBJECT_COLOROP);
    if !mo.is_null() { obj_to_colorop!(mo) } else { core::ptr::null_mut() }
}

extern "C" {
    pub fn drm_colorop_pipeline_destroy(dev: *mut drm_device);
    pub fn drm_colorop_cleanup(colorop: *mut drm_colorop);
    pub fn drm_plane_colorop_curve_1d_init(dev: *mut drm_device, colorop: *mut drm_colorop, plane: *mut drm_plane, funcs: *const drm_colorop_funcs, supported_tfs: u64, flags: u32) -> c_int;
    pub fn drm_plane_colorop_curve_1d_lut_init(dev: *mut drm_device, colorop: *mut drm_colorop, plane: *mut drm_plane, funcs: *const drm_colorop_funcs, lut_size: u32, interpolation: drm_colorop_lut1d_interpolation_type, flags: u32) -> c_int;
    pub fn drm_plane_colorop_ctm_3x4_init(dev: *mut drm_device, colorop: *mut drm_colorop, plane: *mut drm_plane, funcs: *const drm_colorop_funcs, flags: u32) -> c_int;
    pub fn drm_plane_colorop_mult_init(dev: *mut drm_device, colorop: *mut drm_colorop, plane: *mut drm_plane, funcs: *const drm_colorop_funcs, flags: u32) -> c_int;
    pub fn drm_plane_colorop_3dlut_init(dev: *mut drm_device, colorop: *mut drm_colorop, plane: *mut drm_plane, funcs: *const drm_colorop_funcs, lut_size: u32, interpolation: drm_colorop_lut3d_interpolation_type, flags: u32) -> c_int;
    pub fn drm_atomic_helper_colorop_create_state(colorop: *mut drm_colorop) -> *mut drm_colorop_state;
    pub fn drm_atomic_helper_colorop_duplicate_state(colorop: *mut drm_colorop) -> *mut drm_colorop_state;
    pub fn drm_colorop_atomic_destroy_state(colorop: *mut drm_colorop, state: *mut drm_colorop_state);
    pub fn drm_colorop_reset(colorop: *mut drm_colorop);
    pub fn drm_colorop_destroy(colorop: *mut drm_colorop);
    pub fn drm_get_colorop_type_name(type_: drm_colorop_type) -> *const c_char;
    pub fn drm_get_colorop_curve_1d_type_name(type_: drm_colorop_curve_1d_type) -> *const c_char;
    pub fn drm_get_colorop_lut1d_interpolation_name(type_: drm_colorop_lut1d_interpolation_type) -> *const c_char;
    pub fn drm_get_colorop_lut3d_interpolation_name(type_: drm_colorop_lut3d_interpolation_type) -> *const c_char;
    pub fn drm_colorop_set_next_property(colorop: *mut drm_colorop, next: *mut drm_colorop);
}

pub unsafe fn drm_colorop_index(colorop: *const drm_colorop) -> c_uint { (*colorop).index }

// C macro: list_for_each_entry(colorop, &(dev)->mode_config.colorop_list, head)
// is preserved as a dependency on the translated list iteration primitive.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
