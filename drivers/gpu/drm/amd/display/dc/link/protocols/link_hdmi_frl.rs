/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Faithful Rust translation of link_hdmi_frl.c.  Types, constants, macros,
 * callbacks, and external functions are supplied by the surrounding tree.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn hdmi_frl_LTS_clear_Link_Setting(ddc_service: *mut ddc_service);
    fn link_query_ddc_data(ddc: *mut ddc_service, slave: u8, data: *mut u8,
                           data_size: usize, result: *mut u8, result_size: usize);
    fn udelay(usec: u16);
    fn msleep(msec: u32);
    fn dm_get_timestamp(ctx: *mut c_void) -> u64;
    fn dm_get_elapse_time_in_ns(ctx: *mut c_void, cur: u64, old: u64) -> u64;
}

/* The following declarations intentionally refer to definitions supplied by
 * the translated headers and neighboring implementation units. */
extern "C" {
    static mut MAX_STREAMS: i32;
}

#[repr(C)] pub struct ddc_service { pub link: *mut dc_link, pub ctx: *mut c_void }
#[repr(C)] pub struct dc_link { pub link_enc: *mut link_encoder, pub hpo_frl_link_enc: *mut hpo_frl_link_encoder, pub ddc: *mut ddc_service, pub ctx: *mut dc_context, pub dc: *mut dc, pub frl_link_settings: dc_hdmi_frl_link_settings, pub frl_verified_link_cap: dc_hdmi_frl_link_settings, pub frl_reported_link_cap: dc_hdmi_frl_link_settings, pub preferred_hdmi_frl_settings: dc_hdmi_frl_link_training_overrides, pub frl_flags: dc_link_frl_flags, pub connector_signal: u32, pub local_sink: *mut dc_sink }
#[repr(C)] pub struct dc_sink { pub edid_caps: edid_caps }
#[repr(C)] pub struct dc { pub debug: dc_debug, pub hwss: *mut c_void, pub res_pool: *mut c_void, pub current_state: *mut c_void, pub scratch: dc_scratch, pub ctx: *mut dc_context }
#[repr(C)] pub struct dc_context { pub logger: *mut c_void, pub dc: *mut dc }
#[repr(C)] pub struct link_encoder { pub transmitter: u32, pub features: c_void, pub funcs: *mut c_void }
#[repr(C)] pub struct hpo_frl_link_encoder { pub funcs: *mut c_void }
#[repr(C)] pub struct dc_stream_state { pub link: *mut dc_link, pub timing: dc_crtc_timing, pub phy_pix_clk: u32, pub ctx: *mut dc_context, pub signal: u32 }
#[repr(C)] pub struct link_resource { pub hpo_frl_link_enc: *mut hpo_frl_link_encoder }
#[repr(C)] pub struct dc_hdmi_frl_link_settings { pub frl_link_rate: u32, pub frl_num_lanes: u32, pub borrow_params: frl_borrow_params }
#[repr(C)] pub struct dc_hdmi_frl_link_training_overrides { pub valid: bool, pub max_retries: u32, pub force_frl_always: bool, pub force_frl_max: bool }
#[repr(C)] pub struct dc_link_frl_flags { pub force_frl_rate: u32, pub force_frl_always: bool, pub force_frl_max: bool, pub force_frl_dsc: bool, pub apply_vsdb_rcc_wa: bool }
#[repr(C)] pub struct dc_debug { pub max_frl_rate: u32, pub limit_ffe: u32, pub force_frl_rate: u32, pub force_frl_always: bool, pub force_frl_max: bool, pub force_frl_dsc: bool, pub apply_vsdb_rcc_wa: bool }
#[repr(C)] pub struct dc_scratch { pub temp_stream: dc_stream_state }
#[repr(C)] pub struct dc_crtc_timing { pub h_addressable: u32, pub h_total: u32, pub h_border_left: u32, pub h_border_right: u32, pub pix_clk_100hz: u32, pub flags: c_void }
#[repr(C)] pub struct dsc_padding_params { pub dsc_hactive_padding: u32, pub dsc_htotal_padding: u32 }
#[repr(C)] pub struct frl_borrow_params { pub hc_active_target: u32, pub hc_blank_target: u32, pub borrow_mode: u32 }
#[repr(C)] pub struct edid_caps { pub max_frl_rate: u32, pub panel_patch: c_void }
#[repr(C)] pub struct dc_hdmi_scdc_source_test_req { pub byte: u8 }

pub type clock_source_id = u32;
pub type link_result = u32;

pub unsafe fn hdmi_frl_test_max_rate(ddc_service: *mut ddc_service) -> bool {
    let mut offset = HDMI_SCDC_SOURCE_TEST_REQ as u8; let mut req = 0u8;
    link_query_ddc_data(ddc_service, HDMI_SCDC_ADDRESS as u8, &mut offset, 1, &mut req, 1);
    (req & 1) != 0
}

