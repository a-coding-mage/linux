/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

#[repr(C)]
pub enum phyd32clk_clock_source { PHYD32CLKA, PHYD32CLKB, PHYD32CLKC, PHYD32CLKD, PHYD32CLKE, PHYD32CLKF, PHYD32CLKG }

#[repr(C)]
pub enum physymclk_clock_source {
    PHYSYMCLK_FORCE_SRC_SYMCLK,    // Select symclk as source of clock which is output to PHY through DCIO.
    PHYSYMCLK_FORCE_SRC_PHYD18CLK, // Select phyd18clk as the source of clock which is output to PHY through DCIO.
    PHYSYMCLK_FORCE_SRC_PHYD32CLK, // Select phyd32clk as the source of clock which is output to PHY through DCIO.
}

#[repr(C)]
pub enum streamclk_source {
    REFCLK,  // Selects REFCLK as source for hdmistreamclk.
    DTBCLK0, // Selects DTBCLK0 as source for hdmistreamclk.
    DPREFCLK, // Selects DPREFCLK as source for hdmistreamclk
}

#[repr(C)]
pub enum dentist_dispclk_change_mode { DISPCLK_CHANGE_MODE_IMMEDIATE, DISPCLK_CHANGE_MODE_RAMPING }

#[repr(C)]
pub struct dp_dto_params { pub otg_inst: i32, pub signal: signal_type, pub clk_src: streamclk_source, pub pixclk_hz: u64, pub refclk_hz: u64 }

#[repr(C)]
pub enum pixel_rate_div { PIXEL_RATE_DIV_BY_1 = 0, PIXEL_RATE_DIV_BY_2 = 1, PIXEL_RATE_DIV_BY_4 = 3, PIXEL_RATE_DIV_NA = 0xF }

