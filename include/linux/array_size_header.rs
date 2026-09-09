/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: __must_be_array is supplied by linux/compiler.h.

/**
 * ARRAY_SIZE - get the number of elements in array `arr`
 * `arr`: array to be sized
 */
#[macro_export]
macro_rules! ARRAY_SIZE {
    ($arr:expr) => {
        (core::mem::size_of_val(&$arr) / core::mem::size_of_val(&$arr[0])
            + __must_be_array!($arr))
    };
}

/**
 * ARRAY_END - get a pointer to one past the last element in array `arr`
 * `arr`: array
 */
#[macro_export]
macro_rules! ARRAY_END {
    ($arr:expr) => {
        core::ptr::addr_of!($arr[ARRAY_SIZE!($arr)])
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
