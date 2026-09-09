/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */

// PA header file -- do not include this header file for non-PA builds.

/* amount is assumed to be a constant between 0 and 32 (non-inclusive) */
macro_rules! Shiftdouble {
    ($left:expr, $right:expr, $amount:expr, $dest:ident) => {
        $dest = (($left as u32).wrapping_shl(32u32 - ($amount as u32)))
            | (($right as u32) >> ($amount as u32));
    };
}

/* amount must be less than 32 */
macro_rules! Variableshiftdouble {
    ($left:expr, $right:expr, $amount:expr, $dest:ident) => {
        if $amount == 0 {
            $dest = $right;
        } else {
            $dest = (((($left as u32) & 0x7fffffff).wrapping_shl(
                32u32 - ($amount as u32),
            )) | (($right as u32) >> ($amount as u32)));
        }
    };
}

/* amount must be between 0 and 32 (non-inclusive) */
macro_rules! Variable_shift_double {
    ($left:expr, $right:expr, $amount:expr, $dest:ident) => {
        $dest = (($left as u32).wrapping_shl(32u32 - ($amount as u32)))
            | (($right as u32) >> ($amount as u32));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