#[repr(C)]
pub struct dcn_dccg_reg_state {
    pub dc_mem_global_pwr_req_cntl: u32, pub dccg_audio_dtbclk_dto_modulo: u32, pub dccg_audio_dtbclk_dto_phase: u32,
    pub dccg_audio_dto_source: u32, pub dccg_audio_dto0_module: u32, pub dccg_audio_dto0_phase: u32,
    pub dccg_audio_dto1_module: u32, pub dccg_audio_dto1_phase: u32, pub dccg_cac_status: u32, pub dccg_cac_status2: u32,
    pub dccg_disp_cntl_reg: u32, pub dccg_ds_cntl: u32, pub dccg_ds_dto_incr: u32, pub dccg_ds_dto_modulo: u32, pub dccg_ds_hw_cal_interval: u32,
    pub dccg_gate_disable_cntl: u32, pub dccg_gate_disable_cntl2: u32, pub dccg_gate_disable_cntl3: u32, pub dccg_gate_disable_cntl4: u32, pub dccg_gate_disable_cntl5: u32, pub dccg_gate_disable_cntl6: u32,
    pub dccg_global_fgcg_rep_cntl: u32, pub dccg_gtc_cntl: u32, pub dccg_gtc_current: u32, pub dccg_gtc_dto_incr: u32, pub dccg_gtc_dto_modulo: u32,
    pub dccg_perfmon_cntl: u32, pub dccg_perfmon_cntl2: u32, pub dccg_soft_reset: u32, pub dccg_test_clk_sel: u32, pub dccg_vsync_cnt_ctrl: u32, pub dccg_vsync_cnt_int_ctrl: u32,
    pub dccg_vsync_otg0_latch_value: u32, pub dccg_vsync_otg1_latch_value: u32, pub dccg_vsync_otg2_latch_value: u32, pub dccg_vsync_otg3_latch_value: u32, pub dccg_vsync_otg4_latch_value: u32, pub dccg_vsync_otg5_latch_value: u32,
    pub dispclk_cgtt_blk_ctrl_reg: u32, pub dispclk_freq_change_cntl: u32, pub dp_dto_dbuf_en: u32,
    pub dp_dto0_modulo: u32, pub dp_dto0_phase: u32, pub dp_dto1_modulo: u32, pub dp_dto1_phase: u32, pub dp_dto2_modulo: u32, pub dp_dto2_phase: u32, pub dp_dto3_modulo: u32, pub dp_dto3_phase: u32,
    pub dpiaclk_540m_dto_modulo: u32, pub dpiaclk_540m_dto_phase: u32, pub dpiaclk_810m_dto_modulo: u32, pub dpiaclk_810m_dto_phase: u32, pub dpiaclk_dto_cntl: u32, pub dpiasymclk_cntl: u32,
    pub dppclk_cgtt_blk_ctrl_reg: u32, pub dppclk_ctrl: u32, pub dppclk_dto_ctrl: u32, pub dppclk0_dto_param: u32, pub dppclk1_dto_param: u32, pub dppclk2_dto_param: u32, pub dppclk3_dto_param: u32,
    pub dprefclk_cgtt_blk_ctrl_reg: u32, pub dprefclk_cntl: u32, pub dpstreamclk_cntl: u32, pub dscclk_dto_ctrl: u32, pub dscclk0_dto_param: u32, pub dscclk1_dto_param: u32, pub dscclk2_dto_param: u32, pub dscclk3_dto_param: u32,
    pub dtbclk_dto_dbuf_en: u32, pub dtbclk_dto0_modulo: u32, pub dtbclk_dto0_phase: u32, pub dtbclk_dto1_modulo: u32, pub dtbclk_dto1_phase: u32, pub dtbclk_dto2_modulo: u32, pub dtbclk_dto2_phase: u32, pub dtbclk_dto3_modulo: u32, pub dtbclk_dto3_phase: u32, pub dtbclk_p_cntl: u32,
    pub force_symclk_disable: u32, pub hdmicharclk0_clock_cntl: u32, pub hdmistreamclk_cntl: u32, pub hdmistreamclk0_dto_param: u32, pub microsecond_time_base_div: u32, pub millisecond_time_base_div: u32, pub otg_pixel_rate_div: u32,
    pub otg0_phypll_pixel_rate_cntl: u32, pub otg0_pixel_rate_cntl: u32, pub otg1_phypll_pixel_rate_cntl: u32, pub otg1_pixel_rate_cntl: u32, pub otg2_phypll_pixel_rate_cntl: u32, pub otg2_pixel_rate_cntl: u32, pub otg3_phypll_pixel_rate_cntl: u32, pub otg3_pixel_rate_cntl: u32,
    pub phyasymclk_clock_cntl: u32, pub phybsymclk_clock_cntl: u32, pub phycsymclk_clock_cntl: u32, pub phydsymclk_clock_cntl: u32, pub phyesymclk_clock_cntl: u32,
    pub phyplla_pixclk_resync_cntl: u32, pub phypllb_pixclk_resync_cntl: u32, pub phypllc_pixclk_resync_cntl: u32, pub phyplld_pixclk_resync_cntl: u32, pub phyplle_pixclk_resync_cntl: u32,
    pub refclk_cgtt_blk_ctrl_reg: u32, pub socclk_cgtt_blk_ctrl_reg: u32, pub symclk_cgtt_blk_ctrl_reg: u32, pub symclk_psp_cntl: u32, pub symclk32_le_cntl: u32, pub symclk32_se_cntl: u32,
    pub symclka_clock_enable: u32, pub symclkb_clock_enable: u32, pub symclkc_clock_enable: u32, pub symclkd_clock_enable: u32, pub symclke_clock_enable: u32,
}

#[repr(C)]
pub struct dccg { pub ctx: *mut dc_context, pub funcs: *const dccg_funcs, pub pipe_dppclk_khz: [i32; MAX_PIPES], pub ref_dppclk: i32, pub dpp_clock_gated: [bool; MAX_PIPES] }

#[repr(C)]
pub struct dtbclk_dto_params { pub timing: *const dc_crtc_timing, pub otg_inst: i32, pub pixclk_khz: i32, pub req_audio_dtbclk_khz: i32, pub num_odm_segments: i32, pub ref_dtbclk_khz: i32, pub is_hdmi: bool }

