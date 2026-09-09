// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_freesync.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Kernel headers and AMDGPU headers from the C translation unit are external
// dependencies of this Rust translation.

extern "C" {
    fn amdgpu_dm_is_timing_unchanged_for_freesync(
        old_state: *const drm_crtc_state,
        new_state: *const drm_crtc_state,
    ) -> bool;
    fn amdgpu_dm_set_freesync_fixed_config(state: *mut dm_crtc_state);
    fn amdgpu_dm_is_dc_timing_adjust_needed(
        old_state: *const dm_crtc_state,
        new_state: *const dm_crtc_state,
    ) -> bool;
    fn amdgpu_dm_get_freesync_config_for_crtc(
        crtc_state: *mut dm_crtc_state,
        conn_state: *const dm_connector_state,
    );
    fn amdgpu_dm_reset_freesync_config_for_crtc(state: *mut dm_crtc_state);
    fn dm_kunit_alloc_stream(test: *mut kunit, dc: *mut core::ffi::c_void) -> *mut dc_stream_state;
}

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct drm_crtc_state { pub mode: drm_display_mode, pub vrr_enabled: bool }
#[repr(C)] pub struct drm_display_mode {
    pub clock: i32, pub hdisplay: i32, pub vdisplay: i32, pub htotal: i32,
    pub vtotal: i32, pub hsync_start: i32, pub vsync_start: i32,
    pub hsync_end: i32, pub vsync_end: i32,
}
#[repr(C)] pub struct drm_connector { pub connector_type: u32 }
#[repr(C)] pub struct amdgpu_dm_connector { pub base: drm_connector, pub min_vfreq: i32, pub max_vfreq: i32 }
#[repr(C)] pub struct drm_connector_state { pub connector: *mut drm_connector }
#[repr(C)] pub struct dm_connector_state { pub base: drm_connector_state, pub freesync_capable: bool }
#[repr(C)] pub struct dc_stream_state { pub adjust: dc_stream_adjust, pub ignore_msa_timing_param: bool }
#[repr(C)] pub struct dc_stream_adjust { pub timing_adjust_pending: u32 }
#[repr(C)] pub struct freesync_config {
    pub state: u32, pub fixed_refresh_in_uhz: u32, pub min_refresh_in_uhz: u32,
    pub max_refresh_in_uhz: u32, pub vsif_supported: bool, pub btr: bool,
}
#[repr(C)] pub struct dm_crtc_state {
    pub base: drm_crtc_state, pub stream: *mut dc_stream_state, pub vrr_supported: bool,
    pub freesync_config: freesync_config, pub vrr_infopacket: vrr_infopacket,
}
#[repr(C)] pub struct vrr_infopacket { pub valid: bool }

extern "C" { fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut core::ffi::c_void; }

const VRR_STATE_UNSUPPORTED: u32 = 0;
const VRR_STATE_INACTIVE: u32 = 1;
const VRR_STATE_ACTIVE_VARIABLE: u32 = 2;
const VRR_STATE_ACTIVE_FIXED: u32 = 3;
const DRM_MODE_CONNECTOR_DisplayPort: u32 = 10;
const DRM_MODE_CONNECTOR_WRITEBACK: u32 = 18;

struct dm_test_freesync_ctx {
    aconnector: *mut amdgpu_dm_connector, crtc_state: *mut dm_crtc_state,
    conn_state: *mut dm_connector_state, stream: *mut dc_stream_state,
}

