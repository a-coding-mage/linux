// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit test helpers for amdgpu_dm tests.
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Kernel and DRM dependencies are supplied by the surrounding translation unit.

pub unsafe fn dm_kunit_alloc_adev(test: *mut kunit) -> *mut amdgpu_device {
    let mut drm: *mut drm_device;
    let dev: *mut device;

    dev = drm_kunit_helper_alloc_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, dev);

    drm = __drm_kunit_helper_alloc_drm_device(
        test,
        dev,
        core::mem::size_of::<amdgpu_device>(),
        core::mem::offset_of!(amdgpu_device, ddev),
        DRIVER_MODESET | DRIVER_ATOMIC,
    );
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, drm);

    drm_to_adev(drm)
}

pub unsafe fn dm_kunit_alloc_link(test: *mut kunit) -> *mut dc_link {
    let link: *mut dc_link;

    link = kunit_kzalloc(test, core::mem::size_of::<dc_link>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, link);

    link
}

pub unsafe fn dm_kunit_alloc_dc_with_ctx(test: *mut kunit) -> *mut dc {
    let ctx: *mut dc_context;
    let dc: *mut dc;

    dc = kunit_kzalloc(test, core::mem::size_of::<dc>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, dc);

    ctx = kunit_kzalloc(test, core::mem::size_of::<dc_context>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, ctx);

    (*dc).ctx = ctx;
    (*ctx).dc = dc;

    dc
}

pub unsafe fn dm_kunit_alloc_link_with_ctx(test: *mut kunit) -> *mut dc_link {
    let link: *mut dc_link;
    let dc: *mut dc;

    link = dm_kunit_alloc_link(test);
    dc = dm_kunit_alloc_dc_with_ctx(test);
    (*link).ctx = (*dc).ctx;

    link
}

pub unsafe fn dm_kunit_alloc_dm(test: *mut kunit) -> *mut amdgpu_display_manager {
    let dm: *mut amdgpu_display_manager;
    let dc: *mut dc;
    let state: *mut dc_state;

    dm = kunit_kzalloc(test, core::mem::size_of::<amdgpu_display_manager>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, dm);

    dc = kunit_kzalloc(test, core::mem::size_of::<dc>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, dc);

    state = kunit_kzalloc(test, core::mem::size_of::<dc_state>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, state);

    (*dm).dc = dc;
    (*dc).current_state = state;

    dm
}

pub unsafe fn dm_kunit_alloc_stream(
    test: *mut kunit,
    link: *mut dc_link,
) -> *mut dc_stream_state {
    let stream: *mut dc_stream_state;

    stream = kunit_kzalloc(test, core::mem::size_of::<dc_stream_state>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, stream);

    (*stream).link = link;
    kref_init(&mut (*stream).refcount);

    stream
}

pub unsafe fn dm_kunit_alloc_dc_state(test: *mut kunit) -> *mut dc_state {
    let state: *mut dc_state;

    state = kunit_kzalloc(test, core::mem::size_of::<dc_state>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, state);

    state
}

pub unsafe fn dm_kunit_alloc_clk_mgr(test: *mut kunit) -> *mut clk_mgr {
    let clk_mgr: *mut clk_mgr;
    let funcs: *mut clk_mgr_funcs;

    clk_mgr = kunit_kzalloc(test, core::mem::size_of::<clk_mgr>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, clk_mgr);

    funcs = kunit_kzalloc(test, core::mem::size_of::<clk_mgr_funcs>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, funcs);

    (*clk_mgr).funcs = funcs;

    clk_mgr
}

pub unsafe fn dm_kunit_add_stream_to_state(
    test: *mut kunit,
    state: *mut dc_state,
    index: u32,
    link: *mut dc_link,
) {
    let stream: *mut dc_stream_state;

    KUNIT_ASSERT_LT!(test, index, MAX_PIPES as u32);

    stream = kunit_kzalloc(test, core::mem::size_of::<dc_stream_state>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, stream);

    (*stream).link = link;
    (*state).streams[index as usize] = stream;
    if (*state).stream_count <= index {
        (*state).stream_count = index + 1;
    }
}

pub unsafe fn dm_kunit_alloc_connector(
    test: *mut kunit,
    adev: *mut amdgpu_device,
    link: *mut dc_link,
) -> *mut amdgpu_dm_connector {
    let aconnector: *mut amdgpu_dm_connector;

    aconnector = drmm_kzalloc(
        adev_to_drm(adev),
        core::mem::size_of::<amdgpu_dm_connector>(),
        GFP_KERNEL,
    );
    KUNIT_ASSERT_NOT_NULL!(test, aconnector);

    if !adev.is_null() {
        (*aconnector).base.dev = &mut (*adev).ddev;
    }
    (*aconnector).dc_link = link;

    aconnector
}

pub unsafe fn dm_kunit_alloc_drm_with_connector_list(test: *mut kunit) -> *mut drm_device {
    let dev: *mut drm_device;

    dev = kunit_kzalloc(test, core::mem::size_of::<drm_device>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, dev);

    INIT_LIST_HEAD(&mut (*dev).mode_config.connector_list);
    spin_lock_init(&mut (*dev).mode_config.connector_list_lock);

    dev
}

// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("KUnit test helpers for amdgpu_dm tests");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
