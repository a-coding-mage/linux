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

// The declarations below depend on types supplied by the surrounding kernel
// translation unit.
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_encoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_encoder_atom_dig {
    _private: [u8; 0],
}

// C enum drm_connector_status has an int-compatible representation.
pub type drm_connector_status = i32;

unsafe extern "C" {
    pub fn amdgpu_atombios_encoder_get_backlight_level_from_reg(
        adev: *mut amdgpu_device,
    ) -> u8;
    pub fn amdgpu_atombios_encoder_set_backlight_level_to_reg(
        adev: *mut amdgpu_device,
        backlight_level: u8,
    );
    pub fn amdgpu_atombios_encoder_get_backlight_level(
        amdgpu_encoder: *mut amdgpu_encoder,
    ) -> u8;
    pub fn amdgpu_atombios_encoder_set_backlight_level(
        amdgpu_encoder: *mut amdgpu_encoder,
        level: u8,
    );
    pub fn amdgpu_atombios_encoder_init_backlight(
        amdgpu_encoder: *mut amdgpu_encoder,
        drm_connector: *mut drm_connector,
    );
    pub fn amdgpu_atombios_encoder_fini_backlight(
        amdgpu_encoder: *mut amdgpu_encoder,
    );
    pub fn amdgpu_atombios_encoder_is_digital(encoder: *mut drm_encoder) -> bool;
    pub fn amdgpu_atombios_encoder_mode_fixup(
        encoder: *mut drm_encoder,
        mode: *const drm_display_mode,
        adjusted_mode: *mut drm_display_mode,
    ) -> bool;
    pub fn amdgpu_atombios_encoder_get_encoder_mode(encoder: *mut drm_encoder) -> i32;
    pub fn amdgpu_atombios_encoder_setup_dig_encoder(
        encoder: *mut drm_encoder,
        action: i32,
        panel_mode: i32,
    );
    pub fn amdgpu_atombios_encoder_setup_dig_transmitter(
        encoder: *mut drm_encoder,
        action: i32,
        lane_num: u8,
        lane_set: u8,
    );
    pub fn amdgpu_atombios_encoder_set_edp_panel_power(
        connector: *mut drm_connector,
        action: i32,
    ) -> bool;
    pub fn amdgpu_atombios_encoder_dpms(encoder: *mut drm_encoder, mode: i32);
    pub fn amdgpu_atombios_encoder_set_crtc_source(encoder: *mut drm_encoder);
    pub fn amdgpu_atombios_encoder_init_dig(adev: *mut amdgpu_device);
    pub fn amdgpu_atombios_encoder_dac_detect(
        encoder: *mut drm_encoder,
        connector: *mut drm_connector,
    ) -> drm_connector_status;
    pub fn amdgpu_atombios_encoder_dig_detect(
        encoder: *mut drm_encoder,
        connector: *mut drm_connector,
    ) -> drm_connector_status;
    pub fn amdgpu_atombios_encoder_setup_ext_encoder_ddc(encoder: *mut drm_encoder);
    pub fn amdgpu_atombios_encoder_set_bios_scratch_regs(
        connector: *mut drm_connector,
        encoder: *mut drm_encoder,
        connected: bool,
    );
    pub fn amdgpu_atombios_encoder_get_lcd_info(
        encoder: *mut amdgpu_encoder,
    ) -> *mut amdgpu_encoder_atom_dig;
    pub fn amdgpu_atombios_encoder_get_dig_info(
        amdgpu_encoder: *mut amdgpu_encoder,
    ) -> *mut amdgpu_encoder_atom_dig;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
