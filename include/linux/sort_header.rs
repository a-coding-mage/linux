/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/types.h>: `size_t`, `cmp_func_t`,
// `cmp_r_func_t`, `swap_func_t`, and `swap_r_func_t` are supplied externally.

/**
 * cmp_int - perform a three-way comparison of the arguments
 * @l: the left argument
 * @r: the right argument
 *
 * Return: 1 if the left argument is greater than the right one; 0 if the
 * arguments are equal; -1 if the left argument is less than the right one.
 */
macro_rules! cmp_int {
    ($l:expr, $r:expr) => {
        (($l > $r) as i32) - (($l < $r) as i32)
    };
}

extern "C" {
    pub fn sort_r(
        base: *mut core::ffi::c_void,
        num: size_t,
        size: size_t,
        cmp_func: cmp_r_func_t,
        swap_func: swap_r_func_t,
        priv_: *const core::ffi::c_void,
    );

    pub fn sort(
        base: *mut core::ffi::c_void,
        num: size_t,
        size: size_t,
        cmp_func: cmp_func_t,
        swap_func: swap_func_t,
    );

    /* Versions that periodically call cond_resched(): */

    pub fn sort_r_nonatomic(
        base: *mut core::ffi::c_void,
        num: size_t,
        size: size_t,
        cmp_func: cmp_r_func_t,
        swap_func: swap_r_func_t,
        priv_: *const core::ffi::c_void,
    );

    pub fn sort_nonatomic(
        base: *mut core::ffi::c_void,
        num: size_t,
        size: size_t,
        cmp_func: cmp_func_t,
        swap_func: swap_func_t,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
