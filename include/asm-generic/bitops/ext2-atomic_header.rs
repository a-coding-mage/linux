/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Spinlock based version of ext2 atomic bitops
 */

/* The original C header provides these as statement-expression macros. */
macro_rules! ext2_set_bit_atomic {
    ($lock:expr, $nr:expr, $addr:expr) => {{
        let ret: i32;
        spin_lock($lock);
        ret = __test_and_set_bit_le($nr, $addr);
        spin_unlock($lock);
        ret
    }};
}

macro_rules! ext2_clear_bit_atomic {
    ($lock:expr, $nr:expr, $addr:expr) => {{
        let ret: i32;
        spin_lock($lock);
        ret = __test_and_clear_bit_le($nr, $addr);
        spin_unlock($lock);
        ret
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
