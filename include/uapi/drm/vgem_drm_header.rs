/*
 * Copyright 2016 Intel Corporation
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * TUNGSTEN GRAPHICS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency: the __u32 type and DRM_IOWR/DRM_IOW/DRM_COMMAND_BASE are
// supplied by the corresponding DRM header translation.

/* Please note that modifications to all structs defined here are
 * subject to backwards-compatibility constraints.
 */
pub const DRM_VGEM_FENCE_ATTACH: u32 = 0x1;
pub const DRM_VGEM_FENCE_SIGNAL: u32 = 0x2;

pub const DRM_IOCTL_VGEM_FENCE_ATTACH: u64 =
    DRM_IOWR(DRM_COMMAND_BASE + DRM_VGEM_FENCE_ATTACH, drm_vgem_fence_attach);
pub const DRM_IOCTL_VGEM_FENCE_SIGNAL: u64 =
    DRM_IOW(DRM_COMMAND_BASE + DRM_VGEM_FENCE_SIGNAL, drm_vgem_fence_signal);

#[repr(C)]
pub struct drm_vgem_fence_attach {
    pub handle: u32,
    pub flags: u32,
    pub out_fence: u32,
    pub pad: u32,
}

pub const VGEM_FENCE_WRITE: u32 = 0x1;

#[repr(C)]
pub struct drm_vgem_fence_signal {
    pub fence: u32,
    pub flags: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
