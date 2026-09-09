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
 *
 */

// Dependency supplied by the surrounding translation unit: dm_services_types.h

/* If HW itself ever powered down it will be 0.
 * fwDmcuInit will write to 1.
 * Driver will only call MCP init if current state is 1,
 * and the MCP command will transition this to 2.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmcu_state {
    DMCU_UNLOADED = 0,
    DMCU_LOADED_UNINITIALIZED = 1,
    DMCU_RUNNING = 2,
}

#[repr(C)]
pub struct dmcu_version {
    pub interface_version: ::core::ffi::c_uint,
    pub abm_version: ::core::ffi::c_uint,
    pub psr_version: ::core::ffi::c_uint,
    pub build_version: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dmcu {
    pub ctx: *mut dc_context,
    pub funcs: *const dmcu_funcs,

    pub dmcu_state: dmcu_state,
    pub dmcu_version: dmcu_version,
    pub cached_wait_loop_number: ::core::ffi::c_uint,
    pub psp_version: u32,
    pub auto_load_dmcu: bool,
}

#[repr(C)]
pub struct dmcu_funcs {
    pub dmcu_init: Option<unsafe extern "C" fn(dmcu: *mut dmcu) -> bool>,
    pub load_iram: Option<unsafe extern "C" fn(
        dmcu: *mut dmcu,
        start_offset: ::core::ffi::c_uint,
        src: *const ::core::ffi::c_char,
        bytes: ::core::ffi::c_uint,
    ) -> bool>,
    pub set_psr_enable: Option<unsafe extern "C" fn(dmcu: *mut dmcu, enable: bool, wait: bool)>,
    pub setup_psr: Option<unsafe extern "C" fn(
        dmcu: *mut dmcu,
        link: *mut dc_link,
        psr_context: *mut psr_context,
    ) -> bool>,
    pub get_psr_state: Option<unsafe extern "C" fn(dmcu: *mut dmcu, dc_psr_state: *mut dc_psr_state)>,
    pub set_psr_wait_loop: Option<unsafe extern "C" fn(dmcu: *mut dmcu, wait_loop_number: ::core::ffi::c_uint)>,
    pub get_psr_wait_loop: Option<unsafe extern "C" fn(dmcu: *mut dmcu, psr_wait_loop_number: *mut ::core::ffi::c_uint)>,
    pub is_dmcu_initialized: Option<unsafe extern "C" fn(dmcu: *mut dmcu) -> bool>,
    pub lock_phy: Option<unsafe extern "C" fn(dmcu: *mut dmcu) -> bool>,
    pub unlock_phy: Option<unsafe extern "C" fn(dmcu: *mut dmcu) -> bool>,
    pub send_edid_cea: Option<unsafe extern "C" fn(
        dmcu: *mut dmcu,
        offset: i32,
        total_length: i32,
        data: *mut u8,
        length: i32,
    ) -> bool>,
    pub recv_amd_vsdb: Option<unsafe extern "C" fn(
        dmcu: *mut dmcu,
        version: *mut i32,
        min_frame_rate: *mut i32,
        max_frame_rate: *mut i32,
    ) -> bool>,
    pub recv_edid_cea_ack: Option<unsafe extern "C" fn(dmcu: *mut dmcu, offset: *mut i32) -> bool>,
    // Preserved from #if defined(CONFIG_DRM_AMD_SECURE_DISPLAY).
    #[cfg(feature = "CONFIG_DRM_AMD_SECURE_DISPLAY")]
    pub forward_crc_window: Option<unsafe extern "C" fn(dmcu: *mut dmcu, rect: *mut rect, mux_mapping: *mut otg_phy_mux)>,
    #[cfg(feature = "CONFIG_DRM_AMD_SECURE_DISPLAY")]
    pub stop_crc_win_update: Option<unsafe extern "C" fn(dmcu: *mut dmcu, mux_mapping: *mut otg_phy_mux)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
