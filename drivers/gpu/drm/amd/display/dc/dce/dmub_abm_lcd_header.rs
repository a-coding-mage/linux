/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by the surrounding translation unit: abm.h

#[repr(C)]
pub struct abm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct abm_save_restore {
    _private: [u8; 0],
}

extern "C" {
    pub fn dmub_abm_init(abm: *mut abm, backlight: u32, user_level: u32);
    pub fn dmub_abm_set_level(abm: *mut abm, level: u32, panel_mask: u8) -> bool;
    pub fn dmub_abm_get_current_backlight(abm: *mut abm) -> u32;
    pub fn dmub_abm_get_target_backlight(abm: *mut abm) -> u32;
    pub fn dmub_abm_init_config(
        abm: *mut abm,
        src: *const core::ffi::c_char,
        bytes: u32,
        inst: u32,
    );

    pub fn dmub_abm_set_pause(
        abm: *mut abm,
        pause: bool,
        panel_inst: u32,
        stream_inst: u32,
    ) -> bool;
    pub fn dmub_abm_save_restore(
        dc: *mut dc_context,
        panel_inst: u32,
        p_data: *mut abm_save_restore,
    ) -> bool;
    pub fn dmub_abm_set_pipe(
        abm: *mut abm,
        otg_inst: u32,
        option: u32,
        panel_inst: u32,
        pwrseq_inst: u32,
    ) -> bool;
    pub fn dmub_abm_set_backlight_level(
        abm: *mut abm,
        backlight_pwm_u16_16: u32,
        frame_ramp: u32,
        panel_inst: u32,
    ) -> bool;
    pub fn dmub_abm_set_event(
        abm: *mut abm,
        scaling_enable: u32,
        scaling_strength_map: u32,
        panel_inst: u32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
