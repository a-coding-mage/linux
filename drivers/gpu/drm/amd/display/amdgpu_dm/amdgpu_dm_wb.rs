// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel/Rust translation.

static AMDGPU_DM_WB_FORMATS: [u32; 1] = [DRM_FORMAT_XRGB2101010];

unsafe extern "C" fn amdgpu_dm_wb_encoder_atomic_check(
    _encoder: *mut drm_encoder,
    crtc_state: *mut drm_crtc_state,
    conn_state: *mut drm_connector_state,
) -> i32 {
    let mode = &(*crtc_state).mode;
    let mut found = false;

    if (*conn_state).writeback_job.is_null()
        || (*(*conn_state).writeback_job).fb.is_null()
    {
        return 0;
    }

    let fb = (*(*conn_state).writeback_job).fb;
    if (*fb).width != mode.hdisplay || (*fb).height != mode.vdisplay {
        DRM_DEBUG_KMS!("Invalid framebuffer size %ux%u\n", (*fb).width, (*fb).height);
        return -EINVAL;
    }

    let mut i: u8 = 0;
    while (i as usize) < AMDGPU_DM_WB_FORMATS.len() {
        if (*(*fb).format).format == AMDGPU_DM_WB_FORMATS[i as usize] {
            found = true;
            break;
        }
        i = i.wrapping_add(1);
    }

    if !found {
        DRM_DEBUG_KMS!("Invalid pixel format %p4cc\n", &(*(*fb).format).format);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn amdgpu_dm_wb_connector_get_modes(
    connector: *mut drm_connector,
) -> i32 {
    // Maximum resolution supported by DWB
    drm_add_modes_noedid(connector, 3840, 2160)
}

unsafe extern "C" fn amdgpu_dm_wb_prepare_job(
    _wb_connector: *mut drm_writeback_connector,
    job: *mut drm_writeback_job,
) -> i32 {
    if (*job).fb.is_null() {
        DRM_DEBUG_KMS!("No FB bound\n");
        return 0;
    }

    let afb = to_amdgpu_framebuffer((*job).fb);
    let obj = (*(*job).fb).obj[0];
    let rbo = gem_to_amdgpu_bo(obj);
    let adev = amdgpu_ttm_adev((*rbo).tbo.bdev);

    let mut r = amdgpu_bo_reserve(rbo, true);
    if r != 0 {
        drm_err!(adev_to_drm(adev), "fail to reserve bo: %pe\n", ERR_PTR(r));
        return r;
    }

    r = dma_resv_reserve_fences((*rbo).tbo.base.resv, TTM_NUM_MOVE_FENCES);
    if r != 0 {
        drm_err!(adev_to_drm(adev), "reserving fence slot failed: %pe\n", ERR_PTR(r));
        amdgpu_bo_unreserve(rbo);
        return r;
    }

    let domain = amdgpu_display_supported_domains(adev, (*rbo).flags);
    (*rbo).flags |= AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS;
    r = amdgpu_bo_pin(rbo, domain);
    if r != 0 {
        if r != -ERESTARTSYS {
            DRM_ERROR!("Failed to pin framebuffer: %pe\n", ERR_PTR(r));
        }
        amdgpu_bo_unreserve(rbo);
        return r;
    }

    r = amdgpu_ttm_alloc_gart(&mut (*rbo).tbo);
    if r != 0 {
        DRM_ERROR!("%p bind failed: %pe\n", rbo, ERR_PTR(r));
        amdgpu_bo_unpin(rbo);
        amdgpu_bo_unreserve(rbo);
        return r;
    }

    amdgpu_bo_unreserve(rbo);
    (*afb).address = amdgpu_bo_gpu_offset(rbo);
    amdgpu_bo_ref(rbo);
    0
}

unsafe extern "C" fn amdgpu_dm_wb_cleanup_job(
    _connector: *mut drm_writeback_connector,
    job: *mut drm_writeback_job,
) {
    if (*job).fb.is_null() {
        return;
    }

    let mut rbo = gem_to_amdgpu_bo((*(*job).fb).obj[0]);
    let r = amdgpu_bo_reserve(rbo, false);
    if r != 0 {
        DRM_ERROR!("failed to reserve rbo before unpin: %pe\n", ERR_PTR(r));
        return;
    }

    amdgpu_bo_unpin(rbo);
    amdgpu_bo_unreserve(rbo);
    amdgpu_bo_unref(&mut rbo);
}

static AMDGPU_DM_WB_ENCODER_HELPER_FUNCS: drm_encoder_helper_funcs = drm_encoder_helper_funcs {
    atomic_check: Some(amdgpu_dm_wb_encoder_atomic_check),
};

static AMDGPU_DM_WB_CONNECTOR_FUNCS: drm_connector_funcs = drm_connector_funcs {
    fill_modes: Some(drm_helper_probe_single_connector_modes),
    destroy: Some(drm_connector_cleanup),
    reset: Some(amdgpu_dm_connector_funcs_reset),
    atomic_duplicate_state: Some(amdgpu_dm_connector_atomic_duplicate_state),
    atomic_destroy_state: Some(drm_atomic_helper_connector_destroy_state),
};

static AMDGPU_DM_WB_CONN_HELPER_FUNCS: drm_connector_helper_funcs = drm_connector_helper_funcs {
    get_modes: Some(amdgpu_dm_wb_connector_get_modes),
    prepare_writeback_job: Some(amdgpu_dm_wb_prepare_job),
    cleanup_writeback_job: Some(amdgpu_dm_wb_cleanup_job),
};

unsafe extern "C" fn amdgpu_dm_wb_connector_init(
    dm: *mut amdgpu_display_manager,
    wbcon: *mut amdgpu_dm_wb_connector,
    link_index: u32,
) -> i32 {
    let dc = (*dm).dc;
    let link = dc_get_link_at_index(dc, link_index);
    (*wbcon).link = link;

    drm_connector_helper_add(&mut (*wbcon).base.base, &AMDGPU_DM_WB_CONN_HELPER_FUNCS);

    let res = drm_writeback_connector_init(
        &mut (*(*dm).adev).ddev,
        &mut (*wbcon).base,
        &AMDGPU_DM_WB_CONNECTOR_FUNCS,
        &AMDGPU_DM_WB_ENCODER_HELPER_FUNCS,
        AMDGPU_DM_WB_FORMATS.as_ptr(),
        AMDGPU_DM_WB_FORMATS.len(),
        amdgpu_dm_get_encoder_crtc_mask((*dm).adev),
    );
    if res != 0 {
        return res;
    }

    // Some of the properties below require access to state, like bpc.
    // Allocate some default initial connector state with our reset helper.
    if let Some(reset) = (*(*wbcon).base.base.funcs).reset {
        reset(&mut (*wbcon).base.base);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
