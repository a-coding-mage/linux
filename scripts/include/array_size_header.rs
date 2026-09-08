/* SPDX-License-Identifier: GPL-2.0 */

/// ARRAY_SIZE - get the number of elements in array `arr`
/// `arr`: array to be sized
#[macro_export]
macro_rules! ARRAY_SIZE {
    ($arr:expr) => {
        core::mem::size_of_val(&$arr) / core::mem::size_of_val(&$arr[0])
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
