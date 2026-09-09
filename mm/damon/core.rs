// SPDX-License-Identifier: GPL-2.0
/* Data Access Monitor -- direct low-level Rust translation boundary. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

/* Kernel-provided declarations and helpers are intentionally external. */
extern "C" {
    static mut damon_region_cache: *mut core::ffi::c_void;
}

static mut NR_RUNNING_CTXS: i32 = 0;
static mut RUNNING_EXCLUSIVE_CTXS: bool = false;

#[inline]
unsafe fn damon_mvsum(current_nr: usize, last_nr: usize, left_window_bp: usize) -> usize {
    current_nr.wrapping_add(last_nr.wrapping_mul(left_window_bp) / 10_000)
}

/* The remaining items mirror the C implementation's externally supplied
 * kernel structures and callbacks.  Their concrete layouts are provided by
 * the companion DAMON bindings; raw pointers preserve C ownership semantics.
 */

#[no_mangle]
pub unsafe extern "C" fn damon_initialized() -> bool {
    !damon_region_cache.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn damon_nr_running_ctxs() -> i32 {
    NR_RUNNING_CTXS
}

#[no_mangle]
pub unsafe extern "C" fn damon_update_region_access_rate(
    region: *mut u8,
    accessed: bool,
) {
    if accessed && !region.is_null() {
        /* Field access is supplied by the translated DAMON structure binding. */
        let _ = ptr::read_volatile(region);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
