/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 Russell King
 *  With inspiration from the i915 driver
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as
 * published by the Free Software Foundation.
 */

// Dependency: symbols supplied by `drm.h` remain external to this translation.

pub const DRM_ARMADA_GEM_CREATE: u32 = 0x00;
pub const DRM_ARMADA_GEM_MMAP: u32 = 0x02;
pub const DRM_ARMADA_GEM_PWRITE: u32 = 0x03;

#[repr(C)]
pub struct drm_armada_gem_create {
    pub handle: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct drm_armada_gem_mmap {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
    pub size: __u64,
    pub addr: __u64,
}

#[repr(C)]
pub struct drm_armada_gem_pwrite {
    pub ptr: __u64,
    pub handle: __u32,
    pub offset: __u32,
    pub size: __u32,
}

// ARMADA_IOCTL(dir, name, str) expands to:
// DRM_##dir(DRM_COMMAND_BASE + DRM_ARMADA_##name,
//           struct drm_armada_##str)

#[allow(non_upper_case_globals)]
pub const DRM_IOCTL_ARMADA_GEM_CREATE: _ = ARMADA_IOCTL!(IOWR, GEM_CREATE, gem_create);

#[allow(non_upper_case_globals)]
pub const DRM_IOCTL_ARMADA_GEM_MMAP: _ = ARMADA_IOCTL!(IOWR, GEM_MMAP, gem_mmap);

#[allow(non_upper_case_globals)]
pub const DRM_IOCTL_ARMADA_GEM_PWRITE: _ = ARMADA_IOCTL!(IOW, GEM_PWRITE, gem_pwrite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
