/* SPDX-License-Identifier: GPL-2.0 */

/*
 * CRUSH functions for find rules and then mapping an input to an
 * output set.
 *
 * LGPL2
 */

// Dependency supplied by crush.h in the original header.
use crate::crush::{crush_choose_arg, crush_map};
use core::ffi::c_void;

unsafe extern "C" {
    pub fn crush_find_rule(
        map: *const crush_map,
        ruleset: i32,
        type_: i32,
        size: i32,
    ) -> i32;

    pub fn crush_do_rule(
        map: *const crush_map,
        ruleno: i32,
        x: i32,
        result: *mut i32,
        result_max: i32,
        weight: *const u32,
        weight_max: i32,
        cwin: *mut c_void,
        choose_args: *const crush_choose_arg,
    ) -> i32;

    pub fn crush_init_workspace(map: *const crush_map, v: *mut c_void);
}

/*
 * Returns the exact amount of workspace that will need to be used
 * for a given combination of crush_map and result_max. The caller can
 * then allocate this much on its own, either on the stack, in a
 * per-thread long-lived buffer, or however it likes.
 */
#[inline]
pub unsafe fn crush_work_size(map: *const crush_map, result_max: i32) -> usize {
    (*map).working_size + (result_max as usize) * 3 * core::mem::size_of::<u32>()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
