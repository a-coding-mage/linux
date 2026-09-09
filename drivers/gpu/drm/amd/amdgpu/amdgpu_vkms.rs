// SPDX-License-Identifier: GPL-2.0+
// External DRM/AMDGPU declarations are supplied by the surrounding kernel bindings.

static AMDGPU_VKMS_FORMATS: [u32; 1] = [DRM_FORMAT_XRGB8888];

static AMDGPU_VKMS_CRTC_FUNCS: drm_crtc_funcs = drm_crtc_funcs {
    set_config: Some(drm_atomic_helper_set_config), destroy: Some(drm_crtc_cleanup),
    page_flip: Some(drm_atomic_helper_page_flip), reset: Some(drm_atomic_helper_crtc_reset),
    atomic_duplicate_state: Some(drm_atomic_helper_crtc_duplicate_state),
    atomic_destroy_state: Some(drm_atomic_helper_crtc_destroy_state),
    ..DRM_CRTC_VBLANK_TIMER_FUNCS
};
static AMDGPU_VKMS_CRTC_HELPER_FUNCS: drm_crtc_helper_funcs = drm_crtc_helper_funcs {
    ..DRM_CRTC_HELPER_VBLANK_FUNCS
};

unsafe fn amdgpu_vkms_crtc_init(dev: *mut drm_device, crtc: *mut drm_crtc,
                                primary: *mut drm_plane, cursor: *mut drm_plane) -> i32 {
    let adev = drm_to_adev(dev);
    let amdgpu_crtc = to_amdgpu_crtc(crtc);
    let ret = drm_crtc_init_with_planes(dev, crtc, primary, cursor,
                                        &AMDGPU_VKMS_CRTC_FUNCS, core::ptr::null_mut());
    if ret != 0 { DRM_ERROR!("Failed to init CRTC\n"); return ret; }
    drm_crtc_helper_add(crtc, &AMDGPU_VKMS_CRTC_HELPER_FUNCS);
    (*amdgpu_crtc).crtc_id = drm_crtc_index(crtc);
    (*adev).mode_info.crtcs[drm_crtc_index(crtc) as usize] = amdgpu_crtc;
    (*amdgpu_crtc).pll_id = ATOM_PPLL_INVALID;
    (*amdgpu_crtc).encoder = core::ptr::null_mut();
    (*amdgpu_crtc).connector = core::ptr::null_mut();
    ret
}

static AMDGPU_VKMS_CONNECTOR_FUNCS: drm_connector_funcs = drm_connector_funcs {
    fill_modes: Some(drm_helper_probe_single_connector_modes), destroy: Some(drm_connector_cleanup),
    reset: Some(drm_atomic_helper_connector_reset),
    atomic_duplicate_state: Some(drm_atomic_helper_connector_duplicate_state),
    atomic_destroy_state: Some(drm_atomic_helper_connector_destroy_state),
};

