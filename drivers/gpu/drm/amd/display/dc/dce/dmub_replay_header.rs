// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding C translation units:
// dc_types.h, dmub_cmd.h

#[repr(C)]
pub struct dmub_replay {
    pub ctx: *mut dc_context,
    pub funcs: *const dmub_replay_funcs,
}

#[repr(C)]
pub struct dmub_replay_funcs {
    pub replay_get_state:
        Option<unsafe extern "C" fn(dmub: *mut dmub_replay, state: *mut replay_state, panel_inst: u8)>,
    pub replay_enable:
        Option<unsafe extern "C" fn(dmub: *mut dmub_replay, enable: bool, wait: bool, panel_inst: u8)>,
    pub replay_copy_settings: Option<unsafe extern "C" fn(
        dmub: *mut dmub_replay,
        link: *mut dc_link,
        replay_context: *mut replay_context,
        panel_inst: u8,
    ) -> bool>,
    pub replay_set_power_opt:
        Option<unsafe extern "C" fn(dmub: *mut dmub_replay, power_opt: u32, panel_inst: u8)>,
    pub replay_send_cmd: Option<unsafe extern "C" fn(
        dmub: *mut dmub_replay,
        msg: replay_FW_Message_type,
        cmd_element: *mut dmub_replay_cmd_set,
    )>,
    pub replay_set_coasting_vtotal: Option<unsafe extern "C" fn(
        dmub: *mut dmub_replay,
        coasting_vtotal: u32,
        panel_inst: u8,
        frame_skip_number: u16,
    )>,
    pub replay_residency: Option<unsafe extern "C" fn(
        dmub: *mut dmub_replay,
        panel_inst: u8,
        residency: *mut u32,
        is_start: bool,
        mode: pr_residency_mode,
    )>,
    pub replay_set_power_opt_and_coasting_vtotal: Option<unsafe extern "C" fn(
        dmub: *mut dmub_replay,
        power_opt: u32,
        panel_inst: u8,
        coasting_vtotal: u32,
        frame_skip_number: u16,
    )>,
}

extern "C" {
    pub fn dmub_replay_create(ctx: *mut dc_context) -> *mut dmub_replay;
    pub fn dmub_replay_destroy(dmub: *mut *mut dmub_replay);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
