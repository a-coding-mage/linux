/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020 ROHM Semiconductors */

// Translated from linux/linear_range.h.
// Dependency intent: C types supplied by <linux/types.h> are represented by
// Rust primitive types in this declaration-only translation.

/**
 * struct linear_range - table of selector - value pairs
 *
 * Define a lookup-table for range of values. Intended to help when looking
 * for a register value matching certaing physical measure (like voltage).
 * Usable when increment of one in register always results a constant increment
 * of the physical measure (like voltage).
 *
 * @min:  Lowest value in range
 * @min_sel: Lowest selector for range
 * @max_sel: Highest selector for range
 * @step: Value step size
 */
#[repr(C)]
pub struct linear_range {
    pub min: u32,
    pub min_sel: u32,
    pub max_sel: u32,
    pub step: u32,
}

#[macro_export]
macro_rules! LINEAR_RANGE {
    ($min:expr, $min_sel:expr, $max_sel:expr, $step:expr) => {
        $crate::linear_range {
            min: $min,
            min_sel: $min_sel,
            max_sel: $max_sel,
            step: $step,
        }
    };
}

#[macro_export]
macro_rules! LINEAR_RANGE_IDX {
    ($idx:expr, $min:expr, $min_sel:expr, $max_sel:expr, $step:expr) => {
        $idx: $crate::LINEAR_RANGE!($min, $min_sel, $max_sel, $step)
    };
}

extern "C" {
    pub fn linear_range_values_in_range(r: *const linear_range) -> u32;
    pub fn linear_range_values_in_range_array(r: *const linear_range, ranges: i32) -> u32;
    pub fn linear_range_get_max_value(r: *const linear_range) -> u32;

    pub fn linear_range_get_value(
        r: *const linear_range,
        selector: u32,
        val: *mut u32,
    ) -> i32;
    pub fn linear_range_get_value_array(
        r: *const linear_range,
        ranges: i32,
        selector: u32,
        val: *mut u32,
    ) -> i32;
    pub fn linear_range_get_selector_low(
        r: *const linear_range,
        val: u32,
        selector: *mut u32,
        found: *mut bool,
    ) -> i32;
    pub fn linear_range_get_selector_high(
        r: *const linear_range,
        val: u32,
        selector: *mut u32,
        found: *mut bool,
    ) -> i32;
    pub fn linear_range_get_selector_within(
        r: *const linear_range,
        val: u32,
        selector: *mut u32,
    );
    pub fn linear_range_get_selector_low_array(
        r: *const linear_range,
        ranges: i32,
        val: u32,
        selector: *mut u32,
        found: *mut bool,
    ) -> i32;
    pub fn linear_range_get_selector_high_array(
        r: *const linear_range,
        ranges: i32,
        val: u32,
        selector: *mut u32,
        found: *mut bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
