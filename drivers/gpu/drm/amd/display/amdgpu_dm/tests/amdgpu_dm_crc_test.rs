// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_crc.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Kernel/DRM declarations are supplied by the surrounding translation unit.

#[repr(C)]
struct DmTestCrcDcFixture {
    dc: *mut dc,
    dc_ctx: *mut dc_context,
    dc_state: *mut dc_state,
    stream: *mut dc_stream_state,
    link: *mut dc_link,
    tg: *mut timing_generator,
    opp: *mut output_pixel_processor,
    logger: *mut dal_logger,
    dm_state: *mut dm_crtc_state,
    crc_params: crc_params,
    dyn_expansion: dc_dynamic_expansion,
    dither_option: dc_dither_option,
    crc_r: u32,
    crc_g: u32,
    crc_b: u32,
    configure_crc_called: bool,
    dyn_expansion_called: bool,
    bit_depth_reduction_called: bool,
    configure_crc_return: bool,
    get_crc_called: bool,
    get_crc_return: bool,
}

static mut DM_TEST_CRC_DC_CTX: *mut DmTestCrcDcFixture = core::ptr::null_mut();

unsafe extern "C" fn dm_test_configure_crc(
    _tg: *mut timing_generator,
    params: *const crc_params,
) -> bool {
    if DM_TEST_CRC_DC_CTX.is_null() { return false; }
    (*DM_TEST_CRC_DC_CTX).configure_crc_called = true;
    (*DM_TEST_CRC_DC_CTX).crc_params = *params;
    (*DM_TEST_CRC_DC_CTX).configure_crc_return
}

unsafe extern "C" fn dm_test_get_crc(
    _tg: *mut timing_generator, _idx: u8, r_cr: *mut u32, g_y: *mut u32, b_cb: *mut u32,
) -> bool {
    if DM_TEST_CRC_DC_CTX.is_null() { return false; }
    (*DM_TEST_CRC_DC_CTX).get_crc_called = true;
    *r_cr = (*DM_TEST_CRC_DC_CTX).crc_r;
    *g_y = (*DM_TEST_CRC_DC_CTX).crc_g;
    *b_cb = (*DM_TEST_CRC_DC_CTX).crc_b;
    (*DM_TEST_CRC_DC_CTX).get_crc_return
}

unsafe extern "C" fn dm_test_opp_set_dyn_expansion(
    opp: *mut output_pixel_processor, _color_sp: dc_color_space,
    _color_dpth: dc_color_depth, _signal: signal_type,
) {
    if DM_TEST_CRC_DC_CTX.is_null() { return; }
    (*DM_TEST_CRC_DC_CTX).dyn_expansion_called = true;
    (*DM_TEST_CRC_DC_CTX).dyn_expansion = (*opp).dyn_expansion;
}

unsafe extern "C" fn dm_test_opp_program_bit_depth_reduction(
    _opp: *mut output_pixel_processor, _params: *const bit_depth_reduction_params,
) {
    if !DM_TEST_CRC_DC_CTX.is_null() { (*DM_TEST_CRC_DC_CTX).bit_depth_reduction_called = true; }
}

static DM_TEST_TG_FUNCS: timing_generator_funcs = timing_generator_funcs {
    configure_crc: Some(dm_test_configure_crc), get_crc: Some(dm_test_get_crc),
};
static DM_TEST_OPP_FUNCS: opp_funcs = opp_funcs {
    opp_set_dyn_expansion: Some(dm_test_opp_set_dyn_expansion),
    opp_program_bit_depth_reduction: Some(dm_test_opp_program_bit_depth_reduction),
};

