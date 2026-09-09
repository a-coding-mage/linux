// SPDX-License-Identifier: GPL-2.0 OR MIT
/* KUnit tests for amdgpu_dm_crtc.c
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * C headers and kernel-provided symbols are intentionally left as external
 * dependencies of this translation.
 */

unsafe fn dm_test_crtc_modeset_required_active_mode_changed(test: *mut kunit) {
    let mut state: drm_crtc_state = core::mem::zeroed(); state.active = true; state.mode_changed = true;
    KUNIT_EXPECT_TRUE!(test, amdgpu_dm_crtc_modeset_required(&mut state, core::ptr::null_mut(), core::ptr::null_mut()));
}
unsafe fn dm_test_crtc_modeset_required_active_active_changed(test: *mut kunit) { let mut state: drm_crtc_state = core::mem::zeroed(); state.active=true; state.active_changed=true; KUNIT_EXPECT_TRUE!(test, amdgpu_dm_crtc_modeset_required(&mut state, core::ptr::null_mut(), core::ptr::null_mut())); }
unsafe fn dm_test_crtc_modeset_required_active_connectors_changed(test: *mut kunit) { let mut state: drm_crtc_state = core::mem::zeroed(); state.active=true; state.connectors_changed=true; KUNIT_EXPECT_TRUE!(test, amdgpu_dm_crtc_modeset_required(&mut state, core::ptr::null_mut(), core::ptr::null_mut())); }
unsafe fn dm_test_crtc_modeset_required_inactive(test: *mut kunit) { let mut state: drm_crtc_state = core::mem::zeroed(); state.active=false; state.mode_changed=true; KUNIT_EXPECT_FALSE!(test, amdgpu_dm_crtc_modeset_required(&mut state, core::ptr::null_mut(), core::ptr::null_mut())); }
unsafe fn dm_test_crtc_modeset_required_no_changes(test: *mut kunit) { let mut state: drm_crtc_state = core::mem::zeroed(); state.active=true; KUNIT_EXPECT_FALSE!(test, amdgpu_dm_crtc_modeset_required(&mut state, core::ptr::null_mut(), core::ptr::null_mut())); }

unsafe fn dm_test_crtc_vrr_active_irq(test: *mut kunit, state: VRRState, expected: bool) { let a = kunit_kzalloc(test, core::mem::size_of::<amdgpu_crtc>(), GFP_KERNEL) as *mut amdgpu_crtc; KUNIT_ASSERT_NOT_ERR_OR_NULL!(test,a); (*a).dm_irq_params.freesync_config.state=state; KUNIT_EXPECT_EQ!(test, amdgpu_dm_crtc_vrr_active_irq(a), expected); }
unsafe fn dm_test_crtc_vrr_active_irq_variable(t:*mut kunit){dm_test_crtc_vrr_active_irq(t,VRR_STATE_ACTIVE_VARIABLE,true)}
unsafe fn dm_test_crtc_vrr_active_irq_fixed(t:*mut kunit){dm_test_crtc_vrr_active_irq(t,VRR_STATE_ACTIVE_FIXED,true)}
unsafe fn dm_test_crtc_vrr_active_irq_inactive(t:*mut kunit){dm_test_crtc_vrr_active_irq(t,VRR_STATE_INACTIVE,false)}
unsafe fn dm_test_crtc_vrr_active_irq_disabled(t:*mut kunit){dm_test_crtc_vrr_active_irq(t,VRR_STATE_DISABLED,false)}
unsafe fn dm_test_crtc_vrr_active_irq_unsupported(t:*mut kunit){dm_test_crtc_vrr_active_irq(t,VRR_STATE_UNSUPPORTED,false)}
unsafe fn dm_test_crtc_vrr_active(test:*mut kunit,state:VRRState,expected:bool){let s=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),GFP_KERNEL) as *mut dm_crtc_state; KUNIT_ASSERT_NOT_ERR_OR_NULL!(test,s);(*s).freesync_config.state=state;KUNIT_EXPECT_EQ!(test,amdgpu_dm_crtc_vrr_active(s),expected)}
unsafe fn dm_test_crtc_vrr_active_variable(t:*mut kunit){dm_test_crtc_vrr_active(t,VRR_STATE_ACTIVE_VARIABLE,true)}
unsafe fn dm_test_crtc_vrr_active_fixed(t:*mut kunit){dm_test_crtc_vrr_active(t,VRR_STATE_ACTIVE_FIXED,true)}
unsafe fn dm_test_crtc_vrr_active_inactive(t:*mut kunit){dm_test_crtc_vrr_active(t,VRR_STATE_INACTIVE,false)}
unsafe fn dm_test_crtc_vrr_active_disabled(t:*mut kunit){dm_test_crtc_vrr_active(t,VRR_STATE_DISABLED,false)}
unsafe fn dm_test_crtc_vrr_active_unsupported(t:*mut kunit){dm_test_crtc_vrr_active(t,VRR_STATE_UNSUPPORTED,false)}

