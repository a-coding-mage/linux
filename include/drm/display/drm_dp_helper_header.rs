#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding DRM translation.
pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type size_t = usize;
pub type ssize_t = isize;
pub type c_ulong = usize;

#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_dp_aux_cec { pub lock: mutex, pub adap: *mut cec_adapter, pub connector: *mut drm_connector, pub unregister_work: delayed_work }
#[repr(C)] pub struct drm_panel { _private: [u8; 0] }
#[repr(C)] pub struct drm_crtc { _private: [u8; 0] }
#[repr(C)] pub struct drm_connector { _private: [u8; 0] }
#[repr(C)] pub struct drm_edid { _private: [u8; 0] }
#[repr(C)] pub struct drm_printer { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct cec_adapter { _private: [u8; 0] }
#[repr(C)] pub struct edid { _private: [u8; 0] }
#[repr(C)] pub struct drm_display_mode { _private: [u8; 0] }
#[repr(C)] pub struct dp_sdp { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }

pub type drm_dp_phy = c_int;
pub type dp_pixelformat = c_int;
pub type dp_colorimetry = c_int;
pub type dp_dynamic_range = c_int;
pub type dp_content_type = c_int;
pub type operation_mode = c_int;
pub type drm_mode_subconnector = c_int;
pub type drm_connector_status = c_int;

#[repr(C)]
pub struct drm_dp_vsc_sdp { pub sdp_type: u8, pub revision: u8, pub length: u8, pub pixelformat: dp_pixelformat, pub colorimetry: dp_colorimetry, pub bpc: c_int, pub dynamic_range: dp_dynamic_range, pub content_type: dp_content_type }
#[repr(C)]
pub struct drm_dp_as_sdp { pub sdp_type: u8, pub revision: u8, pub length: u8, pub vtotal: c_int, pub target_rr: c_int, pub duration_incr_ms: c_int, pub duration_decr_ms: c_int, pub target_rr_divider: bool, pub mode: operation_mode, pub coasting_vtotal: c_int }

extern "C" {
    pub fn drm_dp_channel_eq_ok(link_status: *const u8, lane_count: c_int) -> bool;
    pub fn drm_dp_clock_recovery_ok(link_status: *const u8, lane_count: c_int) -> bool;
    pub fn drm_dp_post_lt_adj_req_in_progress(link_status: *const u8) -> bool;
    pub fn drm_dp_get_adjust_request_voltage(link_status: *const u8, lane: c_int) -> u8;
    pub fn drm_dp_get_adjust_request_pre_emphasis(link_status: *const u8, lane: c_int) -> u8;
    pub fn drm_dp_get_adjust_tx_ffe_preset(link_status: *const u8, lane: c_int) -> u8;
    pub fn drm_dp_read_clock_recovery_delay(aux: *mut drm_dp_aux, dpcd: *const u8, dp_phy: drm_dp_phy, uhbr: bool) -> c_int;
    pub fn drm_dp_read_channel_eq_delay(aux: *mut drm_dp_aux, dpcd: *const u8, dp_phy: drm_dp_phy, uhbr: bool) -> c_int;
    pub fn drm_dp_link_train_clock_recovery_delay(aux: *const drm_dp_aux, dpcd: *const u8);
    pub fn drm_dp_lttpr_link_train_clock_recovery_delay();
    pub fn drm_dp_link_train_channel_eq_delay(aux: *const drm_dp_aux, dpcd: *const u8);
    pub fn drm_dp_lttpr_link_train_channel_eq_delay(aux: *const drm_dp_aux, caps: *const u8);
    pub fn drm_dp_128b132b_read_aux_rd_interval(aux: *mut drm_dp_aux) -> c_int;
    pub fn drm_dp_128b132b_lane_channel_eq_done(link_status: *const u8, lane_count: c_int) -> bool;
    pub fn drm_dp_128b132b_lane_symbol_locked(link_status: *const u8, lane_count: c_int) -> bool;
    pub fn drm_dp_128b132b_eq_interlane_align_done(link_status: *const u8) -> bool;
    pub fn drm_dp_128b132b_cds_interlane_align_done(link_status: *const u8) -> bool;
    pub fn drm_dp_128b132b_link_training_failed(link_status: *const u8) -> bool;
    pub fn drm_dp_link_rate_to_bw_code(link_rate: c_int) -> u8;
    pub fn drm_dp_bw_code_to_link_rate(link_bw: u8) -> c_int;
    pub fn drm_dp_phy_name(dp_phy: drm_dp_phy) -> *const c_char;
}

