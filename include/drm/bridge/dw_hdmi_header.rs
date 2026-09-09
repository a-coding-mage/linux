/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2011 Freescale Semiconductor, Inc. */

// Dependency declarations supplied by other translated units.
use core::ffi::c_void;

pub const DW_HDMI_RES_8: u32 = 0;
pub const DW_HDMI_RES_10: u32 = 1;
pub const DW_HDMI_RES_12: u32 = 2;
pub const DW_HDMI_RES_MAX: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dw_hdmi_phy_type {
    DW_HDMI_PHY_DWC_HDMI_TX_PHY = 0x00,
    DW_HDMI_PHY_DWC_MHL_PHY_HEAC = 0xb2,
    DW_HDMI_PHY_DWC_MHL_PHY = 0xc2,
    DW_HDMI_PHY_DWC_HDMI_3D_TX_PHY_HEAC = 0xe2,
    DW_HDMI_PHY_DWC_HDMI_3D_TX_PHY = 0xf2,
    DW_HDMI_PHY_DWC_HDMI20_TX_PHY = 0xf3,
    DW_HDMI_PHY_VENDOR_PHY = 0xfe,
}

#[repr(C)]
pub struct dw_hdmi_mpll_config {
    pub mpixelclock: libc::c_ulong,
    pub res: [dw_hdmi_mpll_config_res; DW_HDMI_RES_MAX],
}
#[repr(C)]
pub struct dw_hdmi_mpll_config_res { pub cpce: u16, pub gmp: u16 }

#[repr(C)]
pub struct dw_hdmi_curr_ctrl {
    pub mpixelclock: libc::c_ulong,
    pub curr: [u16; DW_HDMI_RES_MAX],
}

#[repr(C)]
pub struct dw_hdmi_phy_config {
    pub mpixelclock: libc::c_ulong,
    pub sym_ctr: u16,
    pub term: u16,
    pub vlev_ctr: u16,
}

#[repr(C)]
pub struct dw_hdmi_phy_ops {
    pub init: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void,
        *const drm_display_info, *const drm_display_mode) -> libc::c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void)>,
    pub read_hpd: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void) -> drm_connector_status>,
    pub update_hpd: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void, bool, bool, bool)>,
    pub setup_hpd: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void)>,
}

#[repr(C)]
pub struct dw_hdmi_plat_data {
    pub regm: *mut regmap,
    pub output_port: libc::c_uint,
    pub input_bus_encoding: libc::c_ulong,
    pub use_drm_infoframe: bool,
    pub ycbcr_420_allowed: bool,
    pub priv_data: *mut c_void,
    pub mode_valid: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void,
        *const drm_display_info, *const drm_display_mode) -> drm_mode_status>,
    pub priv_audio: *mut c_void,
    pub enable_audio: Option<unsafe extern "C" fn(*mut dw_hdmi, libc::c_int,
        libc::c_int, libc::c_int, libc::c_int, libc::c_int)>,
    pub disable_audio: Option<unsafe extern "C" fn(*mut dw_hdmi)>,
    pub phy_ops: *const dw_hdmi_phy_ops,
    pub phy_name: *const libc::c_char,
    pub phy_data: *mut c_void,
    pub phy_force_vendor: libc::c_uint,
    pub mpll_cfg: *const dw_hdmi_mpll_config,
    pub cur_ctr: *const dw_hdmi_curr_ctrl,
    pub phy_config: *const dw_hdmi_phy_config,
    pub configure_phy: Option<unsafe extern "C" fn(*mut dw_hdmi, *mut c_void,
        libc::c_ulong) -> libc::c_int>,
    pub disable_cec: libc::c_uint,
}

extern "C" {
    pub fn dw_hdmi_probe(pdev: *mut platform_device, plat_data: *const dw_hdmi_plat_data) -> *mut dw_hdmi;
    pub fn dw_hdmi_remove(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_unbind(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_bind(pdev: *mut platform_device, encoder: *mut drm_encoder,
        plat_data: *const dw_hdmi_plat_data) -> *mut dw_hdmi;
    pub fn dw_hdmi_resume(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_setup_rx_sense(hdmi: *mut dw_hdmi, hpd: bool, rx_sense: bool);
    pub fn dw_hdmi_set_plugged_cb(hdmi: *mut dw_hdmi, fn_: hdmi_codec_plugged_cb,
        codec_dev: *mut device) -> libc::c_int;
    pub fn dw_hdmi_set_sample_non_pcm(hdmi: *mut dw_hdmi, non_pcm: libc::c_uint);
    pub fn dw_hdmi_set_sample_iec958(hdmi: *mut dw_hdmi, iec958: libc::c_uint);
    pub fn dw_hdmi_set_sample_width(hdmi: *mut dw_hdmi, width: libc::c_uint);
    pub fn dw_hdmi_set_sample_rate(hdmi: *mut dw_hdmi, rate: libc::c_uint);
    pub fn dw_hdmi_set_channel_count(hdmi: *mut dw_hdmi, cnt: libc::c_uint);
    pub fn dw_hdmi_set_channel_status(hdmi: *mut dw_hdmi, channel_status: *mut u8);
    pub fn dw_hdmi_set_channel_allocation(hdmi: *mut dw_hdmi, ca: libc::c_uint);
    pub fn dw_hdmi_audio_enable(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_audio_disable(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_set_high_tmds_clock_ratio(hdmi: *mut dw_hdmi, display: *const drm_display_info);
    pub fn dw_hdmi_phy_i2c_set_addr(hdmi: *mut dw_hdmi, address: u8);
    pub fn dw_hdmi_phy_i2c_write(hdmi: *mut dw_hdmi, data: u16, addr: u8);
    pub fn dw_hdmi_phy_gen1_reset(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_phy_gen2_pddq(hdmi: *mut dw_hdmi, enable: u8);
    pub fn dw_hdmi_phy_gen2_txpwron(hdmi: *mut dw_hdmi, enable: u8);
    pub fn dw_hdmi_phy_gen2_reset(hdmi: *mut dw_hdmi);
    pub fn dw_hdmi_phy_read_hpd(hdmi: *mut dw_hdmi, data: *mut c_void) -> drm_connector_status;
    pub fn dw_hdmi_phy_update_hpd(hdmi: *mut dw_hdmi, data: *mut c_void,
        force: bool, disabled: bool, rxsense: bool);
    pub fn dw_hdmi_phy_setup_hpd(hdmi: *mut dw_hdmi, data: *mut c_void);
    pub fn dw_hdmi_bus_fmt_is_420(hdmi: *mut dw_hdmi) -> bool;
    pub fn dw_hdmi_to_plat_data(hdmi: *mut dw_hdmi) -> *const dw_hdmi_plat_data;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
