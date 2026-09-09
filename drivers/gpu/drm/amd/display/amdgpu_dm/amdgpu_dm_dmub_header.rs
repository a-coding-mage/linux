/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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

use std::ffi::c_char;

// Types and callback aliases are supplied by the translated amdgpu dependency.
extern "C" {
    pub fn dm_dmub_aux_setconfig_callback(
        adev: *mut amdgpu_device,
        notify: *mut dmub_notification,
    );
    pub fn dm_dmub_aux_fused_io_callback(
        adev: *mut amdgpu_device,
        notify: *mut dmub_notification,
    );
    pub fn dm_register_dmub_notify_callback(
        adev: *mut amdgpu_device,
        type_: dmub_notification_type,
        callback: dmub_notify_interrupt_callback_t,
        dmub_int_thread_offload: bool,
    ) -> bool;
    pub fn dm_dmub_hw_init(adev: *mut amdgpu_device) -> i32;
    pub fn dm_dmub_hw_resume(adev: *mut amdgpu_device);
    pub fn dm_get_default_ips_mode(adev: *mut amdgpu_device) -> dmub_ips_disable_type;
    pub fn dm_dmub_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn dm_init_microcode(adev: *mut amdgpu_device) -> i32;
}

pub const FIRMWARE_RENOIR_DMUB: *const c_char = b"amdgpu/renoir_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_SIENNA_CICHLID_DMUB: *const c_char = b"amdgpu/sienna_cichlid_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_NAVY_FLOUNDER_DMUB: *const c_char = b"amdgpu/navy_flounder_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_GREEN_SARDINE_DMUB: *const c_char = b"amdgpu/green_sardine_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_VANGOGH_DMUB: *const c_char = b"amdgpu/vangogh_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DIMGREY_CAVEFISH_DMUB: *const c_char = b"amdgpu/dimgrey_cavefish_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_BEIGE_GOBY_DMUB: *const c_char = b"amdgpu/beige_goby_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_YELLOW_CARP_DMUB: *const c_char = b"amdgpu/yellow_carp_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_314_DMUB: *const c_char = b"amdgpu/dcn_3_1_4_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_315_DMUB: *const c_char = b"amdgpu/dcn_3_1_5_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN316_DMUB: *const c_char = b"amdgpu/dcn_3_1_6_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_V3_2_0_DMCUB: *const c_char = b"amdgpu/dcn_3_2_0_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_V3_2_1_DMCUB: *const c_char = b"amdgpu/dcn_3_2_1_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_35_DMUB: *const c_char = b"amdgpu/dcn_3_5_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_351_DMUB: *const c_char = b"amdgpu/dcn_3_5_1_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_36_DMUB: *const c_char = b"amdgpu/dcn_3_6_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_401_DMUB: *const c_char = b"amdgpu/dcn_4_0_1_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_42_DMUB: *const c_char = b"amdgpu/dcn_4_2_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_42B_DMUB: *const c_char = b"amdgpu/dcn_4_2_1_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_DCN_60_DMUB: *const c_char = b"amdgpu/dcn_6_0_0_dmcub.bin\0".as_ptr().cast();
pub const FIRMWARE_RAVEN_DMCU: *const c_char = b"amdgpu/raven_dmcu.bin\0".as_ptr().cast();
pub const FIRMWARE_NAVI12_DMCU: *const c_char = b"amdgpu/navi12_dmcu.bin\0".as_ptr().cast();

// The following declarations are conditional on CONFIG_DRM_AMD_DC_KUNIT_TEST.
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub struct dc_context;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
pub struct dmub_cmd_fused_request;
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
extern "C" {
    pub fn dm_dmub_get_vbios_bounding_box(adev: *mut amdgpu_device) -> *mut core::ffi::c_void;
    pub fn abort_fused_io(ctx: *mut dc_context, request: *const dmub_cmd_fused_request);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