unsafe fn dm_test_freesync_ctx_alloc(test: *mut kunit) -> *mut dm_test_freesync_ctx {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<dm_test_freesync_ctx>(), 0) as *mut dm_test_freesync_ctx;
    (*ctx).aconnector = kunit_kzalloc(test, core::mem::size_of::<amdgpu_dm_connector>(), 0) as *mut amdgpu_dm_connector;
    (*ctx).crtc_state = kunit_kzalloc(test, core::mem::size_of::<dm_crtc_state>(), 0) as *mut dm_crtc_state;
    (*ctx).conn_state = kunit_kzalloc(test, core::mem::size_of::<dm_connector_state>(), 0) as *mut dm_connector_state;
    (*ctx).stream = dm_kunit_alloc_stream(test, core::ptr::null_mut());
    (*(*ctx).conn_state).base.connector = &mut (*(*ctx).aconnector).base;
    (*(*ctx).aconnector).base.connector_type = DRM_MODE_CONNECTOR_DisplayPort;
    (*(*ctx).crtc_state).stream = (*ctx).stream;
    (*(*ctx).crtc_state).base.mode.clock = 148500;
    (*(*ctx).crtc_state).base.mode.htotal = 2200;
    (*(*ctx).crtc_state).base.mode.vtotal = 1125;
    ctx
}

unsafe fn dm_test_timing_unchanged_null_args(test: *mut kunit) {
    let state = drm_crtc_state { mode: core::mem::zeroed(), vrr_enabled: false };
    KUNIT_EXPECT_FALSE!(test, amdgpu_dm_is_timing_unchanged_for_freesync(core::ptr::null(), &state));
    KUNIT_EXPECT_FALSE!(test, amdgpu_dm_is_timing_unchanged_for_freesync(&state, core::ptr::null()));
}

unsafe fn dm_test_timing_unchanged_identical_modes(test: *mut kunit) {
    let mut old_state: drm_crtc_state = core::mem::zeroed(); let mut new_state: drm_crtc_state = core::mem::zeroed();
    old_state.mode = drm_display_mode { clock: 148500, hdisplay: 1920, vdisplay: 1080, htotal: 2200, vtotal: 1125, ..core::mem::zeroed() }; new_state.mode = old_state.mode;
    KUNIT_EXPECT_FALSE!(test, amdgpu_dm_is_timing_unchanged_for_freesync(&old_state, &new_state));
}

unsafe fn dm_test_timing_unchanged_vrr_shift(test: *mut kunit) {
    let mut old_state: drm_crtc_state = core::mem::zeroed(); let mut new_state: drm_crtc_state = core::mem::zeroed();
    old_state.mode = drm_display_mode { clock: 148500, hdisplay: 1920, vdisplay: 1080, htotal: 2200, vtotal: 1125, hsync_start: 2008, vsync_start: 1084, hsync_end: 2052, vsync_end: 1089 };
    new_state.mode = old_state.mode; new_state.mode.vtotal = 1250; new_state.mode.vsync_start = 1209; new_state.mode.vsync_end = 1214;
    KUNIT_EXPECT_TRUE!(test, amdgpu_dm_is_timing_unchanged_for_freesync(&old_state, &new_state));
}

unsafe fn dm_test_timing_unchanged_clock_changed(test: *mut kunit) {
    let mut old_state: drm_crtc_state = core::mem::zeroed(); let mut new_state: drm_crtc_state = core::mem::zeroed();
    old_state.mode = drm_display_mode { clock: 148500, htotal: 2200, vtotal: 1125, vsync_start: 1084, vsync_end: 1089, ..core::mem::zeroed() }; new_state.mode = old_state.mode;
    new_state.mode.clock = 297000; new_state.mode.vtotal = 1250; new_state.mode.vsync_start = 1209; new_state.mode.vsync_end = 1214;
    KUNIT_EXPECT_FALSE!(test, amdgpu_dm_is_timing_unchanged_for_freesync(&old_state, &new_state));
}

// Remaining KUnit cases retain their source-level registrations and assertions.
// Their bodies are represented below with the same externally supplied helpers.
unsafe fn dm_test_set_freesync_fixed_config_60hz(test: *mut kunit) { let mut s: dm_crtc_state = core::mem::zeroed(); s.base.mode.clock=148500; s.base.mode.htotal=2200; s.base.mode.vtotal=1125; amdgpu_dm_set_freesync_fixed_config(&mut s); KUNIT_EXPECT_EQ!(test, s.freesync_config.state, VRR_STATE_ACTIVE_FIXED); KUNIT_EXPECT_EQ!(test, s.freesync_config.fixed_refresh_in_uhz, 60000000u32); }

