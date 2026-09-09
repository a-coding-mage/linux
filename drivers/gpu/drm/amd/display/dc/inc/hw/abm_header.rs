/* Copyright 2012-15 Advanced Micro Devices, Inc.
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

use core::ffi::c_char;

// External types supplied by the corresponding dependencies.
#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct abm_save_restore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct abm {
    pub ctx: *mut dc_context,
    pub funcs: *const abm_funcs,
    pub dmcu_is_running: bool,
}

#[repr(C)]
pub struct abm_funcs {
    pub abm_init: Option<unsafe extern "C" fn(abm: *mut abm, back_light: u32, user_level: u32)>,
    pub set_abm_level: Option<unsafe extern "C" fn(abm: *mut abm, abm_level: u32) -> bool>,
    pub set_abm_immediate_disable:
        Option<unsafe extern "C" fn(abm: *mut abm, panel_inst: u32) -> bool>,
    pub set_pipe: Option<unsafe extern "C" fn(abm: *mut abm, controller_id: u32, panel_inst: u32) -> bool>,

    /* backlight_pwm_u16_16 is unsigned 32 bit,
     * 16 bit integer + 16 fractional, where 1.0 is max backlight value.
     */
    pub set_backlight_level_pwm: Option<unsafe extern "C" fn(
        abm: *mut abm,
        backlight_pwm_u16_16: u32,
        frame_ramp: u32,
        controller_id: u32,
        panel_inst: u32,
    ) -> bool>,

    pub get_current_backlight: Option<unsafe extern "C" fn(abm: *mut abm) -> u32>,
    pub get_target_backlight: Option<unsafe extern "C" fn(abm: *mut abm) -> u32>,
    pub init_abm_config: Option<unsafe extern "C" fn(
        abm: *mut abm,
        src: *const c_char,
        bytes: u32,
        inst: u32,
    ) -> bool>,
    pub set_abm_pause: Option<unsafe extern "C" fn(
        abm: *mut abm,
        pause: bool,
        panel_inst: u32,
        otg_inst: u32,
    ) -> bool>,
    pub save_restore: Option<unsafe extern "C" fn(
        abm: *mut abm,
        panel_inst: u32,
        p_data: *mut abm_save_restore,
    ) -> bool>,
    pub set_pipe_ex: Option<unsafe extern "C" fn(
        abm: *mut abm,
        otg_inst: u32,
        option: u32,
        panel_inst: u32,
        pwrseq_inst: u32,
    ) -> bool>,
    pub set_abm_event: Option<unsafe extern "C" fn(
        abm: *mut abm,
        full_screen: u32,
        trans_info: u32,
        hdr_mode: u32,
        scaling_enable: u32,
        scaling_strength_map: u32,
        panel_inst: u32,
    ) -> bool>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