unsafe fn amdgpu_vkms_conn_get_modes(connector: *mut drm_connector) -> i32 {
    let dev = (*connector).dev;
    let common_modes: [(i32, i32); 22] = [(640,480),(720,480),(800,600),(848,480),(1024,768),(1152,768),
        (1280,720),(1280,800),(1280,854),(1280,960),(1280,1024),(1440,900),(1400,1050),(1680,1050),
        (1600,1200),(1920,1080),(1920,1200),(2560,1440),(4096,3112),(3656,2664),(3840,2160),(4096,2160)];
    for &(w,h) in &common_modes {
        let mode = drm_cvt_mode(dev, w, h, 60, false, false, false);
        if !mode.is_null() { drm_mode_probed_add(connector, mode); }
    }
    drm_set_preferred_mode(connector, XRES_DEF, YRES_DEF);
    common_modes.len() as i32
}
static AMDGPU_VKMS_CONN_HELPER_FUNCS: drm_connector_helper_funcs = drm_connector_helper_funcs { get_modes: Some(amdgpu_vkms_conn_get_modes) };
static AMDGPU_VKMS_PLANE_FUNCS: drm_plane_funcs = drm_plane_funcs {
    update_plane: Some(drm_atomic_helper_update_plane), disable_plane: Some(drm_atomic_helper_disable_plane),
    destroy: Some(drm_plane_cleanup), reset: Some(drm_atomic_helper_plane_reset),
    atomic_duplicate_state: Some(drm_atomic_helper_plane_duplicate_state),
    atomic_destroy_state: Some(drm_atomic_helper_plane_destroy_state),
};
unsafe fn amdgpu_vkms_plane_atomic_update(_plane: *mut drm_plane, _old_state: *mut drm_atomic_commit) {}
unsafe fn amdgpu_vkms_plane_atomic_check(plane: *mut drm_plane, state: *mut drm_atomic_commit) -> i32 {
    let new_state = drm_atomic_get_new_plane_state(state, plane);
    if (*new_state).fb.is_null() || WARN_ON!((*new_state).crtc.is_null()) { return 0; }
    let crtc_state = drm_atomic_get_crtc_state(state, (*new_state).crtc);
    if IS_ERR!(crtc_state) { return PTR_ERR!(crtc_state); }
    let ret = drm_atomic_helper_check_plane_state(new_state, crtc_state, DRM_PLANE_NO_SCALING, DRM_PLANE_NO_SCALING, false, true);
    if ret != 0 { return ret; }
    if !(*new_state).visible { return -EINVAL; }
    0
}

unsafe fn amdgpu_vkms_prepare_fb(plane: *mut drm_plane, new_state: *mut drm_plane_state) -> i32 {
    if (*new_state).fb.is_null() { DRM_DEBUG_KMS!("No FB bound\n"); return 0; }
    let afb = to_amdgpu_framebuffer((*new_state).fb);
    let obj = drm_gem_fb_get_obj((*new_state).fb, 0);
    if obj.is_null() { DRM_ERROR!("Failed to get obj from framebuffer\n"); return -EINVAL; }
    let rbo = gem_to_amdgpu_bo(obj); let adev = amdgpu_ttm_adev((*rbo).tbo.bdev);
    let mut r = amdgpu_bo_reserve(rbo, true); if r != 0 { dev_err!((*adev).dev, "fail to reserve bo (%d)\n", r); return r; }
    r = dma_resv_reserve_fences((*rbo).tbo.base.resv, TTM_NUM_MOVE_FENCES); if r != 0 { amdgpu_bo_unreserve(rbo); return r; }
    let domain = if (*plane).type_ != DRM_PLANE_TYPE_CURSOR { amdgpu_display_supported_domains(adev, (*rbo).flags) } else { AMDGPU_GEM_DOMAIN_VRAM };
    (*rbo).flags |= AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS;
    r = amdgpu_bo_pin(rbo, domain); if r != 0 { if r != -ERESTARTSYS { DRM_ERROR!("Failed to pin framebuffer with error %d\n", r); } amdgpu_bo_unreserve(rbo); return r; }
    r = amdgpu_ttm_alloc_gart(&mut (*rbo).tbo); if r != 0 { DRM_ERROR!("%p bind failed\n", rbo); amdgpu_bo_unpin(rbo); amdgpu_bo_unreserve(rbo); return r; }
    amdgpu_bo_unreserve(rbo); (*afb).address = amdgpu_bo_gpu_offset(rbo); amdgpu_bo_ref(rbo); 0
}
unsafe fn amdgpu_vkms_cleanup_fb(_plane: *mut drm_plane, old_state: *mut drm_plane_state) {
    if (*old_state).fb.is_null() { return; } let obj = drm_gem_fb_get_obj((*old_state).fb, 0);
    if obj.is_null() { DRM_ERROR!("Failed to get obj from framebuffer\n"); return; }
    let rbo = gem_to_amdgpu_bo(obj); let r = amdgpu_bo_reserve(rbo, false);
    if r != 0 { DRM_ERROR!("failed to reserve rbo before unpin\n"); return; }
    amdgpu_bo_unpin(rbo); amdgpu_bo_unreserve(rbo); amdgpu_bo_unref(&mut (rbo as *mut amdgpu_bo));
}
static AMDGPU_VKMS_PRIMARY_HELPER_FUNCS: drm_plane_helper_funcs = drm_plane_helper_funcs {
    atomic_update: Some(amdgpu_vkms_plane_atomic_update), atomic_check: Some(amdgpu_vkms_plane_atomic_check),
    prepare_fb: Some(amdgpu_vkms_prepare_fb), cleanup_fb: Some(amdgpu_vkms_cleanup_fb),
};
unsafe fn amdgpu_vkms_plane_init(dev: *mut drm_device, type_: drm_plane_type, index: i32) -> *mut drm_plane {
    let plane = kzalloc_obj::<drm_plane>(); if plane.is_null() { return ERR_PTR(-ENOMEM); }
    let ret = drm_universal_plane_init(dev, plane, 1u32 << index, &AMDGPU_VKMS_PLANE_FUNCS, AMDGPU_VKMS_FORMATS.as_ptr(), 1, core::ptr::null(), type_, core::ptr::null());
    if ret != 0 { kfree(plane as *mut core::ffi::c_void); return ERR_PTR(ret); }
    drm_plane_helper_add(plane, &AMDGPU_VKMS_PRIMARY_HELPER_FUNCS); plane
}

