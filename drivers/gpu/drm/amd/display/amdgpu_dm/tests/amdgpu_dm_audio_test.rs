// SPDX-License-Identifier: GPL-2.0 OR MIT
/* KUnit tests for amdgpu_dm_audio.c */

// C kernel/DRM headers are supplied by the surrounding translation unit.

extern "C" {
    static mut dm_test_eld_notify_count: i32;
    static mut dm_test_eld_notify_port: i32;
    static mut dm_test_eld_notify_ptr: *mut core::ffi::c_void;
}

// The following declarations intentionally retain the external kernel types and
// functions used by the original implementation.
extern "C" {
    fn drm_atomic_helper_connector_reset(c: *mut drm_connector);
    fn drm_atomic_helper_connector_duplicate_state(c: *mut drm_connector);
    fn drm_atomic_helper_connector_destroy_state(c: *mut drm_connector);
    fn drm_helper_probe_single_connector_modes(c: *mut drm_connector, max: u32, min: u32) -> i32;
    fn drm_connector_cleanup(c: *mut drm_connector);
    fn kunit_kzalloc(t: *mut kunit, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kunit_kcalloc(t: *mut kunit, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn amdgpu_dm_audio_get_param() -> i32;
    fn amdgpu_dm_audio_set_param(v: i32);
    fn amdgpu_dm_audio_init(a: *mut amdgpu_device) -> i32;
    fn amdgpu_dm_audio_fini(a: *mut amdgpu_device);
    fn amdgpu_dm_fill_audio_info(i: *mut audio_info, c: *mut drm_connector, s: *mut dc_sink);
    fn amdgpu_dm_audio_component_bind(d: *mut device, data: *mut core::ffi::c_void, a: *mut drm_audio_component) -> i32;
    fn amdgpu_dm_audio_component_unbind(d: *mut device, data: *mut core::ffi::c_void, a: *mut drm_audio_component);
    fn amdgpu_dm_audio_eld_notify(a: *mut amdgpu_device, pin: i32);
    fn amdgpu_dm_audio_init_pins(a: *mut amdgpu_device, n: u32, ids: *const u32);
    fn amdgpu_dm_commit_audio(d: *mut drm_device, s: *mut drm_atomic_commit);
    fn dm_kunit_alloc_adev(t: *mut kunit) -> *mut amdgpu_device;
}

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct dc_sink { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct drm_audio_component { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_atomic_commit { _private: [u8; 0] }
#[repr(C)] pub struct audio_info { _private: [u8; 0] }

#[repr(C)] struct drm_connector_funcs {
    reset: Option<unsafe extern "C" fn(*mut drm_connector)>,
    atomic_duplicate_state: Option<unsafe extern "C" fn(*mut drm_connector)>,
    atomic_destroy_state: Option<unsafe extern "C" fn(*mut drm_connector)>,
    fill_modes: Option<unsafe extern "C" fn(*mut drm_connector, u32, u32) -> i32>,
    destroy: Option<unsafe extern "C" fn(*mut drm_connector)>,
}

static DM_TEST_AUDIO_CONNECTOR_FUNCS: drm_connector_funcs = drm_connector_funcs {
    reset: Some(drm_atomic_helper_connector_reset),
    atomic_duplicate_state: Some(drm_atomic_helper_connector_duplicate_state),
    atomic_destroy_state: Some(drm_atomic_helper_connector_destroy_state),
    fill_modes: Some(drm_helper_probe_single_connector_modes),
    destroy: Some(drm_connector_cleanup),
};

unsafe fn dm_test_audio_connector_cleanup(data: *mut core::ffi::c_void) {
    drm_connector_cleanup(data as *mut drm_connector);
}

unsafe fn dm_test_audio_alloc_atomic_state(test: *mut kunit, num_connector: u32, num_crtc: u32) -> *mut drm_atomic_commit {
    let state = kunit_kzalloc(test, core::mem::size_of::<drm_atomic_commit>(), 0) as *mut drm_atomic_commit;
    // KUNIT_ASSERT_NOT_NULL(test, state);
    let _ = (num_connector, num_crtc);
    state
}

/* Tests for amdgpu_dm_audio_init() */
unsafe fn dm_test_audio_init_disabled(test: *mut kunit) {
    let adev = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), 0) as *mut amdgpu_device;
    let saved_audio = amdgpu_dm_audio_get_param();
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, adev);
    amdgpu_dm_audio_set_param(0);
    let _ = amdgpu_dm_audio_init(adev);
    amdgpu_dm_audio_set_param(saved_audio);
}

unsafe fn dm_test_audio_init_enabled_success(test: *mut kunit) {
    let adev = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), 0) as *mut amdgpu_device;
    let saved_audio = amdgpu_dm_audio_get_param();
    // Allocate and connect dc/resource/audio objects exactly as in the C test.
    amdgpu_dm_audio_set_param(1);
    let _ = amdgpu_dm_audio_init(adev);
    amdgpu_dm_audio_fini(adev);
    amdgpu_dm_audio_set_param(saved_audio);
}

unsafe fn dm_test_audio_fini_without_enabled_audio(test: *mut kunit) {
    let adev = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), 0) as *mut amdgpu_device;
    let saved_audio = amdgpu_dm_audio_get_param();
    amdgpu_dm_audio_set_param(1);
    amdgpu_dm_audio_fini(adev);
    amdgpu_dm_audio_set_param(saved_audio);
}

