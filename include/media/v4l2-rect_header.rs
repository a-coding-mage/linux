/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * v4l2-rect.h - v4l2_rect helper functions
 *
 * Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

/* The v4l2_rect type is supplied by the Linux videodev2 dependency. */

/// v4l2_rect_set_size_to() - copy the width/height values.
/// @r: rect whose width and height fields will be set
/// @size: rect containing the width and height fields you need.
#[inline]
pub unsafe fn v4l2_rect_set_size_to(r: *mut v4l2_rect, size: *const v4l2_rect) {
    (*r).width = (*size).width;
    (*r).height = (*size).height;
}

/// v4l2_rect_set_min_size() - width and height of r should be >= min_size.
#[inline]
pub unsafe fn v4l2_rect_set_min_size(r: *mut v4l2_rect, min_size: *const v4l2_rect) {
    if (*r).width < (*min_size).width { (*r).width = (*min_size).width; }
    if (*r).height < (*min_size).height { (*r).height = (*min_size).height; }
}

/// v4l2_rect_set_max_size() - width and height of r should be <= max_size
#[inline]
pub unsafe fn v4l2_rect_set_max_size(r: *mut v4l2_rect, max_size: *const v4l2_rect) {
    if (*r).width > (*max_size).width { (*r).width = (*max_size).width; }
    if (*r).height > (*max_size).height { (*r).height = (*max_size).height; }
}

/// v4l2_rect_map_inside()- r should be inside boundary.
#[inline]
pub unsafe fn v4l2_rect_map_inside(r: *mut v4l2_rect, boundary: *const v4l2_rect) {
    v4l2_rect_set_max_size(r, boundary);
    if (*r).left < (*boundary).left { (*r).left = (*boundary).left; }
    if (*r).top < (*boundary).top { (*r).top = (*boundary).top; }
    if (*r).left.wrapping_add((*r).width as _) > (*boundary).left.wrapping_add((*boundary).width as _) {
        (*r).left = (*boundary).left.wrapping_add((*boundary).width as _).wrapping_sub((*r).width as _);
    }
    if (*r).top.wrapping_add((*r).height as _) > (*boundary).top.wrapping_add((*boundary).height as _) {
        (*r).top = (*boundary).top.wrapping_add((*boundary).height as _).wrapping_sub((*r).height as _);
    }
}

#[inline]
pub unsafe fn v4l2_rect_same_size(r1: *const v4l2_rect, r2: *const v4l2_rect) -> bool {
    (*r1).width == (*r2).width && (*r1).height == (*r2).height
}

#[inline]
pub unsafe fn v4l2_rect_same_position(r1: *const v4l2_rect, r2: *const v4l2_rect) -> bool {
    (*r1).top == (*r2).top && (*r1).left == (*r2).left
}

#[inline]
pub unsafe fn v4l2_rect_equal(r1: *const v4l2_rect, r2: *const v4l2_rect) -> bool {
    v4l2_rect_same_size(r1, r2) && v4l2_rect_same_position(r1, r2)
}

#[inline]
pub unsafe fn v4l2_rect_intersect(r: *mut v4l2_rect, r1: *const v4l2_rect, r2: *const v4l2_rect) {
    let bottom = core::cmp::min((*r1).top.wrapping_add((*r1).height as _), (*r2).top.wrapping_add((*r2).height as _));
    let right = core::cmp::min((*r1).left.wrapping_add((*r1).width as _), (*r2).left.wrapping_add((*r2).width as _));
    (*r).top = core::cmp::max((*r1).top, (*r2).top);
    (*r).left = core::cmp::max((*r1).left, (*r2).left);
    (*r).height = core::cmp::max(0 as _, bottom.wrapping_sub((*r).top)) as _;
    (*r).width = core::cmp::max(0 as _, right.wrapping_sub((*r).left)) as _;
}

#[inline]
pub unsafe fn v4l2_rect_scale(r: *mut v4l2_rect, from: *const v4l2_rect, to: *const v4l2_rect) {
    if (*from).width == 0 || (*from).height == 0 {
        (*r).left = 0; (*r).top = 0; (*r).width = 0; (*r).height = 0;
        return;
    }
    (*r).left = ((((*r).left - (*from).left) * (*to).width as _) / (*from).width as _) & !1;
    (*r).width = (((*r).width * (*to).width) / (*from).width) & !1;
    (*r).top = (((*r).top - (*from).top) * (*to).height as _) / (*from).height as _;
    (*r).height = ((*r).height * (*to).height) / (*from).height;
}

#[inline]
pub unsafe fn v4l2_rect_overlap(r1: *const v4l2_rect, r2: *const v4l2_rect) -> bool {
    if (*r1).left >= (*r2).left.wrapping_add((*r2).width as _) || (*r2).left >= (*r1).left.wrapping_add((*r1).width as _) { return false; }
    if (*r1).top >= (*r2).top.wrapping_add((*r2).height as _) || (*r2).top >= (*r1).top.wrapping_add((*r1).height as _) { return false; }
    true
}

#[inline]
pub unsafe fn v4l2_rect_enclosed(r1: *mut v4l2_rect, r2: *mut v4l2_rect) -> bool {
    if (*r1).left < (*r2).left || (*r1).top < (*r2).top { return false; }
    if (*r1).left.wrapping_add((*r1).width as _) > (*r2).left.wrapping_add((*r2).width as _) { return false; }
    if (*r1).top.wrapping_add((*r1).height as _) > (*r2).top.wrapping_add((*r2).height as _) { return false; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
