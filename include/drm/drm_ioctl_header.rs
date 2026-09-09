/*
 * Internal Header for the Direct Rendering Manager
 *
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * Copyright (c) 2009-2010, Code Aurora Forum.
 * All rights reserved.
 *
 * Author: Rickard E. (Rik) Faith <faith@valinux.com>
 * Author: Gareth Hughes <gareth@valinux.com>
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
 * VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/bitops.h, and asm/ioctl.h.

pub enum drm_device {}
pub enum drm_file {}
pub enum file {}

/**
 * drm_ioctl_t - DRM ioctl function type.
 * @dev: DRM device inode
 * @data: private pointer of the ioctl call
 * @file_priv: DRM file this ioctl was made on
 *
 * This is the DRM ioctl typedef. Note that drm_ioctl() has alrady copied @data
 * into kernel-space, and will also copy it back, depending upon the read/write
 * settings in the ioctl command code.
 */
pub type drm_ioctl_t = unsafe extern "C" fn(
    dev: *mut drm_device,
    data: *mut core::ffi::c_void,
    file_priv: *mut drm_file,
) -> core::ffi::c_int;

/** Compatibility DRM ioctl function type. */
pub type drm_ioctl_compat_t = unsafe extern "C" fn(
    filp: *mut file,
    cmd: core::ffi::c_uint,
    arg: core::ffi::c_ulong,
) -> core::ffi::c_int;

// _IOC_NR and _IOC_TYPE are supplied by asm/ioctl.h.
#[inline]
pub const fn DRM_IOCTL_NR(n: core::ffi::c_uint) -> core::ffi::c_uint { _IOC_NR(n) }
#[inline]
pub const fn DRM_IOCTL_TYPE(n: core::ffi::c_uint) -> core::ffi::c_uint { _IOC_TYPE(n) }
pub const DRM_MAJOR: core::ffi::c_uint = 226;

/** DRM ioctl flags. */
#[repr(i32)]
pub enum drm_ioctl_flags {
    DRM_AUTH = 1 << 0,
    DRM_MASTER = 1 << 1,
    DRM_ROOT_ONLY = 1 << 2,
    DRM_RENDER_ALLOW = 1 << 5,
}

/** DRM driver ioctl entry. */
#[repr(C)]
pub struct drm_ioctl_desc {
    pub cmd: core::ffi::c_uint,
    pub flags: drm_ioctl_flags,
    pub func: Option<drm_ioctl_t>,
    pub name: *const core::ffi::c_char,
}

// Small helper macro corresponding to DRM_IOCTL_DEF_DRV(). Rust macro_rules
// cannot paste DRM_IOCTL_ with an identifier, so the command expression is
// supplied as the fourth macro argument; DRM_COMMAND_BASE indexing is retained
// by the caller when constructing an ioctl table.
#[macro_export]
macro_rules! DRM_IOCTL_DEF_DRV {
    ($ioctl:ident, $func:expr, $flags:expr, $command:expr) => {
        drm_ioctl_desc {
            cmd: DRM_IOCTL_NR($command),
            func: Some($func),
            flags: $flags,
            name: concat!(stringify!($ioctl), "\0").as_ptr() as *const core::ffi::c_char,
        }
    };
}

extern "C" {
    pub fn drm_ioctl(filp: *mut file, cmd: core::ffi::c_uint, arg: core::ffi::c_ulong) -> core::ffi::c_long;
    pub fn drm_ioctl_kernel(filp: *mut file, func: drm_ioctl_t, data: *mut core::ffi::c_void, flags: u32) -> core::ffi::c_long;

    // CONFIG_COMPAT selects the declaration below; otherwise C defines this
    // symbol as NULL for unconditional .compat_ioctl assignment.
    #[cfg(CONFIG_COMPAT)]
    pub fn drm_compat_ioctl(filp: *mut file, cmd: core::ffi::c_uint, arg: core::ffi::c_ulong) -> core::ffi::c_long;

    pub fn drm_ioctl_flags(nr: core::ffi::c_uint, flags: *mut core::ffi::c_uint) -> bool;

    pub fn drm_noop(dev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> core::ffi::c_int;
    pub fn drm_invalid_op(dev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_COMPAT))]
pub const drm_compat_ioctl: Option<unsafe extern "C" fn(*mut file, core::ffi::c_uint, core::ffi::c_ulong) -> core::ffi::c_long> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