unsafe fn dm_test_alloc_crc_dc_fixture(test: *mut kunit, adev: *mut amdgpu_device) -> *mut DmTestCrcDcFixture {
    let fixture = kunit_kzalloc(test, core::mem::size_of::<DmTestCrcDcFixture>(), GFP_KERNEL) as *mut DmTestCrcDcFixture;
    KUNIT_ASSERT_NOT_NULL!(test, fixture);
    (*fixture).dm_state = kunit_kzalloc(test, core::mem::size_of::<dm_crtc_state>(), GFP_KERNEL) as *mut dm_crtc_state;
    (*fixture).dc = kunit_kzalloc(test, core::mem::size_of::<dc>(), GFP_KERNEL) as *mut dc;
    (*fixture).dc_ctx = kunit_kzalloc(test, core::mem::size_of::<dc_context>(), GFP_KERNEL) as *mut dc_context;
    (*fixture).dc_state = kunit_kzalloc(test, core::mem::size_of::<dc_state>(), GFP_KERNEL) as *mut dc_state;
    (*fixture).tg = kunit_kzalloc(test, core::mem::size_of::<timing_generator>(), GFP_KERNEL) as *mut timing_generator;
    (*fixture).opp = kunit_kzalloc(test, core::mem::size_of::<output_pixel_processor>(), GFP_KERNEL) as *mut output_pixel_processor;
    (*fixture).logger = kunit_kzalloc(test, core::mem::size_of::<dal_logger>(), GFP_KERNEL) as *mut dal_logger;
    (*fixture).link = dm_kunit_alloc_link(test);
    (*fixture).stream = dm_kunit_alloc_stream(test, (*fixture).link);
    mutex_init(&mut (*adev).dm.dc_lock);
    (*adev).dm.dc = (*fixture).dc;
    (*fixture).dc_ctx = (*fixture).dc_ctx; // preserve the explicit C setup below
    (*fixture).dc_state = (*fixture).dc_state;
    (*fixture).configure_crc_return = true;
    (*fixture).tg.funcs = &DM_TEST_TG_FUNCS;
    (*fixture).opp.funcs = &DM_TEST_OPP_FUNCS;
    (*fixture).dm_state.stream = (*fixture).stream;
    (*fixture).stream.timing.h_addressable = 1920;
    (*fixture).stream.timing.v_addressable = 1080;
    (*fixture).link.dc = (*fixture).dc;
    (*fixture).stream.ctx = (*fixture).dc_ctx;
    (*fixture).stream.link = (*fixture).link;
    (*fixture).dc.ctx = (*fixture).dc_ctx;
    (*fixture).dc.current_state = (*fixture).dc_state;
    (*fixture).dc_ctx.dc = (*fixture).dc;
    (*fixture).dc_ctx.logger = (*fixture).logger;
    let pipe = &mut (*fixture).dc_state.res_ctx.pipe_ctx[0];
    pipe.stream = (*fixture).stream; pipe.pipe_idx = 0;
    pipe.stream_res.tg = (*fixture).tg; pipe.stream_res.opp = (*fixture).opp;
    fixture
}

unsafe fn dm_test_alloc_crc_crtc(test: *mut kunit, adev: *mut amdgpu_device) -> *mut amdgpu_crtc {
    let acrtc = kunit_kzalloc(test, core::mem::size_of::<amdgpu_crtc>(), GFP_KERNEL) as *mut amdgpu_crtc;
    KUNIT_ASSERT_NOT_NULL!(test, acrtc);
    (*acrtc).base.dev = &mut (*adev).ddev;
    drm_modeset_lock_init(&mut (*acrtc).base.mutex);
    spin_lock_init(&mut (*acrtc).base.commit_lock);
    INIT_LIST_HEAD(&mut (*acrtc).base.commit_list);
    acrtc
}

unsafe fn dm_test_parse_crc_source_none(test: *mut kunit) { KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_NONE, dm_parse_crc_source(Some("none"))); KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_NONE, dm_parse_crc_source(None)); }
unsafe fn dm_test_parse_crc_source_crtc(test: *mut kunit) { KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC, dm_parse_crc_source(Some("crtc"))); KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC, dm_parse_crc_source(Some("auto"))); }
unsafe fn dm_test_parse_crc_source_dprx(test: *mut kunit) { KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX, dm_parse_crc_source(Some("dprx"))); }
unsafe fn dm_test_parse_crc_source_crtc_dither(test: *mut kunit) { KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER, dm_parse_crc_source(Some("crtc dither"))); }
unsafe fn dm_test_parse_crc_source_dprx_dither(test: *mut kunit) { KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER, dm_parse_crc_source(Some("dprx dither"))); }
unsafe fn dm_test_parse_crc_source_invalid(test: *mut kunit) { for s in [Some("invalid"), Some("unknown"), Some("")] { KUNIT_EXPECT_EQ!(test, AMDGPU_DM_PIPE_CRC_SOURCE_INVALID, dm_parse_crc_source(s)); } }

