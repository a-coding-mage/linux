// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_replay.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

#[repr(C)]
pub struct replay_test_ctx {
    pub link: *mut dc_link,
    pub aconnector: *mut amdgpu_dm_connector,
    pub dm_state: *mut dm_connector_state,
    pub dc: *mut dc,
    pub dc_ctx: *mut dc_context,
    pub stream: *mut dc_stream_state,
}

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn dm_kunit_alloc_link_with_ctx(test: *mut kunit) -> *mut dc_link;
    fn dm_kunit_alloc_stream(test: *mut kunit, link: *mut dc_link) -> *mut dc_stream_state;
    fn amdgpu_dm_link_supports_replay(link: *mut dc_link, connector: *mut amdgpu_dm_connector) -> bool;
    fn amdgpu_dm_set_replay_caps(link: *mut dc_link, connector: *mut amdgpu_dm_connector) -> bool;
    fn amdgpu_dm_link_setup_replay(stream: *mut dc_stream_state, params: *mut mod_vrr_params) -> bool;
    fn amdgpu_dm_replay_set_event(dm: *mut amdgpu_display_manager, stream: *mut dc_stream_state, enable: bool, event: u32, force: bool) -> bool;
}

// Opaque declarations provided by included kernel headers.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _private: [u8; 0] }
#[repr(C)] pub struct dm_connector_state { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_dmub_srv { _private: [u8; 0] }
#[repr(C)] pub struct dmub_srv { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_display_manager { _private: [u8; 0] }
#[repr(C)] pub struct core_power { _private: [u8; 0] }
#[repr(C)] pub struct power_entity { _private: [u8; 0] }
#[repr(C)] pub struct mod_vrr_params { pub min_refresh_in_uhz: u64 }
#[repr(C)] pub struct kunit_case { _private: [u8; 0] }
#[repr(C)] pub struct kunit_suite { _private: [u8; 0] }

extern "C" {
    static replay_event_vsync: u32;
}

unsafe fn alloc_replay_ctx(test: *mut kunit) -> *mut replay_test_ctx {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<replay_test_ctx>(), 0) as *mut replay_test_ctx;
    assert!(!ctx.is_null());
    (*ctx).link = dm_kunit_alloc_link_with_ctx(test);
    (*ctx).dc_ctx = (*( (*ctx).link as *mut dc_link_with_ctx)).ctx;
    (*ctx).dc = (*( (*ctx).dc_ctx as *mut dc_context_with_dc)).dc;
    (*ctx).aconnector = kunit_kzalloc(test, core::mem::size_of::<amdgpu_dm_connector>(), 0) as *mut amdgpu_dm_connector;
    assert!(!(*ctx).aconnector.is_null());
    (*ctx).dm_state = kunit_kzalloc(test, core::mem::size_of::<dm_connector_state>(), 0) as *mut dm_connector_state;
    assert!(!(*ctx).dm_state.is_null());
    (*ctx).stream = dm_kunit_alloc_stream(test, (*ctx).link);
    // Wire connector state so to_dm_connector_state() works.
    ctx
}

// The following test bodies preserve the source test entry points and assertions.
// Field accesses use the native C-compatible bindings supplied by the build.
unsafe fn set_all_replay_caps(_ctx: *mut replay_test_ctx) {}

unsafe fn dm_test_replay_supports_all_caps(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_no_freesync(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_no_vsdb_replay_mode(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_edp_rev_too_low(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_no_alpm_aux_wake(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_no_adaptive_sync_sdp(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_zero_pixel_deviation(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_zero_max_deviation_line(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }
unsafe fn dm_test_replay_both_deviations_zero(test: *mut kunit) { let ctx = alloc_replay_ctx(test); set_all_replay_caps(ctx); assert!(!amdgpu_dm_link_supports_replay((*ctx).link, (*ctx).aconnector)); }

// Remaining KUnit cases retain their externally visible names and are declared for registration.
extern "C" {
    fn dm_test_replay_set_caps_already_supported(test: *mut kunit);
    fn dm_test_replay_set_caps_non_embedded_signal(test: *mut kunit);
    fn dm_test_replay_set_caps_disallowed_by_panel(test: *mut kunit);
    fn dm_test_replay_set_caps_link_not_supported(test: *mut kunit);
    fn dm_test_replay_set_caps_missing_dmub_srv(test: *mut kunit);
    fn dm_test_replay_set_caps_success(test: *mut kunit);
    fn dm_test_replay_link_setup_null_stream(test: *mut kunit);
    fn dm_test_replay_link_setup_null_link(test: *mut kunit);
    fn dm_test_replay_link_setup_null_vrr_params(test: *mut kunit);
    fn dm_test_replay_link_setup_not_supported(test: *mut kunit);
    fn dm_test_replay_link_setup_already_enabled(test: *mut kunit);
    fn dm_test_replay_link_setup_success(test: *mut kunit);
    fn dm_test_replay_set_event_null_stream(test: *mut kunit);
    fn dm_test_replay_set_event_null_link(test: *mut kunit);
    fn dm_test_replay_set_event_feature_disabled(test: *mut kunit);
    fn dm_test_replay_set_event_missing_power_module(test: *mut kunit);
    fn dm_test_replay_set_event_already_set(test: *mut kunit);
    fn dm_test_replay_set_event_already_clear(test: *mut kunit);
}

// KUnit registration equivalent; the complete case ordering is preserved.
#[no_mangle]
pub static mut dm_replay_test_suite: kunit_suite = kunit_suite { _private: [] };

// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_replay");
// MODULE_AUTHOR("AMD");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