#[inline] pub unsafe fn drm_dp_max_link_rate(dpcd: *const u8) -> c_int { drm_dp_bw_code_to_link_rate(*dpcd.add(DP_MAX_LINK_RATE as usize)) }
#[inline] pub unsafe fn drm_dp_max_lane_count(dpcd: *const u8) -> u8 { *dpcd.add(DP_MAX_LANE_COUNT as usize) & DP_MAX_LANE_COUNT_MASK }
#[inline] pub unsafe fn drm_dp_enhanced_frame_cap(dpcd: *const u8) -> bool { *dpcd.add(DP_DPCD_REV as usize) >= 0x11 && (*dpcd.add(DP_MAX_LANE_COUNT as usize) & DP_ENHANCED_FRAME_CAP) != 0 }
#[inline] pub unsafe fn drm_dp_post_lt_adj_req_supported(dpcd: *const u8) -> bool { *dpcd.add(DP_DPCD_REV as usize) >= 0x13 && (*dpcd.add(DP_MAX_LANE_COUNT as usize) & DP_POST_LT_ADJ_REQ_SUPPORTED) != 0 }
#[inline] pub unsafe fn drm_dp_fast_training_cap(dpcd: *const u8) -> bool { *dpcd.add(DP_DPCD_REV as usize) >= 0x11 && (*dpcd.add(DP_MAX_DOWNSPREAD as usize) & DP_NO_AUX_HANDSHAKE_LINK_TRAINING) != 0 }
#[inline] pub unsafe fn drm_dp_tps3_supported(dpcd: *const u8) -> bool { *dpcd.add(DP_DPCD_REV as usize) >= 0x12 && (*dpcd.add(DP_MAX_LANE_COUNT as usize) & DP_TPS3_SUPPORTED) != 0 }
#[inline] pub unsafe fn drm_dp_max_downspread(dpcd: *const u8) -> bool { *dpcd.add(DP_DPCD_REV as usize) >= 0x11 || (*dpcd.add(DP_MAX_DOWNSPREAD as usize) & DP_MAX_DOWNSPREAD_0_5) != 0 }
#[inline] pub unsafe fn drm_dp_tps4_supported(dpcd: *const u8) -> bool { *dpcd.add(DP_DPCD_REV as usize) >= 0x14 && (*dpcd.add(DP_MAX_DOWNSPREAD as usize) & DP_TPS4_SUPPORTED) != 0 }
#[inline] pub unsafe fn drm_dp_training_pattern_mask(dpcd: *const u8) -> u8 { if *dpcd.add(DP_DPCD_REV as usize) >= 0x14 { DP_TRAINING_PATTERN_MASK_1_4 } else { DP_TRAINING_PATTERN_MASK } }
#[inline] pub unsafe fn drm_dp_is_branch(dpcd: *const u8) -> bool { (*dpcd.add(DP_DOWNSTREAMPORT_PRESENT as usize) & DP_DWN_STRM_PORT_PRESENT) != 0 }

#[repr(C)] pub struct drm_dp_aux_msg { pub address: c_ulong, pub request: u8, pub reply: u8, pub buffer: *mut c_void, pub size: size_t }
#[repr(C)] pub struct drm_dp_aux { pub name: *const c_char, pub ddc: i2c_adapter, pub dev: *mut device, pub drm_dev: *mut drm_device, pub crtc: *mut drm_crtc, pub hw_mutex: mutex, pub crc_work: work_struct, pub crc_count: u8, pub transfer: Option<unsafe extern "C" fn(*mut drm_dp_aux, *mut drm_dp_aux_msg) -> ssize_t>, pub wait_hpd_asserted: Option<unsafe extern "C" fn(*mut drm_dp_aux, c_ulong) -> c_int>, pub i2c_nack_count: c_uint, pub i2c_defer_count: c_uint, pub cec: drm_dp_aux_cec, pub is_remote: bool, pub powered_down: bool, pub no_zero_sized: bool, pub dpcd_probe_disabled: bool }
pub type c_uint = u32;

