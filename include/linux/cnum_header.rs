/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from linux/types.h-dependent C declarations. */

/*
 * cnum32: a circular number.
 * A unified representation for signed and unsigned ranges.
 *
 * Assume that a 32-bit range is a circle, with 0 being in the 12 o'clock
 * position, numbers placed sequentially in clockwise order and U32_MAX
 * in the 11 o'clock position. Signed values map onto the same circle:
 * S32_MAX sits at 5 o'clock, S32_MIN sits at 6 o'clock (opposite 0),
 * negative values occupy the left half and positive values the right half.
 *
 * @cnum32 represents an arc on this circle drawn clockwise.
 * @base corresponds to the first value of the range.
 * @size corresponds to the number of integers in the range excluding @base.
 * (The @base is excluded to avoid integer overflow when representing the full
 * 0..U32_MAX range, which corresponds to 2^32, which can't be stored in u32).
 *
 * For example: {U32_MAX, 1} corresponds to signed range [-1, 0],
 *              {S32_MAX, 1} corresponds to unsigned range [S32_MAX, S32_MIN].
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnum32 {
    pub base: u32,
    pub size: u32,
}

pub const CNUM32_UNBOUNDED: cnum32 = cnum32 { base: 0, size: u32::MAX };
pub const CNUM32_EMPTY: cnum32 = cnum32 { base: u32::MAX, size: u32::MAX };

extern "C" {
    pub fn cnum32_from_urange(min: u32, max: u32) -> cnum32;
    pub fn cnum32_from_srange(min: i32, max: i32) -> cnum32;
    pub fn cnum32_umin(cnum: cnum32) -> u32;
    pub fn cnum32_umax(cnum: cnum32) -> u32;
    pub fn cnum32_smin(cnum: cnum32) -> i32;
    pub fn cnum32_smax(cnum: cnum32) -> i32;
    pub fn cnum32_intersect(a: cnum32, b: cnum32) -> cnum32;
    pub fn cnum32_intersect_with(dst: *mut cnum32, src: cnum32);
    pub fn cnum32_intersect_with_urange(dst: *mut cnum32, min: u32, max: u32);
    pub fn cnum32_intersect_with_srange(dst: *mut cnum32, min: i32, max: i32);
    pub fn cnum32_contains(cnum: cnum32, v: u32) -> bool;
    pub fn cnum32_is_const(cnum: cnum32) -> bool;
    pub fn cnum32_is_empty(cnum: cnum32) -> bool;
    pub fn cnum32_add(a: cnum32, b: cnum32) -> cnum32;
    pub fn cnum32_negate(a: cnum32) -> cnum32;
    pub fn cnum32_is_subset(outer: cnum32, inner: cnum32) -> bool;

    /* Same as cnum32 but for 64-bit ranges */
    pub fn cnum64_from_urange(min: u64, max: u64) -> cnum64;
    pub fn cnum64_from_srange(min: i64, max: i64) -> cnum64;
    pub fn cnum64_umin(cnum: cnum64) -> u64;
    pub fn cnum64_umax(cnum: cnum64) -> u64;
    pub fn cnum64_smin(cnum: cnum64) -> i64;
    pub fn cnum64_smax(cnum: cnum64) -> i64;
    pub fn cnum64_intersect(a: cnum64, b: cnum64) -> cnum64;
    pub fn cnum64_intersect_with(dst: *mut cnum64, src: cnum64);
    pub fn cnum64_intersect_with_urange(dst: *mut cnum64, min: u64, max: u64);
    pub fn cnum64_intersect_with_srange(dst: *mut cnum64, min: i64, max: i64);
    pub fn cnum64_contains(cnum: cnum64, v: u64) -> bool;
    pub fn cnum64_is_const(cnum: cnum64) -> bool;
    pub fn cnum64_is_empty(cnum: cnum64) -> bool;
    pub fn cnum64_add(a: cnum64, b: cnum64) -> cnum64;
    pub fn cnum64_negate(a: cnum64) -> cnum64;
    pub fn cnum64_is_subset(outer: cnum64, inner: cnum64) -> bool;

    pub fn cnum32_from_cnum64(cnum: cnum64) -> cnum32;
    pub fn cnum64_cnum32_intersect(a: cnum64, b: cnum32) -> cnum64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnum64 {
    pub base: u64,
    pub size: u64,
}

pub const CNUM64_UNBOUNDED: cnum64 = cnum64 { base: 0, size: u64::MAX };
pub const CNUM64_EMPTY: cnum64 = cnum64 { base: u64::MAX, size: u64::MAX };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
