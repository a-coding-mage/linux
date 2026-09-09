/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2018 BayLibre, SAS
 */

// Dependency: `u8` and `u32` correspond to the supplied kernel integer types.

pub const MESON_CANVAS_WRAP_NONE: u32 = 0x00;
pub const MESON_CANVAS_WRAP_X: u32 = 0x01;
pub const MESON_CANVAS_WRAP_Y: u32 = 0x02;

pub const MESON_CANVAS_BLKMODE_LINEAR: u32 = 0x00;
pub const MESON_CANVAS_BLKMODE_32X32: u32 = 0x01;
pub const MESON_CANVAS_BLKMODE_64X64: u32 = 0x02;

pub const MESON_CANVAS_ENDIAN_SWAP16: u32 = 0x1;
pub const MESON_CANVAS_ENDIAN_SWAP32: u32 = 0x3;
pub const MESON_CANVAS_ENDIAN_SWAP64: u32 = 0x7;
pub const MESON_CANVAS_ENDIAN_SWAP128: u32 = 0xf;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct meson_canvas {
    _private: [u8; 0],
}

extern "C" {
    /**
     * meson_canvas_get() - get a canvas provider instance
     *
     * @dev: consumer device pointer
     */
    pub fn meson_canvas_get(dev: *mut device) -> *mut meson_canvas;

    /**
     * meson_canvas_alloc() - take ownership of a canvas
     *
     * @canvas: canvas provider instance retrieved from meson_canvas_get()
     * @canvas_index: will be filled with the canvas ID
     */
    pub fn meson_canvas_alloc(
        canvas: *mut meson_canvas,
        canvas_index: *mut u8,
    ) -> i32;

    /**
     * meson_canvas_free() - remove ownership from a canvas
     *
     * @canvas: canvas provider instance retrieved from meson_canvas_get()
     * @canvas_index: canvas ID that was obtained via meson_canvas_alloc()
     */
    pub fn meson_canvas_free(canvas: *mut meson_canvas, canvas_index: u8) -> i32;

    /**
     * meson_canvas_config() - configure a canvas
     *
     * @canvas: canvas provider instance retrieved from meson_canvas_get()
     * @canvas_index: canvas ID that was obtained via meson_canvas_alloc()
     * @addr: physical address to the pixel buffer
     * @stride: width of the buffer
     * @height: height of the buffer
     * @wrap: undocumented
     * @blkmode: block mode (linear, 32x32, 64x64)
     * @endian: byte swapping (swap16, swap32, swap64, swap128)
     */
    pub fn meson_canvas_config(
        canvas: *mut meson_canvas,
        canvas_index: u8,
        addr: u32,
        stride: u32,
        height: u32,
        wrap: u32,
        blkmode: u32,
        endian: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