extern "C" {
    pub fn drm_dp_dpcd_probe(aux: *mut drm_dp_aux, offset: c_uint) -> c_int;
    pub fn drm_dp_dpcd_set_powered(aux: *mut drm_dp_aux, powered: bool);
    pub fn drm_dp_dpcd_set_probe(aux: *mut drm_dp_aux, enable: bool);
    pub fn drm_dp_dpcd_read(aux: *mut drm_dp_aux, offset: c_uint, buffer: *mut c_void, size: size_t) -> ssize_t;
    pub fn drm_dp_dpcd_write(aux: *mut drm_dp_aux, offset: c_uint, buffer: *mut c_void, size: size_t) -> ssize_t;
}

#[inline] pub unsafe fn drm_dp_dpcd_readb(aux: *mut drm_dp_aux, offset: c_uint, valuep: *mut u8) -> ssize_t { drm_dp_dpcd_read(aux, offset, valuep.cast(), 1) }
#[inline] pub unsafe fn drm_dp_dpcd_read_data(aux: *mut drm_dp_aux, offset: c_uint, buffer: *mut c_void, size: size_t) -> c_int { let ret = drm_dp_dpcd_read(aux, offset, buffer, size); if ret >= 0 { return if ret < size as isize { -EPROTO } else { 0 }; } let buf = buffer as *mut u8; for i in 0..size { let r = drm_dp_dpcd_readb(aux, offset + i as u32, buf.add(i)); if r < 0 { return r as c_int; } } 0 }
#[inline] pub unsafe fn drm_dp_dpcd_write_data(aux: *mut drm_dp_aux, offset: c_uint, buffer: *mut c_void, size: size_t) -> c_int { let ret = drm_dp_dpcd_write(aux, offset, buffer, size); if ret < 0 { ret as c_int } else if ret < size as isize { -EPROTO } else { 0 } }
#[inline] pub unsafe fn drm_dp_dpcd_writeb(aux: *mut drm_dp_aux, offset: c_uint, value: u8) -> ssize_t { drm_dp_dpcd_write(aux, offset, (&value as *const u8 as *mut u8).cast(), 1) }
#[inline] pub unsafe fn drm_dp_dpcd_read_byte(aux: *mut drm_dp_aux, offset: c_uint, valuep: *mut u8) -> c_int { drm_dp_dpcd_read_data(aux, offset, valuep.cast(), 1) }
#[inline] pub unsafe fn drm_dp_dpcd_write_byte(aux: *mut drm_dp_aux, offset: c_uint, value: u8) -> c_int { drm_dp_dpcd_write_data(aux, offset, (&value as *const u8 as *mut u8).cast(), 1) }

#[repr(C, packed)] pub struct drm_dp_dpcd_ident { pub oui: [u8; 3], pub device_id: [u8; 6], pub hw_rev: u8, pub sw_major_rev: u8, pub sw_minor_rev: u8 }
#[repr(C)] pub struct drm_dp_desc { pub ident: drm_dp_dpcd_ident, pub quirks: u32 }
#[repr(i32)] pub enum drm_dp_quirk { DP_DPCD_QUIRK_CONSTANT_N, DP_DPCD_QUIRK_NO_PSR, DP_DPCD_QUIRK_NO_SINK_COUNT, DP_DPCD_QUIRK_DSC_WITHOUT_VIRTUAL_DPCD, DP_DPCD_QUIRK_CAN_DO_MAX_LINK_RATE_3_24_GBPS, DP_DPCD_QUIRK_HBLANK_EXPANSION_REQUIRES_DSC, DP_DPCD_QUIRK_DSC_THROUGHPUT_BPP_LIMIT }
#[inline] pub unsafe fn drm_dp_has_quirk(desc: *const drm_dp_desc, quirk: drm_dp_quirk) -> bool { ((*desc).quirks & (1u32 << quirk as u32)) != 0 }

