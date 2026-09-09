/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the fallback for when the architecture doesn't
 * support the runtime const operations.
 *
 * We just use the actual symbols as-is.
 */
macro_rules! runtime_const_ptr {
    ($sym:expr) => {
        $sym
    };
}

macro_rules! runtime_const_shift_right_32 {
    ($val:expr, $sym:expr) => {
        (($val as u32) >> $sym)
    };
}

macro_rules! runtime_const_mask_32 {
    ($val:expr, $sym:expr) => {
        (($val as u32) & $sym)
    };
}

macro_rules! runtime_const_init {
    ($type:ty, $sym:expr) => {{
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
