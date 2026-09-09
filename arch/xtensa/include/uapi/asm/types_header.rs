/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/types.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependency provided by asm-generic/int-ll64.h.

// The assembler branch expands these macros without a cast.  Rust source
// translation uses the non-assembler form below.
macro_rules! __XTENSA_UL {
    ($x:expr) => {
        ($x) as usize
    };
}

macro_rules! ___XTENSA_UL_CONST {
    ($x:expr) => {
        $x
    };
}

macro_rules! __XTENSA_UL_CONST {
    ($x:expr) => {
        ___XTENSA_UL_CONST!($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
