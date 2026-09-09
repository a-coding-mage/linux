// SPDX-License-Identifier: MIT
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 * Authors: AMD
 */

// Kernel and DRM dependencies supplied by other translation units.

pub const MAX_COLOR_PIPELINE_OPS: usize = 10;
pub const LUT3D_SIZE: u32 = 17;

pub static amdgpu_dm_supported_degam_tfs: u64 =
    (1u64 << DRM_COLOROP_1D_CURVE_SRGB_EOTF) |
    (1u64 << DRM_COLOROP_1D_CURVE_PQ_125_EOTF) |
    (1u64 << DRM_COLOROP_1D_CURVE_BT2020_INV_OETF) |
    (1u64 << DRM_COLOROP_1D_CURVE_GAMMA22);

pub static amdgpu_dm_supported_shaper_tfs: u64 =
    (1u64 << DRM_COLOROP_1D_CURVE_SRGB_INV_EOTF) |
    (1u64 << DRM_COLOROP_1D_CURVE_PQ_125_INV_EOTF) |
    (1u64 << DRM_COLOROP_1D_CURVE_BT2020_OETF) |
    (1u64 << DRM_COLOROP_1D_CURVE_GAMMA22_INV);

pub static amdgpu_dm_supported_blnd_tfs: u64 =
    (1u64 << DRM_COLOROP_1D_CURVE_SRGB_EOTF) |
    (1u64 << DRM_COLOROP_1D_CURVE_PQ_125_EOTF) |
    (1u64 << DRM_COLOROP_1D_CURVE_BT2020_INV_OETF) |
    (1u64 << DRM_COLOROP_1D_CURVE_GAMMA22);

static dm_colorop_funcs: drm_colorop_funcs = drm_colorop_funcs {
    destroy: Some(drm_colorop_destroy),
};

unsafe fn amdgpu_dm_build_default_pipeline(
    dev: *mut drm_device,
    plane: *mut drm_plane,
    hw_3d_lut: bool,
    list: *mut drm_prop_enum_list,
) -> i32 {
    let mut ops: [*mut drm_colorop; MAX_COLOR_PIPELINE_OPS] =
        [core::ptr::null_mut(); MAX_COLOR_PIPELINE_OPS];
    let mut ret: i32;
    let mut i: usize = 0;

    /* 1D curve - DEGAM TF */
    ops[i] = kzalloc_obj::<drm_colorop>();
    if ops[i].is_null() {
        ret = -ENOMEM;
        goto_cleanup!(cleanup);
    }

    ret = drm_plane_colorop_curve_1d_init(
        dev, ops[i], plane, &dm_colorop_funcs,
        amdgpu_dm_supported_degam_tfs, DRM_COLOROP_FLAG_ALLOW_BYPASS,
    );
    if ret != 0 { goto_cleanup!(cleanup); }

    (*list).type_ = (*ops[i]).base.id;
    i += 1;

    /* Multiplier */
    ops[i] = kzalloc_obj::<drm_colorop>();
    if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
    ret = drm_plane_colorop_mult_init(dev, ops[i], plane, &dm_colorop_funcs,
                                      DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if ret != 0 { goto_cleanup!(cleanup); }
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    i += 1;

    /* 3x4 matrix */
    ops[i] = kzalloc_obj::<drm_colorop>();
    if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
    ret = drm_plane_colorop_ctm_3x4_init(dev, ops[i], plane, &dm_colorop_funcs,
                                         DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if ret != 0 { goto_cleanup!(cleanup); }
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    i += 1;

    if hw_3d_lut {
        /* 1D curve - SHAPER TF */
        ops[i] = kzalloc_obj::<drm_colorop>();
        if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
        ret = drm_plane_colorop_curve_1d_init(dev, ops[i], plane, &dm_colorop_funcs,
                                              amdgpu_dm_supported_shaper_tfs,
                                              DRM_COLOROP_FLAG_ALLOW_BYPASS);
        if ret != 0 { goto_cleanup!(cleanup); }
        drm_colorop_set_next_property(ops[i - 1], ops[i]);
        i += 1;

        /* 1D LUT - SHAPER LUT */
        ops[i] = kzalloc_obj::<drm_colorop>();
        if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
        ret = drm_plane_colorop_curve_1d_lut_init(dev, ops[i], plane, &dm_colorop_funcs,
                                                  MAX_COLOR_LUT_ENTRIES,
                                                  DRM_COLOROP_LUT1D_INTERPOLATION_LINEAR,
                                                  DRM_COLOROP_FLAG_ALLOW_BYPASS);
        if ret != 0 { goto_cleanup!(cleanup); }
        drm_colorop_set_next_property(ops[i - 1], ops[i]);
        i += 1;

        /* 3D LUT */
        ops[i] = kzalloc_obj::<drm_colorop>();
        if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
        ret = drm_plane_colorop_3dlut_init(dev, ops[i], plane, &dm_colorop_funcs,
                                           LUT3D_SIZE,
                                           DRM_COLOROP_LUT3D_INTERPOLATION_TETRAHEDRAL,
                                           DRM_COLOROP_FLAG_ALLOW_BYPASS);
        if ret != 0 { goto_cleanup!(cleanup); }
        drm_colorop_set_next_property(ops[i - 1], ops[i]);
        i += 1;
    }

    /* 1D curve - BLND TF */
    ops[i] = kzalloc_obj::<drm_colorop>();
    if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
    ret = drm_plane_colorop_curve_1d_init(dev, ops[i], plane, &dm_colorop_funcs,
                                          amdgpu_dm_supported_blnd_tfs,
                                          DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if ret != 0 { goto_cleanup!(cleanup); }
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    i += 1;

    /* 1D LUT - BLND LUT */
    ops[i] = kzalloc_obj::<drm_colorop>();
    if ops[i].is_null() { ret = -ENOMEM; goto_cleanup!(cleanup); }
    ret = drm_plane_colorop_curve_1d_lut_init(dev, ops[i], plane, &dm_colorop_funcs,
                                              MAX_COLOR_LUT_ENTRIES,
                                              DRM_COLOROP_LUT1D_INTERPOLATION_LINEAR,
                                              DRM_COLOROP_FLAG_ALLOW_BYPASS);
    if ret != 0 { goto_cleanup!(cleanup); }
    drm_colorop_set_next_property(ops[i - 1], ops[i]);
    (*list).name = kasprintf(GFP_KERNEL, c"Color Pipeline %d", (*ops[0]).base.id);
    return 0;

cleanup:
    if ret == -ENOMEM { drm_err(dev, c"KMS: Failed to allocate colorop\n"); }
    drm_colorop_pipeline_destroy(dev);
    ret
}

pub unsafe fn amdgpu_dm_initialize_default_pipeline(
    plane: *mut drm_plane,
    list: *mut drm_prop_enum_list,
) -> i32 {
    let dev = (*plane).dev;
    let adev = drm_to_adev(dev);
    let hw_3d_lut = (*(*(*adev).dm).dc).caps.color.dpp.hw_3d_lut ||
                    (*(*(*adev).dm).dc).caps.color.mpc.preblend;
    amdgpu_dm_build_default_pipeline(dev, plane, hw_3d_lut, list)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
