/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// C dependency: amdgpu_dm_crc.h

#[repr(C)]
pub struct dm_irq_params {
    pub last_flip_vblank: u32,
    pub vrr_params: mod_vrr_params,
    pub stream: *mut dc_stream_state,
    pub active_planes: i32,
    pub allow_sr_entry: bool,
    pub freesync_config: mod_freesync_config,

    // Preserves #ifdef CONFIG_DEBUG_FS.
    #[cfg(CONFIG_DEBUG_FS)]
    pub crc_src: amdgpu_dm_pipe_crc_source,
    #[cfg(CONFIG_DEBUG_FS)]
    pub crc_poly_mode: i32, // enum crc_poly_mode from timing_generator.h

    // Preserves #ifdef CONFIG_DRM_AMD_SECURE_DISPLAY.
    #[cfg(all(CONFIG_DEBUG_FS, CONFIG_DRM_AMD_SECURE_DISPLAY))]
    pub window_param: [crc_window_param; MAX_CRC_WINDOW_NUM],
    #[cfg(all(CONFIG_DEBUG_FS, CONFIG_DRM_AMD_SECURE_DISPLAY))]
    // At least one CRC window is activated or not
    pub crc_window_activated: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
