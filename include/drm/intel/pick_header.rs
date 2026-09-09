/* SPDX-License-Identifier: MIT */
/* Copyright © 2026 Intel Corporation */

/*
 * Given the first two numbers __a and __b of arbitrarily many evenly spaced
 * numbers, pick the 0-based __index'th value.
 *
 * Always prefer this over _PICK() if the numbers are evenly spaced.
 */
macro_rules! _PICK_EVEN {
    ($index:expr, $a:expr, $b:expr) => {
        (($a) + ($index) * (($b) - ($a)))
    };
}

/*
 * Like _PICK_EVEN(), but supports 2 ranges of evenly spaced address offsets.
 * @__c_index corresponds to the index in which the second range starts to be
 * used. Using math interval notation, the first range is used for indexes [ 0,
 * @__c_index), while the second range is used for [ @__c_index, ... ). Example:
 *
 * #define _FOO_A                 0xf000
 * #define _FOO_B                 0xf004
 * #define _FOO_C                 0xf008
 * #define _SUPER_FOO_A           0xa000
 * #define _SUPER_FOO_B           0xa100
 * #define FOO(x)                 _MMIO(_PICK_EVEN_2RANGES(x, 3, \
 *                                            _FOO_A, _FOO_B, \
 *                                            _SUPER_FOO_A, _SUPER_FOO_B))
 *
 * This expands to:
 *     0: 0xf000,
 *     1: 0xf004,
 *     2: 0xf008,
 *     3: 0xa000,
 *     4: 0xa100,
 *     5: 0xa200,
 *     ...
 */
macro_rules! _PICK_EVEN_2RANGES {
    ($index:expr, $c_index:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        (BUILD_BUG_ON_ZERO(!__is_constexpr!($c_index)) +
            if ($index) < ($c_index) {
                _PICK_EVEN!($index, $a, $b)
            } else {
                _PICK_EVEN!(($index) - ($c_index), $c, $d)
            })
    };
}

/*
 * Given the arbitrary numbers in varargs, pick the 0-based __index'th number.
 *
 * Always prefer _PICK_EVEN() over this if the numbers are evenly spaced.
 */
macro_rules! _PICK {
    ($index:expr, $($value:expr),+ $(,)?) => {
        ([ $($value as u32),+ ][$index])
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