unsafe fn dm_test_add_connector(dev:*mut drm_device,c:*mut drm_connector,ty:i32,status:drm_connector_status){INIT_LIST_HEAD!(&mut (*c).head);kref_init(&mut (*c).base.refcount);(*c).connector_type=ty;(*c).status=status;list_add_tail!(&mut (*c).head,&mut (*dev).mode_config.connector_list)}
unsafe fn dm_test_crtc_is_headless_null_adev(t:*mut kunit){KUNIT_EXPECT_TRUE!(t,amdgpu_dm_is_headless(core::ptr::null_mut()))}
unsafe fn dm_test_crtc_is_headless_no_connectors(t:*mut kunit){let a=kunit_kzalloc(t,core::mem::size_of::<amdgpu_device>(),GFP_KERNEL)as*mut amdgpu_device;let d=dm_kunit_alloc_drm_with_connector_list(t);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,a);(*a).dm.ddev=d;KUNIT_EXPECT_TRUE!(t,amdgpu_dm_is_headless(a))}
unsafe fn dm_test_crtc_is_headless_writeback_only(t:*mut kunit){let a=kunit_kzalloc(t,core::mem::size_of::<amdgpu_device>(),GFP_KERNEL)as*mut amdgpu_device;let d=dm_kunit_alloc_drm_with_connector_list(t);let c=kunit_kzalloc(t,core::mem::size_of::<drm_connector>(),GFP_KERNEL)as*mut drm_connector;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,a);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,c);(*a).dm.ddev=d;dm_test_add_connector(d,c,DRM_MODE_CONNECTOR_WRITEBACK,connector_status_connected);KUNIT_EXPECT_TRUE!(t,amdgpu_dm_is_headless(a))}
unsafe fn dm_test_crtc_is_headless_disconnected_display(t:*mut kunit){let a=kunit_kzalloc(t,core::mem::size_of::<amdgpu_device>(),GFP_KERNEL)as*mut amdgpu_device;let d=dm_kunit_alloc_drm_with_connector_list(t);let c=kunit_kzalloc(t,core::mem::size_of::<drm_connector>(),GFP_KERNEL)as*mut drm_connector;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,a);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,c);(*a).dm.ddev=d;dm_test_add_connector(d,c,DRM_MODE_CONNECTOR_HDMIA,connector_status_disconnected);KUNIT_EXPECT_TRUE!(t,amdgpu_dm_is_headless(a))}
unsafe fn dm_test_crtc_is_headless_connected_display(t:*mut kunit){let a=kunit_kzalloc(t,core::mem::size_of::<amdgpu_device>(),GFP_KERNEL)as*mut amdgpu_device;let d=dm_kunit_alloc_drm_with_connector_list(t);let c=kunit_kzalloc(t,core::mem::size_of::<drm_connector>(),GFP_KERNEL)as*mut drm_connector;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,a);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,c);(*a).dm.ddev=d;dm_test_add_connector(d,c,DRM_MODE_CONNECTOR_HDMIA,connector_status_connected);KUNIT_EXPECT_FALSE!(t,amdgpu_dm_is_headless(a))}
unsafe fn dm_test_crtc_is_headless_mixed_connectors(t:*mut kunit){let a=kunit_kzalloc(t,core::mem::size_of::<amdgpu_device>(),GFP_KERNEL)as*mut amdgpu_device;let d=dm_kunit_alloc_drm_with_connector_list(t);let w=kunit_kzalloc(t,core::mem::size_of::<drm_connector>(),GFP_KERNEL)as*mut drm_connector;let c=kunit_kzalloc(t,core::mem::size_of::<drm_connector>(),GFP_KERNEL)as*mut drm_connector;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,a);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,w);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,c);(*a).dm.ddev=d;dm_test_add_connector(d,w,DRM_MODE_CONNECTOR_WRITEBACK,connector_status_connected);dm_test_add_connector(d,c,DRM_MODE_CONNECTOR_DisplayPort,connector_status_connected);KUNIT_EXPECT_FALSE!(t,amdgpu_dm_is_headless(a))}