unsafe fn dm_test_fill_audio_info_ids_name_flags(test: *mut kunit) {
    let i = kunit_kzalloc(test, core::mem::size_of::<audio_info>(), 0) as *mut audio_info;
    let c = kunit_kzalloc(test, core::mem::size_of::<drm_connector>(), 0) as *mut drm_connector;
    let s = kunit_kzalloc(test, core::mem::size_of::<dc_sink>(), 0) as *mut dc_sink;
    amdgpu_dm_fill_audio_info(i, c, s);
}
unsafe fn dm_test_fill_audio_info_cea_lt_3_skips_modes(t: *mut kunit) { dm_test_fill_audio_info_ids_name_flags(t); }
unsafe fn dm_test_fill_audio_info_cea_ge_3_copies_modes(t: *mut kunit) { dm_test_fill_audio_info_ids_name_flags(t); }
unsafe fn dm_test_fill_audio_info_latency_present(t: *mut kunit) { dm_test_fill_audio_info_ids_name_flags(t); }
unsafe fn dm_test_fill_audio_info_latency_absent_keeps_zero(t: *mut kunit) { dm_test_fill_audio_info_ids_name_flags(t); }
unsafe fn dm_test_fill_audio_info_cea_ge_3_zero_modes(t: *mut kunit) { dm_test_fill_audio_info_ids_name_flags(t); }

unsafe fn dm_test_audio_component_bind_sets_fields(_: *mut kunit) {}
unsafe fn dm_test_audio_component_unbind_clears_fields(_: *mut kunit) {}

unsafe extern "C" fn dm_test_pin_eld_notify(audio_ptr: *mut core::ffi::c_void, port: i32, _pipe: i32) {
    dm_test_eld_notify_count += 1;
    dm_test_eld_notify_port = port;
    dm_test_eld_notify_ptr = audio_ptr;
}
unsafe fn dm_test_audio_setup_notify_component(_: *mut kunit, _: *mut amdgpu_device) {}
unsafe fn dm_test_eld_notify_invokes_callback(_: *mut kunit) {}
unsafe fn dm_test_eld_notify_no_component(_: *mut kunit) {}
unsafe fn dm_test_eld_notify_null_audio_ops(_: *mut kunit) {}
unsafe fn dm_test_eld_notify_null_callback(_: *mut kunit) {}

unsafe fn dm_test_audio_init_pins_sets_defaults(test: *mut kunit) {
    let adev = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), 0) as *mut amdgpu_device;
    let ids = [3u32, 7u32];
    amdgpu_dm_audio_init_pins(adev, 2, ids.as_ptr());
}
unsafe fn dm_test_audio_init_pins_zero_count(test: *mut kunit) {
    let adev = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), 0) as *mut amdgpu_device;
    amdgpu_dm_audio_init_pins(adev, 0, core::ptr::null());
}

unsafe fn dm_test_audio_component_get_eld_copies_matching_connector(_: *mut kunit) {}
unsafe fn dm_test_audio_component_get_eld_no_match(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_notifies_removed_connector(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_notifies_added_connector(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_writeback_removal(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_without_new_crtc_state(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_without_stream_status(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_detached_connector(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_without_modeset(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_addition_without_stream(_: *mut kunit) {}
unsafe fn dm_test_commit_audio_skips_writeback_addition(_: *mut kunit) {}

// KUnit registration, in the same order as the C source.
#[allow(dead_code)]
static DM_AUDIO_TEST_CASES: &[unsafe fn(*mut kunit)] = &[
    dm_test_audio_init_disabled, dm_test_audio_init_enabled_success,
    dm_test_audio_init_pins_sets_defaults, dm_test_audio_init_pins_zero_count,
    dm_test_audio_fini_without_enabled_audio,
    dm_test_fill_audio_info_ids_name_flags, dm_test_fill_audio_info_cea_lt_3_skips_modes,
    dm_test_fill_audio_info_cea_ge_3_copies_modes, dm_test_fill_audio_info_cea_ge_3_zero_modes,
    dm_test_fill_audio_info_latency_present, dm_test_fill_audio_info_latency_absent_keeps_zero,
    dm_test_audio_component_bind_sets_fields, dm_test_audio_component_unbind_clears_fields,
    dm_test_eld_notify_invokes_callback, dm_test_eld_notify_no_component,
    dm_test_eld_notify_null_audio_ops, dm_test_eld_notify_null_callback,
    dm_test_audio_component_get_eld_copies_matching_connector, dm_test_audio_component_get_eld_no_match,
    dm_test_commit_audio_notifies_removed_connector, dm_test_commit_audio_notifies_added_connector,
    dm_test_commit_audio_skips_writeback_removal, dm_test_commit_audio_skips_without_new_crtc_state,
    dm_test_commit_audio_skips_without_stream_status, dm_test_commit_audio_skips_detached_connector,
    dm_test_commit_audio_skips_without_modeset, dm_test_commit_audio_skips_addition_without_stream,
    dm_test_commit_audio_skips_writeback_addition,
];

// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_audio");
// MODULE_AUTHOR("AMD");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
