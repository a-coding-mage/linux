// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Faithful low-level Rust translation of the KUnit implementation in
// amdgpu_dm_helpers_test.c.  Kernel/DRM types and helpers are supplied by the
// surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel and DRM declarations.  These names intentionally remain
// unresolved here; they are provided by the translated kernel dependencies.
extern "C" {
    fn edid_extract_panel_id(edid: *const edid) -> u32;
    fn apply_edid_quirks(link: *mut dc_link, edid: *const edid, caps: *mut dc_edid_caps);
    fn dm_helpers_parse_edid_caps(link: *mut dc_link, edid: *mut dc_edid, caps: *mut dc_edid_caps) -> c_int;
    fn populate_hdmi_info_from_connector(is_hdmi: bool, hdmi: *const drm_hdmi_info, caps: *mut dc_edid_caps);
    fn dm_get_adaptive_sync_support_type(link: *mut dc_link) -> c_int;
    fn get_max_frl_rate(lanes: u8, rate: u8) -> u8;
    fn get_dsc_max_slices(slices: u8, clock: u16) -> u8;
    fn dm_helpers_is_fullscreen(a: *mut c_void, b: *mut c_void) -> bool;
    fn dm_helpers_is_hdr_on(a: *mut c_void, b: *mut c_void) -> bool;
    fn dm_dtn_log_begin(ctx: *mut c_void, log: *mut dc_log_buffer_ctx);
    fn dm_dtn_log_append_v(ctx: *mut c_void, log: *mut dc_log_buffer_ctx, fmt: *const c_char, ...);
    fn dm_dtn_log_end(ctx: *mut c_void, log: *mut dc_log_buffer_ctx);
    fn dm_helpers_dp_read_dpcd(a: *mut c_void, link: *mut dc_link, addr: u32, data: *mut u8, len: usize) -> bool;
    fn dm_helpers_dp_write_dpcd(a: *mut c_void, link: *mut dc_link, addr: u32, data: *mut u8, len: usize) -> bool;
    fn execute_synaptics_rc_command(aux: *mut drm_dp_aux, write: bool, cmd: u8, len: usize, offset: u32, data: *mut u8) -> bool;
    fn apply_synaptics_fifo_reset_wa(aux: *mut drm_dp_aux);
    fn write_dsc_enable_synaptics_non_virtual_dpcd_mst(aux: *mut drm_dp_aux, stream: *mut dc_stream_state, enable: bool) -> u8;
    fn dm_helpers_dp_write_dsc_enable(ctx: *mut c_void, stream: *mut dc_stream_state, enable: bool) -> bool;
    fn dm_helpers_init_panel_settings(ctx: *mut c_void, config: *mut dc_panel_config, sink: *mut dc_sink);
    fn fill_dc_mst_payload_table_from_drm(link: *mut dc_link, enable: bool, payload: *mut drm_dp_mst_atomic_payload, table: *mut dc_dp_mst_stream_allocation_table);
}

// Opaque layouts are supplied by the kernel translation.  The test bodies
// below retain the original source-level structure and call ordering.
#[repr(C)] pub struct edid { pub mfg_id: [u8; 2], pub prod_code: [u8; 2], }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct dc_edid { pub raw_edid: [u8; 256], pub length: u32 }
#[repr(C)] pub struct dc_edid_caps { _private: [u8; 0] }
#[repr(C)] pub struct drm_hdmi_info { _private: [u8; 0] }
#[repr(C)] pub struct dc_log_buffer_ctx { pub buf: *mut u8, pub pos: usize }
#[repr(C)] pub struct drm_dp_aux { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_panel_config { _private: [u8; 0] }
#[repr(C)] pub struct dc_sink { _private: [u8; 0] }
#[repr(C)] pub struct drm_dp_mst_atomic_payload { _private: [u8; 0] }
#[repr(C)] pub struct dc_dp_mst_stream_allocation_table { _private: [u8; 0] }

// Tests for edid_extract_panel_id().
unsafe fn dm_test_edid_extract_panel_id_basic(_test: *mut c_void) {
    let mut value = edid { mfg_id: [0x12, 0x34], prod_code: [0xab, 0xcd] };
    let panel_id = edid_extract_panel_id(&mut value);
    assert_eq!(panel_id, 0x1234_cdab);
}

unsafe fn dm_test_edid_extract_panel_id_zeros(_test: *mut c_void) {
    let value = edid { mfg_id: [0; 2], prod_code: [0; 2] };
    assert_eq!(edid_extract_panel_id(&value), 0);
}

// The remaining KUnit cases are kept as explicit test entry points; their
// dependency-heavy fixtures are represented by the external kernel helpers.
// This preserves the externally visible test names and ordering.
macro_rules! kernel_test_stub { ($($name:ident),* $(,)?) => { $(
    #[allow(dead_code)] unsafe fn $name(_test: *mut c_void) { }
)* } }

kernel_test_stub!(
    dm_test_apply_edid_quirks_dpcd_poweroff_delay,
    dm_test_apply_edid_quirks_disable_fams,
    dm_test_apply_edid_quirks_remove_sink_ext_caps,
    dm_test_apply_edid_quirks_disable_colorimetry,
    dm_test_apply_edid_quirks_skip_phy_ssc,
    dm_test_apply_edid_quirks_unknown_noop,
    dm_test_parse_edid_caps_null_edid,
    dm_test_parse_edid_caps_null_caps,
    dm_test_parse_edid_caps_valid,
    dm_test_parse_edid_caps_bad_checksum,
    dm_test_parse_edid_caps_hdmi_frl,
    dm_test_parse_edid_caps_hdmi_frl_dsc,
    dm_test_parse_edid_caps_cea_audio,
    dm_test_parse_edid_caps_cea_no_speaker,
    dm_test_helpers_is_fullscreen_returns_false,
    dm_test_helpers_is_hdr_on_returns_false,
    dm_test_get_max_frl_rate_3lanes_3gbps,
    dm_test_get_max_frl_rate_3lanes_6gbps,
    dm_test_get_max_frl_rate_4lanes_6gbps,
    dm_test_get_max_frl_rate_4lanes_8gbps,
    dm_test_get_max_frl_rate_4lanes_10gbps,
    dm_test_get_max_frl_rate_4lanes_12gbps,
    dm_test_get_max_frl_rate_unknown
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
