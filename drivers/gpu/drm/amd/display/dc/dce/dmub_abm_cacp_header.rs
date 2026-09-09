/* Copyright (C) 2022 Advanced Micro Devices, Inc. All rights reserved. */

// Dependency supplied by dmub_cmd.h.

pub unsafe extern "C" {
    pub fn dmub_cacp_init(
        abm: *mut crate::abm,
        src: *const core::ffi::c_char,
        bytes: u32,
        panel_inst: u32,
    );

    pub fn dmub_cacp_set_level(
        abm: *mut crate::abm,
        cacp_level: u32,
        panel_mask: u8,
    ) -> bool;

    pub fn dmub_cacp_set_pipe(
        abm: *mut crate::abm,
        otg_inst: u32,
        pipe_option: u32,
        panel_inst: u32,
        pwrseq_inst: u32,
    ) -> bool;

    pub fn dmub_cacp_set_event(
        abm: *mut crate::abm,
        full_screen: u32,
        trans_info: u32,
        hdr_mode: u32,
        scaling_enable: u32,
        panel_inst: u32,
    ) -> bool;

    pub fn dmub_cacp_set_pause(
        abm: *mut crate::abm,
        pause: bool,
        panel_inst: u32,
        otg_inst: u32,
    ) -> bool;

    pub fn dmub_cacp_set_backlight_level(
        abm: *mut crate::abm,
        backlight_pwm_u16_16: u32,
        frame_ramp: u32,
        panel_inst: u32,
    ) -> bool;

    pub fn dmub_cacp_enable_fractional_pwm(abm: *mut crate::abm, panel_mask: u8);

    pub fn dmub_cacp_get_histogram(
        dc: *mut crate::dc_context,
        panel_inst: u32,
        histogram: *mut u32,
        histogram_type: crate::dmub_abm_histogram_type,
        size: u32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
