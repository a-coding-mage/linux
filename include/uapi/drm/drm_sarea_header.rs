/* SPDX-License-Identifier: MIT */
/**
 * \file drm_sarea.h
 * \brief SAREA definitions
 *
 * \author Michel Dänzer <michel@daenzer.net>
 */

/*
 * Copyright 2002 Tungsten Graphics, Inc., Cedar Park, Texas.
 * All Rights Reserved.
 */

/* Translated from drm_sarea.h. The C include and linkage/header guards are
 * intentionally omitted; drm_hw_lock and drm_context_t are supplied by the
 * surrounding DRM bindings. */

/* SAREA area needs to be at least a page. */
#[cfg(target_arch = "alpha")]
pub const SAREA_MAX: u32 = 0x2000u32;
#[cfg(target_arch = "mips")]
pub const SAREA_MAX: u32 = 0x4000u32;
#[cfg(target_arch = "ia64")]
pub const SAREA_MAX: u32 = 0x10000u32; /* 64kB */
#[cfg(not(any(
    target_arch = "alpha",
    target_arch = "mips",
    target_arch = "ia64"
)))]
pub const SAREA_MAX: u32 = 0x2000u32; /* Intel 830M driver needs at least 8k SAREA */

/** Maximum number of drawables in the SAREA */
pub const SAREA_MAX_DRAWABLES: usize = 256;

pub const SAREA_DRAWABLE_CLAIMED_ENTRY: u32 = 0x80000000u32;

/** SAREA drawable */
#[repr(C)]
pub struct drm_sarea_drawable {
    pub stamp: u32,
    pub flags: u32,
}

/** SAREA frame */
#[repr(C)]
pub struct drm_sarea_frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub fullscreen: u32,
}

/** SAREA */
#[repr(C)]
pub struct drm_sarea {
    /** first thing is always the DRM locking structure */
    pub lock: drm_hw_lock,
    /** \todo Use readers/writer lock for drm_sarea::drawable_lock */
    pub drawable_lock: drm_hw_lock,
    pub drawableTable: [drm_sarea_drawable; SAREA_MAX_DRAWABLES], /* drawables */
    pub frame: drm_sarea_frame, /* frame */
    pub dummy_context: drm_context_t,
}

/* The C header exposes these typedefs only outside the kernel build. */
#[cfg(not(feature = "kernel"))]
pub type drm_sarea_drawable_t = drm_sarea_drawable;
#[cfg(not(feature = "kernel"))]
pub type drm_sarea_frame_t = drm_sarea_frame;
#[cfg(not(feature = "kernel"))]
pub type drm_sarea_t = drm_sarea;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
