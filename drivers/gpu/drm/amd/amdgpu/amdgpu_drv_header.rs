/* amdgpu_drv.h -- Private header for amdgpu driver -*- linux-c -*-
 *
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Fremont, California.
 * All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

/* General customization: */

pub const DRIVER_AUTHOR: &str = "AMD linux driver team";

pub const DRIVER_NAME: &str = "amdgpu";
pub const DRIVER_DESC: &str = "AMD GPU";

unsafe extern "C" {
    pub static amdgpu_partition_driver: drm_driver;

    pub fn amdgpu_drm_ioctl(
        filp: *mut file,
        cmd: core::ffi::c_uint,
        arg: core::ffi::c_ulong,
    ) -> core::ffi::c_long;

    pub fn amdgpu_kms_compat_ioctl(
        filp: *mut file,
        cmd: core::ffi::c_uint,
        arg: core::ffi::c_ulong,
    ) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
