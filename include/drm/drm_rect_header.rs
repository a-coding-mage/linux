/*
 * Copyright (C) 2011-2013 Intel Corporation
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
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependency supplied by the surrounding translation.

/** Utility functions to help manage rectangular areas for clipping, scaling, etc. calculations. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[macro_export]
macro_rules! DRM_RECT_INIT {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        $crate::drm_rect { x1: $x, y1: $y, x2: ($x).wrapping_add($w), y2: ($y).wrapping_add($h) }
    };
}

pub const DRM_RECT_FMT: &str = "%dx%d%+d%+d";

#[macro_export]
macro_rules! DRM_RECT_ARG {
    ($r:expr) => {
        (unsafe { $crate::drm_rect_width($r) }, unsafe { $crate::drm_rect_height($r) }, unsafe { (*$r).x1 }, unsafe { (*$r).y1 })
    };
}

pub const DRM_RECT_FP_FMT: &str = "%d.%06ux%d.%06u%+d.%06u%+d.%06u";

#[macro_export]
macro_rules! DRM_RECT_FP_ARG {
    ($r:expr) => {
        (unsafe { $crate::drm_rect_width($r) >> 16 }, unsafe { (($crate::drm_rect_width($r) & 0xffff).wrapping_mul(15625)) >> 10 },
         unsafe { $crate::drm_rect_height($r) >> 16 }, unsafe { (($crate::drm_rect_height($r) & 0xffff).wrapping_mul(15625)) >> 10 },
         unsafe { (*$r).x1 >> 16 }, unsafe { (((*$r).x1 & 0xffff).wrapping_mul(15625)) >> 10 },
         unsafe { (*$r).y1 >> 16 }, unsafe { (((*$r).y1 & 0xffff).wrapping_mul(15625)) >> 10 })
    };
}

#[inline]
pub unsafe fn drm_rect_init(r: *mut drm_rect, x: i32, y: i32, width: i32, height: i32) {
    (*r).x1 = x;
    (*r).y1 = y;
    (*r).x2 = x.wrapping_add(width);
    (*r).y2 = y.wrapping_add(height);
}

#[inline]
pub unsafe fn drm_rect_adjust_size(r: *mut drm_rect, dw: i32, dh: i32) {
    (*r).x1 = (*r).x1.wrapping_sub(dw >> 1);
    (*r).y1 = (*r).y1.wrapping_sub(dh >> 1);
    (*r).x2 = (*r).x2.wrapping_add(dw.wrapping_add(1) >> 1);
    (*r).y2 = (*r).y2.wrapping_add(dh.wrapping_add(1) >> 1);
}

#[inline]
pub unsafe fn drm_rect_translate(r: *mut drm_rect, dx: i32, dy: i32) {
    (*r).x1 = (*r).x1.wrapping_add(dx);
    (*r).y1 = (*r).y1.wrapping_add(dy);
    (*r).x2 = (*r).x2.wrapping_add(dx);
    (*r).y2 = (*r).y2.wrapping_add(dy);
}

#[inline]
pub unsafe fn drm_rect_translate_to(r: *mut drm_rect, x: i32, y: i32) {
    drm_rect_translate(r, x.wrapping_sub((*r).x1), y.wrapping_sub((*r).y1));
}

#[inline]
pub unsafe fn drm_rect_downscale(r: *mut drm_rect, horz: i32, vert: i32) {
    (*r).x1 /= horz;
    (*r).y1 /= vert;
    (*r).x2 /= horz;
    (*r).y2 /= vert;
}

#[inline]
pub unsafe fn drm_rect_width(r: *const drm_rect) -> i32 { (*r).x2.wrapping_sub((*r).x1) }

#[inline]
pub unsafe fn drm_rect_height(r: *const drm_rect) -> i32 { (*r).y2.wrapping_sub((*r).y1) }

#[inline]
pub unsafe fn drm_rect_visible(r: *const drm_rect) -> bool {
    drm_rect_width(r) > 0 && drm_rect_height(r) > 0
}

#[inline]
pub unsafe fn drm_rect_equals(r1: *const drm_rect, r2: *const drm_rect) -> bool {
    (*r1).x1 == (*r2).x1 && (*r1).x2 == (*r2).x2 && (*r1).y1 == (*r2).y1 && (*r1).y2 == (*r2).y2
}

#[inline]
pub unsafe fn drm_rect_fp_to_int(dst: *mut drm_rect, src: *const drm_rect) {
    drm_rect_init(dst, (*src).x1 >> 16, (*src).y1 >> 16, drm_rect_width(src) >> 16, drm_rect_height(src) >> 16);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
