/* SPDX-License-Identifier: GPL-2.0 */

// The alignment primitives below are supplied by vdso/const.h.

/* @a is a power of 2 value */
macro_rules! ALIGN {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL!($x, $a)
    };
}

macro_rules! ALIGN_DOWN {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL!(($x) - (($a) - 1), $a)
    };
}

macro_rules! __ALIGN_MASK {
    ($x:expr, $mask:expr) => {
        __ALIGN_KERNEL_MASK!($x, $mask)
    };
}

macro_rules! PTR_ALIGN {
    ($p:expr, $a:expr) => {
        (($p as usize + (($a as usize) - 1)) & !((($a as usize) - 1))) as _
    };
}

macro_rules! PTR_ALIGN_DOWN {
    ($p:expr, $a:expr) => {
        (((($p as usize) - (($a as usize) - 1)) & !((($a as usize) - 1)))) as _
    };
}

macro_rules! IS_ALIGNED {
    ($x:expr, $a:expr) => {
        (($x) & (($a as _) - 1)) == 0
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
