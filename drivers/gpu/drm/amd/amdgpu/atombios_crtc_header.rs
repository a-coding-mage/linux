/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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
 */

// External C types and functions are supplied by other translation units.
extern "C" {
    pub fn amdgpu_atombios_crtc_overscan_setup(
        crtc: *mut drm_crtc,
        mode: *mut drm_display_mode,
        adjusted_mode: *mut drm_display_mode,
    );
    pub fn amdgpu_atombios_crtc_scaler_setup(crtc: *mut drm_crtc);
    pub fn amdgpu_atombios_crtc_lock(crtc: *mut drm_crtc, lock: ::core::ffi::c_int);
    pub fn amdgpu_atombios_crtc_enable(crtc: *mut drm_crtc, state: ::core::ffi::c_int);
    pub fn amdgpu_atombios_crtc_blank(crtc: *mut drm_crtc, state: ::core::ffi::c_int);
    pub fn amdgpu_atombios_crtc_powergate(crtc: *mut drm_crtc, state: ::core::ffi::c_int);
    pub fn amdgpu_atombios_crtc_powergate_init(adev: *mut amdgpu_device);
    pub fn amdgpu_atombios_crtc_set_dtd_timing(
        crtc: *mut drm_crtc,
        mode: *mut drm_display_mode,
    );
    pub fn amdgpu_atombios_crtc_set_disp_eng_pll(adev: *mut amdgpu_device, dispclk: u32);
    pub fn amdgpu_atombios_crtc_set_dce_clock(
        adev: *mut amdgpu_device,
        freq: u32,
        clk_type: u8,
        clk_src: u8,
    ) -> u32;
    pub fn amdgpu_atombios_crtc_program_pll(
        crtc: *mut drm_crtc,
        crtc_id: u32,
        pll_id: ::core::ffi::c_int,
        encoder_mode: u32,
        encoder_id: u32,
        clock: u32,
        ref_div: u32,
        fb_div: u32,
        frac_fb_div: u32,
        post_div: u32,
        bpc: ::core::ffi::c_int,
        ss_enabled: bool,
        ss: *mut amdgpu_atom_ss,
    );
    pub fn amdgpu_atombios_crtc_prepare_pll(
        crtc: *mut drm_crtc,
        mode: *mut drm_display_mode,
    ) -> ::core::ffi::c_int;
    pub fn amdgpu_atombios_crtc_set_pll(
        crtc: *mut drm_crtc,
        mode: *mut drm_display_mode,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