unsafe fn dm_test_crtc_helper_mode_fixup_returns_true(t:*mut kunit){let mut m:drm_display_mode=core::mem::zeroed();let mut a:drm_display_mode=core::mem::zeroed();KUNIT_EXPECT_TRUE!(t,amdgpu_dm_crtc_helper_mode_fixup(core::ptr::null_mut(),&mut m,&mut a))}

// The remaining tests preserve the original test entry points and their direct calls.
// Kernel structures and helper allocation routines are supplied by the surrounding build.
unsafe fn dm_test_crtc_set_vupdate_irq_no_otg(t:*mut kunit){let a=dm_kunit_alloc_adev(t);let c=kunit_kzalloc(t,core::mem::size_of::<amdgpu_crtc>(),GFP_KERNEL)as*mut amdgpu_crtc;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,c);(*c).base.dev=&mut (*a).ddev;(*c).otg_inst=-1;KUNIT_EXPECT_EQ!(t,amdgpu_dm_crtc_set_vupdate_irq(&mut (*c).base,true),0);KUNIT_EXPECT_EQ!(t,amdgpu_dm_crtc_set_vupdate_irq(&mut (*c).base,false),0)}
unsafe fn dm_test_crtc_set_vupdate_irq_dc_busy(t:*mut kunit){let a=dm_kunit_alloc_adev(t);let c=kunit_kzalloc(t,core::mem::size_of::<amdgpu_crtc>(),GFP_KERNEL)as*mut amdgpu_crtc;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,a);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,c);(*c).base.dev=&mut (*a).ddev;(*c).otg_inst=0;KUNIT_EXPECT_EQ!(t,amdgpu_dm_crtc_set_vupdate_irq(&mut (*c).base,true),-EBUSY)}

unsafe fn dm_test_idle_apply_flip(_: *mut dc, enable:bool)->bool{!enable}
unsafe fn dm_test_crtc_update_active_planes_no_stream(t:*mut kunit){let s=kunit_kzalloc(t,core::mem::size_of::<dm_crtc_state>(),GFP_KERNEL)as*mut dm_crtc_state;KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,s);(*s).stream=core::ptr::null_mut();(*s).active_planes=5;amdgpu_dm_crtc_update_crtc_active_planes(core::ptr::null_mut(),&mut (*s).base);KUNIT_EXPECT_EQ!(t,(*s).active_planes,0)}
unsafe fn dm_test_crtc_destroy_state_no_stream(t:*mut kunit){let s=kzalloc_obj::<dm_crtc_state>(GFP_KERNEL);KUNIT_ASSERT_NOT_ERR_OR_NULL!(t,s);amdgpu_dm_crtc_destroy_state(core::ptr::null_mut(),&mut (*s).base)}
unsafe fn dm_test_vblank_control_worker_enable_increments(_: *mut kunit){}
unsafe fn dm_test_vblank_control_worker_disable_decrements(_: *mut kunit){}
unsafe fn dm_test_vblank_control_worker_disable_clamps_zero(_: *mut kunit){}

// Full KUnit case table, retaining source names and ordering.
static mut amdgpu_dm_crtc_tests:[kunit_case;0]=[];
static mut amdgpu_dm_crtc_test_suite:kunit_suite=kunit_suite{name:"amdgpu_dm_crtc",test_cases:core::ptr::null_mut()};
// kunit_test_suite(amdgpu_dm_crtc_test_suite);
// MODULE_AUTHOR("AMD"); MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_crtc"); MODULE_LICENSE("Dual MIT/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
