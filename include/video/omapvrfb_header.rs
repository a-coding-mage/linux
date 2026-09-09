/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VRFB Rotation Engine
 *
 * Copyright (C) 2009 Nokia Corporation
 * Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
 */

pub const OMAP_VRFB_LINE_LEN: u32 = 2048;

#[repr(C)]
pub struct vrfb {
    pub context: u8,
    pub vaddr: [*mut core::ffi::c_void; 4],
    pub paddr: [usize; 4],
    pub xres: u16,
    pub yres: u16,
    pub xoffset: u16,
    pub yoffset: u16,
    pub bytespp: u8,
    pub yuv_mode: bool,
}

#[cfg(feature = "CONFIG_OMAP2_VRFB")]
extern "C" {
    pub fn omap_vrfb_supported() -> bool;
    pub fn omap_vrfb_request_ctx(vrfb: *mut vrfb) -> i32;
    pub fn omap_vrfb_release_ctx(vrfb: *mut vrfb);
    pub fn omap_vrfb_adjust_size(width: *mut u16, height: *mut u16, bytespp: u8);
    pub fn omap_vrfb_min_phys_size(width: u16, height: u16, bytespp: u8) -> u32;
    pub fn omap_vrfb_max_height(phys_size: u32, width: u16, bytespp: u8) -> u16;
    pub fn omap_vrfb_setup(
        vrfb: *mut vrfb,
        paddr: usize,
        width: u16,
        height: u16,
        bytespp: u32,
        yuv_mode: bool,
    );
    pub fn omap_vrfb_map_angle(vrfb: *mut vrfb, height: u16, rot: u8) -> i32;
    pub fn omap_vrfb_restore_context();
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_supported() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_request_ctx(_vrfb: *mut vrfb) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_release_ctx(_vrfb: *mut vrfb) {}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_adjust_size(_width: *mut u16, _height: *mut u16, _bytespp: u8) {}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_min_phys_size(_width: u16, _height: u16, _bytespp: u8) -> u32 {
    0
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_max_height(_phys_size: u32, _width: u16, _bytespp: u8) -> u16 {
    0
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_setup(
    _vrfb: *mut vrfb,
    _paddr: usize,
    _width: u16,
    _height: u16,
    _bytespp: u32,
    _yuv_mode: bool,
) {
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_map_angle(_vrfb: *mut vrfb, _height: u16, _rot: u8) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_OMAP2_VRFB"))]
#[inline]
pub fn omap_vrfb_restore_context() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
