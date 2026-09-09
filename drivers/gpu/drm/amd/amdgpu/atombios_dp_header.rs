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

// Types are supplied by the surrounding kernel translation.
extern "C" {
    pub fn amdgpu_atombios_dp_aux_init(amdgpu_connector: *mut amdgpu_connector);
    pub fn amdgpu_atombios_dp_get_sinktype(amdgpu_connector: *mut amdgpu_connector) -> u8;
    pub fn amdgpu_atombios_dp_get_dpcd(amdgpu_connector: *mut amdgpu_connector) -> i32;
    pub fn amdgpu_atombios_dp_get_panel_mode(
        encoder: *mut drm_encoder,
        connector: *mut drm_connector,
    ) -> i32;
    pub fn amdgpu_atombios_dp_set_link_config(
        connector: *mut drm_connector,
        mode: *const drm_display_mode,
    );
    pub fn amdgpu_atombios_dp_mode_valid_helper(
        connector: *mut drm_connector,
        mode: *const drm_display_mode,
    ) -> i32;
    pub fn amdgpu_atombios_dp_needs_link_train(
        amdgpu_connector: *mut amdgpu_connector,
    ) -> bool;
    pub fn amdgpu_atombios_dp_set_rx_power_state(
        connector: *mut drm_connector,
        power_state: u8,
    );
    pub fn amdgpu_atombios_dp_link_train(
        encoder: *mut drm_encoder,
        connector: *mut drm_connector,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