#[repr(C)]
pub struct dccg_funcs {
    pub update_dpp_dto: Option<unsafe extern "C" fn(*mut dccg, i32, i32)>,
    pub get_dccg_ref_freq: Option<unsafe extern "C" fn(*mut dccg, u32, *mut u32)>,
    pub set_fifo_errdet_ovr_en: Option<unsafe extern "C" fn(*mut dccg, bool)>, pub otg_add_pixel: Option<unsafe extern "C" fn(*mut dccg, u32)>, pub otg_drop_pixel: Option<unsafe extern "C" fn(*mut dccg, u32)>, pub dccg_init: Option<unsafe extern "C" fn(*mut dccg)>, pub refclk_setup: Option<unsafe extern "C" fn(*mut dccg)>, pub allow_clock_gating: Option<unsafe extern "C" fn(*mut dccg, bool)>, pub enable_memory_low_power: Option<unsafe extern "C" fn(*mut dccg, bool)>, pub is_s0i3_golden_init_wa_done: Option<unsafe extern "C" fn(*mut dccg) -> bool>,
    pub enable_hdmicharclk: Option<unsafe extern "C" fn(*mut dccg, i32, i32)>, pub disable_hdmicharclk: Option<unsafe extern "C" fn(*mut dccg, i32)>, pub set_hdmistreamclk: Option<unsafe extern "C" fn(*mut dccg, streamclk_source, u32)>, pub set_hdmistreamclk_root_clock_gating: Option<unsafe extern "C" fn(*mut dccg, bool)>, pub set_dpstreamclk_root_clock_gating: Option<unsafe extern "C" fn(*mut dccg, i32, bool)>, pub set_dpstreamclk: Option<unsafe extern "C" fn(*mut dccg, streamclk_source, i32, i32)>,
    pub enable_symclk32_se: Option<unsafe extern "C" fn(*mut dccg, i32, phyd32clk_clock_source)>, pub disable_symclk32_se: Option<unsafe extern "C" fn(*mut dccg, i32)>, pub enable_symclk32_le: Option<unsafe extern "C" fn(*mut dccg, i32, phyd32clk_clock_source)>, pub disable_symclk32_le: Option<unsafe extern "C" fn(*mut dccg, i32)>, pub set_symclk32_le_root_clock_gating: Option<unsafe extern "C" fn(*mut dccg, i32, bool)>, pub set_physymclk: Option<unsafe extern "C" fn(*mut dccg, i32, physymclk_clock_source, bool)>, pub set_physymclk_root_clock_gating: Option<unsafe extern "C" fn(*mut dccg, i32, bool)>, pub set_dtbclk_dto: Option<unsafe extern "C" fn(*mut dccg, *const dtbclk_dto_params)>, pub set_audio_dtbclk_dto: Option<unsafe extern "C" fn(*mut dccg, *const dtbclk_dto_params)>, pub set_dispclk_change_mode: Option<unsafe extern "C" fn(*mut dccg, dentist_dispclk_change_mode)>,
    pub disable_dsc: Option<unsafe extern "C" fn(*mut dccg, i32)>, pub enable_dsc: Option<unsafe extern "C" fn(*mut dccg, i32)>, pub set_pixel_rate_div: Option<unsafe extern "C" fn(*mut dccg, u32, pixel_rate_div, pixel_rate_div)>, pub get_pixel_rate_div: Option<unsafe extern "C" fn(*mut dccg, u32, *mut u32, *mut u32)>, pub set_valid_pixel_rate: Option<unsafe extern "C" fn(*mut dccg, i32, i32, i32)>, pub trigger_dio_fifo_resync: Option<unsafe extern "C" fn(*mut dccg)>, pub dpp_root_clock_control: Option<unsafe extern "C" fn(*mut dccg, u32, bool)>, pub enable_symclk_se: Option<unsafe extern "C" fn(*mut dccg, u32, u32)>, pub disable_symclk_se: Option<unsafe extern "C" fn(*mut dccg, u32, u32)>,
    pub set_dp_dto: Option<unsafe extern "C" fn(*mut dccg, *const dp_dto_params)>, pub set_dtbclk_p_src: Option<unsafe extern "C" fn(*mut dccg, streamclk_source, u32)>, pub set_dto_dscclk: Option<unsafe extern "C" fn(*mut dccg, u32, u32)>, pub set_ref_dscclk: Option<unsafe extern "C" fn(*mut dccg, u32)>, pub dccg_root_gate_disable_control: Option<unsafe extern "C" fn(*mut dccg, u32, u32)>, pub dccg_read_reg_state: Option<unsafe extern "C" fn(*mut dccg, *mut dcn_dccg_reg_state)>, pub dccg_enable_global_fgcg: Option<unsafe extern "C" fn(*mut dccg, bool)>, pub dccg_get_global_fgcg_status: Option<unsafe extern "C" fn(*mut dccg) -> bool>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