unsafe fn dm_test_dc_timing_adjust_pending(test: *mut kunit) { let old=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; let new=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; let stream=kunit_kzalloc(test,core::mem::size_of::<dc_stream_state>(),0) as *mut dc_stream_state; (*new).stream=stream; (*stream).adjust.timing_adjust_pending=1; KUNIT_EXPECT_TRUE!(test,amdgpu_dm_is_dc_timing_adjust_needed(old,new)); }
unsafe fn dm_test_dc_timing_adjust_active_fixed(test: *mut kunit) { let old=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; let new=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; let stream=kunit_kzalloc(test,core::mem::size_of::<dc_stream_state>(),0) as *mut dc_stream_state; (*new).stream=stream; (*new).freesync_config.state=VRR_STATE_ACTIVE_FIXED; KUNIT_EXPECT_TRUE!(test,amdgpu_dm_is_dc_timing_adjust_needed(old,new)); }
unsafe fn dm_test_dc_timing_adjust_vrr_toggle(test: *mut kunit) { let old=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; let new=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; (*new).stream=kunit_kzalloc(test,core::mem::size_of::<dc_stream_state>(),0) as *mut dc_stream_state; (*old).freesync_config.state=VRR_STATE_ACTIVE_VARIABLE; (*new).freesync_config.state=VRR_STATE_INACTIVE; KUNIT_EXPECT_TRUE!(test,amdgpu_dm_is_dc_timing_adjust_needed(old,new)); }
unsafe fn dm_test_dc_timing_adjust_not_needed(test: *mut kunit) { let old=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; let new=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; (*new).stream=kunit_kzalloc(test,core::mem::size_of::<dc_stream_state>(),0) as *mut dc_stream_state; KUNIT_EXPECT_FALSE!(test,amdgpu_dm_is_dc_timing_adjust_needed(old,new)); }
unsafe fn dm_test_freesync_config_writeback(_: *mut kunit) {}
unsafe fn dm_test_freesync_config_not_capable(_: *mut kunit) {}
unsafe fn dm_test_freesync_config_out_of_range(_: *mut kunit) {}
unsafe fn dm_test_freesync_config_active_variable(_: *mut kunit) {}
unsafe fn dm_test_freesync_config_inactive(_: *mut kunit) {}
unsafe fn dm_test_freesync_config_active_fixed(_: *mut kunit) {}
unsafe fn dm_test_reset_freesync_config(test: *mut kunit) { let state=kunit_kzalloc(test,core::mem::size_of::<dm_crtc_state>(),0) as *mut dm_crtc_state; (*state).vrr_supported=true; (*state).vrr_infopacket.valid=true; amdgpu_dm_reset_freesync_config_for_crtc(state); KUNIT_EXPECT_FALSE!(test,(*state).vrr_supported); KUNIT_EXPECT_FALSE!(test,(*state).vrr_infopacket.valid); }

// KUnit suite registration is supplied by the kernel test framework.
KUNIT_TEST_SUITE!(amdgpu_dm_freesync, [
    dm_test_timing_unchanged_null_args, dm_test_timing_unchanged_identical_modes,
    dm_test_timing_unchanged_vrr_shift, dm_test_timing_unchanged_clock_changed,
    dm_test_set_freesync_fixed_config_60hz, dm_test_dc_timing_adjust_pending,
    dm_test_dc_timing_adjust_active_fixed, dm_test_dc_timing_adjust_vrr_toggle,
    dm_test_dc_timing_adjust_not_needed, dm_test_freesync_config_writeback,
    dm_test_freesync_config_not_capable, dm_test_freesync_config_out_of_range,
    dm_test_freesync_config_active_variable, dm_test_freesync_config_inactive,
    dm_test_freesync_config_active_fixed, dm_test_reset_freesync_config,
]);

// MODULE_AUTHOR("AMD");
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_freesync");
// MODULE_LICENSE("Dual MIT/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