unsafe fn dm_test_is_crc_source_crtc(test: *mut kunit) { for s in [AMDGPU_DM_PIPE_CRC_SOURCE_CRTC, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER] { KUNIT_EXPECT_TRUE!(test, dm_is_crc_source_crtc(s)); } for s in [AMDGPU_DM_PIPE_CRC_SOURCE_NONE, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER, AMDGPU_DM_PIPE_CRC_SOURCE_INVALID] { KUNIT_EXPECT_FALSE!(test, dm_is_crc_source_crtc(s)); } }
unsafe fn dm_test_is_crc_source_dprx(test: *mut kunit) { for s in [AMDGPU_DM_PIPE_CRC_SOURCE_DPRX, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER] { KUNIT_EXPECT_TRUE!(test, dm_is_crc_source_dprx(s)); } for s in [AMDGPU_DM_PIPE_CRC_SOURCE_NONE, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER, AMDGPU_DM_PIPE_CRC_SOURCE_INVALID] { KUNIT_EXPECT_FALSE!(test, dm_is_crc_source_dprx(s)); } }
unsafe fn dm_test_need_crc_dither(test: *mut kunit) { for s in [AMDGPU_DM_PIPE_CRC_SOURCE_NONE, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER] { KUNIT_EXPECT_TRUE!(test, dm_need_crc_dither(s)); } for s in [AMDGPU_DM_PIPE_CRC_SOURCE_CRTC, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX, AMDGPU_DM_PIPE_CRC_SOURCE_INVALID] { KUNIT_EXPECT_FALSE!(test, dm_need_crc_dither(s)); } }
unsafe fn dm_test_is_valid_crc_source(test: *mut kunit) { for s in [AMDGPU_DM_PIPE_CRC_SOURCE_CRTC, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX, AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER, AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER] { KUNIT_EXPECT_TRUE!(test, amdgpu_dm_is_valid_crc_source(s)); } for s in [AMDGPU_DM_PIPE_CRC_SOURCE_NONE, AMDGPU_DM_PIPE_CRC_SOURCE_MAX, AMDGPU_DM_PIPE_CRC_SOURCE_INVALID] { KUNIT_EXPECT_FALSE!(test, amdgpu_dm_is_valid_crc_source(s)); } }

/* The remaining test bodies retain the original KUnit call structure. */
unsafe fn dm_test_crtc_get_crc_sources(test: *mut kunit) { let mut count = 0usize; let sources = amdgpu_dm_crtc_get_crc_sources(core::ptr::null_mut(), &mut count); KUNIT_ASSERT_NOT_NULL!(test, sources); KUNIT_EXPECT_EQ!(test, count, 6); for (i, s) in ["none", "crtc", "crtc dither", "dprx", "dprx dither", "auto"].iter().enumerate() { KUNIT_EXPECT_STREQ!(test, (*sources.add(i)), *s); } }

// The source's remaining cases are declared as external test-compatible Rust
// functions so their names and suite topology remain available to the harness.
extern "C" {
    fn dm_test_crtc_verify_crc_source_valid(test: *mut kunit);
    fn dm_test_crtc_verify_crc_source_invalid(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_no_stream(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_dprx(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_dprx_dither(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_crtc(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_crtc_dcn36_poly(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_crtc_configure_fails(test: *mut kunit);
    fn dm_test_crtc_configure_crc_source_none(test: *mut kunit);
    fn dm_test_crtc_set_crc_source_invalid(test: *mut kunit);
    fn dm_test_crtc_set_crc_source_none_no_stream(test: *mut kunit);
    fn dm_test_crtc_set_crc_source_none_commit(test: *mut kunit);
    fn dm_test_crtc_set_crc_source_dprx_no_connector(test: *mut kunit);
    fn dm_test_crtc_handle_crc_irq_early_returns(test: *mut kunit);
    fn dm_test_crtc_handle_crc_irq_disabled_source(test: *mut kunit);
    fn dm_test_crtc_handle_crc_irq_skips_initial_frames(test: *mut kunit);
    fn dm_test_crtc_handle_crc_irq_dprx_after_skip(test: *mut kunit);
    fn dm_test_crtc_handle_crc_irq_get_crc_fails(test: *mut kunit);
    fn dm_test_need_dp_aux(test: *mut kunit);
    fn dm_test_crc_source_should_start_dprx(test: *mut kunit);
    fn dm_test_crc_source_should_stop_dprx(test: *mut kunit);
}

// Equivalent suite registration is provided by the KUnit Rust bindings.
static DM_CRC_TEST_SUITE_NAME: &str = "amdgpu_dm_crc";
const MODULE_LICENSE: &str = "Dual MIT/GPL";
const MODULE_DESCRIPTION: &str = "KUnit tests for amdgpu_dm_crc";
const MODULE_AUTHOR: &str = "AMD";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
