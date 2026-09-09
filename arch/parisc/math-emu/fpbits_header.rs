/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */

// PA header file -- do not include this header file for non-PA builds.

/*
 * These macros are designed to be portable to all machines that have a
 * wordsize greater than or equal to 32 bits that support the portable C
 * compiler and the standard C preprocessor. Wordsize (default 32) and
 * bitfield assignment (default left-to-right, unlike VAX, PDP-11) should be
 * predefined using the constants HOSTWDSZ and BITFRL.
 *
 * The macro arguments assume that the integer being referenced is a 32-bit
 * integer (right-justified on the 20) and that bit 0 is the most significant
 * bit.
 */

#[allow(dead_code)]
pub const HOSTWDSZ: u32 = 32;

/*
 * NewDeclareBitField_Reference - Declare a structure similar to the
 * simulator function "DeclBitfR" except its use is restricted to occur
 * within a larger enclosing structure or union definition.
 */

#[macro_export]
macro_rules! Bitfield_extract {
    ($start:expr, $length:expr, $object:expr) => {
        (($object) >> (HOSTWDSZ - ($start) - ($length))
            & (u32::MAX >> (HOSTWDSZ - ($length))))
    };
}

#[macro_export]
macro_rules! Bitfield_signed_extract {
    ($start:expr, $length:expr, $object:expr) => {
        (((($object) << ($start)) as i32) >> (HOSTWDSZ - ($length)))
    };
}

#[macro_export]
macro_rules! Bitfield_mask {
    ($start:expr, $len:expr, $object:expr) => {
        (($object)
            & ((u32::MAX >> (HOSTWDSZ - ($len)))
                << (HOSTWDSZ - ($start) - ($len))))
    };
}

#[macro_export]
macro_rules! Bitfield_deposit {
    ($value:expr, $start:expr, $len:expr, $object:expr) => {
        $object = (($object)
            & !((u32::MAX >> (HOSTWDSZ - ($len)))
                << (HOSTWDSZ - ($start) - ($len))))
            | ((($value) & (u32::MAX >> (HOSTWDSZ - ($len))))
                << (HOSTWDSZ - ($start) - ($len)))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
