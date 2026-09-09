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
 *
 */

unsafe extern "C" {
    pub fn amdgpu_connector_hotplug(connector: *mut drm_connector);
    pub fn amdgpu_connector_get_monitor_bpc(connector: *mut drm_connector) -> ::core::ffi::c_int;
    pub fn amdgpu_connector_encoder_get_dp_bridge_encoder_id(
        connector: *mut drm_connector,
    ) -> u16;
    pub fn amdgpu_connector_is_dp12_capable(connector: *mut drm_connector) -> bool;
    pub fn amdgpu_connector_add(
        adev: *mut amdgpu_device,
        connector_id: u32,
        supported_device: u32,
        connector_type: ::core::ffi::c_int,
        i2c_bus: *mut amdgpu_i2c_bus_rec,
        connector_object_id: u16,
        hpd: *mut amdgpu_hpd,
        router: *mut amdgpu_router,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
