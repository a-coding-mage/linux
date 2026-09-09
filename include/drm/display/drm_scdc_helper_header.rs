/*
 * Copyright (c) 2015 NVIDIA Corporation. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sub license,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the corresponding Linux/Rust bindings:
// linux/types.h and drm/display/drm_scdc.h

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_scdc_read(
        adapter: *mut i2c_adapter,
        offset: u8,
        buffer: *mut core::ffi::c_void,
        size: usize,
    ) -> isize;

    pub fn drm_scdc_write(
        adapter: *mut i2c_adapter,
        offset: u8,
        buffer: *const core::ffi::c_void,
        size: usize,
    ) -> isize;
}

/**
 * drm_scdc_readb - read a single byte from SCDC
 * @adapter: I2C adapter
 * @offset: offset of register to read
 * @value: return location for the register value
 *
 * Reads a single byte from SCDC. This is a convenience wrapper around the
 * drm_scdc_read() function.
 *
 * Returns:
 * 0 on success or a negative error code on failure.
 */
#[inline]
pub unsafe fn drm_scdc_readb(
    adapter: *mut i2c_adapter,
    offset: u8,
    value: *mut u8,
) -> i32 {
    drm_scdc_read(adapter, offset, value.cast(), core::mem::size_of::<u8>()) as i32
}

/**
 * drm_scdc_writeb - write a single byte to SCDC
 * @adapter: I2C adapter
 * @offset: offset of register to read
 * @value: return location for the register value
 *
 * Writes a single byte to SCDC. This is a convenience wrapper around the
 * drm_scdc_write() function.
 *
 * Returns:
 * 0 on success or a negative error code on failure.
 */
#[inline]
pub unsafe fn drm_scdc_writeb(
    adapter: *mut i2c_adapter,
    offset: u8,
    value: u8,
) -> i32 {
    drm_scdc_write(
        adapter,
        offset,
        (&value as *const u8).cast(),
        core::mem::size_of::<u8>(),
    ) as i32
}

extern "C" {
    pub fn drm_scdc_get_scrambling_status(connector: *mut drm_connector) -> bool;

    pub fn drm_scdc_set_scrambling(connector: *mut drm_connector, enable: bool) -> bool;
    pub fn drm_scdc_set_high_tmds_clock_ratio(connector: *mut drm_connector, set: bool) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