#[repr(C)] pub struct drm_edp_backlight_info { pub pwmgen_bit_count: u8, pub pwm_freq_pre_divider: u8, pub max: u32, pub lsb_reg_used: bool, pub aux_enable: bool, pub aux_set: bool, pub luminance_set: bool }
#[repr(C)] pub struct drm_dp_phy_test_params { pub link_rate: c_int, pub num_lanes: u8, pub phy_pattern: u8, pub hbr2_reset: [u8; 2], pub custom80: [u8; 10], pub enhanced_frame_cap: bool }

// Remaining non-inline declarations retain the C ABI and external linkage.
extern "C" {
    pub fn drm_dp_vsc_sdp_log(p: *mut drm_printer, vsc: *const drm_dp_vsc_sdp);
    pub fn drm_dp_as_sdp_log(p: *mut drm_printer, as_sdp: *const drm_dp_as_sdp);
    pub fn drm_dp_vsc_sdp_supported(aux: *mut drm_dp_aux, dpcd: *const u8) -> bool;
    pub fn drm_dp_as_sdp_supported(aux: *mut drm_dp_aux, dpcd: *const u8) -> bool;
    pub fn drm_dp_psr_setup_time(psr_cap: *const u8) -> c_int;
    pub fn drm_dp_dsc_sink_bpp_incr(dsc_dpcd: *const u8) -> u8;
    pub fn drm_dp_dsc_slice_count_to_mask(slice_count: c_int) -> u32;
    pub fn drm_dp_sink_supports_fec(fec_capable: u8) -> bool;
    pub fn drm_dp_channel_coding_supported(dpcd: *const u8) -> bool;
    pub fn drm_dp_128b132b_supported(dpcd: *const u8) -> bool;
    pub fn drm_dp_alternate_scrambler_reset_cap(dpcd: *const u8) -> bool;
    pub fn drm_dp_sink_can_do_video_without_timing_msa(dpcd: *const u8) -> bool;
    pub fn drm_edp_backlight_supported(edp_dpcd: *const u8) -> bool;
    pub fn drm_dp_is_uhbr_rate(link_rate: c_int) -> bool;
    pub fn drm_dp_read_dpcd_caps(aux: *mut drm_dp_aux, dpcd: *mut u8) -> c_int;
    pub fn drm_dp_dpcd_read_link_status(aux: *mut drm_dp_aux, status: *mut u8) -> c_int;
    pub fn drm_dp_dpcd_read_phy_link_status(aux: *mut drm_dp_aux, dp_phy: drm_dp_phy, link_status: *mut u8) -> c_int;
    pub fn drm_dp_aux_init(aux: *mut drm_dp_aux);
    pub fn drm_dp_aux_register(aux: *mut drm_dp_aux) -> c_int;
    pub fn drm_dp_aux_unregister(aux: *mut drm_dp_aux);
    pub fn drm_dp_read_desc(aux: *mut drm_dp_aux, desc: *mut drm_dp_desc, is_branch: bool) -> c_int;
    pub fn drm_dp_get_phy_test_pattern(aux: *mut drm_dp_aux, data: *mut drm_dp_phy_test_params) -> c_int;
    pub fn drm_dp_set_phy_test_pattern(aux: *mut drm_dp_aux, data: *mut drm_dp_phy_test_params, dp_rev: u8) -> c_int;
}

// Constants and declarations supplied by drm_dp.h and other included headers.
extern "C" {
    static DP_MAX_LINK_RATE: c_int; static DP_MAX_LANE_COUNT: c_int; static DP_DPCD_REV: c_int;
    static DP_MAX_DOWNSPREAD: c_int; static DP_DOWNSTREAMPORT_PRESENT: c_int;
    static DP_MAX_LANE_COUNT_MASK: u8; static DP_ENHANCED_FRAME_CAP: u8;
    static DP_POST_LT_ADJ_REQ_SUPPORTED: u8; static DP_NO_AUX_HANDSHAKE_LINK_TRAINING: u8;
    static DP_TPS3_SUPPORTED: u8; static DP_MAX_DOWNSPREAD_0_5: u8; static DP_TPS4_SUPPORTED: u8;
    static DP_TRAINING_PATTERN_MASK_1_4: u8; static DP_TRAINING_PATTERN_MASK: u8; static DP_DWN_STRM_PORT_PRESENT: u8;
    static EPROTO: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
