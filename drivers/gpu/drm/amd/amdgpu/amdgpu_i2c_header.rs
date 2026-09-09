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

// Opaque types supplied by the corresponding dependencies.
pub enum drm_device {}
pub enum amdgpu_i2c_bus_rec {}
pub enum amdgpu_i2c_chan {}
pub enum amdgpu_device {}
pub enum amdgpu_connector {}

extern "C" {
    pub fn amdgpu_i2c_create(
        dev: *mut drm_device,
        rec: *const amdgpu_i2c_bus_rec,
        name: *const core::ffi::c_char,
    ) -> *mut amdgpu_i2c_chan;

    pub fn amdgpu_i2c_destroy(i2c: *mut amdgpu_i2c_chan);

    pub fn amdgpu_i2c_init(adev: *mut amdgpu_device);

    pub fn amdgpu_i2c_fini(adev: *mut amdgpu_device);

    pub fn amdgpu_i2c_lookup(
        adev: *mut amdgpu_device,
        i2c_bus: *const amdgpu_i2c_bus_rec,
    ) -> *mut amdgpu_i2c_chan;

    pub fn amdgpu_i2c_router_select_ddc_port(connector: *const amdgpu_connector);

    pub fn amdgpu_i2c_router_select_cd_port(connector: *const amdgpu_connector);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
