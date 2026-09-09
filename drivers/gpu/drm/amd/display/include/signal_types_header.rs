/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

/* Minimum pixel clock, in KHz. For TMDS signal is 25.00 MHz */
pub const TMDS_MIN_PIXEL_CLOCK: i32 = 25000;
/* Maximum pixel clock, in KHz. For TMDS signal is 165.00 MHz */
pub const TMDS_MAX_PIXEL_CLOCK: i32 = 165000;
/* Maximum pixel clock, in KHz. For HDMI2 TMDS signal is 600 MHz */
pub const HDMI2_TMDS_MAX_PIXEL_CLOCK: i32 = 600000;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum signal_type {
    SIGNAL_TYPE_NONE = 0,
    SIGNAL_TYPE_DVI_SINGLE_LINK = 1 << 0,
    SIGNAL_TYPE_DVI_DUAL_LINK = 1 << 1,
    SIGNAL_TYPE_HDMI_TYPE_A = 1 << 2,
    SIGNAL_TYPE_LVDS = 1 << 3,
    SIGNAL_TYPE_RGB = 1 << 4,
    SIGNAL_TYPE_DISPLAY_PORT = 1 << 5,
    SIGNAL_TYPE_DISPLAY_PORT_MST = 1 << 6,
    SIGNAL_TYPE_EDP = 1 << 7,
    SIGNAL_TYPE_HDMI_FRL = 1 << 8,
    SIGNAL_TYPE_VIRTUAL = 1 << 9,
}

#[inline]
pub fn signal_type_to_string(type_: i32) -> *const core::ffi::c_char {
    match type_ {
        x if x == signal_type::SIGNAL_TYPE_NONE as i32 => b"No signal\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_DVI_SINGLE_LINK as i32 => b"DVI: Single Link\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_DVI_DUAL_LINK as i32 => b"DVI: Dual Link\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_HDMI_TYPE_A as i32 => b"HDMI: TYPE A\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_LVDS as i32 => b"LVDS\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_RGB as i32 => b"RGB\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_DISPLAY_PORT as i32 => b"Display Port\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_DISPLAY_PORT_MST as i32 => b"Display Port: MST\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_EDP as i32 => b"Embedded Display Port\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_HDMI_FRL as i32 => b"HDMI: FRL\0".as_ptr() as *const core::ffi::c_char,
        x if x == signal_type::SIGNAL_TYPE_VIRTUAL as i32 => b"Virtual\0".as_ptr() as *const core::ffi::c_char,
        _ => b"Unknown\0".as_ptr() as *const core::ffi::c_char,
    }
}

#[inline]
pub fn dc_is_hdmi_tmds_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_HDMI_TYPE_A }
#[inline]
pub fn dc_is_hdmi_frl_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_HDMI_FRL }
#[inline]
pub fn dc_is_hdmi_signal(signal: signal_type) -> bool { dc_is_hdmi_tmds_signal(signal) || dc_is_hdmi_frl_signal(signal) }
#[inline]
pub fn dc_is_dp_sst_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_DISPLAY_PORT || signal == signal_type::SIGNAL_TYPE_EDP }
#[inline]
pub fn dc_is_dp_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_DISPLAY_PORT || signal == signal_type::SIGNAL_TYPE_EDP || signal == signal_type::SIGNAL_TYPE_DISPLAY_PORT_MST }
#[inline]
pub fn dc_is_embedded_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_EDP || signal == signal_type::SIGNAL_TYPE_LVDS }
#[inline]
pub fn dc_is_lvds_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_LVDS }
#[inline]
pub fn dc_is_dvi_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_DVI_SINGLE_LINK || signal == signal_type::SIGNAL_TYPE_DVI_DUAL_LINK }

/**
 * dc_is_rgb_signal() - Whether the signal is analog RGB.
 *
 * Returns whether the given signal type is an analog RGB signal
 * that is used with a DAC on VGA or DVI-I connectors.
 * Not to be confused with other uses of "RGB", such as RGB color space.
 */
#[inline]
pub fn dc_is_rgb_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_RGB }

#[inline]
pub fn dc_is_tmds_signal(signal: signal_type) -> bool {
    signal == signal_type::SIGNAL_TYPE_DVI_SINGLE_LINK || signal == signal_type::SIGNAL_TYPE_DVI_DUAL_LINK || signal == signal_type::SIGNAL_TYPE_HDMI_TYPE_A
}
#[inline]
pub fn dc_is_dvi_single_link_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_DVI_SINGLE_LINK }
#[inline]
pub fn dc_is_dual_link_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_DVI_DUAL_LINK }
#[inline]
pub fn dc_is_audio_capable_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_DISPLAY_PORT || signal == signal_type::SIGNAL_TYPE_DISPLAY_PORT_MST || dc_is_hdmi_signal(signal) }
#[inline]
pub fn dc_is_virtual_signal(signal: signal_type) -> bool { signal == signal_type::SIGNAL_TYPE_VIRTUAL }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