static DRM_ENCODER_FUNCS_CLEANUP: drm_encoder_funcs = drm_encoder_funcs { destroy: Some(drm_encoder_cleanup) };
// The remaining output/IP callbacks retain the C driver's direct delegation to external DRM/AMDGPU APIs.
pub const AMDGPU_VKMS_MODE_FUNCS: drm_mode_config_funcs = drm_mode_config_funcs {
    fb_create: Some(amdgpu_display_user_framebuffer_create), atomic_check: Some(drm_atomic_helper_check), atomic_commit: Some(drm_atomic_helper_commit),
};
unsafe fn amdgpu_vkms_hw_fini(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
unsafe fn amdgpu_vkms_is_idle(_ip_block: *mut amdgpu_ip_block) -> bool { true }
unsafe fn amdgpu_vkms_set_clockgating_state(_ip_block: *mut amdgpu_ip_block, _state: amd_clockgating_state) -> i32 { 0 }
unsafe fn amdgpu_vkms_set_powergating_state(_ip_block: *mut amdgpu_ip_block, _state: amd_powergating_state) -> i32 { 0 }

unsafe fn amdgpu_vkms_output_init(dev: *mut drm_device, output: *mut amdgpu_vkms_output, index: i32) -> i32 {
    let connector = &mut (*output).connector; let encoder = &mut (*output).encoder;
    let crtc = &mut (*output).crtc.base; let primary = amdgpu_vkms_plane_init(dev, DRM_PLANE_TYPE_PRIMARY, index);
    if IS_ERR!(primary) { return PTR_ERR!(primary); }
    let mut ret = amdgpu_vkms_crtc_init(dev, crtc, primary, core::ptr::null_mut());
    if ret != 0 { drm_plane_cleanup(primary); return ret; }
    ret = drm_connector_init(dev, connector, &AMDGPU_VKMS_CONNECTOR_FUNCS, DRM_MODE_CONNECTOR_VIRTUAL);
    if ret != 0 { DRM_ERROR!("Failed to init connector\n"); drm_crtc_cleanup(crtc); drm_plane_cleanup(primary); return ret; }
    drm_connector_helper_add(connector, &AMDGPU_VKMS_CONN_HELPER_FUNCS);
    ret = drm_encoder_init(dev, encoder, &DRM_ENCODER_FUNCS_CLEANUP, DRM_MODE_ENCODER_VIRTUAL, core::ptr::null());
    if ret != 0 { DRM_ERROR!("Failed to init encoder\n"); drm_connector_cleanup(connector); drm_crtc_cleanup(crtc); drm_plane_cleanup(primary); return ret; }
    (*encoder).possible_crtcs = 1u32 << index;
    ret = drm_connector_attach_encoder(connector, encoder);
    if ret != 0 { DRM_ERROR!("Failed to attach connector to encoder\n"); drm_encoder_cleanup(encoder); drm_connector_cleanup(connector); drm_crtc_cleanup(crtc); drm_plane_cleanup(primary); return ret; }
    drm_mode_config_reset(dev); 0
}
unsafe fn amdgpu_vkms_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev;
    (*adev).amdgpu_vkms_output = kzalloc_objs::<amdgpu_vkms_output>((*adev).mode_info.num_crtc);
    if (*adev).amdgpu_vkms_output.is_null() { return -ENOMEM; }
    adev_to_drm(adev).max_vblank_count = 0; adev_to_drm(adev).mode_config.funcs = &AMDGPU_VKMS_MODE_FUNCS;
    adev_to_drm(adev).mode_config.max_width = XRES_MAX; adev_to_drm(adev).mode_config.max_height = YRES_MAX;
    adev_to_drm(adev).mode_config.preferred_depth = 24; adev_to_drm(adev).mode_config.prefer_shadow = 1;
    adev_to_drm(adev).mode_config.fb_modifiers_not_supported = true;
    let mut r = amdgpu_display_modeset_create_props(adev); if r != 0 { return r; }
    for i in 0..(*adev).mode_info.num_crtc { r = amdgpu_vkms_output_init(adev_to_drm(adev), &mut (*adev).amdgpu_vkms_output[i as usize], i); if r != 0 { return r; } }
    r = drm_vblank_init(adev_to_drm(adev), (*adev).mode_info.num_crtc); if r != 0 { return r; }
    drm_kms_helper_poll_init(adev_to_drm(adev)); (*adev).mode_info.mode_config_initialized = true; 0
}
unsafe fn amdgpu_vkms_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip_block).adev; drm_kms_helper_poll_fini(adev_to_drm(adev)); drm_mode_config_cleanup(adev_to_drm(adev));
    (*adev).mode_info.mode_config_initialized = false; drm_edid_free((*adev).mode_info.bios_hardcoded_edid); kfree((*adev).amdgpu_vkms_output as *mut _); 0
}
unsafe fn amdgpu_vkms_hw_init(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
unsafe fn amdgpu_vkms_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { drm_mode_config_helper_suspend(adev_to_drm((*ip_block).adev)) }
unsafe fn amdgpu_vkms_resume(ip_block: *mut amdgpu_ip_block) -> i32 { let r = amdgpu_vkms_hw_init(ip_block); if r != 0 { r } else { drm_mode_config_helper_resume(adev_to_drm((*ip_block).adev)) } }

static AMDGPU_VKMS_IP_FUNCS: amd_ip_funcs = amd_ip_funcs {
    name: "amdgpu_vkms", sw_init: Some(amdgpu_vkms_sw_init), sw_fini: Some(amdgpu_vkms_sw_fini),
    hw_init: Some(amdgpu_vkms_hw_init), hw_fini: Some(amdgpu_vkms_hw_fini), suspend: Some(amdgpu_vkms_suspend),
    resume: Some(amdgpu_vkms_resume), is_idle: Some(amdgpu_vkms_is_idle),
    set_clockgating_state: Some(amdgpu_vkms_set_clockgating_state), set_powergating_state: Some(amdgpu_vkms_set_powergating_state),
};
pub static AMDGPU_VKMS_IP_BLOCK: amdgpu_ip_block_version = amdgpu_ip_block_version {
    type_: AMD_IP_BLOCK_TYPE_DCE, major: 1, minor: 0, rev: 0, funcs: &AMDGPU_VKMS_IP_FUNCS,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
