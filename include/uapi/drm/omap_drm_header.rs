/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/uapi/drm/omap_drm.h
 *
 * Copyright (C) 2011 Texas Instruments
 * Author: Rob Clark <rob@ti.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published by
 * the Free Software Foundation.
 */

/* The original header includes "drm.h" and uses its DRM ioctl macros. */

/* Please note that modifications to all structs defined here are
 * subject to backwards-compatibility constraints.
 */

pub const OMAP_PARAM_CHIPSET_ID: u32 = 1; /* ie. 0x3430, 0x4430, etc */

#[repr(C)]
pub struct drm_omap_param {
    pub param: u64, /* in */
    pub value: u64, /* in (set_param), out (get_param) */
}

/* Scanout buffer, consumable by DSS */
pub const OMAP_BO_SCANOUT: u32 = 0x00000001;

/* Buffer CPU caching mode: cached, write-combining or uncached. */
pub const OMAP_BO_CACHED: u32 = 0x00000000;
pub const OMAP_BO_WC: u32 = 0x00000002;
pub const OMAP_BO_UNCACHED: u32 = 0x00000004;
pub const OMAP_BO_CACHE_MASK: u32 = 0x00000006;

/* Use TILER for the buffer. The TILER container unit can be 8, 16 or 32 bits. */
pub const OMAP_BO_TILED_8: u32 = 0x00000100;
pub const OMAP_BO_TILED_16: u32 = 0x00000200;
pub const OMAP_BO_TILED_32: u32 = 0x00000300;
pub const OMAP_BO_TILED_MASK: u32 = 0x00000f00;

#[repr(C)]
#[derive(Copy, Clone)]
pub union omap_gem_size {
    pub bytes: u32, /* (for non-tiled formats) */
    pub tiled: omap_gem_size_tiled, /* (for tiled formats) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_gem_size_tiled {
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
pub struct drm_omap_gem_new {
    pub size: omap_gem_size, /* in */
    pub flags: u32, /* in */
    pub handle: u32, /* out */
    pub __pad: u32,
}

/* mask of operations: */
#[repr(u32)]
pub enum omap_gem_op {
    OMAP_GEM_READ = 0x01,
    OMAP_GEM_WRITE = 0x02,
}

#[repr(C)]
pub struct drm_omap_gem_cpu_prep {
    pub handle: u32, /* buffer handle (in) */
    pub op: u32, /* mask of omap_gem_op (in) */
}

#[repr(C)]
pub struct drm_omap_gem_cpu_fini {
    pub handle: u32, /* buffer handle (in) */
    pub op: u32, /* mask of omap_gem_op (in) */
    /* TODO maybe here we pass down info about what regions are touched
     * by sw so we can be clever about cache ops?  For now a placeholder,
     * set to zero and we just do full buffer flush..
     */
    pub nregions: u32,
    pub __pad: u32,
}

#[repr(C)]
pub struct drm_omap_gem_info {
    pub handle: u32, /* buffer handle (in) */
    pub pad: u32,
    pub offset: u64, /* mmap offset (out) */
    /* note: in case of tiled buffers, the user virtual size can be
     * different from the physical size (ie. how many pages are needed
     * to back the object) which is returned in DRM_IOCTL_GEM_OPEN..
     * This size here is the one that should be used if you want to
     * mmap() the buffer:
     */
    pub size: u32, /* virtual size for mmap'ing (out) */
    pub __pad: u32,
}

pub const DRM_OMAP_GET_PARAM: u32 = 0x00;
pub const DRM_OMAP_SET_PARAM: u32 = 0x01;
pub const DRM_OMAP_GEM_NEW: u32 = 0x03;
pub const DRM_OMAP_GEM_CPU_PREP: u32 = 0x04; /* Deprecated, to be removed */
pub const DRM_OMAP_GEM_CPU_FINI: u32 = 0x05; /* Deprecated, to be removed */
pub const DRM_OMAP_GEM_INFO: u32 = 0x06;
pub const DRM_OMAP_NUM_IOCTLS: u32 = 0x07;

/* DRM_IOW/DRM_IOWR and DRM_COMMAND_BASE are supplied by drm.h. */
pub const DRM_IOCTL_OMAP_GET_PARAM: u64 = crate::DRM_IOWR!(DRM_COMMAND_BASE + DRM_OMAP_GET_PARAM, drm_omap_param);
pub const DRM_IOCTL_OMAP_SET_PARAM: u64 = crate::DRM_IOW!(DRM_COMMAND_BASE + DRM_OMAP_SET_PARAM, drm_omap_param);
pub const DRM_IOCTL_OMAP_GEM_NEW: u64 = crate::DRM_IOWR!(DRM_COMMAND_BASE + DRM_OMAP_GEM_NEW, drm_omap_gem_new);
pub const DRM_IOCTL_OMAP_GEM_CPU_PREP: u64 = crate::DRM_IOW!(DRM_COMMAND_BASE + DRM_OMAP_GEM_CPU_PREP, drm_omap_gem_cpu_prep);
pub const DRM_IOCTL_OMAP_GEM_CPU_FINI: u64 = crate::DRM_IOW!(DRM_COMMAND_BASE + DRM_OMAP_GEM_CPU_FINI, drm_omap_gem_cpu_fini);
pub const DRM_IOCTL_OMAP_GEM_INFO: u64 = crate::DRM_IOWR!(DRM_COMMAND_BASE + DRM_OMAP_GEM_INFO, drm_omap_gem_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
