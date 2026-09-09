/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/types.h>.
use core::ffi::c_ulong;

extern "C" {
    pub fn is_prime_number(x: c_ulong) -> bool;
    pub fn next_prime_number(x: c_ulong) -> c_ulong;
}

/**
 * for_each_prime_number - iterate over each prime upto a value
 * @prime: the current prime number in this iteration
 * @max: the upper limit
 *
 * Starting from the first prime number 2 iterate over each prime number up to
 * the @max value. On each iteration, @prime is set to the current prime number.
 * @max should be less than ULONG_MAX to ensure termination. To begin with
 * @prime set to 1 on the first iteration use for_each_prime_number_from()
 * instead.
 */
#[macro_export]
macro_rules! for_each_prime_number {
    ($prime:expr, $max:expr) => {
        for_each_prime_number_from!($prime, 2, $max)
    };
}

/**
 * for_each_prime_number_from - iterate over each prime upto a value
 * @prime: the current prime number in this iteration
 * @from: the initial value
 * @max: the upper limit
 *
 * Starting from @from iterate over each successive prime number up to the
 * @max value. On each iteration, @prime is set to the current prime number.
 * @max should be less than ULONG_MAX, and @from less than @max, to ensure
 * termination.
 */
#[macro_export]
macro_rules! for_each_prime_number_from {
    ($prime:expr, $from:expr, $max:expr) => {
        for $prime = $from; $prime <= $max; $prime = unsafe { next_prime_number($prime) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
