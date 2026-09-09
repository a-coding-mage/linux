/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// Dependency supplied externally: drm/drm_panic.h

#[macro_export]
macro_rules! amdgpu_display_vblank_get_counter {
    ($adev:expr, $crtc:expr) => {
        (*$adev).mode_info.funcs.vblank_get_counter($adev, $crtc)
    };
}
#[macro_export]
macro_rules! amdgpu_display_backlight_set_level {
    ($adev:expr, $e:expr, $l:expr) => {
        (*$adev).mode_info.funcs.backlight_set_level($e, $l)
    };
}
#[macro_export]
macro_rules! amdgpu_display_backlight_get_level {
    ($adev:expr, $e:expr) => {
        (*$adev).mode_info.funcs.backlight_get_level($e)
    };
}
#[macro_export]
macro_rules! amdgpu_display_hpd_sense {
    ($adev:expr, $h:expr) => {
        (*$adev).mode_info.funcs.hpd_sense($adev, $h)
    };
}
#[macro_export]
macro_rules! amdgpu_display_hpd_set_polarity {
    ($adev:expr, $h:expr) => {
        (*$adev).mode_info.funcs.hpd_set_polarity($adev, $h)
    };
}
#[macro_export]
macro_rules! amdgpu_display_hpd_get_gpio_reg {
    ($adev:expr) => {
        (*$adev).mode_info.funcs.hpd_get_gpio_reg($adev)
    };
}
#[macro_export]
macro_rules! amdgpu_display_bandwidth_update {
    ($adev:expr) => {
        (*$adev).mode_info.funcs.bandwidth_update($adev)
    };
}
#[macro_export]
macro_rules! amdgpu_display_page_flip {
    ($adev:expr, $crtc:expr, $base:expr, $async_:expr) => {
        (*$adev).mode_info.funcs.page_flip($adev, $crtc, $base, $async_)
    };
}
#[macro_export]
macro_rules! amdgpu_display_page_flip_get_scanoutpos {
    ($adev:expr, $crtc:expr, $vbl:expr, $pos:expr) => {
        (*$adev).mode_info.funcs.page_flip_get_scanoutpos($adev, $crtc, $vbl, $pos)
    };
}
#[macro_export]
macro_rules! amdgpu_display_add_encoder {
    ($adev:expr, $e:expr, $s:expr, $c:expr) => {
        (*$adev).mode_info.funcs.add_encoder($adev, $e, $s, $c)
    };
}
#[macro_export]
macro_rules! amdgpu_display_add_connector {
    ($adev:expr, $ci:expr, $sd:expr, $ct:expr, $ib:expr, $coi:expr, $h:expr, $r:expr) => {
        (*$adev).mode_info.funcs.add_connector($adev, $ci, $sd, $ct, $ib, $coi, $h, $r)
    };
}

extern "C" {
    pub fn amdgpu_display_hotplug_work_func(work: *mut work_struct);
    pub fn amdgpu_display_update_priority(adev: *mut amdgpu_device);
    pub fn amdgpu_display_supported_domains(adev: *mut amdgpu_device, bo_flags: u64) -> u32;
    pub fn amdgpu_display_user_framebuffer_create(
        dev: *mut drm_device,
        file_priv: *mut drm_file,
        info: *const drm_format_info,
        mode_cmd: *const drm_mode_fb_cmd2,
    ) -> *mut drm_framebuffer;
    pub fn amdgpu_lookup_format_info(format: u32, modifier: u64) -> *const drm_format_info;
    pub fn amdgpu_display_suspend_helper(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_display_resume_helper(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_display_get_scanout_buffer(
        plane: *mut drm_plane,
        sb: *mut drm_scanout_buffer,
    ) -> i32;
}

pub const ABM_SYSFS_CONTROL: i32 = -1;
pub const ABM_LEVEL_OFF: i32 = 0;
pub const ABM_LEVEL_MIN: i32 = 1;
pub const ABM_LEVEL_BIAS_MIN: i32 = 2;
pub const ABM_LEVEL_BIAS_MAX: i32 = 3;
pub const ABM_LEVEL_MAX: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
