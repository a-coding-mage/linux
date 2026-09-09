// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding display implementation are intentionally
// referenced here rather than redefined in this translation unit.

use core::ffi::{c_char, c_void};

extern "C" {
    fn hpo_frl_link_enc3_setup_link_encoder();
    fn hpo_frl_link_enc3_set_training_pattern();
    fn hpo_frl_link_enc3_get_training_pattern();
    fn hpo_frl_link_enc3_enable_output();
    fn hpo_frl_link_enc3_disable();
    fn hpo_frl_link_enc3_read_state();
    fn hpo_frl_link_enc3_destroy();
    fn hpo_frl_link_enc3_apply_vsdb_rcc_wa();
    fn dc_log_hdmi_frl(fmt: *const c_char, ...);
    fn break_to_debugger();
    fn transmitter_control(
        bp: *mut dc_bios,
        cntl: *mut bp_transmitter_control,
    ) -> bp_result;
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dc_bios {
    pub funcs: *mut dc_bios_funcs,
}
#[repr(C)]
pub struct dc_bios_funcs {
    pub transmitter_control: Option<unsafe extern "C" fn(*mut dc_bios, *mut bp_transmitter_control) -> bp_result>,
}
#[repr(C)]
pub struct link_encoder {
    pub preferred_engine: u32,
    pub ctx: *mut dc_context,
    pub transmitter: u32,
    pub hpd_source: u32,
}
#[repr(C)]
pub struct dcn10_link_encoder {
    pub base: link_encoder,
}
#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder {
    pub base: hpo_frl_link_encoder,
    pub regs: *const dcn30_hpo_frl_link_encoder_registers,
    pub hpo_le_shift: *const dcn30_hpo_frl_link_encoder_shift,
    pub hpo_le_mask: *const dcn30_hpo_frl_link_encoder_mask,
}
#[repr(C)]
pub struct hpo_frl_link_encoder {
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub funcs: *const hpo_frl_link_encoder_funcs,
}
#[repr(C)]
pub struct hpo_frl_link_encoder_registers { _private: [u8; 0] }
#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder_registers { _private: [u8; 0] }
#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder_shift { _private: [u8; 0] }
#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder_mask { _private: [u8; 0] }
#[repr(C)]
pub struct hpo_frl_link_encoder_funcs {
    pub setup_link_encoder: Option<unsafe extern "C" fn()>,
    pub set_hdmi_training_pattern: Option<unsafe extern "C" fn()>,
    pub get_hdmi_training_pattern: Option<unsafe extern "C" fn()>,
    pub enable_frl_phy_output: Option<unsafe extern "C" fn()>,
    pub enable_output: Option<unsafe extern "C" fn()>,
    pub disable_link_encoder: Option<unsafe extern "C" fn()>,
    pub read_state: Option<unsafe extern "C" fn()>,
    pub destroy: Option<unsafe extern "C" fn()>,
    pub apply_vsdb_rcc_wa: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct bp_transmitter_control {
    pub action: u32,
    pub engine_id: u32,
    pub transmitter: u32,
    pub pll_id: clock_source_id,
    pub signal: u32,
    pub hpd_sel: u32,
    pub pixel_clock: u32,
    pub hpo_engine_id: u32,
    pub lanes_number: u32,
}

pub type clock_source_id = u32;
pub type hdmi_frl_link_rate = u32;
pub type bp_result = u32;

pub const TRANSMITTER_CONTROL_ENABLE: u32 = 0;
pub const SIGNAL_TYPE_HDMI_FRL: u32 = 0;
pub const ENGINE_ID_HPO_0: u32 = 0;
pub const BP_RESULT_OK: bp_result = 0;
pub const HDMI_FRL_LINK_RATE_3GBPS: hdmi_frl_link_rate = 0;
pub const HDMI_FRL_LINK_RATE_6GBPS: hdmi_frl_link_rate = 1;
pub const HDMI_FRL_LINK_RATE_6GBPS_4LANE: hdmi_frl_link_rate = 2;
pub const HDMI_FRL_LINK_RATE_8GBPS: hdmi_frl_link_rate = 3;
pub const HDMI_FRL_LINK_RATE_10GBPS: hdmi_frl_link_rate = 4;
pub const HDMI_FRL_LINK_RATE_12GBPS: hdmi_frl_link_rate = 5;
pub const HDMI_FRL_LINK_RATE_16GBPS: hdmi_frl_link_rate = 6;
pub const HDMI_FRL_LINK_RATE_20GBPS: hdmi_frl_link_rate = 7;

unsafe fn link_transmitter_control(
    enc10: *mut dcn10_link_encoder,
    cntl: *mut bp_transmitter_control,
) -> bp_result {
    let bp = (*enc10).base.ctx;
    ((*(*bp).funcs).transmitter_control.unwrap())(bp as *mut dc_bios, cntl)
}

unsafe fn hpo_frl_link_enc60_enable_phy_output(
    hpo_enc: *mut hpo_frl_link_encoder,
    enc: *mut link_encoder,
    clock_source: clock_source_id,
    frl_link_rate: hdmi_frl_link_rate,
) {
    let enc3 = hpo_enc as *mut dcn30_hpo_frl_link_encoder;
    let enc10 = enc as *mut dcn10_link_encoder;
    let mut cntl: bp_transmitter_control = core::mem::zeroed();
    cntl.action = TRANSMITTER_CONTROL_ENABLE;
    cntl.engine_id = (*enc).preferred_engine;
    cntl.transmitter = (*enc10).base.transmitter;
    cntl.pll_id = clock_source;
    cntl.signal = SIGNAL_TYPE_HDMI_FRL;
    cntl.hpd_sel = (*enc10).base.hpd_source;
    cntl.pixel_clock = match frl_link_rate {
        HDMI_FRL_LINK_RATE_3GBPS => 166667,
        HDMI_FRL_LINK_RATE_6GBPS | HDMI_FRL_LINK_RATE_6GBPS_4LANE => 333333,
        HDMI_FRL_LINK_RATE_8GBPS => 444444,
        HDMI_FRL_LINK_RATE_10GBPS => 555555,
        HDMI_FRL_LINK_RATE_12GBPS => 666667,
        HDMI_FRL_LINK_RATE_16GBPS => 888889,
        HDMI_FRL_LINK_RATE_20GBPS | _ => 1111111,
    };
    cntl.hpo_engine_id = (*enc3).base.inst + ENGINE_ID_HPO_0;
    cntl.lanes_number = if frl_link_rate <= HDMI_FRL_LINK_RATE_6GBPS { 3 } else { 4 };
    let result = link_transmitter_control(enc10, &mut cntl);
    if result != BP_RESULT_OK {
        dc_log_hdmi_frl(b"%s: Failed to execute VBIOS command table!\n\0".as_ptr() as *const c_char);
        break_to_debugger();
    }
}

static mut dcn60_hpo_frl_link_encoder_funcs: hpo_frl_link_encoder_funcs = hpo_frl_link_encoder_funcs {
    setup_link_encoder: Some(hpo_frl_link_enc3_setup_link_encoder),
    set_hdmi_training_pattern: Some(hpo_frl_link_enc3_set_training_pattern),
    get_hdmi_training_pattern: Some(hpo_frl_link_enc3_get_training_pattern),
    enable_frl_phy_output: Some(hpo_frl_link_enc60_enable_phy_output),
    enable_output: Some(hpo_frl_link_enc3_enable_output),
    disable_link_encoder: Some(hpo_frl_link_enc3_disable),
    read_state: Some(hpo_frl_link_enc3_read_state),
    destroy: Some(hpo_frl_link_enc3_destroy),
    apply_vsdb_rcc_wa: Some(hpo_frl_link_enc3_apply_vsdb_rcc_wa),
};

pub unsafe extern "C" fn hpo_frl_link_encoder60_construct(
    enc3: *mut dcn30_hpo_frl_link_encoder,
    ctx: *mut dc_context,
    inst: u32,
    hpo_le_regs: *const dcn30_hpo_frl_link_encoder_registers,
    hpo_le_shift: *const dcn30_hpo_frl_link_encoder_shift,
    hpo_le_mask: *const dcn30_hpo_frl_link_encoder_mask,
) {
    (*enc3).base.ctx = ctx;
    (*enc3).base.inst = inst;
    (*enc3).base.funcs = &raw const dcn60_hpo_frl_link_encoder_funcs;
    (*enc3).regs = hpo_le_regs;
    (*enc3).hpo_le_shift = hpo_le_shift;
    (*enc3).hpo_le_mask = hpo_le_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