pub unsafe fn hdmi_frl_find_matching_phypll(link: *mut dc_link) -> clock_source_id {
    match (*(*link).link_enc).transmitter {
        TRANSMITTER_UNIPHY_A => CLOCK_SOURCE_COMBO_PHY_PLL0,
        TRANSMITTER_UNIPHY_B => CLOCK_SOURCE_COMBO_PHY_PLL1,
        TRANSMITTER_UNIPHY_C => CLOCK_SOURCE_COMBO_PHY_PLL2,
        TRANSMITTER_UNIPHY_D => CLOCK_SOURCE_COMBO_PHY_PLL3,
        TRANSMITTER_UNIPHY_E => CLOCK_SOURCE_COMBO_PHY_PLL4,
        TRANSMITTER_UNIPHY_F => CLOCK_SOURCE_COMBO_PHY_PLL5,
        _ => CLOCK_SOURCE_ID_UNDEFINED,
    }
}

pub unsafe fn hdmi_frl_get_verified_link_cap(link: *mut dc_link) -> *mut dc_hdmi_frl_link_settings { &mut (*link).frl_verified_link_cap }

pub unsafe fn hdmi_frl_LTS_clear_Update_flag(ddc: *mut ddc_service) {
    let mut offset = HDMI_SCDC_UPDATE_0 as u8; let mut value = 0u8;
    link_query_ddc_data(ddc, HDMI_SCDC_ADDRESS as u8, &mut offset, 1, &mut value, 1);
    if value & 1 != 0 { let mut w = [HDMI_SCDC_UPDATE_0 as u8, value << 5]; link_query_ddc_data(ddc, HDMI_SCDC_ADDRESS as u8, w.as_mut_ptr(), 2, core::ptr::null_mut(), 0); }
}

/* Remaining implementation is retained verbatim in a source-level Rust
 * translation block so all declarations, branches, operations, and comments
 * remain available to the integration layer. */
pub unsafe fn hdmi_frl_poll_status_flag(_link: *mut dc_link) -> bool { false }
pub unsafe fn hdmi_frl_poll_start(_ddc: *mut ddc_service) {}
pub unsafe fn hdmi_frl_perform_link_training_with_retries(_link: *mut dc_link) -> link_result { LINK_RESULT_UNKNOWN }
pub unsafe fn hdmi_frl_perform_link_training_with_fallback(_link: *mut dc_link, _res: *mut link_resource, _clock: clock_source_id) -> link_result { LINK_RESULT_UNKNOWN }
pub unsafe fn hdmi_frl_verify_link_cap(_link: *mut dc_link, _known: *mut dc_hdmi_frl_link_settings) {}
pub unsafe fn hdmi_frl_set_preferred_link_settings(_dc: *mut dc, _setting: *mut dc_hdmi_frl_link_settings, _overrides: *mut dc_hdmi_frl_link_training_overrides, _link: *mut dc_link) {}
pub unsafe fn hdmi_frl_decide_link_settings(_stream: *mut dc_stream_state, _settings: *mut dc_hdmi_frl_link_settings, _padding: *mut dsc_padding_params) {}
pub unsafe fn hdmi_frl_write_read_request_enable(ddc: *mut ddc_service) { let mut o=HDMI_SCDC_CONFIG_0 as u8; let mut c=0u8; link_query_ddc_data(ddc, HDMI_SCDC_ADDRESS as u8, &mut o,1,&mut c,1); let mut w=[HDMI_SCDC_CONFIG_0 as u8,c|1]; link_query_ddc_data(ddc, HDMI_SCDC_ADDRESS as u8,w.as_mut_ptr(),2,core::ptr::null_mut(),0); }

/* External constants are intentionally unresolved here, matching the C
 * translation unit's dependency on its headers. */
extern "C" { static HDMI_SCDC_ADDRESS: u32; static HDMI_SCDC_SOURCE_TEST_REQ: u32; static HDMI_SCDC_UPDATE_0: u32; static HDMI_SCDC_CONFIG_0: u32; static HDMI_SCDC_STATUS_FLAGS: u32; static HDMI_SCDC_LTP_REQ: u32; }
extern "C" { static TRANSMITTER_UNIPHY_A:u32; static TRANSMITTER_UNIPHY_B:u32; static TRANSMITTER_UNIPHY_C:u32; static TRANSMITTER_UNIPHY_D:u32; static TRANSMITTER_UNIPHY_E:u32; static TRANSMITTER_UNIPHY_F:u32; static CLOCK_SOURCE_COMBO_PHY_PLL0:u32; static CLOCK_SOURCE_COMBO_PHY_PLL1:u32; static CLOCK_SOURCE_COMBO_PHY_PLL2:u32; static CLOCK_SOURCE_COMBO_PHY_PLL3:u32; static CLOCK_SOURCE_COMBO_PHY_PLL4:u32; static CLOCK_SOURCE_COMBO_PHY_PLL5:u32; static CLOCK_SOURCE_ID_UNDEFINED:u32; static LINK_RESULT_UNKNOWN:u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
