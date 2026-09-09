/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Atomic bitops based version of ext2 atomic bitops
 */

macro_rules! ext2_set_bit_atomic {
    ($l:expr, $nr:expr, $addr:expr) => {
        test_and_set_bit_le($nr, $addr)
    };
}

macro_rules! ext2_clear_bit_atomic {
    ($l:expr, $nr:expr, $addr:expr) => {
        test_and_clear_bit_le($nr, $addr)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
