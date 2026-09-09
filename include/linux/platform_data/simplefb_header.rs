/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * simplefb.h - Simple Framebuffer Device
 *
 * Copyright (C) 2013 David Herrmann <dh.herrmann@gmail.com>
 */

// C dependencies: drm/drm_fourcc.h, linux/fb.h, and linux/types.h.

/* format array, use it to initialize a "struct simplefb_format" array */
macro_rules! SIMPLEFB_FORMATS {
    () => {
        [
            simplefb_format { name: b"r5g6b5\0".as_ptr() as *const i8, bits_per_pixel: 16, red: fb_bitfield { offset: 11, length: 5, msb_right: 0 }, green: fb_bitfield { offset: 5, length: 6, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 5, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 }, fourcc: DRM_FORMAT_RGB565 },
            simplefb_format { name: b"r5g5b5a1\0".as_ptr() as *const i8, bits_per_pixel: 16, red: fb_bitfield { offset: 11, length: 5, msb_right: 0 }, green: fb_bitfield { offset: 6, length: 5, msb_right: 0 }, blue: fb_bitfield { offset: 1, length: 5, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 1, msb_right: 0 }, fourcc: DRM_FORMAT_RGBA5551 },
            simplefb_format { name: b"x1r5g5b5\0".as_ptr() as *const i8, bits_per_pixel: 16, red: fb_bitfield { offset: 10, length: 5, msb_right: 0 }, green: fb_bitfield { offset: 5, length: 5, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 5, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 }, fourcc: DRM_FORMAT_XRGB1555 },
            simplefb_format { name: b"a1r5g5b5\0".as_ptr() as *const i8, bits_per_pixel: 16, red: fb_bitfield { offset: 10, length: 5, msb_right: 0 }, green: fb_bitfield { offset: 5, length: 5, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 5, msb_right: 0 }, transp: fb_bitfield { offset: 15, length: 1, msb_right: 0 }, fourcc: DRM_FORMAT_ARGB1555 },
            simplefb_format { name: b"r8g8b8\0".as_ptr() as *const i8, bits_per_pixel: 24, red: fb_bitfield { offset: 16, length: 8, msb_right: 0 }, green: fb_bitfield { offset: 8, length: 8, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 8, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 }, fourcc: DRM_FORMAT_RGB888 },
            simplefb_format { name: b"x8r8g8b8\0".as_ptr() as *const i8, bits_per_pixel: 32, red: fb_bitfield { offset: 16, length: 8, msb_right: 0 }, green: fb_bitfield { offset: 8, length: 8, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 8, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 }, fourcc: DRM_FORMAT_XRGB8888 },
            simplefb_format { name: b"a8r8g8b8\0".as_ptr() as *const i8, bits_per_pixel: 32, red: fb_bitfield { offset: 16, length: 8, msb_right: 0 }, green: fb_bitfield { offset: 8, length: 8, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 8, msb_right: 0 }, transp: fb_bitfield { offset: 24, length: 8, msb_right: 0 }, fourcc: DRM_FORMAT_ARGB8888 },
            simplefb_format { name: b"x8b8g8r8\0".as_ptr() as *const i8, bits_per_pixel: 32, red: fb_bitfield { offset: 0, length: 8, msb_right: 0 }, green: fb_bitfield { offset: 8, length: 8, msb_right: 0 }, blue: fb_bitfield { offset: 16, length: 8, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 }, fourcc: DRM_FORMAT_XBGR8888 },
            simplefb_format { name: b"a8b8g8r8\0".as_ptr() as *const i8, bits_per_pixel: 32, red: fb_bitfield { offset: 0, length: 8, msb_right: 0 }, green: fb_bitfield { offset: 8, length: 8, msb_right: 0 }, blue: fb_bitfield { offset: 16, length: 8, msb_right: 0 }, transp: fb_bitfield { offset: 24, length: 8, msb_right: 0 }, fourcc: DRM_FORMAT_ABGR8888 },
            simplefb_format { name: b"x2r10g10b10\0".as_ptr() as *const i8, bits_per_pixel: 32, red: fb_bitfield { offset: 20, length: 10, msb_right: 0 }, green: fb_bitfield { offset: 10, length: 10, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 10, msb_right: 0 }, transp: fb_bitfield { offset: 0, length: 0, msb_right: 0 }, fourcc: DRM_FORMAT_XRGB2101010 },
            simplefb_format { name: b"a2r10g10b10\0".as_ptr() as *const i8, bits_per_pixel: 32, red: fb_bitfield { offset: 20, length: 10, msb_right: 0 }, green: fb_bitfield { offset: 10, length: 10, msb_right: 0 }, blue: fb_bitfield { offset: 0, length: 10, msb_right: 0 }, transp: fb_bitfield { offset: 30, length: 2, msb_right: 0 }, fourcc: DRM_FORMAT_ARGB2101010 },
        ]
    };
}

/* Data-Format for Simple-Framebuffers */
#[repr(C)]
pub struct simplefb_format {
    pub name: *const i8,
    pub bits_per_pixel: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub fourcc: u32,
}

/* Simple-Framebuffer description */
#[repr(C)]
pub struct simplefb_platform_data {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: *const i8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
