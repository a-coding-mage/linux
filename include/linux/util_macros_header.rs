/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/util_macros.h.
// Dependencies supplied by the surrounding translation are intentionally not
// implemented here.

/// Helper for handling conditionals in various for_each macros.
#[macro_export]
macro_rules! for_each_if {
    ($condition:expr) => {
        if !($condition) {}
        else
    };
}

/// Locate the closest element in an ascending sorted array.
///
/// Returns the index of the element closest to `x`. As in the C macro, the
/// array must be sorted in ascending order.
#[macro_export]
macro_rules! find_closest {
    ($x:expr, $a:expr, $as:expr) => {{
        let __fc_as = ($as) - 1;
        let mut __fc_i = 0;
        let __fc_x = $x;
        let __fc_a = $a;
        while __fc_i < __fc_as {
            let __fc_mid_x = (__fc_a[__fc_i] + __fc_a[__fc_i + 1]) / 2;
            if __fc_x <= __fc_mid_x {
                let __fc_left = __fc_x - __fc_a[__fc_i];
                let __fc_right = __fc_a[__fc_i + 1] - __fc_x;
                if __fc_right < __fc_left {
                    __fc_i += 1;
                }
                break;
            }
            __fc_i += 1;
        }
        __fc_i
    }};
}

/// Locate the closest element in a descending sorted array.
#[macro_export]
macro_rules! find_closest_descending {
    ($x:expr, $a:expr, $as:expr) => {{
        let __fc_as = ($as) - 1;
        let mut __fc_i = __fc_as;
        let __fc_x = $x;
        let __fc_a = $a;
        while __fc_i >= 1 {
            let __fc_mid_x = (__fc_a[__fc_i] + __fc_a[__fc_i - 1]) / 2;
            if __fc_x <= __fc_mid_x {
                let __fc_left = __fc_x - __fc_a[__fc_i];
                let __fc_right = __fc_a[__fc_i - 1] - __fc_x;
                if __fc_right < __fc_left {
                    __fc_i -= 1;
                }
                break;
            }
            __fc_i -= 1;
        }
        __fc_i
    }};
}

/// Evaluate to `ptr` if `cond` is true, or to a null pointer otherwise.
#[macro_export]
macro_rules! PTR_IF {
    ($cond:expr, $ptr:expr) => {
        if $cond { $ptr } else { ::core::ptr::null_mut() }
    };
}

/// Cast a pointer passed as `u64` from user space to a raw user pointer.
#[macro_export]
macro_rules! u64_to_user_ptr {
    ($x:expr) => {{
        // The original invokes the external C `typecheck(u64, x)` macro.
        let __x: u64 = $x;
        __x as usize as *mut core::ffi::c_void
    }};
}

/// Check whether `ptr` lies inside the memory range occupied by `var`.
#[macro_export]
macro_rules! is_insidevar {
    ($ptr:expr, $var:expr) => {
        ($ptr as usize) >= (&$var as *const _ as usize) &&
        ($ptr as usize) < (&$var as *const _ as usize) + ::core::mem::size_of_val(&$var)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
